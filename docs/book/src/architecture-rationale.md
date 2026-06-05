# Architecture Rationale

This chapter explains why `specforge` is built the way it is.

The short version is:

- do not ask a model to "understand the whole spec" in one shot
- preserve source structure first
- lift grounded evidence second
- build typed semantic meaning third
- keep uncertainty visible instead of flattening it away

## The problem `specforge` is trying to solve

Chip and protocol specifications are not just text.
They are mixed evidence fields made of:

- prose
- signal tables
- timing tables
- figures
- captions
- diagrams
- page-local structure

If the tool treats the whole document as plain text, it loses too much of the shape that later meaning depends on.

## What this is, in the literature's terms: forward specification mining

There is an established name for what `specforge` does. The research community calls it
**specification mining** — automatically discovering the formal specification a system obeys,
because engineers so rarely write one by hand (Ammons, Bodík & Larus, *Mining Specifications*,
POPL 2002). Almost all of that work runs **backward**: it takes an existing *implementation* —
execution traces, RTL, source code — and recovers the spec it must be obeying.

`specforge` runs the same idea **forward**. Its input is not an implementation; it is the
*human-authored specification document* itself (the PDF — prose, tables, figures). It mines
typed design **intent** out of that document, *before any implementation exists*. So the
one-line framing is: **`specforge` is forward specification mining — spec → intent, not
implementation → spec.**

That direction is the genuinely novel part; the *machinery* is borrowed, deliberately, from
the (backward) spec-mining literature. The temporal rules `specforge` mines are exactly the
`G(antecedent → consequent)` property template Pnueli's temporal logic introduced and that
GoldMine and Texada mine from traces and RTL — `specforge` just instantiates that template
from spec prose instead. A per-author **adopt / defer ledger**
(`docs/research/grounding/adopt-defer-ledger.md`) records, for each author the literature
sweep surfaced, exactly what `specforge` takes, what it leaves out for now, and why — so the
borrowing is deliberate and the boundaries are written down, not rediscovered.

## Why there is a staged IR pipeline

`specforge` uses:

`SourceIR -> EvidenceIR -> SemanticIR -> IntentIR`

because different kinds of truth belong in different places.

- `SourceIR` preserves document structure
- `EvidenceIR` records grounded extracted evidence
- `SemanticIR` lifts that evidence into typed domain meaning
- `IntentIR` is the canonical backend-independent product surface

This separation makes the system:

- inspectable
- debuggable
- easier to validate
- less likely to hallucinate

## Why `specforge` does not try to "program understanding English"

The design goal is not to teach code to understand arbitrary English.

The design goal is narrower and more realistic:

- define a typed hardware/protocol world model
- recover evidence that maps into that model
- validate what survives into canonical intent

That is a much stronger engineering target than trying to solve unrestricted language directly.

## Why AI is bounded instead of central

AI is useful in `specforge`, but it should not be the final authority.

The intended role of AI is:

- propose local hypotheses
- help with ambiguous prose
- help with image-heavy evidence such as timing diagrams or figure captions

The intended role of the rest of the pipeline is:

- ground those hypotheses in the current document
- check them against the typed schema
- surface conflicts and uncertainty
- decide what is safe enough to keep

So the guiding rule is:

- models can propose
- validation and arbitration decide

The dedicated chapter [Multimodal Evidence And Visual Grounding](pipeline/multimodal-evidence.md) explains how visual assets, captions, and VLM observations enter that bounded flow.

### Checking a claim *semantically* — the NLI entailment gate

"Ground those hypotheses in the current document" used to mean a string check: does the
claim's signal name appear near the source text? That catches a model inventing a signal
out of thin air, but it misses a subtler — and more common — failure: reading a
*condition* as an *obligation*. *"PBUSER must be valid **when** PSEL, PENABLE, and PREADY
are asserted"* does not say PSEL must be asserted — PSEL being asserted is the *situation*,
and the obligation is on PBUSER. A string match just sees "PSEL" and "asserted" and waves
it through.

The **NLI entailment verifier** closes that gap. NLI — Natural Language Inference — is the
standard test of whether one sentence *entails* another. SpecForge treats the source
statement as the premise and each extracted claim as the hypothesis, and asks a strong
*text* model one well-posed question: *"does the source actually support this claim?"* A
claim that adds, changes, contradicts, or turns a condition into an obligation is **not
entailed**, and is routed to a residual decision instead of being trusted.

Three things keep it honest and safe:

