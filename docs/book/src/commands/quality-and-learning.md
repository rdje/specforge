# Quality, Validation, And Learning Commands

These commands are what make `specforge` more than a one-shot extractor.

This page is the operational command map.
For the deeper rationale behind validation, fixture truthfulness, and cross-document learning, continue with:

- [Validation And Learning](../quality/validation.md)
- [KG Bench And Fixtures](../quality/kg-bench.md)
- [Corpus Memory And Priors](../quality/corpus-memory.md)
- [Corpus Knowledge Base](../quality/corpus-kb.md)

## `validate`

```bash
cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/<document_key>/intent_ir.json
```

`validate` writes a deterministic stage-local `validation_report.json` and backannotates the artifact.

It reports things like:

- direction coverage
- graph direction coverage
- width coverage
- visual classification observations
- negative-knowledge prior matches
- temporal conflicts
- semantic conflicts
- residual decisions
- overall score

## `project-validation`

```bash
cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/.../intent_ir.json
```

This command validates the passed artifacts and refreshes the tracked crash-safe snapshot docs, especially:

- `VALIDATION_SNAPSHOT.md`
- the managed validation block in `LIVE_ACHIEVEMENT_STATUS.md`

It also consumes validation-level rescan guidance and writes a local generated extractor-selection target list.
That currently includes negative-knowledge corroboration findings plus visual-motif corroboration findings for prior-classified normative visuals that still need VLM/multimodal confirmation:

`generated/validation/rescan_plan.json`

