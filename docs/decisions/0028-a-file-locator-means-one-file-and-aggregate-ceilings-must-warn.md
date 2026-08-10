---
id: a-file-locator-means-one-file-and-aggregate-ceilings-must-warn
title: A file locator means one file, so multi-file surfaces stopped being blind to their own aggregate ceilings
date: 2026-08-10
status: accepted
scope: documentation, containment, live-document-size, pressure, continuity
evidence: docs/tasks/FACT-CARD-CAPACITY-HEADROOM.md; scripts/check_live_document_size.pl (validate_limits, the single-file aggregate exemption and the locator cardinality rule); doctrine/live_document_size/surfaces.jsonl; docs/decisions/0026-fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards.md
---

# ADR 0028: A file locator means one file, so multi-file surfaces stopped being blind to their own aggregate ceilings

> This record deliberately carries no `answers:` block. It would consume one of the two remaining fact slots it
> is about. `FACT-CARD-CAPACITY-HEADROOM.3` restores headroom; the retrieval keys are added after it lands.

## Context

[ADR 0026](0026-fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards.md) enumerated what it
called every authority bounding the fact plane and found four. Measuring the plane again at the start of
`FACT-CARD-CAPACITY-HEADROOM.3` — after `.2` made the landing fixed-size — found a fifth, and it is tighter than
any of the four:

| Authority | Measured | Hard ceiling | Headroom | Warned first? |
| --- | ---: | ---: | ---: | --- |
| `knowledge_cards` `lines_total` | 9,773 | 10,000 | ~4.5 average cards | **no** |
| `max_facts` (shard contract) | 198 | 200 | 2 facts | n/a |
| `knowledge_cards` `files` → `max_cards` | 193 | 198 | 5 cards | yes, at 97.5% |
| `knowledge_cards` `bytes_total` | 853,926 | 1,048,576 | ~44 cards | **no** |

The census did not miss it through carelessness. **No warning could ever have surfaced it.**
`validate_limits` in `scripts/check_live_document_size.pl` enforces every declared aggregate ceiling as a hard
error, and separately reports warning/rollover milestones — but it skips the `lines_total` and `bytes_total`
milestones whenever the surface declares `locator: "file"`. That exemption is correct for a surface holding one
file, where `lines_total` merely repeats `lines_each` and warning on both is noise. It is wrong for a surface
whose `file` locator sits over a glob matching many files, and four surfaces do exactly that:

| Surface | Locator | Files matched | `lines_total` | Ceiling | Utilisation |
| --- | --- | ---: | ---: | ---: | ---: |
| `knowledge_cards` | `file` | 195 | 9,773 | 10,000 | **97.7%** |
| `task_evidence` | `file` | 130 | 29,818 | 40,000 | 74.5% |
| `decision_records` | `file` | 30 | 2,236 | 4,000 | 55.9% |
| `fsmgen_issue_packets` | `file` | 7 | 285 | 5,000 | 5.7% |

So an entire class of surfaces runs silent right up to a fail-closed error. Only one of the four is currently in
the region its milestones exist to announce — but that is the point: nobody could have known which, because the
answer was never reported. The other three are cheap reassurance obtained the same way the danger was, by
measuring instead of assuming.

A first pass over this class using shell-style globbing put `task_evidence` at 88.1%. That was wrong: the
matcher allowed `*` to cross a directory separator, so it counted the bounded `docs/tasks/corpus-coverage/`
parts as members of the root task surface. The checker's own `glob_regex` does not, and its measurement — 130
files, 74.5% — is the one recorded above. The correction is noted rather than quietly dropped because it is the
same failure mode as the finding: a convenient approximation of a surface's membership disagreeing with the
authority that enforces it.

One adjacent measurement came free and is worth recording, because it constrains how this class of correction
can be documented at all: `decision_records` holds 30 files against a hard ceiling of 32. Accepting this record
consumed one of the last three slots. The decision plane, like the fact plane, is nearly full, and `.3` must
re-derive it too.

How close this came to costing real work is measurable rather than rhetorical. Before `.2`, the landing
contributed 186 of those lines: `knowledge_cards` `lines_total` was **9,959 of 10,000 — 41 lines**, against an
average card of 50.1 lines. The next ordinary fact card would have broken a hard ceiling nobody could see
approaching, in whatever unrelated slice happened to write it. `.2` bought that headroom as a side effect of
fixing a different dimension, not by design.

## Decision

**1. The aggregate exemption is scoped to what it was actually reasoning about.** `lines_total` and
`bytes_total` milestones are suppressed only when the surface genuinely matches **one** file, tested against the
measured file count rather than the declared locator. A surface's own measurements decide whether the exemption
applies.

**2. A `file` locator must match exactly one path.** The registry vocabulary now means what it says: `file`
addresses one document, `collection` addresses many. A `file` locator over a multi-file match fails closed, so
this class of blindness cannot be reintroduced by a target that quietly grows.

**3. The four mislabelled surfaces are reclassified as collections.** No ceiling, health target, milestone,
lifecycle, index, or verifier changes with them. This is a correction of how each surface was described, not a
change to what it is allowed to hold — and it is what makes decisions 1 and 2 agree on real data.

**4. Capacity is still not re-derived here.** `.3` owns that, and it must now derive against `lines_total` and
`bytes_total` as first-class dimensions alongside the file count, the fact count, and the projection shape.

## Consequences

- `knowledge_cards` immediately begins reporting the pressure it was always under — rollover on `lines_total`
  (97.7%) and warning on `bytes_total` (81.4%). That is not a new problem; it is a formerly invisible one. The
  gate stays green because milestones warn and only ceilings fail.
- `.3`'s scope grows. Raising `max_cards` without raising the aggregate line ceiling would buy nothing: the plane
  would stop at roughly 197 cards on a dimension the earlier derivation never mentioned. It must also re-derive
  the 32-file decision ceiling, which has two slots left.
- The other three reclassified surfaces gain a permanent early-warning path they never had, at no cost today.
- The general lesson is recorded, not just the instance: a checker that enforces a bound it never warns about has
  no early-warning path at all, and a declared classification that disagrees with the measured shape will
  eventually be trusted over the measurement. Both are now mechanically prevented.

## Links

- Task tree: [`FACT-CARD-CAPACITY-HEADROOM`](../tasks/FACT-CARD-CAPACITY-HEADROOM.md)
- Corrects the census in: [ADR 0026](0026-fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards.md)
- Preceded by: [ADR 0027](0027-fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound.md)
- Containment doctrine: [ADR 0007](0007-live-document-containment-and-data-locality.md)
