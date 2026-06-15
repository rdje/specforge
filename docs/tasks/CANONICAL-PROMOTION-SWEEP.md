# CANONICAL-PROMOTION-SWEEP: land the default LLM-primary constraint promotion across the corpus's canonical artifacts

## Metadata

- Tree ID: `CANONICAL-PROMOTION-SWEEP`
- Status: `active`
- Roadmap lane: `R15e`/`R16` (extraction quality) — successor forward work to `LLM-PRIMARY-PROMOTION` (CLOSED)
- Created: `2026-06-15`
- Last updated: `2026-06-15`
- Owner: repo-local workflow (owner-directed `2026-06-15` — "do all these")

## Goal

Apply the now-DEFAULT LLM-primary constraint promotion (`LLM-PRIMARY-PROMOTION.5` flip, `converge` default for
live-NLP runs) across the corpus's **canonical** artifacts, so the measured gauge improvement that the `.4`
packet proved on REDIRECTED `/tmp` copies (gauge improved on 14/15 measurable docs) actually lands on the
tracked canonical EvidenceIR/SemanticIR/IntentIR surfaces — converting a proven capability into realized
per-document quality. Every promoted document is a **canonical IR mutation**, so this tree is the
review-gated, RAM-safe, per-document execution of that sweep, NOT a new promotion mechanism.

## Non-Goals

- NOT changing the promotion mechanism — `LLM-PRIMARY-PROMOTION` owns `promote_constraints` / the
  `should_promote_constraints` gate / polarity refinement / dedup. This tree only RUNS it on canonical docs.
- NOT regressing the wire-based 100% bar — APB/AHB/AXI/SWD canonical artifacts must re-gate `1.000` on every
  scored surface after any promotion that touches them (the `WIRE-BASED-100` non-negotiable). Wire docs are
  promoted ONLY with their full gold battery re-verified green (the `.3`/`.5` battery), else reverted.
- NOT an unbounded 14B run — RAM-safety is non-negotiable (`[[feedback_ram_ceiling_monitor]]`): one heavy job
  at a time, `ollama stop` / model serialized against any Docling ingest, autonomous kill at ≥85% used, never
  approach the 90→93% reboot danger zone. A sweep that cannot stay under the ceiling PAUSES, it does not push.
- NOT a silent auto-promotion of canonical truth — multi-doc canonical mutation stays **review-gated**
  (`R7-VALIDATION` / `promotion_status` doctrine: `not_promoted_review_required` until current-document
  evidence review + explicit approval). Each promoted doc records its before/after gauge + gate evidence.
- NOT re-ingesting docs whose source PDFs are host-local — the sweep operates on docs whose normalized bundles
  (or a re-ingestable git-tracked source) are present; host-local-source docs stay honestly out of scope until
  re-provided (`[[feedback_source_pdfs_in_repo]]`).

## Acceptance Criteria

- Each in-scope document's canonical constraint surface is promoted via the default `converge` path (live
  `--nlp-provider`), the swap manifest-recorded as `constraints.llm_primary`, the standing quality gauge
  re-measured on the promoted surface, and the before/after gauge + finding recorded per document.
- Wire docs (APB/AHB/AXI/SWD) promote ONLY with the full gold battery re-verified `1.000` (constraints +
  relations + temporal + SWD-derivation) on the CANONICAL artifact; any caught regression → revert that doc +
  re-verify (the `.3` AXI precedent: a real defect was caught and the doc reverted).
- RAM never crosses the ≥85%-used kill ceiling during any promotion run; model serialized vs Docling; recorded.
- `kg-bench` stays green; provider-free converge stays byte-identical by construction (no promotion fires).
- Per-document review/approval status tracked honestly (`promotion_status`); no canonical mutation is presented
  as auto-approved.
- Live docs + book (if a user-facing claim changes) updated; each completed promotion batch committed per
  `COMMIT.md` with this tree's leaf id.