- **It only ever strengthens.** A confident "not entailed" drops a claim to a residual; a
  clear "entailed" keeps it. But if the model is unavailable or its answer is unclear, the
  gate **abstains** — it leaves the existing rule-based grounding in charge. A model outage
  can never silently delete what SpecForge extracted.
- **It uses a *text* model, not the vision model.** Entailment is pure language reasoning
  (negation, scope, condition-vs-obligation); vision is for figures and diagrams. We
  measured this — the entailment framing markedly outperforms asking the same model to
  re-label from scratch.
- **It never makes the test suite depend on a running model.** The check rides the same
  provider plumbing as the other LLM steps, including a hook that lets the suite mock the
  model's answers — so the gate's logic is fully tested without ever needing Ollama.

This is "models propose, validation decides" made literal: the model's own claim is handed
back to a model — but as a *checkable yes/no entailment question*, with the deterministic
pipeline still holding the final say.

You can run it directly:

```bash
specforge nli-verify path/to/evidence_ir.json
```

It reads an EvidenceIR, asks the text model whether each constraint's source sentence entails
the constraint-as-a-claim (carrying any stated *condition* into the claim — "PSTRB must be LOW
**for read transfers**" — so a conditional constraint is judged fairly), and lists the ones that
are **not entailed** — the likely hallucinations worth a second look. Run on the real AMBA APB
spec it flags genuinely mis-extracted constraints: protocol *states* (`ACCESS`), *width
parameters*, the *clock*, and *condition* signals that were never the obligation's subject. Pass
`--vlm-provider skip` to no-op (the gate abstains on everything) or `--model <name>` to override
the default text model.

To make the gate *active* — not just a report — run it during IntentIR construction:

```bash
specforge intent semantic_ir.json --nli-verify
```

Now any contract whose source sentence doesn't entail it is **demoted into the residual
decisions** rather than passed downstream as a trusted obligation. Demoted, not deleted: it
becomes an honest "this needs review" item, so even a wrong verdict from the model costs a
review, never a lost fact. The gate only touches contracts it can phrase as a clean claim, and
abstains the moment the model is unavailable — so turning it on can *demote* a borderline
contract to a residual, but it can never *invent* one. And because each demotion is recorded in
the artifact, `specforge validate <intent_ir.json>` surfaces how many the gate demoted as the
**`nli_demoted_contracts`** metric — a read-only count that needs no model. *Authoritative
tracking:* `docs/tasks/NLI-ENTAILMENT-VERIFIER.md`, `docs/tasks/NLI-INTENT-GATE.md`,
`docs/tasks/NLI-GATE-METRIC.md`.

## Why provenance matters so much

Every promoted fact should stay tied to the evidence that justified it.

That matters because the project is trying to produce implementation-facing truth, not just plausible summaries.

Provenance makes it possible to:

- inspect where a fact came from
- compare competing evidence
- debug false positives
- keep ambiguity explicit

## Why residuals and conflicts are first-class

When the evidence is not decisive, `specforge` should not silently guess.

That is why the project preserves things like:

- residual decisions
- semantic conflicts
- connectivity conflicts
- temporal conflicts

This is not only a validator feature.
It is part of the core architecture.

## Why infrastructure semantics are separated

Some hardware signals are not ordinary protocol edges.

Clocks and resets are the clearest example.
They can appear in the same tables as other signals, but they carry system-level meaning:

- clocks define the sequential timing reference
- resets define initialization and recovery discipline
- reset assertion and release timing are polarity-sensitive
- sourcing and distribution are infrastructure concerns, not ordinary producer/consumer protocol relations

That is why `specforge` treats clock and reset handling as system-contract infrastructure instead of flattening it into the same category as payload or handshake connectivity.

There is also a document-scope boundary here.
Protocol and chip-interface PDFs are usually shared contracts for RTL designers and verification-IP authors.
They describe the clock/reset semantics that participants can rely on at the boundary, but they rarely define the final chip's physical clock tree or reset tree.
Those trees are normally custom SoC integration work owned by the chip team.

The dedicated chapter [Clock And Reset Infrastructure](domain/clock-reset.md) explains that boundary in more detail.

## Why signal roles are evidence-based

Protocol signal names are useful, but they are not enough.

A signal named `XVALID` may look valid-like, but `specforge` should still prefer grounded role evidence from tables, prose, captions, and timing annotations.

That is why semantic-role recovery is modeled through:

- observations
- candidates
- arbitration
- consensus
- explicit blocked fallbacks when name shape is unsafe

The dedicated chapter [Handshake And Semantic Roles](domain/handshake-semantics.md) explains how that role surface works.

## Why connectivity is graph-first

Flat `input` / `output` labels are useful, but they lose actor perspective.

`specforge` prefers to recover structural facts such as:

- `(Requester, Drives, PSEL)`
- `(Completer, Reads, PSEL)`

Those facts explain who produces and consumes a signal, and they can later support actor-relative ports, signal connectivity, and actor-grounded temporal predicates.

The dedicated chapter [Actor Connectivity And Graph Direction](domain/actor-connectivity.md) explains that graph-first direction model.

## Why timing is typed

Timing prose is too important to remain only as text.

`specforge` turns recoverable timing obligations into typed temporal rules with:

- clock edge
- tick phase
- predicates
- cycle windows
- actor grounding
- explicit conflict records

The dedicated chapter [Temporal Semantics And Timing](domain/temporal-semantics.md) explains that temporal model.

## Why learning is symbolic and explicit

`specforge` can get stronger across many documents, but the thing that grows is not hidden neural state.

What grows is an explicit typed prior store:

`generated/prior_memory/corpus_memory.json`

That store learns reusable extraction knowledge such as:

- actor vocabulary
- semantic phrase patterns
- temporal phrase patterns
- table-shape patterns

The critical safety boundary is:

- priors widen interpretation of the current document
- priors do not directly author canonical truth

## Why document truth and cross-document experience stay separate

The current document pipeline should remain provenance-pure.

That means:

- canonical IR facts must be justified by the current PDF
- earlier documents may help the extractor interpret new evidence
- earlier documents must not leak their facts directly into new canonical artifacts

This is one of the most important design boundaries in the whole project.

## Why this architecture should scale

The architecture is designed to improve in two directions at once:

- stronger per-document truthfulness
- stronger cross-document extraction experience

That is the reason for the layered shape:

- document-local canonical truth
- cross-document prior memory
- and, over time, a broader corpus knowledge-base layer

`specforge` is trying to become expert-like by accumulating reusable extraction knowledge, not by replacing the document pipeline with a black box.

## Why the design is grounded in published research

Almost everything `specforge` does — turning PDFs into structured documents, lowering
meaning through staged representations, pulling normative requirements and protocol timing
out of prose, building a knowledge graph of actors and signals, fusing text with tables and
figures, using a bounded LLM, learning across documents, measuring its own recall, and
handing off to hardware tooling — is something a research community has already studied for
years.

That is good news, and we treat it as such. It means the project does not have to invent
its foundations from scratch: where a problem is settled in the literature, `specforge`
should *adopt the proven approach and the standard vocabulary* rather than re-derive a
weaker version. And it means the genuinely new ideas can be named and defended, instead of
being lost in the noise.

So one task-tree (`LITERATURE-GROUNDING`) deliberately maps each part of the design onto its
prior art. For every aspect it records three things: where `specforge` already matches the
established work (validate the design), which techniques are worth borrowing (adopt), and
where `specforge` genuinely does something the literature does not (claim it). A short
unifying map and a prioritized "what to improve next" backlog live alongside the per-aspect
write-ups under `docs/research/grounding/`.

A few threads recur across every aspect — these are the parts worth claiming as genuinely
ours:

- **We work forwards from the spec.** Most of the related work (program-invariant mining,
  assertion generation, specification mining) works *backwards* from an existing
  implementation — traces, RTL, or code. `specforge` recovers intent from the
  human-authored specification *before* any implementation exists.
- **When the evidence is not decisive, we keep a structured record instead of guessing.**
  The wider field usually stops at a confidence number or simply drops the uncertain case.
  `specforge` emits a first-class, provenance-carrying residual decision — uncertainty stays
  visible and auditable.
- **We estimate the misses we cannot see.** Borrowing capture–recapture from software
  inspection, `specforge` gives an honest lower bound on what it *failed* to extract, not
  just a score on what it found.

There is one non-negotiable rule behind all of this: **every citation must be real and
verifiable.** A literature-grounding effort is only trustworthy if its references resolve.
Each source is checked against a resolvable identifier (an arXiv id, DOI, RFC number, ISBN,
IEEE standard, or stable URL), the riskiest recent works are double-checked by hand, and any
source that cannot be confirmed is dropped — never guessed. The goal is to help you trust
that `specforge` stands on real ground, and to show you exactly where it goes beyond it.

The full grounding survey — one document per aspect plus the unifying synthesis map — lives
in `docs/research/grounding/` (`README.md` is the synthesis and index).
