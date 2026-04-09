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

## Practical role in the project

If `validate` tells us how strong one artifact is, `kg-bench` tells us whether the extraction logic is still behaving correctly across targeted truthfulness cases.

In practice, it is one of the main ways `specforge` stays honest as the pipeline grows more capable.
