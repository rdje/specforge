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

> Superseded in part by `CORPUS-COVERAGE.4.0` (`2026-08-11`): the completion count, stage census, retention, and
> emitted-target figures above all hold, but "51 of 56 … five real documents remaining" is short by one. The
> derived cohort is 57 = 51 refreshed + six remaining; the sixth is
> `nvme_base_specification_2_0a_2021_07_26`, which a hand-carried decrement had dropped from the queue. This
> refresh's own selection is unaffected — NVMe is the largest of the six.

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

## CORPUS-COVERAGE.2.52

- Status: `done` (`2026-08-11`, DATA/DOC; one child tree opened, see below)
- Goal: re-ingest the OpenCAPI 25 Gbps PHY Mechanical Specification with the current release, then rebuild and
  validate EvidenceIR → SemanticIR → IntentIR → ISF without promoting connector-mechanical prose, glossary
  acronyms, drawing callouts, or section headings into unsupported signals, interfaces, or executable behavior.
- Document key: `opencapi_25gbps_phy_mechanical_spec_v10`
- Source: `.cache/local-references/chipdoc/cxl/opencapi/current/OpenCAPI-25Gbps_PHY_mechanical_spec_v10.pdf`
- Children: none unless the fresh result exposes a generic defect that cannot safely remain inside this refresh.

### Part boundary

This refresh appends to `refreshes-51-56` rather than opening a new part. The part holds the `.2.51` record at
206 lines against a 640-line health target — 32.2%, with the 80% warning at 512 and the 90% rollover at 576. A
refresh has cost 134 lines (`.2.49`), 295 lines (`.2.50` including its `.2.50a` child), and 195 lines (`.2.51`),
so even the largest observed cost lands at 78.3% and the write is guaranteed to finish below the rollover. This
is the measurement `.2.51` made before splitting; here it says append.

### Selection and source authority

The frontier is the six documents `CORPUS-COVERAGE.4.0` derived and `CORPUS-FRONTIER` now gates. The
smallest-retained-source policy selects this specification at 760 elements; every alternative is larger:

| Rank | Document key | Elements | Pages | Source bytes |
| --- | --- | ---: | ---: | ---: |
| 1 | `opencapi_25gbps_phy_mechanical_spec_v10` | 760 | 34 | 4,494,801 |
| 2 | `opencapi_3_0_transaction_layer_28jan2020` | 774 | 121 | 712,534 |
| 3 | `opencapi_3_1_transaction_layer_28jan2020` | 870 | 137 | 870,740 |
| 4 | `lpc_memory_agent_reference_design_guide_17jul2020` | 891 | 59 | 1,356,427 |
| 5 | `den0034_a_2013_09_13_debug_and_trace_configuration_and_usage_models` | 936 | 48 | 946,338 |
| 6 | `nvme_base_specification_2_0a_2021_07_26` | 4,577 | 454 | 5,154,704 |

Rank 6 is the document `.4.0` restored; it is last by the ordinary policy, so the correction changes the tail
rather than this pick. Every row is measured from its own persisted SourceIR `document_profile`, and each of the
six independently satisfies the gated unrefreshed test: no retained normalized bundle, and a persisted source
path still rooted at the retired boot volume.

The source resolves through the owner-authorized SSD-local `chipdoc` symlink. Source and repository share device
`16777240`; the PDF is 4,494,801 bytes at SHA-256
`0621543ade2c1f6369186a7f4de1cb1262e3097ca0e5f796bfc496c260ec7bf3`. That byte count equals the `source.size_bytes`
the stale SourceIR recorded, so the source content is unchanged since the stale ingest and any downstream
movement in this refresh is attributable to code rather than content — the `.2.51` attribution shape.

### Exact stale-chain boundary

The normalized bundle is absent and no `.isf` is emitted. The current six-file chain totals 1,151,838 bytes:

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| SourceIR | 295,886 | `ef115b0312842944fcac88a387301e06d29b5f920ce3077703ff16d2dcbcafb1` |
| EvidenceIR | 651,994 | `348487711e15aada7de88d991aa97ca8014ba1532fff76b6f2450d04e7c4e498` |
| Evidence validation | 11,897 | `2f65ce1682cc368b10783aa84caed448e3c9fd8b779cf88a6deb41a5a4d25370` |
| SemanticIR | 85,495 | `e633ed790874af6fe2c5609c0dd7566e18a2d66b985c07b30a80f952d310b058` |
| IntentIR | 105,093 | `e1c80f1c05f55656b5547e2ce962aa3617c40dc45ad6ad39ad0af286570a432a` |
| Adapter manifest | 1,473 | `05b94c3c380691f7cd2aada0e5e4e271b852eab8e5d65480872b813a22b944f6` |

