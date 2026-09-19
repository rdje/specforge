# BOUNDED-DECISION-PROVIDER: a constrained decision model may rank candidates the rules already found, and nothing else

## Metadata

- Tree ID: `BOUNDED-DECISION-PROVIDER`
- Status: `active`
- Roadmap lane: `R15c`/`R15d` (convergence + arbitration), with a required `ROADMAP.md` amendment — see `.2`
- Created: `2026-09-19`
- Owner: repo-local workflow

## Goal

Decide, with measurement rather than enthusiasm, whether a **System One** decision model — TypeSafe's
**Jev** (`jev-1.13.0`) is the concrete instance — earns a bounded place in SpecForge, and if so install it
behind a contract narrow enough that it cannot become an authority.

The capability being evaluated is **not** generation and **not** reading. It is *ranking among candidates
the deterministic extractors already found*, which is the one thing the product currently has no good
answer for: several open trees are precision/recall defects where a hand-written rule keeps mis-firing and
no amount of further rule-writing separates the cases.

## The finding that makes this worth a tree

Jev returns typed values from a closed set with a probability distribution, and **cannot emit a string**.
The docs' pre-parsed extraction pattern is the load-bearing property:

> "Because TypeSafe only ever chooses among the spans the regex found, the value you get back is one of
> those spans, copied unchanged. **It cannot invent a value or transpose a digit.**"

So the shape is: **code finds candidate spans → model selects among them → code normalizes.** The output is
a span that already exists in the source with its provenance intact. That is a ranking function over
deterministic candidates, not a generator, and it is the only shape that can pass ADR 0006 — because the
production core never receives a symbol the model authored.

The open defects are all that shape:

| Tree | The decision | Primitive |
| --- | --- | --- |
| `SIGNAL-DECLARATION-ROW-DROP` | is this row a declaration? (18.3% discarded today) | Noul |
| `PROSE-NAME-CELL-DECLARATION` | which word of the phrase is the signal? | Choice over found spans |
| `TEXT-LAYER-IDENTIFIER-SPLIT` | one identifier, or two? | Noul |
| `INVARIANT-SHAPE-ADMISSION` | admit or reject this candidate? (739 captions) | Noul |
| `ACTOR-NOUN-RELATION-DECLARATION` | is this ordinary noun a wire? | Noul |

## Non-Goals

- **Generation of any kind.** No symbol, name, value or explanation may originate from the model.
- **Any role inside the proof kernel.** The vendor states there is no guarantee that
  `P(noul) == 1 - P(not_noul)`, nor that results across primitives cohere arithmetically. A source whose own
  probabilities are not complementary cannot sit inside a kernel carrying 120/120 conservation.
- **Reading numbers.** Jev 1.13 is documented weak on "hex values, binary, and low-level numeric formats"
  and on counting. SpecForge extracts bit positions, widths, enum rows and reset values. The model may
  *choose between* numbers the code already parsed; it may never *read* one.
- **Document-level input.** 64k context / 32k state, and "unrelated detail acts as a distractor". Spans only.
- **Replacing a deterministic rule that works.** The roadmap's order stands: deterministic extraction first
  where the surface is crisp; this is for the residue that survives it.
- Re-opening ADR 0006 or ADR 0038.

## Burden of proof — the default is REJECT

**Nothing is integrated because it is interesting.** This tree starts from rejection and the provider has
to earn its way out, against a bar fixed **before** it is measured:

1. **The bar is pre-registered in `.1`, in writing, before any model sees the data.** A threshold chosen
   after seeing the result is not a threshold. `.1` publishes the minimum precision/recall delta that would
   justify a remote dependency, and `.6` is held to that number and no other.
2. **A marginal win is a rejection.** The cost side is not zero and must be beaten, not tied: a network
   dependency on a service with **no stated version-availability policy**, a doctrine amendment to
   `ROADMAP.md:37`, corpus spans leaving the volume, and a permanent offline-cache obligation so claims stay
   re-derivable. A couple of points of precision does not pay for that.
3. **The honest alternative is named up front: fix the rule instead.** `.1` may well show that the 18.3% of
   rows `SIGNAL-DECLARATION-ROW-DROP` discards are a *rule defect* with a deterministic fix, not genuine
   ambiguity. If the disagreement rows share a repairable pattern, the right answer is a better extractor
   and **no provider at all** — which is strictly preferable, keeps everything local, and costs nothing to
   maintain. `.1` is designed to expose that outcome, not to hide it.
