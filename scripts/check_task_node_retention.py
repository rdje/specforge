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

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys

ROOTS = ("docs/tasks", "docs/task-catalog", "docs/archive")
REMOVALS = os.path.join("doctrine", "task_nodes", "removals.jsonl")
NODE_ID = re.compile(r"^- ID: `([^`]+)`", re.M)
REGISTRY_FIELDS = {
    "record_type",
    "schema_version",
    "max_records",
    "max_record_bytes",
    "expected_removals",
}
REMOVAL_FIELDS = {"record_type", "schema_version", "id", "owning_leaf", "date", "reason"}
EXPECTED_SELF_TEST_CASES = 6


def repo_root() -> str:
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def ids_in_text(text: str) -> set[str]:
    return set(NODE_ID.findall(text))


def ids_at_head(root: str) -> set[str] | None:
    """Every node id `HEAD` carries, or `None` when there is no `HEAD` to compare against."""
    head = subprocess.run(
        ["git", "rev-parse", "--verify", "HEAD"],
        cwd=root,
        capture_output=True,
        text=True,
    )
    if head.returncode != 0:
        return None
    found = subprocess.run(
        ["git", "grep", "-h", "-oE", r"^- ID: `[^`]*`", "HEAD", "--", *ROOTS],
        cwd=root,
        capture_output=True,
        text=True,
    )
    return ids_in_text(found.stdout)


def ids_in_worktree(root: str) -> set[str]:
    ids: set[str] = set()
    for top in ROOTS:
        for dirpath, _, filenames in os.walk(os.path.join(root, top)):
            for name in filenames:
                if not name.endswith(".md"):
                    continue
                with open(os.path.join(dirpath, name), "r", encoding="utf-8") as handle:
                    ids |= ids_in_text(handle.read())
    return ids


def load_removals(root: str) -> tuple[dict, dict[str, dict], list[str]]:
    """`(registry, {id: record}, breaches)` — unknown fields and shape errors fail closed."""
    path = os.path.join(root, REMOVALS)
    breaches: list[str] = []
    registry: dict = {}
    declared: dict[str, dict] = {}
    if not os.path.isfile(path):
        return registry, declared, [f"{REMOVALS} is missing"]
    with open(path, "r", encoding="utf-8") as handle:
        for number, line in enumerate(handle, start=1):
            if not line.strip():
                continue
            if len(line.encode("utf-8")) > 4096:
                breaches.append(f"{REMOVALS}:{number} exceeds the declared record byte cap")
                continue
            try:
                record = json.loads(line)
            except json.JSONDecodeError as error:
                breaches.append(f"{REMOVALS}:{number} is not valid JSON: {error}")
                continue
            kind = record.get("record_type")
            if kind == "registry":
                unknown = set(record) - REGISTRY_FIELDS
                if unknown:
                    breaches.append(f"{REMOVALS}:{number} registry has unknown field(s): {sorted(unknown)}")
                registry = record
            elif kind == "removal":
                unknown = set(record) - REMOVAL_FIELDS
                if unknown:
                    breaches.append(f"{REMOVALS}:{number} removal has unknown field(s): {sorted(unknown)}")
                missing = REMOVAL_FIELDS - set(record)
                if missing:
                    breaches.append(f"{REMOVALS}:{number} removal is missing: {sorted(missing)}")
                    continue
                declared[record["id"]] = record
            else:
                breaches.append(f"{REMOVALS}:{number} has unknown record_type {kind!r}")
    expected = registry.get("expected_removals")
    if expected is None:
        breaches.append(f"{REMOVALS} registry declares no expected_removals")
    elif expected != len(declared):
        breaches.append(
            f"{REMOVALS} declares expected_removals {expected} but carries {len(declared)} — "
            "a removal was added or dropped without moving the declared total"
        )
    return registry, declared, breaches


def superseded_by_descendant(node: str, present: set[str]) -> list[str]:
    prefix = f"{node}."
    return sorted(other for other in present if other.startswith(prefix))


def audit(head: set[str] | None, present: set[str], declared: dict[str, dict]) -> list[str]:
    """Every `HEAD` id that is gone and neither split nor declared."""
    if head is None:
        return []
    breaches = []
    for node in sorted(head - present):
        if superseded_by_descendant(node, present):
            continue
        if node in declared:
            continue
        breaches.append(node)
    return breaches


