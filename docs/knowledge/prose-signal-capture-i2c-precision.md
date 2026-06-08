---
id: prose-signal-capture-i2c-precision
title: Prose-signal capture is recall-perfect but precision-leaky on I2C (1.000 / 0.600) — over-captures conditions + acronyms
answers:
  - "how good is prose signal capture / .3a quality"
  - "what is the I2C declared-signal recall / precision"
  - "why are ACK / NACK / DDC / SDR extracted as I2C signals"
  - "what is the declared-signal eval surface / EvalTask::DeclaredSignal"
  - "what is declared_signal_complete_gold_precision"
  - "where is the I2C signal gold seed"
date: 2026-06-08
tags: [eval, signals, prose-capture, pdf-variant-digestion, i2c, scoring-rigor]
evidence: crates/specforge/src/eval.rs (EvalTask::DeclaredSignal, GoldFact::DeclaredSignal, declared_signal_record_key, index_declared_signal_predictions, declared_signal_complete_gold_precision); crates/specforge/src/commands/eval_extraction.rs (declared-signal surface); crates/specforge/test_data/llm_eval/seed_i2c_signals.json; docs/tasks/PDF-VARIANT-DIGESTION.md (.4a.4/.4a.5)
reverify: ./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_i2c_signals.json --provider skip 2>&1 | grep -A2 "declared-signal surface"
---

`PDF-VARIANT-DIGESTION.4a.4`/`.4a.5`. The prose-signal capture ([[prose-signal-capture]], `.3a`, which reads
signals from prose like "a serial data line (SDA)") is now MEASURED per-fact.

**The eval surface (`.4a.4`, additive — no extraction change):** `EvalTask::DeclaredSignal` +
`GoldFact::DeclaredSignal { signal, direction? }`. The canonical inventory lives on the SemanticIR
(`interfaces[].signal_records`, `InterfaceSignalRecord` = `signal_name` + `direction_hint`
Input/Output/Internal), so the runner builds the SemanticIR on a temp copy (like the temporal-rule task) and
pools every interface's `signal_records`. Identity = name + optional direction (a name-only gold matches a
no-direction record; a wrong direction is a miss). Runtime stays PDF-agnostic (ADR 0006): the surface holds no
signal names — they come from the gold answer-key + the records.

**I2C result (`.4a.5`):** gold = the COMPLETE set of I2C-bus physical signals (`SDA`/`SCL` §3.1.1, Hs-mode
`SDAH`/`SCLH` §3.6, UFm `USDA`/`USCL` §3.2.1 — each verified against UM10204's own "signals" sections).
Measured (`--provider skip`, against verified-current persisted evidence): **recall 1.000** (all 6 genuine
signals found, source-tolerant) but **precision 6/10 = 0.600** — four over-captures named explicitly: `ACK`,
`NACK` (acknowledge/not-acknowledge *conditions* on the SDA line, §3.1.6 — not separate wires), `DDC` (Display
Data Channel, a *different* bus, §4.6), `SDR` (an I3C "standard data rate" acronym). So prose capture is
greedy: it finds every real signal but also pulls in protocol conditions and uppercase acronyms.

**Two scoring notes:** (1) ordinary statement-scoped precision reports `fp=0` here because each spurious
signal is attributed to its own synthesized declaration statement, not the one labelled sentence — so
`declared_signal_complete_gold_precision` reports precision over the produced signal SET and names the false
positives. (2) That document-level precision is valid ONLY when the gold enumerates EVERY true signal (a
small fully-specified bus like I2C); the surface labels the assumption rather than reporting a number that
would be wrong on a sampled gold.

**Fix leaf identified (future tree, NOT measurement):** tighten the prose acronym/condition filter so
acknowledge-type conditions, cross-bus names, and bare rate acronyms are not minted as interface signals —
now quantified (precision 0.600 → target higher) so a fix is measurable against this gold.
