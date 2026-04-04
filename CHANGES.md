# CHANGES

## 2026-04-04 (steering note: multimodal semantic recovery)

### Changed: implementation guidance now explicitly centers multimodal semantic recovery
- Logged the current extraction philosophy in `DEVELOPMENT_NOTES.md` as a steering principle for future work.
- The note makes the target explicit: recover enough grounded intent from tables, figures, and prose for downstream RTL/verification generation rather than treating PDF parsing as the end goal.
- It also records the preferred tactic for future hurdles: use the KG as a search index for repeated rescans, keep the pipeline provenance-first, and prefer reusable document-native lifting strategies over speculative adapter-side inference.

## 2026-04-04 (full-converge defaults + AXI convergence stabilization)

### Changed: `specforge converge` now defaults to the full Ollama-backed loop
- `ConvergeArgs` now default `--vlm-provider` and `--nlp-provider` to `ollama` instead of `skip`.
- The intended default pipeline path is now encoded in the CLI itself: figure enrichment and NLP Level 3 run automatically during `specforge converge` unless the caller explicitly opts out.

### Fixed: monotone knowledge accounting no longer treats fewer residuals as less knowledge
- `crates/specforge/src/commands/converge.rs` no longer counts downstream adapter residual work toward `knowledge_fact_count`.
- This fixes the false `pipeline knowledge shrank` failure mode seen on a full AXI rerun, where later passes correctly reduced residual decisions but the old accounting treated that as regression.
- `EvidenceIr` and `specforge nlp-enrich` now also normalize duplicate loopback NLP records before persistence so repeated identical extractions do not inflate later passes.

### Validation
- Added regression tests:
  - `dedup_loopback_records_removes_duplicate_constraints_and_rules`
  - `nlp_enrich_dedups_duplicate_extractions_before_persisting`
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 110/110 passed
- Full original-PDF AXI converge:
  - `cargo run -p specforge -- converge /Users/richarddje/Documents/livework/chipdoc/arm/amba/core/axi/current/IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf --target fsm --max-iterations 5 --vlm-provider ollama --vlm-model qwen2.5vl:7b --nlp-provider ollama --nlp-model qwen2.5vl:7b` → converged in 2 passes
- Refreshed projected AMBA validation snapshot:
  - APB `IHI0024_D`: 95/100 EXCELLENT
  - AHB `IHI0033_C`: 95/100 EXCELLENT
  - AXI `IHI0022_L`: 94/100 EXCELLENT
  - `cargo run -p specforge -- project-validation generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json` → passed

## 2026-04-04 (validation snapshot projection into tracked live docs)

### Added: deterministic live-doc projection for persisted validation reports
- Added `specforge project-validation <artifact>...`.
- The new command validates each passed IR artifact, reuses the persisted `validation_reports`, writes a tracked `VALIDATION_SNAPSHOT.md`, and refreshes the managed validation projection block in `LIVE_ACHIEVEMENT_STATUS.md`.
- Projection output is deterministic: artifact ordering uses `document_key`, findings sort by severity/category/id, and repo-internal artifact paths are rendered as relative paths.

### Changed: staged validation continuity no longer depends on manual markdown edits
- `generated/` remains untracked, but validation snapshots can now be pulled back into tracked docs on demand after local APB/AHB/AXI or other validation runs.
- The live roadmap/status/docs now treat staged IR validation projection as implemented; remaining validation work is adapter-focused.
- Refreshed the tracked validation snapshot against the current AMBA `IntentIR` artifacts:
  - APB `IHI0024_D`: 95/100 EXCELLENT
  - AHB `IHI0033_C`: 95/100 EXCELLENT
  - AXI `IHI0022_L`: 89/100 GOOD

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 107/107 passed
- `cargo run -p specforge -- project-validation generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json` → passed

## 2026-04-04 (validation back-annotation on IR artifacts)

### Added: persisted validation reports for the four IR stages
- `specforge validate <artifact>` now writes a deterministic `validation_report.json` sidecar next to `SourceIR`, `EvidenceIR`, `SemanticIR`, and `IntentIR` artifacts
- the same validation report is now backannotated into the artifact itself via a `validation_reports` field
- `SemanticIR` / `IntentIR` validation findings now include graph-aware checks for missing producers, missing consumers, and compatibility-surface lag relative to the actor-relative KG

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 106/106 passed

## 2026-04-04 (actor-relative KG carry-through into SemanticIR / IntentIR)

### Added: downstream preservation of the structural knowledge graph
- `SemanticIR` now preserves the extracted actor-signal graph via:
  - `actor_signal_relations`
  - `actor_ports`
  - `signal_connectivity`
- `IntentIR` now carries the same actor-relative KG surface forward as canonical output instead of forcing downstream consumers to rediscover relation evidence from `EvidenceIR`
- actor records now preserve grounded actor names when relation evidence makes them explicit

### Changed: validation now surfaces KG-native counts
- `specforge validate` now reports actor-signal relation, actor-port, and signal-connectivity counts for `SemanticIR` and `IntentIR`
- flat `direction_hint` fields remain as a compatibility surface, but they are no longer the only downstream representation of signal direction semantics

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 104/104 passed

## 2026-04-04 (continuity sync + local validation snapshot)

### Changed: live continuity docs now reflect the current post-converge state
- Updated the live documentation surface so crash recovery and handoff notes match the current repository status after the converge/VLM work.
- `generated/` is now treated as a local artifact root only: artifacts still materialize there, but the directory is git-ignored and no longer versioned.

### Validation
- `cargo test --manifest-path Cargo.toml` → 102/102 passed
- Current local validation snapshot:
  - APB `IHI0024_D`: 95/100 EXCELLENT after a full original-PDF `specforge converge` run with Ollama VLM + NLP Level 3; converged in 2 passes
  - AHB `IHI0033_C`: 95/100 EXCELLENT from the current local `IntentIR` snapshot
  - AXI `IHI0022_L`: 89/100 GOOD from the current local `IntentIR` snapshot
- Current AXI caveat:
  - the local AXI `SourceIR` has 20 timing diagrams classified, but the current local artifact still lacks persisted VLM timing enrichment, so timing remains the most obvious remaining score gap

## 2026-04-03 (whole-pipeline converge command + preserved loopback knowledge)

### Added: `specforge converge`
- Introduced a new top-level `converge` command that materializes the staged pipeline as a fixed-point loop instead of a one-shot chain.
- The command ingests a source once, optionally re-runs VLM figure enrichment and NLP Level 3 backannotation, rebuilds `EvidenceIR`, `SemanticIR`, `IntentIR`, and the selected adapter artifact, and stops when the persisted knowledge snapshot is unchanged between passes.
- The snapshot currently covers the staged artifacts that already persist facts today: `SourceIR`, `EvidenceIR`, `SemanticIR`, `IntentIR`, and adapter artifacts.

### Changed: `EvidenceIr::build()` now preserves prior loopback knowledge across rebuilds
- When rebuilding from the same persisted `SourceIR`, `EvidenceIr::build()` now carries forward:
  - learned signal aliases,
  - NLP-upgraded `ExtractedStatement` classes,
  - prior `SignalConstraintRecord`s,
  - prior `ConditionalRuleRecord`s.
- This closes the architectural gap where pass `N+1` could previously forget what `nlp-enrich` discovered in pass `N`.

### Changed: `specforge enrich` is now idempotent for already-enriched figures
- Timing/state-machine diagrams whose `VisualAsset.note` already contains a VLM extraction payload are skipped on later passes.
- This keeps multi-pass orchestration from re-querying the same diagram needlessly.

### Validation
- Added regression tests:
  - `converge_rebuilds_pipeline_until_snapshot_stabilizes`
  - a shared process-global test env lock now serializes `SPECFORGE_VLM_HELPER` mutations across convergence/NLP tests
- `cargo test --manifest-path Cargo.toml` → 100/100 passed

## 2026-04-03 (nlp-enrich alias marker filter + README staged-flow validation)

### Fixed: `extract_alias_phrase()` in `crates/specforge/src/commands/nlp_enrich.rs`
- Alias learning now rejects subject phrases that begin with markdown/table markers `-`, `|`, or `#`.
- This closes the remaining Form 2 cleanup gap where bullet rows, table cells, or heading-prefixed text could otherwise be learned as garbage aliases such as `- the address`.
- Ordinary prose alias learning remains unchanged for real noun phrases such as `address bus`.

### Validation
- Added regression test:
  - `extract_alias_phrase_rejects_markdown_marker_prefixes`
- `cargo test --manifest-path Cargo.toml` → 99/99 passed
- Re-executed the documented README entry flow on `README.md`:
  - `cargo run -p specforge -- --help`
  - `cargo run -p specforge -- inspect README.md`
  - `cargo run -p specforge -- ingest README.md --dry-run`
  - `cargo run -p specforge -- ingest README.md`
  - `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json --dry-run`
  - `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json`
  - `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json --dry-run`
  - `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json`
  - `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json --dry-run`
  - `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json`
  - `cargo run -p specforge -- adapt generated/intent_ir/readme/intent_ir.json --target fsm --dry-run`
- Materialized README artifacts now validate the staged flow end-to-end:
  - SourceIR: 0 page artifacts, 0 visual assets
  - EvidenceIR: 15 section anchors, 190 evidence spans, 190 extracted statements
  - SemanticIR: 2 actors, 6 phases, 5 invariants, 8 gates
  - IntentIR: 2 actors, 14 behaviors, 6 constraints, 1 assumption

## 2026-04-03 (convergent EvidenceIR enrichment + refreshed APB/AHB/AXI artifacts)

### Added: monotone convergent extraction loop in `evidence.rs`
- Replaced the one-shot post-table extraction tail in `EvidenceIr::build()` with `converge_evidence_extractions()`.
- Each pass now:
  - seeds from table-derived signal/enum facts,
  - rescans signal-anchored encoding tables,
  - collects newly discovered enum/value atoms,
  - extracts additional prose value constraints,
  - refines asserted/deasserted constraints with active-low/high polarity prose,
  - synthesizes KG-derived direction declarations,
  - repeats until no new synthesized statements appear.
