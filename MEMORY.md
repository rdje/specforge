# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **none in flight**. Gates that must cover the graph tool say `--workspace`.
- Next action: ERG `.41.c`, `.43.a`, `.3.c.vii`-`.3.f.iii`, KG-ISF `.1h`/`.2a.vii.c.iv`, EGF `.14`/`.12`, TLIS `.3`,
  LDPH `.34c` (isf-adapter.md at 83%)/`.42`. Score all 11 seeds every slice.
  Pin `54ac1c22`; `FSMGEN-REFRESH-INTEGRATE-10` + push wait for FSMGen's next, exceptional push after it fixes our
  requests (cadence 300+ commits; don't poll). All forwarded requests acknowledged (`FSMGEN-REQUESTS.6`); close each on
  the first pin containing it. Comparing FSMGen revisions: complete exports + known-good first.
- **Relation replay method** (`.1e`/`.1d.i`): (a) a throwaway `#[ignore]` probe in `ir/evidence.rs` replays
  `extract_actor_signal_relations` over all 78 chains (back the file up, `cmp` after); (b) `evidence --dry-run` over
  the 31 with the HEAD binary (`git stash`, build, copy) AND the new one. **Persisted artifacts are NOT a HEAD
  baseline**; dry-run output is a report — parse after `evidence_ir_json:`. Read every removed fact.
  **Score WIRE-BASED-100 each side with its OWN binary** — `eval-extraction` replays proofs with the running one.
- **A fact card's `reverify` names the exclusion, never the crate** (`COMMIT-GATE-SINGLE-RUN.17`):
  `-p specforge` ran 0 tests for 61 of 93 card commands.
- **A `source.rs` edit re-seals all 31 chains iff it touches the SOURCE seal closure** (timing classifier helpers
  are in it): list it first (KM `stage-seal-closure-membership`).
- Part sizes: ERG `.3` records go to `census-leaves`, other ERG leaves to `clause-and-register-leaves`; KG-ISF
  actor-surface leaves to `actor-surface-leaves`. A new part needs a surfaces target + census record.
  SPEC-TO-INTENT's index must roll before its next OPEN leaf; CLAIM-VERIFICATION's `current-and-open-work` is at
  84.7% lines, so its next leaf opens a new part. Read which contract raised a warning before acting on it.
- **Replay the producer; never read its output's SHAPE** — and replay the producer that MINTED the
  artifact: `producer_name_column`. LOCATE BY ATTRIBUTION (`located_tables`): a shared-member locator paired
  68% of tables with enums they never minted; identical-replay twins are reported ambiguous.
- **RE-PINNING A MOVED CENSUS NEEDS A SEARCH, NOT AN INTENTION.** Census `2026-09-29`: **32 enums / 179 members**
  replayable, 522 / 20,571 total; collisions 243 / 8. Grep the old number; never recall it.
- **A MIRROR THAT DROPS ONE CLAUSE CAN MANUFACTURE A FINDING** (`.5.xiii.a`): `.5.xiii`'s AxSIZE refutation
  came from scoring the sanitized cell; the signal seam's identifier test refuses it on its own.
- **A NAMING CHANGE MOVES CONSTRAINTS**: member names are the discovered-value vocabulary. Rebuild the chain
  and diff SemanticIR id-free (statement ids renumber); WIRE-BASED-100 labels 5 AXI statements and is blind.
- **A PARTITION MUST BOUND THE GROWABLE FRACTION, NOT THE PART**; a FRONTIER leaf must be declared in an ACTIVE
  part (ERG, partitioned by `.38`: `.3` family -> `program-and-census`, other leaves -> `recent-leaves`). KG-ISF's
  and ERG's contracts are HAND-maintained: part metrics/sha256, `current_frontier`, `root_required_literals`,
  `leaf_routes`; re-serialise with Perl `JSON::PP->utf8` canonical pretty, then `--write`.
- **A new kg-quality fixture moves `doctrine/live_document_size/corpus_kb.json` AND the literal in
  `scripts/check_corpus_kb_currentness.pl`** — stage the fixture first (git index is authoritative).
- **Claim-digest bill on every governed edit**: `repin_claim_regions.py --apply` (+ `--accept-value`).
- `cargo test -p specforge --lib` says 0 passed for an `ir/` change — use `--workspace --lib --exclude
  specforge-production-graph`. A new production FUNCTION fails `PRODUCTION-GENERICITY`: roll
  `flow_census.json` forward under `aggregate_change`, never edit the number.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none.
