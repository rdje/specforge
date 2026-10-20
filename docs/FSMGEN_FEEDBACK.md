# FSMGEN Feedback From SPECFORGE

## Current channel

`docs/FSMGEN_FEEDBACK.md` is SpecForge's stable, tracked channel for directed questions, answers,
suggestions, and feature requests exchanged with FSMGen. It is a bounded status-and-routing surface:
long designs belong in task or research records, and reproducible bugs belong in issue bundles.

FSMGen's authoritative replies remain in
[`subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md`](../subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md).
The complete pre-containment conversation is directly retrievable through
[`docs/archive/fsmgen-feedback/INDEX.md`](archive/fsmgen-feedback/INDEX.md).

## Current downstream boundary

SPECFORGE's single adapter target is now `.isf`.

`SPECFORGE IntentIR → .isf → FSMGEN`

The pinned FSMGen gitlink at this boundary is
`54ac1c22d5314943027494f4c270f9c612e62704`. Historical `.fsm` planning and earlier response pins are
preserved only in the exact source capsule; they do not describe the current integration contract.

## Open correspondence

<!-- fsmgen_feedback_open:start -->
### derived-width-parameter — an `.isf` port width derived from actor parameters

- Direction: `SpecForge → FSMGen`
- Kind: `feature_request`
- Status: `accepted`
- Owner: `FSMGEN-REQUESTS.1`
- Evidence: `docs/upstream/fsmgen/sf-isf-derived-width-parameter/README.md`

Forwarded and acknowledged `2026-09-24`; FSMGen is working on it. Asks FSMGen to accept a derived actor parameter
(`(STRB_WIDTH (/ DATA_WIDTH 8))`) as a port width, ceiling division and `clog2`, with the parameter kept
symbolic through instantiation; 56 SPECFORGE signals need it.

### undeclared-rule-operand — strict accepts an undeclared rule operand as an invented input port

- Direction: `SpecForge → FSMGen`
- Kind: `bug_report`
- Status: `accepted`
- Owner: `FSMGEN-REQUESTS.2`
- Evidence: `docs/upstream/fsmgen/sf-isf-undeclared-rule-operand/README.md`

Forwarded and acknowledged `2026-09-24`; fixed in FSMGen's development line at `c6b8bc783` (verified `2026-09-25`,
the guard read too), not yet published; lands with `FSMGEN-REFRESH-INTEGRATE-10`. `(rule r_a (A VALID))` with `VALID` undeclared
passes `--strict --check` and generates `input wire VALID`; book 10 lists undeclared operands as errors.

### issue-bundle-foreign-git-head — the bundle helper records a foreign repository's commit

- Direction: `SpecForge → FSMGen`
- Kind: `bug_report`
- Status: `accepted`
- Owner: `FSMGEN-REQUESTS.3`
- Evidence: `docs/upstream/fsmgen/sf-tool-issue-bundle-foreign-git-head/README.md`

Forwarded and acknowledged `2026-09-24`: FSMGen keeps it as its revision-attribution ticket. Run from a
non-git FSMGen copy inside another repository, `bin/fsmgen-issue-bundle` records that repository's `HEAD` as
FSMGen's revision.
### optional-signal — an `.isf` interface port that can be absent, present by a parameter, or tied off

- Direction: `SpecForge → FSMGen`
- Kind: `feature_request`
- Status: `accepted`
- Owner: `EXTRACTION-REPRESENTATION-GAPS.4`
- Evidence: `docs/upstream/fsmgen/sf-isf-optional-signal/README.md`

Forwarded `2026-09-25`; accepted `2026-09-26`: FSMGen intends to implement it (FSMGen
`SEMANTIC-INTROSPECTION-MCP-FRONTIER.33.5`), required items as acceptance criteria, each optional item a child slice.
FSMGen also found the packet's positive control vacuous (its undriven `AWMECID` is dropped from HDL, `.33.9`);
criterion 2 is corrected. Asks for a zero width to mean an absent port, a port present only when a parameter enables
it, and the value an absent input reads; AXI alone has 14 width parameters defaulting to 0 (21 signals) and the three
AMBA specifications state 18 tie-offs.

### parameter-value-set — an actor parameter's legal values and recommended range

- Direction: `SpecForge → FSMGen`
- Kind: `feature_request`
- Status: `accepted`
- Owner: `EXTRACTION-REPRESENTATION-GAPS.5`
- Evidence: `docs/upstream/fsmgen/sf-isf-parameter-value-set/README.md`

