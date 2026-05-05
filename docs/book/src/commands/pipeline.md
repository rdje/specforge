# Pipeline Commands

These commands build the canonical four-stage IR chain.

## `inspect`

```bash
cargo run --manifest-path Cargo.toml -- inspect /path/to/file
```

Use it when you want a cheap sanity check on:

- canonical path
- source kind
- extension
- file size

## `ingest`

```bash
cargo run --manifest-path Cargo.toml -- ingest /path/to/spec.pdf
```

`ingest` produces `SourceIR`.

For Markdown, it mostly promotes the existing source.

For PDF, it performs structured normalization and materializes:

- promoted markdown
- page images
- page metadata sidecars
- visual asset crops
- backend metadata and raw JSON
- `source_ir.json`

## `evidence`

```bash
cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/<document_key>/source_ir.json
```

`evidence` produces `EvidenceIR` from a ready `SourceIR`.

It builds:

- section anchors
- evidence spans
- extracted statements
- visual evidence items
- figure/caption links
- early structural KG relations

## `semantic`

```bash
cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/<document_key>/evidence_ir.json
```

`semantic` produces `SemanticIR`.

This is where the pipeline starts to become domain-native instead of text-native:

- actors
- interfaces
- system contract
- phases
- invariants
- gates
- temporal rules
- semantic arbitration
- residual decisions

## `intent`

```bash
cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/<document_key>/semantic_ir.json
```

`intent` produces canonical `IntentIR`.

That is the main product surface today.

It carries:

- actor responsibilities
- interface/control/system surface
- behaviors
- constraints
- assumptions
- residual decisions

## `adapt`

```bash
cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/<document_key>/intent_ir.json --target fsm
```

`adapt` lowers canonical intent into a backend-specific adapter artifact.

Right now the active downstream target is `.fsm`, and the project still treats adapters as downstream consumers rather than the core product boundary.
When `.fsm` lowering is blocked, the adapter artifact can still preserve recovered context, such as top-link-derived boundary port directions, so users can inspect what was learned separately from why target text was not emitted.
For top roots, flat `direction_hint` and graph-backed `graph_direction_hint` remain separate in the signal inventory when they disagree; the renderable top port is blocked, but the artifact does not pretend the graph itself conflicted unless graph evidence disagreed with graph evidence.
For standalone DT/FSM roots, renderability also checks graph-backed output roles across the full signal inventory: an output recovered from actor-relative graph evidence still needs a typed driving action before target text is emitted.
Top-linked child endpoints remain composition diagnostics, so a child port referenced by a top link must resolve to an emitted child module port before a `?top:name` document is renderable.

## `converge`

```bash
cargo run --manifest-path Cargo.toml -- converge /path/to/spec.pdf --target fsm
```

`converge` is the default end-to-end path.

It:

1. ingests once
2. runs enrichment
3. rebuilds downstream stages
4. repeats until the persisted knowledge snapshot stops changing

This is the main command when you want a serious local run on a real spec.

After convergence stabilizes, the command can optionally inspect a schema-v2 validation rescan queue:

```bash
cargo run --manifest-path Cargo.toml -- converge /path/to/spec.pdf --target fsm --rescan-plan generated/validation/rescan_plan.json
```

That is dry-run by default.
The plan is automatically filtered to the current source document key, so a multi-document validation queue will not execute unrelated document targets from a single `converge` run.
To execute the same guarded replay hints used by `rescan-plan --execute`, add `--execute-rescan-plan`:

```bash
cargo run --manifest-path Cargo.toml -- converge /path/to/spec.pdf --target fsm --rescan-plan generated/validation/rescan_plan.json --execute-rescan-plan
```

This remains an arbitration surface, not an auto-fix path.
The stable convergence snapshot is the convergence result.
Post-rescan validation changes are reported as `changed_requires_validation_review` until validation and evidence arbitration say they are safe to promote.
The convergence summary also exposes review-required counters split across possible-improvement, regression, and neutral artifact-change verdicts from the persisted recommendation execution summaries.

## `clean`

```bash
cargo run --manifest-path Cargo.toml -- clean
```

`clean` is the local artifact-reclamation command.

Its default behavior is intentionally narrow:

- dry-run only
- scans `generated/source_ir/*/normalized`
- reports reclaimable size before deleting anything

That default targets the heavyweight PDF normalization bundles because they are usually the main disk consumers and are fully rebuildable by rerunning ingest.

To actually delete them:

```bash
cargo run --manifest-path Cargo.toml -- clean --execute
```

To delete full generated stage trees for a document instead of only the normalized PDF bundle:

```bash
cargo run --manifest-path Cargo.toml -- clean --scope document --document-key <document_key> --execute
```

That document-scope cleanup removes the per-document roots under:

- `generated/source_ir/`
- `generated/evidence_ir/`
- `generated/semantic_ir/`
- `generated/intent_ir/`
- `generated/adapters/*/`

The cleanup command is deliberately local-only.
It does not touch tracked docs, curated fixtures, or learned priors.

If you intentionally want a full local reset of the generated artifact root itself:

```bash
cargo run --manifest-path Cargo.toml -- clean --scope all-generated --execute
```

That removes the whole `generated/` tree in one sweep.
It is the CLI equivalent of "throw away all rebuildable local execution state and start fresh."
