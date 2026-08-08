# RUST_CODEBASE_ANALYSIS
## Purpose
- maintain a live, deep-dive analysis of the Rust codebase
- record the current architecture, risks, subsystem boundaries, and recommended implementation direction
- remain useful even while only the early IR stages are implemented

## Session update (2026-08-08 — SWD interface-edge timing and protocol projection boundary)

- **New EvidenceIR surface:** `InterfaceEdgeTimingRecord` carries actor, data signal, clock signal,
  explicit rising/falling edge, sample/drive-state-change flags, and statement provenance.
  `timing.interface_edge_prose` runs through the common extraction manifest and admits only timing-class
  universal grammar whose data and clock are already declared by the document.
- **New deterministic eval surface:** `EvalTask::InterfaceEdgeTiming` and
  `GoldFact::InterfaceEdgeTimingFact` use the complete semantic tuple as identity. `eval-extraction`
  indexes the EvidenceIR records directly; fresh ADI evidence scores its one B4.3.1 fact at P/R/F1
  1.000 and expands the SWD protocol gold to 29 facts.
- **Architecture boundary:** `serial_frame_fields`, `swd_operations`, `protocol_states`, and
  `interface_edge_timings` are EvidenceIR-only. Neither `SemanticIr`, `IntentIr`, nor `IsfIr` consumes
  them, so extraction scoring does not prove downstream product availability; `.7` owns that projection.
- **New portability risk:** stage builders canonicalize input paths before serializing upstream pointers.
  A local census found 335 generated artifacts retaining the deleted boot-volume root. This violates the
  root-relative persistence contract and needs a dedicated compatibility/migration task before another
  canonical artifact promotion.

## Session update (2026-08-08 — repository-volume runtime boundary; `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.6a`)

- **One runtime root now owns project data.** `crates/specforge/src/project_data.rs` discovers the
  current repository from an explicit process value, current-directory ancestors, or executable
  ancestors; it never persists the build-time `CARGO_MANIFEST_DIR`. Startup prepares repository-local
  temp/cache roots, rejects canonical paths that escape the repository, and on Unix compares device
  ids before command dispatch.
- **Production tempdirs no longer trust ambient `TMPDIR`.** The six runtime sites in doctor, enrich,
  extraction evaluation, KG bench, LLM text transport, and Docling use
  `tempdir_in(.project-data/tmp)`. Child curl/VLM, Docling, and FSMGen boundaries receive explicit
  repository-derived cache/temp values. Cargo's forced relative environment covers the test corpus
  and compiler/build-script processes before application startup.
- **The pinned FSMGen boundary has invocation-scoped cleanup.** Its Perl lowering uses
  `File::Temp::tempfile` without removing the closed `.fsm` path. A long-lived shared `TMPDIR` left
  eight intermediates after the complete Rust suite. The parent now supplies one disposable
  repository-local child tempdir per invocation and drops it after the child exits; focused strict and
  schedule tests leave zero `.fsm` residue, and full CI rechecks residue after every producer.
- **Move-sensitive Python state is explicitly rebuildable.** Exact Docling and evaluation locks plus
  rollback-safe bootstraps replace copied virtual environments whose console shebangs still named the
  old tree. Runtime probes now resolve the SSD environments and local Hugging Face model cache; shared
  Cargo/Rustup and ambiguous shared model caches remain protected external inputs.

## Session update (2026-08-08 — managed corpus-KB review authority and currentness)

- **`corpus-kb` now distinguishes reviewed and ambient validation inputs.** `CorpusKbArgs` adds
  `--validation-snapshot`; the command rejects simultaneous positional reports and snapshot input.
  The reviewed parser consumes only `VALIDATION_SNAPSHOT.md`'s projected-artifact section, validates
  unique complete records/findings, and renders the tracked validation managed block.
- **Currentness remains read-only.** `scripts/check_corpus_kb_currentness.pl` does not invoke the Rust
  writer. It binds the reviewed snapshot contract, 312 Git-indexed KG input files / 156 fixtures, all
  eleven managed Markdown regions, paired JSON, and seven producer regions. The executable `kg-bench`
  run remains the behavior leg; identity is not treated as a replacement for fixture execution.
- **Boundary and risk:** the writer still mutates only corpus-KB managed outputs and does not write
  canonical IR or typed prior memory. Human prefix/suffix identity is independently enforced. The
  refreshed aggregate is 977 lines (81.4% of its governed ceiling), now an explicit `.5i` partition-
  before-rollover obligation rather than a widened limit.
- **Same-volume test compatibility:** the Docling PATH-ordering test now calls an internal diagnosis
  seam with repo-local discovery disabled, preventing repository-local `TMPDIR` from exposing the real
  ancestor venv. Production `inspect_docling_runtime()` still enables discovery. The shared environment
  mutex recovers its guard after poison, avoiding dependent-failure cascades; 38 source tests pass at
  16 threads on the repository volume.

## Session update (2026-06-24 ramp-up currency correction — size + command + IR-module + test inventory; `AUDIT-DOC-RECONCILE.3`)

State verified directly from the working tree at HEAD `c212a6d0` (`origin/main..HEAD` = 63; push
held at the ~200 threshold). This entry refreshes four counts that drifted since the `2026-06-08`
currency correction below, as the `EXTRACTION-QUALITY-GAUGE` / `PDF-VARIANT-DIGESTION` /
`KG-ISF-TRANSACTIONS` / `KG-ISF-COMPLETENESS` / `MEMORY-BOUNDED-INGEST` work landed. Older dated
sections are preserved as historical record and are superseded by this entry where they cite
smaller counts. **No architecture claim is reversed** — `IrStage` is still the same four IR stages
(`SourceIR → EvidenceIR → SemanticIR → IntentIR`), `.isf` via `IsfIr` is still the sole adapter
target (`.fsm`/HDL out of scope, owned by FSMGen downstream), the R16 ContractIR layer is still a
typed layer over the four stages (no fifth stage), and the Ollama + Qwen2.5VL LLM/VLM provider is
still production-default.

- **Whole `crates/specforge/src` = 128,742 lines across 65 `.rs` files** (single workspace crate,
  edition 2024), up from the ≈103,200 cited on `2026-06-08`. Verified:
  `find crates/specforge/src -name '*.rs' -exec cat {} + | wc -l`.
- **Command surface = 28 subcommands** (`crates/specforge/src/cli.rs` `enum Commands`, dispatched in
  `lib.rs`), up from the 25 cited on `2026-06-08` (that entry's own enumeration was itself an
  undercount — it described `recover-register-bits` in prose yet omitted it from the list, so the
  exact per-command delta is not reconstructed here; only the verified live total is stated).
  Verified: `grep -oE 'Commands::[A-Za-z]+' crates/specforge/src/lib.rs | sort -u | wc -l` = 28. The full set:
  inspect · doctor · converge · ingest · evidence · semantic · intent · adapt · enrich · nlp-enrich ·
  extract-contracts · signal-resolve · nli-verify · entity-type · extract-conditions ·
  extract-constraints-llm · eval-extraction · audit-extraction · grits-consensus ·
  recover-register-bits · validate · project-validation · rescan-plan · kg-bench · learn-priors ·
  corpus-cluster · corpus-kb · clean. (`commands/mod.rs` also carries `pub(crate) mod llm_text`, a
  shared text/VLM transport helper — not a subcommand.)
- **IR namespace = 28 `ir/*.rs` modules** (plus `ir/source/docling_backend.rs`), up from 25. Still a
  typed layer over the four IR stages — no sixth stage. Verified: `ls crates/specforge/src/ir/*.rs | wc -l` = 28.
- **`cargo test -p specforge --lib` = 1709 passing, 0 failed, 4 ignored** (the canonical validation
  command; `2026-06-24`, finished in ~6.9s), up from 1435. Any earlier
  `1435`/`1433`/`1427`/`1421`/`1360`/`1014`/`666` counts below are historical.
- **Largest modules (top, by lines):** `ir/evidence.rs` 26,195 · `ir/semantic.rs` 22,604 ·
  `commands/validate.rs` 16,105 · `commands/project_validation.rs` 7,476 · `ir/intent.rs` 5,506 ·
  `ir/isf_ir.rs` 5,022 · `commands/kg_bench.rs` 3,486 · `commands/rescan_plan.rs` 3,108 ·
  `commands/learn_priors.rs` 3,022 · `eval.rs` 2,855 · `ir/source/docling_backend.rs` 2,666 ·
  `ir/prior_memory.rs` 2,578. The two IR builders `evidence.rs` + `semantic.rs` (≈48.8K combined)
  remain the centre of gravity — the table-driven extraction and the actor/contract synthesis.

## Session update (2026-06-22 — DOC-INTENT-TAXONOMY.4a.ii: register bit-fields now lower to ISF field-structured storage)

- **`crates/specforge/src/ir/isf_ir.rs` — the storage emitter gained a field substructure.** `IsfStorageVar` now
  carries `fields: Vec<IsfStorageField>` (new struct: name, msb, lsb, optional access/reset, enum members). The storage
  render emits the nested `(fields (field …))` block when present and the byte-identical opaque `(var …)` line when not.
  The field map is derived by the pure `register_storage_fields(r, var_width, parent_reset)` + `normalize_field_access`,
  sitting alongside the existing `classify_register_reset` / `register_var_width` / `register_field_extent`
  register-lowering helpers and reusing the same u64 tiling bound. This is the Gap-A lowering for
  `DOC-INTENT-TAXONOMY.4a.ii`: 6,570 register bit-fields across 2,531 registers in 24 docs now reach `.isf` (was 0).
- **Admission is structural and fail-closed (ADR 0006).** Located fields only; sanitized-name-collision groups dropped
  (FSMGen fails closed on duplicate field names); residual overlap fails the register's field block closed; access
  normalized to FSMGen's 10-token set (omit when unmapped); field `(reset)` only when the parent reset is composed, as
  the parent value's bit slice; width-fitting enum members. The unlowered remainder is surfaced as the
  `isf_register_fields_not_lowered` residual (`ir/adapters.rs`); nothing is fabricated.
- **Risk picture unchanged.** The change is emitter-only and additive (metadata-only / schedule-safe per FSMGen), so the
  extraction/semantic/intent surfaces and the wire golds are orthogonal by construction (4 wire golds emit 0 fields →
  byte-identical `.isf`). A test-only `run_fsmgen_schedule_json` helper (`ir/mod.rs`) was added beside
  `run_fsmgen_strict_check` to assert the `inferred_storage[].fields[]` round-trip against the real pinned binary.

## Session update (2026-06-21 — ISF-VALUE-WIDTH-EMIT.2: the measured width bug FIXED in the emitter; TREE CLOSED)

- **Code landed (emitter-only, `crates/specforge/src/ir/isf_ir.rs`).** The `.0/.1` design is now implemented and
  verified end-to-end against the real FSMGen `--strict --check`. Both measured root causes are closed:
  1. **Grounded width now recovered across all signal records.** `from_intent_ir` adds an `interface_widths`
     aggregate (single unambiguous `WidthHint::Numeric > 1` across **all** `interfaces[].signal_records`; conflict
     keeps width-1) and the signal-width fallback chains `interface_widths → port_widths (.2a.i) → 1`. A width in
     a NON-FIRST interface record (trace-bus `ATID` width 7, no actor-port) is no longer lost to the first-seen
     dedup → `(output ATID (width 7))`.
  2. **Value literals now width-reconciled.** A new `align_rule_drive_widths` post-pass (before
     `dedup_conflicting_rules`) with `parse_sized_literal` / `align_value_to_width` / `ValueAlign` /
     `value_width_residual_packet` re-renders an over-wide-but-fitting based literal as `W'd<v>` (DTI
     `0B01`→`1'd1`, trace `0x7D`→`7'd125`) and DROPS a rule whose value overflows the signal width with an
     `isf_value_width_*` residual — never truncating. Bare decimals / symbols / matching literals untouched.
