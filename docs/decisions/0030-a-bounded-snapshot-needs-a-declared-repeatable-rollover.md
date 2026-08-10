---
id: a-bounded-snapshot-needs-a-declared-repeatable-rollover
title: A bounded snapshot needs a declared repeatable rollover, not a one-time migration
date: 2026-08-11
status: accepted
scope: documentation, containment, live-document-size, roadmap, archive, retrieval
evidence: docs/tasks/LIVE-DOC-STOP-RISK.md; scripts/check_roadmap_projection_contract.pl (rollover_schema_errors, rollover_content_errors, 31 self-test cases); doctrine/live_document_size/roadmap_projection.json; doctrine/live_document_size/surfaces.jsonl; docs/archive/roadmap/INDEX.md
answers:
  - "what is ADR 0030"
  - "how do I roll ROADMAP.md when it approaches its ceiling"
  - "where does retired roadmap direction go"
  - "why does the roadmap archive hold more than one capsule"
  - "what stops a bounded snapshot from accreting chronology"
  - "how is a roadmap rollover capsule verified"
---

# ADR 0030: A bounded snapshot needs a declared repeatable rollover, not a one-time migration

## Context

[ADR 0010](0010-bounded-current-roadmap-and-exact-history.md) contained `ROADMAP.md` by sealing the
1,487-line pre-containment monolith into one immutable capsule and replacing the stable path with a
153-line bounded root. That migration was correct and it worked — for three days.

Measured on `2026-08-11`, before this change:

| Dimension | Actual | Health target | Enforcement ceiling | Pressure |
| --- | ---: | ---: | ---: | --- |
| Lines | 364 | 256 | 384 | 142.2% of health, **20 lines from the wall** |
| Bytes | 34,938 | 32,768 | 49,152 | 106.6% of health |
| Max line bytes | 205 | 384 | 512 | 53.4% — healthy |

Two things were wrong, and only one of them was the size.

**The growth had a single shape.** Of 364 lines, 213 were one bullet under `Current strategic priorities`
that had grown by one sentence per closed task-tree leaf — `.8c` refused closure, `.10b.ii` copied the
source, `.2.43` closed the 15-page OpenCAPI Ready Definition. Nine lines further down, the same file says
it "does not mirror leaf frontiers or delivery chronology." The `bounded_snapshot` lifecycle forbids
embedded chronology, and the contract's `forbidden_literals` were supposed to enforce it — but they name
three legacy shapes (`### Applied task trees since`, `- done:`, `- **Progress (`). Chronology written as
ordinary prose inside a bullet matched none of them.

**There was no declared way to fix it.** ADR 0010 modelled the archive as one sealed pre-migration source,
so the contract schema held exactly one capsule and the archive index said Git preserves everything after
it. When the bounded root filled a second time, the compliant options were to hand-delete direction (losing
it to Git object reachability, which the doctrine's version-object rule treats as conditional retention,
not an archive) or to widen the ceiling. Neither is a rollover. This is the shape
[ADR 0029](0029-fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy.md) named:
*a bound a surface can actually reach must have a remedy compliant work can take.* ADR 0029 fixed it for
the fact plane; `ROADMAP.md` is the same defect on a `bounded_snapshot`.

## Decision

**1. The roadmap archive is a series, not a single capsule.** A rollover seals the exact bytes of the
bounded root it retires as a new immutable capsule under `docs/archive/roadmap/root-through-<date>.md`,
then rewrites the root. The pre-containment `source-through-2026-08-08.md` capsule keeps its distinct
identity and role; it is a different product and may never be reused as a rollover target.

**2. The operation is declared in data and re-proved on every gate run.** Each capsule declares its id,
path, SHA-256, lines, bytes, max line bytes, sealed date, and reason in
`doctrine/live_document_size/roadmap_projection.json`. `check_roadmap_projection_contract.pl` re-reads every
capsule and fails on a missing file, a metric mismatch, a digest mismatch, or a capsule that the bounded
archive index does not link. Recovering retired direction therefore never depends on Git object
reachability.

**3. A rollover capsule must have been legal when it was sealed.** Every declared capsule is checked
against the *current root's* enforcement ceilings. A capsule above them would mean the surface breached its
own bound before the rollover, not that the rollover rescued it — so that is a gate failure, and the
gate's silence is evidence the surface never actually overflowed.

**4. The series is itself bounded, reported, and remedied.** `rollover_policy` declares
`max_capsules: 16`, `warning_capsules: 12`, and the remedy in the same record. The checker fails above the
maximum and prints the count plus the named remedy at or above the warning. This matters because
`archive_terminal` surfaces are exempt from the live-document milestone report by design — immutable
history has no per-file remedy — so a growing terminal collection would otherwise be silent until its hard
ceiling. Declaring the signal where the remedy is declared keeps the new surface from being an instance of
the defect this tree exists to remove.

**5. The bound is derived, not chosen.** Per ADR 0029 decision 1, the `roadmap_root_rollovers` aggregates
are the file bound times the per-file bound: 16 × 384 = 6,144 lines and 16 × 49,152 = 786,432 bytes. Each
capsule's per-file bound is the current root's ceiling, because that is exactly the largest thing a
rollover can retire.

**6. The retired content is proved duplicate before it leaves, not after.** The 213 chronology lines
narrate leaves whose canonical homes are `docs/tasks/`, `CHANGES.md`, and the decision records. They are
removed from the live root because the root's lifecycle forbids them, and retained byte-exact in the
capsule because a byte-exact copy is cheaper and more honest than a 213-line duplicate proof.

## Consequences

- `ROADMAP.md` is 156 lines / 12,394 bytes — 60.9% of its line health target and 40.6% of its ceiling. The
  `142.2% of health` warning is gone, and ordinary status edits have ~100 lines of room before warning.
- No ceiling, health target, or milestone moved. The surface came back under its bounds by rolling, which
  is what the doctrine asks for and what was not previously possible.
- The next rollover is a four-step transaction with no design work left: copy the root to a dated capsule,
  append one contract record, add one archive-index row, rewrite the root. The gate proves all four.
- `docs/archive/roadmap/root-through-2026-08-11.md` holds the exact 364-line / 34,938-byte root at SHA-256
  `2cc2fa4a39ff7a654ec0c24ec59a715d016bb0625a4846a9e583afc94fb77190`.
- The contract checker grows from 9 to 31 self-test cases; 22 cover rollover schema and identity, including
  duplicate ids and capsules, pre-migration capsule reuse, non-chronological seal dates, unsafe and
  non-Markdown paths, over-cap counts, a capsule above the current-root ceiling, a rollover declared in
  `planned` state, and every capsule identity mismatch.
- **This ADR does not fix the accretion itself.** A future session can still write chronology into the root
  one sentence at a time; it will simply hit a wall it can now leave. `LIVE-DOC-STOP-RISK.0a` owns making
  the growing section fail closed at its own bound, so the signal names the section rather than the file.

## Links

- Task tree: [`LIVE-DOC-STOP-RISK`](../tasks/LIVE-DOC-STOP-RISK.md)
- Extends: [ADR 0010](0010-bounded-current-roadmap-and-exact-history.md)
- Applies the general rule from:
  [ADR 0029](0029-fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy.md)
- Made measurable by: [ADR 0028](0028-a-file-locator-means-one-file-and-aggregate-ceilings-must-warn.md)
