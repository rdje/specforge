#!/usr/bin/env python3
"""TASK-NODE-RETENTION.0 — a task node may be renamed, split or deliberately retired; it may not vanish.

The task-trees under `docs/tasks/` are layer B of `MEMORY_ARCHITECTURE.md`: the project's work memory,
and the only record of why a closed leaf was closed the way it was. Every other layer is guarded — the
resume pointer by `memory-arch`, the fact cards by `knowledge-map`, the catalog INDEX by
`check_task_tree_catalog.pl`. The CONTENTS of a tree file were guarded by nothing, so a bad edit could
delete a node and every gate would still report green.

That is not hypothetical. `ab2c6ee0` replaced one node by splicing from its `- ID:` line to the next
section header and deleted the eight nodes in between — four closed leaves' full records and three open
leaves the container's ordering depends on — while all fourteen registered doctrines passed. This check
exists because a green build must never hide that.

**The rule has to admit the legitimate operation, and there is exactly one.** Measured over the last 200
revisions, a node id vanished from every tracked task surface in **two** of them:

  * `3c17ae5c` — `CLAIM-VERIFICATION-ADOPTION.7.2` split into `.7.2.0` and `.7.2.1`. Correct: the parent
    stopped being a leaf and its children took over.
  * `ab2c6ee0` — the accident above.

So a vanished id is admitted when a DESCENDANT id appeared in its place (`X` → `X.0`, `X.1`, …), and
otherwise only when `doctrine/task_nodes/removals.jsonl` names it with an owning leaf and a reason. The
discriminator was checked against both real instances before it was written: it passes the split and
fails the deletion, which is what makes it a control rather than a tripwire.

Read-only and deterministic: reads `HEAD` through Git and the working tree, writes nothing.

Usage:
    python3 scripts/check_task_node_retention.py --check
    python3 scripts/check_task_node_retention.py --report
    python3 scripts/check_task_node_retention.py --self-test
"""

# RUSTDOC-AND-XTASK.13.b — native since that leaf in `cargo xtask check task-node-retention`
# (xtask/src/checks/task_node_retention.rs). This shim keeps the doctrine's registered path and its argparse front
# end, so help, abbreviations such as `--self` and refusals are exactly as they were; it passes the native check one
# mode and runs it from the repository root, where the `cargo xtask` alias resolves.

from __future__ import annotations

import argparse
import os
import sys


def repo_root() -> str:
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="fail when a task node vanished")
    parser.add_argument("--report", action="store_true", help="emit the census as JSON")
    parser.add_argument("--self-test", action="store_true", help="run the discriminator cases only")
    args = parser.parse_args()
    mode = "--self-test" if args.self_test else "--report" if args.report else "--check"
    os.chdir(repo_root())
    os.execvp("cargo", ["cargo", "xtask", "check", "task-node-retention", mode])
    return 1


if __name__ == "__main__":
    sys.exit(main())
