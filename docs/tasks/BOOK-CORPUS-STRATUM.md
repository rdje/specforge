# BOOK-CORPUS-STRATUM: the book does not tell the owner that half its corpus cannot ground a number

## Metadata

- Tree ID: `BOOK-CORPUS-STRATUM`
- Status: `done` (`2026-09-18`; `.1` shipped the section and registered its one quantity)
- Roadmap lane: `R0` (live-doc continuity / accuracy)
- Created: `2026-09-18`
- Last updated: `2026-09-18`
- Owner: repo-local workflow
- Opened by: `EXTRACTION-QUALITY-GAUGE.3j.4`

## Goal

Make the two persisted-corpus strata visible in the mdBook, with their populations and the rule that
governs them, so a reader of the book alone cannot mistake a figure measured over the historical stratum
for a current one.

`ADR 0048` established that `generated/` is two populations — a **measured** stratum whose artifacts the
canonical loader accepts and which alone may ground a current actionable claim, and a **historical**
stratum that is inspection-only and is dated evidence about its own era. The book is the owner's only
window into the project, and it says none of this. It describes the *mechanism* well — proof ledgers,
schema refusal, why a reclaimed bundle is honestly unmeasurable — but never the *population* or the
*claim rule* that follows from it. A reader who sees a book figure has no way to tell which stratum it
came from, and `EXTRACTION-QUALITY-GAUGE.3j` published one that came from the wrong one.

## Non-Goals

- Do not restate `ADR 0048` in the book. The book states what a user must know; the ADR keeps the rationale.
- Do not re-audit existing book figures for stratum provenance in this tree. That is a census of its own and
  is only worth opening if `.1` shows the ambiguity is load-bearing beyond the one known case.
- Do not change any production behaviour, artifact, or gate. This is a documentation currency tree.
- Do not evade the quantitative-claim registry by spelling numbers out in words. Registering the new
  quantities is the point of `.1`, not an obstacle to route around.

## Acceptance Criteria

- The book states, in the reference chapter that already owns `generated/`, that the persisted corpus has
  two strata, gives each one's current population, and states which one may ground a current number.
- Every new quantitative prose line is adjudicated in `doctrine/claim_verification/book_quantitative_claims.jsonl`
  with an outcome and, where the outcome is `derived`, a re-runnable authority — not left as an untracked
  candidate and not registered as `incomplete` merely to pass.
- `perl scripts/check_book_quantitative_claims.pl --check` and the full doctrine driver pass, with the
  candidate-line and candidate-file denominators **re-derived** rather than edited to match.
- A reader who knows only the book can answer "may I quote this figure as current?" for the corpus.

## Task Tree

- ID: `BOOK-CORPUS-STRATUM`
  Status: `done`
  Goal: the book describes the corpus strata and the claim rule that separates them
  Children: `.1`

- ID: `BOOK-CORPUS-STRATUM.1`
  Status: `done` (`2026-09-18`, DOC)
  Goal: **add the strata to `docs/book/src/reference/generated-artifacts.md` and register the quantities.**
  Shipped as `### Two strata, and only one of them can carry a current number`, placed inside
  `## Artifact lifecycle and cleanup` immediately after the retention subsection — retention already
  explains which documents *can* be replayed, and this is the sentence it stops one short of: what a
  number measured over them is worth.
  **The figure is published as dated evidence, not as a current one, and that was a decision rather than
  a convenience.** A population that every ingest moves cannot be frozen into a page that no gate re-derives.
  Three legs were weighed against `CLAIM_VERIFICATION.md`: RE-DERIVE is earned — the section names the
  command — and there is even a second, dimensionally different oracle in `ADR 0048`'s proof-ledger grep.
  FALSIFY has no tracked known-bad RED case, and DURABILITY has no stale-state gate that reddens when a
  re-ingest moves the split. Publishing `27` as a standing figure would therefore have asserted two legs
  this repository does not hold. The section states the **rule**, which does not rot; gives the command,
  which is always current; and records the split as an explicitly dated observation under the registry's
  own `dated_boundary_evidence` scope — the category `ADR 0048` §3 created.
  **The phrasing was corrected once, in the honest direction.** The first draft read *"27 measured against
  51 historical"*, and the candidate detector did not see it: its unit vocabulary is a closed list and
  `split` is not in it. A published quantity sitting in a known blind spot of its own gate is worse than
  no gate, so the line was rewritten to *"27 measured documents against 51 historical documents"* — which
  the detector does see — and adjudicated. The gate was made to look, not looked away from.
  Prerequisite: none. Blocks: nothing.
  Verification: the book claim registry re-derivation, the repository-relative link convention, and the
  doctrine gate
  Commit: `BOOK-CORPUS-STRATUM.1 — tell the reader which half of the corpus can carry a number`

