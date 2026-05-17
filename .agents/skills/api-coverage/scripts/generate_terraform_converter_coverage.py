#!/usr/bin/env python3
"""Generate Terraform converter field coverage report for winterbaume.

Analyses each converter in winterbaume-terraform to determine which
Terraform resource attributes are handled in inject (Terraform →
winterbaume) and extract (winterbaume → Terraform), then compares
against the official Terraform AWS provider schema.

Usage:
    python3 .agents/skills/api-coverage/scripts/generate_terraform_converter_coverage.py [--output PATH]

Default output: .agents/docs/TERRAFORM_CONVERTER_COVERAGE.md

Requirements:
    - Terraform CLI with hashicorp/aws provider (for schema extraction)
    - Or a pre-generated aws_provider_schema.json in the working directory
"""

import argparse
import re
import sys
from collections import defaultdict
from datetime import date
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from tf_schema import collect_schema_attributes, get_terraform_schema  # noqa: E402

ROOT = Path(__file__).resolve().parents[4]  # repo root
CONVERTERS_DIR = ROOT / "crates" / "winterbaume-terraform" / "src" / "converters"
GENERATED_MODELS_DIR = (
    ROOT / "crates" / "winterbaume-tfstate-resource-models" / "src"
)


# ---------------------------------------------------------------------------
# Generated TfModel index
# ---------------------------------------------------------------------------

# Matches a generated `pub struct FooTfModel { ... }` block. The fields
# are extracted from `group(2)`; `#[serde(rename = "x")]` attributes are
# parsed separately per field below.
_TF_MODEL_BLOCK_RE = re.compile(
    r'pub\s+struct\s+(\w+TfModel)\s*\{([^}]*)\}',
    re.DOTALL,
)
# Matches one struct field, optionally preceded by attributes. Captures
# the attribute block (group 1) and the Rust field identifier (group 2).
_TF_MODEL_FIELD_RE = re.compile(
    r'((?:\s*#\[[^\]]*\]\s*)*)\s*pub\s+(\w+)\s*:',
)
_SERDE_RENAME_RE = re.compile(r'serde\s*\(\s*[^)]*rename\s*=\s*"([^"]+)"')


def index_generated_tf_models() -> dict[str, set[str]]:
    """Walk the generated TfModel crate and return `{StructName -> {tf_attrs}}`.

    The EC2-and-other converters consume codegen'd `*TfModel` structs via
    `serde_json::from_value(attrs.clone())` rather than touching the
    `attrs` map directly. Without this index the inject heuristic
    massively under-counts those converters' field coverage.
    """
    index: dict[str, set[str]] = {}
    if not GENERATED_MODELS_DIR.is_dir():
        return index
    for source_path in GENERATED_MODELS_DIR.glob("*.rs"):
        text = source_path.read_text()
        for block in _TF_MODEL_BLOCK_RE.finditer(text):
            struct_name = block.group(1)
            body = block.group(2)
            attrs: set[str] = set()
            for field in _TF_MODEL_FIELD_RE.finditer(body):
                attr_block, field_name = field.group(1), field.group(2)
                rename = _SERDE_RENAME_RE.search(attr_block) if attr_block else None
                if rename:
                    attrs.add(rename.group(1))
                else:
                    # Generated codegen uses trailing `_` to escape Rust
                    # keywords ( e.g. `default_` ); strip it so the name
                    # matches the underlying TF attribute.
                    attrs.add(field_name.rstrip("_"))
            attrs.discard("id")
            if attrs:
                index[struct_name] = attrs
    return index


# Matches a `*TfModel` reference inside an inject method body. The
# converter source uses patterns like `let model: ec2_gen::VpcTfModel
# = serde_json::from_value(...)`. We only need the struct name.
_TF_MODEL_REF_RE = re.compile(r'\b(\w+TfModel)\b')

# Matches the start of a snake_case macro invocation. The body
# delimiter ( either `(` or `{` ) is captured so we can find the
# matching closing delimiter by depth counting. Used to extract
# resource-type literals from macro-emitted converters like the
# per-service `*_warning_only_converter!` macros and S3's
# `impl_bucket_*_converter!` macros.
_MACRO_INVOCATION_START_RE = re.compile(r'([a-z_][a-z0-9_]*)\s*!\s*([({])')


