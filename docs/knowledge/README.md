# Knowledge Map fact cards

Fact files for the **Knowledge Map** retrieval layer live here. Each is a small,
front-mattered `.md` card whose `answers:` list holds the questions a future agent would
search for; a card is a **signpost** that points at the canonical home (a decision record,
the code, a book chapter), not a copy of it.

- Author one: copy `knowledge-map/templates/FACT_TEMPLATE.md`, rename it `<id>.md`.
- Read first: `knowledge-map/KNOWLEDGE_MAP_ARCHITECTURE.md` and `knowledge-map/FAQ.md`.
- Browse by id/title: [bounded fact-card catalog](INDEX.md), derived from this directory.
- Search by question: open the derived [`KNOWLEDGE_MAP.md`](../../KNOWLEDGE_MAP.md) landing and run
  its one `rg` command across the linked shards; neither landing nor shards is hand-edited.

Add a card **lazily**: whenever you establish a durable, structural/causal fact, or catch
yourself re-deriving one that should already have been written down. No migration sweeps.
