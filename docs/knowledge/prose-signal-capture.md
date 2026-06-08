---
id: prose-signal-capture
title: Signals introduced in PROSE (not tables) are captured — pin appositive + parenthetical abbreviation, as a sparse-catalog fallback
answers:
  - "how does SpecForge capture signals that are in prose not tables (I2C SDA/SCL)"
  - "what is PDF-VARIANT-DIGESTION.3 prose entity capture"
  - "why do I2C/CCIX/USB4 have 0 table signals and how are they recovered"
  - "how is prose signal over-capture prevented (no garbage)"
  - "how does SpecForge capture protocol actors/agents defined in prose (.3b/.8 extract_protocol_actors)"
  - "how is the agent-definition grammar kept garbage-free without a fragile noun denylist"
  - "what are AGENT_CLASS_NOUNS / the parenthetical-strip / sentence-boundary / no-preposition guards"
date: 2026-06-08
tags: [prose, signals, actors, extraction, pdf-variant-digestion, lever-b]
evidence: crates/specforge/src/ir/evidence.rs (synthesize_signal_declarations_from_prose, agent_definitions, extract_protocol_actors)
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
I2C acronyms (ACK/NACK/DDC/SDR). Wire-based specs unchanged.

**Prose ACTOR/AGENT capture (`.3b`, broadened by `.8`)** — `ProtocolActorRecord` + `extract_protocol_actors`
capture agents a spec DEFINES in prose. `.8` generalized the `.3b` literal "is the device that/which" anchor to
`agent_definitions(text)`: `"<NAME> is a/an/the <agent-class> {that|which} <capability>"` over a conservative
generic agent-class ALLOWLIST `AGENT_CLASS_NOUNS` (device/component/agent/module/entity/controller/manager/
master/initiator/peripheral/bridge/engine/processor/host/node/subsystem — the ambiguous "unit"/"block"
EXCLUDED). Form 2 ("considered a/the/an <NAME>") unchanged; `is_agent_noun` (function-word denylist) UNCHANGED.

**Garbage stays out via STRUCTURAL signals, NOT a growing noun denylist** (an expert reviewer flagged a
structural-noun denylist as fragile → redesigned):
- `strip_trailing_parenthetical` drops "(refer to section 3.1.2.1)" before the NAME → recovers the real
  "controller", not "section";
- the NAME search is confined to the CURRENT SENTENCE so "… host system. It is the entity that …" can't reach
  back across the period and name "system";
- a NO-PREPOSITION-in-subject guard (prepositions = a CLOSED grammatical class) rejects a prepositional-phrase
  object: "a use case FOR multiple HSEL signals is a peripheral that …" → "signals" rejected.

I2C → controller + target. `.8` lifts the corpus from 14 → 20 docs with a recovered agent (projected via a
faithful mirror; canonical on the 3 un-reclaimed-`normalized` docs: NVMe 0→1 "controller"), every gained actor
genuine (A76 core / ETM trace unit, CoreSight splitter, CCIX Transport port, AXI manager). Additive surface; no
eval impact; APB/AHB/AXI/AXI-Stream/SWD recover 0 actors (clean). ADR 0006 (no chip names). KM
[[prose-pin-appositive-signal-capture]].
