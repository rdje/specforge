# ACTOR-NOUN-RELATION-DECLARATION: the relation path declares an actor role as a wire

## Metadata

- Tree ID: `ACTOR-NOUN-RELATION-DECLARATION`
- Status: `active` (`2026-09-12`; opened by `PROSE-NAME-CELL-DECLARATION.3`, `.0` open)
- Roadmap lane: `R2` (extraction correctness / false-positive control)
- Created: `2026-09-12`
- Last updated: `2026-09-12`
- Owner: repo-local workflow

## Goal

`synthesize_declarations_from_relations` (`crates/specforge/src/ir/evidence.rs`, the
`WIRE-BASED-100.10b` path) is the one place that turns an actor→signal **relation** into a formal
**declaration**, and the comment there already says so: *"it is the one path that can mint a name the
document never declared."* It carries one guard — `is_alpha_variant_placeholder` — for a placeholder
standing in for a family of declared signals.

It carries no guard for the other obvious way a relation's "signal" can fail to be a wire: **the
relation extractor mistook the actor for the signal.** AHB's persisted EvidenceIR declares

```text
Signal Manager is output width Exclusive okay, selected by the decoder. a.
```

`Manager` is a role in this document's own actor taxonomy — the same taxonomy
`infer_signal_direction_from_actor_text` and `builtin_actor_taxonomy_role_in_text` consult on every
signal table — and the declaration path never asks it.

## How it was found

`PROSE-NAME-CELL-DECLARATION.3` wrote and measured a guard refusing a description sentence as a
parametric width, observed it RED, and then rebuilt AHB to check the blast radius. The declarations
were unchanged (79 → 79, same set) but **AHB's IntentIR interface count went from 42 to 62**. Every one
of the 24 new interfaces is a `…_manager` grouping:

```text
interface_haddr_manager   interface_hclk_manager   interface_hrdata_manager   interface_htrans_manager   …
```

The cause is not the width rule. Before it, the phantom's declaration read
`Signal Manager is output width Exclusive okay, selected by the decoder. a.`; after it,
`Signal Manager is output.` **Removing the garbage width made the phantom declaration well-formed, and
a well-formed phantom propagates further.** `Reset` behaves the same way from `statement_0856`.

That is why `.3` is deferred rather than shipped: its only live effect in the corpus is to strengthen
this defect, and shipping it would publish a 48% increase in one document's interface count, all
phantom. Fix the cause, then `.3` becomes free.

## Non-Goals

- Do not fix it by refusing the *width*. That is `PROSE-NAME-CELL-DECLARATION.3`, it is correct on its
  own terms, and it is blocked on this tree rather than the other way round.
- Do not widen the actor taxonomy. The roles this needs are the ones the repository already ships and
  already treats as universal interface grammar; adding a word to that list is a different decision
  with a different blast radius.
- Do not touch the relation extractor's own precision. A relation naming an actor as its signal may
  well be a relation worth keeping; what must stop is turning it into a **declaration**.

## Acceptance Criteria

- The population is measured **through the reader**: how many relation-derived declarations name a term
  the built-in actor taxonomy resolves to a role, per document and per stratum, with every distinct
  name listed for adjudication.
- Any rule shipped carries a corpus-wide count of what it newly refuses **and an adjudicated sample**,
  and is shown not to refuse a real wire whose name merely contains a role word.
- The blast radius is measured on the artifact, not assumed: AHB's interface, constraint and rule counts
  before and after, and the chain rebuilt rather than left stale.
- `scripts/check_doctrines.sh` green; no ceiling, milestone, or contract widened.

## Task Tree

- ID: `ACTOR-NOUN-RELATION-DECLARATION` · Status: `active` (`2026-09-12`) · Children: `.0`

- ID: `ACTOR-NOUN-RELATION-DECLARATION.0` · Status: `pending` · Goal: **measure before proposing
  anything** — the standing rule in this area, and five rules in the sibling reader have now over-fired,
  been withdrawn, or had their premise falsified on adjudication.
  Census every statement of the form `Signal <name> is …` that is **not** in
  `table_signal_declaration_provenance` (those are the relation-derived and prose-derived ones), resolve
  each `<name>` through `builtin_actor_taxonomy_role_in_text`, and report per document and per stratum
  with every distinct name listed. Two questions the census must answer before any rule is written:
  1. how many such declarations exist at all, and how many name an actor role;
  2. whether any **real** signal name resolves to a role — a wire whose name contains `master`,
     `target` or `source` would be refused by a naive rule, and this repository's history says look
     before counting.
  Non-goal: any code change.
  Prerequisite: none. Verification: read-only; no artifact written or mutated.

## Current Frontier

Ordered; PNT selects the first eligible leaf.

1. `ACTOR-NOUN-RELATION-DECLARATION.0` — the census. Nothing else in this tree may start before it.

## Decisions

- `2026-09-12` — **opened as its own tree rather than a leaf of `PROSE-NAME-CELL-DECLARATION`.** That
  tree is about what a table **row** declares; this is about what a **relation** declares. They meet
  only because a width guard made the symptom visible, and the sibling tree's frontier is already four
  leaves deep in a different mechanism.
- `2026-09-12` — **`PROSE-NAME-CELL-DECLARATION.3` waits on this, not the reverse.** Both touch the same
  two AHB statements. Shipping `.3` first publishes 24 phantom interfaces; shipping this first makes
  `.3` free. The same ordering argument settled `.2` before `.1` in the sibling tree, and for the same
  reason: repair the cause, then the guard costs nothing.

## Open Questions

- Is the right refusal *the declaration* or *the relation*? Refusing only the declaration keeps a
  possibly-useful relation and is the smaller change; refusing the relation is more honest if the
  extractor genuinely mis-parsed its subject. `.0`'s adjudication should say which, with examples.

## Blockers

None.

## Verification Log

Pending: `.0` is a read-only census.

## Commit Log

Opened in the commit that deferred `PROSE-NAME-CELL-DECLARATION.3`.

## Changelog

- `2026-09-12` — tree created. A width guard measured in `PROSE-NAME-CELL-DECLARATION.3` moved AHB's
  IntentIR interface count from 42 to 62 by making an existing phantom declaration well-formed; the
  phantom is the actor role `Manager`, minted as a wire by the relation→declaration path.
