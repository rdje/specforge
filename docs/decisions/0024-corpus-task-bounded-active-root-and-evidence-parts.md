---
id: corpus-task-bounded-active-root-and-evidence-parts
title: Corpus coverage keeps a bounded active root over seven evidence parts and exact provenance
date: 2026-08-10
status: accepted
scope: documentation, continuity, task-tree, corpus-coverage, active-work, archive, retrieval
evidence: docs/research/corpus-task-evidence-containment-census.md; docs/tasks/CORPUS-TASK-EVIDENCE-CONTAINMENT.md
answers:
  - "what architecture contains the active CORPUS-COVERAGE task tree"
  - "what is ADR 0024"
  - "how does the corpus task preserve all evidence after containment"
  - "what is the corpus task evidence writer transaction after migration"
  - "what limits govern the corpus task root index parts and capsule"
  - "why does the corpus task reuse the active task evidence checker"
---

# ADR 0024: Corpus coverage keeps a bounded active root over seven evidence parts and exact provenance

## Context

`docs/tasks/CORPUS-COVERAGE.md` is active at 2,308 lines / 277,636 bytes / 4,746 maximum content-line bytes,
only 892 bytes below its registered ceiling. The fourteen latest path-growth intervals average 4,134.7 bytes and
never fall below 1,215 bytes, so the source cannot own refresh #49 safely.

The corpus-specific census accounts for every byte in seven natural regions. It finds 48 fully qualified formal
ids, 41 of them in completion-subject history, with no commit-only or shorthand-only route. At committed census
input, 42 tracked files cite the stable path and 105 mention the task identity. No path citation uses a fragment
or query and no executable opens the target. The manual task-tree/`COMMIT.md` transaction is its only writer.

Current evidence agrees that 48 of 56 real chip-spec refreshes are done, eight remain, stage coverage is
80 SourceIR / 20 normalized / 80 EvidenceIR / 79 downstream chains, and 59/59 emitted ISFs are strict-clean.
Metadata remains one refresh stale, and the containment prerequisite is embedded in historical task material
rather than a bounded `Current Frontier` section. Literal history must remain exact while current authority is
normalized explicitly.

ADR 0019 establishes the reusable architecture class but does not authorize copying the first target's regions,
routes, thresholds, or conclusions. This decision is based only on the measured corpus source and its consumers.

## Decision

Adopt three distinct authorities:

1. **Current authority:** keep `docs/tasks/CORPUS-COVERAGE.md` as a bounded active root. It owns stable identity,
   goal/non-goals/acceptance, normalized activity state, current counts, current frontier/blockers, the one active
   product leaf contract/checklist, bounded recent verification/commit evidence, and the route to detail.
2. **Browsable task authority:** create `docs/tasks/corpus-coverage/INDEX.md`, `manifest.json`, and seven exact
   marker-delimited legacy parts: `program-and-latest-refreshes.md`, `refreshes-33-40.md`,
   `refreshes-41-43.md`, `repair-acceptance-and-frontier.md`, `refresh-ledger.md`, `recent-chronology.md`, and
   `legacy-chronology.md`. The index gives all 48 formal ids one primary route. Future work starts in one active
   `refreshes-49-56.md` part rather than reopening a sealed legacy part.
3. **Literal provenance:** copy the locked source byte-for-byte to
   `docs/archive/tasks/corpus-coverage/source-through-2026-08-10.md`. The capsule is an immutable terminal and
   never accepts future writes.

The existing neutral `scripts/check_active_task_evidence.pl` owns source/Git authentication, exhaustive region
coverage, raw-byte payload identity, routes, limits, manifest/index/root coherence, source-locked destination
absence, root-last writing, rollback, non-ASCII safety, sealing, and positive future continuation. Completion-
subject ids use `legacy` routes; the seven formal container ids absent from those subjects use source-backed
`structural` routes. A separate
`doctrine/live_document_size/corpus_task_evidence.json` carries all corpus-specific identity and policy. The
unconditional live-document driver invokes both contracts independently.

## Current-state precedence