- **Surface area.** All new symbols are module-private helpers + one private enum (`ValueAlign`) inside the
  existing `isf_ir` emitter; no public API, IR-schema, or subsystem-boundary change. The IR types it reads
  (`WidthHint`, `InterfaceSignalRecord`, `IsfRule`, `ResidualDecisionPacket`) are unchanged. So the architecture
  map is unchanged; this is depth, not breadth.
- **Risk/steering.** Emitter-only and downstream of extraction → WIRE-BASED-100 orthogonal by construction
  (measures extraction F1, not `.isf` bytes), verified by 0-new-diagnostic regeneration of the 4 wire golds.
  Verified: 47/47 `isf_ir` tests, full suite 1682 passed / 0 failed, `scripts/run_ci.sh` GREEN, `kg-bench`
  156/156. The remaining ISF-emitter levers (AXI `(port expr)` rule-assignment grammar; DTI ATST upstream
  mis-attribution) stay spun out as separate ownership boundaries.

## Session update (2026-06-21 — ISF-VALUE-WIDTH-EMIT.0/.1: a latent ISF-emitter width bug, measured; no code change yet)

- **No code changed** — this is a measurement-first slice that pins a latent defect in the ISF emitter
  (`ir/isf_ir.rs`) and designs the fix for the compile-gated `.2`. Two coupled root causes:
  1. **First-seen signal dedup drops a grounded width.** The signal-collection loop (`isf_ir.rs:696-700`)
     keeps the *first* `interfaces[].signal_records` entry per name (`if
     seen_signal_names.contains(name) { continue; }`) and skips later records. When the first entry has
     `width_hint=None` (→ default 1) and a later entry carries the concrete width (e.g. trace-bus ATID's
     3rd record = `width 7`), the grounded width is lost. The `.2a.i` recovery (`isf_ir.rs:715-736`) only
     falls back to `actor_ports[].width_hint`, so a signal with no actor-port (ATID) keeps width 1.
  2. **Value literals are never width-reconciled.** `render_isf_control_expression`
     (`isf_ir.rs:1493-1495`) clones the constraint/temporal value literal verbatim; the rule-body emit
     (`isf_ir.rs:418-432`) writes `(SIGNAL value)` with no width check. FSMGen strict
     (`OperandContractValidationSupport.pm`) reads a literal's width by notation digit count and requires
     an exact width-cast match → an over-width literal (`0x7D`→8 bits on width 1) fails HDL pre-generation.
- **Risk/steering.** The fix is emitter-only and downstream of extraction, so WIRE-BASED-100 is orthogonal
  (it measures extraction F1, not `.isf` bytes). The `.2` design: recover the grounded width across **all**
  `signal_records` + `actor_ports`, then re-render value literals width-aligned `W'<radix><digits>` when
  `value < 2^W`, else **residualize** (the `ISF-RULE-CONFLICT-RESIDUAL` pattern) — never truncate (ADR-0006).
  Scope is bounded (4 docs / 13 clauses corpus-wide). The DTI ATST case is an *upstream mis-attribution*
  (SHCFG's value bound to ATST), spun out of this emitter tree. Report
  `docs/research/isf-value-width-alignment-measurement.md`; KM `isf-value-width-operand-contract`.

## Session update (2026-06-17 — KG-ISF-TRANSACTIONS.2m: deterministic channel-membership lever; new IR surface)
- **New typed surface `SignalChannelMembershipRecord` + `EvidenceIr.signal_channel_memberships:
  Vec<SignalChannelMembershipRecord>`** (`ir/evidence.rs`, serde-default + skip-if-empty ⇒ pre-`.2m` artifacts
  deserialize unchanged and docs with no channel captions serialize byte-identically). Built by the pure
  `build_signal_channel_memberships(&source_ir.structured_tables, &provenance)` in `EvidenceIr::build` — the only
  stage with BOTH the table-signal provenance and the SourceIR captions. Helpers `derive_channel_role` /
  `derive_continuation_table_number` / `split_leading_table_number` / `strip_leading_table_word` parse the
  universal `<role> channel signals` caption grammar (ADR-0006, no name list), with continuation-number chaining
  and an ambiguity gate.
- **Public surface change:** `SemanticIr` gains `signal_channel_memberships` (carried by clone from `evidence_ir`,
  mirroring `transaction_anchors`); `TransactionIntent` gains `channel_membership: Vec<TransactionChannelMembership>`
  (new public type `TransactionChannelMembership {channel_role, ports}`), populated in `mint_named_transaction`
  by grouping the transaction's ports by channel role. All three fields are serde-default + skip-if-empty.
- **Consumer:** `commands/validate.rs` (intent path) gains `transactions_with_channel_membership` /
  `transaction_channel_groups` metrics + an `intent_transaction_channel_membership` Info finding + a human-summary
  line — mirroring the `.2i` `phase_membership` surface exactly.
- **Blast radius / boundaries:** purely additive metadata. The ISF emitter (`ir/isf_ir.rs`) lowers `tx.steps` only
  and never reads `channel_membership`, so the emitted `.isf` is byte-identical; the extraction surfaces
  (relations/constraints/temporal/conditional/polarities) are untouched, so WIRE-BASED-100 is provably orthogonal
  (a `git stash` baseline-vs-change AXI EvidenceIR diff shows the only changed field is the new
  `signal_channel_memberships`, 154 records). lib tests 1662→1664 (+2). One `collapsible_if` clippy let-chain
  collapsed. This is a metadata DIMENSION distinct from `phase_membership`: phases are prose-derived (`.2g`),
  channels are caption/provenance-derived (`.2m`) — the channel role is kept verbatim, never mapped to abstract
  address/data/response phases.

## Session update (2026-06-16 — ISF-REGISTER-RESET-EMIT.3: storage var width = true register width; TREE CLOSED)
- **`ir/isf_ir.rs` storage var width corrected.** New `register_var_width(r)` = `size_bits ⊔
  max(bits_high)+1` (declared width, never below the highest located field bit; 32 fallback) replaces
  the prior max-single-field-extent width, which mis-sized multi-field registers. The over-width
  composable resets from `.2` now fit and emit at the true register width.
- **Blast radius:** 1045/2108 register vars change width, **0 in wire docs** (their register fields are
  unlocated/absent → fallback unchanged) → AXI/AHB/APB/SWD `.isf` byte-identical, WIRE-BASED-100
  untouched. CoreSight SoC-600 storage resets 120→183; FSMGen `--strict --check` 0-new (NVMe/HBM2 keep
  pre-existing rule-conflict/enum-literal diagnostics, orthogonal to storage). lib 1649→1651 (+2).
- **`ISF-REGISTER-RESET-EMIT` CLOSED** (`.0`/`.1`/`.2`/`.3`). The ISF emitter now lowers a register's
  documented reset to `(storage (var NAME (width <true>) (reset V)))` whenever it composes a clean
  integer, with an honest residual otherwise — closing the register-reset ISF-fidelity gap.

## Session update (2026-06-16 — ISF-REGISTER-RESET-EMIT.2: register reset values reach the `.isf`)
- **ISF emitter (`ir/isf_ir.rs`) now lowers register reset values.** `IsfStorageVar` gains
  `reset: Option<u64>`; `from_intent_ir` composes each register's reset from its per-field
  `reset_value`s by LSB-tiling (`classify_register_reset` + `parse_reset_literal` +
  `register_field_extent`, a strict-composable gate), and `render` emits
  `(storage (var NAME (width W) (reset V)))` only for a clean in-width non-negative integer — else the
  byte-identical reset-less form (FSMGen defaults to all-0s). This closes an ISF-fidelity gap: the
  register reset was extracted (`RegisterFieldRecord.reset_value`) and carried to IntentIR but dropped
  at the emit boundary.
- **Honest-residual surface added:** `IsfIr.storage_reset_residuals` (one proportionate summary packet,
  `isf_storage_reset_not_lowered`) flows to the adapter artifact's `residual_decisions` via
  `ir/adapters.rs`, mirroring `temporal_residuals`. ADR-0006: numeric parsing only, no name list; no
  value fabricated.
- **Blast radius:** purely additive — only register-bearing docs gain `(reset V)` (live: CoreSight
  SoC-600 +120); all protocol wire specs (APB/AHB/AXI/SWD) emit byte-identical `.isf`. FSMGen
  `--strict --check` 0-new; lib tests 1645→1649 (+4). The storage var width is still max-field-extent
  (a latent bug for multi-field registers) — reconciling it to the true register width is the remaining
  `.3` leaf.

## Session update (2026-06-16 — KG-ISF-TRANSACTIONS.2g: structural transaction-PHASE recognition)
- **New typed surface `TransactionPhaseRecord` + `SemanticIr.transaction_phases: Vec<TransactionPhaseRecord>`**
  (`ir/semantic.rs`, serde-default + skip-if-empty ⇒ pre-`.2g` artifacts deserialize unchanged and docs with no phases
  serialize byte-identically). Built by `build_transaction_phases(&context)` in `SemanticIr::build` (alongside
  `build_transaction_anchors`) — a deterministic prose recogniser over `context.statements`, distinct from the
  section-heading input the anchor builder uses (the `.2g` STEP-1 measurement: the `<qualifier> phase` vocabulary lives in
  `extracted_statements`, not `section_anchors`).
- **Recogniser / gate seam:** `derive_phase_name` (precision gate on the token before a `phase`/`phases` head) + helpers
  `normalize_phase_token` / `is_phase_head_token` + the `PHASE_NAME_STOPWORDS` constant — a stronger, prose-tuned sibling of
  the anchor surface's `derive_transaction_name` / `TXN_NAME_STOPWORDS`. Reuses `normative_vocab::transaction_head_singular`
  (rejects a head noun used as a modifier) and `sanitize_transaction_name`. Universal English grammar, no chip-name list
  (ADR 0006).
- **Boundary / layering:** `transaction_phases` is a SemanticIR-only recognition surface — it is NOT carried into
  `IntentIr.transactions`, so the IntentIR→ISF lowering is untouched and emitted `.isf` is byte-identical (it is the
  substrate for the future `.2h`/`.2i` ordered-body composition, not yet a lowering input). The `validate <semantic-ir>` path
  (`commands/validate.rs`) gains a `transaction_phases` metric + a `semantic_transaction_phase_inventory` Info finding +
  a human-summary line (read-only observation off built IR).
- **Risk/architecture impact:** additive only — no existing extraction/lowering seam changed; the only SemanticIR struct
  literal (`SemanticIr::build`) updated; no other `SemanticIr { … }` constructor exists. `run_ci.sh` green, `kg-bench`
  156/156, lib tests 1642 → **1645**.

## Session update (2026-06-16 — KG-ISF-TRANSACTIONS.2c: grounded signal-set membership)
- **`TransactionAnchorRecord` (`ir/semantic.rs`) gains `signal_set: Vec<String>`** (serde-default,
  skip-if-empty ⇒ pre-`.2c` artifacts deserialize unchanged). `build_transaction_anchors` now takes the
  document's declared-signal inventory (`declared_signal_names`, already built in `SemanticIr::build` from
  `interfaces[].signal_records`) and computes, per transaction, the union of the signal-shaped tokens its
  defining section's statements reference (`StatementContext.signals`) **intersected with the declared
  inventory**. The intersection is essential: the raw token extractor over-captures enum VALUES (`IDLE`,
  `INCR4`) and prose abbreviations (`MPMC`, `AHB5`); keeping only declared signals makes the set faithful.
