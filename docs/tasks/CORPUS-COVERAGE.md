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

49 of 56 real chip-spec refreshes are complete.

- Stage coverage: 80 SourceIR / 21 normalized / 80 EvidenceIR / 79 downstream chains.
- Emitted ISF: 58/58 current targets pass FSMGen strict validation.
- Completed program lanes: `.0` build-out, `.1` stage-staleness validation, and `.3` lifecycle reconciliation.
- Active program lane: `.2` current-binary corpus refresh, with seven real documents remaining.
- Blockers: none.

## Current Frontier

Active product leaf: `CORPUS-COVERAGE.2.50`.

Child `.2.50a` is complete: a passive obligation's subject must now be named before its `must/shall be|remain`
lead, which removes 26 measured false constraints corpus-wide and rebuilds the three affected rebuildable
cascades. The parent now re-ingests the OpenCAPI Data Link Layer specification from the repaired release.

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
| `2026-08-10` | repair `.2.50a` | 26 false pre-bind subjects measured; eight removed in three rebuilt cascades; 58/58 strict-clean |
| `2026-08-10` | refresh `.2.49` | 49 done / seven remaining; 80/21/80/79; 58/58 strict-clean |
| `2026-08-10` | refresh `.2.48` | 48 done / eight remaining; 80/20/80/79; 59/59 strict-clean |
| `2026-08-10` | containment source lock | 2,308 lines / 277,636 bytes / seven exact regions / 48 formal routes |

## Commit Log

| Unit | Durable evidence |
| --- | --- |
| `CORPUS-COVERAGE.2.49` | latest completed product refresh |
| `CORPUS-COVERAGE.2.48` | latest completed product refresh |
| `CORPUS-TASK-EVIDENCE-CONTAINMENT` | bounded root, semantic evidence parts, and exact provenance |
