# MDBOOK-DOCTEST-HYGIENE: classify examples and make the live book doctest-safe

## Metadata

- Tree ID: `MDBOOK-DOCTEST-HYGIENE`
- Status: `done`
- Roadmap lane: cross-cutting mdBook product quality and executable documentation
- Created: `2026-08-08`
- Last updated: `2026-08-08`
- Owner: repo-local workflow

## Goal

Make `mdbook test docs/book` distinguish executable Rust from illustrative ISF, console, JSON, schema, and
pseudocode examples, then add an appropriate maintained gate so future book edits cannot silently reintroduce
misclassified fences.

## Non-Goals

- Do not rewrite the product explanations, examples, or historical R16 design content merely to satisfy Rust
  syntax.
- Do not treat ISF S-expressions, console output, JSON, schemas, or architecture diagrams as Rust.
- Do not weaken the existing `mdbook build`, book-current-truth, membership, or size contracts.
- Do not mix this cleanup into `ARTIFACT-PATH-PORTABILITY`; its `.2` commit remains independently reviewable.

## Acceptance Criteria

- Every fenced block in the affected chapters has an explicit truthful language/execution classification.
- Intended Rust examples compile as doctests or carry a narrowly justified `ignore`/`no_run` classification;
  illustrative non-Rust material is never passed to rustdoc.
- `mdbook test docs/book` and `mdbook build docs/book` both pass.
- The canonical docs/CI entrypoint runs the doctest gate, or a recorded decision explains why a different
  mechanically enforced boundary is safer.
- The Knowledge Map, task tree, live docs, and mdBook size authority stay aligned; every leaf commits through
  `COMMIT.md`.

## Task Tree

- ID: `MDBOOK-DOCTEST-HYGIENE` · Status: `done` (`2026-08-08`) · Children: `.0`–`.2`
- ID: `MDBOOK-DOCTEST-HYGIENE.0` · Status: `done` (`2026-08-08`) · Goal: reproduce and classify the latent doctest
  failure, prove its relationship to the canonical book/CI gate, and open a bounded repair tree.
- ID: `MDBOOK-DOCTEST-HYGIENE.1` · Status: `done` (`2026-08-08`; depends on `.0`) · Goal: classify and correct all affected
  fences without changing their rendered meaning; make both mdBook test and build pass.
- ID: `MDBOOK-DOCTEST-HYGIENE.2` · Status: `done` (`2026-08-08`; depends on `.1`) · Goal: add drift prevention to the
  canonical docs/CI workflow, update public/maintainer guidance, and close the tree with full verification.

## Acceptance Checklist (enforced) — `MDBOOK-DOCTEST-HYGIENE.0`

- [x] **REPRODUCE / MEASURE** — `mdbook test docs/book` deterministically reports 26 failures across four
  chapters: five in `pipeline/isf-adapter.md`, 18 in `direction/temporal-intent-capture.md`, one in
  `quality/validation.md`, and two in `reference/troubleshooting.md`.
- [x] **ROOT CAUSE (WHY + WHERE)** — mdBook passes unclassified/Rust fences to rustdoc. The failures are ISF
  S-expressions, console output, architecture diagrams, incomplete illustrative Rust type/API fragments, and
  similar examples that build/render correctly but are not standalone Rust doctests.
- [x] **ADDRESSED (verified)** — the gap is durably owned and split into content classification (`.1`) plus
  enforcement/closure (`.2`); no fence or book rendering changes in this measurement leaf.
- [x] **NO REGRESSION** — the canonical full CI and `mdbook build docs/book` passed immediately before this
  census; the optional doctest failure predates and is independent of `ARTIFACT-PATH-PORTABILITY.2`.
- [x] **GENERICITY** — the repair will classify examples by language and executability, not by chapter-specific
  text matching or deletion of examples that happen to fail.
- [x] **LOCKSTEP** — this task tree and its Knowledge Map fact own the new finding before any fence or CI change.

## Acceptance Checklist (enforced) — `MDBOOK-DOCTEST-HYGIENE.1`

- [x] **REPRODUCE / MEASURE** — the `.0` 26-failure command was rerun against the unchanged baseline before
  editing; all 34 fence openings across the four affected chapters were enumerated and classified.
- [x] **ROOT CAUSE (WHY + WHERE)** — 26 openings lacked truthful attributes: non-Rust diagrams, ISF, console,
  and formula blocks used bare fences, while ten intentionally incomplete Rust API/type fragments used ordinary
  `rust` fences. mdBook therefore asked rustdoc to compile material that was never presented as standalone code.
