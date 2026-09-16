# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`LIVE-DOCUMENT-PRESSURE-HEADROOM`** — **`.18`** made `durability.stale_check` real. It was
  schema-validated but NEVER executed: a marker its producer can never print passes at HEAD with **exit 0**.
  It could not simply be turned on — **3 of the 5** declared staleness gates name THIS checker as their
  producer, so a naive execution re-enters itself and the first attempt did not terminate. A gate now runs
  only when a DIFFERENT producer discharges it, recognised from the ARGV that would re-enter; the summary
  reports **executed / deferred / self-referential** so no tier can imply a run that did not happen.
  Staleness execution is CI-tier via `CLAIM_VERIFICATION_EXECUTE_STALE_GATES=1`, exported by
  `check_doctrines.sh --all`. **A cost figure was WITHDRAWN**: HEAD's apparent 1.0 s was an early exit on
  unrelated stale digests, because `validate_registry` SKIPS execution when errors already exist. Clean:
  gate **30.4 s**, staleness tier **55.2 s**.
  **`.15`** registered the 16th doctrine `OWNERSHIP-CITATIONS`. Earlier: **`.1`** split the 296-line card;
  **`.23`** encoders are per FILE; **`.24`**-**`.24b`** raised the fact plane to 7 parts (TERMINAL);
  **`.25`**/**`.26`**/**`.26a`** stopped a full collection being the one silent state.
- Also newly owned: **`.27`** — `TOOLBOX.md` **676/700 lines = 96.6%**, no band, and `.7`'s assignment never
  covered this row; `.23` already DECLINED to ship a tool rather than spend it (median entry 12 lines of 24).
- Next action: **`.4d.ii`** — the nearest measured stop: `VALIDATION_SNAPSHOT.md` refuses its 5th reviewed
  document at any marginal cost (544/640; per-document 163/147/112/110 over 12 fixed) and 78 built artifacts
  stand behind 4 reviewed, so the surface is O(corpus) against a constant bound. It is a RUST producer change
  (`render_validation_snapshot_doc`), so it needs the TOOLBOX acceptance checklist and the `flow_census.json`
  re-derivation, and the reviewed content may NOT be regenerated.
- **Silent-failure lessons**: **[[chomp-is-a-no-op-under-a-callers-slurp]]** (a checker walked ZERO
  registries while every doctrine reported PASS — assert the POPULATION, not the exit code); and a sweep
  run with the wrong flag is a false green (`--check` on `check_rolling_ledger_protocol.pl` prints usage).
- **Continuation path PROVEN** (`CLAIM-VERIFICATION-ADOPTION.17`): append to the ONE active part OUTSIDE
  the markers, `- ID:` in the root registry, a `post_migration` route with NO `source_literal`, re-pin the
  part's `sha256`/metrics, `--write`, `--check`; index routes **2 open of 47**. That contract IS `JSON::PP`
  pretty+canonical, but DERIVE the encoder per file (`.23`) instead of assuming it.
- Also open: `SIGNAL-DECLARATION-ROW-DROP.2i` (doc claims an arrow arm it lacks; **0 of 663** rows, so fix
  the COMMENT), `.2f`, `.2h.2`; `COMMIT-GATE-SINGLE-RUN.0`; `TEXT-LAYER-IDENTIFIER-SPLIT.1`;
  `EXTRACTION-QUALITY-GAUGE.3j`/`.4c`; `CLAIM-VERIFICATION-ADOPTION.16`/`.17`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.15`-`.18`/`.20`/`.23`/`.4d.ii`.
- In-flight uncommitted: none. No background job outstanding.
- Blockers: none. Push cadence **400** (directive `2026-09-13`, FIXED), none due at 290; directive 16 gates
  it on full CI. **Corpus CURRENT — 27/27 at semantic and intent, retention exactly 24.**
  **Any new production Rust FUNCTION moves `flow_census.json`** — re-derive via `aggregate_change`, never
  edit the literal; boundary counts (144/9) must NOT move. **After editing a governed file OR a checker
  script run `python3 scripts/repin_claim_regions.py --check` then `--apply`**, refresh `shipped_behavior`,
  then the `claims.jsonl` digests LAST — that set IS the `Published-claims:` line. More hazards:
  **[[claim-verification-task-evidence-migrated]]**, **[[live-surface-edit-bookkeeping-chain]]**,
  **[[a-cheap-structural-rule-overfires-until-you-read-its-selection]]**, **[[actor-taxonomy-grows-in-pairs-not-terms]]**,
  **[[a-single-index-bit-cell-is-a-width-of-one]]**, **[[declaration-replay-reads-the-legacy-stratum]]**,
  **[[prior-phrase-utf8-byte-as-char]]**, **[[a-dropped-declaration-row-is-usually-not-a-signal]]**.
