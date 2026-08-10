# CORPUS-COVERAGE — refreshes 49–56

- Part ID: `refreshes-49-56`
- State: `active`

## CORPUS-COVERAGE.2.49

- Status: `active` (`2026-08-10`, DATA/DOC; ownership and exact stale boundary only)
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

### Acceptance

- Authenticate rollback copies of every stale chain artifact inside a repository-derived same-volume workspace.
- Run guarded CPU ingest with the 85% memory abort ceiling and no off-volume project temp/cache/output.
- Rebuild and validate the complete current-binary chain; rerun to establish deterministic hashes where required.
- Classify every stale→fresh delta at its first causal stage and refuse unsupported hardware authority.
- Run focused validation, provider-free evals, KG fixtures, emitted-ISF FSMGen strict checks, doctrines, mdBook,
  project-path/locality checks, and broader CI proportional to any code change.
- Update root, active part, index/manifest/contract, roadmap, current status, live docs/book, facts, and memory;
  commit before deleting authenticated rollback evidence or moving to refresh #50.

### Decisions and incidents

- `2026-08-10`: selection uses the existing smallest-retained-source rule; no vendor or document exception is
  introduced. The GIC overview wins at 430 elements, ahead of the 527-element OpenCAPI Data Link Layer document.
- `2026-08-10`: an initial read-only candidate census mistakenly wrote directory names to
  `/tmp/specforge-current-source-keys.txt`. It contained no source content, was deleted immediately, and an exact
  absence check passed. All subsequent census work is in-memory; every task-owned workspace/output remains on the
  repository volume. This incident does not authorize `/tmp` for later workflow steps.

### Verification log

| Date | Boundary | Result |
| --- | --- | --- |
| `2026-08-10` | ownership selection | eight candidates ranked; same-device source/hash and seven-file stale chain authenticated; normalized bundle absent; no generated mutation |
| `2026-08-10` | prospective ownership topology | focused task-evidence, catalog, Knowledge Map, 720-file live-document gate, mdBook test/build, and diff checks pass |

### Commit log

| Unit | Durable evidence |
| --- | --- |
| `CORPUS-COVERAGE.2.49` | `CORPUS-COVERAGE.2.49 — own GIC overview refresh` |