- **`mint_named_transaction` (`ir/intent.rs`) attaches the membership as ports.** Each `signal_set` member
  becomes a `TransactionPortRecord` with the document-grounded direction from a new map built in
  `recognize_named_transactions` off `actor_signal_relations` (Drives → output, Reads → input, both/neither
  → in/out). Confidence is now keyed on Cue-B corroboration explicitly (not "ports non-empty"), so adding
  membership ports never inflates a non-corroborated transaction to `High`.
- **No ISF / strict change.** Membership lives as IntentIR `TransactionIntent.ports` metadata; the ISF
  emitter lowers `steps`, not `ports`, so the emitted `.isf` and FSMGen `--strict` results are byte-identical
  to `.2b` (APB passes; AHB/AXI keep their pre-existing non-transaction rule errors). Measured live: AHB
  `basic_transfer`→{HCLK,HRDATA,HREADY,HREADYOUT,HWDATA,HWRITE}, `burst_operation`→{HADDR,HBURST,HSIZE},
  `idle_transfer`→{HTRANS,HREADY}+drive body. WIRE-BASED-100 constraint+temporal F1 = 1.000; `kg-bench`
  156/156; `run_ci.sh` green (lib 1641; the existing anchor/mint tests extended to assert membership + the
  enum-value filter).

## Session update (2026-06-16 — KG-ISF-TRANSACTIONS.2b: composed step-by-step bodies + `*_behavior` re-levelling)
- **`intent.rs` transaction synthesis re-levelled:** `synthesize_transactions` lost its per-actor
  `{actor}_behavior` step (the second of its two synthesis sources). The census
  (`docs/research/transaction-capture-census.md` §3.6) established that a `*_behavior` blob is NOT a
  transaction at any level — it is an actor's aggregate timed behaviour built entirely from
  `semantic_ir.temporal_rules`, which are already carried into IntentIR and lowered to valid `.isf`
  through the dedicated temporal path (`txn_temporal_*` asserts / `(rule …)` / explicit residual). The
  `*_behavior` transaction was therefore a redundant SECOND rendering of the same rules — and an
  `.isf`-invalid one: its `when` condition was a multi-word antecedent phrase (`HREADY == HIGH @PreTick`)
  that FSMGen's S-expression parser tokenises into scalar body clauses, tripping
  `when body clauses must be list forms` under `--strict --check`. Dropping it clears that pervasive
  strict-error class corpus-wide while losing no temporal semantics. The two helpers it solely served
  (`render_temporal_predicate`, `temporal_consequent_to_step`) were removed with it (no other callers).
- **`mint_named_transaction` now composes a grounded body:** when a Cue-B match holds (a transaction
  named after an enumerated value of a declared signal, e.g. AHB `idle_transfer` ⟺ `HTRANS = IDLE`), the
  matched `(drive signal value)` is emitted as the transaction's step-by-step body, so the recognised
  transaction RENDERS to `.isf` (passes the `!steps.is_empty()` ISF emit filter) instead of staying
  body-less. Universal structural grammar over the document's own enumeration — no name list (ADR 0006);
  boundary-exact (only the keyed signal it names). Non-corroborated named transactions keep empty `steps`
  (honest residual) until `.2c` grounds full signal-set membership.
- **No subsystem boundary moved**; the change is confined to the IntentIR transaction synthesis. Measured:
  `*_behavior` blobs now 0 across the corpus; APB went strict-FAIL→PASS (its only blocker was the behavior
  error); AHB `idle_transfer` renders with `(drive HTRANS IDLE)`. Remaining wire-doc strict failures
  (AHB HAUSER rule-write conflict, AXI/axi-and-ace/lti `constraint_*` assignment-action grammar, axi-stream/
  generic-flash rule-write conflicts, trace-bus ATID width contract, hbm2 enum-member emission) are
  PRE-EXISTING, non-transaction rule/enum-lowering issues that `.2b` only unmasked — candidate future
  slices, out of `.2b` scope. WIRE-BASED-100 constraint+temporal F1 held at 1.000 (orthogonal); `kg-bench`
  156/156; `run_ci.sh` green (lib 1641; one existing test updated for the composed body, no net new tests).

## Session update (2026-06-16 — KG-ISF-TRANSACTIONS.2a: structural transaction recognition; new SemanticIR surface)
- **Public surface change:** `SemanticIr` gains a typed field `transaction_anchors: Vec<TransactionAnchorRecord>`
  (serde-default, skip-if-empty ⇒ existing artifacts deserialize unchanged; docs that name no transactions
  stay byte-identical). This is the recognition substrate for the transaction layer (Cue A — section
  headings that name transactions), built in `SemanticIr::build` from the EvidenceIR section anchors the
  builder already reads. Rationale captured because it adds a public IR field and a new EvidenceIR→SemanticIR
  carry.
- **`intent.rs` transaction subsystem de-hardcoded:** `recognize_digital_patterns` lost its three hardcoded
  protocol blocks (Patterns 3/4/5 — the only ADR-0006 breach in the transaction path); a new
  `recognize_named_transactions` + pure `mint_named_transaction` consume `SemanticIr.transaction_anchors`
  (Cue A) and `SemanticIr.symbol_definitions` (Cue B corroboration). Patterns 1/2/6 (structural valid-ready /
  req-ack / FIFO shapes) are unchanged.
- **`normative_vocab.rs`** gains `TRANSACTION_HEAD_NOUNS` + `transaction_head_singular()` — the universal
  transaction head-noun vocabulary, joining the relation-verb and logic-level lists as the single authority
  for grammar-not-names (ADR 0006).
- No subsystem boundary moved; the staged `Source→Evidence→Semantic→Intent→adapter` pipeline is intact (the
  recognizer still runs at the IntentIR stage from `SemanticIr` alone — Cue A is carried forward, not a
  back-reference to EvidenceIR). `run_ci.sh` green; lib tests +4.

## Session update (2026-06-14 — bounded-memory/disk big-PDF ingestion + built-in RAM guard + disk pre-flight in the Docling backend)

`MEMORY-BOUNDED-INGEST.1` reworked the embedded Docling helper inside
`ir/source/docling_backend.rs` so very large PDFs ingest with bounded peak memory. The two
per-document extraction loops (page artifacts + element iteration) were lifted verbatim into a
module-level `process_converted_document(doc, acc, …)` driven by an `_IngestAccumulator` (record
lists + id counters externalized so they continue across batches). `main()` now computes a cheap
page count (`detect_pdf_page_count` via pypdfium2) and either runs the historical single-pass
`converter.convert(path)` (when `page_count <= SPECFORGE_INGEST_BATCH_THRESHOLD`, default 512) or
converts in `SPECFORGE_INGEST_BATCH_PAGES`-sized page ranges (default 64) via
`converter.convert(path, page_range=(lo,hi))`, freeing each batch's heavy converted document
(`del`+`gc.collect()`) so peak memory is O(batch) instead of O(page count). **Architecture/risk
note:** ingestion is no longer unbounded in memory — the path that could exhaust the host's RAM on
a >500-page PDF is closed. The change is gated above 500p, so every current corpus doc keeps the
exact single-pass call (byte-identical, proven). No SourceIR schema change; the `DoclingBackendSummary`
manifest is unchanged. Verified by full `run_ci.sh` (1587) + a temp-14p single-pass-byte-identity /
batched-correctness proof; the CHI 585p memory proof under the RAM guard is `MEMORY-BOUNDED-INGEST.2`.

`MEMORY-BOUNDED-INGEST.3` added the **disk** dimension in the same helper: a new `_env_flag` plus a
`save_page_images` decision threaded into `process_converted_document`. Page images are still
generated in memory (region cropping reads them via `PictureItem/TableItem.get_image`), but the
per-page PNG `.save()` is skipped for large docs — so ingest disk is O(assets), not O(pages). The
gate defaults to "persist at/below `SPECFORGE_INGEST_BATCH_THRESHOLD`, skip above it" (small docs
byte-identical) and is overridable via `SPECFORGE_INGEST_SAVE_PAGE_IMAGES=1/0`. When skipped,
`page_image_path`/`rendered_image.path` are `None` while `width_px`/`height_px`/`dpi` stay recorded.
**No Rust type change** — both `PageArtifact` path fields are already `Option<PathBuf>`, so a null
deserializes cleanly. The investigation behind it confirmed no downstream consumer reads page images
(only `VisualAsset.image_path` region crops), so this is a pure, zero-fidelity-loss disk win.

`MEMORY-BOUNDED-INGEST.4a` made the autonomous RAM guard a first-class Rust subsystem (the `.1`/`.3`
work was Python-helper-only; this is the first Rust-side change in the tree). `materialize_pdf` no
longer calls the blocking `command.output()` directly — it builds a `RamGuardConfig` from the
environment and runs the backend through `run_backend_with_ram_guard(command, display_name, &guard,
stdout_path, stderr_path, used_percent_fn)`: a pre-spawn memory sample, then spawn with stdout/stderr
redirected to temp files, then a poll loop (`child.try_wait()` every 50 ms, memory sampled every
`sample_interval`) that `kill()`s the child and returns a typed error on breach. The memory reader is
injected (`&dyn Fn() -> Option<f64>`) so the abort/kill/complete branches are unit-tested with no real
pressure; production wires `current_used_memory_percent` (a no-dependency `memory_pressure` /
`/proc/meminfo` reader, pure text parsers gated `#[cfg(any(target_os = "…", test))]`). **New public
error surface:** `error.rs` gains `AppError::IngestAbortedForMemory { program, used_percent,
ceiling_percent }` with an actionable Display — the typed boundary now distinguishes a self-protective
memory abort from a generic `ExternalCommandFailed`. **Architecture/risk note:** the ingest path can no
longer crash the host on memory pressure — it fails closed with the staged-swap intact (only
`normalized.staging` is discarded on abort). Behavior with headroom is unchanged except the child's
stdout/stderr move from pipes to temp files (`render_backend_output_files` mirrors the old
`render_command_output` formatting for error reporting). Config: `SPECFORGE_INGEST_RAM_ABORT_PERCENT`
(85; off/out-of-range disables), `SPECFORGE_INGEST_RAM_SAMPLE_SECS` (2, floor 1). Verified by full
`run_ci.sh` (lib 1587 → **1596**, +9 tests) + kg-bench 156/156.