That plan is deliberately advisory, but it is now replay-oriented rather than only descriptive.
Each target carries the current artifact path, typed replay inputs such as `source_ir`, `semantic_ir`, or `evidence_ir`, a structured command-hint sequence, and an explicit `planned_not_executed` status.
Non-decisive semantic-role arbitration findings now also feed this queue through the same bounded local `nlp-enrich -> semantic -> intent? -> validate` replay lane used for other evidence-level semantic rescans.
Fallback-only resolved semantic roles now feed that same queue as well, so a carried role without observation-backed consensus becomes an explicit local replay target instead of a passive warning.
Alias-dependent semantic consensus findings now feed it too, so meaning that still depends only on alias grounding can be replayed for stronger non-alias corroboration through the same local pipeline.
Prior-guided semantic consensus findings now feed it as well, so meaning that converged with learned-prior help can be replayed for stronger current-document corroboration through the same local pipeline.
Graph-direction coverage guidance now feeds it too, so already-canonical but still graph-uncovered signals can be replayed for actor-relative direction recovery through the same local pipeline.
Actor-port-gap guidance now feeds it too, so relation-only graph remnants can be replayed for actor-relative port synthesis through that same local pipeline.
Graph-direction self-conflict guidance now feeds it too, so preserved actor-aware conflict ids can be replayed for stronger same-actor direction disambiguation through the same local pipeline.
Signal-semantic-conflict guidance now feeds it too, so preserved `semantic_conflict_*` ids can be replayed for stronger role disambiguation through that same local pipeline.
Source-stage missing-VLM-enrichment guidance now feeds it too, but through a SourceIR-local `enrich -> validate` loop before any EvidenceIR rebuild is attempted.
Evidence-stage missing-VLM guidance now feeds it too, but through a source-side `enrich -> evidence -> validate` loop before any evidence-local NLP or downstream canonical rebuild is attempted.
Evidence-stage structural-KG guidance now feeds it too, but through a narrower evidence-local `nlp-enrich -> validate` loop before any downstream canonical rebuild is attempted.
Evidence-stage normative-residual guidance now feeds it too, but through a narrower evidence-local `nlp-enrich -> validate` loop before any downstream canonical rebuild is attempted.
Evidence-stage signal-polarity-conflict guidance now feeds it too, but through a narrower evidence-local `nlp-enrich -> validate` loop before any downstream canonical rebuild is attempted.
Evidence-stage signal-semantic-conflict guidance now feeds it too, but through a narrower evidence-local `nlp-enrich -> validate` loop before any downstream canonical rebuild is attempted.
Evidence-stage negative-knowledge guidance now feeds it too, but through a bounded `SourceIR -> EvidenceIR -> validate` replay loop so learned caution can demand stronger current-document corroboration without pretending the conflict or residual has already escaped into canonical stages.
Protocol connectivity endpoint guidance now feeds it too, so already-canonical signals that still have only a producer side or only a consumer side can be replayed for missing endpoint recovery through the same local pipeline.
Infrastructure clock/reset sourcing is intentionally not in that replay family; it stays a system-contract note when the protocol document does not name a concrete producer.
Interface-signal conflict guidance now feeds it too, so preserved direction/width conflict ids can be replayed for stronger local interface-shape disambiguation through the same local pipeline.
Temporal-conflict guidance now feeds it too, so preserved contradiction ids can be replayed for stronger local timing corroboration through the same local pipeline.
Signal-connectivity conflict guidance now feeds it too, so preserved producer-ambiguity conflict ids can be replayed for stronger local disambiguation through the same local pipeline.
Signal-polarity conflict guidance now feeds it too, so preserved active-level conflict ids can be replayed for stronger local polarity disambiguation through the same local pipeline.
Temporal actor-grounding guidance now feeds it too, so already-typed but still-actorless temporal rules can be replayed for explicit actor-relative drive/sample recovery through the same local pipeline.
Temporal clock-grounding guidance now feeds it too, so already-typed but still-clockless temporal rules can be replayed for explicit clock or edge recovery through the same local pipeline.
Temporal cycle-window guidance now feeds it too, so already-typed but still-unbounded temporal rules can be replayed for explicit bound recovery through the same local pipeline.
For visual-motif corroboration, that sequence now starts with a local VLM `enrich_source_ir` hint, then rebuilds `EvidenceIR`, then validates the current artifact.
By default, `--rescan-vlm-provider auto-local` prefers a ready local Ollama `qwen2.5vl:7b` model and falls back to a ready local LM Studio `qwen2.5vl:7b` model before emitting the install-guiding Ollama hint.
Use `--rescan-vlm-provider ollama`, `--rescan-vlm-provider lm-studio`, or `--rescan-vlm-provider skip` to force the generated hint, and `--rescan-vlm-model <model>` to bake a model override into the plan.
The command hints are there so `rescan-plan` and opt-in `converge --rescan-plan <plan>` runs can rebuild the right stage safely from machine-readable args instead of scraping a prose note.
The compact live-status queue now also projects the replay-input kind chain plus a concise action summary for each pending recommendation, so the first review surface already tells operators what will be replayed.

The plan still does not mutate IR, suppress findings, run rescans automatically, or promote facts from prior memory.
It only tells downstream loops which current conflict or residual ids deserve targeted rechecking and stronger local corroboration.

Use it when the live baseline should be updated, not just an individual artifact.
If the existing local rescan plan already contains executed recommendation summaries, the refresh preserves matching entries and projects their verdict/delta summary into the validation snapshot and live-status projection.

## `rescan-plan`

```bash
cargo run --manifest-path Cargo.toml -- rescan-plan
```

`rescan-plan` reads `generated/validation/rescan_plan.json`.
By default it is a dry-run inspector: it reports pending `planned_not_executed` recommendations and prints the artifact path, extractor lane, replay inputs, recommended action, related ids, current automation status, and structured command hints that would be used.
When a pending recommendation has no replay inputs, related ids, or command hints, those fields render as explicit `none` values instead of disappearing from the preview.
Use `--document-key <key>` to scope a multi-document plan to one document; the scoped view still selects only pending `planned_not_executed` recommendations, so executed matches and missing document keys produce no pending work.
When a limit is supplied with a document key, the limit is applied after the pending/document filter so earlier recommendations from other documents do not consume the scoped budget.

