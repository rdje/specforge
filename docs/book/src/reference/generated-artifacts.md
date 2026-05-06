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
Standalone DT actor-port direction recovery keeps graph-backed direct signal directions in `graph_direction_hint` with actor-port provenance, and renderable recovery leaves no signal-inventory residual.
Target-actor-backed direct output selection follows the same rule while excluding shared external actor evidence from the selected inventory.
Direct control-read input recovery also stays signal-inventory-residual clean while retaining control-branch support IDs and avoiding external actor direction import.
Direct actor-port width recovery keeps actor-port width provenance on the recovered control input without importing external actor direction, and renderable recovery leaves no signal-inventory residual.
Unrelated actor-port graph context follows the same residual-clean rule while keeping side-band graph evidence out of the selected inventory.
Ambiguous actor-port graph context remains blocked instead of selecting graph provenance, and the artifact keeps signal-inventory residual guidance for upstream enrichment.
Actor-port direction conflicts also remain blocked while preserving both conflicting support IDs and signal-inventory residual guidance.
Actor-port width conflicts follow the same blocked guidance pattern while preserving both conflicting width support IDs.
Flat direction conflicts with graph-backed actor provenance remain blocked and keep signal-inventory residual guidance on the flat-conflict regression itself.
Flat/graph direction disagreements likewise keep interface and actor-port provenance plus signal-inventory residual guidance while blocked.
Graph-backed undriven standalone outputs keep selected actor support and signal-inventory residual guidance while blocked.
Structured-FSM graph-backed undriven outputs keep the same residual guidance alongside FSM-state blocker text.
Missing-initial structured-FSM blockers keep state and transition provenance plus state-graph residual guidance for upstream enrichment.
Undeclared-target structured-FSM blockers keep declared-state and rejected-transition provenance plus the same state-graph repair guidance.