`MEMORY-BOUNDED-INGEST.4b` added the disk pre-flight as the first statement of `materialize_pdf`
(before any staging dir, so a refusal touches nothing). New error surface
`AppError::IngestAbortedForDisk { path, free_mb, required_mb }`. The requirement scales off the cheap
pre-ingest signal Rust has — the source PDF file size — via `estimate_required_disk_mb` (128 base +
source_mb × 4); a precise estimate is ill-posed pre-ingest (asset count unknown; page count only in
the subprocess). Free disk is read via no-dep POSIX `df -P -k` (`available_disk_mb` +
`parse_df_available_kb`), permissive when unreadable (`check_disk_preflight` only refuses on a reading
below the requirement). Config knob `SPECFORGE_INGEST_MIN_FREE_DISK_MB` → `DiskPreflightRequirement`
(estimate / fixed floor / off). **Architecture note:** ingestion now pre-flights both resources (RAM
via `.4a`'s pre-spawn sample, disk via `.4b`) before launching, and fails closed with an intact prior
bundle. CI also surfaced a latent test concurrency bug (PATH-lookup spawns racing the
`inspect_docling_runtime` `PATH=""` tests) now fixed by holding `env_var_lock()` in the spawning tests.
Verified by full `run_ci.sh` (lib 1596 → **1604**, +8 tests) + kg-bench 156/156.

`MEMORY-BOUNDED-INGEST.4c` made the page-range batch size adaptive to the host. `materialize_pdf`
now resolves an effective batch via `BatchSizePolicy::from_env().effective_pages(&current_total_memory_mb)`
and sets it on the child's `SPECFORGE_INGEST_BATCH_PAGES` env (the Docling helper already reads it, so
the Python side is unchanged — Rust is the single decision point). The decision is a pure, deterministic
function of **total physical RAM** (a per-machine constant — deliberately not free memory, which would
jitter the batch boundaries and break the determinism doctrine): `adaptive_batch_pages(total_mb,
ceiling, floor)` clamps a discrete RAM-band ladder (`>= 16 GB` → ceiling, `8–16 GB` → ≤32, `4–8 GB` →
≤16, `< 4 GB` → floor 8) into `[floor, ceiling]`. `SPECFORGE_INGEST_BATCH_PAGES` is reinterpreted as a
ceiling (Rust now reads it too, via `parse_batch_pages_ceiling`); `SPECFORGE_INGEST_ADAPTIVE_BATCH`
(`parse_adaptive_batch_enabled`) forces the fixed ceiling. Total RAM is read with no new dependency
(`sysctl -n hw.memsize` on macOS, `/proc/meminfo` `MemTotal` on Linux; pure parsers
`parse_sysctl_memsize_bytes` / `parse_linux_meminfo_total_mb` gated `#[cfg(any(target_os, test))]`), and
the reader is injected (`&dyn Fn() -> Option<u64>`) for host-safe DI tests — mirroring `.4a`. No new
error surface (sizing never errors — the `.4a` guard remains the hard backstop). **Architecture note:**
this is the second Rust-side ingest change in the tree; `>= 16 GB` hosts resolve the unchanged 64
ceiling so all gold/intact docs stay byte-identical, while a small machine completes at a smaller batch
(speed flexes, quality invariant). Verified by full `run_ci.sh` (lib 1604 → **1611**, +7 tests) +
kg-bench, and live-proven byte-identity (adaptive-on vs `off`) on the 24 GB dev host.

## Session update (2026-06-14 — prior-memory UTF-8 OOM blocker fixed; lib 1587)

`PDF-VARIANT-DIGESTION.13b.1` fixed a latent correctness/OOM defect in
`ir/prior_memory.rs::replace_term_with_placeholder`: its non-matching copy did
`result.push(bytes[index] as char)` (a Latin-1 cast, not a UTF-8 decode), mangling every
multi-byte character into per-byte mojibake that roughly doubled in length; because
`normalize_prior_phrase` chains one pass per multi-word replacement term, the mangling
re-doubled `2^N` in the multi-word-term count. On a real corpus doc (ACE — 173 multi-word
actor names + a `•`-bearing signal-table cell) this drove the `evidence` build to 17.2 GB RSS
and an OS SIGKILL, blocking the whole `.13b` corpus re-ingest sweep. The `else` arm now copies
one whole UTF-8 char (`text[index..].chars().next()` → `push(ch)` → `index += ch.len_utf8()`),
byte-for-byte identical to the old copy on pure-ASCII input. **Risk-picture update:** the only
non-CI-tested path that could OOM the pipeline on a real document is now closed; the prior-memory
subsystem is unchanged in shape (no schema/boundary/public-surface change — it is a one-function
fix plus 4 hermetic regression tests). `cargo test -p specforge --lib` = **1587** passing;
kg-bench 156/156; full `run_ci.sh` GREEN. Byte-stability re-proof: 15/15 non-ACE intact bundles
byte-identical pre-fix vs post-fix; ACE evidence now completes (21 s / 56 MB). KM card
`prior-phrase-utf8-byte-as-char`.

## Session update (2026-06-11 — tenth framework surface: `signal_presence`)

`PDF-VARIANT-DIGESTION.12b` registered the TENTH EvidenceIR surface on the extractor framework:
`signal_presence` (`SignalPresenceRecord` — literal-case signal name, verbatim
`presence_condition: Option<String>`, literal `(variant_label, code)` entries, table provenance), a
key-merge surface whose key is the full row CONTENT (name + condition + variant entries, deliberately
NOT the table id, so page-break re-listed rows dedup first-wins) with `presence_id`s assigned post-merge.
Like `message_fields` it reads ONLY `SourceIr.structured_tables`; unlike every other surface its entire
gate/capture pipeline is ONE shared pure function (`capture_signal_presence_rows` in `ir/evidence.rs`:
structural gate → `.5h`-style content-rotation remap → split-spill integrity (orphan row-labels in a
second column ⇒ whole-table refusal) → header-designated column roles (presence/property = condition;
declaration vocabulary excluded) → all-rows-consistent fused-pair label/code splitting → per-row
all-code capture), consumed by BOTH the extractor and
`completeness::signal_presence_capture_covers` (validate-time coverage, strict ≥1-captured/0-refused) —
the `.12a` one-definition-no-drift pattern applied to a whole capture pipeline rather than a single
row rule. The surface mints records only (no statements, no ids shared with the assembly phase), which
is what made the 12-bundle byte-identity parity proof exact: every non-matrix doc differs only by the
manifest's new `signal_presence` entry. kg-bench gained the `signal_presence_*` expectation surface
(count / include with per-variant label↔code locks / name excludes) and a gold + malformed-refusal
fixture pair.

## Session update (2026-06-10 extractor framework — converge-loop reach; subsystem now 9 surfaces)

`EXTRACTOR-ARCHITECTURE.2`–`.9c` built a subsystem this analysis had not yet captured (closing that gap
here): the **unified extractor framework** in `crates/specforge/src/ir/extractor.rs`, now the wiring layer
for NINE EvidenceIR surfaces. The ninth (`EXTRACTION-QUALITY-GAUGE.FIELD.2`, same day) is
`message_fields` — a key-merge surface (`MessageFieldRecord`, key = container+name) extracting
packet/flit protocols' declared message fields from container-captioned field-titled tables, with the
register surface keeping priority over shared `Field` columns via `is_register_field_header` (the
one-place discriminator). `PDF-VARIANT-DIGESTION.10b` (2026-06-10) gave the surface its second
strategy, `message_fields.bit_position_table`: two-column `bits | description` tables declaring in-memory
STRUCTURE layouts (AMD DTE, NVMe command dwords) extract through a strict pure-bit-cell parser plus a
bit-exact caption-less fragment-chain stitcher, and `MessageFieldRecord` gained an additive
`bit_range: Option<(u32, u32)>`; `field_id`s are now assigned at the surface level after the key-merge so
they stay unique across strategies. `PDF-VARIANT-DIGESTION.10c` (same day) unified the bit-layout
machinery (`collect_bit_layout_tables` + `stitch_bit_layout_chains`: one 2-col/3-col family scan, one
bit-exact chain pass, label-kind routing) and added a THIRD register-surface strategy
(`registers.bit_assignment_table`: caption-named registers from TRM `bits|name|function` chains, with
access/reset honestly absent). `PDF-VARIANT-DIGESTION.10d` (2026-06-11) widened the same strategy's
literal grammar — no new extractor — for dword-relative offset-suffixed cells:
`parse_offset_suffixed_bit_position` beside the pure parser (2-col arm only), an additive
`MessageFieldRecord.byte_offset: Option<u32>` + `BitLayoutRow.byte_offset` carrying the offset verbatim
(absolute positions deliberately never derived — description brackets are value slices), a dword-relative
chain-adjacency branch in `bit_position_chain_adjacent` (the `(offset asc, bit desc)` successor with a
`dword_rows_forward` guard; conventions never cross-join), two new forms in the shared
`recover_field_mnemonic` chain (`bracket_slice_field_name` — verbatim names incl. the slice;
`single_letter_framed_name` — colon/dot frame + per-table uniqueness through
`fused_name_lead_count_key`, whose 1-char keys are disjoint from the existing 2–40-char keys), and a
shared `trim_continued_marker` handling `(Continued)` markers fused to the previous caption word.
`PDF-VARIANT-DIGESTION.10e` (2026-06-11) added the surface's THIRD strategy,
`message_fields.byte_location_table`: `byte location | size (bytes) | register description` placement
tables (probe-overturned: in-memory STRUCTURE layouts, not register maps) extract through a family-local
collector (`byte_location_layout_columns` header-position gate, row-level wrapped-prose skips unlike the
`.10b` whole-table rule), a byte-exact chain pass (`byte_location_chain_adjacent`:
`next_offset == prev_offset + size`, symbolic-size tails close the chain) reusing `BitLayoutLabel` /
`bit_layout_labels_agree` / `bit_position_container_label`, and a family-LOCAL name grammar
(`byte_location_field_name`: head-before-definitional-frame multi-word names, trailing-paren mnemonic
trusted past bleed, period-is-bleed refusal — deliberately NOT the shared `recover_field_mnemonic`
chain, which would truncate `FRU ID`→`FRU`); `byte_offset` doubles as the field's own byte offset when
`bit_range` is `None` (doc contract amended; the two readings are disjoint by construction) and
`bit_width` converts plain-count byte sizes exactly (×8). `PDF-VARIANT-DIGESTION.10f` (2026-06-17) added a
FOURTH strategy, `message_fields.section_header_field` — the first message-field reader that reads
`SourceIr.document_sections` rather than `structured_tables`: DTI-class message protocols write each field
as its own `<NAME>, bits [hi:lo]` section HEADING under a dotted-numbered message container with a
`Field descriptions` anchor (`extract_section_header_message_fields` + `parse_dotted_container_heading` /
`parse_section_header_field` [reusing `parse_pure_bit_position`] / `normalize_section_field_name` /
`is_section_header_field_name` / `is_message_container_name` / `caption_names_register` /
`is_register_attribute_heading`). Container-decides routing (the `.10b`/`.10c`/`.10e` register-iff-attribute
rule: a container is a register iff its caption says `register` or it carries an `Attributes`/`Accessing`
sub-heading, else a message/structure) keeps GIC/SMMU/CoreSight register fields out of the message surface
(routed to the register surface by the sibling `.10g`). Live: DTI 0→159 fields / 17 containers, only DTI fires (1/79); old-vs-new
parity byte-identical except the new manifest entry (NVMe 216 / AMD 217 unchanged). `PDF-VARIANT-DIGESTION.10g`
(2026-06-17) added the register surface's FOURTH strategy, `registers.section_header_field`, the
register-routed twin of `.10f`: `.10f`'s container-walk was factored into ONE shared classifier
(`scan_section_header_field_containers` → `{name, is_register, has_anchor, fields}`) plus a shared field
gate (`distinct_section_header_fields`), so the register-vs-message routing lives in exactly one place and
cannot drift — `.10f` keeps the non-register containers, `extract_section_header_registers` keeps the
register-routed ones and emits a `RegisterRecord` per container (access/reset/offset/description honestly
absent). Its decisive precision lever is a per-document name-uniqueness residual gate: a short register
mnemonic reused across access-port blocks (ARM-Debug `AUTHSTATUS`/`CSW`/`IDR`) is structurally ambiguous —
the occurrences are a mix of identical cross-refs, subset views, and genuinely-different registers — so a
name reused across ≥2 register containers is held as an honest residual, never over-counted nor conflated
by the existing all-distinct `consolidate_register_field_fragments` merge; a unique name matching an
existing 0-field record (e.g. `DPIDR`) instead MERGES through that post-pass (`.10g` runs last, so the
existing identity is kept — no double-count). Live: 180 registers / 934 fields across exactly 5 architecture
specs (GIC 73/468, SMMU 88/381, CoreSight 5/24, ACC 2/4, ARM-Debug 12/57); ARM-Debug `evidence` 29→37
records, +8 brand-new, `DPIDR` enriched, zero duplicate names; register/wire golds byte-identical except the
new manifest entry. The message-field surface is one of the surfaces whose extractor reads ONLY `SourceIr` document content
and persists a new typed inventory (`EvidenceIr.message_field_records`) consumed by the entity-typing
ground (`EntityType::Field`, `.FIELD.3`) and — via `.FIELD.4` — by the LLM-primary constraint extractor:
`ir/constraint_extract_llm.rs` now grounds into a typed `GroundedConstraint::{Signal,Field}` dispatch
(`ground_constraint_typed`; shared gates, subject-type routing; `ground_constraint` kept as the
signal-only view) and the parallel `EvidenceIr.message_field_constraints` surface
(`MessageFieldConstraintRecord`: field subject + catalog containers + the shared
`SignalConstraintKind` vocabulary) keeps the canonical signal surface wires-only by construction,
with `dedup_merge_by` as the generic provenance-merging dedup core behind both surfaces.

