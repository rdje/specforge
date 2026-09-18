# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`EXTRACTION-GAP-FIX.5b`** — wire the signal-keyed obligation row reader **in one
  transaction with the corpus rebuild it forces**. Rule, guards and expected effect are frozen by `.5a`:
  **+10 records, 0 fabrications, 2 correct refusals**.
- Next action: **`.5b` is blocked on `CORPUS-CHAIN-CURRENCY.10`, a decision — not on a window.** The
  rebuild is cheap (evidence 0.5s, semantic 0.8s per document; 24 of 27 in about a minute). The cost is the
  other three: AXI, APB and AHB retain **no normalized bundle** and cannot be rebuilt at all, and all ten
  of the rule's records are in AXI. Their only route back is a re-ingest that rewrites the SourceIR
  `WIRE-BASED-100` holds at 1.000. `.10` decides; `EXTRACTION-QUALITY-GAUGE.3k.9` is in the same position.
- Current state: `.5` found deterministic constraint recall is **15.8%** and the bound is **classification,
  not grammar** (the path reads only `SignalValueConstraint`; 195 of 379 obligations carry
  `NormativeStatement`; within its allowed input the grammar converts **69.8%**). `.5a` then refused the
  general widening — simulated, all 43 records read, ~15 correct — and specified the one exact shape.
  **The finding with the widest reach**: composing a reader into a registered evidence derivation
  invalidates proof-carrying artifacts (AXI left the stratum; A/B-confirmed), so a producer change and a
  corpus rebuild are ONE transaction. That is why `.5b` exists and why `.3k.9` was parked.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none is a decision. Two leaves want one detached rebuild window (`.5b`, `.3k.9`); `.3j.4.a`
  wants a model provider. `CLAIM-VERIFICATION-ADOPTION.16` and `SCRATCH-RESIDUE-CONTAINMENT.4` stay open.
