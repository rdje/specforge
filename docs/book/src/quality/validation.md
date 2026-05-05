# Validation And Learning

This chapter explains how `specforge` judges its own output quality and how it improves over time without letting learned memory overwrite document truth.

There are two separate concerns here:

- validation:
  deciding how strong, complete, and internally consistent a current artifact is
- learning:
  accumulating reusable extraction priors from earlier validated artifacts

Those concerns are connected, but they are not the same thing.

## Validation is about present-document truth

`specforge validate` operates on one artifact and reports how well that artifact currently holds up.

It measures things like:

- declared-signal direction coverage
- graph-derived direction coverage
- compatibility-direction findings that distinguish flat-hint lag from signals that still lack both flat direction and actor-relative graph coverage
- table-backed signal coverage
- width coverage
- temporal rule cycle-window grounding
- temporal rule actor grounding
- handshake-completion temporal predicates
- infrastructure-signal source and distribution status
- VLM readiness and visual enrichment coverage
- visual semantic grounding
- cross-modality semantic grounding
- negative-knowledge prior matches
- semantic conflicts
- connectivity conflicts
- temporal conflicts
- residual decisions
- overall score and grade

The validator is important because `specforge` is not trying to sound plausible.
It is trying to be inspectable and honest about what was recovered, what remains ambiguous, and where the current artifact is still weak.

## Scores are useful, but not the whole truth

The quality score is a compact summary, not a replacement for the detailed findings.

That matters because:

- a score can stay flat while the artifact becomes more truthful
- a score can even go down when the pipeline stops cheating
- removing false structure is often a real improvement, even if a coarse coverage metric gets stricter

One useful example is `with_table_support` on `SemanticIR` and `IntentIR`.
It reports how many canonical signal records still carry `supporting_table_ids` from structured `SourceIR` tables.
That number is a coverage and explainability signal, not a new truth source: exact signal-to-table provenance is still checked by canonical IR shape and benchmark expectations.
The earlier EvidenceIR metric `table_signal_declaration_provenance` plays the same role one stage earlier: it counts table-synthesized declaration links before canonical signal records exist.

So the right way to read a validation result is:

1. inspect the findings
2. inspect the conflict and residual surfaces
3. then use the score as a compact rollup

## `project-validation`

`project-validation` exists because single-artifact validation is not enough for project steering.

It validates selected artifacts and refreshes the tracked live projection docs, especially:

- `VALIDATION_SNAPSHOT.md`
- the managed validation block in `LIVE_ACHIEVEMENT_STATUS.md`

That command belongs to the continuity plane, but users still benefit from understanding that it is how the project keeps its published local baseline honest.

## Residuals and conflicts are first-class

One of the central design choices in `specforge` is that unresolved ambiguity must stay visible.

That is why the IR and validator preserve things like:

- residual decisions
- interface conflicts
- connectivity conflicts
- semantic-role conflicts
- temporal conflicts

This is not an implementation detail.
It is part of the product philosophy.

