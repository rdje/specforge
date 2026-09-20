# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`COMMIT-GATE-SINGLE-RUN`** — `.9` SHIPPED `2026-09-20`. Step 8 prescribed
  `cargo clippy`, which **prints its findings and exits 0**, while `scripts/run_ci.sh` has always
  denied warnings: the workspace had drifted to 5 findings and the branch was **un-pushable**. Both
  halves fixed; step 8 now prescribes `--all-targets -- -D warnings`. **Re-read the other step-8
  commands with the same question: can this exit code express its own findings?**
- Previous unit: **`SIGNAL-DECLARATION-ROW-DROP`** — `.2h.2` SHIPPED `2026-09-20`. A drifted table's rows
  are read against one anchor and a rotated row rotates whole. Corpus declares **the same 1,677
  distinct signals before and after**; what moved is correctness — TMC `table_0074` loses its `Data`
  phantom, gains `AFREADYM`, and has three directions corrected `input` -> `output`; AXI/ACE gains six
  rows. The 27 proof-carrying chains produce the same **604** declarations byte-identical, `kg-bench`
  156/156.
- Next action: pick from **`EXTRACTION-GAP-FIX`**, **`TEXT-LAYER-IDENTIFIER-SPLIT`** or the newly
  eligible **`CORPUS-CHAIN-CURRENCY.11`**. `SIGNAL-DECLARATION-ROW-DROP.2f` is the only remaining
  eligible leaf in ITS tree
  (`.4c` is owner-gated behind `KG-ISF-COMPLETENESS.2a`), and it is still blocked on a discriminator:
  122 real recoveries against 19 `Unused` phantoms, and the repeated-name candidate was measured and
  refuses real signals, so it still needs a different discriminator.
- **Measure the PASS, not the function.** `.2h.2`'s function-level probe of its own rule reported
  `+13` declarations and three new signals; the whole pass reports `+8` and none, because the
  base-name-template guard and the trapped-row pass both sit downstream. One table (SDC-600
  `table_0059`) went 2 declarations -> 0 and that is CORRECT — recovering three rows pushed it over
  `WIRE-BASED-100.10b`'s three-member floor and it is a base-name template.
- **A probe that omits an argument production passes is not a probe of production** (`.2h.2`).
- **A persisted LEGACY artifact is not evidence about the current reader.** TMC `table_0074` records
  `DATA` where the reader emits `Data`. The 51 legacy chains stop at `build_unproved_from_source_ir`.
  Owned by **`CORPUS-CHAIN-CURRENCY.11`**; do not quote a `declared` count off a legacy artifact.
- **Do not restate a handed-down number — re-derive it.** `.2j.1`'s census counts 9 drifted tables;
  the RULE reaches **8**, because the census matches abbreviations and no rule may. Two populations,
  pinned separately.
- `INVARIANT-SHAPE-ADMISSION.4` is a **PROGRAM, not a slice** (~380 obligation-bearing matrix rows);
  its first deliverable is a scoping decision about which tree owns it. Do not start it as a slice.
- `.36d` owns the claim registry's lifecycle and triggers at the 80% record band (**17 of 21**).
  **Never carry the residual as a number** — read it from `measure_registry_capacity_coherence.py`.
- `BOUNDED-DECISION-PROVIDER` is **DECIDED: provider REJECTED** (ADR 0051); `.3` stays open. Do not
  buy `TYPESAFE_API_KEY`. Use `rebuild_stage_cascade.sh`'s snapshot discipline for any corpus rebuild.
- The corpus is 27/27 current (`check_chain_currency.sh`).
- A slow gate must be **measured, not attributed**: `scripts/probe_exec_assessment_latency.sh`.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none.
