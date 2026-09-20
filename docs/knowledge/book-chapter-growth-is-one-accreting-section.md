---
id: book-chapter-growth-is-one-accreting-section
title: A book chapter that keeps breaching its ceiling is one append-only section, not a crowded chapter — evidenceir.md is 78.8% a single "closed task trees" log, and splitting the tidy sections buys about six slices
answers:
  - "why does the EvidenceIR book chapter keep breaching its size ceiling"
  - "which section should a chapter split move"
  - "why did splitting a book chapter only buy six slices"
  - "where should a new extraction rule document itself in the book"
  - "may a book section be named after a task leaf"
  - "what drives docs/book/src/reference/live-docs.md to 92.7% of its ceiling"
  - "how much of the mdBook is closed-task-tree log sections"
  - "how do I stop a live document becoming an append-only log"
  - "why can LIVE-DOCUMENT-PRESSURE-HEADROOM not accept a new leaf"
  - "what blocks adding a node to the pressure-headroom task tree"
  - "can I trim the root verification log into the chronology part"
  - "which leaves have no row in verification-and-chronology.md"
  - "how many pressure-headroom closures have no row in the sealed chronology part"
  - "which block of the live-docs book chapter was moved out and why"
date: 2026-09-20
status: current
tags: [live-document-containment, mdbook, pressure-headroom, method, routing, census]
evidence: docs/tasks/live-document-pressure-headroom/book-chapter-routing.md (.19, .20, .21); COMMIT.md (documentation routing contract); doctrine/live_document_size/surfaces.jsonl (shipped_behavior)
reverify: "for each docs/book/src/**/*.md, sum the bytes between each '## ' heading and the next, skipping fenced blocks — expect pipeline/evidenceir.md at 104,450 bytes (79.7% of the 131,072 ceiling) with its 'Closed task trees' section at 82,900 (79.4%), and reference/live-docs.md at 75,698 (57.8%) after LIVE-DOCUMENT-PRESSURE-HEADROOM.34 routed its chronology to reference/containment-adoption.md (49,293). The pre-.34 figures in this card's table were taken with a slightly different section-boundary convention and differ from this one by tens of bytes; the shares are unaffected"
---

`pipeline/evidenceir.md` breached its per-file ceiling twice in three days and was split both times
(`LIVE-DOCUMENT-PRESSURE-HEADROOM.3`, then `.19`). Each split moved material **by reader concern** and
each bought roughly six slices. The reason is not that the chapter is crowded.

**One section is 78.8% of it.** Measured `2026-09-20`:

| chapter | bytes | % of ceiling | largest section | % of file |
| --- | ---: | ---: | --- | ---: |
| `pipeline/evidenceir.md` | 104,483 | 79.7% | *Closed task trees — how each was implemented and verified* | **78.8%** |
| `reference/live-docs.md` (before `.34`) | 121,517 | **92.7%** | *Containment and project-data locality are fully enforced* | **79.6%** |
| `quality/validation.md` | 67,768 | 51.7% | *Closed task trees…* | 41.1% |
| `pipeline/isf-adapter.md` | 89,553 | 68.3% | *Closed task trees…* | 29.6% |

Both splits moved the **tidy** sections — obligation reading, failure modes, each 15–17 KB — and left
the accreting one where it was. Across the whole manual these log sections are **171,373 bytes, 13.4%**
of the book.

## Why that section can only grow

**67 of its 124 subsections are named for a work unit**, not for a behaviour:
`` `EXTRACTION-GAP-FIX.4c` — putting a split register back together ``. A work-unit heading removes the
authoring question *where does this belong?* — every leaf simply gets another subsection — so the
section has no shape that could ever be full. A behaviour-named heading forces the question, and the
answer is usually an existing section.

## The three rules this produced (`COMMIT.md`, documentation routing contract)

1. **Name a section for the behaviour it explains, never for the work unit that shipped it.** The leaf
   id goes in the closing *Authoritative tracking* pointer.