- Rationale: the previous build order could synthesize useful `Enum ...` facts and then end the build before later prose extraction had a chance to reuse them in the same build.

### Added: anchored encoding-table recovery without hardcoded protocol value lists
- New helpers in `crates/specforge/src/ir/evidence.rs`:
  - `scan_encoding_tables_by_signal_anchor()`
  - `collect_discovered_enum_values()`
  - `extract_discovered_state_value_from_text()`
  - `extract_signal_polarity_from_prose()`
  - `apply_signal_polarity_to_constraints()`
  - `extract_dynamic_signal_constraints()`
  - `dedup_actor_signal_relations()`
- `synthesize_encoding_declarations()` now delegates to `synthesize_encoding_declarations_for_enum()` so the same encoding synthesis logic can be reused by the convergence loop.
- No new APB/AHB/AXI-specific enum/value list was added; discovered values come from extracted tables and synthesized `Enum ...` statements only.

### Changed: `classify_table_kind()` in `docling_backend.rs`
- Signature widened to `classify_table_kind(header_rows, body_rows=None, caption_text=None)`.
- Table classification now uses caption cues, header cues, and first-column body content together instead of headers alone.
- Added content-based encoding detection for weakly labeled tables by scanning body rows for binary/hex literals and bit-field references such as `HTRANS[1:0]`.
- The call site now passes `body_rows` into the helper so the classifier can use real cell content.

### Validation
- Added regression tests:
  - `anchored_encoding_scan_unlocks_dynamic_value_constraint_extraction`
  - `prose_polarity_refines_asserted_constraint_kind`
- `cargo test --manifest-path Cargo.toml` → 98/98 passed
- `cargo build --release --manifest-path Cargo.toml` → passed
- Refreshed generated APB/AHB/AXI artifacts and revalidated the current `IntentIR` outputs:
  - APB: 90/100 EXCELLENT
  - AHB: 95/100 EXCELLENT
  - AXI: 90/100 GOOD
- Net score change from the earlier baseline:
  - APB: 75 → 90
  - AHB: 95 → 95
  - AXI: 90 → 90

## 2026-04-03 (broader timing_diagram classification for figure captions)

### Fixed: classify_diagram_kind() in docling_backend.rs Python helper
- Added a second pass to the timing_diagram check: for any asset whose caption
  contains "figure", also classify as timing_diagram when caption uses protocol
  execution vocabulary: "transfer", "transaction", "handshake", "burst",
  "exit from reset", "sequence diagram".
- Rationale: bus protocol specs name clocked waveform figures after the operation
  they depict. Explicit "timing" / "waveform" words are often absent. The check
  is intentionally inclusive; VLM handles borderline cases gracefully.
- Simulated impact on AXI after re-ingest:
  - timing_diagram: 2 → 20 (+18)
  - New: VALID/READY handshake waveforms, write/read transaction dependencies,
    atomic transactions, wrapping transfers, PCMO, snoop, sequence diagrams.
  - Non-timing figures (architecture, data structure, topology) stay unknown.
- 96/96 tests pass.
## 2026-04-03 (caption-gated signal_description classification in Docling ingest helper)

### Fixed: classify_table_kind() in docling_backend.rs Python helper
- Added `caption_text=None` parameter; caption is now checked BEFORE header-based classification.
- Root cause: protocol payload/message field tables share the Name|Width|Description
  header structure with interface signal tables but describe message payload fields,
  not hardware interface pins. Without a caption check they were misclassified as
  signal_description.
  - AXI "Table A15.3: DVM message fields" is the confirmed instance: VA, PA, ASID,
    ASIDV, VMID, VMIDV, DVMType are DVM message payload fields, not interface signals.
- Fix: `caption_is_payload` flag blocks signal_description when caption contains:
  "message field(s)", "payload field(s)", "packet field(s)", "command field(s)",
  "frame field(s)".
- Call site updated: `classify_table_kind(header_rows, caption_text)`.
- Impact (requires re-ingest to take effect):
  - AXI DVM message field table: signal_description → unknown
  - DVM fields removed from declared signal set
  - 82 legitimate AXI signal_description tables unaffected
  - Width coverage: 98% → 100% after re-ingest + pipeline re-run
- 96/96 tests pass
## 2026-04-03 (row-scan clock/reset detection, immune to Docling column-ordering bugs)

### Fixed: synthesize_system_contract_from_table_descriptions() (evidence.rs)
- Replaced column-index-based detection with a full row-scan that inspects every
  cell in each body row, independent of column position.
- Root cause: Docling mis-assigns body cells to wrong column buckets for tables with
  visually distinctive (bold/boxed) cells whose PDF span-count arithmetic shifts.
  AHB Table 2-1 "Global signals" is a confirmed instance: HCLK/HRESETn are in
  col 0 of the PDF but Docling places them in col 3 of the parsed grid.
- New per-row algorithm:
  - Signal candidate: cell with ≤2 whitespace tokens, first token is a valid
    hardware signal name (not a role word like CLOCK/RESET/SOURCE).
    Excludes description cells (many words) and role cells ("Clock source" etc.).
  - Clock/reset desc: scan all cells for keyword patterns; keep the longest
    matching text so "The bus clock times all bus transfers…" beats "Clock source",
    giving accurate polarity/kind inference for resets.
- Result: AHB system contract (HCLK + HRESETn) correctly detected.
- AHB score: 90/100 → 95/100 EXCELLENT (contract_bonus 0/5 → 5/5).
- APB and AXI scores unchanged (signal tables were already correct).
- 96/96 tests pass (no new tests needed — existing contract detection tests pass).
## 2026-04-03 (header-clue + positional column detection; AMBA 5 direction mapping)

### Fixed: synthesize_signal_declarations() — direction column semantics (evidence.rs)
- Split the single dir_col into three distinct column types with correct semantics:
  - `explicit_dir_col`: header contains "direction" → literal input/output cell value
  - `source_col`: header contains "source" or "driver" → cell names the DRIVING actor
    - Requester/Initiator/Master → output; Completer/Subordinate/Slave/Target/Responder → input
    - Clock/Reset/System-bus/Global → input (infrastructure distributed into all blocks)
  - `dest_col`: header contains "destination" → cell names the RECEIVING actor (inverted)
    - Signal flows TO Subordinate/Completer → output; flows TO Manager/Requester → input
- Fixes APB: "Source" column with values "Requester"/"Completer"/"Clock"/"System bus reset"
  was previously unrecognised → all APB signals silently dropped; now correctly mapped
- Fixes AHB test: "Destination" column with "Subordinate"/"Manager" values now uses
  inverted semantics (flowing TO Subordinate = output, not input)

### Fixed: name column now header-detected with positional fallback (evidence.rs)
- name_col: search headers for signal/name/port/pin; fall back to col 0 (leftmost)
- Previously hardcoded to row.first(); now honours header position when available

### Fixed: emit width-only declaration when direction is unknown (evidence.rs)
- Added (None, Some(WidthHint::Numeric)) and (None, Some(WidthHint::Parametric)) arms
- Signals with known width but no determinable direction now emit "Signal X is width N."
  instead of being silently dropped

### Fixed: infer_signal_direction_from_section() (evidence.rs)
- Added "requester" → output (AMBA 5 APB terminology)
- Added "completer"/"target" → input
- Added "reset" to the infrastructure group → input

### Fixed: synthesize_system_contract_from_table_descriptions() (evidence.rs)
- Replaced misleading comment "No header analysis needed" with header-first detection
- name_col: headers with signal/name/port/pin → else col 0
- desc_col: headers with description/desc → else last column

### Fixed: duplicate OKAY pattern in collect_subject_signal_tokens() (evidence.rs)
- Removed second OKAY from the exclusion match arm (compiler unreachable_patterns warning)

### Test suite: 96/96 pass (no change in count)
## 2026-04-03 (WidthHint: parametric widths + table width map for KG synthesis)

### New type: WidthHint (source.rs)
- Replaces Option<u32> for signal width throughout the IR
- WidthHint::Numeric(u32) — fixed bit width (1, 2, 32, 64, ...)
- WidthHint::Parametric(String) — user-configurable RTL parameter (ADDR_WIDTH, DATA_WIDTH/8, ...)
- Backward-compatible serde: Numeric(32) -> JSON 32, Parametric("ADDR_WIDTH") -> JSON "ADDR_WIDTH"
- Both variants count as "known width" in coverage metrics and scoring

### Changed: synthesize_signal_declarations() in evidence.rs
- Width parsing now returns Option<WidthHint> instead of Option<u32>
- Numeric: parse::<u32>() -> WidthHint::Numeric
- Parametric: non-numeric, non-empty, contains alphabetic -> WidthHint::Parametric
- No artificial upper bound on numeric widths (removed the w <= 1024 filter)
- Synthesized text includes parametric widths: "Signal PADDR is output width ADDR_WIDTH."

### New: collect_signal_widths_from_tables() in evidence.rs
- Extracts width (numeric or parametric) from signal-description table Width columns
- Passed to synthesize_directions_from_relations() so KG-synthesized declarations carry width

### Changed: synthesize_directions_from_relations() in evidence.rs
- Now accepts width_map: HashMap<String, WidthHint>
- Produces "Signal PADDR is output width ADDR_WIDTH." instead of just "Signal PADDR is output."

