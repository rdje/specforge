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
- latest_commit (baseline): **`a9c8d415`** `DOCTRINE-ENFORCEMENT-ADOPT.1 — SpecForge-native task-acceptance evidence check + TOOLBOX.md`. **26 ahead** of `origin/main` (push at ~200 → HOLD). Next commit (`DOCTRINE-ENFORCEMENT-ADOPT.2`) → 27 ahead. Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (fresh session `2026-06-22`). Owner doctrine-enforcement directive COMPLETE; resuming the prior `DOC-INTENT-TAXONOMY.3b` frontier next.
- **OWNER DIRECTIVE (`2026-06-22`) COMPLETE: "adopt this doctrine enforcement system" → `DOCTRINE_ENFORCEMENT.md`** (the portable 4th architecture). OWNED + CLOSED as tree **`DOCTRINE-ENFORCEMENT-ADOPT`** (`.0`+`.1`+`.2` all done). Driver `scripts/check_doctrines.sh` registers `MEMORY-ARCH`+`KNOWLEDGE-MAP`+`TASK-ACCEPTANCE` (3/3 PASS), gated at pre-commit (E3) + `run_ci.sh`/CI (E4). `TOOLBOX.md` = SpecForge's OWN debug tools. KM `[[doctrine-enforcement-adoption]]`; decision `0006`; book `reference/doctrine-enforcement.md`.
- **✅ `DOCTRINE-ENFORCEMENT-ADOPT.0` COMMITTED `810b510b`** (framework) · **✅ `.1` COMMITTED `a9c8d415`** (task-acceptance check + `TOOLBOX.md`) · **✅ `.2` DONE (this slice), committing now** (closing leaf, docs-only): user-facing mdBook chapter `docs/book/src/reference/doctrine-enforcement.md` (in `SUMMARY.md`; `mdbook build` green) + KM card (map 112→113). **TREE CLOSED.**
- next_action (PNT): **resume `DOC-INTENT-TAXONOMY.3b`** — implement `classify_document_intent_category` in `crates/specforge/src/ir/completeness.rs` (sibling of `classify_document`, pure-fn + in-file tests) per the committed `.3a` design, wired into `validate.rs` ~2888 (all cues in scope; extend the census with `message_field_records`). **⚠️ This is the FIRST Rust slice gated by the new `TASK-ACCEPTANCE` check → its `docs/tasks/DOC-INTENT-TAXONOMY.md` leaf MUST carry the `## Acceptance Checklist (enforced)` (template in `TOOLBOX.md`): ROOT CAUSE / ADDRESSED / NO REGRESSION ticked + evidence-backed (`kg-bench 156/156`, `WIRE-BASED-100 1.000`, `run_ci`) or the commit is blocked.** Build RAM-constrained (`CARGO_BUILD_JOBS≤2`, kill ≥85% — [[feedback_ram_ceiling_monitor]]); then `.3c` fixtures+CI.
- in_flight_uncommitted: **`DOCTRINE-ENFORCEMENT-ADOPT.2`** staged set (docs/book/src/reference/doctrine-enforcement.md, docs/book/src/SUMMARY.md, docs/knowledge/doctrine-enforcement-adoption.md, KNOWLEDGE_MAP.md, tree + TASK_TREE.md, CHANGES/DEVELOPMENT_NOTES/LIVE_ACHIEVEMENT_STATUS/MEMORY). blockers: none. **Repo HANDOFF-READY.**
- **OPERATIONAL REALITY:** host RAM healthy (`memory_pressure`). Builds run `CARGO_BUILD_JOBS≤2`, RAM monitored, kill ≥85% used ([[feedback_ram_ceiling_monitor]]). ollama up, no model loaded (serialize vs Docling).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication. `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
