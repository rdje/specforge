# BOOK-USER-FRIENDLY-BACKFILL: upgrade existing book subsections to the user-friendly standard

## Metadata

- Tree ID: `BOOK-USER-FRIENDLY-BACKFILL`
- Status: `active`
- Roadmap lane: `R0` (live-doc continuity / accuracy)
- Created: `2026-05-20`
- Last updated: `2026-05-20`
- Owner: repo-local workflow

## Goal

Bring every existing per-tree book method-doc subsection up to the
**user-friendly thorough** standard established `2026-05-20` (per
the `BOOK-METHOD-DOC.md` Decisions entry + the
`feedback-book-user-friendly` memory). The standard, recorded
verbatim from the user's direction:

> *"the user shall be able to understand everything; user-friendly;
> engage not scare; explain things so they understand; we want them
> to understand if we want them to use specforge."*

The worked template is the `R7-VALIDATION.5` subsection in
`docs/book/src/quality/validation.md` (rewritten `2026-05-20`).
Every concept-introducing subsection that pre-dates that rewrite
needs to meet the same bar.

## Non-Goals

- **Not** rewriting subsections that are already meeting the
  standard (e.g. the R7-VALIDATION.5 template itself).
- **Not** rewriting one-paragraph closure summaries that don't
  introduce new concepts (e.g. *"R6-SOURCE-HARDENING closed
  zero-coverage assertion gaps on SourceIR via mutation testing"*
  is concept-light and concise; it does what it needs to do).
  Per the AUDIT-DOC-RECONCILE doctrine, *accurate* is non-negotiable;
  *more-words* is not a goal when fewer is honest.
- **Not** modifying the task-tree files — those remain the spec
  authority; the book is the human-facing explanation.
- **Not** widening any tree's claims — book language describes what
  the code/design actually does, never more.

## Acceptance Criteria

- Every concept-introducing book subsection (the ones that
  introduce new typed records, new doctrines, new pipeline
  surfaces, etc.) meets the standard:
  - **thorough** — mirrors every concept the tree introduces,
    not just the typed names;
  - **accurate** — language describes what the design / code does;
  - **user-friendly** — explains *why* before *what*; introduces
    typed names alongside plain-language summaries; walks through
    worked examples; frames safety properties as user benefits;
  - **engage-not-scare** — opens with the user's problem, not the
    type lattice; uses second-person; short paragraphs; code
    blocks sparingly.
- `scripts/run_docs_ci.sh` green per leaf (mdBook builds).
- Every leaf via `COMMIT.md`.

## Scope audit (`.1` output, 2026-05-20)

Existing book subsections, classified by whether they need the
full user-friendly upgrade (concept-introducing) or are
acceptable as-is (concise closure summary):

