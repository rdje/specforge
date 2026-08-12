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
`c0d8b668db2527108d1c23c20d184400c51efea6`. Historical `.fsm` planning and earlier response pins are
preserved only in the exact source capsule; they do not describe the current integration contract.

## Open correspondence

<!-- fsmgen_feedback_open:start -->
- None at the sealed source boundary (`2026-08-08`).
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
| stage-contract-bugs | resolved | SpecForge → FSMGen | [issue episode](archive/fsmgen-feedback/source-through-2026-08-08.md#tracked-finding-2026-05-18--isf_downstream_integration_specmd-118-doc-vs-strict-mismatches) | [eventually-flat resolution](fsmgen-issues/sf-isf-contract-eventually-flat/RESOLUTION.md), [ready/valid resolution](fsmgen-issues/sf-isf-stage-ready-valid/RESOLUTION.md) |
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
