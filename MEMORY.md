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
- latest_commit (baseline): **`6092942d`** `KG-ISF-COMPLETENESS.1c — OWN + PROBE Lever E …`. **47 ahead** of `origin/main` (push at ~200 → HOLD). This `KG-ISF-COMPLETENESS.1c.i` CODE commit → **48 ahead**. Untracked (stays): `.claude/settings.json`, `.cache/` (gitignored symlink).
- **MODE: PNT loop** (fresh session `2026-06-23`). Bootstrap-read README/AGENTS/MEMORY_ARCHITECTURE/MEMORY/CORPUS-COVERAGE/KG-ISF-COMPLETENESS/TASK_TREE/COMMIT/TOOLBOX; survey-verified ROADMAP (no drift) + mdBook; `check_doctrines.sh` GREEN. **PNT-PIVOTED off `CORPUS-COVERAGE.2`** to Lever E. Done this session: **`.1c` probe (`6092942d`)** then **`.1c.i` CODE gate (committing now)**.
- **✅ `KG-ISF-COMPLETENESS.1c.i` CODE DONE (committing now) — dense-prose trailing preposition/auxiliary strip.** Extended `consolidate_trailing_fragment` (`ir/evidence.rs`) with a new `NON_ACTOR_TRAILING_FUNCTION_WORDS` const (prepositions + auxiliaries/modals, SUBSET of `NON_ACTOR_LEADING_FUNCTION_WORDS`, conjunctions EXCLUDED — drift-guarded), as a third strip class inside `normalize_relation_actor_name` BEFORE the `.1a` reject (same seam as `.1b.i`). So `host has`/`host to`/`host is`/`host with`→`host`, `cache in`→`cache`. **Verified:** eMMC live cascade **actors 153→138** (4 `host` aux/prep variants merge; 29 phantom names removed; remaining `host selects`/`host tries` = trailing-verb residuals → `.1c.ii`), `host.isf` strict-clean. **WIRE-BASED-100 = 1.000** (fresh-Pattern new-binary eval into temp evidence-root: constraints 6/6·6/6·3/3, relations 5/5·6/6·6/6·1/1, temporal 3/3·4/4·3/3; SWD lone constraint 0/1 documented promotion-only); `kg-bench` 156/156; `run_ci.sh` GREEN (lib 1704, +2). KM `agent-trailing-function-word-consolidation`; README/book/live-docs synced; `.1c.i` leaf carries the enforced acceptance checklist.
- next_action (PNT): **`KG-ISF-COMPLETENESS.1c.ii` — single-relation noun-phrase phantom precision** (the bulk ≈120 of eMMC's 153: `basic bus`/`actual sector`/`B write` — leading noun, single `REL-INFERRED` relation). Probe-FIRST (read-only): a name-shape drop is DISPROVEN unsafe (AMBA real `agent`/`controller`/`decoder` share the shape), so measure a participation+grounding discriminator — what fraction of single-relation subjects are phantom vs. real rare agents, by provenance shape, across the dense-prose docs — BEFORE any gate (completeness forbids dropping a rare-but-real agent). Sibling alts: Lever F/A (ISF enum/value literal, `ISF-VALUE-WIDTH-EMIT` family); continue `CORPUS-COVERAGE.2` (29 docs remain, lever-surfacing tail); also could extend the trailing strip to a closed class of trailing common VERBS (`host selects`/`host tries` residue, kin to `.1c.i`).
- in_flight_uncommitted: the **`KG-ISF-COMPLETENESS.1c.i`** staged set (crates/specforge/src/ir/evidence.rs, docs/tasks/KG-ISF-COMPLETENESS.md, docs/TASK_TREE.md, docs/knowledge/agent-trailing-function-word-consolidation.md [new], KNOWLEDGE_MAP.md, README.md, docs/book/src/pipeline/evidenceir.md, CHANGES.md, DEVELOPMENT_NOTES.md, LIVE_ACHIEVEMENT_STATUS.md, MEMORY.md). blockers: none. **Repo HANDOFF-READY** after commit.
- **OPERATIONAL REALITY:** host RAM monitor before any Docling/14B work — kill ≥85% used ([[feedback_ram_ceiling_monitor]]). This session: read-only probe + one Rust build (`CARGO_BUILD_JOBS=2`, ~1.5min) + run_ci. NOTE: fsmgen is the Perl `subs/fsmgen/bin/fsmgen` (NOT cargo). The eMMC generated/ adapter was refreshed by the `.1c.i` verify (git-ignored cache — intent_ir still the old persisted 153; harmless). Artifact-cleanup standing (≥24h) — no stray build/log artifacts.
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
