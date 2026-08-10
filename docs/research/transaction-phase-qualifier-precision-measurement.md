# Transaction-phase qualifier precision — `CORPUS-COVERAGE.2.43a.ii`

Owning leaf: `CORPUS-COVERAGE.2.43a.ii` (PROBE/CODE/DOC).
Date: `2026-08-10`.

## Trigger and question

The retirement audit for generic section phases incidentally found three typed
`transaction_phases[]` records that were not phase names:

- Wishbone `called`, from `individual cycles (called phases)` and `transfers ... are called phases`;
- CAN `edge`, from `compensate for edge phase errors`;
- CAN `positive`, from `positive PHASE ERROR`.

The typed producer took the normalized token immediately before an exact `phase` / `phases` token. Its qualifier
stoplist rejected function words, counts, ordinals, transaction-head nouns, gerunds, and some punctuation, but it
never established that `phase` was the head of a phase phrase. This leaf asks which of all retained records are
actual named phases and whether one universal phrase boundary can remove the rest without narrowing real phase
provenance or signal membership.

## Two baselines, kept separate

The 79 retained SemanticIR artifacts are mixed-vintage. Therefore the audit reports both the requested retained
snapshot and a deterministic current-rule replay over all 79 matching EvidenceIR inputs.

| View | Before repair | Valid | False | Why it matters |
| --- | ---: | ---: | ---: | --- |
| retained `transaction_phases[]` snapshot | 82 | 59 | 23 | exact task acceptance population |
| current-rule replay from unchanged EvidenceIR | 101 | 70 | 31 | actual producer behavior on a complete rebuild |

The replay has 19 candidates absent from the retained snapshot: eleven are valid phases in older artifacts that
had not materialized typed phases, while eight are additional false positives. This is artifact staleness, not a
rule-dependent addition.

## Exact 82-record classification

The 59 valid retained records occur in 23 documents. Twenty-nine are protocol/transfer phases and 30 are named
link, state, command, or process phases. Counts include document variants:

| Valid class | Phase names (record count) | Records |
| --- | --- | ---: |
| protocol / transfer | `access` 1, `acknowledge` 1, `address` 7, `data` 12, `nodata` 1, `read` 1, `response` 1, `setup` 1, `turnaround` 1, `write` 3 | 29 |
| named state / lifecycle / process | `activation` 2, `aligned` 1, `capture` 1, `compliant` 3, `configuration` 3, `discovery` 3, `equalization` 3, `esm` 3, `execution` 1, `identification` 1, `initialization` 1, `invalidation` 1, `link` 1, `locked` 1, `rate` 1, `startup` 1, `transition` 1, `unaligned` 1, `validation` 1 | 30 |

The 23 false retained records are exact and exhaust the snapshot remainder:

| False phase | Document(s) / records | Actual phrase role |
| --- | --- | --- |
| `called` | Wishbone / 1 | passive naming predicate, not a phase name |
| `different` | USB 3.2 / 1 | quantifies three phases that are named later |
| `dynamic`, `static` | OpenCAPI PHY mechanical / 2 | phase-tolerance measurement modifiers |
| `edge`, `positive` | CAN / 2 | modifiers inside `phase error(s)` |
| `introduces` | USB 3.2 / 1 | verb before `phase and frequency offset` |
| `jitter` | eMMC / 1 | comma-separated `jitter, phase noise` |
| `key` | GIC-400 / 1 | qualitative modifier in `key phases of ...` |
| `lanes` | three CCIX variants / 3 | table/list adjacency: `Lanes, Phase 2` |
| `limited` | USB 3.2 / 1 | modifier inside `phase tracking device` |
| `momentary` | eMMC / 1 | phase measurement from clock to outputs |
| `random` | eMMC / 1 | modifier inside `phase relation` |
| `recovery` | OpenCAPI PHY signaling / 1 | comma adjacency before `phase lock loop` |
| `see` | USB 3.2 / 1 | glossary verb before `Phase Locked Loop` |
| `sequence` | DTI / 1 | table column header `Sequence phase | Actions` |
| `significant` | AMBA LPI / 1 | modifier inside `phase difference` |
| `stable` | CoreSight SoC-600 / 1 | descriptive waveform intervals, not a named phase |
| `total` | USB 3.2 / 1 | organization name `Total Phase` in contributor data |
| `triangular` | OpenCAPI PHY signaling / 1 | modifier inside `phase modulation` |
| `what` | DTI / 1 | interrogative determiner in `what phase` |