### Changed throughout: semantic.rs, intent.rs, adapters.rs
- InterfaceSignalRecord.width_hint: Option<u32> -> Option<WidthHint>
- ExplicitTopPortRecord.width_hint: Option<u32> -> Option<WidthHint>
- InterfaceSignalAccumulator.width_hint: Option<u32> -> Option<WidthHint>
- ParsedInterfaceSignalDeclaration.width_hint: Option<u32> -> Option<WidthHint>
- parse_width_token() -> returns Option<WidthHint> (both numeric and parametric)
- merge_signal_hint<T: Copy+Eq> -> <T: Clone+Eq> (WidthHint is Clone but not Copy)
- register_interface_signal_record() signature updated
- WidthCast in expression parser kept as u32 (literal numeric, not parametric)
- Adapters convert Option<WidthHint> -> Option<u32> via .as_numeric() for FSM emission

### validate.rs: display numeric vs parametric width breakdown
- "with_width: N (X%) [N numeric, N parametric]"
- Both numeric and parametric count in width coverage score

### Results
| Spec | Before | After | Change |
|------|--------|-------|--------|
| AHB | 88/100 GOOD, 80% width | 90/100 EXCELLENT, 100% width [10 num, 11 para] | +2 pts |
| APB | 60/100 ADEQUATE, 0% width | 70/100 GOOD, 100% width [10 num, 8 para] | +10 pts |
| AXI | 75/100 GOOD, 0% width | 85/100 GOOD, 99% width [126 num, 51 para] | +10 pts |
## 2026-04-03 (R13: Tier 2 Knowledge Graph — actor-signal relation extraction)

### New types (source.rs)
- RelationKind enum (Drives | Reads)
- ActorSignalRelation struct { relation_id, actor_name, signal_name, relation, source_statement_ids, automation_confidence }

### New EvidenceIR field (evidence.rs)
- actor_signal_relations: Vec<ActorSignalRelation> (serde default = empty; extracted at build time)

### Tier 2 extraction: two complementary sources
1. **Prose verb-pattern extraction** (extract_actor_signal_relations()):
   - Passive drives: "SIGNAL is {driven|asserted|returned|...} by ACTOR" and "...from ACTOR"
   - Active drives: "ACTOR {drives|asserts|returns|...} SIGNAL" and "ACTOR must {drive|...} SIGNAL"
   - Passive reads: "SIGNAL is {read|sampled|monitored|...} by ACTOR"
   - Active reads: "ACTOR {reads|samples|...} SIGNAL"
   - Only searches for confirmed signal names (from tables + declarations)
2. **Signal table Source column extraction** (extract_relations_from_signal_tables()):
   - Reads Source/Driver/Direction column from signal_description tables
   - APB "PADDR | Requester | ..." → (Requester, Drives, PADDR)
   - APB "PREADY | Completer | ..." → (Completer, Drives, PREADY)
   - AHB tables already have direction column (covered by existing synthesis)
   - AXI tables have no Source column (no triples from tables, only from prose)

### Signal name collection: two sources
- collect_signal_names_from_tables(): ALL first-column signal names from signal_description tables (regardless of whether direction was extracted — covers APB/AXI where Source column is non-standard)
- collect_known_signal_names(): Signal names from existing "Signal X is input/output" prose declarations
- Union of both used as the search universe for prose verb patterns

### Direction synthesis: non-conflicting
- synthesize_directions_from_relations(): creates "Signal X is output." for Drives triples
- Skips signals already declared from tables (table declarations are authoritative)
- KG synthesis only adds direction for signals that had NO prior table-derived declaration

### Updated Layer D (semantic.rs) and Layer E (validate.rs)
- Declared signal set now includes Medium confidence (KG-derived) signals in addition to High confidence (table-derived)

### Results after R13
| Spec | Before R13 | After R13 | Change |
|------|-----------|-----------|--------|
| AHB  | 86/100 GOOD     | 85/100 GOOD | -1 (21 vs 17 declared, 100% dir, 47% width) |
| APB  | 35/100 NEEDS IMP | 60/100 ADEQUATE | +25 pts, 18 declared signals, 100% dir |
| AXI  | 85/100 (misleading, 1 sig) | 75/100 GOOD (honest, 182 signals) | Honest |

### Tests: 6 new (90 → 96, all passing)
- passive_drive_pattern_extracts_actor_and_signal
- active_drive_pattern_extracts_actor_and_signal
- passive_read_pattern_extracts_actor_and_signal
- must_drive_pattern_extracts_actor_from_requester_sentence
- synthesize_directions_produces_signal_is_output_declaration
- kg_extraction_produces_graph_declarations_in_evidence_ir
## 2026-04-02 (AHB + APB end-to-end pipeline validation run)

### AHB (IHI0033_C) results — full feedback loop on existing SourceIR
- Layer A suppressed 13 boilerplate NormativeStatements (85 → 72 residuals before nlp-enrich)
- nlp-enrich Pass 1: 72 candidates → 26 extracted (13 signal + 13 conditional), Form 1 backannotated 26, 1 alias learned (low-quality: "- the address" from markdown table row)
- nlp-enrich Pass 2: 46 candidates → 0 extracted → convergence at residual=46 (pass 3 stable check)
- Layer D gating: 17 declared signals (100% direction, 58% width), 248 heuristic noise excluded
- Final score: **86/100 — GOOD** (signal_dir=25/25, width=5.8/10, constraints=30/30, enums=15/15, registers=5/5, timing=5/5)
- Residual NormativeStatements: 46 (architectural/infrastructure sentences with no named signal)

### APB (IHI0024_E) results — full ingest from PDF + feedback loop
- Ingested: 48 pages, 35 visual assets
- EvidenceIR: 517 statements, 18 NormativeStatements, 37 signal_constraints (Level 2)
- nlp-enrich Pass 1: 18 candidates → 8 extracted, Form 1 backannotated 8, 0 aliases learned
- nlp-enrich Pass 2: 10 candidates → 0 extracted → convergence at residual=10
- Layer D gating: **0 declared signals** — APB signal description tables not detected as SignalDescription kind, so no High-confidence records; direction/width coverage = 0%
- Final score: **35/100 — NEEDS IMPROVEMENT** (constraints=30/30, registers=5/5, all signal coverage zero)
- Root cause: APB table classification is returning Unknown instead of SignalDescription for the signal description tables → no synthesized "Signal X is input/output" statements → Layer D has no declared set → direction/width = 0 → score tank

### Issues identified
1. **APB signal table classification**: APB tables not being classified as SignalDescription; need to inspect APB structured_tables
2. **Alias extraction quality**: "- the address" alias from markdown table row prefix is garbage — need to filter phrases starting with "-" or pure markdown tokens
## 2026-04-02 (remove --max-passes: residual-stable convergence criterion)
- **Removed --max-passes CLI option** from NlpEnrichArgs: was a safety net that is no longer needed.
- **New convergence criterion**: loop stops when residual(N) == residual(N-1).  Termination is guaranteed because the residual pool is finite and can only decrease or stay flat (monotone).  The criterion covers Form 2 alias reclassifications AND LLM extractions together, unlike the previous "pass_extracted == 0" check which only counted LLM extractions and could stop prematurely.
- **Loop structure**:  replaced by  with pass counter for display only.  dry-run breaks after one pass.
- **All 90 tests updated**: removed max_passes field from all NlpEnrichArgs struct literals; convergence test comment updated to describe residual-stable criterion.
## 2026-04-02 (Form 2: signal alias learning feedback loop)
- **EvidenceIr.signal_alias_map** (evidence.rs): new BTreeMap<String,String> field (serde default = empty). Persisted to JSON so aliases accumulate across nlp-enrich runs.
- **apply_alias_reclassification()** (EvidenceIr pub method): applies accumulated alias map to re-classify remaining NormativeStatements WITHOUT LLM calls. For each sentence containing a known alias phrase, substitutes the signal name (uppercase) and re-checks is_signal_value_constraint(). If true: reclassifies statement to SignalValueConstraint, synthesises a SignalConstraintRecord (AutomationConfidence::Low, alias-derived).
- **detect_constraint_kind_from_substituted()** (evidence.rs): helper detects must_not_change / must_be_stable / must_be_high / must_be_low / must_be_asserted / must_be_deasserted from substituted mixed-case text.
- **extract_alias_phrase()** (nlp_enrich.rs): after each successful LLM extraction, if the signal name does not appear literally in the source text, extracts a 2-4 word noun phrase (strips leading articles, rejects pronouns, limits to 4 words) and inserts it into evidence_ir.signal_alias_map.
- **Pass loop integration**: (1) START of each pass: apply_alias_reclassification() shrinks candidate pool for free; (2) AFTER each LLM extraction: learn alias if signal not in text; (3) write EvidenceIR even when only aliases were learned (no LLM extractions). Summary reports total_alias_reclassified and signal_alias_map_size.
- **Converging loop**: with --max-passes N, iteration 1 builds alias dict; iteration 2+ applies it, progressively reducing NormativeStatement residuals without LLM calls; converges when neither LLM extraction nor alias reclassification produces anything new.
- **7 new tests** (83 -> 90 total, all passing):
  - evidence.rs: apply_alias_reclassification_reclassifies_normative_statement_with_alias, apply_alias_reclassification_skips_already_covered_sentences
  - nlp_enrich.rs: extract_alias_phrase_returns_none_when_signal_appears_literally, extract_alias_phrase_extracts_noun_phrase_when_signal_absent, extract_alias_phrase_rejects_pronoun_only_subjects, extract_alias_phrase_limits_to_four_words, nlp_enrich_learns_alias_and_stores_in_evidence_ir
