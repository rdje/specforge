#!/usr/bin/env python3
"""Report how each doctrine check counts its own self-tests — EVIDENCE, not a verdict (read-only).

`PRODUCTION-GRAPH-CENSUS-PIN.2a`.

**This producer deliberately does not classify.** Its first version did, and it was wrong twice:

1. It read only the PRINT line, so a script printing the literal `13/13` was called unguarded even
   though it compares `$passed != 13` forty lines earlier. Three of five were misclassified.
2. The repair attempt — "find a comparison involving a counter anywhere" — produced false positives
   (`$lines != $parts`, `$count != 1`) and a false negative on a case already verified by hand.

The property is not regex-decidable, and a producer that guesses it publishes exactly the defect this
tree is about: a number nothing verified. So this prints the EVIDENCE a human needs — every self-test
report line, and every comparison involving a counted variable — and the adjudication lives in the
task leaf, per `.2`'s own acceptance criterion: *the census is the deliverable; a count with no
adjudication is not.*

The question to adjudicate, per check, is narrow:

    Does DELETING one self-test case make this check fail?

It does only when the expected total is declared INDEPENDENTLY of the suite — a literal. The
`$passed/$total` form does NOT qualify when `$total` is incremented in the same loop as `$passed`
(`my ($passed, $total) = (0, 0); for my $case (@cases) { $total++; ... }`): deleting a case drops both
and the ratio stays `N/N`. That form detects a FAILING case, which is a different and weaker property.

Read-only and deterministic: reads the check scripts as text, executes nothing.

Usage:
    python3 scripts/measure_self_test_coverage_reports.py
    python3 scripts/measure_self_test_coverage_reports.py --json
"""

from __future__ import annotations

import argparse
import glob
import json
import os
import re
import sys

GLOBS = (
    "scripts/check_*.pl",
    "scripts/check_*.sh",
    "scripts/check_*.py",
    "scripts/test_*.pl",
    "knowledge-map/scripts/check_*.sh",
)
REPORT = re.compile(
    r"^\s*(?:print|printf|echo|note|fail_note)\b.*(?:self-test|cases pass|checks pass|tests pass)",
    re.IGNORECASE,
)
# Any comparison that could be a coverage guard. Deliberately broad: the adjudicator needs the
# candidates, not this script's opinion of them.
COMPARISON = re.compile(
    r"[^\n]*(?:!=\s*\d+|-ne\s+\d+|!=\s*\$\w+|expected\s+\d+\s+(?:case|check|test))[^\n]*"
)
# Counter increments in perl (`$passed++`) and shell (`passed=$((passed + 1))`). The shell form
# needs the doubled parenthesis: an earlier version omitted it and silently dropped
# check_chain_currency.sh's real guard from the evidence — the same failure this producer exists
# to stop reporting as a clean result.
COUNTER = re.compile(r"(\w+)\s*(?:\+\+|=\s*\$?\(*\s*\$?\w+\s*\+\s*1)")


def repo_root() -> str:
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def evidence(root: str) -> list[dict]:
    rows = []
    seen: set[str] = set()
    for pattern in GLOBS:
        for path in sorted(glob.glob(os.path.join(root, pattern))):
            if path in seen:
                continue
            seen.add(path)
            try:
                with open(path, "r", encoding="utf-8") as handle:
                    text = handle.read()
            except OSError:
                continue
            reports = [line.strip() for line in text.splitlines() if REPORT.match(line)]
            if not reports:
                continue
            counters = sorted({m.group(1) for m in COUNTER.finditer(text)})
            comparisons = [
                line.strip()
                for line in text.splitlines()
                if COMPARISON.search(line) and any(c in line for c in counters)
            ]
            rows.append(
                {
                    "check": os.path.relpath(path, root),
                    "report_lines": reports,
                    "counter_comparisons": comparisons,
                }
            )
    return rows


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--json", action="store_true", help="emit the evidence as JSON")
    args = parser.parse_args()
    rows = evidence(repo_root())

    if args.json:
        json.dump({"checks": rows}, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return 0

    print("=== self-test report lines and their candidate coverage guards (read-only evidence) ===")
    print("Adjudicate per check: does DELETING one self-test case make this check fail?")
    print("It does only if the expected total is a LITERAL, declared independently of the suite.")
    for row in rows:
        print()
        print(f"--- {row['check']}")
        for line in row["report_lines"]:
            print(f"    reports : {line[:150]}")
        if row["counter_comparisons"]:
            for line in row["counter_comparisons"][:4]:
                print(f"    compares: {line[:150]}")
        else:
            print("    compares: (no comparison involving a counted variable)")
    print()
    print(f"{len(rows)} checks report a self-test count. The verdict for each is recorded in")
    print("docs/tasks/PRODUCTION-GRAPH-CENSUS-PIN.md, not inferred here.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
