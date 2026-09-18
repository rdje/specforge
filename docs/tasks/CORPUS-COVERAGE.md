# CORPUS-COVERAGE: build every ingested doc through to IntentIR/.isf + keep downstream stages non-stale

## Metadata

- Tree ID: `CORPUS-COVERAGE`
- Status: `active`
- Roadmap lane: `R15e`/`R16` corpus digestion
- Updated: `2026-08-11`

## Goal

Build every real ingested chip specification through the current SourceIR → EvidenceIR → SemanticIR → IntentIR →
ISF pipeline, keep completed chains non-stale, and record extraction gaps without fabricating unsupported intent.

## Non-Goals

- Do not count cache retention as completion; completion means a current-binary EvidenceIR and downstream refresh.
- Do not invent interfaces, signals, behavior, or emitted targets for documents that lack grounded authority.
- Do not select or start a new document without a separately owned task-tree leaf.

## Current State

52 of 57 real chip-spec refreshes are complete. A refresh is a **provenance** fact — the document's source
is repository-local and it was rebuilt with the then-current binary — and `.5` measured that it is not a
currency fact: **31 of those 52 are no longer loadable**, because the EvidenceIR schema bumped to 3 on
`2026-08-13` after their refreshes landed. The current figure is `ADR 0048`'s: **27 of 78 load canonically**,
and 6 of those 27 were never in this cohort at all, having been ingested from `corpus/` and so never needing
a refresh.

- Stage coverage: 78 SourceIR / 24 normalized / 78 EvidenceIR / 78 downstream chains, all measured current by
  the `CHAIN-CURRENCY` gate — *current* there means each stage still replays from its persisted upstream, which
  is stage-local reproducibility and not canonical loadability (`ADR 0048`). The measurable evidence-stage
  population grows by exactly one per refresh.
- Emitted ISF: 44/44 current targets pass FSMGen strict validation. The population fell from 57 because 14
  documents' stale heuristic interfaces collapsed to zero under current authority and now block honestly.
- Completed program lanes: `.0` build-out, `.1` stage-staleness validation, and `.3` lifecycle reconciliation.
- Active program lane: `.2` current-binary corpus refresh, with five real documents remaining. Completed lane
  `.4` frontier census integrity derived that count, restored the document a hand-carried decrement had lost,
  and made the census a registered doctrine so it can no longer drift.
- Blockers: none. `.2.52` measured a general defect it deliberately did not repair inside a data refresh:
  `SEMANTIC-EMPTY-CATALOG-FILTER` owns it.

## Current Frontier

No eligible product leaf.

`CORPUS-COVERAGE.4` is complete in the `frontier-census-integrity` part, and the 57-document cohort it derived is
now gated by the `CORPUS-FRONTIER` doctrine rather than carried, so the split below is re-derived on every run
instead of decremented by hand.

`CORPUS-COVERAGE.2.52` is complete in `refreshes-51-56`, so the census, the retention declaration, and this root
all read 52 refreshed with five remaining. The next product slice selects `opencapi_3_0_transaction_layer_28jan2020`
at 774 elements, the smallest of the five the `CORPUS-FRONTIER` gate now declares. `refreshes-49-56` is closed to
further product writes at the `.2.50a` boundary, and `refreshes-51-56` holds two of its six refreshes.

`.2.52` also measured a general defect and routed it to its own tree rather than repairing it inside a data
refresh: `SEMANTIC-EMPTY-CATALOG-FILTER` owns the `semantic.rs` empty-declared-set branch that disables the
conditional-rule and signal-constraint grounding filters exactly when a document has no declared signals.

## Detailed task evidence

[Complete task evidence](corpus-coverage/INDEX.md)

The index routes every established task id to one bounded semantic part. Its manifest authenticates part metrics,
route ownership, and exact source regions; the archive capsule preserves the complete pre-containment source.

## Slice Transaction

Every future refresh atomically updates this current root, exactly one active semantic part, the index/manifest,
and every genuinely affected current/book/fact surface. Completed legacy parts stay sealed. The active part splits
at a refresh boundary before its next write would reach rollover. Run the corpus task-evidence contract, all
doctrines, and the risk-proportionate `COMMIT.md` gates before committing each leaf.

## Verification Log

| Date | Boundary | Result |
| --- | --- | --- |
| `2026-08-11` | refresh `.2.52` | 52 done / five remaining; 78/24/78/78; 44/44 strict-clean; two false acronym signals retired and a 29-document unfiltered-rule defect measured and routed |
| `2026-08-11` | gate `.4.1` | `CORPUS-FRONTIER` registered gate-tier; self-test 10/10; two live negatives on the real corpus fail closed; driver 7 executed / 8 registered PASS |
| `2026-08-11` | census `.4.0` | derived 57 cohort = 51 refreshed + six remaining; the lost document is `nvme_base_specification_2_0a_2021_07_26`; no artifact moved |
| `2026-08-11` | refresh `.2.51` | 51 done; 78/23/78/78; 44/44 strict-clean (its "five remaining" is superseded by `.4.0`) |
| `2026-08-10` | refresh `.2.50` | 50 done / six remaining; 80/22/80/79; 57/57 strict-clean |
| `2026-08-10` | repair `.2.50a` | 26 false pre-bind subjects measured; eight removed in three rebuilt cascades; 58/58 strict-clean |
| `2026-08-10` | refresh `.2.49` | 49 done / seven remaining; 80/21/80/79; 58/58 strict-clean |
| `2026-08-10` | refresh `.2.48` | 48 done / eight remaining; 80/20/80/79; 59/59 strict-clean |
| `2026-08-10` | containment source lock | 2,308 lines / 277,636 bytes / seven exact regions / 48 formal routes |

## Commit Log

| Unit | Durable evidence |
| --- | --- |
| `CORPUS-COVERAGE.2.52` | latest completed product refresh |
| `CORPUS-COVERAGE.4.1` | the `CORPUS-FRONTIER` doctrine gating the derived census |
| `CORPUS-COVERAGE.4.0` | derived frontier census and the restored cohort document |
| `CORPUS-COVERAGE.2.51` | preceding completed product refresh |
| `CORPUS-COVERAGE.2.50` | earlier completed product refresh |
| `CORPUS-COVERAGE.2.49` | earlier completed product refresh |
| `CORPUS-COVERAGE.2.48` | earlier completed product refresh |
| `CORPUS-TASK-EVIDENCE-CONTAINMENT` | bounded root, semantic evidence parts, and exact provenance |
