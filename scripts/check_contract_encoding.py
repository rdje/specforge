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
from __future__ import annotations

import argparse
import itertools
import json
import pathlib
import subprocess
import sys
import tempfile

ROOT = pathlib.Path(__file__).resolve().parent.parent
REGISTRY = pathlib.Path("doctrine/contract_encoding/encoders.jsonl")
SCAN_GLOB = "doctrine/**/*.json"

INDENTS = (None, 1, 2, 3, 4)
SEPARATORS = ((",", ": "), (",", " : "))


def derive_encoder(raw: bytes):
    """The exact `json.dumps` options that reproduce `raw`, or None when none does.

    Deriving is the whole point: the options are a property of the last writer, never of the file
    or of the repository, and every document that has stated one has gone stale.
    """
    try:
        data = json.loads(raw)
    except (ValueError, UnicodeDecodeError):
        return None
    for indent, sort_keys, ensure_ascii, separators in itertools.product(
        INDENTS, (True, False), (True, False), SEPARATORS
    ):
        body = json.dumps(
            data, indent=indent, sort_keys=sort_keys,
            ensure_ascii=ensure_ascii, separators=separators,
        ).encode()
        for trailing_newline in (True, False):
            if raw == body + (b"\n" if trailing_newline else b""):
                return {
                    "indent": indent,
                    "sort_keys": sort_keys,
                    "ensure_ascii": ensure_ascii,
                    "separators": list(separators),
                    "trailing_newline": trailing_newline,
                }
    return None


def encoder_label(encoder) -> str:
    if encoder is None:
        return "hand-authored (no encoder reproduces it)"
    return (
        f"indent={encoder['indent']} sort_keys={encoder['sort_keys']} "
        f"ensure_ascii={encoder['ensure_ascii']} separators={tuple(encoder['separators'])!r} "
        f"trailing_newline={encoder['trailing_newline']}"
    )


def contracts(base: pathlib.Path):
    return sorted(
        p.relative_to(base).as_posix()
        for p in base.glob(SCAN_GLOB)
        if p.is_file()
    )


def read_registry(base: pathlib.Path, errors):
    path = base / REGISTRY
    if not path.exists():
        errors.append(f"registry '{REGISTRY}' is missing")
        return {}
    records = {}
    for number, line in enumerate(path.read_text(encoding="utf-8").split("\n"), start=1):
        if not line.strip():
            continue
        try:
            record = json.loads(line)
        except ValueError as exc:
            errors.append(f"registry line {number} is not JSON: {exc}")
            continue
        if record.get("record_type") != "contract":
            errors.append(f"registry line {number} has unknown record_type "
                          f"'{record.get('record_type')}'")
            continue
        target = record.get("path")
        if not target:
            errors.append(f"registry line {number} lacks a path")
            continue
        if target in records:
            errors.append(f"registry declares '{target}' twice")
        records[target] = record
    return records


def evaluate(base: pathlib.Path):
    errors = []
    declared = read_registry(base, errors)
    present = contracts(base)
    findings = []
    for relative in present:
        raw = (base / relative).read_bytes()
        actual = derive_encoder(raw)
        record = declared.get(relative)
        if record is None:
            errors.append(
                f"contract '{relative}' is not declared in {REGISTRY} — a new contract must "
                f"declare the encoder that wrote it ({encoder_label(actual)})")
            continue
        if record.get("hand_authored"):
            if actual is not None:
                errors.append(
                    f"contract '{relative}' is declared hand-authored but is now reproduced by "
                    f"{encoder_label(actual)} — it has been re-encoded, and its inline layout is "
                    f"gone")
            findings.append((relative, "hand-authored", "hand-authored"))
            continue
        expected = record.get("encoder")
        if actual is None:
            errors.append(
                f"contract '{relative}' no longer round-trips under any encoder; declared "
                f"{encoder_label(expected)}")
        elif actual != expected:
            errors.append(
                f"contract '{relative}' was re-encoded: declared {encoder_label(expected)}, "
                f"found {encoder_label(actual)}. Preserve the declared encoder, or update this "
                f"record in the same commit so the migration is visible in the diff")
        findings.append((relative, encoder_label(expected), encoder_label(actual)))
    for relative in sorted(set(declared) - set(present)):
        errors.append(f"registry declares '{relative}', which does not exist")
    return errors, findings, declared


def write_registry(base: pathlib.Path):
    lines = []
    for relative in contracts(base):
        encoder = derive_encoder((base / relative).read_bytes())
        record = {"path": relative, "record_type": "contract", "schema_version": 1}
        if encoder is None:
            record["hand_authored"] = True
        else:
            record["encoder"] = encoder
        lines.append(json.dumps(record, sort_keys=True, ensure_ascii=True))
    target = base / REGISTRY
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return len(lines)