- [x] **ADDRESSED (verified)** — non-Rust material is explicitly `text`; incomplete illustrative Rust is
  `rust,ignore`; the three self-contained Rust examples remain executable `rust`; existing explicit Bash/text
  fences remain unchanged. No example prose or body changed.
- [x] **NO REGRESSION** — `mdbook test docs/book` moves from 26 failures to success, and
  `mdbook build docs/book` remains green with the same 36-page membership and rendered example bodies.
- [x] **GENERICITY** — classification follows language and standalone executability, not chapter name, failure
  text, or a blanket ignore rule. Future intended Rust can still be compiled by the doctest runner.
- [x] **LOCKSTEP** — task tree, Knowledge Map fact, live change/status ledgers, resume pointer, and maintained
  book aggregate authority agree that content classification is complete and `.2` owns enforcement.

## Acceptance Checklist (enforced) — `MDBOOK-DOCTEST-HYGIENE.2`

- [x] **REPRODUCE / MEASURE** — `bash scripts/run_docs_ci.sh` passes after processing all 13 Rust-classified
  fences: three executable examples and ten explicitly ignored illustrations; the same 36-page book then builds.
- [x] **ROOT CAUSE (WHY + WHERE)** — the canonical docs entrypoint previously invoked only `mdbook build`, so
  fence classification could drift while both the rendered book and full CI remained green.
- [x] **ADDRESSED (verified)** — `scripts/run_docs_ci.sh` now runs `mdbook test docs/book` before
  `mdbook build docs/book`; `scripts/run_ci.sh` already consumes that entrypoint exactly once.
- [x] **NO REGRESSION** — the focused docs entrypoint and final full `scripts/run_ci.sh` both pass from the
  completed tree, including all doctrines, Rust gates, rustdoc, doctests, HTML build, and locality checks.
- [x] **GENERICITY** — enforcement compiles every future executable Rust fence through mdBook's native test
  command; it contains no chapter, example, or failure-message allowlist.
- [x] **LOCKSTEP** — the task tree, Knowledge Map fact, live ledgers, close-leaf mdBook method record, resume
  pointer, and maintained-book aggregate authority describe the same closed gate.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | None | `done` | The bounded tree is exhausted. |

## Decisions

- `2026-08-08`: Treat the failure as a latent documentation-test gap, not a `.2` regression. The canonical
  `scripts/run_ci.sh` invokes `scripts/build_docs.sh`/`mdbook build` and passed after the Source/Evidence changes;
  the separate `mdbook test` command interprets older illustrative fences as Rust.
- `2026-08-08`: Do not bulk-label all fences `text` or `ignore`. `.1` must distinguish executable Rust from
  ISF, JSON, shell/console, schema, diagrams, and pseudocode so useful examples remain truthfully classified.
- `2026-08-08`: Put `mdbook test` immediately before `mdbook build` in `scripts/run_docs_ci.sh`. That is the
  one docs-only entrypoint used directly by maintainers and exactly once by full CI, so one edit closes both
  enforcement paths and doctest failures remain distinct from rendering failures.

## Open Questions

- None.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-08` | `.0` | filtered and full `mdbook test docs/book`; canonical full CI/book-build evidence cross-check | 26 failures / 4 chapters reproduced; canonical build and CI unaffected |
| `2026-08-08` | `.1` | exact 34-fence opening census; `mdbook test docs/book`; `mdbook build docs/book` | 26 failures → 0; test/build pass; example bodies unchanged |
| `2026-08-08` | `.2` | `bash scripts/run_docs_ci.sh`; `bash scripts/run_ci.sh` | canonical doctest/build entrypoint and final full CI pass |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.0` | `MDBOOK-DOCTEST-HYGIENE.0 — own the latent book doctest gap` | Measurement and ownership only; `.1` changes fences. |
| `.1` | `MDBOOK-DOCTEST-HYGIENE.1 — classify live-book examples truthfully` | 26 openings corrected; `.2` owns enforcement. |
| `.2` | `MDBOOK-DOCTEST-HYGIENE.2 — enforce book doctests in docs CI` | Canonical docs/full-CI enforcement; tree closed. |

## Changelog

- `2026-08-08`: Opened after the post-`.2` optional mdBook test exposed 26 pre-existing examples that render
  successfully but are misclassified as standalone Rust doctests.
- `2026-08-08`: Classified every affected fence by actual language and standalone executability; mdBook
  doctesting now passes without changing example bodies or suppressing the three executable Rust examples.
- `2026-08-08`: Added native mdBook doctesting to the canonical docs entrypoint before the HTML build, updated
  public workflow guidance, passed focused and full gates, and closed the tree.
