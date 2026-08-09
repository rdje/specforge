# Semantic-phase authority measurement — `CORPUS-COVERAGE.2.43a`

Owning leaf: `CORPUS-COVERAGE.2.43a` (PROBE/CODE/DATA/DOC).  
Date: `2026-08-10`.

## Trigger and question

The first current-binary cascade for `opencapi_3_0_ready_definition_v1_1` preserved all 173 EvidenceIR
statements and found no declared signal, relation, register, signal constraint, or timing record. Nevertheless,
`SemanticIR.phases[]` contained three records and IntentIR turned all three into behaviors:

| Section promoted as a phase | Authorizing statement | Sentence cue |
| --- | --- | --- |
| `2.1 Functional test for OpenCAPI 3.0 devices` | `statement_0148` | `once` |
| `3. OpenCAPI Ready 3.0 host system definition` | `statement_0153` | `when` |
| `3.1 Functional test for OpenCAPI 3.0 hosts` | `statement_0172` | `once` |

The first and third statements say that validation completes once an application reports success. The middle
statement says that a manufacturer shall assert when it has met the listed requirements. These temporal words
describe a test procedure or qualification attestation; none establishes a hardware or transaction phase.

## Root cause

`build_phases` historically admitted every non-empty section through either side of one disjunction:

1. the section title contains one of the generic phase-heading phrases; or
2. any supporting statement contains `before`, `after`, `until`, `during`, `then`, `next`, `once`, `when`, or
   `while`.

The second path grants whole-section semantic authority to one ubiquitous sentence word. It does not require a
phase heading, a declared interface, a transaction, a signal, or any other structural corroboration. Git blame
traces the disjunction unchanged to the initial SemanticIR lifting implementation, commit `f8e42eaa9`
(`2026-04-01`).

This broad `phases[]` surface is separate from `transaction_phases[]`. The latter is built by
`build_transaction_phases` from a bounded `<qualifier> phase` grammar, records the exact supporting statements,
and intersects referenced signals with the declared inventory. Removing sentence-level authority from generic
section phases does not remove typed transaction-phase recognition.

## Exact retained-artifact census

The measurement read all 79 retained `generated/semantic_ir/*/semantic_ir.json` artifacts and their referenced
EvidenceIR. It extracted each phase title from the producer's fixed summary form and replicated production
`contains_phrase` semantics exactly: ASCII-alphanumeric boundaries around lower-cased phrases. Because the
historical producer emitted a phase only through the disjunction above, a retained phase whose title matches no
title phrase is exactly statement-fallback-only.

| Classification | Phase records | Documents |
| --- | ---: | ---: |
| all retained generic phases | 11,286 | 79 |
| title-authorized | 3,180 | 69 |
| statement-fallback-only | 8,106 | 77 |
| separately typed `transaction_phases[]` | 82 | not used by this classification |

Sequencing-cue memberships within the 8,106 fallback records are non-exclusive because one section can contain
several cues:

| Cue | Fallback phases containing it |
| --- | ---: |
| `before` | 1,174 |
| `after` | 1,525 |
| `until` | 703 |
| `during` | 756 |
| `then` | 1,770 |
| `next` | 738 |
| `once` | 303 |
| `when` | 5,731 |
| `while` | 818 |

Frequent fallback titles demonstrate that the issue is corpus-wide rather than OpenCAPI-specific:

| Title | Records |
| --- | ---: |
| `Bit descriptions` | 165 |
| `Configurations` | 158 |
| `Attributes` | 145 |
| `Note` | 136 |
| `Configuration` | 96 |
| `Approved Approved` | 81 |
| `Hardware Specification` | 51 |
| `Purpose` | 42 |
| `Field descriptions` | 37 |
| `Additional information` | 35 |

These artifacts are mixed-vintage snapshots; the census measures records already emitted by the unchanged
historical rule, not a claim that every retained downstream artifact has already been rebuilt with the repair.

