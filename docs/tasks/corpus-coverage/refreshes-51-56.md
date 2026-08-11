# CORPUS-COVERAGE — refreshes 51–56

- Part ID: `refreshes-51-56`
- State: `active`

## CORPUS-COVERAGE.2.51

- Status: `done` (`2026-08-11`, DATA/DOC; no child required)
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

### Current result

Two guarded CPU ingests and two full cascades reproduce the document byte-identically at every stage. The fresh
SourceIR holds the stale structure exactly — 52 pages / 29 visuals / three tables / 87 sections / 600 elements —
and differs from the stale artifact in exactly two values, `requested_path` and `canonical_path`, which move from
the retired boot-volume root to the same-SSD one. The normalized bundle is restored at 138 files / 39,252,339
bytes, digest `16b1d5b4462ced873634881f46ec6664dd8cfd418bd9a36c590cbde9bd169da8` over sorted repository-relative
path and per-file SHA-256.

Because the source content is unchanged, the whole downstream delta is a pure code delta accumulated since this
document's last EvidenceIR build, and it is exactly one thing. EvidenceIR loses three `source_fact` statements —
`Enum TABLE THE_STREAM_IS_A_{NON_SECURE,SECURE,REALM}_STREAM_AND_USES_… = 0|1|2.` — and nothing else moves: 523
spans, 100 links, 87 anchors, 29 visual records, seven conditional rules, 27 normative statements, six derived
rules, four timing constraints, and two explicit abstractions are unchanged, and no statement is added. Those
three statements were the only members of `symbol_0_table`, so SemanticIR and IntentIR each drop
`symbol_definitions` 1 → 0 and are otherwise byte-identical; IntentIR's identity summary follows the count. The
adapter keeps `lowering_status: blocked` on `no signals declared in interface` and emits no target, but its
rendered enum surface falls 1 → 0.

The retired enum is the generic-`TABLE` mega-enum class. `Table 3-1: Stream Security determination` is an
`encoding` table whose header is `SEC_SID value | Description`; the stale build could not match a declared
signal, fell back to the first caption token passing `is_hardware_signal_token`, and named the enum after the
word `Table`, with the three whole description sentences as uppercased members. `KG-ISF-COMPLETENESS.5.i` closed
that fallback at `crates/specforge/src/ir/evidence.rs:4715-4733`: the candidate survives only when independently
evidenced as a declared signal or a column-header reference token of that same table. `TABLE` is neither, so
`derive_encoding_enum_name` returns `None`, both call sites `continue`, and no enum is minted. The retirement is
doubly determined — `.5.ii`'s per-member sentence-spine gate would also have emptied it, since every member name
carries `THE`/`IS`/`A` — and this refresh is the first time either gate has been applied to this document.

Validation calls the result honest rather than thin. EvidenceIR classifies the document `guide` at
`document_intent_category: methodology-guide` with **high** confidence, from the document's own front-matter
self-declaration — category 6, an ISF non-target — so zero signals, zero registers, an empty interface set, and a
blocked adapter are the correct answer for this source, not an extraction miss. Thirteen semantic actors, 76
invariants, 17 contracts, five assertions, 59 decomposition candidates, five intent actors, 17 behaviors, 77
constraints, 76 temporal invariants, and three assumptions all survive intact; the completeness gauge reports
"not applicable — guide", and the standing residuals it does raise are the 27 partially structured normative
statements, the 29 visual assets with no VLM observation, and eight vague-language statements.

The final validated artifacts reproduce twice, at 1,128,958 bytes across nine files:

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| SourceIR | 266,822 | `5f71d69e2532b7fa6616eb548f542f20f5134eda1a133eeaf3c47d66ea545b11` |
| EvidenceIR | 495,154 | `d6a80088c3923c73dd571f9edd766236d6c239d9e5fc269bc2b8a73d7f5839d1` |
| Evidence validation | 13,803 | `d77a161c9df0de27bc785f0c4292ec3e51bdfaa8f438d06e1c28b4fc69795c6d` |
| SemanticIR | 134,350 | `784adc95bbff26741076fa929c15609ebb5cd79854f71f88b804e618afa4ff47` |
| Semantic validation | 7,392 | `c332b7011c0716e80c055a9112abb32546b2d8522a47da57b9fcfbf7c94ade3d` |
| IntentIR | 197,391 | `82d7c70270e2217e2e39f70250cd367d7ffba6f303b38316a5c496314b318b63` |
| Intent validation | 8,818 | `1327d6fedfcd88258040ccd19932fe878eb2c9fe334f4576ea993671af782141` |
| Adapter manifest | 3,454 | `bb85deea6966b06cb78a2edce1fd0fdd7ea066faf1384724f6cc93f9100103c5` |
| Adapter validation | 1,774 | `0d3d616120832d4792df0f6f6651a6b5025ac6288fcd67b9ff78c507114855bc` |

The corpus is 51 of 56 refreshes complete with five real documents remaining, at 78 SourceIR / 23 normalized / 78
EvidenceIR / 78 SemanticIR / 78 IntentIR / 78 adapters and 44 emitted targets. Retention grows by exactly one:
`doctrine/chain_currency/retained_bundles.json` declares 23 keys.

### Acceptance

- [x] Authenticate rollback copies of every stale chain artifact inside a repository-derived same-volume
  workspace.
- [x] Run guarded CPU ingest from the directly resolved caller-authorized same-SSD input, with the 85% memory
  abort ceiling and no off-volume project temp/cache/output.
