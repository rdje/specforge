# CORPUS-COVERAGE — refreshes 49–56

- Part ID: `refreshes-49-56`
- State: `active`

## CORPUS-COVERAGE.2.49

- Status: `done` (`2026-08-10`, DATA/DOC)
- Goal: re-ingest the Generic Interrupt Controller Overview Guide with the current release, then rebuild and
  validate EvidenceIR → SemanticIR → IntentIR → ISF without treating glossary, architecture-overview prose,
  generic sections, diagram labels, or administrative material as typed hardware authority.
- Document key: `198123_0302_03_2025_04_22_generic_interrupt_controller_overview_guide`
- Source: `.cache/local-references/chipdoc/arm/system-ip/gic/current/198123_0302_03_2025-04-22_Generic_Interrupt_Controller_Overview_Guide.pdf`
- Children: none unless the fresh result exposes a generic defect that cannot safely remain inside this refresh.

### Selection and source authority

The durable frontier contains eight real documents. The existing smallest-retained-source policy selects this
guide at 430 elements; all alternatives are larger:

| Rank | Document key | Elements | Pages | Source bytes |
| --- | --- | ---: | ---: | ---: |
| 1 | `198123_0302_03_2025_04_22_generic_interrupt_controller_overview_guide` | 430 | 45 | 1,571,128 |
| 2 | `opencapi_data_link_layer_v20_09jul2020` | 527 | 57 | 468,716 |
| 3 | `109242_0100_01_2023_09_04_arm_smmu_software_guide` | 600 | 52 | 878,792 |
| 4 | `opencapi_25gbps_phy_mechanical_spec_v10` | 760 | 34 | 4,494,801 |
| 5 | `opencapi_3_0_transaction_layer_28jan2020` | 774 | 121 | 712,534 |
| 6 | `opencapi_3_1_transaction_layer_28jan2020` | 870 | 137 | 870,740 |
| 7 | `lpc_memory_agent_reference_design_guide_17jul2020` | 891 | 59 | 1,356,427 |
| 8 | `den0034_a_2013_09_13_debug_and_trace_configuration_and_usage_models` | 936 | 48 | 946,338 |

The source resolves through the owner-authorized SSD-local `chipdoc` symlink. Source and repository share device
`16777240`; the PDF is 1,571,128 bytes at SHA-256
`5358701e080664f2db98fb9d066a0c90a4deb2ba34ca35c81597332534707e97`.

### Exact stale-chain boundary

The normalized bundle is absent. The current seven-file chain totals 1,400,092 bytes:

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| SourceIR | 231,780 | `5c1d5c6033b9194900da479daacc286cc8767a01582a9eed765d5a8c0bfb58a4` |
| EvidenceIR | 418,346 | `08a374b5accb1de4c038a54dbe486df034689af5e5b86e9daa9cbe2684b620d1` |
| Evidence validation | 13,854 | `afdafc8acf30f9891eafec7809a2277c874f831be0fffa4b62a54c013587fcf7` |
| SemanticIR | 312,067 | `ed3077b6ae7e42595614d53be29c45584eadf08dec7369d666075fe4ca5bcc98` |
| IntentIR | 409,531 | `fef383275d611ca7e41c4c97f46373555bdee623a1d950e682cce5605e390938` |
| Adapter manifest | 8,703 | `893df07c3869ee7faa5b98f39e09c7d8548de53678e7e32f3e9998cc84a181a1` |
| `controller.isf` | 5,811 | `3b7108778ba2015998771b511cb32616e4d01de2990203b4fa40e13efc8e3c1a` |

Stale SourceIR contains 45 pages / 44 visuals / 13 tables / 58 sections / 430 elements and still persists the
retired boot-volume source path. EvidenceIR has 439 statements / 33 links / zero relations / 12 conditionals /
zero registers / three timings. SemanticIR has seven actors / 83 interfaces / zero ports or relations / 26
phases / 54 gates / 87 invariants / four contracts / one assertion / three timings. IntentIR has six actors / 83
interfaces / 83 behaviors / 124 constraints / nine assumptions / one transaction / three timings. The adapter is
renderable and emits `controller.isf`; the fresh run must prove whether that surface remains grounded.

The release binary is the `.2.48a` final binary at SHA-256
`efb57ab3904c1652605624e79b64abf087d7bed6b0ae79dd5d72f2a3697a8ab4`; the latest Rust authority is commit
`7eda2928ac086fdb8bbeb91cac9250e0fe1beca0`.