def _macro_invocation_bodies(source: str) -> list[tuple[str, str]]:
    """Return `[(macro_name, body)]` for every macro invocation in `source`.

    The body string is the bracket-balanced content between the
    invocation's opening delimiter and the matching closing one. The
    enclosing delimiters are not included.
    """
    pairs = {"(": ")", "{": "}"}
    out: list[tuple[str, str]] = []
    for m in _MACRO_INVOCATION_START_RE.finditer(source):
        open_pos = m.end() - 1
        open_char = source[open_pos]
        close_char = pairs[open_char]
        depth = 1
        i = open_pos + 1
        n = len(source)
        while i < n and depth > 0:
            c = source[i]
            if c == open_char:
                depth += 1
            elif c == close_char:
                depth -= 1
            i += 1
        if depth == 0:
            out.append((m.group(1), source[open_pos + 1 : i - 1]))
    return out


# ---------------------------------------------------------------------------
# Converter source analysis
# ---------------------------------------------------------------------------

# Patterns matching attribute key extraction in inject methods
INJECT_PATTERNS = [
    # require_str(attrs, "key", ...)
    re.compile(r'require_str\s*\(\s*attrs\s*,\s*"([^"]+)"'),
    # optional_str(attrs, "key")
    re.compile(r'optional_str\s*\(\s*attrs\s*,\s*"([^"]+)"'),
    # optional_bool(attrs, "key")
    re.compile(r'optional_bool\s*\(\s*attrs\s*,\s*"([^"]+)"'),
    # optional_i64(attrs, "key")
    re.compile(r'optional_i64\s*\(\s*attrs\s*,\s*"([^"]+)"'),
    # optional_f64(attrs, "key")
    re.compile(r'optional_f64\s*\(\s*attrs\s*,\s*"([^"]+)"'),
    # attrs.get("key") or attrs["key"]
    re.compile(r'attrs\s*(?:\.\s*get\s*\(\s*|(?:\[))\s*"([^"]+)"'),
    # extract_tags(attrs) implies "tags" and "tags_all"
    re.compile(r'extract_tags\s*\(\s*attrs\s*\)'),
]

# Patterns matching attribute key output in extract methods
EXTRACT_PATTERNS = [
    # serde_json::json!({ "key": ... })
    re.compile(r'"([^"]+)"\s*:'),
    # obj.insert("key", ...) or attrs.insert("key", ...)
    re.compile(r'\w+\s*\.\s*insert\s*\(\s*"([^"]+)"'),
]


# Matches the trailing positional string-literal argument of a macro
# invocation. The S3 `impl_bucket_subresource_converter!` family emits
# extracted attributes through a `$extract_attr` argument that appears
# as the last positional `"name"` literal in the macro body. Standard
# `"key":` and `.insert("key", ...)` patterns do not credit those
# literals.
#
# A "trailing positional string literal" here is a `"name"` (lowercase
# ASCII identifier shape) that appears at the very end of the macro
# body, allowing only whitespace, an optional trailing comma, and
# comments after it.
_MACRO_TRAILING_STRING_LITERAL_RE = re.compile(
    r'"([a-z][a-z0-9_]*)"\s*,?\s*\Z',
)


def _macro_trailing_extract_attrs(body: str) -> set[str]:
    """Return the trailing positional `"name"` literal of a macro body, if any.

    Stripping shell-style trailing whitespace and a trailing comma, the
    last token of many converter-emitting macros is the
    `$extract_attr:literal` argument naming the TF attribute the
    converter writes back when extracting state. Crediting this token
    as an extract attribute lets the heuristic reflect macro-emitted
    coverage that the `"key":` and `.insert("key", ...)` patterns miss.
    """
    m = _MACRO_TRAILING_STRING_LITERAL_RE.search(body)
    if not m:
        return set()
    return {m.group(1)}