## 2026-04-02 (Form 1: backannotation feedback loop)
- **Form 1: backannotation** (nlp_enrich.rs): after each nlp-enrich pass, ExtractedStatement.class updated in-place: NormativeStatement -> SignalValueConstraint or ConditionalRule. Closes feedback loop from Level 3 back to EvidenceIR.
- **Fixed test parallelism bug**: added vlm_helper_lock() mutex (OnceLock<Mutex<()>>) to serialize 4 tests sharing SPECFORGE_VLM_HELPER env var.
- **Test suite: 82 -> 83 (+1, all passing)**
## 2026-04-02 (NLP pipeline Layers A/B/C/D/E: boilerplate suppression, grounded multi-pass NLP, declared-signal gating, spec-type-aware scoring)
- **Layer A — Section-aware boilerplate suppression** (`evidence.rs`)
  - New `is_boilerplate_section_title()` helper: matches Introduction, Revision History, Legal Notice, Normative/Informative References, Glossary, Acronyms, Bibliography, Scope, Terms and Definitions, About this Document, and related headings
  - `EvidenceIr::build()` block loop: looks up each sentence's section heading; if boilerplate, downgrades `NormativeStatement` → `SourceFact`
  - Effect: ~12 legal/compliance normative sentences removed from residual pool per real spec (e.g. AHB). Residuals: 59 → ~47
  - 2 tests: `is_boilerplate_section_title` unit test (12 positive + 5 negative assertions), integration test verifying intro section normative sentence becomes SourceFact while protocol section stays NormativeStatement
- **Layer D — Declared-signal gating** (`semantic.rs`)
  - `SemanticIr::build()`: after `build_interfaces()`, extracts declared signal set from `AutomationConfidence::High` interface records (those from formal `Signal X is input/output` synthesized declarations)
  - Filters `signal_constraints` and `conditional_rules` to only records where the subject/consequent signal is in the declared set; gating is disabled (all kept) if no signal declarations exist (prose-only specs)
  - Effect: heuristic noise signals (from NLP token extraction) suppressed from NLP records; only real declared signals survive. Eliminates the signal noise that diluted coverage metrics
  - 1 test: `signal_constraints_for_undeclared_signals_are_filtered_by_layer_d` — HREADY (declared) survives, NOTSIG (undeclared) removed
- **Layer E — Spec-type-aware quality scoring** (`validate.rs`)
  - Imports `AutomationConfidence` to filter signal records in `validate_intent_ir()`
  - Coverage metrics now count ONLY `AutomationConfidence::High` (declared) signals; heuristic signals reported separately as `heuristic_signal_records (excluded from coverage)`
  - New formula (100 pt max, additive, no FSM penalty for non-FSM specs):
    - Signal direction coverage (declared only): 0–25 pts
    - Signal width coverage (declared only): 0–10 pts
    - NLP constraint richness (signal + conditional, capped at 30): 0–30 pts
    - Encoding enum definitions: 0–15 pts
    - Register map records: 0–5 pts
    - Timing constraint records: 0–5 pts
    - State machine (bonus, not penalty): 0–5 pts
    - System contract (bonus, not penalty): 0–5 pts
  - Score breakdown printed per component for transparency
  - AHB projected score after all layers: ~80/100 (GOOD) vs. 27/100 before
- **Layers B+C — Grounded multi-pass NLP Level 3** (`cli.rs`, `nlp_enrich.rs`)
  - `NlpEnrichArgs`: added `--grounding-signals` (comma-separated declared signal names, or omit for auto-extraction) and `--max-passes` (default 1; multi-pass stops early on convergence)
  - `auto_extract_declared_signals()`: parses `Signal X is input/output` statements from EvidenceIR to auto-build grounding list
  - `build_nlp_prompt()` now accepts `grounding_signals: &[String]`; injects "Known hardware signals: HADDR, HTRANS, ..." section before the sentence when non-empty
  - Multi-pass loop: each pass re-derives candidates (skipping already-extracted sentences); stops when pass extracts 0 new records (convergence) OR max_passes reached; writes EvidenceIR after every productive pass
  - `count_candidate_statements()` extracted as helper for `skip` mode hint
  - 5 new tests: prompt includes grounding signals, no grounding section when empty, `parse_signal_declaration_name` extracts uppercase name, multi-pass convergence test (max_passes=3 stops after 1 productive pass)
  - Updated existing tests to include new `grounding_signals: None, max_passes: 1` fields
- **Test suite: 75 → 82 (+7 tests, all passing)**
## 2026-04-02 (NLP Level 1+2 pattern expansion: ~50%→70%+ coverage uplift)
- **Level 1 `classify_statement()` vocabulary expanded significantly**
  - `NormativeStatement`: added `cannot/can not`, `is not permitted/allowed`, `are not permitted/allowed`, `may not`, `is forbidden/illegal`, `will not`, `must/shall never`, `it is mandatory`, `is not valid/legal/supported`, `are required`
  - `ConditionalRule`: added `unless`, `provided that`, `as long as` (both leading and embedded); `while/during/after/before` now also work as leading conditionals; `cannot` added to consequent verb list
  - `SignalValueConstraint` (`is_signal_value_constraint()`): added `is tied high/low/to`, `is driven high/low`, `is held/kept high/low/stable/asserted`, `remains high/low/asserted/deasserted/stable`, `cannot change`, `cannot/will not/must not/shall not be changed`, `must/shall indicate`, `must/shall not be asserted/deasserted` (passive negation forms)
  - `TimingConstraint`: added `tco/tpd/toh/tih`, `rising/falling/clock/positive/negative edge`, `within one/two clock`, `cycles` plural
- **Level 2 `extract_signal_constraints()` multi-signal extraction**
  - Strip condition clause before scanning subject signals: `HREADY` in `"...when HREADY is LOW"` is no longer confused with the constrained signal
  - New helper `text_before_condition_marker()`: returns text before first `when/while/during/unless/provided/after/before` marker
  - New helper `collect_subject_signal_tokens()`: collects ALL valid uppercase signal tokens from a text fragment (excludes logic levels, protocol states, protocol family names, role names)
  - Multi-signal sentences like `"Both HTRANS and HADDR shall be stable"` now produce one `SignalConstraintRecord` per signal instead of one
  - Negation detection now includes `cannot` and `will not`
- **Level 2 `split_conditional_sentence()`**: added `unless`, `provided that`, `as long as` as leading conditional markers
- **Level 2 `extract_protocol_state_value()`**: added INCR4/INCR8/INCR16, WRAP4/WRAP8/WRAP16, EXCLUSIVE, RETRY, SPLIT, BYTE, HALFWORD, WORD
- **15 NLP regression tests added** (60 → 75 total; all passing)
  - Tests confirm: `cannot/is not permitted/may not` → NormativeStatement; `is tied high/is held stable/cannot change/remains stable` → SignalValueConstraint; `unless/provided that/before` → ConditionalRule; `rising edge period` → TimingConstraint; multi-signal subject extraction; condition-clause stripping; logic-level exclusion from subjects
  - Clarifying comments: tests document that more-specific `SignalValueConstraint` correctly wins over NormativeStatement when a sentence contains both a value-binding phrase and a prohibition keyword
## 2026-04-02 (qwen2.5vl:7b integration: VLM fix + NLP Level 3 nlp-enrich command)
- **Critical VLM truncation bug fixed in `enrich.rs`**
  - Removed `.min(120)` cap on VLM response storage that silently corrupted every real VLM response
  - Replaced fragile `"content":` string-search with proper `serde_json` parsing of `{choices[0].message.content}`; handles both string and array content parts with clear error on invalid JSON
  - `max_tokens` increased 1024 → 2048 for VLM diagram responses
- **Default Ollama model updated**: `llava:13b` → `qwen2.5vl:7b` (both VLM enrichment and NLP Level 3)
  - `qwen2.5vl:7b` outperforms GPT-4o-mini on document/diagram understanding benchmarks; available via `ollama pull qwen2.5vl:7b` (6GB)
  - `qwen2.5vl:7b` pulled and ready on local Ollama instance
- **`specforge nlp-enrich` command (NLP Level 3) implemented**
  - `specforge nlp-enrich <evidence-ir> --vlm-provider ollama [--vlm-model qwen2.5vl:7b] [--dry-run] [--max-sentences N]`
  - Reads `NormativeStatement` sentences from EvidenceIR not already covered by Level 2
  - Sends each sentence to LLM with a structured extraction prompt (text-only, no image)
  - Prompt yields a single JSON: `signal_constraint / conditional_rule / none`
  - Robust to markdown code-fence wrapping; validates uppercase signal names; graceful `none` handling
  - Writes new `SignalConstraintRecord` / `ConditionalRuleRecord` entries back to EvidenceIR JSON
  - `SPECFORGE_VLM_HELPER` env var override for unit testing
  - 5 tests: end-to-end pipeline, dry-run isolation, code-fence JSON parsing, invalid signal rejection, conditional rule extraction
- **Test suite: 55 → 60 (+5 NLP Level 3 tests)**
## 2026-04-02 (VLM wiring, validate command, 55-test suite, doc corrections)
- **VLM observations wired into EvidenceIR** (Steps 3.2/3.3 complete end-to-end)
  - Added `TimingDiagramExtraction` and `StateMachineExtraction` to `VisualObservationKind` in `evidence.rs`
  - New `inject_vlm_observations()`: reads `VisualAsset.note` prefix `"vlm_timing_diagram_extraction: {json}"` / `"vlm_state_machine_extraction: {json}"` and injects typed `VisualObservation` entries into the matching `VisualEvidenceItem`
  - Enriched figures automatically upgraded to `VisualEvidenceRole::Normative` (highest-priority evidence)
- **SemanticIR VLM observation parsing** (timing + state machine → typed records)
  - New `extract_records_from_vlm_observations()` in `semantic.rs` iterates EvidenceIR visual observations
  - `parse_timing_diagram_observation()`: each VLM annotation string → `TimingConstraintRecord { description: annotation, confidence: Medium }`; merged with table-synthesized timing constraints
  - `parse_state_machine_observation()`: each VLM state → `RegularStateRecord`; each VLM transition → `StateTransitionRecord`; merged with formal syntax records (non-duplicate append)
  - Result: timing constraints from both tables and VLM diagrams, state records from both formal syntax and VLM extraction