Stale SourceIR contains 34 pages / 32 visuals / 11 tables / 61 sections / 760 elements and still persists the
retired boot-volume source path. EvidenceIR has 807 statements / 805 spans / 297 links / 61 section anchors /
one actor-signal relation / two conditionals, and zero registers, signal constraints, timings, or polarities.
SemanticIR has seven actors / two interfaces / one relation / 65 invariants / seven contracts / two assertions /
46 decomposition candidates / five abstractions, and zero phases, gates, symbol definitions, or timings.
IntentIR has four actors / two interfaces / seven behaviors / 66 constraints / 65 temporal invariants / five
assumptions, and zero transactions, registers, or timings.

The stale typed hardware surface is exactly two one-bit signals, and both are the acronym class the sibling
OpenCAPI physical-link refreshes retired. `interface_explicit_document_interface` carries `IS` and
`interface_explicit_interface_section_0039_3_5_mezzanine_sideband_signals` carries `OD`; the adapter builds actor
`x_connector` with `(output IS (width 1))` and `(output OD (width 1))` plus a synthetic clock, reset, and
watchdog, then blocks with `is_renderable: false` on `no behavioral content (temporal rules, conditional rules,
signal constraints, or control blocks)`, emitting no target and no residual. Three semantic actors — `receiver`,
`source`, `transmitter` — are already dropped by the time IntentIR is built.

The release binary is the current `EXTRACTION-QUALITY-GAUGE.3h` build at SHA-256
`7c8c68ef47ec5fe9a995b43d5269044f7037e67540267033fe3ffbb781747ce4`; the latest Rust authority is commit
`c489050015a182c8c42ef88bc01dc00ff0df58ce`.

### Expectation to test, not assume

Refreshes `.2.44`, `.2.45`, and `.2.48` all took OpenCAPI physical-link specifications and all removed false
acronym interfaces under current signal authority. This document is the mechanical sibling, so the plausible
result is that `IS` and `OD` fail the declared-signal gate and the adapter blocks on no signals rather than on no
behavior. That is a hypothesis the fresh chain must confirm or refute from its own evidence; it is recorded here
so a confirming result is not mistaken for a measurement, and so a *diverging* result is recognised as the
interesting one.

### Current result

Two guarded CPU ingests and two full cascades reproduce the document byte-identically at every stage. The fresh
SourceIR holds the stale structure exactly — 34 pages / 32 visuals / 11 tables / 21 figures / 61 sections / 760
elements — and differs from the stale artifact in exactly two values, `requested_path` and `canonical_path`,
which move from the retired boot-volume root to the same-SSD one. A whole-tree leaf diff finds no third
difference. The normalized bundle is restored at 105 files / 35,661,265 bytes, digest
`9866810fac9296cdec1cb5f53790bbfff1202218e9bbfa8e7ee2684620b6c543` over sorted repository-relative path and
per-file SHA-256.

Because the source content is unchanged, the whole downstream delta is a pure code delta accumulated since this
document's last EvidenceIR build. EvidenceIR loses exactly two `source_fact` statements — `Signal OD is width 1.`
(`statement_0807`) and `Signal IS is output.` (`statement_0808`) — plus the single actor-signal relation
`X CONNECTOR|Drives|IS` and its one `fact_provenance` row. Nothing else moves: 805 spans, 297 links, 61 anchors,
32 visual records, two conditional rules, and one protocol actor are unchanged, no statement is added, and no
statement changes in place.

Both retired tokens are connector-mechanical prose, confirmed against the document's own normalized text:

- `OD` is the electrical drive-type acronym *open-drain*, qualifying the real signal `PWR_BRAKE_N` —
  "`1.8 V level signal only (OD, pull up is on motherboard)`" (normalized line 1284). The parenthetical
  single-wire rule at `crates/specforge/src/ir/evidence.rs:8964-9007` requires the noun-phrase **head**
  immediately before the abbreviation to be a wire noun; that head is `only`, so the candidate is refused. The
  superseded window-style rule saw `signal` earlier in the same phrase and admitted it.