4. **Per-decision, not blanket.** Adoption applies only to decisions that individually cleared the bar and
   passed the `.4` identity test. There is no "we adopted Jev"; there is at most "this decision is
   adjudicated, these are not."

## What must be PROVEN before integration

**Director's rule (`2026-09-19`): integration is a serious matter. It must bring something substantial
that SpecForge does not have right now — and that has to be proven, not argued.** It is not all-or-nothing
scoring; it is one central question with a few hard lines around it.

### The central question: is the capability actually NEW?

Beating today's rules is **not** the bar, because today's rules are known-defective — that is why the
defects have trees. A remote dependency only earns its place if it delivers something no local option can.
So the decisive test is a **three-arm comparison** on the same frozen set:

| Arm | What it is | Owned by |
| --- | --- | --- |
| **A — status quo** | the current deterministic rules, scored per row | `.1` |
| **B — best local alternative** | the honest local fix: a repaired extractor rule, and/or the local NLP tier the roadmap already mandates, run through `system-one-adapter-python` against Ollama so B uses the **identical** harness | `.1a` |
| **C — Jev** | the same decisions under the `.3` contract | `.5` |

**Integration is justified only if C beats BOTH A and B by a pre-registered material margin.** The arm
that matters is **B**, not A. If a repaired rule or a local model closes the gap, then Jev brings nothing
SpecForge does not already have, and the correct answer is to fix it locally — cheaper, no egress, no
doctrine amendment, no vendor dependency, and it keeps `ROADMAP.md:37` intact.

**What "substantial" means is fixed in `.1` before any model runs**, as a named per-row delta on a named
decision, so it cannot be renegotiated once results exist. A marginal or ambiguous win is a rejection: the
cost side is a roadmap amendment, corpus spans leaving the volume, a permanent offline-cache obligation,
and a vendor with **no stated version-availability policy**.

### The adapter changes how arm B is run, and how much lock-in costs

`github.com/typesafe-ai/system-one-adapter-python` (**MIT**) is "a drop-in replacement for `typesafe_sdk`'s
`system_one` evaluation API, backed by LLM APIs instead of TypeSafe." It implements the same
`Choice`/`Score`/`Noul` interface, supports `llm_answer_mode="probabilities"`, and accepts an
OpenAI-compatible `base_url` — which means it points at **Ollama**, the local provider `README.md:32`
already names.

Two consequences, both good:

1. **Arm B stops being hand-rolled.** The same harness, the same questions, the same per-row scoring, with
   only the backend swapped. That removes the biggest threat to this comparison's honesty — that arm B was
   a weaker implementation rather than a weaker model — and it lets `.1a` run **entirely locally, zero
   egress**.
2. **Lock-in drops sharply.** If SpecForge's integration is written against this interface rather than
   against Jev's client, the provider becomes swappable, and one of the larger costs on the rejection side
   of the ledger shrinks. `.3` must write the contract against the interface for this reason.

**The fairness risk, named because it is real:** the adapter is authored by the vendor and exists
explicitly to enable "cost and performance comparisons" — it is the vendor's own tool for measuring its
competition. That is not an accusation, and MIT source makes it auditable, but it does mean a
suspiciously weak arm B must not be accepted at face value. If B underperforms, `.1a` reads the adapter's
prompt construction and, if it is doing the local model no favours, re-runs B through a direct
implementation before concluding anything.

**What the org does NOT contain:** no self-hostable model, no local runtime, no published evaluation or
benchmark harness — only client SDKs, the adapter, and an agent-skills repo.

**Jev is hosted-only and a key IS required — verified directly, because the opposite was proposed and it
would have changed the plan.** The quickstart and Python SDK usage pages state it plainly: the SDK reads
**`TYPESAFE_API_KEY`** from the environment, authenticates with `Authorization: Bearer <API_KEY>`, calls
`https://api.typesafe.ai` (overridable via `TYPESAFE_BASE_URL`), and keys are issued from
`https://console.typesafe.ai/keys`. Nothing documents running Jev locally or offline.

**The distinction that makes this confusing is worth stating once:** the *adapter* runs locally, but the
adapter is **not Jev** — it is explicitly "backed by LLM APIs **instead of TypeSafe**". Running locally
therefore buys arm **B**, not arm **C**. There is no configuration in which Jev itself is evaluated
without egress and without a key, so **`.2` stands exactly as written.**

