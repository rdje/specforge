---
id: an-llm-constraint-record-cannot-be-judged-by-a-positional-gate
title: An LLM constraint record carries no clause span, so a positional subject gate judges it against the wrong obligation
answers:
  - "can the positional spurious-subject gates be wired into the LLM constraint path"
  - "why does is_post_passive_binding_only_subject refuse correct llm_sigcon records"
  - "what does an llm_sigcon record cite as its provenance"
  - "how many llm_sigcon records would the subject gates refuse"
  - "why is constraint_bearing_sentence the wrong narrowing for a model proposal"
  - "does the LLM constraint path apply positional subject gates"
  - "what blocks EXTRACTION-QUALITY-GAUGE.3j from wiring a gate"
date: 2026-09-17
status: current
tags: [extraction-quality, constraints, llm-path, gates]
evidence: crates/specforge/src/ir/evidence.rs; crates/specforge/src/ir/constraint_extract_llm.rs; scripts/measure_llm_subject_gate_refusals.py; docs/tasks/EXTRACTION-QUALITY-GAUGE.md
reverify: python3 scripts/measure_llm_subject_gate_refusals.py
---

The five positional spurious-subject gates (`CORPUS-COVERAGE.2.50a`, `EXTRACTION-QUALITY-GAUGE.3e`,
`.3g`, `.3h`, `INVARIANT-SHAPE-ADMISSION.5`) are wired as `subject_signals.retain(…)` in the two
deterministic extractors only. Wiring them into `constraint_extract_llm.rs` looks like a small
omission. It is not, and the measurement says so.

Censused over the **149** persisted `llm_sigcon_*` records in **7** documents, only one gate has any
population: `is_post_passive_binding_only_subject` would refuse **7** (4.7%), and the other three refuse
**0**. Two caveats belong with those counts. **The population is not current** — all seven documents sit
outside the refreshed cohort of `doctrine/corpus_frontier/census.json` (five outside the cohort rule,
`opencapi_3_0`/`3_1` listed `remaining`), so the counts describe the persisted corpus rather than what the
current binary emits; the mechanism below is a property of the code and is unaffected. And **a gate
reading 0 is only meaningful because the mirror is self-tested**: `--self-test` proves each gate fires and
declines on its own doc-comment example, 11/11, so the zeroes are real populations rather than a dead
mirror. Do not try to validate that mirror against surviving DETERMINISTIC records — those paths gate
`statement.text`, not the persisted `source_text`, so the comparison asks a different question and its
"21 of 195 collisions" mean nothing. Of those seven, **four are correct records the gate would destroy** — AXI's two `WTAG` records
(`must_be_value zero` against `WTAG must be zero`, `must_be_value VALID` against `WTAG bits must be
valid…`) and ATB's `AFVALID`/`ATVALID` `must_be_low` records, each against `… and <SUBJECT> must be
driven LOW` in the same sentence. Precision on this path is **3 of 7**.

**The mechanism is a span mismatch, not a bad rule.** `is_post_passive_binding_only_subject` narrows to
`constraint_bearing_sentence`, the FIRST clause of the statement carrying a modal. A model proposal can
be minted from any obligation in the statement, so every record after the first is judged against a
clause it did not come from. The repository already solved this for the deterministic path in
`EXTRACTION-QUALITY-GAUGE.3k.3`, by adding `is_post_passive_binding_only_subject_in` — the same gate
*told which obligation the record came from*. **The LLM record cannot use it**: it cites only
`supporting_statement_ids`, which names the whole statement, never the clause.

So the prerequisite for any positional gate on this path is that the record carry its own clause span
(`EXTRACTION-QUALITY-GAUGE.3j.1`). Re-deriving the clause at gate time from the record's kind and value
is not a substitute — that is a second reader of the same statement, and it will disagree with the
first exactly where the disagreement matters.

**A premise formed from a subject spelling is worth re-reading against the record's kind.** `.3j` was
opened citing AXI `llm_sigcon_0025`/`0027` as the LLM path mis-attributing `WTAGUPDATE must be
deasserted` to `WTAG`. Both records are in fact correct; only their `constraint_kind` shows it, and the
premise had been formed without reading it. The gate flagged them, the extractor did not err.

See [[a-cheap-structural-rule-overfires-until-you-read-its-selection]].
