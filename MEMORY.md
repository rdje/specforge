# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey.
> History lives in `git log` + `CHANGES.md`; work state lives in the task-trees
> (`docs/tasks/`); durable facts/decisions live in `docs/decisions/`. Do **not** append
> session narration here — overwrite the "Current state" block instead.
> (The prior ever-growing MEMORY.md is preserved in git history before commit `6d668eb2`.)

## How to resume (any AI, any harness)
- Read `MEMORY_ARCHITECTURE.md` (memory system), `DOCTRINE_ENFORCEMENT.md` (the 4th
  architecture — doctrines are mechanically gated), and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`);
  follow `COMMIT.md` after every slice (unit id in the commit subject).
- Non-negotiable doctrine: see `docs/decisions/0003-task-tree-and-commit-doctrine.md`
  (no code change without an owning task-tree first; signoff quality; zero
  ROADMAP↔code↔mdBook drift; push ~every 200 commits; artifact cleanup ≥ every 24h).
- Before committing run `scripts/check_doctrines.sh` (the registry/driver: memory-arch +
  knowledge-map + task-acceptance); hooks + CI run it too. Retrieval: `KNOWLEDGE_MAP.md`.

## Current state (OVERWRITE this block each update — do not append)
- latest_commit (baseline): **`9e7f595c`** `KG-ISF-COMPLETENESS.2a.iv — MEMORY post-commit refresh …` (code baseline `72585f8d` Lever F). **52 ahead** of `origin/main` (push at ~200 → HOLD); the pending `.2a.v` commit → **53 ahead**. Untracked (stays): `.claude/settings.json`, `.cache/` (gitignored symlink).
- **MODE: PNT loop** (fresh session `2026-06-23`). Bootstrap-read README/AGENTS/MEMORY_ARCHITECTURE/MEMORY/TASK_TREE/COMMIT/SESSION_BOOTSTRAP + KG-ISF-COMPLETENESS/CORPUS-COVERAGE frontiers; doctrines GREEN. **PNT picked Lever C** (the RAM-safe ISF-emit frontier of the first active tree; `.1c.ii` deferred-NLP, `.3` corpus-refresh Docling-gated).
- **✅ `KG-ISF-COMPLETENESS.2a.v` DONE (CODE, this slice — Lever C, ISF unconditional-rule-overlap conflict residual).** FSMGen `isf_conflicting_rule_writes` rejects an UNCONDITIONAL rule (empty guard ⇒ overlaps every guard, never proven disjoint) vs a guarded rule driving the same target to a different value; the same-guard `dedup_conflicting_rules` (keyed `(signal,guard)`) MISSED it. New post-pass `drop_unconditional_overlap_conflicts` (`isf_ir.rs`, after same-guard dedup, before priority emit) keeps the unconditional value + residualizes the conflicting rule (`isf_unconditional_overlap_*`); the `(priority …)` hatch rejected as ungrounded precedence. **MEASUREMENT correction:** the "27/28 clean, 1 FAIL (LPI)" tally was STALE — a fresh re-emit + FSMGen sweep showed the AXI/AHB/AXI-Stream wire golds + LTI + NVMe ALSO failing on it. Fix: **6 docs FAIL→PASS**, **0 PASS→FAIL**, **100/107 `.isf` byte-identical**, post-fix **97/107 PASS**. AXI+ACE `ihi0022_h_c` stays FAIL on the orthogonal `(port expr)` Non-Goal. `run_ci.sh` GREEN (lib **1708**, +2); `kg-bench` 156/156; doctrines GREEN; WIRE-BASED-100 orthogonal by construction (emitter-only). KM `[[isf-unconditional-rule-overlap-conflict]]`.
- next_action (PNT): **pick the next lever.** The ISF-emit strict-FAIL levers A/B/C/F are now resolved; remaining 10 FAIL `.isf` are ORTHOGONAL (junk secondary-actor names → a precision lever; AXI+ACE `(port expr)` grammar = spun-out `ISF-VALUE-WIDTH-EMIT` Non-Goal). Candidates: **(a)** junk-actor-name precision so secondary `.isf` stop failing (KG bar #1, RAM-safe, persisted IR); **(b)** the `(port expr)` grammar lever (AXI+ACE); **(c)** `CORPUS-COVERAGE.2` corpus refresh (Docling — RAM-gated); **(d)** `KG-ISF-COMPLETENESS.3` relation-completeness corpus refresh. Also pending: `RUST_CODEBASE_ANALYSIS.md` line-count refresh + mdBook SUMMARY nav gaps.
- in_flight_uncommitted: the **`KG-ISF-COMPLETENESS.2a.v`** staged set (crates/specforge/src/ir/isf_ir.rs [CODE +2 tests]; docs/tasks/KG-ISF-COMPLETENESS.md + CORPUS-COVERAGE.md; docs/knowledge/isf-unconditional-rule-overlap-conflict.md + KNOWLEDGE_MAP.md; README/CHANGES/DEVELOPMENT_NOTES/LIVE_ACHIEVEMENT_STATUS/MEMORY; docs/book/src/pipeline/isf-adapter.md). blockers: none. **Repo HANDOFF-READY** after commit.
- **OPERATIONAL REALITY:** host RAM monitor before any Docling/14B work — kill ≥85% used ([[feedback_ram_ceiling_monitor]]); this session RAM-light (read-only probes + `CARGO_BUILD_JOBS=2` builds + run_ci; RAM steady ~82% free). fsmgen is the Perl `subs/fsmgen/bin/fsmgen` (NOT cargo). All generated/ adapters were re-emitted by the `.2a.v` verification sweep (git-ignored cache; intent_ir unchanged). Artifact-cleanup standing (≥24h) — no stray build/log artifacts.
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
