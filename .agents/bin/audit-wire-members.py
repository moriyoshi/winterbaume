#!/usr/bin/env python3
"""audit-wire-members.py — surface modelled members a service crate never touches.

The Smithy-generated `model.rs` / `wire.rs` of a service crate parse every
request member and can serialise every response member.  Nothing forces the
hand-written `handlers.rs` to use them, and operation coverage is counted per
operation, so a member that is deserialised and then dropped ( or modelled in
an output and never populated ) is invisible to every other report.  GitHub
issues #41 – #47 and #51 – #54 were all of this class.

Four mechanical heuristics over `src/model.rs`, `src/wire.rs` and
`src/handlers.rs`:

  A. Unread request members
     For each `wire::deserialize_<op>_request(...)` call, the members of the
     operation's input structure that the enclosing handler never reads
     through the bound variable.  A call whose result is discarded, or bound
     to `_`, reports every member.

  B. Deserialiser never called
     An operation the handlers route without calling its generated request
     deserialiser: either unimplemented, or parsed by hand, in which case the
     header / query bindings the generated code handles are likely missing.

  C. Never-populated response members
     For each generated model structure the handlers build with a struct
     literal, the members no literal anywhere in the crate's hand-written
     sources ever sets.

  D. Unevenly populated response members
     Members that some literals of a structure set and others leave to
     `..Default::default()`: the same resource described fully by one
     operation and partially by another ( `Role.Tags` from `GetRole`, #52 ).

False positives are expected: a member may legitimately be unsupported, be
read through a helper that takes the whole input, or be an output AWS itself
omits.  The report is a worklist for the request modifier inventory in
QUALITY_GATE.md §5.3, not a compile gate.  It cannot see a member that is
read and then ignored, nor a wrong state transition; only reading the
operation against its AWS contract finds those.

Usage:
  .agents/bin/audit-wire-members.py <crate-suffix> [...]   # e.g. lambda s3
  .agents/bin/audit-wire-members.py --all
  .agents/bin/audit-wire-members.py --dir <path-to-crate-src> # audit a copy

Options:
  --summary   print one line of counts per crate instead of the full report
  --op NAME   restrict sections A and B to one operation ( snake_case )
  --dossier   print the body of a service dossier's `## Member Audit` section

Output goes to stdout; redirect it under `.agents-workspace/tmp/`.
"""

import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]

STRUCT_RE = re.compile(r"^pub struct (\w+)\s*\{(.*?)^\}", re.M | re.S)
FIELD_RE = re.compile(r"^\s+pub (r#)?(\w+):", re.M)
DESER_RE = re.compile(
    r"pub fn deserialize_(\w+)_request\s*\((.*?)\)\s*->\s*Result<\s*([\w:]+)", re.S
)
FN_RE = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+\w+", re.M)


def strip_comments(src: str) -> str:
    src = re.sub(r"/\*.*?\*/", "", src, flags=re.S)
    return re.sub(r"//[^\n]*", "", src)


def model_structs(model_src: str) -> dict:
    return {
        m.group(1): [f.group(2) for f in FIELD_RE.finditer(m.group(2))]
        for m in STRUCT_RE.finditer(model_src)
    }


def matching_brace(src: str, open_idx: int) -> int:
    depth = 0
    for i in range(open_idx, len(src)):
        c = src[i]
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                return i
    return len(src) - 1


def enclosing_fn_body(src: str, pos: int) -> str:
    """The text of the function containing `pos`, from its `fn` to its end."""
    start = 0
    for m in FN_RE.finditer(src, 0, pos):
        start = m.start()
    brace = src.find("{", start)
    if brace < 0 or brace > pos:
        return src[start : pos + 2000]
    return src[start : matching_brace(src, brace) + 1]


def bound_variable(src: str, call_pos: int):
    """Name the deserialised input is bound to, or None when it is discarded."""
    # Look back over a short window: `let input =\n    match wire::...`.
    window = src[max(0, call_pos - 200) : call_pos]
    m = None
    for m in re.finditer(r"let\s+(?:mut\s+)?(\w+)\s*(?::[^=]+)?=\s*(?:match\s+)?(?:\w+::)*$", window):
        pass
    if m:
        return None if m.group(1) == "_" else m.group(1)
    m = None
    for m in re.finditer(r"Ok\((?:mut\s+)?(\w+)\)\s*=\s*(?:\w+::)*$", window):
        pass
    if m:
        return None if m.group(1) == "_" else m.group(1)
    return None


