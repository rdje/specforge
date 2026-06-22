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
- latest_commit (baseline): **`1b4657dc`** `DOC-INTENT-TAXONOMY.1 — corpus census by category (36/7/15/2/4/14); surface counts alone can't recover purpose`. **21 ahead** of `origin/main` (push at ~200 → HOLD). Next commit → 22 ahead. Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (fresh session `2026-06-22`).
- **🆕 OWNER DIRECTIVE this session (`2026-06-22`, multi-message) — now OWNED as tree `DOC-INTENT-TAXONOMY` ([[project_doc_intent_taxonomy]]):** every chip-spec PDF is *about* something — a small set of INTENT CATEGORIES; SpecForge must (1) quickly determine a PDF's category, (2) extract → IntentIR, (3) lower EVERYTHING to ISF (ISF = the way to SYNTHESIZE PDF intent); in fine ALL categories FULLY handled (SOTA, no hurry). Reinforces NORTH STAR [[project_kg_isf_completeness]]. FSMGen is NOW adding a verification-oriented SV/UVM + VHDL lowering path alongside the default synthesizable HDL; new ISF abstractions (memory banks, single/dual-port memory, …) anticipated → feed ISF-abstraction gaps back to FSMGen (verified FR, not emitter hack; [[feedback_verify_fsmgen_before_fr]]).
- **✅ `DOC-INTENT-TAXONOMY.0` DONE + COMMITTED `8815a8c5`, docs-only.** Defined the **6-category purpose taxonomy** (1 wire-protocol MATURE / 2 register-IP PARTIAL / 3 platform-system-IP PARTIAL / 4 CPU-ISA THIN / 5 PHY honest-thin non-target / 6 methodology/guide non-target), captured IDENTICALLY in mdBook `docs/book/src/document-categories.md` (+ `SUMMARY.md`) and `docs/tasks/DOC-INTENT-TAXONOMY.md`; registered in `docs/TASK_TREE.md`. Built ON the coarse structural `document_class` (4-way), not replacing it.
- **✅ `DOC-INTENT-TAXONOMY.1` DONE (this slice), read-only census.** Distribution over 78 docs: **36 wire-protocol / 7 register-IP / 15 platform-system-IP / 2 CPU-ISA / 4 PHY / 14 methodology-guide**. 3 measured structural blind spots (cat 2↔3 inseparable / ISA no signature / cat 5↔6 near-empty) + the register-heavy-protocol trap (8 cat-1 protocols are register/message-dominant) → the `.3` recognizer needs wire-relation shape + front-matter + topology cue, not counts. Report `docs/research/document-intent-category-census.md`; KM `[[document-intent-category-census]]`. No mutation.
- **✅ `CORPUS-COVERAGE.2` re-ingest #21 DONE earlier this session, committed `3ce2b2d2`.** RISC-V IOMMU (108pp): register/arch refresh + honest "already-current" finding (register 33 held / 147 fields, IntentIR unchanged — Jun-15 evidence rebuilt from retained source_ir before only the `normalized/` page-image bundle was reclaimed → already had `.10` families); `.isf` 175 sig / 33 storage / 5 enums / 45 rules, fsmgen --strict 0 diag; surfaced a RISC-V STRUCTURE-table recall opp (Lever D). 19/21 strict-clean.
- next_action (PNT): **`DOC-INTENT-TAXONOMY.2` — per-category ISF-lowering completeness gauge** (the `.1` denominator 36/7/15/2/4/14 is now known): for each category measure honestly what fraction of the document's intent reaches `.isf` (honest-absence vs true gap), per-item demonstrated ([[feedback_scoring_rigor]]), cat 5/6 treated as non-targets; report under `docs/research/` + KM. Then `.3` fast category recognizer (CODE). **Parallel queued thread:** `CORPUS-COVERAGE.2` re-ingest #22 — genuinely pre-`.10`-stale register docs (CoreSight SoC-600 TRM `100806_*` / JEDEC eMMC `jesd84_b50` / AMD-IOMMU `48882_*`); 36 normalized-missing docs remain.
- in_flight_uncommitted: **none** (this MEMORY.md baseline refresh is the only change → committed as a continuity checkpoint). `DOC-INTENT-TAXONOMY.0`+`.1` both committed (`8815a8c5`, `1b4657dc`); `CORPUS-COVERAGE.2` #21 committed (`3ce2b2d2`). All gates green; mdBook builds. Untracked (stays): `.claude/settings.json`. blockers: none. **Repo HANDOFF-READY.**
- **OPERATIONAL REALITY:** host RAM healthy at ~72–78% free (`memory_pressure`), swap 1.0G/2.0G. Builds run `CARGO_BUILD_JOBS≤2`, RAM monitored, kill ≥85% used (`[[feedback_ram_ceiling_monitor]]`). ollama server up but **no model loaded** (idle — serialize vs Docling).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication. Re-ingest = re-run of existing deterministic extractors → WIRE-BASED-100 + register/wire golds + `kg-bench` **156/156** orthogonal by construction (the 4 gold docs are not re-ingested). `scripts/run_ci.sh` green before committing CODE (not needed for a re-ingest data slice). Push ~every 200 (**18 ahead after this commit → HOLD**).