For renderable `.fsm` top-composition roots, inspect the selected top candidate as well as the emitted target text.
The adapter artifact keeps the declared top-port direction and numeric width, the top-port automation confidence, child declaration support IDs, topology-link support IDs, and the renderable top root.
When multiple child instances reuse the same module, the top candidate still keeps per-instance child support IDs plus top-port and link provenance, while the renderable source document emits the shared child module root only once without leaving composition-topology residuals.
When a renderable source document emits the `?top` root before producer/consumer child direct roots, the selected top candidate and renderable top root still carry the public top-port and topology-link support IDs without leaving composition-topology residuals.
Single-FSM-child top documents preserve the same top-port and topology-link provenance while keeping the child reference typed as an FSM child root and leaving no composition-topology residual once renderable.
Reused-FSM-child top documents also preserve per-output top-port and link provenance while emitting the shared FSM direct root only once without leaving composition-topology residuals.
Mixed DT/FSM child top documents preserve public top-port and topology-link provenance while retaining child root kinds and direct-root order without leaving composition-topology residuals.
When explicit top-link topology recovers a public top-port direction, the recovered top records retain the original top-port declaration support IDs as well as the topology evidence that supplied the direction, and renderable recovery leaves no composition-topology residual.
If that recovered top-port direction is still blocked by another composition gate, the adapter also emits the composition-topology residual decision so child-module reference repair remains visible.
Recovered top-port evidence can also raise the selected top root-kind confidence while keeping that same residual decision visible when topology is still incomplete.
High-confidence child declarations can raise the selected top root-kind confidence without emitting composition residuals once the single-child topology is renderable.
High-confidence top-link evidence follows the same residual-clean rule once the linked topology is renderable.
Actor-port-backed top direction recovery follows the same rule: the graph evidence is added without dropping the public top-port declaration support IDs, and renderable recovery leaves no composition-topology residual.
Actor-port-backed top width recovery also retains public top-port declaration support while adding the graph width evidence, and renderable recovery leaves no composition-topology residual.
Child-link-backed top width recovery likewise retains public top-port declaration support while adding the topology width evidence, and renderable recovery leaves no composition-topology residual.
Child-system-contract-backed top width recovery retains the clock/reset top-port declaration support while adding topology-link and system-contract evidence, and renderable recovery leaves no composition-topology residual.
Top-link-backed child width recovery retains the child signal declaration support while adding topology width evidence, and renderable recovery leaves no composition-topology residual.
Sibling-link-backed child width recovery follows the same rule for recovered child inputs, and renderable recovery leaves no composition-topology residual.
Source-side sibling-link child width recovery follows it for recovered child outputs, and renderable recovery leaves no composition-topology residual.
Transitive child width recovery keeps declaration support for each recovered child signal while recording the topology links that propagated the width, and renderable recovery leaves no composition-topology residual.
Child link-topology direction recovery keeps declaration support for each recovered child signal while recording the topology links that supplied graph directions, and renderable recovery leaves no composition-topology residual.
Child actor-port direction recovery keeps declaration support for each recovered child signal while recording the graph actor-port evidence that supplied directions, and renderable recovery leaves no composition-topology residual.
Top-boundary actor-port direction conflicts preserve top-port direction repair guidance on the blocked top candidate and aggregate adapter renderability while retaining graph-backed support.
Top-boundary actor-port direction conflicts also emit the composition-topology residual decision so explicit-top-port repair remains visible in structured diagnostics.
Top-boundary top-link direction conflicts preserve the same direction repair guidance while retaining topology-link support.
Top-boundary top-link direction conflicts also emit the composition-topology residual decision so explicit-top-port repair remains visible in structured diagnostics.
Duplicate top-boundary direction declarations also preserve that direction repair guidance while retaining both duplicate declaration support-ID sets.
Duplicate top-boundary direction blockers also preserve explicit top-port deduplication guidance on both blocked renderability surfaces.
Duplicate top-boundary direction blockers also emit the composition-topology residual decision so explicit-top-port repair remains visible in structured diagnostics.
When child actor-port direction evidence conflicts, blocked module renderability preserves actor-relative graph-direction repair guidance and top aggregate renderability carries that child-module guidance upward alongside the conflicting evidence.
Child actor-port direction conflicts also emit the composition-topology residual decision so renderable-child repair remains visible in structured diagnostics.
Child topology direction conflicts preserve the same repair-guidance channel while retaining the contradictory topology-link support IDs that caused the graph-direction conflict.
Child topology direction conflicts also emit the composition-topology residual decision so renderable-child repair remains visible in structured diagnostics.
Child topology width conflicts likewise preserve canonical width repair guidance on blocked module and aggregate renderability surfaces while retaining signal-declaration and topology-link support IDs.
Child topology width conflicts also emit the composition-topology residual decision so renderable-child repair remains visible in structured diagnostics.
Sibling child-link width conflicts keep that width repair guidance while preserving both conflicting topology-link support-ID sets.
Sibling child-link width conflicts also emit the composition-topology residual decision so renderable-child repair remains visible in structured diagnostics.
Top-boundary actor-port, duplicate declaration, and child-link width conflicts preserve top-port width repair guidance on the blocked top candidate and aggregate adapter renderability.
Top-boundary actor-port width conflicts also emit the composition-topology residual decision so explicit-top-port repair remains visible in structured diagnostics.
Top child-link width conflicts also emit the composition-topology residual decision so explicit-top-port repair remains visible in structured diagnostics.
Top-link endpoint width mismatches preserve width-compatible repair guidance on the blocked top candidate and aggregate adapter renderability while keeping the mismatched topology-link support visible.
Those width-mismatched top-link blockers also emit the composition-topology residual decision so the same repair lane remains visible in structured adapter diagnostics.
Duplicate top-boundary width blockers also preserve explicit top-port deduplication guidance alongside the width repair guidance.
Duplicate top-boundary width blockers also emit the composition-topology residual decision so explicit-top-port repair remains visible in structured diagnostics.
Widthless public top ports preserve top-boundary width recovery guidance on the same blocked renderability surfaces while keeping missing numeric width distinct from symbolic and conflicted width evidence.
Widthless public top-port blockers also emit the composition-topology residual decision so explicit-top-port repair remains visible in structured diagnostics.
Parametric public top ports preserve top-boundary width resolution guidance while keeping symbolic width evidence inspectable in the selected top signal inventory.
Parametric public top-port blockers also emit the composition-topology residual decision so explicit-top-port repair remains visible in structured diagnostics.
Top-link direction-role mismatches preserve source/output and target/input repair guidance on blocked child, top, and aggregate renderability surfaces while keeping topology-link provenance visible.
Source-side top-link direction-role mismatches also emit the composition-topology residual decision so renderable-child repair remains visible in structured adapter diagnostics.
Target-side top-link direction-role mismatches now mirror that residual decision coverage.
Dedicated child endpoint role regressions back those source/target residual guarantees alongside the public top-boundary role conflict checks.
Topology-backed child target direction-role mismatches preserve the same role guidance even when the blocker is a flat/graph direction disagreement rather than a graph-only conflict.
Top-boundary top-link direction conflicts preserve the same endpoint-role guidance alongside boundary direction-conflict guidance while keeping top-port and topology-link provenance visible.
Top-target boundary role conflicts preserve that role guidance with boundary direction-conflict guidance while retaining the selected top signal-inventory evidence.
Top links to child endpoints that are not emitted preserve source- or target-endpoint repair guidance on both the blocked top candidate and aggregate renderability surfaces.
For blocked composition roots, the same artifact shape is useful for diagnostics: a missing child can block emission while the surviving top-port provenance remains available for review.
Missing-child top blockers also preserve child-source declaration guidance on both blocked renderability surfaces.
The primary missing-child composition fixture locks the same guidance beside top-port and child-declaration provenance.
Missing-child top blockers also emit the composition-topology residual decision so child-module reference repair remains visible in structured diagnostics.
Duplicate top child-instance blockers preserve deduplication guidance while keeping both child declaration support-ID sets visible.
Duplicate top child-instance blockers also emit the composition-topology residual decision so child-module reference context remains visible in structured diagnostics.
Multi-child top roots that omit explicit top-link records preserve top-link enrichment guidance while keeping child declaration provenance and resolved child root kinds visible.
Multi-child top roots that omit explicit top-link records also emit the composition-topology residual decision so width-compatible-link repair remains visible in structured diagnostics.
Top roots that omit child-module references preserve child-module enrichment guidance while keeping the declared top-port provenance visible.
Top roots that omit child-module references also emit the composition-topology residual decision so child-module reference repair remains visible in structured diagnostics.
Top roots that omit top-port records preserve top-port enrichment guidance while keeping child declaration provenance visible.
Top roots that omit top-port records also emit the composition-topology residual decision so explicit-top-port repair remains visible in structured diagnostics.
Top links that target undeclared top-boundary ports preserve target-endpoint enrichment guidance while keeping the declared top-port, child, and link provenance visible.
Top links that target undeclared top-boundary ports also emit the composition-topology residual decision so explicit-top-port repair remains visible in structured diagnostics.
Top links that originate from undeclared top-boundary ports preserve source-endpoint enrichment guidance while keeping the declared top-port, child, and link provenance visible.
Top links that originate from undeclared top-boundary ports also emit the composition-topology residual decision, mirroring target-side explicit-top-port repair diagnostics.
Top links that originate from child endpoints absent from emitted child modules preserve source-endpoint enrichment guidance while keeping the declared top-port, child, and link provenance visible.
Top links that target child endpoints absent from emitted child modules preserve target-endpoint enrichment guidance while keeping the declared top-port, child, and high-confidence link provenance visible.
Source-side unemitted child endpoint blockers also emit the composition-topology residual decision, mirroring the target-side residual lock.
Target-side unemitted child endpoint blockers also emit the composition-topology residual decision so renderable-child repair remains visible in structured adapter diagnostics.

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
