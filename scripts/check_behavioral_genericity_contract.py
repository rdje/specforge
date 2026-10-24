#!/usr/bin/env python3
"""Validate the frozen behavioral-genericity population and relational oracle."""

# RUSTDOC-AND-XTASK.15.d — native since that leaf in `cargo xtask check behavioral-genericity-contract`
# (xtask/src/checks/behavioral_genericity_contract.rs, reading JSON through xtask/src/pyjson.rs exactly as Python's
# json module does). This shim keeps the component's path, which PRODUCTION-GENERICITY and two claims' re-derivations
# run, and its argparse front end, so help, abbreviations and refusals are exactly as they were; it passes the native
# check the flag it was given and runs it from the repository root, where the `cargo xtask` alias resolves.
from __future__ import annotations

import argparse
import os
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    flags = ["--self-test"] if args.self_test else []
    os.chdir(ROOT)
    os.execvp("cargo", ["cargo", "xtask", "check", "behavioral-genericity-contract", *flags])
    return 1


if __name__ == "__main__":
    sys.exit(main())
