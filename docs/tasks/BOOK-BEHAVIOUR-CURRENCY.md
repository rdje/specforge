# BOOK-BEHAVIOUR-CURRENCY: the book can describe behaviour the code no longer has, and no gate sees it

## Metadata

- Tree ID: `BOOK-BEHAVIOUR-CURRENCY`
- Status: `active`
- Roadmap lane: `R0` (live-doc continuity / accuracy)
- Created: `2026-09-11`
- Last updated: `2026-09-11`
- Owner: repo-local workflow

## Goal

The mdBook is the owner's only window into the project, and the owner-set contract is that it
"must always reflect what the codebase actually does". `WIRE-BASED-100.4a` found it did not, for a
month, in a chapter nobody suspected — and every gate was green throughout.

Make a **deleted or replaced production rule** unable to leave its book description standing.

## The measured case

`docs/book/src/domain/temporal-semantics.md` carried a section titled *"Indexed signal families in
antecedents"* stating: *"SpecForge **now resolves** an un-indexed prose reference to its declared
indexed family member (`PSEL` → `PSELx`, including numeric indices like `FOO0`) using the universal
`x`/digit index convention."*

`f88d463d` (`2026-08-12`, `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.ii`) deleted
`resolve_indexed_signal_family` while implementing ADR 0037, and **pinned the opposite behaviour as a
unit control** (`temporal_condition_does_not_infer_numeric_or_x_index_aliases`). So for roughly one
month the book asserted, as current behaviour, the exact thing the test suite proved the product
refuses. `WIRE-BASED-100.4a` rewrote the section on `2026-09-11`.

## Why no gate saw it

`scripts/check_book_current_truth.sh` is real and it works — it is a **hand-curated list of literal
pairings**: for about twenty high-value claims it requires a code literal to exist *and* the book to
carry the matching sentence, plus `reject_literal` entries that fail if a known stale phrasing comes
back. `LIVE-DOC-SIZE`, `book-quantitative-claims`, `published-assertions` and `roadmap-projection`
each police a different axis (size, numbers, published values, routes).

None of them is coverage. **A behavioural claim enters the book with no obligation to enrol a
pairing**, so the gate's reach is exactly what past authors happened to think of. The temporal
chapter was never enrolled, so deleting the producer it described moved nothing red.

## The obvious mechanical fix does not catch this case, and saying so is the point

The tempting version — scan the book for backticked Rust-looking identifiers and require each to
exist in the tree or be declared historical — is cheap, useful, and **would not have caught this
one**: the stale paragraph names no symbol. It describes the behaviour in prose (*"the universal
`x`/digit index convention"*). A symbol scan is worth having for what it does catch, but it must not
be mistaken for the fix, the way `WIRE-BASED-100.10b`'s suffix-shape rule must not be mistaken for a
template detector.

## Non-Goals

- Do not widen, weaken, or bypass `check_book_current_truth.sh`; it is the mechanism, not the problem.
- Do not rewrite book chapters for style. This tree is about the pairing obligation, not prose.
- Do not fabricate a pairing for a claim nobody can name an oracle for; an unpaired behavioural claim
  is a finding, not a line to satisfy the gate with.

## Acceptance Criteria

- The obligation is **stated where a slice will meet it** — the `TOOLBOX.md` acceptance checklist's
  LOCKSTEP item, so removing or replacing a production rule requires either a `reject_literal` for the
  phrasing that described it or an explicit "no book text described this" statement.
- A measured census: for every production function deleted since the doctrine gate landed, whether any
  book paragraph still describes it.
- The symbol-scan arm, if built, ships with its own measured false-negative statement — the case above
  is its first known miss.
- `scripts/check_doctrines.sh` green; no ceiling, milestone, or contract widened.

## Task Tree

- ID: `BOOK-BEHAVIOUR-CURRENCY` · Status: `active` · Children: `.0`, `.1`

- ID: `BOOK-BEHAVIOUR-CURRENCY.0` · Status: `open` (`2026-09-11`) · Goal: **census what else is stale
  right now.** Before adding any obligation, measure the standing exposure: enumerate production
  functions removed since `2026-06-22` (when the doctrine gate landed) from `git log -S`, and for each
  one check whether a book paragraph still describes its behaviour. The `.4a` case is one confirmed
  member; the census says whether it is the only one. Acceptance: a per-symbol table with the deleting
  commit, the book path and line if any, and a verdict; every stale paragraph either corrected or
  owned by a named leaf. Non-goal: writing the enforcement.
  Prerequisite: none.
  Verification: pending
  Commit: pending

- ID: `BOOK-BEHAVIOUR-CURRENCY.1` · Status: `open` (`2026-09-11`) · Goal: **make the obligation part of
  the acceptance checklist a slice already has to pass.** A deletion or replacement of a production
  rule must state, in its LOCKSTEP item, either the `reject_literal` it added for the phrasing that
  described the old behaviour or that no book text described it. This is enforceable by the existing
  task-acceptance check, which already reads the checklist, so it needs no new gate tier.
  Acceptance: the `TOOLBOX.md` template carries the sub-item; `scripts/check_task_acceptance.sh`
  recognises it; one worked example (the `.4a` rewrite, retrofitted) shows the intended shape.
  Prerequisite: `BOOK-BEHAVIOUR-CURRENCY.0` (so the obligation is sized by real exposure).
  Verification: pending
  Commit: pending