# Per-macro-family always-credited attributes. The S3 sub-resource
# macros hard-code `"bucket"` (and a synthetic `"id"`, which the TF
# schema strips anyway) into every emitted converter's extract body
# regardless of the macro arguments, so any invocation of those macros
# legitimately emits a `bucket` attribute the per-invocation arg
# scanner can't see.
_MACRO_ALWAYS_EXTRACT_ATTRS: dict[str, set[str]] = {
    "impl_bucket_subresource_converter": {"bucket"},
    "impl_bucket_named_config_converter": {"bucket", "name"},
}
# Inject side: the macros require `"bucket"` (and `"name"` for the
# named-config variant) via `attrs.get("bucket")` / `attrs.get("name")`
# in the macro template body, which the `attrs.get("key")` regex would
# normally credit — but those calls live in the macro definition, not
# the invocation arguments scanned by the heuristic. Always credit
# them on the inject side too.
_MACRO_ALWAYS_INJECT_ATTRS: dict[str, set[str]] = {
    "impl_bucket_subresource_converter": {"bucket"},
    "impl_bucket_named_config_converter": {"bucket", "name"},
}


def parse_converter_file(
    path: Path,
    tf_model_index: dict[str, set[str]] | None = None,
) -> list[dict]:
    """Parse a converter .rs file and return info for each converter.

    Returns a list of dicts with keys:
        resource_type, inject_attrs, extract_attrs, depends_on
    """
    source = path.read_text()
    tf_model_index = tf_model_index or {}

    # Find all resource types defined in this file via the trait impl.
    resource_types = re.findall(
        r'fn\s+resource_type\s*\(\s*&self\s*\)\s*->\s*&str\s*\{\s*"([^"]+)"',
        source,
    )

    # Some converter families ( e.g. `*_warning_only_converter!`,
    # `impl_bucket_subresource_converter!` ) are declared via macros;
    # their `fn resource_type` body refers to a macro parameter rather
    # than a string literal, so the regex above misses them. Scan macro
    # invocations and collect `(macro_name, resource_type, macro_body)`
    # triples for any that aren't already covered.
    macro_triples: list[tuple[str, str, str]] = []
    existing = set(resource_types)
    for macro_name, body in _macro_invocation_bodies(source):
        match = re.search(r'"(aws_[A-Za-z0-9_]+)"', body)
        if not match:
            continue
        rt = match.group(1)
        if rt in existing:
            continue
        macro_triples.append((macro_name, rt, body))
        existing.add(rt)

    if not resource_types and not macro_triples:
        return []

    # Split source into converter impl blocks.
    # Each converter has a do_inject and do_extract method.
    # We use the struct name pattern to split.
    results = []

    for rt in resource_types:
        # Find the impl block that contains this resource_type
        # Look for do_inject and do_extract near this resource_type
        inject_attrs = set()
        extract_attrs = set()

        # Find do_inject body for this resource type
        # Strategy: find the impl block that returns this resource_type,
        # then find its do_inject method
        inject_match = _find_method_body(source, rt, "do_inject")
        extract_match = _find_method_body(source, rt, "do_extract")

        if inject_match:
            for pat in INJECT_PATTERNS:
                for m in pat.finditer(inject_match):
                    if m.group(0).startswith("extract_tags"):
                        inject_attrs.add("tags")
                        inject_attrs.add("tags_all")
                    else:
                        inject_attrs.add(m.group(1))
            # "region" is always extracted via extract_region
            if "extract_region" in inject_match:
                inject_attrs.add("region")
            # Converters that consume a codegen'd `*TfModel` via
            # `serde_json::from_value(attrs.clone())` access TF attributes
            # through the deserialised struct, which the attribute-key
            # regexes above can't see. Credit every field of any
            # referenced TfModel ( resolved against the index of the
            # generated-models crate ).
            for ref in _TF_MODEL_REF_RE.finditer(inject_match):
                fields = tf_model_index.get(ref.group(1))
                if fields:
                    inject_attrs.update(fields)

        if extract_match:
            for pat in EXTRACT_PATTERNS:
                for m in pat.finditer(extract_match):
                    extract_attrs.add(m.group(1))

        # Remove internal-only keys that aren't Terraform attributes
        for internal_key in ("resource_type",):
            inject_attrs.discard(internal_key)
            extract_attrs.discard(internal_key)

        # Find depends_on_types for this specific converter's trait impl
        dep_types = []
        if inject_match:
            # The depends_on_types is in the TerraformResourceConverter impl
            # for this struct, search near the resource_type method
            dep_pattern = re.compile(
                r'impl\s+TerraformResourceConverter\s+for\s+'
                + re.escape(rt.replace("aws_", "Aws").title().replace("_", ""))
                + r'.*?fn\s+depends_on_types.*?\{([^}]*)\}',
                re.DOTALL,
            )
            dep_match = dep_pattern.search(source)
            if dep_match:
                dep_types = re.findall(r'"([^"]+)"', dep_match.group(1))

        results.append({
            "resource_type": rt,
            "inject_attrs": sorted(inject_attrs),
            "extract_attrs": sorted(extract_attrs),
            "depends_on": dep_types,
            "source_file": str(path.relative_to(ROOT)),
        })

    # Macro-emitted converters: treat the entire macro invocation body
    # as the inject/extract body. Both the warning-only macros and the
    # S3 sub-resource macros credit any referenced `*TfModel` and any
    # explicit `attrs.get("...")` / `serde_json::json!({...})` calls
    # within the macro args.
    for macro_name, rt, body in macro_triples:
        inject_attrs: set[str] = set()
        extract_attrs: set[str] = set()
        for pat in INJECT_PATTERNS:
            for m in pat.finditer(body):
                if m.group(0).startswith("extract_tags"):
                    inject_attrs.add("tags")
                    inject_attrs.add("tags_all")
                else:
                    inject_attrs.add(m.group(1))
        if "extract_region" in body:
            inject_attrs.add("region")
        for ref in _TF_MODEL_REF_RE.finditer(body):
            fields = tf_model_index.get(ref.group(1))
            if fields:
                inject_attrs.update(fields)
        for pat in EXTRACT_PATTERNS:
            for m in pat.finditer(body):
                extract_attrs.add(m.group(1))
        # The trailing positional string literal of macro args like
        # `impl_bucket_subresource_converter!(..., "status")` names a
        # TF attribute the converter emits during extract. The
        # standard `"key":` / `.insert("key", ...)` patterns can't see
        # it because the macro template ( not the call site ) is what
        # actually inserts that key into the JSON map.
        extract_attrs |= _macro_trailing_extract_attrs(body)
        # Macro templates with hard-coded attributes ( `bucket`, `name` )
        # that don't appear in the call-site args. The macro emits
        # these on every invocation regardless of arguments.
        extract_attrs |= _MACRO_ALWAYS_EXTRACT_ATTRS.get(macro_name, set())
        inject_attrs |= _MACRO_ALWAYS_INJECT_ATTRS.get(macro_name, set())
        inject_attrs.discard("resource_type")
        extract_attrs.discard("resource_type")
        # `aws_*` literal that named the resource type is part of the
        # macro args; strip it so it doesn't leak into extract_attrs.
        extract_attrs.discard(rt)
        results.append({
            "resource_type": rt,
            "inject_attrs": sorted(inject_attrs),
            "extract_attrs": sorted(extract_attrs),
            "depends_on": [],
            "source_file": str(path.relative_to(ROOT)),
        })

    return results