All 23 false records have empty `signal_set`. Every payload of every retained valid record—phase id, name,
supporting statement ids, signal set, and confidence—replays unchanged.

## Current-rule replay additions

The current-rule replay adds eleven valid records that mixed-vintage artifacts did not contain:

- APB-d: `setup`, `access`;
- AXI/ACE-h: `address`, `data`;
- AMBA trace bus: `address`, `data`;
- CCIX r1.0a evaluation: `compliant`, `configuration`, `discovery`, `equalization`, `esm`.

It also adds eight false candidates: README `access`, `facts`, `records`, `semantics`, and `signal`; NVMe `entry`
from `Completion Queue Entry Phase Tag`; AXI/ACE `integration` from the design-flow integration phase; and CCIX
r1.0a `lanes` from the same `Lanes, Phase 2/3` adjacency. Thus a complete current-rule rebuild is 101 candidates,
not 82. The repaired replay emits 70 valid records across 27 documents and removes all 31 false candidates.

## Root cause

Git history locates the qualifier gate and one-pass collection in commit `bb83a32b0` (`2026-06-16`), with
signal-set membership added in `4edb2c1fa` later that day. `derive_phase_name` examined only the token before the
head, and `build_transaction_phases` admitted the candidate immediately. The gate could reject a bad qualifier,
but it had no representation of the head's following token, local clause, predicate, temporal governor, or naming
construction. Consequently it could not distinguish `<name> phase` from `<modifier> phase error`, a passive verb,
or flattened table adjacency. The original wire-only tuning did not measure that distinction corpus-wide.

## Selected universal boundary

Recognition remains name-agnostic and now has two stages:

1. The qualifier gate rejects comma boundaries, unmatched opening delimiters, interrogative determiners, and
   non-naming comparison adjectives in addition to its existing universal grammar.
2. At least one occurrence of a candidate name must positively use the phrase as a phase: a standalone or
   heading phrase, a numbered phase, a local phase predicate, a temporal governor, a copular/possessive or
   explicit naming construction, or the bounded `indicates ... phase busy by ...` object-complement form.

The producer collects candidates in statement order, marks names with positive authority, and emits only marked
names. Once a name is authorized, every grammatical occurrence remains provenance and continues to contribute
declared-signal membership. This two-pass shape is what preserves AHB `address phase signal` rows even when the
separate sentence that authorizes `address` is `During the address phase ...`.

There is no document, vendor, protocol, section, signal, exact valid-phase, or exact invalid-phase list in the
rule. The lexical sets are English function words, local predicates, temporal governors, and naming/reporting
verbs.

## Paired controls and downstream result

Focused controls retain:

- APB `Setup phase ... occurs` and `During an Access phase`;
- AHB/AXI `During the address phase`, `transaction has an address phase`, and `address phase signal` provenance;
- SWD `entering the acknowledge phase`, `response phase`, and `turnaround phase`;
- numbered link/equalization phases, USB alignment headings, and eMMC transition-phase busy reporting.

They reject Wishbone passive `called phases`, CAN `edge phase errors` / `positive PHASE ERROR`, measurement noun
compounds, the DTI table header, and comma/table adjacency.

An authenticated release-binary replay over all 79 downstream-ready EvidenceIR inputs reports:

- 101→70 phase records and exactly 31 removals;
- zero changed provenance lists among the 70 retained names;
- zero non-empty signal sets among the 31 removed names, verified by a name-substitution authority probe that
  preserves each original statement's other tokens;
- zero differences across all affected old-rule-populated versus repaired IntentIR variants. Empty-signal-set
  phases never enter `phase_groups`, so adapter and executable surfaces are unchanged by construction.

The repaired release binary hashes to
`463a79e3fa7837f154692a84d46a50dbaa86ff1bae6d5866953b55e99db06510`.

## Verification

Completed for signoff:

- 12 phase-focused tests;
- all 407 SemanticIR module tests;
- warning-deny Clippy across all `specforge` targets;
- exact retained and deterministic producer replays described above;
- all nine WIRE/I2C/SWD provider-free datasets and KG bench 156/156;
- full composed CI: 1,791 passed / five ignored;
- 66/66 retained ISF files accepted by real FSMGen strict with zero diagnostics;
- mdBook doctests/build, all six doctrines, persisted-path checks, and project-data locality.