- [x] Rebuild and validate the complete current-binary chain; rerun to establish deterministic hashes where
  required.
- [x] Classify every stale→fresh delta at its first causal stage and refuse unsupported hardware authority.
- [x] Declare the retained normalized bundle in `doctrine/chain_currency/retained_bundles.json` and prove
  `CHAIN-CURRENCY` green on both its currency and retention legs.
- [x] Run focused validation, provider-free evals, KG fixtures, emitted-ISF FSMGen strict checks, doctrines,
  mdBook, project-path/locality checks, and broader CI proportional to any code change.
- [x] Update root, active part, index/manifest/contract, roadmap, current status, live docs/book, facts, and
  memory; commit before deleting authenticated rollback evidence or moving to refresh #52.

### Decisions and incidents

- `2026-08-11`: selection re-measures all six remaining documents from their persisted SourceIR profiles rather
  than inheriting `.2.50`'s ranking, so the smallest-retained-source rule is applied to current evidence. The Arm
  SMMU Software Guide wins at 600 elements, ahead of the 760-element OpenCAPI 25 Gbps PHY mechanical
  specification. No vendor or document exception is introduced.
- `2026-08-11`: the persisted-path contract already proves a repository-looking path may not escape through the
  ignored `chipdoc` symlink. Runtime ingest used the directly resolved caller-authorized same-SSD input and kept
  every project-owned workspace and output repository-relative; the failed symlink ingest was not repeated.
- `2026-08-11`: this is the cleanest attribution any refresh in the program has had, and the reason is structural
  rather than lucky. The source structure was already current, so the SourceIR comparison isolates the entire
  downstream delta to code, and `CORPUS-CHAIN-CURRENCY.3` had already brought the last three stages to the
  current binary, so the delta is further isolated to the evidence stage alone. No judgement call was needed to
  decide what caused what.
- `2026-08-11`: no new Knowledge Map card is written. `[[generic-enum-conflation]]` already owns this mechanism,
  its fix, and its measured corpus effect; this refresh is a confirming instance on a document that had never
  been rebuilt under the gate, not a new durable fact. Writing a second card would duplicate a card already
  under capacity pressure.
- `2026-08-11`: an observation worth recording but deliberately **not** acted on here. The header of the retired
  table names the real field, `SEC_SID`, so a correctly named `SEC_SID` enum was in principle recoverable — but
  `derive_encoding_enum_name` only ever *validates* a candidate against the column headers, it never *sources*
  one from them; candidates come from the caption or section title alone. Sourcing from the header could recover
  genuine enums corpus-wide. It is not attempted in this slice for two reasons: it is a shared-extractor change
  that would need its own leaf, its own corpus-wide old-versus-new replay, and a before/after WIRE-BASED-100
  eval; and for *this* document the honest answer is already correct, because a self-declared methodology guide
  is a category-6 ISF non-target where minting an enum buys no lowering value. Recorded here so the lever is not
  lost.

### Verification log

| Date | Boundary | Result |
| --- | --- | --- |
| `2026-08-11` | ownership selection | six candidates re-measured and ranked from their own SourceIR profiles; same-device source and SHA-256 authenticated; six-file 1,087,030-byte stale chain pinned; normalized bundle absent; no generated artifact mutated |
| `2026-08-11` | part boundary | `refreshes-49-56` measured at 434/640 lines against refresh costs of 134 and 295 lines; new part opened; the closed part left byte-identical to `c4f03838` |
| `2026-08-11` | rollback and locality | six rollback files / 1,087,030 bytes match every pinned stale hash on device `16777240`; workspace is repository-derived under `.project-data/tmp/` |
| `2026-08-11` | deterministic refresh | two guarded ingests and two cascades reproduce all nine artifact/report hashes and the 138-file / 39,252,339-byte bundle byte-identically; four stages validate; no emitted target |
| `2026-08-11` | delta attribution | SourceIR differs from stale in exactly two path values; EvidenceIR loses exactly three `Enum TABLE …` statements and nothing else; Semantic/Intent differ only by `symbol_definitions` 1 → 0 and the identity summary; adapter `enum_count` 1 → 0 with the same blocking reason |
| `2026-08-11` | chain currency | `check_chain_currency.sh` exits 0: evidence 23/23, semantic 78/78, intent 78/78, isf-adapter 78/78 current, 78 emitted `.isf` bodies checked, retention exactly the 23 declared bundles |
| `2026-08-11` | no regression | 44/44 emitted ISFs pass FSMGen `--strict --check --json`; `kg-bench` 156/156; nine provider-free evals at baseline with every WIRE-BASED-100 filtered surface at 1.000 and only the known SWD `CSYSPWRUPACK` residual missing; persisted paths 2,627 artifacts / 359,062 values / zero absolute repository paths |
| `2026-08-11` | full gate | `scripts/run_ci.sh` exits 0 under `set -euo pipefail` — all seven doctrines including CI-tier `CHAIN-CURRENCY`, `cargo fmt --all --check`, warning-deny clippy, 1,807 tests / five ignored, rustdoc, mdBook examples and build, and the closing project-data residue recheck |

### Commit log

| Unit | Durable evidence |
| --- | --- |
| `CORPUS-COVERAGE.2.51` ownership | `83b81f25` — `CORPUS-COVERAGE.2.51 — own Arm SMMU software-guide refresh` |
| `CORPUS-COVERAGE.2.51` completion | `CORPUS-COVERAGE.2.51 — refresh the Arm SMMU guide and retire its caption-named enum` |
