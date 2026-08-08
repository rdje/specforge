# 0011 — FSMGen feedback keeps a bounded current channel over exact correspondence history

- Date: 2026-08-08
- Status: accepted
- Deciders: project owner, SpecForge repository workflow

## Context

`docs/FSMGEN_FEEDBACK.md` is the stable SpecForge-to-FSMGen handoff path, but it is not a uniform
chronological ledger. Its pinned 936 lines / 57,980 bytes contain five directed correspondence records,
one three-section two-bug episode, a current `.isf` scope override, and a 456-line legacy primer. All
six exchanges are closed and independently evidenced by FSMGen's tracked response or the two issue-
bundle resolution files; there is no open request at the boundary.

The legacy primer is also a current-truth hazard. It says `.fsm` is one of SpecForge's adapter targets
and names `030f8c273` as the latest reviewed response baseline. Those claims predate the `.isf`-only
consolidation and the current pinned FSMGen gitlink `d327129b7`. A scope note at the top labels the old
`.fsm` material historical, but a reader can still encounter contradictory current-facing headings and
“latest” wording in the same live file.

The stable path has 26 known current-tree consumers. `TOOLBOX.md` treats it as the authoring channel;
Rust diagnostics and task/research/fact records link it as durable evidence. Changing the path or
discarding resolved detail would break those contracts.

## Decision

Keep `docs/FSMGEN_FEEDBACK.md` as a bounded, human-authored current channel. Its migrated form will
contain:

1. the current `.isf`-only downstream boundary and exact pinned FSMGen gitlink;
2. an explicitly marked open-correspondence region;
3. a concise closed-correspondence register for all six pinned exchanges, with direct history/evidence
   routes;
4. the authoring schema for future records; and
5. direct routes to FSMGen's response, reproducible issue bundles, and the exact local history archive.

Future open records are task-owned and must declare `Direction`, `Kind`, `Status`, `Owner`, and
`Evidence`. Direction is explicit (`SpecForge → FSMGen` or `FSMGen → SpecForge`); kind and status use
the controlled contract vocabularies. At most eight open records may remain live, each bounded to 96
lines / 12,288 bytes / 512 bytes per line. Large reproductions belong in the existing issue-bundle
collection, and detailed design evidence belongs in task/research records; the channel carries the
request and routes, not duplicate payloads.

Before replacing the root, freeze the exact pinned source at
`docs/archive/fsmgen-feedback/source-through-2026-08-08.md`. A bounded archive index and JSON manifest
record its identity and retrieval contract. The capsule is immutable and retains every legacy claim,
full request, embedded response, and issue narrative byte-for-byte.

`doctrine/live_document_size/fsmgen_feedback.json` pins five exhaustive source regions, all six closed
exchange spans and status/direction/evidence records, both current-truth findings, all 26 consumers,
the future root schema, and archive topology. `scripts/check_fsmgen_feedback_protocol.pl` runs
unconditionally under `LIVE-DOC-SIZE`: planned state freezes the current source; migrated state will
authenticate the capsule and validate the bounded root, open-record schema, closed register, manifest,
and index.

The root targets 256 lines / 32,768 bytes / 512 bytes per line and hard-fails above 384 lines / 49,152
bytes / 1,024 bytes per line. The archive index hard-fails above 96 lines / 8,192 bytes / 256 bytes per
line. These limits support several evidence-linked active exchanges without admitting another primer or
embedded reproduction corpus.

## Consequences

- `.5f.i` changes no feedback record and creates no archive; it freezes identity and activates the
  planned-state design/evidence/consumer contract.
- `.5f.ii` must copy and verify the exact capsule before rewriting the stable root, preserve direct
  retrieval for all six closed exchanges, migrate readers and surface records atomically, and remove
  transition debt.
- Incoming authoritative detail remains in
  `subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md`; the local channel records its status and direct
  evidence route rather than copying the upstream response wholesale.
- Bug reproductions remain in `docs/fsmgen-issues/` under the existing issue-bundle protocol.
- The exact capsule preserves obsolete `.fsm` and stale-pin prose as historical evidence; the current
  root must not present either as live truth.

## Links

- `doctrine/live_document_size/fsmgen_feedback.json`
- `scripts/check_fsmgen_feedback_protocol.pl`
- `docs/knowledge/fsmgen-feedback-channel.md`
- `docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md`