Forwarded `2026-09-25`; accepted `2026-09-26`: FSMGen intends to implement it (FSMGen
`SEMANTIC-INTROSPECTION-MCP-FRONTIER.33.4`), required items as acceptance criteria, each optional item a child slice.
Asks for `(values …)`/`(range …)` on an actor parameter, checked on the default and every override, reported in
`actor_params[]`, and an advisory recommended range; 36 AMBA parameters carry a legal set or a recommended range.

### generic-port-widths — a parameter-sized port that stays generic in the generated HDL

- Direction: `SpecForge → FSMGen`
- Kind: `feature_request`
- Status: `accepted`
- Owner: `EXTRACTION-REPRESENTATION-GAPS.6`
- Evidence: `docs/upstream/fsmgen/sf-isf-generic-port-widths/README.md`

Forwarded `2026-09-25`; accepted `2026-09-26`: FSMGen intends to implement it (FSMGen
`SEMANTIC-INTROSPECTION-MCP-FRONTIER.33.3`), required items as acceptance criteria, each optional item a child slice.
The generated module declares `parameter DATA_WIDTH` but sizes the port `[31:0]`, so an instantiation override
silently does nothing; asks for ports and registers that follow the parameter, and overrides that re-specialize them.
62 SPECFORGE ports are parameter-sized.

### actor-level-assertion — an always-on check that belongs to no transaction

- Direction: `SpecForge → FSMGen`
- Kind: `feature_request`
- Status: `accepted`
- Owner: `EXTRACTION-REPRESENTATION-GAPS.15`
- Evidence: `docs/upstream/fsmgen/sf-isf-actor-level-assertion/README.md`

Forwarded `2026-09-25`; accepted `2026-09-26`: FSMGen intends to implement it (FSMGen
`SEMANTIC-INTROSPECTION-MCP-FRONTIER.33.6`), required items as acceptance criteria, each optional item a child slice.
`(assert …)`/`(assume …)`/`(cover …)` are transaction clauses although each is emitted always-on; asks for
actor-level placement (an actor with no transaction cannot state an invariant today) and a documented placement
semantics. Modest need: 1 of 7 signal-bearing documents.

### known-value-predicate — "X must be valid when Y" as a checkable property

- Direction: `SpecForge → FSMGen`
- Kind: `feature_request`
- Status: `accepted`
- Owner: `EXTRACTION-REPRESENTATION-GAPS.16`
- Evidence: `docs/upstream/fsmgen/sf-isf-known-value-predicate/README.md`

Forwarded `2026-09-25`; accepted `2026-09-26`: FSMGen intends to implement it (FSMGen
`SEMANTIC-INTROSPECTION-MCP-FRONTIER.33.7`), required items as acceptance criteria, each optional item a child slice.
Asks for `(known SIG)` → `!$isunknown(SIG)` in the verification property language, beside
`stable`/`changed`/`rose`/`fell`; 28 AMBA statements say *"must be valid"*.

### strict-unknown-port-clause — should strict mode reject an interface-port clause it does not recognise?

- Direction: `SpecForge → FSMGen`
- Kind: `question`
- Status: `accepted`
- Owner: `EXTRACTION-REPRESENTATION-GAPS.19`
- Evidence: `docs/upstream/fsmgen/sf-isf-strict-unknown-port-clause/README.md`

Forwarded `2026-09-25`; answered `2026-09-26`: silent acceptance is **not** intended support, and FSMGen will reject
unknown and unimplemented port options through its shared parser (FSMGen `SEMANTIC-INTROSPECTION-MCP-FRONTIER.33.2`).
SPECFORGE emits only `(width …)` on ports, a documented option. `(output AWPROT (width 3) (bogus-clause xyz))` and
`(… (present PROT_Present))` pass `--strict --check` with no diagnostic, and each generated module equals the one
without the clause (pin `54ac1c22`, upstream `354844fe1`). Book 10 and 13k do not say whether an unrecognised port
clause is an error. Since ADR 0054 the question is a packet like any request.

### transaction-phase-groups — checked phase-group metadata on a transaction

- Direction: `SpecForge → FSMGen`
- Kind: `feature_request`
- Status: `accepted`
- Owner: `EXTRACTION-REPRESENTATION-GAPS.20`
- Evidence: `docs/upstream/fsmgen/sf-isf-transaction-phase-groups/README.md`

