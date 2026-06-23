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
- latest_commit (baseline): **`f2d48a3e`** `KG-ISF-COMPLETENESS.1c.i — CODE: dense-prose trailing prep/aux strip …`. **48 ahead** of `origin/main` (push at ~200 → HOLD). This `KG-ISF-COMPLETENESS.1c.ii` measurement commit → **49 ahead**. Untracked (stays): `.claude/settings.json`, `.cache/` (gitignored symlink).
- **MODE: PNT loop** (fresh session `2026-06-23`). Bootstrap-read README/AGENTS/MEMORY_ARCHITECTURE/MEMORY/CORPUS-COVERAGE/KG-ISF-COMPLETENESS/TASK_TREE/COMMIT/TOOLBOX; survey-verified ROADMAP (no drift) + mdBook; doctrines GREEN. **PNT-PIVOTED off `CORPUS-COVERAGE.2`** to Lever E and **fully scoped it**: `.1c` probe (`6092942d`) → `.1c.i` CODE gate (`f2d48a3e`) → `.1c.ii` measurement (committing now).
- **✅ `KG-ISF-COMPLETENESS.1c.ii` MEASURED (committing now) — Lever E FULLY SCOPED.** Read-only (no code). The bulk dense-prose phantom class (multi-word, single-`REL-INFERRED`, leading-noun: `basic bus`/`actual sector`; eMMC 109 multi-word actors) has **no clean within-doc structural gate**: a `.1b.ii`-style connectivity fold is disproven unsafe on AXI+ACE `ihi0022_h_c` (`caching Manager`→`Manager` correct but `Manager component`→`component` WRONG — agent token position varies), and a real descriptive ref vs a phantom are structurally indistinguishable (no `ProtocolActorRecord` grounding surface). The genuine fix is UPSTREAM relation-subject extraction precision on prose (`[[project_nlp_shallow_parse_direction]]`/`NLP-SHALLOW-PARSE`) → honest residual (never reaches `.isf`). **Lever E outcome: `.1c.i` clean structural win SHIPPED (eMMC 153→138, WIRE-100 1.000); `.1c.ii` upstream-NLP-gated residual.** Report §8; KM card updated.
- next_action (PNT): Lever E is scoped — **pick a NEW substantive lever.** Strongest candidates: **(a) `NLP-SHALLOW-PARSE`** — the owner-directed in-Rust shallow-parse (SVO/who-does-what) that is the UPSTREAM fix for `.1c.ii` AND broadly improves dense-prose extraction (read `docs/tasks/NLP-SHALLOW-PARSE.md` frontier `.2h` first; high-value, owner-aligned). **(b) Lever F/A** — ISF enum/value-literal emit (`ISF-VALUE-WIDTH-EMIT` family; the HBM2 #28 strict-FAIL — a concrete compile-gated emitter fix; DTI #8 Lever A width-align also open). **(c) continue `CORPUS-COVERAGE.2`** (29 docs remain — but lever-surfacing tail, lower marginal value). **Lever code = high-stakes → a NEW lever is best STARTED on a FRESH session for full sharpness** (this session already delivered 3 Lever-E slices).
- in_flight_uncommitted: the **`KG-ISF-COMPLETENESS.1c.ii`** staged set (docs/tasks/KG-ISF-COMPLETENESS.md, docs/TASK_TREE.md, docs/research/agent-identity-prose-class-measurement.md, docs/knowledge/agent-identity-prose-class-measurement.md, KNOWLEDGE_MAP.md, CHANGES.md, DEVELOPMENT_NOTES.md, LIVE_ACHIEVEMENT_STATUS.md, MEMORY.md — docs-only, no code). blockers: none. **Repo HANDOFF-READY** after commit.
- **OPERATIONAL REALITY:** host RAM monitor before any Docling/14B work — kill ≥85% used ([[feedback_ram_ceiling_monitor]]). This session: 2 read-only probes + 1 Rust build (`CARGO_BUILD_JOBS=2`) + run_ci. NOTE: fsmgen is the Perl `subs/fsmgen/bin/fsmgen` (NOT cargo). The eMMC generated/ adapter was refreshed by the `.1c.i` verify (git-ignored cache — intent_ir still the old persisted 153; harmless). Artifact-cleanup standing (≥24h) — no stray build/log artifacts.
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
