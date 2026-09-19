# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`INVARIANT-SHAPE-ADMISSION`** — `.6b` SHIPPED `2026-09-19`. The caption test now runs
  **before** the modal route in `is_invariant_like`; a caption is decided by R1+R2+R3 over its
  sentences, and a non-caption admits on a modal phrase or on a negated permission (R3). Measured on
  rebuilt artifacts: **-8 / +15 = net +7** over 7 of 27 proof-carrying documents, every moved record
  classified and **0 unexplained**; invariants 5,119 -> 5,126. `.6` is closed.
- Next action: **`INVARIANT-SHAPE-ADMISSION.4`** — the last leaf in that tree, and it is a PROGRAM,
  not a slice: ~380 obligation-bearing matrix rows. Its first deliverable is a scoping decision about
  whether an existing table-semantics tree should own it. **Do not start it as a slice.** If picking
  elsewhere, `EXTRACTION-GAP-FIX`, `SIGNAL-DECLARATION-ROW-DROP` and `TEXT-LAYER-IDENTIFIER-SPLIT`
  all carry open bounded leaves.
- **Do not restate a handed-down number — re-derive it.** `.36a` sized `.36b` at a ceiling a
  10,223-byte record had already broken; `.6a`'s census measured two cells of a two-by-two the rule
  has three of (`.6a.1`); `.36b` then published a residual its own arithmetic refused (`.36b.1`).
- **A census that compares against ONE route is an upper bound on a published delta.** `.6a` measured
  additions against route `r1` alone; production has three, so one was already admitted by route 2.
- **A persisted artifact's digest is not evidence about its content.** `.6b`'s cascade moved four
  downstream artifacts whose input was byte-identical; root-caused to accumulated `validate`
  mutations (`proof_context`/`proof_ledger` move, `validation_reports` does not). The corpus is now
  uniformly one-validate fresh and `check_chain_currency.sh` reported 27/27 current throughout.
- **Use `scripts/rebuild_stage_cascade.sh`'s snapshot discipline for any corpus rebuild.** `.6b`
  hand-rolled one and kept only digests plus the predicted movers, so four unpredicted artifacts
  could not be compared section by section.
- `BOUNDED-DECISION-PROVIDER` is **DECIDED: provider REJECTED** (ADR 0051), keyless and zero-egress.
  Do not buy `TYPESAFE_API_KEY` for it. `.3` (the bounded-use contract) stays open and worth writing
  as the standard the next provider is measured against; `.1a.2`/`.2`/`.4`/`.5` are conditional.
- `SIGNAL-DECLARATION-ROW-DROP.2j` is **CLOSED**; `.2j.1` sized the column garble at **9 drifted
  tables across 5 documents** (91 rows, 40 declarations, 51 with none), not one table, and routed it
  to **`.2h.2`, which is now UNBLOCKED** — its stated prerequisite was exactly that census. The drift
  also FABRICATES: TMC `table_0074` mints `DATA` from the word *data* and publishes three `Output`
  wires as `input`. Read `docs/research/direction-column-drift-census.md` before proposing a rule.
- `.36d` owns the claim registry's lifecycle and triggers at the 80% record band (**17 of 21**); 13
  of 21 used. `.36b.1` refused `.36b`'s "envelope spent" — nearly spent, not spent. **Never carry the
  residual as a number**: it shrinks with every claim record written, and writing `.36b.1` itself took
  it 2 -> 1. Read it from `measure_registry_capacity_coherence.py`. Nothing returns a slot:
  `rollover`/`segment`/`archive` appear zero times in `check_claim_verification.pl`.
- The corpus is 27/27 current (`check_chain_currency.sh`).
- A slow gate must be **measured, not attributed**: `scripts/probe_exec_assessment_latency.sh`.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none.
