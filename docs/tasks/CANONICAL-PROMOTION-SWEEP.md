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
  Status: `pending` (frontier)
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
  Verification: `pending`
  Commit: `pending`

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
  Status: `pending` (gated on `.1`)
  Goal: **scale to the remaining in-scope corpus**, one doc at a time, each with before/after gauge + RAM
  recorded, host-local-source docs honestly skipped. Roll up a corpus-wide before/after gauge summary +
  refresh `VALIDATION_SNAPSHOT.md` / `LIVE_ACHIEVEMENT_STATUS.md`.
  Acceptance: every in-scope doc promoted-or-honestly-skipped with recorded evidence; corpus gauge summary
  refreshed; no wire regression anywhere; RAM-safe throughout.
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `CANONICAL-PROMOTION-SWEEP.1` | `pending` | Pilot on non-wire docs locks the RAM-safe, review-gated per-doc protocol before any wire-critical mutation. RAM-heavy (qwen2.5:14b) → run in a fresh session with full attention to the watchdog. |
| 2 | `CANONICAL-PROMOTION-SWEEP.2` | `pending` (gated) | Wire docs under the full gold battery on canonical. |
| 3 | `CANONICAL-PROMOTION-SWEEP.3` | `pending` (gated) | Scale to the rest of the in-scope corpus, one doc at a time. |

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
| — | — | — | (pending first promotion) |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `CANONICAL-PROMOTION-SWEEP` (tree) | `CANONICAL-PROMOTION-SWEEP.0 — create tree` | ownership/scoping slice; no canonical mutation yet |

## Changelog

- `2026-06-15`: Created task tree (owner-directed "do all these"). Captured the goal (land the default
  LLM-primary promotion on canonical artifacts), the RAM-safety protocol (per-doc sequential, model serialized,
  ≥85%-used kill), the review-gating (canonical mutation stays `promotion_status`-tracked), and the wire-doc
  strict-battery gate. Frontier = `.1` pilot on non-wire docs to lock the protocol. Ownership/scoping slice —
  no canonical mutation performed; ready for fresh-session execution with the 14B model.
