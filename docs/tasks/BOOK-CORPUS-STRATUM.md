# BOOK-CORPUS-STRATUM: the book does not tell the owner that half its corpus cannot ground a number

## Metadata

- Tree ID: `BOOK-CORPUS-STRATUM`
- Status: `active`
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
  Status: `active`
  Goal: the book describes the corpus strata and the claim rule that separates them
  Children: `.1`

- ID: `BOOK-CORPUS-STRATUM.1` · Status: `pending` (opened `2026-09-18` by `EXTRACTION-QUALITY-GAUGE.3j.4`) · Goal:
  **add the strata to `docs/book/src/reference/generated-artifacts.md` and register the quantities.**
  The chapter already has the right home: `## Artifact lifecycle and cleanup` explains retention and why an
  unretained bundle is *unmeasurable*, which is the same idea one step short of the conclusion. Add a
  subsection there stating that the persisted corpus is two strata, that the canonical loader is what
  separates them, their current populations, that neither is deleted and neither is rebuilt for a schema
  bump, and that only the measured stratum may ground a current figure.
  The cost is the registry, and it is why this is its own leaf rather than a line folded into `.3j.4`:
  `book_quantitative_claims.jsonl` holds one `region` record per quantitative prose line, pinned by
  `line_range_sha256` **and line number**, with `expected_candidate_lines` and `expected_candidate_files`
  as declared denominators. Inserting digit-bearing prose into a book member adds candidates and shifts the
  line numbers of every later candidate in that file, so the slice must re-derive the denominators, re-pin
  the shifted regions with `python3 scripts/repin_claim_regions.py --check` then `--apply`, and adjudicate
  an outcome for each new region. `--produce` emits the derived candidate set, but `validate_contract` runs
  first, so the re-derivation is a read of the failing check's own output rather than a regeneration
  command. Prerequisite: none.
  Verification: pending
  Commit: pending

## Current Frontier

`BOOK-CORPUS-STRATUM.1` — the only leaf. Nothing blocks it; it was split out of
`EXTRACTION-QUALITY-GAUGE.3j.4` to keep the claim-registry re-derivation from riding along inside an
adjudication slice.

## Decisions

- `2026-09-18` — the strata belong in the **reference** chapter that owns `generated/`, not in the quality
  chapter that owns the gauge. The fact is about what the artifacts on disk *are*, and a reader meets them
  there first.

## Links

- `docs/decisions/0048-the-persisted-corpus-has-a-measured-and-a-historical-stratum.md`
- Fact card: `docs/knowledge/measured-stratum-promotion-population.md`
- `docs/tasks/extraction-quality-gauge/llm-path-family.md` (`.3j.4`)
