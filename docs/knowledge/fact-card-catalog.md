---
id: fact-card-catalog
title: Fact cards have a bounded derived human catalog distinct from question retrieval
answers:
  - "how can I browse every SpecForge knowledge fact card by id or title"
  - "why does the fact-card file count differ from the Knowledge Map fact count"
  - "how is docs knowledge INDEX kept complete"
date: 2026-08-08
status: current
tags: [knowledge-map, navigation, generated-index, continuity]
evidence: docs/knowledge/INDEX.md; scripts/check_fact_card_catalog.pl
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
