# Generated Artifacts

This chapter is the public map of what `specforge` writes and where those files fit in the project.

The most important rule is simple:

`generated/` is local execution state, not source code.

That rule is intentional. Real chip PDFs can produce large intermediate artifacts, and those artifacts can change whenever extraction, enrichment, validation, or learning logic improves.

## Tracked source PDFs

Source PDFs intentionally kept for reproducible re-ingest live below `corpus/`; they are inputs, not
generated artifacts. `corpus/SOURCE_PDF_REGISTRY.md` names every such PDF, its descriptive class, and
the `document_key` that `SourceIR` derives from the filename.

The Git-indexed PDF set below `corpus/` is the membership authority. The unconditional read-only
currentness check requires one registry row per tracked PDF, verifies the code-derived key and parent
directory, checks the PDF signature, and pins the Rust derivation seams. A larger host-local source
library and git-ignored stage artifacts are not part of this durable set. Run:

```bash
perl scripts/check_source_pdf_registry_currentness.pl --report
```

## Generated artifact roots

- `generated/source_ir/<document_key>/source_ir.json`
- `generated/evidence_ir/<document_key>/evidence_ir.json`
- `generated/semantic_ir/<document_key>/semantic_ir.json`
- `generated/intent_ir/<document_key>/intent_ir.json`
- `generated/adapters/isf/<document_key>/adapter.json`
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

SourceIR and EvidenceIR use a two-form path contract. In memory, their paths are resolved against the current
repository root and can be opened directly. In their persisted JSON, repository-owned paths are relative—for
example `generated/source_ir/<document_key>/source_ir.json` rather than an absolute workstation path. An
explicitly authorized source outside the repository remains absolute and is labeled as an external input.
Legacy absolute repository paths are accepted only through the bounded, unique-target compatibility resolver.
SemanticIR, IntentIR, and adapter adoption is still in progress, so do not manually rewrite the current
generated corpus ahead of the verified migration.

## Source-side sidecars

PDF ingest may also materialize source-side sidecars under the same document root.
Depending on the input and backend path, these can include:

- promoted markdown
- page metadata
- page images (persisted for normal documents; skipped for very large ones to bound disk — see
  the [SourceIR chapter](../pipeline/sourceir.md#bounded-disk-footprint-of-very-large-pdfs))
- visual asset crops
- normalized backend dumps
- backend diagnostics

Those files are part of the local evidence trail.
They are useful for debugging and visual grounding, but they should not be treated as hand-authored project assets.
Paths recorded in the SourceIR JSON and page/visual manifests are repository-relative even though the loaded
SourceIR exposes their current absolute runtime locations.

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

- `generated/adapters/isf/<document_key>/adapter.json`

`.isf` is SpecForge's single adapter target. The adapter is a downstream
product of `IntentIR`: it lowers the canonical model through the typed
`IsfIr` model (`IntentIR -> IsfIr::from_intent_ir() -> IsfIr::render() -> .isf`).
FSMGen consumes `.isf` and owns scheduling, `.fsm`, and HDL downstream;
SpecForge does not do cycle scheduling and does not emit `.fsm` or HDL.

The adapter artifact (`AdapterArtifact` with an `isf` payload) records:

- `actor_name` and `source_text` (the rendered `.isf` S-expression text)
- `is_renderable` and `blocking_reasons`
- `signal_count`, `transaction_count`, `rule_count`, `constant_count`,
  `enum_count`, `storage_count`
- `lowering_status` (`renderable` / `blocked`), `residual_decisions`, and
  `validation_reports`

When the artifact is renderable, the emitted `.isf` text is also written to
`generated/adapters/isf/<document_key>/<actor_name>.isf`.

Adapter artifacts should not define what the canonical document meaning is.
If an adapter needs meaning that is not present in `IntentIR`, the fix
belongs upstream in the IR pipeline, not inside the adapter.

### Renderability policy

ISF lowering blocks on exactly two conditions:

1. no signals declared in any interface, and
2. no behavioral content (temporal rules, conditional rules, signal
   constraints, or control blocks).

Missing per-signal direction or width is deliberately **not** a blocker:
the ISF IR defaults an unknown direction to `output` and an unknown width
to `1`, and emission proceeds. FSMGen performs the cycle scheduling for
`.isf` and accepts a default direction/width, so blocking on those would
over-restrict otherwise-honest lowering without improving downstream
correctness.

### Validation

`specforge validate <isf adapter.json>` auto-detects the `isf_adapter`
stage and runs structural plus coverage checks (missing ISF payload,
unexpected schema version, not renderable, empty signal inventory, no
behavioral surface, residual decisions). Emitted `.isf` is additionally
validated against FSMGen `--strict --check --json` in the tracked test
suite, so SpecForge-generated ISF stays on FSMGen's strict acceptance
surface.

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

`VALIDATION_SNAPSHOT.md` is the last **reviewed** projection, not an automatic claim about the newest
git-ignored artifact on the current host. Its tracked currentness contract records the reviewed report
identities and is checked without reading or mutating `generated/`. A refresh becomes validated state
only after its projection is reviewed and committed with that declaration.

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
