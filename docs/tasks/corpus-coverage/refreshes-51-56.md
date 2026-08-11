# CORPUS-COVERAGE — refreshes 51–56

- Part ID: `refreshes-51-56`
- State: `active`

## CORPUS-COVERAGE.2.51

- Status: `active` (`2026-08-11`, DATA/DOC; ownership and exact stale boundary only)
- Goal: re-ingest the Arm SMMU Software Guide with the current release, then rebuild and validate EvidenceIR →
  SemanticIR → IntentIR → ISF without promoting programming-guide prose, worked examples, code listings, table
  headings, or diagram labels into unsupported signals, registers, interfaces, or executable behavior.
- Document key: `109242_0100_01_2023_09_04_arm_smmu_software_guide`
- Source: `.cache/local-references/chipdoc/arm/system-ip/smmu/current/109242_0100_01_2023-09-04_Arm_SMMU_Software_Guide.pdf`
- Children: none unless the fresh result exposes a generic defect that cannot safely remain inside this refresh.

### Part boundary

This refresh opens a new part instead of appending to `refreshes-49-56`. That part is complete at the `.2.50a`
boundary at 434 lines against a 640-line health target, and a refresh has cost 134 lines (`.2.49`) or 295 lines
(`.2.50` with its `.2.50a` repair child). Appending would land between 89% and 114% of the target, so the write
could not be guaranteed to finish under the part's 90% rollover point — and a rollover forced open mid-refresh is
exactly the failure `CHANGES-LEDGER-ROLLOVER.1` recorded. The root Slice Transaction requires the split at a
refresh boundary before that write, and this is that boundary.

`refreshes-49-56` therefore stays byte-identical to commit `c4f03838` and keeps contract state `active` even
though it is closed to further product writes. It cannot say more: `legacy` is reserved for parts owning an exact
capsule source region, and `sealed` is unreachable by any compliant commit sequence.
`TASK-PART-SEAL-REACHABILITY` owns that defect with its measured reproduction; it blocks nothing here.

### Selection and source authority

The durable frontier contains six real documents. The established smallest-retained-source policy selects this
guide at 600 elements; all alternatives are larger:

| Rank | Document key | Elements | Pages | Source bytes |
| --- | --- | ---: | ---: | ---: |
| 1 | `109242_0100_01_2023_09_04_arm_smmu_software_guide` | 600 | 52 | 878,792 |
| 2 | `opencapi_25gbps_phy_mechanical_spec_v10` | 760 | 34 | 4,494,801 |
| 3 | `opencapi_3_0_transaction_layer_28jan2020` | 774 | 121 | 712,534 |
| 4 | `opencapi_3_1_transaction_layer_28jan2020` | 870 | 137 | 870,740 |
| 5 | `lpc_memory_agent_reference_design_guide_17jul2020` | 891 | 59 | 1,356,427 |
| 6 | `den0034_a_2013_09_13_debug_and_trace_configuration_and_usage_models` | 936 | 48 | 946,338 |

Every row is re-measured from its own persisted SourceIR `document_profile` rather than copied from `.2.50`'s
table, and all six independently confirm the two properties that make them unrefreshed: no normalized bundle, and
a persisted source path still rooted at the retired boot volume. The source resolves through the owner-authorized
SSD-local `chipdoc` symlink; source and repository share device `16777240`, and the PDF is 878,792 bytes at
SHA-256 `21cd873e75f01fb186691a10cf09b028a586f3ec002ada227a4fe3b1ef5a7f14`.

### Exact stale-chain boundary

The normalized bundle is absent and no `.isf` is emitted. The current six-file chain totals 1,087,030 bytes:

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| SourceIR | 266,832 | `c36323df806e8e675cd2e815bdec7d27285d8bc4caa29889884bb9131ec3c79d` |
| EvidenceIR | 489,740 | `ed7e297bd8716c10c0ca8317c934aeaba754c4afad6d55ed4019d694ab5c23cd` |
| Evidence validation | 13,330 | `6c84a5cc9d566d861617d1ec1e818a4ec5f9a451d7d4d3d01a4b673589b7f2f2` |
| SemanticIR | 127,037 | `53131104f725f6888bb2131117c872928c65a0d180e0c58123d3c19ccb7727fc` |
| IntentIR | 188,396 | `057dba66fe6892fbadbed93311fdef8db6869ac3454203290896964385a4874e` |
| Adapter manifest | 1,695 | `68abde0aed8a6ac6e1914ca312c9b07fee441687f0f36a20369fb74e73e403fa` |

