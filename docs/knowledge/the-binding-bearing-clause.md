---
id: the-binding-bearing-clause
title: The dynamic constraint path's span is the clause its BINDER bound in, not the clause carrying a modal — and it is located after the binding is found, never searched for per clause
answers:
  - "why is constraint_bearing_sentence the wrong narrowing for the dynamic constraint path"
  - "what is binding_bearing_clause and why does it find before it locates"
  - "why does a dynamic constraint record need no modal"
  - "why was PREQ must_be_high published as negated"
  - "why did LRPROT must be 0 carry the condition When LRRESP is FaultAbort"
  - "what happens if the dynamic binder is run per clause"
  - "why is the dynamic path's subject search still statement-scoped"
  - "why does narrowing the dynamic subject lose NVMe register-row constraints"
  - "does replay-constraints show what moved when a record is not reproduced"
  - "which fields does ConstraintReplayVerdict carry"
  - "how is a logic level paired with a signal in the dynamic path"
date: 2026-09-13
status: current
evidence: crates/specforge/src/ir/evidence.rs (binding_bearing_clause, extract_dynamic_signal_constraints, logic_level_binding_kind_from_text, ConstraintReplayVerdict, mod extraction_quality_gauge_3k_4); docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.3i, .3k, .3k.4, .3k.11); docs/book/src/pipeline/obligation-reading.md
reverify: "cargo test -p specforge-core --lib extraction_quality_gauge_3k_4 — five controls; and `specforge replay-constraints --evidence-root generated/evidence_ir` for the corpus figure"
tags: [evidence-ir, constraint-extraction, genericity, extraction-quality-gauge, claim-verification]
---

Three producers mint `SignalConstraintRecord`s and they are minted by different things, so "the span
that produced this record" is a different construct for each ([[constraint-record-producer-strata]]).
The **dynamic** path is minted by a VALUE BINDING, and a value binding need not be modal at all:

```text
QDENY is tied LOW when denial is not implemented.
AERR, DERR are driven LOW when parity check is suspended during power-down.
```

`EXTRACTION-QUALITY-GAUGE.3i` nevertheless gave this path `constraint_bearing_sentence` for its
negation — a helper that locates an obligation MODAL. Where the statement contains one somewhere
else, the record's negation then comes from a clause the record has nothing to do with, while its
condition still came from the whole statement:

| the document writes | the published record | what went wrong |
| --- | --- | --- |
| `… sets PACCEPT HIGH. Once the controller samples PACCEPT HIGH, the device cannot assume …` | `PACCEPT must_be_high`, **negated** | the `cannot` is two sentences away and about something else |
| `If LATRANS is SPEC, LRPROT must be 0. When LRRESP is FaultAbort, this signal is not valid.` | `LRPROT must_be_value 0` **when LRRESP is FaultAbort** | the condition contradicts the record it is attached to |

## Find first, then locate — and that order is measured, not stylistic

`binding_bearing_clause` finds the binding **exactly as the statement-wide reader found it** — same
kind, same value, bit for bit — and then locates the first clause that reproduces it. Two wider
shapes were built and measured before this one, and both were rejected:

* **Narrowing the SUBJECT to that clause costs 30 reproduced records corpus-wide**, ten in NVMe
  alone. A register row names its subject in the cell mnemonic and binds in the descriptive body —
  `| 17:16 | Record Format (RECFMT): … The format of the record specified in this definition shall be
  0h. |` — so the row's other parts legitimately supply the subject its obligation constrains. This is
  the same asymmetry `is_post_passive_binding_only_subject` gate (2) already encodes for table rows.
* **Searching the binders per clause ADDS six records in AMBA LPI alone**, and three of the six are
  `PREQ`/`PACCEPT must_be_high` off state-table rows that set those signals LOW. The cause is a
  different defect: `logic_level_binding_kind_from_text` pairs a level with the BIND VERB rather than
  with a signal, so `Controller must set PREQ LOWand PREQCHK HIGH` attaches `HIGH` to `PREQ`
  (`EXTRACTION-QUALITY-GAUGE.3k.11`). **A span leaf must not ship recall through a pairing that is
  still wrong** — the ordering `.3k` imposed on `.3k.2` before `.3k.3`, applied again.

So the leaf changes WHICH SPAN the condition and negation are read from and nothing else. Measured
corpus-wide: `replayed_total` unchanged in every one of the 77 loadable documents (304 → 304), zero
records added, zero removed, **19 corrected** — 8 conditions taken from a foreign clause, 5 that ran
past their own clause into the next sentence, 5 negations from a clause two sentences away, and one
run-on condition spanning a duplicated cell.

## A verdict you cannot adjudicate is not a verdict

`replay-constraints` reported *"the kind, condition or negation moved"* and printed none of the
three, so acting on it meant re-deriving the thing the instrument exists to spare you.
`ConstraintReplayVerdict` now carries `condition_text`, `negated` and a bounded `source_text`
excerpt, and all 19 corrections above were adjudicated from the report alone. An unpersisted replay
record has no id the artifact knows, which is why its provenance excerpt is the only way to find the
sentence it came from.

Related: [[one-record-per-obligation-clause]], [[one-modal-vocabulary-per-constraint-record]],
[[persisted-census-measures-published-not-current]].
