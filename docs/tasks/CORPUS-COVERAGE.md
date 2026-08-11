# CORPUS-COVERAGE: build every ingested doc through to IntentIR/.isf + keep downstream stages non-stale

## Metadata

- Tree ID: `CORPUS-COVERAGE`
- Status: `active`
- Roadmap lane: `R15e`/`R16` corpus digestion
- Updated: `2026-08-10`

## Goal

Build every real ingested chip specification through the current SourceIR → EvidenceIR → SemanticIR → IntentIR →
ISF pipeline, keep completed chains non-stale, and record extraction gaps without fabricating unsupported intent.

## Non-Goals

- Do not count cache retention as completion; completion means a current-binary EvidenceIR and downstream refresh.
- Do not invent interfaces, signals, behavior, or emitted targets for documents that lack grounded authority.
- Do not select or start a new document without a separately owned task-tree leaf.

## Current State

51 of 57 real chip-spec refreshes are complete.

- Stage coverage: 78 SourceIR / 23 normalized / 78 EvidenceIR / 78 downstream chains, all measured current by
  the `CHAIN-CURRENCY` gate. The measurable evidence-stage population grows by exactly one per refresh.
- Emitted ISF: 44/44 current targets pass FSMGen strict validation. The population fell from 57 because 14
  documents' stale heuristic interfaces collapsed to zero under current authority and now block honestly.
- Completed program lanes: `.0` build-out, `.1` stage-staleness validation, and `.3` lifecycle reconciliation.
- Active program lane: `.2` current-binary corpus refresh, with six real documents remaining. Completed lane
  `.4` frontier census integrity derived that count, restored the document a hand-carried decrement had lost,
  and made the census a registered doctrine so it can no longer drift.
- Blockers: none.

## Current Frontier

No eligible product leaf.

`CORPUS-COVERAGE.4` is complete in the `frontier-census-integrity` part, and the frontier it derived — 57 cohort
documents = 51 refreshed + six remaining — is now gated by the `CORPUS-FRONTIER` doctrine rather than carried.

`CORPUS-COVERAGE.2.52` is owned and `in_progress` in `refreshes-51-56`: it selects
`opencapi_25gbps_phy_mechanical_spec_v10` at 760 elements, the smallest of the six, and pins its authenticated
same-device source and its exact six-file / 1,151,838-byte stale chain. No artifact has been mutated, so the
census still reads 51 refreshed and the gate is green. The next slice executes that refresh: guarded CPU ingest,
full current-binary cascade, per-stage delta attribution, and — in the same transaction — moving
`doctrine/corpus_frontier/census.json` and this root to 52/57 with five remaining, or `CORPUS-FRONTIER` fails
closed. `refreshes-49-56` is closed to further product writes at the `.2.50a` boundary.

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
| `CORPUS-COVERAGE.4.1` | the `CORPUS-FRONTIER` doctrine gating the derived census |
| `CORPUS-COVERAGE.4.0` | derived frontier census and the restored cohort document |
| `CORPUS-COVERAGE.2.51` | latest completed product refresh |
| `CORPUS-COVERAGE.2.50` | preceding completed product refresh |
| `CORPUS-COVERAGE.2.49` | earlier completed product refresh |
| `CORPUS-COVERAGE.2.48` | earlier completed product refresh |
| `CORPUS-TASK-EVIDENCE-CONTAINMENT` | bounded root, semantic evidence parts, and exact provenance |
