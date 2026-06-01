# EvidenceIR

`EvidenceIR` is where the pipeline starts lifting grounded facts out of the source.

## What belongs in `EvidenceIR`

- extracted statements
- section anchors
- evidence spans
- visual evidence
- figure/caption links
- actor-signal relations
- early signal facts
- semantic hints
- polarity evidence

This is the first stage where the system begins to say:

- "this table row looks like a signal declaration"
- "this prose sentence looks like a constraint"
- "this caption appears to ground a semantic role"
- "this visual observation may support a timing fact"

## The mindset of this stage

This is still an evidence layer, not the final semantic truth.

So the right behavior is:

- extract what the document supports
- preserve provenance
- keep conflicting evidence visible
- avoid over-promoting generic examples into canonical interface truth

That last point matters a lot.
`EvidenceIR` is not supposed to be clever in the sense of inventing final meaning.
It is supposed to be disciplined in the sense of preserving recoverable evidence without silently flattening ambiguity.

## Typical evidence-level wins

- source/destination table recovery
- table-grounded widths
- table-synthesized signal declarations with provenance back to the originating structured table
- prose-grounded actor relations
- visual-caption semantic hints
- VLM timing-note observations
- polarity extraction, including explicit asserted-when-level prose such as `CS_N is asserted when LOW`, unambiguous collective prose such as `CS_N and WE_N are active LOW signals`, and safe clause-local mixed prose such as `CS_N is active LOW and ENABLE is active HIGH`
- negative-knowledge caution surfacing during validation

These wins are valuable because they give later stages something much stronger than free-form text:

- typed hints
- grounded spans
- table-linked facts
- visual-evidence references
- early KG edges
- explicit caution signals

The dedicated [Multimodal Evidence And Visual Grounding](multimodal-evidence.md) chapter explains the visual part of that evidence flow in more detail.

## What this stage is allowed to do

`EvidenceIR` is allowed to extract and classify.

It is allowed to say:

- this sentence is a constraint-like statement
- this signal appears in a relation-like table row
- this caption text supports a semantic-role hint
- this table suggests a width, polarity, or source/destination relation

It can also apply a collective polarity statement to multiple declared controls when the wording has one unambiguous active level.
For example, `CS_N and WE_N are active LOW signals` can ground both controls as active-low.
Mixed compound wording such as `CS_N is active LOW and ENABLE is active HIGH` can be recovered only when the clause-local parser can split and validate every signal-level pair safely.
If a polarity phrase is detached from an explicit signal, the statement stays unresolved instead of borrowing an implicit subject.
When polarity is resolved, later canonical stages expose it per signal, not only as a validation total.
If prose and a signal-description table disagree about the same signal's active level, `EvidenceIR` keeps a typed `signal_polarity_conflicts` record instead of silently picking the prose or table side.

When `EvidenceIR` synthesizes a formal declaration from a signal-description table, it also records a `table_signal_declaration_provenance` entry.
That entry links the synthetic statement id back to the structured `SourceIR` table id, so later stages can preserve table support on canonical interface signals instead of losing the fact that the declaration came from a real table.
Validation reports this count as `table_signal_declaration_provenance`, which makes the evidence-stage table bridge visible before `SemanticIR` and `IntentIR` decide what survives into canonical signal inventory.

It is not supposed to decide the final canonical meaning of the whole interface.
Likewise, negative-knowledge priors may make validation more alert to a repeated evidence-stage conflict pattern, but they do not suppress the current evidence or decide the conflict.
When that happens, validation can also mark the matched current conflict as rescan/corroboration guidance through `evidence_negative_knowledge_rescan_guidance`.
Later `SemanticIR` and `IntentIR` validation carry the same caution idea forward for repeated conflict and residual shapes.

## Typical evidence-level failure modes

- field tables leaking fake signals
- abstract example tables pretending to be real interfaces
- payload nouns being promoted to actors
- descriptive relative-clause phrases such as `mixture of` being promoted to actors
- signal names creating semantic meaning by spelling alone

Many of the project’s recent truthfulness slices have been about tightening exactly those boundaries.

## Why provenance is critical here

`EvidenceIR` is where the project first needs to defend itself against "plausible but wrong" extraction.

That is why evidence records carry things like:

- supporting statement ids
- supporting table ids
- supporting visual evidence ids
- automation confidence

Without that provenance, later semantic arbitration would not have enough context to judge which evidence is strong, weak, conflicting, or merely suggestive.

## What a good `EvidenceIR` artifact looks like

