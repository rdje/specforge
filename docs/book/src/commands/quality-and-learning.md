# Quality, Validation, And Learning Commands

These commands are what make `specforge` more than a one-shot extractor.

This page is the operational command map.
For the deeper rationale behind validation, fixture truthfulness, and cross-document learning, continue with:

- [Validation And Learning](../quality/validation.md)
- [KG Bench And Fixtures](../quality/kg-bench.md)
- [Corpus Memory And Priors](../quality/corpus-memory.md)

## `validate`

```bash
cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/<document_key>/intent_ir.json
```

`validate` writes a deterministic stage-local `validation_report.json` and backannotates the artifact.

It reports things like:

- direction coverage
- graph direction coverage
- width coverage
- visual classification observations
- negative-knowledge prior matches
- temporal conflicts
- semantic conflicts
- residual decisions
- overall score

## `project-validation`

```bash
cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/.../intent_ir.json
```

This command validates the passed artifacts and refreshes the tracked crash-safe snapshot docs, especially:

- `VALIDATION_SNAPSHOT.md`
- the managed validation block in `LIVE_ACHIEVEMENT_STATUS.md`

It also consumes validation-level rescan guidance and writes a local generated extractor-selection target list.
That currently includes negative-knowledge corroboration findings plus visual-motif corroboration findings for prior-classified normative visuals that still need VLM/multimodal confirmation:

`generated/validation/rescan_plan.json`

That plan is deliberately advisory, but it is now replay-oriented rather than only descriptive.
Each target carries the current artifact path, typed replay inputs such as `source_ir`, `semantic_ir`, or `evidence_ir`, a structured command-hint sequence, and an explicit `planned_not_executed` status.
For visual-motif corroboration, that sequence now starts with a local VLM `enrich_source_ir` hint, then rebuilds `EvidenceIR`, then validates the current artifact.
By default, `--rescan-vlm-provider auto-local` prefers a ready local Ollama `qwen2.5vl:7b` model and falls back to a ready local LM Studio `qwen2.5vl:7b` model before emitting the install-guiding Ollama hint.
Use `--rescan-vlm-provider ollama`, `--rescan-vlm-provider lmstudio`, or `--rescan-vlm-provider skip` to force the generated hint, and `--rescan-vlm-model <model>` to bake a model override into the plan.
The command hints are there so `rescan-plan` and opt-in `converge --rescan-plan <plan>` runs can rebuild the right stage safely from machine-readable args instead of scraping a prose note.

The plan still does not mutate IR, suppress findings, run rescans automatically, or promote facts from prior memory.
It only tells downstream loops which current conflict or residual ids deserve targeted rechecking and stronger local corroboration.

Use it when the live baseline should be updated, not just an individual artifact.
If the existing local rescan plan already contains executed recommendation summaries, the refresh preserves matching entries and projects their verdict/delta summary into the validation snapshot and live-status projection.

## `rescan-plan`

```bash
cargo run --manifest-path Cargo.toml -- rescan-plan
```

`rescan-plan` reads `generated/validation/rescan_plan.json`.
By default it is a dry-run inspector: it reports pending `planned_not_executed` recommendations and prints the structured command hints that would be used.
Use `--document-key <key>` to scope a multi-document plan to one document.

To execute the current pending hints explicitly:

```bash
cargo run --manifest-path Cargo.toml -- rescan-plan --execute
```

Execution is deliberately narrow.
The command does not shell out through the display strings.
It parses the structured `executable` and `args`, accepts only the repository-local `cargo run --manifest-path Cargo.toml -- ...` shape, and dispatches only whitelisted stage commands in-process:

- `ingest`
- `enrich` with local `ollama`, local `lmstudio`, or `skip`
- `evidence`
- `semantic`
- `intent`
- `validate`

The rescan executor intentionally rejects OpenAI enrichment hints for now, so replayable visual corroboration stays local-first unless that policy is deliberately changed later.

After a recommendation executes successfully, `rescan-plan` validates the target artifact again and records a neutral outcome in the local plan:

