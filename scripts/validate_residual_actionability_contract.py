#!/usr/bin/env python3
"""Validate the frozen SPEC-TO-INTENT-ALIGNMENT.8a required-residual contract.

This checker owns the measurement and record design, not extraction behavior. It executes the
required-residual rule over a closed case matrix, joins the frozen decomposition to the tracked
current result, and binds the repaired reproduction command to its published authorities.
SPEC-TO-INTENT-ALIGNMENT.8b must make the vertical evaluator agree with the same rule;
SPEC-TO-INTENT-ALIGNMENT.8c must emit the typed record; SPEC-TO-INTENT-ALIGNMENT.8d must replay
the declared population.
"""

# RUSTDOC-AND-XTASK.13.f — native since that leaf in `cargo xtask check residual-actionability`
# (xtask/src/checks/residual_actionability.rs, comparing values as Python does through xtask/src/pyjson.rs). This
# shim keeps the doctrine's registered path and its argparse front end, so help and refusals are exactly as they
# were, and runs the native check from the repository root, where the `cargo xtask` alias resolves.

from __future__ import annotations

import argparse
import os
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


def main() -> int:
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--check", action="store_true")
    mode.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    os.chdir(ROOT)
    os.execvp(
        "cargo",
        ["cargo", "xtask", "check", "residual-actionability", "--self-test" if args.self_test else "--check"],
    )
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