## Task Tree

- ID: `CANONICAL-PROMOTION-SWEEP`
  Status: `active`
  Goal: land the default LLM-primary constraint promotion on canonical corpus artifacts, RAM-safe + review-gated
  Children: `.1`, `.2`, `.3`

- ID: `CANONICAL-PROMOTION-SWEEP.1`
  Status: `done` (`2026-06-15` — pilot on HBM2; protocol locked)
  Goal: **PILOT + protocol lock (one or a few NON-wire docs).** Pick a small set of already-measured non-wire
  docs (e.g. the CHI/NVMe/CCIX class whose gauge error-classes are documented) with intact normalized bundles,
  run the default-promotion `converge` (live `--nlp-provider`, model serialized — `ollama stop` before any
  ingest, RAM watchdog active), re-measure the gauge on the promoted canonical surface, and record the
  before/after + the exact RAM-safety procedure that kept the host under the ceiling. This locks the repeatable
  per-doc protocol the rest of the sweep follows AND proves the canonical mutation is an improvement-or-neutral
  before touching anything wire-critical.
  Acceptance: ≥1 non-wire doc promoted on canonical with a re-measured gauge that improves-or-is-neutral and an
  honest finding; RAM stayed < ceiling (recorded); `kg-bench` green; per-doc `promotion_status` recorded; the
  repeatable protocol written into Decisions.
  Outcome: **MET.** Pilot doc = `jesd235a_2015_11_hbm2_dram` (HBM2, non-wire). Because the frontier mandates
  **no re-ingest** and `converge` always re-ingests (`SourceIr::build + materialize`), the pilot ran the
  **no-re-ingest equivalent** of converge's post-stability promotion path directly on the intact canonical
  EvidenceIR (protocol in Decisions). Gauge: **BEFORE 85.7% not-entailed (2E/12N/0A, 14 records) → AFTER
  40.0% (12E/8N/0A, 20 records)** — records grew 14→20, a decisive improvement that **reproduces the `.4`
  REDIRECTED-copy datum EXACTLY on the canonical artifact** ("HBM2 grows 14→20 AND cleans 85.7%→40%"; oracle
  reproducibility). RAM stayed **≥42% free** across all three 14B steps (min 42, never near the 15%-free /
  85%-used kill line, never the 90→93% reboot zone); model freed with `ollama stop` after. `kg-bench` **156/156**.
  `promotion_status` = `not_promoted_review_required` (canonical mutation is local generated state — `generated/`
  is git-ignored — recorded with before/after + gate evidence; pre-promote backups retained for revert).
  Verification: `nli-verify` BEFORE/AFTER on the fresh release binary + `kg-bench 156/156`; RAM sampled every 3s
  throughout (see Verification Log).
  Commit: `CANONICAL-PROMOTION-SWEEP.1 — non-wire HBM2 pilot: canonical promotion 85.7%→40.0%, RAM-safe, protocol locked`

- ID: `CANONICAL-PROMOTION-SWEEP.2`
  Status: `pending` (gated on `.1`)
  Goal: **wire-doc canonical promotion under the full gold battery.** Promote APB/AHB/AXI/SWD canonical
  artifacts ONLY with the complete `WIRE-BASED-100` / `LLM-PRIMARY-PROMOTION.5` gold battery re-verified
  `1.000` on the promoted CANONICAL artifact (constraints + relations + temporal + SWD-derivation), reverting
  any doc that regresses (the `.3` AXI precedent). Wire docs are the highest-risk + highest-value; they get the
  strictest gate.
  Acceptance: each wire doc either promoted with the full battery `1.000` on canonical, or reverted with the
  caught regression recorded; provider-free byte-stability unaffected; RAM-safe; recorded.
  Verification: `pending`
  Commit: `pending`

