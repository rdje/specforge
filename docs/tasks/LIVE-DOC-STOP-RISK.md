# LIVE-DOC-STOP-RISK: find the live-document stops that no current signal makes actionable

## Metadata

- Tree ID: `LIVE-DOC-STOP-RISK`
- Status: `active`
- Roadmap lane: repository durability and portability (successor of `FACT-CARD-CAPACITY-HEADROOM`)
- Created: `2026-08-11`
- Last updated: `2026-08-11`
- Owner: repo-local workflow

## Goal

`FACT-CARD-CAPACITY-HEADROOM.3` found that the fact plane's advertised capacity had never been reachable —
198 legal cards exceeded the plane's own aggregate line ceiling — and that the resulting stop had **no legal
exit**, because cards are canonical and are never deleted or rolled over. [ADR 0029](../decisions/0029-fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy.md)
fixed the four fact-plane surfaces and named the general rule: *pressure belongs on a dimension that has a
remedy.* The closing audit then measured the same shape on **ten other surfaces**, and separately found a
bounded snapshot 20 lines from a hard stop. This tree decides each case on its own evidence.

## Non-Goals

- Do not widen a ceiling to silence a warning. Every case is either re-derived with its own measured rationale
  or recorded as deliberately tight because it genuinely has a remedy.
- Do not delete or compact canonical content to buy headroom (`MEMORY_ARCHITECTURE.md` §10, ADR 0026).
- Do not change the live-document doctrine's schema, lifecycles, or milestone semantics.

## Measured findings (`2026-08-11`)

### Finding 1 — ten surfaces carry an aggregate ceiling tighter than their own legal maximum

Enumerated from `doctrine/live_document_size/surfaces.jsonl` over every multi-file surface, comparing each
aggregate ceiling with `files × per-file ceiling` — the largest total a corpus of individually-legal files can
produce:

| Surface | Lifecycle | Aggregate lines vs legal maximum | Ratio |
| --- | --- | --- | ---: |
| `task_evidence` | `partitioned_canonical` | 40,000 vs 160 × 3,000 = 480,000 | 12.0× |
| `kg_fixture_documents` | `partitioned_canonical` | 4,000 vs 256 × 80 = 20,480 | 5.1× |
| `corpus_knowledge_base` | `partitioned_canonical` | 4,000 vs 40 × 1,200 = 48,000 | 12.0× |
| `research_records` | `partitioned_canonical` | 12,000 vs 64 × 640 = 40,960 | 3.4× |
| `workflow_standards` | `partitioned_canonical` | 3,000 vs 16 × 700 = 11,200 | 3.7× |
| `fsmgen_issue_packets` | `partitioned_canonical` | 5,000 vs 32 × 512 = 16,384 | 3.3× |
| `canonical_collection_indexes` | `partitioned_canonical` | 1,024 vs 9 × 384 = 3,456 | 3.4× |
| `knowledge_map_bundle` | `partitioned_canonical` | 2,000 vs 8 × 512 = 4,096 | 2.0× |
| `active_task_evidence_parts` | `partitioned_canonical` | 9,600 vs 24 × 896 = 21,504 | 2.2× |
| `corpus_task_evidence_parts` | `partitioned_canonical` | 6,144 vs 16 × 896 = 14,336 | 2.3× |

A tighter aggregate is **legitimate where a remedy exists** — for a surface with an archive or bounded-parts
route, breaching the total is what triggers the migration. It is a **trap** where none exists: the state is
reachable by ordinary compliant writing and no compliant action leaves it. The two `*_task_evidence_parts`
surfaces have a documented parts route (ADR 0019, ADR 0024); the other eight need their remedy named or their
aggregate re-derived. `task_evidence` is already at 81.2% of its file target, so it is not hypothetical.

`fact_index` is deliberately excluded: its 12,384-line total is `max_landing_lines + max_shards ×
max_shard_lines`, which is that surface's exact legal maximum given a landing bound smaller than a shard bound.

### Finding 2 — `ROADMAP.md` is 20 lines from a hard stop, and its warning understates that

Measured 364 lines / 34,938 bytes against a **384-line / 49,152-byte enforcement ceiling**. The gate reports
`142.2% of health` because the health target is 256, so the loudest number in the report is about a target that
was already blown, not about the wall 20 lines away. The surface is `bounded_snapshot` and does have a remedy —
`doctrine/live_document_size/roadmap_projection.json` owns a current/history boundary with an archive at
`docs/archive/roadmap/` — but the remedy has not been applied and no signal distinguishes "past a soft target"
from "one slice from a hard failure".