To execute the current pending hints explicitly:

```bash
cargo run --manifest-path Cargo.toml -- rescan-plan --execute
```

Execution is deliberately narrow.
The command does not shell out through the display strings.
It parses the structured `executable` and `args`, accepts only the repository-local `cargo run --manifest-path Cargo.toml -- ...` shape, and dispatches only whitelisted stage commands in-process:
The structured hint must also keep `working_directory` as `.`, include real SpecForge args after the `--` separator, use a known rescan intent, and keep that intent aligned with the SpecForge subcommand lane.
Replay source and artifact path arguments must name non-empty relative paths, not `.`, and cannot include `..` parent traversal; empty, current-directory, absolute, or upward-traversing replay paths are rejected before execution.
Accepted local provider values are `ollama`, `lmstudio` / `lm-studio`, and `skip`; OpenAI hints are rejected by the executor.
Malformed provider options, including missing providers, repeated provider/model/classification flags, missing provider or model values, flag-shaped provider or model values, unsupported provider values, and unsupported args, are rejected before execution.

- `ingest`
- `enrich` with local `ollama`, local `lmstudio`, or `skip`
- `evidence`
- `semantic`
- `intent`
- `validate`

The rescan executor intentionally rejects OpenAI enrichment hints for now, so replayable visual corroboration stays local-first unless that policy is deliberately changed later.

After a recommendation executes successfully, `rescan-plan` validates the target artifact again and records a neutral outcome in the local plan:

- `executed_validated_no_change`
- `executed_validated_changed`

That is still not a canonical truth decision.
It means the relevant stage was rebuilt and validated, and that the validation fingerprint/score/finding-count surface either changed or did not.
Any fact promotion still has to survive current-document evidence, validation, and arbitration.
Executed recommendations also carry an optional `execution_summary`.
That summary preserves before/after validation snapshots, score and finding-count deltas, added/removed finding ids, a conservative arbitration verdict, and an explicit promotion gate.
Before/after score labels remain compact for sparse validation reports: score plus grade, score only, grade only, or `n/a` when both fields are absent.
Added and removed finding-id lists are sorted and deduplicated before they are persisted, so review diffs stay stable when validation reports contain duplicate or unsorted ids.
If one execution both adds and removes findings, the added finding keeps the verdict in regression review even when the score and finding count are unchanged.
The conservative verdict values are:

- `validated_no_change`
- `possible_improvement_review_required`
- `regression_review_required`
- `neutral_change_review_required`

Promotion is recorded separately from validation movement.
A no-change run stays `not_promoted_no_change`, and any changed run stays `not_promoted_review_required` with blockers such as `validation_delta_is_not_truth_promotion` and `current_document_evidence_review_required`.
If a score or grade appears where the previous validation snapshot had none, that is treated as neutral review-required drift rather than automatic improvement or regression, because there is no comparable score delta.
The same summary also carries a structured `promotion_review` record.
Changed outcomes are `human_review_required`, require an approval record, and keep `canonical_mutation_allowed` false until current-document evidence, validation-delta direction, explicit mutation scope, and prior-memory non-authority have all been reviewed.
No-change outcomes are `not_reviewable_no_change`.
`promotion_review` is only the review-requirement descriptor.
It is not the approval artifact, and it cannot approve canonical mutation.
Future approval artifacts stay local/generated by default until a deliberate canonical mutation workflow defines tracked approval evidence with current-document support, validation-delta review, exact mutation scope, prior-memory non-authority, reviewer intent, artifact fingerprints, and replayable provenance.
That makes the machine-readable plan say the same thing as the product policy: a favorable validation delta is a review signal, not canonical truth.

The convergent loop can consume the same plan after stability:

```bash
cargo run --manifest-path Cargo.toml -- converge /path/to/spec.pdf --target isf --rescan-plan generated/validation/rescan_plan.json
```

