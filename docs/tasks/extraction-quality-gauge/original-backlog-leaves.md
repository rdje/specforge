# EXTRACTION-QUALITY-GAUGE — original backlog leaves

- Part ID: `original-backlog-leaves`
- State: `legacy`

<!-- extraction-quality-gauge-task-source-region:task-tree-and-original-leaves:start -->
## Task Tree

- ID: `EXTRACTION-QUALITY-GAUGE` · Status: `active` · Children: `.0`–`.4` (incl. `.3a`–`.3k`, and
  `.3k.1`–`.3k.6` with `.3k.2a`–`.3k.2j`)
- ID: `EXTRACTION-QUALITY-GAUGE.gauge` · Status: `done` · Goal: establish the NLI-oracle not-entailed
  rate as a per-doc extraction-quality gauge; measure CHI (~83%) + APB (~29%), hand-validate (18/18).
- ID: `EXTRACTION-QUALITY-GAUGE.1` · Status: `done` (prototype) · Goal: **entity discrimination** —
  derived typed classifier + LLM judgment + Rust grounding + enforcement. Built (`ir/entity_typing.rs`
  + `entity-type` cmd, tested); measured on CHI: 27/66 subjects filtered correctly (TXSACTIVE
  recovered), drops 30% of constraints (84% were wrong), gauge 17%→18%. Architecture validated;
  follow-up = wire the gate into the real extractor path + improve fine sub-typing.
- ID: `EXTRACTION-QUALITY-GAUGE.2` · Status: `done` (prototype) · Goal: conditional/temporal
  constraints first-class. `ir/condition_extract.rs` + `extract-conditions` cmd (LLM-judged,
  source-grounded), tested; CHI 22/25 captured, gauge 17%→22%. Complementary to `.1`.
- ID: `EXTRACTION-QUALITY-GAUGE.5` · Status: `done` · Goal: **LLM-primary grounded constraint
  EXTRACTOR** composing `.1`+`.2`. Built (`ir/constraint_extract_llm.rs` + `extract-constraints-llm`,
  tested) + measured: CHI 162→44 constraints, **precision 17%→64% (~4×)** but recall TRADED (covers
  41% of Pattern-good + 18 net-new). Thesis supported on precision; net-better needs a CHI gold.
- ID: `EXTRACTION-QUALITY-GAUGE.6` · Status: `done` (CORRECTED) · Goal: settle replace-vs-patch
  recall. **Domain correction (owner + CHI B2.4):** CHI is a packet/flit protocol, NOT a wire bus —
  `DBID`/`TxnID`/`ReturnNID` are *transaction-identifier FIELDS* (contents of flits), **not signals**.
  So the LLM-primary extractor was **correct** to exclude them from *signal* constraints; my earlier
  "genuine miss" verdict was wrong (the Pattern extractor had mis-typed fields *as* signals). The CHI
  "recall loss" was largely **correct field-exclusion**, plus a smaller real gap.
- ID: `EXTRACTION-QUALITY-GAUGE.7` · Status: `done` (clean test) · Goal: settle recall on a clean
  WIRE-BASED spec (no signal/field confound). Ran `.5` on **APB** vs its gold: **recall 4/6 = 67%** —
  the 2 misses (`PBUSER`/`PNSE` `must_be_value VALID`) **are** signals. So there IS a real, **localized
  `must_be_value` recall gap** even on clean signals — separable from CHI's field issue. Two distinct
  problems, both characterized. Next = `.8`.
