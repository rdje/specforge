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

## SWD protocol facts — frame, operation, state, and interface edge

The ordinary constraint/relation/temporal score is a poor completeness measure for a serial-debug protocol.
SWD's implementation intent is its packet frame, response-branched operation sequence, line/TAP states, and
the clock edge on which the target samples and changes its drive state. `eval-extraction` therefore has four
deterministic tasks that read the corresponding EvidenceIR records directly:

- `serial_frame_field`: field name, width, phase, SWDIO direction, order, and response values;
- `swd_operation`: response/access branch, phase count, data-phase presence, and turnaround placement;
- `protocol_state`: machine plus state identity;
- `interface_edge_timing`: actor, data signal, clock signal, edge, sampling flag, and drive-change flag.

The last key is deliberately all-or-nothing. A record naming the right data signal but the wrong clock, a
falling rather than rising edge, or only sampling without the drive-state change does not match the gold.

The real `seed_swd_derivation.json` contains 29 independently verified facts: 11 frame fields, four operations,
13 states, and the B4.3.1 `target / SWDIO / SWCLK / rising / samples / drive-changes` fact. Against a fresh
repository-local CPU re-ingest, the source-tolerant WIRE score is `P=R=F1=1.000` for all four tasks. The strict
statement-local frame/state view is lower because one source statement can support several correct document
facts; the source-tolerant view re-resolves source drift and filters predictions to the complete gold universe.

One boundary matters: the score proves the EvidenceIR extractors, while separate parity/accounting checks prove
downstream availability. The canonical builders carry the exact four ordered collections and provenance through
SemanticIR and IntentIR, and the adapter gives every record an explicit disposition. They do not translate the
observations prematurely into generic transactions or temporal rules.

There is deliberately no direct behavioral lowering yet. The state records do not contain transitions,
guards, an initial state, or encodings; the frame, operation, and edge records do not contain every wire,
value, activation, and storage binding needed to generate executable ISF. Guessing those bindings would turn
faithful extraction into fabricated intent. Until a record is complete enough for a supported construct, the
adapter will preserve it as an explicit residual while continuing to render unrelated, independently licensed
content.

The current canonical chain comes from a fresh repository-local CPU ingest of the tracked ADI PDF. SourceIR is
ready with 400 pages, 386 visual assets, 210 tables, and 6,784 content elements; convergence stabilizes after two
passes. EvidenceIR, SemanticIR, and IntentIR each contain the exact 11 frame / 4 operation / 13 state / 1 edge
vectors. The adapter records 29 stable protocol residual packets and remains renderable, and its generated ISF
passes FSMGen strict with zero diagnostics. Repository-owned metadata paths are relative, so this promoted
result is current, portable, and reproducible rather than a disposable scoring artifact.

*Authoritative tracking:* `docs/tasks/SWD-SERIAL-EXTRACTION.md` (`.4e` and `.7a`–`.7e`) and ADR 0016.

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
    register-name association gap            59/60 extracted registers have a real (non-synthetic) name
    bit-extent completeness gap              0/179 extracted fields carry a bit position
