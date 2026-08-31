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
  - "what is the remedy when a research record reaches its per-file line ceiling"
date: 2026-08-31
status: current
tags: [documentation, containment, doctrine, research, genericity]
evidence: doctrine/live_document_size/surfaces.jsonl; docs/research/production-genericity-pipeline-audit.md; docs/research/production-genericity-qualification-results.md; docs/catalogs/research-records.md; docs/tasks/LIVE-DOCUMENT-PRESSURE-HEADROOM.md; docs/decisions/0045-task-plane-cardinality-is-removed-behind-a-declared-exemption.md; scripts/check_live_document_size.sh
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

A split relocates the maximum rather than removing it, so the successor must be inspected for a **live
writer**. Here it is `generic-enum-conflation-measurement.md` at 559/640 (87.3%), still being appended to by
the active `KG-ISF-COMPLETENESS.5` lane; `LIVE-DOCUMENT-PRESSURE-HEADROOM.4e` owns it.
