# Extraction eval — measuring the LLM passes

## Why this exists

SpecForge uses a local LLM/VLM (qwen2.5vl:7b via Ollama, by default) for four optional
enrichment passes — reading diagrams (`enrich`), and recovering typed facts from hard
prose (`nlp-enrich`, `extract-contracts`, `signal-resolve`). Sooner or later you'll ask:
**is a newer/bigger model better here?** (e.g. `qwen3-vl:8b`.) Without a way to *measure*,
that question can only be answered by eyeballing a diff — which is exactly how a "looks
better" swap quietly makes extraction worse.

The two checks SpecForge already had don't answer it: `kg-bench` (151 fixtures) tests the
**deterministic** pipeline and never calls the LLM; the capture–recapture recall gauge is
**unsupervised** (a statistical estimate, no gold answer). So there was no labeled,
supervised score for the LLM passes. `eval-extraction` fills that gap.

## What it gives you

A **labeled precision / recall / F1** score, per task, for the LLM extraction passes —
so a model swap (or a prompt change) is *measured*, not guessed:

```text
specforge eval-extraction <dataset> --provider ollama --model qwen2.5vl:7b
=== Extraction eval (provider: ollama, model: qwen2.5vl:7b) ===
  signal_constraint      P=… R=… F1=…  (tp=… fp=… fn=…; gold=… over … statements)
  actor_signal_relation  P=… R=… F1=…  (…)
```

A model A/B is then just three runs on the same dataset: `--provider skip` (the
deterministic pattern baseline) → `--model qwen2.5vl:7b` → `--model qwen3-vl:8b`. The
deltas tell you what each model actually adds.

## How it works

Three pieces (v1 covers the two text tasks with crisp keys — signal constraints and
actor→signal relations; the contract and diagram tasks are deferred):

- **A labeled dataset** (`crates/specforge/test_data/llm_eval/`). One item per *statement*
  of a real document: the statement text plus the **gold** typed outputs a correct
  extraction must produce (an empty gold list means "the right answer here is *nothing*").
  Gold is **drafted independently from the prose**, never copied from the current
  extractor output — copying would be circular, and the pattern tier has known errors the
  gold deliberately omits.
- **A pure scorer** (`crate::eval`). It compares the model's output to gold by a
  normalized **canonical key** per fact, counting true/false positives and false negatives
  → precision/recall/F1. Scoring is **closed-world over the labeled statements**: only the
  model's output *for a labeled statement* is judged, so an extra extraction on an
  unlabeled statement is never wrongly penalized (which is why each labeled statement is
  labeled *completely*).
- **The runner** (`eval-extraction`). For each document in the dataset it runs the *real*
  extraction command with the chosen model **on a temp copy** of that document's
  EvidenceIR — the IR's write target is redirected to a temp directory, so your corpus
  artifacts are never mutated — then reads the produced records, matches them to the
  labeled statements by provenance, and scores. `--provider skip` makes the commands
  no-op, giving the deterministic pattern baseline for free.

## Honesty notes

- **Labels are agent-drafted, pending human review** (`label_status: agent_drafted`). The
  eval is only as trustworthy as its gold; the dataset is small, versioned, and reviewable.
- The score measures the **whole system's** output for a statement (pattern + LLM), which
  is what matters for "which model gives the best final extraction." Isolate the LLM's
  contribution by diffing against the `--provider skip` baseline.

## How it was verified

The scorer and dataset format are unit-tested (key matching incl. the value-carrying
constraint kind, negation/direction discrimination, closed-world precision/recall/F1,
negative-item false positives, loader round-trips, and the committed seed validates with
≥8 items per task). The runner's orchestration is tested with an injected extractor (no
fixture IR needed), and was confirmed **end-to-end live in skip mode** on the real AMBA
APB seed — establishing the deterministic baseline: the pattern tier recovers every gold
*constraint* (recall 1.0) but over-produces on those statements, and recovers only a third
of the gold *relations* — the prose-only relations are exactly the headroom the LLM
`signal-resolve` pass is meant to fill, which the model A/B now quantifies.

*Authoritative tracking:* `docs/tasks/LLM-EXTRACTION-EVAL.md`.

## Temporal rules — the third measured surface

Signal constraints and actor→signal relations were the first two things this eval could
score. There is a third: **temporal rules** — the `if-this-then-that` timing facts
SpecForge mines from the spec (e.g. *"PNSE must be valid when PSEL is asserted"* becomes a
typed rule: on the rising clock edge, given PSEL asserted, the Requester drives PNSE valid).
These come from the **deterministic temporal parser** as the EvidenceIR is lowered to the
SemanticIR — no LLM involved — so measuring them tells you how faithfully SpecForge turns
timing prose into typed rules.

It works the same way as the rest of the eval, with one twist worth understanding: a
temporal rule's *identity* is its **logical content**, not how it was written down. So the
canonical key is the clock edge plus the rule's antecedent and consequent predicates
(**sorted**, so order doesn't matter) plus the cycle window — and it deliberately ignores
the rule id, the source sentence, and the confidence. Two rules that say the same thing
score as the same fact even if the extractor emitted their parts in a different order.

Run it exactly like the others (the producer is deterministic, so the provider is ignored):

```text
specforge eval-extraction crates/specforge/test_data/llm_eval/seed_apb_temporal.json
=== Extraction eval (provider: skip, model: ) ===
  temporal_rule          P=0.400 R=0.667 F1=0.500  (tp=2 fp=3 fn=1; gold=3 over 6 statements)
```

That score is not a failure — it is the eval **doing its job** on the real AMBA APB spec.
The six labelled statements were chosen to exercise the parser, and the numbers pinpoint two
genuine, actionable issues:

- **Antecedent under-capture (the false negative).** The spec says *"PBUSER must be valid
  when PSEL, PENABLE, **and** PREADY are asserted."* The faithful rule keeps all three
  preconditions; the parser kept only PREADY. A rule that fires on a weaker condition than
  the spec demands is a real semantic gap — now visible as a recall miss instead of hiding
  in 153 rules.
- **Degenerate header rules (two of the false positives).** Sentences like *"The following
  signals must be valid when PSEL is asserted:"* are list *introducers* — the constrained
  signals are in the list that follows, not the sentence itself. The parser bound the
  trigger signal as its own subject, producing a tautological "PSEL valid when PSEL
  asserted" rule. The gold marks these statements as *negatives*, so each degenerate rule
  is counted as a false positive.

A front-matter licence notice is included as a true negative — the parser correctly produces
nothing for it, confirming the eval credits honest abstention. The gold is **judged
independently from the prose** (the under-capture case is deliberately written to *differ*
from what the extractor emits, so it cannot be a self-fulfilling label), and each label
carries a note explaining the call. As with the rest of the eval, the labels are
agent-drafted pending human review.

*Authoritative tracking:* `docs/tasks/TEMPORAL-RULE-EVAL.md` (the `.3` node records the
producer-representation calibration against the 153 real APB temporal rules).