A good evidence artifact is not one that looks clean at all costs.

It is one that:

- extracts a lot of grounded candidate knowledge
- preserves where that knowledge came from
- keeps disagreement visible
- avoids creating false structure from generic or noisy inputs

That makes `EvidenceIR` the main staging area for truthfulness before canonical semantics begin.

## Closed task trees — how each was implemented and verified

### `R6-EVIDENCE-HARDENING` — close zero-coverage assertion gaps on `EvidenceIR`

This tree added regression-only test assertions to populated-but-
untested fields on `EvidenceIr`, `ExtractedStatement`, and
`FsmSignalCandidate` (the latter retained through historical
cycles and re-purposed for signal-binding context). The leaves
worked symbol-by-symbol so failure isolation stayed sharp.
Verified by mutation testing reducing missed mutants to zero on
the targeted symbols + `scripts/run_ci.sh`. *Authoritative
tracking:* `docs/tasks/R6-EVIDENCE-HARDENING.md`.

### `R15C-CONVERGENCE-REPORT` — make the anchored-rescan loop inspectable

**What it gives you:** when you run `specforge validate` on an
`EvidenceIR`, you now see a `Convergence` section — how many passes
the extractor ran, how many *genuinely new* facts it recovered, and
whether it actually settled or was cut off.

**Why that matters.** Building `EvidenceIR` is not a single pass.
Newly discovered signals become anchors that unlock more tables,
whose new value atoms unlock more prose constraints — so the
extractor loops, re-scanning with everything it has learned so far,
until a pass turns up nothing new. That last part is the honesty
question: *did the loop genuinely run out of new facts, or did it
just hit its pass limit while still finding more?* Those two
outcomes look identical in the final artifact, but they mean very
different things for how complete your capture is.

So the loop now records an `EvidenceConvergenceReport`: `passes_run`
(of `max_passes`), `new_facts_per_pass` (counting *deduplicated* new
statements — real knowledge growth, not vectors getting longer),
`total_new_facts`, and a `converged` flag. `validate` turns that into
a finding you can act on:

- **Converged** → an `Info` finding: the loop stabilized; the
  anchored rescan is as complete as this document allows.
- **Capped** → a `Warning` finding: the loop stopped at the
  pass limit while still discovering facts, so convergence is *not*
  proven and the capture may be incomplete — a signal to look closer.

The extraction outcomes themselves are unchanged; this is a pure
visibility surface over a loop that was already running. It satisfies
the R15c criterion "convergence reporting counts genuinely new
persisted facts instead of duplicate vector growth," and it gives
future R15c accuracy work a metric to push against. Verified by unit
tests on the recorded report (converged path) and both `validate`
findings (converged `Info` / capped `Warning`) + `scripts/run_ci.sh`.
*Authoritative tracking:* `docs/tasks/R15C-CONVERGENCE-REPORT.md`
(under `R15C-R15G-LEARNING-PLANE-BACKFILL.1`).

### `COMPLETENESS-REGION-ACCOUNTING` — flag intent-bearing tables that produced nothing

**What it gives you:** when you `validate` an `EvidenceIR`, a `Region
Accounting` section tells you whether any table that SpecForge *recognized* as a
register / signal / timing table came out **empty** — i.e. the table's purpose
was identified but no facts were captured from it.

**Why it matters.** A miss is, by definition, something in the document that did
*not* make it into the extraction — which is exactly what you can't see by
looking at the output alone. The reframe that makes misses findable is to check
the *input* side: every intent-bearing source region should produce at least one
fact. So this detector pairs each `SignalDescription` / `RegisterMap` /
`TimingParameter` table with the records it produced — using the existing
provenance (signal-declaration `table_id`, and the `table_id` embedded in
register/timing record ids, e.g. `reg_table_0026_000`) — and flags any such
table that yielded **zero** records as an `UnexplainedTableResidual` (a Warning
finding + the `region_unexplained_tables` metric). It is flag-only: it never
invents a fact, it makes a *gap visible* so you (or a rescan, or a fix) can act.

This is the first slice of the broader region-accounting instrument (prose and
figure regions, and a unified coverage report, follow). Verified by unit tests
(covered table / zero-yield table / non-intent kinds skipped / id-marker
precision) and a `validate` wiring test + `scripts/run_ci.sh`. *Authoritative
tracking:* `docs/tasks/COMPLETENESS-REGION-ACCOUNTING.md`.

### `COMPLETENESS-REPORT-SURFACE` — one completeness headline