### Credential handling — settled before a key exists

`TYPESAFE_API_KEY` is supplied by the director and is **environment-only**. It is never committed, never
written into a task tree, a claim record, a fact card, or anything under `generated/`, and never printed
by a checker. `.gitignore` was closed against `/.env` and `/.env.*` in this tree's opening commit, before
any key existed, because the gap was there and a credential file would otherwise have been committable.

### The hard lines — genuinely fatal, and few

These are correctness and doctrine, not value. Any one of them failing ends it regardless of how good the
numbers look, because a fast wrong answer is worse than no answer:

| # | Disqualifier | Owning leaf | Why fatal |
| --- | --- | --- | --- |
| **D1** | **Identity dependence** — answers shift under consistent alpha-renaming, or improve when the document is identifiably AMBA | `.4` | this is precisely the `WIRE-BASED-100.8c` SWD failure: a model reading protocol identity fakes competence and ADR 0006 exists to forbid it |
| **D2** | **Reads a number** — any decision where the model reads a hex, binary, bit-range or counted value rather than choosing among code-parsed candidates | `.3`/`.5` | Jev 1.13 is documented weak exactly here and SpecForge's register/bit-field domain is exactly here |
| **D3** | **No offline re-derivation** — a decision that cannot replay from a repository-local digest-pinned cache with the network down | `.3`/`.5` | the vendor states no version-availability policy; an un-replayable claim cannot satisfy `CLAIM_VERIFICATION` |
| **D4** | **Egress refused** — the `ROADMAP.md:37` amendment is declined, or ZDR is not obtainable | `.2` | director's call, and it ends the tree cleanly at any point |

### Measured and reported, but weighed rather than fatal

These inform whether the win is real and affordable; a poor result here is evidence against value, not an
automatic kill: **confidence separation** (does the score actually distinguish right from wrong answers on
the frozen set — the docs make *no* calibration guarantee), **run-to-run stability** at a pinned version,
**adversarial resistance** to instruction text inside a corpus span, and **measured cost and latency** on
the real corpus rather than the quoted price list.

### The outcome most likely to end this early, and it is a good one

`.1a` may simply win. If the disagreement rows turn out to be a repairable rule defect, or the local NLP
tier handles them, this tree closes with a **better extractor, no vendor, and no doctrine change** — and
`.1`'s frozen per-row baseline survives as the thing SpecForge was missing all along. That is a success,
not a wasted tree, and the G-suite becomes the standard the next provider is measured against instead of
being re-argued from scratch.

## Acceptance Criteria

- A measured baseline exists for each candidate decision **before** any model sees it, so "it helped" is a
  comparison and not an impression.
- The bounded-use contract is a decision record, mechanically enforced, not a convention.
- Genericity is **measured** under the existing behavioral harness, not assumed.
- Every published claim resting on a model decision re-derives offline from recorded evidence, without a
  network call.
- Adoption or rejection is recorded with the measurement that decided it.

## Task Tree

- ID: `BOUNDED-DECISION-PROVIDER`
  Status: `active`
  Goal: decide whether a constrained decision model earns a bounded place, and install the boundary if so
  Children: `.1`, `.1a`, `.2`, `.3`, `.4`, `.5`, `.6`

- ID: `BOUNDED-DECISION-PROVIDER.1`
  Status: `pending`
  Goal: **establish the deterministic baseline, with zero egress.** Build a frozen, labelled adjudication
  set from existing reviewed golds for the two defects with the cleanest oracles —
  `SIGNAL-DECLARATION-ROW-DROP` (18.3% of given rows discarded) and `INVARIANT-SHAPE-ADMISSION` (739
  captions) — and measure what the current rules score on it: precision, recall, and the exact rows they
  disagree with the gold on.
  **This leaf is worth doing whether or not Jev is ever adopted**, and it is the reason it goes first: today
  those two defects are quantified as aggregate percentages, not as a per-row set a future change can be
  scored against. Without it, "the model improved things" is unfalsifiable.
  No network call. No dependency on `.2`.
  **It also pre-registers the bar, and classifies the failures.** Two deliverables beyond the baseline
  number. First, the minimum delta that would justify a remote dependency, written down before any model
  runs. Second, a read of the disagreement rows asking the question that could end this tree early: **are
  these ambiguous, or is the rule simply wrong?** If they share a repairable pattern, the finding is an
  extractor fix and this tree closes at `.6` with no provider — the better outcome, and one only this leaf
  can surface.
  Acceptance: a repository-local, digest-pinned adjudication set with per-row gold labels; a baseline score
  per defect re-derivable by one command; the disagreement rows enumerated, not summarised; a pre-registered
  adoption threshold; and a stated classification of the disagreements as rule-defect or genuine ambiguity.
  Prerequisite: none.
  Verification: `pending`
  Commit: `pending`

