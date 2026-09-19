---
id: local-repair-closes-the-caption-decision
title: A deterministic local repair takes the caption decision from macro-F1 0.65014 to 0.95413, which puts the pre-registered bar's remote arm above 1.0 and closes it by arithmetic
answers:
  - "does a local rule beat the caption admission defect"
  - "what does arm B score on the bounded decision frozen set"
  - "is a remote decision provider still worth running after arm B"
  - "why is arm C excluded on caption_admission"
  - "why is arm C excluded on declaration_row"
  - "what are the B1 caption admission rules"
  - "why is the cross-reference refusal anchored to the sentence opening"
  - "how many statements does are-not-permitted newly admit"
  - "why was bare prohibited refused as a deontic form"
  - "why was is-not-valid refused as a deontic form"
  - "which caption does arm B1 lose that arm A admitted"
  - "can arm B2 change the bounded decision outcome"
  - "should we buy the TYPESAFE_API_KEY"
  - "what does R4 recover on the declaration row decision"
  - "how many rows must arm C correct to clear C1"
date: 2026-09-19
status: current
tags: [bounded-decision-provider, invariant-shape-admission, semantic-ir, arm-b, pre-registration, local-repair]
evidence: scripts/score_bounded_decision_arms.py; docs/research/bounded-decision-arm-b.md; docs/research/bounded-decision-adjudication.jsonl; docs/tasks/BOUNDED-DECISION-PROVIDER.md (.1a.1); crates/specforge/src/ir/semantic.rs (is_invariant_like, statement_is_a_caption)
reverify: "python3 scripts/score_bounded_decision_arms.py — expect caption_admission A 0.65014 against B1 0.95413 and declaration_row A 0.90715 against B1 0.91077; `--self-test` 11/11; `--blast-radius` 10 + 2 newly admitted rows and 10 + 7 refused, every row printed; the C1 requirement (13 of 21 declaration rows, unreachable on captions) is derived by the same run, never by hand."
---

`BOUNDED-DECISION-PROVIDER.1a.1` built arm B — the best honest LOCAL alternative — and scored it on
the frozen set `.1` pinned. Nothing is shipped; every rule is a candidate scored against the frozen
rows, so `.1`'s pinned baseline stays valid.

| decision | arm A | arm B1 | delta |
| --- | ---: | ---: | ---: |
| `caption_admission` | `0.65014` | **`0.95413`** | **+0.30399** |
| `declaration_row` | `0.90715` | `0.91077` | +0.00362 |

## The rules, and the one that is easy to get backwards

**R1** a title has no finite main clause, so a deontic word inside it qualifies a noun. **R2** a
sentence that OPENS with a figure/table label plus a reporting verb reports its referent. **R3**
`(is|are) not permitted` and `no … (is|are) allowed` are deontic and route `r1` misses both. **R4**
(declaration rows) a whole-cell direction abbreviation under a header that says `Direction` is a
direction — a population `[[SIGNAL-DECLARATION-ROW-DROP]].2h.0`'s refusal never covered, because it
censused the columns whose header names none.

**R2 must be anchored to the sentence OPENING, not to word order.** The obvious rule — refuse when a
reporting verb precedes the deontic — refuses *"The bit combinations that Table 3-7 does not show,
are not permitted"*, where the reporting verb sits in a relative clause and the deontic heads the
main one. That is a real prohibition, and an order-only test loses it.

## The blast radius is 12 of 12 correct, and two wider forms were refused

Over 15,026 statements in the four documents, counting only what route `r1` does not already admit:
`(is|are) not permitted` newly admits **10, all ten genuine prohibitions**, and `no … (is|are)
allowed` newly admits **2**, both the frozen set's own false negatives. Bare `prohibited` was tried
and **refused** — its 10 are licence boilerplate, an allocation *hint*, and figure titles, none an
obligation — as was `(is|are) not (legal|valid)`, whose 7 are response-code rows and data-validity
statements. Both refusals re-derive from `--blast-radius`
([[a-cheap-structural-rule-overfires-until-you-read-its-selection]]).

B1 loses exactly one row arm A held: `statement_1426`, whose prohibition is written `can … , not
both`. Arm A admitted it by accident, on the `must` inside the reported clause R2 exists to refuse.
Fitting a rule to it was refused — one row is not a grammar.

## What it does to the bar, which is the point

`.1`'s **C1** requires arm C to beat arm A **and arm B** each by ≥ 0.05 absolute macro-F1:

* `caption_admission` — arm C would need **≥ 1.00413**. **Arithmetically impossible.**
* `declaration_row` — arm C would need ≥ 0.96077, which takes **13 of the 21** rows B1 still misses
  (9 corrections reach only `0.94567`; the 9 is C2's separate error floor, not C1's margin). Those are 6
  whose name is not in the column the decision is about, 8 stating no attribute at all, and 7 whose
  attribute the text layer destroyed. Correcting thirteen means admitting identity with no attribute —
  measured at **24% precise** ([[a-dropped-declaration-row-is-usually-not-a-signal]]) — which **C3**
  forbids independently of the score.

**So under the bar as written, arm C cannot clear either decision, established with zero egress and
no API key.** Arm B is the maximum over its sub-arms, so B2 can only raise it: **B2's outcome cannot
change the answer**, and it stays open as not-decision-relevant. The recommendation is to ship R1–R4
under their owning trees, close `BOUNDED-DECISION-PROVIDER` at `.6` with no provider, and not buy the
key for this evaluation. See [[bounded-decision-frozen-baseline]] for the set and the bar itself.

This is the bar's arithmetic, not a measurement of the provider: arm C has not been run, and nothing
here claims what it would score.