Forwarded `2026-09-25`; accepted `2026-09-26`: FSMGen intends to implement it (FSMGen
`SEMANTIC-INTROSPECTION-MCP-FRONTIER.33.8`), required items as acceptance criteria, each optional item a child slice.
FSMGen's `2026-06-16` answer named checked phase-group metadata as the right future shape *"if this becomes
load-bearing"*; SPECFORGE now carries 18 transactions with 14 phase groups and 105 port memberships, 16 of which
reach no `.isf`.

### storage-register-access — a storage register's own access as checked metadata

- Direction: `SpecForge → FSMGen`
- Kind: `feature_request`
- Status: `accepted`
- Owner: `FSMGEN-REQUESTS.5`
- Evidence: `docs/upstream/fsmgen/sf-isf-storage-register-access/README.md`

Forwarded and acknowledged `2026-09-26`: FSMGen is reproducing the controls and tracks scalar support apart from
the optional bank extension (no task id yet). A `(var …)` refuses `(access ro)`, which only a field may carry; asks for the same token set on the word itself, report-only and independent of its fields. 101 registers in
two current chains state an access (TMC 50 of 50, ADIv6 51 of 82), 61 of them with no field at all.

### duration-time-units — a duration stated in time units, lowered to a counter

- Direction: `SpecForge → FSMGen`
- Kind: `feature_request`
- Status: `open`
- Owner: `FSMGEN-REQUESTS.7`
- Evidence: `docs/upstream/fsmgen/sf-isf-duration-time-units/README.md`

Signed off `2026-09-29`; waits for the owner to forward it. `.isf` takes `wait`, `watchdog` and `latency` bounds in
cycles only, and a static wait lowers to one state per cycle. Asks for durations in time units against a declared
clock frequency, lowered to a counter. USB 3.2 states its link timeouts in ms and µs; 172 timing records on the 31
chains are in time units.

### named-timers — several named timers, whose expiry is behaviour

- Direction: `SpecForge → FSMGen`
- Kind: `feature_request`
- Status: `open`
- Owner: `FSMGEN-REQUESTS.8`
- Evidence: `docs/upstream/fsmgen/sf-isf-named-timers/README.md`

Signed off `2026-09-29`; waits for the owner to forward it. One watchdog per transaction, and its timeout is an
error path (`last_error`). Asks for author-declared timers, started and stopped in bodies, running concurrently,
whose expiry is an ordinary condition: USB 3.2's link-layer timers and its 25 LTSSM state timeouts.

### unstated-reset — which `.isf` form states that a specification names no reset pin

- Direction: `SpecForge → FSMGen`
- Kind: `question`
- Status: `open`
- Owner: `FSMGEN-REQUESTS.9`
- Evidence: `docs/upstream/fsmgen/sf-isf-unstated-reset/README.md`

Signed off `2026-09-29`; waits for the owner to forward it. An omitted `(reset …)` is reported exactly like a stated
`(rst_n async active_low)`, and a clock-only domain reports `reset: null`. Asks which form SPECFORGE should write
when a specification names no reset pin, and whether a defaulted reset can be told from a stated one.

### aggregate-interface-ports — a record-typed interface port for packets and headers

- Direction: `SpecForge → FSMGen`
- Kind: `feature_request`
- Status: `open`
- Owner: `FSMGEN-REQUESTS.10`
- Evidence: `docs/upstream/fsmgen/sf-isf-aggregate-interface-ports/README.md`

Signed off `2026-09-29`; waits for the owner to forward it. A record type is accepted on storage only. Asks for
record-typed ports whose fields a transaction reads and drives by name; 191 untyped layout tables on the 31
chains, 75 of them USB 3.2's packet formats. Builds on FSMGen's `2026-06-22` storage-field answer.

### byte-group-reduction — a reduction, and a per-byte reduction over a parameterised width

- Direction: `SpecForge → FSMGen`
- Kind: `feature_request`
- Status: `open`
- Owner: `FSMGEN-REQUESTS.12`
- Evidence: `docs/upstream/fsmgen/sf-isf-byte-group-reduction/README.md`

Signed off `2026-10-20`; waits for the owner to forward it. `^` takes two operands only, so a parity bit per byte of a
parameter-sized bus cannot be stated. Asks for one-operand reductions and a per-group reduction over a parameterised
width; 49 of the 153 AMBA5 check signals SPECFORGE carries are byte-granular.

