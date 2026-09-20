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
date: 2026-09-20
status: current
tags: [live-document-containment, mdbook, pressure-headroom, method, routing, census]
evidence: docs/tasks/live-document-pressure-headroom/book-chapter-routing.md (.19, .20, .21); COMMIT.md (documentation routing contract); doctrine/live_document_size/surfaces.jsonl (shipped_behavior)
reverify: "for each docs/book/src/**/*.md, sum the bytes between each '## ' heading and the next (skipping fenced blocks) — expect pipeline/evidenceir.md at 104,483 bytes with its 'Closed task trees' section at 82,294 (78.8%), and reference/live-docs.md at 121,517 (92.7% of the 131,072 ceiling) with 'Containment and project-data locality are fully enforced' at 96,721 (79.6%)"
---

`pipeline/evidenceir.md` breached its per-file ceiling twice in three days and was split both times
(`LIVE-DOCUMENT-PRESSURE-HEADROOM.3`, then `.19`). Each split moved material **by reader concern** and
each bought roughly six slices. The reason is not that the chapter is crowded.

**One section is 78.8% of it.** Measured `2026-09-20`:

| chapter | bytes | % of ceiling | largest section | % of file |
| --- | ---: | ---: | --- | ---: |
| `pipeline/evidenceir.md` | 104,483 | 79.7% | *Closed task trees — how each was implemented and verified* | **78.8%** |
| `reference/live-docs.md` | 121,517 | **92.7%** | *Containment and project-data locality are fully enforced* | **79.6%** |
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
lasts. Applying it to `reference/live-docs.md` — 9,555 bytes from a hard stop — is the open work.

## The owner tree cannot currently accept the leaf for it

`LIVE-DOCUMENT-PRESSURE-HEADROOM` is the tree that owns live-document pressure, and it has run out of
its own. Measured `2026-09-20` at `3d8f6218`:

| bound | at HEAD | after adding one leaf |
| --- | ---: | ---: |
| **bounded active root, lines (229 of a 256 target)** | **89.5% (warning)** | **90.2% — mandatory rollover** |
| `current-and-open-work` part, bytes | 89.1% (warning) | 96.5% *only if the node is put there* |
| `current-and-open-work` part, lines | 86.7% (warning) | 94.9% *only if the node is put there* |

**The root's line budget is the single binding constraint, and it binds at two lines.** The part rows
are avoidable: a node appended to a part with headroom (`book-chapter-routing.md` at 59 lines,
`warning-assignment.md` at 217) leaves the part collection green, and appending **outside** a marked
source region keeps a sealed region's payload intact — only the per-part metrics move, and
`check_active_task_evidence.pl --write` re-derives those. Exactly **one** line fits in the root
(230 of 256 is 89.8%); the second one fails the commit.

## …and the documented remedy would delete the only copy

Both root log tables say *"The complete dated log is in the verification-and-chronology part. These
are the most recent entries."* So the obvious remedy is to trim the window. **It is not available.**
`verification-and-chronology.md` is a sealed `legacy` part, frozen at the `.31` migration boundary,
and **seven leaves closed after that seal — `.27a`, `.28`, `.29b`, `.29c`, `.30a`, `.30b`, `.30c` —
have no row in it at all.** For those seven the root *is* the complete log, and the header sentence
is false. A trim that trusts it destroys the only record of seven closures.

So the real sequence is: give the post-seal chronology a writable home **first**, migrate those seven
records into it, and only then bound the root window. That is `.2c`'s rule arriving at the tree that
wrote it — a bound a surface can actually reach must have a remedy compliant work can take — and the
remedy here needs one more step than the root's own header advertises.

**Check before you trim.** The refusal above came from asserting, per removed row, that its leaf
appears in the chronology part; seven did not, and nothing else would have caught it.

Links: [[live-document-width-remedy-coupling]], [[doctrine-driver-runs-no-cargo-gate]].
