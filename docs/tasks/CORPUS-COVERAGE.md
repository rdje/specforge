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

48 of 56 real chip-spec refreshes are complete.

- Stage coverage: 80 SourceIR / 20 normalized / 80 EvidenceIR / 79 downstream chains.
- Emitted ISF: 59/59 current targets pass FSMGen strict validation.
- Completed program lanes: `.0` build-out, `.1` stage-staleness validation, and `.3` lifecycle reconciliation.
- Active program lane: `.2` current-binary corpus refresh, with eight real documents remaining.
- Blockers: none.

## Current Frontier

No eligible product leaf.

Containment is closed. The next clean product slice must create and own `CORPUS-COVERAGE.2.49`, select one of
the eight remaining real documents from current corpus evidence, and record its exact source and stale-chain
boundary before any ingest or artifact mutation.

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
| `2026-08-10` | refresh `.2.48` | 48 done / eight remaining; 80/20/80/79; 59/59 strict-clean |
| `2026-08-10` | containment source lock | 2,308 lines / 277,636 bytes / seven exact regions / 48 formal routes |

## Commit Log

| Unit | Durable evidence |
| --- | --- |
| `CORPUS-COVERAGE.2.48` | latest completed product refresh |
| `CORPUS-TASK-EVIDENCE-CONTAINMENT` | bounded root, semantic evidence parts, and exact provenance |
