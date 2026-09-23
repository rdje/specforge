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
- Next action: **`KG-ISF-COMPLETENESS.5.xiii.a.7`** (instrument locator), then **`.5.xiii.a.5`**, then
  **`.5.xiii.a.3.a`** (CODE) — make a constraint's scope carrier (lead-in,
  nested lead-in, caption) visible where lowering is decided. A persisted carrier costs ONE ADR 0038
  inventory row (not a re-seal — that claim was wrong); avoid the 169 record constructors. Then measure
  any lowering gate on the WIRE-BASED golds (a blanket residual costs AHB HAUSER/HBUSER). Owned open
  findings: `.5.xiii.a.4` row keys, `.a.5` dotted subject, `.a.6` cross-table cancel, `.a.7` locator.
- KG-ISF part `enum-surface-recent-leaves` is past its 80% warning (lines/bytes): split it BEFORE the next
  large leaf record, per `KG-ISF-TASK-EVIDENCE-CONTAINMENT` (move the SEALED stratum, not the tidy one).
- **Replay the producer; never read its output's SHAPE** — and replay the producer that MINTED the
  artifact: `producer_name_column` (schema 3 = landed name rule, legacy = header rule). A loose locator is
  confounded once members carry document names (`RESERVED = 3` recurs); the product replay is authority.
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
