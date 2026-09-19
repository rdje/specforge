---
id: bounded-decision-frozen-baseline
title: The frozen bounded-decision baseline scores arm A at macro-F1 0.90715 on 644 declaration rows and 0.65014 on 613 captions, and none of its 31 errors is genuine ambiguity
answers:
  - "what is the frozen adjudication set for the bounded decision provider"
  - "what does arm A score on the declaration row decision"
  - "what does arm A score on the caption admission decision"
  - "what is the pre-registered adoption bar for a remote decision provider"
  - "how many declaration rows does the current reader lose on the frozen set"
  - "does the declaration reader publish any false positive declaration"
  - "are the declaration row disagreements ambiguity or rule defect"
  - "is the signal-or-not decision a table property or a row property"
  - "why is declarations_emitted not the reader's final decision"
  - "what is withhold_base_name_template_declarations doing to the accounting"
  - "how many captions in the frozen set state a normative constraint"
  - "why does route r1 admit Table A8.2 Opcodes which must be cache line sized"
  - "which document loses the most declaration rows on the frozen set"
  - "how do I re-derive the bounded decision baseline"
  - "what would make a remote decision provider worth adopting"
  - "how many tables in the frozen set are mixed real and phantom"
date: 2026-09-19
status: current
tags: [bounded-decision-provider, baseline, gold, evidence-ir, semantic-ir, declarations, invariants, pre-registration]
evidence: scripts/build_bounded_decision_baseline.py; docs/research/bounded-decision-adjudication.jsonl; docs/research/bounded-decision-baseline.md; docs/tasks/BOUNDED-DECISION-PROVIDER.md (.1); crates/specforge/src/ir/evidence.rs (synthesize_signal_declarations, withhold_base_name_template_declarations); crates/specforge/src/ir/semantic.rs (is_invariant_like, statement_is_a_caption)
reverify: "python3 scripts/build_bounded_decision_baseline.py --check && python3 scripts/build_bounded_decision_baseline.py --self-test — expect the pins to agree and 10/10 RED cases; `--verify-currency` additionally proves the persisted corpus still renders the frozen set byte-for-byte, and fails loudly once a rebuild moves it."
---

`BOUNDED-DECISION-PROVIDER.1` froze the set a bounded decision provider has to beat: **644
signal-table rows across 101 tables** (decision `declaration_row`) and **613 figure/table captions**
(decision `caption_admission`), from the four documents carrying
`extraction_manifest.declaration_row_accounting` — AXI `ihi0022_l`, APB `ihi0024_e`, AHB
`ihi0033_c`, ADIv6 `ihi0074_a`. Each row carries its header, caption and cells verbatim, so the
score re-derives from the tracked file alone with no dependency on `generated/`.

## Arm A — the current deterministic rules

| decision | rows | gold + | tp | fp | fn | tn | precision | recall | macro-F1 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `declaration_row` | 644 | 589 | 567 | 0 | 22 | 55 | `1.0000` | `0.9627` | `0.90715` |
| `caption_admission` | 613 | 6 | 2 | 5 | 4 | 602 | `0.2857` | `0.3333` | `0.65014` |

## `declarations_emitted` is not the reader's final answer

AXI `table_0011` records `declarations_emitted == 6` and **zero**
`table_signal_declaration_provenance` records: `withhold_base_name_template_declarations`
(`WIRE-BASED-100.10b`) removes a base-name template table's declarations once every table producer
has run. Scoring the accounting alone publishes six false positives the product does not make and
reports precision `0.9895` instead of `1.0000`. The frozen set therefore carries three reader
states — `emitted`, `withheld`, `dropped` — and only `emitted` scores positive. See
[[declaration-reader-drops-uninterpretable-rows]] for the accounting itself.

## The decision is a TABLE property in 101 of 101 tables

Not one table in the population is mixed. That reproduces
[[a-dropped-declaration-row-is-usually-not-a-signal]]'s ten-table finding on a population ten times
larger and is the load-bearing fact for the evaluation: **a per-row ranking model is being asked a
question whose answer is settled one level up**, so its whole achievable win is re-deciding eight
tables, and the three table-level discriminators already tried and refuted remain the frontier.

## Every error is classified, and none of them is ambiguity

| decision | cause | rows |
| --- | --- | ---: |
| `declaration_row` | name-cell selection — the name is not in the column the reader chose | 6 |
| `declaration_row` | no attribute stated anywhere — identity and nothing else | 8 |
| `declaration_row` | attribute lost by ingest — the text layer merged or scattered the direction | 7 |
| `declaration_row` | abbreviated direction refused — `Out` as a whole cell under a `Direction` header | 1 |
| `caption_admission` | a deontic WORD inside a title or under a reporting verb (all 5 FP) | 5 |
| `caption_admission` | a deontic CLAUSE outside route `r1`'s phrase list (all 4 FN) | 4 |
| — | **genuine ambiguity** | **0** |

The loss is also concentrated, not diffuse: AXI and AHB score **0 errors across 559 rows**, APB
loses one, and **ADIv6 loses 21 of its 24 rows while all 24 are real signals** — an 87.5% loss in a
wire-bearing document that no gold score and no currency gate can see
([[a-green-score-is-evidence-only-about-its-gold]]).

## The bar, fixed before any model ran

Adoption requires arm C to beat **arm A and arm B each** by **≥ 0.05 absolute macro-F1**; to cut
total errors to **≤ 60%** of the better of them, by at least `ceil(0.4 × arm A errors)` rows with a
floor of 4 (today: `declaration_row` ≤ 13 errors and ≥ 12 rows corrected, `caption_admission` ≤ 5
and ≥ 4); to add **no** false positive over the better of them; and to hold the margin with the 8
recorded contested rows removed as well as included. Disqualifiers D1–D4 stay fatal independently.
The full statement, the per-row enumeration, and what the baseline does **not** establish are in
`docs/research/bounded-decision-baseline.md`.

`--check` refuses any drift in the frozen digest, either population size, or either confusion
matrix, so production moving is an event that has to be re-derived rather than absorbed.
