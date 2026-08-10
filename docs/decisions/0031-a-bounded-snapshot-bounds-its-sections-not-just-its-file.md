---
id: a-bounded-snapshot-bounds-its-sections-not-just-its-file
title: A bounded snapshot bounds its sections, not just its file
date: 2026-08-11
status: accepted
scope: documentation, containment, live-document-size, roadmap, pressure
evidence: docs/tasks/LIVE-DOC-STOP-RISK.md; scripts/check_roadmap_projection_contract.pl (section_limit_schema_errors, section_line_counts, section_limit_errors, section_pressure_warnings, 49 self-test cases); doctrine/live_document_size/roadmap_projection.json
answers:
  - "what is ADR 0031"
  - "why does ROADMAP.md have per-section line bounds"
  - "how are the roadmap section bounds derived"
  - "what happens when a roadmap section grows too long"
  - "why is a file-level line ceiling not enough for a bounded snapshot"
---

# ADR 0031: A bounded snapshot bounds its sections, not just its file

## Context

[ADR 0030](0030-a-bounded-snapshot-needs-a-declared-repeatable-rollover.md) gave `ROADMAP.md` a compliant
exit and recorded, in its own consequences, that it did not close the entrance: nothing bounds a section, so
the same accretion can start again and will simply reach a wall it can now leave.

That entrance is worth closing, because a file-level bound is a *bad detector* for this failure. The
observed accretion was 213 lines in one of seven sections. Measured against the file, that reads as
"`ROADMAP.md` is at 142.2% of health" — a number about the whole document that names no cause, arrives after
the fact, and points at no action. Measured against the section, the same growth reads as "`Current strategic
priorities` is at 82% of its 56-line bound; route per-leaf detail to its owning task tree." One of those a
future session can act on.

Measured section shape of the post-rollover root (`2026-08-11`, 156 lines total):

| Section | Lines | Changes when |
| --- | ---: | --- |
| Objective | 7 | the product's purpose changes |
| Canonical pipeline | 2 | never, in practice |
| Cross-cutting implementation doctrine | 57 | a cross-cutting rule is adopted |
| Current strategic priorities | 34 | a program group opens or closes — **the section that accreted** |
| Workstream status | 29 | a workstream is added (23 rows + 6 scaffold) |
| Recommended implementation order | 11 | strategy is re-ordered |
| History and execution | 9 | a route is added or replaced |

## Decision

**1. Each H2 of the bounded root declares a line bound and its own remedy.** `current_root.section_limits`
holds one record per section: `heading`, `max_lines`, and the `remedy` a breach calls for. The remedy is part
of the declaration, not the prose, so the failure and warning messages carry it — the reader of a red gate
never has to work out what the compliant action is.

**2. The declared sections must be exactly the required H2 set, in order.** A new section cannot appear
without a bound, a removed one cannot leave a stale entry, and a reorder is a breach. Chronology therefore
has nowhere unbounded to land.

**3. No legal combination of sections may exceed the reviewed working set.** The gate requires
`Σ max_lines + section_count ≤ health_targets.lines` (the count covers the H1 and the blank separators).
This is ADR 0029's rule applied inward: an aggregate must be reachable by individually legal parts. The
present set sums to 243 of the 256-line health target, so a fully legal root is a healthy root — the file
ceiling stays a quarantine backstop rather than the operative control.

**4. Bounds are derived from measured shape and change frequency, not split evenly.** A section that changes
on product purpose gets little headroom; the one that grew gets enough for a new program group and no more.
Current pressure is 50.0% / 33.3% / 71.2% / 60.7% / 65.9% / 55.0% / 56.2% — every section below the 80%
warning, with `Cross-cutting implementation doctrine` the tightest at 71.2%, whose declared remedy is that a
rule needing its own paragraph is a decision record rather than a roadmap bullet.

**5. Section measurement is defined so it cannot be gamed by whitespace.** A section is its heading plus its
body up to the next H2, with trailing blank lines trimmed.

## Consequences

- The failure now arrives ~200 lines earlier and names its cause. Replaying the real accretion, the gate
  warns at 46/56 section lines (82.1%) and fails at 58/56 — with the whole file at 180 lines, less than half
  its 384-line ceiling and still under its 256-line health target.
- The rollover is no longer the first line of defence. It stays the remedy for legitimate growth of the
  direction itself; ordinary chronology is refused at the section that receives it.
- The contract's focused cases go from 31 to 49. The 18 new ones cover the declaration (missing, reordered,
  extra, or unbounded sections; a missing remedy; an unknown field; bounds summing above the health target),
  the measurement (heading counted, trailing blanks trimmed), the boundary (exactly at bound passes, one line
  over fails), the real accretion shape, an undeclared section present in the root, and that the warning
  fires at the declared percentage and stays silent both below it and above the hard bound.
- No file-level ceiling, health target, or milestone moved. This adds a tighter inner control; it relaxes
  nothing.
- The control is roadmap-specific by design. Whether other `bounded_snapshot` surfaces need the same inner
  bound is a separate measurement, not an assumption — `LIVE-DOC-STOP-RISK.1` owns the remaining survey.

## Links

- Task tree: [`LIVE-DOC-STOP-RISK`](../tasks/LIVE-DOC-STOP-RISK.md)
- Closes the limit stated by:
  [ADR 0030](0030-a-bounded-snapshot-needs-a-declared-repeatable-rollover.md)
- Applies inward the rule from:
  [ADR 0029](0029-fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy.md)
