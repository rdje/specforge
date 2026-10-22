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

- Direction: `SpecForge → FSMGen` · Kind: `feature_request` · Status: `accepted` · Owner: `FSMGEN-REQUESTS.1` · Evidence: `docs/upstream/fsmgen/sf-isf-derived-width-parameter/README.md`

Forwarded and acknowledged `2026-09-24`; FSMGen is working on it. Asks for a port width derived from parameters (`(/ DATA_WIDTH 8)`, ceiling division, `clog2`), kept symbolic; 56 SPECFORGE signals need it.

### undeclared-rule-operand — strict accepts an undeclared rule operand as an invented input port

- Direction: `SpecForge → FSMGen` · Kind: `bug_report` · Status: `accepted` · Owner: `FSMGEN-REQUESTS.2` · Evidence: `docs/upstream/fsmgen/sf-isf-undeclared-rule-operand/README.md`

Forwarded and acknowledged `2026-09-24`; fixed in FSMGen's development line at `c6b8bc783` (value and guard reads), not yet published; lands with `FSMGEN-REFRESH-INTEGRATE-10`.

### issue-bundle-foreign-git-head — the bundle helper records a foreign repository's commit

- Direction: `SpecForge → FSMGen` · Kind: `bug_report` · Status: `accepted` · Owner: `FSMGEN-REQUESTS.3` · Evidence: `docs/upstream/fsmgen/sf-tool-issue-bundle-foreign-git-head/README.md`

Forwarded and acknowledged `2026-09-24`; FSMGen keeps it as its revision-attribution ticket. Run from a non-git FSMGen copy inside another repository, the bundle helper records that repository's `HEAD`.

### optional-signal — an `.isf` interface port that can be absent, present by a parameter, or tied off

- Direction: `SpecForge → FSMGen` · Kind: `feature_request` · Status: `accepted` · Owner: `EXTRACTION-REPRESENTATION-GAPS.4` · Evidence: `docs/upstream/fsmgen/sf-isf-optional-signal/README.md`

