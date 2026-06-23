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
- latest_commit (baseline): **`a8d2274f`** `DOC-INTENT-TAXONOMY.4d.i — cat-4 RISC-V CSR bit-recovery MEASURED non-viable …`. **39 ahead** of `origin/main` (push at ~200 → HOLD). This `CORPUS-COVERAGE.2 #22` commit → **40 ahead**. Untracked (stays): `.claude/settings.json`, `.cache/` (gitignored symlink).
- **MODE: PNT loop** (fresh session `2026-06-23`). Active tree: **`CORPUS-COVERAGE.2`** (host-local re-ingest of the 57 normalized-missing docs; owner re-provisioned `chipdoc` via the git-ignored `.cache/local-references/chipdoc` symlink). This session bootstrap-read README/AGENTS/MEMORY-ARCH/MEMORY + ROADMAP + mapped codebase + mdBook, surveyed the active trees (NLP-SHALLOW-PARSE / MEMORY-BOUNDED-INGEST = exhausted; CORPUS-PATTERN-REUSE = owner-gated; KG-ISF-COMPLETENESS/TRANSACTIONS = FSMGen-parked), and PNT-picked the active owner-provisioned re-ingest frontier.
- **🚧 `CORPUS-COVERAGE.2` re-ingest #22 DONE (committing now) — Cortex-A76 TRM** (`100798_0401`, 620pp). Docling CPU (620 page artifacts / 476 visual / 0 residuals; RAM steady 73–83% free, `.4a` guard armed, Ollama idle) → deterministic cascade. **Honest finding: already current-binary-equivalent (the #21 class)** — evidence byte-identical (register_records 42 / conditional_rules 26 held; 0 msg/relations/interfaces/txns; intent 9 actors / 707 behaviors / 393 constraints), so value = **`normalized/` bundle RESTORED + current-binary confirmation**. `.isf` renderable (`agent.isf`, 189 signals / 42 storage / 13 enums); **`fsmgen --strict --check --json` success / 0 diagnostics** (strict-clean); `validate` no stage-staleness (0-vs-0), 24/100 INCOMPLETE (honest CPU-core TRM). Honest residuals: 469 bit-fields + 17 field-resets not lowered (UNLOCATED). **Coverage 22 of 57. 20/22 strict-clean** (2 FAIL = DTI Lever A width-align, LPI Lever C rule-conflict).
- **PHASE CORRECTION (#22 survey):** the named register-heavy "pre-`.10`-stale" candidates are ALSO already current-binary-equivalent (CoreSight SoC-600 ×3 = 597/631/833 regs, AMD-IOMMU 217 msg-fields, VT-d 103 regs) → **the marquee-jump phase is over**; the remaining ~35 slices are the #21/#22 confirmation+restoration class (still genuine coverage — restores `normalized/` so a doc isn't one disk-reclaim from losing its evidence).
- next_action (PNT): **continue `CORPUS-COVERAGE.2`** — next re-ingest slice by descending `.isf` richness: CoreSight SoC-600 0701 (`100806_0701`, 833 regs), Intel VT-d (`5_0_…`, 103 regs), JEDEC eMMC/HBM (`JESD84-B50`/`JESD235*`), then the thin OpenCAPI/PHY/guide tail. Check RAM first; one doc per slice; commit per COMMIT.md. **Alternatively** the most substantive non-gated CODE lever is **Lever D** (non-AMBA `Signal|Direction|Width` table-shape + Wishbone `*_O`/`*_I` signal-recall, surfaced #18/#19) — but it needs probe-first design + its own owned `PDF-VARIANT-DIGESTION` leaf (kin to the parked `.9.10`; mind the owner no-denylist steer). `KG-ISF-COMPLETENESS`/`KG-ISF-TRANSACTIONS` FSMGen-parked.
- in_flight_uncommitted: the **`CORPUS-COVERAGE.2 #22`** staged set (docs/tasks/CORPUS-COVERAGE.md, CHANGES.md, DEVELOPMENT_NOTES.md, LIVE_ACHIEVEMENT_STATUS.md, MEMORY.md). blockers: none. **Repo HANDOFF-READY** after commit. (generated/ is git-ignored — the `.2` log table is the durable trace.)
- **OPERATIONAL REALITY:** host RAM monitor before any Docling/14B work — kill ≥85% used ([[feedback_ram_ceiling_monitor]]). This session ran ONE CPU Docling re-ingest (RAM stayed 73–83% free, no abort); no Rust build (binary current at HEAD). Artifact-cleanup directive standing (≥24h) — re-ingest restored one `normalized/` bundle (intended); no stray build artifacts.
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