- ID: `CANONICAL-PROMOTION-SWEEP.3`
  Status: `in_progress` (`2026-06-15`, dedicated session — owner authorized "run both, .3 then .2")
  Goal: **scale to the remaining in-scope corpus**, one doc at a time, each with before/after gauge + RAM
  recorded, host-local-source docs honestly skipped. Roll up a corpus-wide before/after gauge summary +
  refresh `VALIDATION_SNAPSHOT.md` / `LIVE_ACHIEVEMENT_STATUS.md`.
  Acceptance: every in-scope doc promoted-or-honestly-skipped with recorded evidence; corpus gauge summary
  refreshed; no wire regression anywhere; RAM-safe throughout.
  Scope (inventoried `2026-06-15`): 78 evidence bundles present → **31 carry a Pattern `signal_constraints`
  surface**; minus 4 already-promoted (AXI `ihi0022_l`, APB5 `ihi0024_e`, AHB `ihi0033_c` from
  `LLM-PRIMARY-PROMOTION.5`; HBM2 from `.1`) minus the wire-gold SWD `ihi0074_a` (→ `.2`) = **26 non-wire
  docs in `.3` scope**. The other 47 bundles carry 0 constraints (nothing to promote — honestly skipped).
  Promotion runs the locked no-re-ingest protocol per doc; **keep/revert rule** (below) decides each.
  Batch A (13 smallest, `2026-06-15`): **12 kept / 1 reverted** — see Verification Log.
  Verification: batch A done (12 kept, 1 reverted `soc600_0701`, kg-bench 156/156, RAM min 43% free);
  batches B/C `pending`.
  Commit: `CANONICAL-PROMOTION-SWEEP.3 — batch A (13 smallest non-wire): 12 promoted on canonical, 1 reverted, RAM-safe`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `CANONICAL-PROMOTION-SWEEP.1` | `done` (`2026-06-15`, dedicated session) | **DONE.** Non-wire HBM2 pilot promoted on canonical (85.7%→40.0% not-entailed, 14→20 records), RAM ≥42% free throughout, `kg-bench` 156/156, repeatable no-re-ingest protocol locked into Decisions. Reproduced the `.4` /tmp datum exactly on canonical. |
| 2 | `CANONICAL-PROMOTION-SWEEP.2` | `pending` (**frontier** — unblocked by `.1`) | Wire docs (APB/AHB/AXI/SWD) under the full `WIRE-BASED-100` gold battery on canonical; revert any doc that regresses (the `.3` AXI precedent). Highest-risk + highest-value → strictest gate. RAM-heavy → dedicated-session discipline (same protocol as `.1`, plus the full battery re-verified `1.000` on the promoted canonical artifact). |
| 2b | `CANONICAL-PROMOTION-SWEEP.3` | `in_progress` (**frontier** — batch A done `2026-06-15`) | Scale to the 26 non-wire docs with constraints, one at a time, before/after gauge + RAM recorded; keep/revert rule per doc. **Batch A (13 smallest) DONE: 12 kept, 1 reverted, kg-bench 156/156, RAM min 43%.** Remaining: batch B (9 medium, 11–20 cons) + batch C (4 big — LPI 30, LTI 41, AXI+ACE 97, DTI 114). |

## Decisions

- `2026-06-15`: **Created as a NEW tree (not re-opening the closed `LLM-PRIMARY-PROMOTION`).** That tree
  delivered + closed the FLIP (promotion is the `converge` default); applying it across canonical artifacts is
  distinct forward work with its own RAM-safety + review-gating obligations, so it earns its own ownership per
  the no-code-change-without-a-tree doctrine. The mechanism is reused as-is; this tree is the execution plane.
- `2026-06-15`: **RAM-safety is the binding operational constraint** (`[[feedback_ram_ceiling_monitor]]`,
  `[[project_big_pdf_memory_bounded_ingest]]`). The sweep needs the 14B model; the owner's hard rule is one
  heavy job at a time, model serialized vs Docling (`ollama stop` before ingests), autonomous kill at ≥85%
  used, never the 90→93% reboot zone. So the sweep runs **per-document, sequentially**, with the RAM watchdog
  active — never a parallel corpus blast. A doc that cannot promote under the ceiling is deferred, not forced.
