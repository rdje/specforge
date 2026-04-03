# MEMORY
## Purpose
- maintain a compact but actionable continuity record for interrupted sessions
- preserve enough context to resume work quickly after session loss, tool restart, machine crash, or model handoff

## Current project identity
- repository name: `specforge`
- CLI/binary name: `specforge`
- implementation language: Rust
- canonical deliverable: `IntentIR`
- stage model: `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`

## User-required continuity rules
- live documents are critical infrastructure, not optional afterthoughts
- update `MEMORY.md` after completed tasks and at meaningful checkpoints during long-running work
- `MEMORY.md` must record the latest already-committed Git hash/message known at the time of update
- `CHANGES.md` must capture the detailed changes about to be committed
- `DEVELOPMENT_NOTES.md` must capture engineering decisions, rationale, and validation context
- `ROADMAP.md`, `LIVE_ACHIEVEMENT_STATUS.md`, and `RUST_CODEBASE_ANALYSIS.md` must be reviewed before each commit when the task changes them
- `README.md` remains the single entry point and its last line must be `Read SESSION_BOOTSTRAP.md and start from there.`
- `subs/fsmgen` is contextual-only and must stay read-only from `specforge`
- if `fsmgen` behavior looks wrong, file a local tracked bug report using `FSMGEN-BUG-####` instead of patching the submodule

## Latest committed baseline
- latest_commit_hash: `1a5f7bc`
- latest_commit_brief_message: `fix(ingest): broader timing_diagram classification for figure captions`
- note: this is the pre-commit baseline; after the next commit, refresh this section again so it points at the newly created commit

## Recent commit chain (last 5)
- `1a5f7bc` fix(ingest): broader timing_diagram classification for figure captions
- `a264144` fix(ingest): caption-gated signal_description classification
- `dc3b34d` fix(evidence): row-scan contract detection, immune to Docling column-ordering bugs
- `3e6ab6c` fix(evidence): header-clue + positional column detection; AMBA 5 direction mapping
- `c8915fa` feat(ir): WidthHint enum — parametric widths + table width map for KG synthesis

## Current repository state
- active workspace member: `crates/specforge`
- runnable CLI surface includes `inspect`, `ingest`, `evidence`, `semantic`, `intent`, `adapt`, `enrich`, `validate`, and `nlp-enrich`
- canonical generated artifact roots are under `generated/source_ir/`, `generated/evidence_ir/`, `generated/semantic_ir/`, and `generated/intent_ir/`
- `subs/fsmgen/` is a local read-only reference checkout for `.fsm` behavior

## Completed technical work in this session
- `EvidenceIr::build()` no longer stops after a one-shot extraction block; it now runs a monotone convergence loop in `crates/specforge/src/ir/evidence.rs`
- new convergence helpers landed in `crates/specforge/src/ir/evidence.rs`, including:
  - `scan_encoding_tables_by_signal_anchor()`
  - `collect_discovered_enum_values()`
  - `extract_discovered_state_value_from_text()`
  - `extract_signal_polarity_from_prose()`
  - `apply_signal_polarity_to_constraints()`
  - `extract_dynamic_signal_constraints()`
  - `dedup_actor_signal_relations()`
  - `converge_evidence_extractions()`
- `synthesize_encoding_declarations()` now delegates to `synthesize_encoding_declarations_for_enum()` so anchored rescans and the original table pass share the same enum synthesis logic
- no new APB/AHB/AXI-specific enum/value list was hardcoded; discovered enum/value atoms come from extracted tables and synthesized `Enum ...` facts only
- `docling_backend.rs` also has pending table-kind classification changes that widen `classify_table_kind()` to use caption, header, and body-row evidence together, including body-aware encoding detection
- regression tests added:
  - `anchored_encoding_scan_unlocks_dynamic_value_constraint_extraction`
  - `prose_polarity_refines_asserted_constraint_kind`
- validation completed:
  - `cargo test --manifest-path Cargo.toml` → 98 passed
  - `cargo build --release --manifest-path Cargo.toml` → passed
  - `target/release/specforge validate generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json` → 90/100 EXCELLENT
  - `target/release/specforge validate generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json` → 95/100 EXCELLENT
  - `target/release/specforge validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json` → 90/100 GOOD
- refreshed generated `evidence_ir`, `semantic_ir`, and `intent_ir` artifacts for APB/AHB/AXI are still in the working tree and should be committed with the code

## Current working tree before commit
- modified tracked files currently include:
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/ir/evidence.rs`
  - `crates/specforge/src/ir/source/docling_backend.rs`
  - `generated/evidence_ir/...` for APB/AHB/AXI
  - `generated/semantic_ir/...` for APB/AHB/AXI
  - `generated/intent_ir/...` for APB/AHB/AXI
- tracked docs already refreshed for the pending commit:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `MEMORY.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
- the user explicitly called out `LIVE_ACHIEVEMENT_STATUS.md`, `ROADMAP.md`, `RUST_CODEBASE_ANALYSIS.md`, `crates/specforge/src/ir/evidence.rs`, and `crates/specforge/src/ir/source/docling_backend.rs` as files not to forget
- no approval is required during the commit workflow

## Exact next steps
1. write `git_message_brief.txt`
2. stage only the intended tracked files
3. commit with `git commit -F git_message_brief.txt` and include `Co-Authored-By: Oz <oz-agent@warp.dev>`
4. clear `git_message_brief.txt`
5. verify post-conditions:
   - `git ls-files --error-unmatch git_message_brief.txt` must fail
   - `wc -c git_message_brief.txt` must report `0`
   - `git status --short` must show only the expected leftovers, if any

## Remaining engineering gaps after this commit
- finish the markdown-marker alias garbage filter in `crates/specforge/src/commands/nlp_enrich.rs`
- choose between validation/back-annotation (`R7`) and actor-relative direction modeling (`R15`) as the next larger implementation slice
- downstream interfaces still flatten actor-aware relations too early

## If resuming from an interruption
1. read `README.md`
2. read `SESSION_BOOTSTRAP.md`
3. read `LIVE_ACHIEVEMENT_STATUS.md`
4. read `ROADMAP.md`
5. read `RUST_CODEBASE_ANALYSIS.md`
6. inspect `git --no-pager status --short`
7. continue the commit workflow unless the user redirects
