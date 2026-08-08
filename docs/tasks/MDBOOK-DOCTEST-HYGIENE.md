# MDBOOK-DOCTEST-HYGIENE: classify examples and make the live book doctest-safe

## Metadata

- Tree ID: `MDBOOK-DOCTEST-HYGIENE`
- Status: `active`
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

- ID: `MDBOOK-DOCTEST-HYGIENE` · Status: `active` · Children: `.0`–`.2`
- ID: `MDBOOK-DOCTEST-HYGIENE.0` · Status: `done` (`2026-08-08`) · Goal: reproduce and classify the latent doctest
  failure, prove its relationship to the canonical book/CI gate, and open a bounded repair tree.
- ID: `MDBOOK-DOCTEST-HYGIENE.1` · Status: `done` (`2026-08-08`; depends on `.0`) · Goal: classify and correct all affected
  fences without changing their rendered meaning; make both mdBook test and build pass.
- ID: `MDBOOK-DOCTEST-HYGIENE.2` · Status: `pending` (depends on `.1`) · Goal: add drift prevention to the
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

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `MDBOOK-DOCTEST-HYGIENE.2` | `pending` | Gate the now-green doctest command and close the tree. |

## Decisions

- `2026-08-08`: Treat the failure as a latent documentation-test gap, not a `.2` regression. The canonical
  `scripts/run_ci.sh` invokes `scripts/build_docs.sh`/`mdbook build` and passed after the Source/Evidence changes;
  the separate `mdbook test` command interprets older illustrative fences as Rust.
- `2026-08-08`: Do not bulk-label all fences `text` or `ignore`. `.1` must distinguish executable Rust from
  ISF, JSON, shell/console, schema, diagrams, and pseudocode so useful examples remain truthfully classified.

## Open Questions

- Which incomplete Rust-shaped fragments should become self-contained `no_run` examples versus explicitly
  illustrative `rust,ignore` blocks? Owner: `.1`; decide from the surrounding teaching purpose.
- Should `scripts/build_docs.sh` run `mdbook test` before build, or should `run_ci.sh` call it separately for a
  clearer failure boundary? Owner: `.2`; decide after measuring runtime and failure output on the repaired book.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-08` | `.0` | filtered and full `mdbook test docs/book`; canonical full CI/book-build evidence cross-check | 26 failures / 4 chapters reproduced; canonical build and CI unaffected |
| `2026-08-08` | `.1` | exact 34-fence opening census; `mdbook test docs/book`; `mdbook build docs/book` | 26 failures → 0; test/build pass; example bodies unchanged |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.0` | `MDBOOK-DOCTEST-HYGIENE.0 — own the latent book doctest gap` | Measurement and ownership only; `.1` changes fences. |
| `.1` | `MDBOOK-DOCTEST-HYGIENE.1 — classify live-book examples truthfully` | 26 openings corrected; `.2` owns enforcement. |

## Changelog

- `2026-08-08`: Opened after the post-`.2` optional mdBook test exposed 26 pre-existing examples that render
  successfully but are misclassified as standalone Rust doctests.
- `2026-08-08`: Classified every affected fence by actual language and standalone executability; mdBook
  doctesting now passes without changing example bodies or suppressing the three executable Rust examples.