- `2026-06-15`: **Canonical mutation stays review-gated** (`R7-VALIDATION` `promotion_status` doctrine). Each
  promoted doc records before/after gauge + gate evidence + `not_promoted_review_required`-style status; the
  sweep makes the mutation + records the evidence, it does not declare canonical truth auto-approved.
- `2026-06-15`: **Owner explicitly DEFERRED `.1` to a dedicated session.** After the same-session PNT loop drove
  the two cheap/safe forward levers to measured-STANDING (`NLP-SHALLOW-PARSE` build-exhausted;
  `CORPUS-PATTERN-REUSE` consume built-deferred), `.1` was the only remaining buildable lever. Asked the owner
  whether to launch the RAM-heavy live-14B pilot now (RAM had headroom — 83% free, no model loaded) or defer;
  the owner chose **defer to a dedicated session**. Rationale: this lever loads the 14B live (the host-safety
  non-negotiable) and mutates canonical IR under a review gate, so it warrants a session that can give the
  watchdog continuous attention rather than the tail of a context-heavy multi-slice turn. No state change to the
  tree's scope — `.1` stays the frontier, now flagged owner-deferred. PNT pauses here with the repo handoff-ready.
- `2026-06-15`: **`.1` executed in the dedicated session the owner confirmed.** Asked "is this the dedicated
  session?" at session start; the owner said **start the pilot**. Ran it with the full RAM watchdog.
- `2026-06-15`: **The pilot runs the NO-RE-INGEST equivalent of converge's promotion, NOT `converge` itself.**
  `converge <source>` always re-ingests (`run_convergence` calls `SourceIr::build + materialize` →
  Docling, RAM-heavy), which the frontier explicitly forbids ("intact evidence bundle, no re-ingest"). The
  promotion mechanism (`extract_constraints_llm::promote_constraints`) loads the EvidenceIR **from disk** and
  rewrites it in place, so the standalone `extract-constraints-llm` command achieves the identical canonical
  mutation with zero ingest. converge's post-stability sequence (stabilize → promote → downstream rebuild →
  gauge) is reproduced manually by the steps below. (KM card `canonical-promotion-no-reingest-protocol`.)
- `2026-06-15`: **THE REPEATABLE PER-DOC PROTOCOL (locked by `.1`; `.2`/`.3` follow it):**
  1. **Pre-flight:** `ollama stop <model>` (free the 14B); confirm **≥40% RAM free** (`memory_pressure`); ensure
     no Docling/ingest is running (serialize the 14B vs Docling); `cargo build --release` for a fresh
     `target/release/specforge` — **`cargo test` does NOT rebuild the bin**, and live measurement needs the
     fresh binary.
  2. **Backup** the canonical artifacts (`evidence_ir.json` + `semantic_ir.json` + `intent_ir.json` +
     `adapter.json` → `*.prepromote.bak`) — revert capability for the review gate.
  3. **BEFORE gauge:** `specforge nli-verify <evidence_ir>` (defaults `ollama` + `qwen2.5:14b-instruct`) —
     re-measure on the fresh binary (oracle baseline; should reproduce any persisted gauge).
  4. **PROMOTE (canonical mutation, no re-ingest):** `specforge extract-constraints-llm <evidence_ir>` — loads
     the existing `evidence_ir.json`, replaces `signal_constraints` with the LLM-primary grounded surface
     (polarity-refined via `apply_persisted_polarity_to_constraints`, deduped), records manifest
     `constraints.llm_primary`, drops the stale gauge, writes back.
  5. **Rebuild downstream (deterministic, no model):** `specforge semantic <evidence_ir>` → `intent
     <semantic_ir>` → `adapt <intent_ir> --target isf`.
  6. **AFTER gauge:** `specforge nli-verify <evidence_ir>` — re-measure on the promoted surface.
  7. **Gates:** `specforge kg-bench` green; provider-free byte-stability holds by construction (the standalone
     path touches only this doc, and `generated/` is git-ignored — no tracked drift).
  8. **RAM watchdog:** sample `memory_pressure` free% every ~3s throughout the 14B steps; **abort at ≤15% free
     (≥85% used)**; `ollama stop` to free the model when done.
  9. **Record** before/after gauge + RAM trace + `promotion_status` (`not_promoted_review_required` — canonical
     mutation stays review-gated; backups retained for revert). For **wire docs (`.2`) ONLY**: also re-verify
     the full `WIRE-BASED-100` gold battery `1.000` on the promoted canonical artifact, else revert that doc.
