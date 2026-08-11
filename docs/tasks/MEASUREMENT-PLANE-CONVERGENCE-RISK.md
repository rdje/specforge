# MEASUREMENT-PLANE-CONVERGENCE-RISK: can we tell whether SpecForge is converging?

## Metadata

- Tree ID: `MEASUREMENT-PLANE-CONVERGENCE-RISK`
- Status: `superseded` by [`SIGNOFF-BURNDOWN`](SIGNOFF-BURNDOWN.md) (`2026-08-11`, same day)
- Roadmap lane: cross-cutting (`R15e` evaluation / `R9` extraction breadth)
- Created: `2026-08-11`
- Owner: repo-local workflow
- Raised by: the director, mid-`SIGNAL-CATALOG-CAPTURE-GAP.1` — *"is SpecForge's spine, backbone, structure
  robust? I am starting to wonder if we'll converge towards SpecForge's objectives."*

## Superseded the same day — read `SIGNOFF-BURNDOWN` instead

This tree named the right symptom and the **wrong cause**. It concluded that SpecForge cannot measure whether
a document was understood. Running all 11 eval datasets hours later showed the instrument exists, is
rigorous, and reads **1.000 on every filtered `WIRE-BASED-100` surface but one**. The instrument is not
missing — it is *saturated*, because gold was sized to seed the method and never rescaled to the artifact it
now judges (AXI: 13 gold facts against 502 emitted constructs). A pegged gauge steers no better than no
gauge, which is why effort drifted to the unbounded breadth and governance lanes.

The corrected diagnosis and the falsifiable recovery plan live in
[`SIGNOFF-BURNDOWN`](SIGNOFF-BURNDOWN.md). Everything below is kept unedited as the honest record of the
first pass; the gold-coverage measurement in it remains correct and is reused there.

## Why this tree exists

This is a `proposed` tree, not active work. It exists so a strategic concern raised in conversation survives
in a durable layer with the evidence that provoked it, rather than living only in a transcript. It does not
claim the concern is correct; it makes it falsifiable.

## The concern, stated precisely

The worry is usually voiced as "is the architecture right?" The evidence gathered in
`SIGNAL-CATALOG-CAPTURE-GAP.1` suggests the architecture is *not* the risk, and points somewhere narrower:

**SpecForge can measure that a stage ran, and cannot measure whether a document was understood.**

Everything in the project's doctrine says measurement-led — the roadmap says it, every tree carries a
verification log, eight doctrines gate it. But the supervised ground truth the project actually owns is:

| Gold task | Facts |
| --- | ---: |
| `signal_constraint` | 25 |
| `actor_signal_relation` | 21 |
| `protocol_state` | 13 |
| `temporal_rule` | 12 |
| `serial_frame_field` | 11 |
| `register_field` | 5 |
| `swd_operation` | 4 |
| **`declared_signal`** | **1** |
| `interface_edge_timing` | 1 |
| **total** | **93, across ~8 of 78 documents** |

(`crates/specforge/test_data/llm_eval/*.json`, 11 seed datasets, measured `2026-08-11`.)

The single load-bearing extraction in the whole pipeline has **one** gold example. Since
`SEMANTIC-EMPTY-CATALOG-FILTER` — correctly — made the grounding filter refuse any record naming an
undeclared signal, the causal chain is:

```text
declared signals → grounded constraints/rules → IntentIR → .isf
```

Every downstream number is conditioned on the first link, and the first link's recall is measured against
gold on one fact, in one document, for a corpus of 78.

## The consequence

On the 70 documents with no gold, **"honest absence" is unfalsifiable by machine.** A document that declares
nothing and a document whose declarations we cannot read produce the same artifacts, the same green gates, and
the same clean CI.

`SIGNAL-CATALOG-CAPTURE-GAP` is the worked example, and it cuts both ways:

- `.0` read five `protocol`-classed documents as prima-facie capture misses.
- `.1` established that four are honest absence and one is real.
- It took a full slice of hand-built probes to learn that. **The answer could equally have been five real
  misses, and nothing in the gate set would have said so.**

That is the actual convergence risk. Not that the IRs are wrong — that effort is being allocated without an
instrument that says whether it is working. It also explains the felt contradiction the director named: 130+
task trees can each be individually green while the top-line objective feels stalled, because the trees
measure *delivery* and nothing measures *understanding per document*.

## What the same evidence says is NOT the problem

- **The typed staged IR spine holds up.** `.1` traced a defect from a demoted `SemanticIR` record → the
  `EvidenceIR` statement → its evidence span → a byte offset in a normalized markdown file, and cross-checked
  it against `SourceIR`'s typed elements. Four stages, provenance intact, mechanism recovered in one sitting.
  A pipeline that could not do that would not be repairable at all.
- **Fail-closed grounding is right and should not be relaxed.** `dense-prose-false-signal-loop-reaches-isf`
  is what happens when the bar drops: syntactically valid, semantically fabricated `.isf`.
- **The doctrine machinery works.** In this one slice its gates caught an unregistered research record, three
  stale derived projections, and a mandatory ledger rollover — all real, none noticed by the author first.

## Candidate leaves (none accepted yet)

| Leaf | Status | Scope |
| --- | --- | --- |
| `.0` | `proposed` | promote the `.1` presence probes from a one-off script into a tracked, per-document **declaration-presence oracle**: for every document, does a declaration-bearing modality exist that produced no catalog? Corpus-wide, mechanical, no gold required |
| `.1` | `proposed` | make that oracle a doctrine surface, so a new capture miss is reported the day it appears rather than the day someone goes looking |
| `.2` | `proposed` | grow `declared_signal` gold beyond one fact — enough documents to calibrate the oracle's precision, not to replace it |
| `.3` | `proposed` | re-express the roadmap's completeness lane in terms of *documents understood*, not stages run |

The point of `.0` is that it is **affordable**: it needs no answer key, because it asks a presence question
about the source, not a correctness question about the output. Gold sets do not scale to 78 documents; a
structural presence oracle does.

## Non-goals

- Not a rewrite proposal. Nothing here argues for changing the IR spine, the stage boundaries, or `.isf` as
  the product boundary.
- Not a case for relaxing grounding. The measured cost of a low bar is already recorded.
- Not a claim that the 130+ existing trees were misallocated — only that nothing currently ranks them.

## Open Questions

- Is a presence oracle honest enough to gate on, given that `.1` needed human reading of every probe hit to
  separate a wire heading from a glossary acronym? Perhaps it should *report*, not gate.
- The recall gauge (`COMPLETENESS-RECALL-GAUGE`, Lincoln–Petersen over `FactKind::SignalConstraint`) already
  estimates unseen facts, and its own tree records that its two tiers share prose input and are therefore
  optimistic. Should the oracle extend it, or stay independent?

## Blockers

- None. This tree blocks nothing and is blocked by nothing. It should be accepted, reshaped, or closed by an
  explicit decision rather than left `proposed` indefinitely.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-11` | — | counted every gold fact in `crates/specforge/test_data/llm_eval/*.json` by task | 93 facts, 11 seed datasets, ~8 documents; `declared_signal` = 1 |
| `2026-08-11` | — | crossed that with the empty-catalog population | 33 of 78 documents produce zero declared signals; only one is a capture miss, established by hand in `SIGNAL-CATALOG-CAPTURE-GAP.1`, not by any gate |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| — | `MEASUREMENT-PLANE-CONVERGENCE-RISK — record the convergence concern with its evidence` | ownership only, no code, no accepted leaf |

## Changelog

- `2026-08-11`: created to hold a director-raised strategic concern in a durable layer, with the gold-coverage
  measurement that makes it checkable. `proposed`; it does not enter any frontier until accepted.
