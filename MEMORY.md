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
- **Do not restate a handed-down sizing — re-derive it.** Twice in one day: `.36a` sized `.36b` at an
  8,192 per-record ceiling that a 10,223-byte record had already broken, and `.6a`'s census measured
  two cells of a two-by-two the production rule has three of (`.6a.1`). A number handed forward is an
  input to re-derive, not a value to apply.
- **A census that compares against ONE route is an upper bound on a published delta.** `.6a` measured
  additions against route `r1` alone; production has three, so one predicted addition was already
  admitted by route 2 and the measured delta came in one below the prediction.
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
- `SIGNAL-DECLARATION-ROW-DROP.2j`: the direction-abbreviation census is **done and REFUSES the
  rule** — 837 rows write the full word, exactly 1 usable cell writes an abbreviation, inside the
  garbled ADIv6 `table_0108`. What remains is sizing that table's column garble (9 of 22 lost rows)
  and routing it to the ingest tree.
- `.36d` owns the claim registry's lifecycle and triggers at the 80% record band (**17 of 21**): the
  derivation bought 9 records and then spends the class portable envelope, and nothing returns a slot.
- The corpus is 27/27 current (`check_chain_currency.sh`).
- A slow gate must be **measured, not attributed**: `scripts/probe_exec_assessment_latency.sh`.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none.