- ID: `BOUNDED-DECISION-PROVIDER.1a`
  Status: `pending`
  Goal: **build arm B — the best honest local alternative — and give it a fair run.** This is the leaf that
  decides whether Jev brings anything new, so it must not be a strawman. Two sub-arms on `.1`'s frozen set:
  **B1, the repaired rule.** Take the disagreement rows `.1` enumerated and attempt a deterministic fix.
  If they share a pattern, this is the whole answer.
  **B2, the local model tier, through the same interface.** `system-one-adapter-python` (MIT) is a drop-in
  `TypeSafeClient` replacement that accepts an OpenAI-compatible `base_url`, so Ollama — already a declared
  dependency — answers the *same* `Choice`/`Noul` questions through the *same* harness with
  `llm_answer_mode="probabilities"`. Only the backend differs, which is the only way this comparison is
  worth anything.
  **Audit the adapter before trusting a weak B.** It is the vendor's own comparison tool. MIT source makes
  that auditable; if B underperforms, read its prompt construction and re-run B directly before concluding
  the local model is worse.
  **The trap this leaf exists to avoid:** measuring Jev only against known-broken rules, finding it better,
  and concluding it is necessary. Almost anything beats a defective rule. The only comparison that supports
  a remote dependency is against the best thing that can be done locally.
  **Fairness is part of acceptance.** B must get comparable effort to C — the same frozen set, the same
  span construction, the same per-row scoring — and the leaf states what was tried and what was not, so a
  later reader can judge whether B was given a real chance.
  Acceptance: B1 and B2 scored per row on `.1`'s frozen set by one re-derivable command; a written
  statement of the effort each arm received; if B closes the gap, this leaf recommends shipping the local
  fix and closing the tree at `.6` with no provider.
  Prerequisite: `.1`. **Zero egress** — Ollama is local; no call to any remote provider.
  Verification: `pending`
  Commit: `pending`

- ID: `BOUNDED-DECISION-PROVIDER.2`
  Status: `pending` — **DIRECTOR'S DECISION, and it is larger than it first looked**
  Goal: **adjudicate corpus egress, which is a ROADMAP AMENDMENT and not only a data-policy call.**
  `ROADMAP.md:37` mandates "use AI/VLM/NLP as bounded **local** hypothesis generators"; `README.md:32`
  names Ollama as the default local path with LM Studio as fallback. **Jev has no self-hosted, on-prem or
  VPC option anywhere in its documentation** — it is API-only. So adopting it does not merely stretch
  `PROJECT-DATA-LOCALITY`; it contradicts a standing cross-cutting doctrine line, and that line must be
  amended deliberately or the provider must be refused.
  What would actually leave the volume, stated precisely rather than reassuringly: **spans, not documents**
  — a candidate row or cell plus surrounding context, bounded by the 32k state limit. Not the PDFs, not the
  bundles. But 21 of the 24 current chains are **authorized external vendor specifications**, and whether a
  licensed spec's text may transit a third-party API is a licensing question this repository cannot answer
  from its own files — it has no project licence, and the corpus is vendor material.
  Mitigations on the vendor side, recorded as found: TypeSafe commits to **not training on customer data**
  and offers **Zero Data Retention** for enterprise (via `privacy@typesafe.ai`). That addresses
  confidentiality. It does not address locality, and it does not address licensing.
  **DIRECTION AUTHORIZED `2026-09-19`.** By electing to procure a `TYPESAFE_API_KEY` the director accepted
  bounded egress in principle, so this leaf is no longer a question handed back — it is a drafting task.
  What remains is the written doctrine change, which must not be skipped just because the intent is clear:
  a decision record amending `ROADMAP.md:37` from "bounded **local** hypothesis generators" to admit a
  bounded **remote** decision provider under stated conditions, `PROJECT-DATA-LOCALITY` updated to match,
  the ZDR arrangement confirmed in force, and the conditions themselves named — spans not documents, the
  32k state bound, no corpus artifact or PDF transmitted, and the offline cache from `.3` mandatory so no
  claim depends on the service remaining reachable.
  Acceptance: that decision record merged, or — if the amendment is refused on reflection — this tree
  closes at `.6` with `.1`/`.1a` retained as the lasting value. **No network call before this leaf closes**,
  independent of whether a key exists.
  Prerequisite: none. Blocks `.4`, `.5`.
  Verification: `pending`
  Commit: `pending`

