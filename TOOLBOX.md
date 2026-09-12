# TOOLBOX.md — SpecForge's diagnostic & debug toolbox (USE THIS FIRST)

> **STANDING DIRECTIVE (non-negotiable).** SpecForge ships a deep, self-explaining diagnostic surface
> for the PDF→IntentIR→`.isf` pipeline. For **any** "why did extraction produce X", "why is this signal/
> register/transaction missing", "why won't this IntentIR lower to `.isf`", "why did a gold/`kg-bench`
> fixture move", or "is this a real spec we under-extracted or an honest non-target" question, **reach
> for these tools FIRST and systematically** — never eyeball the IR, never guess a root cause, never
> offer a strategy menu before a tool has shown you the exact mechanism and location. If the existing
> tools cannot surface the WHY and WHERE, the next step is to **build** a probe (own it in a task-tree),
> not to speculate. Honest **residual over fabrication**; runtime **PDF-agnostic**, no chip/vendor/
> protocol-name lists (ADR 0006).

This file is the **single, authoritative catalog** of SpecForge's own tools. Each entry says **WHAT**
it is, **WHEN** to reach for it, **HOW** to run it (exact command), and **WHAT** the output looks like.

- Mirrored surfaces (kept in lockstep): the mdBook command chapters under `docs/book/src/commands/`
  (`pipeline.md`, `quality-and-learning.md`) and the validation chapter `docs/book/src/quality/`;
  the standard `DOCTRINE_ENFORCEMENT.md` (the enforcement model); the decision records under
  `docs/decisions/`.

## Enforcement — this is not optional, and not "trust me"

Using the toolbox is **mechanically enforced**, so it cannot be silently skipped:

- A **Rust code change cannot commit** unless its owning task leaf (`docs/tasks/<TREE>.md`) carries the
  acceptance checklist below with the required boxes ticked AND backed by real SpecForge tool output —
  enforced by `scripts/check_task_acceptance.sh`, run by the general doctrine enforcer
  `scripts/check_doctrines.sh` via `.githooks/pre-commit` (E3) and `scripts/run_ci.sh` / CI (E4).
  Demonstrated: a staged Rust change without that evidence is **blocked**.
- The project's **deterministic oracles** — `kg-bench` (156/156), the WIRE-BASED-100 constraint+temporal/
  relation golds (= 1.000), the byte-identical evidence/`.isf` checks, `cargo fmt`/`clippy`/`test` —
  **re-run the real tools** in `scripts/run_ci.sh` / CI, so any number you cite is independently
  re-verified; a fabricated final state does not reproduce and fails. That is the "not trust-me" leg.
- The portable model for how every mechanizable doctrine is enforced is `DOCTRINE_ENFORCEMENT.md`.

### The task-acceptance checklist (required for any Rust code-change commit)

Every code-landing task leaf (`docs/tasks/<TREE>.md`) MUST carry this checklist, each required box
**ticked `[x]`** and backed by the cited tool output. An **unticked or unbacked required box BLOCKS the
commit** (`scripts/check_task_acceptance.sh`). Copy this into the leaf:

```markdown
## Acceptance Checklist (enforced)
- [ ] **REPRODUCE / MEASURE** — <the baseline: a validate metric/finding, an adapt --dry-run count, a kg-bench result, a measured surface count>
- [ ] **ROOT CAUSE (WHY + WHERE)** — <SpecForge tool output naming the mechanism + location: a validate finding/metric (`evidence_*`/`semantic_*`/`intent_*`, `document_class`, `rationale:`), an `adapt --target isf` `blocking_reason`, a kg-bench fixture diagnostic, a failing `cargo test <name>`, an `evidence --dry-run` diff, `file:line`>
- [ ] **ADDRESSED (verified)** — <the change does what it should, measured per item: before→after, a per-document count, a recovered surface (e.g. "AHB Subordinate 25 → 27 ports")>
- [ ] **NO REGRESSION** — <kg-bench 156/156; WIRE-BASED-100 = 1.000; the relevant golds byte-identical; run_ci.sh / cargo fmt|clippy|test green; emitter changes: FSMGen --strict --check 0 new diagnostics>
- [ ] **GENERICITY (ADR 0006)** — <the rule is structural/behavioral grammar, no chip/vendor/protocol-name list; or N/A + reason>
- [ ] **LOCKSTEP** — <mdBook / live-docs / KM card updated, or N/A + reason>
```

**LOCKSTEP has one sub-clause that only a producer change can satisfy.** If the slice **deletes or
replaces a production rule**, say which book text described the old behaviour and what happened to it —
or state that none did. A deleted producer leaves its description standing, and the book is the owner's
only window: `WIRE-BASED-100.4a` found the temporal chapter asserting a resolver that had been gone for
a month, with every gate green. `BOOK-BEHAVIOUR-CURRENCY.0` measured the two ways to look, and only one
of them works:

```bash
# Useless here: no deleted production function is named by symbol anywhere in the book (measured: 167
# removed functions, 0 true hits). Do not build a symbol scanner on this evidence.
# What actually finds it — the ~18-line present-tense behavioural population, adjudicate by hand:
grep -rnE 'SpecForge (now|currently) [a-z]+|(now|currently) (resolves|infers|uses|treats|derives|applies|promotes|accepts|recognizes|recognises|canonicalizes|canonicalises|expands|merges)' docs/book/src --include='*.md'
```

This is deliberately **not** hard-gated. The measured exposure is one stale claim per quarter, and the
hard-gated boxes are reserved for what a green build must never hide; a required tick for a rare,
judgement-bound check buys false positives, not currency (`[[book-behaviour-currency-instrument]]`).

**Hard-gated (required, must be ticked + evidence-backed): ROOT CAUSE, ADDRESSED, NO REGRESSION.**
REPRODUCE/MEASURE, GENERICITY, and LOCKSTEP are part of the template and good practice (and SpecForge's
own doctrine), but are not hard-blocked by the gate, to avoid false-positives. The whole task tree's
"start→finish" is the sequence of its leaves, each passing this checklist, plus the tree's Acceptance
Criteria.

**A box is EARNED, not ticked.** A `[x]` you write is a *claim*; the proof is the **oracle re-run**. The
NO-REGRESSION box must cite a **named, re-runnable oracle** — `kg-bench`, the WIRE-BASED-100 golds, the
byte-identical evidence/`.isf` check, `cargo` — so CI re-executes exactly that and earns the box
independently of your tick (`DOCTRINE_ENFORCEMENT.md` §6.1). The local pre-commit hook is leg 1
(presence); the oracle re-run (`scripts/run_ci.sh` / CI, currently manual-only) is leg 3 — the
un-self-tickable one.

### Published-claim evidence (required for every slice)

The implementation checklist above proves a change; it does not independently earn a current number or status
published from that change. Every commit and review description also carries exactly
`Published-claims: none` or comma-separated stable claim IDs under `CLAIM_VERIFICATION.md`.

For each claim ID, the durable task/claim evidence must name:

- **RE-DERIVE** — exact canonical source plus a repository-root-relative reproducing command/accessor;
- **FALSIFY** — a competing hypothesis, a dimensionally different oracle, and a tracked known-bad RED case;
- **DURABILITY** — tracked producer/control paths, complete artifact-input identity, and a stale-state gate;
- **STATUS** — `verified`, `incomplete` with every missing leg named, or `superseded` with its successor.

Use `none` only when the slice publishes or changes no current actionable assertion. A passing regression suite
may be one named oracle, but repeating the implementation's own classifier or source interpretation is not an
independent falsification leg.

Four questions decide whether the legs are earned, and each one has caught a real defect in this tree
(`CLAIM_VERIFICATION.md` §2, §3):

- **What does this check still permit?** A digest over a governed region proves the text did not change, never that
  the numbers inside it still re-derive — which is how published counts go stale under a green gate. A check and the
  thing it checks must not share a parent.
- **Would the competing account have produced this same observation?** If yes, the leg has illustrated, not tested.
  This binds a published *mechanism* — an account of why a value moved — exactly as it binds the value.
- **Was this shape already adjudicated here?** The cheapest oracle is the owning task tree and the commit body that
  introduced the value. If an earlier ruling went the other way, name the difference or the earlier ruling wins.
- **Is the classifier derived from the producer, or from a description of it?** A vocabulary authored from what a
  surface is believed to publish is green precisely where it is blind.

Attribute a RED check by revert-and-re-apply, or by re-deriving from each revision's own producer input
(`git show <rev>:<path>`) — never by reading a diff and inferring the cause.

The active registry is `doctrine/claim_verification/claims.jsonl`. Use
`perl scripts/check_claim_verification.pl --report` to see status/publication resolution,
`--check` for the real gate, and `--self-test` for its positive plus missing/unknown/duplicate/stale/untracked RED
matrix. Commands are argv arrays and all producer/input/evidence paths are tracked, digest-bound, and covered by
the declared stale check; the checker never evaluates registry content through a shell. Each falsification
control also binds one exact known-bad source region. Two `control_audit` fields are published here because the
checker fails on any other value: `ignored_candidates` and `untracked_candidates` are **0**, and a nonzero
result for either raises `governed producer census contains ignored or untracked candidates`. `cited_controls`,
`exact_red_evidence`, and `governed_producers` are counts nothing fails on, so read them from `--report`; what
is gated is their *relation* — every cited control must bind an exact RED region, so the first two are always
equal. `[claim: claim-provenance-gate-active]`

