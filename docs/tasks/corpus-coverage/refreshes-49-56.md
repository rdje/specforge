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

- Status: `done` (`2026-08-10`, DATA/CODE/DOC; child `.2.50a` done)
- Goal: re-ingest the OpenCAPI Data Link Layer v2.0 specification with the current release, then rebuild and
  validate EvidenceIR → SemanticIR → IntentIR → ISF without promoting glossary entries, encoding-table labels,
  protocol prose, or diagram text into unsupported signals, enums, phases, gates, or executable behavior.
- Document key: `opencapi_data_link_layer_v20_09jul2020`
- Source: `.cache/local-references/chipdoc/cxl/opencapi/current/OpenCAPI-Data-Link-Layer_v20_09JUL2020.pdf`
- Children: `.2.50a` passive-binding constraint-subject authority repair (done).

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

### Fresh current-binary result

Two guarded CPU ingests reproduce 57 pages / 64 visuals / 53 tables / 111 sections / 527 elements and a 183-file /
51,754,156-byte normalized bundle at SourceIR `13769977…ca7e`, with 83–84% system memory free throughout and every
project-owned workspace repository-relative. Two full downstream cascades then reproduce all seven current
artifact and report hashes, including EvidenceIR `590b53ab…fb14`, SemanticIR `b2740965…6b8a`, IntentIR
`a8c747e8…582d`, and adapter manifest `415c0f43…f6f0`.

Every stale→fresh delta is classified at its first causal stage:

| Surface | Stale | Current | First cause |
| --- | ---: | ---: | --- |
| Source elements | 527 | 527 | structure holds; only the retired boot-volume provenance is replaced |
| Evidence statements | 1,176 | 1,018 | flattened visual-label and duplicate-presentation suppression |
| Actor-signal relations | 4 | 0 | the document declares no interface signals to relate |
| Signal constraints | 0 | 0 | `.2.50a` keeps the four post-binding candidates out of the subject slot |
| Semantic actors / interfaces / ports | 10 / 4 / 4 | 6 / 0 / 0 | retired heuristic interface and port synthesis |
| Semantic phases / gates / contracts | 54 / 71 / 38 | 0 / 0 / 0 | retired prose phase, gate, and contract authority |
| Semantic invariants / assertions | 88 / 4 | 87 / 4 | one invariant loses its statement source |
| Intent actors / behaviors / constraints | 10 / 144 / 114 | 4 / 24 / 87 | the removed interface, phase, and gate topology |
| Emitted target | `endpoint_dlx.isf` | none | lowering blocks honestly on no declared interface signals |

The stale target's `CDR`/`DL` outputs and `REPRESENTATION`/`DL` enums had no grounded protocol authority; the
current chain refuses them and retains no residual decisions. Conditional rules hold at 14, and registers and
timing constraints remain honestly absent.

### Verification log

| Date | Boundary | Result |
| --- | --- | --- |
| `2026-08-10` | ownership selection | seven candidates ranked; same-device source/hash and seven-file stale chain authenticated; normalized bundle absent; no generated mutation |
| `2026-08-10` | deterministic refresh | two guarded ingests reproduce the bundle and SourceIR `13769977…ca7e`; two cascades reproduce all seven downstream hashes; four stages validate |
| `2026-08-10` | bundle measurement corrected (`CORPUS-CHAIN-CURRENCY.2`) | the originally recorded 184 files / 52,570,034 bytes measured the whole document root; the `normalized/` bundle itself is 183 files / 51,754,156 bytes, and the 815,878-byte difference is exactly this document's `source_ir.json`. Re-measured referentially complete: 5 manifests + 64 asset crops + 57 page images + 57 page sidecars, every path in `page_artifacts.json`/`visual_assets.json` present, no file lost |
| `2026-08-10` | no regression | KG 156/156; 57/57 current emitted ISFs FSMGen strict; nine provider-free evals at baseline; full CI 1,804/five ignored; doctrines, persisted paths, and locality pass |
| `2026-08-10` | corpus frontier | 50 done / six remaining at 80 SourceIR / 22 normalized / 80 EvidenceIR / 79 downstream chains |

### Commit log

