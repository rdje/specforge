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

`validate` writes a deterministic `validation_report.json` beside the artifact and backannotates validation
metadata into the artifact path you explicitly passed. That path is authoritative for the write. If you validate
a copied snapshot whose embedded `artifact_layout` still names the canonical generated tree, only the copied JSON
and its adjacent report change; validation does not follow the embedded output layout or materialize stage-owned
siblings. Extracted evidence and semantic records are not rewritten.

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
- the persisted NLI extraction-quality gauge (`extraction_quality_labeled` /
  `extraction_quality_not_entailed` / `extraction_quality_abstained` /
  `extraction_quality_not_entailed_pct` — `n/a` until `nli-verify` or `converge` has measured
  the document, plus a majority-erroneous warning and a staleness warning; see the
  [validation chapter](../quality/validation.md))

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
The tracked projection remains the last reviewed boundary. Host-local generated artifacts may advance
independently; they do not become validated-state authority until the refreshed snapshot, live block,
and declared report identities pass review and land together.

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

## `grits-consensus`

```text
grits-consensus <witnesses-json> [--min-agree 2] [--adjudicate-out <queue.json>]
```

How good is the table extraction the rest of the pipeline is built on? Every
signal table, register map, and timing table SpecForge reasons over comes from
one tool — docling — reading the PDF, so it is worth measuring honestly how often
docling gets a table *cell* right. The honesty is the whole point: **you cannot
grade an extractor against itself.** A score docling computes about its own output
is circular. So the gold here is not docling's opinion — it is the *agreement of
independent witnesses* whose mistakes are uncorrelated with docling's.

Two witnesses read each table a different way. **pdfplumber** reconstructs the
table geometrically from the PDF content stream — no machine learning at all, so
its errors have nothing to do with docling's; it is strongest on ruled tables.
**qwen2.5vl** reads the rendered table image with vision, so it catches the
borderless tables a geometric tool misses. A cell enough witnesses agree on
becomes the silver gold for that cell; a cell the witnesses *split* on is not
silently averaged — it is flagged for adjudication, because a disagreement is
exactly where the truth is uncertain. docling — the system under test — is then
scored against that gold and is never itself a witness.

The command owns only the metric. Its input is the witness JSON produced by
`scripts/grits_cross_tool.py`, which runs the two witnesses plus docling, aligns
the tables by page, and matches them. For each matched table `grits-consensus`
builds the consensus gold, scores docling against it with the GriTS content metric
(precision / recall / F1 per table and in aggregate), and reports how many cells
the witnesses split on — the human-flag count. `--min-agree` sets how many
witnesses must agree for a cell to count as gold (default `2`).

`--adjudicate-out <queue.json>` turns the score into a work-list: it writes every
cell where docling disagrees with the consensus gold — each one a *candidate*
docling error, with the gold value and docling's value side by side. Feed that
queue to `scripts/grits_adjudicate.py` to render the disputed table regions from
the source PDF, and an evidence-grounded agent (or a person) rules each cell
against the rendered page — **never** by a correlated vote. That closes the loop
honestly: the witnesses propose, the source decides. On the real APB spec it
surfaced a genuine docling bug — a word-merge `"forAPB5"` where the page plainly
prints `"for APB5"`.

