# Generic section-phase retirement measurement — `CORPUS-COVERAGE.2.43a.i`

Owning leaf: `CORPUS-COVERAGE.2.43a.i` (PROBE/CODE/DATA/DOC).
Date: `2026-08-10`.

## Question and boundary

Child `.2.43a` proved that sentence sequencing words cannot authorize a whole section as a generic semantic
phase. Its exact positive census left 3,180 records admitted by the historical title vocabulary. This follow-up
asks whether any universal heading grammar makes that legacy `SemanticIR.phases[]` surface semantically useful.

The separate `transaction_phases[]` recognizer is not changed here. It records explicit `<qualifier> phase`
prose with statement provenance and declared-signal membership. A distinct pending leaf owns the qualifier
grammar's precision after this repair commits.

## Historical producer and consumer

Git history traces both halves to the project's first executable IR passes:

- initial SemanticIR commit `f8e42eaa9` made one `PhaseRecord` for every admitted section and summarized it only
  as `semantic phase derived from section <title>`;
- initial IntentIR commit `b128fd58` turned each summary into a behavior assigned to **every** retained actor and
  added `participate in <summary>` to every actor whose statement or section support overlapped.

Neither commit records a stronger phase type, phase name, ordering, entry/exit condition, actor role, signal
membership, or executable effect. `PhaseRecord` still carries only id, synthetic summary, and provenance. The
later typed `TransactionPhaseRecord` is the structural surface for actual named protocol phases.

## Exact title classification

The census replicates production ASCII-alphanumeric word boundaries over all 79 retained SemanticIR artifacts.
The 3,180 title-authorized records occur in 69 documents. Phrase memberships overlap:

| Title phrase | Records | Documents |
| --- | ---: | ---: |
| `phase` | 6 | 3 |
| `sequence` | 54 | 24 |
| `flow` | 148 | 32 |
| `timing` | 106 | 36 |
| `transaction` | 270 | 25 |
| `handshake` | 31 | 10 |
| `operation` | 207 | 40 |
| `mode` | 246 | 36 |
| `startup` | 0 | 0 |
| `shutdown` | 7 | 1 |
| `reset` | 870 | 41 |
| `request` | 283 | 42 |
| `response` | 304 | 38 |
| `transport` | 48 | 12 |
| `state` | 190 | 37 |
| `write` | 392 | 35 |
| `read` | 323 | 37 |

The largest exact title is `Reset value` at 720 records. Other frequent titles include `Integration Mode Control
Register, ITCTRL` (27), `Flow control result` (24), `Timing diagrams` (24), `Operation:` (12), `Transaction
Structure` (12), `Transport DVSEC` (11), and named read/write/mode registers. These are topics, definitions,
register metadata, results, or diagrams—not one canonical phase apiece.

Only six records literally contain the word `phase`:

| Document/heading | Records | Classification |
| --- | ---: | --- |
| CAN `PHASE SEG1, PHASE SEG2` | 2 | one heading names two bit-time segments; not one section phase |
| CAN `PHASE ERROR of an edge` | 2 | measurement/error definition, not a transaction phase |
| SWD `Data transfer phase` | 1 | genuine phase topic; typed `transaction_phases[]` already captures `data` |
| Wishbone `Phase (Bus Cycle)` | 1 | glossary definition, not a concrete behavioral phase |

Even the sole clean heading contributes only the tautological section summary; it supplies no executable or
actor-specific meaning. Therefore narrowing the title vocabulary to literal `phase` would keep five false or
multi-phase records to preserve one redundant record.

## Downstream two-variant experiment

For every retained document, the task-local deterministic probe created two same-volume SemanticIR variants from
the same original artifact:

1. **title-authorized** — keep exactly the records admitted by the current post-`.2.43a` title predicate;
2. **no-generic** — set only legacy `phases[]` to empty.

It ran `intent --dry-run` and `adapt --target isf --dry-run` for both variants with repaired `.2.43a` release
binary `c61f05798ebfc9cfc5a690e1f9a09cb93943dcffbfcea4f5b05cfceaeb23b117`. All task data, temporary files, and
caches remained below the repository root on the SSD volume.

| Effect of removing title-authorized generic phases | Result |
| --- | ---: |
| phase records removed | 3,180 across 69 documents |
| deduplicated synthetic behaviors removed | 2,238 across all 69 documents |
| pure-inferred actors no longer preserved | 48 across 26 documents |
| actor responsibilities removed | 2,257 across 63 documents |
| documents with adapter renderability change | 0 |
| documents with adapter lowering-status change | 0 |
| documents with emitted signal/transaction/rule count change | 0 |

Three adapter source hashes changed, but only at the module-name fallback. With no structurally selected
initiator, removal of the first phase-preserved phantom changes CoreSight SDC-600 `agent`→`channel`, Generic
Flash Bus `device`→`manager`, and the project readme fixture `agent`→`channel`. Signal, transaction, rule,
constant, enum, and storage counts remain identical. This improves label grounding; it does not remove executable
behavior.

## Decision and compatibility contract

There is no smallest defensible title grammar for the old projection. The record is a section-topic wrapper, not
a typed phase, and its consumer fabricates a document-wide actor assignment from no actor-role evidence. The
repair therefore retires the authority path rather than replacing one keyword list with another:

- current SemanticIR producers serialize the schema-stable `phases` field as an empty collection;
- `PhaseRecord` and the field remain deserializable and round-trippable so old artifacts stay auditable;
- current IntentIR builders ignore populated legacy phase records, preventing stale artifacts from restoring
  synthetic behavior or pure-inferred actors;
- `transaction_phases[]`, named transactions, gates, contracts, constraints, and all other typed surfaces are
  unchanged.

This is a universal structural decision: no document, vendor, protocol, title, section number, or phase-name
allowlist/denylist appears in production code.

## Post-repair equivalence

Rebuilt release binary `834e335a0229371edad9ffadb23bb019ad5d6e3086fbb79059e64118096ed3d3` replayed the same 79
title-authorized/no-generic pairs. All 79 IntentIR dry-run artifacts are byte-identical between variants, and all
79 adapter summaries and rendered source hashes are identical. Thus old populated `phases[]` remains loadable
but has zero current semantic authority, exactly matching newly produced empty collections.

Verification passes 404 SemanticIR tests, 50 IntentIR tests including the legacy-artifact compatibility case,
warning-deny Clippy, release compilation, all nine provider-free WIRE/I2C/SWD datasets, KG 156/156, full CI at
1,789 pass/five ignored, all 66 current emitted ISFs through FSMGen strict with zero diagnostics, mdBook tests
and build, the six-doctrine driver, persisted-path census, and project-data locality.
