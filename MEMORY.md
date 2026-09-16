# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`LIVE-DOCUMENT-PRESSURE-HEADROOM`** — **`.30`** partitioned THIS TREE's own evidence under
  the accepted active-task-evidence contract: root **266,362 -> 8,588 bytes (94.0% -> 3.1%)**, 10 semantic
  parts over 17 contiguous regions, `docs/tasks/live-document-pressure-headroom/`. **The contract has a
  property `.21` could not have found**: a migration seals every marked payload byte-exact against the
  capsule and refuses a leaf declared twice in its primary part, so **a leaf cannot close itself inside the
  transaction it seals**. The first run was rolled back whole and re-sealed against a source carrying the
  closing record — write the record, commit, lock, migrate, in that order. Lossless: 17/17 regions
  re-harvested from the part files rebuild the committed blob byte-for-byte; 59/59 nodes survive.
  Earlier today: **`.27`** (toolbox landing, 676->368), **`.29`**/**`.29a`** (census bound 128->224 derived
  from the surface identity, authority retired RED-first).
- Next action: **`.27a`** is the frontier — `DOCTRINE_ENFORCEMENT.md` at **597/700 lines**, ~5 lines per
  revision, ~19 revisions out, the maximum `.27` relocated there. Then **`.29b`** (the surface registry
  declares 95 surfaces; 224 census records fund 82), `.28`, `.20`, `.16`, `.17`, `.6`, `.8`-`.13`,
  `.14b`/`.14c`, `.4`. `task_evidence` bytes/lines are back on `EXTRACTION-QUALITY-GAUGE.md`.
- **To edit a migrated tree**: per-leaf detail lives in its `docs/tasks/<tree>/` parts and marked payloads are
  IMMUTABLE. A part is now read as TWO STRATA (**`.30a`**): inside the markers is pre-migration history,
  outside is current, and the outside declaration WINS — which is the only way to close a leaf the migration
  sealed as open (22 such leaves existed across three trees). New leaf or closure: append to the active part
  OUTSIDE the markers, `- ID:` in the bounded root, a `post_migration` route with NO `source_literal`, re-pin
  that part's `sha256`/metrics, then `--write` and `--check`.
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
