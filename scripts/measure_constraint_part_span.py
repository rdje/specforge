#!/usr/bin/env python3
"""Census which SPAN of its statement each published signal constraint read its parts from.

`EXTRACTION-QUALITY-GAUGE.3k`.

A `SignalConstraintRecord` is assembled from several parts — subject, kind, bound value,
condition, negation — and each part is read from some span of the statement that produced the
record. When two parts are read from DIFFERENT spans, the published record is a composite of
clauses the document never joined. `.3i` proved that for the negation (`The DV operand must be 1
for IODIR` was published NEGATED because a later sentence said `must not`); this census measures
the same question for the KIND.

**The population of a kind-span change is the population of the CLASSIFIER, not of the record
table.** `classify_signal_constraint_kind` has exactly two callers, and only one of them reads the
whole statement:

  * `extract_signal_constraints`                    (`sigcon_*`)      — reads the WHOLE statement
  * `extract_signal_description_row_constraints`    (`row_sigcon_*`)  — reads one clause already

The other two producers of the same record type never reach the classifier at all:

  * `extract_dynamic_signal_constraints` (`dyn_sigcon_*`) types the record from its VALUE BINDER
    (`extract_discovered_state_value_from_text` / `logic_level_binding_kind_from_text`);
  * `crates/specforge/src/ir/constraint_extract_llm.rs` (`llm_sigcon_*`) takes the kind the model
    NAMED and parses it (`parse_kind`).

Comparing "classify over the clause" with "classify over the whole text" for those two strata
therefore measures a function that does not run on them. This census reports every stratum, but it
reports them SEPARATELY and labels the two that are out of the classifier's reach, so the number
can never again be read as the population a classifier change would move.

`--check` fails closed when that call-site topology changes: if `classify_signal_constraint_kind`
gains, loses, or moves a caller, the stratification above is stale and the census must be
re-derived rather than re-run.

This census classifies a POPULATION by mirroring the extractor's rule. It is NOT a check
on the Rust implementation: mirror and original agreeing carries no information
(`CLAIM_VERIFICATION.md` section 2). The implementation's evidence is its own controls plus the
rebuilt-artifact diff.

Read-only and deterministic: no network, no clock, no randomness, no write, no rebuild.
Boundary: the persisted `generated/evidence_ir/*/evidence_ir.json` corpus plus the two call sites
in `crates/specforge/src/ir/evidence.rs`.

Usage:
    python3 scripts/measure_constraint_part_span.py
    python3 scripts/measure_constraint_part_span.py --json
    python3 scripts/measure_constraint_part_span.py --check
    python3 scripts/measure_constraint_part_span.py --self-test
"""

# RUSTDOC-AND-XTASK.14.b — native since that leaf in `cargo xtask check constraint-part-span --measure`
# (xtask/src/checks/constraint_part_span.rs, where the mirrors now live). This shim keeps the census's registered
# path and its argparse front end, so help, abbreviations such as `--self` and refusals are as they were; it passes
# the native census the flags that were set and runs it from the repository root, where the `cargo xtask` alias
# resolves.

from __future__ import annotations

import argparse
import os
import sys


def repo_root() -> str:
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    parser.add_argument(
        "--check",
        action="store_true",
        help="fail when the kind-classifier call-site topology no longer matches this census",
    )
    parser.add_argument("--self-test", action="store_true", help="run the mirror cases only")
    args = parser.parse_args()
    modes = (("--json", args.json), ("--check", args.check), ("--self-test", args.self_test))
    os.chdir(repo_root())
    os.execvp(
        "cargo",
        ["cargo", "xtask", "check", "constraint-part-span", "--measure", *(flag for flag, on in modes if on)],
    )
    return 1


if __name__ == "__main__":
    sys.exit(main())
