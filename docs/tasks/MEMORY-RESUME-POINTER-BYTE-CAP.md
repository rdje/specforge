# MEMORY-RESUME-POINTER-BYTE-CAP: set the durable resume pointer's one-read ceiling

## Metadata

- Tree ID: `MEMORY-RESUME-POINTER-BYTE-CAP`
- Status: `done`
- Roadmap lane: repository durability and portability
- Created: `2026-08-15`
- Last updated: `2026-08-15`
- Owner: repo-local workflow

## Goal

Make exactly 32,768 bytes (`32 * 1024 B`) the repository's durable maximum file size for `MEMORY.md`, as
directed by the project owner, while preserving its independent overwrite-only, line-count, and line-width
constraints and keeping the portable memory architecture, executable gates, live-size registry, mdBook, and
resume state in lockstep.

## Non-Goals

- Do not turn `MEMORY.md` into a history log or relax its 50-line or 160-byte content-line limits.
- Do not widen another live-document surface or change product behavior.
- Do not reopen the closed memory-architecture or live-document-adoption task trees.

## Acceptance Criteria

- The memory-architecture checker defaults to an exact 32,768-byte `MEMORY.md` ceiling and still permits an
  explicit environment override for portable adoption/testing.
- The `active_resume` live-document surface declares the same exact per-file and aggregate byte maximum.
- The portable architecture and mdBook explain the independent one-read byte ceiling without weakening routing,
  overwrite-only behavior, or line/line-width containment.
- Focused positive and fail-closed checks, mdBook test/build, live-size enforcement, and all doctrines pass.
- The completed leaf is committed through `COMMIT.md` before product PNT resumes.

## Task Tree

- ID: `MEMORY-RESUME-POINTER-BYTE-CAP`
  Status: `done`
  Goal: enforce and document the exact 32,768-byte resume-pointer maximum
  Children: `.1`, `.2`

- ID: `MEMORY-RESUME-POINTER-BYTE-CAP.1`
  Status: `done`
  Goal: align the byte ceiling across executable memory/live-size gates and every reader-facing authority
  Acceptance: exact default and registry limits agree; current pointer and boundary controls pass; docs/book,
    retrieval, task catalog, and doctrines remain synchronized
  Verification: `default checker reports 2,610/32,768 bytes; a 2,609-byte override fails on the exact current
    size; active_resume reports 2,610 bytes / 32 lines / 116 max-line bytes under 32,768 / 50 / 160; 84/84
    live-document cases, 227-card catalog, 243-fact / 1,860-question Knowledge Map, mdBook test/build, and all
    doctrines pass`
  Commit: `MEMORY-RESUME-POINTER-BYTE-CAP.1 — set MEMORY.md maximum to 32,768 bytes`

- ID: `MEMORY-RESUME-POINTER-BYTE-CAP.2`
  Status: `done`
  Goal: retire the consumed exact ceiling-increase authority after `.1` establishes the clean Git boundary
  Acceptance: the authority record is deleted only after `.1` commits; checker and registry retain 32,768 bytes;
    the live-size/doctrine gate rejects no unused or banked authority; resume pointer returns to product PNT
  Verification: `committed boundary 7e42d0f3 contains the exact 4,096→32,768-byte authority and increase;
    deleting only the consumed authority leaves checker/registry equality at 32,768 bytes; live-size and all
    doctrines pass without unused or banked authority`
  Commit: `MEMORY-RESUME-POINTER-BYTE-CAP.2 — retire consumed ceiling authority`

## Current Frontier

No active frontier. Product PNT resumes at `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii.a` clean-revision comparator
replay and publication.

## Decisions

- `2026-08-15`: set the maximum to exactly 32,768 bytes. The separate 50-line and 160-byte content-line limits
  remain because they enforce resume-pointer concision and readability; the byte ceiling is the outer one-read
  safety boundary, not permission to accumulate chronology.