- ID: `BOUNDED-DECISION-PROVIDER.3`
  Status: `pending`
  Goal: **write the bounded-use contract and make it mechanically enforced, not conventional.** Design only;
  no network. The contract:
  1. **Selection only.** Every question is a `Choice` over spans the deterministic pass already found, or a
     `Noul` over an already-extracted candidate. A question whose option set is not derived from located
     spans is refused.
  2. **No number is read.** Numeric values are parsed by code; the model may only choose between parsed
     candidates. Enforced by refusing questions whose options are raw numeric literals.
  3. **Never a proof carrier.** A model decision enters as a typed hypothesis with its own provenance and
     must pass the existing schema/grounding/conflict/convergence checks. It may never mint a proof claim.
  4. **Versioned pinning.** Call with the versioned ID `jev-1.13.0`, never the `jev-latest`/`jev-preview`
     aliases, which move silently. Record the `model` field the response returns — the vendor reports which
     versioned model actually answered, and that is the producer identity a claim record needs.
  5. **Offline re-derivation.** Every decision is cached to a repository-local, digest-pinned record so a
     published claim re-derives without a network call. This is not an optimisation: the vendor states **no
     deprecation or availability policy** for old versions, so an un-cached claim becomes unre-derivable the
     day `jev-1.13.0` retires.
  6. **Write against the adapter interface, not the vendor client.** `system-one-adapter-python` (MIT)
     defines a provider-neutral `Choice`/`Score`/`Noul` surface. Binding SpecForge to that interface keeps
     the provider swappable and is a precondition for adoption, not a nicety — it is most of what makes a
     remote dependency reversible.
  7. **Credentials are environment-only.** `TYPESAFE_API_KEY` from the environment; never committed, never
     in a tree, claim record, fact card or `generated/`, never echoed by a checker.
  8. **The middle band is a residual.** Following the vendor's own entity-alignment pattern, a three-level
     score whose middle level routes to a typed residual / clarification candidate rather than forcing a
     binary. This is the roadmap's "preserve ambiguity explicitly" expressed in the provider's own idiom.
  Acceptance: a decision record; a checker that refuses a question violating rules 1, 2 or 4, with a
  controlled RED case each; registered in `scripts/check_doctrines.sh`.
  Prerequisite: none (design is independent of `.2`; only execution is blocked).
  Verification: `pending`
  Commit: `pending`

- ID: `BOUNDED-DECISION-PROVIDER.4`
  Status: `pending`
  Goal: **measure whether the provider carries protocol identity — the ADR 0006 gate.** SpecForge already
  owns the oracle: the behavioral genericity harness does alpha-renaming, adversarial document identity and
  reviewed paraphrase. Run the candidate decisions through it. If the model's answer changes when a symbol
  is consistently renamed, or improves when the document is identifiably AMBA, it is reading identity and is
  disqualified for that decision.
  **This is not hypothetical.** `WIRE-BASED-100.8c` retired SWD's 29/29 because the frame extractor
  recognised the protocol by name. A model trained on public specifications is a far larger version of the
  same hazard, and the harness is the reason it can be caught rather than argued about.
  Acceptance: an identity-sensitivity measurement per candidate decision, with the alpha-rename and
  adversarial-identity deltas published; any decision showing identity dependence is excluded by name.
  Prerequisite: `.2` (needs a live call), `.3` (needs the contract).
  Verification: `pending`
  Commit: `pending`