### Current result

Two guarded CPU ingests from the directly resolved, caller-authorized same-SSD external input reproduce the
45-page / 44-visual / 13-table / 58-section / 430-element SourceIR. The normalized bundle contains 139 files /
30,731,394 bytes at SHA-256 `b1905855cbacf03a4a2fc07644f2418749d85a00d1d02e78fe3bb46cde1e4b56`.
The source counts hold exactly; current table classification changes the CPU-family compatibility matrix from
`timing_parameter` to `feature_matrix`, so three false GICv2/GICv3/GICv4 timing records disappear.

EvidenceIR retains 439 statements / 33 links / 12 narrative conditionals and has zero typed signals, relations,
registers, or timings. Validation classifies the self-declared overview as a high-confidence methodology guide.
Current generic authority removes 83 heuristic interface groups built from CPU names, exception levels, register
identifiers, and interrupt labels; it also retires 26 section-derived phases and 54 whole-prose gates. SemanticIR
retains seven actors / 87 invariants / four contracts / one assertion. IntentIR retains three actors / four
grounded contract behaviors / 88 constraints / nine assumptions / one recognition-only transaction, with zero
interfaces or timings. Adapter lowering blocks only on `no signals declared in interface`, emits no target, and
removes the stale `controller.isf`.

The final validated artifacts reproduce twice:

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| SourceIR | 234,486 | `421166d5f13c914229462e034cc0311e90b0891e9cc783fa08f5f9d364181dd5` |
| Source validation | 2,330 | `57cdf6e4a5c17921b9b65aff36798be914121e1aba14b6192e6604598dae493e` |
| EvidenceIR | 423,844 | `6468c0a74ddcad621ff79214a92c339aa8853db5a6998886df9238c49ab286c9` |
| Evidence validation | 14,327 | `8d4648e0ae16ef47a4a7c6e7065a04acfa8000905a52e15c32e871132ec18d65` |
| SemanticIR | 103,167 | `a9efb0f0ceda67b04448bbdbf9b47691820385718ff4cdf90e89cf85da9e76f6` |
| Semantic validation | 7,462 | `c992378923416bc0acfe5e7eb77d21bfc3278cb9ef919a752e4374626e1bee52` |
| IntentIR | 150,437 | `64ab094a652fe56043a474d1b2b0ed0d56802aafea0da21416dd523452862376` |
| Intent validation | 9,278 | `5e743c6c7cb9dcae5262f5550721aa0be31dbed35887da74186c8b0fd1022702` |
| Adapter manifest | 3,620 | `47724699ba894ecad6b62c7a7f3819de4be5c32a5cbdb2cafeeaef882238063e` |
| Adapter validation | 1,794 | `ef2a75255de63f5ee02f56cfada2b60031059c5b318e819923791894181a7c22` |

The eight downstream artifact/report hashes combine to
`99bd34c14c5bb638df35e183ab4dea68f6e7a2769f5a6ea81fff915d33a89a00`.

### Acceptance

- [x] Authenticate rollback copies of every stale chain artifact inside a repository-derived same-volume workspace.
- [x] Run guarded CPU ingest with the 85% memory abort ceiling and no off-volume project temp/cache/output.
- [x] Rebuild and validate the complete current-binary chain; rerun to establish deterministic hashes where required.
- [x] Classify every stale→fresh delta at its first causal stage and refuse unsupported hardware authority.
- [x] Run focused validation, provider-free evals, KG fixtures, emitted-ISF FSMGen strict checks, doctrines, mdBook,
  project-path/locality checks, and broader CI proportional to any code change.
- [x] Update root, active part, index/manifest/contract, roadmap, current status, live docs/book, facts, and memory;
  commit before deleting authenticated rollback evidence or moving to refresh #50.

### Decisions and incidents

- `2026-08-10`: selection uses the existing smallest-retained-source rule; no vendor or document exception is
  introduced. The GIC overview wins at 430 elements, ahead of the 527-element OpenCAPI Data Link Layer document.
- `2026-08-10`: an initial read-only candidate census mistakenly wrote directory names to
  `/tmp/specforge-current-source-keys.txt`. It contained no source content, was deleted immediately, and an exact
  absence check passed. All subsequent census work is in-memory; every task-owned workspace/output remains on the
  repository volume. This incident does not authorize `/tmp` for later workflow steps.
