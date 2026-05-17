#!/usr/bin/env bash
# audit-state-fields.sh — surface cross-call invariant gaps in winterbaume service crates.
#
# Two heuristics, both mechanical greps over `src/state.rs` and `src/handlers.rs`:
#
#   A. Toggle-without-consumer
#      A `pub` field on the top-level *State struct that is written by Enable* /
#      Disable* / Modify* / Set* handlers but never read by Create* / Run* /
#      Allocate* / Attach* handlers. This is the EnableEbsEncryptionByDefault
#      bug class — the toggle has no consumer.
#
#   B. Counter reuse across allocation and association
#      A `self.counters.X` field that appears in a `format!()` ID expression
#      from two or more state methods. The AssociateAddress bug had the EIP
#      creation counter reused for association IDs, so re-association produced
#      duplicate eipassoc IDs.
#
# Usage:
#   .agents/bin/audit-state-fields.sh <crate-suffix>      # e.g. ec2, sqs, sns
#   .agents/bin/audit-state-fields.sh --all
#
# Output:
#   .agents-workspace/tmp/audit-<crate-suffix>.md
#
# Portable: uses grep + sed only ( no gawk-specific features ). False positives
# are acceptable — treat the report as a worklist for /write-tests, not a
# compile gate.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="$REPO_ROOT/.agents-workspace/tmp"
mkdir -p "$OUT_DIR"

# extract_state_fields <state.rs> <state-struct-name>
# Lists `pub <field>` names from the body of the named struct, until the first `^}`.
extract_state_fields() {
    local file="$1" name="$2"
    sed -n "/^pub struct ${name}\b/,/^}/p" "$file" \
        | sed -nE 's/^[[:space:]]+pub ([a-z_][a-z0-9_]*):.*$/\1/p'
}

