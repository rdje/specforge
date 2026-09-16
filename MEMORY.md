# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`LIVE-DOCUMENT-PRESSURE-HEADROOM`** — **`.29b`** made the census/surface-registry identity
  MECHANICAL. The pair cannot agree by comparing headers: `surfaces.jsonl` bounds RECORDS (96 -> 95 surfaces)
  while the census mirrors only the CURRENT subset, and **nothing bounds that subset**. Measured: 50 current
  of 68 surfaces, census **133/224**, **1.28** evidence records per surface beyond the mandatory disposition,
  so 95 all-current needs **236** of 224 and the bound warns at **64** current surfaces / stops at **83**.
  Two arms now: reachable (today + one 3-surface partition event) is an ERROR; the declared worst case is a
  WARNING printing its own arithmetic. Earlier today: **`.27`**, **`.27a`** (doctrine instance routed out;
  `workflow_standards` warns on nothing), **`.29`**/**`.29a`**, **`.30`** (this tree partitioned), **`.30a`**
  (two-strata rule, which `.27a` and `.29b` then used to close sealed leaves in place).
- Next action: **`.29c`** — decide how many live-document surfaces this repo may have. Mirroring 95 WITH a
  band needs ~**295** census records against a **portable cap of 256**, so: (a) raise that portable cap,
  (b) give `surfaces.jsonl` a declared CURRENT-population bound (64 at today's cost, vs 50 now), or
  (c) reduce the 1.28 per-surface cost. **This is a scope decision — see the callout.** Then `.12` (the
  relocated 909-byte table row), `.28`, `.20`, `.16`, `.17`, `.6`, `.8`-`.13`, `.14b`/`.14c`, `.4`.
- **Registering a live-document surface costs exactly TWO permanent census records** (a `surface` + one frozen
  `evidence`, pinned to the first non-blank line of the surface's ALPHABETICALLY FIRST member), plus
  `expected_current_surfaces` +1. `surfaces.jsonl` declares up to **95** surfaces; 224 census records fund
  **82**, and the portable 256-record cap cannot fund 95 with a band — **`.29b`** owns that.
- **Silent-failure lessons**: **[[chomp-is-a-no-op-under-a-callers-slurp]]** (a checker walked ZERO
  registries while every doctrine reported PASS — assert the POPULATION, not the exit code); and a sweep
  run with the wrong flag is a false green (`--check` on `check_rolling_ledger_protocol.pl` prints usage).
- Also open: `SIGNAL-DECLARATION-ROW-DROP.2i` (doc claims an arrow arm it lacks; **0 of 663** rows, so fix
  the COMMENT), `.2f`, `.2h.2`; `COMMIT-GATE-SINGLE-RUN.0`; `TEXT-LAYER-IDENTIFIER-SPLIT.1`;
  `EXTRACTION-QUALITY-GAUGE.3j`/`.4c`; `CLAIM-VERIFICATION-ADOPTION.16`/`.17`.
- In-flight uncommitted: none. No background job outstanding.
- Blockers: none. Push cadence **400** (directive `2026-09-13`, FIXED), none due at 291; directive 16 gates
  it on full CI. **Corpus CURRENT — 27/27 at semantic and intent, retention exactly 24.**
  **Any new production Rust FUNCTION moves `flow_census.json`** — re-derive via `aggregate_change`, never
  edit the literal; boundary counts (144/9) must NOT move. **After editing a governed file OR a checker
  script run `python3 scripts/repin_claim_regions.py --check` then `--apply`**, refresh `shipped_behavior`,
  then the `claims.jsonl` digests LAST — that set IS the `Published-claims:` line. More hazards:
  **[[toolbox-catalog-is-a-routed-landing]]**, **[[live-surface-edit-bookkeeping-chain]]**,
  **[[claim-verification-task-evidence-migrated]]**,
  **[[a-cheap-structural-rule-overfires-until-you-read-its-selection]]**, **[[actor-taxonomy-grows-in-pairs-not-terms]]**,
  **[[declaration-replay-reads-the-legacy-stratum]]**, **[[prior-phrase-utf8-byte-as-char]]**,
  **[[a-dropped-declaration-row-is-usually-not-a-signal]]**.
