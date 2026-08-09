# Administrative-workflow semantic-authority measurement — `CORPUS-COVERAGE.2.42a`

Owning leaf: `CORPUS-COVERAGE.2.42a` (PROBE/CODE/DATA/DOC).
Date: `2026-08-10`.

## Trigger and question

The first current-binary cascade for `opencapi_3_0_certified_definition_v1_1` faithfully retained 167
EvidenceIR statements but promoted certification administration into engineering intent. Sections for requesting
a product listing and independent test laboratories became semantic phases. Email review, conflict resolution,
derivative-product listing, certification-mark use, and submission procedure became gates and IntentIR
behaviors. The document has no declared interface, port, relation, connectivity, signal constraint, register,
timing record, or typed temporal rule, so those process records had no hardware authority.

The existing legal boundary from `CORPUS-COVERAGE.2.41a` recognizes rights, licensing, liability, warranty,
patent, and similar compound legal concepts. It does not cover organizational workflows. This measurement asks
whether the new family is corpus-wide, whether a generic structural boundary can distinguish it from protocol
request/response behavior, and which real certification requirements must survive.

## Retained-corpus measurement

A word-boundary projection of the selected administrative section and statement predicates over all retained
EvidenceIR artifacts finds 238 candidate statements across 23 documents:

| Reason | Statements |
| --- | ---: |
| reference-list section | 119 |
| internal-request-processing section | 36 |
| submission or product-listing workflow | 32 |
| certification-mark administration | 20 |
| certification-administration section | 9 |
| institutional-test-lab section | 6 |
| document-reading guidance | 5 |
| submission-administration section | 4 |
| institutional testing administration | 3 |
| institutional conflict workflow | 2 |
| certification attestation workflow | 1 |
| product qualification testing | 1 |

The 23 documents include the OpenCAPI definition/test-resource family, BSA, SWP, SMMU, LPC, NVMe, Cortex-A76,
USB4 Connection Manager, Wishbone, and Intel VT-d artifacts. A few Intel/NVMe/USB4 matches are trademark notices
that overlap the already-correct legal boundary; the administrative projection does not grant them a second or
different semantic meaning.

Across the current mixed-vintage SemanticIR snapshots, matched statements participate in 170 decomposition, 45
phase, 11 actor, ten invariant, seven gate, and one contract memberships. These are statement memberships, not
distinct semantic-record counts, and #42 had already been rebuilt by the time the final corpus projection ran.
They establish the general failure family without bulk-rewriting unrelated retained artifacts; each artifact
will take the current boundary on its normal refresh.

## Rejected broad alternatives and controls

An initial broad probe using isolated words such as `request`, `apply`, `contact`, `test`, or `conflict` was
rejected. It matched real engineering text, including:

- a RISC-V Page Request and its response;
- a USB4 Link State Status Request Packet;
- SWP electrical contacts;
- an arbiter resolving a write conflict;
- a certified device meeting electrical requirements;
- a test report recording measured receiver behavior;
- `READY`/`VALID` protocol conditions.

The selected rule therefore requires compound organizational context. Submission terms must pair with an
administrative channel or listing/mark object; conflict language must pair with institutional escalation;
test-lab language must pair with certification-program administration; certification attestation must name an
organizational actor; and qualification testing must describe market/submission process rather than measured
hardware behavior. Recognized administrative section titles use the same compound rule, except for exact generic reference-list headings. There is no vendor, consortium,
document key, organization name, section index, exact source sentence, or single-token denylist.

## Selected boundary

The repair extends the existing `SemanticContext::from_evidence_ir` authority boundary. It excludes statements
whose section or content establishes document-reading guidance, submission/listing administration,
organizational conflict resolution, certification-mark administration, institutional test-program process, or
product-qualification workflow. The filter runs before actors, phases, invariants, contracts, gates, interfaces,
and decompositions consume statements.

EvidenceIR remains lossless. Filtering EvidenceIR would erase auditable source material, while patching only
phase or gate builders would allow the same non-engineering process to enter other semantic surfaces. The shared
context boundary is the earliest point that preserves evidence and withholds semantic authority consistently.

## Real-document result

The repaired release binary hashes to
`9692af3e5019b53a6c849270b24a16a1f8bbfa4d8f94162ada23f56534c61b4b`. The EvidenceIR hash remains
`63a94ec5e44a580372d6cc1fb47faa65d086850cd763d9db3a98d4d79ba87000` before and after the repair.
Rebuilding #42 from that unchanged evidence produces:

| Surface | First fresh cascade | Repaired cascade |
| --- | ---: | ---: |
| EvidenceIR statements | 167 | 167 |
| SemanticIR actors | 5 | 2 |
| SemanticIR interfaces | 0 | 0 |
| SemanticIR phases | 5 | 0 |
| SemanticIR invariants | 20 | 8 |
| SemanticIR contracts | 6 | 4 |
| SemanticIR gates | 6 | 0 |
| SemanticIR decompositions | 22 | 14 |
| IntentIR behaviors | 15 | 4 |
| IntentIR constraints | 20 | 8 |
| adapter signals / rules | 0 / 0 | 0 / 0 |

The two surviving actors are `device` and `host`. The four surviving behaviors are real product-compliance
contracts: the device and host must meet the Ready Definition and each must have a passing transaction-layer
compliance test report. The administrative workflows disappear. Lowering remains honestly blocked on no
declared interface signals and emits no target.

Two complete semantic→intent→adapter cascades reproduce these six artifact/report hashes:

- SemanticIR/report: `2622a69b821d3af6712be0869da2da2926c2275eb2fe868c9fff0de20784fdbd` /
  `346724ddc56f3b6240cfc854cdeda5fbb19a2cea9ac74766ec9381212eda4644`
- IntentIR/report: `211d277aa7e485efa1e0b4ea1dc1ba37fd737e472fc3974f19dbf2c2148dfd7d` /
  `5824b2c6af786823f961fe22bbb5bf87dc3895687006204f7143d7f8808d7e8d`
- adapter/report: `0d55f785509264fc7cc03794a229427ace0549902cac3afea475ef997c76f7cc` /
  `deefda25d1b25215609dbbc10d2209359c3eaaafb3a71dcf3a1a81cdf318dc23`

## Verification

- Three paired regressions cover administrative section titles, compound organizational workflows, lossless
  EvidenceIR carry, downstream exclusion, and technical request/contact/conflict/compliance/measurement controls.
- All 402 SemanticIR module tests pass.
- All nine provider-free WIRE/I2C/SWD datasets retain filtered F1 1.000 on their declared surfaces; the documented
  promotion-only SWD constraint remains visible at 0/1.
- KG bench passes 156/156 fixtures.
- All 66 current emitted ISFs pass FSMGen strict with zero diagnostics.
- Full CI passes 1,793 tests with five ignored, warning-deny Clippy, rustdoc, mdBook, doctrines, and locality.