- ID: `EXTRACTION-QUALITY-GAUGE.8` · Status: `done` (`2026-06-10`) · Goal: close the `must_be_value`
  recall gap in the LLM-primary extractor; re-measure on APB + AXI/AHB. **Root cause (probed live
  BEFORE coding, qwen2.5:14b-instruct temp 0 on the exact persisted sentences):** (1) the extraction
  prompt's kind vocabulary cannot express a *validity* requirement — for both gold sentences
  ("PNSE/PBUSER must be valid when …") the model outputs `[]`, so the whole requirement vanishes
  before grounding ever runs; (2) two silent-drop paths compound it: `parse_kind` rejects the model's
  natural `must_be_valid` spelling, and rejects `must_be_value` with no echoed value (`value?`). The
  gold convention (= the Pattern extractor's own output) is `must_be_value` + `VALID`. **Fix
  (generic, no chip names, no lists):** (a) the prompt states the typed convention ("a validity
  requirement — <signal> must be valid — is kind must_be_value with value VALID"; probed pre-code:
  both misses recover, asserted-control + negative-control unchanged); (b) Rust backstop —
  `parse_kind` accepts the `must_be_valid`/`valid` spellings, and `ground_constraint` recovers a
  value the model named but did not echo from the SOURCE sentence by reusing
  `extract_protocol_state_value` (the Pattern extractor's own binder grammar, now `pub(crate)` —
  grounded, never fabricated; unrecoverable → honest drop); (c) +4 pure tests (injected, no
  provider; lib 1503). **Measured (pre-fix vs post-fix, same redirected-copy protocol, eval
  canonical keys, document-level gold-fact recall):** APB **4/6 → 6/6** (pre-fix independently
  reproduced the `.7` number; both misses = `PNSE`/`PBUSER` `must_be_value VALID`), AHB **2/6 →
  6/6** (all four pre-fix misses were `HAUSER`/`HWUSER`/`HRUSER`/`HBUSER` `must_be_value VALID`;
  labeled-statement FPs *dropped* 2→1 — the pre-fix condition-junk `HREADY must_be_high` +
  `HRESP must_not_change` read out of the HRUSER sentence is gone), AXI (zero `must_be_value` gold —
  the no-regression control) **4/4 → 4/4** doc-level and **3/4 → 4/4** strict (the prompt change
  un-suppressed the conditional `ASKSTOP must be LOW when ACTIVATEACK is LOW` statement). **Total:
  10/16 → 16/16 gold constraint facts; every one of the six pre-fix misses was a `must_be_value
  VALID` fact and every one is recovered.** Honest FP ledger (all in the open `.3`
  condition/permission class, net 3→3): APB `PSELx must_be_high` unchanged pre/post; AHB 2→1; AXI
  0→1 (`ACTIVATEACK must_be_value LOW` — the model reads the *when*-clause subject as a second
  obligation on the newly-extracted statement). LLM-primary volumes: APB 12→21, AHB 11→16, AXI
  54→59 (vs Pattern 14/15/102).
<!-- extraction-quality-gauge-task-source-region:task-tree-and-original-leaves:end -->

<!-- extraction-quality-gauge-task-source-region:dedup-and-standing-gauge:start -->
- ID: `EXTRACTION-QUALITY-GAUGE.4` · Status: `done` (`2026-06-10`) · Goal: constraint dedup by
  (subject, kind, condition) in the LLM-primary extractor. Shipped: pure `dedup_constraints` —
  canonical key = the eval's `signal_constraint_record_key` (subject + kind incl. value + negation)
  plus the normalized condition; first record kept (stable ids/order), duplicates'
  `supporting_statement_ids` merged in so provenance is preserved, never lost; lookup-only map (no
  hash-iteration order reaches output — `EVIDENCE-DETERMINISM`); called once in
  `extract-constraints-llm` after grounding, which now prints `grounded → deduped (N merged)`.
  +2 pure tests (lib 1519). **Measured live (3 docs): AXI 54→50 — `AWIDUNQ must_be_asserted (if
  present)` re-extracted from 3 statements collapsed to ONE record carrying all 3 statement ids,
  same for `WTAGUPDATE must_be_deasserted`; APB 20→20 and AHB 12→12 this run (this run's two AHB
  `HRESP` records carry DIFFERENT conditions — "To start the ERROR response" vs "In the next
  cycle…" — genuinely different facts, correctly NOT merged; the no-condition ×2 shape from the
  earlier run is locked by unit test instead). Eval stays P=R=F1=1.000 on all three (scoring uses
  key sets — the win is canonical-artifact cleanliness + merged provenance).**
- ID: `EXTRACTION-QUALITY-GAUGE.0` · Status: `done` (`2026-06-10`; PNT pick — `.3c`'s
  relational-vs-value class has no measurable target on persisted artifacts: it was observed on CHI,
  whose fresh re-measure needs the host-local PDF re-provided, and the wire docs' FP ledger is
  currently clean at P=1.000 ×3) · Goal: wire the gauge into converge/CI as the STANDING per-doc
  quality report. **Shipped exactly per design (below) + verified live.** Verification
  (`2026-06-10`): +5 pure tests (2 `ir/nli_verify` builder/staleness + 3 `validate` reporting; lib
  1532→1537) + kg fixture `extraction_quality_gauge_persisted_gold` (kg-bench 153→**154/154**;
  locks metrics, Info finding, exactly-half-is-NOT-majority boundary, warnings excluded).
  **Live wire docs (qwen2.5:14b-instruct):** APB 4/14 not-entailed (28.6% — reproduces the
  tree's recorded ~29%; Info only; persisted ids `sigcon_0007/0008/0010`+`dyn_sigcon_0015` all
  verified present in the artifact, metrics back-annotated `28.6`), AHB 9/15 (60.0% — Warning
  fires), degraded persisted CHI 9/13 (69.2% — Warning), AXI 91/100 labeled + 2 abstained
  (91.0% — Warning). KEY HONEST FINDING the standing gauge makes visible: the CANONICAL
  artifacts still carry the PATTERN surface (the `.3a`/`.3b`/`.4` cleaned LLM-primary surfaces
  live only on /tmp redirected measurement copies — promoting them is the natural follow-up
  lever). **Live converge end-to-end (I2S, vlm skip + nlp ollama, DOCLING_DEVICE=cpu):** 2
  passes stable → gauge measured post-stability, persisted (`nlp3_sigcon_0001`), printed at the
  stable branch AND in the convergence summary; per-item read: `SCK must_be_asserted` extracted
  from an edge-synchronization *permission* sentence = genuine mis-extraction, correctly
  flagged. `run_ci.sh` GREEN. Book: `quality/validation.md` standing-gauge section,
  `pipeline/evidenceir.md` `.0` subsection, `commands/pipeline.md` converge subsection,
  `commands/quality-and-learning.md` validate list, `architecture-rationale.md` NLI section
  extension. KM [[extraction-quality-gauge-standing]]. Design:
  - **Persist the measurement** — today `nli-verify` is print-only, so the gauge dies with the
    terminal. New additive `EvidenceIr.extraction_quality_gauge: Option<ExtractionQualityGaugeRecord>`
    (`#[serde(default, skip_serializing_if = Option::is_none)]` — old artifacts load; absent
    serializes to nothing): `model`, `constraints_total` (signal-constraint surface size at
    measurement), `entailed`, `not_entailed`, `abstained` (Unknown verdicts — honest no-label),
    `not_entailed_constraint_ids` (review routing). Derived fractions are computed, never stored.
    Pure builder `gauge_from_conformal_pass` in `ir/nli_verify.rs` reuses the ONE existing NLI pass
    (`nli_conformal_pass`) — no second sweep of LLM calls. Encounter-order ids only
    (`EVIDENCE-DETERMINISM`: no hash-iteration order reaches output).
  - **`nli-verify` persists it** (same back-annotation semantics as `validate`:
    `ir.write_to_disk()` honors the recorded `artifact_layout`, so the redirected-copy measurement
    protocol keeps working). `--vlm-provider skip` stays a strict no-op.
  - **`converge` measures it after stability** — at the stable branch (after the rescan-plan step,
    so the gauge describes the FINAL artifact), when `--nlp-provider` is not `skip`, via a shared
    `measure_and_persist_gauge` helper (one implementation for both commands); summary prints the
    per-doc gauge. Evidence rebuilds drop the field to `None` by construction
    (`carry_forward_existing_knowledge` never carries it) — a rebuilt surface honestly requires a
    fresh measurement, and converge provides exactly that.
  - **`validate` reports it (the CI-safe surface — no provider needed, reads the persisted record):**
    metrics `extraction_quality_labeled` / `extraction_quality_not_entailed` /
    `extraction_quality_abstained` / `extraction_quality_not_entailed_pct` (`n/a` when never
    measured — the `recall_estimate_pct` precedent); Info finding `evidence_extraction_quality_gauge`
    (related_ids = the not-entailed constraint ids); Warning
    `evidence_extraction_quality_majority_not_entailed` when not_entailed > labeled/2 (scale-free
    "more wrong than right" line — the CHI-class shape, no magic corpus-tuned threshold); Warning
    `evidence_extraction_quality_gauge_stale` when the constraint surface changed since measurement
    (count mismatch OR a recorded not-entailed id no longer present — catches the
    `extract-constraints-llm` replace case whose ids are re-keyed).
  - **Tracked lock**: kg-bench `EvidenceIrPatch` gains an optional gauge patch + a fixture locking
    the validate metrics/finding; pure unit tests cover builder counts, staleness, majority warning,
    and honest absence.
  Acceptance: live gauge persisted + validated on the persisted wire docs (APB/AHB/AXI) and on a
  CHI-class doc (the Warning shape); full `run_ci.sh` GREEN; book (nli-verify, validate, converge,
  EvidenceIR pages) + README + KM card.

<!-- extraction-quality-gauge-task-source-region:dedup-and-standing-gauge:end -->
