# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`LIVE-DOCUMENT-PRESSURE-HEADROOM`** — **`.29`** re-derived the census bound from the identity
  it mirrors: `current_claim_census.jsonl` **128->224 records / 65,536->131,072 bytes** under ONE consumed
  authority (`.29a` must retire it on the VERY NEXT commit or the gate refuses it as banked). **The answered
  lifecycle was answering a different question**: over 182 revisions the `change_history` head rows sat at
  **10** through the whole 115->125 move, so that +10 was **100% structural** at a measured **2 records per
  new surface** — and a `surface` record cannot retire while its surface exists. Reads **55.8%**; warning now
  at **64** current surfaces, stop at **82**. Before it, **`.27`** made `TOOLBOX.md` a bounded landing over
  `docs/toolbox/` (**676->368**, `workflow_standards.lines_each` 96.6%->85.3%); its cut came from attributing
  all 43 revisions' growth per section (§5-§7 = 254 of 370; **§1-§4 = +1 in three months**).
- Next action: **`.29a`** (retire the consumed authority — do this FIRST, it is a one-commit deadline), then
  **`.30`** (this tree's own evidence file is now the `task_evidence` bytes maximum at **94.0%**, 16,621 B
  left), **`.27a`** (`DOCTRINE_ENFORCEMENT.md` 597/700), **`.29b`**, `.28`, `.20`, `.16`, `.17`, `.6`,
  `.8`-`.13`, `.14b`/`.14c`, `.4`.
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
