#!/bin/bash
# PostCompact hook: re-read live-docs after context compaction.
# Outputs key project docs so Claude retains project awareness after compaction.
set -euo pipefail

HOOK_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(cd "$HOOK_DIR/../.." && pwd -P)}"
source "$PROJECT_DIR/scripts/project_data_env.sh"
specforge_activate_project_data "$PROJECT_DIR"

FSMGEN_SUB_DIR="$PROJECT_DIR/subs/fsmgen"
FSMGEN_MAIN_DIR="${SPECFORGE_FSMGEN_MAIN_DIR:-$PROJECT_DIR/../fsmgen}"
FSMGEN_MAIN_AVAILABLE=0
if [[ -d "$FSMGEN_MAIN_DIR" ]]; then
    FSMGEN_MAIN_DIR="$(cd "$FSMGEN_MAIN_DIR" && pwd -P)"
    if [[ "$(df -P "$FSMGEN_MAIN_DIR" | awk 'END { print $1 }')" == \
          "$(df -P "$PROJECT_DIR" | awk 'END { print $1 }')" ]]; then
        FSMGEN_MAIN_AVAILABLE=1
    else
        printf 'PostCompact: skipping off-volume optional FSMGen checkout: %s\n' \
            "$FSMGEN_MAIN_DIR" >&2
    fi
fi

echo "=== PostCompact: live-doc re-read ==="
echo ""

dump_file() {
    local label="$1"
    local path="$2"
    local max_lines="${3:-100}"
    if [ -f "$path" ]; then
        local lines
        lines=$(wc -l < "$path")
        echo "--- $label ($lines lines) ---"
        if [ "$lines" -gt "$max_lines" ]; then
            head -"$max_lines" "$path"
            echo "... (truncated, $((lines - max_lines)) more lines) ..."
        else
            cat "$path"
        fi
        echo ""
    fi
}

# === SpecForge project docs ===
for f in README.md COMMIT.md MEMORY.md SESSION_BOOTSTRAP.md; do
    dump_file "$f" "$PROJECT_DIR/$f"
done

dump_file "docs/TASK_TREE.md" "$PROJECT_DIR/docs/TASK_TREE.md" 999

# === subs/fsmgen live-docs ===
for f in README.md COMMIT.md MEMORY.md CHANGES.md SESSION_BOOTSTRAP.md; do
    dump_file "subs/fsmgen/$f" "$FSMGEN_SUB_DIR/$f"
done

for f in TASK_TREE.md TASK_TREE_README.md; do
    dump_file "subs/fsmgen/docs/$f" "$FSMGEN_SUB_DIR/docs/$f" 999
done

dump_file "subs/fsmgen/docs/book/src/SUMMARY.md" "$FSMGEN_SUB_DIR/docs/book/src/SUMMARY.md" 999

if [[ "$FSMGEN_MAIN_AVAILABLE" -eq 1 ]]; then
    # Optional sibling checkout: read-only, same-volume, and possibly ahead of the pinned submodule.
    dump_file "fsmgen/README.md" "$FSMGEN_MAIN_DIR/README.md"
    dump_file "fsmgen/COMMIT.md" "$FSMGEN_MAIN_DIR/COMMIT.md"

    for f in \
        docs/ISF_DOWNSTREAM_INTEGRATION_SPEC.md \
        docs/ISF_SPEC.md \
        docs/ISF_PUBLIC_INTERFACE_CONTRACT.md \
        docs/INTENT_SCHEDULING_BRAINSTORM.md \
        docs/ISF_LIBRARY_CATALOG.md \
        docs/DOWNSTREAM_ISSUE_REPORTING.md; do
        dump_file "fsmgen/$f" "$FSMGEN_MAIN_DIR/$f"
    done
fi

# mdBook chapters — subs/fsmgen (preferred)
for f in \
    docs/book/src/13-intent-scheduling.md \
    docs/book/src/13a-actor-interface.md \
    docs/book/src/13b-transactions.md \
    docs/book/src/13c-drive-blocks.md \
    docs/book/src/13d-control-flow.md \
    docs/book/src/13e-data-manipulation.md \
    docs/book/src/13f-composition.md \
    docs/book/src/13g-rules.md \
    docs/book/src/13h-lowering-reference.md \
    docs/book/src/13i-downstream-integration.md; do
    dump_file "subs/fsmgen/$f" "$FSMGEN_SUB_DIR/$f"
done

dump_file "subs/fsmgen/docs/book/src/SUMMARY.md" "$FSMGEN_SUB_DIR/docs/book/src/SUMMARY.md" 999

if [[ "$FSMGEN_MAIN_AVAILABLE" -eq 1 ]]; then
    for f in \
        docs/book/src/13-intent-scheduling.md \
        docs/book/src/13h-lowering-reference.md; do
        dump_file "fsmgen/$f" "$FSMGEN_MAIN_DIR/$f"
    done

    dump_file "fsmgen/docs/book/src/SUMMARY.md" "$FSMGEN_MAIN_DIR/docs/book/src/SUMMARY.md" 999
fi

echo "=== PostCompact: live-doc re-read complete ==="