def audit_requests(handlers: str, wire: str, structs: dict, only_op=None):
    unread, never_called = [], []
    for m in DESER_RE.finditer(wire):
        op, ty = m.group(1), m.group(3).split("::")[-1]
        if only_op and op != only_op:
            continue
        fields = structs.get(ty)
        calls = [c.start() for c in re.finditer(rf"\bdeserialize_{op}_request\s*\(", handlers)]
        if not calls:
            never_called.append((op, ty, len(fields or [])))
            continue
        if not fields:
            continue
        read, whole, discarded = set(), False, False
        for pos in calls:
            var = bound_variable(handlers, pos)
            if var is None:
                discarded = True
                continue
            body = enclosing_fn_body(handlers, pos)
            read.update(re.findall(rf"\b{re.escape(var)}\s*\.\s*(?:r#)?(\w+)", body))
            # Destructuring: `let Ty { a, b, .. } = input;`
            for d in re.finditer(rf"{ty}\s*\{{([^}}]*)\}}\s*=\s*{re.escape(var)}\b", body):
                read.update(re.findall(r"(\w+)\s*(?:,|:|$)", d.group(1)))
            # The whole value handed to a helper: the helper may read anything.
            if re.search(rf"[(,]\s*&?(?:mut\s+)?{re.escape(var)}\s*[,)]", body):
                whole = True
        missing = [f for f in fields if f not in read]
        if missing and not whole:
            unread.append((op, ty, missing, len(fields), discarded and not read))
    return unread, never_called


def audit_responses(sources: str, structs: dict):
    """Members of model structs that no struct literal ever sets."""
    set_fields, literal_count, per_field = {}, {}, {}
    for m in re.finditer(r"\b(?:model::|wire::)?(\w+)\s*\{", sources):
        ty = m.group(1)
        if ty not in structs:
            continue
        before = sources[max(0, m.start() - 12) : m.start()]
        if re.search(r"(struct|impl|for|enum|trait|mod)\s+$", before):
            continue
        end = matching_brace(sources, m.end() - 1)
        body = sources[m.end() : end]
        # Only top-level `field:` / shorthand `field,` entries of this literal.
        depth, top = 0, []
        for ch in body:
            if ch in "{([":
                depth += 1
            elif ch in "})]":
                depth -= 1
            top.append(ch if depth == 0 else " ")
        flat = "".join(top)
        names = set(re.findall(r"(?:^|,)\s*(?:r#)?(\w+)\s*(?::|,|$)", flat))
        if not names and ".." not in flat:
            continue
        literal_count[ty] = literal_count.get(ty, 0) + 1
        set_fields.setdefault(ty, set()).update(names)
        counts = per_field.setdefault(ty, {})
        for n in names:
            counts[n] = counts.get(n, 0) + 1
    never, uneven = [], []
    for ty, names in sorted(set_fields.items()):
        missing = [f for f in structs[ty] if f not in names]
        if missing:
            never.append((ty, missing, len(structs[ty]), literal_count[ty]))
        total = literal_count[ty]
        partial = [
            (f, per_field[ty][f]) for f in structs[ty] if 0 < per_field[ty].get(f, 0) < total
        ]
        if partial and total > 1:
            uneven.append((ty, partial, total))
    return never, uneven