The bounded root may normalize current state but may not alter literal history:

1. ID-bearing completion commits plus inline acceptance/verification evidence establish completed leaves.
2. Row 48, its cumulative statement, the newest changelog, current roadmap/status/book, and committed `.2.48`
   result jointly establish 48 done / eight remaining and the 80/20/80/79 stage census.
3. `.0`, `.1`, and `.3` are done; `.2` remains active because eight real documents remain.
4. Containment does not select #49. At migration the root states `No eligible product leaf.` and directs the
   next clean slice to create and own `CORPUS-COVERAGE.2.49`.
5. Every contrary older number remains byte-identical in sealed legacy payloads and the capsule.

## Writer transaction

Every post-migration corpus product leaf is one atomic transaction:

1. update the stable root's current counts, current leaf/checklist, frontier, blockers, and recent evidence;
2. update exactly one active semantic part with the complete leaf rationale, measurements, result, verification,
   and chronology;
3. update the index and manifest with the new fully qualified route and exact part metrics/state;
4. stage root plus owning part plus index/manifest and all genuinely impacted live/book/fact surfaces; and
5. run the corpus contract, all doctrines, and the ordinary risk-proportionate `COMMIT.md` gates.

Seven migrated legacy parts seal. The active continuation part is measured before every update. Warning triggers
review; before the next write would reach rollover, split at a refresh boundary and update root/index/manifest in
the same commit. No limit widens and no completed payload reopens.

## Local limits

Warning is 80%; rollover is 90%. Bounds derive from the seven planned parts (largest source payload 485 lines /
59,259 bytes / 4,746 maximum content-line bytes) and the 48-route index.

| Surface | Health targets | Inclusive ceilings |
| --- | --- | --- |
| stable active root | 224 lines / 24,576 bytes / 512 max-line bytes | 320 / 36,864 / 1,024 |
| bounded index | 128 lines / 16,384 bytes / 512 max-line bytes | 192 / 24,576 / 768 |
| semantic parts | 12 files; 640 lines / 76,800 bytes / 6,144 max-line bytes each; 4,096 lines / 524,288 bytes aggregate | 16 files; 896 / 98,304 / 6,400 each; 6,144 / 655,360 aggregate |
| exact capsule | exact 2,308 lines / 277,636 bytes / 4,746 max-line bytes | exact same values |

The schema-closed manifest permits at most 65,536 bytes, 1,024 line/scalar bytes, 16 parts, 32 regions, and
96 leaf routes.

## Migration stages

1. `CORPUS-TASK-EVIDENCE-CONTAINMENT.3a` extends the neutral checker only to the already-registered 6,400-byte
   direct task-evidence line ceiling and to source-backed formal container routes, with focused fail-closed cases.
2. `.3b` commits this decision and the complete `source_locked/complete` corpus contract at a clean boundary. It
   runs the neutral self-test and real corpus validation while the target remains byte-identical and every
   destination is absent.
3. `.4` reviews a bounded root template, invokes the neutral raw-byte/root-last writer, activates dedicated
   root/index/part/capsule surface records, proves exact reconstruction/routes/future continuation, synchronizes
   live docs/book/fact, and closes containment. Refresh #49 remains the next separate product slice.

## Consequences

- Startup/PNT keeps the stable task route but reads a bounded truthful root.
- All 48 established ids remain directly browsable; all 277,636 source bytes remain independently provable.
- Existing citations need no bulk rewrite.
- Task evidence is duplicated deliberately between live semantic payloads and exact terminal provenance, with
  independent lifecycle and bounds.
- A new top-level continuation, archive-only authority, and chronological-only authority remain rejected for this
  active parent because they break identity, browseability, or hierarchy respectively.

## Links

- `docs/tasks/CORPUS-TASK-EVIDENCE-CONTAINMENT.md`
- `docs/research/corpus-task-evidence-containment-census.md`
- `docs/decisions/0019-bounded-active-task-root-and-semantic-evidence-parts.md`
- `docs/knowledge/corpus-task-evidence-containment-design.md`