- `2026-08-10`: ingest through the ignored `.cache/local-references/chipdoc` symlink failed closed before mutation
  because a repository-looking persisted source may not escape through a symlink. The stale SourceIR hash and
  normalized absence re-verified. The successful runs used the directly resolved caller-authorized same-SSD input,
  persisted it as `external_input`, and kept every project-owned output repository-relative.
- `2026-08-10`: verification found one stale fact-card command naming removed
  `scripts/check_persisted_paths.pl`; the canonical checker is `scripts/check_persisted_artifact_paths.pl`.
  The existing fact is corrected in this slice; no executable or gate was missing.

### Verification log

| Date | Boundary | Result |
| --- | --- | --- |
| `2026-08-10` | ownership selection | eight candidates ranked; same-device source/hash and seven-file stale chain authenticated; normalized bundle absent; no generated mutation |
| `2026-08-10` | prospective ownership topology | focused task-evidence, catalog, Knowledge Map, 720-file live-document gate, mdBook test/build, and diff checks pass |
| `2026-08-10` | rollback and locality | seven rollback files / 1,400,092 bytes match every stale hash on device `16777240`; symlink invocation fails before mutation; direct external input persists with the correct origin label |
| `2026-08-10` | deterministic refresh | two guarded ingests reproduce 139 files / 30,731,394 bytes, bundle hash, and all ten artifact/report hashes; five stages validate; no emitted target |
| `2026-08-10` | no regression | EvidenceIR 350/five ignored; nine provider-free datasets; KG 156/156; 58/58 FSMGen strict; 2,508 artifacts / 357,904 paths; full CI 1,798/five ignored; locality pass |

### Commit log

| Unit | Durable evidence |
| --- | --- |
| `CORPUS-COVERAGE.2.49` ownership | `d23c26b4` — `CORPUS-COVERAGE.2.49 — own GIC overview refresh` |
| `CORPUS-COVERAGE.2.49` completion | `CORPUS-COVERAGE.2.49 — refresh GIC overview without false topology` |

## CORPUS-COVERAGE.2.50

- Status: `active` (`2026-08-10`, DATA/CODE/DOC; child `.2.50a` active)
- Goal: re-ingest the OpenCAPI Data Link Layer v2.0 specification with the current release, then rebuild and
  validate EvidenceIR → SemanticIR → IntentIR → ISF without promoting glossary entries, encoding-table labels,
  protocol prose, or diagram text into unsupported signals, enums, phases, gates, or executable behavior.
- Document key: `opencapi_data_link_layer_v20_09jul2020`
- Source: `.cache/local-references/chipdoc/cxl/opencapi/current/OpenCAPI-Data-Link-Layer_v20_09JUL2020.pdf`
- Children: `.2.50a` passive-binding constraint-subject authority repair (active).

### Selection and source authority

The durable frontier contains seven real documents. The established smallest-retained-source policy selects this
specification at 527 elements; all alternatives are larger:

| Rank | Document key | Elements | Pages | Source bytes |
| --- | --- | ---: | ---: | ---: |
| 1 | `opencapi_data_link_layer_v20_09jul2020` | 527 | 57 | 468,716 |
| 2 | `109242_0100_01_2023_09_04_arm_smmu_software_guide` | 600 | 52 | 878,792 |
| 3 | `opencapi_25gbps_phy_mechanical_spec_v10` | 760 | 34 | 4,494,801 |
| 4 | `opencapi_3_0_transaction_layer_28jan2020` | 774 | 121 | 712,534 |
| 5 | `opencapi_3_1_transaction_layer_28jan2020` | 870 | 137 | 870,740 |
| 6 | `lpc_memory_agent_reference_design_guide_17jul2020` | 891 | 59 | 1,356,427 |
| 7 | `den0034_a_2013_09_13_debug_and_trace_configuration_and_usage_models` | 936 | 48 | 946,338 |

The source resolves through the owner-authorized SSD-local `chipdoc` symlink. Source and repository share device
`16777240`; the PDF is 468,716 bytes at SHA-256
`778ecc99a791716a6f793008b300732cbb79bcb683ae797789cb61d2dd875ec7`.

### Exact stale-chain boundary

