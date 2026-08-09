# Legal and administrative semantic-authority measurement — `CORPUS-COVERAGE.2.41a`

Owning leaf: `CORPUS-COVERAGE.2.41a` (PROBE/CODE/DATA/DOC).
Date: `2026-08-09`.

## Trigger and question

The current-binary refresh of `opencapi_afu_address_space_usage` preserved EvidenceIR
`statement_0011`:

> The limited permissions granted above are perpetual and will not be revoked by OpenCAPI or its successors or
> assigns while the specification in question is the current version. Upon release by the OpenCAPI Consortium of
> a new version, all the above rights shall automatically cease.

The statement is faithful source evidence, but it is a legal condition rather than engineering intent. Docling
placed it under the generic heading `Approved`. `SemanticContext::from_evidence_ir` previously excluded only
recognized boilerplate *section titles*, while `build_gates` accepted the word `while` as conditional authority.
The paragraph therefore became one SemanticIR gate, helped mint an `Approved` phase, entered the invariant set,
and became an IntentIR behavior.

The measurement asks whether this is a document-specific anomaly, how broad a safe repair can be, and which
technical controls must survive.

## Retained-corpus measurement

The before census reads 27,180 gates from 79 retained SemanticIR documents. A word-boundary projection of the
selected production predicate identifies 32 legal or administrative gates across 21 documents. Twenty-six of
the 32 have no related interface. The matches include:

- rights, copyright, permission, derivative-work, and license notices;
- warranty, liability, and commercial-sale terms;
- patent, infringement, intellectual-property, and IPR declarations;
- confidential-document access notices.

The set spans OpenCAPI, CCIX, NVMe, Arm, AMD, Intel, ETSI, NXP, USB, SMBus, and Wishbone material. It is therefore
not safely modeled as a vendor, document, section, or sentence exception. The retained artifacts are mixed-vintage
snapshots; the measurement describes the family and does not rewrite those 31 unrelated current artifacts. Their
normal corpus refreshes will apply the repaired semantic boundary.

The initial broad-token probe was rejected. It also matched legitimate technical language such as:

- a controller granting write permission after ownership transfer;
- access-right fields assigned while a request remains valid;
- protocol-version selection and lifecycle behavior;
- reliability status and error handling;
- NVMe's literal ASCII string `Copyright`;
- a register named `license`.

Those examples prove that individual words are not a safe authority boundary.

## Selected boundary

`is_legal_or_administrative_statement` requires a word-bounded compound signature. Examples are copyright plus
document/notice/license context, warranty or liability plus document/product/customer context, patent plus
rights/infringement context, and permissions plus revocation/successor context. No single token is sufficient.

The predicate runs once at the earliest shared SemanticIR statement boundary, before section lookup and before
actors, phases, invariants, gates, interfaces, or decompositions consume the statement. EvidenceIR remains
lossless: legal text and its source spans are still available for audit. IntentIR receives no semantic record
from prose that failed this authority check.

This placement is deliberate. Filtering only `build_gates` would leave the same non-engineering paragraph free
to become a phase, invariant, actor, or later semantic surface. Filtering EvidenceIR would instead erase valid
source evidence. The shared SemanticContext boundary separates preservation from semantic promotion.

## Real-document result

The repaired release binary hashes to
`361dd97f983f832a899a281249f441ddddfade2eb52508def00a403d2b284a8c`. Rebuilding the real #41 chain from
unchanged EvidenceIR produces:

| Surface | Before repair | After repair |
| --- | ---: | ---: |
| EvidenceIR statements | 106 | 106 |
| SemanticIR actors / interfaces | 0 / 0 | 0 / 0 |
| SemanticIR phases | 1 | 0 |
| SemanticIR invariants | 17 | 15 |
| SemanticIR gates | 1 | 0 |
| SemanticIR decompositions | 14 | 14 |
| IntentIR behaviors | 2 | 0 |
| IntentIR constraints / temporal invariants | 17 / 17 | 15 / 15 |
| adapter signals / rules | 0 / 0 | 0 / 0 |

The two removed invariants and both removed behaviors are the legal copyright/permissions paragraphs, not
engineering intent. The adapter remains honestly blocked on no declared signals plus no behavioral content and
emits no target. Two complete post-repair semantic→intent→adapter cascades reproduce these six artifact/report
hashes:

- SemanticIR/report: `2f38bc36d6f451a5785ef4b47e3a9ad4465357388c3ff6b9c8bfe90aa48dedf7` /
  `c879198903e8c7db059958100e3d15403f38b2b454b877542dfd66be743ee7d4`
- IntentIR/report: `03e9adb9061001f4d4e86d2e524b5ea25205e980349342d62bbfd20af61473a1` /
  `d4846b1b8fea8d72611393973d15bc164f27765e40f8db03c394d05f9a460e2e`
- adapter/report: `64d5a9f5eca28644045a67d1ac810fda1f772ffd4a880f6a38c28210410343bb` /
  `43a2a01373251e8940bceee7149f32de0c6c86ce14e349c45450df13d5883740`

The resulting retained corpus has 27,179 gates. The same projection finds 31 remaining legal/administrative
gates across 20 older artifacts: exactly the before set minus #41's false gate.

## Verification

- Two paired regressions cover compound legal signatures, technical permission/right/version/reliability and
  literal-`Copyright` controls, legal text under `Approved`, a real `While READY ... VALID` gate, and the downstream
  IntentIR behavior boundary.
- All 399 SemanticIR module tests pass.
- All nine provider-free WIRE/I2C/SWD datasets hold filtered F1 1.000 on their declared surfaces; the documented
  promotion-only SWD constraint remains 0/1 rather than being hidden.
- KG bench passes 156/156 fixtures.
- Full CI passes 1,790 tests with five ignored, warning-deny Clippy, rustdoc, mdBook, doctrines, and locality.
- All 66 current emitted ISFs pass FSMGen strict with zero diagnostics.
