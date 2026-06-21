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
- latest_commit (baseline): **`725814e5`** `ISF-VALUE-WIDTH-EMIT.1 — own + measure ISF value-literal width-alignment (GO)`. **12 ahead** of `origin/main` (push at ~200). The `ISF-VALUE-WIDTH-EMIT.2` close commit is being created THIS turn (subject `ISF-VALUE-WIDTH-EMIT.2 — emit value-width-aligned ISF literals + complete width recovery (CLOSE)`) → will be **13 ahead**; the next checkpoint refreshes this hash. Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (session `2026-06-21`).
- **✅ `ISF-VALUE-WIDTH-EMIT` CLOSED — `.2` done, VERIFIED, committed.** The prior in-flight `.2` code is now COMPILED + TESTED + FSMGen-verified + CI-GREEN. Two emitter-only ADR-0006 fixes in `crates/specforge/src/ir/isf_ir.rs`: `interface_widths` aggregate (recovers a grounded width from a NON-FIRST interface `signal_record` → trace `ATID` width 7) chained ahead of `.2a.i` `port_widths`; and `align_rule_drive_widths` post-pass (re-render an over-wide-but-fitting based literal `W'd<v>`, else DROP the rule with an `isf_value_width_*` residual — never truncate). VERIFIED on real `subs/fsmgen/bin/fsmgen --strict --check --json`: DTI `(ATST 1'd1)` + TRACE `(output ATID (width 7))`/`(ATID 7'd125)` clear the OperandContract value-width error (`has_diagnostics: false`); 4 wire golds **0 NEW** diagnostics (AHB/AXI keep pre-existing orthogonal `isf_conflicting_rule_writes`/`(port expr)`); full suite **1682 passed / 0 failed**; `run_ci.sh` GREEN; `kg-bench` **156/156**.
- next_action: **continue the PNT loop — pick the next eligible frontier.** Active trees remaining: `KG-ISF-COMPLETENESS` (north star — agent-surface `.1b`/`.2+` levers), `KG-ISF-TRANSACTIONS` (body-lever `.2n` NO-GO; next substantive lever = interface DIRECTION emission, owner-design-gated), `CORPUS-COVERAGE.2` (re-ingest 57 normalized-missing docs — RAM-gated Docling, host-local PDFs), `MEMORY-BOUNDED-INGEST`, `NLP-SHALLOW-PARSE`, `CORPUS-PATTERN-REUSE`. Prefer a RAM-light slice; Docling re-ingest only with RAM headroom (`memory_pressure` ≥ ~30% free; autonomously kill ≥85% used).
- in_flight_uncommitted: **none after the `.2` commit** (the `.2` code + KNOWLEDGE_MAP regen + all live docs are in that commit). Untracked (stays): `.claude/settings.json`. blockers: none. Repo handoff-ready.
- **OPERATIONAL REALITY:** host RAM healthy at ~81% free (`memory_pressure`); the earlier 95%-used external pressure has cleared. Builds run `CARGO_BUILD_JOBS≤2`, RAM monitored, kill ≥85% used (`[[feedback_ram_ceiling_monitor]]`). ollama idle.
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication. WIRE-BASED-100 + golds + `kg-bench` **156/156** green by construction for emitter-only changes (the gold measures extraction, not `.isf` bytes). `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (**13 ahead → HOLD**).