Unlike refreshes #49 and #50, only the SourceIR and EvidenceIR are stale here. `CORPUS-CHAIN-CURRENCY.3` rebuilt
this document's SemanticIR, IntentIR, and adapter from its unchanged persisted EvidenceIR, so those three already
reproduce under the current binary and every downstream movement in this refresh must be attributed to the fresh
ingest and evidence rebuild alone.

Stale SourceIR contains 52 pages / 29 visuals / three tables / 87 sections / 600 elements and still persists the
retired boot-volume source path. EvidenceIR has 526 statements / 523 spans / 100 links / seven conditionals / 87
section anchors / 29 visual records, and zero relations, registers, signal constraints, polarities, or timings.
SemanticIR has 13 actors / 76 invariants / 17 contracts / five assertions / seven conditional rules / 59
decomposition candidates / two abstractions / one symbol definition, and zero interfaces, ports, relations,
phases, or gates. IntentIR has five actors / 17 behaviors / 77 constraints / 76 temporal invariants / three
assumptions / seven conditional rules / one symbol definition, and zero interfaces, transactions, or timings. The
adapter already blocks on `no signals declared in interface` and emits no target, yet still renders a single
two-bit `TABLE` enum whose grounding the fresh chain must establish or retire.

The release binary is the current `EXTRACTION-QUALITY-GAUGE.3h` build at SHA-256
`7c8c68ef47ec5fe9a995b43d5269044f7037e67540267033fe3ffbb781747ce4`; the latest Rust authority is commit
`c489050015a182c8c42ef88bc01dc00ff0df58ce`.

### Acceptance

- Authenticate rollback copies of every stale chain artifact inside a repository-derived same-volume workspace.
- Run guarded CPU ingest from the directly resolved caller-authorized same-SSD input, with the 85% memory abort
  ceiling and no off-volume project temp/cache/output.
- Rebuild and validate the complete current-binary chain; rerun to establish deterministic hashes where required.
- Classify every stale→fresh delta at its first causal stage and refuse unsupported hardware authority.
- Declare the retained normalized bundle in `doctrine/chain_currency/retained_bundles.json` and prove
  `CHAIN-CURRENCY` green on both its currency and retention legs.
- Run focused validation, provider-free evals, KG fixtures, emitted-ISF FSMGen strict checks, doctrines, mdBook,
  project-path/locality checks, and broader CI proportional to any code change.
- Update root, active part, index/manifest/contract, roadmap, current status, live docs/book, facts, and memory;
  commit before deleting authenticated rollback evidence or moving to refresh #52.

### Decisions and incidents

- `2026-08-11`: selection re-measures all six remaining documents from their persisted SourceIR profiles rather
  than inheriting `.2.50`'s ranking, so the smallest-retained-source rule is applied to current evidence. The Arm
  SMMU Software Guide wins at 600 elements, ahead of the 760-element OpenCAPI 25 Gbps PHY mechanical
  specification. No vendor or document exception is introduced.
- `2026-08-11`: the persisted-path contract already proves a repository-looking path may not escape through the
  ignored `chipdoc` symlink. Runtime ingest will use the directly resolved caller-authorized same-SSD input and
  keep every project-owned workspace and output repository-relative; the failed symlink ingest is not repeated.

### Verification log

| Date | Boundary | Result |
| --- | --- | --- |
| `2026-08-11` | ownership selection | six candidates re-measured and ranked from their own SourceIR profiles; same-device source and SHA-256 authenticated; six-file 1,087,030-byte stale chain pinned; normalized bundle absent; no generated artifact mutated |
| `2026-08-11` | part boundary | `refreshes-49-56` measured at 434/640 lines against refresh costs of 134 and 295 lines; new part opened; the closed part left byte-identical to `c4f03838` |

### Commit log

| Unit | Durable evidence |
| --- | --- |
| `CORPUS-COVERAGE.2.51` ownership | `CORPUS-COVERAGE.2.51 — own Arm SMMU software-guide refresh` |
