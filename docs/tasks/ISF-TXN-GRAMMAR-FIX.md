# ISF-TXN-GRAMMAR-FIX: correct SpecForge's `.isf` transaction-step emitter grammar

## Metadata

- Tree ID: `ISF-TXN-GRAMMAR-FIX`
- Status: `done`
- Roadmap lane: `R6` (`.isf` adapter)
- Created: `2026-05-29`
- Last updated: `2026-05-29`
- Owner: repo-local workflow

## Goal

Fix the latent grammar bugs in SpecForge's `.isf` transaction-step emitter
(`isf_ir.rs::render_txn_step`) surfaced by the `FSMGEN-REFRESH-INTEGRATE.2`
assessment and **verified against the FSMGen book/contract grammar** (the
authority — *parser acceptance ≠ support*, [[feedback-fsmgen-contract]]):

- `shift-left` / `shift-right` (hyphen) → `shift_left` / `shift_right`
  (book `13e-data-manipulation.md:42`: "The form is exact:
  `(shift_left reg bit)` …").
- `await-all` / `await-any` (hyphen) → `await_all` / `await_any`
  (book `13f-composition.md`: `(await_all done)` / `(await_any done)`).
- `(spawn child instance)` → `(spawn child as instance)` — the `as`
  keyword is mandatory (book `13f-composition.md:120,151`: "The base form
  is exact: `(spawn child as name)`").

These steps are reachable via the `TransactionStep::{Spawn,AwaitAll,
AwaitAny}` and `ShiftLeft`/`ShiftRight` mappings in `from_intent_ir`; they
are dormant on the current corpus but would emit FSMGen-strict-invalid
`.isf` the moment a richer IntentIR transaction produces them. Correcting
them now keeps emission contract-faithful (residual-honesty).

## Non-Goals

- `(do child)` is **not** changed — its syntax is contract-exact (book
  `13f:18`). The "unknown child transaction" concern flagged in the
  assessment is a *semantic* (child-must-be-declared) issue, not a grammar
  typo; it is out of scope here and recorded as a separate future concern.
- No new IntentIR plumbing; this is purely an emitter-grammar correction.

## Acceptance Criteria

- All six emitter sites emit the FSMGen-contract-exact grammar.
- A unit test renders these steps and locks the underscore + `as` forms
  (and asserts the hyphen / missing-`as` forms are absent).
- Full `scripts/run_ci.sh` green.

## Task Tree

- ID: `ISF-TXN-GRAMMAR-FIX`
  Status: `done`
  Goal: correct the six transaction-step emitter grammar sites
  Children: `ISF-TXN-GRAMMAR-FIX.1`

- ID: `ISF-TXN-GRAMMAR-FIX.1`
  Status: `done`
  Goal: >
    Fix the six `render_txn_step` sites (shift_left/shift_right ×3,
    await_all/await_any ×2, spawn-`as` ×1); add a render-lock unit test;
    add the `BOOK-METHOD-DOC` close-rule subsection; close the tree.
  Acceptance: `six sites corrected; render-lock test green; book subsection added; scripts/run_ci.sh green.`
  Verification: >
    passed (`2026-05-29`) — all six `render_txn_step` sites corrected to the
    FSMGen-contract-exact grammar (book-verified, not binary-probed); added
    `transaction_steps_use_fsmgen_contract_grammar` render-lock test
    (asserts `shift_left`/`shift_right`/`await_all`/`await_any` + `(spawn …
    as …)` present and the old hyphen / missing-`as` forms absent); book
    method-doc subsection added to `pipeline/isf-adapter.md` per the
    close-rule. Lib `1147 → 1148`; full `scripts/run_ci.sh` green.
  Commit: `see Commit Log`

## Current Frontier

**Tree closed `2026-05-29`** — the single fix leaf is `done`; no eligible
leaf remains.

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `ISF-TXN-GRAMMAR-FIX.1` | `done` | six emitter sites corrected; render-lock test; book subsection; tree CLOSED |

## Decisions

- `2026-05-29`: grammar verified against the FSMGen **book/contract**, not
  the binary (per the lesson from the `FSMGEN-REFRESH-INTEGRATE` clarity
  request). `(do)` deliberately excluded (contract-exact already).

## Open Questions

- The `(do child)` unknown-declared-child semantic guard (ensuring the
  referenced child transaction is emitted, else residual) — deferred as a
  separate future concern; not a grammar typo.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-29` | `ISF-TXN-GRAMMAR-FIX.1` | 6 emitter sites fixed (book-verified); `transaction_steps_use_fsmgen_contract_grammar` render-lock test; lib `1147 → 1148`; full `scripts/run_ci.sh` (incl. mdBook) | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `ISF-TXN-GRAMMAR-FIX.1` | `ISF-TXN-GRAMMAR-FIX.1 — emit FSMGen-contract-exact transaction-step grammar; close tree` | shift_left/shift_right/await_all/await_any + spawn-`as`; render-lock test + book method-doc |

## Changelog

- `2026-05-29`: `.1` — fixed the six `render_txn_step` grammar sites
  (`shift_left`/`shift_right`/`await_all`/`await_any` + `spawn … as`),
  book-verified; render-lock test; book method-doc subsection; **tree
  CLOSED**.
- `2026-05-29`: Created — fix the latent `.isf` transaction-step emitter
  grammar bugs (book-verified), with a render-lock test.
