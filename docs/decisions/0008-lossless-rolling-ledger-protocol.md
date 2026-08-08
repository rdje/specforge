---
id: lossless-rolling-ledger-protocol
date: 2026-08-08
status: accepted
scope: documentation, continuity, archive, retrieval
---

# ADR 0008: Lossless rolling-ledger protocol

## Context

Four root continuity documents mix a bounded recent working set with valuable chronology. They do
not share one Markdown grammar: `CHANGES.md` crosses from modern H3 records to legacy H2 records and
contains two detached containment records at its tail; `DEVELOPMENT_NOTES.md` and
`RUST_CODEBASE_ANALYSIS.md` use H2 records with different prologues; `LIVE_ACHIEVEMENT_STATUS.md`
uses one-line bullets between stable headings and has a generated validation trailer written by
Rust code. A generic heading split would corrupt at least two of these surfaces.

Git history alone is not the promised archive. Shallow clones, history rewriting, and ordinary
reader navigation make a Git-only terminal conditional. Before removing bytes from a live root, the
project needs a repository-volume terminal whose identity and retrieval are executable.

## Decision

Adopt one protocol with four closed parser grammars, declared in
`doctrine/live_document_size/rolling_ledgers.jsonl` and enforced by
`scripts/check_rolling_ledger_protocol.pl` through `LIVE-DOC-SIZE`.

Each initial migration must atomically:

1. freeze the exact pre-migration source as an immutable repository-relative source capsule;
2. record its SHA-256, record/line/byte/max-line metrics, first and last record, owner, sealing date,
   reason, and verifier in the bounded archive manifest;
3. retain a deliberately measured newest-first whole-record window at the stable root path;
4. keep a bounded index that names the live root and capsule and provides a tool-neutral query route;
5. prove parser reconstruction, capsule identity, live limits, and the exact retained-record suffix;
6. preserve any load-bearing writer marker and update all routes in the same commit.

The initial source capsule intentionally overlaps the retained live window. That declared overlap is
the simplest durable proof of complete-source identity: the capsule reproduces every original byte,
while the root is the bounded current view. The capsule is not an author-overflow destination.
Subsequent records prepend only to the root; rotation seals whole record ranges rather than creating
another full capsule. Sealed evidence is corrected by a successor/superseding record, never edited.

The two `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.0` and `.1` records appended below the
`CHANGES.md` 2026-03-31 tail are a measured compatibility exception. The first migration will promote
those exact records into the live window after the newest prefix so current containment history stays
visible. The immutable source capsule preserves their original ordinals and bytes, so promotion is
not represented as a correction to historical ordering.

## Consequences

- No `.4b`–`.4e` leaf may move a record until its source matches the pinned `.4a` measurement or the
  task explicitly remeasures a legitimate new leading record before freezing the capsule.
- `LIVE_ACHIEVEMENT_STATUS.md` remains at its root path, and the validation projection markers remain
  present for `project_validation.rs`.
- A migration is blocked by ambiguous record boundaries, non-reconstructing bytes, missing consumers,
  an off-root path, a manifest/index mismatch, or a survivor over any independent local limit.
- Exact source identity and bounded current utility are separate products. Their declared overlap is
  measured, not hidden by disjoint arithmetic.
