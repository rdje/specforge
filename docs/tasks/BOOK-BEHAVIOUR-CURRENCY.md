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

- ID: `BOOK-BEHAVIOUR-CURRENCY` · Status: `active` (`.0` done `2026-09-11`; `.1` open) · Children: `.0`, `.1`

- ID: `BOOK-BEHAVIOUR-CURRENCY.0` · Status: `done` (`2026-09-11`) · Goal: **census what else is stale
  right now.** Before adding any obligation, measure the standing exposure rather than generalise from
  one case.

  **ARM 1 — deleted production functions, by symbol.** `git log --since=2026-06-22 -p` over
  `crates/specforge/src` yields 102 commits and 193 distinct removed `fn` names; **167 of those are
  genuinely gone** from the current tree (the rest were moved or renamed and still exist). Grepping all
  167 against `docs/book/src` returns **two hits, and both are false positives** — `infer` and
  `overlaps` match as ordinary English words, not symbol references. **No deleted production function
  is named by symbol anywhere in the book.** The symbol-scan arm this tree warned about therefore finds
  *nothing at all*, which is stronger than predicted: it is not merely blind to the `.4a` case, it has
  no true positives to justify itself on this corpus.

  **ARM 2 — present-tense behavioural claims, by assertion.** The `.4a` paragraph's real signature was
  `SpecForge **now** …`, so the census greps the book for present-tense behavioural assertions
  (`SpecForge now|currently <verb>`, and `now|currently (resolves|infers|uses|treats|derives|applies|
  promotes|accepts|recognises|canonicalises|expands|merges)`). That population is **18 claims across 10
  chapters** — small enough to adjudicate one by one.
  **16 re-derive as current**, each against a named seam: `RelationKind` carries exactly `Drives`/`Reads`
  (`ir/source.rs:729`); `auto-local` is the declared provider default (`cli.rs:393`); live-NLP promotion
  is on by default with `--no-promote-constraints-llm` to opt out (`cli.rs:126`); register width uses
  `size_bits ⊔ field_top` (`ir/isf_ir.rs:1403`); bounded activation derives from total RAM
  (`docling_backend.rs:1480`); `select_initiator_actor` and `initiator_perspective_directions` both
  exist; the quoted-state grammar exists (`evidence.rs:13510`); the connectivity rescan guidance exists
  (`validate.rs:86`); the `(fields (field …))` storage construct exists; the ledger registry pins
  `schema_version == 2`; and the EvidenceIR placeholder paragraph is `.10b`'s own, written today.
  **1 is STALE and is corrected here**: `reference/live-docs.md` said `docs/TASK_TREE.md` "now derives
  one concise row for all **121** real task trees" — the authority prints **152**. It is the same
  derived-state anti-pattern `STATUS-LEDGER-ROLLOVER.1` already removed from that very chapter for
  ledger sizes, so the count is replaced by the derive-on-read command, not by a fresher number.
  Note that no gate covered it: `trees` is not one of the units
  `check_book_quantitative_claims.pl` recognises, so the number was never a governed candidate.
  **1 is NOT re-derived and stays an explicit residual**: `pipeline/isf-adapter.md:1063-1064` claims the
  pinned FSMGen now accepts nested `(eventually s (within N))` and `(stage p (ready r)(valid v))`. That
  is a downstream-consumer claim needing an `fsmgen --strict --check` probe, which this census did not
  run; it is named rather than assumed.

  **What the census changes about `.1`.** The exposure is one stale claim, not a class — so `.1` should
  stay the cheap checklist obligation and must NOT grow into a scanner. Arm 1 says a symbol scanner
  would earn nothing here; arm 2 says the affordable instrument is the `now|currently` population,
  which is 18 lines and can simply be re-adjudicated when a producer is deleted.
  Non-goal: writing the enforcement; the FSMGen probe.
  Prerequisite: none.
  Verification: see this leaf's arms — every figure above is reproducible from the commands it names.
  Commit: see log.

- ID: `BOOK-BEHAVIOUR-CURRENCY.1` · Status: `open` (`2026-09-11`; **scoped down by `.0`'s census** — the
  standing exposure is one stale claim, so this stays a checklist obligation and must not grow into a
  scanner) · Goal: **make the obligation part of the acceptance checklist a slice already has to pass.** A deletion or replacement of a production
  rule must state, in its LOCKSTEP item, either the `reject_literal` it added for the phrasing that
  described the old behaviour or that no book text described it. This is enforceable by the existing
  task-acceptance check, which already reads the checklist, so it needs no new gate tier.
  Acceptance: the `TOOLBOX.md` template carries the sub-item; `scripts/check_task_acceptance.sh`
  recognises it; one worked example (the `.4a` rewrite, retrofitted) shows the intended shape.
  Prerequisite: `BOOK-BEHAVIOUR-CURRENCY.0` (so the obligation is sized by real exposure).
  Verification: pending
  Commit: pending