```

Read that top to bottom. On the real **RISC-V Debug Specification 1.0**, SpecForge recovers the
field *names* well — **20 of 34** gold fields across the `dmstatus` and `dmcontrol` registers,
which is **all 14 of `dmcontrol`** and 6 of `dmstatus`'s 20 (the spec splits `dmstatus`'s field
table across three pages, and the PDF reader dropped the middle page's worth of rows — a real,
located recall miss, not a mystery). It now also recovers each register's real *name*: RISC-V puts
the name not in the field table but in the section heading above it — *"3.14.1. Debug Module Status
(dmstatus, at 0x11)"* — so the reader takes the name from the **nearest preceding heading that
defines a register**, recognised by the universal tell that a register has both a name and an
*address* (the parenthetical carries a `0x` offset). That took register-name association from
`0/60` to `59/60` with **zero invented names** (the one hold-out has no address-bearing heading, so
it honestly stays a placeholder rather than borrow a wrong name). The remaining gap is each field's
*bit position*: those live in a bit-layout *graphic* above the table, which the reader does not yet
parse. Because the strict identity needs the register, the name, **and** the bits, the per-fact
precision/recall on this document is still **0.000** until the bit graphic is read — and that is the
honest truth, not a failure of the eval. The decomposition is what makes the `0.000` *useful*: it
says "the names are there and now the registers are named too; the bit positions are the one thing
left to fix," instead of a single demoralising zero that hides where the value is.

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
    field-name recall (register-agnostic)   28/29 = 0.966
    bit-structure recall (register-scoped)  28/29 = 0.966
    register-name association gap            46/46 extracted registers have a real (non-synthetic) name
    bit-extent completeness gap             201/201 extracted fields carry a bit position
```

NVMe fails the **opposite** way to RISC-V Debug — though it no longer fails on the mnemonic. Here
SpecForge gets the register *names* (because NVMe tables carry a caption like *"Offset 0h: CAP –
Controller Capabilities"* that the reader keeps) and the **bit layout is excellent** — `28/29` of the
gold bit-fields land at exactly the right offset and width across the `CAP`, `CC`, and `CSTS`
registers. The field's *mnemonic* is the interesting part. NVMe lays its registers out as
`Bits | Type | Reset | Description`, so the *name* column is actually the **bit-range** — the reader
would record `"60:59"` as the field name. But the real name isn't missing; it's just in a different
place. Each description opens with the standard defined-term form *"Maximum Queue Entries Supported
**(MQES)**: …"*, and that parenthesized abbreviation **is** the mnemonic. So when the name column is a
bit-range, the reader looks where the name actually lives — the first `(MNEMONIC):` in the description
— and recovers it (`MQES`, `CSS`, `TO`, …). That single change took field-name recall from `0/29` to
`28/29`. The one miss, `CAP.CRMS`, has no clean `(CRMS):` term in this PDF, so it stays an honest
residual — a mnemonic is never invented from a description that doesn't state one.

This is why the surface reports **two** recall views: a field's identity can fail on its name or on
its bit extent independently, and a single number would hide which. Bit-structure recall is
*register-scoped* on purpose — `CAP.CSS` lives at bits 44:37 while `CC.CSS` lives at bits 6:4, so the
same name and even the same extent must not be credited across the wrong register — and it pools a
register's page-split fragments (NVMe's 64-bit `CAP` table spans several of them).

Two documents, the same measured surface, complementary failure modes: NVMe now gets bits, register
names, *and* mnemonics; RISC-V Debug gets field names *and* (now) the owning register's real name,
with the field bit positions — which live in a layout graphic — the one piece left. Nothing is
hidden behind a strict `0.000` — the decomposition says precisely what is fixed and what is left to
fix for each.

*Authoritative tracking:* `docs/tasks/PDF-VARIANT-DIGESTION.md` (`.4a.1` added the register-field
eval surface; `.4a.2` the RISC-V Debug gold; `.4a.3` the NVMe gold + register-scoped bit-structure
recall) and `docs/tasks/EXTRACTION-GAP-FIX.md` (`.2` recovered the NVMe mnemonic from the
description's defined-term prefix, field-name recall `0/29 → 28/29`).

## Declared signals — prose capture, measured then fixed to perfect

Some specs name their signals only in prose, never in a table — the I2C bus is the classic case:
"*Only two bus lines are required; a serial data line (SDA) and a serial clock line (SCL).*"
SpecForge recovers these by reading the prose, and the **declared-signal** surface measures how well.
A declared signal's identity is its **name** plus, optionally, its **direction** (`input` / `output` /
`internal`) — optional because prose rarely states a direction, so a name-only gold matches a
no-direction record, while a wrong direction still scores as a miss.

