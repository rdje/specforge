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

Every canonical stage uses a two-form path contract. In memory, repository-owned paths are resolved against the
current repository root and can be opened directly. In persisted JSON, the same paths are relative—for example
`generated/source_ir/<document_key>/source_ir.json` rather than an absolute workstation path. This covers stage
layouts and cross-stage pointers in SourceIR, EvidenceIR, SemanticIR, IntentIR, and adapter artifacts, including
the adapter's optional emitted `.isf` target.

An explicitly authorized source outside the repository remains absolute and is labeled as an external input.
Legacy absolute repository paths are accepted only when the bounded compatibility resolver finds one
unambiguous contained target below the current repository. Inputs required for current work must exist;
historical provenance may name a deliberately reclaimed leaf, but it still requires a contained existing
ancestor. Traversal, ambiguity, and symlink escape fail closed.

The present corpus has completed its guarded migration: 392 of 978 files changed, 262,996 retired-root values
became relative, and 157 origin labels were added without adding or deleting a file. The migrated tree contains
zero retired-root values. Its 82 remaining absolute path values are explicit external source-library provenance,
not SpecForge-owned project data. The locality doctrine scans every present JSON artifact—including stage files,
validation output, rescan plans, and source sidecars—and rejects any other absolute path-valued field.
Rescan-plan artifact/replay paths and command working directories are included even though those schema fields
are strings rather than `PathBuf`s; command working directories persist as `.`.

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

Three rules now define the normal lifecycle:

1. Re-ingesting the same PDF document key replaces that document's `normalized/` bundle atomically.
2. A document's `normalized/` bundle is **retained** after its ingest, and the retained set is declared.
3. `specforge clean` provides an explicit local reclamation command for rebuildable generated state.

The first rule matters because old page images, visual crops, and backend dumps can otherwise survive across reruns even after the current normalization no longer references them.
`specforge` now stages PDF normalization into a sibling `normalized.staging/` tree and only swaps it into place after the backend succeeds.
That means:

- stale leftovers from earlier runs do not accumulate inside `normalized/`
- failed reruns do not destroy the last good normalized bundle

### Normalized bundles are retained

The second rule exists because retention is what makes a document *checkable*. Rebuilding `EvidenceIR`
reads the document's promoted markdown, which lives inside `normalized/`; every later stage reads only
the persisted JSON one stage upstream. So a document that still has its bundle can be replayed — and
therefore proven current — from the beginning of the chain, while a document whose bundle was reclaimed
is honestly reported as *unmeasurable* at the first proof-gated replay its legacy/proofless authority cannot
enter. A later stage may still be reproducible from its persisted immediate input before that stage receives a
proof schema; this is stage-local currency, not end-to-end authority. This is not a stale-artifact waiver: any
current-schema proof that fails verification remains a hard currency error.

Reclaiming a bundle is no longer part of the routine. It is a deliberate, separately owned decision, and
it is recorded: `doctrine/chain_currency/retained_bundles.json` names every document key whose bundle
must be present, plus any reclamation with the leaf that authorized it and the reason. The
`CHAIN-CURRENCY` doctrine compares that declaration with what is actually on disk and fails closed both
ways — a declared bundle that has disappeared, and a bundle kept by an ingest that never recorded it.
Each refresh therefore adds exactly one document to the checkable population.

The cost is deliberately modest: the retained bundles are roughly 1.4 GB against several terabytes free,
and the full corpus extrapolates to about 4.7 GB. See
[Doctrine Enforcement](doctrine-enforcement.md) for how the check runs.

### Explicit reclamation

The third rule matters because some generated artifacts are intentionally heavy.
The cleanup command is:

```bash
cargo run --manifest-path Cargo.toml -- clean
```

That dry-runs the heavyweight `generated/source_ir/*/normalized` bundles and reports reclaimable size — it never deletes anything without `--execute`.
Add `--execute` to delete them, use `--scope document --document-key <key>` when you intentionally want a cold rebuild of one document's whole generated stage tree, or use `--scope all-generated --execute` when you intentionally want to discard the whole local `generated/` root and rebuild everything later.
Reclaiming a bundle that the retention declaration still names will redden `CHAIN-CURRENCY`; update the declaration in the same task that decides to reclaim.

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

Current adapter manifests use schema 2 and carry a private cumulative derivation ledger. The complete verified
IntentIR ledger is retained as an exact ordered prefix. Adapter-local proof then covers every public field, every
populated array record, every nonblank line of rendered ISF text, and every blocking reason. That last category is
important: an adapter that correctly refuses to emit is a proved product state, not an unchecked absence.

Only verified IntentIR may build an authoritative adapter. Canonical manifest load, pretty serialization, write,
and emitted-file reconciliation replay the current lowering before acting. Current proofless, stale, forged, or
unauthorized manifests reject; schema 1 is inspection-only and future schemas reject. Validation backannotation
is a closed mutation that may replace only `validation_reports` and must extend proof before persistence.

When the artifact is renderable, the emitted `.isf` text is also written to
`generated/adapters/isf/<document_key>/<actor_name>.isf`.

When it is blocked, the manifest has no emitted target and a successful write removes obsolete `.isf` siblings.
Chain currency checks these two states separately. It reports how many adapter states it checked, how many actual
files those states emitted, and how many were blocked/no-file; it never treats a checked blocked state as an
emitted file.

Its `input_artifact`, artifact layout, and optional `emitted_target` follow the same two-form contract as the
four main IR stages: relative in `adapter.json`, current-root absolute after loading. This lets validation and
other consumers open the artifact normally after the repository moves without persisting a workstation root.

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
Each `source_artifacts[].artifact_path` is repository-relative on disk and current-root absolute when resolved
for use, so learned lineage does not pin the memory file to the workstation on which learning ran.

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
