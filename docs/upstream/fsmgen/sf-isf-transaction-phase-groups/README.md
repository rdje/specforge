# FSMGen Feature Request: checked transaction phase-group metadata

Title: sf-isf-transaction-phase-groups — a transaction cannot say which signals each of its protocol phases involves; FSMGen agreed on 2026-06-16 that checked phase-group metadata is the right future shape "if this becomes load-bearing", and it now is
FSMGen command or public API entrypoint: ./bin/fsmgen --strict --check --json sources/fsmgen-input/phase-groups.isf
FSMGen-facing primary artifact path: sources/fsmgen-input/phase-groups.isf
Failure class: rejected-input
FSMGen commit or capability-manifest producer commit: 354844fe1019d2db1e413dd54f8c4a5ae9eb398b (upstream HEAD, 2026-09-25); also reproduced on SPECFORGE's pin 54ac1c22d5314943027494f4c270f9c612e62704 — see "Revision note" below
SPECFORGE version/commit: specforge b92023f8
Expected behavior: a transaction may carry checked phase-group metadata, (phases (phase NAME (signals SIG...)) ...), naming the signals each phase of the protocol involves, with no implied drive value, order or HDL behaviour; --strict --check --json reports success:true for sources/fsmgen-input/phase-groups.isf and the phases appear in the schedule/report JSON.
Observed behavior: --strict --check --json reports success:false with "Transaction 'write_transfer': unsupported '(phases ...)' clause in transaction body"; the actor-level spelling is an unknown actor clause.
First failing command or API call: original (./bin/fsmgen --strict --check --json CASE)
Exit status or exception: 1 (observed/command-logs/original.exit); diagnostic in observed/stdout/original.stdout
Is the attached artifact bundle minimized: yes
Does the minimized/redacted bundle still reproduce: yes

Request kind: **feature request** (FSMGen follows its current public contract; the contract lacks a capability).
SPECFORGE owner: `EXTRACTION-REPRESENTATION-GAPS.20` (bundle); SPECFORGE already extracts phase membership
(`KG-ISF-TRANSACTIONS.2i`).
Local status: see `request.json` (`forwarded` by the owner on `2026-09-25`; `acknowledged` by FSMGen on `2026-09-26`, owner `SEMANTIC-INTROSPECTION-MCP-FRONTIER.33.8`).

## 1. What SPECFORGE needs

Interface specifications describe transactions as named phases and say which signals take part in each: APB 5
(IHI0024 E) — a write transfer has a *Setup* phase (`PSEL`, `PADDR`, `PWRITE`, `PWDATA`) and an *Access* phase
(`PENABLE`, `PREADY`); AHB (IHI0033 C) — an *address* phase and a *data* phase. SPECFORGE extracts these as
`phase_membership` on each transaction, with each member's direction. Often the specification grounds the
membership but not a drive value or an order, and FSMGen's own guidance (below) is not to fabricate either. So
the membership has nowhere to go: a transaction body is *"the wrong container for a set of membership facts"*.

## 2. Measured need (SPECFORGE corpus, 2026-09-25)

**18 transactions** across five interface documents carry a grounded signal set (AXI 7, AHB 6, APB 5 2, Wishbone
2, I2C 1): **14 phase groups and 105 port memberships**. Only 2 of the 18 have grounded steps, so the other 16
reach no `.isf` at all today. Re-derive in a SPECFORGE checkout: the `transactions[].phase_membership` and
`transactions[].ports` of `generated/intent_ir/*/intent_ir.json`.

## 3. Current public contract (why this is a feature request, not a bug)

- The transaction clauses FSMGen documents (13b, 13k) have no phase-membership clause; the observed diagnostic
  is FSMGen refusing an unsupported body clause, as documented.
- **FSMGen's own answer** (`docs/SPECFORGE_FEEDBACK_RESPONSE.md`, *2026-06-16: Transaction Phase Membership Without
  Fabricated Values Or Order*): *"The right future FSMGen feature, if this becomes load-bearing for `.isf`, is
  checked transaction phase-group metadata in ISF"*; *"FSMGen agrees with the direction of a protocol-neutral
  transaction phase-group metadata surface. The first safe version should be checked metadata, not generated
  behavior"*, carrying the transaction name, authored phase names, member signals per phase, each member's
  actor-relative role, and provenance notes; *"signal names should resolve, directions should match the actor
  interface, duplicate/malformed groups should fail closed"*. This request asks for exactly that first slice, now
  that the need is measured.