The honest question for prose capture is twofold: does it find the **real** signals (recall), and does
it *only* find real signals (precision)? On the I2C-bus Specification (UM10204), against a gold that is
the **complete** enumeration of the bus's six physical signals — `SDA`/`SCL`, the Hs-mode `SDAH`/`SCLH`,
and the Ultra-Fast `USDA`/`USCL`, each verified against the spec's own "signals" sections:

```text
specforge eval-extraction crates/specforge/test_data/llm_eval/seed_i2c_signals.json
  declared_signal        P=1.000 R=1.000 F1=1.000  (tp=6 fp=0 fn=0)   [source-tolerant]
  -- declared-signal surface (complete-gold precision) --
    precision (assumes gold enumerates all signals)  6/6 = 1.000
```

Recall is **perfect** (all six real signals) and precision is now **1.000** — but that precision is the
*result of a fix*, and the story of how it got there is the point. When the gold was first authored,
precision was **0.600**: the extractor also emitted four things that are *not* bus signals — `ACK` and
`NACK` (the acknowledge/not-acknowledge *conditions* on the SDA line, §3.1.6 — not separate wires), `DDC`
(the Display Data Channel, a *different* bus, §4.6), and `SDR` (an I3C "standard data rate" acronym). The
gold deliberately excludes those, so they surfaced as named false positives rather than being quietly
accepted — which told the next fix exactly what to remove.

The fix is a small piece of **agnostic grammar**, not a denylist of those four tokens (the runtime never
learns chip-specific names). Prose introduces a signal as "*&lt;descriptor&gt; (NAME)*", e.g. "serial data
**line** (SDA)". The original rule accepted the abbreviation if *any* word in a short preceding window was
a wire descriptor — but that let a non-wire noun phrase qualify whenever a descriptor appeared earlier in
it: "An acknowledge clock **pulse** (ACK)" and "Not Acknowledge clock **pulse** (NACK)" qualified via
`clock`; "Display Data **Channel** (DDC)" and "standard data **rate** (SDR)" via `data`. The fix requires
the **head noun** — the word immediately before the abbreviation — to be the wire noun itself. The real
lines keep their head (`SDA`→`line`, `USCL`→`clock`, `SDAH`→`data`); the over-captures don't (`ACK`→`pulse`,
`DDC`→`channel`, `SDR`→`rate`), so all four drop while all six real signals stay. The same rule generalises
to any spec: a parenthetical abbreviation is a signal only when its noun-phrase head is a wire.

A subtlety worth understanding: the **complete-gold precision** is reported separately *because* the
ordinary statement-scoped precision can't see these over-captures. Each spurious signal is attributed to
its own synthesized declaration statement, not to the one labelled sentence, so the per-statement scorer
reports `fp=0`. Precision over the *produced signal set* only makes sense when the gold enumerates every
true signal — true for a small, fully-specified bus like I2C, which is why the surface labels the
assumption explicitly rather than reporting a precision that would be wrong on a doc with a sampled gold.

*Authoritative tracking:* `docs/tasks/PDF-VARIANT-DIGESTION.md` (`.4a.4`/`.4a.5` built the surface + the gold
and measured 0.600); `docs/tasks/EXTRACTION-GAP-FIX.md` (`.1` fixed it to 1.000 via the head-noun rule).

## Auditing the broadened extraction — a VLM precision estimate

Authoring a hand-verified gold (the sections above) is the gold standard, but it does not
scale: SpecForge now extracts registers, fields, and signals from many table shapes across
the whole 82-PDF corpus, and only a handful of those documents have a gold. How trustworthy
is the *rest*? The `audit-extraction` command gives an honest, automated **estimate** without
authoring a gold per document.

