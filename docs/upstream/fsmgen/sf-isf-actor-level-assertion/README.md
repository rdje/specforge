# FSMGen Feature Request: always-on `assert` / `assume` / `cover` at actor level

Title: sf-isf-actor-level-assertion — an always-on verification check can be written only inside a transaction, so an actor with no transaction cannot state an invariant, and a protocol invariant has to be attached to an unrelated transaction
FSMGen command or public API entrypoint: ./bin/fsmgen --strict --check --json sources/fsmgen-input/no-transaction-assert.isf
FSMGen-facing primary artifact path: sources/fsmgen-input/no-transaction-assert.isf
Failure class: rejected-input
FSMGen commit or capability-manifest producer commit: 354844fe1019d2db1e413dd54f8c4a5ae9eb398b (upstream HEAD, 2026-09-25); also reproduced on SPECFORGE's pin 54ac1c22d5314943027494f4c270f9c612e62704 — see "Revision note" below
SPECFORGE version/commit: specforge 902fc853
Expected behavior: an actor may state an always-on verification check at actor level, (assert COND [message]) / (assume ...) / (cover ...), including an actor with no transaction; --strict --check --json reports success:true for sources/fsmgen-input/no-transaction-assert.isf and the generated module carries the same concurrent property a transaction-level assert produces.
Observed behavior: --strict --check --json reports success:false with "unknown actor clause 'assert' in actor 'i2s_demo'"; the same kind of invariant is accepted only inside a transaction, where it is emitted as an always-on property with no transaction-state gating.
First failing command or API call: original (./bin/fsmgen --strict --check --json CASE)
Exit status or exception: 1 (observed/command-logs/original.exit); JSON diagnostic in observed/stdout/original.stdout
Is the attached artifact bundle minimized: yes
Does the minimized/redacted bundle still reproduce: yes

Request kind: **feature request** (FSMGen follows its current public contract; the contract lacks a capability).
SPECFORGE owner: `EXTRACTION-REPRESENTATION-GAPS.15` (bundle); SPECFORGE's half, lowering stability and exclusion
obligations as assertions, is `EXTRACTION-REPRESENTATION-GAPS.11`.
Local status: see `request.json` (`forwarded` by the owner on `2026-09-25`; `acknowledged` by FSMGen on `2026-09-26`, owner `SEMANTIC-INTROSPECTION-MCP-FRONTIER.33.6`).

## 1. What SPECFORGE needs

Specifications state protocol invariants that hold whenever a condition holds, independent of any transaction:
AHB *"the size of the transfer, as indicated by HSIZE, must not be changed"* during a burst; AHB *"HEXOKAY must
not be asserted in the same cycle as HRESP is asserted"*; APB *"PADDR … must remain stable"* while `PSEL` and
`PENABLE` are asserted. FSMGen's guidance to SPECFORGE (`docs/SPECFORGE_FEEDBACK_RESPONSE.md`, *Ordering as
constraint versus ordering as body*) is that such obligations belong in the verification family
(`assert`/`assume`/`cover`), not in a transaction body. SPECFORGE wants to follow that guidance, and the
verification family can only be written inside a transaction.

## 2. Measured need (SPECFORGE corpus, 2026-09-25)

Over the seven signal-bearing interface documents SPECFORGE rebuilds: **11 stability or must-not-change
obligations** on declared signals (APB 5 6, AHB 3, I2C 2), and 10 more temporal rules the adapter holds back as
stability or same-cycle exclusions. **One of the seven** (I2S, UM11732) has no transaction at all, so it cannot
state any invariant; in the other six, SPECFORGE could host each invariant in some existing transaction. The need
is therefore modest and specific: actors without a transaction, and keeping an invariant from reading as if it
belonged to the transaction that hosts it. Re-derive in a SPECFORGE checkout: the census in
`docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md` (`.2`, `.15`) and `python3 scripts/measure_intent_to_isf_reach.py`.

## 3. Current public contract (why this is a feature request, not a bug)

- `docs/book/src/13d-control-flow.md`, *`(assert COND [message])` — Verification Invariant*: *"Capture a
  design-intent invariant — a condition that must hold every cycle … `(assert …)` is a transaction-level
  clause"*.
- `docs/book/src/13k-isf-feature-support-matrix.md`, row *Immediate verification checks*: *"Transaction-level
  `(assert COND [message])` / `(assume …)` / `(cover …)`"*.

FSMGen does what it documents: an actor-level `assert` is an unknown actor clause.

