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
