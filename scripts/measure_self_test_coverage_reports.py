#!/usr/bin/env python3
"""Census how each registered doctrine check reports its own self-test coverage (read-only).

`PRODUCTION-GRAPH-CENSUS-PIN.2`.

`.0` found a gate-tier check that derived a repository-wide census, printed it, and compared none of
it — so the pins drifted across 29 commits with every gate green. `.1` fixed that one and
`DOCTRINE_ENFORCEMENT.md` section 3 named the shape: **a census that reports is not a check**. This
census sweeps the registry for the same shape one level up — not in what the checks measure about the
repository, but in what they report about THEMSELVES.

Nearly every check prints a reassuring self-test line. This classifies how that number is produced:

  * `derived`     `$passed/$total` — two independently-computed values. Delete a self-test case and
                  they differ, so the check fails. This is the one that works.
  * `TAUTOLOGY`   `$passed/$passed` — the numerator IS the denominator. The line can never be anything
                  but `N/N`; deleting a case silently reduces both.
  * `DECORATIVE`  a hardcoded literal in the message (`"self-test 15/15 passed."`). The number is not
                  derived from anything at all, so it keeps asserting the same figure whatever the
                  suite contains.
  * `BARE`        a running counter with no declared total to compare against.

**The honest limit, which this census does not overstate:** in every class a self-test that FAILS
still fails the check — these scripts `die` on a failing case. What is unguarded is COVERAGE. A case
silently removed, or one never added, is invisible; and for the decorative class the printed number
cannot be checked against the suite at all, so whether it is currently right is undecidable from the
script's own output.

Read-only and deterministic: no network, no clock, no randomness, no write. It reads the check scripts
as text and classifies their report lines; it does not execute them.

Usage:
    python3 scripts/measure_self_test_coverage_reports.py
    python3 scripts/measure_self_test_coverage_reports.py --json
"""

from __future__ import annotations

import argparse
import collections
import glob
import json
import os
import re
import sys

TAUTOLOGY = re.compile(r"\$\{?([A-Za-z_]+)\}?/\$\{?\1\}?(?![A-Za-z_])")
DERIVED = re.compile(r"\$\{?([A-Za-z_]+)\}?/\$\{?([A-Za-z_]+)\}?(?![A-Za-z_])")
LITERAL = re.compile(r"(?:self-test|passed:?)\s+(\d+)/(\d+)")
BARE = re.compile(r"all \$\{?[A-Za-z_]+\}? [^\"\n]*(?:checks|tests|cases) pass")
REPORT_LINE = re.compile(r"\b(?:print|printf|echo|note)\b")
REPORT_SUBJECT = re.compile(r"(?:pass|self-test|cases|checks|tests)")

GLOBS = (
    "scripts/check_*.pl",
    "scripts/check_*.sh",
    "scripts/check_*.py",
    "scripts/test_*.pl",
    "knowledge-map/scripts/check_*.sh",
)
ORDER = {"TAUTOLOGY": 0, "DECORATIVE": 1, "BARE": 2, "derived": 3}


def repo_root() -> str:
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def classify(text: str) -> tuple[str, str] | None:
    """Classify a script by how its own report line produces its coverage number."""
    reported = "\n".join(
        line
        for line in text.splitlines()
        if REPORT_LINE.search(line) and REPORT_SUBJECT.search(line)
    )
    match = TAUTOLOGY.search(reported)
    if match:
        name = match.group(1)
        return ("TAUTOLOGY", f"${name}/${name} — the numerator is the denominator")
    match = LITERAL.search(reported)
    if match and match.group(1) == match.group(2):
        return ("DECORATIVE", f"{match.group(1)}/{match.group(2)} hardcoded in the message")
    match = DERIVED.search(reported)
    if match and match.group(1) != match.group(2):
        return ("derived", f"${match.group(1)}/${match.group(2)}")
    if BARE.search(reported):
        return ("BARE", "a running counter with no declared total")
    return None


def census(root: str) -> list[dict]:
    rows = []
    seen = set()
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
            verdict = classify(text)
            if verdict is None:
                continue
            rows.append(
                {
                    "check": os.path.relpath(path, root),
                    "class": verdict[0],
                    "detail": verdict[1],
                }
            )
    rows.sort(key=lambda row: (ORDER[row["class"]], row["check"]))
    return rows


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    args = parser.parse_args()
    rows = census(repo_root())
    counts = collections.Counter(row["class"] for row in rows)

    if args.json:
        json.dump(
            {"by_class": dict(sorted(counts.items())), "checks": rows},
            sys.stdout,
            indent=2,
            sort_keys=True,
        )
        sys.stdout.write("\n")
        return 0

    print("=== how each registered check reports its own self-test coverage (read-only) ===")
    print(f"{'CHECK':46s} {'CLASS':11s} HOW THE NUMBER IS PRODUCED")
    for row in rows:
        print(f"{os.path.basename(row['check']):46s} {row['class']:11s} {row['detail']}")
    print()
    unguarded = sum(count for cls, count in counts.items() if cls != "derived")
    for cls in ("TAUTOLOGY", "DECORATIVE", "BARE", "derived"):
        if counts.get(cls):
            print(f"    {cls:11s} {counts[cls]}")
    print(f"    checks whose own coverage is unguarded: {unguarded} of {len(rows)}")
    print()
    print("A failing self-test still fails every one of these checks. What is unguarded is COVERAGE:")
    print("a case silently removed, or never added, changes the reported number and nothing compares it.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