| Unit | Durable evidence |
| --- | --- |
| `CORPUS-COVERAGE.2.50` ownership | `9c83062e` — `CORPUS-COVERAGE.2.50 — own OpenCAPI data-link refresh` |
| `CORPUS-COVERAGE.2.50` completion | `CORPUS-COVERAGE.2.50 — refresh OpenCAPI data link without false topology` |

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.50`

- [x] **REPRODUCE / MEASURE** — the same-device 468,716-byte source at `778ecc99…5ec7` and the exact seven-file /
  2,666,088-byte stale chain are authenticated, and a rollback capsule of all seven files is taken inside a
  repository-derived same-volume workspace before ingest.
- [x] **ROOT CAUSE (WHY + WHERE)** — every stale→fresh delta is classified at its first causal stage in the table
  above; the removed interface, port, phase, gate, and contract topology explains the behavior and constraint
  movement, and the four false post-binding constraint subjects are owned and closed by child `.2.50a`.
- [x] **ADDRESSED (verified)** — the complete current-binary chain builds and validates with zero signal
  constraints, temporal rules, temporal conflicts, interfaces, and emitted targets, retaining 87 invariants, four
  assertions, 24 behaviors, and 87 constraints; `adapt` blocks on `no signals declared in interface`.
- [x] **NO REGRESSION** — `kg-bench` 156/156; 57/57 current emitted ISFs pass FSMGen `--strict --check`; nine
  provider-free evals hold their recorded baseline; full CI is 1,804 tests / five ignored; doctrines, mdBook,
  persisted-path, live-size, and project-data locality gates pass.
- [x] **GENERICITY (ADR 0006)** — no OpenCAPI, vendor, document, acronym, or token exception enters production; no
  artifact was hand-edited and no validator was relaxed.
- [x] **LOCKSTEP** — generated chain, this leaf, the tree root and frontier, roadmap, current status, live docs,
  mdBook, the Knowledge Map fact card, and the resume pointer agree before commit; the authenticated rollback
  capsule is deleted only after durable recording.

## CORPUS-COVERAGE.2.50a

- Status: `done` (`2026-08-10`, PROBE/CODE/DATA/DOC)
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

### Implemented repair

`crates/specforge/src/ir/evidence.rs` gains one pure predicate, `is_post_passive_binding_only_subject`, applied in
both deterministic paths beside the existing `.3e`/`.3g` subject gates. It keeps a candidate only when the document
names it at identifier boundaries before the first passive binding lead of its constraint-bearing sentence, where a
lead is a `must`/`shall` modal, optionally negated by `not`/`never`, immediately followed by `be`/`remain`
(`first_passive_binding_lead`). Table-row sources are exempt because a row's other cells legitimately supply the
subject its obligation cell then constrains. An active binding states its object after the verb, carries no passive
lead, and is therefore untouched: `must drive PSTRB LOW` and `must have its WSTRB input tied HIGH` both survive by
construction. The shared identifier-boundary test is factored into `contains_whole_identifier`, which now also
serves the `.3e` gate in place of its private duplicate scan. No name, document, protocol, vendor, or value list
enters production.

### Measured effect

The pre-repair persisted corpus carries 175 `sigcon_*` and 81 `dyn_sigcon_*` deterministic records. The predicate
selects 26 of them — 17 pattern records across eight documents and nine dynamic records across two — and each was
audited as a later condition or scope (`BCOMP`, `HRESP`, `SCL`, `CKE`), a protocol or device name (`WISHBONE`,
`PCI`, `DTI`), the modal word `MUST` itself, or a non-subject field (`OAS`, `DID`, `IODIR`). Adding the four
OpenCAPI records the parent cascade exposed reproduces the recorded pattern leg exactly at 21 across nine
documents.

An isolated old-versus-new `evidence --dry-run` replay over all 21 rebuildable documents changes exactly three of
them and removes exactly eight records with none added; the other 18 are byte-identical, and inside the three the
only other movement is sequential id renumbering. The three affected rebuildable cascades were then rebuilt and
validated through all four stages, twice, byte-identically at combined hash `3538fec9…fd49`:

| Document | Constraints | Temporal rules | Actor contracts | Fidelity findings | Temporal invariants |
| --- | --- | --- | --- | --- | --- |
| `um10204_rev7_0_2021_i2c_bus_specification` | 11 → 10 | 11 → 10 | 7 → 6 | 42 → 36 | 144 → 143 |
| `usb_3_2_revision_1_0_2017_09` | 10 → 9 | 10 → 9 | 10 → 9 | 60 → 54 | 3,014 → 3,013 |
| `wbspec_b4_wishbone_b4_specification` | 42 → 36 | 42 → 36 | 24 → 20 | 144 → 120 | 240 → 234 |

The only emitted-target movement is I2C `wiring_patterns.isf`, which loses exactly the false `(SCL 1)` rule minted
from a timing footnote; everything else in that file is renumbering, and all 58 current emitted targets stay
FSMGen-strict clean. The six affected non-rebuildable documents — AXI, AHB, DTI, HBM2, NVMe, and the RISC-V IOMMU
specification — keep 18 measured records and byte-exact stage directories; they clear at their own refresh leaves.

### Decisions and incidents

- `2026-08-10`: the passive lead is restricted to the `be`/`remain` copula on purpose. Widening it to `have` would
  additionally select NVMe `dyn_sigcon_0011` (`FFFF`), but it would also delete the real AXI `WSTRB` and Arm
  low-power `PREQ` obligations of the form `must have its <signal> input tied HIGH/LOW`. The `FFFF` record is a
  distinct value-position defect — a hex literal in `shall have … set to FFFFh` reaching the subject slot — and is
  recorded as a separate candidate rather than forced into this grammar.
- `2026-08-10`: the three rebuilt documents' pre-existing chains were **not** reproducible under the pre-repair
  binary, so they carried older-release state. Rebuilding them therefore also advanced them across every delta
  accumulated since their own refresh, measured for USB 3.2 as timings 102 → 74, actors 18 → 16, and behaviors
  2,705 → 866 before this repair's own subtraction. The isolated repair delta is proven by the old-versus-new
  replay above, not by comparing against those superseded artifacts. Row 33 continues to describe USB 3.2 at its
  own refresh boundary.

### Verification log

| Date | Boundary | Result |
| --- | --- | --- |
| `2026-08-10` | first parent cascade | four false constraints → four ungrounded temporal rules / one false conflict; adapter safely blocked |
| `2026-08-10` | read-only corpus census | 179 pattern + 81 dynamic records measured; 31 non-table post-binding-only subjects / ten documents; 31/31 audited false |
| `2026-08-10` | clean handoff | pre-ingest seven-file chain restored at 2,666,088 bytes / all stale hashes; normalized and three new reports absent; post-ownership rollback deletion has zero residue; locality passes |
| `2026-08-10` | focused tests | six new gate cases pass: both exact OpenCAPI paragraphs yield no record on either path; single/multi-signal pre-lead subjects, active `must drive`/`must have … tied`, and table-row context survive |
| `2026-08-10` | isolated replay | 21 rebuildable documents replayed old-versus-new: 18 byte-identical, three changed, eight records removed, none added, no other section content moved |
| `2026-08-10` | rebuilt cascades | I2C / USB 3.2 / WISHBONE rebuilt and validated through four stages twice, byte-identical at combined `3538fec9…fd49`; 77 untouched documents byte-exact |
| `2026-08-10` | no regression | corpus census 26 → 18 remaining, all in non-rebuildable documents; KG 156/156; 58/58 FSMGen strict; nine provider-free evals at their recorded baseline with wire constraint/temporal golds 1.000; `cargo fmt`/clippy/test 1,804 / five ignored |

### Commit log

| Unit | Durable evidence |
| --- | --- |
| `CORPUS-COVERAGE.2.50a` ownership | `26cfc13e` — `CORPUS-COVERAGE.2.50a — own passive-binding subject repair` |
| `CORPUS-COVERAGE.2.50a` clean handoff | `b9fc4512` — `CORPUS-COVERAGE.2.50a — checkpoint clean repair handoff` |
| `CORPUS-COVERAGE.2.50a` completion | `CORPUS-COVERAGE.2.50a — require pre-bind passive constraint subjects` |

### Acceptance Checklist (enforced) — `CORPUS-COVERAGE.2.50a`

- [x] **REPRODUCE / MEASURE** — the parent cascade's four false OpenCAPI records and their two exact source
  paragraphs are pinned, and the complete deterministic corpus is censused at 175 `sigcon_*` + 81 `dyn_sigcon_*`
  with 26 in-class records across nine documents before any production code changes.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: `extract_signal_constraints`'s
  full-text fallback and `extract_dynamic_signal_constraints`'s whole-statement subject scan both admit uppercase
  tokens that occur only after a passive `must/shall be|remain` lead, and no existing dotted, descriptive-field,
  value-position, or condition gate expresses the missing grammatical rule.
- [x] **ADDRESSED (verified)** — one shared pre-lead subject predicate now guards both paths; the two exact
  OpenCAPI paragraphs emit nothing, the isolated 21-document replay removes exactly eight records and adds none,
  and the three rebuilt cascades drop the matching temporal rules, contracts, findings, invariants, and the false
  emitted `(SCL 1)` rule.
- [x] **NO REGRESSION** — `kg-bench` 156/156; 58/58 current emitted-ISF FSMGen `--strict --check`; nine
  provider-free APB/AHB/AXI/SWD/I2C evals at their recorded baseline with wire constraint and temporal golds at
  1.000 and the known `CSYSPWRUPACK` residual unchanged; `cargo fmt --all --check`, warning-deny clippy, and
  1,804 tests / five ignored green; doctrines, mdBook, persisted-path, live-size, and locality gates pass.
- [x] **GENERICITY (ADR 0006)** — the rule is modal/copula grammar plus identifier-boundary occurrence and the
  repository-wide table-row marker; no document, protocol, vendor, signal, or value list, and no artifact was
  hand-edited.
- [x] **LOCKSTEP** — code, corpus measurement, this leaf, the tree root and frontier, roadmap, live docs, mdBook,
  the Knowledge Map fact card, and the resume pointer agree before commit.