**Why the placement is only syntactic today.** The accepted baseline's invariant is emitted as
`assert property (@(posedge clk) disable iff (!rst_n) (((PSEL & PENABLE)) |-> ($stable(PADDR))))`
(`observed/command-logs/baseline-good-assert-property.txt`): always on, with no gating by the hosting
transaction's state, exactly as 13d says (*"a condition that must hold every cycle"*). Allowing the same clause at
actor level changes no semantics; it removes the need for a host transaction.

## 4. Reproduction

Run from the FSMGen root: `bash <bundle>/commands.sh`.

| Input | What it is | `--strict --check` | Result |
| --- | --- | --- | --- |
| `baseline-good.isf` | the invariant inside a transaction | exit 0 | accepted; emitted always-on |
| `actor-level-assert.isf` | the same invariant at actor level | exit 1 | *unknown actor clause 'assert'* |
| `actor-level-assume.isf` | the same, as `assume` | exit 1 | *unknown actor clause 'assume'* |
| `no-transaction-control.isf` | an actor with no transaction | exit 0 | accepted |
| `no-transaction-assert.isf` (primary) | that actor with one invariant | exit 1 | *unknown actor clause 'assert'* |

## 5. Requested capability

- **R1 (required) — actor-level placement.** `(assert COND [msg])`, `(assume COND [msg])` and `(cover COND
  [label])` as actor clauses, with the same property language and the same lowering (a clocked concurrent
  property on the actor's clock, reset-gated), including in an actor with no transaction. Failure cases: exactly
  those of the transaction-level clause (unknown signals, unsupported operators, runtime-parameter windows).
- **R2 (required) — document the semantics of placement.** State that a transaction-level check is always on,
  independent of the hosting transaction's state, or, if a transaction-scoped check is intended in future, give it a
  distinct spelling so the two cannot be confused.
- **R3 (optional) — multi-domain actors.** An actor-level check names its clock domain when the actor declares
  several.

## 6. Acceptance criteria (proposed, for FSMGen's regression tests)

1. `no-transaction-assert.isf` passes `--strict --check`, and its generated module contains
   `assert property (@(posedge clk) disable iff (!rst_n) (($rose(SCK)) |-> ($stable(WS))))` or an equivalent.
2. `actor-level-assert.isf` generates the same assertion as `baseline-good.isf`.
3. `actor-level-assume.isf` generates the corresponding `assume property`.
4. An actor-level check with an unknown signal fails under `--strict` with a diagnostic naming it.

## 7. Alternatives SPECFORGE considered and why they are not faithful

- **Host the invariant in an existing transaction** (possible in six of the seven documents): the property is
  still always-on, but the `.isf` then reads as if the invariant belonged to that transaction, which the
  specification does not say.
- **Synthesize a transaction** to host invariants: invents behaviour (an entry trigger, a completion) the
  specification never states.
- **Keep invariants out of `.isf`** (what SPECFORGE does today): the specification's checkable obligations never
  reach the generated verification.

## 7a. What SPECFORGE does until this ships

SPECFORGE keeps these obligations in its IR and lists them in adapter residuals
(`isf_obligations_not_lowered`, `isf_temporal_unrepresentable_*`); lowering them as assertions is its
`EXTRACTION-REPRESENTATION-GAPS.11`, hosted in an existing transaction where one exists.

## 8. Compatibility

Additive: transaction-level checks keep their meaning; only actor-level clauses FSMGen refuses today gain one.

## 9. Revision note

Reproduced on FSMGen upstream HEAD `354844fe1019d2db1e413dd54f8c4a5ae9eb398b` (a read-only `git archive` export,
where `commands.sh` ran and generated `observed/`) and on SPECFORGE's pin
`54ac1c22d5314943027494f4c270f9c612e62704` (strict checks of the same five inputs: identical exit codes). FSMGen's
feature matrix is byte-identical at the two revisions.

## 10. Redactions and simplifications

- Absolute paths in captured outputs are replaced by `<FSMGEN_ROOT>` (the FSMGen export the helper ran in),
  `<SPECFORGE_ROOT>`, `<FSMGEN_CHECKOUT>` and `<HOME>`. No other byte of any captured output changed.
- `env.txt`'s *FSMGen git HEAD* is corrected by hand: the helper recorded SPECFORGE's commit, the defect
  reported as `sf-tool-issue-bundle-foreign-git-head`.
- `commands.sh` is the helper's script plus the SPECFORGE-added companion loop and generation probe at its end.
