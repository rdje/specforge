# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`EXTRACTION-QUALITY-GAUGE.3j.4`** — retarget the LLM-path measurement population onto the
  **27 measured-stratum documents** (proof ledger, schema-3 EvidenceIR the canonical loader accepts). Every
  `.3j` number to date comes from 7 **historical**-stratum documents, and `ADR 0048` §2 forbids publishing
  a current claim from that stratum. `promote_constraints` has never been run on any of the 27.
- Next action: adjudicate which measured documents to promote BEFORE running anything — a promotion mutates
  the artifact and drops its persisted quality gauge, so it is not read-only on a stratum the gates hold
  current. `ihi0022_l_2025_08` is a current AXI counterpart to the legacy `ihi0022_h_c`.
- Current state: `ADR 0048` splits `generated/` into a measured stratum (27) and a historical one (51) and
  rules that only the first can ground a current claim, that neither is deleted, and that neither is
  rebuilt merely because a schema bumped. **`.3j.1.b` is NOT provider-only-blocked** — its recorded blocker
  was wrong and is corrected in `llm-path-sealed.md`; its prerequisite is now `.3j.4`. Do not start a model
  against the historical seven.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none.
