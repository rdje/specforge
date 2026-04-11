# KG Bench And Fixtures

`specforge kg-bench` is the extraction-truthfulness regression harness.

It exists because aggregate scores are not enough.
The project also needs executable examples that prove specific recovery paths and specific negative guards.

## What it checks

The fixture set is used to lock:

- gold paths that must keep working
- negative paths that must stay blocked
- conflict surfacing
- residual quality
- prior-guided before/after behavior

This lets the project protect individual truthfulness properties instead of relying only on broad integration runs.

## Why fixtures matter

Without fixture coverage, the pipeline can drift in subtle ways:

- a false actor might quietly reappear
- a semantic role might start resolving from name noise again
- a learned prior might start overreaching
- a table-shape rule might begin misclassifying field tables as real signal tables

`kg-bench` exists to catch exactly that kind of drift.

## Gold fixtures versus negative fixtures

Gold fixtures prove that a wanted path works.

Negative fixtures prove that a dangerous path stays blocked.

Both are equally important.
For a provenance-first extractor, “did not hallucinate a fact” is often just as valuable as “recovered the intended fact.”

## Prior-guided fixtures

The benchmark harness can also stage fixture-local `CorpusMemory`.

That matters because the learning plane must be tested with the same discipline as the document pipeline:

- without the prior, the unseen local phrase or table should stay unresolved
- with the matching prior, the current document should recover the meaning locally
- the prior must widen interpretation, not author facts on its own
- caution priors must not suppress local conflicts or residuals

Negative-knowledge fixtures make that last rule executable.
The suite now has prior-guided caution fixtures for signal-semantic conflicts, temporal conflicts, residual packets, signal-connectivity conflicts, and interface-signal conflicts.
Those fixtures require the matched local conflict or residual to remain present while validation only adds `negative_knowledge_prior_matches`, rescan recommendations, corroboration requirements, and stage-specific rescan-guidance findings.

Timing-annotation fixtures also cover polarity-sensitive multimodal evidence.
For example, `vlm_timing_active_low_assertion_equivalence_gold` proves that an active-low reset observed by a VLM timing diagram as both `asserted` and `LOW` becomes typed temporal evidence without creating a false temporal conflict.
The companion `vlm_timing_active_low_deassertion_equivalence_gold` fixture proves the reset-release mirror case: `deasserted` and `HIGH` are equivalent for the same active-low reset.
That matters because `ASSERTED` and `DEASSERTED` are polarity-relative, not synonyms for fixed logic levels.

## Practical role in the project

If `validate` tells us how strong one artifact is, `kg-bench` tells us whether the extraction logic is still behaving correctly across targeted truthfulness cases.

In practice, it is one of the main ways `specforge` stays honest as the pipeline grows more capable.