For the current-surface authority sweep, use `perl scripts/check_current_claim_census.pl --check`, `--report`,
`--produce`, and `--self-test`. A field is carried here only when a **control** fails if it moves or an
**authored decision** fixes it — never because a trajectory shows it has held, which is the licence `.10`
retired. Exactly two qualify. `candidate_closure.unresolved` **0** is gated: `validate_candidate_closure`
raises an error for any candidate lacking exact evidence or a current registered annotation, so `--check`
fails the moment it leaves zero. **5** views is authored: `.3a.0` froze five `required_views` and the registry
declares them. **Everything else that report prints is a repository-derived constant and is read from
`--report`**, including `authority_outcomes.derived` and `authority_outcomes.identity_gated`, which were
carried here on `.6a`'s 29-revision trajectory until `LIVE-DOCUMENT-PRESSURE-HEADROOM.2c` registered one
surface and moved both of them — `current_surfaces` 39 -> 40, noticed and withdrawn in that commit, and
`identity_gated` 7 -> 8 on the same sentence, missed until `.11` enumerated the population. `evidence_units`,
`authority_outcomes.excluded`, `authority_outcomes.registered`, and the rest of `candidate_closure` are moved by
ordinary, often unrelated work, and three measured mechanisms move them. A slice
that prepends or seals a rolling-ledger record changes the unit and exclusion totals — 15 rises, 2 falls, and 10
no-changes across the 27 transitions measured from `e6f5012d` to `60a81db7`, so it is not monotone. And any commit that adds or drops a
`[claim: <id>]` annotation moves `registered` and closure: 6 and 86 = 51 + 35 through `fdda3c53`; 5 and
72 = 50 + 22 from `5fe81128`, whose own `CHANGES.md` rollover carried 14 annotated regions out of the live
window inside the commit that published the older values; 4 and 69 = 49 + 20 from `1507adbf`, which rewrote the
resume pointer and dropped its three annotations. And registering a live-document surface adds its evidence
row, which is what moved `current_surfaces` and `identity_gated` together at `d23e8bae`. Read all of them from
`--report`. The self-test
instantiates every outcome family and challenges all coverage joins. Zero outer incomplete does not certify the
incomplete assertion regions exposed by the narrower mdBook contract — that count moves whenever the manual
does, so read it from `check_book_quantitative_claims.pl --report`, `authority_outcomes.incomplete`.
`[claim: current-claim-census-frozen]`

For manual-wide quantitative review, use `perl scripts/check_book_quantitative_claims.pl --check` to derive the
governed mdBook membership and validate the current inventory/frozen contract, `--report` for bounded totals,
`--produce` for stable path/line/SHA candidates, and `--self-test` for the fence, exact-coverage, authority-join,
source-identity, and portable-bound fault matrix. Inventory output is a review denominator only; semantic
authority is accepted only from exact non-overlapping regions in frozen phase. The current report keeps authored,
example/identity, and dated scope separate from registered authority and honest missing evidence legs. The
`mdbook-quantitative-census-frozen` claim verifies that mapping only; an `incomplete` region remains unverified.

Every count in this section was re-derived from those three `--report` commands at this commit, and a count is
carried only where a control fails if it moves or an authored decision fixes it. That is not a style preference: a
`[claim: <id>]` annotation closes its region on the **presence** of the annotation, and the digest gate proves
only that a governed file has not changed — neither reads the numbers in the sentence. `CLAIM-VERIFICATION-ADOPTION.6`
corrected three drifted counts here and recorded eight as “confirmed unchanged”; `.6a` re-derived the 28
consecutive revisions since, each with that commit's own checker, and found four of those eight were already
false in `.6`'s own commit.
That is same-transaction invalidation, not slow decay, so the default is withdrawal plus a named producer field.
The surviving trajectory rule was itself retired by `.10` and its last two beneficiaries withdrawn by `.11`:
a trajectory shows what has not happened, never what cannot. `.7` owns making the remaining ones re-derive
mechanically.

## How to run the SpecForge CLI

```bash
# from the repo root, repository-local manifest, local providers only:
cargo run --manifest-path Cargo.toml -- <subcommand> [args]
# or the built binary after `cargo build --release`:
./target/release/specforge <subcommand> [args]
```
Generated artifacts live under `generated/<stage>/<document_key>/`. Most diagnostic subcommands are
**read-only** and default to a **CI-safe no-op provider** (`--vlm-provider skip` / `--nlp-provider skip`)
so the live Ollama/LM-Studio VLM/NLP is never a CI dependency.

---

## Quick chooser — symptom → tool

