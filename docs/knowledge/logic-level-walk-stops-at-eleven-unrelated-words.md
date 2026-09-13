---
id: logic-level-walk-stops-at-eleven-unrelated-words
title: The logic-level walk's stop is correct — the 14 corpus cases where crossing it would reach a declared signal are 11 unrelated words, and any list wide enough to admit the adverbs also admits a negation
answers:
  - "should the logic-level walk skip predicate adjectives"
  - "why does QDENY output absent or tied low not bind a constraint"
  - "how many logic-level bindings are lost to the backward walk stopping early"
  - "what words stop walk_for_level_subjects"
  - "can DESCRIPTORS be widened to catch more logic-level bindings"
  - "is there a class of word between a signal and its level"
  - "why is TLAST cannot be tied LOW not extracted"
  - "what would crossing cannot cost"
  - "does AERR is always driven LOW produce a constraint"
  - "why did EXTRACTION-QUALITY-GAUGE.3k.12 ship no rule"
  - "how do I measure what a skip-list widening would newly admit"
  - "why does a blocked logic-level walk sometimes mean a missing declaration"
date: 2026-09-14
status: current
tags: [evidence-ir, constraints, logic-level, adr-0006, census-method, extraction-quality-gauge]
evidence: scripts/measure_logic_level_walk_blockers.py; crates/specforge/src/ir/evidence.rs (logic_level_bindings; walk_for_level_subjects; token_logic_level; DESCRIPTORS; MAX_GAP); docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.3k.11, .3k.12, .3k.13)
reverify: "python3 scripts/measure_logic_level_walk_blockers.py — expect 261,858 statements scanned, 14 distinct cases, 11 distinct blocking words: can(2) always(2) again(2) this remains cannot absent pdeny remain write therefore. A new blocking word means a document entered the corpus and the adjudication must be redone before any skip list is widened."
---

`logic_level_bindings` finds a level token (`HIGH`, `LOW`, `1'b0`, …) within six words after a binding
verb (`driven`, `tied`, `set`, `pulled`, …) and walks **backward** to the identifier it belongs to,
skipping the scaffolding English puts between them — `the`, `its`, `input`, `signal`, a subscript, the
verb itself. Anything else stops the walk and the binding is lost.

The tempting repair is to widen that skip list. Measured over all 261,858 persisted statements and
constraint source texts, the whole population of "a word that stops the walk while a **declared** signal
sits within three further words" is **14 distinct cases across 11 different words**:

| word | n | what it is | crossing it |
| --- | ---: | --- | --- |
| `always`, `again`, `therefore` | 5 | adverb | right — *"AERR is always driven LOW"* |
| `remain`, `remains` | 2 | verb | right — *"HRESP remains driven HIGH"* |
| `can` | 2 | modal | **wrong** — *"TVALID can be driven HIGH"* is a permission |
| `cannot` | 1 | negation | **wrong** — *"TLAST cannot be tied LOW"* would publish its opposite |
| `this` | 1 | determiner opening a new sentence | crosses a clause boundary |
| `absent` | 1 | predicate adjective | the one case the question was asked for |
| `PDENY` | 1 | an **undeclared signal** | not this reader's defect |
| `write` | 1 | a garbage table row | — |

**There is no class here, and that is the finding.** Any list long enough to admit the three adverbs also
admits `cannot`, which inverts the fact the sentence states, or `can`, which states a permission as a
requirement — the exact shape `EXTRACTION-QUALITY-GAUGE.3k.13` gated one day earlier. The predicate
adjective the question was opened for occurs **once**, in one document, and a skip list tuned to one
sentence is a mirror of that sentence rather than a rule.

## A blocked walk can be a symptom of a missing declaration

The walk continues through a word only when that word is a **declared** signal, so an incomplete catalog
shows up here as a stop. *"a device must set both PACCEPT and PDENY LOW"* blocks at `PDENY` — and `PDENY`
is genuinely absent from AMBA LPI's catalog, which holds five signals while the document's own
`Table 3-2` names ten. The walk is reporting the catalog, not failing. Tracked as
`SIGNAL-CATALOG-CAPTURE-GAP.6`.

When a level binding looks lost, check the catalog before the walk.

Links: [[constraint-record-producer-strata]], [[one-modal-vocabulary-per-constraint-record]].