Prior memory can make some of those conflict surfaces more informative, but it must not erase them.
For example, `EvidenceIR` validation can report `negative_knowledge_prior_matches` when a current signal-semantic conflict matches a learned caution pattern.
`SemanticIR` and `IntentIR` validation can report the same metric when carried conflict or residual packet patterns match learned negative knowledge.
Those exact matches also surface `negative_knowledge_rescan_recommendations`, `negative_knowledge_corroboration_requirements`, and stage-specific `*_negative_knowledge_rescan_guidance` findings.
That is guidance for targeted rescans and stronger local corroboration, not a correction: the current conflict or residual remains present and still has to be resolved by local evidence and arbitration.
The KG benchmark fixture schema can now assert the related ids on those validation findings, which keeps negative-knowledge guidance tied to the exact conflict or residual packet that matched the learned caution pattern.
`project-validation` is the first consumer for this guidance: it projects those findings into the validation snapshot and writes a generated `generated/validation/rescan_plan.json` target list for later rescan/extractor-selection loops.
It now consumes the EvidenceIR form too, so `evidence_negative_knowledge_rescan_guidance` becomes a bounded `SourceIR -> EvidenceIR -> validate` replay target keyed by the exact conflict or residual ids that triggered the learned caution, while semantic/intent forms keep their downstream rebuild lanes.
The same rescan-guidance channel also covers prior-classified visual-motif evidence that became normative but still needs VLM/multimodal corroboration.
When a matching existing plan already carries executed recommendation summaries, `project-validation` preserves them and projects the review-relevant verdict/delta summary into the tracked validation docs.
That target list is versioned and replay-oriented: each recommendation carries typed replay inputs plus structured command hints for targeted local enrichment, stage rebuild, and follow-up validation, while remaining `planned_not_executed` until an explicit rescan consumer chooses to execute it.
The compact live-status queue now also surfaces the replay-input kind chain and a concise action summary beside each recommendation, so replay scope is visible even from the first-line tracked projection.
Non-decisive semantic-role arbitration is now part of that same guidance surface: when competing role evidence is still non-decisive, validation emits a replay-oriented recommendation to rerun local NLP enrichment on `EvidenceIR` and rebuild the downstream canonical stages before re-validating.
Fallback-only resolved semantic roles now join that same bounded loop too: if canonical role meaning survived without observation-backed `semantic_consensus`, validation emits `semantic_role_consensus_surface_rescan_guidance` so operators can replay the local evidence lane instead of treating provisional meaning as settled truth.
Alias-dependent semantic consensus now joins it as well: if a resolved role still depends only on alias-grounded evidence, validation emits `semantic_alias_dependent_semantic_consensus_surface_rescan_guidance` so the next step is to seek stronger direct or corroborating non-alias grounding, not to silently accept the alias-dependent state as good enough.
Prior-guided final consensus now joins it too: if a resolved role converged with help from learned modality-reliability priors, validation emits `semantic_prior_guided_semantic_consensus_surface_rescan_guidance` so operators can look for stronger current-document corroboration rather than treating the prior-guided state as fully self-sufficient.
At the evidence stage, carried semantic-role conflicts now join it too: validation emits `evidence_signal_semantic_conflict_surface_rescan_guidance` so preserved `semantic_conflict_*` ids become explicit evidence-local NLP replay targets instead of passive upstream disagreement markers.
Canonical signal surfaces that still lack actor-relative graph direction coverage now join it too: validation emits `semantic_graph_direction_coverage_surface_rescan_guidance` so the next step is to revisit local actor/role language and see whether those same signal ids can be rebuilt with graph direction instead of remaining graph-uncovered.
Carried graph-direction self-conflicts now join it as well: validation emits `semantic_graph_direction_conflict_surface_rescan_guidance` so preserved actor-aware conflict ids become explicit replay targets instead of passive same-actor direction disagreement markers.
When one artifact emits both graph-coverage and graph-conflict guidance for the same signal, `project-validation` preserves both replay recommendations at SemanticIR and IntentIR stages: the signal id remains a coverage target, the actor-aware conflict id remains a disagreement target, and each recommendation keeps the expected local NLP enrichment, downstream rebuild, and validation command lane.
Protocol connectivity endpoint gaps now join it too: validation emits `semantic_connectivity_missing_producer_surface_rescan_guidance` or `semantic_connectivity_missing_consumer_surface_rescan_guidance` so signals with only a consumer side or only a producer side become explicit local replay targets instead of passive structural debt.
Infrastructure clock/reset sourcing is intentionally excluded from that replay family, because protocol PDFs often define boundary-visible clock/reset contracts without naming the eventual physical producer.
Carried interface-signal conflicts now join it as well: validation emits `semantic_interface_signal_conflict_surface_rescan_guidance` so preserved direction/width conflict ids become explicit replay targets instead of passive interface disagreement markers.
Carried typed temporal conflicts now join it as well: validation emits `semantic_temporal_conflict_surface_rescan_guidance` so preserved contradiction ids become explicit replay targets instead of passive timing-conflict markers.
Carried signal-connectivity conflicts now join it as well: validation emits `semantic_signal_connectivity_conflict_surface_rescan_guidance` so preserved producer-ambiguity conflict ids become explicit replay targets instead of passive conflict markers.
Carried signal-polarity conflicts now join it as well: validation emits `semantic_signal_polarity_conflict_surface_rescan_guidance` so preserved active-level conflict ids become explicit replay targets instead of passive polarity-disagreement markers.
Carried actor-port gaps now join it as well: validation emits `semantic_actor_port_gap_surface_rescan_guidance` so preserved actor-signal relation ids become explicit replay targets instead of passive relation-only graph debt.
At the source stage, missing VLM enrichment now joins it too: validation emits `source_vlm_enrichment_missing_surface_rescan_guidance` so timing/state diagram asset ids become explicit SourceIR visual replay targets instead of passive source-side enrichment debt.
At the evidence stage, missing VLM observations now join it too: validation emits `evidence_missing_vlm_observations_surface_rescan_guidance` so visual ids without timing/state extraction become explicit source-side visual replay targets instead of passive enrichment debt.
At the evidence stage, structural-KG gaps now join it too: validation emits `evidence_structural_kg_missing_surface_rescan_guidance` so stranded behavioral ids become explicit evidence-local NLP replay targets instead of passive missing-graph debt.
At the evidence stage, partially structured normative residuals now join it too: validation emits `evidence_normative_residual_surface_rescan_guidance` so preserved residual statement ids become explicit evidence-local NLP replay targets instead of passive upstream extraction debt.
At the evidence stage, carried polarity conflicts now join it too: validation emits `evidence_signal_polarity_conflict_surface_rescan_guidance` so preserved `polarity_conflict_*` ids become explicit evidence-local NLP replay targets instead of passive upstream disagreement markers.
Carried signal-semantic conflicts now join it as well: validation emits `semantic_signal_semantic_conflict_surface_rescan_guidance` so preserved semantic-conflict ids become explicit replay targets instead of passive semantic-role disagreement markers.
Typed temporal rules that still lack actor-relative drive/sample grounding now join it too: validation emits `semantic_temporal_actor_grounding_surface_rescan_guidance` so the next step is to revisit local timing language and see whether those same rule ids can be rebuilt with explicit actor grounding instead of remaining actorless.
Typed temporal rules that still lack explicit clock or edge grounding now join it too: validation emits `semantic_temporal_clock_grounding_surface_rescan_guidance` so the next step is to revisit local timing language and see whether those same rule ids can be rebuilt with explicit clock context instead of remaining clockless.
Typed temporal rules that still lack explicit cycle-window bounds now join it as well: validation emits `semantic_temporal_cycle_window_surface_rescan_guidance` so the next step is to revisit local timing language and see whether those same rule ids can be rebuilt with explicit bounds instead of remaining unbounded.
For visual-motif corroboration, the replay sequence is `enrich_source_ir`, rebuild `EvidenceIR`, then validate the current artifact.
The generated enrichment hint now uses a local provider policy: `auto-local` prefers ready Ollama and falls back to ready LM Studio, while explicit `ollama`, `lmstudio`, `skip`, and model overrides remain available on `project-validation`.
`rescan-plan` is the first explicit consumer for that schema: it dry-runs by default, prints artifact path, extractor lane, replay inputs, related ids, and a readable action summary before the command hints, with empty fields rendered as explicit `none` values.
With `--execute`, it dispatches only whitelisted local enrichment/stage rebuild/validate commands from the structured args rather than trusting shell text.
It can also scope a multi-document queue with `--document-key <key>`.
Execution validates before and after the rebuild and records only neutral changed/no-change status, not an improvement claim.
Executed recommendations also persist an `execution_summary` containing before/after validation snapshots, score/finding deltas, added/removed finding ids, and a conservative verdict that distinguishes possible improvement from regression or neutral artifact drift while still requiring review.
That summary also carries an explicit promotion gate: no-change executions remain `not_promoted_no_change`, and changed executions remain `not_promoted_review_required` until current-document evidence and arbitration policy approve any canonical change.
It now also carries `promotion_review`, which records whether human review and an approval record are required, the policy that would have to be satisfied, the required decisions, and the fact that canonical mutation is still not allowed by the rescan executor itself.
That review record is not the approval artifact.
Future approval artifacts remain local/generated by default until a deliberate canonical mutation workflow defines tracked approval evidence with current-document support, validation-delta review, exact mutation scope, prior-memory non-authority, reviewer intent, artifact fingerprints, and replayable provenance.
`converge --rescan-plan <plan>` now reuses that same consumer after the fixed-point loop stabilizes.
It filters the queue to the current source document key, can dry-run the queue, or execute it only when `--execute-rescan-plan` is present, and reports changed outcomes as review-required arbitration state rather than promoting facts.

## Learning depends on validation gates

The learning plane should only absorb reusable knowledge from artifacts that are strong enough to trust.

That means validation is not just a reporting layer.
It is also part of the project’s epistemology:

- weak or unresolved output should not become future memory by default
- stronger, validated output can become advisory prior knowledge

The next chapters explain the benchmark harness and the prior-memory layer in more detail.