| Symptom / question | Go to |
|---|---|
| "Is the runtime ready (Docling / Ollama / LM Studio)?" | [1.1 `doctor`](#11-doctor---strict) |
| "What's actually inside this artifact?" | [2.1 `inspect`](#21-inspect-path) |
| "WHY is extraction X / what did this document yield / what's MISSING?" | [2.2 `validate`](#22-validate-artifact) |
| "Is this a real spec we under-extracted, or an honest non-target (guide/PHY)?" | [2.2 `validate`](#22-validate-artifact) → `document_class` + `evidence_document_underextracted_spec` |
| "Why won't this IntentIR lower to `.isf` / what blocks it?" | [3.1 `adapt --target isf`](#31-adapt-intent-ir---target-isf---dry-run) → `blocking_reasons` |
| "Is the emitted `.isf` valid for the downstream consumer?" | [3.2 FSMGen `--strict --check`](#32-fsmgen---strict--check--json-the-isf-contract-canary) |
| "Did my change move a KG gold/negative? which fixture?" | [4.1 `kg-bench`](#41-kg-bench) |
| "Is a wire-protocol gold still 1.000?" | [4.2 WIRE-BASED-100 golds](#42-wire-based-100--the-gold-eval) |
| "Is my change byte-identical on untouched docs (orthogonality proof)?" | [4.3 `--dry-run` old-vs-new diff](#43---dry-run--byte-identical-orthogonality-proof) |
| "How trustworthy is the extracted constraint set?" | [5.1 `nli-verify`](#51-nli-verify-evidence-ir) |
| "Does today's code still PRODUCE the records this artifact publishes?" | [5.5 `replay-constraints`](#55-replay-constraints-evidence-ir----evidence-root-root) |
| "How faithfully did docling read the tables?" | [5.4 `grits-consensus`](#54-grits-consensus-witnesses-json) |
| "What fraction of each document's intent reaches `.isf`?" | [6.1 `measure_isf_completeness.py`](#61-scriptsmeasure_isf_completenesspy) |
| "Which documents form an extraction family / share a shape?" | [6.2 `corpus-cluster`](#62-corpus-cluster) |
| "Run the full gate before committing code." | [7.1 `run_ci.sh`](#71-scriptsrun_cish--the-full-gate) |
| "How many corpus documents are still unrefreshed?" | [7.2b `check_corpus_frontier.sh`](#72b-scriptscheck_corpus_frontiersh--the-corpus-frontier-derive-and-diff-gate) |
| "Is the terminal task source/archive boundary intact?" | [7.3 `check_task_tree_archive.pl`](#73-scriptscheck_task_tree_archivepl) |

---

## 1. Runtime preflight

### 1.1 `doctor [--strict]`
- **WHAT:** inspects pipeline readiness — Docling ingest (the selected Python candidate), the default
  Ollama loopback endpoint/model, and the LM Studio fallback loopback endpoint/model.
- **WHEN:** the FIRST thing to run when a `converge` / `ingest` run misbehaves, hangs, or yields nothing;
  before a long live-provider run.
- **HOW:** `cargo run --manifest-path Cargo.toml -- doctor --strict`
- **OUTPUT:** per-subsystem status lines (Python path, endpoint/model reachability). `--strict` makes any
  not-ready subsystem a nonzero exit. Cold model-load latency is allowed (no manual prewarm needed).

---

## 2. Inspect & validate (the primary diagnostic surface)

### 2.1 `inspect <path>`
- **WHAT:** quick structural inspection of any staged artifact (SourceIR / EvidenceIR / SemanticIR /
  IntentIR / adapter).
- **WHEN:** "what is actually in this file" before deeper analysis; sanity-check a stage's shape.
- **HOW:** `cargo run --manifest-path Cargo.toml -- inspect generated/evidence_ir/<key>/evidence_ir.json`

### 2.2 `validate <artifact>`
- **WHAT:** THE primary diagnostic. Computes typed **metrics** + **findings** (Info / Warning) with a
  human **rationale**, and writes a deterministic `validation_report.json` sidecar. For an EvidenceIR it
  reports, among others: `document_class` (+ rationale, `evidence_document_underextracted_spec`),
  per-document completeness gaps (`registers_without_fields`, `signals_without_direction`, …), the
  message-field inventory, signal-presence inventory, the `extraction_quality_gauge`, transaction
  inventory, and stage-staleness. For an IntentIR/SemanticIR it reports relation/connectivity/semantic/
  temporal/polarity surfaces + conflicts + the stage-staleness warning.
- **WHEN:** "why is extraction X", "what did this document yield and what is missing", "is this an honest
  absence or an under-extraction" — the WHY+WHERE tool for the IR.
- **HOW:** `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/<key>/intent_ir.json`
- **OUTPUT:** a printed metric/finding summary + the `rationale:` line for the class call; the persisted
  sidecar is back-annotated into the artifact's `validation_reports`. Cite a specific metric/finding id
  in a task leaf's ROOT-CAUSE box.

---

## 3. ISF lowering (the synthesis target)

### 3.1 `adapt <intent-ir> --target isf [--dry-run]`
- **WHAT:** lowers IntentIR through the typed `IsfIr` model and emits `.isf`; when the canonical surface
  is not renderable it **blocks with explicit `blocking_reasons`** instead of emitting.
- **WHEN:** "does this IntentIR lower to `.isf`, and if not, exactly why"; measuring lowering coverage
  read-only (`--dry-run` writes nothing).
- **HOW:** `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/<key>/intent_ir.json --target isf --dry-run`
- **OUTPUT:** the materialized `generated/adapters/isf/<key>/adapter.json` (+ `.isf`) or the list of
  `blocking_reasons`. A `blocking_reason` is a valid ROOT-CAUSE signature.

### 3.2 FSMGen `--strict --check --json` (the `.isf` contract canary)
- **WHAT:** the downstream consumer (`subs/fsmgen`, a pinned read-only submodule) validates the emitted
  `.isf`. SpecForge's `*_passes_fsmgen_strict_validation` tests are the contract canary.
- **WHEN:** any emitter change — confirm "0 NEW diagnostics" on the 4 wire docs before claiming the
  emitter change is safe (`feedback_verify_fsmgen_before_fr`: update the submodule + read current docs +
  empirically test before claiming an FSMGen/ISF gap or filing a feature request).
- **HOW:** run the emitted `.isf` through the pinned FSMGen `--strict --check --json` (see
  `docs/FSMGEN_FEEDBACK.md` + the submodule's own docs: the stable channel owns the current pin/status
  and routes to FSMGen's authoritative invocation).
- **OUTPUT:** strict diagnostics JSON; "0 new diagnostics vs baseline" is the NO-REGRESSION oracle for an
  emitter change.

---

## 4. Regression oracles (the strongest "not trust-me" leg)

### 4.1 `kg-bench`
- **WHAT:** runs the tracked KG-quality fixtures (gold + negative) under `crates/specforge/test_data/
  kg_quality/` through the staged pipeline, asserting actor ports, graph direction, semantic roles,
  temporal rules, conflicts, polarity, residual quality, and name-noise rejection.
- **WHEN:** after ANY change to extraction/semantic/intent logic — `156/156` is the headline regression
  number; a single moved fixture localizes the regression.
- **HOW:** `cargo run --manifest-path Cargo.toml -- kg-bench`
- **OUTPUT:** `… 156/156` plus, on a failure, the exact fixture + the expected-vs-actual diagnostic. Cite
  `kg-bench 156/156` in the NO-REGRESSION box.

### 4.2 WIRE-BASED-100 — the gold eval
- **WHAT:** the wire-protocol gold (APB/AHB/AXI/SWD) constraint + temporal + relation F1, the hard gate
  for cat-1 fidelity. `= 1.000` is the bar.
- **WHEN:** any change that could touch the wire-protocol surfaces; prove orthogonality when it shouldn't.
- **HOW:** the gold eval over the 4 wire docs (the eval harness used in the KG-ISF trees); the 4 gold docs
  are NOT re-ingested, so a re-ingest data slice is orthogonal by construction.
- **OUTPUT:** per-surface F1; `1.000` held = NO REGRESSION on wire fidelity.

### 4.3 `--dry-run` — byte-identical orthogonality proof
- **WHAT:** every stage has a `--dry-run` that previews output without persisting. A `git stash`
  baseline-vs-change `--dry-run` over the untouched corpus that is **byte-identical** proves a change is
  orthogonal to those docs.
- **WHEN:** proving "all N intact corpus bundles rebuild byte-identical" / "WIRE-BASED-100 provably
  orthogonal (emitter/metadata-only)".
- **HOW:** `cargo run --manifest-path Cargo.toml -- evidence <source-ir> --dry-run` (and the sibling
  stage dry-runs) on the baseline vs the change; `diff` the outputs.
- **OUTPUT:** an empty diff = byte-identical = orthogonal. A valid NO-REGRESSION oracle.

---

## 5. Extraction quality (deeper measurement)

### 5.1 `nli-verify <evidence-ir>`
- **WHAT:** entailment-checks the extracted `signal_constraints` against the document's own sentences and
  persists an `extraction_quality_gauge` (entailed / not-entailed / abstained + the not-entailed ids).
- **WHEN:** "how trustworthy is this document's constraint extraction"; measuring the LLM-primary promotion.
- **HOW:** `cargo run --manifest-path Cargo.toml -- nli-verify generated/evidence_ir/<key>/evidence_ir.json`
- **OUTPUT:** the gauge counts + the exact not-entailed constraint ids (also reported by `validate`).

### 5.2 `eval-extraction <dataset>`
- **WHAT:** scores extraction against a labeled dataset (precision/recall over the labeled items).
- **WHEN:** measuring an extractor change against a held-out labeled set.
- **HOW:** `cargo run --manifest-path Cargo.toml -- eval-extraction <dataset> --evidence-root generated/evidence_ir`

### 5.3 `audit-extraction <source-ir> [--sample <n>] [--seed <s>]`
- **WHAT:** samples extracted items for a reproducible spot audit (seeded).
- **WHEN:** a quick honesty spot-check of a document's extracted surface.
- **HOW:** `cargo run --manifest-path Cargo.toml -- audit-extraction generated/source_ir/<key>/source_ir.json --sample 20 --seed 0`

### 5.5 `replay-constraints <evidence-ir>` / `--evidence-root <root>`
- **WHAT:** re-runs the REAL deterministic constraint producer over a persisted artifact's own
  `extracted_statements` and reports, per published record, whether today's code still mints it — and
  when it does not, which gate stands in the way.
- **WHEN:** **before sizing any extractor change.** A count over `generated/` measures what SpecForge
  PUBLISHED; only 24 of 78 documents keep a normalized bundle, so the rest are frozen at the generation
  that wrote them and can carry records the current code would never produce. `EXTRACTION-QUALITY-GAUGE.3k.1`
  sized itself on four published records and found the current extractor reproduces none of them.
- **HOW:** `cargo run --manifest-path Cargo.toml -- replay-constraints generated/evidence_ir/<key>/evidence_ir.json`
  or `-- replay-constraints --evidence-root generated/evidence_ir` for the corpus totals. Read-only: no
  provider, no write, and it reads the legacy/proofless stratum the canonical loader refuses.
- **OUTPUT:** `reproduced` / `not_reproduced` with the refusing gate per record; `granted_declarations`
  (published subjects the artifact no longer declares, granted one so their records still get a trial);
  a named skip for every artifact that would not load.
- **READ THE VERDICT ASYMMETRICALLY:** "not reproduced" is sound, because the replay runs a widened
  catalog and a wider catalog can only admit more subjects. `unpersisted_replay_records` is not a drift
  measure — the build applies convergence stages this replay does not.

### 5.4 `grits-consensus <witnesses-json>`
- **WHAT:** scores how faithfully Docling read a document's TABLES against a CROSS-TOOL consensus gold
  (docling vs pdfplumber's geometric read vs qwen2.5vl's vision read), never against itself; flags
  split cells for adjudication (`--adjudicate-out <queue>`).
- **WHEN:** a suspected table mis-read (merged words, dropped cells) — the loop caught a real docling
  word-merge (`forAPB5` where the page prints `for APB5`).
- **HOW:** offline `.venv-eval` path (`scripts/grits_cross_tool.py` emits witnesses); see
  `docs/book/src/commands/quality-and-learning.md`.

---

## 6. Cross-document & corpus measurement

### 6.1 `scripts/measure_isf_completeness.py`
- **WHAT:** read-only per-surface ISF-lowering ledger over all persisted docs (registers/fields/message-
  fields/constraints/rules/transactions → lowered/partial/true-gap/honest-residual), plus per-doc intent
  category labels. The reproducer behind `DOC-INTENT-TAXONOMY.2`.
- **WHEN:** "what fraction of each document's intent reaches `.isf`, and what is honest-absence vs a true
  gap"; prioritizing a per-category lever.
- **HOW:** `python3 scripts/measure_isf_completeness.py` (reads `generated/`, writes no canonical state).

### 6.2 `corpus-cluster [--threshold <0.0-1.0>]`
- **WHAT:** clusters documents by their ADR-0006-safe structural fingerprint (count-buckets + fired
  extractor strategies), surfacing emergent families + each family's fired-extractor union — no vendor
  names (the shared structure IS the key).
- **WHEN:** "which documents share an extraction shape / form a family"; finding a family to attack.
- **HOW:** `cargo run --manifest-path Cargo.toml -- corpus-cluster --evidence-root generated/evidence_ir`

### 6.3 `learn-priors <intent-ir>...` / `corpus-kb`
- **WHAT:** `learn-priors` builds the advisory typed `CorpusMemory` prior store; `corpus-kb` refreshes the
  tracked corpus knowledge-base pages from reviewable evidence. Both are advisory, never mutate canonical IR.
- **WHEN:** inspecting / refreshing the cross-document learning plane.

### 6.4 `scripts/check_source_pdf_registry_currentness.pl`
- **WHAT:** read-only exact-membership oracle for the durable source corpus: Git-indexed PDFs below
  `corpus/` ↔ registry rows, code-derived document keys, parent directories, PDF signatures, and pinned
  `SourceIR` derivation seams.
- **WHEN:** adding, removing, or renaming a tracked source PDF; changing filename-to-key code; auditing
  whether a fresh clone has every reproducible source named exactly once.
- **HOW:** `perl scripts/check_source_pdf_registry_currentness.pl --report` (13 fail-closed cases run
  unconditionally through `LIVE-DOC-SIZE`; host-local libraries and `generated/` are not inputs).

### 6.5 `scripts/check_corpus_kb_currentness.pl`

- **WHAT:** read-only dependency/output oracle for the corpus KB's eleven managed Markdown regions and
  paired prior-candidate JSON. It binds the reviewed validation snapshot, all Git-indexed KG fixture
  inputs, exact managed and human-side regions, and the Rust producer seams.
- **WHEN:** changing a KG fixture, validation review boundary, corpus-KB producer, managed page, or
  prior-candidate projection; auditing that human synthesis survived a refresh.
- **HOW:** `perl scripts/check_corpus_kb_currentness.pl --report` (15 fail-closed cases run
  unconditionally through `LIVE-DOC-SIZE`; canonical IR and typed prior memory are forbidden outputs).

### 6.6 `scripts/check_derived_state_contracts.pl`

- **WHAT:** the neutral exact-field authority gate. It reads the bounded
  `doctrine/live_document_size/derived_state_contracts.jsonl` registry, proves that every declared path
  belongs to its current governed surface or an explicit repository-local control role, locates exact literal
  primary/secondary markers, distinguishes derive-on-read, verified copies, authored intent, and immutable
  evidence, and executes every declared copy verifier.
- **WHEN:** adding or changing a current-state version, hash, count, projection, resume field, selected next
  action, or revision-bound measurement; use it before deciding that a convenient copy is trustworthy.
- **HOW:** `perl scripts/check_derived_state_contracts.pl --report`. The unconditional `LIVE-DOC-SIZE`
  path also runs 47 neutral fail-closed cases and 25 project-adapter cases. Only
  `scripts/check_derived_state_authorities.pl` knows the local Cargo/Rust and FSMGen-gitlink comparisons;
  it consumes declared copy roles and contains no secondary path fallback. The neutral checker knows no local
  field IDs or roles and does not infer fields from dates, numbers, hash shapes, or prose.

---

## 7. Stage replay, build, and host

### 7.1 `scripts/run_ci.sh` — the full gate
- **WHAT:** the canonical local/hosted gate: the doctrine driver (memory-arch + knowledge-map +
  task-acceptance), `cargo fmt --all --check`, warning-deny clippy, warning-deny tests, rustdoc, mdBook.
- **WHEN:** before committing any Rust code change (the NO-REGRESSION oracle leg).
- **HOW:** `bash scripts/run_ci.sh`

### 7.2 `scripts/check_doctrines.sh` — the doctrine gate only
- **WHAT:** the registry/driver alone. The default `gate` tier is fast (structural + evidence checks,
  no heavy build) and reports every `ci`-tier doctrine as `DEFER` so none is silently absent; `--all`
  also runs the CI-tier doctrines and is what `run_ci.sh` invokes.
- **WHEN:** before any commit; what the pre-commit hook runs.
- **HOW:** `bash scripts/check_doctrines.sh` / `bash scripts/check_doctrines.sh --all`

### 7.2a `scripts/check_chain_currency.sh` — the CHAIN-CURRENCY oracle (CI-tier)
- **WHAT:** replays every persisted corpus artifact `--dry-run` from its persisted input (evidence,
  semantic, intent, `.isf` adapter, plus each emitted `.isf` against the adapter's rendered
  `source_text`) and fails when the persisted artifact is not what the current binary produces.
  `validation_reports` is excluded exactly as the product's `*_ir_fingerprint` helpers exclude it.
  Its second leg compares `doctrine/chain_currency/retained_bundles.json` with the normalized bundles
  actually on disk: a declared bundle that is gone, or a bundle no leaf declared, fails closed.
- **WHEN:** after any shared-extractor or stage change, and before signing off a refresh — it is the
  measurement ADR 0025 decision 1 requires before attributing a delta. Skips loudly with no corpus.
- **HOW:** `bash scripts/check_chain_currency.sh` (`--self-test` for its sixteen fail-closed cases)

### 7.2a-i `scripts/check_proof_seal_currency.sh` — the PROOF-SEAL-CURRENCY gate (gate-tier)
- **WHAT:** reads the `ruleset_sha256` every persisted artifact records at all five chain stages for the
  proof-carrying stratum — a **total** census — then asks the current build's own canonical loader whether
  it still accepts that seal, with one **representative** probe per *distinct* seal per stage. The probe is
  the CONSUMING stage in `--dry-run`; never `specforge validate`, which is not idempotent and would
  invalidate the chain it claims to read. The terminal `adapters/isf` stage has no consumer, so it is
  censused and reported UNPROBED rather than counted as a pass.
- **WHEN:** automatically, on every commit through the doctrine driver — that is the point. Run it by hand
  after editing a stage root or `derivation.rs` if you want the answer before the hook gives it to you.
  Skips loudly and passes with no corpus.
- **LIMIT:** a current seal is **not** content currency. Whether a persisted artifact is still what the
  current binary reproduces stays `CHAIN-CURRENCY`'s question at CI tier.
- **HOW:** `bash scripts/check_proof_seal_currency.sh` (`--self-test` for its sixteen fail-closed cases).
  On a stale seal it names the remedy: `source_proof_migrate --write` for SourceIR (proof-only), or
  `scripts/rebuild_stage_cascade.sh --write` for every stage below it (a real content rebuild).

### 7.2b `scripts/check_corpus_frontier.sh` — the CORPUS-FRONTIER derive-and-diff gate
- **WHAT:** derives the corpus cohort from each document's persisted SourceIR and diffs it against the explicit,
  disjoint `refreshed`/`remaining` partition in `doctrine/corpus_frontier/census.json`. It checks exact identity,
  lifecycle membership, retained-bundle agreement, whole-cohort omission, and the counts stated by the root task.
  Source locations never classify lifecycle state, so a library move cannot impersonate a current-binary refresh.
  A refresh must move the declaration in the same transaction or this fails closed.
- **WHEN:** to answer "how many documents are left, really" — never read a carried number. Also the fastest
  way to confirm a refresh's bookkeeping landed. Skips loudly with no corpus.
- **HOW:** `bash scripts/check_corpus_frontier.sh`
  (`perl scripts/check_corpus_frontier_census.pl --report` for the JSON census;
  `--self-test` for its ten fail-closed cases)

### 7.2c `scripts/check_production_genericity.sh` — the PRODUCTION-GENERICITY structural gate
- **WHAT:** composes the compiler-visible package-direction check, exact production module/claim inventory,
  exact rule/field/producer/seam/bypass joins, and the compiled graph's fixed-point raw/identity flow plus
  proof-only canonical-authority analysis. All four components run and report even when one fails.
- **WHEN:** before any production extraction, proof, persistence, or package-boundary change; it also runs
  unconditionally through the doctrine driver and pre-commit hook.
- **HOW:** `bash scripts/check_production_genericity.sh` for the fast clean-tree doctrine;
  `bash scripts/check_production_genericity.sh --self-test` for the CI qualification matrix and exact
  inventory-to-runtime alpha-obligation join over all 170 rules.
- **LIMIT:** structural alpha qualification proves declared capability/premise/topology invariants, not the
  population renaming/paraphrase/held-out behavior owned by `.f`.

### 7.3 `scripts/check_task_tree_archive.pl`
- **WHAT:** validates the contract-driven terminal task lifecycle. `source_locked` pins the still-live source and
  rejects premature archive paths; `migrated` verifies the exact capsule, bounded closed root/index, manifest,
  provenance, routes, milestones, and ceilings.
- **WHEN:** before or during a terminal task-tree migration; use `--report` to inspect the selected source and
  metrics, and `--self-test` to exercise all 15 fail-closed cases.
- **HOW:** `perl scripts/check_task_tree_archive.pl --check`

### 7.4 `project-validation <artifact>...` / `rescan-plan [--plan <p>] [--execute]`
- **WHAT:** `project-validation` validates + refreshes the tracked validation snapshot + writes the local
  schema-v2 rescan plan with typed replay hints; `rescan-plan` inspects/executes the whitelisted local
  replay hints (dry-run by default, `--execute` only for repository-local `cargo run … --` hints).
- **WHEN:** projecting validation state into a crash-safe snapshot; running a bounded, gated rescan loop.
  The tracked snapshot is the **last reviewed** projection, not ambient git-ignored artifact state:
  review producer output before committing it, and verify the declared boundary read-only with
  `perl scripts/check_validation_snapshot_currentness.pl --check`.

### 7.5 Build & host (RAM-bounded)
- **WHAT:** builds are RAM-constrained on this host. Monitor with `memory_pressure` (macOS); cap parallel
  jobs and kill at the danger line.
- **HOW:** `CARGO_BUILD_JOBS=2 cargo test --manifest-path Cargo.toml`; watch RAM, kill background work at
  ≥85% used (`feedback_ram_ceiling_monitor`); serialize a heavy Docling ingest vs a loaded VLM model.

### 7.6 `clean [--scope …] [--execute]`
- **WHAT:** first-class local artifact reclamation (dry-run by default): `--scope source-normalized` keeps
  `source_ir.json` but drops the heavy `normalized/` bundle; `--scope document`/`all-generated` for deeper
  sweeps. **Never** delete a `source_ir` before a re-ingest succeeds (`project_docling_mps_cpu`).
- **WHEN:** never as refresh routine. A refresh **keeps** its normalized bundle (ADR 0025 decision 3) —
  that retention is what keeps the document's evidence stage replayable. Reclaiming is a deliberate,
  separately owned decision that must also drop the key from
  `doctrine/chain_currency/retained_bundles.json` and record a `reclamations` row, or `CHAIN-CURRENCY`
  reddens.
- **HOW:** `cargo run --manifest-path Cargo.toml -- clean --scope source-normalized --document-key <key>`

---

## Protocol A — diagnose an extraction miss (signal / register / transaction not captured)
1. `validate <evidence-ir>` → read `document_class` (+ `evidence_document_underextracted_spec`) and the
   completeness-gap metrics: is this an honest non-target (guide/PHY) or a real under-extraction?
2. If under-extracted: `inspect` the SourceIR for the table/section that should carry it; check the
   message-field / signal-presence / register inventories in `validate`.
3. Isolate with a `kg-bench` fixture (or add one) that pins the expected vs actual surface — that fixture
   is the WHY+WHERE and the regression lock.

## Protocol B — diagnose an `.isf` lowering block
1. `adapt <intent-ir> --target isf --dry-run` → read the `blocking_reasons`.
2. Trace the blocking surface back through `validate <intent-ir>` (which canonical surface is missing /
   conflicted) → `validate <semantic-ir>` (stage-staleness? a `semantic_*` conflict?).
3. If the block is an FSMGen ISF-abstraction gap, verify empirically against the current submodule pin
   first (`feedback_verify_fsmgen_before_fr`), then create an owning task-tree leaf and add a bounded
   open record in `docs/FSMGEN_FEEDBACK.md` with `Direction`, `Kind`, `Status`, `Owner`, and `Evidence`.
   Put detailed design evidence in the task/research record and reproducible bugs in an indexed issue
   bundle; never hack the emitter (`feedback_isf_no_hacks`).

## Protocol C — localize a `kg-bench` / gold regression
1. `cargo run -- kg-bench` → the failing fixture name + its expected-vs-actual diagnostic.
2. `cargo test -p specforge <suspect_test>` → the in-file unit test that pins the changed behavior.
3. `git stash` baseline vs change + `--dry-run` on the affected doc → the byte-level diff that shows
   exactly what moved; confirm the wire golds are byte-identical (orthogonal) or re-measure 1.000.

---

*This catalog is the diagnostic surface SpecForge presents to itself. Keep it in lockstep when a
diagnostic command, flag, gold, or oracle is added or changed — the mdBook command chapters mirror it.*
