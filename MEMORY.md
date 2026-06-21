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
- latest_commit (baseline): **`4c8c8e85`** `KG-ISF-COMPLETENESS.2a.ii — post-commit resume-pointer refresh + frontier triage`. **An ownership/provisioning commit `CORPUS-COVERAGE.2` is being created now.** **193 ahead** of `origin/main` (→ 194 after; push at ~200). Tree clean except untracked `.claude/settings.json`.
- **MODE: PNT loop** (session `2026-06-21`; open-ended). **OWNER STEER `2026-06-21` (AskUserQuestion): re-provided the host-local spec library to attack substantive gap #2 (corpus coverage)** — the frontier raised on `2026-06-18` is now UNBLOCKED. Library = `chipdoc` (88 PDFs, owner says permanent), reached via a **git-ignored symlink `.cache/local-references/chipdoc`** (owner-chosen `2026-06-21` over copying ~150 MB into tracked `corpus/`); `/.cache/` is git-ignored so the absolute library path is never tracked (`[[feedback_source_pdfs_in_repo]]`). The 22 gold/measured docs stay copied in `corpus/`.
- **ACTIVE: `CORPUS-COVERAGE.2`** — RAM-guarded per-doc re-ingest of the **57 normalized-missing docs** with the CURRENT binary to refresh their STALE EvidenceIR (built before the `.10a`–`.10g` register/message-field + `.12a/.12b` presence + `.2a`–`.2m` transaction families) and cascade evidence→semantic→intent→adapt(.isf). All 57 already reached IntentIR via `.0`; this is a FIDELITY refresh (surface more typed intent), not a coverage-count fix.
- next_action: **re-ingest doc #1 = AMBA CXS (`ihi0079`, 0.39 MB)** from `.cache/local-references/chipdoc/arm/amba/specialized/cxs/current/` via `DOCLING_DEVICE=cpu ./target/release/specforge ingest <symlink path>`; then evidence→semantic→intent→`adapt --target isf`; record before/after typed-surface deltas in `CORPUS-COVERAGE.md`; commit per `COMMIT.md`. PNT protocol-specs-first (CXS→GFB→ACC→ATP→TileLink×2→LPI→DTI→CHI-C2C), then register/TRM/ISA docs. ONE doc per slice; commit each.
- **RAM-SAFETY (owner, non-negotiable):** Docling is RAM-heavy — use `DOCLING_DEVICE=cpu` (`[[project_docling_mps_cpu]]`); the built-in `.4a` guard aborts CLEAN at ≥85% used; keep Ollama idle (`ollama stop`); monitor swap + free% between docs; autonomously kill at ≥85% used. Baseline now: free 73%, swap 898 MB used, ollama idle. `[[feedback_ram_ceiling_monitor]]`. Push ~every 200 commits (**193 ahead → HOLD**; → 194 after this slice).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication. Re-ingest is NOT a code change — it re-runs the EXISTING deterministic extractors, so no new ADR-0006 surface. WIRE-BASED-100 + register/wire golds + `kg-bench` **156/156** stay green by construction (the 4 wire gold docs are NOT re-ingested → orthogonal). `scripts/run_ci.sh` green before any CODE slice. **STALE-PERSISTED GOTCHA:** the 57 docs' `generated/` artifacts were built by an older binary — the whole point of `.2` is to refresh them.
- in_flight_uncommitted: the `CORPUS-COVERAGE.2` ownership/provisioning slice (`.gitignore` +`/.cache/`, `docs/tasks/CORPUS-COVERAGE.md` `.2` leaf, this `MEMORY.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `CHANGES.md`, `README.md` local-path) being committed now; the `.cache/local-references/chipdoc` symlink is created (git-ignored). Untracked (stays): `.claude/settings.json`. blockers: **none — frontier UNBLOCKED by owner re-provisioning.** **Repo HANDOFF-READY.**