## Acceptance Criteria

- Every surface in Finding 1 either names the concrete remedy its aggregate triggers, or has its aggregate
  re-derived under ADR 0029's rule, with the choice recorded per surface and no ceiling widened without its own
  measured rationale.
- `ROADMAP.md` is below its enforcement ceiling with enough headroom for ordinary roadmap-status edits, through
  its declared current/history boundary rather than by widening a bound or deleting direction.
- Whether pressure reporting should distinguish "past health target" from "near enforcement ceiling" is decided
  explicitly — implemented with fail-closed cases, or declined with a recorded reason.
- `scripts/check_doctrines.sh` passes; no canonical content is deleted.

## Task Tree

- ID: `LIVE-DOC-STOP-RISK`
  Status: `active`
  Goal: no live-document surface can reach a stop that compliant work cannot leave, and no near-stop is
  reported only as a stale soft-target percentage
  Children: `LIVE-DOC-STOP-RISK.0`, `.0a`, `.1`

- ID: `LIVE-DOC-STOP-RISK.0`
  Status: `done`
  Goal: roll `ROADMAP.md` through its declared current/history boundary before the 384-line ceiling stops an
  unrelated slice. This is first because it is the only finding with a measured deadline.
  Acceptance: `ROADMAP.md is below its enforcement ceiling with room for ordinary status edits; the archived history is byte-exact and reachable; the roadmap-projection contract, live-document gate, and book routes pass; no milestone or ceiling moves`
  Verification: `ROADMAP.md is 156 lines / 12,394 bytes (40.6% of the 384-line ceiling, 60.9% of the 256-line health target); docs/archive/roadmap/root-through-2026-08-11.md holds the exact 364-line / 34,938-byte / SHA-256 2cc2fa4a…7190 pre-rollover root; check_roadmap_projection_contract.pl --self-test is 31/31 and --check passes; check_live_document_size.sh passes with 736 files across 52 surfaces and no roadmap warning; scripts/check_doctrines.sh is 6/6; no health target, ceiling, or milestone changed`
  Commit: `LIVE-DOC-STOP-RISK.0 — give the bounded roadmap a declared repeatable rollover`

- ID: `LIVE-DOC-STOP-RISK.0a`
  Status: `pending`
  Goal: make the accretion that filled the root fail closed where it happens. `.0` gave the surface an exit
  but did not stop it being re-entered: the 213 retired lines were one prose bullet in one H2 section, and
  nothing bounds a section. Bound each current-root section so the signal names the growing section rather
  than the whole file.
  Acceptance: `the current_root contract declares a per-section line bound derived from the measured section shape; check_roadmap_projection_contract.pl fails closed on an over-bound section with focused cases for the boundary, an unknown section, and a missing declaration; the present root passes with recorded headroom per section; no file-level ceiling or milestone moves`
  Verification: `pending`
  Commit: `pending`

- ID: `LIVE-DOC-STOP-RISK.1`
  Status: `pending`
  Goal: decide each of the eight remedy-less surfaces in Finding 1 — name the remedy its aggregate triggers, or
  re-derive the aggregate as `files × per-file` under ADR 0029 — and settle whether near-ceiling pressure needs
  its own signal.
  Acceptance: `every listed surface has a recorded per-surface decision backed by its measured shape; any re-derivation carries a ceiling-increase authority and is retired after use; the near-ceiling reporting question is answered with cases or a recorded refusal; the gate passes`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `LIVE-DOC-STOP-RISK.0` | `done` | 20 lines of headroom was the only measured deadline here; every other finding is a latent shape |
| 2 | `LIVE-DOC-STOP-RISK.0a` | `pending` | `.0` proved the exit exists but left the entrance open; the root can re-accrete the same way |
| 3 | `LIVE-DOC-STOP-RISK.1` | `pending` | now has `.0`'s worked example of what "a surface with a declared remedy" actually costs |

## Decisions

- `2026-08-11`: track this as its own tree rather than reopening the closed `FACT-CARD-CAPACITY-HEADROOM`.
  That tree closed on its own evidence and owned one plane; this owns a class the closing audit measured
  elsewhere, on surfaces that tree never touched.
- `2026-08-11` (`.0`): seal the retired root byte-exact rather than prove 213 dense prose lines duplicate
  against ~130 task trees and `CHANGES.md`. Both satisfy the doctrine; the capsule costs 35 KB and is
  mechanically re-provable, while the duplicate proof costs a day and is only as good as the reader.
