# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **none in flight**. Gates that must cover the graph tool say `--workspace`.
- Next action: `RUSTDOC-AND-XTASK` `.14.b` (CONSTRAINT-PART-SPAN), `.14.c`-`.14.f`, `.15`. Then ERG
  `.67.b`, `.68`-`.73`, `.3.c.viii.a`; `.61.c`, `.3.c.v.v` wait on KG-ISF `.2a.vii.c.vi`; `.46.c` on FSMGEN-REQUESTS.12;
  `.54.a.ii`, `.44`, SRC `.15`/`.19`, KG-ISF `.1i`/`.1h`/`.2a.vii.c.iv`, EGF, TLIS, LDPH `.34c`/`.42`. Score 11 seeds per slice;
  book regions: `adjudicate_book_quantity_regions.py`. Pin `54ac1c22`; `FSMGEN-REFRESH-INTEGRATE-10` + push wait for
  FSMGen's next exceptional push after it fixes our requests (don't poll); close `FSMGEN-REQUESTS.7`-`.10`, `.12` on
  the first pin containing each. Compare FSMGen revisions on complete, known-good exports.
- **Relation replay method** (`.1e`/`.1d.i`): (a) a throwaway `#[ignore]` probe in `ir/evidence.rs` replays
  `extract_actor_signal_relations` over all 78 chains (back the file up, `cmp` after); (b) `evidence --dry-run` over
  the 31 with the HEAD binary (`git stash`, build, copy) AND the new one. **Persisted artifacts are NOT a HEAD
  baseline**; dry-run output is a report — parse after `evidence_ir_json:`. Read every removed fact.
  **Score WIRE-BASED-100 each side with its OWN binary** — `eval-extraction` replays proofs with the running one.
- **A `source.rs` edit re-seals all 31 chains iff it touches the SOURCE seal closure** (timing classifier helpers
  are in it): list it first (KM `stage-seal-closure-membership`). A new top-level IR field moves that stage's rule
  fields, so it re-seals too: rebuild the content-changed chain by hand, then `rebuild_stage_cascade.sh --write`.
- Part sizes: new ERG leaves -> `table-reading-leaves`; all-closed parts may be archived (`.56.a`); KG-ISF
  actor-surface leaves -> `actor-surface-leaves`. A new part needs a contract part entry + a surfaces target.
  SPEC-TO-INTENT's index must roll before its next OPEN leaf; CLAIM-VERIFICATION's `current-and-open-work` is at
  84.7% lines. Read which contract raised a warning before acting on it.
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
  `leaf_routes`; re-serialise in the encoder CONTRACT-ENCODING declares for it (LDPH: Python indent=1), then `--write`.
- **A new kg-quality fixture moves `doctrine/live_document_size/corpus_kb.json` AND the literal in
  `scripts/check_corpus_kb_currentness.pl`** — stage the fixture first (git index is authoritative).
- **Claim-digest bill on every governed edit**: `repin_claim_regions.py --apply` (+ `--accept-value`).
- `cargo test -p specforge --lib` says 0 passed for an `ir/` change (a card `reverify` too) — use `--workspace
  --lib --exclude specforge-production-graph`. A new production FUNCTION fails `PRODUCTION-GENERICITY`: roll
  `flow_census.json` forward under `aggregate_change`, never edit the number.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none.