The command is read-only with respect to the IR: it consumes a witness JSON and
optionally writes an adjudication queue, and it never mutates a pipeline artifact.
The witness extractors run via a gitignored `.venv-eval` (the system Python is
PEP-668-managed), so this is an offline table-quality measurement path, not a step
in a normal `converge` run. Rebuild it after a repository move with
`bash scripts/bootstrap_eval.sh`; the script verifies
`requirements/eval-macos-arm64.lock.txt`. *Authoritative tracking:*
`docs/tasks/GRITS-CROSS-TOOL.md` (`.2`/`.3`).

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
cargo run --manifest-path Cargo.toml -- corpus-kb --validation-snapshot VALIDATION_SNAPSHOT.md
cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality
```

This refreshes tracked corpus knowledge-base pages under `corpus_kb/`.

The first page families are:

- `corpus_kb/failures/validation-findings.md`
- `corpus_kb/benchmarks/kg-fixtures.md`

They project the reviewed validation snapshot and KG fixture outcomes into managed blocks while preserving human-authored synthesis around those blocks.
This is the first concrete `R15g` surface: persistent corpus-level synthesis beside the KG and prior memory.

The tracked validation projection must use `--validation-snapshot`; positional validation reports are
an unreviewed local mode and cannot be mixed with it. The complete KG projection currently covers
156/156 passing fixtures. Its aggregate uses one pass/fail/path row per fixture with expanded details
only for failures; prior-candidate fixture evidence is one item per line beside the paired JSON
manifest. Verify the tracked dependency/output contract without rewriting pages with
`perl scripts/check_corpus_kb_currentness.pl --report`.

The boundary is strict:

- corpus KB pages can inform humans, future LLM sessions, benchmark design, and prior-candidate design
- corpus KB pages cannot directly mutate canonical IR or become typed priors without a separate validation-gated promotion path

## `replay-constraints`

```bash
cargo run -- replay-constraints generated/evidence_ir/<doc>/evidence_ir.json
cargo run -- replay-constraints generated/evidence_ir/<doc>/evidence_ir.json --json
```

Answers one question about a persisted artifact: **does today's extractor still produce the
constraints this artifact publishes?**

That is not the same question as "what does the artifact contain", and the difference is easy to
miss. A persisted artifact is frozen at the code generation that wrote it, and most of them cannot be
rewritten: the evidence stage reads a document's normalized bundle, and only 24 of the 78 corpus
documents keep one. So a count taken over `generated/` measures what SpecForge **published**, which
drifts away from what its current code **does** every time an extraction rule changes.

The command closes that gap without rebuilding anything, because the deterministic constraint surface
is a function of the artifact's own statements rather than of the source document. It re-runs the
real producer over `extracted_statements` and compares by the producer's own record identity —
subject, kind, value, condition, negation and source text, never the ids.

```text
command: replay-constraints
persisted_deterministic_records: 16
reproduced: 4
not_reproduced: 12
not_reproduced: sigcon_0002 OAS must_be_stable cond="" — EXTRACTION-QUALITY-GAUGE.3k.1 reference-magnitude, CORPUS-COVERAGE.2.50a post-passive-binding-only
not_reproduced: dyn_sigcon_0011 DERR must_be_low cond="parity check is suspended during power-down. Signals are shown with tPARAC=0 …" — no positional gate refuses this subject — the kind, condition or negation moved
unpersisted_replay_record: dyn_sigcon_0013 DERR must_be_low cond="parity check is suspended during power-down" src="5. AERR, DERR are driven LOW when parity check is suspended during power-down. Signals are …"
```

Each record that no longer comes out is listed with the gate that stands in its way, so a published
record can be attributed to the rule that retired it rather than guessed at. When no gate does, the
record's own **condition, negation and a bounded source excerpt** are printed beside it — the second
and third lines above are the same fact, and reading them together shows that what changed is a
condition that used to run past its own sentence. Without those fields the report can only say that
something moved, and finding out what would mean re-deriving exactly what the command exists to spare
you.

Two properties make the output usable:

- **A published subject the artifact no longer declares is still granted its trial.** The replay adds
  a declaration for it first, so "not reproduced" can never quietly mean "the catalog shrank" — and
  the granted names are printed, because a shrinking catalog is itself worth seeing.
- **The verdict is asymmetric, and the command says so.** "Not reproduced" is sound: a widened
  catalog can only admit more subjects. The opposite direction is not, because the build applies
  convergence stages this replay does not, so `unpersisted_replay_records` is evidence to read rather
  than a number to quote.

Calibration is a shape rather than a pair of numbers, because the numbers move with every extraction
slice: an artifact the current binary itself wrote reproduces completely, while a document frozen at
an older generation reproduces partially or not at all. Run the command for the current figures.

### Which producers it judged, and which it could not

Three deterministic producers mint constraint records: the statement path, the dynamic value-binding
path, and the table-row reader. The first two need only the artifact's own statements. The third
reads the document's structured tables, so it needs the `SourceIr` — and, less obviously, one whose
typed classifications survived being loaded.

They do not always survive. An artifact written by an older schema is loaded with every
classification reset to `Unknown`, deliberately: only the current schema plus a verified proof ledger
carries the authority to say *this table is a signal description*.

That reset withdraws two things at once — the label's **content** and its **authority** — and only
the authority had to go. The content is a pure function of the table's caption, header rows and body
rows, every one of which survives the load, so the current classifier can simply be asked again.
This command asks, because it is a diagnostic: it writes nothing, promotes nothing, and the
predicate that decides canonical authority still answers *no* for those artifacts.

Where that question is asked turned out to matter more than whether it is asked. Recomputing the
label *inside* the artifact — a method that writes it back onto the record — is refused by the
compiled information-flow graph, because a function that reads raw evidence and writes a semantic
classification reaches control it has no registered right to. So the classifier returns its verdict
as a value and this command holds the write, which is how every other diagnostic in the codebase is
shaped. No boundary moved.

```text
row_stratum_judged_documents: 78
row_stratum_judged_from_rederived_labels: 51   (…carrying NO canonical authority)
rederived_stratum_replayed_records: 227
row_stratum_unjudged_documents: 0
```

**Read the third number, not the first.** Judging those 51 documents judges an empty set — not one
of them carries a persisted table-row record, so there is nothing to reproduce or fail to reproduce,
and a headline of *78 judged* would be the inverse of the silent zero this command exists to end.
What the recomputation buys is the **227**: records the current producer mints across 51 documents
that were previously unreadable, with nothing persisted to compare them against. It is a recall
signal, not a reproduction verdict, and the report labels it as one.

What this does **not** buy is any change to what the canonical pipeline extracts. Those tables still
cannot reach the production reader, and the reason has nothing to do with classification: the
evidence build needs each document's normalized markdown bundle, and for all 51 that bundle has been
reclaimed. The bundle, not the label, is the binding constraint, and only a re-ingest moves it.
Prior guidance is not applied here either, so a table that only corpus memory would promote is still
invisible — stated rather than assumed away.

## `replay-declarations`

```bash
cargo run -- replay-declarations generated/evidence_ir/<doc>/evidence_ir.json
cargo run -- replay-declarations --evidence-root generated/evidence_ir
```

The same question as `replay-constraints`, one stage later and about a different producer: **does
today's reader still declare the signals this artifact declares?**

SemanticIR builds a document's signal catalog by reading each `Signal <name> is <direction> width
<W>.` statement the evidence stage produced. That catalog decides far more than a list of wires —
an obligation whose subject is not in it is demoted to a residual — so a declaration the reader
cannot parse costs the document the signal *and* everything it says about it.

Asking the persisted SemanticIR what the reader does is not an option for most of the corpus. The
semantic stage refuses a legacy or proofless EvidenceIR outright:

```text
error: invalid stage artifact: EvidenceIR schema version 2 is legacy/proofless and inspection-only
```

51 of the 78 persisted documents answer that way, so their catalog was written by a binary that no
longer exists here. Like the constraint replay, this command sidesteps that: the declaration surface
depends only on the artifact's own statements, so it runs the real reader offline over **every**
document, including the ones the chain refuses.

```text
command: replay-declarations
opened: 463 (sentences that opened as `Signal <name> …`)
read: 461
distinct_read_names: 296
refused: 2
unrecovered: 2
  no_direction_and_no_width: 1
  width_text_unread: 1