The normalized bundle is absent. The current seven-file chain totals 2,666,088 bytes:

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| SourceIR | 815,888 | `fd41a46af955e58d9908cc7f895df41322846db2ee4bce15e515c9c4c1596f65` |
| EvidenceIR | 1,127,724 | `126dc749d7857b6b7e4ebb0f56c33b43ac2563589b5605845020e1403a47a1c2` |
| Evidence validation | 13,766 | `22a6255000afc797e1845673052c369eb84ce984b438772a7ce400e893a95232` |
| SemanticIR | 290,691 | `8eb5fc58092b4780f78ba1020485afbb2198c759ba87329f9fa375cd681c13eb` |
| IntentIR | 413,737 | `d448aefe2185e5ef1bc74cf0960e1b047fa68dc9b80eece3eecab553481edea4` |
| Adapter manifest | 3,249 | `70f4eb0276279bfd9bd08918b642ba3b1222c90e68ccf6f2d4e9d30d09a5f46b` |
| `endpoint_dlx.isf` | 1,033 | `e9ced2f0d7b6e3f99b008da3dfd95a1fe52289319186f24592b9d7b5c9fd0964` |

Stale SourceIR contains 57 pages / 64 visuals / 53 tables / 111 sections / 527 elements and persists the retired
boot-volume source path. EvidenceIR has 1,176 statements / 1,308 links / four actor-signal relations / 14
conditionals / zero registers or timings. SemanticIR has ten actors / four interfaces / four ports and relations /
54 phases / 71 gates / 88 invariants / 38 contracts / four assertions. IntentIR has ten actors / four interfaces /
144 behaviors / 114 constraints / one assumption and no transaction or timing record. The renderable adapter emits
`endpoint_dlx.isf` with synthetic-looking `CDR` and `DL` outputs plus `REPRESENTATION` and `DL` enums; the fresh
chain must establish which, if any, of those surfaces are grounded protocol authority.

The release binary is the `.2.48a` final binary at SHA-256
`efb57ab3904c1652605624e79b64abf087d7bed6b0ae79dd5d72f2a3697a8ab4`; the latest Rust authority is commit
`7eda2928ac086fdb8bbeb91cac9250e0fe1beca0`.

### Acceptance

- Authenticate rollback copies of every stale chain artifact inside a repository-derived same-volume workspace.
- Run guarded CPU ingest from the directly resolved caller-authorized same-SSD input, with the 85% memory abort
  ceiling and no off-volume project temp/cache/output.
- Rebuild and validate the complete current-binary chain; rerun to establish deterministic hashes where required.
- Classify every stale→fresh delta at its first causal stage and refuse unsupported hardware authority.
- Run focused validation, provider-free evals, KG fixtures, emitted-ISF FSMGen strict checks, doctrines, mdBook,
  project-path/locality checks, and broader CI proportional to any code change.
- Update root, active part, index/manifest/contract, roadmap, current status, live docs/book, facts, and memory;
  commit before deleting authenticated rollback evidence or moving to refresh #51.

### Decisions and incidents

- `2026-08-10`: selection uses the established smallest-retained-source rule; no vendor or document exception is
  introduced. The OpenCAPI Data Link Layer document wins at 527 elements, ahead of the 600-element Arm SMMU guide.
- `2026-08-10`: the persisted-path contract already proves that a repository-looking path may not escape through
  the ignored `chipdoc` symlink. Runtime ingest will use the directly resolved caller-authorized same-SSD input and
  keep every project-owned workspace/output repository-relative; no failed symlink ingest is repeated.

### Verification log

| Date | Boundary | Result |
| --- | --- | --- |
| `2026-08-10` | ownership selection | seven candidates ranked; same-device source/hash and seven-file stale chain authenticated; normalized bundle absent; no generated mutation |

### Commit log

| Unit | Durable evidence |
| --- | --- |
| `CORPUS-COVERAGE.2.50` ownership | `CORPUS-COVERAGE.2.50 — own OpenCAPI data-link refresh` |

## CORPUS-COVERAGE.2.50a

- Status: `active` (`2026-08-10`, PROBE/CODE/DATA/DOC; ownership only)
- Parent: `CORPUS-COVERAGE.2.50`; parent completion pauses until this generic repair is committed and the OpenCAPI
  chain is rebuilt from the repaired release.
- Goal: prevent uppercase tokens that occur only after a passive normative binding—or in unrelated later prose—
  from becoming deterministic signal-constraint subjects, without denying any protocol, vendor, document, or
  observed identifier.

### Reproduction and root cause

The first current-binary parent cascade removes the stale `CDR`/`DL` interface and emitted target, but its four
remaining pattern constraints are false:

| Subject | Fabricated value | Actual source role |
| --- | --- | --- |
| `CAPI` | `RESET` | uppercase suffix inside `OpenCAPI`, after `shall be held in reset` |
| `OCDE` | `RESET` | real signal mention after the binding; the sentence constrains the endpoint, not `OCDE` |
| `CAPI` | `COMPATIBLE` | later protocol-name suffix; the binding constrains lane reversal |
| `DLX` | `COMPATIBLE` | later device abbreviation; the binding constrains lane reversal |

Those records create four ungrounded temporal rules and a false `CAPI=RESET` versus `CAPI=COMPATIBLE` temporal
conflict. Adapter lowering still fails closed on no declared signals and emits no target, but the EvidenceIR,
SemanticIR, and IntentIR constraint surfaces are not acceptable.

The shared defect is in `crates/specforge/src/ir/evidence.rs`. Both `extract_signal_constraints` and
`extract_dynamic_signal_constraints` collect every uppercase token in a passive obligation fragment. The
declared-signal filter intentionally becomes a no-op when the catalog is empty. Pattern extraction can then use
tokens after `must/shall be/remain` as subjects; when the bounded clause has no candidate, its full-text fallback
can also sweep unrelated later sentences. Existing dotted-reference, descriptive-field, value-position, and
condition gates do not express the missing grammatical rule: a passive binding's subject must precede its binding
lead.

### Corpus measurement and selected repair

A read-only census over all current persisted EvidenceIR artifacts separates the two deterministic id families
and ignores table-row sources, whose row cells can supply legitimate subject context. Of 179 `sigcon_*` records,
21 across nine documents have no whole-identifier occurrence before their passive binding lead. Of 81
`dyn_sigcon_*` records, ten across three documents have that shape. The union is 31 records / ten documents.
Manual source-role audit classifies all 31 as false subjects: later conditions or scopes (`BCOMP`, `HRESP`, `SCL`,
`CKE`), protocol/device names (`CAPI`, `DLX`, `DTI`, `WISHBONE`, `PCI`), non-subject fields or values (`OAS`,
`DID`, `IODIR`, `FFFF`), and unrelated later signal mentions. No audited record states an obligation about the
candidate token.

Implement one pure universal predicate shared by both deterministic extractors. For non-table prose carrying
`must/shall [not] be/remain`, retain a candidate only if it occurs as a whole identifier before the first passive
binding lead in the same constraint-bearing sentence. Keep table-row context unchanged, and do not alter active
`must drive/set <signal>` grammar. Apply the predicate beside the existing `.3e`/`.3g` subject gates. No name,
document, protocol, vendor, value, or growing denylist is permitted.

### Acceptance

- Add focused unit tests for both extractor paths: the two exact OpenCAPI paragraphs produce no constraint; normal
  `PSEL must be HIGH`, multi-signal passive obligations, active `must drive PSTRB LOW`, and table-row subject
  context remain accepted.
- Re-run the 31-record census against the predicate and prove exact removal with no candidate outside the measured
  class; rebuild the OpenCAPI chain and require zero signal constraints, temporal rules/conflicts, interfaces, or
  emitted target.
- Run all nine provider-free APB/AHB/AXI/SWD/I2C datasets, focused EvidenceIR tests, KG fixtures, full CI, current
  emitted-ISF FSMGen strict checks, mdBook, doctrines, persisted paths, and project-data locality.
- Record the generic causal fact, child/parent result, code/book/live-doc changes, hashes, and verification before
  parent `.2.50` resumes and completes from the repaired committed release.

### Verification log

| Date | Boundary | Result |
| --- | --- | --- |
| `2026-08-10` | first parent cascade | four false constraints → four ungrounded temporal rules / one false conflict; adapter safely blocked |
| `2026-08-10` | read-only corpus census | 179 pattern + 81 dynamic records measured; 31 non-table post-binding-only subjects / ten documents; 31/31 audited false |
| `2026-08-10` | clean handoff | pre-ingest seven-file chain restored at 2,666,088 bytes / all stale hashes; normalized and three new reports absent; post-ownership rollback deletion has zero residue; locality passes |

### Commit log

| Unit | Durable evidence |
| --- | --- |
| `CORPUS-COVERAGE.2.50a` ownership | `26cfc13e` — `CORPUS-COVERAGE.2.50a — own passive-binding subject repair` |
| `CORPUS-COVERAGE.2.50a` clean handoff | `CORPUS-COVERAGE.2.50a — checkpoint clean repair handoff` |