def run_check(root: str) -> int:
    _, declared, breaches = load_removals(root)
    head = ids_at_head(root)
    present = ids_in_worktree(root)
    lost = audit(head, present, declared)
    for node in lost:
        breaches.append(
            f"task node `{node}` is in HEAD and in no tracked task surface now. A node may be renamed "
            "into descendants or retired through doctrine/task_nodes/removals.jsonl; it may not vanish."
        )
    for line in breaches:
        print(f"task-node-retention: {line}", file=sys.stderr)
    if breaches:
        return 1
    if head is None:
        print("task-node-retention: no HEAD to compare against; nothing to retain yet.")
        return 0
    print(
        f"task-node-retention: {len(present)} task nodes present, {len(head - present)} moved or "
        f"retired since HEAD, all accounted for ({len(declared)} declared removal(s))."
    )
    return 0


def run_report(root: str) -> int:
    registry, declared, _ = load_removals(root)
    head = ids_at_head(root)
    present = ids_in_worktree(root)
    json.dump(
        {
            "declared_removals": len(declared),
            "expected_removals": registry.get("expected_removals"),
            "head_nodes": None if head is None else len(head),
            "present_nodes": len(present),
            "vanished_unaccounted": audit(head, present, declared),
        },
        sys.stdout,
        indent=2,
        sort_keys=True,
    )
    sys.stdout.write("\n")
    return 0


SELF_TEST_CASES = (
    # The measured legitimate operation: `3c17ae5c` split .7.2 into .7.2.0 and .7.2.1.
    (
        "a-split-into-descendants-is-not-a-loss",
        lambda: audit({"A.7.2"}, {"A.7.2.0", "A.7.2.1"}, {}),
        [],
    ),
    # The measured accident: `ab2c6ee0` deleted eight nodes outright.
    (
        "an-outright-deletion-is-a-loss",
        lambda: audit({"A.3j", "A.3k.2c"}, {"A.3k.2j"}, {}),
        ["A.3j", "A.3k.2c"],
    ),
    # A declared retirement is admitted, and only for the id it names.
    (
        "a-declared-removal-is-admitted-only-for-its-own-id",
        lambda: audit({"A.1", "A.2"}, set(), {"A.1": {"id": "A.1"}}),
        ["A.2"],
    ),
    # A descendant of a DIFFERENT node does not cover this one.
    (
        "a-sibling-descendant-does-not-cover-a-lost-node",
        lambda: audit({"A.1"}, {"A.2.0"}, {}),
        ["A.1"],
    ),
    # Prefix matching is on the dotted boundary: `A.1` is not covered by `A.10`.
    (
        "a-longer-sibling-id-is-not-a-descendant",
        lambda: audit({"A.1"}, {"A.10"}, {}),
        ["A.1"],
    ),
    # No HEAD (a fresh repository) is not a breach.
    (
        "no-head-is-not-a-breach",
        lambda: audit(None, {"A.1"}, {}),
        [],
    ),
)


def self_test() -> int:
    passed = 0
    for case_id, run, expected in SELF_TEST_CASES:
        actual = run()
        if actual != expected:
            print(f"self-test FAIL {case_id}: expected {expected!r}, got {actual!r}", file=sys.stderr)
            return 1
        passed += 1
    if passed != EXPECTED_SELF_TEST_CASES:
        print(
            f"self-test FAIL: ran {passed} cases, expected {EXPECTED_SELF_TEST_CASES} — a case was "
            "removed; restore it or move the declared total deliberately.",
            file=sys.stderr,
        )
        return 1
    print(f"task-node-retention: {passed}/{EXPECTED_SELF_TEST_CASES} cases pass.")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="fail when a task node vanished")
    parser.add_argument("--report", action="store_true", help="emit the census as JSON")
    parser.add_argument("--self-test", action="store_true", help="run the discriminator cases only")
    args = parser.parse_args()
    root = repo_root()
    if args.self_test:
        return self_test()
    if args.report:
        return run_report(root)
    return run_check(root)


if __name__ == "__main__":
    sys.exit(main())