- The frame: a generic `Extractor<R>` trait (`name`/`tier`/`applies_to`/`run`), a borrowed
  `ExtractionContext` (grows one field per migrated cluster; currently carries `statements`), TWO drivers —
  `run_surface` (first-wins key-dedup merge; registry order = precedence) and `run_surface_concat` (ordered
  concatenation, post-passes stay in the surface helper) — and an inspectable
  `SurfaceRun`/`SurfaceManifest`/`ExtractionManifest` chain persisted on `EvidenceIr.extraction_manifest`
  (additive, `#[serde(default)]`) and surfaced by `validate`. The manifest is the per-document behavioral
  fingerprint the `CORPUS-PATTERN-REUSE` plane clusters on — the two subsystems compose.
- Three explicit producer categories (the two-phase model, documented in `ir/extractor.rs`): key-merge
  surfaces (FSM states, semantic hints, serial-frame fields, message fields), concat surfaces (registers, actors,
  SWD-operations, signal polarity, actor-signal relations), and stateful-assembly orchestrators that
  deliberately stay OFF the drivers (the signal-declaration seed; the constraint family below).
- `PDF-VARIANT-DIGESTION.12a` (2026-06-11) extended the signal-declaration seed with a trapped-row
  GAP-FILL pass (`synthesize_trapped_row_signal_declarations`, run LAST so its inventory gate sees the
  complete declared universe — base statements + table seed + prose fallback): it consumes the now-SHARED
  `.9.11` trapped-row rule (`recovered_trapped_data_rows` — the timing extractor refactored onto the same
  single definition) and the new `continuation_inherited_table_heads` view (an `unknown`-kind
  `Continued from previous page` fragment resolves to its captioned chain head via caption parent-reference
  + exact first-header-signature; pub(crate), also consumed by `completeness::unexplained_intent_bearing_tables`
  and validate's `intent_bearing_table_count` so the gauge's numerator and denominator share one kind view;
  SourceIR itself is never mutated). The completeness coverage (`densest_signal_name_column_tokens`) chains
  trapped rows after `body_rows` with unchanged one-unknown-keeps-it-flagged strictness. Deliberate
  consistency note: the gap-fill registers NO manifest strategy — the seed is a stateful-assembly
  orchestrator off the framework drivers by design, and on the entire rebuildable corpus the pass is a
  measured no-op (13/13 old-vs-new byte-identical).
- `.9b` + `.9c` (this session) extended the framework's reach into `converge_evidence_extractions` — the
  fixed-point evidence loop: **signal polarity** is two `Extractor<SignalPolarityObservationCandidate>`
  units (`signal_polarity.prose` / `signal_polarity.tables`) run per pass via `run_surface_concat` plus the
  unchanged accumulate-and-arbitrate post-pass (`arbitrate_signal_polarity_observations`); **actor-signal
  relations** is two `Extractor<ActorSignalRelation>` units (`relations.prose` / `relations.tables`, the
  table list build-precomputed and re-emitted per pass) plus the two ORDERED legacy post-passes
  (check-signal augmentation over the full pre-dedup list, THEN first-wins dedup — key-merging in the
  driver would change the augment input). The loop threads `&mut ExtractionManifest`, and `record()`'s
  replace-per-surface-name semantics make the manifest hold exactly the final converged pass. The remaining
  loop family — **constraints + conditional rules** — is one per-pass `constraint_counter` minting ids
  across three extractors + a cross-surface polarity post-pass: stateful-assembly, kept as cohesive
  in-loop orchestration by design.
- Verification doctrine for every migration slice: 12-doc intact-bundle rebuild, non-manifest JSON
  byte-identical (baseline double-run fixpoint first — guaranteed by the `EVIDENCE-DETERMINISM` fixes);
  kg-bench 151/151; full `run_ci.sh`. Lib tests at **1499**.
- Remaining seam risk: `EvidenceIr::build()` is still the ~500-line orchestrator; retiring it is the
  tracked end-state of `EXTRACTOR-ARCHITECTURE`.

## Session update (2026-06-09 CorpusMemory schema v6 — extraction-profile priors, the 8th family)

`CORPUS-PATTERN-REUSE.3b.2` extends the cross-document learning plane with one additive schema surface;
no extraction-path or subsystem-boundary change.

- `ir/prior_memory.rs`: `CorpusMemory` schema **v5 → v6**; new `#[serde(default)]`
  `extraction_profile_priors: Vec<ExtractionProfilePriorRecord>` (older v5 stores remain loadable).
  The record is deliberately NOT `ProtocolFamily`-scoped — its `cluster_signature` (the sorted,
  ADR-0006-safe derived feature tokens from `ir/corpus_cluster.rs`) is itself the scope, since
  vendor/layout families cross protocol-name lines. Lookup seam:
  `CorpusMemory::extraction_profile_priors_for(&BTreeSet<String>)` — strict signature-subset match,
  empty signatures skipped. The future consume seam (`.3b.3`) is bound to the recorded activate-only
  contract (a profile may only enable an opt-in extractor, never disable a default-on one).
- `commands/learn_priors.rs`: harvest seam — accepted artifacts' EvidenceIR fingerprints
  (`load_evidence_ir_for_learning`, refactored out of `load_source_ir_for_learning` with no behavior
  change) → pure `materialize_extraction_profile_priors` (multi-member clusters only) at the new shared
  `ir::corpus_cluster::DEFAULT_FINGERPRINT_SIMILARITY_THRESHOLD` (= 0.6, also the `corpus-cluster`
  CLI `--threshold` default via `default_value_t` — one constant, two surfaces, no drift).
- Mechanical surface: 8 `schema_version` literals bumped, 9 exhaustive `CorpusMemory` literals + the
  kg-bench `PriorMemoryPatch` extended. `cargo test -p specforge --lib` = **1491** passing; kg-bench
  151/151.

## Session update (2026-06-08 ramp-up currency correction — command + IR-module + size + test inventory)

State verified directly from the working tree at HEAD `67aee533` (fully pushed; `origin/main..HEAD`
= 0). This entry refreshes four counts that drifted as the downstream extraction/eval/NLI command
family and its supporting IR modules landed between `2026-05-30` and `2026-06-08`. Older dated
sections below are preserved as historical record; where they cite smaller counts they are
superseded by this entry. No architecture claim below is reversed — `IrStage` is still the same
five stages, `.isf` via `IsfIr` is still the sole adapter target, and the Ollama+Qwen2.5VL
LLM/VLM provider is still production-default.

- **Command surface = 25 subcommands** (`crates/specforge/src/cli.rs` `enum Commands`, dispatched
  in `lib.rs`), not 17. The 2026-05-30 entry's list of 17 predates the extraction/eval family added
  since: `extract-contracts`, `signal-resolve`, `eval-extraction`, `nli-verify`, `grits-consensus`,
  `entity-type`, `extract-conditions`, `extract-constraints-llm` (plus the already-listed core 17).
  `commands/mod.rs` also carries `pub(crate) mod llm_text` (a shared text-chat transport helper, not
  a subcommand).
- **IR namespace = 25 modules** (`crates/specforge/src/ir/mod.rs`), not 14. Beyond the seven R16
  modules, the measurement/extraction work added: `completeness`, `condition_extract`,
  `constraint_extract_llm`, `entity_typing`, `extraction_filters`, `nli_verify`,
  `nlp_relation_extract`, `normative_vocab`, `ambiguity`, `temporal_ltl`, `register_bits` (plus
  `prior_memory` and `adapters`). Still a typed layer over the four IR stages — no sixth stage.
  `register_bits` (EXTRACTION-GAP-FIX.4a) is a PURE gated core: it reconstructs register-field bit
  ranges from VLM-read `(name, width)` proposals by cumulative LSB tiling, attaching bits only when
  the widths tile a standard register width AND the names match the register's own field table (else
  an honest residual). Its production seam is the `recover-register-bits` command.
  `completeness` also now owns the `PDF-VARIANT-DIGESTION.5a`/`.5b`/`.5c` document-quality surface: pure
  `classify_document` (doc-class from a structural census), `front_matter_doc_type_hint`, and
  `document_completeness_gauge` (a class-aware per-document completeness gauge — `DocumentCompletenessGauge`
  / `CompletenessGap`, where a `Guide` is "not applicable" and the other classes gauge each dimension only
  when its denominator > 0). `completeness` also owns the `DOC-INTENT-TAXONOMY.3` PURPOSE recognizer: pure
  `classify_document_intent_category` (the 6-category semantic taxonomy — `DocumentIntentCategory` /
  `IntentCategoryConfidence` / `DocumentIntentClassification`) built ON the same `DocumentClassCensus` (now
  carrying `message_field_records` / `signal_presence_records` / `front_matter_isa` / `front_matter_phy`) plus
  the generic front-matter `front_matter_declares_isa` / `front_matter_declares_phy` helpers (ADR 0006), with
  only clean-wire and self-declared-guide at HIGH confidence and an explicit residual otherwise. Its
  production seam is `commands/validate.rs` (`document_class` / `document_intent_category` /
  `document_completeness_gaps` metrics + `evidence_document_*` / `evidence_document_intent_category` findings).
- **Whole `crates/specforge/src` ≈ 103,200 lines** (single workspace crate, edition 2024), up from
  the ~88.7K cited on 2026-05-29.
- **`cargo test -p specforge --lib` = 1435 passing, 0 failed, 0 ignored** (the canonical validation
  command). Any earlier `1433`/`1427`/`1421`/`1360`/`1014`/`666` counts below are historical.
- **Eval/gold infrastructure** (load-bearing for the active `PDF-VARIANT-DIGESTION` precision lane):
  the pure scorer is `crates/specforge/src/eval.rs` (`EvalTask`/`GoldFact`/`score_dataset` +
  `score_dataset_source_tolerant`); the runner is `commands/eval_extraction.rs`; committed gold
  seeds live under `crates/specforge/test_data/llm_eval/seed_*.json`. A new surface is wired by
  adding an `EvalTask` arm + a `GoldFact` variant + a canonical-key fn + a prediction indexer + an
  `extract_on_copy` branch, mirrored by a `committed_*_seed_loads_and_validates` regression test.