2. **Extend the section that owns the reader's concern**; open a new one only when none does.
3. **When a chapter must be split, move the section that is ACCRETING**, not the tidy ones.

Rule 3 is the one the two earlier splits missed, and it is the only one that changes how long a split
lasts.

**It was applied to `reference/live-docs.md` on `2026-09-20` (`.34`) and the result is the evidence for
the rule.** The block that moved was chosen by attributing growth across the file's last 40 revisions:
`### Collection and projection closure order` was **+24,085 bytes of +55,799** and the largest block at
48,634. (That attribution accumulates per-revision section deltas; differencing the window's two endpoints
gives +23,984 of +55,557 — same ranking, same margin. Quote the method with the number.) It was also a *chronology* — eighteen subsections named for landing events — sitting inside a
chapter of current doctrine, so the cut is between narrative kinds rather than between topics. Moved
byte-identically with no heading promoted, the chapter went **121,517 -> 75,698 bytes, 92.7% -> 57.8%**,
and the `shipped_behavior` byte warning cleared outright. **A chronology grows and a doctrine statement
does not, which is why this cut should not have to be made again in that chapter.**

The next one is already measured rather than waiting to be discovered: `pipeline/evidenceir.md` is now
the manual's largest file at 104,450 bytes (79.7%), its *"Closed task trees"* section is 82,900 of them
(79.4%), and `LIVE-DOCUMENT-PRESSURE-HEADROOM.34a` owns making the same cut before a commit is blocked.

## The owner tree could not accept the leaf for it — `.37` fixed that, and corrected the count

`LIVE-DOCUMENT-PRESSURE-HEADROOM` is the tree that owns live-document pressure, and it had run out of
its own. Measured `2026-09-20` at `3d8f6218`, the **bounded active root's line budget was the single
binding constraint and it bound at one line**: 229 of a 256 health target is 89.5%, 230 is 89.8%, and
231 is 90.2% — mandatory rollover. The `current-and-open-work` part rows published alongside it
(89.1% bytes, 86.7% lines) were avoidable rather than binding, because a node appended to a part with
headroom leaves the part collection green.

**`.37` bounded the root instead of trimming it**, and the shape is the point: the root now carries
the two log tables' headers and **no rows**, and its `## Current Frontier` is overwritten rather than
appended to. Measured after: **229 -> 166 lines (89.5% -> 64.8%)**, bytes 82.4% -> 33.6%, widest line
80.3% -> 22.1%. A closure now costs the root **one owner-registry line** and nothing else.

## …and the documented remedy would have deleted the only copy of fourteen closures, not seven

Both root log tables said *"The complete dated log is in the verification-and-chronology part. These
are the most recent entries."* **It was false.** That part is sealed `legacy` at the `2026-09-16`
migration boundary and cannot take a row, so every closure after it wrote into the root instead.

**The first published count of the exposure — seven — was an under-count, and the oracle is why.**
Re-derived per row on `2026-09-20`: the root's Verification Log held **21 rows of which 14 existed
nowhere else**, and its Commit Log **19 rows of which 14 existed nowhere else**. The six leaves the
published list missed — `.31`, `.32`, `.33`, `.36b`, `.36b.1`, `.36c` — were the six most recent
closures. Byte-presence in the sealed part clears only **1 verification row and 5 commit rows**; the
other 7 verification rows are condensed renderings the migration made for the bounded root, and each
was cleared by testing that its **content-bearing tokens** (numbers, percentages, code spans) are a
subset of the sealed row for the same date and leaf.

**The token test refused a row that reading had passed.** The second `.30` row records that three
closed leaves still declared `` `pending` `` — a token absent from its sealed counterpart — so it
moved instead of being dropped. Seven is the number of rows it is safe to delete; it was never the
number of records at risk.

**Check before you trim, and check mechanically.** A leaf-level check ("does this leaf appear in the
chronology part?") passes for rows whose content is unique, because the leaf appears there for a
*different* closure. The row-level token test is the one that discriminates.

Links: [[live-document-width-remedy-coupling]], [[doctrine-driver-runs-no-cargo-gate]].
