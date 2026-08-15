---
id: workflow-standard-capacity-is-rederived-from-explicit-member-growth
title: Workflow-standard capacity is re-derived from explicit member growth
date: 2026-08-15
status: accepted
scope: workflow, documentation, capacity, catalogs, containment, continuity
evidence: docs/tasks/CLAIM-VERIFICATION-ADOPTION.md; scripts/measure_workflow_standard_capacity.pl; doctrine/live_document_size/surfaces.jsonl; docs/catalogs/workflow-standards.md
reverify: "perl scripts/measure_workflow_standard_capacity.pl --check; perl scripts/check_canonical_collection_catalogs.pl --check; perl scripts/check_live_document_size.pl --report"
answers:
  - "what is ADR 0043"
  - "why does workflow-standard capacity become 21"
  - "how is workflow-standard file capacity derived"
  - "why do workflow standards remain an explicit target list"
  - "what is the measured peak day for workflow standards"
  - "does the workflow catalog fit the full 21-file profile"
  - "when must workflow-standard capacity be measured again"
---

# ADR 0043: Workflow-standard capacity is re-derived from explicit member growth

## Context

The claim-verification contract necessarily adds two canonical workflow documents: the root standard and the
pull-request review template. The collection moves from 12/16 to 14/16 files, or 87.5%, leaving two slots and
crossing its warning milestone. No content member is individually near its bound: the largest remains 478/700
lines and 33,076/65,536 bytes. The warning is collection growth, not member shape.

Git addition history for the 14 explicit current members is closed and reproducible:

| Active date | Added members |
| --- | ---: |
| 2026-04-01 | 2 |
| 2026-05-31 | 1 |
| 2026-06-02 | 4 |
| 2026-06-22 | 2 |
| 2026-08-08 | 3 |
| 2026-08-15 | 2 |

The measured peak active day is four. `scripts/measure_workflow_standard_capacity.pl` derives this directly from
the surface's explicit targets and each tracked path's Git creation history; five focused cases pin strict
milestone inequality, hard-cap refusal, and aggregate arithmetic.

The existing topology is intentional. Workflow standards live at stable root, `.github/`, and `docs/` paths and
are enumerated so an unreviewed Markdown file cannot silently become policy. A generated external-membership
catalog already provides one-hop browsing. At 21 members it is 31 lines; even treating every future row as the
512-byte maximum yields 11,138 bytes, far below the catalog's 384-line / 65,536-byte per-file ceilings. The
surface registry permits 32 target-array items, so the explicit topology does not bind at the selected profile.

## Decision

### 1. Re-run the established warning/rollover equation

Choose the smallest integer file bound for which the current population is strictly below the 80% warning and
one measured peak active day is strictly below the 90% rollover:

- `14 / 21 = 66.7%`;
- `(14 + 4) / 21 = 85.7%`;
- 20 is insufficient because `18 / 20 = 90.0%`, exactly the rollover threshold.

Twenty-one is the selected capacity. It is not permission to spend the registry's full 32-item portable cap.
When the generic live-size gate next reports count warning, re-run the population and Git-growth census.

### 2. Preserve explicit membership and stable paths

No workflow document moves, merges, or changes identity. The `targets` array continues to enumerate actual
members; adding a standard remains a reviewed registry edit followed by catalog regeneration. A glob would make
policy membership implicit, while a routed topology would add navigation and migration cost before either the
registry array or catalog approaches a structural limit.

### 3. Keep aggregate capacity reachable

Per-file limits remain 700 lines, 65,536 bytes, and 1,024 line bytes. Both health and ceiling aggregates remain
exactly `files × per-file`: 14,700 lines and 1,376,256 bytes. A collection of individually legal standards can
therefore never be refused by a tighter total.

### 4. Apply and retire one exact authority

`CLAIM-VERIFICATION-ADOPTION.1a` carries one exact 16→21 ceiling-increase record and synchronizes containment,
catalog, decision, book, task, and claim evidence. `.1b` removes that record immediately after the profile is
committed; it cannot remain reusable authority.

## Consequences

- The claim-verification checker can be implemented without consuming the workflow collection's last two slots.
- Workflow membership remains explicit, reviewable, and browsable at stable paths.
- The profile is derived from authority/lifecycle and measured growth, not from the subject of any standard.
- The measurement script is a tracked reproducer; generic live-size warning remains the stale-state signal that
  triggers a new profile decision rather than recalculating capacity on every ordinary member addition.

## Links

- Task tree: [`CLAIM-VERIFICATION-ADOPTION`](../tasks/CLAIM-VERIFICATION-ADOPTION.md)
- Claim standard: [`CLAIM_VERIFICATION.md`](../../CLAIM_VERIFICATION.md)
- Aggregate reachability: [ADR 0032](0032-no-collection-may-declare-an-aggregate-below-its-own-legal-maximum.md)
- Analogous measured capacity law: [ADR 0041](0041-decision-capacity-is-rederived-without-moving-stable-records.md)