- ID: `BOUNDED-DECISION-PROVIDER.5`
  Status: `pending`
  Goal: **one bounded trial — arm C — scored against BOTH arm A and arm B.** A single defect — `SIGNAL-DECLARATION-ROW-DROP`
  is the strongest candidate, being the highest-volume and having the clearest gold — adjudicated under the
  `.3` contract, measured on the `.1` set, reported as a delta with its disagreement rows enumerated.
  Report the cost too: the vendor quotes `$0.042/MTok` in, free output, 70–500 ms; measure what the corpus
  actually costs rather than quoting the price list.
  Acceptance: precision/recall delta against **both** arm A and arm B on the frozen set, judged against `.1`'s
  **pre-registered** threshold and not a threshold chosen afterwards; per-row disagreements published;
  measured latency and spend; and an explicit statement of what it did **not** improve. A result that
  clears the bar only after the bar is moved is recorded as a rejection.
  Prerequisite: `.1`, `.1a`, `.2`, `.3`, `.4`.
  Verification: `pending`
  Commit: `pending`

- ID: `BOUNDED-DECISION-PROVIDER.6`
  Status: `pending`
  Goal: **adopt, reject, or defer — recorded with the measurement that decided it.** Adoption requires
  arm **C to beat both A and B** by `.1`'s pre-registered margin, with no disqualifier D1–D4 fired. A win
  over A alone is **not** adoption: it means the local fix in `.1a` should ship instead. An adoption must name the decisions it covers and the ones it does not. A rejection
  must say what would change the answer, and must keep `.1`'s baseline and the G1–G11 suite as the durable
  product — so the next provider is measured, not re-argued.
  Acceptance: a decision record; `ROADMAP.md` and the mdBook updated to whichever answer landed; any claim
  published by `.5` registered under `CLAIM_VERIFICATION.md` with its offline re-derivation.
  Prerequisite: `.5`.
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `BOUNDED-DECISION-PROVIDER.1` | `pending` | zero egress, no prerequisites, and valuable even if the tree ends in rejection — without a per-row baseline no later claim of improvement is falsifiable |
| 2 | `BOUNDED-DECISION-PROVIDER.1a` | `pending` | zero egress, and the arm that actually decides the question — if the local fix closes the gap, the tree ends here with a better extractor and no vendor |
| 3 | `BOUNDED-DECISION-PROVIDER.3` | `pending` | design is independent of the egress decision; writing the contract first means `.2` is decided against a concrete boundary rather than an open-ended dependency |
| 4 | `BOUNDED-DECISION-PROVIDER.2` | `pending` | direction authorized by the key procurement; what remains is drafting the `ROADMAP.md:37` amendment, which still gates every network call |
| 5 | `BOUNDED-DECISION-PROVIDER.4` | `pending` | **blocked on `TYPESAFE_API_KEY` procurement**; the ADR 0006 gate, and nothing proceeds past an identity-dependent model |
| 6 | `BOUNDED-DECISION-PROVIDER.5` | `pending` | **blocked on `TYPESAFE_API_KEY` procurement**; the trial — arm C, scored against A and B |
| 7 | `BOUNDED-DECISION-PROVIDER.6` | `pending` | the decision — adopt only if **C beats both A and B** by the pre-registered margin and no disqualifier D1–D4 fired |

## Decisions

- `2026-09-19`: **Scope the yes narrowly, and name what it excludes.** The evaluation is admitted for
  *ranking among deterministically-found candidates* and refused for generation, number reading,
  document-level input, and any role inside the proof kernel. The reason a yes is possible at all is that
  the model cannot emit a string: the production core therefore never receives a symbol the model authored,
  which is the property ADR 0006 actually protects.
- `2026-09-19`: **The bar is "something SpecForge does not have right now", proven by a three-arm
  comparison** (director). Beating the current rules is not enough, because those rules are known-defective
  — that is why the defects have trees. Jev must beat the **best local alternative** (a repaired rule, or
  the local NLP tier the roadmap already mandates), by a margin fixed before any model runs. Four
  disqualifiers stay genuinely fatal because they are correctness and doctrine rather than value:
  identity dependence, numeric reading, no offline re-derivation, egress refused. Everything else is
  measured and weighed. An earlier revision of this tree made all eleven checks fatal; that was stricter
  than the director asked for and mis-stated the question, which is about new capability rather than
  passing a checklist.
- `2026-09-19`: **The default is rejection and the bar is pre-registered.** Per the director: it must bring
  something real to the table before integration. Encoded rather than promised — `.1` fixes the threshold
  before any model runs, a marginal win counts as a failure because the cost side is a doctrine amendment
  plus a permanent remote dependency, and adoption is per-decision rather than blanket. The outcome the
  tree most wants to find early is that the defects are repairable rules, which needs no provider at all.
