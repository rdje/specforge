# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **none in flight**. `cargo run --manifest-path Cargo.toml -- <cmd>` works again
  (`SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.v.ii.a`); gates that must cover the graph tool say `--workspace`.
- Next action: **`KG-ISF-COMPLETENESS.2a.vii.b`** (measured by `.a`): `(width P)` + `(params (P 1))` placeholder with a
  residual per parameter; derived widths stay residual (`.d`). Then `.c`. FSMGen probe files go under `generated/tmp/`.
  KG-ISF root frontier holds only the next step + eligible order; dated notes go beside their leaf (`CONTAINMENT.4`).
- **Relation replay method** (`.1e`/`.1d.i`): (a) a throwaway `#[ignore]` probe appended to `ir/evidence.rs`
  replays `extract_actor_signal_relations` over all 78 chains' persisted statements — back the file up and
  `cmp` it after; (b) `evidence --dry-run` over the 27 with the HEAD binary AND the new one (`git stash` the
  change, build, copy the binary). **Persisted artifacts are NOT a HEAD baseline** (only 1 of 27 matched on
  every surface); dry-run output is a report — parse after `evidence_ir_json:`. Read every removed fact.
  **Score WIRE-BASED-100 each side with its OWN binary** — `eval-extraction` replays proofs with the running one.
- **A fact card's `reverify` names the exclusion, never the crate** (`COMMIT-GATE-SINGLE-RUN.17`):
  `-p specforge` ran 0 tests for 61 of 93 card commands.
- Part sizes: KG-ISF `enum-surface-recent-leaves` is region-less (post-migration) so it cannot be sealed as
  legacy; new concerns go to a new ACTIVE part (`relation-precision` holds `.1d`-`.1f`, ~43% of its 64 KiB
  health target; the 84% part warning is the collection MAX, not this part). SPEC-TO-INTENT's index must
  roll before its next OPEN leaf; CLAIM-VERIFICATION's `current-and-open-work` is at 84.7% lines, so its next
  leaf opens a new part. Read which contract raised a warning before acting on it.
- **Replay the producer; never read its output's SHAPE** — and replay the producer that MINTED the
  artifact: `producer_name_column`. LOCATE BY ATTRIBUTION (`located_tables`): a shared-member locator paired
  68% of tables with enums they never minted; identical-replay twins are reported ambiguous.
- **RE-PINNING A MOVED CENSUS NEEDS A SEARCH, NOT AN INTENTION.** Census now: **25 enums / 136 members**
  replayable, 555 / 21,595 total; collisions 264 / 7. Grep the old number; never recall it.
- **A MIRROR THAT DROPS ONE CLAUSE CAN MANUFACTURE A FINDING** (`.5.xiii.a`): `.5.xiii`'s AxSIZE refutation
  came from scoring the sanitized cell; the signal seam's identifier test refuses it on its own.
- **A NAMING CHANGE MOVES CONSTRAINTS**: member names are the discovered-value vocabulary. Rebuild the chain
  and diff SemanticIR id-free (statement ids renumber); WIRE-BASED-100 labels 5 AXI statements and is blind.
- **A PARTITION MUST BOUND THE GROWABLE FRACTION, NOT THE PART**; a FRONTIER leaf must be declared in the
  ACTIVE part. KG-ISF contract is HAND-maintained: part metrics/sha256, `current_frontier`,
  `root_required_literals`, `leaf_routes`; re-serialise with Perl `JSON::PP` canonical pretty, then `--write`.
- **A new kg-quality fixture moves `doctrine/live_document_size/corpus_kb.json` AND the literal in
  `scripts/check_corpus_kb_currentness.pl`** — stage the fixture first (git index is authoritative).
- **Claim-digest bill on every governed edit**: `repin_claim_regions.py --apply` (+ `--accept-value`).
- `cargo test -p specforge --lib` says 0 passed for an `ir/` change — use `--workspace --lib --exclude
  specforge-production-graph`. A new production FUNCTION fails `PRODUCTION-GENERICITY`: roll
  `flow_census.json` forward under `aggregate_change`, never edit the number.
- `BOUNDED-DECISION-PROVIDER`: **provider REJECTED** (ADR 0051); do not buy `TYPESAFE_API_KEY`.
- **A parked leaf keeps its blocker until re-derived. Do not restate a handed-down number.**
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none.
