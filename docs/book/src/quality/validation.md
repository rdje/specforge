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
`project-validation` is the first consumer for this guidance: it projects those findings into the validation snapshot and writes a generated `generated/validation/rescan_plan.json` target list for later rescan/extractor-selection loops.
That target list is versioned and replay-oriented: each recommendation carries typed replay inputs plus structured command hints for the stage rebuild and follow-up validation, while remaining `planned_not_executed` until a future convergent loop chooses to execute it.
`rescan-plan` is the first explicit consumer for that schema: it dry-runs by default, and `--execute` dispatches only whitelisted stage rebuild/validate commands from the structured args rather than trusting shell text.

## Learning depends on validation gates

The learning plane should only absorb reusable knowledge from artifacts that are strong enough to trust.

That means validation is not just a reporting layer.
It is also part of the project’s epistemology:

- weak or unresolved output should not become future memory by default
- stronger, validated output can become advisory prior knowledge

The next chapters explain the benchmark harness and the prior-memory layer in more detail.
