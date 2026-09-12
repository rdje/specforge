# ACTOR-NOUN-RELATION-DECLARATION: an inferred declaration mints an ordinary word as a wire

## Metadata

- Tree ID: `ACTOR-NOUN-RELATION-DECLARATION`
- Status: `active` (`2026-09-12`; `.0` and `.1` done; the tree's own goal is met, one downstream finding is tracked elsewhere)
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

**The tree id keeps the name it was opened under; `.0` corrected its subject.** The census found three
phantoms in the current stratum, not one, and only `Manager` is an actor: the others are AHB's `Reset`
(a signal *function*; the document's real reset is `HRESETn`) and ADIv6's `In` (a preposition). What
the three share is orthography — an initial capital over an all lower-case remainder — which no
document in this corpus uses for a wire. The goal is therefore the general one: **an inferred
declaration must not mint a name that is spelled like an ordinary word.**

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

- ID: `ACTOR-NOUN-RELATION-DECLARATION` · Status: `active` (`2026-09-12`) · Children: `.0`-`.1`

- ID: `ACTOR-NOUN-RELATION-DECLARATION.0` · Status: `done` (`2026-09-12`) · Goal: **measure before
  proposing anything** — the standing rule in this area, and five rules in the sibling reader have now over-fired,
  been withdrawn, or had their premise falsified on adjudication.
  Census every statement of the form `Signal <name> is …` that is **not** in
  `table_signal_declaration_provenance` (those are the relation-derived and prose-derived ones), resolve
  each `<name>` through `builtin_actor_taxonomy_role_in_text`, and report per document and per stratum
  with every distinct name listed. Two questions the census must answer before any rule is written:
  1. how many such declarations exist at all, and how many name an actor role;
  2. whether any **real** signal name resolves to a role — a wire whose name contains `master`,
     `target` or `source` would be refused by a naive rule, and this repository's history says look
     before counting.
  **Both answered, and the second answer changed the tree** — see the result section. Delivered by
  `scripts/measure_untabled_signal_declarations.py`.
  Verification: read-only; no artifact written or mutated.
  Commit: `ACTOR-NOUN-RELATION-DECLARATION.0`

- ID: `ACTOR-NOUN-RELATION-DECLARATION.1` · Status: `done` (`2026-09-12`) · Goal: **refuse a relation- or
  prose-derived declaration whose name is spelled like an ordinary word**, not one that resolves to an
  actor role. `.0` measured that the role test catches 1 of the 3 phantoms and the orthographic test
  catches 3 of 3 at zero measured cost.
  The rule applies **only** to the two untabled paths. A table declaration is the document's own
  spelling and is never subject to it — that containment is what makes an orthographic test safe, and
  it must be visible in the code rather than implied.
  Prerequisite: `.0`. Verification: observed RED against each of the three names; the 24 sole-source
  declarations (ADIv6, I2C, I2S) shown to survive by name; corpus-wide refusal count with the sample
  adjudicated; AHB's chain rebuilt and its interface/constraint counts compared before and after,
  because `PROSE-NAME-CELL-DECLARATION.3` measured that this document's downstream is sensitive to
  exactly these statements. Then re-measure and unblock `PROSE-NAME-CELL-DECLARATION.3`.
  Shipped as `inferred_name_is_an_ordinary_word`, applied at all three inference sites and at none of
  the table sites. Result below.
  Commit: `ACTOR-NOUN-RELATION-DECLARATION.1`

## Acceptance Checklist (enforced)

- [x] **REPRODUCE / MEASURE** — `python3 scripts/measure_untabled_signal_declarations.py`: 53 current
  declarations with no table provenance; 26 duplicate, 24 sole-source, **3 spelled like an ordinary
  word** (`Manager`, `Reset`, `In`).
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`
  `synthesize_directions_from_relations` and the two prose appositive emitters: each mints a
  declaration from an INFERRED name and guards only `is_alpha_variant_placeholder`.
  `cargo test -p specforge-core --lib a_relation_naming_an_ordinary_word_declares_nothing` reproduces
  it exactly — `["Signal Manager is output.", "Signal Reset is output.", "Signal SWCLK is output."]`.
- [x] **ADDRESSED (verified)** — measured on rebuilt artifacts, not assumed. AHB: 2 phantoms gone,
  declarations 79 → 79, **interfaces unchanged at 42**, conditional rules **29 → 37** with all 29
  pre-existing rules byte-identical and every new rule resolving to a real signal. ADIv6: `In` gone,
  interfaces 21 → 14 (all 7 lost are phantom groupings), 44 constraints keep their id and shed only
  phantom anchors, `PORTCONNECTED` still declared. Corpus-wide there is now no inferred declaration
  spelled like an ordinary word.
- [x] **NO REGRESSION** — `cargo test` 472 / 168 / **1421** / 4 green (the last after re-pinning the
  production-graph census, `PRODUCTION-GRAPH-CENSUS-PIN`); `cargo fmt --check` and
  `cargo clippy --all-targets -D warnings` green; `scripts/check_doctrines.sh` green;
  `scripts/check_chain_currency.sh` current across all four stages with retention at the declared 24.
  **The one cost is stated, not hidden**: 13 ADIv6 invariants anchored solely to the phantom vanish
  with no record — shipped deliberately, tracked as `[[ANCHORLESS-INVARIANT-DROP]]`.
- [x] **GENERICITY (ADR 0006)** — the test reads orthography (an initial capital over an all
  lower-case remainder), never meaning. It is applied only where a name was inferred and at no table
  site, and the control pins both the refused and the kept shapes under that rule.
- [x] **LOCKSTEP** — the mdBook's EvidenceIR chapter describes the table declaration reader and the
  relation path's `is_alpha_variant_placeholder` guard; it makes no claim about what an inferred name
  may be, so no book text became false. No production rule was deleted or replaced. Task trees and
  `MEMORY.md` updated; the new finding has its own tree.

## Current Frontier

No eligible leaf: `.0` and `.1` are done and the tree's goal is met — no inferred declaration in the
corpus now mints an ordinary word. Two things it uncovered are tracked elsewhere and neither belongs
here: `[[ANCHORLESS-INVARIANT-DROP]]` (13 ADIv6 invariants lost their only anchor and vanished with no
record) and `PROSE-NAME-CELL-DECLARATION.3`, which this leaf **unblocks**.

## `.0` — census result (`2026-09-12`)

Producer: `python3 scripts/measure_untabled_signal_declarations.py`. Read-only.

Two passes mint a declaration with no `table_signal_declaration_provenance` record: the
relation→declaration path and the prose appositive path. **Measure what they are worth before
measuring what they cost**, because a rule here can delete a document's only signals:

| stratum | declarations | duplicate of a table declaration | **sole source** of that signal | spelled like an ordinary word | resolves to an actor role |
| --- | ---: | ---: | ---: | ---: | ---: |
| current | 53 | 26 | **24** | 3 | 1 |
| legacy | 373 | 199 | 174 | 0 | 0 |

**The path is load-bearing, not incidental.** For I2C (10) and I2S (2) it is the *only* source of any
signal — those documents declare none from tables — and it supplies 12 real SWD/JTAG wires in ADIv6
(`SWCLK`, `SWDIO`, `TDI`, `TDO`, `nSRST`, `nSRSTOUT`, `DBGTDO`, `PORTCONNECTED`, `CSYSPWRUPACK`, …).
Any rule that costs those is worse than the defect.

### The proposed discriminator is the wrong one

The tree proposed refusing a name the built-in actor taxonomy resolves. Measured:

- it selects **1** declaration corpus-wide — AHB's `Manager`;
- it costs **0**: no table-declared name anywhere in either stratum resolves to a role;
- but the current stratum holds **3** phantoms, and it catches one of them.

The other two are AHB `Reset` — a signal *function*, where the document's real reset is `HRESETn` —
and ADIv6 `In`, an English preposition. Neither is an actor.

### What all three share is orthography, not meaning

`Manager`, `Reset`, `In`: an initial capital over an all lower-case remainder. That is how English prose
spells a word and not how any document in this corpus spells a wire:

| population | members | spelled `Initial-capital + all lower-case` |
| --- | ---: | ---: |
| untabled declarations, current | 53 | **3** — exactly the three phantoms |
| untabled declarations, legacy | 373 | **0** |
| distinct table-declared names, both strata | 1,605 | **0** |

It is orthography, not vocabulary (ADR 0006): the test cannot tell what `Manager` means, only that it
is spelled like a word. **State the limit plainly**: nothing forbids a document from naming a wire
`Clk`. Nothing in this corpus does — 0 of 1,605 — but that is a fact about the corpus, not a law. What
makes it safe to act on is containment: the rule applies only where a name was *inferred*, never to a
table declaration, where the document's own spelling is authoritative and this reader has no business
overruling it.

## `.1` — result (`2026-09-12`)

`inferred_name_is_an_ordinary_word` at three sites: the relation→declaration path and both prose
appositive forms. **Not** at `synthesize_signal_declarations`, and that omission is the safety argument
rather than an oversight — a table declaration is the document's own spelling.

### Corpus effect, measured on rebuilt artifacts

Two documents were affected, and both were rebuilt rather than left stale.

**AHB** — `Manager` and `Reset` removed; table declarations unchanged at 79; **interfaces unchanged at
42**, so neither phantom was holding a real grouping together. And a recovery nobody predicted:
**conditional rules 29 → 37**, with all 29 pre-existing rules byte-identical. All 8 new rules carry
`Manager` in their `source_text` — sentences such as *"During a waited transfer, the Manager is
permitted to change the transfer type…"* — and their `consequent_signal` now resolves to the real
`HTRANS` and `HRDATA`. While `Manager` was a declared signal those sentences resolved to the phantom
and the rule was lost. **Removing a phantom recovered eight real requirements.**

**ADIv6** — `In` removed; interfaces **21 → 14**, and every one of the 7 lost is an `…in…` grouping
built on the phantom. Of the 57 constraints that moved, **44 keep their id** and shed only the
phantom-bearing entries from `related_interface_ids`, keeping their real ones. `PORTCONNECTED` stays
declared; it loses its only grouping, which was a pairing with the phantom and therefore never an
interface.

### The cost, stated rather than buried

**13 invariants vanish**, and nothing records it. Their ids show why they were exposed —
`constraint_invariant_in_the_capture_dr_state_…`, `…in_the_update_dr_state_…` — these are real JTAG
requirements whose sentences *begin with the word "In"*, which is what minted the phantom from them.
They were anchored to nothing else, so they left with it.

Shipped anyway, and the reasoning is on the record: an invariant anchored to a wire that does not exist
was never correctly anchored, and asking a downstream consumer to synthesise a port called `In` is worse
than a recorded gap. The gap is now recorded as `[[ANCHORLESS-INVARIANT-DROP]]`, which also notes that
this contradicts `SPEC-TO-INTENT-ALIGNMENT`'s "zero unexplained drops" — that claim is scoped to a
reviewed population, and the mechanism has no accounting at all.

## Decisions

- `2026-09-12` — **opened as its own tree rather than a leaf of `PROSE-NAME-CELL-DECLARATION`.** That
  tree is about what a table **row** declares; this is about what a **relation** declares. They meet
  only because a width guard made the symptom visible, and the sibling tree's frontier is already four
  leaves deep in a different mechanism.
- `2026-09-12` — **the tree's own proposed discriminator was replaced by measurement.** It asked for an
  actor-taxonomy refusal; that catches 1 of the 3 phantoms. The orthographic test catches 3 of 3 and
  costs 0 of 1,605 table-declared names. Recorded rather than silently swapped, because the actor-role
  idea is the obvious one and the next reader will have it too.
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

- `2026-09-12` — `.1`. Controls, **observed RED**: `a_relation_naming_an_ordinary_word_declares_nothing`
  yields `["Signal Manager is output.", "Signal Reset is output.", "Signal SWCLK is output."]` without
  the guard and `["Signal SWCLK is output."]` with it — the wire this path is the only source of
  survives while both phantoms go. `an_inferred_name_spelled_like_a_word_is_not_a_wire` pins the shape
  over **every** distinct sole-source name the census found, not a sample, plus the table shapes the
  test must never reach (`HRESETn`, `qactive_cg`, `AMEVCNTRn_EL0`, `PSELx`, a single letter).
  `cargo test` 472 / 168 / **1421** / 4 green; fmt and clippy `-D warnings` green.
  Chain: AHB rebuilt via the preserved bundle (byte-identical before and after, retention still the
  declared 24) and ADIv6 rebuilt directly from its retained bundle, both in the interleaved order with
  exactly one validate per artifact; `scripts/check_chain_currency.sh` current afterwards.
- `2026-09-12` — `.0`. `python3 scripts/measure_untabled_signal_declarations.py` over all 78 persisted
  EvidenceIR artifacts. Read-only: nothing written, rebuilt or mutated. The taxonomy mirror is
  `builtin_actor_taxonomy_role_in_text` transcribed with its `normalize_actor_term` /
  `normalized_text_contains_term` whole-token semantics, so `HMASTER` does **not** resolve to
  `master` — checked, and it is why the role test selects 1 rather than dozens.

## Commit Log

- Opened in the commit that deferred `PROSE-NAME-CELL-DECLARATION.3` (`3df80f4e`).
- `.0` — `ACTOR-NOUN-RELATION-DECLARATION.0` (`792ad79b`).
- `.1` — `ACTOR-NOUN-RELATION-DECLARATION.1`.

## Changelog

- `2026-09-12` — `.1` closed. `inferred_name_is_an_ordinary_word` at three inference sites removes all
  three phantoms. AHB: interfaces unchanged, **+8 real conditional rules** recovered. ADIv6: 7 phantom
  interfaces gone, 44 constraints keep their id and shed only phantom anchors, **13 invariants vanish
  unrecorded** — tracked as `[[ANCHORLESS-INVARIANT-DROP]]`. Unblocks `PROSE-NAME-CELL-DECLARATION.3`.
- `2026-09-12` — `.0` closed. The path is the sole source of 24 current signals (all of I2C's and
  I2S's), so a careless rule is worse than the defect. The proposed actor-role test catches 1 of 3
  phantoms; an orthographic test catches 3 of 3 at zero measured cost. `.1` opened on the latter.
- `2026-09-12` — tree created. A width guard measured in `PROSE-NAME-CELL-DECLARATION.3` moved AHB's
  IntentIR interface count from 42 to 62 by making an existing phantom declaration well-formed; the
  phantom is the actor role `Manager`, minted as a wire by the relation→declaration path.