- `2026-06-15`: **`promotion_status` = `not_promoted_review_required` for HBM2.** The mutation is realized in
  local generated state (improvement recorded), but the canonical surface is NOT declared owner-approved; the
  `.prepromote.bak` backups stand by for revert pending review (the `R7-VALIDATION` doctrine).
- `2026-06-15`: **Owner authorized the full sweep — "run both, .3 then .2"** (fresh dedicated session, 82% RAM
  free, no model loaded; the recorded gate that paused after `.1` was the question, and the owner chose to run
  the whole sweep, non-wire scale-out first then wire docs). PNT resumes here under the locked protocol.
- `2026-06-15`: **WRITE-PATH FINDING (durable; KM `canonical-promotion-output-path-artifact-layout`):** stage
  commands persist by the artifact's recorded `artifact_layout` (canonical `generated/<stage>/<key>/...`),
  **not** by the input path. So passing the canonical evidence path rewrites it in place (correct promotion),
  but running any command on a `*.prepromote.bak`/copy CLOBBERS canonical with the copy's content. The `.3`
  driver therefore always passes canonical paths, captures BEFORE in-flow before the promote overwrites it,
  and reverts by `cp`-ing a backup back (a plain file copy), never by *running a command* on the backup.
- `2026-06-15`: **LOCKED KEEP/REVERT RULE for the scale-out (quality-grounded, generalizes the `.1`
  improves-or-neutral acceptance).** Per doc, compare the NLI gauge BEFORE (Pattern surface) vs AFTER (promoted
  surface): **REVERT** the doc iff `entailed_after < entailed_before` (promotion lost a verified-correct
  constraint) **OR** (both surfaces are gauge-measured AND the not-entailed *fraction* worsened). **KEEP**
  otherwise — i.e. keep improvements, gauge-neutral results, and the precision-play case where an all-not-
  entailed Pattern surface (e.g. CHI 0E/5N, eMMC 0E/6N) correctly collapses to 0 groundable constraints
  (removed only un-entailed claims, lost no verified one). KEEP is also what the default-`converge` pipeline
  would produce, so canonical stays consistent with the default flip. Revert = `cp` all 4 stage `*.bak` back;
  if a reverted doc's downstream stages were freshly created from the promoted evidence (no prior stage to
  back up), rebuild them deterministically (`semantic`→`intent`→`adapt`) from the reverted Pattern evidence.
- `2026-06-15`: **The RAM watchdog is integrated into the per-doc driver** (`/tmp/sweep.sh`, untracked
  operational tooling — the watchdog the protocol mandates, not product code): a background sampler reads
  `memory_pressure` free% every 3s, records the min, and on `≤15% free` writes a STOP flag + `pkill`s the
  specforge child + `ollama stop`s the model, aborting the whole sweep. Each per-command step also carries a
  `gtimeout 1800` hang-guard. Sequential, one heavy job at a time; model `ollama stop`-freed at batch end.

## Open Questions

