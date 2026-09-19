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

The **catalog is partitioned**. The landing keeps the standing directive, the enforcement and
acceptance-checklist contract, the quick chooser, the first-reach tools (§1-§4), and the diagnosis
protocols; the deeper sections that actually grow live under [`docs/toolbox/`](docs/toolbox/) and are
routed from §5-§7 below. Search every entry at once with `rg -i 'term' TOOLBOX.md docs/toolbox`.

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
| "The gate is stuck with no output — is it hung, or is the host?" | [7 gates/build/host](docs/toolbox/gates-build-and-host.md) → §7.2-ii `probe_exec_assessment_latency.sh` |
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
| "Does today's reader still DECLARE the signals this artifact declares?" | [5.6 `replay-declarations`](#56-replay-declarations-evidence-ir----evidence-root-root) |
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

**Routed to [`docs/toolbox/extraction-quality.md`](docs/toolbox/extraction-quality.md)** — §5.1
`nli-verify`, §5.2 `eval-extraction`, §5.3 `audit-extraction`, §5.4 `grits-consensus`, §5.5
`replay-constraints`, §5.6 `replay-declarations`.

---

## 6. Cross-document & corpus measurement

**Routed to [`docs/toolbox/corpus-measurement.md`](docs/toolbox/corpus-measurement.md)** — §6.1
`measure_isf_completeness.py`, §6.2 `corpus-cluster`, §6.3 `learn-priors` / `corpus-kb`, §6.4
`check_source_pdf_registry_currentness.pl`, §6.5 `check_corpus_kb_currentness.pl`, §6.6
`check_derived_state_contracts.pl`, §6.7 `measure_actor_taxonomy_blast_radius.py`, §6.8
`measure_declaration_row_notations.py`.

---

## 7. Stage replay, build, and host

**Routed to [`docs/toolbox/gates-build-and-host.md`](docs/toolbox/gates-build-and-host.md)** — §7.1
`run_ci.sh`, §7.2 `check_doctrines.sh`, §7.2a `check_chain_currency.sh`, §7.2a-i
`check_proof_seal_currency.sh`, §7.2-ii `probe_exec_assessment_latency.sh`, §7.2b
`check_corpus_frontier.sh`, §7.2c
`check_production_genericity.sh`, §7.3 `check_task_tree_archive.pl`, §7.4 `project-validation` /
`rescan-plan`, §7.5 build & host, §7.6 `clean`, §7.7 `repin_claim_regions.py`.

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
