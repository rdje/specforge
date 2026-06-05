# CONSTRAINT-DRIVE-LEVEL-RECALL: extract "drive <signal> LOW/HIGH" as a value constraint

## Metadata

- Tree ID: `CONSTRAINT-DRIVE-LEVEL-RECALL`
- Status: `done` (CLOSED `2026-06-06`; `.1`)
- Roadmap lane: `R16`/`R15e` (extraction quality / recall)
- Created: `2026-06-06`
- Parent context: `EVAL-DOCUMENT-RECALL.2`'s `missed_gold_facts` pinpointed the single missed gold
  constraint on the real APB spec — `PSTRB|must_be_low` from *"the Requester must drive all bits of
  PSTRB LOW."* The relation (Requester→PSTRB) was extracted; the value constraint was not.

## The gap + fix

The sentence is a `normative_statement` (not a `SignalValueConstraint`), and the dynamic constraint
extractor keyed only on doc-**discovered enum values** (`must be <value>`) — so an active
`drive <signal> LOW/HIGH` construction, whose value is a **logic level** (the universal "how",
LOGIC-LEVEL-BOUNDARY), was missed. Fix: `logic_level_binding_kind_from_text` in `extract_dynamic_signal_constraints`
recognizes a logic-level binding → `MustBeHigh`/`MustBeLow`, **gated three ways against
over-generation**:

1. **whole-word** bind verbs (`drive`/`driven`/`set`/`tied`/`held`/`pulled`/`forced`) — so `set`
   does NOT match the substring in `reset`;
2. a **proximity window** (the level must be the verb's object within ~6 words, not a distant
   condition like *"… driven correctly every cycle in which X is True"*);
3. **alphabetic word forms only** (`high`/`low`/`true`/…, never numeric `1`/`0` = bit indices) and
   `"active <level>"` polarity descriptions excluded.

Also removed the early-return on empty `discovered_values` (the logic-level path finds constraints
even when a doc declares no enums).

## Verification

Passed (`2026-06-06`) — re-extracted the real APB **fresh**: constraint document-level recall
**5/6 → 6/6** (`PSTRB must_be_low` now found), with **only +1 constraint** (13→14) — the 3 candidate
false positives (`PCLK must_be_high` from a distant condition, `PRESET`/`PRESETN` from the `reset`
substring + `active low` polarity) are correctly gated out. +1 regression test
(`drive_signal_level_binds_must_be_low_but_gates_false_positives`). Full `scripts/run_ci.sh` GREEN
(1272→1273).

## Task Tree

- ID: `CONSTRAINT-DRIVE-LEVEL-RECALL` · Status: `done` · Children: `.1`
- ID: `CONSTRAINT-DRIVE-LEVEL-RECALL.1` · Status: `done` · Goal: recognize logic-level bindings in the
  dynamic extractor (gated). Verification above.

## Changelog

- `2026-06-06`: Created + CLOSED — closed the 1 missed APB gold constraint (drive-level recall);
  APB constraint recall 5/6→6/6, no over-generation. (Backlog item: Actionable/discrete follow-up #1.)