- `IS` is the English copula — "`The X1 connector size provides full power and is most universal …`"
  (normalized line 1319) — which also produced the false actor `X connector`.

The removal order is causal, not coincidental. `Signal IS is output.` is synthesized only from a `Drives` triple
(`evidence.rs:3992-4028`), and `extract_actor_signal_relations` returns immediately on an empty signal catalog
(`evidence.rs:3733-3735`). Refusing the one parenthetical candidate therefore empties the catalog, and the empty
catalog structurally forecloses the relation and its synthesized direction declaration. The stale binary's own
admission path for `IS` is not reconstructable from the current tree and is not claimed here; the *refusal* is
fully grounded in the document's own sentence.

SemanticIR drops `actor_x_connector` (actors 7 → 6), both interfaces, its one actor port, its one relation, and
its one connectivity record. IntentIR follows exactly (actors 4 → 3, interfaces 2 → 0, drive relation 1 → 0), and
its identity summary tracks the counts. The adapter's blocking reason **moves**: `x_connector` with
`(output IS (width 1))` / `(output OD (width 1))` blocked on `no behavioral content`; `device` with no interface
now blocks on `no signals declared in interface`, signals 2 → 0. Still no emitted target and no residual, so the
44 emitted ISFs are untouched. This is the `.2.44`/`.2.45`/`.2.48` acronym outcome the ownership record predicted,
reached through a gate those refreshes did not exercise.

Validation calls the result honest rather than thin. EvidenceIR classifies the document
`document_intent_category: physical-link` — category 5, whose rationale states that a physical/electrical/link
layer is "behaviorally near-empty by nature — a thin `.isf` is correct, not a gap" — so zero signals, an empty
interface set, and a blocked adapter are the correct answer for this source. The standing residuals are the 32
visual assets with no VLM observation, 20 partially structured normative statements, and three vague-language
statements; the structural `document_class: guide` call keeps its honest under-extracted-spec warning, because
the front matter self-declares a specification and the document is image-heavy.

