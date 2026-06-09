---
id: definitional-signal-capture
title: Signals named only in prose (no signal table) are captured via a DEFINITIONAL grammar — copula "NAME is a/an signal" + glossary colon "NAME: signal" (SWP S1/S2)
answers:
  - "how does specforge capture a signal that is defined in prose not a signal table (SWP S1/S2)"
  - "what is definitional_signal_names / the copula + glossary-colon prose-signal grammar"
  - "why is descriptor apposition (signal NAME / NAME signal) NOT used to capture signal names"
  - "why is the abbreviation-table I/O-expansion not used to capture signals (MMIO/DMA/IOVA garbage)"
  - "why is SWIO not captured as a third SWP signal"
  - "how is the prose definitional signal grammar kept garbage-free without a denylist (ADR 0006)"
date: 2026-06-09
tags: [swp, signals, prose, extraction, evidence-ir, pdf-variant-digestion, adr-0006]
evidence: docs/tasks/PDF-VARIANT-DIGESTION.md (.9.8); crates/specforge/src/ir/evidence.rs (definitional_signal_names, synthesize_signal_declarations_from_prose)
reverify: python3 -c "import json,re; e=json.load(open('generated/evidence_ir/etsi_ts_102613_v16_0_0_2021_10_smart_cards_uicc_clf_single_wire_protocol_swp/evidence_ir.json')); print(sorted({re.match(r'Signal (\w+)',s['text']).group(1) for s in e['extracted_statements'] if s['text'].startswith('Signal ')}))"
---

Some specs name their signals only in PROSE, never in a signal-description table. The Single Wire Protocol
(SWP, ETSI TS 102 613) is the canonical case: it defines its two signals in sentences —
*"S1 is a signal in the voltage domain …"*, *"S2 is a signal in the current domain …"* — and a glossary line
*"S1: signal from the master to a slave"*. The two earlier prose readers both miss these: the pin-appositive
([[prose-pin-appositive-signal-capture]]) needs a `pin,` anchor; the parenthetical form
([[prose-signal-capture]]) needs `(NAME)`. SWP therefore entered the pipeline with **0 signals** until `.9.8`.

`synthesize_signal_declarations_from_prose` gained a THIRD additive form, `definitional_signal_names(text)`:
- **copula** — the token immediately before `is a|an signal` (`<NAME> is a/an signal …`);
- **glossary colon** — the single token before the first `:` when the next word is `signal` (`<NAME>: signal …`).

The discriminator that keeps it garbage-free — without any growing word-denylist — is that the candidate must
pass `is_hardware_signal_token` on the **original (un-cased)** token, i.e. it must be an **all-uppercase
identifier** (`S1`/`S2`/`SWIO`/`SWCLK`), so lowercase English subjects (`an interrupt is a signal`,
`it is a signal`, `Note: signal integrity …`) can never qualify. The shared `is_signal_synthesis_non_signal`
denylist still removes role/logic words. It runs under the same sparse-catalog fallback gate as the
parenthetical form, so table-rich specs never run it.

**Why not the other forms (decided by a corpus probe over ALL persisted evidence docs BEFORE coding):**
- **descriptor apposition** (`signal <NAME>` / `<NAME> signal`) — REJECTED, corpus-toxic. Bare `signal X`
  captures `IS/TO/NAMES/FROM/CONTROL/DATA` across 30–47 docs (`X signal` usually means "a signal of TYPE X",
  not a name). Even a recurrence-self-gated variant floods SWD/ADI (sparse → runs prose) with
  `AP/APB/ARM/DATA/OF/JTAG/HPROT`, which would regress its WIRE-BASED-100 100% (declared signals gate
  constraint/relation subjects).
- **abbreviation-table I/O-expansion** (`SWIO | … Input/Output`) — REJECTED, not corpus-clean. Gets SWIO +
  eMMC's real `DAT1-7`/`CMD`, but also `MMIO`(7 docs)/`MEM`/`DMA`/`IOVA`/`IOTLB` (memory/addressing concepts
  whose expansion merely contains "input/output").
- **definitional copula + glossary-colon** — ACCEPTED. Corpus-wide it yields EXACTLY `S1` + `S2`, zero garbage.

**Result:** SWP signals 0 → `S1`/`S2`. This is the complete, correct capture: SWP has exactly two signals —
S1 (voltage, master→slave) and S2 (current, slave→master). **SWIO is the shared physical contact (C6) that
carries both, not a third logical signal** (*"S1 shares the same electrical contact as S2"*), and has no
corpus-clean definitional form → an honest residual, never fabricated (the HONESTY GUARDRAIL). No regression:
a before/after `git stash` diff proves the SWD/ADI declared set is BYTE-IDENTICAL with vs without `.9.8`.
Grammar, not names (ADR 0006).