## Session update (2026-05-30 ramp-up currency correction — LLM/VLM provider + command surface)

State verified directly from the working tree. This entry corrects three stale
spots a ramp-up audit found, so the document is not read as implying
capabilities are absent that are in fact production-default. Older sections
below are preserved as historical record. (Owned by
`docs/tasks/AUDIT-PROVIDER-FRAMING-RECONCILE.md`.)

### The LLM/VLM provider is production-default — NOT deferred future work
SpecForge ships fully-integrated Ollama + Qwen2.5VL as the **production
default**, not a deferred upstream dependency. Where this file (or `ROADMAP.md`)
frames "a prose LLM/VLM provider" as missing, it refers ONLY to *wiring that
provider's output into the R16 CVE producer `parse_constrained_contract`* —
not to the provider's existence.
- `VlmProviderArg` (`crates/specforge/src/cli.rs`) = `ollama | open-ai |
  lm-studio | skip`; `converge` defaults BOTH `--vlm-provider` and
  `--nlp-provider` to `ollama`. `DEFAULT_LOCAL_MODEL = "qwen2.5vl:7b"`
  (`commands/doctor.rs`); OpenAI-compatible
  `http://localhost:11434/v1/chat/completions`.
- `commands/enrich.rs` — VLM timing/state-diagram enrichment (diagram PNG →
  signal names + cycle states → `VisualAsset.note` → EvidenceIR
  `VisualObservation` → SemanticIR `TimingConstraintRecord`).
- `commands/nlp_enrich.rs` — NLP Level-3 prose relation extraction
  (`NormativeStatement` → `SignalConstraintRecord` / `ConditionalRuleRecord`).
- `commands/doctor.rs` — preflights Ollama / LM-Studio reachability + model
  presence. Validated end-to-end on AMBA (AXI/APB/AHB at 90–95/100).
The only genuinely upstream-absent piece is a typed PDF→`FigureRegion`
raster/vector decoder (the VLM already reads diagram PNGs, just to text).

### Current command surface (representative core; all live)
`inspect`, `doctor`, `converge`, `ingest`, `evidence`, `semantic`, `intent`,
`adapt`, `enrich`, `nlp_enrich`, `validate`, `project-validation`,
`rescan-plan`, `kg-bench`, `learn-priors`, `corpus-cluster`, `corpus-kb`, `clean`
(plus the LLM/eval surfaces `extract-contracts`, `signal-resolve`,
`eval-extraction`, `nli-verify`, `entity-type`, `extract-conditions`,
`extract-constraints-llm`, `audit-extraction`, `recover-register-bits`,
`grits-consensus`). `corpus-cluster` (`commands/corpus_cluster.rs`,
`CORPUS-PATTERN-REUSE.3a`) is a read-only inspection command that surfaces the
pure `ir::corpus_cluster` derived-fingerprint clustering over the persisted
`generated/evidence_ir` corpus — additive, no extraction-path change. The
authoritative list is `crates/specforge/src/cli.rs`, dispatched in `lib.rs`.
`nli-verify` is no longer print-only (`EXTRACTION-QUALITY-GAUGE.0`,
`2026-06-10`): it back-annotates a persisted `extraction_quality_gauge` record
onto the EvidenceIR via `commands/nli_verify.rs::measure_and_persist_gauge` —
the same helper `converge` calls after stabilization for the standing
per-document quality report — and `validate` reports the persisted gauge
provider-free (metrics + Info/Warning findings; pure builders
`gauge_from_conformal_pass` / `gauge_is_stale` in `ir/nli_verify.rs`).

### `.fsm` adapter sections are historical
Any subsection below describing a `.fsm` adapter / HDL lowering as *present* in
SpecForge predates `ISF-ONLY-CONSOLIDATION` (which removed the entire `.fsm`
adapter + HDL surface). SpecForge's sole adapter target is `.isf` via `IsfIr`
(`ir/isf_ir.rs`); FSMGen owns scheduling / `.fsm` / HDL downstream of `.isf`.

---

## Session update (2026-05-29 ramp-up re-analysis — R16 ContractIR subsystem captured; program complete)

State verified directly from the working tree at HEAD `44bf2723`, not inherited
from prior notes. This entry closes a documentation gap: the entire **R16 SOTA
design-intent-capture** subsystem landed `2026-05-19 → 2026-05-20` (six closed
sub-trees) but this analysis file had not yet captured it — its newest prior
entry was the `2026-05-18` ISF-ONLY snapshot ("IR module count: 7, ~82K lines").

### Architecture delta since the 2026-05-18 snapshot
- The `ir/` namespace grew from **7 → 14 modules**. R16 added seven new typed IR
  modules (`crates/specforge/src/ir/mod.rs`): `contract.rs`, `protocol_graph.rs`,
  `fidelity.rs`, `fusion.rs`, `waveform.rs`, `figure_region.rs`, `cve.rs`
  (4,657 lines combined). Whole `crates/specforge/src` is now **88,673 lines**
  (was ~82K). No IR *stage* was added — R16 is a **typed layer over the existing
  four stages**, not a sixth stage. `IrStage` is unchanged
  (`SourceIr/EvidenceIr/SemanticIr/IntentIr/IsfAdapter`).
- `semantic.rs` is now 21,424 lines and `intent.rs` 5,103 lines; the R16
  producers/consumers were wired into `SemanticIr::build` and carried by
  `IntentIr` without changing on-disk artifacts (additive, serde-skipped fields).

### The R16 ContractIR subsystem (typed timed-contract layer)
The thesis (recorded in `docs/tasks/R16-INTENT-CAPTURE.md`): a design PDF encodes
intent as the temporal behavior of actors at their boundary; the crux is
accurate prose+waveform → typed-KG extraction, and everything downstream of an
accurate typed KG is almost mechanical. R16 built the **mechanical-to-lower typed
target + the objective fidelity metric + the honesty enforcement** first, leaving
upstream extraction (waveform raster/vector bytes; prose LLM/VLM provider) as
honestly-deferred future trees. Module-by-module (test counts are `#[cfg(test)]`
unit tests inside each module):

- **`contract.rs`** (725 ln, 8 tests) — the timed-contract algebra and lowering
  spine. `ActorContract` (per-actor assume/guarantee over boundary signals) with
  a closed operator algebra: `Obligation` (Eventually/Stable/Drive/
  HandshakeBarrier/Persist/Sequence/Mutex/OrderedBefore/`Observe`),
  `EventExpr` (Edge/Level/HandshakeFire/Start/PhaseBoundary), `Window`
  (Within/Between/SameCycle), `Condition` (Eq). `LoweringDisposition`
  (`Lowerable` | `Residual{reason}`) is the honesty seam.
  `contract_from_temporal_rule()` is the lossless `TemporalRuleRecord →
  ActorContract` migration; `.isf` lowering re-points onto it (3-way-proven
  parity gate). `Observe` is the deliberate "honest weak fact" obligation for
  under-licensed evidence.
- **`protocol_graph.rs`** (337 ln, 5 tests) — first-class protocol structure:
  `Channel`/`ProtocolPhase`/`Transaction`/`HandshakePair` + `ChannelRole`.
  `ProtocolPhase` (protocol-stage granularity) is deliberately distinct from
  `TickPhase` (clock-edge). `project_handshake_pairs()` derives `HandshakePair`
  from `HandshakeBarrier` obligations — a **lossless restatement, not PDF
  extraction**. `is_empty()`/`counts()`/`dangling_contract_refs()` support the
  serde-skip discipline and validation.
- **`fidelity.rs`** (784 ln, 9 tests) — the program's **objective function**.
  Six `FidelityGate`s (RealizableBoundary/RealizableDirection/
  RealizableHandshake/ResidualHonesty/NoStrictInvalid/FigureConformance) with a
  three-valued `FindingStatus` (`Pass`/`Fail`/`NotEvaluated` — `NotEvaluated`
  is never silently `Pass`). `FidelitySummary.score()` ranges over evaluated
  gates only; `meets_threshold(1.0)` is the honest default.
  `evaluate_figure_trace()` is the bounded structural trace-replay primitive
  reused by the waveform verifier.
- **`fusion.rs`** (607 ln, 9 tests) — multimodal contract fusion. `FusionKey`
  (actor, channel, phase, obligation_kind, primary_signal); `merge_cluster()`
  deterministically merges agreement (provenance union + `Mixed` modality + min
  confidence + `"fused:…"` id) and routes disagreement to `Residual`.
  `apply_fusion()` is the producer (order-preserving, idempotent).
- **`waveform.rs`** (1,043 ln, 13 tests) — the crux extraction intermediate.
  `PartialTrace`/`LaneEdge`/`EdgeKind`/`ValueSpan`/`RelativeDelay`/`CausalArrow`
  is the typed contract between an (out-of-tree) extractor and the generalizer.
  `generalize_partial_trace()` applies four conservative rules
  (RelativeDelay→Eventually, multi-tick ValueSpan→Stable, next-tick
  CausalArrow→Eventually; **under-determined → `Observe`+`Residual`**).
  `verify_contract_against_trace()` is a round-trip oracle reusing
  `fidelity::evaluate_figure_trace`. `figure_region_to_partial_trace()` adapts
  the input contract below.
- **`figure_region.rs`** (150 ln, 0 tests — input schema, no producer yet) —
  `FigureRegion`/`FigureLane`/`LaneSample`/`LaneLevel`/`FigureAnnotation`/
  `BoundingBox`: the typed input the upstream PDF pipeline must produce when it
  classifies a `VisualAsset` as a timing diagram. Exercised via the waveform
  adapter's end-to-end smoke test; raster/vector decoding is a deferred future
  tree.
- **`cve.rs`** (1,011 ln, 23 tests) — constrained, verified extraction (the
  continuous crux). `actor_contract_json_schema_summary()` is the provider-facing
  schema; `parse_constrained_contract()` is the **fails-closed serde-authoritative
  adapter**; `entailment_check()`/`apply_entailment_to_contract()` is the
  conservative lexical/structural verifier (every signal must appear; every bound
  must match a digit run; never silently `Pass`). `instantiate_template()` plus
  the `ProtocolTemplate` library (ReadyValidHandshake/CreditFlowControl/
  SetupAccess/AsyncAssertSyncReleaseReset/BurstLast) is match-grounded.
  `voi_score()`/`select_top_n_by_voi()` is the deterministic uncertainty-driven
  converge selector.

### Cross-cutting wiring + the three structural honesty doctrines
`SemanticIr::build` runs the producers in this order (semantic.rs ~268–287):
`temporal_rules → contract_from_temporal_rule` → **`apply_fusion` (first)** →
**`apply_fidelity_gates` (second, so gates see fused contracts)**. The three
load-bearing honesty doctrines are now **structural, not authorial** — a
`Lowerable` contract that fails is mechanically demoted to a diagnostic-bearing
`Residual` *before* the `.isf` adapter can consume it, so fabrication is
prevented end-to-end:
- **fidelity Fail → Residual** — `reason="fidelity:<Gate>: <message>"`
  (semantic.rs ~2856–2862).
- **fusion disagreement → Residual** — `reason="disagreement: <sorted fields>"`
  (fusion.rs ~171).
- **entailment Fail → Residual** — `reason="entailment fail: …"`
  (cve.rs ~291–293).
