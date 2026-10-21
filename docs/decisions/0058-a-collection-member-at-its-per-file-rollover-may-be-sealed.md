---
id: a-collection-member-at-its-per-file-rollover-may-be-sealed
title: A collection member that has reached its per-file rollover may be sealed; per-file growth milestones then measure the members that can still grow, and every ceiling still measures all of them
date: 2026-10-21
status: accepted
scope: live-document-size, decision-records, collections, milestones, doctrine
evidence: scripts/check_live_document_size.pl (validate_sealed_members; validate_limits' milestone_metrics); scripts/test_live_document_size.pl (the six sealed-member cases); doctrine/live_document_size/surfaces.jsonl (decision_records sealed_members); docs/tasks/live-document-pressure-headroom/warning-assignment.md (LIVE-DOCUMENT-PRESSURE-HEADROOM.8)
reverify: "perl scripts/test_live_document_size.pl --quiet && perl scripts/check_live_document_size.pl"
answers:
  - "what is ADR 0058"
  - "what is a sealed member of a live-document-size collection (sealed_members: path, sha256, owner)"
  - "why does decision_records no longer warn on bytes_each and lines_each (ADR 0038 is sealed)"
  - "can a new ADR be written while decision_records shows per-file pressure (yes: per-file pressure is one member's; a new ADR is a new file)"
  - "how is a sealed ADR amended (by a new record; an edit fails the seal)"
  - "which members may be sealed (only one at or past its per-file rollover, so sealing cannot silence a growing file)"
  - "what is LIVE-DOCUMENT-PRESSURE-HEADROOM.8"
---

# ADR 0058: A collection member at its per-file rollover may be sealed

## Context

A collection surface bounds each member (`lines_each`, `bytes_each`, `line_bytes_each`) and reports a warning at
80% and a required rollover at 90% of the health target. For a member that still takes writes, rollover means the
next content goes to a new member. Some members take no more writes at all. An accepted decision record is
amended by a new record, not rewritten: `DECISION-RECORD-CAPACITY-HEADROOM` recorded on `2026-08-15` that ADR 0038
"is accepted, stable, and no longer appendable", and that its per-file warning "remains honest".

The warning stayed. ADR 0038 sits at 474 of 512 lines (92.6%) and 31,833 of 32,768 bytes (97.1%), past rollover
on both, so `decision_records` reported a rollover on every run. That warning describes a file that cannot grow
and asks for a remedy that does not apply, which teaches a reader to ignore the surface. It also misled: on
`2026-10-21` three task records skipped writing a decision record because `decision_records` was "past its
rollover", although per-file pressure says nothing about adding a member. `LIVE-DOCUMENT-PRESSURE-HEADROOM.8` owns
the axis and asked for a remedy the surface can sustain, not a wider bound.

## Decision

1. A `collection` surface may declare `sealed_members`, a list of `{path, sha256, owner}`. A sealed member takes
   no further writes, and content that would have grown it goes to a new member. That is the rollover's own
   remedy, applied to a member that is complete.
2. `check_live_document_size.pl` (`validate_sealed_members`) refuses a sealed member that is not one of the
   surface's files or is sealed twice. It also refuses one whose content no longer hashes to its seal ("amend it
   with a new record"). It refuses one that has not reached its per-file rollover on some per-file dimension:
   sealing is not a way to silence a file still growing toward its band.
3. The per-file milestones (`*_each` warning and rollover) measure the unsealed members. Every enforcement
   ceiling, the file count and both aggregates still measure every member. A sealed member over a ceiling still
   fails.
4. `decision_records` seals ADR 0038 under `LIVE-DOCUMENT-PRESSURE-HEADROOM.8`. Its `bytes_each` and `lines_each`
   milestones now read the other members (largest 15,745 bytes, `INDEX.md`; 191 lines, ADR 0039).

## Consequences

- The surface reports only pressure someone can act on. ADR 0038's size is still enforced by every ceiling, and
  its content is now pinned: an edit fails closed instead of passing silently.
- Sealing is generic. Any collection whose members can be complete (accepted decisions, closed research records)
  may use it, member by member, once each reaches rollover. It is not an exemption by name: the checker derives
  eligibility from the member's own measure.
- A decision record is never blocked by another record's size. The collection's count axis keeps ADR 0041's
  population and peak-day re-derivation; this record is the 60th of 74 files, so that axis now warns, and
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.8.a` owns the re-derivation.
- Archiving ADR 0038 (`archive_terminal`) would move it off its stable path and out of the Knowledge Map's
  reading. Raising `bytes_each` would widen a bound for one file. Neither is needed.
