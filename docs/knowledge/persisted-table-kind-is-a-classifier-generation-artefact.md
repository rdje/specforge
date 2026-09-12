---
id: persisted-table-kind-is-a-classifier-generation-artefact
title: 124 legacy tables carry a table_kind the current classifier would not assign, minting 598 declarations — and 0 in the proof-carrying stratum
answers:
  - "why does a persisted SourceIR table say signal_description when it is a characteristics matrix"
  - "does the SourceIR classifier type a bus-mode matrix as a signal table"
  - "why does eMMC table_0020 declare HS400 as a signal"
  - "is a persisted table_kind evidence about the current classifier"
  - "how many persisted signal_description tables would the current classifier reject"
  - "what refuses a table whose headers are Mode Name and Bus Width"
  - "why does Mode Name not count as a name role"
  - "why does Bus Width not count as a width role"
  - "how do I check whether a persisted artifact field describes current behaviour"
  - "which legacy declarations come from tables that are not signal tables"
  - "is SIGNAL-DECLARATION-ROW-DROP.2c still blocked on a guard"
date: 2026-09-12
status: current
tags: [source-ir, classification, legacy-artifacts, adr-0006, adr-0025, prose-name-cell-declaration]
evidence: scripts/measure_signal_table_classification_drift.py; crates/specforge/src/ir/source.rs (classified_table_kind; header_has_role; header_names_signals; a_matrix_that_qualifies_every_role_with_its_subject_is_not_a_signal_table); docs/tasks/PROSE-NAME-CELL-DECLARATION.md (.4)
reverify: "python3 scripts/measure_signal_table_classification_drift.py — expect CURRENT 118 still typed (107 caption / 11 header) and 0 that would not be; LEGACY 360 still typed (242 / 118) and 124 that would not be, minting 598 declarations. Then cargo test -p specforge-core --lib a_matrix_that_qualifies_every_role_with_its_subject_is_not_a_signal_table"
---

`table_kind` is a persisted field. Like every persisted field it records **the producer that wrote it**,
and this repository has now mistaken one for current behaviour three times.

`PROSE-NAME-CELL-DECLARATION` opened on eMMC `table_0020` — a bus-speed-mode matrix
(`Mode Name | Data Rate | IO Voltage | Bus Width | Frequency | Max Data Transfer`) carrying
`table_kind: signal_description`, whose `HS400` row mints a signal. The obvious reading is that the
SourceIR classifier types a characteristics matrix as a signal inventory, and a whole task leaf was
written to decide whether to fix that at the table or at the classifier.

**The current classifier already refuses it.** Applying `classified_table_kind`'s two
`SignalDescription` paths to every persisted table already carrying that kind:

| stratum | still typed `signal_description` | would **not** be | declarations minted by the latter |
| --- | ---: | ---: | ---: |
| current (proof-carrying) | 118 — 107 by caption, 11 by header | **0** | 0 |
| legacy (inspection-only) | 360 — 242 by caption, 118 by header | **124** | **598** of that stratum's 2,085 |

## What refuses the matrix

Two conditions, and neither alone is enough — measured, not assumed.

1. **Whole-label role equality.** Every closed role must match a header's entire normalized label, so a
   matrix that qualifies each role with its own subject offers none: `Mode Name` is not `Name`,
   `Bus Width` is not `Width`. Relaxing `header_has_role` to word containment leaves the matrix refused.
2. **A signal noun must appear.** Neither a header nor the caption (`Table 4 - Bus Speed Modes`) contains
   `signal`/`port`/`pin`, and there is no `Direction` column, so `has_direction || (has_explicit_signal
   && has_width)` fails. Forcing `header_names_signals` to `true` also leaves the matrix refused.

Only **both** relaxations together admit it. That pair is the observed RED for
`a_matrix_that_qualifies_every_role_with_its_subject_is_not_a_signal_table`, which also carries a GREEN
control — the same fixture with unqualified roles must still be a signal table — so the assertion cannot
pass by refusing everything.

## Why this matters beyond one table

A legacy declaration count is not a statement about the extraction. **598 of the legacy stratum's 2,085
declarations come from tables the current producer would never hand to the declaration reader at all** —
register summaries (`Name | CRn | Opc1 | CRm | Opc2 | Width | Description`), Extended-CSD field tables,
and matrices. Read a legacy number as a fact about files, not about behaviour
(`[[declared-spelling-is-the-document-spelling]]`, the same principle on a different field).

Two consequences were load-bearing for their trees:

- `SIGNAL-DECLARATION-ROW-DROP.2c` was deferred pending a guard against the 4 phantom rows an
  enumerated-width reading would newly admit. All 4 are eMMC `table_0020`. The current producer never
  reaches them, so the deferral reason is gone and `.2c`'s remaining population is three Avalon rows —
  `readdata`, `writedata`, `byteenable` — all real.
- Of the 11 legacy `phrase` name-cell declarations, 8 are in tables refused here. The 3 that survive are
  all AXI-H `table_0036` (`ARSIZE bus`, `ARBURST , INCR`, `ARLOCK zeros,`), a scrambled table whose
  cells drifted across columns — and every one of them names a **real** signal with a fragment of its
  neighbour fused on. The surviving phrase population is entirely real wires
  (`[[declared-population-is-not-the-candidate-row-population]]`).

## The check to run first

Before treating any persisted field as a defect, ask which producer wrote it. The stratum boundary is
`EVIDENCE_IR_SCHEMA_VERSION`, and the corpus splits 27 proof-carrying / 51 legacy. A finding that
appears only in the legacy stratum is a fact about files.