- `2026-08-15`: use a new task tree because the two prior architecture/containment trees are closed and explicitly
  forbid reopening. This bounded policy change returns directly to `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii.a`.
- `2026-08-15`: carry the exact old/new ceiling authority in `.1`, then delete it in `.2` after the clean commit.
  The live-size protocol rejects both an unexplained increase and an authority banked beyond its transaction.

## Open Questions

- None.

## Blockers

- None.

## Acceptance Checklist (enforced) — `MEMORY-RESUME-POINTER-BYTE-CAP.1`

- [x] **REPRODUCE / MEASURE** — prior checker/registry maximum was 4,096 bytes and the opening pointer was 3,284.
- [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/check_memory_architecture.sh` and the `active_resume` registry record
  were independent executable authorities; architecture, local containment, mdBook, and retrieval are readers.
- [x] **ADDRESSED (verified)** — default checker and `active_resume` registry maximum both equal 32,768 bytes.
- [x] **NO REGRESSION** — exact positive/negative boundary checks, 84/84 live-size cases, mdBook test/build,
  catalogs/retrieval, project locality, and doctrines pass.
- [x] **GENERICITY** — portable environment override and independent concision limits remain intact.
- [x] **LOCKSTEP** — task tree/catalog, memory architecture, live-size authority, mdBook, fact retrieval, and
  `MEMORY.md` publish one non-conflicting policy.

## Acceptance Checklist — `MEMORY-RESUME-POINTER-BYTE-CAP.2`

- [x] **REPRODUCE / MEASURE** — committed revision `7e42d0f3` contains the exact authority and matching increase.
- [x] **ROOT CAUSE (WHY + WHERE)** — ceiling authority is transactional; retaining it after the increase is Git
  history would bank permission for a future unrelated change and must fail the live-size gate.
- [x] **ADDRESSED (verified)** — the consumed data-only record is removed while both live ceilings remain 32,768.
- [x] **NO REGRESSION** — live-size and composed doctrine checks pass without an unused-authority violation.
- [x] **GENERICITY** — the generic authority lifecycle is followed unchanged; no checker exception is added.
- [x] **LOCKSTEP** — task/catalog and resume pointer close the policy unit and restore the product frontier.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-15` | `.1` | opening census | checker and `active_resume` registry both cap `MEMORY.md` at 4,096 bytes; current pointer is 3,284 bytes |
| `2026-08-15` | `.1` | checker boundary | default accepts 2,610/32,768 bytes; an explicit 2,609-byte override rejects the same file; 50-line and 160-byte content-line gates remain active |
| `2026-08-15` | `.1` | synchronized gates | `active_resume` 2,610 bytes / 32 lines / 116 max-line bytes; 84/84 live-size cases; 145 task routes; 227 cards; 243 facts / 1,860 questions; mdBook test/build; doctrines green |
| `2026-08-15` | `.2` | authority retirement | committed `.1` boundary `7e42d0f3`; consumed record deleted; exact 32,768-byte checker/registry limits retained; live-size and doctrines green with no banked authority |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.1` | `MEMORY-RESUME-POINTER-BYTE-CAP.1 — set MEMORY.md maximum to 32,768 bytes` | exact maximum and synchronized enforcement/documentation |
| `.2` | `MEMORY-RESUME-POINTER-BYTE-CAP.2 — retire consumed ceiling authority` | retire the consumed one-transaction authority and restore the product frontier |

## Changelog

- `2026-08-15`: created and activated from the owner's exact 32,768-byte directive after clean production commit
  `2cdcd131`; no prior closed task tree is reopened.
- `2026-08-15`: `.1` establishes exact checker/registry agreement and preserves independent concision rules;
  `.2` must retire the consumed authority before return to the interrupted product frontier.
- `2026-08-15`: `.2` removes the authority after committed boundary `7e42d0f3`, closes the tree, and restores the
  exact product resume pointer without changing the new limit.
