# Generic whole-statement gate retirement measurement — `CORPUS-COVERAGE.2.43b`

Owning leaf: `CORPUS-COVERAGE.2.43b` (PROBE/CODE/DOC).
Date: `2026-08-10`.

## Question and boundary

The OpenCAPI 3.0 Ready Definition refresh retained a product-compliance paragraph as one `SemanticIR.gates[]`
record because it said that a manufacturer shall assert compliance *when* host requirements have been met. The
record had no related interface, parsed antecedent, consequent, effect, actor role, or executable action, and the
same statement already survived as a contract, invariant, and assertion. This leaf asks whether the generic gate
surface has any independently defensible semantic or downstream authority.

The separately known precision risk in `conditional_rules[]` is outside this leaf. This measurement distinguishes
that typed record family and `temporal_rules[]` from the provenance-only generic gate collection; it neither
changes nor describes those typed surfaces as perfectly precise.

## Historical producer and consumers

Git history traces the unchanged path to the first executable IR passes:

- initial SemanticIR commit `f8e42eaa9` copied every retained statement containing word-bounded `if`, `when`,
  `unless`, `while`, `after`, `before`, or `until` into a `GateRecord`;
- initial IntentIR commit `b128fd58` turned every gate's complete source sentence into a behavior assigned to
  every retained actor;
- the same IntentIR pass made an `[interface-coupled rule]` constraint when the record carried a related
  interface, again assigning it to every interface actor.

`GateRecord` carries an id, one whole `condition` string, statement/section provenance, and optional interface
co-mentions. It cannot represent the condition/action split required of a canonical gate. In particular, it has
no antecedent, consequent, effect, signal/value operands, actor role, or executable operation.

## Exact retained-artifact census

The task-local deterministic probe replicated the production word-boundary cues over all 79 retained SemanticIR
artifacts. Cue memberships overlap because one sentence can contain several terms.

| Census item | Result |
| --- | ---: |
| documents with generic gates | 77 / 79 |
| generic gate records | 27,168 |
| related interface present | 5,974 |
| no related interface | 21,194 |
| duplicate of contract | 5,962 |
| duplicate of invariant | 11,851 |
| duplicate of assertion | 1,223 |
| duplicate of any stronger semantic surface | 12,877 |
| overlap with typed conditional rule | 2,552 |
| overlap with typed temporal rule | 139 |
| overlap with either typed rule family | 2,676 |
| no typed-rule overlap | 24,492 |
| neither stronger-surface duplicate nor typed-rule overlap | 14,115 |

| Word-bounded cue | Records |
| --- | ---: |
| `if` | 12,981 |
| `when` | 11,916 |
| `unless` | 523 |
| `while` | 1,223 |
| `after` | 2,963 |
| `before` | 2,081 |
| `until` | 1,098 |

Frequent section anchors include `Note` (340), `Approved Approved` (277), `Bit descriptions` (238),
`Configurations` (197), `Attributes` (117), and `Additional information` (116). A temporal conjunction therefore
does not distinguish a protocol gate from an explanatory note, field description, administrative condition, or
ordinary sequencing prose.

## Downstream two-variant experiment

For every retained document, the task-local probe made two same-volume variants from the same SemanticIR:

1. **populated** — preserve every retained legacy `gates[]` record;
2. **empty** — set only `gates[]` to an empty collection.

It ran `intent --dry-run` and `adapt --target isf --dry-run` for both variants with pre-repair release binary
`834e335a0229371edad9ffadb23bb019ad5d6e3086fbb79059e64118096ed3d3`. All generated inputs, outputs, and caches
stayed below `.cache/task-work/CORPUS-COVERAGE.2.43b/` on the repository's SSD volume.

| Effect of removing generic gate authority | Result |
| --- | ---: |
| deduplicated whole-sentence behaviors removed | 21,206 |
| all-actor behavior assignments removed | 642,401 |
| generic interface-coupled constraints removed | 5,974 |
| interface assignments removed | 366,087 |
| IntentIR artifacts changed | 76 / 79 |
| adapter manifests changed | 2 / 79 |
| rendered ISF sources changed | 0 / 79 |
| renderability decisions changed | 0 / 79 |
| lowering statuses changed | 0 / 79 |
| executable signal/transaction/rule counts changed | 0 / 79 |

The two manifest-only changes make absence more honest:

- AArch64 External Debug Guide loses 56 gate-only behaviors and gains one no-explicit-behavior residual; its 55
  independent constraints and blocked empty source hold.
- Cortex-A76 Software Optimization Guide loses 23 gate-only behaviors and 13 generic gate constraints, leaving
  15 independent constraints and one additional no-explicit-behavior residual. Its rendered source is identical.

The Ready Definition's one gate contributes no unique Intent behavior or constraint because its statement is
already represented by stronger surfaces and has no related interface.

## Decision and compatibility contract

There is no smaller cue grammar that can make the record canonical: changing the keyword set cannot add the
missing condition/effect structure or justify assignment to every actor. The repair retires authority rather
than substituting another lexical list:

- new SemanticIR artifacts serialize the schema-stable `gates` field as an empty collection;
- `GateRecord` and populated old artifacts remain loadable and round-trippable for provenance audit;
- current IntentIR ignores populated legacy gates and cannot restore the all-actor behavior/constraint projection;
- EvidenceIR statements, invariants, contracts, assertions, and IntentIR constraints remain independent;
- typed `conditional_rules[]` and `temporal_rules[]` remain independent and unchanged.

The decision is universal: production code contains no document, vendor, protocol, section, sentence, signal,
or cue allowlist/denylist for this retirement.

## Post-repair equivalence

Rebuilt release binary `5a43f1b5894ba98ffd332f56b5e83823b02efa475b06d5743d574adc54d7c11f`
replayed the same 79 populated/empty pairs. All 79 IntentIR dry-run artifacts are byte-identical between variants,
and all 79 adapter manifests and rendered source hashes are identical. Thus populated old `gates[]` remains
loadable but has zero current semantic authority, exactly matching newly produced empty collections.

Focused compatibility, handshake, legal-condition, and administrative-workflow controls pass, as do all 407
SemanticIR tests, all 49 IntentIR tests, warning-deny Clippy, and release compilation. All nine provider-free
WIRE/I2C/SWD datasets meet their declared gates; KG is 156/156; full CI passes 1,788 tests with five ignored; all
66 emitted ISFs pass FSMGen strict with zero diagnostics; mdBook tests/build, six doctrines, 2,008-artifact
persisted-path census, and project-data locality pass.

The required `DEVELOPMENT_NOTES.md` record crossed its registered 90% line rollover. The exact plan in
`corpus-coverage-2.43b-development-notes-rollover-plan.jsonl` authenticated clean opening commit `2a6355b3`,
sealed 24 oldest post-migration records into `segment-0003-2026-08-10.md`, updated the manifest/index, installed
the live root last, and left it warning-safe at 61 records / 1,303 lines. The focused checker proves the archive
chain remains lossless.