**What it gives you:** a `Completeness Summary` at the end of `validate` that
answers "how completely did SpecForge capture this document?" in one place —
a `candidate_misses` total with its breakdown, and the anchored-rescan
convergence status.

The completeness checks each report their own signal (register tiling, region
accounting, prose residuals, convergence); this summary **adds them up** so you
don't have to. `candidate_misses` = register-field overlaps + interior gaps +
unexplained intent-bearing tables + partially-structured normative statements,
and the headline names each component. It is honest by construction: a *count of
candidate misses*, never a claim of completeness — and because it only sums
values the detectors already produced, it can never introduce a gap the
detectors didn't already surface (a wiring test asserts the total equals the sum
of its component metrics). *Authoritative tracking:*
`docs/tasks/COMPLETENESS-REPORT-SURFACE.md`.

### `COMPLETENESS-CLOSURE-INVARIANTS` — register bit-fields that contradict themselves

**What it gives you:** when you `validate` an `EvidenceIR`, a `Register Tiling`
section flags any register whose documented bit-fields **overlap** (two fields
claim the same bit — a contradiction) or leave an **interior gap** (an uncovered
bit *between* the lowest and highest documented field — a likely missed field).

**Why it's a closure invariant.** A register is supposed to tile its bits: each
bit belongs to exactly one field. That's a structural law you can check exactly,
with no ground truth — so a violation is a high-confidence signal that the
extraction (or the spec) is wrong. It is deliberately conservative: the IR
carries no register *width*, so bits *above* the highest documented field are
never flagged (that would require guessing the width); reversed bounds are
normalized; registers with fewer than two bit-bounded fields are skipped.

On the real corpus this detector did double duty as an **extraction-precision
signal** — its overlap finding on an I2C table was what first exposed an
over-eager register-map classifier (since fixed; see SourceIR), and across 84
genuine registers it raised zero false positives. Overlaps surface as a Warning,
interior gaps as Info (a gap is a *candidate* missed field, not a proven defect).
*Authoritative tracking:* `docs/tasks/COMPLETENESS-CLOSURE-INVARIANTS.md`.

### `PER-EXTRACTOR-FACT-TAGGING` — who found which fact (recall-gauge groundwork)

This is plumbing for a future **calibrated recall estimate**. To estimate how
much a document states that SpecForge *didn't* capture, you compare independent
extractors: the fraction each finds alone vs. together lets you infer the unseen
remainder (capture–recapture). That needs to know *which* extractor found each
fact — but the pipeline normally merges (and de-duplicates) everything into one
`EvidenceIR`, erasing that.

So `EvidenceIR` now carries a `fact_provenance` index: each entry records that a
tier (**Pattern** = the structural prose/table extractor at build, **Nlp** = the
LLM in `nlp-enrich`) found a fact, under a **canonical key** that normalizes the
fact (e.g. signal + constraint kind + value) so the *same* constraint found by
two tiers maps to the *same* key — which is exactly the overlap the estimate
needs. Crucially, the NLP tier's finds are recorded *before* de-duplication, so
an overlap with a pattern find isn't silently dropped. `validate` shows the
per-tier counts. This slice records the data (signal constraints first); the
recall estimate that consumes it is a separate, designed follow-on. *Authoritative
tracking:* `docs/tasks/PER-EXTRACTOR-FACT-TAGGING.md`.

### `COMPLETENESS-RECALL-GAUGE` — an honest estimate of what's still missing

**What it gives you:** a `Recall Estimate` line in `validate` that puts a number
on the *unseen* — roughly how many signal-constraint facts the document states
that **neither** extractor captured — instead of only listing the misses it can
point to.

It uses **capture–recapture**: if the pattern tier finds a set of facts and the
NLP tier finds another, the size of their *overlap* tells you how much you're
likely still missing (a lot of overlap ⇒ you've probably found most of it; little
overlap ⇒ there's likely a large unseen remainder). With two extractors that is
the Lincoln–Petersen estimator `N̂ = |a|·|b| / overlap`; remaining misses ≈ `N̂ −
distinct-found`.

It is deliberately honest: it computes **only** when both tiers have findings
that actually overlap (so it never invents a "0 misses" out of no data — you'll
see *"insufficient — run nlp-enrich"* instead), and it reports the remaining
misses as a **lower bound** with its assumptions printed (the two tiers read the
same prose, so they're partially correlated and the estimate is optimistic). A
future third, independent extractor would let it move to the sharper Chao
estimator. *Authoritative tracking:* `docs/tasks/COMPLETENESS-RECALL-GAUGE.md`.