## Selected boundary and controls

Generic section-derived phases now require a non-empty section with phase-like title authority. Supporting prose
can supply provenance but cannot create the section phase by itself. The rule contains no vendor, protocol,
document, section index, exact title, or exact sentence exception.

Paired controls establish the boundary:

- `Functional test for controller`, `Host system definition`, and `Bit descriptions` do not become phases merely
  because their sentences contain `once`, `when`, or `after`;
- `Setup phase`, `Reset sequence`, and `Transaction timing` remain generic section-derived phases;
- `During the address phase the manager drives HADDR` still creates a typed `address`
  `transaction_phases[]` record even when its containing section is not a generic phase.

## Real-document result

The repaired release binary hashes to
`c61f05798ebfc9cfc5a690e1f9a09cb93943dcffbfcea4f5b05cfceaeb23b117`. Rebuilding downstream from unchanged
EvidenceIR hash `410548a7067b815ab75378f90d0ef8b6705c0d9e3505c2e579ae8935abf4f34c` produces:

| Surface | First fresh cascade | Repaired cascade |
| --- | ---: | ---: |
| EvidenceIR statements | 173 | 173 |
| SemanticIR actors | 3 | 3 |
| SemanticIR interfaces | 0 | 0 |
| SemanticIR phases | 3 | 0 |
| SemanticIR transaction phases | 0 | 0 |
| SemanticIR invariants | 21 | 21 |
| SemanticIR contracts | 19 | 19 |
| SemanticIR gates | 1 | 1 |
| SemanticIR decompositions | 17 | 17 |
| IntentIR behaviors | 22 | 19 |
| IntentIR constraints | 25 | 25 |
| adapter signals / rules | 0 / 0 | 0 / 0 |

Lowering remains honestly blocked on no declared interface signals and emits no target. The remaining actor,
gate, and contract classifications are distinct authority families owned by parent `.2.43`; this phase-only
repair does not hide or reclassify them.

## Follow-on title-authority finding

The exact positive census also exposed a separate risk: the historical title phrase family is broader than a
strict phase grammar. Of the 3,180 title-authorized records, 720 are headed `Reset value`; other common headings
name registers, modes, reads, writes, and responses. Pending child `CORPUS-COVERAGE.2.43a.i` owns that audit.
It must not be conflated with the proven sentence-fallback cause or silently expanded into this repair.

## Reproduction and verification

Two complete semantic→validate→intent→validate→adapter→validate cascades reproduce all six downstream hashes:

- SemanticIR/report: `9309abd83f2379c77837cf1cac5127e06039fb1e9cfb8f77fc1a0d6cc005d729` /
  `8550ed8b8c550818e15577e4c76f6df4f0f2cac49180ec74e8629fb13bbe9421`
- IntentIR/report: `91797f8f1052360a770f217e3cc66b322831011636596a6ec3b201aebc5e9300` /
  `91ec2866703efa1a98d7b8b7c694a225e23f6ef75038ccd2f98db9d19d689f20`
- adapter/report: `908ac593db48fbb525a7deb8b2005126e0e765e4320734d44f0ae4f161c6094d` /
  `0964bbe5132d245a7fedd0b01124a1e463f6c593c579fc4b37209b1ccbceee3d`

The regression and integration gates pass:

- all 408 SemanticIR module tests;
- all nine provider-free WIRE/I2C/SWD datasets at their declared source-tolerant filtered 1.000 gates, including
  the documented promotion-only SWD constraint miss;
- KG bench 156/156;
- full CI with 1,796 tests passed / five intentionally ignored, warning-deny Clippy and rustdoc, plus mdBook;
- all 66 emitted ISFs through FSMGen `--strict --check --json`, zero failures or diagnostics;
- persisted-path census of 2,008 JSON artifacts / 355,710 path values / 93 authorized external absolutes / zero
  repository-owned absolutes;
- all six doctrines and project-data locality.
