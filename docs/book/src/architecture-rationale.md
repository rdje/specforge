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

## Why `specforge` never memorizes a spec's names

`specforge` has to read *hundreds* of different chip-spec PDFs — AMBA today, NVMe and I²C and some
vendor's in-house bus tomorrow — and keep working when any one of them is revised next year. (AMBA
itself renamed "Master/Slave" to "Manager/Subordinate" between revisions; signal names and
encodings drift all the time.) A tool with one spec's vocabulary baked into its code would be wrong
for the next spec and brittle for the next revision of even the same one.

So `specforge` follows one rule, and treats it as a hard correctness requirement:

> **Be smart about *how* to extract things from a spec. Remember how to do it right — never
> remember the particular names.**

The *how* is the intelligence worth keeping: how to read a signal table, how normative English
works (`must`, `shall`, `when`), how a sentence is shaped. The *names* — signal names, value and
state names, protocol and vendor names — are **read from the document in front of it, every time,
and never stored in the code.** (This section uses only placeholders and an obviously-invented
token; deliberately, no real spec's vocabulary appears even as an example.)

### Finding the value by *position*, not by *name*

Consider a value constraint — `⟨signal⟩` is whatever signal the document names, `FOOBARBAZ` a value
the tool has never seen:

```text
⟨signal⟩   must be   FOOBARBAZ
└─signal─┘ └─verb──┘ └──value──┘
```

Every value constraint has the same shape: a signal, a normative verb (`must be` / `shall be` /
`must remain`), and then the value — and the value always lands in the *same grammatical slot*,
right after the verb. So `specforge` does not ask *"is `FOOBARBAZ` a value I know?"* — it never has.
It asks *"what word is sitting in the value slot?"* and takes whatever is there.

|  | by **name** (the old, brittle way) | by **position** (how it works now) |
| --- | --- | --- |
| what it knows | a baked-in list of specific value names | "the value is the word after the normative verb" |
| the question it asks | "is this word in my list?" | "is this word in the value slot?" |
| a new spec's value | invisible — not in the list | found — it is whatever sits there |

That is why a value `specforge` has never seen still extracts cleanly: it reads `FOOBARBAZ` as the
value without being told what `FOOBARBAZ` means. It learned the *grammar* (the how), not the
*vocabulary* (the names). The same idea runs throughout extraction — validate a candidate signal
against the signals *this document declares*, not against a list of one protocol's words.

### Where the line sits

The English the specs are *written in* — `must`, `shall`, `when`, `if` — is the language `specforge`
reads, not a name owned by any spec, so it stays; that is part of the *how*. Anything that genuinely
*belongs to a particular document* — its signals, its values, its protocol name — is derived from
that document. Boundary cases (is some token a universal engineering concept, or a particular spec's
word?) are decided deliberately, and when the convenient choice and the independence-preserving
choice diverge, `specforge` chooses independence.

This rule is binding, not aspirational: it is recorded as an architecture decision
(`docs/decisions/0006-no-hardcoded-chip-spec-vocabulary.md`) and treated as a release-signoff
criterion. A more ambitious future option — letting a language model
read the spec end-to-end — is captured but deliberately parked until the current approach is proven;
even then it would generalize the *how* and still never memorize names.

### Current implementation status: structural proof and held-out remediation complete, population signoff open

The rule above is the required architecture, but an August 2026 whole-production audit found that the
current implementation does not yet satisfy it completely. SourceIR classification, generic protocol evidence,
prior selection, identifier-spelling authority, production model prompts, and corpus-KB routing have now been
repaired. Current-document identities are opaque, downstream stages cannot reconstruct their meaning from
spelling, ISF clock/reset lowering fails closed without a typed system contract, prompts teach only typed
digital-design structure with current-document symbols, and corpus pages group fixtures only by populated typed
schema capabilities. SourceIR, EvidenceIR, SemanticIR, IntentIR, and the ISF adapter now require executable
cumulative derivations at every canonical seam. Adapter authority includes every rendered nonblank ISF line and
every honest blocking reason; proofless or stale input cannot emit or preserve output. A separate enforcement
tool derives the complete Cargo-rooted production syntax graph and fails closed on missing, ambiguous, unparsed,
or unsupported structure. A closed typed registry now drives fixed-point helper/macro information flow over that
graph and rejects raw/identity semantic control, unregistered canonical mutation, protected-authority forgery,
and proofless seams. The structural doctrine and its adversarial qualification are now complete; full
population behavioral qualification remains release-blocking work, not an accepted shortcut.

The bounded identity-remediation parent remains jointly qualified over the exact three-commit range
`89d8dee7..9c38b569`. The later proof migration exposes a stricter current frontier: exactly 25 documents retain
verifiable source capture and current SourceIR/EvidenceIR proof, while 53 historical chains are inspection-only.
SemanticIR, IntentIR, and adapter replay are consequently 25 current / 53 unmeasurable; all 25 current adapters
are honestly blocked and reconcile to zero emitted files. Existing later-stage files for the other 53 documents
are historical, not a substitute for the missing upstream proof chain. The frontier was 24 / 54 at the migration
and moves only by owned re-ingest, one document at a time (`WIRE-BASED-100.9b`, APB, `2026-09-10`). Focused alpha-renaming, identity, prompt, spelling, fixture-name, and fail-closed proposal
controls cover the repaired paths. Structural enforcement is complete; population-level metamorphic
qualification remains mandatory before the implementation can claim this invariant as a whole.

The remediation is tracked by `SPEC-TO-INTENT-ALIGNMENT.6d.ii`. Its proof is deliberately stronger than
a forbidden-word list: the production/conformance dependency boundary must prevent named test knowledge
from entering the core, raw text decisions must go through registered universal grammars, and
alpha-renaming plus filename/title perturbation must preserve the typed result modulo the renaming. A token
census remains useful for diagnostics, but cannot prove neutrality because an unseen alias, hash, threshold,
or neutral-looking corpus phrase could still encode the same coupling.

Proof identity now follows production semantics rather than source-file churn. During compilation, the core
derives one digest per stage from this closure:

```text
canonical production rule registry
          │
          ├── selected production verifier
          ├── referenced local production items
          ├── exact referenced import bindings
          └── complete trusted derivation kernel
                         │
                         ▼
              stage implementation digest
                         │
                         ▼
                 ruleset / proof currency
```

Comments, documentation, formatting, and configured test/conformance branches are removed from that identity,
including nested test-only fields or statements. A referenced helper change or import rebinding changes it;
an unrelated production item does not. Missing or ambiguous registry roots stop the build. Controlled build
mutants prove both directions, and the runtime proves that all five registries consume the generated digests.
The final migration changed only proof/validation metadata across 24 artifacts at each of five stages—120 exact
comparisons—and left every measurable chain current.

This result is deliberately bounded. It proves the identity of the registered production proof relation and
removes false staleness from tests/comments. Canonical verification also reconstructs each stage with the current
binary, so builder or lowering changes still cannot silently authorize a mismatching old conclusion. The
structural graph supplies the deterministic substrate: all four production targets, 77 reachable
production files plus one explicit test-support-only file, and 78 target-module identities are parsed into
module, item, import/re-export/alias, call-candidate, macro-definition, macro-invocation, and attribute nodes.
Exact local calls are distinguished from method, associated, external, prelude/binding, and unresolved compiler
dispatch rather than pretending that an AST alone is a type checker. The information-flow layer now makes the
raw-text, opaque-identity, helper, macro, module, and unregistered-rule boundary mechanically checked. Its
140-row registry contains structural data classes and exact Rust paths—not vocabulary—and its fixed point covers
2,272 functions, 11,909 helper edges, 11,382 decision sites, and 1,444 sensitive macros. Canonical mutation/protected
authority and all proof-only seams are checked independently of taint. Cargo compilation/privacy remains the
type oracle and executable replay remains the semantic oracle. The dependency, inventory, rule, graph, and flow
checks now execute together on every doctrine gate as `PRODUCTION-GENERICITY`. CI also exercises 27 controlled
dependency/schema/rule/flow faults and joins every one of the 170 runtime descriptors to its independently
inventoried structural alpha contract. Final structural qualification covered 15 committed slices and 120
proof-bearing artifacts, and its exact comparison found zero non-proof/non-validation or residual delta. Those
same 24 chain ledgers contained all 170 rule ids and 150,942 cumulative claims when that qualification was
measured; claim totals are re-measured per slice rather than carried as an invariant, and the retained
population has since grown to 25. The other 53 chains remain explicitly proof-unmeasurable.

Population behavioral qualification is active under `.6d.ii.f` and is deliberately relational rather than one
byte-equality gate. `.f.i` has frozen 24 current rows: three repository-owned and 21 portable external PDF
authorities, seven historically exposed reviewed calibration rows, and 17 prospective holdouts. Four holdouts
are vendor-novel and 13 family-novel relative to calibration. Provider-free replay of every retained Markdown
view yields nonempty semantic intent on 23 rows; the one vacuous row is text-unmeasurable, not passed.

The oracle separates input planes. Unchanged replay and adversarial identity use exact PDFs and cover pages,
visuals, structured tables, content elements, and sections. Symbol alpha-renaming and reviewed text variants use
baseline and transformed normalized Markdown, so they qualify downstream text behavior only; they cannot claim
PDF conversion, visual/table capture, or geometry. Six relations classify every field and proof claim through
SourceIR, EvidenceIR, SemanticIR, IntentIR, and adapter lowering. Missing authority/provider or vacuity is
`unmeasurable`; stale/ambiguous/partial evidence is `invalid`; missing expected or undeclared semantic/proof/
provenance/validation/lowering deltas fail. The contract and eight mutations now run inside the production-
genericity wrapper without claiming that the complete population evidence already passes. `.f.ii` now owns
and implements deterministic PDF identity, normalized-text alpha, reviewed paraphrase/layout, and semantic-
negative comparison. It
copies hash-pinned sources into repository-local scratch, executes fresh five-stage pairs, reloads every persisted
proof-bearing artifact, and emits digest-pinned JSON evidence. Source-bound aliases are bijective, complete,
familiar-looking, and deliberately reverse lexical order. Reviewed transforms require a checked-in manifest and
digest-pinned recipe naming every exact span, per-span provenance field, derived-id projection, preserved
conclusion, complete unaffected complement, and rich-capture exclusion. The released calibration passes one
equivalent sentence plus heading, table, whitespace, and emphasis variants across all five stages. Candidate
“at least,” added-line, and heading-rewording variants crossed real parser boundaries and were rejected rather
than excused. The “at least” candidate now acts as a digest-pinned semantic negative: it removes exactly one
SemanticIR assertion plus its cumulative claim at three stages, ordinary invariance must reject it, and the
comparison passes only after stripping those exact deltas. Nine synthetic controls cover omission,
contradiction, reversal, value/timing, undeclared-symbol, misleading-name, proof-corruption, and disabled-stage
faults. Unavailable or vacuous attempts are unmeasurable; stale, ambiguous, escaped, or partial attempts are
invalid. Those remain harness calibrations. The first `.f.iii` prospective execution exposed an omitted
filename-derived stem allowance and an overbroad alpha catalog; its exact diagnostic remains in Git history. The
corrected pre-remediation aggregate was 34 pass / one fail / 16 unmeasurable / zero invalid. Unchanged PDF and
adversarial identity pass 17/17. Fifteen rows have no typed opaque alpha surface and one is vacuous. The I2C row
alone has a measurable six-signal catalog; its bijective 352-occurrence rename exposed four generic coupling
mechanisms. Opaque underscore identifiers had been split into grammar words; relation/conflict ordinals and
signal collections followed lexical spelling; and a familiar word inside a declared alias could influence
section-topic grammar. `.f.iii.a` keeps opaque identifiers atomic, orders source-owned signal collections by
first occurrence, and removes complete declared aliases before interpreting title topics. Context-aware inverse
projection and narrow set/declaration canonicalization close the representation-only comparison surfaces while
non-bijective ids and ordered transaction steps remain exact. The clean-revision aggregate is 35 pass / zero fail /
16 unmeasurable / zero invalid: all 34 PDF relations and I2C alpha execute fresh under revision `2cdcd131`, while
retained evidence supplies only alpha eligibility. The shared-extractor movement is reconciled exactly: all 25 measurable persisted chains are current and zero stale
through EvidenceIR, SemanticIR, IntentIR, and adapter; all 25 adapters remain blocked with no emitted ISF.
Transformation recipes, expected
relations, held-out labels, and comparison evidence remain in conformance; core receives each variant only as
current input. The [behavioral design report](../../research/behavioral-genericity-qualification-design.md)
publishes the exact boundary; the [initial-run report](../../research/behavioral-held-out-initial-run.md)
preserves the diagnostic; and the [corrected qualification](../../research/behavioral-held-out-qualification.md)
publishes denominators, execution provenance, uncertainty, the measured production failure, and its remediated
clean-revision closure. Its reviewed-population replay was 39/0/1 IntentIR TP/FP/FN with 42/42 provenance;
subsequent `.7c.ii` recovery publishes 40/0/0 at all three reviewed stages, 43/43 provenance, zero fabrication,
and zero unexplained drops. Earlier `.f.v` composes the behavioral evidence with the unconditional structural gate
and closes production-genericity signoff within the governed boundary. This does not claim perfect recovery or a
complete PDF-to-ISF product; 16 alpha strata and several coverage/accounting surfaces remain explicitly open.

This finding does not invalidate the project. A specification-neutral extractor may retain universal
digital concepts—signals, fields, registers, states, events, obligations, timing, provenance, conflicts, and
uncertainty—while deriving every document-owned symbol and relation from the current input. The honest ceiling
is not perfect recovery from corrupt, missing, contradictory, or ambiguous source material; it is one
identity-independent engine that promotes only source-justified intent and emits explicit residuals for what
it cannot decide. The complete audit is
[`docs/research/production-genericity-pipeline-audit.md`](../../research/production-genericity-pipeline-audit.md),
and its per-leaf qualification results are
[`docs/research/production-genericity-qualification-results.md`](../../research/production-genericity-qualification-results.md).

### Autonomous-first clarification foundation

The product target is not “run once or fail.” SpecForge should finish every safely decidable branch
autonomously. When required information is absent, contradictory, ambiguous, extraction-limited, or an external
design choice, `SPEC-CLARIFICATION-LOOP` will emit a typed clarification packet instead of guessing.
That packet will identify the exact source/proof context, unresolved engineering proposition, known alternatives,
reason automation stopped, downstream facts or ISF constructs blocked, priority, accepted answer shape and units,
validation rules, and deterministic resume plan.

An answer will be treated as provenance-bearing evidence, not automatic truth. It must be current, typed,
consistent, grounded, and authorized before it can extend canonical proof. Unknown, unavailable, not-applicable,
deferred, stale, incomplete, and conflicting answers remain explicit. Accepted answers will invalidate and replay
only the affected dependency closure, preserving unrelated autonomous work and allowing an interrupted exchange
to resume. Schema-1 clarification and untrusted-answer envelopes now implement the interchange/currentness
foundation; planning, CLI exchange, semantic validation, and proof replay remain subsequent leaves. See
[Clarification and Assisted Completion](pipeline/clarification-loop.md).

### The honest limit of pattern-matching — and what comes after

It is worth naming the ceiling of the current approach plainly, because it is real. `specforge`
recognizes a requirement by matching *grammar* — a normative verb (`must`/`shall`), a behavioral
predicate (`asserted`/`stable`/`driven`), a sentence shape. That is deterministic, inspectable, and
testable without any model. But it has a built-in blind spot: **it can only find what its patterns
already know.** No pattern, no recall. Concretely, a grammar engine seeded on `must`/`shall` walks
straight past:

- **other ways to say "required"** — *"the controller **is required to** drive X"*, *"Y **needs
  to** be stable"*;
- **plain descriptive behavior with no obligation word at all** — *"PCLK **is sampled** on the
  rising edge"*, *"the manager **drives** the address"* — which states a behavior without ever
  saying `must`;
- **imperatives** — *"**Set** the enable bit"*; and
- **another language entirely** — a spec not written in English.

This is the same seed-dependence in two places: the engine needs a list of verbs to *extract*, and
even *growing* that list (mining a corpus, see `VERB-COVERAGE-CORPUS`) needs a seed to find the
verbs in the first place. A bigger, corpus-derived list pushes the ceiling up — it does not remove
it. **No seed, no recall.**

What removes the ceiling is a different tool: a **language model** that recognizes *"this sentence
states a signal's required behavior"* from meaning, with no seed words — across phrasings,
imperatives, even languages. That is genuinely more powerful, and it is the parked future
(`PURE-NLP-INTENT-EXTRACTION`). It is parked, not adopted, on purpose, because the model trades
away exactly what the grammar engine guarantees:

| pattern-matching (now) | language model (parked future) |
| --- | --- |
| deterministic, same answer every run | probabilistic; must be guarded |
| every rule is readable in the code | competence hidden in weights |
| tested with no model in the loop | needs the model to run |
| cannot invent what is not on the page | can hallucinate — needs the entailment check |
| bounded by its seed vocabulary | generalizes across phrasings and languages |

So the order is deliberate: make the inspectable, deterministic engine genuinely excellent first —
including a corpus-wide verb vocabulary so the ceiling is as high as pattern-matching allows — and
only then let a model take over the *how* to reach the rest of the world's specs. Even then, the
invariant holds: it would generalize the method and **still never memorize a name.**

## What extraction actually recovers: actor → verb → signal

Strip the pipeline to its core and there is one job: read prose and recover **who does what to
which signal.** A signal never behaves on its own — *something is always done to it, and the doer is
an actor* (a Requester, a Completer, a clock). So the unit `specforge` hunts for is a triple:

> **actor — verb → signal**

— *"the Manager **drives** HTRANS"*, *"the Completer **samples** PREADY"*. That triple is an edge,
and the collection of edges is the **Knowledge Graph**. That is *why* the KG exists: it is the
natural shape of the thing being extracted.

So recognition has three vocabularies and one assembly step:

- **signals** — recovered from the document's own declarations (never hardcoded — see *"never
  memorizes a spec's names"* above),
- **actors** — the role entities that act,
- **normative verbs** — the edge labels (`drives`, `samples`, `asserts`, `clears`, `polls`, …),
- **the relation** — assembling the triple, typed by *kind*: does the actor **drive** the signal
  (source/change it) or **read** it (observe/sample it)?

This is why *the verb list is the engine*: a verb it doesn't recognize is an edge it cannot draw. So
`specforge` keeps that list as broad as a real corpus of chip specs demands — the behavioral verbs
are mined seedlessly across dozens of vendors' specifications — because **each missing verb is a
missing edge in the graph.** And because verbs are *grammar*, not *names*, the list can grow without
ever tying the tool to one spec.

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

### The measurement survives the terminal — the persisted extraction-quality gauge

A measurement that only scrolls past in a terminal is gone the moment the window closes. So
`nli-verify` now also **persists** what it measured into the EvidenceIR itself, as the
document's **extraction-quality gauge**: which model judged, how many constraints were checked,
how many the source entailed, how many it did *not*, how many the oracle abstained on, and the
exact ids of the not-entailed constraints so review can go straight to the items. The fraction
of constraints the source does not entail is a cheap, automatic production-readiness signal —
it discriminates sharply between a simple peripheral bus (a quarter flagged) and a dense,
deeply conditional spec (the large majority flagged).

Three honesty rules govern the persisted gauge:

- **It is a measurement *about* the extraction, never extraction truth.** Persisting it changes
  no constraint, no relation, no canonical fact. The NLI oracle is also *noisy* — some
  not-entailed verdicts are the judge stumbling on a complex claim — so everything downstream
  reports it as an estimate.
- **A vacuous pass is not a measurement.** If the model was unreachable and every verdict came
  back unknown, nothing is persisted — a dead provider must never overwrite a real prior
  measurement with an empty one.
- **A stale gauge says so.** Rebuilding the EvidenceIR drops the gauge (a new constraint
  surface honestly requires a new measurement), and if the surface changed under a persisted
  gauge — different count, or a measured constraint id that no longer exists —
  `specforge validate` flags the gauge as stale instead of letting an old number masquerade as
  current.

`specforge converge` completes the loop: after the pipeline stabilizes (and after any rescan
step), it re-runs the same measurement over the *final* EvidenceIR and prints the per-document
gauge in its convergence summary — so every full pipeline run ends with a standing,
provider-measured quality report, and `specforge validate <evidence_ir.json>` re-reports it
afterwards without needing any model at all. *Authoritative tracking:*
`docs/tasks/EXTRACTION-QUALITY-GAUGE.md` (leaf `.0`).

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
