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
cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/<document_key>/intent_ir.json --target isf
```

`adapt` lowers canonical intent into a backend-specific adapter artifact.

`.isf` is SpecForge's single adapter target, and the project treats the adapter as a downstream consumer rather than the core product boundary.
`.fsm` and HDL lowering (SystemVerilog, Verilog, VHDL) are out of scope — FSMGen consumes `.isf` and owns scheduling, `.fsm`, and HDL downstream; SpecForge does not do cycle scheduling.
The adapter lowers `IntentIR` through the typed `IsfIr` model and emits `.isf` S-expression source only when the canonical signal/behavior surface is renderable; otherwise it blocks with explicit `blocking_reasons` instead of fabricating target text.
When ISF lowering is blocked, the adapter artifact still preserves the recovered context (signal/behavior counts, residual decisions) so users can inspect what was learned separately from why target text was not emitted.

## `converge`

```bash
cargo run --manifest-path Cargo.toml -- converge /path/to/spec.pdf --target isf
```

`converge` is the default end-to-end path.

It:

1. ingests once
2. runs enrichment
3. rebuilds downstream stages
4. repeats until the persisted knowledge snapshot stops changing

This is the main command when you want a serious local run on a real spec.

### Flags

Full surface and defaults (authoritative source: `crates/specforge/src/cli.rs`
`ConvergeArgs`):

| Flag | Default | Meaning |
| --- | --- | --- |
| `<source>` | (required) | Source specification file to iterate on |
| `--target <isf>` | `isf` | Adapter target to materialize each pass (`isf` is the only target) |
| `--max-iterations <n>` | `8` | Safety cap on whole-pipeline convergence passes |
| `--vlm-provider <ollama\|open-ai\|lm-studio\|skip>` | `ollama` | VLM provider for figure enrichment; `skip` opts out |
| `--vlm-model <name>` | (provider default) | Model-name override for figure enrichment |
| `--nlp-provider <ollama\|open-ai\|lm-studio\|skip>` | `ollama` | LLM provider for Level 3 NLP backannotation; `skip` opts out |
| `--nlp-model <name>` | (provider default) | Model-name override for NLP Level 3 backannotation |
| `--nlp-max-sentences <n>` | `0` | Max sentences sent to NLP Level 3 per pass (`0` = all) |
| `--prior-memory <path>` | `generated/prior_memory/corpus_memory.json` | Advisory local prior-memory store consulted during extraction when present |
| `--rescan-plan <path>` | (none) | Optional schema-v2 validation rescan plan to inspect after convergence stabilizes |
| `--execute-rescan-plan` | off | Execute whitelisted recommendations from `--rescan-plan` after stabilization |
| `--rescan-plan-limit <n>` | `0` | Max pending rescan-plan recommendation(s) to process (`0` = all) |

After convergence stabilizes, the command can optionally inspect a schema-v2 validation rescan queue:

```bash
cargo run --manifest-path Cargo.toml -- converge /path/to/spec.pdf --target isf --rescan-plan generated/validation/rescan_plan.json
```

That is dry-run by default.
The plan is automatically filtered to the current source document key, so a multi-document validation queue will not execute unrelated document targets from a single `converge` run.
To execute the same guarded replay hints used by `rescan-plan --execute`, add `--execute-rescan-plan`:

```bash
cargo run --manifest-path Cargo.toml -- converge /path/to/spec.pdf --target isf --rescan-plan generated/validation/rescan_plan.json --execute-rescan-plan
```

This remains an arbitration surface, not an auto-fix path.
The stable convergence snapshot is the convergence result.
Post-rescan validation changes are reported as `changed_requires_validation_review` until validation and evidence arbitration say they are safe to promote.
The convergence summary also exposes review-required counters split across possible-improvement, regression, and neutral artifact-change verdicts from the persisted recommendation execution summaries.

### The standing extraction-quality gauge

After the loop stabilizes (and after any rescan step, so the measurement describes the *final*
artifact), `converge` runs one NLI pass over the persisted EvidenceIR's signal constraints —
"does each constraint's own source sentence entail it?" — and back-annotates the result into
the artifact as its `extraction_quality_gauge`. The convergence summary then ends with the
per-document quality report:

```text
extraction_quality_gauge: not_entailed 1/1 labeled (100.0%), abstained 0 (model qwen2.5:14b-instruct)
```

So every full pipeline run finishes with a standing measurement of how trustworthy its
extracted constraint surface is, and `specforge validate <evidence_ir.json>` re-reports it
afterwards without needing a model. The step needs the text LLM, so `--nlp-provider skip`
skips it honestly (no gauge is fabricated), and a pass that labeled nothing — provider
unreachable mid-run — is never persisted. See the
[validation chapter](../quality/validation.md) for how the gauge is reported and when its
warnings fire.

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