The surface is carried by three additive fields on both `SemanticIr` and
`IntentIr` — `actor_contracts`, `fidelity_findings`, `protocol_graph` — each
`#[serde(default, skip_serializing_if = …)]`, so on-disk IR artifacts are
**byte-identical until extraction populates them** (zero artifact churn; the
CONTRACT-IR.2 / KG-ONTOLOGY.2 discipline). On the real (nvme) corpus the R16
surface is honestly dormant: `fidelity fail=0 score=1.000`, `fusion
groups_merged=0 disagreements=0`, `protocol_graph` all-zero, `waveform
figure_contracts=0`, `constrained 0/0/0` — the producers are load-bearing for
any *future* contract-producer drift, not faked passes.

### Verified quality state
- `cargo test -p specforge --lib`: **1128 passed, 0 failed** (370s). HEAD meets
  the signoff bar at the lib-test layer. (The full `scripts/run_ci.sh` gate —
  fmt/clippy-`-D`/tests-`-D`/rustdoc/mdBook — is exercised on the
  book-touching slices in this session.)
- 67 of the 1128 tests live inside the seven R16 modules
  (8+5+9+9+13+0+23).

### Risk / steering picture
- **Healthiest area:** the typed-target + honesty-enforcement spine is complete,
  unit-tested, and CI-parity-proven against the prior temporal lowering. The
  `.isf` adapter consumes `actor_contracts` with a pre-ContractIR fallback.
- **The real frontier is upstream of this subsystem, and is genuinely blocked:**
  the thesis crux (accurate prose+waveform → typed-KG extraction) needs
  (a) a PDF→`FigureRegion` raster/vector extractor and (b) an integrated
  prose LLM/VLM provider feeding `parse_constrained_contract`. Both are
  honestly-deferred future trees, not re-opened leaves. Until they exist, the
  R16 surface stays dormant-but-verified by design.
- **No architectural smell introduced:** R16 held the additive/serde-skip
  discipline throughout, so there is no artifact-churn or back-compat debt.

### Recommended implementation direction
1. Close the `R16-INTENT-CAPTURE` umbrella governance leaf (`.2`): all six
   sub-trees are `done`, so its acceptance ("program closed when all six done")
   is met; reconcile the stale tree file + ROADMAP R16 "promote/execute" text +
   live docs, and record the two deferred future trees explicitly.
2. After that, the task-tree frontier is exhausted; remaining roadmap crux work
   is blocked on the two upstream extractors above. Favor honest reporting of
   that blocker over inventing low-value churn; any unblocked work is incremental
   hardening (e.g. additional `kg-bench` fixtures, mutation-test backfill) rather
   than new capability.

## Session update (2026-05-18 ISF-ONLY-CONSOLIDATION — HDL + `.fsm` removed; `.isf` is the sole adapter)

Major architecture change (user-authorized `ISF-ONLY-CONSOLIDATION` batch).
SpecForge now emits only `.isf`; FSMGen owns scheduling/`.fsm`/HDL downstream.

- **Adapter layer collapsed to ISF.** `crates/specforge/src/ir/adapters.rs`
  went from **28,113 → 575 lines**: it is now just the shared adapter
  scaffolding (`AdapterTarget` = `{Isf}`, `AdapterStatus`, `AdapterPlan`,
  `AdapterLoweringStatus`, `AdapterArtifact` + impl, `AdapterArtifactLayout`,
  `AdapterIdentity`, `IsfAdapterArtifact`), the 5 ISF lowering fns + the
  renderability-policy comment, `canonicalize_existing_path`, `StageProbe`,
  and a 3-test ISF module. All FSM lowering (`FsmAdapterArtifact`, ~25
  `Fsm*` structs, `build_fsm_adapter_artifact` + ~150 helpers,
  `validate_system_*_renderability`) and ~160 FSM tests are gone.
- **Type surface narrowed.** `AdapterArtifact.fsm` removed; `IrStage` no
  longer has `FsmAdapter`; `AdapterTargetArg` = `{Isf}`.
  `validate.rs` lost `validate_fsm_adapter`/persist/fingerprint + dispatch;
  `project_validation.rs` lost all 25 FSM `IrStage` match sites;
  `converge.rs` `AdapterSnapshot` now tracks ISF metrics; `cli.rs`
  `--target` defaults to `isf`.
- **IR module count: 7** (`isf_ir.rs` is the typed ISF IR; `adapters.rs`
  is now effectively the ISF adapter + shared scaffolding). Whole
  `crates/specforge/src` ~110K → ~82K lines.
- `subs/fsmgen` submodule and `isf_output_passes_fsmgen_strict_validation`
  retained — FSMGen is the downstream `.isf` consumer.
- Lib test count dropped sharply (~160 FSM adapter tests removed); the
  exact post-removal count is recorded in CHANGES/MEMORY for this slice.
- `scripts/run_ci.sh` green (clippy/fmt/tests/rustdoc/mdBook). Recommended
  direction: `.5` retires FSM-adapter-only kg fixtures, `.6` sweeps the
  mdBook, `.7` reconciles remaining live docs + closes the tree.
- Batch outcome (closed 2026-05-18): all 7 `ISF-ONLY-CONSOLIDATION`
  leaves done and pushed. `.5` audit found no FSM-adapter-specific kg
  fixtures (kg-bench exercises the IR pipeline, not the adapter); `.6`
  swept 11 mdBook pages to ISF-only; `.7` reconciled README/ROADMAP
  current-state and closed the tree. No further Rust architecture change
  since `.4`; `crates/specforge/src/ir/adapters.rs` is the ISF adapter +
  shared scaffolding (575 lines), `isf_ir.rs` is the typed ISF IR.
  Final `scripts/run_ci.sh` green; 1040 lib tests.

## Session update (2026-05-17 R6-ISF-ADAPTER batch — ISF ownership backfilled)

- The ISF adapter is no longer untracked: `R6-ISF-ADAPTER` (lane R6, authorized
  5-leaf batch) now owns it. Leaf `.1` (docs only, no architecture change)
  registered the tree and recorded the ISF adapter across CHANGES /
  DEVELOPMENT_NOTES / LIVE_ACHIEVEMENT_STATUS / ROADMAP (R6 `.isf` status).
- Supersedes the "Process / continuity gap" note in the entry below: the
  task-tree-ownership and root-doc-sync gap is being remediated; the
  recommended-direction items there (`IrStage::IsfAdapter`, `isf_ir.rs` unit
  tests, renderability-policy decision) are now scoped as leaves `.2`–`.4`.
- Architecture/metrics unchanged by `.1`: still 7 IR modules, ~110K lines,
  1191 lib tests, `scripts/run_ci.sh` green.
- Leaf `.2` (code): `IrStage` now has an `IsfAdapter` variant
  (`crates/specforge/src/ir/mod.rs`); `build_isf_adapter_artifact` tags ISF
  artifacts with it instead of `IrStage::FsmAdapter`. New public-ish surface
  in `validate.rs`: `validate_isf_adapter` / `persist_isf_adapter_validation`
  / `isf_adapter_fingerprint`, and a dedicated `IrStage::IsfAdapter` arm in
  the `specforge validate` stage dispatch (ISF artifacts no longer misroute
  into the FSM validator). All 25 exhaustive `IrStage` matches in
  `project_validation.rs` updated; ISF behaves in parallel with FSM as a
  terminal adapter stage. +1 test (1192 lib tests); `scripts/run_ci.sh` green.
- Leaf `.3` (tests): `isf_ir.rs` now carries a `#[cfg(test)] mod tests`
  (9 tests) — closes the "0 unit tests of its own" gap flagged in the
  bootstrap entry below. Coverage targets the emitter (`render` /
  `render_txn_step`) and pure helpers (`sanitize_isf_name`,
  `sanitize_rule_condition`, `render_isf_binary_operator`,
  `render_isf_width_hint`, `branch_predicate_guard`). `from_intent_ir`
  remains integration-covered (43-field `IntentIr`; hand-construction
  rejected as brittle). 1201 lib tests; `scripts/run_ci.sh` green.

## Session update (2026-05-17 bootstrap re-analysis — ISF adapter landed, signoff regression detected)

State verified directly from the working tree at commit `490e6aed` (HEAD), not inherited from prior notes.

### Architecture delta since the 2026-05-15 snapshot
- A new 7th IR module exists: `crates/specforge/src/ir/isf_ir.rs` (~1.1K lines). It is a typed `.isf` lowering IR with the pipeline `IntentIR -> IsfIr::from_intent_ir() -> IsfIr -> IsfIr::render() -> .isf text`. It deliberately uses type-level invariants (e.g. `BTreeSet<IsfSignal>` dedup, non-optional `IsfReset`) to make malformed S-expression output unrepresentable.
- The `.isf` adapter is now fully wired end to end: `cli.rs` exposes `--target isf`; `adapters.rs` carries `AdapterTarget::Isf`, `IsfAdapterArtifact`, `build_isf_adapter_artifact`, `assess_isf_renderability`, and `derive_isf_actor_name`; `adapt.rs` prints the ISF artifact surface.
- IR source files now total ~70K lines: `adapters.rs` ~28.1K, `semantic.rs` ~21.4K, `evidence.rs` ~10.8K, `intent.rs` ~5.1K (grew from ~3.3K), `prior_memory.rs` ~2.1K, `source.rs` ~1.6K, `isf_ir.rs` ~1.1K. Whole `crates/specforge/src` ~110K lines.
- `IrStage` still has no `IsfAdapter` variant; `build_isf_adapter_artifact` tags ISF artifacts with `stage: IrStage::FsmAdapter`. This is a modeling smell worth tracking when ISF validation is formalized.
- `isf_ir.rs` carries 0 unit tests of its own. ISF coverage is via 3 adapters.rs integration tests, including `isf_output_passes_fsmgen_strict_validation`, which runs the real pinned `subs/fsmgen/bin/fsmgen --strict --check --json` against generated `.isf` text.

