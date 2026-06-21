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
- latest_commit (baseline): **`bcd2a6ce`** `ISF-VALUE-WIDTH-EMIT.2 — emit value-width-aligned ISF literals + complete width recovery (CLOSE)`. **13 ahead** of `origin/main` (push at ~200 → HOLD). This MEMORY-only continuity refresh is the next commit (→ 14 ahead). Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (session `2026-06-21`).
- **✅ `ISF-VALUE-WIDTH-EMIT` CLOSED — `.2` done, VERIFIED, committed.** The prior in-flight `.2` code is now COMPILED + TESTED + FSMGen-verified + CI-GREEN. Two emitter-only ADR-0006 fixes in `crates/specforge/src/ir/isf_ir.rs`: `interface_widths` aggregate (recovers a grounded width from a NON-FIRST interface `signal_record` → trace `ATID` width 7) chained ahead of `.2a.i` `port_widths`; and `align_rule_drive_widths` post-pass (re-render an over-wide-but-fitting based literal `W'd<v>`, else DROP the rule with an `isf_value_width_*` residual — never truncate). VERIFIED on real `subs/fsmgen/bin/fsmgen --strict --check --json`: DTI `(ATST 1'd1)` + TRACE `(output ATID (width 7))`/`(ATID 7'd125)` clear the OperandContract value-width error (`has_diagnostics: false`); 4 wire golds **0 NEW** diagnostics (AHB/AXI keep pre-existing orthogonal `isf_conflicting_rule_writes`/`(port expr)`); full suite **1682 passed / 0 failed**; `run_ci.sh` GREEN; `kg-bench` **156/156**.
- next_action (PNT): **`CORPUS-COVERAGE.2` re-ingest #17** is the next eligible leaf — re-ingest the next normalized-missing doc from `.cache/local-references/chipdoc` (mounted), one doc per slice, protocol-specs-first then the large register/TRM/ISA docs (#1–16 done; the remaining trend LARGE ≈900pp). Method: `DOCLING_DEVICE=cpu`, the built-in `.4a` RAM guard active (clean abort ≥85% used), Ollama idle, monitor RAM+swap between docs (`[[feedback_ram_ceiling_monitor]]`); then `evidence → semantic → intent → adapt --target isf` + `validate`, record before/after typed-surface deltas in the `.2` re-ingest log, commit per `COMMIT.md`. **This is a RAM-heavy, long-running Docling slice → the tree's own "dedicated-session discipline" applies — best run from a FRESH, sharp session.** Other active trees (`KG-ISF-COMPLETENESS` `.1b.ii`/`.1b.iv` deferred-with-trigger / `.2+`; `KG-ISF-TRANSACTIONS` direction-emit owner-design-gated; `NLP-SHALLOW-PARSE` + `MEMORY-BOUNDED-INGEST` measured-exhausted/standing; `CORPUS-PATTERN-REUSE` consume-side built-deferred) have no freely-pickable light leaf, so the re-ingest IS the substantive frontier.
- in_flight_uncommitted: **none after the `.2` commit** (the `.2` code + KNOWLEDGE_MAP regen + all live docs are in that commit). Untracked (stays): `.claude/settings.json`. blockers: none. Repo handoff-ready.
- **OPERATIONAL REALITY:** host RAM healthy at ~81% free (`memory_pressure`); the earlier 95%-used external pressure has cleared. Builds run `CARGO_BUILD_JOBS≤2`, RAM monitored, kill ≥85% used (`[[feedback_ram_ceiling_monitor]]`). ollama idle.
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication. WIRE-BASED-100 + golds + `kg-bench` **156/156** green by construction for emitter-only changes (the gold measures extraction, not `.isf` bytes). `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (**14 ahead → HOLD**).
