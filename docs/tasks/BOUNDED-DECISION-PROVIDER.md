# BOUNDED-DECISION-PROVIDER: a constrained decision model may rank candidates the rules already found, and nothing else

## Metadata

- Tree ID: `BOUNDED-DECISION-PROVIDER`
- Status: `active` (`2026-09-19`; **`.6` decided — REJECTED on measurement, ADR 0051**; `.1`, `.1a.1` and `.6` closed; `.3` open and still worth writing; `.1a.2`, `.2`, `.4`, `.5` open and conditional)
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
  Status: `active` (`2026-09-19`; `.1`, `.1a.1`, `.6` done — the decision is a REJECTION)
  Goal: decide whether a constrained decision model earns a bounded place, and install the boundary if so
  Children: `.1`, `.1a` (`.1a.1`, `.1a.2`), `.2`, `.3`, `.4`, `.5`, `.6`

- ID: `BOUNDED-DECISION-PROVIDER.1`
  Status: `done` (`2026-09-19`)
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

  **DONE `2026-09-19`. The frozen set is 1,257 labelled rows across four documents** —
  `docs/research/bounded-decision-adjudication.jsonl`, sha256
  `d3c5898f657ee6c12c3ef062dbd3d33fe34931a48c08f256ada55a25cf403d05`, produced and scored by
  `scripts/build_bounded_decision_baseline.py`. It carries every row's header, caption and cells
  **verbatim**, so the score re-derives from the tracked file alone and never depends on untracked
  `generated/` state. Full record: `docs/research/bounded-decision-baseline.md`; fact card
  `[[bounded-decision-frozen-baseline]]`.

  | decision | rows | gold + | tp | fp | fn | tn | precision | recall | macro-F1 | errors |
  | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
  | `declaration_row` | 644 | 589 | 567 | 0 | 22 | 55 | `1.0000` | `0.9627` | `0.90715` | 22 |
  | `caption_admission` | 613 | 6 | 2 | 5 | 4 | 602 | `0.2857` | `0.3333` | `0.65014` | 9 |

  **Three findings, and two of them bear directly on whether `.5` is worth running.**

  **(1) `declarations_emitted` is not the reader's final answer, and scoring it would have published
  six false positives the product does not make.** AXI `table_0011` records 6 emitted declarations and
  **zero** `table_signal_declaration_provenance` records, because
  `withhold_base_name_template_declarations` (`WIRE-BASED-100.10b`) removes a base-name template
  table's declarations after every table producer has run. Arm A's real precision on
  `declaration_row` is `1.0000`, not `0.9895`. The frozen set therefore records three reader states —
  `emitted`, `withheld`, `dropped` — and `--self-test` case `withheld-is-not-emitted` keeps it that
  way.

  **(2) The `declaration_row` decision is a TABLE property in 101 of 101 tables. Not one is mixed.**
  That reproduces `[[a-dropped-declaration-row-is-usually-not-a-signal]]`'s ten-table finding on a
  population ten times larger. A per-row `Noul` — the exact primitive this tree is evaluating — is
  therefore being asked a question whose answer is settled one level up, so its whole achievable win
  is re-deciding eight tables, and the three table-level discriminators already tried and refuted
  remain the honest frontier.

  **(3) The classification the leaf demanded: none of the 31 errors is genuine ambiguity.**

  | decision | cause | rows |
  | --- | --- | ---: |
  | `declaration_row` | name-cell selection — the name is not in the column the reader chose; a *different* decision (`PROSE-NAME-CELL-DECLARATION`) | 6 |
  | `declaration_row` | no attribute stated anywhere — the standing question, answered NO three times | 8 |
  | `declaration_row` | attribute lost by ingest — the text layer merged or scattered the direction | 7 |
  | `declaration_row` | abbreviated direction refused under a `Direction` header — opened as `SIGNAL-DECLARATION-ROW-DROP.2j` | 1 |
  | `caption_admission` | a deontic WORD inside a title or under a reporting verb (all 5 FP) | 5 |
  | `caption_admission` | a deontic CLAUSE outside route `r1`'s phrase list (all 4 FN) | 4 |
  | — | **genuine ambiguity** | **0** |

  Every one is a rule defect with a deterministic repair, a different decision, an upstream ingest
  defect, or a contract limit this repository has already adjudicated. Stated against its own
  weakness: 31 errors in four documents, classified by one annotator. It bounds what arm C could win
  *here*; it does not prove no corpus row is ambiguous.

  **The loss is concentrated, not diffuse:** AXI and AHB score **0 errors across 559 rows**, APB loses
  one, and **ADIv6 loses 21 of its 24 rows while all 24 are real signals** — 87.5% in a wire-bearing
  document, invisible to every gold score and currency gate.

  **The pre-registered bar, fixed before any model — local or remote — was run against this set.**
  `.6` is held to these numbers and no others. Arm C must satisfy all four **simultaneously, in one
  run at `jev-1.13.0`, re-derivable offline from its cached record**:
  **C1** beat arm A **and** arm B **each** by ≥ `0.05` absolute macro-F1;
  **C2** cut total errors to ≤ 60% of the better of them, by at least `ceil(0.4 × arm A errors)` rows
  with a floor of 4 — today `declaration_row` ≤ 13 errors and ≥ 12 rows corrected,
  `caption_admission` ≤ 5 and ≥ 4;
  **C3** add **no** false positive over the better of them, because a phantom declaration propagates
  silently into the KG while a miss stays countable;
  **C4** hold the margin with the 8 recorded contested rows **removed** as well as included.
  A marginal or ambiguous result is a rejection, and D1–D4 stay fatal independently.

  Verification: see the `.1` acceptance checklist below.
  Commit: `BOUNDED-DECISION-PROVIDER.1 — freeze the per-row set, score arm A, and fix the bar before any model runs`