- **specforge validate command** (Step 4.1 complete)
  - New `crates/specforge/src/commands/validate.rs` — auto-detects IR stage from `stage` field in artifact JSON
  - `validate_source_ir`: document profile, table classification, diagram classification, VLM readiness, section classification, residual count
  - `validate_evidence_ir`: statement classification breakdown, NLP coverage %, structured extraction counts, VLM observation counts
  - `validate_semantic_ir`: signal coverage (with_direction %, with_width %, fully_typed %), semantic record counts, system contract, residual decisions
  - `validate_intent_ir`: signal coverage, intent record counts, quality score (0–100) with grade EXCELLENT/GOOD/ADEQUATE/NEEDS IMPROVEMENT/INCOMPLETE
  - Wired in `cli.rs` as `Commands::Validate(ValidateArgs)` and dispatched in `lib.rs`
- **Test suite expanded: 49 → 55 (+6)**
  - `ir::semantic::tests::vlm_timing_diagram_observation_produces_timing_constraint_records` — full chain: SourceIR note → EvidenceIR observation → SemanticIR timing constraints
  - `ir::semantic::tests::vlm_state_machine_observation_produces_state_and_transition_records` — full chain: SourceIR note → EvidenceIR observation → SemanticIR states/transitions
  - `commands::validate::tests::validate_source_ir_artifact_reports_without_error`
  - `commands::validate::tests::validate_evidence_ir_artifact_reports_without_error`
  - `commands::validate::tests::validate_semantic_ir_artifact_reports_without_error`
  - `commands::validate::tests::validate_intent_ir_artifact_reports_without_error`
  - All 55 tests pass, 0 failures
- **EXTRACTION_ARCHITECTURE.md** corrected with accurate status for all completed steps:
  - SourceIR: DiagramKind ✅, VLM enrichment ✅
  - EvidenceIR: SignalConstraintRecord ✅, ConditionalRuleRecord ✅, TimingDiagramExtraction/StateMachineExtraction ✅
  - SemanticIR: signal_constraints ✅, conditional_rules ✅, VLM state/transition merge ✅, VLM timing merge ✅
  - IntentIR: signal_constraints ✅, conditional_rules ✅, VLM state/transition records ✅
  - Tier 3 Steps 3.1/3.2/3.3: ✅ done; added Step 3.4 (NLP Level 3: LLM-based reclassification) as planned next step
  - Step 4.1 Validation: ✅ done
## 2026-04-02 (NLP Level 2 structured extraction + VLM enrichment pipeline)
- **EXTRACTION_ARCHITECTURE.md** updated as authoritative reference: NLP 4-level pyramid, classification-vs-extraction gap analysis, VLM provider architecture (Ollama/OpenAI/LM Studio), all implementation steps with precise ✅/❌ status
- **Level 2 NLP: SignalConstraintRecord extraction**
  - New types in `source.rs`: `SignalConstraintKind`, `SignalConstraintRecord`, `ConditionalRuleRecord`
  - `extract_signal_constraints()` in `evidence.rs`: for each `SignalValueConstraint` sentence, extracts `{subject_signal, constraint_kind, target_value, condition_text, negated}` via syntactic pattern matching; stop-worded for AMBA/ARM/company names
  - `extract_conditional_rules()` in `evidence.rs`: for each `ConditionalRule` sentence, extracts `{antecedent_text, consequent_signal, consequent_action}` by sentence splitting on when/if/while/during
  - `EvidenceIr.signal_constraints` + `EvidenceIr.conditional_rules` as first-class typed fields
  - Carried through `SemanticIr.signal_constraints` and `IntentIr.signal_constraints`
  - AHB result: 12 `SignalConstraintRecord` (HAUSER must_not_change, HEXOKAY must_be_deasserted, etc.), 36 `ConditionalRuleRecord`
- **DiagramKind classification in SourceIR (Step 3.1)**
  - New `DiagramKind` enum in `source.rs`: `TimingDiagram`, `StateMachineDiagram`, `BlockDiagram`, `RegisterBitfield`, `TruthTable`, `FlowChart`, `Unknown`
  - `VisualAsset.diagram_kind` field set from caption text in Docling Python helper
  - `classify_diagram_kind()` in Python helper: pattern-matches AMBA-specific caption vocabulary ("read transfer", "write transfer", "burst", "wait state" → `timing_diagram`; "Manager interface", "multiplexor interconnection" → `block_diagram`; etc.)
  - AHB result: **17 timing diagrams** correctly classified, 3 block diagrams
