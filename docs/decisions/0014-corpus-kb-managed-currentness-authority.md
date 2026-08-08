# 0014 — Tracked inputs and reviewed validation define corpus-KB currentness

- Date: 2026-08-08
- Status: accepted
- Deciders: project owner, SpecForge repository workflow

## Context

Eleven Markdown pages below `corpus_kb/` contain generated regions while preserving human synthesis
outside their markers. The KG regions still described 150 passing fixtures after the tracked suite had
grown to 156. The validation region described a later host-local report state with scores
`85/90/94/90`, while the independently enforced last-reviewed snapshot described `71/74/65/67`.
Both planes looked current even though neither had an exact dependency/output oracle.

The existing `corpus-kb` writer also accepted ambient ignored validation reports. That is useful for
local exploration, but such reports cannot silently replace the tracked reviewed boundary.

## Decision

The tracked corpus-KB validation region is derived from `VALIDATION_SNAPSHOT.md`, whose authority and
review boundary remain owned by ADR 0012. `corpus-kb --validation-snapshot VALIDATION_SNAPSHOT.md`
parses only the snapshot's projected-artifact section, rejects malformed or duplicate records, and
rewrites the validation managed block. Positional report inputs remain an explicitly local alternative
and cannot be combined with the reviewed-snapshot mode.

The Git-indexed tree below `crates/specforge/test_data/kg_quality/` is the KG input denominator. Its
complete 312-file / 156-fixture identity is bound to the generated aggregate, eight family pages,
review-only prior-candidate Markdown, and paired JSON manifest. The executable `kg-bench` run remains
the behavioral verifier; generated page identity is not a substitute for running the fixtures.

`scripts/check_corpus_kb_currentness.pl` is read-only. It binds exact input membership/content,
reviewed validation semantics, all eleven managed regions, paired JSON, producer regions, and both
human regions around every marker. Its mutation fixtures use guarded repository-local `generated/`
storage and prove cleanup. Canonical IR and typed prior memory remain forbidden mutation targets.

## Consequences

- A tracked KG fixture change requires a successful complete refresh and executable benchmark pass.
- A reviewed validation-boundary change requires the corpus validation region to move in the same
  reviewed commit.
- Human synthesis cannot be replaced accidentally: prefix and suffix identities fail closed.
- A local report projection is never tracked currentness authority merely because the writer ran.
- Prior-candidate pages and JSON remain review surfaces; no candidate is promoted by refresh.
- `corpus_knowledge_base.currency` is now enforced rather than transition debt.

## Links

- `VALIDATION_SNAPSHOT.md`
- `corpus_kb/`
- `doctrine/live_document_size/corpus_kb.json`
- `scripts/check_corpus_kb_currentness.pl`
- `docs/knowledge/corpus-kb-managed-currentness.md`
- `docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md`