## 4. Reproduction

Run from the FSMGen root: `bash <bundle>/commands.sh`.

| Input | What it states | `--strict --check` | Result |
| --- | --- | --- | --- |
| `baseline-good.isf` | the APB write transfer, no phase metadata | exit 0 | accepted |
| `phase-groups.isf` (primary) | `(phases (phase setup (signals PSEL PADDR PWRITE PWDATA)) (phase access (signals PSEL PENABLE PREADY)))` in the transaction | exit 1 | *unsupported '(phases ...)' clause in transaction body* |
| `phase-groups-actor-level.isf` | the same as an actor-level `(phase-groups write_transfer …)` | exit 1 | *unknown actor clause 'phase-groups'* |
| `phase-groups-unknown-signal.isf` | a member that is not a port | exit 1 | same as the primary today; **must stay refused** once supported |

The spellings are SPECFORGE's placeholders; FSMGen chooses the syntax.

## 5. Requested capability (FSMGen's own first slice)

- **R1 (required) — checked metadata.** A transaction states named phases and the signals of each, with an
  optional actor-relative role per member (driven output / sampled input). It implies no drive value, no order, no
  HDL, no SV/UVM output.
- **R2 (required) — validation.** Member names resolve to the actor's ports; a stated role matches the port's
  direction; a duplicate phase, an empty phase, or an unknown signal fails closed.
- **R3 (required) — reported.** The phases appear in the schedule/semantic JSON, discoverable through the public
  contract, so verification tooling can consume them.
- **R4 (optional) — a transaction with membership and no steps.** A transaction whose only content is its phase
  metadata is accepted as documentation of the protocol, without generated behaviour.

## 6. Acceptance criteria (proposed, for FSMGen's regression tests)

1. `phase-groups.isf` passes `--strict --check`; its generated module equals `baseline-good.isf`'s; its
   schedule/semantic JSON lists `write_transfer` with phases `setup` and `access` and their members.
2. `phase-groups-unknown-signal.isf` fails under `--strict` with a diagnostic naming `NOT_A_PORT`.
3. A member whose stated role contradicts the port direction fails closed.
4. A duplicate phase name fails closed.

## 7. Alternatives SPECFORGE considered and why they are not faithful

- **Transaction-body drives for each member**: invents values and an order the source does not state (FSMGen
  advised against it).
- **Comments**: not checked, not discoverable, and they drift.
- **Keep membership out of `.isf`** (what SPECFORGE does today): the protocol's phase structure never reaches
  FSMGen's verification tooling.

## 7a. What SPECFORGE does until this ships

SPECFORGE keeps phase membership in IntentIR and emits a transaction only when its steps are grounded.

## 8. Compatibility

Additive: metadata only, with no effect on generated behaviour.

## 9. Revision note

Reproduced on FSMGen upstream HEAD `354844fe1019d2db1e413dd54f8c4a5ae9eb398b` (a read-only `git archive` export,
where `commands.sh` ran and generated `observed/`) and on SPECFORGE's pin
`54ac1c22d5314943027494f4c270f9c612e62704` (strict checks of the same four inputs: identical exit codes). FSMGen's
feature matrix is byte-identical at the two revisions.

## 10. Redactions and simplifications

- Absolute paths in captured outputs are replaced by `<FSMGEN_ROOT>` (the FSMGen export the helper ran in),
  `<SPECFORGE_ROOT>`, `<FSMGEN_CHECKOUT>` and `<HOME>`. No other byte of any captured output changed.
- `env.txt`'s *FSMGen git HEAD* is corrected by hand: the helper recorded SPECFORGE's commit, the defect
  reported as `sf-tool-issue-bundle-foreign-git-head`.
- `commands.sh` is the helper's script plus the SPECFORGE-added companion loop at its end.
