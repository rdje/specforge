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
- latest_commit (baseline): **`bb633394`** `KG-ISF-COMPLETENESS.2a.vi — MEMORY post-commit refresh (resume pointer -> HEAD c9d82fd3)` (prior code: `c9d82fd3` `.2a.vi` 70/70 strict-clean). **57 ahead** of `origin/main` (push at ~200 → HOLD); this `.3`-closure commit → **58 ahead**. Untracked (stays): `.claude/settings.json`, `.cache/` (gitignored symlink).
- **MODE: PNT loop** (fresh session started `2026-06-23`, now `2026-06-24`). Bootstrap-read README/AGENTS/MEMORY_ARCHITECTURE/MEMORY/TASK_TREE/COMMIT/SESSION_BOOTSTRAP; surveyed ROADMAP + Rust codebase + mdBook + active frontiers (4 parallel agents); doctrines GREEN. PNT this session: **`KG-ISF-COMPLETENESS.3` CLOSED** (first eligible leaf of the first active tree).
- **✅ `KG-ISF-COMPLETENESS.3` DONE (`2026-06-24`, verification-only — no code change) — north-star bar #2 relation-completeness RESOLVED.** Full re-census over all 78 persisted `intent_ir.json` vs `evidence_ir.json` → **0 stale docs** (zero with `evidence>0 & intent==0`): the `CORPUS-COVERAGE.2` re-ingest sweep rebuilt the once-stale docs via `converge` (whole-chain cascade), so recovered relations landed canonically (tilelink 33/34, I2C 17, gic_600 101, mmu_700 25, ATS 9, DTI 1, opencapi-TL 15, USB4 13 — each `intent`==`evidence`). Sub-step (ii) stage-staleness `validate` detector already shipped + unit-tested as `CORPUS-COVERAGE.1`. 33 register/PHY/command docs at 0/0 = correct honest-absence (relation bar N/A; intent on register/message-field/transaction surfaces); wire class held WIRE-BASED-100 = 1.000. No fabrication; all oracles orthogonal (no code change); doctrines GREEN. KM `[[relation-completeness-staleness-vs-absence]]` (closure addendum) + `[[stage-staleness-validate-detector]]`.
- next_action (PNT): **KG-ISF-COMPLETENESS now has NO immediately-buildable leaf** (remaining `.1c.ii` upstream-NLP-gated, `.2b` measured-marginal — both deferred/out-of-frontier), so PNT advances to the next active tree. Candidates: **(a)** RAM-safe doc-drift fix — add the missing `nli-verify` dedicated command section to `docs/book/src/commands/quality-and-learning.md` (confirmed gap; the command ships but is only documented in architecture-rationale) + refresh `RUST_CODEBASE_ANALYSIS.md` STALE counts (states ~103.2k LoC / 1435 tests; actual ~128.7k LoC / lib 1709); **(b)** `PDF-VARIANT-DIGESTION.9.3b` CAN serial frame-field parser (pure-Rust, in-progress); **(c)** `CORPUS-COVERAGE.2` continue re-ingest sweep (Docling — RAM has headroom now, ~16% used). Recommend (a): RAM-safe, durable tracked-doc value, fixes real ROADMAP↔codebase↔mdBook drift.
- in_flight_uncommitted: **none** once committed — this `.3` closure is docs-only (task tree + research note + KM + CHANGES/DEV_NOTES/LIVE_ACHIEVEMENT/ROADMAP/TASK_TREE/MEMORY). blockers: none. **Repo HANDOFF-READY.**
- **OPERATIONAL REALITY:** host RAM has headroom this session (~84% free / ~16% used — well under the ≥85%-used kill line, [[feedback_ram_ceiling_monitor]]); Docling/14B work is viable but serialize + monitor. fsmgen is the Perl `subs/fsmgen/bin/fsmgen` (NOT cargo). `generated/` is git-ignored (corpus refresh lands locally; the durable guard is the `validate` stage-staleness detector). Artifact-cleanup standing (≥24h) — no stray build/log artifacts.
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
