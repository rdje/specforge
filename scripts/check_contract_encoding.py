#!/usr/bin/env python3
"""check_contract_encoding.py — a doctrine contract may not be silently re-encoded.

`LIVE-DOCUMENT-PRESSURE-HEADROOM.39a`. `.39` measured **31 encoder flips across 12 of the 23**
JSON contracts under `doctrine/` that have more than one revision. Each flip is a commit that
rewrote a whole file to carry a handful of real changes: `0a45c9d2` spent **2,071 changed lines on
8 changed fields**, and `34cab4d8` spent **1,280 lines on none at all**. A reviewer cannot see the
real change in any of them, and `65e2b4d2` silently invalidated the fact card that warns about
exactly this.

## Why this is an ABSOLUTE invariant and not a diff

The obvious gate compares the encoder at `HEAD` with the encoder in the worktree and refuses a
change. `.39a` measured that design and rejected it twice over:

1. It fires on **31 of 31** historical flips, **13 of which declare a migration** in their commit
   message. A gate that refuses 42% of its own population as it arrives is a policy defect
   (`LIVE-DOCUMENT-PRESSURE-HEADROOM.25`), and a keyword scan of a commit message is not a
   declaration anybody can gate on.
2. A diff has no fixed meaning outside a pre-commit hook. On a fresh CI checkout there is nothing
   to compare against, so the check would be silently inert exactly where it is un-bypassable.

So the invariant is absolute instead: **every contract's encoder is DECLARED, and the file must
still round-trip under the encoder declared for it.** A reformat then fails because the file stops
matching its declaration, and a deliberate migration passes because the migrating commit updates
the declaration — a one-line registry edit that sits in the diff where a reviewer will see it.
That is the escape hatch, and it costs a line rather than an authority record.

It also retires a carried constant. `CLAIM_VERIFICATION.md` §3 requires a value that is a function
of this repository to be derived or gated, never carried; the fact card
`[[live-surface-edit-bookkeeping-chain]]` carried a per-file encoder and it went stale within a
week. The registry this gate reads is now the answer, and the gate keeps it true.

## What it still permits

A contract rewritten under the SAME encoder — reordered keys under `sort_keys=False`, or a
wholesale content rewrite that happens to re-encode identically. This gate is about the encoding,
not about churn; `--report` prints the per-file churn ratio so a reviewer can see the other kind.

Usage:
  python3 scripts/check_contract_encoding.py [--check] [--report] [--write] [--self-test]
"""

# RUSTDOC-AND-XTASK.13.c — native since that leaf in `cargo xtask check contract-encoding`
# (xtask/src/checks/contract_encoding.rs, reading and writing JSON through xtask/src/pyjson.rs exactly as Python's
# json module does). This shim keeps the doctrine's registered path and its argparse front end, so help,
# abbreviations such as `--wr` and refusals are exactly as they were; it passes the native check the flags it was
# given and runs it from the repository root, where the `cargo xtask` alias resolves.
from __future__ import annotations

import argparse
import os
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--report", action="store_true")
    parser.add_argument("--write", action="store_true")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    flags = [flag for flag, given in (("--check", args.check), ("--report", args.report),
                                      ("--write", args.write), ("--self-test", args.self_test)) if given]
    os.chdir(ROOT)
    os.execvp("cargo", ["cargo", "xtask", "check", "contract-encoding", *flags])
    return 1


if __name__ == "__main__":
    sys.exit(main())
