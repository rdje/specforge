---
id: constraint-record-producer-strata
title: Five producers mint a SignalConstraintRecord and only three reach the kind classifier, so a census of the constraint table is not the population of a classifier change
answers:
  - "which functions call classify_signal_constraint_kind"
  - "does the dynamic constraint path use classify_signal_constraint_kind"
  - "does the LLM constraint path use classify_signal_constraint_kind"
  - "what is the population of a kind-classifier change in evidence.rs"
  - "how many producers mint a SignalConstraintRecord"
  - "what do the sigcon dyn_sigcon row_sigcon keyed_row_sigcon llm_sigcon id prefixes mean"
  - "do dyn_sigcon records ever carry the untyped MustBeStable default"
  - "which span does a sigcon record read its subject condition negation and kind from"
  - "which span does a dyn_sigcon record read its parts from"
  - "how do I census which span a published constraint read its parts from"
  - "why is 18 of 349 the wrong population for the constraint kind span"
  - "can constraint_bearing_sentence be used to narrow the dynamic constraint path"
date: 2026-10-23
status: current
tags: [evidence-ir, signal-constraints, extraction-quality-gauge, census, claim-verification]
evidence: crates/specforge/src/ir/evidence.rs (extract_signal_constraints; extract_dynamic_signal_constraints; extract_signal_description_row_constraints; extract_signal_keyed_obligation_row_constraints; classify_signal_constraint_kind_typed, the gateway to classify_signal_constraint_kind, and its three production callers); crates/specforge/src/ir/constraint_extract_llm.rs (parse_kind); scripts/measure_constraint_part_span.py (native since RUSTDOC-AND-XTASK.14.b, xtask/src/checks/constraint_part_span.rs); docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.3i, .3k, .3k.2a, .3k.2e); docs/tasks/RUSTDOC-AND-XTASK.md (.14.b, the 2026-10-23 counts)
reverify: "bash scripts/check_constraint_part_span.sh — expect 'kind-classifier call sites unchanged (3 producers, 1 reading the whole statement; the untyped arm is reached only by classify_signal_constraint_kind_typed)'; then python3 scripts/measure_constraint_part_span.py and read the per-stratum block"
---

`EvidenceIr.signal_constraints` is **one table with five producers**, distinguishable only by the
`constraint_id` prefix. They do not share a typing path, so a census of the table measures five
different rules at once. The three that reach the kind classifier all go through its typed gateway,
`classify_signal_constraint_kind_typed` (`EXTRACTION-QUALITY-GAUGE.3k.2a`, `.3k.2e`), and the gateway is
the only caller of the bare `classify_signal_constraint_kind`.

| prefix | producer | how the kind is decided |
| --- | --- | --- |
| `sigcon_*` | `extract_signal_constraints` | the kind classifier over the **whole statement** |
| `row_sigcon_*` | `extract_signal_description_row_constraints` | the kind classifier over **one description-cell clause** |
| `keyed_row_sigcon_*` | `extract_signal_keyed_obligation_row_constraints` | the kind classifier over the sentence a two-cell obligation row states, already **one clause** (`EXTRACTION-GAP-FIX.5b`) |
| `dyn_sigcon_*` | `extract_dynamic_signal_constraints` | its own **value binder** — `extract_discovered_state_value_from_text`, else `logic_level_binding_kind_from_text`; no record is minted when neither fires |
| `llm_sigcon_*` | `ir/constraint_extract_llm.rs` | `parse_kind` over the kind the **model named** |

Two consequences that have already cost this tree two published numbers
(`EXTRACTION-QUALITY-GAUGE.3i`, corrected by `.3k`):

1. **A change to the kind classifier can only move `sigcon_*`, `row_sigcon_*` and
   `keyed_row_sigcon_*` records.** Comparing "classify over the clause" with "classify over the whole
   text" across the full table gives 21 of 364 records (`2026-10-23`); 15 of those 21 are
   `dyn_sigcon_*` or `llm_sigcon_*`, where the comparison runs a function that never touches them. The
   classifier's own population is 6, all `sigcon_*`. At `.3k` (`2026-09-12`) it was 19 of 349, 15, and 4.
2. **The dynamic path never publishes the untyped `MustBeStable` default.** Its record only exists
   when a binder typed it, so all 73 persisted `dyn_sigcon_*` records (`2026-10-23`; 77 at `.3k`) carry
   `must_be_value` (36), `must_be_low` (20), `must_be_high` (12) or `tied_to` (5). Any census row of
   the form "records sitting on the untyped default" is vacuously zero for that stratum, and a non-zero
   answer means the census mirrored the wrong producer.

The same stratification decides **what "the clause" means**, which is why the two deterministic paths
cannot simply share a helper. A `sigcon_*` record is minted by a MODAL obligation, so its clause is
`constraint_bearing_sentence` — the first `.`/`;`/`•`/newline part containing `must`/`shall`, already
the span of its subject, its condition and (since `.3i`) its negation. A `dyn_sigcon_*` record is
minted by a VALUE BINDING that need not be modal at all (*"X is tied HIGH"*), so
`constraint_bearing_sentence` would narrow it to a sentence that does not contain its binding, or
fall through to the whole text; it needs a binding-bearing clause instead, and no helper computes one
today.

`scripts/measure_constraint_part_span.py` is the census (native since `RUSTDOC-AND-XTASK.14.b`, in
`cargo xtask check constraint-part-span`), stratified by producer and marking every classifier-derived
row `[n/a]` for the two strata out of the classifier's reach. Its `--check` mode, which the
`CONSTRAINT-PART-SPAN` doctrine runs, fails closed when the typed gateway gains, loses, or moves a
production caller, or when anything else reaches the bare classifier, because that is exactly when
the stratification above stops being true.

Related: [[self-test-coverage-guard-is-in-the-exit-path]] — the same failure shape, a number
published from a proxy nothing exercised.