| Subsection | File | Class | Action |
| --- | --- | --- | --- |
| `R16-CONTRACT-IR` — how it is implemented and verified + Status delivered | `direction/temporal-intent-capture.md` | concept-introducing (typed `ActorContract` / `Obligation` algebra) | upgrade (`.2`) |
| `R16-KG-PROTOCOL-ONTOLOGY` — how it is implemented and verified + Status | `direction/temporal-intent-capture.md` | concept-introducing (`Channel` / `ProtocolPhase` / `Transaction` / `HandshakePair`) | upgrade (`.2`) |
| `R16-CAPTURE-FIDELITY-GATES` — how it is implemented and verified + Status | `direction/temporal-intent-capture.md` | concept-introducing (6 gates / 3-valued `FindingStatus` / `FidelitySummary` / honesty routing) | upgrade (`.2`) |
| `R16-MULTIMODAL-CONTRACT-FUSION` — how it is implemented and verified + Status | `direction/temporal-intent-capture.md` | concept-introducing (`FusionKey` / `merge_cluster` / disagreement routing) | upgrade (`.2`) |
| `R16-WAVEFORM-CONTRACT-MINING` — how it is implemented and verified + `.3.1` + Status | `direction/temporal-intent-capture.md` | concept-introducing (`PartialTrace` / generalizer rules / round-trip verifier / `FigureRegion` / adapter) | upgrade (`.2`) |
| `R16-CONSTRAINED-VERIFIED-EXTRACTION` — how it is implemented and verified + Status R16 program complete | `direction/temporal-intent-capture.md` | concept-introducing (`parse_constrained_contract` / `entailment_check` / `ProtocolTemplate` / VoI selector) | upgrade (`.2`) |
| ISF / FSMGen 7-tree backfill | `pipeline/isf-adapter.md` | concise closure summaries; no new concepts beyond the .isf adapter chapter's own coverage | keep as-is |
| `AUDIT-DOC-RECONCILE` | `reference/documentation-scope.md` | doctrine-introducing (text-describes-code) | light upgrade — frame the doctrine in user terms (`.3`) |
| `SIGNOFF-REMEDIATION` | `reference/live-docs.md` | doctrine-introducing (signoff is non-negotiable; idiomatic-fixes-only) | light upgrade — frame the doctrine in user terms (`.3`) |
| R6 hardening trees (6 of them) | `pipeline/*` + `quality/corpus-memory.md` | concise closure summaries (mutation-testing methodology) | keep as-is |
| `PROVENANCE-HARDENING` | `quality/validation.md` | concise closure summary | keep as-is |
| `R7-VALIDATION` close-out summary | `quality/validation.md` | mostly concise but introduces the per-leaf finding-and-metric structure | light upgrade (`.4`) |
| `R7-VALIDATION.5` design | `quality/validation.md` | **already the worked template** | keep |
| `R15-GRAPH-DIRECTION-MIGRATION` | `domain/actor-connectivity.md` | doctrine-introducing (actor-relative graph as source of truth) | light upgrade — frame in user terms (`.5`) |

Honest scope total: **6 R16 family subsections** (`.2`) +
**2 doctrine subsections** in `reference/*` (`.3`) +
**1 closure summary** that introduces structure (`.4`) +
**1 doctrine subsection** in `domain/*` (`.5`). Concise closure
summaries (ISF/FSMGen + R6 + PROVENANCE) are left as-is per the
Non-Goal above — accurate is the bar; more-words is not.

## Task Tree

- ID: `BOOK-USER-FRIENDLY-BACKFILL.1`
  Status: `done` (`2026-05-20`)
  Goal: tree scaffolding + scope audit (recorded above).
  Acceptance: `Tree + scope audit recorded; chapters classified into upgrade-needed vs keep-as-is; mdBook still builds.`
  Verification: `passed` — see "Scope audit (.1 output)" above;
    `scripts/run_docs_ci.sh` green.
  Commit: `see Commit Log`

- ID: `BOOK-USER-FRIENDLY-BACKFILL.2`
  Status: `pending`
  Goal: upgrade the 6 R16-family subsections in
  `docs/book/src/direction/temporal-intent-capture.md` to the
  user-friendly standard (use the `R7-VALIDATION.5` template).
  Each subsection introduces typed records / doctrines a new user
  needs to understand to use SpecForge; this is the highest-impact
  leaf for user adoption.
  Acceptance: `All 6 R16 family subsections rewritten to meet the user-friendly standard; mdBook green; per-tree task-tree files unchanged.`
  Verification: `pending`
  Commit: `pending`

- ID: `BOOK-USER-FRIENDLY-BACKFILL.3`
  Status: `pending`
  Goal: light upgrade of the two `reference/*` doctrine
  subsections (`AUDIT-DOC-RECONCILE` →
  `reference/documentation-scope.md`; `SIGNOFF-REMEDIATION` →
  `reference/live-docs.md`) — frame each doctrine as a user
  benefit, with concrete examples of what the doctrine prevents.
  Acceptance: `Both subsections framed in user terms; the doctrine each tree established is presented as a property the user can rely on; mdBook green.`
  Verification: `pending`
  Commit: `pending`