def dossier_section(label, unread, never_called, responses, uneven):
    """Markdown for the `## Member Audit` section of a service dossier."""
    out = [
        f"Generated by `.agents/bin/audit-wire-members.py {label.replace('winterbaume-', '')} --dossier`. "
        "Rows are unclassified candidates, not confirmed defects: each one is resolved as honoured, "
        "rejected with the AWS error, ignored because AWS ignores it, or filed in `TODO.md` "
        "( QUALITY_GATE.md §5.3 and §5.4 ). Regenerate after changing handlers.",
        "",
        f"- Unread request members: {sum(len(u[2]) for u in unread)} in {len(unread)} operations.",
        f"- Operations whose generated request deserialiser is never called: {len(never_called)}.",
        f"- Never-populated response members: {sum(len(r[1]) for r in responses)} in "
        f"{len(responses)} structures ( run the script for the list ).",
        f"- Response structures populated unevenly across literals: {len(uneven)}.",
        "",
        "### Unread request members",
        "",
    ]
    for op, ty, missing, total, discarded in sorted(unread):
        note = " ( input discarded )" if discarded else ""
        out.append(f"- `{op}` ( {len(missing)} of {total} ){note}: " + ", ".join(f"`{f}`" for f in missing))
    if not unread:
        out.append("- none")
    out += ["", "### Operations not using the generated request deserialiser", ""]
    if never_called:
        out.append(", ".join(f"`{op}`" for op, _, _ in sorted(never_called)))
    else:
        out.append("- none")
    out += ["", "### Unevenly populated response structures", ""]
    for ty, partial, total in uneven:
        out.append(f"- `{ty}` ( {total} literals ): " + ", ".join(f"`{f}` {n}/{total}" for f, n in partial))
    if not uneven:
        out.append("- none")
    return "\n".join(out) + "\n"


def audit(src_dir: Path, summary: bool, only_op=None, label=None, dossier=False):
    label = label or src_dir.parent.name
    try:
        model = (src_dir / "model.rs").read_text()
        wire = (src_dir / "wire.rs").read_text()
        handlers = strip_comments((src_dir / "handlers.rs").read_text())
    except FileNotFoundError as e:
        if not summary:
            print(f"# {label}: skipped ( {e.filename} not found )\n")
        return
    structs = model_structs(model)
    hand_written = "\n".join(
        strip_comments(p.read_text())
        for p in sorted(src_dir.rglob("*.rs"))
        if p.name not in ("model.rs", "wire.rs")
    )
    unread, never_called = audit_requests(handlers, wire, structs, only_op)
    responses, uneven = ([], []) if only_op else audit_responses(hand_written, structs)
    if dossier:
        print(dossier_section(label, unread, never_called, responses, uneven), end="")
        return
    if summary:
        print(
            f"{label}: {sum(len(u[2]) for u in unread)} unread request members in "
            f"{len(unread)} operations; {len(never_called)} deserialisers never called; "
            f"{sum(len(r[1]) for r in responses)} never-populated members in "
            f"{len(responses)} response structures; {len(uneven)} structures populated "
            f"unevenly"
        )
        return
    print(f"# Wire member audit: {label}\n")
    print("## A. Unread request members\n")
    for op, ty, missing, total, discarded in sorted(unread):
        note = " ( input discarded )" if discarded else ""
        print(f"- `{op}` ( `{ty}`, {len(missing)} of {total} ){note}: " + ", ".join(f"`{f}`" for f in missing))
    if not unread:
        print("- none")
    print("\n## B. Deserialiser never called\n")
    for op, ty, n in sorted(never_called):
        print(f"- `{op}` ( `{ty}`, {n} members )")
    if not never_called:
        print("- none")
    if not only_op:
        print("\n## C. Never-populated response members\n")
        for ty, missing, total, n in responses:
            print(f"- `{ty}` ( {len(missing)} of {total}, {n} literals ): " + ", ".join(f"`{f}`" for f in missing))
        if not responses:
            print("- none")
        print("\n## D. Members populated by some literals of a structure and not others\n")
        for ty, partial, total in uneven:
            print(
                f"- `{ty}` ( {total} literals ): "
                + ", ".join(f"`{f}` {n}/{total}" for f, n in partial)
            )
        if not uneven:
            print("- none")
    print()


def main(argv):
    summary = "--summary" in argv
    dossier = "--dossier" in argv
    argv = [a for a in argv if a not in ("--summary", "--dossier")]
    only_op = None
    if "--op" in argv:
        i = argv.index("--op")
        only_op = argv[i + 1]
        del argv[i : i + 2]
    if not argv or argv[0] in ("-h", "--help"):
        print(__doc__)
        return 0 if argv else 2
    if argv[0] == "--dir":
        audit(Path(argv[1]), summary, only_op, label=argv[1], dossier=dossier)
        return 0
    if argv[0] == "--all":
        dirs = sorted((REPO_ROOT / "crates").glob("winterbaume-*/src"))
    else:
        dirs = [REPO_ROOT / "crates" / f"winterbaume-{a}" / "src" for a in argv]
    for d in dirs:
        audit(d, summary, only_op, dossier=dossier)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