- ID: `BOUNDED-DECISION-PROVIDER.1a`
  Status: `active` (`2026-09-19`; SPLIT into `.1a.1` and `.1a.2` — `.1a.1` closed, `.1a.2` open and
  recorded as not decision-relevant)
  Children: `.1a.1` (B1, the repaired rule), `.1a.2` (B2, the local model tier)
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

  **SPLIT `2026-09-19`, after `.1a.1` measured B1.** The two sub-arms stopped being one slice the
  moment B1 landed: B1 needs nothing but the frozen set and it **answered the tree's question**,
  while B2 needs `ollama serve`, a repository-volume venv and the MIT adapter, and its outcome
  **cannot change the answer** (arm B is the maximum over its sub-arms, so B2 can only raise it, and
  arm C is already excluded at B1's scores). Splitting lets the decision-relevant half close and
  records the other half honestly as optional rather than quietly skipping it.
  Verification: see `.1a.1`.
  Commit: see `.1a.1`.

- ID: `BOUNDED-DECISION-PROVIDER.1a.1`
  Status: `done` (`2026-09-19`)
  Goal: **B1 — the repaired rule, scored per row on `.1`'s frozen set, with its selection
  adjudicated corpus-wide before anything is claimed for it.**
  Producer: `python3 scripts/score_bounded_decision_arms.py`. Full record:
  `docs/research/bounded-decision-arm-b.md`; fact card `[[local-repair-closes-the-caption-decision]]`.
  **Deliberately a SEPARATE producer from `.1`'s**, because the baseline is pinned by SHA-256 in
  `doctrine/claim_verification/claims.jsonl` and its RED case pins a line range inside it; the arm
  scorer imports the frozen-set reader from it instead, so the two cannot drift into two opinions
  about the population, and its first RED case asserts arm A reproduced here equals `.1`'s pins.

  | decision | arm A | arm B1 | errors A → B1 | delta |
  | --- | ---: | ---: | :---: | ---: |
  | `caption_admission` | `0.65014` | **`0.95413`** | 9 → **1** | **+0.30399** |
  | `declaration_row` | `0.90715` | `0.91077` | 22 → 21 | +0.00362 |

  **Four rules, three of them grammar.** R1 a title has no finite main clause, so a deontic inside it
  qualifies a noun. R2 a sentence that OPENS with a figure/table label plus a reporting verb reports
  its referent — **anchored to the opening, not to word order**, because *"The bit combinations that
  Table 3-7 does not show, are not permitted"* is a real prohibition whose reporting verb sits in a
  relative clause, and an order-only test loses it. R3 `(is|are) not permitted` and
  `no … (is|are) allowed` are deontic and route `r1` misses both. R4 a whole-cell direction
  abbreviation under a header that says `Direction` is a direction.

  **The selection was adjudicated in full, not sampled.** Over 15,026 statements in the four
  documents, R3 newly admits **12 — and 12 of 12 are genuine prohibitions**. Two wider forms were
  tried and **refused on their own selection**: bare `prohibited` (10, of which 4 are licence
  boilerplate, 2 an allocation *hint* and 4 figure titles — zero obligations) and
  `(is|are) not (legal|valid)` (7 response-code rows and data-validity statements). Both refusals
  re-derive from `--blast-radius`.

  **B1 loses exactly one row arm A held**, `statement_1426`, whose prohibition is written
  `can … , not both`. Arm A admitted it by accident, on the `must` inside the reported clause R2
  exists to refuse. Fitting a rule to it was refused: one row is not a grammar — the bar `.2b`
  applied to the leftward arrow and `.2h.0` to the abbreviations.

  **What this does to the pre-registered bar, which is the reason the leaf matters.** C1 requires arm
  C to beat arm A **and arm B** each by ≥ 0.05 absolute macro-F1. On `caption_admission` that is
  **≥ 1.00413 — arithmetically impossible**. On `declaration_row` it is ≥ 0.96077, which takes
  **13 of the 21** rows B1 still misses — `--bar` derives that from the measured arms, and the first
  draft of this leaf published `9`, which is C2's separate error floor and reaches only `0.94567`.
  Those 21 are 6 whose name is not in the column the decision is about, 8 stating no attribute at
  all, and 7 whose attribute the text layer destroyed, so correcting thirteen means admitting
  identity with no attribute — measured at **24% precise** — which **C3** forbids independently of
  the score. **Under the bar as written, arm C cannot clear either decision**, and
  that was established with zero egress, no key, and no model run.
  **Stated against itself:** this is the bar's arithmetic, not a measurement of Jev, which has not
  been run; it rests on B1 being shippable, which the blast-radius adjudication supports over four
  documents and not the corpus; and if the director judges the `0.05` margin too strict now that a
  local arm is strong, that is a decision to change the bar — which `.1` pre-registered precisely so
  it would have to be made openly rather than by drift.
  Prerequisite: `.1`. **Zero egress**; no network call of any kind.
  Verification: see the `.1a.1` acceptance checklist below.
  Commit: `BOUNDED-DECISION-PROVIDER.1a.1 — the local repair closes the decision the provider was for`

- ID: `BOUNDED-DECISION-PROVIDER.1a.2`
  Status: `pending` — **open, and recorded as NOT decision-relevant**
  Goal: **B2 — the local model tier through the same harness.** `system-one-adapter-python` (MIT) is
  a drop-in `TypeSafeClient` replacement taking an OpenAI-compatible `base_url`, so Ollama answers
  the *same* `Choice`/`Noul` questions with `llm_answer_mode="probabilities"` and only the backend
  differs.
  **Why it is open but not blocking.** Arm B is the **maximum** over its sub-arms, so B2 can only
  raise arm B or leave it where B1 put it — it cannot lower the bar arm C has to clear. Arm C is
  already excluded at B1's scores, so **B2's outcome cannot change `.6`'s answer.** It stays open
  because the tree's fairness rule says arm B must get comparable effort and because a local model
  tier is worth knowing about on its own, and it is recorded as optional so the director can decide
  whether to spend the setup rather than have it silently skipped.
  **Setup, stated so its egress is not assumed away:** `ollama serve` (installed, not running), a
  repository-volume venv, and the MIT adapter. Installing that adapter is egress to a package index
  for a BUILD dependency — not corpus egress to a decision provider — so it does not touch `.2`'s
  bar, but it is named here rather than left to pass silently.
  **Audit the adapter before trusting a weak B2**: it is the vendor's own comparison tool, and MIT
  source makes that auditable.
  Prerequisite: `.1a.1`. **Zero egress to any decision provider.**
  Verification: `pending`
  Commit: `pending`

- ID: `BOUNDED-DECISION-PROVIDER.2`
  Status: `pending` — **MOOT unless `.6` is reopened** (`2026-09-19`). The amendment exists to admit a
  bounded *remote* generator; `.6` admitted none, so `ROADMAP.md:37` stands as written and there is
  nothing to draft. Left open rather than retired because reopening `.6` — by running arm C or by
  revisiting the margin — makes it live again unchanged.
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
  Status: `pending` — **reachable only if the director elects to run arm C** (`2026-09-19`); `.6`
  rejected on measurement without it
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
  Status: `pending` — **reachable only if the director elects to run arm C** (`2026-09-19`). `.6`
  established that no arm-C result can clear `caption_admission`, so this trial can inform but cannot
  decide
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
  Status: `done` (`2026-09-19`) — **REJECTED, on measurement, without running arm C**
  Goal: **adopt, reject, or defer — recorded with the measurement that decided it.** Adoption requires
  arm **C to beat both A and B** by `.1`'s pre-registered margin, with no disqualifier D1–D4 fired. A win
  over A alone is **not** adoption: it means the local fix in `.1a` should ship instead. An adoption must name the decisions it covers and the ones it does not. A rejection
  must say what would change the answer, and must keep `.1`'s baseline and the G1–G11 suite as the durable
  product — so the next provider is measured, not re-argued.
  Acceptance: a decision record; `ROADMAP.md` and the mdBook updated to whichever answer landed; any claim
  published by `.5` registered under `CLAIM_VERIFICATION.md` with its offline re-derivation.
  Prerequisite: `.5`.

  **CLOSED `2026-09-19` as a REJECTION. ADR 0051 is the record.**

  **Why it closed without its stated prerequisite, stated plainly rather than stepped over.** `.6`'s
  prerequisite is `.5` — the arm-C trial. `.5` was never run, and closing over a prerequisite is
  exactly the shortcut this tree exists to refuse, so the reason has to hold on its own: **no arm-C
  result can change the answer on `caption_admission`.** C1 requires arm C to beat the best of A and
  B by `0.05` absolute macro-F1; arm B1 scored `0.95413`, so the requirement is `≥ 1.00413`. A
  *perfect* arm C scores `1.00000`. The trial is not merely unlikely to pass, it is unable to.
  `scripts/score_bounded_decision_arms.py --bar` derives that from the measured arms and a RED case
  pins it.

  **`declaration_row` is refused on evidence, not arithmetic, and the asymmetry is deliberate.** There
  C1 needs **13 of the 21** rows arm B1 still misses, which is possible in principle. It is refused
  because of what those rows contain: 6 whose name is not in the column the decision is about, 8
  stating no attribute anywhere, and 7 whose attribute the text layer destroyed. The only mechanism
  that reaches thirteen of them is admitting identity with no attribute — measured three times in
  this repository and refused, most recently at **24% precision, 14 real against 45 phantom** — and
  **C3** forbids buying recall with false positives independently of the score. That is a strong
  evidential judgement, not a proof, and it is recorded as such.

  **What a rejection must contain, per this leaf's own terms:**
  - **What would change the answer** — three things, and the first two are the director's: running
    arm C anyway; revisiting the `0.05` margin, which was pre-registered precisely so a strong local
    arm would count against adoption; or a *different* decision where the evidence IS in the
    candidate and no deterministic rule separates the cases, which gets a fresh three-arm comparison
    against this same bar.
  - **The durable product kept** — `.1`'s frozen set and pre-registered bar survive as the standard
    the next provider is measured against, and `.3`'s bounded-use contract stays open for the same
    reason: it is worth writing whether or not anything is ever adopted.

  **The acceptance's documentation legs, and the one that is a deliberate no-op.**
  - Decision record: **ADR 0051**, indexed.
  - mdBook: `docs/book/src/architecture-rationale.md` gains *"The boundary was tested against a real
    offer, and it held"* under **Why AI is bounded instead of central** — the public, durable version
    of the transferable idea, that a provider is measured against the best LOCAL arm rather than
    against the defect it was proposed for. Written without a single digit, deliberately, so it adds
    no region to the mdBook quantitative census.
  - `ROADMAP.md`: **unchanged, and that is the correct update.** The only amendment this tree
    contemplated was `.2`'s — relaxing `ROADMAP.md:37` from bounded *local* generators to admit a
    bounded *remote* one. No remote provider is being admitted, so the line stands as written. Per
    `COMMIT.md`, a surface whose truth did not change is documented by leaving it byte-identical, not
    by appending that it was reviewed.
  - `CLAIM_VERIFICATION.md`: `.5` published no claim because `.5` never ran, so there is none to
    register. The measurements this decision rests on are carried by
    `bounded-decision-baseline-frozen`, already registered and covering both arms.
  Verification: see the `.6` acceptance checklist below.
  Commit: `BOUNDED-DECISION-PROVIDER.6 — reject the provider on measurement, and keep what the evaluation built`

## Acceptance Checklist (enforced) — `BOUNDED-DECISION-PROVIDER.1`

- [x] **REPRODUCE / MEASURE** — `python3 scripts/build_bounded_decision_baseline.py` over the frozen
  set: `declaration_row` **644 rows / 101 tables**, tp=567 fp=0 fn=22 tn=55, macro-F1 `0.90715`;
  `caption_admission` **613 rows**, tp=2 fp=5 fn=4 tn=602, macro-F1 `0.65014`. All 31 disagreements
  are printed individually, not summarised. `--verify-currency` re-derives the whole population from
  the persisted corpus and reproduces the frozen file **byte-for-byte**.
- [x] **ROOT CAUSE (WHY + WHERE)** — for a MEASURE leaf this is the population boundary and why it is
  defensible. `declaration_row` is every body row in
  `extraction_manifest.declaration_row_accounting`, which only the four documents rebuilt since
  `SIGNAL-DECLARATION-ROW-DROP.1` carry; `caption_admission` is every extracted statement matching
  `statement_is_a_caption`'s shape, mirrored from `crates/specforge/src/ir/semantic.rs:8862`. The
  reader's name column is reproduced with `column_selection` from
  `scripts/measure_parametric_width_cell_shapes.py`, and it **reproduces the recorded drop sequence
  of all 101 tables exactly** — the mirror is validated by the artifact rather than asserted.
- [x] **ADDRESSED (verified)** — all five acceptance items delivered: a digest-pinned per-row set
  (`d3c5898f…`), one command that re-derives each score, every disagreement enumerated, the bar
  pre-registered as C1–C4 **before any model ran**, and the disagreements classified at
  **0 genuine ambiguity of 31**. `--check` refuses any drift in the digest, either population size,
  or either confusion matrix, so production moving is an event that must be re-derived rather than
  absorbed.
- [x] **NO REGRESSION** — **no Rust, fixture, artifact, gold, seal or `.isf` is touched**; the slice
  adds one read-only producer, one frozen evidence file, one research record and one fact card, so no
  score can move. `--self-test` **10/10 RED cases**, and three of them were observed failing on
  known-bad inputs: scoring `withheld` as a declaration trips `withheld-is-not-emitted` and
  `live-result-matches-pin`; making the scorer ignore the gold trips four cases; truncating the
  mirrored modal list trips `modal-route-reproduces-the-reader`. The producer was restored
  byte-identically after each (sha256 re-checked). Determinism: two consecutive `--emit` runs are
  byte-identical.
- [x] **GENERICITY (ADR 0006)** — the gold is stated as ten reusable **basis ids** in the frozen set's
  own rubric header (`base_name_template`, `metavariable_grid`, `width_parameters`, `encoding_matrix`,
  `legend`, `encoding_rows`, `finite_prohibition`, …), each a property of a table's own shape or of
  what the document states about it. No production rule is added or changed, and the verbatim corpus
  cells live only in the adjudication evidence, which is where document text belongs.
- [x] **LOCKSTEP** — no user-visible behaviour, no command, no emitted artifact and no public contract
  changes, so the book is unchanged by the producer sub-clause; the durable surfaces are the research
  record, the fact card `[[bounded-decision-frozen-baseline]]`, and the claim record
  `bounded-decision-baseline-frozen`. The one finding that belongs to another tree is **owned rather
  than reported**: `SIGNAL-DECLARATION-ROW-DROP.2j` opened for the abbreviated-direction refusal and
  for handing ADIv6 `table_0108`'s column garble to the ingest tree.

## Acceptance Checklist (enforced) — `BOUNDED-DECISION-PROVIDER.1a.1`

- [x] **REPRODUCE / MEASURE** — `python3 scripts/score_bounded_decision_arms.py`:
  `caption_admission` arm A `0.65014` (tp=2 fp=5 fn=4 tn=602) against arm B1 **`0.95413`**
  (tp=5 fp=0 fn=1 tn=607), 9 of 9 arm-A errors corrected; `declaration_row` `0.90715` against
  `0.91077`, 1 row corrected. Every remaining error and every corrected row is printed individually.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/semantic.rs`, `is_invariant_like` route
  `r1`: `contains_any_phrase` over a 12-phrase list, tested before `statement_is_a_caption`. It fires
  on a deontic **word** wherever it sits — inside a title, inside a clause a reporting verb governs —
  and misses a deontic **clause** whose wording the list does not carry. The five false positives and
  four false negatives are exact mirrors of that one shape.
- [x] **ADDRESSED (verified)** — four candidate rules, scored on the frozen set, **not shipped**. The
  selection is adjudicated corpus-wide rather than sampled: over 15,026 statements R3 newly admits
  **12, all 12 genuine prohibitions**, and two wider forms were tried and refused on their own
  selection (`prohibited` → 10 non-obligations; `(is|are) not (legal|valid)` → 7 data-validity rows).
  The one row B1 loses is named, and the rule that would have kept it was refused for a stated
  reason.
- [x] **NO REGRESSION** — **no Rust, fixture, artifact, gold, seal or `.isf` is touched**, and `.1`'s
  producer is not edited, so its pinned claim digest and RED line range are unmoved. The arm scorer's
  first RED case asserts arm A reproduced here equals `.1`'s pinned matrices exactly.
  `--self-test` **11/11 RED cases**, the eleventh asserting the C1 requirement is DERIVED from the
  measured arms — added after the first draft of this leaf published `9` where the derivation says
  `13`. Determinism: two consecutive runs byte-identical.
- [x] **GENERICITY (ADR 0006)** — R1/R2 are sentence shape, R4 is a column-header property, and R3 is
  a two-form deontic grammar whose every corpus admission was read. No document, vendor or protocol
  name appears in any rule; the verbatim corpus text lives only in the adjudication evidence.
- [x] **LOCKSTEP** — no user-visible behaviour changes because nothing is shipped, so the book is
  unchanged by the producer sub-clause. The durable surfaces are
  `docs/research/bounded-decision-arm-b.md` and the fact card
  `[[local-repair-closes-the-caption-decision]]`. The shipping work is routed rather than implied:
  R1–R3 to `INVARIANT-SHAPE-ADMISSION`, R4 to `SIGNAL-DECLARATION-ROW-DROP.2j`.

## Acceptance Checklist (enforced) — `BOUNDED-DECISION-PROVIDER.6`

- [x] **REPRODUCE / MEASURE** — `python3 scripts/score_bounded_decision_arms.py` derives the bar's
  requirement from the measured arms rather than from prose: `caption_admission` best local
  `0.95413`, arm C must reach `1.00413`, reported as **"ABOVE 1.0, so no arm can reach it"**;
  `declaration_row` best local `0.91077`, arm C must reach `0.96077`, **13 of the 21** rows arm B1
  still misses. `--self-test` **11/11**, the eleventh pinning that derivation.
- [x] **ROOT CAUSE (WHY + WHERE)** — for a DECISION leaf this is why the answer is what it is, and it
  is the bar rather than the vendor: C1 asks a provider to beat the best LOCAL arm by `0.05`, and a
  local arm reached a score whose `+0.05` lies outside the range of the metric. The evaluation was
  designed so a strong local arm counts against adoption; it worked exactly as designed.
- [x] **ADDRESSED (verified)** — ADR 0051 written and indexed; the mdBook gains the public rationale
  under **Why AI is bounded instead of central**; `ROADMAP.md` deliberately byte-identical, with the
  reason recorded; `.2`, `.4`, `.5` annotated as conditional rather than retired, so reopening `.6`
  makes them live again unchanged; `.3` kept open because a rejection must keep the contract as the
  standard the next provider is measured against.
- [x] **NO REGRESSION** — **no Rust, fixture, artifact, gold, seal or `.isf` is touched**, and no
  production rule changed, so no score can move. Both producers still agree with their pins
  (`build_bounded_decision_baseline.py --check`, `--self-test` 10/10;
  `score_bounded_decision_arms.py --self-test` 11/11). The mdBook addition carries **no digit**, so
  the quantitative census is unmoved at 539 adjudicated regions.
- [x] **GENERICITY (ADR 0006)** — the decision names a vendor because the evaluation was of a vendor;
  no rule, vocabulary or production path is touched, and ADR 0006 is what made the shape admissible
  in principle in the first place. Nothing document-, protocol- or corpus-specific enters production.
- [x] **LOCKSTEP** — the answer is published on every surface whose truth changed: ADR 0051 and its
  index, the tree's status/frontier/decisions/changelog, the book chapter, `MEMORY.md`, and the fact
  cards. `ROADMAP.md` is the stated no-op. **Producer sub-clause: no production rule was deleted or
  replaced**, so no book text describes behaviour that has gone.

## Current Frontier

**The tree's question is answered: `.6` REJECTED the provider on measurement (ADR 0051).** What
remains is one leaf worth doing regardless, and four that only become live if the director reopens
the decision.

| Order | Leaf | Status | Why |
| --- | --- | --- | --- |
| — | `BOUNDED-DECISION-PROVIDER.1` | `done` | the frozen 1,257-row set, arm A, and the bar fixed as C1–C4 before any model ran |
| — | `BOUNDED-DECISION-PROVIDER.1a.1` | `done` | arm B1 took `caption_admission` to `0.95413`, putting C1's requirement for arm C above 1.0 |
| — | `BOUNDED-DECISION-PROVIDER.6` | `done` | **REJECTED**, keyless and zero-egress, with what would change the answer recorded |
| 1 | `BOUNDED-DECISION-PROVIDER.3` | `pending` | **still worth writing.** A rejection must keep the bounded-use contract as the standard the NEXT provider is measured against, so the next proposal is measured rather than re-argued. Design only, no network |
| 2 | `BOUNDED-DECISION-PROVIDER.1a.2` | `pending` | B2, the local model tier — fairness and completeness only; arm B is the maximum over its sub-arms, so B2 can only raise it |
| — | `BOUNDED-DECISION-PROVIDER.2` | `pending` | **moot unless `.6` reopens**: no remote provider is admitted, so `ROADMAP.md:37` stands and there is nothing to amend |
| — | `BOUNDED-DECISION-PROVIDER.4` | `pending` | reachable only if the director elects to run arm C |
| — | `BOUNDED-DECISION-PROVIDER.5` | `pending` | reachable only if the director elects to run arm C; it can inform but cannot decide |

**The shipping work this tree produced is owned elsewhere and is not complete**: arm B1's caption
rules go to `INVARIANT-SHAPE-ADMISSION` and its direction rule to `SIGNAL-DECLARATION-ROW-DROP.2j`,
each needing the corpus-wide adjudication over all 78 persisted documents that shipping requires.

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
- `2026-09-19`: **Score the PRODUCT's decision, not the reader's intermediate one** (`.1`). A row counts
  as declared only when a declaration from it survives to `table_signal_declaration_provenance`. AXI
  `table_0011` records 6 emitted declarations and zero provenance records because
  `withhold_base_name_template_declarations` removes them, so scoring the accounting would have
  published six false positives the product does not make and reported precision `0.9895` instead of
  `1.0000`. Anything measured against an intermediate counter measures the wrong system.
- `2026-09-19`: **The gold labels the ROW, not the token the reader lifted out of it** (`.1`). A row that
  declares a real wire whose name the reader could not find is a miss, and burying that in the gold
  would hide the defect the set exists to measure. It costs a divergence from
  `[[a-dropped-declaration-row-is-usually-not-a-signal]]`'s table-level verdict on ADIv6 `table_0108`,
  which asked the different question "should this NAME CELL be admitted" and answered no; the
  classification names those 6 rows as a different decision rather than relabelling them.
- `2026-09-19`: **What `.1` found weakens the case for arm C before arm C runs, and that is recorded
  now rather than after results exist.** The `declaration_row` decision is a table property in 101 of
  101 tables, arm A's precision on it is already `1.0000`, and **0 of 31** disagreements are genuine
  ambiguity. The headroom that does exist — `caption_admission` at macro-F1 `0.65014` — is ordinary
  grammar with a deterministic fix, which is arm B's case rather than arm C's.
- `2026-09-19`: **`.6` closed over its own prerequisite, and the reason is stated rather than stepped
  over.** `.6`'s prerequisite is `.5`, the arm-C trial, which was never run. Closing over a
  prerequisite is the shortcut this tree exists to refuse, so the justification has to stand alone:
  no arm-C result can clear `caption_admission`, because C1's requirement is `≥ 1.00413` and a
  perfect arm scores `1.00000`. On `declaration_row` the refusal is evidential rather than
  arithmetic — 13 of 21 rows is possible in principle and refused on what those rows contain — and
  the asymmetry is recorded instead of flattened.
- `2026-09-19`: **A rejection is documented by what does NOT change as much as by what does.**
  `ROADMAP.md:37` mandates bounded *local* generators; the only amendment this tree contemplated was
  to relax it, and no remote provider is admitted, so the correct roadmap update is none at all.
  `.2`, `.4` and `.5` are annotated conditional rather than retired, so reopening the decision makes
  them live again unchanged.
- `2026-09-19`: **Arm B is scored, not shipped, and that separation is deliberate** (`.1a.1`). Every B1
  rule is a candidate measured against the frozen set; shipping needs a corpus-wide adjudication
  across all 78 documents and belongs to the owning extraction trees, not here. It also keeps `.1`'s
  pinned baseline valid — a shipped repair would move arm A, and the comparison would lose its
  fixed point.
- `2026-09-19`: **The `0.05` margin now excludes arm C by arithmetic, and that is the bar working, not
  a loophole** (`.1a.1`). Arm B1 reached `0.95413` on `caption_admission`, so C1's requirement for arm
  C is `≥ 1.00413`. The margin was fixed before any model ran precisely so a strong local arm would
  count against adoption instead of being talked around. If it is now judged too strict, that is a
  decision to change the bar, taken openly.
- `2026-09-19`: **`.1a` split, and B2 is recorded as optional rather than skipped** (`.1a.1`). Arm B is
  the maximum over its sub-arms, so B2 can only raise it — it cannot lower what arm C must clear.
  Recording that lets the decision-relevant half close while the other half stays honestly open, and
  it avoids the failure mode where an unfinished sub-arm is quietly treated as if it had been run.
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
- **The keyless work has now answered the question the key was going to be spent on.** `.1` (the frozen
  baseline and the pre-registered bar) and `.1a.1` (arm B1) are **done**, both zero-egress and keyless,
  and together they show arm C cannot clear either decision under the bar as written. `.3` (the contract)
  is still design and still worth writing, because a rejection keeps it as the standard the next provider
  is measured against. **The procurement block is therefore no longer the thing standing between this tree
  and its decision** — `.6` is reachable on measurement, and buying the key is now a choice to run arm C
  despite the bar rather than a prerequisite for deciding.
- `.2`'s roadmap amendment is a drafting task, no longer a question — see that leaf.

## Changelog

- `2026-09-19`: **`.6` decided: REJECTED (ADR 0051), keyless and zero-egress, without running arm C.**
  The bar the director set did the work. C1 requires a provider to beat the best LOCAL arm by `0.05`
  absolute macro-F1; arm B1 reached `0.95413` on `caption_admission`, so arm C would need
  `≥ 1.00413` and a perfect arm scores `1.00000`. On `declaration_row` it would need 13 of 21 rows
  whose evidence is not in the row, reachable only by admitting identity with no attribute — 24%
  precise, and refused by C3 independently of the score. ADR 0051 records the transferable idea: a
  remote provider is measured against the best LOCAL arm, never against the defect it was proposed
  for. The book gains the public version under **Why AI is bounded instead of central**;
  `ROADMAP.md` is deliberately unchanged; `.3` stays open because a rejection keeps the contract as
  the standard the next provider is measured against.
- `2026-09-19`: **`.1a.1` closed, and it answered the tree.** Arm B1 — four deterministic rules, three
  of them sentence grammar — took `caption_admission` from macro-F1 `0.65014` to **`0.95413`**,
  correcting 9 of arm A's 9 errors and introducing no false positive; `declaration_row` moved
  `0.90715` to `0.91077`, one row, exactly as `.1`'s classification predicted. Its selection was
  adjudicated in full over 15,026 statements — R3 newly admits **12, all 12 genuine prohibitions** —
  and two wider forms were tried and refused on their own selection. The consequence is the bar's:
  C1 now requires arm C to reach **≥ 1.00413** on `caption_admission`, and 13 of 21 rows on
  `declaration_row` whose evidence is not in the row and whose recovery C3 forbids. **Arm C cannot
  clear either decision under the bar as written**, established with zero egress and no key. `.1a`
  split; B2 is open and recorded as not decision-relevant.
- `2026-09-19`: **`.1` closed.** The frozen set is 1,257 labelled rows — 644 declaration rows across 101
  tables and 613 captions, from the four documents carrying the reader's own row accounting — digest-pinned
  at `d3c5898f…` and scored by `scripts/build_bounded_decision_baseline.py`. Arm A: `declaration_row`
  precision `1.0000` / recall `0.9627` / macro-F1 `0.90715` with 22 errors; `caption_admission` precision
  `0.2857` / recall `0.3333` / macro-F1 `0.65014` with 9. The bar is fixed as **C1–C4** before any model
  ran. Three findings changed the picture: the product withholds AXI `table_0011`'s six base-name
  declarations so arm A has **no** false positive to win back; the decision is a **table property in 101
  of 101 tables**, so a per-row `Noul` is answering a question settled one level up; and **0 of 31**
  disagreements are genuine ambiguity. `SIGNAL-DECLARATION-ROW-DROP.2j` opened for the one deterministic
  notation the classification surfaced and for ADIv6 `table_0108`'s column garble.
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
