---
id: a-remote-decision-provider-is-measured-against-the-best-local-arm
title: A remote decision provider is measured against the best LOCAL arm, not against the defect it was proposed for — and on that measurement SpecForge rejects one
date: 2026-09-19
status: accepted
scope: bounded-decision-provider, invariant-shape-admission, signal-declaration-row-drop, measurement-integrity, adr-0006
evidence: docs/research/bounded-decision-baseline.md; docs/research/bounded-decision-arm-b.md; docs/research/bounded-decision-adjudication.jsonl; scripts/build_bounded_decision_baseline.py; scripts/score_bounded_decision_arms.py; docs/tasks/BOUNDED-DECISION-PROVIDER.md (.1, .1a.1, .6); docs/knowledge/bounded-decision-frozen-baseline.md; docs/knowledge/local-repair-closes-the-caption-decision.md
reverify: "python3 scripts/build_bounded_decision_baseline.py --check && python3 scripts/score_bounded_decision_arms.py --self-test && python3 scripts/score_bounded_decision_arms.py"
answers:
  - "what is ADR 0051"
  - "why did SpecForge reject the TypeSafe Jev decision provider"
  - "how is a remote model evaluated in this repository"
  - "what is the three-arm comparison"
  - "why must a provider beat the best local alternative and not the current rules"
  - "why is the adoption margin fixed before any model runs"
  - "was Jev ever run against the corpus"
  - "what would reopen the bounded decision provider question"
  - "why was no ROADMAP amendment made for a decision provider"
  - "does rejecting a provider mean models are unwelcome in SpecForge"
  - "what survives a rejected provider evaluation"
---

# ADR 0051: A remote decision provider is measured against the best LOCAL arm, not against the defect it was proposed for — and on that measurement SpecForge rejects one

## Context

`BOUNDED-DECISION-PROVIDER` opened on the director's greenlight to evaluate a **System One** decision
model — TypeSafe's **Jev** — for bounded use inside SpecForge. The shape was genuinely admissible:
the model returns a typed value from a closed set and **cannot emit a string**, so the production
core would never receive a symbol the model authored, which is the property ADR 0006 protects. Two
open defects were the natural candidates, both of them a yes/no over a candidate the deterministic
pass had already located: `SIGNAL-DECLARATION-ROW-DROP` (is this table row a declaration?) and
`INVARIANT-SHAPE-ADMISSION` (is this caption a normative constraint?).

The director set the bar: **integration must prove it brings something SpecForge does not have right
now.** That rules out the comparison everyone reaches for first — measuring a provider against the
rules whose defects motivated the evaluation. Those rules are known-defective; that is *why* they
have trees. Almost anything beats a defective rule, and a win over one proves nothing about
necessity.

## Decision

**1. A remote provider is measured on a three-arm comparison, and the arm that decides is B.**

| arm | what it is |
| --- | --- |
| **A** | the current deterministic rules, scored per row |
| **B** | the best honest LOCAL alternative — a repaired rule, and/or the local model tier the roadmap already mandates |
| **C** | the remote provider under the bounded-use contract |

Adoption requires **C to beat both A and B**. A win over A alone means shipping B instead.

**2. The bar is fixed in writing before any model sees the data, and a marginal win is a rejection.**
A threshold chosen after seeing a result is not a threshold. The cost side is not zero and must be
beaten rather than tied: a doctrine amendment, corpus spans leaving the volume, a permanent
offline-cache obligation so published claims stay re-derivable, and a vendor with no stated
version-availability policy.

**3. The comparison runs on a frozen, digest-pinned, per-row set with gold labels**, carrying each
row's source cells verbatim so a score re-derives from tracked evidence alone, with no dependency on
rebuildable state.

**4. The evaluation is scored on the PRODUCT's decision, not on an intermediate counter.** A
declaration counts only when it survives to the artifact the pipeline publishes. Scoring a producer's
own emission counter credited the reader with declarations a later stage deliberately withholds, and
would have published false positives the product does not make.

**5. On this measurement, the provider is REJECTED.** Arm B — four deterministic rules, three of them
sentence grammar — took the caption decision to a score that puts the bar's requirement for arm C
**above 1.0**, which no arm can reach. On the declaration decision arm C would have to correct
thirteen of the twenty-one rows arm B still misses; those rows' evidence is not in the row at all —
the name is in a different column, or no attribute is stated anywhere, or the text layer destroyed it
— and the only mechanism that reaches them is admitting identity with no attribute, which this
repository has measured three times and refused, most recently at 24% precision.

**No model was ever run, and no corpus text ever left the volume.** The evaluation was decided
keyless and zero-egress, by the bar the director set.

**6. A rejection keeps its product.** The frozen set, the pre-registered bar, and the bounded-use
contract survive as the standard the **next** provider is measured against, so the next proposal is
measured rather than re-argued from scratch.

## Consequences

- **`ROADMAP.md:37` stands unchanged.** The amendment admitting a bounded *remote* generator is not
  made, because none is being admitted. A roadmap that is not changing is documented by leaving it
  byte-identical, not by writing that it was reviewed.
- **The local repairs are the deliverable, and they ship under their owning trees**, not here, and
  each needs the corpus-wide adjudication over every persisted document that shipping requires.
- **Arm B's own sub-arm B2 — the local model tier — stays open and is recorded as not
  decision-relevant.** Arm B is the maximum over its sub-arms, so B2 can only raise it; it cannot
  lower what arm C must clear.
- **This is the bar's arithmetic, not a measurement of the vendor.** Nothing here claims what Jev
  would score, and nothing here says a decision model is unwelcome in SpecForge. It says this one was
  not needed for these two decisions, on a comparison designed so a strong local arm counts against
  adoption instead of being talked around.

## What would reopen it

Exactly three things, and the first two are the director's:

1. **Running arm C anyway**, for its own sake or to test the reasoning. The contract and the frozen
   set make that cheap to do later.
2. **Revisiting the margin.** It was pre-registered precisely so that a strong local arm would count
   against adoption — but it is a number, and changing it is a decision to be taken openly.
3. **A decision the local arms cannot reach.** The rejection is scoped to these two decisions on this
   stratum. A defect where the evidence *is* in the candidate, and no deterministic rule separates
   the cases, is a different question and gets a fresh three-arm comparison against this same bar.

## Links

- `docs/tasks/BOUNDED-DECISION-PROVIDER.md` — the owning tree, the bar as `C1`–`C4`, and the
  disqualifiers `D1`–`D4` that remain fatal independently of any score
- `docs/research/bounded-decision-baseline.md` — the frozen set, arm A, and the bar as registered
- `docs/research/bounded-decision-arm-b.md` — arm B, its corpus-wide selection adjudication, and the
  arithmetic this record rests on
- `docs/decisions/0006-no-hardcoded-chip-spec-vocabulary.md` — the identity-independence doctrine
  that made a non-generating decision model admissible in principle at all