- ID: `BOOK-USER-FRIENDLY-BACKFILL.4`
  Status: `pending`
  Goal: light upgrade of the `R7-VALIDATION` close-out summary in
  `quality/validation.md` so the per-leaf findings/metrics are
  framed as *what the user gets out of running validation today*,
  not a leaf-by-leaf changelog.
  Acceptance: `R7-VALIDATION close-out summary reads as a user guide to today's validation surface; mdBook green.`
  Verification: `pending`
  Commit: `pending`

- ID: `BOOK-USER-FRIENDLY-BACKFILL.5`
  Status: `pending`
  Goal: light upgrade of `R15-GRAPH-DIRECTION-MIGRATION` in
  `domain/actor-connectivity.md` — frame "actor-relative graph
  as the source of truth for signal direction" as the user-facing
  property, not a migration story.
  Acceptance: `R15 subsection presents actor-relative graph semantics as a property the user benefits from; mdBook green.`
  Verification: `pending`
  Commit: `pending`

- ID: `BOOK-USER-FRIENDLY-BACKFILL.6`
  Status: `pending`
  Goal: close the tree. Final verification: re-read every upgraded
  subsection cold; confirm a new user could understand the
  introduced concepts from the book alone; record this audit pass
  in the Verification Log; refresh the BOOK-METHOD-DOC Decisions
  entry with a cross-reference to this tree so the standard is
  fully self-referential.
  Acceptance: `Cold-read audit confirms each upgraded subsection is user-comprehensible; BOOK-METHOD-DOC Decisions cross-references this tree; tree marked done.`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `BOOK-USER-FRIENDLY-BACKFILL.1` | `done` | Scope audit recorded |
| 2 | `BOOK-USER-FRIENDLY-BACKFILL.2` | `pending` | **Next** — R16-family upgrade (highest user-impact surface; 6 subsections in `direction/temporal-intent-capture.md`) |
| 3 | `BOOK-USER-FRIENDLY-BACKFILL.3` | `pending` | reference/* doctrine subsections |
| 4 | `BOOK-USER-FRIENDLY-BACKFILL.4` | `pending` | R7-VALIDATION close-out summary |
| 5 | `BOOK-USER-FRIENDLY-BACKFILL.5` | `pending` | R15 actor-relative-graph framing |
| 6 | `BOOK-USER-FRIENDLY-BACKFILL.6` | `pending` | Close tree |

## Dependencies / Order

- Depends on `BOOK-METHOD-DOC` (closed `2026-05-20`) for the
  standard the upgrades target, and on
  [[feedback-book-user-friendly]] for the user-friendly clarification.
- Independent of every other R0 / R6 / R7 / R15 / R16 tree —
  this is pure docs-quality work.

## Decisions

- `2026-05-20`: Created and immediately promoted `active`. The
  standard was established same day during the `R7-VALIDATION.5`
  book rewrite (the worked template). Spinning up this tree
  immediately is the right move because the standard now structurally
  applies via the BOOK-METHOD-DOC close-rule, and existing
  subsections that don't meet it are a real (per
  AUDIT-DOC-RECONCILE) doc-quality gap.
- `2026-05-20`: Non-Goal recorded — concise closure summaries
  (1-paragraph "tree X closed gap Y") that don't introduce new
  concepts are KEPT AS-IS. The standard is *user-friendly thorough*
  for concept-introducing subsections, not *more-words for
  more-words' sake*. Accurate brevity remains the right answer
  for the ISF/FSMGen/R6/PROVENANCE 1-paragraph summaries.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-20` | `BOOK-USER-FRIENDLY-BACKFILL.1` | scope audit recorded; chapters classified; mdBook builds | `passed` (docs-only) |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `BOOK-USER-FRIENDLY-BACKFILL.1` | `BOOK-USER-FRIENDLY-BACKFILL.1 — create tree + scope audit + chapter classification` | docs-only; the new tree's own scaffolding |

## Changelog

- `2026-05-20`: Created and promoted `active` same day the
  user-friendly standard was established. `.1` scope audit
  recorded; frontier → `.2` (R16 family upgrade).
