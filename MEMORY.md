# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`LIVE-DOCUMENT-PRESSURE-HEADROOM`** — **`.28`** closed the fact plane and **`.30b`**, which
  `.28`'s own closure opened, shipped with it. **The portable 4,096-key cap was never the binding authority.**
  `max_question_keys` is DERIVED as `ceil(max_facts x 8 / 512) x 512`, a FIXED eight keys per fact, so 449 AND
  **505 both round to 4,096**: the contract would have ACCEPTED an eighth title part and only the projection
  would have refused, later. Funded form, integers: `int(4096 x 327 / 2767)` = **484 facts**; `max_facts`
  **449** is inside it, an eighth part's **505** is 21 above; the only lever is the RATIO falling to **8.11**
  (it is 8.46), never a cap raise. `max_facts`/`max_question_keys` were the last caps here refusing with **no
  band** (`surfaces.jsonl` bands a file, `.22b` a registry count, a FACT is neither); both now carry 80/90,
  `max_shards` deliberately not. Quiet: facts 72.8%, keys 67.6%. Suite 9 -> 13, each pinning warning TEXT.
  Detail: **[[fact-plane-capacity-is-funded-not-declared]]**.
- Next action: **`.12`** — the 909-byte `doctrine_instance` table row `.27a` relocated. Its part
  `warning-assignment` is now the **active** one, because `current_frontier.mode: eligible` requires the
  frontier leaf's primary part to be `active`; moving the frontier means moving that flag (contract field +
  the part's `- State:` literal, outside every marked region). Then `.20`, `.16`, `.17`, `.6`, `.8`-`.13`,
  `.14b`/`.14c`, `.4`.
- **`.30b`'s lesson, general**: a migrated part reports ONE percentage for two different things.
  `current-and-open-work.md` read 87.0% of 65,536 while three quarters was SEALED history, so the spendable
  budget was 15,768 B, **87.3% used, 1,997 left — under one 3,667-byte closure record**. Measure the WRITABLE
  stratum, not the file. Fixed by routing the sealed frontier region to its own `legacy` part: 87.0% -> 65.5%,
  budget 15,768 -> 36,773; `toolbox-and-census-nodes` (21,941 B) is the same weight one region later.
- **Two near-misses.** A `Status:` edit matched the SEALED declaration first (a post-migration node repeats
  its Goal line verbatim — anchor on the text that FOLLOWS it); and `check_live_document_size.pl --check`
  prints usage and reports nothing, the `check_rolling_ledger_protocol.pl` false-green class. 26 warnings.
- Also open: `SIGNAL-DECLARATION-ROW-DROP.2i` (doc claims an arrow arm it lacks; **0 of 663** rows, so fix
  the COMMENT), `.2f`, `.2h.2`; `COMMIT-GATE-SINGLE-RUN.0`; `TEXT-LAYER-IDENTIFIER-SPLIT.1`;
  `EXTRACTION-QUALITY-GAUGE.3j`/`.4c`; `CLAIM-VERIFICATION-ADOPTION.16`/`.17`.
- In-flight uncommitted: none. No background job outstanding.
- **Registering a live-document SURFACE costs exactly TWO permanent census records** (a `surface` + one frozen
  `evidence`, pinned to the first non-blank line of its ALPHABETICALLY FIRST member) plus
  `expected_current_surfaces` +1; adding a MEMBER to an existing surface costs none. `surfaces.jsonl` declares
  up to **95**; 224 census records fund **82** — **`.29b`** owns that.
- Blockers: none. Push cadence **400** (directive `2026-09-13`, FIXED), none due at 292; directive 16 gates
  it on full CI. **Corpus CURRENT — 27/27 at semantic and intent, retention exactly 24.**
  **Any new production Rust FUNCTION moves `flow_census.json`** — re-derive via `aggregate_change`, never
  edit the literal; boundary counts (144/9) must NOT move. **After editing a governed file OR a checker
  script run `python3 scripts/repin_claim_regions.py --check` then `--apply`**, then refresh the `source`
  records in ALL FOUR claim registries, then `claims.jsonl` durability digests **LAST** (they pin the other
  three, so any earlier order cascades). That set IS the `Published-claims:` line. More hazards:
  **[[chomp-is-a-no-op-under-a-callers-slurp]]**, **[[toolbox-catalog-is-a-routed-landing]]**,
  **[[live-surface-edit-bookkeeping-chain]]**, **[[claim-verification-task-evidence-migrated]]**,
  **[[a-cheap-structural-rule-overfires-until-you-read-its-selection]]**, **[[actor-taxonomy-grows-in-pairs-not-terms]]**,
  **[[declaration-replay-reads-the-legacy-stratum]]**, **[[prior-phrase-utf8-byte-as-char]]**, **[[a-dropped-declaration-row-is-usually-not-a-signal]]**.