Add `--execute-rescan-plan` only when you want those whitelisted hints to run.
Unlike standalone `rescan-plan`, the convergence hook automatically filters the queue to the current source document key.
That automatic filter uses the same pending-only, filter-before-limit selection rule as `rescan-plan --document-key`.
The convergence summary reports whether the post-rescan artifact snapshot changed and emits an arbitration status such as `dry_run_not_promoted` or `changed_requires_validation_review`.
That keeps rescans visible without pretending that a changed validation surface is already an improvement.

## `kg-bench`

```bash
cargo run --manifest-path Cargo.toml -- kg-bench
```

This runs the tracked truthfulness fixture set under `crates/specforge/test_data/kg_quality/`.

The benchmark surface exists to lock:

- gold paths
- negative expectations
- conflict surfacing
- residual quality
- prior-guided before/after behavior

It is the repo’s main extraction-truthfulness regression harness.

## `learn-priors`

```bash
cargo run --manifest-path Cargo.toml -- learn-priors generated/intent_ir/.../intent_ir.json
```

This updates the cross-document learning plane.

The main output is:

`generated/prior_memory/corpus_memory.json`

What it learns today:

- actor-taxonomy priors
- semantic phrase priors
- semantic modality-reliability priors
- temporal phrase priors
- table-shape priors
- visual-motif priors
- negative-knowledge priors
- extraction-profile priors

The safety rule is critical:

- priors widen interpretation of the current document
- priors do not directly author canonical truth

Extraction-profile priors are the newest family: `learn-priors` clusters the accepted
documents' derived structural fingerprints (the same vendor-name-free fingerprint
`corpus-cluster` shows you, at the same default `0.6` threshold, so the families you
*see* are the families the store *learns*) and persists one advisory profile per
**multi-document** cluster — its shared structural signature, its member documents, and
which extractor strategies fired across those members with per-member support. A
single-document cluster is never persisted, because a cluster of one carries no
*cross-document* pattern to reuse. For example, a run over ten validated `IntentIR`
artifacts currently yields profiles like:

```text
extraction_profile_priors: 2
  support 5: tilelink_1_7_1, tilelink_1_8_0, i2c_bus, hbm2_dram, generic_flash_bus
  signature: shape:conditional_rules:b2, shape:protocol_states:b0, shape:relations:b2, ...
```