# fns_writing_field <handlers.rs> <field>
# Lists handler function names whose body assigns `.field =`.
# Approximates body span by walking from `async fn handle_<name>` until the
# next sibling `async fn handle_` or end-of-file.
fns_writing_field() {
    local file="$1" field="$2"
    awk -v field="$field" '
        /^[[:space:]]+(async )?fn handle_[a-z_]+\(/ {
            # extract function name after "fn "
            line = $0
            sub(/.*fn /, "", line)
            sub(/\(.*/, "", line)
            current = line
            depth = 0
            in_fn = 1
        }
        in_fn {
            opens = gsub(/\{/, "{")
            closes = gsub(/\}/, "}")
            depth += opens - closes
        }
        in_fn && index($0, "." field " =") {
            print current
        }
        in_fn && index($0, "." field "=") {
            print current
        }
        in_fn && depth <= 0 && /^[[:space:]]*\}/ {
            in_fn = 0
        }
    ' "$file" | sort -u
}

# fns_reading_field <handlers.rs> <field>
# Lists handler function names whose body references `.field` other than as an assignment.
fns_reading_field() {
    local file="$1" field="$2"
    awk -v field="$field" '
        /^[[:space:]]+(async )?fn handle_[a-z_]+\(/ {
            line = $0
            sub(/.*fn /, "", line)
            sub(/\(.*/, "", line)
            current = line
            depth = 0
            in_fn = 1
        }
        in_fn {
            opens = gsub(/\{/, "{")
            closes = gsub(/\}/, "}")
            depth += opens - closes
        }
        in_fn && index($0, "." field) && !index($0, "." field " =") && !index($0, "." field "=") {
            print current
        }
        in_fn && depth <= 0 && /^[[:space:]]*\}/ {
            in_fn = 0
        }
    ' "$file" | sort -u
}

# counter_uses_with_fn <state.rs>
# Emits "<counter>\t<fn-name>" pairs for every line that references
# `self.counters.X`. State.rs uses such references both inside `format!`
# id-minting expressions and outside ( e.g. `self.counters.x += 1` ); we
# emit all of them and let later filtering decide.
counter_uses_with_fn() {
    local file="$1"
    awk '
        /^[[:space:]]*(pub )?(async )?fn [a-z_][a-z_0-9]*\(/ {
            line = $0
            sub(/.*fn /, "", line)
            sub(/\(.*/, "", line)
            current = line
        }
        {
            tmp = $0
            while (match(tmp, /self\.counters\.[a-z_][a-z0-9_]*/)) {
                token = substr(tmp, RSTART, RLENGTH)
                sub(/^self\.counters\./, "", token)
                print token "\t" current
                tmp = substr(tmp, RSTART + RLENGTH)
            }
        }
    ' "$file" | sort -u
}

# direct_counter_uses_with_fn <state.rs>
# Heuristic B fallback for crates that do not use the EC2-style `self.counters`
# substruct. Emits "<field>\t<fn-name>" pairs for every line that
#
#   - contains a `format!(` invocation ( typical for client-visible ID minting )
#     AND a `self.<field>` reference, where `<field>` is NOT one of the
#     well-known non-counter shapes ( see `is_known_noncounter` ).
#
# OR
#
#   - increments a `self.<field>` via `+= 1` / `= self.<field> + 1`. Counters
#     are almost always incremented as part of the ID-minting step, so the
#     intersection of "appears in format!" and "is incremented" is a strong
#     candidate even when the surface field-name pattern differs from EC2's
#     `counters.X` substruct.
#
# False positives are expected ( e.g. a `format!()` that interpolates a
# `self.name` rather than a counter ); the report is a worklist for
# /write-tests, not a compile gate.
direct_counter_uses_with_fn() {
    local file="$1"
    awk '
        function is_known_noncounter(name) {
            return (name == "counters" || name == "notifier" || \
                    name == "next_token" || name == "tags" || \
                    name == "state" || name == "config" || \
                    name == "metadata" || name == "items" || \
                    name == "name" || name == "arn" || name == "id")
        }
        /^[[:space:]]*(pub )?(async )?fn [a-z_][a-z_0-9]*\(/ {
            line = $0
            sub(/.*fn /, "", line)
            sub(/\(.*/, "", line)
            current = line
        }
        {
            # Emit on lines that look like ID-minting: a format! invocation
            # combined with at least one self.<field> reference.
            if (index($0, "format!(") > 0) {
                tmp = $0
                while (match(tmp, /self\.[a-z_][a-z0-9_]*/)) {
                    token = substr(tmp, RSTART, RLENGTH)
                    sub(/^self\./, "", token)
                    if (!is_known_noncounter(token) && token != "") {
                        print token "\t" current
                    }
                    tmp = substr(tmp, RSTART + RLENGTH)
                }
            }
            # Also emit on lines that increment a self.<field>; counters
            # almost always grow this way.
            if (match($0, /self\.[a-z_][a-z0-9_]*[[:space:]]*\+=[[:space:]]*1/) || \
                match($0, /self\.[a-z_][a-z0-9_]*[[:space:]]*=[[:space:]]*self\.[a-z_][a-z0-9_]*[[:space:]]*\+[[:space:]]*1/)) {
                if (match($0, /self\.[a-z_][a-z0-9_]*/)) {
                    token = substr($0, RSTART, RLENGTH)
                    sub(/^self\./, "", token)
                    if (!is_known_noncounter(token) && token != "") {
                        print token "\t" current
                    }
                }
            }
        }
    ' "$file" | sort -u
}

audit_crate() {
    local svc="$1"
    local crate_dir="$REPO_ROOT/crates/winterbaume-$svc"
    local state_rs="$crate_dir/src/state.rs"
    local handlers_rs="$crate_dir/src/handlers.rs"
    local report="$OUT_DIR/audit-$svc.md"

    if [[ ! -f "$state_rs" ]]; then
        echo "skip: $crate_dir/src/state.rs missing" >&2
        return 0
    fi
    if [[ ! -f "$handlers_rs" ]]; then
        echo "skip: $crate_dir/src/handlers.rs missing" >&2
        return 0
    fi

    {
        echo "# Cross-call invariant audit: winterbaume-$svc"
        echo
        echo "Generated by \`.agents/bin/audit-state-fields.sh\`. Heuristic worklist for \`/write-tests\` — false positives are expected. Each flagged item is a candidate for an invariant-style test, not a confirmed bug."
        echo
    } > "$report"

    # ---------- Heuristic A: toggle-without-consumer ----------

    local state_struct
    state_struct=$(grep -m1 -E '^pub struct [A-Z][A-Za-z0-9]*State[[:space:]]*\{' "$state_rs" \
        | sed -E 's/^pub struct ([A-Za-z0-9]+).*/\1/')

    {
        echo "## A. Toggle-without-consumer"
        echo
    } >> "$report"

    if [[ -z "$state_struct" ]]; then
        echo "_skipped — no top-level \`*State\` struct in \`src/state.rs\`._" >> "$report"
        echo >> "$report"
    else
        {
            echo "_State struct: \`$state_struct\`. Fields written by an Enable / Disable / Modify / Set handler with no Create / Run / Allocate / Attach reader. The EBS-encryption-by-default bug had this shape._"
            echo
        } >> "$report"

        local fields
        fields=$(extract_state_fields "$state_rs" "$state_struct")
        local any_flag=0

        local f
        while IFS= read -r f; do
            [[ -z "$f" ]] && continue
            case "$f" in
                counters|notifier|next_token|tags) continue ;;
            esac

            local writers
            writers=$(fns_writing_field "$handlers_rs" "$f")
            [[ -z "$writers" ]] && continue

            # Are any writers in the toggle-style family?
            local toggle_writers
            toggle_writers=$(echo "$writers" | grep -E '^handle_(enable|disable|modify|set)_' || true)
            [[ -z "$toggle_writers" ]] && continue

            # Are there any readers in the create-style family?
            local readers
            readers=$(fns_reading_field "$handlers_rs" "$f")
            local create_readers
            create_readers=$(echo "$readers" | grep -E '^handle_(create|run|allocate|attach|request)_' || true)

            if [[ -z "$create_readers" ]]; then
                any_flag=1
                {
                    echo "### \`$f\`"
                    echo
                    echo "Toggle writers:"
                    echo
                    echo '```'
                    echo "$toggle_writers"
                    echo '```'
                    echo
                    if [[ -n "$readers" ]]; then
                        echo "Other readers (none of which are Create / Run / Allocate / Attach):"
                        echo
                        echo '```'
                        echo "$readers"
                        echo '```'
                        echo
                    else
                        echo "_No readers found at all — the toggle has no consumer._"
                        echo
                    fi
                    echo "Verify whether a documented AWS contract requires Create / Run / Allocate / Attach to consult \`$f\` for default behaviour."
                    echo
                } >> "$report"
            fi
        done <<< "$fields"

        if [[ "$any_flag" -eq 0 ]]; then
            echo "_No toggle-without-consumer suspects found._" >> "$report"
            echo >> "$report"
        fi
    fi

    # ---------- Heuristic B: counter reuse ----------

    {
        echo "## B. Counter reuse across allocation and association"
        echo
        echo "_A \`self.counters.X\` referenced from two or more state methods is suspect when both methods mint client-visible IDs. The AssociateAddress bug had the EIP creation counter reused for association IDs, so re-association produced duplicate eipassoc-XXXX IDs._"
        echo
    } >> "$report"

    local counter_uses
    counter_uses=$(counter_uses_with_fn "$state_rs")
    local used_fallback=0

    if [[ -z "$counter_uses" ]]; then
        # No EC2-style `self.counters.*` substruct — fall back to the direct
        # `self.<field>` heuristic so non-EC2 crates that mint IDs through
        # ad-hoc counter fields (e.g. `self.next_user_id`) still get audited.
        counter_uses=$(direct_counter_uses_with_fn "$state_rs")
        used_fallback=1
    fi

    if [[ -z "$counter_uses" ]]; then
        echo "_No \`self.counters.*\` or direct ID-minting \`self.<counter>\` references found in state.rs._" >> "$report"
        echo >> "$report"
    else
        if [[ "$used_fallback" -eq 1 ]]; then
            {
                echo "_Note: no \`self.counters\` substruct in this crate. Using the direct \`self.<field>\` fallback heuristic — fields that appear in a \`format!()\` ID-minting expression or are incremented via \`+= 1\`. Higher false-positive rate than the EC2 path._"
                echo
            } >> "$report"
        fi
        # Group by counter name; flag groups with more than one distinct fn.
        local found_dup=0
        local current_counter=""
        local current_fns=""
        # Use a sentinel line so the loop flushes the last group cleanly.
        local input
        input="$counter_uses"$'\n''__SENTINEL__\t__'

        while IFS=$'\t' read -r counter fn_name; do
            if [[ "$counter" != "$current_counter" ]]; then
                # Flush previous group
                if [[ -n "$current_counter" ]]; then
                    local fn_count
                    fn_count=$(echo "$current_fns" | sort -u | grep -c .)
                    if [[ "$fn_count" -gt 1 ]]; then
                        found_dup=1
                        local field_prefix
                        if [[ "$used_fallback" -eq 1 ]]; then
                            field_prefix="self.$current_counter"
                        else
                            field_prefix="self.counters.$current_counter"
                        fi
                        {
                            echo "### \`$field_prefix\`"
                            echo
                            echo "Referenced from:"
                            echo
                            echo "$current_fns" | sort -u | while read -r fn; do
                                echo "- \`$fn\`"
                            done
                            echo
                            echo "If both functions mint IDs that are exposed back to clients, they must use distinct counters. AWS does not reuse association IDs across reassociations."
                            echo
                        } >> "$report"
                    fi
                fi
                current_counter="$counter"
                current_fns="$fn_name"
            else
                current_fns="$current_fns"$'\n'"$fn_name"
            fi
        done <<< "$input"

        if [[ "$found_dup" -eq 0 ]]; then
            echo "_No counter is shared across multiple state methods._" >> "$report"
            echo >> "$report"
        fi
    fi

    {
        echo "---"
        echo
        echo "Each flagged item should become a row in the cross-call invariant inventory ( see \`/write-tests\` Step 1d )."
    } >> "$report"

    echo "$report"
}

if [[ $# -lt 1 ]]; then
    echo "usage: $0 <crate-suffix> | --all" >&2
    exit 64
fi

if [[ "$1" == "--all" ]]; then
    for d in "$REPO_ROOT"/crates/winterbaume-*/src/state.rs; do
        svc=$(basename "$(dirname "$(dirname "$d")")" | sed 's/^winterbaume-//')
        [[ "$svc" == "core" || "$svc" == "terraform" ]] && continue
        audit_crate "$svc"
    done
else
    audit_crate "$1"
fi
