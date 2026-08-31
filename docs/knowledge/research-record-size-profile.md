---
id: research-record-size-profile
title: Research-record line ceiling fits the population; an oversized record is a composite
answers:
  - "what is the line-size profile of SpecForge research records"
  - "how large is the average docs/research record"
  - "is the 640-line research record ceiling correctly calibrated"
  - "should an oversized research record be split or should its ceiling be raised"
  - "why was the production-genericity pipeline audit partitioned"
  - "where are the .6d.ii per-leaf qualification results"
  - "where did the .6d.ii.e.v.iii information-flow result move to"
  - "where is the .6d.ii.f behavioral signoff recorded"
  - "which research record is closest to its line ceiling"
  - "does docs/research have a file-count ceiling"
  - "how do I prove a live-document partition is lossless"
  - "what happens to section deep-links when I partition a record (they break unless the retained record keeps the moved headings as redirects; SECTION-ANCHORS gates it, and links inside sealed archive segments can only ever be repaired at the target end)"
  - "is losslessness enough when partitioning a canonical record (no — .4e proved a byte-exact partition can still break 14 section anchors while every doctrine passes)"
  - "what is the remedy when a research record reaches its per-file line ceiling"
  - "does a research record with a live writer need a rollover instead of a partition"
  - "how do I decide between partitioning a research record and declaring a rollover for it"
date: 2026-08-31
status: current
tags: [documentation, containment, doctrine, research, genericity]
evidence: scripts/check_section_anchors.pl; doctrine/live_document_size/surfaces.jsonl; docs/research/production-genericity-pipeline-audit.md; docs/research/production-genericity-qualification-results.md; docs/research/generic-enum-conflation-results.md; docs/catalogs/research-records.md; docs/tasks/LIVE-DOCUMENT-PRESSURE-HEADROOM.md; docs/decisions/0045-task-plane-cardinality-is-removed-behind-a-declared-exemption.md; scripts/check_live_document_size.sh
reverify: find docs/research -name '*.md' -exec wc -l {} \; | awk '{print $1}' | sort -n | awk '{a[NR]=$1; s+=$1} END {printf "n=%d mean=%.1f median=%d p95=%d max=%d\n", NR, s/NR, a[int(NR*0.5)], a[int(NR*0.95)], a[NR]}'; bash scripts/check_live_document_size.sh 2>&1 | grep research_records
---

# Research-record line ceiling fits the population; an oversized record is a composite

The `research_records` surface (`docs/research/*.md` plus `docs/research/**/*.md`) carries a per-file
`lines_each` bound of **640** in both `health_targets` and `enforcement_ceilings` — health equals ceiling, so
this axis has **no warning band**: a record reaching 640 is refused outright, including for a one-line
correction. `LIVE-DOCUMENT-PRESSURE-HEADROOM.4a` removed the surface's `files` count behind an ADR 0045
`cardinality_exemption`, so the collection is unbounded in membership; the per-file line bound was untouched
by that exemption and is the axis that still stops work.

Measured `2026-08-31` over 63 members: **mean 172 lines, median 130, p75 194, p90 297, p95 372, max 639**, and
only **two** records exceed 80% of the ceiling. The bound therefore fits the population, and re-deriving the
per-file profile to accommodate a single outlier would be a bound raised to quiet one warning. Aggregates are
far from binding at `lines_total` 10,840/40,960 (26.5%) and `bytes_total` 740,829/4,194,304 (17.7%).

**The diagnosis for an oversized record is composition, not length.** When
`production-genericity-pipeline-audit.md` reached 639/640, the cause was that it held two documents: a
genericity audit, plus a per-leaf qualification chronology that thirteen `SPEC-TO-INTENT-ALIGNMENT.6d.ii`
leaves had appended to it over three weeks. Splitting at that seam retires the growth driver; raising the
ceiling only postpones it. The surface's `partitioned_canonical` lifecycle makes the split the legal remedy,
and `LIVE-DOCUMENT-PRESSURE-HEADROOM.4c` performed it: the audit is 467 lines and keeps its question,
boundary, denominator, method, confirmed violations, source disposition, forbidden-vocabulary rebuttal,
required signoff architecture, historical correction, and exit criteria, plus a `Qualification outcome`
section stating the composed verdict by derivation.

The moved material is [`production-genericity-qualification-results.md`](../research/production-genericity-qualification-results.md):
the `.6d.ii.d.iv` combined identity remediation, the `.e.i` proof-architecture freeze, the `.e.ii`
core/conformance boundary, the `.e.iii` sealed capability substrate, the `.e.iv.i` rule/seam census, the
`.e.v.ii` compiled production syntax graph, the `.e.v.iii` whole-production information-flow result, the
`.e.iv.vii` proof-identity result, the `.f.iii` held-out diagnostic, and the `.f.iii.a`/`.f.iv`/`.f.v`
behavioral and population signoff. They appear in their **original appended order, not strict leaf order** —
`.e.iv.vii` follows `.e.v.iii` because that is where it was written, and reordering evidence is not lossless.
The per-chain ledger and residual attribution remain in `production-genericity-structural-qualification.md`.

**Prove a partition lossless against `git HEAD`, not by reading the diff.** Three mechanical checks together
are sufficient and were what `.4c` ran: each moved block occurs byte-identically *and in order* inside the
destination; the retained prefix and suffix of the source are byte-identical to the original's; and the
multiset difference of original lines minus (new source ∪ destination) is empty. Inspect inbound routes
separately — anchor deep-links and intra-document back-references ("the findings above") are what a
content-preserving move can still break.

**Routes are now gated, and the repair goes at the TARGET end.** `LIVE-DOCUMENT-PRESSURE-HEADROOM.4e` proved
by counterexample that author care is not enough: it repointed the current-facing citations, argued the rest
were dated history that a pointer section would route "one hop", passed all 12 doctrines, and still took the
repository from **20 resolving / 0 unresolved** section anchors to **13 / 14**. Zero unresolved was the
standing invariant, so that was a regression, not a policy. `.4f` repaired it and registered
`SECTION-ANCHORS` (`scripts/check_section_anchors.pl`) so a partition cannot break a route silently again.
Repair at the source end is not always possible — seven of those fourteen links were inside sealed
`archive_terminal` ledger segments the rollover doctrine forbids editing — so **a partitioned record must keep
every cited heading as a redirect**, which repairs all inbound links at once and rewrites no dated entry.

A split relocates the maximum rather than removing it, so the successor must be inspected for a **live
writer**. `production-genericity-qualification-results.md` has none; the audit's `.6d.ii` chronology is closed.
The next-largest record did: `generic-enum-conflation-measurement.md` reached 559/640 (87.3%) while
`KG-ISF-COMPLETENESS.5` was still appending. `LIVE-DOCUMENT-PRESSURE-HEADROOM.4e` partitioned it on the same
seam into [`generic-enum-conflation-results.md`](../research/generic-enum-conflation-results.md).

**A live writer does not by itself change the remedy — the size of the remaining writer SET does.** Measure it
before choosing. Partition is the right answer when the residual writers fit inside the successor's band with
margin; a declared rollover is what an *unbounded* writer set needs. Here the `.5` lane had exactly one
unwritten leaf left (`.5.iv.a`, the deferred CODE slice; `.5` and `.5.i`-`.5.iv` are all done) against a
measured append distribution of mean 68 / max 105 lines over six appends — so one worst-case append lands the
428-line successor at 533/640 (83%), inside the band, and the lane then closes. Had the writer set been open,
partition would only have restarted the countdown. After it, the surface maximum is the already-partitioned
audit at 467/640 (73.0%) and the research line warning clears.
