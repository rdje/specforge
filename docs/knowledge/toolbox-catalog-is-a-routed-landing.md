---
id: toolbox-catalog-is-a-routed-landing
title: TOOLBOX.md is a bounded landing over docs/toolbox parts, so a new tool entry costs the landing nothing
answers:
  - "where do I document a new SpecForge diagnostic tool or probe (in the docs/toolbox/ part that owns its numbered section - extraction-quality.md for 5.x, corpus-measurement.md for 6.x, gates-build-and-host.md for 7.x - never by appending to TOOLBOX.md, which is the bounded landing)"
  - "what does TOOLBOX.md still hold after the partition (the standing directive, the enforcement and task-acceptance checklist contract, the published-claim evidence contract, how to run the CLI, the quick chooser, the frozen first-reach tools in sections 1-4, the three diagnosis protocols, and one route row per part)"
  - "how do I search every SpecForge tool entry at once (rg -i 'term' TOOLBOX.md docs/toolbox)"
  - "does a citation like TOOLBOX.md section 7.7 still resolve after the toolbox partition (yes - section numbers were deliberately preserved and every cited number is still named in the landing, which routes it to its part in one hop; 26 such citations exist and several sit in sealed archive segments whose source end can never be repaired)"
  - "why was TOOLBOX.md partitioned at sections 5-7 rather than anywhere else (measured over its 43 revisions, sections 5, 6 and 7 added 254 of the 370 lines and 100% of the last nine revisions', while sections 1-4 added exactly ONE line in three months - the cut follows the writer, not the line count)"
  - "which workflow_standards member binds lines_each now (DOCTRINE_ENFORCEMENT.md at 597 of 700 = 85.3%, growing about 5 lines per revision; LIVE-DOCUMENT-PRESSURE-HEADROOM.27a owns it, and the partition relocated the maximum rather than releasing the surface)"
  - "what must I register when I add a new live-document surface (a surface record in doctrine/live_document_size/surfaces.jsonl, a census surface record plus at least one frozen evidence record in current_claim_census.jsonl with expected_current_surfaces bumped by one, and a published_assertions surface_disposition ONLY if some member file carries a [claim: ...] annotation)"
  - "which line must a new included census collection surface pin its evidence record to (the first non-blank line of its ALPHABETICALLY FIRST member path - produce_candidates sorts the surface's paths and emits one surface_review candidate from the first, so pinning any other member fails with 'lacks exact evidence')"
  - "why can a new toolbox part not simply join workflow_standards.targets (14 explicit members plus three parts is 17 of the 21-file profile ADR 0043 derived = 81.0%, and 17 plus the measured four-member peak day is exactly 21 - the stop; a landing and its parts are two surfaces, as validation_snapshot and validation_snapshot_parts already are)"
date: 2026-09-16
status: current
tags: [toolbox, live-document-size, doctrine, claim-verification, partition, workflow]
evidence: "TOOLBOX.md; docs/toolbox/extraction-quality.md; docs/toolbox/corpus-measurement.md; docs/toolbox/gates-build-and-host.md; doctrine/live_document_size/surfaces.jsonl (toolbox_parts); doctrine/claim_verification/current_claim_census.jsonl; docs/tasks/LIVE-DOCUMENT-PRESSURE-HEADROOM.md (.27, .27a, .29); docs/decisions/0043-workflow-standard-capacity-is-rederived-from-explicit-member-growth.md"
reverify: "perl scripts/check_live_document_size.pl && perl scripts/check_current_claim_census.pl --check"
---

# The toolbox landing routes; the parts grow

`TOOLBOX.md` is the single authoritative catalog of SpecForge's own diagnostic tools, and the standing
directive at the top of it says that when no tool can surface the WHY and WHERE, the next step is to
**build** one. That makes the file grow by construction: measured over its 43 revisions it went
**307 -> 677 lines**, about 8.6 lines per revision, against a `workflow_standards` per-file bound of
**700** whose health target equals its ceiling. At 676 lines it had 24 left, and a median tool entry
is 12 lines — so the bound had already started deciding what got built
(`LIVE-DOCUMENT-PRESSURE-HEADROOM.23` declined to ship a script with a toolbox entry for exactly this
reason). A containment bound that silently changes what gets built is past being a warning.

**The cut follows the writer, and the writer was measured, not guessed.** Attributing every revision's
delta to its section gives: section 7 **+115**, the acceptance-checklist contract **+109**, section 6
**+82**, section 5 **+57**, the quick chooser +4, the protocols +2 — and sections **1-4 together: +1**,
one line in three months. So the live catalog is sections 5-7 and the stable core is sections 1-4. The
partition moved 5-7 into `docs/toolbox/` and left 1-4 in the landing, where an agent still finds
`doctor`, `inspect`, `validate`, `adapt` and `kg-bench` without a hop. Moving them too would have
bought 85 lines against a population that does not grow, which is the "shard mechanically by arbitrary
line count" the containment doctrine explicitly forbids.

**Section numbers were preserved on purpose.** 26 places in the repository cite `TOOLBOX.md` §5.5,
§5.6, §6.7, §6.8, §7.2, §7.2a, §7.2a-i, §7.6 or §7.7, and several of them sit inside sealed
`archive_terminal` rolling-ledger segments that the rollover doctrine forbids editing — the source end
of those links can never be repaired. `scripts/check_section_anchors.pl` would not have caught the
break either, because it only resolves the fully backticked `` `<path>.md` §`<section>` `` form and all 26
write the section bare. So the landing keeps every cited number visible in its route row, and a reader
following `§7.7` reaches the part in one hop.

**The partition relocated the bound; it did not remove it.** `workflow_standards.lines_each` went from
96.6% (rollover, 24 lines left) to 85.3% (warning, 103 lines left) — and the maximum moved from
`TOOLBOX.md` onto `DOCTRINE_ENFORCEMENT.md`, which is growing about 5 lines per revision and is roughly
19 revisions from the same stop. That successor is owned by `LIVE-DOCUMENT-PRESSURE-HEADROOM.27a`.
Stating the relocation rather than claiming a removal is this tree's standing rule
(`[[live-surface-edit-bookkeeping-chain]]` covers the derived-state refreshes the move set off).
