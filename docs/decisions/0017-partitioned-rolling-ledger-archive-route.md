---
id: partitioned-rolling-ledger-archive-route
date: 2026-08-09
status: accepted
scope: documentation, continuity, archive, retrieval
---

# ADR 0017: Rolling-ledger archives use a bounded landing and per-ledger authority partitions

## Context

The lossless rolling-ledger protocol originally used one shared Markdown index and one shared JSONL manifest for
four ledgers. After five sealed segments, the index is 81 lines / 5,311 bytes. Its 90% byte rollover begins at
5,530 bytes, leaving 218 bytes; the most recent segment route added 301 bytes. Another route cannot be admitted
without first containing the route itself.

The shared manifest is still within its control bounds at ten lines / 7,673 bytes, but it grows with every
ledger. More importantly, its required `predecessor` and `successor` values are checked only for scalar presence;
the verifier does not yet prove that all segment edges form one complete, acyclic newest-to-oldest chain.

## Decision

Keep `docs/archive/rolling-ledgers/INDEX.md` as a fixed bounded landing. It links exactly four ledger-specific
indexes, one under each existing archive directory. The landing never lists capsules or segments, so a segment
addition does not grow it.

Each ledger directory owns:

- `INDEX.md`, with direct links to the current root, every segment, the source capsule, and `manifest.jsonl` in
  exact newest-to-oldest order; and
- `manifest.jsonl`, containing the existing bounded control row plus only that ledger's byte-identical capsule
  and segment records.

The existing landing keeps its 96-line / 6,144-byte health targets and 128-line / 8,192-byte ceilings. A new
four-file `partitioned_canonical` surface governs the per-ledger indexes through the external landing, with
64 lines / 8,192 bytes / 512 maximum line bytes per file and 256 lines / 32,768 bytes in aggregate as health
targets; ceilings are 96 / 12,288 / 1,024 per file and 384 / 49,152 in aggregate. No existing ceiling widens.

Each per-ledger manifest retains the current 32-record / 32,768-byte / 2,048-record-byte / 768-scalar-byte
controls. Existing archive-segment ceilings permit at most 28 segments per ledger, so one capsule plus 28
segments uses 29 data records. Projecting the largest measured row for each ledger yields 26,134, 26,069,
28,107, and 28,420 bytes respectively, all below the unchanged manifest byte cap.

The rolling-ledger verifier must reject foreign-ledger manifest members, missing or duplicate edges, broken
predecessor/successor agreement, cycles, disconnected segments, missing landing routes, missing direct member
links, or index order that differs from the verified chain. Every manifest member path must occur exactly once in
its ledger index, in chain order.

Migration is atomic and lossless: split the shared manifest by copying its control row and exact data lines;
verify the four-part union equals the old data-line set byte-for-byte; switch registry and surface consumers;
replace the shared index with the fixed landing; verify direct retrieval and all identities; then delete only the
exact retired shared manifest and prove no current consumer or filesystem residue remains. Capsules and sealed
segments are never rewritten.

## Consequences

- Reader navigation becomes landing → ledger index → immutable member, at most two bounded hops.
- Adding a segment changes only its ledger manifest/index and chain neighbors; the shared landing is stable.
- The manifest and index independently expose the same complete ordered membership, and executable chain checks
  replace prose-trusted chronology.
- The next segment remains blocked until `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.9b` implements and verifies this
  topology. Future exhaustion of a per-ledger manifest or index requires a separately owned partition decision,
  not a ceiling increase.

## Links

- `docs/decisions/0008-lossless-rolling-ledger-protocol.md`
- `docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md` (`.9a` / `.9b`)
- `docs/research/rolling-ledger-archive-route-containment-design.md`
- `docs/knowledge/rolling-ledger-record-grammars.md`
