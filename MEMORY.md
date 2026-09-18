# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`EXTRACTION-QUALITY-GAUGE.3j.2.a.ii`** — `.3j.2.a.i` shipped a THIRD subject-resolution
  mode (the full-width-slice alias) and `ADR 0037` §3 enumerates only two: exact, and unique case-fold.
  A shipped resolution mode whose authority is a task leaf is not authorized.
- Next action: resolve it as a decision record that either extends §3 with this bounded third mode and
  its two guards — landing on a declared name, warranted by a typed stated width rather than by the
  spelling resemblance §1 forbids, alpha-equivariant under §7 — or withdraws the rule. Do not widen
  subject resolution any further until it is settled.
- Current state: `.3j.2.b` closed `2026-09-18` by reading the durable layer instead of deriving: ADR 0037
  §1/§3 already forbid the `PSELx` alias and the Knowledge Map already answered it, so `.3j.2`'s refusal
  of `PSEL` is doctrine working. The real defect found there is `.3j.2.b.i`: a same-clause appositive IS a
  local declaration (`SPEC-TO-INTENT-ALIGNMENT.7a`), implemented at one deterministic call site, so the
  LLM path re-refuses APB `llm_sigcon_0000` — a CORRECT record. `.3j.2.c` also open. `.3j.1.b` blocked.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none.
