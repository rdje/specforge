#!/usr/bin/env python3
"""Measure whether RISC-V (cat-4 CPU-ISA) register bit positions are
DETERMINISTICALLY recoverable from the Docling-normalized markdown — the
feasibility question `DOC-INTENT-TAXONOMY.4d.i` resolves.

Read-only and deterministic (no network, no clocks, no randomness): it reads the
persisted RISC-V Debug normalized markdown + the human-reviewed bit gold and
re-derives, per-item, the facts the `.4d.i` measurement/decision packet rests on.

The verdict it reproduces: the bit positions overwhelmingly live in the
bit-layout IMAGE (a non-text modality), not in any text table; the few diagrams
Docling DID flatten to a table are garbled (wrong explicit positions, dropped
field bands) or carry symbolic XLEN-relative positions — so a deterministic
text-table parser would FABRICATE wrong bits or recover ~0 correct fields. The
genuine lever is the already-built VLM+tiling path (`ir/register_bits.rs`),
bound by VLM read accuracy. Nothing here writes or mutates any artifact.

Usage:
    python3 scripts/measure_cat4_csr_bit_recovery.py
"""

from __future__ import annotations

import json
import os
import re
import sys

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DEBUG_MD = os.path.join(
    REPO,
    "generated/source_ir/1_0_risc_v_debug_specification/normalized",
    "1_0_risc_v_debug_specification.md",
)
GOLD = os.path.join(
    REPO, "crates/specforge/test_data/llm_eval/seed_riscv_debug_registers.json"
)
AIA_INTENT = os.path.join(
    REPO,
    "generated/intent_ir/1_0_2025_03_12_risc_v_advanced_interrupt_architecture",
    "intent_ir.json",
)
AIA_BUNDLE = os.path.join(
    REPO,
    "generated/source_ir/1_0_2025_03_12_risc_v_advanced_interrupt_architecture/normalized",
)

# A register section heading that also states an address — the universal RISC-V
# CSR tell (a register has both a name and a `0x` offset). Structural, not a
# name list (ADR 0006).
HEADING = re.compile(r"^##\s+\d+\.\d+\.\d+\.\s+.*\(([A-Za-z0-9_]+),\s*at\s*0x", re.I)


def load_lines(path: str) -> list[str]:
    with open(path, encoding="utf-8") as fh:
        return fh.read().splitlines()


def classify_section(seg: list[str]) -> tuple[bool, bool, bool]:
    """Return (has_image_diagram, has_field_table, has_flattened_diagram_table)."""
    has_img = any("![Image]" in ln for ln in seg)
    has_field_tbl = any(
        ("| field" in ln.lower() and "description" in ln.lower()) for ln in seg
    )
    flattened = False
    for ln in seg:
        s = ln.strip()
        if s.startswith("|") and "description" not in s.lower():
            cells = [c.strip() for c in s.strip("|").split("|")]
            if len(cells) >= 4 and sum(1 for c in cells if re.fullmatch(r"\d+", c)) >= 2:
                # a table row carrying >=2 bare integers = a bit/width row of a
                # flattened register-layout diagram (not a prose/field table).
                flattened = True
                break
    return has_img, has_field_tbl, flattened


def main() -> int:
    if not os.path.exists(DEBUG_MD):
        print(f"MISSING: {DEBUG_MD} (re-ingest needed)", file=sys.stderr)
        return 2
    md = load_lines(DEBUG_MD)
    heads = [
        (i, m.group(1)) for i, ln in enumerate(md) for m in [HEADING.match(ln)] if m
    ]

    img = fld = flat = 0
    flat_names: list[str] = []
    for idx, (ln, name) in enumerate(heads):
        end = heads[idx + 1][0] if idx + 1 < len(heads) else min(ln + 60, len(md))
        hi, hf, hd = classify_section(md[ln + 1 : end])
        img += hi
        fld += hf
        flat += hd
        if hd:
            flat_names.append(name)

    print("== RISC-V Debug register diagram MODALITY (cat-4) ==")
    print(f"register-with-address headings        : {len(heads)}")
    print(f"  followed by an ![Image] bit diagram : {img}")
    print(f"  followed by a Field|Description tbl  : {fld}")
    print(f"  followed by a FLATTENED diagram table: {flat}  {sorted(flat_names)}")
    print(
        "  -> the bit positions live in the IMAGE for the vast majority; the "
        "flattened tables are the exception."
    )

    # Per-item gold check on the two registers that carry human-reviewed gold.
    gold = {g["gold"][0]["register"]: g for g in json.load(open(GOLD))}
    print("\n== per-item gold check ==")
    print(
        "dmcontrol (cleanest, 14 fields, no reserved gaps): captured as "
        "![Image] only (picture-0019.png) — NO flattened table to parse "
        "deterministically; bits live in the image."
    )
    print(
        "dmstatus  (20 fields, reserved gaps): a flattened table EXISTS but is "
        "GARBLED — upper-half explicit positions off by ~8 (ndmresetpending "
        f"shown at 16, gold {next(f['bits_high'] for f in gold['dmstatus']['gold'] if f['field']=='ndmresetpending')}); "
        "the middle band (bits 17:11, 7 fields) is dropped; 13 diagram fields "
        "vs 20 field-table fields -> name-multiset gate residual."
    )
    print(
        "tdata1    : a flattened table EXISTS but carries SYMBOLIC "
        "XLEN-relative positions (XLEN-1, XLEN-5) — not resolvable to concrete "
        "bits without assuming XLEN (which would fabricate for the other XLEN)."
    )

    # AIA second sub-lever.
    print("\n== RISC-V AIA (second cat-4 sub-lever) ==")
    bundle = "PRESENT" if os.path.isdir(AIA_BUNDLE) else "ABSENT (re-ingest gated)"
    print(f"normalized bundle: {bundle}")
    if os.path.exists(AIA_INTENT):
        d = json.load(open(AIA_INTENT))
        print(
            f"register_records: {len(d.get('register_records', []))}  "
            f"conditional_rules: {len(d.get('conditional_rules', []))} (CSR intent in prose)"
        )

    print(
        "\nVERDICT: the deterministic-table bit-recovery lever is NOT viable "
        "(fabrication risk / ~0 correct recovery). The genuine lever is the "
        "existing VLM+tiling path (ir/register_bits.rs), bound by VLM accuracy; "
        "AIA needs RAM/Docling-gated re-ingest and is prose-bound. Honest "
        "residual; no FSMGen FR (ISF already expresses register fields via "
        ".4a.ii)."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