- `2026-08-11` (`.0`): make the rollover a *declared repeatable operation* rather than a second one-off
  migration ([ADR 0030](../decisions/0030-a-bounded-snapshot-needs-a-declared-repeatable-rollover.md)).
  `ROADMAP.md` filled twice in three days, so the question was never "how do we shrink it once" but "what
  is the compliant exit next time" — which is the tree's own goal applied to its first case.
- `2026-08-11` (`.0`): give the new capsule series its own count signal in the roadmap contract rather than
  rely on the live-document report. `archive_terminal` surfaces are exempt from milestone warnings by
  design, so a growing immutable collection is silent to its hard ceiling — exactly Finding 1's shape.
  Declaring `rollover_policy` where the remedy is declared avoids adding an instance of the defect.

## Open Questions

- Does a `partitioned_canonical` surface with a bounded-parts route count as having a remedy for its *root*
  aggregate, or only for its parts? `active_task_evidence_parts` and `corpus_task_evidence_parts` are the test
  cases.
- Should the live-document report rank findings by distance to the enforcement ceiling rather than by
  percentage of health target? Finding 2 is the motivating case: the loudest percentage was the least urgent
  fact about that surface.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-11` | ownership | enumerated every multi-file surface in `doctrine/live_document_size/surfaces.jsonl`, comparing each aggregate ceiling with `files × per-file ceiling`; read `ROADMAP.md` metrics from `check_live_document_size.pl --report` | 10 surfaces carry an aggregate tighter than their legal maximum, 8 of them with no named remedy; `ROADMAP.md` is 364/384 lines and 34,938/49,152 bytes |
| `2026-08-11` | `.0` root cause | read the 364-line root against its own `bounded_snapshot` contract | 213 of 364 lines were one `Current strategic priorities` bullet grown one sentence per closed leaf — chronology the lifecycle forbids and the file itself disclaims nine lines later; the contract's three `forbidden_literals` name legacy shapes and match none of it |
| `2026-08-11` | `.0` sealed identity | `cp ROADMAP.md docs/archive/roadmap/root-through-2026-08-11.md` then recomputed metrics with the checker's own routine | capsule is byte-exact: 364 lines / 34,938 bytes / 205 max line bytes / SHA-256 `2cc2fa4a39ff7a654ec0c24ec59a715d016bb0625a4846a9e583afc94fb77190` |
| `2026-08-11` | `.0` contract | `perl scripts/check_roadmap_projection_contract.pl --self-test` | 31/31 (9 pre-existing + 22 rollover cases); a first run failed `non-array rollovers rejected` because `return (LIST)` collapses to its last element in scalar context — fixed at the source by returning `@found` |
| `2026-08-11` | `.0` bounded root | recomputed `ROADMAP.md` metrics after the rewrite | 156 lines / 12,394 bytes / 205 max line bytes = 40.6% of the line ceiling, 60.9% of the line health target; ~100 lines before warning |
| `2026-08-11` | `.0` gate | `perl scripts/check_roadmap_projection_contract.pl --check`; `bash scripts/check_live_document_size.sh`; `bash scripts/check_doctrines.sh` | contract passes in migrated state; live-document gate passes with 736 Markdown files across 52 governed surfaces and no `roadmap` warning (was 142.2% of health); doctrines 6/6 PASS, `CHAIN-CURRENCY` DEFER as registered |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `LIVE-DOC-STOP-RISK` ownership | `LIVE-DOC-STOP-RISK — track live-document stops with no compliant exit` | surfaced by `FACT-CARD-CAPACITY-HEADROOM.3`'s closing audit |
| `LIVE-DOC-STOP-RISK.0` | `LIVE-DOC-STOP-RISK.0 — give the bounded roadmap a declared repeatable rollover` | ADR 0030; 364 → 156 lines with the retired root sealed byte-exact |
| `LIVE-DOC-STOP-RISK.0a` | `pending` | `pending` |
| `LIVE-DOC-STOP-RISK.1` | `pending` | `pending` |

## Changelog

- `2026-08-11`: Created from the measured class `FACT-CARD-CAPACITY-HEADROOM.3` found while fixing one instance
  of it, plus the `ROADMAP.md` near-ceiling measurement taken in the same pass.
- `2026-08-11`: `.0` closed. Root-causing the 364 lines found the growth was not diffuse — 213 of them were a
  single chronology bullet the lifecycle already forbids — and that the surface had no declared rollover at
  all, only ADR 0010's one-time migration. ADR 0030 makes the rollover a repeatable, data-declared,
  gate-proved operation with its own bounded and reported capsule series. `.0a` opened for the entrance the
  rollover does not close: nothing bounds a section, so the root can re-accrete the same way.