def _find_method_body(source: str, resource_type: str, method_name: str) -> str | None:
    """Find the body of a method associated with a specific resource type.

    Uses heuristic: finds the struct impl block that contains
    `resource_type() -> "resource_type"`, then finds the method body.
    """
    # Find the struct name that returns this resource_type
    # Pattern: impl SomeConverter { ... fn resource_type ... "resource_type" ... }
    # Then find: impl SomeConverter { ... fn do_inject ... }

    # First, find the struct name from the TerraformResourceConverter impl
    # We look for the resource_type() method that returns *exactly* this type,
    # not just a substring match (which would false-positive on depends_on_types).
    rt_method_pattern = re.compile(
        r'impl\s+TerraformResourceConverter\s+for\s+(\w+)\s*\{[^}]*'
        r'fn\s+resource_type\s*\([^)]*\)[^{]*\{\s*"' + re.escape(resource_type) + r'"\s*\}',
        re.DOTALL,
    )
    struct_name = None
    rt_match = rt_method_pattern.search(source)
    if rt_match:
        struct_name = rt_match.group(1)

    if not struct_name:
        return None

    # Now find the impl block with do_inject/do_extract for this struct
    impl_pattern = re.compile(
        rf'impl\s+{re.escape(struct_name)}\s*\{{',
    )
    for m in impl_pattern.finditer(source):
        block_start = m.start()
        depth = 0
        for i in range(m.end() - 1, len(source)):
            if source[i] == '{':
                depth += 1
            elif source[i] == '}':
                depth -= 1
                if depth == 0:
                    block = source[block_start:i + 1]
                    # Find the method within this block
                    method_pat = re.compile(
                        rf'async\s+fn\s+{re.escape(method_name)}\s*\(',
                    )
                    method_match = method_pat.search(block)
                    if method_match:
                        # Find method body (from the opening { after fn signature)
                        fn_start = method_match.start()
                        fn_depth = 0
                        fn_body_start = None
                        for j in range(fn_start, len(block)):
                            if block[j] == '{':
                                fn_depth += 1
                                if fn_body_start is None:
                                    fn_body_start = j
                            elif block[j] == '}':
                                fn_depth -= 1
                                if fn_depth == 0 and fn_body_start is not None:
                                    return block[fn_body_start:j + 1]
                    break

    return None


