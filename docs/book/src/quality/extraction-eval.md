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
  temporal_rule          P=0.600 R=1.000 F1=0.750  (tp=3 fp=2 fn=0; gold=3 over 6 statements)
```

That number tells a story — and it is the eval **doing its job** on the real AMBA APB spec.
The six labelled statements were chosen to exercise the parser, and the very first run (before
any fix) scored `P=0.400 R=0.667` because it pinpointed two genuine, actionable issues. One of
them is already fixed — which is exactly why a measurement surface is worth building:

- **Antecedent under-capture — caught, then fixed.** The spec says *"PBUSER must be valid
  when PSEL, PENABLE, **and** PREADY are asserted."* The parser originally kept only PREADY —
  a rule that fires on a weaker precondition than the spec demands. The eval surfaced it as a
  recall miss (instead of it hiding among 153 rules), and `TEMPORAL-ANTECEDENT-RECALL` then
  taught the condition parser to distribute the shared "are asserted" across the coordinated
  signal list, so all three preconditions are now captured. **Recall moved from 0.667 to
  1.000** — visibly, in this same score.
- **Degenerate header rules (the remaining false positives).** Sentences like *"The following
  signals must be valid when PSEL is asserted:"* are list *introducers* — the constrained
  signals are in the list that follows, not the sentence itself. The parser bound the trigger
  signal as its own subject, producing a tautological "PSEL valid when PSEL asserted" rule.
  The gold marks these statements as *negatives*, so each degenerate rule is a false positive.
  These trace to the upstream constraint tier — the same class `CONSTRAINT-SUBJECT-PRECISION`
  fixed — on a spec snapshot that predates that fix, so a clean re-ingest is expected to clear
  them.

A front-matter licence notice is included as a true negative — the parser correctly produces
nothing for it, confirming the eval credits honest abstention. The gold is **judged
independently from the prose** (the under-capture case is deliberately written to *differ*
from what the extractor emits, so it cannot be a self-fulfilling label), and each label
carries a note explaining the call. As with the rest of the eval, the labels are
agent-drafted pending human review.

*Authoritative tracking:* `docs/tasks/TEMPORAL-RULE-EVAL.md` (the `.3` node records the
producer-representation calibration against the 153 real APB temporal rules).

## Register fields — measuring the breadth, and surfacing the gaps honestly

As SpecForge learned to digest more kinds of chip-spec PDFs, it began recovering **register
fields** — the named bit-fields inside a register, like `dmcontrol.haltreq` at bit 31. That is
new ground far beyond the AMBA buses, and the honest question is: *how much of it is actually
right?* So register fields became the eval's fourth measured surface.

A register field's *identity* here is the full tuple — **owning register, field name, and bit
extent** (offset + width). A `[31:31]` range and an `offset 31, width 1` form are normalised to
the same identity, so a field scores as correct only when its register, name, **and** bit
position all match. (Access type — `R`, `WARL`, `W1`, … — is deliberately left out of identity,
because every vendor writes it differently and it is metadata, not the field's shape.)

That strict bar immediately taught us something important, and rather than hide it behind a
single number, the eval **decomposes** the register-field result into the part that works and the
gaps it does not:

```text
specforge eval-extraction crates/specforge/test_data/llm_eval/seed_riscv_debug_registers.json
  -- register-field surface (measure & surface) --
    field-name recall (register-agnostic)   20/34 = 0.588
    bit-structure recall (register-scoped)  0/34 = 0.000
    register-name association gap            0/60 extracted registers have a real (non-synthetic) name
    bit-extent completeness gap              0/179 extracted fields carry a bit position
```

Read that top to bottom. On the real **RISC-V Debug Specification 1.0**, SpecForge recovers the
field *names* well — **20 of 34** gold fields across the `dmstatus` and `dmcontrol` registers,
which is **all 14 of `dmcontrol`** and 6 of `dmstatus`'s 20 (the spec splits `dmstatus`'s field
table across three pages, and the PDF reader dropped the middle page's worth of rows — a real,
located recall miss, not a mystery). But two things it does **not** yet recover: the register's
real *name* (it reads the per-field table but not the heading above it, so every register is
labelled with a placeholder like `register_table_0026`), and each field's *bit position* (those
live in a bit-layout *graphic* above the table, which the reader doesn't parse). Because the
strict identity needs all three, the per-fact precision/recall on this document is **0.000** — and
that is the honest truth, not a failure of the eval. The decomposition is what makes the `0.000`
*useful*: it says "the names are mostly there; the register association and the bit positions are
the two things to fix next," instead of a single demoralising zero that hides where the value is.

The gold is **transcribed independently from the spec's own register definitions** (the bit
positions are read straight from each register's bit-layout graphic in the PDF, field by field),
so it grades the extractor against the source of truth, never against the extractor's own output.
This is the same discipline the AMBA buses are held to — measure first, per fact, no faking —
applied to the much wider world of register-bearing specifications.

### The same surface, a different document, the opposite failure

The fourth line above — **bit-structure recall** — is the register-scoped, mnemonic-agnostic view:
*ignoring* the field's name, did we recover a bit-field at the right offset and width inside the
right register? On RISC-V Debug it is `0/34` (no bit positions captured at all). But run the very
same eval against the **NVMe Base Specification 2.0a**, and the picture flips:

```text
specforge eval-extraction crates/specforge/test_data/llm_eval/seed_nvme_registers.json
  -- register-field surface (measure & surface) --
    field-name recall (register-agnostic)   0/29 = 0.000
    bit-structure recall (register-scoped)  27/29 = 0.931
    register-name association gap            44/44 extracted registers have a real (non-synthetic) name
    bit-extent completeness gap             199/199 extracted fields carry a bit position
```

NVMe fails the **opposite** way to RISC-V Debug. Here SpecForge gets the register *names* (all 44,
because NVMe tables carry a caption like *"Offset 0h: CAP – Controller Capabilities"* that the reader
keeps) and the **bit layout is excellent** — `27/29` of the gold bit-fields land at exactly the
right offset and width across the `CAP`, `CC`, and `CSTS` registers, with the two misses being a
field at the very top of `CAP` and the `CC.EN` bit on its own page fragment (both real, located
drops). What it does *not* get is the field's *mnemonic*: NVMe lays its registers out as
`Bits | Type | Reset | Description`, so the reader records `"60:59"` as the field name and tucks the
real name (`CRMS`) inside the description — hence field-name recall `0/29`. That is exactly why the
surface reports **two** recall views: a field's identity can fail on its name or on its bit extent
independently, and a single number would hide which. Bit-structure recall is *register-scoped* on
purpose — `CAP.CSS` lives at bits 44:37 while `CC.CSS` lives at bits 6:4, so the same name and even
the same extent must not be credited across the wrong register — and it pools a register's
page-split fragments (NVMe's 64-bit `CAP` table spans five of them).

Two documents, the same measured surface, inverse failure modes: RISC-V Debug gets names but not
bits or register association; NVMe gets bits and register names but not mnemonics. Neither is
hidden behind a strict `0.000` — the decomposition says precisely what to fix next for each.

*Authoritative tracking:* `docs/tasks/PDF-VARIANT-DIGESTION.md` (`.4a.1` added the register-field
eval surface; `.4a.2` the RISC-V Debug gold + field-name/completeness decomposition; `.4a.3` the
NVMe gold + the register-scoped bit-structure recall).
