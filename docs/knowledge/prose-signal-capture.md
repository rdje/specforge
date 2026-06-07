---
id: prose-signal-capture
title: Signals introduced in PROSE (not tables) are captured — pin appositive + parenthetical abbreviation, as a sparse-catalog fallback
answers:
  - "how does SpecForge capture signals that are in prose not tables (I2C SDA/SCL)"
  - "what is PDF-VARIANT-DIGESTION.3 prose entity capture"
  - "why do I2C/CCIX/USB4 have 0 table signals and how are they recovered"
  - "how is prose signal over-capture prevented (no garbage)"
date: 2026-06-07
tags: [prose, signals, extraction, pdf-variant-digestion, lever-b]
evidence: crates/specforge/src/ir/evidence.rs (synthesize_signal_declarations_from_prose)
reverify: ./target/debug/specforge evidence generated/source_ir/um10204_rev7_0_2021_i2c_bus_specification/source_ir.json && python3 -c "import json,re;e=json.load(open('generated/evidence_ir/um10204_rev7_0_2021_i2c_bus_specification/evidence_ir.json'));print(sorted({re.match(r'Signal (\w+)',x['text']).group(1) for x in e['extracted_statements'] if x['text'].startswith('Signal ')}))"
---

Lever B: some specs name their signals only in PROSE, not signal-description tables — the triage showed
I2C/CCIX/USB4 with 0 table signals. `synthesize_signal_declarations_from_prose` recovers them, two forms:
- **Pin appositive** (SWD-SERIAL-EXTRACTION.2, always on): "a clock pin, SWCLK" → SWCLK/SWDIO.
- **Parenthetical abbreviation** (PDF-VARIANT-DIGESTION.3): "a serial data line **(SDA)**", "serial clock
  **(SCL)**" → the `(NAME)` is captured when a signal DESCRIPTOR (line/signal/clock/data/wire/bus/pin) is in
  the preceding ≤4 words.

**No-garbage guards (learned from live I2C runs):**
- **Sparse-catalog FALLBACK gate** — the parenthetical form runs only when the document has < 8
  table-declared signals. Table-rich specs (AXI: hundreds) get signals from tables; running prose capture
  there is redundant noise that admitted a garbage constraint (AXI signal_constraint regressed 1.000→0.857
  before the gate). With the gate: AXI/APB/AHB/SWD stay 1.000.
- **Uppercase-acronym gate** — the parenthetical name must be an uppercase acronym (2–10 chars, no
  lowercase), so "(resulting from …)" / "(Section …)" are not captured (I2C 23→11 declared signals).
- **Universal denylist** — `is_signal_synthesis_non_signal` rejects operation/role/logic words
  (READ/WRITE/MODE/THE/HIGH/…); ADR 0006 (no chip names).

Result: I2C 0 → 10 declared signals — SDA/SCL + Hs-mode SCLH/SDAH + USCL/USDA (real wires), plus a few real
I2C acronyms (ACK/NACK/DDC/SDR). Wire-based specs unchanged. NEXT (`.3b`): prose ACTOR/AGENT capture ("A
controller is the device which initiates …") — ground the agent model from prose, not only as a relation
subject ([[project_pdf_variant_digestion]]).
