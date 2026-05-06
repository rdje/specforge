# Generated Artifacts

This chapter is the public map of what `specforge` writes and where those files fit in the project.

The most important rule is simple:

`generated/` is local execution state, not source code.

That rule is intentional. Real chip PDFs can produce large intermediate artifacts, and those artifacts can change whenever extraction, enrichment, validation, or learning logic improves.

## Generated artifact roots

- `generated/source_ir/<document_key>/source_ir.json`
- `generated/evidence_ir/<document_key>/evidence_ir.json`
- `generated/semantic_ir/<document_key>/semantic_ir.json`
- `generated/intent_ir/<document_key>/intent_ir.json`
- `generated/adapters/fsm/<document_key>/adapter.json`
- `generated/prior_memory/corpus_memory.json`

## Stage artifacts

The four main artifact roots mirror the four pipeline stages:

- `generated/source_ir/<document_key>/source_ir.json`
- `generated/evidence_ir/<document_key>/evidence_ir.json`
- `generated/semantic_ir/<document_key>/semantic_ir.json`
- `generated/intent_ir/<document_key>/intent_ir.json`

The `document_key` is the stable local key `specforge` derives for the input document.
For a PDF run, that key usually comes from the normalized file stem.

These artifacts are useful for different kinds of inspection:

- inspect `SourceIR` when table, section, page, figure, or backend normalization looks wrong
- inspect `EvidenceIR` when signal declarations, extracted statements, visual evidence, semantic hints, polarity evidence, or early KG relations look wrong
- inspect `SemanticIR` when actors, ports, connectivity, temporal rules, arbitration, conflicts, or residuals look wrong
- inspect `IntentIR` when you want the canonical product surface that validators, adapters, and prior learning consume

## Source-side sidecars

PDF ingest may also materialize source-side sidecars under the same document root.
Depending on the input and backend path, these can include:

- promoted markdown
- page metadata
- page images
- visual asset crops
- normalized backend dumps
- backend diagnostics

Those files are part of the local evidence trail.
They are useful for debugging and visual grounding, but they should not be treated as hand-authored project assets.

## Artifact lifecycle and cleanup

Generated artifacts are not meant to grow forever without supervision.

Two cleanup rules now define the normal lifecycle:

1. Re-ingesting the same PDF document key replaces that document's `normalized/` bundle atomically.
2. `specforge clean` provides an explicit local reclamation command for rebuildable generated state.

The first rule matters because old page images, visual crops, and backend dumps can otherwise survive across reruns even after the current normalization no longer references them.
`specforge` now stages PDF normalization into a sibling `normalized.staging/` tree and only swaps it into place after the backend succeeds.
That means:

- stale leftovers from earlier runs do not accumulate inside `normalized/`
- failed reruns do not destroy the last good normalized bundle

The second rule matters because some generated artifacts are intentionally heavy.
The default cleanup path is:

```bash
cargo run --manifest-path Cargo.toml -- clean
```

That dry-runs the heavyweight `generated/source_ir/*/normalized` bundles and reports reclaimable size.
Add `--execute` to delete them, use `--scope document --document-key <key>` when you intentionally want a cold rebuild of one document's whole generated stage tree, or use `--scope all-generated --execute` when you intentionally want to discard the whole local `generated/` root and rebuild everything later.

## Validation reports

`validate` writes deterministic validation reports next to the artifact being validated.
For example, validating an `IntentIR` artifact creates or refreshes the corresponding validation report for that stage.

The reports are useful because they separate:

- artifact content
- deterministic quality assessment
- tracked project-level projection

That separation matters.
An artifact can improve without the project snapshot being refreshed until `project-validation` is run.

## Rescan plans and approval boundaries

`project-validation` can also write:

- `generated/validation/rescan_plan.json`

That plan is a local replay queue for targeted rescans and stronger corroboration.
Compact review surfaces now expose more of that queue directly:

