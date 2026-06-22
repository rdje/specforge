# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey.
> History lives in `git log` + `CHANGES.md`; work state lives in the task-trees
> (`docs/tasks/`); durable facts/decisions live in `docs/decisions/`. Do **not** append
> session narration here — overwrite the "Current state" block instead.
> (The prior ever-growing MEMORY.md is preserved in git history before commit `6d668eb2`.)

## How to resume (any AI, any harness)
- Read `MEMORY_ARCHITECTURE.md` (the memory system) and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`);
  follow `COMMIT.md` after every slice (unit id in the commit subject).
- Non-negotiable doctrine: see `docs/decisions/0003-task-tree-and-commit-doctrine.md`
  (no code change without an owning task-tree first; signoff quality; zero
  ROADMAP↔code↔mdBook drift; push ~every 200 commits; artifact cleanup ≥ every 24h).
- Durable cross-cutting facts: `docs/decisions/` (e.g. Docling CPU device 0001,
  LLM/VLM provider default 0002); retrieval index: `KNOWLEDGE_MAP.md` (grep before re-deriving).

## Current state (OVERWRITE this block each update — do not append)
- latest_commit (baseline): **`0fa00469`** `CORPUS-COVERAGE.2 — re-ingest #20: GIC-400 TRM (ddi0471, 57pp) healthy register/TRM-phase refresh`. **18 ahead** of `origin/main` (push at ~200 → HOLD). This `CORPUS-COVERAGE.2` re-ingest #21 slice is the next commit (→ 19 ahead). Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (fresh session `2026-06-22`).
- **🆕 OWNER DIRECTIVE this session (`2026-06-22`, 2 messages — ADDRESS + likely a new tree):** (1) chip-spec PDFs fall into INTENT CATEGORIES — *protocol spec* (APB/AXI/CHI/…), *platform/system-IP* (CoreSight/GIC/…), *CPU ISA*, register/TRM, etc.; SpecForge should understand "what is this PDF about". (2) SpecForge must understand ALL these variations → extract → IntentIR → **lower EVERYTHING (constraints, relations, …) to ISF — ISF is the way to SYNTHESIZE the PDF intent.** This reinforces the NORTH STAR ([[project_kg_isf_completeness]]). Note: a `document_class` surface already exists (`protocol`/`register`/`interface`/`guide`, PDF-VARIANT-DIGESTION.5a) — owner wants a richer/explicit taxonomy + completeness of ISF lowering across ALL categories. Scope a new tree before any code (doctrine: no code w/o task-tree).
- **✅ `CORPUS-COVERAGE.2` re-ingest #21 DONE (this slice), committed.** RISC-V IOMMU arch (108pp): register/arch refresh + honest **"already-current" finding** — register 33 held / 147 fields, constraints 6→8, IntentIR unchanged (the Jun-15 stale evidence was rebuilt from the retained `source_ir.json` before only the `normalized/` page-image bundle was disk-reclaimed → already had the `.10` families); re-ingest value = bundle-restoration + current-binary confirm, NOT a marquee `.10c`/`.10g` jump. `.isf` `agent.isf` 175 sig / 33 storage / 5 enums / 45 rules, **fsmgen --strict success / 0 diag**. Honest absences (0 rel/iface; 0 msg-fields — IOMMU device-context/command-queue STRUCTURE tables miss the `.10b`/`.10d`/`.10e` two-column families → Lever-D recall opp; 0 txns). 19/21 strict-clean (2 FAIL = DTI Lever A, LPI Lever C).
- next_action (PNT): **first ADDRESS the owner's 2 messages** (PDF-intent taxonomy + ISF-synthesis-completeness — articulate the categories, map to `document_class`, propose a scoped tree). Then continue **`CORPUS-COVERAGE.2` re-ingest #22 — genuinely pre-`.10`-stale register/structure docs** (bigger gains than #21): **CoreSight SoC-600 TRM** (`100806_*`, `.10c` lifted TMC 2→30), **JEDEC eMMC** (`jesd84_b50`/`JESD84-B50`, EXT_CSD register-heavy), or **AMD-IOMMU** (`48882_*`). Method: rebuild release if any `.rs` newer than binary, `DOCLING_DEVICE=cpu`, `.4a` RAM guard, Ollama idle, monitor RAM; cascade + `validate` + fsmgen `--strict`, record deltas in the `.2` log, commit per `COMMIT.md`. **36 normalized-missing docs remain.**
- in_flight_uncommitted: **the #21 live-doc updates are staged for THIS commit** (`docs/tasks/CORPUS-COVERAGE.md` + `CHANGES.md` + `LIVE_ACHIEVEMENT_STATUS.md` + `MEMORY.md`; `generated/` is git-ignored — durable trace is the `.2` table). Untracked (stays): `.claude/settings.json`. blockers: none. Repo handoff-ready after commit.
- **OPERATIONAL REALITY:** host RAM healthy at ~72–78% free (`memory_pressure`), swap 1.0G/2.0G. Builds run `CARGO_BUILD_JOBS≤2`, RAM monitored, kill ≥85% used (`[[feedback_ram_ceiling_monitor]]`). ollama server up but **no model loaded** (idle — serialize vs Docling).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication. Re-ingest = re-run of existing deterministic extractors → WIRE-BASED-100 + register/wire golds + `kg-bench` **156/156** orthogonal by construction (the 4 gold docs are not re-ingested). `scripts/run_ci.sh` green before committing CODE (not needed for a re-ingest data slice). Push ~every 200 (**18 ahead after this commit → HOLD**).