<!-- fsmgen_feedback_open:end -->

## Closed correspondence register

<!-- fsmgen_feedback_register:start -->
| Record | Status | Direction | Complete history | Independent closure evidence |
| --- | --- | --- | --- | --- |
| enum-type-clarity | resolved | SpecForge → FSMGen | [request](archive/fsmgen-feedback/source-through-2026-08-08.md#clarity-request-2026-05-29--actor-local-types--enums-same-name-relationship) | [FSMGen response](../subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md#2026-05-29-actor-local-typesenums-relationship-clarity) |
| min-window-answer | answered | SpecForge → FSMGen | [answer](archive/fsmgen-feedback/source-through-2026-08-08.md#answer-2026-06-04--min--1-window-confirmation-integer-literal-bounds-specforge-guarantees-min--1) | [FSMGen response](../subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md#2026-06-04-follow-up-the-two-flagged-deltas--stable--shipped-min--1-windows-proposed) |
| ltl-mtl-suggestion | answered_and_integrated | SpecForge → FSMGen | [suggestion](archive/fsmgen-feedback/source-through-2026-08-08.md#suggestion-2026-06-04--first-class-ltlmtl-temporal-properties-in-isf) | [FSMGen response](../subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md#2026-06-04-first-class-ltlmtl-temporal-properties--already-generalized-in-the-verification-family) |
| phase-membership | answered | SpecForge → FSMGen | [question](archive/fsmgen-feedback/source-through-2026-08-08.md#question--feature-request-2026-06-16--lowering-a-transactions-phase-membership-without-fabricating-drive-values-or-step-order) | [FSMGen response](../subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md#2026-06-16-transaction-phase-membership-without-fabricated-values-or-order) |
| field-structured-storage | shipped | SpecForge → FSMGen | [feature request](archive/fsmgen-feedback/source-through-2026-08-08.md#feature-request-2026-06-22--declarative-field-structured-storage-named-bit-fields-in-a-register--packed-structure-layout) | [FSMGen response](../subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md#2026-06-22-declarative-field-structured-storage) |
| stage-contract-bugs | resolved | SpecForge → FSMGen | [issue episode](archive/fsmgen-feedback/source-through-2026-08-08.md#tracked-finding-2026-05-18--isf_downstream_integration_specmd-118-doc-vs-strict-mismatches) | [eventually-flat resolution](upstream/fsmgen/sf-isf-contract-eventually-flat/RESOLUTION.md), [ready/valid resolution](upstream/fsmgen/sf-isf-stage-ready-valid/RESOLUTION.md) |
<!-- fsmgen_feedback_register:end -->

## Feedback record format

Create the owning task-tree leaf before adding an open record. Use a stable lowercase-hyphen id, place
the record inside the open markers, and keep its detailed evidence in the canonical task, research, or
issue-bundle surface. The required shape is:

```markdown
### record-id — concise subject

- Direction: `SpecForge → FSMGen`
- Kind: `question`
- Status: `awaiting_response`
- Owner: `TASK-TREE-WORK-UNIT`
- Evidence: `docs/tasks/TASK-TREE-WORK-UNIT.md`

Concise request and the exact decision sought.
```

Allowed directions, kinds, statuses, record limits, and the verified-before-filing workflow are in
[`TOOLBOX.md`](../TOOLBOX.md). When a response closes an exchange, move it from the open region into
the closed register in the same task-owned commit and preserve direct history plus closure evidence.

## History and retrieval

- [Archive index and recovery procedure](archive/fsmgen-feedback/INDEX.md)
  (`docs/archive/fsmgen-feedback/INDEX.md`)
- [Exact source capsule](archive/fsmgen-feedback/source-through-2026-08-08.md), preserving every byte
  of the original 936-line channel
- [FSMGen's response ledger](../subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md)
  (`subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md`)
- [Reproducible FSMGen issue-bundle catalog](catalogs/fsmgen-issue-packets.md)
  (`docs/catalogs/fsmgen-issue-packets.md`)
- [Diagnostic and verified-feedback workflow](../TOOLBOX.md) (`TOOLBOX.md`)

Reverify the complete current/history contract from the repository root:

```bash
perl scripts/check_fsmgen_feedback_protocol.pl --check
```