- the tracked live-status validation block shows each recommendation's replay-input kind chain plus a concise action summary
- `rescan-plan` dry-run output prints artifact path, extractor lane, full `replay_inputs`, `recommended_action`, related ids, and `automation_status` before any command hints; empty replay inputs, related ids, or command hints render as explicit `none` values

Its execution summaries may contain `promotion_review`, but that field is only a review-requirement descriptor.
It is not an approval artifact, and it does not authorize canonical IR mutation.

If a changed rescan outcome says an approval record is required, the current meaning is deliberately narrow: future human or policy review would be needed before any canonical mutation could exist.
While `rescan-plan` and `converge --rescan-plan <plan>` cannot mutate canonical IR, any future approval artifact stays local/generated by default.

A tracked approval-evidence artifact should only appear with an explicit canonical mutation workflow.
That future schema would need to record current-document evidence support, validation-delta direction, exact mutation scope, prior-memory non-authority, reviewer intent, artifact fingerprints, and replayable provenance.

## Adapter artifacts

Adapter output lives under:

- `generated/adapters/fsm/<document_key>/adapter.json`

Adapter artifacts are downstream products of `IntentIR`.
They should not define what the canonical document meaning is.
If an adapter needs meaning that is not present in `IntentIR`, the fix should usually be upstream in the IR pipeline, not hidden inside the adapter.
When an adapter artifact is blocked, inspect both the renderability reasons and the signal inventory provenance.
For `.fsm` top roots, a top-port direction can be unresolved for rendering while the inventory still shows the declared `direction_hint` and the actor/topology-derived `graph_direction_hint` separately.

For renderable `.fsm` top-composition roots, inspect the selected top candidate as well as the emitted target text.
The adapter artifact keeps the declared top-port direction and numeric width, the top-port automation confidence, child declaration support IDs, topology-link support IDs, and the renderable top root.
For blocked composition roots, the same artifact shape is useful for diagnostics: a missing child can block emission while the surviving top-port provenance remains available for review.

For structured FSM roots, the artifact preserves the graph surface that led to renderability or blocking.
Renderable structured-FSM cases keep named state and transition candidates, and reset-block cases keep renderable state bodies plus synchronous/asynchronous reset block roles.
Blocked state-graph cases preserve the declared states, rejected transitions, support IDs, and automation confidence so a missing initial state or undeclared transition target can be corrected upstream without guessing what the adapter saw.

## Prior memory

The cross-document learning plane writes:

- `generated/prior_memory/corpus_memory.json`

That file is the symbolic memory that grows over time as validated artifacts teach `specforge` reusable extraction priors.
It can contain actor-taxonomy, semantic-phrase, semantic-modality-reliability, temporal-phrase, table-shape, visual-motif, and negative-knowledge priors.

It is still local generated state.
The code defines how learning works; `corpus_memory.json` stores what the local workspace has learned so far.

## Why generated artifacts are not tracked

`generated/` stays local and untracked.

That rule matters because:

- generated artifacts can be large
- they change often
- they can include backend dumps that exceed hosted Git limits
- they are reproducible enough to regenerate when the runtime is available
- continuity should live in stable tracked docs, not in versioned generated blobs

There are exceptions only when the project deliberately creates small tracked fixtures under test data.
Those fixtures are curated regression inputs and expected outputs, not arbitrary live pipeline output.

## Important tracked docs

Tracked docs are not generated artifacts.
They are continuity and steering surfaces.

The most important user-visible ones are:

- `README.md`
- `VALIDATION_SNAPSHOT.md`
- `LIVE_ACHIEVEMENT_STATUS.md`

The more development-centric ones are described in the next chapter.

## How to inspect an artifact

Most artifacts are JSON.
The easiest path is to open the relevant stage file directly and compare it with the validation report for the same document.

When debugging, prefer following the pipeline backward:

- an `IntentIR` issue often comes from `SemanticIR`
- a `SemanticIR` issue often comes from `EvidenceIR`
- an `EvidenceIR` issue often comes from `SourceIR`
- a `SourceIR` issue often comes from ingest/backend normalization

That staged backward trace is one of the main reasons the project preserves intermediate artifacts at all.