- **specforge enrich command (Step 3.2/3.3)**
  - New `specforge enrich <source-ir> --vlm-provider <provider>` command
  - Providers: `ollama` (localhost:11434, model `llava:13b`), `openai` (OPENAI_API_KEY, model `gpt-4o`), `lmstudio` (localhost:1234), `skip` (default)
  - `--classify-only` flag: shows timing/state-machine diagram counts without calling VLM
  - `--vlm-model` override for custom models
  - `--dry-run` shows which figures would be sent to VLM without making calls
  - `SPECFORGE_VLM_HELPER` env var override for unit testing (same pattern as `SPECFORGE_DOCLING_HELPER`)
  - Structured prompts: timing diagram → `{signals, cycles, annotations}` JSON; state machine → `{states, transitions}` JSON
  - All providers use OpenAI-compatible chat completions API (supports Docling's granite-docling model via Ollama or LM Studio)
  - VLM enrichment writes updated `VisualAsset.note` with typed extraction; downstream `specforge evidence` picks it up
- 48 tests, 0 failures
## 2026-04-02 (Tier 2: Register/Timing type system + NormativeStatement sub-classes)
- added `RegisterRecord` and `RegisterFieldRecord` types to `source.rs` (foundation layer, no circular deps)
- added `TimingConstraintRecord` type to `source.rs`
- re-exported these types from `semantic.rs` so `IntentIR` and adapters import from `semantic` as before
- added `synthesize_register_records()` in `evidence.rs`: reads `SourceIR.structured_tables` where `table_kind == RegisterMap`; extracts register name, offset address, bit field rows
- added `synthesize_timing_constraints()` in `evidence.rs`: reads `SourceIR.structured_tables` where `table_kind == TimingParameter`; extracts parameter name, min/typ/max values, unit
- added `EvidenceIr.register_records: Vec<RegisterRecord>` and `EvidenceIr.timing_constraints: Vec<TimingConstraintRecord>` as typed first-class fields
- carried `register_records` and `timing_constraints` through `SemanticIr` and `IntentIr` unchanged
- extended `StatementClass` with `TimingConstraint` (cycle counts, setup/hold references, latency bounds) and `ConditionalRule` (`when X then Y` / `if A then B` conditional behavioral structures)
- updated `classify_statement()` to detect `TimingConstraint` and `ConditionalRule` patterns before the generic `NormativeStatement` check
- updated `EXTRACTION_ARCHITECTURE.md` to reflect precise current done/pending status and sharpen modality descriptions with exact type names
- validated on AMBA AHB PDF: 21 register records, 8 timing constraints, statement classes: 91 normative_statement, 42 conditional_rule, 30 timing_constraint, 14 derived_rule, 5 explicit_abstraction (vs. 100% source_fact before)
- all `cargo fmt` and `cargo test` pass: 48 tests, 0 failures
## 2026-04-02 (SOTA SourceIR and EvidenceIR)
- created `EXTRACTION_ARCHITECTURE.md` — comprehensive reference document capturing the full SOTA extraction vision for chip spec PDFs: six information modalities, quality gap analysis per IR stage, target architecture, and priority-ordered implementation plan (Tier 1–4)
- updated `ROADMAP.md` with new workstreams R8 (SourceIR SOTA capture), R9 (EvidenceIR SOTA typed evidence), and R10 (EvidenceIR VLM visual content)
- extended Docling Python helper to extract in a single pass:
  - **structured table cell grids** (`StructuredTableRecord` with header/body row cells, row/col spans, `is_header` flags)
  - **table kind classification** (`classify_table_kind`): `signal_description`, `encoding`, `register_map`, `timing_parameter`, `feature_matrix`, `unknown`
  - **typed content elements** (`ContentElementRecord`): all text elements with Docling type labels (title, section_header, body_text, list_item, code, caption, footnote, formula), reading order, page provenance
  - **section hierarchy with semantic classification** (`ContentSectionRecord` with `SectionKind`): `Boilerplate`, `SignalDescription`, `Normative`, `Timing`, `RegisterDescription`, `Glossary`, `Appendix`, `TableOfContents`
  - **document profile** (`DocumentProfile`): title, page/table/figure/section counts
- added new Rust types to `source.rs`: `StructuredTableRecord`, `StructuredTableCellRecord`, `TableKind`, `ContentElementRecord`, `ContentElementKind`, `ContentSectionRecord`, `SectionKind`, `DocumentProfile`
- updated `SourceIr` struct with new `#[serde(default)]` fields: `structured_tables`, `content_elements`, `document_sections`, `document_profile`
- updated `DoclingBackendSummary` to deserialize all new fields; updated `SourceIr::materialize()` to populate them
- added `StatementClass::NormativeStatement` to `EvidenceIR` statement classification — sentences with `shall`/`must`/`shall not` in non-boilerplate sections are now correctly classified as behavioral requirements rather than generic `SourceFact`
- added `synthesize_declarations_from_tables()` in `EvidenceIR` that reads `source_ir.structured_tables` and synthesizes formal typed declarations:
  - `SignalDescription` tables → `Signal X is output/input [width N].` declarations (High confidence)
  - `Encoding` tables → `Enum <name> <member> = <value>.` declarations (High confidence)
  - Direction inferred from `ContentSectionRecord.section_kind` + section title keywords; width from numeric cell values; non-signal tokens filtered via `is_signal_synthesis_non_signal()`
- **removed `parse_signal_table_row` band-aid from `SemanticIR`** — signal declarations now flow cleanly from `EvidenceIR` structured table synthesis through `SemanticIR`'s existing `parse_explicit_signal_declaration` and `parse_explicit_symbol_definition` parsers
- validated on AMBA AHB Protocol Specification PDF (SOTA pipeline, re-ingested):
  - `source_ir.structured_tables`: 40 tables (11 signal_description, 2 encoding, 3 register_map, 2 timing_parameter, 22 unknown)
  - `source_ir.content_elements`: 1004 typed text elements
  - `source_ir.document_sections`: 172 sections (115 normative, 37 signal_description, 8 boilerplate, 5 timing, 4 appendix, 2 glossary, 1 table_of_contents)
  - `source_ir.document_profile`: page_count=104, table_count=40, figure_count=30, section_count=172
  - 17 AHB signals with explicit direction+width in adapter signal inventory (HSELX newly added from Decoder table)
  - `NormativeStatement` classification active for behavioral requirements
  - No SemanticIR band-aid; signal declarations flow architecturally
- all `cargo fmt` and `cargo test` checks pass: 48 tests, 0 failures
## 2026-04-02 (continued)
- improved `SemanticIR` extraction quality for real chip specification PDFs with three targeted fixes:
  - **expanded `signal_stop_words()`** with ~200 entries covering legal/contractual vocabulary, common English all-caps words (HIGH, LOW, etc.), AMBA/ARM protocol family names, company names, document structure words, and technology abbreviations that are never hardware signal names; this eliminates legal front-matter contamination from signal extraction
  - **added boilerplate section filtering** in `SemanticContext::from_evidence_ir` so statements from sections matching legal/admin patterns (licence, proprietary notice, change history, etc.) are excluded from actor/interface/invariant extraction entirely
  - **added interface noise filtering** in `build_interfaces` so heuristic interfaces with >8 signals require ≥2 supporting statements; this eliminates the large spurious interfaces created by co-mentions in legal paragraphs while keeping all small hardware signal groups
- added **markdown signal-table row parsing** in `build_interfaces`: when a table row's first cell looks like a hardware signal name and the section title matches a known direction context ("Manager signals" → output, "Subordinate signals" → input, "Global/Decoder signals" → input), the row is parsed directly into a typed `InterfaceSignalRecord` with explicit direction and width, extracted at Medium automation confidence
- validated improvements on the AMBA AHB Protocol Specification PDF (`IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf`):
  - 104 page artifacts and 70 visual assets materialized correctly by Docling
  - `interface_count` reduced from 172 → 94 (45% reduction, legal text gone)
  - `signal_candidate_count` in the adapter reduced from 250 → 57 (77% reduction, mostly real AHB signals)
  - 16 AHB signals now carry explicit direction and width from signal table parsing:
    - Manager outputs (direction=output): HADDR, HBURST, HEXCL, HMASTER, HMASTLOCK, HNONSEC, HPROT, HSIZE, HTRANS, HWDATA, HWRITE, HWSTRB
    - Subordinate outputs / Manager inputs (direction=input): HEXOKAY, HRDATA, HREADYOUT, HRESP
    - Key widths extracted: HTRANS=2, HSIZE=3, HWRITE=1, HMASTLOCK=1, HEXCL=1, HNONSEC=1, HREADYOUT=1, HRESP=1, HEXOKAY=1
  - adapter correctly blocked (honest: AHB spec prose does not carry formal control blocks or system contract declarations)
- added 1 new regression test: `extracts_signal_direction_and_width_from_markdown_signal_description_table`
  - verifies Manager-section rows are extracted as Output with correct numeric widths
  - verifies Subordinate-section rows are extracted as Input with correct numeric widths
  - total tests: 48 passing, 0 failing
- installed Docling 2.84.0 globally into Python 3.11 (`/opt/homebrew/lib/python3.11/site-packages/`) to enable PDF processing
- all `cargo fmt` and `cargo test` checks pass
## 2026-04-02
- widened `SemanticIR` so it now preserves canonical `.fsm`-relevant symbol-definition and structured-control surface rather than relying only on legacy decision-tree fragments:
  - canonical symbol definitions for `Constant`, `Define`, `Param`, and `Enum`
  - canonical control expressions, branch-local actions, and dedicated synchronous-reset/asynchronous-reset control-block roles
  - richer module-scoped carry-through for the same widened semantic surface
- widened the canonical reset contract so `SystemContractRecord` now preserves reset kind, reset polarity, assertion timing, release timing, and reset-target semantics explicitly instead of leaving real hardware reset behavior implicit
- tightened reset normalization so explicit reset declarations now:
  - preserve synchronous reset as synchronous assertion + synchronous release through the data-input path
  - preserve asynchronous reset as asynchronous assertion + synchronous release through the dedicated reset pin
  - infer active-low polarity from `_n` / `_b` reset naming and otherwise fall back to active-high with lower automation confidence when explicit polarity wording is omitted
- widened `IntentIR` so it now carries canonical symbol-definition sections and structured control blocks unchanged for downstream adapters
- refactored the `.fsm` adapter to lower from canonical `symbol_definitions` and `control_blocks` first, with legacy decision-tree fragments kept only as a fallback when the widened canonical surface is absent
- tightened `.fsm` system-contract renderability so reset polarity must stay recoverable honestly from `sreset` / `asreset` plus the reset signal name in the current target slice
- widened emitted `.fsm` text so the renderable slices now cover:
  - `+constants`, `+define`, `+params`, and `+enums` sections
  - structured standalone/DT and FSM-root lowering from canonical control blocks
  - canonical synchronous-reset and asynchronous-reset control-role blocks
  - explicit public-output targets and dual-output assignment forms carried through the widened control model when renderable
- widened the `.fsm` adapter so selector/test-node control and canonical compound-update actions now lower honestly into emitted `.fsm` text when their canonical selector/predicate/update shapes map directly to explicit `.fsm` test-selector tokens and update shorthand, while unsupported selector predicates remain blocked explicitly
- repaired accidental corruption in the `adapters.rs` regression module and tightened adapter residual logic so renderable compound-update artifacts no longer keep a stale `fsm_adapter_dt_action_graph` packet
- reviewed the current `fsmgen` direct-root contract and confirmed that `?mod:name` / `?module:name` are still compatibility-level accepted spellings on a shared single-module path rather than a settled backend-neutral semantic distinction for SpecForge
- tightened the SpecForge `.fsm` adapter root-kind model so it now only represents the current honest canonical roots (`dt`, `fsm`, `top`) and no longer carries speculative `mod` / `module` placeholder variants in adapter JSON or deferred-root decisions
- tightened explicit reset parsing so both of these phrasing styles now normalize into the widened backend-neutral reset contract:
  - `Reset rst_n is asynchronous active low.`
  - `Reset rst is synchronous active high.`
- added regression coverage for:
  - semantic extraction of synchronous active-high reset phrasing
  - intent carry-through of synchronous active-high reset phrasing
  - semantic and intent carry-through of inferred active-low reset polarity from `rst_n`
  - honest adapter blocking when reset polarity cannot be preserved through the reset signal name
  - renderable standalone DT lowering with canonical symbol-definition sections
  - renderable structured FSM lowering with canonical reset-role blocks
  - renderable selector-based standalone DT lowering
  - renderable computed-selector standalone DT lowering
  - honest blocking when a selector branch predicate does not map relative to the chosen selector
  - renderable compound-update standalone DT lowering
  - tightened deferred-root decisions so renderable/blocked adapter artifacts keep only the current honest root-kind set (`dt`, `fsm`, `top`)
- validated the widened `.fsm` semantic slice with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo fmt --all --manifest-path Cargo.toml --check`
  - `cargo test --manifest-path Cargo.toml adapters`
  - `cargo test --manifest-path Cargo.toml`
  - an execute-mode end-to-end CLI pipeclean on a temporary inferred-polarity reset sample through `ingest -> evidence -> semantic -> intent -> adapt`
  - an execute-mode end-to-end CLI pipeclean on a temporary synchronous-active-high reset sample through `ingest -> evidence -> semantic -> intent -> adapt`
  - an execute-mode end-to-end CLI pipeclean on a temporary selector/test-node sample through `ingest -> evidence -> semantic -> intent -> adapt`
  - an execute-mode end-to-end CLI pipeclean on a temporary compound-update sample through `ingest -> evidence -> semantic -> intent -> adapt`
- confirmed the representative inferred-polarity end-to-end adapter output is now safely renderable while preserving the widened reset contract in JSON:
  - `document_key: inferred_reset_cli`
  - `semantic/system_contract.reset_polarity: active_low`
  - `semantic/system_contract.assertion_timing: asynchronous_to_clock`
  - `semantic/system_contract.release_timing: synchronous_to_clock`
  - `semantic/system_contract.target_kind: dedicated_reset_pin`
  - `semantic/system_contract.automation_confidence: medium`
  - `intent/system_contract` matches the widened semantic reset contract exactly
  - `emitted_target_path: generated/adapters/fsm/inferred_reset_cli/inferred_reset_cli.fsm`
- confirmed the representative synchronous-active-high end-to-end adapter output is now safely renderable:
  - `document_key: sync_control`
  - `lowering_status: renderable`
  - `selected_root_kind: fsm`
  - `emitted_target_path: generated/adapters/fsm/sync_control/sync_control.fsm`
- confirmed the representative selector/test-node end-to-end adapter output is now safely renderable:
  - `document_key: selector_dt`
  - `lowering_status: renderable`
  - `selected_root_kind: dt`
  - `residual_decision_count: 2`
  - `emitted_target_path: generated/adapters/fsm/selector_dt/selector_dt.fsm`
  - emitted test-node block includes `(?MODE ...)` and the selector branch token `=mode_t.idle`
- confirmed the representative compound-update end-to-end adapter output is now safely renderable:
  - `document_key: compound_update_dt`
  - `lowering_status: renderable`
  - `selected_root_kind: dt`
  - `residual_decision_count: 4`
  - `emitted_target_path: generated/adapters/fsm/compound_update_dt/compound_update_dt.fsm`
  - emitted update block includes `(-bump` and `(+= ACC STEP)`
- refreshed the live documentation surface so the README, user guide, live status tracker, codebase analysis, development notes, roadmap, and continuity records now describe the widened canonical reset contract, the landed selector/test-node and compound-update slice, the current reset-naming convention, and the remaining direct-module alias gap plus explicit unsupported selector/predicate boundaries
- refreshed the same live documentation surface again so it now records the direct-module defer decision explicitly, removes speculative adapter root-kind language, and advances the next milestone to validation/back-annotation
## 2026-04-01
- enriched `SemanticIR` so it now preserves explicit backend-neutral module and top-composition records from `Module ...` and `Top ...` statements, including typed top ports, child-module references, and explicit wiring links
- tightened semantic extraction so module-scoped and top-scoped statements are handled through scoped parsing helpers and no longer leak into document-global direct-root inference
- enriched `IntentIR` so it now carries canonical explicit module and top-composition surface forward unchanged for downstream adapters
- widened the `.fsm` adapter beyond a single direct-root model so it now:
  - inventories explicit module candidates and explicit top candidates from canonical intent records
  - selects an honest `?top:name` root when exactly one explicit top composition is renderable
  - renders stable top-level support blocks such as `?ports:public_io` and `?toplink:wiring`
  - embeds referenced renderable child module roots after the selected `?top:name` root
  - keeps standalone direct `?mod:name` / `?module:name` alias roots deferred until there is a real backend-neutral direct-module distinction
- tightened adapter renderability checks for explicit top composition so emitted `.fsm` text now requires:
  - fully typed explicit top ports
  - existing referenced child modules
  - renderable child module roots
  - width-compatible and direction-compatible explicit link endpoints
  - explicit links for the current multi-child composition slice
- tightened adapter residual logic so standalone DT residuals are suppressed when an explicit `?top:name` source document is selected and composition-specific residuals remain honest when child modules or links are missing
- extended `specforge adapt` execute-mode summaries with `module_candidate_count` and `top_candidate_count`
- added regression coverage for:
  - explicit module/top extraction in `SemanticIR`
  - explicit module/top carry-through in `IntentIR`
  - renderable and blocked explicit top-composition adapter paths
- validated the new explicit composition slice with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/explicit_top.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/explicit_top/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/explicit_top/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/explicit_top/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/explicit_top/intent_ir.json --target fsm`
- confirmed the representative explicit top-composition end-to-end adapter output is now safely renderable:
  - `lowering_status: renderable`
  - `selected_root_kind: top`
  - `signal_candidate_count: 1`
  - `decision_tree_candidate_count: 0`
  - `state_candidate_count: 0`
  - `transition_candidate_count: 0`
  - `module_candidate_count: 2`
  - `top_candidate_count: 1`
  - `residual_decision_count: 3`
  - `emitted_target_path: generated/adapters/fsm/explicit_top/datapath.fsm`
- refreshed the live documentation surface so the roadmap, status trackers, user guide, architecture docs, and continuity files now describe the landed explicit `?top:name` slice and the still-deferred direct module-alias roots
- enriched `SemanticIR` so it now preserves backend-neutral regular-state and transition records from explicit `State ...` and `Transition ...` statements
- enriched `IntentIR` so it now carries canonical regular-state and transition surface forward for downstream adapters
- widened the `.fsm` adapter so it now selects honest `?fsm:name` roots from the explicit canonical state graph, groups state-matching control fragments into state bodies, preserves unmatched control fragments as standalone `-block` children, and renders sequential state-body assignments with `<=`
- tightened adapter-side residual logic so the unresolved state-graph packet only remains when structured FSM lowering is actually blocked
- extended `specforge adapt` execute-mode summaries with `transition_candidate_count` for explicit FSM-root pipecleans
- added regression coverage for:
  - explicit regular-state and transition extraction in `SemanticIR`
  - canonical regular-state and transition carry-through in `IntentIR`
  - renderable and blocked structured `?fsm:name` adapter paths
- validated the new structured FSM slice with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/explicit_fsm.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/explicit_fsm/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/explicit_fsm/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/explicit_fsm/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/explicit_fsm/intent_ir.json --target fsm`
- confirmed the representative explicit FSM end-to-end adapter output is now safely renderable:
  - `lowering_status: renderable`
  - `selected_root_kind: fsm`
  - `signal_candidate_count: 7`
  - `decision_tree_candidate_count: 1`
  - `state_candidate_count: 2`
  - `transition_candidate_count: 2`
  - `residual_decision_count: 2`
  - `emitted_target_path: generated/adapters/fsm/explicit_fsm/explicit_fsm.fsm`
- enriched `SemanticIR` so it now preserves backend-neutral system contract and init-assignment records from explicit `Clock ...`, `Reset ...`, and `Init ...` statements
- enriched `IntentIR` so it now carries canonical system contract and init-assignment surface forward for downstream adapters
- widened the `.fsm` adapter so it now renders explicit standalone sequential `?dt:name` text with `(+system ...)` and `(:= ...)` when the canonical system/init facts are complete
- added a dedicated adapter-side residual for unresolved system/init surface so sequential standalone DT cases stay blocked explicitly instead of inventing reset semantics
- added regression coverage for:
  - explicit system/init extraction in `SemanticIR`
  - canonical system/init carry-through in `IntentIR`
  - renderable and blocked standalone sequential `.fsm` adapter paths
- validated the new standalone sequential slice with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/seq_dt.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/seq_dt/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/seq_dt/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/seq_dt/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/seq_dt/intent_ir.json --target fsm`
- confirmed the representative explicit sequential end-to-end adapter output is now safely renderable:
  - `lowering_status: renderable`
  - `selected_root_kind: dt`
  - `signal_candidate_count: 4`
  - `decision_tree_candidate_count: 1`
  - `residual_decision_count: 4`
  - `emitted_target_path: generated/adapters/fsm/seq_dt/seq_dt.fsm`
- enriched `SemanticIR` so it now preserves typed signal records and backend-neutral guarded/action control fragments when the evidence is explicit enough
- enriched `IntentIR` so it now carries the canonical interface inventory and backend-neutral control fragments forward for downstream adapters
- widened the `.fsm` adapter so it now consumes the canonical interface/control surface instead of relying only on mined prose hints
- the `.fsm` adapter now emits a real standalone `?dt:name` file for explicit canonical cases and keeps broader sequential/system-contract/composition cases blocked instead of inventing semantics
- added regression coverage for:
  - explicit typed-signal/control extraction in `SemanticIR`
  - canonical interface/control carry-through in `IntentIR`
  - blocked and renderable `.fsm` adapter paths
- validated the new canonical/renderable slice with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/comb_dt.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/comb_dt/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/comb_dt/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/comb_dt/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/comb_dt/intent_ir.json --target fsm`
- confirmed the representative explicit end-to-end adapter output is now safely renderable:
  - `lowering_status: renderable`
  - `selected_root_kind: dt`
  - `signal_candidate_count: 3`
  - `decision_tree_candidate_count: 1`
  - `residual_decision_count: 2`
  - `emitted_target_path: generated/adapters/fsm/comb_dt/comb_dt.fsm`
- implemented the first real adapter slice on top of persisted `IntentIR` artifacts
- added `specforge adapt <intent-ir> --target fsm [--dry-run]` to preview or materialize `generated/adapters/fsm/<document_key>/adapter.json`
- replaced the old adapter planning-only scaffolding with a typed adapter artifact model in `crates/specforge/src/ir/adapters.rs`
- the first `.fsm` adapter slice now:
  - loads persisted `IntentIR` JSON from disk
  - selects a conservative DT-oriented root instead of inventing FSM or composition semantics
  - inventories low-confidence signal candidates and DT/state candidate structure from canonical intent records
  - preserves upstream residual decisions and emits adapter-side residuals for missing signal inventory, DT fragments, and broader root-kind expansion
  - blocks emitted `.fsm` text when target syntax would require semantic invention
- added adapter-stage unit tests for:
  - handshake-driven `.fsm` adapter artifact construction
  - wrong-stage input rejection before deserializing as `IntentIR`
- validated the new adapter slice with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/handshake.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/handshake/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/handshake/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/handshake/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/handshake/intent_ir.json --target fsm`
- confirmed the representative end-to-end adapter output is currently honest and blocked rather than fabricated:
  - `lowering_status: blocked`
  - `selected_root_kind: dt`
  - `signal_candidate_count: 2`
  - `decision_tree_candidate_count: 1`
  - `residual_decision_count: 3`
- pivoted the repository objective so `IntentIR` is now the canonical product boundary
- rewrote the core docs around the explicit staged pipeline:
  - `SourceIR`
  - `EvidenceIR`
  - `SemanticIR`
  - `IntentIR`
  - adapters
- added `INTENTIR_SPEC.md` as the canonical architecture/specification document for the new direction
- renamed the active Rust crate and CLI direction from `spec2fsm` to `specforge` with no compatibility aliasing
- renamed the workspace member path to `crates/specforge`
- refactored the Rust code layout around explicit staged IR modules:
  - `crates/specforge/src/ir/source.rs`
  - `crates/specforge/src/ir/evidence.rs`
  - `crates/specforge/src/ir/semantic.rs`
  - `crates/specforge/src/ir/intent.rs`
  - `crates/specforge/src/ir/adapters.rs`
- replaced the previous ingest-manifest framing with a real `SourceIR` artifact
- updated `specforge ingest` so:
  - dry-run prints computed `SourceIR` JSON
  - execute mode materializes `generated/source_ir/<document_key>/source_ir.json`
- formalized a stricter SOTA ingestion stance:
  - structured parser first
  - provenance-preserving page and visual asset capture second
  - selective multimodal enrichment for figures, charts, diagrams, and image-heavy regions third
- extended `SourceIR` scaffolding so it now reserves:
  - parser backend identity
  - page-artifact manifests
  - visual-asset manifests
  - placeholder bindings for normalized sources
- extended `EvidenceIR` scaffolding so it now reserves:
  - multimodal evidence spans
  - visual evidence items
  - text-to-figure links
  - picture-description / OCR-over-image / chart-extraction observations
- recorded adapter targets as downstream of `IntentIR`:
  - `.fsm`
  - SystemVerilog
  - Verilog
  - VHDL
- kept residual decision packets as a first-class mechanism for unresolved automation
- updated the live status tracker so the next highest-priority gap is:
  - close the remaining `SourceIR` structured-PDF-normalization gap and build the first real multimodal `EvidenceIR` extractor
- validated the renamed crate and staged IR refactor with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test`
  - `cargo run -p specforge -- --help`
  - `cargo run -p specforge -- ingest README.md --dry-run`
- confirmed that `specforge ingest README.md --dry-run` now exposes parser backend, page-artifact manifest, visual-asset manifest, and placeholder-binding fields in `SourceIR`
- confirmed the remaining `spec2fsm` mentions are historical continuity references rather than active product naming
- implemented the first real structured PDF normalization backend for `SourceIR`
- added `crates/specforge/src/ir/source/docling_backend.rs` to orchestrate a Docling-backed PDF conversion flow from Rust
- `specforge ingest <pdf>` now materializes:
  - promoted markdown
  - page images and per-page metadata sidecars
  - cropped picture and table assets
  - backend raw JSON and metadata JSON
  - `page_artifacts.json` and `visual_assets.json`
- `SourceIR` PDF execute mode now upgrades its normalization status from `planned_conversion` to `ready` after successful backend materialization
- added a `source_ref` field to visual-asset records so later stages can trace assets back into backend-native structured output
- added runtime dependency guidance:
  - discover `docling` from `python3` or `python`
  - optionally override with `SPECFORGE_DOCLING_PYTHON`
- added a backend-override seam for tests and advanced local integration with `SPECFORGE_DOCLING_HELPER`
- added a stubbed PDF materialization unit test so the real SourceIR backend path is exercised without requiring Docling inside `cargo test`
- validated the new PDF backend with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test`
  - `cargo run -p specforge -- ingest README.md`
  - `SPECFORGE_DOCLING_PYTHON=/tmp/specforge-docling-venv/bin/python cargo run -p specforge -- ingest /tmp/specforge-docling-sample.pdf`
- updated the live status tracker so the remaining top-priority gap is now the first real `EvidenceIR` extractor rather than the SourceIR PDF-normalization backend
- implemented the first real `EvidenceIR` extractor on top of persisted `SourceIR` artifacts
- added `specforge evidence <source-ir> [--dry-run]` to preview or materialize `generated/evidence_ir/<document_key>/evidence_ir.json`
- `EvidenceIR::build` now:
  - loads persisted `SourceIR` JSON from disk
  - requires `normalization_status: ready`
  - parses promoted markdown into section anchors and block-level evidence spans
  - projects `SourceIR` visual assets into typed visual evidence items
  - links caption spans with `describes` and figure/table references with `cites`
  - emits heuristic statement classes for source facts, derived rules, local design decisions, and explicit abstractions
- added stage-artifact loading support and deserialize coverage needed to rebuild `EvidenceIR` from saved `SourceIR` JSON
- added unit tests for:
  - markdown-only `EvidenceIR` construction
  - caption plus figure-reference grounding into visual evidence
- validated the new `EvidenceIR` stage with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run -p specforge -- ingest README.md`
  - `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json --dry-run`
  - `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json`
  - `SPECFORGE_DOCLING_PYTHON=/tmp/specforge-docling-venv/bin/python cargo run -p specforge -- ingest /tmp/specforge-docling-sample.pdf`
  - `cargo run -p specforge -- evidence generated/source_ir/specforge_docling_sample/source_ir.json`
- confirmed live execute-mode outputs for validation:
  - markdown-backed `EvidenceIR`: 14 section anchors, 167 evidence spans, 167 extracted statements
  - PDF-backed `EvidenceIR`: 18 section anchors, 225 evidence spans, 11 visual evidence items, 19 evidence links, 225 extracted statements
- updated the live status tracker so the remaining top-priority gap is now the first real `SemanticIR` constructor rather than the `EvidenceIR` extraction stage
- implemented the first real `SemanticIR` extractor on top of persisted `EvidenceIR` artifacts
- added `specforge semantic <evidence-ir> [--dry-run]` to preview or materialize `generated/semantic_ir/<document_key>/semantic_ir.json`
- `SemanticIR::build` now:
  - loads persisted `EvidenceIR` JSON from disk
  - derives typed semantic artifacts under `generated/semantic_ir/<document_key>/semantic_ir.json`
  - discovers actors, interfaces, phases, invariants, contracts, gates, abstractions, and decomposition candidates from deterministic heuristics
  - emits residual decisions for unresolved actor boundaries, overlapping interface groups, and ambiguous visual semantics
- added stage-artifact loading support needed to rebuild `SemanticIR` from saved `EvidenceIR` JSON
- added unit tests for:
  - handshake-driven actor/interface/invariant extraction
  - ambiguous visual grounding residual decisions
- validated the new `SemanticIR` stage with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run -p specforge -- ingest README.md`
  - `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json`
  - `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json --dry-run`
  - `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json`
  - `SPECFORGE_DOCLING_PYTHON=/tmp/specforge-docling-venv/bin/python cargo run -p specforge -- ingest /tmp/specforge-docling-sample.pdf`
  - `cargo run -p specforge -- evidence generated/source_ir/specforge_docling_sample/source_ir.json`
  - `cargo run -p specforge -- semantic generated/evidence_ir/specforge_docling_sample/evidence_ir.json`
- confirmed live execute-mode outputs for validation:
  - markdown-backed `SemanticIR`: 2 actors, 0 interfaces, 4 phases, 3 invariants, 3 gates, 1 abstraction, 12 decomposition candidates, 0 residual decisions
  - PDF-backed `SemanticIR`: 2 actors, 22 interfaces, 7 phases, 17 invariants, 2 contracts, 20 gates, 12 decomposition candidates, 2 residual decisions
- updated the live status tracker so the remaining top-priority gap is now the first real canonical `IntentIR` constructor rather than the `SemanticIR` stage
- implemented the first real `IntentIR` constructor on top of persisted `SemanticIR` artifacts
- added `specforge intent <semantic-ir> [--dry-run]` to preview or materialize `generated/intent_ir/<document_key>/intent_ir.json`
- `IntentIR::build` now:
  - loads persisted `SemanticIR` JSON from disk
  - derives canonical intent artifacts under `generated/intent_ir/<document_key>/intent_ir.json`
  - canonicalizes actor responsibilities, behaviors, constraints, and assumptions from deterministic heuristics over semantic records
  - preserves semantic residual decisions and emits additional canonicalization residuals only when the intent model would otherwise become speculative
- added unit tests for:
  - handshake-driven intent identity, behavior, constraint, and assumption construction
  - residual-decision preservation from `SemanticIR` into `IntentIR`
- validated the new `IntentIR` stage with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run -p specforge -- ingest README.md`
  - `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json`
  - `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json`
  - `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json --dry-run`
  - `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json`
  - `SPECFORGE_DOCLING_PYTHON=/tmp/specforge-docling-venv/bin/python cargo run -p specforge -- ingest /tmp/specforge-docling-sample.pdf`
  - `cargo run -p specforge -- evidence generated/source_ir/specforge_docling_sample/source_ir.json`
  - `cargo run -p specforge -- semantic generated/evidence_ir/specforge_docling_sample/evidence_ir.json`
  - `cargo run -p specforge -- intent generated/semantic_ir/specforge_docling_sample/semantic_ir.json`
- confirmed live execute-mode outputs for validation:
  - markdown-backed `IntentIR`: 2 actors, 7 behaviors, 3 constraints, 1 assumption, 0 residual decisions
  - PDF-backed `IntentIR`: 2 actors, 28 behaviors, 24 constraints, 1 assumption, 2 residual decisions
- updated the live status tracker so the remaining top-priority gap is now the first real adapter lowering pass rather than the `IntentIR` stage
- added `subs/fsmgen` as a pinned git submodule using the SSH remote `git@github.com:rdje/fsmgen.git`
- pinned the local `fsmgen` reference checkout at submodule revision `57f00e581b4fc9a2aa02318846d1eb8a726c8960`
- updated the live documentation surface so the repo map and continuity notes now treat `subs/fsmgen` as the local `.fsm` reference implementation for upcoming adapter work
- recorded the workflow rule that `subs/fsmgen` is contextual and read-only inside `specforge`
- established the local upstream bug-report ID format `FSMGEN-BUG-####` for any future `fsmgen` misbehavior found during adapter work
- no new `fsmgen` misbehavior was identified in this slice, so no local `FSMGEN-BUG-####` report was filed yet

## 2026-03-31
- initialized the `specforge` Git repository
- established the initial live documentation surface:
  - `README.md`
  - `SESSION_BOOTSTRAP.md`
  - `ROADMAP.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `USER_GUIDE.md`
  - `DEVELOPMENT_NOTES.md`
  - `CHANGES.md`
  - `MEMORY.md`
- defined `README.md` as the single project entry point
- recorded the working project/binary naming:
  - project: `specforge`
  - CLI: `spec2fsm`
- recorded the staged-tool architecture direction and the initial Rust architecture baseline
- updated `COMMIT.md` to reinforce live-document continuity requirements during long-running tasks
- added `.gitignore` rules so local workflow files and build artifacts remain untracked
- created the initial Rust workspace and bootstrap CLI
- established the initial continuity workflow and live-doc surface
- created the first repository baseline commit
