# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`LIVE-DOCUMENT-PRESSURE-HEADROOM.34`** — `docs/book/src/reference/live-docs.md` is the `shipped_behavior` byte maximum at **118,277 of
  131,072 = 90.2%**, past rollover, and nothing has ever owned it. Health and enforcement are the SAME number there, so the warning band is the only room.
- Next action: read `.20` FIRST and decide whether this is its second instance. `.20` records that every extraction rule lands in the STAGE chapter by
  default, forcing two byte-identical re-splits in six slices; the containment chapter has the same default and grew **3 times today (+3,289 bytes)**. The
  deliverable is a ROUTING RULE cheap enough to follow at authoring time, not a split — a split buys six slices and repeats.
- **`.31` DONE** — `EXTRACTION-QUALITY-GAUGE.md` partitioned: 12 parts, 14 regions, **2,998 -> 132 lines (99.9% -> 4.4%)**, lossless at **268,250 bytes**
  re-harvested byte-for-byte, 56 of 56 nodes re-declared, **`max_unverified_routes` 0**. `task_evidence` relocates to `WIRE-BASED-100.md` at **75.8%/74.9%**
  — below the warning band on both. Contract `doctrine/live_document_size/extraction_quality_gauge_task_evidence.json`.
- **`.33` DONE** — the pressure tree's own active part: **95.2% of its WRITABLE stratum -> 40.9%**, and it now owns NO region, so the whole 58,982-byte
  budget is spendable. **A sealed region cannot move out of an active part ALONE**: `.30a`'s two strata are read inside ONE file, so the post-migration
  records that supersede a sealed `pending` must travel WITH it — 3 of the 7 leaves here did.
  **[[a-sealed-region-cannot-move-out-of-an-active-part-alone]]**.
- **`.32`**: the lifecycle reader now reads the INLINE node shape too — **466 of 1,362** declarations state status on the id's own line, so EQG went 0 of
  56 corroborated to 56 of 56. **[[node-status-has-two-declaration-shapes]]**.
- **Before locking a partition source, derive every node's lifecycle and ADJUDICATE the census** — `.3j` declared `pending` over its own closure record;
  the same rule flagged `.3k`, which is CORRECT. **[[partitioning-a-task-tree-has-a-fixed-registration-price]]**.
- EQG lane next: `.3j.1.a` (frontier), then `.3j.1.b`; also `.3j.2` and `.3j.3`. The partitioned root's Current Frontier section carries the detail,
  including why a PROXY census of `.3j.2` was wrong.
- **NEVER run the full driver by hand** (`G+G` vs `G`). Step 8 = `--fast` every slice + the oracle for the risk (Rust:
  `cargo fmt`/`clippy`/`test -p specforge --lib`, which the gate NEVER runs; producer graph: `--only PRODUCTION-GENERICITY`; a surface or a registered
  enforcer: `--only LIVE-DOC-SIZE`). `--fast` REFUSES without the hook and is NOT a green gate. **Run ONE doctrine with `--only ID[,ID...]` (`--list` for
  ids) — never invoke a gate script by hand.** A gate doctrine has **no stable wall clock**.
- **Any book edit re-derives the `shipped_behavior` `aggregate_change` authority** in `surfaces.jsonl` (baseline = previous baseline+delta, delta = yours,
  rationale <= 512 BYTES) **and may need new `book_quantitative_claims` regions** — the frozen census demands one per candidate line. Run the oracle AFTER
  the last edit; the hook caught this twice today.
- Pressure-tree frontier is **`.12`** (909-byte `doctrine_instance` row), part `warning-assignment`; it and `current-and-open-work` are both `active`. Also
  open: `CLAIM-VERIFICATION-ADOPTION.16`/`.17`; `COMMIT-GATE-SINGLE-RUN.0a` — **runnability is a property of the MOMENT**: ask `measure_doctrine_cost.sh`,
  never assert the machine. `SIGNAL-DECLARATION-ROW-DROP` has no eligible leaf (`.4c` parked); `TEXT-LAYER-IDENTIFIER-SPLIT.1` needs a live VLM (none up).
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none. Push cadence **400** (directive `2026-09-13`, FIXED) — derive ahead/behind from Git, never from a number here; directive 16 gates a push
  on full CI. **Corpus CURRENT — 27/27 at semantic and intent, retention exactly 24.** **Any new production Rust FUNCTION moves `flow_census.json`** —
  re-derive via `aggregate_change`, never edit the literal; boundary counts (144/9) must NOT move. **After editing a governed file OR a checker script run
  `python3 scripts/repin_claim_regions.py --check` then `--apply`**, then refresh the `source` records in ALL FOUR claim registries, then `claims.jsonl`
  durability digests **LAST** (they pin the other three, so any earlier order cascades). A repin REFUSAL on an ABSENT region is yours to decide: prove the
  new digest matches exactly ONE window before re-pinning. That set IS the `Published-claims:` line. More hazards:
  **[[a-doctrine-subset-must-assert-the-leg-that-pays-for-the-rest]]**, **[[chomp-is-a-no-op-under-a-callers-slurp]]**,
  **[[a-gate-doctrine-has-no-stable-wall-clock-on-this-host]]**, **[[toolbox-catalog-is-a-routed-landing]]**, **[[live-surface-edit-bookkeeping-chain]]**,
  **[[claim-verification-task-evidence-migrated]]**, **[[prior-phrase-utf8-byte-as-char]]**,
  **[[a-cheap-structural-rule-overfires-until-you-read-its-selection]]**, **[[actor-taxonomy-grows-in-pairs-not-terms]]**,
  **[[declaration-replay-reads-the-legacy-stratum]]**, **[[a-dropped-declaration-row-is-usually-not-a-signal]]**.