- Scope of `.3`: how many of the ~corpus docs have intact normalized bundles vs need a (RAM-heavy) re-ingest
  first? (Resolve at `.1`/`.3` by inventorying `generated/evidence_ir/*` against re-ingestable git-tracked
  sources; host-local-source docs are out of scope until re-provided.) Does not block `.1`.

## Blockers

- None to start `.1`. (Operationally gated by RAM headroom at run time — a `.1`/`.2`/`.3` run pauses if the
  host cannot stay under the ceiling, per the RAM-safety Decision.)

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-15` | `.1` | HBM2 BEFORE `nli-verify` (fresh release bin) | 12/14 not-entailed = **85.7%** (2E/12N/0A) — reproduces persisted baseline + `.4` datum (oracle) |
| `2026-06-15` | `.1` | HBM2 `extract-constraints-llm` (promote, no re-ingest) | 14 (Pattern) → 20 (LLM-primary grounded) → 20 (0 merged); 11 distinct sentences; manifest `constraints.llm_primary` (produced 20); gauge dropped |
| `2026-06-15` | `.1` | HBM2 downstream rebuild (`semantic`→`intent`→`adapt`) | rc=0 all stages (deterministic, no model) |
| `2026-06-15` | `.1` | HBM2 AFTER `nli-verify` (promoted surface) | 8/20 not-entailed = **40.0%** (12E/8N/0A) — reproduces `.4` "85.7%→40%" EXACTLY on canonical |
| `2026-06-15` | `.1` | RAM watchdog (3s sampling, all three 14B steps) | min **42% free** (BEFORE 43, PROMOTE 42, AFTER 42); never ≤15% free; model `ollama stop`-freed → 43% |
| `2026-06-15` | `.1` | `specforge kg-bench` | **156/156** passed, 0 failed |
| `2026-06-15` | `.3` batch A | 13 smallest non-wire docs, full no-re-ingest protocol each | **12 kept / 1 reverted**; aggregate over the 12 kept: BEFORE **5E/44N = 89.8% not-entailed** → AFTER **26E/11N = 29.7%** |
| `2026-06-15` | `.3` batch A | per-doc highlights | mmu_700 4E/3N→9E/2N (7→11 recs); opencapi_3_0 0E/4N→5E/2N (4→7); opencapi_3_1 0E/8N→5E/2N (8→7); gic_600 1E/8N→2E/1N (9→3); intel_vtd 0E/4N→1E/0N (4→1); CXS 0E/2N→1E/2N; usb4_cm 0E/1N→1E/1N; soc600_0100 0E/1N→2E/1N; CHI 0E/5N→0 (precision-collapse), eMMC 0E/6N→0, i2s/soc600_0200 1→0 (all-NE Pattern removed, no verified lost — kept) |
| `2026-06-15` | `.3` batch A | REVERT | `100806_0701_17` coresight_soc_600: 1E/1N → 0E/2N (lost its one entailed constraint) → reverted all stages to Pattern (recs2, 1E/1N), downstream rebuilt deterministically |
| `2026-06-15` | `.3` batch A | `specforge kg-bench` (global gate) | **156/156** passed, 0 failed |
| `2026-06-15` | `.3` batch A | RAM watchdog (3s sampling, model 9.7 GB @ 100% GPU) | **min 43% free** throughout; never ≤15% free; `ollama stop`-freed at batch end |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `CANONICAL-PROMOTION-SWEEP` (tree) | `CANONICAL-PROMOTION-SWEEP.0 — create tree` | ownership/scoping slice; no canonical mutation yet |
| `.1` | `CANONICAL-PROMOTION-SWEEP.1 — non-wire HBM2 pilot: canonical promotion 85.7%→40.0%, RAM-safe, protocol locked` | docs-only commit (the canonical mutation is in git-ignored `generated/`); HBM2 promoted on canonical; protocol locked in Decisions |
| `.3` (batch A) | `CANONICAL-PROMOTION-SWEEP.3 — batch A (13 smallest non-wire): 12 promoted on canonical, 1 reverted, RAM-safe` | docs-only (canonical mutation in git-ignored `generated/`); 12 docs promoted + kept (89.8%→29.7% NE aggregate), `soc600_0701` reverted; KM card `canonical-promotion-output-path-artifact-layout` added |

## Changelog

- `2026-06-15`: Created task tree (owner-directed "do all these"). Captured the goal (land the default
  LLM-primary promotion on canonical artifacts), the RAM-safety protocol (per-doc sequential, model serialized,
  ≥85%-used kill), the review-gating (canonical mutation stays `promotion_status`-tracked), and the wire-doc
  strict-battery gate. Frontier = `.1` pilot on non-wire docs to lock the protocol. Ownership/scoping slice —
  no canonical mutation performed; ready for fresh-session execution with the 14B model.
- `2026-06-15`: **Owner explicitly DEFERRED `.1` to a dedicated session** (see Decisions). After the same-session
  PNT loop closed `CORPUS-PATTERN-REUSE.3b.3a`/`.3b.3a2` (consume side measured-STANDING), `.1` became the only
  remaining buildable lever; the owner chose to defer the RAM-heavy live-14B pilot rather than launch it inside a
  context-heavy turn. Flagged owner-deferred in the frontier; PNT pauses with the repo handoff-ready. No scope
  change, no canonical mutation. Docs-only continuity update.
- `2026-06-15`: **`.1` DONE in the confirmed dedicated session.** Owner answered "start the pilot" at session
  start. Pilot doc = HBM2 (`jesd235a_2015_11_hbm2_dram`, non-wire, 14 constraints, documented `.4` improver).
  Found that `converge` always re-ingests, so ran the **no-re-ingest equivalent** of its promotion path directly
  on the intact canonical EvidenceIR (BEFORE `nli-verify` → `extract-constraints-llm` promote → downstream
  rebuild → AFTER `nli-verify`). Result: **85.7% → 40.0% not-entailed, 14 → 20 records — reproduces the `.4`
  REDIRECTED-copy datum EXACTLY on canonical** (oracle reproducibility). RAM **≥42% free** throughout (watchdog,
  3s sampling); `kg-bench` 156/156; `promotion_status` = `not_promoted_review_required` (mutation in git-ignored
  `generated/`; backups retained). Repeatable per-doc protocol locked into Decisions; frontier advances to `.2`
  (wire docs, full gold battery) / `.3` (non-wire scale-out). KM card `canonical-promotion-no-reingest-protocol`.
  Docs-only commit (no tracked code/IR change). No README/book change (not a closing leaf; no user-facing command
  or capability change — promotion is already documented by `LLM-PRIMARY-PROMOTION.5`).
- `2026-06-15`: **`.3` started — owner authorized "run both, .3 then .2".** Inventoried scope (78 evidence
  bundles → 31 with constraints → 26 non-wire docs in `.3` after excluding 4 already-promoted + wire-gold SWD).
  Locked the per-doc keep/revert rule (revert iff entailed dropped or NE-fraction worsened; else keep) and the
  durable WRITE-PATH finding (commands persist by `artifact_layout`, not the input path → never run a command
  on a backup; KM card added). Built the integrated-RAM-watchdog per-doc driver. **Batch A (13 smallest
  non-wire docs) DONE:** 12 promoted on canonical and kept (aggregate over the 12: **89.8% → 29.7%
  not-entailed**, entailed 5→26), `100806_0701_17` coresight_soc_600 reverted (lost its one entailed
  constraint) and its downstream deterministically rebuilt to a consistent Pattern state. `kg-bench` 156/156;
  RAM min **43% free** throughout (watchdog, model 9.7 GB @ GPU, `ollama stop`-freed). All canonical mutations
  in git-ignored `generated/` with `*.prepromote.bak` retained; `promotion_status = not_promoted_review_required`.
  Docs-only commit. Frontier → batch B (9 medium) + batch C (4 big).