# ---------------------------------------------------------------------------
# Report generation
# ---------------------------------------------------------------------------

def generate_report(
    converters: list[dict],
    tf_schemas: dict,
    output_path: Path,
) -> None:
    """Generate the Markdown coverage report."""
    lines = []
    lines.append("# Terraform Converter Field Coverage Report")
    lines.append("")
    lines.append(f"Generated: {date.today()}")
    lines.append("")

    # Group converters by source file (service)
    by_service: dict[str, list[dict]] = defaultdict(list)
    for c in converters:
        service = Path(c["source_file"]).stem
        by_service[service].append(c)

    # Summary table
    lines.append("## Summary")
    lines.append("")
    lines.append("| Resource Type | Inject | Extract | TF Schema | Inject% | Extract% | Rating |")
    lines.append("|---|---|---|---|---|---|---|")

    total_inject_covered = 0
    total_extract_covered = 0
    total_tf_attrs = 0
    ratings = {"excellent": 0, "good": 0, "fair": 0, "poor": 0, "n/a": 0}

    detail_sections = []

    for service in sorted(by_service.keys()):
        for c in sorted(by_service[service], key=lambda x: x["resource_type"]):
            rt = c["resource_type"]
            inject = set(c["inject_attrs"])
            extract = set(c["extract_attrs"])

            # Get TF schema attributes
            tf_attrs = set()
            if rt in tf_schemas:
                tf_attrs = collect_schema_attributes(tf_schemas[rt]["block"])

            # Compute coverage against TF schema
            tf_count = len(tf_attrs) if tf_attrs else 0

            # For coverage calculation, compare inject/extract against TF schema attrs
            # (excluding computed-only attrs that can't be injected)
            inject_relevant = inject & tf_attrs if tf_attrs else inject
            extract_relevant = extract & tf_attrs if tf_attrs else extract

            inject_pct = (len(inject_relevant) / tf_count * 100) if tf_count > 0 else 0
            extract_pct = (len(extract_relevant) / tf_count * 100) if tf_count > 0 else 0

            if tf_count == 0:
                rating = "n/a"
            elif inject_pct >= 60 and extract_pct >= 50:
                rating = "excellent"
            elif inject_pct >= 40 or extract_pct >= 30:
                rating = "good"
            elif inject_pct >= 20 or extract_pct >= 15:
                rating = "fair"
            else:
                rating = "poor"

            ratings[rating] += 1
            total_inject_covered += len(inject_relevant)
            total_extract_covered += len(extract_relevant)
            total_tf_attrs += tf_count

            emoji = {"excellent": "+", "good": "~", "fair": "-", "poor": "!", "n/a": "?"}[rating]

            lines.append(
                f"| `{rt}` | {len(inject)} | {len(extract)} "
                f"| {tf_count} | {inject_pct:.0f}% | {extract_pct:.0f}% "
                f"| {rating} [{emoji}] |"
            )

            # Build detail section
            detail = []
            detail.append(f"### `{rt}`")
            detail.append("")
            detail.append(f"**Source:** `{c['source_file']}`")
            detail.append("")

            if c["depends_on"]:
                detail.append(f"**Depends on:** {', '.join(f'`{d}`' for d in c['depends_on'])}")
                detail.append("")

            detail.append(f"**Inject attributes** ({len(inject)}): {', '.join(f'`{a}`' for a in sorted(inject)) or '(none)'}")
            detail.append("")
            detail.append(f"**Extract attributes** ({len(extract)}): {', '.join(f'`{a}`' for a in sorted(extract)) or '(none)'}")
            detail.append("")

            if tf_attrs:
                missing_inject = sorted(tf_attrs - inject)
                missing_extract = sorted(tf_attrs - extract)
                if missing_inject:
                    detail.append(f"**Missing from inject** ({len(missing_inject)}): {', '.join(f'`{a}`' for a in missing_inject)}")
                    detail.append("")
                if missing_extract:
                    detail.append(f"**Missing from extract** ({len(missing_extract)}): {', '.join(f'`{a}`' for a in missing_extract)}")
                    detail.append("")
            else:
                detail.append("**Note:** Resource type not found in Terraform AWS provider schema.")
                detail.append("")

            detail_sections.append("\n".join(detail))

    lines.append("")

    # Overall stats
    total_converters = len(converters)
    lines.append("## Statistics")
    lines.append("")
    lines.append(f"- **Total converters:** {total_converters}")
    lines.append(f"- **Distinct resource types:** {len(set(c['resource_type'] for c in converters))}")
    lines.append(f"- **Overall inject coverage:** {total_inject_covered}/{total_tf_attrs} ({total_inject_covered / total_tf_attrs * 100:.1f}%)" if total_tf_attrs else "- **Overall inject coverage:** n/a")
    lines.append(f"- **Overall extract coverage:** {total_extract_covered}/{total_tf_attrs} ({total_extract_covered / total_tf_attrs * 100:.1f}%)" if total_tf_attrs else "- **Overall extract coverage:** n/a")
    lines.append("")
    lines.append("### Rating distribution")
    lines.append("")
    for r in ("excellent", "good", "fair", "poor", "n/a"):
        lines.append(f"- **{r}:** {ratings[r]}")
    lines.append("")

    lines.append("### Rating criteria")
    lines.append("")
    lines.append("- **excellent:** inject >= 60% AND extract >= 50% of TF schema attributes")
    lines.append("- **good:** inject >= 40% OR extract >= 30%")
    lines.append("- **fair:** inject >= 20% OR extract >= 15%")
    lines.append("- **poor:** below fair thresholds")
    lines.append("- **n/a:** resource type not found in TF provider schema")
    lines.append("")

    # Detailed sections
    lines.append("## Per-Resource Detail")
    lines.append("")
    for section in detail_sections:
        lines.append(section)

    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text("\n".join(lines) + "\n")
    print(f"Report written to {output_path}")
    print(f"  {total_converters} converters analysed")
    print(f"  Ratings: {ratings}")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--output",
        type=Path,
        default=ROOT / ".agents" / "docs" / "TERRAFORM_CONVERTER_COVERAGE.md",
        help="Output path for the report",
    )
    parser.add_argument(
        "--schema-cache",
        type=Path,
        default=None,
        help="Path to cached aws_provider_schema.json",
    )
    args = parser.parse_args()

    print("Loading Terraform AWS provider schema...")
    tf_schemas = get_terraform_schema(cache_path=args.schema_cache)
    print(f"  {len(tf_schemas)} resource types in schema")

    print("Indexing generated TfModel structs...")
    tf_model_index = index_generated_tf_models()
    print(f"  Found {len(tf_model_index)} TfModel structs in {GENERATED_MODELS_DIR.name}")

    print("Analysing converter source files...")
    converters = []
    for rs_file in sorted(CONVERTERS_DIR.glob("*.rs")):
        if rs_file.name == "mod.rs":
            continue
        parsed = parse_converter_file(rs_file, tf_model_index=tf_model_index)
        converters.extend(parsed)
    print(f"  Found {len(converters)} converters in {len(list(CONVERTERS_DIR.glob('*.rs'))) - 1} files")

    generate_report(converters, tf_schemas, args.output)


if __name__ == "__main__":
    main()
