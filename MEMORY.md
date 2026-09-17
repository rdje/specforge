# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- `LIVE-DOCUMENT-PRESSURE-HEADROOM` `.28`/`.30b`/`.30c` closed: the portable 4,096-key cap was never the fact
  plane's binding authority — **[[fact-plane-capacity-is-funded-not-declared]]**.
- Active unit: **`COMMIT-GATE-SINGLE-RUN`** — `.0`/`.1`/`.2`/`.3` done. **NEVER run the full driver by hand**,
  whatever the slice touched: with `G` one gate, manual-first is `G+G` on pass against `G`, and `G+fix+G`
  either way on fail. Step 8 = `--fast` every slice + the oracle for the risk (Rust: `cargo fmt`/`clippy`/
  `test -p specforge --lib`, which the gate NEVER runs; producer graph: `--only PRODUCTION-GENERICITY`;
  governed surface/enforcer: `--only LIVE-DOC-SIZE`). `--fast` REFUSES when the hook is inactive, and a green
  `--fast` is **not** a green gate. **[[a-doctrine-subset-must-assert-the-leg-that-pays-for-the-rest]]**.
- Next action: **`COMMIT-GATE-SINGLE-RUN.3a`** — a `--only LIVE-DOC-SIZE` run exceeded **600s** where green
  takes **96s**; load (2.0-2.5x) does not cover it. Per-surface invocation and a slow nested verifier are both
  REFUTED (1 surface, 13s standalone); untested is whether a STALE input is the expensive path. It decides
  whether a gate cost is a property of the doctrine or of the tree state — which `FAST_EXCLUDE` and `.0a` assume.
- **Quote MEMBERSHIP, never a share** — `.0`'s are WITHDRAWN (load 12.95; one tree read 335s/429s/524s); what
  survives is the costliest four, which is what `FAST_EXCLUDE` names. **A cost measured on a FAILING tree is
  the cost of its first error** (0.8s vs ~31.8s, 40x). `.0a` is **not runnable here**: refuses above load 2.0.
- `SIGNAL-DECLARATION-ROW-DROP.2i` closed (comment, not rule: **0 of 663** trapped rows carry an arrow). That
  tree has **no eligible leaf**: `.2f`/`.2h.2` blocked, `.4c` parked behind `KG-ISF-COMPLETENESS.2a`.
- **Run ONE doctrine with `--only ID[,ID...]` (`--list` for ids) — never invoke a gate script by hand.** All 42
  gates now REFUSE an unknown flag (was 34/42); one registry argv named a flag its script never had.
- Pressure-tree frontier is **`.12`** (909-byte `doctrine_instance` row), part `warning-assignment`, now
  `active` — `current_frontier.mode: eligible` needs the frontier leaf's part `active`. Then `.20`, `.16`,
  `.17`, `.6`, `.8`-`.13`, `.14b`/`.14c`, `.4`.
- **`.30b`**: a migrated part reports ONE percentage for two things — `current-and-open-work.md` read 87.0% of
  65,536 while three quarters was SEALED, leaving **1,997 B**. Measure the WRITABLE stratum, not the file.
- A post-migration node REPEATS its Goal line, so anchor a `Status:` edit on what FOLLOWS it, not on the line.
- Also open: `TEXT-LAYER-IDENTIFIER-SPLIT.1`; `EXTRACTION-QUALITY-GAUGE.3j`/`.4c`;
  `CLAIM-VERIFICATION-ADOPTION.16`/`.17`.
- In-flight uncommitted: none. No background job outstanding.
- **Registering a live-document SURFACE costs TWO permanent census records** (a `surface` + one frozen
  `evidence`, pinned to the first non-blank line of its ALPHABETICALLY FIRST member) plus
  `expected_current_surfaces` +1; adding a MEMBER costs none. **`.29b`** owns the cap.
- Blockers: none. Push cadence **400** (directive `2026-09-13`, FIXED) — derive ahead/behind from Git, never
  from a number here; directive 16 gates a push on full CI. **Corpus CURRENT — 27/27 at semantic and intent,
  retention exactly 24.** **Any new production Rust FUNCTION moves `flow_census.json`** — re-derive via
  `aggregate_change`, never edit the literal; boundary counts (144/9) must NOT move. **After editing a governed
  file OR a checker script run `python3 scripts/repin_claim_regions.py --check` then `--apply`**, then refresh
  the `source` records in ALL FOUR claim registries, then `claims.jsonl` durability digests **LAST** (they pin
  the other three, so any earlier order cascades). That set IS the `Published-claims:` line. More hazards:
  **[[chomp-is-a-no-op-under-a-callers-slurp]]**, **[[toolbox-catalog-is-a-routed-landing]]**,
  **[[live-surface-edit-bookkeeping-chain]]**, **[[claim-verification-task-evidence-migrated]]**,
  **[[a-cheap-structural-rule-overfires-until-you-read-its-selection]]**, **[[actor-taxonomy-grows-in-pairs-not-terms]]**,
  **[[declaration-replay-reads-the-legacy-stratum]]**, **[[prior-phrase-utf8-byte-as-char]]**, **[[a-dropped-declaration-row-is-usually-not-a-signal]]**.