def churn_ratio(base: pathlib.Path, relative: str):
    """Changed lines per changed field in the last commit that touched this contract."""
    revisions = subprocess.run(
        ["git", "-C", str(base), "log", "-1", "--format=%h", "--", relative],
        capture_output=True, text=True).stdout.split()
    if not revisions:
        return None
    revision = revisions[0]

    def load(reference):
        out = subprocess.run(["git", "-C", str(base), "show", f"{reference}:{relative}"],
                             capture_output=True)
        return json.loads(out.stdout) if out.returncode == 0 and out.stdout else None

    before, after = load(f"{revision}^"), load(revision)
    if before is None or after is None:
        return None

    def flatten(value, prefix=""):
        if isinstance(value, dict):
            for key, item in value.items():
                yield from flatten(item, f"{prefix}.{key}")
        elif isinstance(value, list):
            for index, item in enumerate(value):
                yield from flatten(item, f"{prefix}[{index}]")
        else:
            yield prefix, value

    a, b = dict(flatten(before)), dict(flatten(after))
    fields = len([k for k in set(a) | set(b) if a.get(k) != b.get(k)])
    stat = subprocess.run(
        ["git", "-C", str(base), "diff", "--numstat", f"{revision}^", revision, "--", relative],
        capture_output=True, text=True).stdout.split()
    churn = int(stat[0]) + int(stat[1]) if len(stat) >= 2 and stat[0].isdigit() else None
    return revision, churn, fields


def self_test() -> int:
    """Every refusal observed RED on a real tree before the gate is trusted to be green."""
    passed = 0
    with tempfile.TemporaryDirectory(dir=ROOT / "generated") as workspace:
        base = pathlib.Path(workspace)
        (base / "doctrine" / "sub").mkdir(parents=True)
        good = base / "doctrine" / "sub" / "good.json"
        good.write_bytes(json.dumps({"b": 1, "a": 2}, indent=2, sort_keys=True).encode() + b"\n")
        hand = base / "doctrine" / "hand.json"
        hand.write_bytes(b'{\n  "inline": {"a": 1, "b": 2}\n}\n')
        write_registry(base)

        errors, _, _ = evaluate(base)
        assert not errors, f"a freshly written registry must be green: {errors}"
        passed += 1

        # RED 1: re-encode a declared contract.
        good.write_bytes(json.dumps({"b": 1, "a": 2}, indent=4, sort_keys=True).encode() + b"\n")
        errors, _, _ = evaluate(base)
        assert any("was re-encoded" in e for e in errors), errors
        passed += 1
        good.write_bytes(json.dumps({"b": 1, "a": 2}, indent=2, sort_keys=True).encode() + b"\n")

        # RED 2: a contract that is not declared at all.
        fresh = base / "doctrine" / "fresh.json"
        fresh.write_bytes(b"{}\n")
        errors, _, _ = evaluate(base)
        assert any("is not declared" in e for e in errors), errors
        passed += 1
        fresh.unlink()

        # RED 3: a declared contract that has vanished.
        good.unlink()
        errors, _, _ = evaluate(base)
        assert any("does not exist" in e for e in errors), errors
        passed += 1
        good.write_bytes(json.dumps({"b": 1, "a": 2}, indent=2, sort_keys=True).encode() + b"\n")

        # RED 4: a hand-authored contract that has been machine re-encoded — the shape `.23`
        # measured expands 9-27% and destroys the inline layout, and it must not pass silently.
        hand.write_bytes(json.dumps({"inline": {"a": 1, "b": 2}}, indent=2).encode() + b"\n")
        errors, _, _ = evaluate(base)
        assert any("declared hand-authored" in e for e in errors), errors
        passed += 1
        hand.write_bytes(b'{\n  "inline": {"a": 1, "b": 2}\n}\n')

        errors, _, _ = evaluate(base)
        assert not errors, f"restored tree must be green again: {errors}"
        passed += 1

        # The derivation itself: the separator and newline dimensions must be distinguished, or a
        # JSON::PP-written contract would read as a json.dumps one.
        pretty = json.dumps({"a": 1}, indent=3, sort_keys=True, separators=(",", " : ")).encode()
        assert derive_encoder(pretty + b"\n")["separators"] == [",", " : "]
        assert derive_encoder(pretty)["trailing_newline"] is False
        assert derive_encoder(b'{"a": 1,}\n') is None
        passed += 1
    print(f"contract-encoding: self-test {passed}/{passed} derivation and refusal cases pass.")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--report", action="store_true")
    parser.add_argument("--write", action="store_true")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()

    if args.self_test:
        return self_test()
    if args.write:
        count = write_registry(ROOT)
        print(f"contract-encoding: declared {count} contract(s) in {REGISTRY}.")

    errors, findings, declared = evaluate(ROOT)
    if args.report:
        for relative, _, _ in findings:
            measured = churn_ratio(ROOT, relative)
            if measured and measured[1] and measured[2] is not None:
                revision, churn, fields = measured
                ratio = f"{churn / fields:.1f}x" if fields else "inf (0 fields)"
                print(f"  {relative:58s} last={revision} churn={churn:5d} "
                      f"fields={fields:3d} {ratio}")
    if errors:
        for error in errors:
            print(f"contract-encoding: {error}", file=sys.stderr)
        print(f"contract-encoding: FAILED with {len(errors)} violation(s).", file=sys.stderr)
        return 1
    hand = sum(1 for record in declared.values() if record.get("hand_authored"))
    print(f"contract-encoding: {len(findings)} doctrine JSON contract(s) still round-trip under "
          f"their declared encoder ({hand} hand-authored).")
    return 0


if __name__ == "__main__":
    sys.exit(main())
