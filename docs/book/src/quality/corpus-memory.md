# Corpus Memory And Priors

`specforge` has a cross-document learning plane, but it is not a neural network and it is not hidden model state.

The thing that grows over time is a typed symbolic prior store:

`generated/prior_memory/corpus_memory.json`

There is now a second cross-document artifact plane too:

`corpus_kb/`

That corpus knowledge base is for reviewable synthesis and failure-pattern notes, not machine-usable priors.
If a corpus KB observation should become executable extraction behavior, it must still move through a typed prior schema, KG-bench coverage, and validation-gated promotion.
The corpus KB can include a typed prior-memory fixture-family projection at `corpus_kb/prior_memory/kg-fixtures.md`, but that page is only reviewable benchmark synthesis; it is not the `CorpusMemory` store and cannot write machine-usable priors.
It can also include a review-only prior-candidate readiness manifest at `corpus_kb/prior_candidates/kg-fixture-candidates.json`; that manifest makes candidate fixture coverage easier to inspect, but it is not an approval artifact and cannot write `generated/prior_memory/corpus_memory.json`.

## Two separate planes

The architecture keeps document truth and learned experience separate.

Document plane:

`SourceIR -> EvidenceIR -> SemanticIR -> IntentIR`

Learning plane:

`CorpusMemory`

The document plane stays grounded in the current PDF.
The learning plane stores reusable extraction knowledge learned from earlier validated artifacts.

## What the system learns

Today the prior store can contain families such as:

- actor-taxonomy priors
- semantic phrase priors
- semantic modality-reliability priors
- temporal phrase priors
- table-shape priors
- visual-motif priors
- negative-knowledge priors

These are not copied document facts.
They are reusable interpretation patterns.

Examples:

- a local actor term is requester-like
- a phrase shape is ready-like
- a timing phrase implies a one-cycle window
- a header signature often means a signal-description table
- a captioned visual asset is a timing-diagram motif
- a repeated conflict or residual class should make future extraction more cautious

## What the system does not learn

The learning plane must not smuggle facts from one spec into another.

So the safe rule is:

- priors widen interpretation of the current document
- priors do not directly author canonical truth

That is especially important for visual-motif and negative-knowledge priors.
Visual-motif priors already have a first bounded consumer: if a current visual asset is still classified as `unknown` but has a local caption matching one unique learned motif, `EvidenceIR` may add a prior-memory `Classification` observation and use the recovered diagram kind for the visual evidence role.
That does not rewrite `SourceIR` and does not create semantic facts by itself.
If that prior makes the visual evidence normative before VLM extraction exists, validation reports a visual-motif corroboration target instead of treating the prior as truth.
That gives `project-validation` a rescan-planning hook for targeted multimodal corroboration while preserving the local-grounding rule.

Negative-knowledge priors are cautionary memory.
They also have bounded validation consumers: if current `EvidenceIR` already contains a local signal-semantic conflict whose pattern matches learned negative knowledge, validation may report a caution finding and increment `negative_knowledge_prior_matches`.
The same caution surface now extends into `SemanticIR` and `IntentIR` validation for carried signal-semantic conflicts, temporal value conflicts, interface-signal conflicts, signal-connectivity conflicts, and residual decision packet classes.
Validation also emits machine-readable rescan/corroboration hooks for those exact matches through `negative_knowledge_rescan_recommendations`, `negative_knowledge_corroboration_requirements`, and stage-specific `*_negative_knowledge_rescan_guidance` findings.
That does not suppress conflicts, change arbitration, remove residuals, or create canonical facts.
It only tells users and downstream tooling that the current failure shape is one the corpus memory has seen before and should be targeted for stronger local corroboration before canonical promotion.
The KG-bench suite locks that boundary across the current negative-knowledge families, including carried interface-signal and signal-connectivity conflicts, and now asserts that the validation finding payloads carry the exact related conflict/residual ids.
`project-validation` consumes that routing metadata into the generated validation rescan plan, keeping the learned caution inspectable instead of hidden inside scoring.
`rescan-plan` can then inspect that generated plan and, with explicit `--execute`, rerun only whitelisted local enrichment/stage rebuild/validate hints.
Even then, it records only validated changed/no-change status; the prior still only routes attention and does not decide canonical truth.

That is why `specforge` can get smarter over time without collapsing into cross-document contamination.

## Why this is explicit memory instead of hidden weights

The learning system is symbolic and inspectable.

That means:

- the code defines how learning works
- the prior store captures what has been learned so far
- the contents can be read, debugged, filtered, and regenerated

This is a strong fit for a provenance-first compiler pipeline because the memory stays visible instead of disappearing into opaque model parameters.

## Validation-gated feedback

Not every artifact should feed the learning plane.

The intended discipline is:

1. recover local truth from the current document
2. validate it
3. only then harvest reusable priors from sufficiently trusted output

So the learning plane depends on validation rather than bypassing it.

## Why this matters

This is how `specforge` can become better on PDF `N + 1` after analyzing PDFs `1..N`:

- not by leaking old facts into the new document
- but by learning how chip specs usually express meaning

That distinction is one of the most important architectural choices in the project.

## Closed task trees — how each was implemented and verified

### `R6-PRIOR-MEMORY-HARDENING` — close mutation gaps in `prior_memory.rs`

`prior_memory.rs` (over 2 000 lines) carried 101 missed mutants
when this tree opened — substantial functions had no unit tests
at all. The tree worked function-by-function: each leaf ran
cargo-mutants on a bounded subset of `prior_memory.rs` symbols,
added regression-only assertions for the surviving mutants, and
re-ran the harness to delta-to-zero on that subset. Verified by
cargo-mutants on the full module reading zero missed across the
hardened functions + `scripts/run_ci.sh`. *Authoritative
tracking:* `docs/tasks/R6-PRIOR-MEMORY-HARDENING.md`.

### `PRIOR-DECAY` — noticing when documents disagree (contested priors)

Corpus memory *only ever accumulates* — every document a phrase or
term appears in adds to that prior's support. That is exactly what you
want when documents *agree*. But what if they *disagree*? If one
validated spec uses a term in a *requester*-like role and another uses
the same term in a *completer*-like role, the accrete-only store
quietly keeps both — and trusts each as if it were already settled.

`PRIOR-DECAY` makes that disagreement visible. A read-only check,
`CorpusMemory::contested_priors()`, groups priors by key *within one
protocol family* and flags any key that two or more documents map to
**different** values. For each contested key it lists the competing
values (with how much support each has and which documents back it) and
names the *strongest-supported* value as a hint — but it never picks a
winner. The `learn-priors` command prints a `contested_priors:` line, so
you can see at a glance where the corpus is genuinely unsettled.

Why only flag, and not auto-fix? Because honest uncertainty beats a
confident guess. A contested key is a place where the specifications
themselves disagree (or where extraction is shaky) — exactly the kind of
thing that deserves a human's eye, not a silent override. It is the same
residual-honesty instinct as the rest of the pipeline: surface what is
not settled instead of pretending it is. The check is deliberately
*additive* — it reads the priors and changes nothing about how they are
harvested or consulted, so it can only ever add insight, never alter a
result. (Grounded in the continual-learning literature's "revise on
contradicting evidence" idea — Parisi et al., 2019 — applied here as
detection; actually *down-weighting* a contested prior during
consultation, and time-based staleness, are deliberate later steps.)
*Authoritative tracking:* `docs/tasks/PRIOR-DECAY.md`.