— the two TileLink versions fall into the same learned profile as other bus-protocol
specs purely on shared shape, with no vendor list anywhere. The `fired_extractors`
side of a profile is honestly empty for documents whose evidence predates the
extraction run manifest; it fills in as the corpus is re-ingested. Looking a profile up
later uses a strict subset rule (every signature feature must be present in the new
document's own fingerprint), and the only consumption these profiles will ever be
allowed is **activate-only**: a matching profile may switch on an opt-in extractor for
a look, never switch off a default-on one — so a profile can add recall but can never
suppress or fabricate a fact.

Visual-motif priors remember recurring visual patterns, while negative-knowledge priors remember conflict and residual archetypes that should make future extraction more careful.
Visual-motif priors now have a first bounded consumer: `EvidenceIR` may add a prior-memory classification observation for a current unknown visual asset when its local caption matches a unique learned motif.
Negative-knowledge priors now have bounded validation consumers too: `EvidenceIR` may surface caution for repeated local signal-semantic conflict patterns, while `SemanticIR` and `IntentIR` may surface caution for repeated carried conflict and residual packet patterns.
That caution does not suppress evidence, remove residuals, or decide semantic truth.

## `corpus-kb`

```bash
cargo run --manifest-path Cargo.toml -- corpus-kb generated/intent_ir/.../validation_report.json
cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality
```

This refreshes tracked corpus knowledge-base pages under `corpus_kb/`.

The first page families are:

- `corpus_kb/failures/validation-findings.md`
- `corpus_kb/benchmarks/kg-fixtures.md`

They project validation reports and KG fixture outcomes into managed blocks while preserving human-authored synthesis around those blocks.
This is the first concrete `R15g` surface: persistent corpus-level synthesis beside the KG and prior memory.

The boundary is strict:

- corpus KB pages can inform humans, future LLM sessions, benchmark design, and prior-candidate design
- corpus KB pages cannot directly mutate canonical IR or become typed priors without a separate validation-gated promotion path

## `corpus-cluster`

Specifications from the same source tend to be *organised* the same way — the
same kinds of tables in the same places, the same balance of registers versus
timing rules versus prose. If `specforge` could recognise that a new PDF "looks
like ones it has already digested," it could reuse what worked on those — which
is how extraction quality should improve as the corpus grows. `corpus-cluster`
is the first, honest step toward that: it groups the documents you have already
ingested into **emergent families** and shows you the shared structure that
defines each family, so you (and, later, the extractor) can see those patterns.

```bash
cargo run --manifest-path Cargo.toml -- corpus-cluster
cargo run --manifest-path Cargo.toml -- corpus-cluster --evidence-root generated/evidence_ir --threshold 0.6
```

It reads every `generated/evidence_ir/<document_key>/evidence_ir.json` you have
on disk, derives a small **structural fingerprint** for each one — a coarse
count bucket per typed surface (registers, protocol states, signal constraints,
actor→signal relations, actors, serial-frame fields, conditional rules) plus
which extractor strategies actually fired — and clusters documents whose
fingerprints overlap enough (Jaccard similarity ≥ `--threshold`, default `0.6`).
It prints each multi-document family with the feature tokens its members share,
then the single-document (unique-shape) specs.

For each multi-document family it also prints an advisory **extraction profile**:
the *union* of extractor strategies that fired across the family's members, each
with its per-member support (e.g. `registers.field_table (2), registers.prose
(1)`). This is more than the shared signature above — the signature is the
*intersection* (only features every member has), while the profile keeps a
strategy even if it fired on only some members, because that is exactly the
"what tends to work for documents shaped like this" knowledge the reuse plane
wants. The profile is honestly **sparse today**: the "which strategies fired"
information only exists on documents (re-)ingested recently, so most families
currently print `none recorded yet (run after a corpus re-ingest sweep)` — and
that is reported truthfully rather than guessed. As more of the corpus is
re-ingested, the profiles fill in.

Two design choices keep it trustworthy and generic:

- **No vendor names anywhere.** The cluster key is the shared *structure*
  itself, never a baked-in "ARM"/"NXP"/"Intel" string (ADR 0006). The families
  are emergent — e.g. the three versions of one technical reference manual fall
  together because they share a shape, not because the tool was told they are
  related — so this keeps working on any chip-spec PDF, including ones from
  vendors it has never seen.
- **Read-only and additive.** The command never rebuilds, mutates, or scores any
  IR, and it changes no extraction behavior. It is a *window* into the corpus
  structure, not an action on it. The learn side of reuse now exists —
  `learn-priors` persists each multi-document family as an advisory
  extraction-profile prior (see above) — but actually steering extraction with a
  profile remains a separate, deliberately gated, activate-only step
  (`CORPUS-PATTERN-REUSE.3b.3`).

The fingerprint is fully deterministic, so the same corpus always produces the
same families — making this a stable thing to inspect and to build the reuse
plane on top of.

## `enrich` and `nlp-enrich`

These are narrower enrichment entrypoints used when you want to operate on the staged pipeline more manually.

They are still valuable, but `converge` is the preferred user-facing path when you want the full loop.

`nlp-enrich` can learn prose aliases for signals during Form 2 reclassification, but it treats that alias map as evidence-sensitive state: markdown/list prefixes, link targets, and source-layout labels such as table, figure, and section markers are filtered before they can become reusable aliases.

## `recover-register-bits`

Some specifications draw a register's bit layout as a **picture** — a horizontal
strip of labelled cells — and never repeat those bit numbers in the field table
beside it. The deterministic table reader then recovers the field *names* but
leaves every bit position empty, because the numbers simply are not in any text
it can read. This command fills that gap *without guessing*.

```text
recover-register-bits <evidence-ir> [--vlm-provider ollama|open-ai|lm-studio|skip] [--vlm-model <name>] [--dry-run]
```

**How it works, and why you can trust it.** A vision model is good at reading the
field *names*, their left-to-right *order*, and each cell's *width* off the
diagram — but it is unreliable about the absolute bit *number* a wide cell starts
at (a wide cell prints two edge numbers and the model often grabs the wrong one).
So this command throws the model's bit numbers away and **reconstructs** them from
a law the model cannot break: a register's fields tile it edge-to-edge from the
most-significant bit down to bit 0, with no gaps and no overlap. Reading the
widths MSB→LSB and laying them down from the top gives every field's exact range.

The reconstruction is accepted **only** when it is provably grounded, by two gates:

1. **The widths tile a standard register width** (8, 16, 32, 64, or 128 bits). If
   the model misread one cell, the widths will not sum to a real register size,
   and the result is rejected.
2. **The names match the register's own field table.** If the model invented,
   dropped, or renamed a field, the name sets disagree, and the result is
   rejected.

When either gate fails, the bit positions stay an **honest residual** — they are
left empty, never filled with a guess. That is the whole point: on a register
whose layout has reserved gaps the field table does not name (or whose wide
reserved cell the model misreads), the gates fire and nothing is fabricated; on a
register whose named fields tile every bit, the bits come back exactly. The
command reports, per register, whether it `recovered` the bits or left a labelled
residual, and writes the enriched `EvidenceIR` back only for the registers it
recovered.

Like the other enrichment commands it defaults to `--vlm-provider skip` (a no-op,
so it is safe to wire into any run) and honors the `SPECFORGE_VLM_HELPER` test
hook, so the live vision model is never required for the build or the tests.

**Honest note on live behavior (as of `2026-06-08`).** The reconstruction *math*
is proven — given clean widths it recovers a 32-bit register's 14 fields exactly.
What is still maturing is the *read*: on real, dense register diagrams the local
`qwen2.5vl:7b` reliably reads field names and order, but it sometimes mis-sizes a
cell (e.g. inventing a narrow "reserved" gap, or an off-by-one width). When it
does, the widths no longer sum to a standard register size, the first gate fires,
and the command leaves an honest residual rather than a wrong bit. That is the
intended behavior — the command is wired to *confirm, never guess* — so today, on
the RISC-V Debug diagrams, it reports residuals rather than recovered bits. A
sharper read (a stronger vision model, image upscaling, or a tighter prompt) is
the lever that turns those residuals into recoveries; until then nothing is
fabricated. Two upstream pieces also have to line up for the command to even reach
a diagram: the register's bit-layout image must be classified as a register
bit-field diagram, and the register's field table must be captured as one table
(not split into fragments). Tracking: `docs/tasks/EXTRACTION-GAP-FIX.md`.

## `extract-contracts` and `signal-resolve`

These two commands ask a local LLM to read the *hard prose* a specification
buries in sentences — the meaning that tables and verb-pattern heuristics miss —
and turn it into typed knowledge you can trust. They are deliberately additive:
each one reads an existing `EvidenceIR` artifact and writes a richer one, so you
can run them after `evidence`/`converge` without disturbing anything already
recovered. Both default to the production-default provider (local Ollama with
`qwen2.5vl:7b`), and both accept the same provider menu as `enrich`/`nlp-enrich`.

```text
extract-contracts <evidence-ir> [--provider ollama|open-ai|lm-studio|skip] [--model <name>] [--dry-run] [--max-statements 0]
signal-resolve     <evidence-ir> [--provider ollama|open-ai|lm-studio|skip] [--model <name>] [--dry-run] [--max-statements 0] [--grounding-signals <csv>]```

**`extract-contracts`** mines normative prose ("the master must hold ADDR stable
until READY is asserted") into typed `ActorContract`s for the R16 constrained-
verified surface. Why you can rely on it: the model's answer is *not* trusted on
sight. Each candidate passes through the fails-closed `parse_constrained_contract`
gate — a malformed or unparseable answer is counted as a `schema_reject`, never a
fabricated contract — and survivors run entailment, so any claim the evidence
cannot back becomes an explicit **residual** rather than a silent "fact." That is
why `validate`'s `constrained:` block now reports real `schema_rejects`: the
honesty is measured, not assumed. (See the [R16 chapter](../direction/temporal-intent-capture.md)
for the contract model itself.)

**`signal-resolve`** mines actor→signal `drives`/`reads` edges from the same hard
prose, appending grounded, de-duplicated `ActorSignalRelation` records to the
knowledge graph the `SemanticIR` already consumes for actor ports and graph
direction. Why it stays safe: every proposed edge must clear grounding gates —
an upper-case signal name, a non-empty actor, a relation that is exactly `drives`
or `reads`, and (with `--grounding-signals`) membership in the declared signal
set — and is checked against existing edges before it is kept. An ungrounded or
malformed answer is skipped, not invented. (See [Actor Connectivity](../domain/actor-connectivity.md).)

Use `--dry-run` to preview which sentences would be sent without making a single
LLM call, `--max-statements` to cap how many candidates are sent (handy for a
quick check), and `--provider skip` to exercise the full wiring offline. The
three text commands — `nlp-enrich`, `extract-contracts`, and `signal-resolve` —
now route through one shared OpenAI-compatible *text* transport, so the
`SPECFORGE_VLM_HELPER` hook, provider flags, and OpenAI auth behave identically
across them; only the per-command prompt, response parsing, and response-token
budget differ. (`enrich` honors the same `SPECFORGE_VLM_HELPER` test-hook and
provider-flag convention, but keeps its own image-capable transport because it
sends diagrams, not text.)

## `extract-constraints-llm` (research lane)

```text
extract-constraints-llm <evidence-ir> [--vlm-provider ollama|open-ai|lm-studio|skip] [--model <name>] [--max-sentences 0]
```

This is the **LLM-primary, Rust-grounded constraint extractor** — the live test of
the "replace, don't patch" thesis from the extraction-quality program. Where the
deterministic Pattern extractor matches phrasings it knows, this command hands each
constraint-bearing sentence to a local text model and asks for the *structured*
requirement — `(subject, kind, condition, value)` — then lets Rust ground every
field before anything is kept: the subject must type as a real **signal** (entity
typing — a table reference, feature name, or transaction type is rejected), the
kind must parse, and a condition survives only if the source sentence actually
contains it. What the model proposes but cannot ground is dropped, never invented.
Heads up before you run it: it **replaces** the artifact's `signal_constraints`
in place — point it at a copy if you want to keep the Pattern set side by side.

One honest subtlety the program learned the hard way: a specification's
*validity* requirement — "PBUSER **must be valid** when PSEL, PENABLE, and PREADY
are asserted" — fits none of the obvious kinds (asserted, stable, high, low), and
a model given no way to express a sentence stays silent about *all* of it. The
typed convention is that "must be valid" is a **value constraint with the value
`VALID`**, the same shape the deterministic extractor produces. The prompt states
that convention outright, and if the model names a value constraint without
echoing the value, Rust recovers the value **from the source sentence itself**
(the same "must be ⟨value⟩" reading the Pattern extractor uses) — recovered from
the document or dropped, never guessed.

Measured against the hand-validated AMBA gold (document-level fact recall,
before → after that convention landed): APB **4/6 → 6/6**, AHB **2/6 → 6/6**, and
AXI — which has no validity facts in gold, so it serves as the no-regression
control — steady at **4/4**. Every one of the six previous misses was a
"must be valid" fact, and every one is now recovered. *Authoritative tracking:*
`docs/tasks/EXTRACTION-QUALITY-GAUGE.md`.
