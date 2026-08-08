---
id: knowledge-map-architecture-location
title: Knowledge-map architecture lives inside the knowledge-map bundle
answers:
  - "where is KNOWLEDGE_MAP_ARCHITECTURE.md"
  - "why does root KNOWLEDGE_MAP_ARCHITECTURE.md not exist"
  - "what is the canonical knowledge-map architecture path"
date: 2026-08-08
status: current
tags: [knowledge-map, repository-layout, bootstrap]
evidence: knowledge-map/KNOWLEDGE_MAP_ARCHITECTURE.md; AGENTS.md
reverify: test -f knowledge-map/KNOWLEDGE_MAP_ARCHITECTURE.md && test ! -e KNOWLEDGE_MAP_ARCHITECTURE.md
---

The canonical architecture document is
`knowledge-map/KNOWLEDGE_MAP_ARCHITECTURE.md`, inside the portable bundle. Root
`KNOWLEDGE_MAP.md` is the bounded landing for the derived question-shard projection set; there is no
root architecture file. Bootstrap and
navigation references must use the bundle-relative path. The README route guard exposed and now
prevents this stale-root-path failure for the public landing page.