### Acceptance Checklist (enforced) — `BOOK-CORPUS-STRATUM.1`

- [x] **REPRODUCE / MEASURE** — before: `grep -rn "measured stratum\|historical stratum" docs/book/src`
  returned nothing, and the one place the book used the word *measured* for a corpus
  (`commands/quality-and-learning.md`) meant the **opposite** stratum; `EXTRACTION-QUALITY-GAUGE.3j.4`
  fixed that term and left the gap. After: `perl scripts/check_book_quantitative_claims.pl --check`
  reports **42 book files / 476 prose candidate lines across 25 files / 476 adjudicated regions**, up one
  from 475, and the one new region is this section's dated split.
- [x] **ROOT CAUSE (WHY + WHERE)** — `ADR 0048` was accepted on `2026-09-18` and its commit touched 26
  files, none of them under `docs/book/`. The reference chapter that owns `generated/` described the
  *mechanism* thoroughly — proof ledgers at `generated-artifacts.md:166`, schema refusal at `:226`, why an
  unretained bundle is honestly *unmeasurable* at `:112-118` — and never stated the population or the
  claim rule those add up to. A reader of the book alone could not tell which stratum a figure came from.
- [x] **ADDRESSED (verified)** — the section states both strata, names the canonical loader as what
  separates them, gives the dated split and the command that re-derives it, states the rule that only the
  measured stratum grounds a current claim, and records the two things that deliberately do not follow
  (nothing is deleted, nothing is rebuilt for a schema bump). The worked example is the failure that
  motivated the ADR, so the rule arrives with its evidence attached. The ADR link uses the book's own
  `../../../decisions/` convention; the first draft's absolute `github.com` URL would have been the only
  one in the entire book and violates the repository-root-relative path rule.
- [x] **NO REGRESSION** — `perl scripts/check_book_quantitative_claims.pl --check` green with the
  denominator **re-derived**, not edited to match: `expected_candidate_lines` 475 → 476 because exactly one
  candidate was added, and `expected_candidate_files` is unmoved at 25 because this file already carried a
  region. `python3 scripts/repin_claim_regions.py --check` reports no shifted region — the file's only
  prior region is at line 66, above the insertion. Documentation only: no Rust, no artifact, no gate logic.
- [x] **GENERICITY (ADR 0006)** — N/A: no production rule. The section names no vendor, protocol or
  document; the strata are defined by what the canonical loader does.
- [x] **LOCKSTEP** — this leaf *is* the book change, so the `BOOK-METHOD-DOC` close rule is satisfied by
  its own deliverable: the tree's method and verification are the section it adds. No production rule was
  deleted. Fact card `[[measured-stratum-promotion-population]]` already carries the machine-side facts and
  is not duplicated here; the book carries the reader-side rule.

## Current Frontier

None — `.1` closed on `2026-09-18` and it was the only leaf. The tree stays as project history.

One residual is named rather than hidden: the dated split is published under `dated_boundary_evidence`
because the repository holds no staleness gate that reddens when a re-ingest moves it. Building one is a
real piece of work and belongs to whichever tree next needs the population to be a *current* claim —
`EXTRACTION-QUALITY-GAUGE.3j.4.a` is the first that will.

## Decisions

- `2026-09-18` — the strata belong in the **reference** chapter that owns `generated/`, not in the quality
  chapter that owns the gauge. The fact is about what the artifacts on disk *are*, and a reader meets them
  there first.

## Links

- `docs/decisions/0048-the-persisted-corpus-has-a-measured-and-a-historical-stratum.md`
- Fact card: `docs/knowledge/measured-stratum-promotion-population.md`
- `docs/tasks/extraction-quality-gauge/llm-path-family.md` (`.3j.4`)
