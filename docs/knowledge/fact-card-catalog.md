---
id: fact-card-catalog
title: Fact cards have a bounded derived human catalog distinct from question retrieval
answers:
  - "how can I browse every SpecForge knowledge fact card by id or title"
  - "why does the fact-card file count differ from the Knowledge Map fact count"
  - "how is docs knowledge INDEX kept complete"
  - "why is the SpecForge fact-card catalog almost out of capacity"
  - "which task owns fact-card catalog containment"
  - "how much fact-card catalog index headroom remains"
date: 2026-08-08
status: current
tags: [knowledge-map, navigation, generated-index, continuity]
evidence: docs/knowledge/INDEX.md; scripts/check_fact_card_catalog.pl; docs/decisions/0020-bounded-fact-card-browse-projection.md; docs/decisions/0021-cross-directory-fact-catalog-links-preserve-destinations.md; docs/decisions/0022-fact-catalog-parts-pack-below-warning.md; docs/decisions/0023-fact-catalog-landing-fits-full-capacity.md
reverify: perl scripts/check_fact_card_catalog.pl --check
---

`docs/knowledge/INDEX.md` is the bounded human catalog for immediate fact cards. It derives one
concise id/title/date/status row per card, links the collection README separately, and never copies
questions, evidence, reverify commands, or bodies. The focused checker and the generic Markdown-link
membership gate both run through `LIVE-DOC-SIZE`.

The collection file count is not the generated Knowledge Map fact count. At `.5c.ii` entry,
`docs/knowledge/` held 136 Markdown files: 135 front-mattered cards plus `README.md`. The generated
map reported 136 facts because it also scans `docs/decisions/`, where ADR 0007 participates as one
front-mattered fact. After this card and the derived catalog land, the directory has 138 Markdown
files, 136 of which are cards; the generated map has 137 facts including ADR 0007.

At committed boundary `47e91540`, the collection has 158 cards plus `README.md` and the generated
index: 160 immediate Markdown files, exactly the generic surface's 80% file-warning point. The
focused generator is tighter: its 160-card limit leaves two card slots, while the 32,634-byte
monolithic index has only 134 bytes below its 32,768-byte ceiling. The generated question map still
permits 200 total facts and currently reports 159, so the three capacity authorities disagree before
their advertised ceilings. `FACT-CARD-CATALOG-CONTAINMENT` owns a lossless bounded topology; it may
not delete facts or widen an existing limit to hide the pressure.

Recording the boundary adds three question keys. That moves the separate, already-sharded Knowledge Map
projection from 209,621 bytes (94 bytes below warning) to 209,962 bytes, or 80.1% of its aggregate health target.
Its 90% rollover and 393,216-byte enforcement ceiling remain independent; the containment decision must account
for this adjacent pressure without changing question-shard semantics by convenience.

ADR 0020 resolves the browse architecture without changing the stable path or the generic 200-file ceiling.
`docs/knowledge/INDEX.md` is now a bounded landing with one direct ID link per card and direct links to
deterministic count-packed title parts under `docs/knowledge-catalog/`. The detailed parts retain the existing
ID/date/status/title rows. The maximum 198 cards derives from 200 collection files minus the fixed README and
index. The committed `legacy_locked` state enforced exact old-row provenance and destination absence before the
separate migration commit changed generated output.

ADR 0021 corrects one pre-implementation detail: copying `(card-id.md)` rows byte-for-byte into the sibling
`docs/knowledge-catalog/` directory would break every card link. The migrated parts must preserve each row's
ID/date/status/title tuple and exact resolved `docs/knowledge/<card-id>.md` destination while rewriting only the
relative target to `../knowledge/<card-id>.md`. At ADR 0021's boundary, the corrected 64-card simulation totaled
176 lines / 34,720 bytes; ADR 0022 later superseded only its packing. The legacy monolith remains exact
provenance.

The executable renderer then exposed line pressure that the earlier byte-focused arithmetic missed. ADR 0022
supersedes only the original 64-card packing count: 56 cards plus seven scaffold lines make a full part 63/80
health lines, below warning, while 198 cards still fit in four parts. ADR 0023 applies the same proof to the root:
three scaffold lines plus 198 direct routes yield 201/224 health lines, below mandatory rollover. Current output
is a 161-line / 11,984-byte landing and three parts totaling 179 lines / 34,906 bytes; the complete four-file
projection is 340 lines / 46,890 bytes with no warning. Forty-one focused cases prove both states, fixed capacity
and rollover,
Git/file/card/row/output identity, external authorities, routes, residue, and safe write cleanup. Before `.2.2`,
the monolithic landing remained unchanged and `docs/knowledge-catalog/` remained absent.

`FACT-CARD-CAPACITY-HEADROOM.2` later removed the landing's per-card line, because that line — not the file
ceiling — was what capped the plane. The landing is now a fixed-size router: H1, the generated/navigation line, a
count line, and one table row per title part carrying the part link, its card count, and its inclusive first and
last id. Any id is still reached deterministically, in one extra hop through the part whose range contains it.
At 193 cards it fell from 196 lines / 15,417 bytes to 10 lines / 878 bytes and now scales with the part count
(7 lines at one part, 10 at the four-part maximum) rather than the card count. The parts were byte-identical.
The membership proof moved with it: the landing must not link a card directly, its rendered table must re-derive
the canonical card list exactly (ordered rows, counts summing to the catalog, confirmed boundary ids), every card
must resolve exactly once across the parts, and the `knowledge_cards` surface declares `routed_membership`
through `fact_card_titles` so the generic gate proves the same completeness in one declared hop. Capacity itself
is unchanged at this leaf — `.3` re-derives the whole profile against the new shape.

`.2.2` has now activated the exact `fact_card_titles` generated surface and written those four pinned outputs.
The first real migrated check also found that decoded non-ASCII semantic rows must be explicitly encoded before
SHA-256; the row-digest helper now hashes raw UTF-8 and an em-dash fixture prevents an ASCII-only regression.
Every direct ID and title-part route resolves, no stale or foreign residue exists, and canonical facts remain the
only evidence authority. The containment tree is closed.