The idea is a **proposer/verifier audit**: the deterministic pipeline already *proposed* a
classification for each table it extracted from; the VLM independently *verifies* it by
re-reading the table's rendered image. For a bounded, reproducible sample of the
intent-bearing tables in a `SourceIR`, the VLM is shown the picture and asked one structural
question — "an extractor read this as a register / signal / encoding / timing table; looking
only at the image, is that correct?" The fraction it confirms is a **table-kind precision
estimate**, and every disagreement is printed by name as a flagged mismatch for a human to
check. The VLM is an imperfect oracle, so this is deliberately an *estimate*, never a score —
and nothing is hidden: a disagreement becomes a review item, not a silent deletion.

By default the command is **plan-only** (`--provider skip`): it lists exactly which tables it
*would* audit, making no VLM calls — useful for seeing the sample and as the CI-safe path.
Add a provider to run the live audit:

```text
# plan-only — list the sampled intent-bearing tables (no VLM calls)
specforge audit-extraction generated/source_ir/<key>/source_ir.json --sample 8

# live — estimate precision over the sample with the local VLM (real run, RISC-V Debug)
specforge audit-extraction generated/source_ir/1_0_risc_v_debug_specification/source_ir.json \
    --sample 8 --provider ollama
  ...
  audited: 8
  judged: 8
  consistent: 2
  vlm_errors: 0
  table_kind_precision_estimate: 0.250
  flagged_mismatches: 6
  flagged: table_0080 page=page_0084 kind=register reason="The table contains description not access or reset values."
  flagged: table_0085 page=page_0089 kind=register reason="...not a detailed bit-field definition table."
```

### The audit discriminates — and agrees with the gold

The estimate is only useful if it is *low where extraction is weak and high where it is
strong* — otherwise it is just noise. A live run on two opposite-shaped register specs shows
exactly that, and the result independently confirms what the hand-authored golds found:

| document | live estimate | what the VLM flagged | matching gold finding |
|---|---|---|---|
| RISC-V Debug | **0.250** / 0.375 (two seeds) | register tables "lack bit positions" / "not a bit-field definition table" | `.4a.2`: bit-extent 0/179 — RISC-V's bits live in the layout *graphic*, not the field table |
| NVMe 2.0a | **0.750** | register tables confirmed; one `timing` table is really a feature matrix | `.4a.3`: bit-structure recall 0.931 — NVMe's tables *do* carry bits |

The audit and the gold are two **independent** methods (one a VLM re-reading the picture, one
a human-verified answer key), and they agree on which document has trustworthy register
extraction. That agreement is the point: it lets the audit stand in for a gold on the hundreds
of corpus documents that will never get one. The NVMe run also caught a genuine false positive
(a feature-matrix table misclassified as `timing`) — surfaced by name, for a human to fix.

One honest limit: the audit reads the table *image*, so it needs the document's rendered
`normalized/` assets on disk. If they were reclaimed (e.g. by `clean --scope source-normalized`),
the run reports `vlm_errors` and `table_kind_precision_estimate: n/a (no table judged)` rather
than inventing a number — re-ingest the document to audit it.

Two design choices keep it honest and **chip-spec-PDF-agnostic** (the project's non-negotiable
signoff rule — no hardcoded chip vocabulary in the runtime). First, the sample is chosen and
the verdict is judged **purely by table structure** — register/field/signal/encoding/timing
are universal digital-design categories, and the VLM prompt carries no chip, vendor, or
protocol names. Second, the VLM is a **targeted, sampled tool, not a full-document pass**: a
VLM call per table is far too slow on a table-heavy spec, so the sample size and a seed are
explicit and reproducible (the same `--seed` always picks the same tables).

The estimate is named precisely — *table-kind* precision, the rate at which the VLM agrees a
sampled table is the kind we extracted from. That catches the dominant false-positive mode for
the broadened extraction (pulling records from a table that is not what we thought it was);
it is not a per-field fact check, and the name says so.

*Authoritative tracking:* `docs/tasks/PDF-VARIANT-DIGESTION.md` (`.4b.1` built the audit
harness; `.4b.2` records the live corpus-scale estimate).
