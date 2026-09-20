---
id: bounded-task-root-accretes-in-its-frontier
title: A migrated task tree's bounded root accretes in its Current Frontier section, not in its log windows — 39 of 83 lines — so the frontier is overwritten and the chronology moves to a writable part
answers:
  - "which section of a bounded task root grows fastest"
  - "why is a bounded active root near its line ceiling"
  - "where should a closed leaf's verification and commit rows live after a task tree migration"
  - "how much should one closure cost a bounded task root"
  - "should a bounded task root carry verification log rows"
  - "how do I give a migrated task tree a writable chronology"
  - "what is a current-chronology task-evidence part"
  - "how do I add a semantic part to a migrated task tree"
  - "what must I update when adding a task-evidence part"
  - "why did adding a task-evidence part trip the files warning"
date: 2026-09-20
status: current
tags: [live-document-containment, task-trees, pressure-headroom, method, routing]
evidence: docs/tasks/live-document-pressure-headroom/current-chronology.md (`.37`); doctrine/live_document_size/pressure_headroom_task_evidence.json; scripts/check_active_task_evidence.pl
reverify: "wc -l docs/tasks/LIVE-DOCUMENT-PRESSURE-HEADROOM.md — expect 166 of a 256-line health target (64.8%); grep -c '^| `' on the same file — expect 0, because the root carries both log tables' headers and no rows; perl scripts/check_active_task_evidence.pl --contract doctrine/live_document_size/pressure_headroom_task_evidence.json --check — expect valid with the part-collection files warning at 81.2%"
---

`.30` migrated `LIVE-DOCUMENT-PRESSURE-HEADROOM` on `2026-09-16`, leaving a **bounded active root** of
146 lines over twelve semantic parts. Four days and 19 revisions later it was **229 of a 256-line
health target — 89.5%, one line from mandatory rollover** — and could not accept the next leaf.

## Attribute the growth before choosing the cut

| root section | lines gained over 19 revisions | bytes gained |
| --- | ---: | ---: |
| `## Current Frontier` | **+39** | +3,777 |
| `## Executable Owner Registry` | +18 | +788 |
| `## Verification Log` | +13 | **+4,588** |
| `## Commit Log` | +13 | +2,419 |
| `## Current Program State` | +0 | +84 |

**The binding axis was lines, and on it the frontier section is the largest single writer by a factor
of two.** The two log windows lead on bytes instead (+7,007 of +11,656), so which section "is the
problem" depends on which axis is actually binding — measure, do not assume. The owner registry's +18
is one line per leaf and is structural: it is the irreducible cost of a closure.

## Why a frontier section accretes at all

A frontier should say what the next step is. This one had accumulated **a paragraph per closure** —
the same failure mode as a book section named for a work unit ([[book-chapter-growth-is-one-accreting-section]]):
nothing in the shape answers *where does this belong?*, so every leaf appends. The fix is not a
smaller paragraph, it is a rule: **the frontier section is overwritten, never appended to**, and the
per-closure narrative goes to a chronology.

## A migrated tree has nowhere to put a closure record until you make one

The migration seals the chronology part as `legacy` — immutable, byte-exact against the archived
capsule — so **no post-migration closure can add a row to it**. The rows go to the root instead and
accrete there. The remedy is a second, `active` chronology part, and then the root can carry each log
table's header with **no rows** and a pointer to both halves. A closure then costs the root one
owner-registry line.

## Adding a semantic part: the four places, in order

1. Write the part file: `# <TREE> — <label>` as byte 0, then `- Part ID: \`<id>\`` and
   `- State: \`active\``. Only a `legacy` part must own a marked source region; an `active` one must
   not, so a new part carries no markers.
2. `doctrine/live_document_size/<tree>_task_evidence.json`: append to `destinations.parts` with
   `heading`, `path`, `part_id`, `state`, `sha256`, and `metrics` (`bytes`, `lines`, `line_bytes` —
   `lines` counts `\n`, plus one if the file does not end in one). Add the leaf route
   (`origin: post_migration`, `part_id: <id>`) and the root `- ID:` literal to
   `migrated_requirements.root_required_literals`. **Derive the encoder before writing**: this file
   round-trips exactly under `json.dumps(indent=1, sort_keys=True, ensure_ascii=True)` plus a
   trailing newline, and `leaf_routes` is append-ordered, not sorted.
3. `doctrine/live_document_size/surfaces.jsonl`: add the path to the `…_task_evidence_parts`
   surface's `targets`, which is an explicit list, not a glob.
4. `perl scripts/check_active_task_evidence.pl --contract … --write` regenerates `INDEX.md`, the route
   catalog and `manifest.json`. It does **not** compute the contract's part metrics or digests — those
   are yours, and its preflight rejects the write if they are wrong.

**A thirteenth part trips a warning and that is the honest price**: the part collection's `files`
health target is 16, so 13 is 81.25%. There are 11 parts of headroom to the 24-file portable ceiling
and the collection totals sit at 44.0% of bytes and 42.6% of lines, so the count is not near a stop —
it is the second instance of the axis `LIVE-DOCUMENT-PRESSURE-HEADROOM.14e` owns, and it argues that a
file count is the wrong instrument for a collection whose parts are semantic rather than sized.

## The bill every governed edit pays, and the one way to pay it wrong

Touching any governed file stales the **whole-file `sha256` artifact digests** recorded in the four
claim registries, and those registries reference each other, so the refresh is a **fixpoint**: refresh,
recompute, repeat until nothing moves (two iterations in practice). Refresh **only objects that carry a
`path` and a `sha256` and no `start_line`/`end_line`. An object with a line range is a region pin** —
`control.red_case`, `red_evidence.source_region`, and the `region` of a census or assertion record — and
a walker that rewrites those with a whole-file digest silently destroys them. Those belong to
`python3 scripts/repin_claim_regions.py --check` / `--apply`, which resolves by content and refuses when
a match is ambiguous.

A `REFUSED ABSENT` from the re-pinner is not a nuisance: it means a pinned prose line **changed**, which
is usually a published number moving. Fix the number from its producer first, then the digest, and then
the identifier — a census evidence record carries the region digest's first 12 hex in both `evidence_id`
and `claim_key`, and the gate rejects a re-pin that did not rename them.

Adding a fact card also moves `docs/knowledge/INDEX.md`, so it needs
`perl scripts/check_fact_card_catalog.pl --print-plan` written into `planned_outputs` (encoder:
`indent=2, sort_keys=True, ensure_ascii=True`) before `--write` will run. A new book paragraph
containing a number needs an adjudicated region in `book_quantitative_claims.jsonl` and its
`expected_candidate_lines` raised to match.

Links: [[book-chapter-growth-is-one-accreting-section]], [[live-document-width-remedy-coupling]].
