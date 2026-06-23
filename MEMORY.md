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
- latest_commit (baseline): **`72585f8d`** `KG-ISF-COMPLETENESS.2a.iv — ISF enum value-literal emit-gate (Lever F) …` (prior: `25f46b05` `DOCTRINE-ENFORCEMENT-ADOPT.3 — fix check_task_acceptance.sh pipefail/SIGPIPE …`). **51 ahead** of `origin/main` (push at ~200 → HOLD); this MEMORY-refresh checkpoint → **52 ahead**. Untracked (stays): `.claude/settings.json`, `.cache/` (gitignored symlink).
- **MODE: PNT loop** (fresh session `2026-06-23`). Bootstrap-read README/AGENTS/MEMORY_ARCHITECTURE/MEMORY/TASK_TREE/COMMIT + KG-ISF-COMPLETENESS/CORPUS-COVERAGE/NLP-SHALLOW-PARSE/KG-ISF-TRANSACTIONS frontiers; agent-surveyed codebase + mdBook (book ~95% in sync — only nav gaps; RUST_CODEBASE_ANALYSIS.md line-counts STALE = doc-refresh candidate); doctrines GREEN. **PNT picked Lever F** (after a probe showed NLP-SHALLOW-PARSE BUILD frontier is measured-EXHAUSTED, not a clean lever).
- **✅ `KG-ISF-COMPLETENESS.2a.iv` DONE (CODE, committed `72585f8d`) — ISF enum value-literal emit-gate (Lever F).** Probe re-verified the strict tally vs the real FSMGen: **DTI (Lever A) ALREADY clean** (`ATST`=`1'd1`; #8 log was STALE → corrected); **HBM2 (Lever F) fixed** — FSMGen rejects a bare `[01]`-only token len≥4 (verified by value sweep: `1000`/`1111` fail, `69152`/`999`/`4'b1000` pass), so the emitter now drops an enum carrying such a value (HBM2's mega-conflated `TABLE` binary-codes-as-decimals) to an `isf_enum_value_literal_*` residual → HBM2 strict **0 diagnostics** (only `TABLE` dropped; `EXTEST_RX`/`DWORD_MISR` kept). The initial "width-overflow" hypothesis was DISPROVEN by an `.isf` diff (measurement-first). **Corpus byte-identical EXCEPT HBM2** (wire golds + GIC-600 `69152`/ARM-Debug `3360` untouched). `run_ci.sh` GREEN (lib 1706, +2); `kg-bench` 156/156; doctrines GREEN. Strict tally now **27/28 renderable clean, 1 FAIL (LPI Lever C)**.
- next_action (PNT): **pick the next lever.** Strongest: **(a) Lever C** — LPI `isf_conflicting_rule_writes` on `PREQ` (the LAST open strict-FAIL → 28/28 clean): a CROSS-SURFACE conflict (`rule_5` PREQ←1 vs `temporal_…_dyn_sigcon_0012` PREQ←0, different guards) the same-guard `ISF-RULE-CONFLICT-RESIDUAL` dedup misses — own a leaf, measure WHY it slips before coding; LPI intent_ir is persisted (RAM-safe, no Docling). **(b) continue `CORPUS-COVERAGE.2`** (29 docs remain; needs Docling — RAM-gated). **(c) Lever D / NLP-SHALLOW-PARSE upstream** (dense-prose recall — bigger, owner-aligned, but build-exhausted tree → high-stakes re-open). Also pending: a `RUST_CODEBASE_ANALYSIS.md` refresh (line-counts/module-inventory stale per the codebase survey) + the mdBook nav gaps (`nli-verify`/`doctor`/`audit-extraction`/`eval-extraction` SUMMARY).
- in_flight_uncommitted: the **`KG-ISF-COMPLETENESS.2a.iv`** staged set (crates/specforge/src/ir/isf_ir.rs + adapters.rs [CODE]; docs/tasks/KG-ISF-COMPLETENESS.md + CORPUS-COVERAGE.md; docs/knowledge/isf-enum-value-literal-emit-gate.md + KNOWLEDGE_MAP.md; README/CHANGES/DEVELOPMENT_NOTES/LIVE_ACHIEVEMENT_STATUS/MEMORY; docs/book/src/pipeline/isf-adapter.md). blockers: none. **Repo HANDOFF-READY** after commit.
- **OPERATIONAL REALITY:** host RAM monitor before any Docling/14B work — kill ≥85% used ([[feedback_ram_ceiling_monitor]]); this session was RAM-light (read-only probes + `CARGO_BUILD_JOBS=2` builds + run_ci; RAM steady ~79% free). fsmgen is the Perl `subs/fsmgen/bin/fsmgen` (NOT cargo). The HBM2/DTI/LPI generated/ adapters were re-emitted by the probe (git-ignored cache; intent_ir unchanged). Artifact-cleanup standing (≥24h) — no stray build/log artifacts.
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