### Verified quality state (signoff regression)
- `cargo test -p specforge --lib`: **1191 passed, 0 failed** (the 2026-05-15 note's "1014" is stale; R7-VALIDATION + ISF added the rest).
- `cargo fmt --all --check`: **FAILS** — unformatted diffs in `validate.rs`, `adapters.rs`, `intent.rs` (extensive), and `isf_ir.rs`.
- `cargo clippy -p specforge --all-targets -- -D warnings`: **FAILS — 25 errors** (intent.rs 14, isf_ir.rs 5, nlp_enrich.rs 4, adapters.rs 2; kinds: `contains` vs `iter().any()` x7, redundant closure x7, collapsible `if` x6, needless borrow x2, map-keys iteration x1, immediate deref x1, push-after-creation x1).
- Consequence: the canonical CI entrypoint `scripts/run_ci.sh` (warning-deny clippy + fmt-check + warning-deny tests + rustdoc + mdBook) would **reject HEAD**. The project's non-negotiable signoff bar is currently not met at `main`.

### Process / continuity gap
- The ISF adapter landed across commits `bfe4f973` -> `4aa730cb` -> `48f04ee7` -> `490e6aed` with **no owning task tree** (none in `docs/TASK_TREE.md` or `docs/tasks/`), contrary to the task-tree-ownership doctrine.
- COMMIT.md live-doc sync was skipped for that work: `CHANGES.md`, `DEVELOPMENT_NOTES.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `MEMORY.md`, `ROADMAP.md`, and (until this entry) `RUST_CODEBASE_ANALYSIS.md` do not describe the ISF adapter. `MEMORY.md`'s "latest committed baseline" still points at `4f2eb367`.
- All 10 task-tree frontiers are exhausted; the only open leaf, `R7-VALIDATION.5`, is explicitly `deferred` (gated on a future canonical-IR-mutation decision). No eligible PNT leaf exists.

### Recommended implementation direction
1. Restore signoff first: clear all 25 clippy errors and `cargo fmt` the four affected files until `scripts/run_ci.sh` is green. This is remediation of a regression and should be its own task-tree-owned slice.
2. Backfill an ISF adapter task tree so the already-landed and remaining `.isf` work has ownership, then re-sync the skipped live docs.
3. Add unit tests inside `isf_ir.rs` itself (currently only integration-covered) and decide whether `IrStage` needs an `IsfAdapter` variant before ISF validation is formalized under R7.

## Session update (2026-05-15 live-doc sync after task tree closures)
- All 8 task trees are now closed (`done`). R6-SEMANTIC-HARDENING (leaves .22–.30) and R6-PRIOR-MEMORY-HARDENING (leaves .1–.7) were the last to close.
- 1014/1014 Rust tests passing (up from 666 — the hardening lanes added ~348 tests through mutant catching and unit test backfill).
- R16 (HDL lowering) removed from roadmap — SystemVerilog/Verilog/VHDL lowering now explicitly out of scope for SpecForge.
- Architecture: 6 IR source files totaling ~66K lines (`adapters.rs` 27.5K, `semantic.rs` 21K, `evidence.rs` 10.5K, `intent.rs` 3.3K, `prior_memory.rs` 2.1K, `source.rs` 1.6K).
- 23 commits ahead of origin/main. Near push checkpoint (~30).
- Next: resume PNT from roadmap remaining items — R7 (validation), R15 (actor-relative direction), R15b (temporal model), R15c (KG-guided rescans), R15d (evidence arbitration), R15e (KG-quality benchmarks).

## Session update (2026-05-14 task-tree tracking system scaffolding)
- Adopted task-tree tracking workflow from fsmgen. Documentation/workflow-only change — zero Rust code changes.
- Architecture, risk profile, and subsystem boundaries unchanged.
- 666/666 tests remain passing (no code changes).

## Session update (2026-05-14 evidence.rs EvidenceLink provenance ID hardening)
- Added `from_evidence_span_id` and `to_visual_evidence_id` assertions to the evidence links test. Previously untested. Zero production changes.
- 666/666 tests passing.

## Session update (2026-05-14 learn_priors.rs supporting_document_keys hardening)
- Added `supporting_document_keys` assertions (8 assertions across 3 tests) covering all 7 prior record types.
- The field links harvested priors back to the documents they were learned from. Previously untested. Zero production changes.
- 666/666 tests passing.

## Session update (2026-05-14 evidence.rs table_signal_declaration_provenance hardening)
- Added `table_signal_declaration_provenance` assertions (3 assertions across 3 tests) to evidence.rs signal-description table tests.
- The field links synthesized signal declarations back to their SourceIR table origin. Previously untested. Zero production changes.
- 666/666 tests passing.

## Session update (2026-05-14 intent.rs supporting_actor_ids hardening)
- Added `supporting_actor_ids` assertions (2 assertions across 2 tests) to intent.rs actor tests.
- The field is populated in `build_intent_actors` and was previously untested. Zero production changes.
- 666/666 tests passing.

## Session update (2026-05-14 hardening — pass-through field regression)
- Four hardening lanes completed: intent.rs temporal pass-through (12 assertions), learn_priors.rs strongest_automation_confidence (7 assertions), semantic.rs supporting_table_ids (8 assertions), semantic.rs supporting_visual_evidence_ids (1 assertion).
- All work is regression-only: test assertion additions, zero production behavior changes. Architecture, risk profile, and subsystem boundaries unchanged.
- 666/666 tests passing. Branch pushed to origin.
- Next PNT target: continue hardening remaining test assertion gaps in the IR/adapter surface.

## Session update (2026-05-13 `.fsm` root_kind_decision lane complete)
- All fsm-adapter root_kind_decision gaps closed. Every test family proves root_kind_decision: standalone DT, standalone sequential DT, structured FSM, explicit module, DT-centric FSM, top composition (37/37).
- 108 `selected_root_kind` assertions across 89 fsm test functions (some check both fsm-level and child-candidate root_kind_decision).
- 666/666 tests passing. Lane closed. Next PNT target TBD.

## Session update (2026-05-13 `.fsm` standalone DT root_kind_decision complete)

## Session update (2026-05-13 `.fsm` explicit module root_kind_decision complete)
- Added root_kind_decision to the 5 remaining explicit module tests (2 renderable + 3 blocked). All 6 now carry root_kind_decision — zero gaps.
- 666/666 tests passing; ~28 commits ahead of origin/main (push threshold ~30).
- Next targets: remaining standalone DT (13 tests), top composition (21 tests).

## Session update (2026-05-13 `.fsm` structured FSM root_kind_decision complete)
- Added root_kind_decision to all 3 structured FSM tests (1 renderable + 2 blocked). FSM-root pattern: Fsm/[Top]/High/"carries explicit regular-state facts".
- 666/666 tests passing; 25 commits ahead of origin/main.
- Next targets: explicit-module DT (5 tests), remaining standalone DT (13 tests), top composition (21 tests).

## Session update (2026-05-13 `.fsm` standalone sequential DT root_kind_decision complete)
- Added root_kind_decision to the 5 remaining standalone sequential DT tests (4 renderable + 1 blocked). All 8 sequential DT tests now carry root_kind_decision — zero gaps.
- 666/666 tests passing; 24 commits ahead of origin/main.

## Session update (2026-05-13 `.fsm` root_kind_decision for sequential DT blocked tests)
- Added root_kind_decision to 3 standalone sequential DT blocked tests. 666 all passing.

## Session update (2026-05-13 `.fsm` root_kind_decision confidence — standalone DT blocked tests)
- Completed root_kind_decision contract for 3 standalone DT blocked tests: added automation_confidence (Medium) and rationale checks.
- The Rust test listing remains `666` tests, all passing.

## Session update (2026-05-13 `.fsm` root_kind_decision for standalone DT blocked tests)
- Added `root_kind_decision` assertions to 3 standalone DT blocked tests, proving the selected root kind stays `Dt` with `Fsm` and `Top` deferred.
- Many tests still lack root_kind_decision coverage; this category is large (~40 tests).
- The Rust test listing remains `666` tests, all passing.

## Session update (2026-05-13 `.fsm` renderable_module.is_none() — all blocked tests complete)
- Added missing `renderable_module.is_none()` and `!blocking_reasons.is_empty()` guards to the last 2 blocked tests lacking them.
- All blocked tests now prove the complete stale-output prevention contract.
- The Rust test listing remains `666` tests, all passing.

## Session update (2026-05-13 `.fsm` renderable-document presence — all 12 tests complete)
- Added `assert!(fsm.renderable_document.is_some())` to the final 5 standalone recovery tests, closing the renderable-document presence gap.
- All 12 renderable tests now prove both `renderable_module` and `renderable_document` existence (or use helper functions that cover both).
- The Rust test listing remains `666` tests, all passing.

## Session update (2026-05-13 `.fsm` renderable-document presence — builds_renderable DT complete)
- Closed remaining gap: all 6 builds_renderable DT tests now prove `renderable_module.is_some()` and `renderable_document.is_some()`.
- Added standalone DT document presence, and both module + document presence for standalone sequential DT and compound update DT tests.
- ~5 standalone recovery tests remain without renderable_document presence assertions.
- The Rust test listing remains `666` tests, all passing.

## Session update (2026-05-13 `.fsm` renderable-document presence — 3 builds_renderable DT tests)
- Added `assert!(fsm.renderable_module.is_some())` and `assert!(fsm.renderable_document.is_some())` to 3 builds_renderable DT tests that previously had no output artifact presence checks.
- Tests: symbolic DT, selector-based DT, computed-selector DT. Each now proves both the renderable module and source document exist alongside the renderability status.
- ~8 more renderable tests (both builds_renderable and standalone) still lack renderable_document presence assertions.
- The Rust test listing remains `666` tests, all passing.

## Session update (2026-05-12 `.fsm` dt_candidate.automation_confidence — gap closed)
- Added `dt_candidate.automation_confidence` (`AutomationConfidence::Medium`) to `builds_blocked_dt_centric_fsm_adapter_artifact`, closing the last remaining decision tree candidate confidence gap.
- Production code at lines 2199-2203: non-explicit DT candidates get `Medium` when `!behaviors.is_empty()`, `Low` when `behaviors.is_empty()`. The handshake fixture has behaviors → Medium.
- All tests with `decision_tree_candidates` now prove `automation_confidence` on extracted candidates.
- The Rust test listing remains `666` tests, all passing.

## Session update (2026-05-12 `.fsm` top composition remaining fsm-level blocking_reasons)
- 3 more complete; ~21 of 24 done with ~3 remain. Rust test listing `666` all passing.

## Session update (2026-05-12 `.fsm` top composition direction-role fsm-level blocking_reasons)
- 3 more top composition tests now complete; 18 of 24 done, ~6 remain.
- The Rust test listing remains `666` tests, all passing.

## Session update (2026-05-12 `.fsm` top composition fsm-level blocking_reasons continued)
- 3 more top composition tests now have complete fsm-level renderability contracts; 12 of 24 done, 12 remain.
- The Rust test listing remains `666` tests, all passing.

## Session update (2026-05-12 `.fsm` top composition link_from fsm-level blocking_reasons)
- Added fsm-level `blocking_reasons.is_empty()` and `renderable_module.is_none()` to 3 more top composition tests (link_from category).
- 9 of 24 top composition tests now complete; 15 remain.
- The Rust test listing remains `666` tests, all passing.

## Session update (2026-05-12 `.fsm` top composition conflicting-evidence fsm-level blocking_reasons)
- Added fsm-level `blocking_reasons.is_empty()` and `renderable_module.is_none()` to 3 additional top composition tests (conflicting top-port direction, width, child-link width).
- 6 of 24 top composition tests now have fsm-level blocking_reasons; 18 remain.
- The Rust test listing remains `666` tests, all passing.

## Session update (2026-05-12 `.fsm` top composition keeps fsm-level blocking_reasons)
- Added fsm-level `blocking_reasons.is_empty()` and `renderable_module.is_none()` assertions to the three keeps top composition tests in `crates/specforge/src/ir/adapters.rs`.
- These tests previously checked blocking only at the `top_candidate` level; they now prove the aggregate fsm-level contract.
- 21 of 24 top composition tests still need fsm-level blocking_reasons checks; the 3 keeps tests are now complete.
- The Rust test listing remains `666` tests, all passing.

## Session update (2026-05-12 `.fsm` remaining enrichment diagnostics closed — zero gaps)
- Added `fsm.renderability.required_canonical_enrichments` assertions to the final 7 tests with `blocking_reasons` but no enrichment checks, closing all remaining diagnostic gaps.
- Tests span four categories: standalone DT (4 tests), sequential DT (1), structured FSM (2), and top composition (1).
- Three tests also received missing `renderable_module.is_none()` companion guards, completing the full renderability contract.
- Enrichment diagnostic coverage is now 100%: all 68 tests that check `blocking_reasons` also prove `required_canonical_enrichments`.
- The Rust test listing remains `666` tests, all passing.

## Session update (2026-05-12 `.fsm` remaining standalone DT enrichment diagnostics)
- Added `fsm.renderability.required_canonical_enrichments` companion assertions to four remaining standalone DT blocked tests in `crates/specforge/src/ir/adapters.rs`.
- Tests now cover four distinct enrichment categories: output alignment, interface inventory promotion, graph direction resolution, and width resolution.
- The Rust test listing remains `666` tests, and the adapter-filtered suite reports `143/143` passing tests.