One delta is a **regression, not a gain**, and it is why this refresh opened a child tree. SemanticIR and
IntentIR conditional rules move 0 → 2. Replaying the semantic stage from the *stale* EvidenceIR with the current
binary reproduces the stale result exactly (0 rules / 2 interfaces / 7 actors), which proves the movement is
input-driven rather than a code delta: emptying the declared-signal set flips the guard at
`crates/specforge/src/ir/semantic.rs:293`, which disables the grounding filter whenever a document declares no
signals. The two promoted rules are ungrounded — `consequent_signal` `PWR` (a prefix of `PWR_GOOD`, action
`(see source_text)`) and `OPEN` (a prefix of `OPEN_CAPI`, action `must be taken`, captured from the idiom "Care
must be taken that…"). A census over all 78 persisted SemanticIR artifacts finds 33 empty-catalog documents, 29
of which carry 1,423 unfiltered conditional rules and 100 unfiltered signal constraints. No such record reaches
an emitted target — all 44 come from populated-catalog documents — so the defect is pre-existing, corpus-wide,
and off the product boundary. `SEMANTIC-EMPTY-CATALOG-FILTER` owns it; repairing it inside a data refresh is
refused for the reason `.2.51` refused the `SEC_SID` lever.

The final validated artifacts reproduce twice, at 1,197,052 bytes across nine files:

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| SourceIR | 295,876 | `dbf9afd22200bc7ba5d8b7a978248bc06afe4f1e622b38a49a667593f75ba062` |
| EvidenceIR | 658,989 | `bf94399b9bd5d32f013bd65feeaa9a95446ce85f25926c20092d899e3b74b152` |
| Evidence validation | 13,840 | `3864fb68d08a0ef0018ecdb3282baf74798e1f91b4fc09014c9d9d295508c433` |
| SemanticIR | 93,198 | `29f799a336c2337152fbfa39d7772d479c2e382c2c0585b7d355678f8a0debfb` |
| Semantic validation | 7,131 | `d85307ece02eafb56ab4dcde13cba67854ab97e383377b6dc37a154a899a45f6` |
| IntentIR | 114,325 | `c58c9166446f52fc6444960138a2e0d42513fea1e520eb54cc71b4977f4f10ea` |
| Intent validation | 8,557 | `067925259fc50dfd1b4835437c32f0f57442a30a87d48866b7c7e861d4a4271c` |
| Adapter manifest | 3,372 | `35b443cd95178cac066f8c7b400ce87293b3e4a8a4bd0a5b0482657e7cced5df` |
| Adapter validation | 1,764 | `1d6b42f1fac4733f3dea84acc7bc7a3bb68aa8240af82414c15ae46581557977` |

The corpus is 52 of 57 refreshes complete with five real documents remaining, at 78 SourceIR / 24 normalized / 78
EvidenceIR / 78 SemanticIR / 78 IntentIR / 78 adapters and 44 emitted targets. Retention grows by exactly one:
`doctrine/chain_currency/retained_bundles.json` declares 24 keys, and `CORPUS-FRONTIER` re-derives
57 = 52 + 5 rather than accepting the number.

### Acceptance

- [x] Authenticate rollback copies of every stale chain artifact inside a repository-derived same-volume
  workspace.
- [x] Run guarded CPU ingest from the directly resolved caller-authorized same-SSD input, with the 85% memory
  abort ceiling and no off-volume project temp/cache/output.
- [x] Rebuild and validate the complete current-binary chain; rerun to establish deterministic hashes.
- [x] Classify every stale→fresh delta at its first causal stage and refuse unsupported hardware authority.
- [x] Declare the retained normalized bundle in `doctrine/chain_currency/retained_bundles.json` and prove
  `CHAIN-CURRENCY` green on both its currency and retention legs.
- [x] Move `doctrine/corpus_frontier/census.json` and the root's stated counts to 52/57 with five remaining in
  the same transaction, so `CORPUS-FRONTIER` stays green rather than failing closed.
- [x] Run focused validation, provider-free evals, KG fixtures, emitted-ISF FSMGen strict checks, doctrines,
  mdBook, project-path/locality checks, and broader CI proportional to any code change.
- [x] Update root, active part, index/manifest/contract, roadmap, current status, live docs/book, facts, and
  memory; commit before deleting authenticated rollback evidence or moving to refresh #53.

### Decisions and incidents

- `2026-08-11`: the recorded hypothesis is confirmed but not by the predicted route. `.2.44`/`.2.45`/`.2.48`
  retired acronym interfaces that never had wire authority; here the two candidates fail *different* gates —
  `OD` at the parenthetical head rule, `IS` by never surviving an empty catalog — and the adapter's block moves
  from `no behavioral content` to `no signals declared in interface` exactly as predicted. Recorded as a
  confirming instance reached through an unexercised gate, not as a re-measurement of the sibling refreshes.
- `2026-08-11`: the `OD` case is worth keeping because it is *not* the ordinary "acronym was never a signal"
  story. `OD` is a genuine electrical term, and the sentence it sits in genuinely describes a wire — but the
  wire is `PWR_BRAKE_N`, and `OD` is its drive type. The head rule gets this right for a structural reason: a
  drive-type qualifier never occupies the head slot of the noun phrase it qualifies. That is the general form of
  the fix, and it needs no vendor or document exception.
- `2026-08-11`: no new Knowledge Map card is written for the retirement. `[[parenthetical-data-head-requires-wire-qualifier]]`
  already owns the parenthetical single-wire authority rule and its measured corpus effect; this refresh is a
  confirming instance on a document that had never been rebuilt under the head rule, not a new durable fact.
- `2026-08-11`: the conditional-rule regression is **not** repaired here. It moves 29 documents and 1,423
  records, so it needs its own leaf, its own corpus-wide old-versus-new replay, and its own before/after evals —
  the same reasoning `.2.51` applied to the `SEC_SID` header-sourcing lever. `SEMANTIC-EMPTY-CATALOG-FILTER`
  owns it with the reproduction, the census, and three candidate rules. This refresh records its own two
  promoted rules as the honest current-binary result rather than hand-suppressing them, because suppressing
  them would have hidden the defect inside the artifact that exposed it.
- `2026-08-11`: the stale binary's admission path for `IS` is deliberately left unclaimed. The binary that
  produced the stale EvidenceIR is superseded and the current tree cannot mint `IS` at all, so any reconstructed
  pass sequence would be inference, not measurement. The refusal is what this refresh proves, and it is proved
  from the document's own sentence.

### Verification log

| Date | Boundary | Result |
| --- | --- | --- |
| `2026-08-11` | ownership selection | six gated candidates re-measured from their own SourceIR profiles; same-device source authenticated at 4,494,801 bytes / `0621543a…7bf3`, equal to the stale `source.size_bytes`; six-file 1,151,838-byte stale chain pinned; normalized bundle absent; no generated artifact mutated |
| `2026-08-11` | part boundary | `refreshes-51-56` at 206/640 lines (32.2%); largest observed refresh cost 295 lines lands at 78.3%, below the 90% rollover, so this refresh appends |
| `2026-08-11` | rollback and locality | six rollback files / 1,151,838 bytes match every pinned stale hash on device `16777240`; workspace is repository-derived under `.project-data/tmp/`; ingest ran `DOCLING_DEVICE=cpu` with `SPECFORGE_INGEST_RAM_ABORT_PERCENT=85` and the host held at 39% used |
| `2026-08-11` | deterministic refresh | two guarded ingests and two cascades reproduce all nine artifact/report hashes and the 105-file / 35,661,265-byte bundle byte-identically; four stages validate; no emitted target |
| `2026-08-11` | delta attribution | SourceIR differs from stale in exactly two path leaves and no third; EvidenceIR loses exactly `statement_0807`/`statement_0808`, one relation, and one provenance row, with nothing added or changed in place; Semantic/Intent drop `actor_x_connector`, both interfaces, the port, the relation, and the connectivity record; adapter `signal_count` 2 → 0 with its blocking reason moved to `no signals declared in interface` |
| `2026-08-11` | regression found | semantic replay from the *stale* EvidenceIR with the current binary yields 0 rules / 2 interfaces / 7 actors, proving the 0 → 2 conditional-rule movement is input-driven; census over 78 SemanticIR artifacts finds 33 empty-catalog documents and 29 carrying 1,423 unfiltered rules / 100 unfiltered constraints; zero reach an emitted target |
| `2026-08-11` | chain currency | `check_chain_currency.sh` exits 0: evidence 24/24, semantic 78/78, intent 78/78, isf-adapter 78/78 current, 78 emitted `.isf` bodies checked, retention exactly the 24 declared bundles |
| `2026-08-11` | corpus frontier | `check_corpus_frontier.sh` exits 0: self-test 10/10; 57 cohort = 52 refreshed + 5 remaining; declaration, retention, and the root frontier agree |
| `2026-08-11` | persisted paths | 2,669 persisted JSON artifacts / 359,231 path values / 104 authorized external absolute paths / zero repository-owned absolute paths |
| `2026-08-11` | no regression | 44/44 emitted ISFs pass FSMGen `--strict --check --json`; `kg-bench` 156/156; nine provider-free WIRE/I2C/SWD evals at baseline with every filtered surface at 1.000 and only the known SWD `CSYSPWRUPACK` residual missing; the two register datasets hold their known baseline |
| `2026-08-11` | locality incident | the first gate run failed closed on `PROJECT-DATA-LOCALITY`: this slice's own `*.log` files sat at the top level of `.project-data/tmp`, which the residue rule forbids. Logs moved one directory down and the check returns PASS — the gate caught the operator, which is the point |
| `2026-08-11` | full gate | `scripts/run_ci.sh` exits 0 under `set -euo pipefail` — all eight doctrines including CI-tier `CHAIN-CURRENCY`, `cargo fmt --all --check`, warning-deny clippy, warning-deny tests, rustdoc, mdBook examples and build, and the closing project-data residue recheck |

### Commit log

| Unit | Durable evidence |
| --- | --- |
| `CORPUS-COVERAGE.2.52` ownership | `CORPUS-COVERAGE.2.52 — own the OpenCAPI 25 Gbps PHY mechanical refresh` |
| `CORPUS-COVERAGE.2.52` completion | `CORPUS-COVERAGE.2.52 — refresh the OpenCAPI 25 Gbps PHY mechanical spec and retire its acronym signals` |