refused: no_direction_and_no_width names [statement_0623] <- Signal names are the base name, when …
refused: width_text_unread RUSERCHK [statement_6182] <- Signal RUSERCHK is width ceil((USER_DATA_WIDTH USER_RESP_WIDTH)/8)
```

Three things in that output are deliberate:

- **A sentence that never opened as a declaration is not counted.** `opened` counts only sentences of
  the form `Signal <name> …`; everything else is not an event, and reporting it would drown the real
  refusals in ordinary prose.
- **`unrecovered` is computed from this replay's own reading, never from the stored catalog.** A
  specification commonly declares the same wire twice — a signal-description table and a version
  matrix — and a refusal whose identity another statement declares successfully has lost nothing.
  Joining against the stored catalog instead would answer a question about the binary that wrote it,
  which is the substitution this command exists to refuse.
- **The three refusal arms mean different things.** `no_direction_and_no_width` is nearly always
  English prose that happens to open with the word "signal", because the evidence stage only
  synthesizes a declaration from a row that yielded an attribute — the first line above is exactly
  that. `width_text_unread` is the arm where a real declaration was lost: the reader had an attribute
  in hand and threw the identity away with the width text it could not finish reading.

Measured over the corpus on `2026-09-14`: 78 documents replayed and **none skipped**, 3,196 sentences
opened as declarations, 2,927 read, 269 refused — 186 `width_text_unread`, 80
`no_direction_and_no_width`, 3 `name_not_an_identifier` — and **94 identities** that no read
declaration in their own document recovers, spread over 24 documents with one technical reference
manual holding 49 of them. Run the command for the current figures; they move with every reader
change, which is the point of having it.

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
direction. Why it stays safe: the EvidenceIR's current-document declarations are
mandatory authority. Every proposed edge needs an identifier that exactly matches
that catalog, a non-empty actor, and a relation that is exactly `drives` or `reads`.
Identifier case and length have no semantic meaning. `--grounding-signals` may
only narrow the document catalog; it cannot add an external name or disable
grounding. An ungrounded, case-colliding, or malformed answer is skipped, not
invented. (See [Actor Connectivity](../domain/actor-connectivity.md).)

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

## `extract-constraints-llm`

```text
extract-constraints-llm <evidence-ir> [--vlm-provider ollama|open-ai|lm-studio|skip] [--model <name>] [--max-sentences 0]
```

This is the **LLM-primary, Rust-grounded constraint extractor** — the live test of
the "replace, don't patch" thesis from the extraction-quality program. It is no
longer only a research lane: as of the measured default flip
(`LLM-PRIMARY-PROMOTION.5`), `converge` runs this same replacement **automatically
after the pipeline stabilizes whenever a live `--nlp-provider` is used** (provider-free
runs stay on the Pattern surface; `--no-promote-constraints-llm` opts out — see the
[pipeline commands page](pipeline.md)), the surface swap is **recorded in the
extraction manifest** as `constraints.llm_primary` (no silent surface changes —
the fingerprint and clustering plane can see which documents carry the promoted
surface), and any persisted extraction-quality gauge is **dropped on replace**
(the old measurement's constraint ids are definitively gone, so keeping it would
report a number about a surface that no longer exists). Where the
deterministic Pattern extractor matches phrasings it knows, this command hands each
constraint-bearing sentence to a local text model and asks for the *structured*
requirement — `(subject, kind, condition, value, clause)` — then lets Rust ground
every field before anything is kept: the subject must type as a real **signal** or as a
document-declared **message field** (entity typing — a table reference, feature
name, or transaction type is rejected), the kind must parse, and a condition
survives only if the source sentence actually contains it. What the model
proposes but cannot ground is dropped, never invented.

A span can also **declare an identifier of its own**, and the extractor now reads that. When a sentence
names a token in apposition to the word *signal* — *"The select signal, PSEL, is asserted"* — that clause
declares `PSEL` for **that span**, even where the document's signal table declares only the parameterised
`PSELx`. This is not an alias and no suffix is ever read: `PSEL` and `PSELX` stay distinct identities, and
the local one is never added to the document's catalog, so the identical subject in any other sentence
still resolves only against the table. The deterministic extractor has honoured that reading since the
same-clause appositive rule was introduced; the model-primary path was refusing the very fact it recovers.
Measured across every span this path visits in the promoted corpus, the grammar declares **exactly one**
identifier the catalog does not already hold — so what it admits and what it recovers are the same thing.

A subject the catalog does not declare is dropped — but one spelling of a *declared* signal
survives that test, and only one. A document that writes **`ARLEN[7:0]`** where it declares
`ARLEN` with a stated width of 8 has named the whole signal, so the slice is resolved to the
signal and the record carries `ARLEN`. Nothing else is. A **proper sub-slice** is left to be
refused: `AWSNOOP[3]` of a stated width 4 is one bit of four, and resolving it would put
*"AWSNOOP must be LOW"* on the record when the document only said *"AWSNOOP[3] must be tied
LOW"* — a strictly stronger obligation, fabricated silently. A slice whose signal states **no
width** is refused too, because the comparison cannot be evaluated, and only 209 of 353
declared names in the promoted corpus state a width at all. Nor does a bare **qualifier**
resolve: *"WSTRB bits"*, *"Subordinate LAPM"*, *"snoop response"*. That was measured rather
than assumed — of the 16 ungrounded subjects that carry a declared name, resolving all of them
would make **4 correct**, while inventing one subject and strengthening one obligation, so only
the full-width case is wired. Both guards are load-bearing and neither implies the other: a
top-bit slice `X[w-1]` satisfies the width comparison on its own, and is excluded because its
span does not start at bit 0.

**A whole span can also be refused, before any of that runs.** When the sentence the model was shown is a
**table row whose first cell binds a configuration** — `| LTI_MMU = True LTI_GPC = False | … |` — no
obligation inside it holds on its own, so nothing is minted from it at all. A compatibility matrix scopes
each cell on two axes: the row key and the column header. *"Subordinate LAPM is tied LOW"* is true in one
cell of one row, and a record that states it flatly is simply wrong. The extractor therefore refuses the
span rather than trying to repair it, and that was measured before it was wired. Across the 78 documents
of the persisted corpus 109 such spans exist, and **7 records** had ever been minted from one — every one
of them wrong as written. Reconstruction was considered and rejected: in 5 of the 109 cases the **column**
header binds the other axis and lives in a *different* statement, so lifting the row key into the record's
condition would leave the obligation just as wrong while making it look checked. Nor does a condition the
model supplies earn an exemption — the condition test is literal occurrence in the span, and the row key
occurs there along with everything else in the row, so carrying it evidences nothing. An ordinary table
row binds nothing and is untouched; so is a two-cell definition row, whose single value cell scopes
nothing.

`clause` is the newest of those fields and the only one that exists to answer a question
about the record rather than to fill it. A grounded record cites the whole statement it
came from, so nothing downstream can tell *which* obligation inside that statement
produced it — and a sentence routinely states two. That is not academic: measured over
the 149 persisted LLM-primary records, wiring in the deterministic paths' positional
subject gates would refuse 7 and **4 of the 7 refusals would be wrong**, every one
because the gate narrows to the *first* modal clause while the record was minted from a
later one. The model is the only reader that knows which it read, so it is asked — and
the answer is checked rather than trusted: the clause must occur **literally** in the
sentence the model was shown, and a proposal that names a clause the document never
wrote is dropped whole. Whitespace is normalized on both sides, because a model that
re-wraps a long clause has still quoted it; nothing else is relaxed, and the clause is
never re-derived from the record's own kind or value, which would simply be a second
reader disagreeing with the first exactly where it matters. A proposal that names no
clause is not refused — carrying the clause and re-measuring the gates against it are
deliberately separate steps. The field typing is fully
deterministic on packet protocols: a name the document declares in its own field
tables (`TxnID`, `DBID` — the `message_field_records` inventory) types as a
**field** with no model call at all, while a name declared in a signal table
always stays a signal. Only names the document declares in *neither* place reach
the model's judgment, and the probe-tested boundary there is phrasing: prose that
says *"the ReturnNID field"* types as a field, a bare mention may not — which is
exactly why the declared catalog, not the model, carries the ontology.

What happens to a field-typed subject changed with the field-constraint surface:
it is no longer *dropped*, it is **routed**. An obligation whose subject is a
declared message field — *"For all other REQ channel messages, the TagOp field is
inapplicable and must be 0"* — is real protocol intent, just intent about message
*content* rather than about a wire. Such a record now lands in the artifact's
`message_field_constraints` surface, carrying the field name, the containers the
document declares it in (`TagOp` lives in the Request channel, Response packet,
and Data packet), the same typed kind/condition/value as a signal constraint, and
full statement provenance. Crucially, a field obligation gets **no discipline
discount**: it passes through exactly the same grounding gates as a signal
constraint — the condition-subject guard, the permissive-frame guard, the
source-grounded value recovery, and the provenance-merging de-duplication —
before it is kept. Measured live on the persisted CHI artifact (its field catalog
re-derived from the persisted tables by the real extractor): the signal surface
collapses to exactly the document's four real flit-valid wires
(`REQFLITV`/`RSPFLITV`/`SNPFLITV`/`DATFLITV`), while the `TagOp` and `PBHA`
obligations — previously either junk "signal" constraints or silent drops — come
out as two field-scoped records, the twice-stated `TagOp` fact merged into one
record carrying both statements. The wire-based controls are untouched: APB, AHB,
and AXI re-measure at their exact prior volumes with zero field constraints and
perfect labeled scores. One boundary stays honest: a field *presence* requirement
(*"the MPAM field must be included on the REQ and SNP channels"*) has no slot in
the constraint-kind vocabulary yet, and the model stays silent on such sentences —
a candidate future kind, recorded as a residual rather than guessed at.

One more guard runs on the subject itself. In *"ASKSTOP must be LOW **when
ACTIVATEACK is LOW**"*, only ASKSTOP carries an obligation — ACTIVATEACK merely
names the *situation* — yet a model happily proposes a constraint on both. The
grounding pass therefore checks where the subject lives in the sentence: a
subject that appears **only inside subordinate conditional clauses**
(when/if/unless/while/until and kin) is the condition's subject, not an
obligation's, and the proposal is dropped. The check is careful about real
grammar: a subject that also appears in the main clause is kept, a clause never
leaks past the end of its own sentence, and *"while **driving** HREADYOUT LOW"*
is recognized as a prescribed concurrent action — an obligation on HREADYOUT —
rather than a condition.

The *framing* of the sentence is checked the same way. *"It is **recommended**
that a Manager sets HPROT[0] HIGH"* and *"An alternative implementation **would
be** for HSEL to be tied HIGH"* state a recommendation and a hypothetical — not
obligations — so a "must be HIGH" proposal from them is dropped. Two details
keep this honest: a mandatory clause always wins (*"It is permitted to issue an
Exclusive Write … in this case HEXOKAY **must** be deasserted"* keeps its very
real obligation), and the frame is judged only in the **sentences that mention
the subject** — an incidental *"an OKAY response **can** be given in a single
cycle"* elsewhere in the paragraph does not soften the ERROR-procedure
requirements that follow it.

One last guard handles a failure the promotion gold gates caught in the wild: the model
occasionally **misspells** a subject it otherwise read perfectly — *"SYSCOREQ and SYSCOACK
must be deasserted when ARESETn is asserted"* came back with a `SYCOREQ` record (one letter
dropped). Such a phantom name would ground and then silently vanish downstream where canonical
stages keep only declared signals. The extractor's subjects are supposed to be *quotes from
the sentence*, so a proposed subject that does not occur in its own source sentence is treated
as suspect: if the sentence contains **exactly one** declared, signal-shaped token within one
character edit of it, the subject is snapped to the document's own spelling; any ambiguity —
or a candidate the document does not declare — and no correction happens. A subject that does
appear in its sentence is never rewritten, so the snap can fix a typo but can never invent or
substitute a name.

The grounded set then gets the same **polarity refinement** the normal evidence build
applies — another lesson the promotion gold gates taught. When a document grounds a signal's
active level (a *"Active-High"* column in its own signal table, explicit polarity prose), the
build pipeline collapses a symbolic *"must be deasserted"* into the concrete level it means —
SYSCOREQ and SYSCOACK are declared active-high, so *"must be deasserted when ARESETn is
asserted"* becomes **must be LOW** — before any downstream consumer reads the surface. A
replaced surface has to honor that same invariant, or the typed temporal layer (and every
other consumer of the constraint kind) suddenly sees a symbolic `DEASSERTED` where the rest
of the pipeline — and the hand-validated gold — knows the document already said `LOW`. That
exact gap is what the AXI temporal gate caught: the promoted surface lost both reset-rule
facts purely because the replacement skipped the refinement. The refinement uses **only the
document's own persisted polarity records** — a signal whose polarity the document never
grounds keeps its symbolic asserted/deasserted kind, because turning "deasserted" into a
voltage level without the document saying which way the signal is active would be a guess.

Finally, the surviving set is **de-duplicated**: a specification often restates
the same requirement in several places, and the same `(signal, kind, value,
condition)` fact re-extracted from three statements becomes *one* record that
lists all three supporting statements — the duplicate noise goes away while
every scrap of provenance is kept. Facts that differ in their condition stay
separate records, because a different condition is a different requirement.
Heads up before you run it: it **replaces** the artifact's `signal_constraints`
and `message_field_constraints` in place — point it at a copy if you want to
keep the Pattern set side by side.

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

All current production prompts share the same neutrality boundary. They describe typed digital-hardware
relations, constraints, contracts, conditions, entity types, diagrams, tables, audits, or register fields;
document-owned labels are opaque and must be copied from the current sentence, declaration catalog, or visible
image. Signal/value-carrier catalogs preserve declaration/provenance order rather than sorting by spelling.
Entity typing never receives the identifier at all: an opaque placeholder replaces it in context and helper
transport, leaving typed evidence and grammar as the only inputs. Undeclared proposed contract signals become an
explicit residual, and the LLM-primary constraint path cannot validate its own proposed subject. No prompt teaches
a real vendor/protocol signal example or frames every input as one protocol family. Alpha-renaming tests require
prompt policy to stay identical after replacing the current-document symbols.

Measured against the hand-validated AMBA gold (document-level fact recall,
before → after that convention landed): APB **4/6 → 6/6**, AHB **2/6 → 6/6**, and
AXI — which has no validity facts in gold, so it serves as the no-regression
control — steady at **4/4**. Every one of the six previous misses was a
"must be valid" fact, and every one is now recovered. With the condition-subject
guard added on top, the extractor currently scores **perfect precision, recall,
and F1 (1.000)** on the labeled constraint statements of all three documents —
the three remaining false positives, each a condition read as an obligation,
are gone, and recall stayed intact. *Authoritative tracking:*
`docs/tasks/EXTRACTION-QUALITY-GAUGE.md`.

## `nli-verify`

```text
nli-verify <evidence-ir> [--vlm-provider ollama|open-ai|lm-studio|skip] [--model <name>]
```

This is the **semantic** grounding gate — the natural sibling of
`extract-constraints-llm` above. Rule-based grounding asks a *string* question
(does the constraint's signal name appear near its source text?), which catches a
model inventing a signal out of thin air but misses a subtler and more common
error: reading a *condition* as an *obligation*. "*`PBUSER` must be valid **when**
`PSEL`, `PENABLE`, and `PREADY` are asserted*" does not oblige `PSEL` — `PSEL`
being asserted is the *situation*, and the obligation is on `PBUSER`. A string
match sees "`PSEL`" and "asserted" and waves it through.

`nli-verify` closes that gap with **Natural Language Inference**: it treats each
constraint's source sentence as the *premise* and the constraint-as-a-claim as the
*hypothesis*, and asks a **text** model one well-posed question — *does the source
actually support this claim?* A claim that adds, changes, contradicts, or turns a
condition into an obligation is **not entailed**, and is reported as a likely
hallucination / residual candidate. It carries any stated condition into the claim
("`PSTRB` must be LOW **for read transfers**"), so a conditional constraint is
judged fairly rather than failed for naming its trigger.

Each run reads an `EvidenceIR` and prints: the constraint count; the list of
**not-entailed** claims (each with its constraint id, subject signal, claim text,
and source sentence — the items worth a second look); a calibrated NLI-oracle
**split-conformal** line (a tier-agreement accept threshold with its coverage and
empirical error); and a one-line **extraction-quality gauge** summary. Run on the
real AMBA APB spec it flags genuinely mis-extracted constraints — protocol *states*
(`ACCESS`), *width parameters*, the *clock*, and *condition* signals that were
never the obligation's subject.

**It only ever strengthens, and it is CI-safe.** A confident "not entailed" flags a
claim; a clear "entailed" keeps it; and if the model is unavailable or its answer
is unclear, the gate **abstains** — it leaves the existing rule-based grounding in
charge, so a model outage can never silently delete what SpecForge extracted. It
uses a *text* model (entailment is pure language reasoning — negation, scope,
condition-vs-obligation — not vision), rides the same provider plumbing as the
other LLM steps, and honors a test hook that mocks the model, so the build and the
test suite never depend on a running model. `--vlm-provider skip` (the default-safe
no-op) abstains on every claim and leaves the artifact untouched; `--model <name>`
overrides the default text model.

**The measurement survives the terminal.** Unless the pass labeled *nothing*,
`nli-verify` also **persists** what it measured into the `EvidenceIR` as the
document's `extraction_quality_gauge`: which model judged, how many constraints
were checked, how many the source entailed, how many it did *not*, how many were
abstained, and the exact ids of the not-entailed constraints. A *vacuous* pass
(every verdict abstained — e.g. the provider was down) is deliberately **not**
persisted, so it can never overwrite a real prior gauge with empty data. `converge`
reuses the same measurement as its standing post-stability gauge, and `validate` /
`project-validation` then report it provider-free (the
`extraction_quality_not_entailed_pct` surface). To make the gate *active* rather
than a report — demoting any non-entailed contract into the residual decisions
during IntentIR construction — run `specforge intent semantic_ir.json --nli-verify`
instead; demoted, not deleted, so even a wrong verdict costs a review rather than a
lost fact, and `validate` then surfaces the count as `nli_demoted_contracts`.
*Authoritative tracking:* `docs/tasks/NLI-ENTAILMENT-VERIFIER.md`,
`docs/tasks/NLI-INTENT-GATE.md`, `docs/tasks/NLI-GATE-METRIC.md`,
`docs/tasks/EXTRACTION-QUALITY-GAUGE.md` (`.0`).

## `entity-type`

```text
entity-type <evidence-ir> [--vlm-provider ollama|open-ai|lm-studio|skip] [--model <name>] [--max-subjects 0]
```

This is the diagnostic that answers a blunt question: *are the subjects of this
document's constraints actually signals?* It exists because the extraction-quality
program found that they are not always — on a packet protocol like CHI, names such
as `TxnID` and `DBID` are message **fields**, not wires, and a naive constraint
extractor will happily mint "`DBID` must be …" records that pollute the signal
surface. `entity-type` makes that error visible and measurable before it reaches
canonical IR.

It runs the same **LLM-judges, Rust-grounds** harness the constraint extractor
uses, only pointed at one question. For each distinct constraint subject in the
artifact, Rust first gathers that token's grounding evidence — the sentences it
appears in, and whether the document declares it in a signal table or a field
table — then the local text model proposes a type, Rust grounds that judgment
against the evidence, and a final enforcement gate keeps only the subjects that
type as a real **signal**. The command prints the type breakdown, how many
subjects it would keep versus filter, and a sample of the filtered ones (the
spurious non-signal subjects), so you can see exactly which "constraints" were
about something other than a wire.

It is **read-only** — it never rewrites the artifact, it only reports. The
ontology it measures here is the same one the `extract-constraints-llm` promotion
enforces deterministically (a name the document declares in its own field tables
types as a field with *no* model call at all), so `entity-type` is the way to
inspect that typing on an artifact you already have. `--vlm-provider skip` makes
it grounding-only: with no model to ask, ungrounded tokens stay `Unknown` rather
than being guessed, which keeps the command honest and CI-safe. `--model`
overrides the default text model (`qwen2.5:14b-instruct`), and `--max-subjects`
caps how many distinct subjects are typed (`0` = all). *Authoritative tracking:*
`docs/tasks/EXTRACTION-QUALITY-GAUGE.md` (`.1`).

## `extract-conditions`

```text
extract-conditions <evidence-ir> [--vlm-provider ollama|open-ai|lm-studio|skip] [--model <name>] [--max-constraints 0]
```

A flat requirement is often a *broken* requirement. When the document says
"PSTRB must be LOW **for read transfers**" but the extracted record keeps only
"PSTRB must be LOW", the claim has quietly become stronger than the
specification — and an entailment check will rightly flag it. `extract-conditions`
repairs that class of loss: it recovers the dropped condition clause and attaches
it to the constraint, so the obligation says exactly what the document says, no
more.

It is the second member of the extraction-quality harness, and it works exactly
like its sibling — Rust hands the model the source sentence and the bare
obligation, the model proposes the condition clause, and Rust **grounds** the
proposal against the source before keeping it: a condition the model invents but
the sentence does not contain is dropped, never written. The captured, grounded
condition lands in the constraint's `condition_text`, and the command reports how
many candidates it considered and how many conditions it captured.

Two guards keep it cheap and safe. It only spends a model call on a constraint
that is **still flat** *and* whose source sentence actually carries a condition
cue (`when`, `until`, `before`, `after`, `while`, `unless`, `whenever`, `once`,
`during`, `provided`, `if`, `on receiving`, `in the same cycle`) — an
already-conditional constraint or a sentence with no cue is left untouched. Unlike
`entity-type`, this command **modifies the artifact in place** (it writes the
updated EvidenceIR back to disk), so point it at a copy if you want to keep the
original side by side. `--vlm-provider skip` is a no-op, `--model` overrides the
default text model (`qwen2.5:14b-instruct`), and `--max-constraints` caps how many
candidates are processed (`0` = all). *Authoritative tracking:*
`docs/tasks/EXTRACTION-QUALITY-GAUGE.md` (`.2`).