Forwarded `2026-09-25`; accepted `2026-09-26` (FSMGen SEM `.33.5`; the packet's vacuous positive control corrected, `.33.9`). Asks for absent, parameter-present and tied-off ports; AXI has 14 width parameters defaulting to 0.

### parameter-value-set — an actor parameter's legal values and recommended range

- Direction: `SpecForge → FSMGen` · Kind: `feature_request` · Status: `accepted` · Owner: `EXTRACTION-REPRESENTATION-GAPS.5` · Evidence: `docs/upstream/fsmgen/sf-isf-parameter-value-set/README.md`

Forwarded `2026-09-25`; accepted `2026-09-26` (FSMGen SEM `.33.4`). Asks for a parameter's legal values and recommended range, checked on every default and override; 36 AMBA parameters carry one.

### generic-port-widths — a parameter-sized port that stays generic in the generated HDL

- Direction: `SpecForge → FSMGen` · Kind: `feature_request` · Status: `accepted` · Owner: `EXTRACTION-REPRESENTATION-GAPS.6` · Evidence: `docs/upstream/fsmgen/sf-isf-generic-port-widths/README.md`

Forwarded `2026-09-25`; accepted `2026-09-26` (FSMGen SEM `.33.3`). Asks that a parameter-sized port stay generic in the generated HDL, which sizes it from the default today; 62 SPECFORGE ports are parameter-sized.

### actor-level-assertion — an always-on check that belongs to no transaction

- Direction: `SpecForge → FSMGen` · Kind: `feature_request` · Status: `accepted` · Owner: `EXTRACTION-REPRESENTATION-GAPS.15` · Evidence: `docs/upstream/fsmgen/sf-isf-actor-level-assertion/README.md`

Forwarded `2026-09-25`; accepted `2026-09-26` (FSMGen SEM `.33.6`). Asks for `assert`/`assume`/`cover` at actor level with documented placement semantics; 1 of 7 signal-bearing documents needs it.

### known-value-predicate — "X must be valid when Y" as a checkable property

- Direction: `SpecForge → FSMGen` · Kind: `feature_request` · Status: `accepted` · Owner: `EXTRACTION-REPRESENTATION-GAPS.16` · Evidence: `docs/upstream/fsmgen/sf-isf-known-value-predicate/README.md`

Forwarded `2026-09-25`; accepted `2026-09-26` (FSMGen SEM `.33.7`). Asks for `(known SIG)` (`!$isunknown`) in the property language; 28 AMBA statements say *must be valid*.

### strict-unknown-port-clause — should strict mode reject an interface-port clause it does not recognise?

- Direction: `SpecForge → FSMGen` · Kind: `question` · Status: `accepted` · Owner: `FSMGEN-REFRESH-INTEGRATE-10.1` · Evidence: `docs/upstream/fsmgen/sf-isf-strict-unknown-port-clause/README.md`

Forwarded `2026-09-25`; answered `2026-09-26` as a defect (FSMGen SEM `.33.2`); upstream `c51e22712` refuses an unknown port option (verified `2026-10-22`), not yet published; lands with `FSMGEN-REFRESH-INTEGRATE-10`.

### transaction-phase-groups — checked phase-group metadata on a transaction

- Direction: `SpecForge → FSMGen` · Kind: `feature_request` · Status: `accepted` · Owner: `EXTRACTION-REPRESENTATION-GAPS.20` · Evidence: `docs/upstream/fsmgen/sf-isf-transaction-phase-groups/README.md`

Forwarded `2026-09-25`; accepted `2026-09-26` (FSMGen SEM `.33.8`). Asks for checked phase-group metadata on a transaction: 18 transactions, 14 phase groups, 105 port memberships, 16 reaching no `.isf`.

### storage-register-access — a storage register's own access as checked metadata

- Direction: `SpecForge → FSMGen` · Kind: `feature_request` · Status: `accepted` · Owner: `FSMGEN-REQUESTS.5` · Evidence: `docs/upstream/fsmgen/sf-isf-storage-register-access/README.md`

Forwarded and acknowledged `2026-09-26`; the scalar contract is implemented in FSMGen's checkout (SEM `.38.1`, unpublished), banks apart. Asks for `(access …)` on a storage word; 101 registers state one.

### duration-time-units — a duration stated in time units, lowered to a counter

- Direction: `SpecForge → FSMGen` · Kind: `feature_request` · Status: `accepted` · Owner: `FSMGEN-REQUESTS.7` · Evidence: `docs/upstream/fsmgen/sf-isf-duration-time-units/README.md`

Signed off `2026-09-29`; received `2026-09-30`, acknowledged `2026-10-21` (SEM `.60.1`), implementation open. Asks for durations in time units against a declared clock frequency; 172 timing records are in time units.

### named-timers — several named timers, whose expiry is behaviour

- Direction: `SpecForge → FSMGen` · Kind: `feature_request` · Status: `accepted` · Owner: `FSMGEN-REQUESTS.8` · Evidence: `docs/upstream/fsmgen/sf-isf-named-timers/README.md`

Signed off `2026-09-29`; received `2026-09-30`, acknowledged `2026-10-21` (SEM `.60.2`), implementation open. Asks for named concurrent timers whose expiry is an ordinary condition (USB 3.2's 25 LTSSM timeouts).

### unstated-reset — which `.isf` form states that a specification names no reset pin

- Direction: `SpecForge → FSMGen` · Kind: `question` · Status: `answered` · Owner: `FSMGEN-REQUESTS.9` · Evidence: `docs/upstream/fsmgen/sf-isf-unstated-reset/README.md`

Signed off `2026-09-29`; answered `2026-09-30` (SEM `.60.3`): an omitted reset is FSMGen's `rst_n` default, reported `legacy_default`, and a clock-only domain has none. SPECFORGE does not accept the default (`.9.a`, `2026-10-22`).

### aggregate-interface-ports — a record-typed interface port for packets and headers

- Direction: `SpecForge → FSMGen` · Kind: `feature_request` · Status: `accepted` · Owner: `FSMGEN-REQUESTS.10` · Evidence: `docs/upstream/fsmgen/sf-isf-aggregate-interface-ports/README.md`

Signed off `2026-09-29`; received `2026-09-30`, acknowledged `2026-10-21` (SEM `.60.4`); record input ports delivered in FSMGen's checkout, outputs open. Asks for record-typed ports; 191 untyped layout tables.

### byte-group-reduction — a reduction, and a per-byte reduction over a parameterised width

- Direction: `SpecForge → FSMGen` · Kind: `feature_request` · Status: `accepted` · Owner: `FSMGEN-REQUESTS.12` · Evidence: `docs/upstream/fsmgen/sf-isf-byte-group-reduction/README.md`

Signed off `2026-10-20`; acknowledged `2026-10-21` (SEM `.92`: `(^ X)` and `(reduce-groups OP X N)` selected), delivery open. Asks for one-operand and per-byte reductions; 49 of 153 AMBA5 check signals need one.

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
issue-bundle surface. An open record indexes its packet, which holds the request: its fields go on one line
and its state and ask in one more (`LIVE-DOCUMENT-PRESSURE-HEADROOM.45`). The required shape is:

```markdown
### record-id — concise subject

- Direction: `SpecForge → FSMGen` · Kind: `question` · Status: `awaiting_response` · Owner: `TASK-UNIT` · Evidence: `docs/upstream/fsmgen/<id>/README.md`

Its dates and upstream state, then the ask, in one line.
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
