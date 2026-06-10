---
id: llm-primary-condition-subject-gate
title: Condition-only-subject gate — APB/AHB/AXI constraint task all P=R=F1=1.000 (FPs 3→0)
answers:
  - "why was PSELx / HRESP ERROR / ACTIVATEACK LOW extracted as a constraint (and how was it fixed)"
  - "what is the condition-read-as-obligation error class and its gate"
  - "what is is_condition_only_subject / conditional_clause_spans"
  - "how does the LLM-primary extractor avoid extracting a when/if/unless clause subject as an obligation"
  - "why is a gerund after while/when not a condition (action coordination)"
  - "what is the LLM-primary extractor's measured precision on APB / AHB / AXI gold"
date: 2026-06-10
tags: [extraction-quality, llm-primary, constraints, condition, precision, clause-grammar]
evidence: crates/specforge/src/ir/constraint_extract_llm.rs (is_condition_only_subject, conditional_clause_spans, token_occurrences, CONDITION_CLAUSE_MARKERS); docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.3a)
reverify: cargo test -p specforge constraint_extract_llm 2>&1 | tail -2   # → 16 passed; full re-measure = the .8/.3a redirected-copy protocol in docs/tasks/EXTRACTION-QUALITY-GAUGE.md
---

After `.8` closed the recall gap, ALL three remaining labeled-statement FPs were one class —
**condition-read-as-obligation**: the model proposes a constraint on the subject of a
*when/if/unless* clause ("ASKSTOP must be LOW **when ACTIVATEACK is LOW**" → a fake
`ACTIVATEACK must_be_value LOW`). The `.3a` gate kills the class deterministically: a proposed
subject whose every identifier-boundary occurrence lies inside a subordinate conditional clause
(when/whenever/if/unless/while/until/after/before/provided-that/as-long-as — universal clause
grammar, no signal vocabulary, ADR 0006) is the condition's subject, not an obligation's → drop.
A subject with any main-clause occurrence is kept ("**PWAKEUP** must remain asserted … if PWAKEUP
and PSELx are asserted" keeps PWAKEUP, drops PSELx).

**Two first-cut defects the per-item audit caught before commit** (the reason per-item audits are
non-negotiable):
1. Clause spans terminated only at `,;:` — a span crossed the sentence-final period and swallowed
   the NEXT sentence's main clause. Fix: spans also end at `.!?`.
2. "while **driving** HREADYOUT LOW" is action *coordination* — the obligation IS on HREADYOUT —
   not a condition. Fix: a marker followed by a gerund (first word ending `-ing`) yields no span.
   The defective cut wrongly dropped AHB's two `HREADYOUT` records; the corrected gate restores
   them while still killing all three target FPs. Both behaviors are test-locked.

**Measured** (post-`.8` = baseline, same redirected-copy protocol, eval canonical keys):
APB P 0.857→**1.000**, AHB P 0.857→**1.000**, AXI P 0.800→**1.000**; recall held at 16/16; the
split-conformal tier-agreement threshold now calibrates on all three docs (empirical_error 0.000).
Killed per item: APB `PSELx must_be_high` (if-clause), AHB `HRESP must_be_value ERROR`
(unless-clause), AXI `ACTIVATEACK must_be_value LOW` (when-clause). Remaining error work is `.3b`:
permission-vs-obligation frames ("It is recommended that … HPROT[0] HIGH" — unlabeled today) and
relational-vs-value ("set to the same value as"). Related: [[llm-primary-must-be-value-recall]].
