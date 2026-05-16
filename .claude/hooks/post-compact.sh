#!/bin/bash
# PostCompact hook: re-read live-docs after context compaction.
# Outputs key project docs so Claude retains project awareness after compaction.
set -euo pipefail

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-/Users/richarddje/Documents/github/specforge}"
FSMGEN_SUB_DIR="$PROJECT_DIR/subs/fsmgen"
FSMGEN_MAIN_DIR="/Users/richarddje/Documents/github/fsmgen"

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

# === Main fsmgen repo ISF docs (the authoritative downstream integration spec) ===
dump_file "fsmgen/README.md" "$FSMGEN_MAIN_DIR/README.md"
dump_file "fsmgen/COMMIT.md" "$FSMGEN_MAIN_DIR/COMMIT.md"

# Core ISF integration spec and its key references
for f in \
    docs/ISF_DOWNSTREAM_INTEGRATION_SPEC.md \
    docs/ISF_SPEC.md \
    docs/ISF_PUBLIC_INTERFACE_CONTRACT.md \
    docs/INTENT_SCHEDULING_BRAINSTORM.md \
    docs/ISF_LIBRARY_CATALOG.md \
    docs/DOWNSTREAM_ISSUE_REPORTING.md; do
    dump_file "fsmgen/$f" "$FSMGEN_MAIN_DIR/$f"
done

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

# mdBook chapters — main fsmgen repo (may carry ahead-of-submodule chapters)
for f in \
    docs/book/src/13-intent-scheduling.md \
    docs/book/src/13h-lowering-reference.md; do
    dump_file "fsmgen/$f" "$FSMGEN_MAIN_DIR/$f"
done

dump_file "fsmgen/docs/book/src/SUMMARY.md" "$FSMGEN_MAIN_DIR/docs/book/src/SUMMARY.md" 999

echo "=== PostCompact: live-doc re-read complete ==="