- `2026-09-19`: **`.1` before `.2`, deliberately.** The baseline costs nothing, leaks nothing, and is the
  only thing that makes a later improvement claim falsifiable. Ordering it after the policy decision would
  have made the tree's value contingent on an answer nobody has yet.

## Open Questions

- Does the 32k state bound leave enough surrounding context for a declaration-row decision, or does the
  span have to carry its table header and footnotes to be decidable? Measurable in `.1` without a provider.
- Jev's documented weakness on low-level numerics overlaps SpecForge's register/bit-field domain. The
  pre-parsed pattern mitigates it, but whether it mitigates it *enough* for bit-range adjudication is an
  open measurement, and register work should stay out of scope until `.5` reports.

## Blockers

- **BLOCKED ON PROCUREMENT: `TYPESAFE_API_KEY`.** Director, `2026-09-19`: "we will need `TYPESAFE_API_KEY`
  to test anything Jev related… block it on me buying an API KEY from Typesafe." Verified against the
  vendor docs — Jev is hosted-only, authenticates with `Authorization: Bearer <API_KEY>` against
  `https://api.typesafe.ai`, and keys are issued from `https://console.typesafe.ai/keys`. There is no
  configuration that evaluates Jev without a key.
  **This blocks arm C only** — `.4` and `.5`. Nothing else waits on it.
- **NOT blocked, and they are the work that matters first:** `.1` (frozen per-row baseline + pre-registered
  margin) and `.1a` (arm B, local, through the adapter against Ollama) are **zero-egress and keyless**, and
  `.3` (the contract) is design. All three can complete before a key exists, and together they decide
  whether arm C is even worth running: if `.1a` closes the gap locally, the key is never needed.
- `.2`'s roadmap amendment is a drafting task, no longer a question — see that leaf.

## Changelog

- `2026-09-19`: Created, on the director's greenlight, after reading TypeSafe's System One announcement and
  the `docs.typesafe.ai` reference. The verdict recorded: yes, narrowly — the `pick`-among-found-spans
  pattern is a genuine fit for SpecForge's open precision defects, and the model's inability to generate a
  string is what makes it admissible under ADR 0006. Two findings sharpened the scope. Jev 1.13 is
  documented weak on exactly the low-level numeric formats SpecForge extracts, so it may never read a
  number. And adopting it is a **roadmap amendment**: `ROADMAP.md:37` requires bounded *local* hypothesis
  generators and Jev is API-only with no self-hosted option, which makes `.2` a doctrine change rather than
  a preference.
- `2026-09-19`: Blocked on procurement at the director's instruction — a `TYPESAFE_API_KEY` must be bought
  before anything Jev-related is tested. Confirmed against the vendor docs that no keyless path exists:
  hosted-only, bearer auth, keys from the console. The block is scoped to **arm C** (`.4`, `.5`); `.1`,
  `.1a` and `.3` are keyless and proceed now, and if `.1a` closes the gap locally the key is never needed.
  `.2` also stops being a question: procuring the key authorizes the direction, leaving the
  `ROADMAP.md:37` amendment as a drafting task rather than an open ask.
- `2026-09-19`: `github.com/typesafe-ai` reviewed. The decisive artifact is
  **`system-one-adapter-python`** (MIT): a drop-in `TypeSafeClient` replacement over ordinary LLM APIs that
  accepts an OpenAI-compatible `base_url`, so arm B can run through the *identical* harness against Ollama
  with zero egress, and SpecForge's integration can be written provider-neutral so Jev stays swappable.
  Recorded with its bias risk: it is the vendor's own comparison tool, so a weak arm B gets audited rather
  than believed. The org contains **no self-hostable model, no local runtime and no eval harness**, so the
  egress position and `.2` are unchanged. `.gitignore` closed against `/.env*` ahead of any key existing.
- `2026-09-19`: Bar restated by the director — integration must **prove** it brings something SpecForge
  does not have today. Reframed from a pass/fail checklist to a **three-arm comparison** (A status quo,
  B best local alternative, C Jev) with `.1a` added to build arm B honestly, because beating known-broken
  rules proves nothing. Four disqualifiers remain fatal on correctness/doctrine grounds; the rest is
  measured and weighed.