- `executed_validated_no_change`
- `executed_validated_changed`

That is still not a canonical truth decision.
It means the relevant stage was rebuilt and validated, and that the validation fingerprint/score/finding-count surface either changed or did not.
Any fact promotion still has to survive current-document evidence, validation, and arbitration.
Executed recommendations also carry an optional `execution_summary`.
That summary preserves before/after validation snapshots, score and finding-count deltas, added/removed finding ids, a conservative arbitration verdict, and an explicit promotion gate:

- `validated_no_change`
- `possible_improvement_review_required`
- `regression_review_required`
- `neutral_change_review_required`

Promotion is recorded separately from validation movement.
A no-change run stays `not_promoted_no_change`, and any changed run stays `not_promoted_review_required` with blockers such as `validation_delta_is_not_truth_promotion` and `current_document_evidence_review_required`.
The same summary also carries a structured `promotion_review` record.
Changed outcomes are `human_review_required`, require an approval record, and keep `canonical_mutation_allowed` false until current-document evidence, validation-delta direction, explicit mutation scope, and prior-memory non-authority have all been reviewed.
No-change outcomes are `not_reviewable_no_change`.
`promotion_review` is only the review-requirement descriptor.
It is not the approval artifact, and it cannot approve canonical mutation.
Future approval artifacts stay local/generated by default until a deliberate canonical mutation workflow defines tracked approval evidence with current-document support, validation-delta review, exact mutation scope, prior-memory non-authority, reviewer intent, artifact fingerprints, and replayable provenance.
That makes the machine-readable plan say the same thing as the product policy: a favorable validation delta is a review signal, not canonical truth.

The convergent loop can consume the same plan after stability:

```bash
cargo run --manifest-path Cargo.toml -- converge /path/to/spec.pdf --target fsm --rescan-plan generated/validation/rescan_plan.json
```

Add `--execute-rescan-plan` only when you want those whitelisted hints to run.
Unlike standalone `rescan-plan`, the convergence hook automatically filters the queue to the current source document key.
The convergence summary reports whether the post-rescan artifact snapshot changed and emits an arbitration status such as `dry_run_not_promoted` or `changed_requires_validation_review`.
That keeps rescans visible without pretending that a changed validation surface is already an improvement.

## `kg-bench`

```bash
cargo run --manifest-path Cargo.toml -- kg-bench
```

This runs the tracked truthfulness fixture set under `crates/specforge/test_data/kg_quality/`.

The benchmark surface exists to lock:

- gold paths
- negative expectations
- conflict surfacing
- residual quality
- prior-guided before/after behavior

It is the repo’s main extraction-truthfulness regression harness.

## `learn-priors`

```bash
cargo run --manifest-path Cargo.toml -- learn-priors generated/intent_ir/.../intent_ir.json
```

This updates the cross-document learning plane.

The main output is:

`generated/prior_memory/corpus_memory.json`

What it learns today:

- actor-taxonomy priors
- semantic phrase priors
- semantic modality-reliability priors
- temporal phrase priors
- table-shape priors
- visual-motif priors
- negative-knowledge priors

The safety rule is critical:

- priors widen interpretation of the current document
- priors do not directly author canonical truth

Visual-motif priors remember recurring visual patterns, while negative-knowledge priors remember conflict and residual archetypes that should make future extraction more careful.
Visual-motif priors now have a first bounded consumer: `EvidenceIR` may add a prior-memory classification observation for a current unknown visual asset when its local caption matches a unique learned motif.
Negative-knowledge priors now have bounded validation consumers too: `EvidenceIR` may surface caution for repeated local signal-semantic conflict patterns, while `SemanticIR` and `IntentIR` may surface caution for repeated carried conflict and residual packet patterns.
That caution does not suppress evidence, remove residuals, or decide semantic truth.

## `enrich` and `nlp-enrich`

These are narrower enrichment entrypoints used when you want to operate on the staged pipeline more manually.

They are still valuable, but `converge` is the preferred user-facing path when you want the full loop.
