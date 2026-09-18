# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`EXTRACTION-QUALITY-GAUGE.3j.2.b.i`** — a same-clause appositive IS a local declaration
  (`SPEC-TO-INTENT-ALIGNMENT.7a`, `is_same_clause_signal_appositive`, ONE call site inside the
  deterministic antecedent parser), so the LLM path re-refuses APB `llm_sigcon_0000` — a CORRECT record.
- Next action: decide whether that local declaration belongs in the catalog the LLM path types against,
  **scoped to the sentence it was read in**. A global widening would let one sentence's appositive validate
  a subject everywhere, which is the identity minting ADR 0037 §3 forbids. Adjudicate before wiring, on a
  refreshed population — the persisted records predate catalog grounding entirely.
- Current state: `.3j.2.a.ii` closed with **ADR 0047** accepted — subject resolution has exactly three
  modes and any fourth needs its own record; both guards are written in as non-redundant. It also caught
  ADR 0037 verifying alpha-equivariance with a command reaching 3 of its 14 controls; corrected and named.
  `.3j.2.c` open. `.3j.1.b` blocked: no model.
- **Capacity note:** `llm-path-family.md` is at **80.3%** of its bytes health target (52,652 of 65,536;
  rollover at 90% = 58,982). Roughly one more leaf of this size fits before a part split is required.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none.
