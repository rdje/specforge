# CHANGES

## 2026-04-09 (mdBook is now the canonical user-facing docs surface)

### Added: a real `mdBook` for layered user-facing documentation
- Added the canonical book scaffold under [docs/book/book.toml](/Users/richarddje/Documents/github/specforge/docs/book/book.toml) and [docs/book/src/SUMMARY.md](/Users/richarddje/Documents/github/specforge/docs/book/src/SUMMARY.md).
- Seeded the first layered chapter set for:
  - introduction
  - getting started
  - runtime and `doctor`
  - command workflow
  - pipeline model (`SourceIR`, `EvidenceIR`, `SemanticIR`, `IntentIR`)
  - reference material for generated artifacts, live docs, and troubleshooting

### Changed: root docs now point to the book instead of trying to be the full user-doc surface themselves
- [README.md](/Users/richarddje/Documents/github/specforge/README.md) now marks the `mdBook` as the canonical user-facing documentation path and explains how to build it locally.
- [USER_GUIDE.md](/Users/richarddje/Documents/github/specforge/USER_GUIDE.md) is now a compatibility pointer to the book instead of a second large parallel user-doc surface.

### Changed: CI now treats docs as first-class project quality, not an optional side task
- Added [scripts/run_docs_ci.sh](/Users/richarddje/Documents/github/specforge/scripts/run_docs_ci.sh) as the canonical local docs build entrypoint.
- [scripts/run_ci.sh](/Users/richarddje/Documents/github/specforge/scripts/run_ci.sh) now runs the mdBook build after Rust formatting and tests.
- [.github/workflows/ci.yml](/Users/richarddje/Documents/github/specforge/.github/workflows/ci.yml) now installs `mdbook v0.5.2` before running the shared CI script, so GitHub checks the same Rust + docs path that local CI runs.

### Validation
- `bash scripts/run_docs_ci.sh` → passed
- `bash scripts/run_ci.sh` → passed (`245/245` tests, then mdBook build)

## 2026-04-09 (abstract transport tables no longer leak into canonical AXI interfaces)

### Fixed: generic `Tx` / `Rx` transport-primitives no longer masquerade as top-level interface signals
- Tightened [evidence.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/evidence.rs) so `signal_description` tables are rejected from the top-level signal surface when they are really abstract transport exemplars: bare transport primitive names such as `VALID`, `PENDING`, `CRDT`, `CRDTSH`, `SHAREDCRD`, and `RP` combined with only `Tx` / `Rx` actor terms.
- This keeps real prefixed interface tables like `AWVALID`, `ARCRDT`, or `AWSHAREDCRD` intact, while preventing appendix-level transport teaching tables from authoring canonical declarations, actor relations, and semantic hints.

### Added: regression coverage for abstract transport-table leakage
- Added a focused evidence regression proving that a `Credited channel signals` table containing only abstract `Tx` / `Rx` transport primitives does not synthesize top-level signal declarations, actor relations, or semantic hints.
- Kept the existing standalone `VALID` / `READY` semantic regression green, so the fix stays narrow instead of globally banning simple protocols that really do use those signal names.

### Changed: AXI still scores `85/100 GOOD`, but the artifact is much cleaner and more honest
- Rebuilt AXI from `EvidenceIR -> SemanticIR -> IntentIR -> validate` and refreshed the four-artifact validation projection.
- The fake bare-transport `VALID` surface is gone from canonical AXI. Actor count dropped from `19` to `17`, unresolved consumer-less connectivity collapsed from `178` signals to `6`, and structural producer ambiguity dropped from `2` signal-connectivity conflicts to `1`.
- The remaining AXI residual/finding surface is now narrower and more truthful:
  - blocked handshake fallback moved from contested bare `VALID` to contested `CRVALID`
  - semantic conflicts are now specific to `AWAKEUP` and `CRVALID`
  - the remaining structural producer ambiguity is `ARCHUNKEN`
  - interface conflicts remain the infrastructure direction disagreement on `ACLK` / `ARESETN`

### Validation
- `cargo test --manifest-path Cargo.toml abstract_transport_signal_tables_do_not_become_top_level_interfaces -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml builds_semantic_ir_from_handshake_evidence -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/source_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json` → passed (`actor_count: 17`, `residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json` → passed (`behavior_count: 1202`, `constraint_count: 1244`)
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json` → passed (`85/100 GOOD`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-09 (axi field-like message tables no longer leak pseudo-signals)

### Fixed: field-like `Name | Width | Description` tables no longer masquerade as interface signal tables
- Tightened [evidence.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/evidence.rs) so top-level signal-table recovery now considers the nearest section title as well as the local caption and headers.
- Continued-page DVM message-field tables now stay classified as field-like context instead of leaking pseudo-signals such as `IS`, `PA`, and `COMPLETION` into `EvidenceIR`.

### Added: regression coverage for continued-page field-table leakage
- Added a focused evidence regression proving that a misclassified field-like `Name | Width | Description` continuation table does not synthesize fake signal declarations, polarity facts, or semantic hints.

### Changed: AXI live quality improved and its remaining gaps are more honest
- Rebuilt AXI from `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> validate` and refreshed the four-artifact validation projection.
- AXI improved from `84/100 GOOD` to `85/100 GOOD`.
- The fake `PA` / `COMPLETION` missing-producer warning is gone, the fake `IS` polarity conflict is gone, the canonical signal denominator dropped from `312` to `294`, graph-derived direction coverage improved from `57%` to `59%`, and the remaining AXI residual surface is now the single blocked handshake-name fallback on contested `VALID`.

### Validation
- `cargo test --manifest-path Cargo.toml misclassified_field_table_does_not_synthesize_fake_signal_semantics -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml field_like_width_table_does_not_leak_message_fields_as_signals -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/source_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json` → passed (`actor_count: 19`, `residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json` → passed (`behavior_count: 1202`, `constraint_count: 1272`)
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json` → passed (`85/100 GOOD`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (passive visual links no longer force semantic residuals)

### Fixed: ambiguous-visual residuals now require live semantic lift
- Tightened [semantic.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/semantic.rs) so `semantic_ambiguous_visual_grounding` is emitted only when ambiguous or unknown visual evidence actually survives into carried semantic observations.
- Passive figure references that are merely linked from prose no longer keep a semantic-stage residual alive by themselves.

### Added: regression coverage for passive-vs-live visual grounding
- Added a focused semantic regression proving that a passive ambiguous figure link does not emit a residual packet.
- Added a paired regression proving that an actually lifted visual-backed semantic observation still keeps the residual visible when the visual role remains ambiguous.

### Changed: APB, AHB, and AXI now carry zero residual decisions
- Rebuilt `SemanticIR` / `IntentIR` / validation for the live APB, AHB, and AXI artifacts and refreshed the tracked four-artifact projection.
- APB, AHB, and AXI now all carry `0` residual decisions end to end; the old common `semantic_ambiguous_visual_grounding` residual is gone because those live artifacts were only carrying passive figure links, not active visual semantic lift.
- The refreshed live projection is now AXI `85/100 GOOD`, APB `94/100 EXCELLENT`, AHB `94/100 EXCELLENT`, and AXI-Stream `90/100 EXCELLENT`; a follow-on rebuild from current `SourceIR` / `EvidenceIR` restored APB and AHB to the excellent lane while leaving AXI as the main live quality outlier.

### Validation
- `cargo test --manifest-path Cargo.toml passive_ambiguous_visual_links_do_not_emit_residual_decision -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml emits_residual_decision_for_ambiguous_visual_semantic_grounding -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/evidence_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (authoritative signal surface now anchors interface grouping)

### Fixed: heuristic interface grouping now respects declared signal vocabularies
- Tightened [semantic.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/semantic.rs) so statement-derived interface fragments are filtered against document-grounded explicit signal declarations whenever that authoritative signal surface exists.
- This means phase words, enum labels, width symbols, and similar metadata no longer survive into heuristic interface grouping just because they were co-mentioned next to real signals in prose.

### Added: regression coverage for authoritative grouping filters
- Added a focused semantic regression proving authoritative signal vocabularies suppress undeclared metadata like `SETUP` / `ACCESS` while retaining real declared signals.

### Changed: APB, AHB, and AXI all lost the carried interface-grouping residual
- Rebuilt `SemanticIR` / `IntentIR` / validation for the live APB, AHB, and AXI artifacts and refreshed the tracked four-artifact projection.
- `semantic_interface_grouping` is gone across all three; a later follow-up also removed the remaining passive visual residuals, so the current live baseline no longer carries any residual decisions on APB/AHB/AXI.
- AXI improved from the stale projected `79/100 GOOD` back to `84/100 GOOD`; APB remains `84/100 GOOD`, AHB later moved to `85/100 GOOD`, and AXI-Stream stays `90/100 EXCELLENT`.

### Validation
- `cargo test --manifest-path Cargo.toml retain_authoritative_interface_candidate_signals_prefers_declared_surface -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml overlapping_interface_signals_ignore_fragments_subsumed_by_explicit_interfaces -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/evidence_ir.json` → passed (`residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json` → passed (`residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json` → passed (`residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed
- `bash scripts/run_ci.sh` → passed
## 2026-04-08 (AXI-Stream interface grouping residual removed cleanly)

### Fixed: heuristic interface grouping now ignores width/table metadata noise
- Tightened [semantic.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/semantic.rs) so heuristic interface grouping filters out metadata-only symbols like `*_WIDTH`, `_WIDTH`, `MIN`, and `MAX` before building statement-derived interface fragments.
- This complements the earlier signal-token gate that already rejected leading-digit hex-like values such as `0A`, `0B`, `0E`, and `0F`.

### Fixed: explicit interfaces now subsume smaller grouped fragments for overlap review
- `semantic_interface_grouping` residual generation now ignores heuristic fragments that are fully subsumed by an explicit interface, so carried overlap is only reported when there is still a real unresolved grouping question.
- In AXI-Stream, that resolves the last carried residual decision instead of preserving a bookkeeping artifact caused by one explicit interface plus many smaller statement fragments.

### Added: regression coverage for metadata filtering and explicit-subsumption overlap handling
- Added focused semantic regressions proving width/table metadata is filtered from heuristic interface candidates.
- Added focused semantic regressions proving explicit interfaces suppress already-subsumed overlap while genuinely unsubsumed heuristic overlap still remains visible.

### Changed: AXI-Stream now carries zero residual decisions without score inflation
- Rebuilt AXI-Stream `SemanticIR` and `IntentIR`, then refreshed the tracked four-artifact validation projection.
- AXI-Stream remains at `90/100 EXCELLENT`, but interface count drops from `46` to `29`, `SemanticIR` / `IntentIR` residual decisions both drop to `0`, and the only remaining projected finding is the infrastructure `system_contract` note for `ACLK` / `ARESETN`.

### Validation
- `cargo test --manifest-path Cargo.toml filtered_interface_candidate_signals_drop_width_and_table_metadata_noise -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml overlapping_interface_signals_ignore_fragments_subsumed_by_explicit_interfaces -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml overlapping_interface_signals_keep_unsubsumed_heuristic_overlap_visible -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json` → passed (`interface_count: 29`, `residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`90/100 EXCELLENT`, `residual_decisions: 0`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (doctor now checks LM Studio fallback readiness too)

### Added: doctor now verifies the LM Studio fallback path as well as the default Ollama path
- Extended [doctor.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/commands/doctor.rs) so `specforge doctor [--strict]` now checks and reports:
  - LM Studio `/v1/models`
  - default-model presence for `qwen2.5vl:7b`
  - LM Studio OpenAI-compatible `/v1/chat/completions`
- The strict gate still reflects the default local-first pipeline (`Docling` + `Ollama`), but the CLI now surfaces whether the `lmstudio` fallback is actually usable before a long rerun depends on it.

### Added: shared OpenAI-compatible parsing for local provider preflight
- `doctor.rs` now parses OpenAI-compatible `/v1/models` payloads and reuses the same chat-response parser for both Ollama and LM Studio, instead of keeping the loopback preflight logic Ollama-specific.
- Added focused unit coverage for `/v1/models` parsing and kept the OpenAI-compatible chat parsing under test.

### Changed: the live runtime picture is now more honest
- Verified outside the sandbox that `cargo run --manifest-path Cargo.toml -- doctor --strict` now reports:
  - Docling ready via `python3.11` + `docling 2.84.0`
  - Ollama loopback fully ready for `qwen2.5vl:7b`
  - LM Studio fallback not currently reachable at `http://localhost:1234`, even though LM Studio is installed locally
- That distinction matters: “installed” is not the same as “serving a model,” and `doctor` now makes that operational difference explicit.

### Validation
- `cargo test --manifest-path Cargo.toml parse_openai_models_response_detects_default_model -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml parse_openai_chat_response_accepts_openai_compatible_string_content -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml doctor_defaults_to_non_strict -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- doctor --strict` → passed (outside sandbox; Docling + Ollama ready, LM Studio fallback reported unavailable)

## 2026-04-08 (doctor now checks Ollama loopback readiness too)

### Added: doctor now verifies the default local Ollama runtime, not just Docling
- Extended [doctor.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/commands/doctor.rs) so `specforge doctor [--strict]` now checks:
  - Docling ingest readiness
  - Ollama `/api/tags`
  - default-model presence for `qwen2.5vl:7b`
  - Ollama OpenAI-compatible `/v1/chat/completions`
- This catches the exact failure mode discovered during the fresh AXI rerun: a long `converge` can otherwise get all the way through fresh ingest before discovering that the local chat-completions path is not actually usable in the current execution environment.

### Added: typed parsing and reporting for the default Ollama loopback path
- `doctor.rs` now parses visible Ollama models from `/api/tags`, validates OpenAI-compatible chat responses from `/v1/chat/completions`, and reports both readiness and resolution text explicitly.
- Added focused unit coverage for Ollama tags parsing and chat-response parsing.

### Changed: the local runtime preflight now covers the full default local-first pipeline
- Verified outside the sandbox that `cargo run --manifest-path Cargo.toml -- doctor --strict` now reports:
  - Docling ready via `python3.11` + `docling 2.84.0`
  - Ollama tags reachable
  - Ollama chat-completions reachable
  - default model `qwen2.5vl:7b` present
- That means the default `specforge converge` runtime can now be preflighted honestly before a large PDF run instead of discovering the Ollama-side failure deep into the loop.

### Validation
- `cargo test --manifest-path Cargo.toml doctor_defaults_to_non_strict -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml parse_ollama_tags_response_detects_default_model -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml parse_ollama_chat_response_accepts_openai_compatible_string_content -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml parse_ollama_chat_response_accepts_array_content -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- doctor --strict` → passed (outside sandbox; Docling + Ollama loopback both ready)

## 2026-04-08 (Docling runtime discovery, doctor command, and bootstrap path)

### Added: a first-class Docling runtime doctor command
- Added [doctor.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/commands/doctor.rs) and wired `specforge doctor [--strict]` into the CLI in [cli.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/cli.rs), [commands/mod.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/commands/mod.rs), and [lib.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/lib.rs).
- The new command reports Docling readiness, the selected Python candidate, version information, all probe results, the repo-local bootstrap script path, and the exact missing-runtime resolution when `--strict` is used.

### Changed: Docling runtime discovery is now operationally stronger
- Extended [docling_backend.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/source/docling_backend.rs) so the backend no longer depends only on ambient `python3` / `python`.
- Runtime resolution now proceeds in this order:
  - `SPECFORGE_DOCLING_PYTHON`
  - repo-local `.venv-docling`
  - versioned Python probes such as `python3.11`, `python3.12`, and `python3.10`
  - generic `python3` / `python`
- The resolver now keeps a typed diagnosis surface instead of a one-bit import probe, which is shared by both `specforge doctor` and the actual ingest backend.

### Added: supported repo-local Docling bootstrap path
- Added [bootstrap_docling.sh](/Users/richarddje/Documents/github/specforge/scripts/bootstrap_docling.sh) as the supported repository-local Docling runtime bootstrap entrypoint.
- The script creates `.venv-docling`, installs the known-good `docling==2.84.0` runtime family by default, and prints the resulting interpreter/version state.
- Added `/.venv-docling/` to [.gitignore](/Users/richarddje/Documents/github/specforge/.gitignore) so that runtime stays local and untracked.

### Changed: the local runtime issue is now concretely verified, not just documented
- `cargo run --manifest-path Cargo.toml -- doctor --strict` now succeeds locally and selects `python3.11` with `docling 2.84.0`, while explicitly reporting that the ambient `python3` probe is still broken because it resolves to Python `3.14.3` without `docling`.
- A fresh original-PDF ingest rerun on the AHB spec now succeeds again:
  - `cargo run --manifest-path Cargo.toml -- ingest /Users/richarddje/Documents/livework/chipdoc/arm/amba/core/ahb/current/IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf`
  - result: `normalization_status: ready`, `page_artifact_count: 104`, `visual_asset_count: 70`

### Validation
- `cargo test --manifest-path Cargo.toml doctor_defaults_to_non_strict -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml inspect_docling_runtime_prefers_repo_local_venv -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml inspect_docling_runtime_prefers_python311_path_probe_over_generic_python3 -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- doctor --strict` → passed
- `cargo run --manifest-path Cargo.toml -- ingest /Users/richarddje/Documents/livework/chipdoc/arm/amba/core/ahb/current/IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf` → passed


## 2026-04-08 (system-contract infrastructure signals now populate canonical interfaces)

### Fixed: clock/reset signals from the system contract now reach the canonical interface surface
- Extended [semantic.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/semantic.rs) so grounded `system_contract` clock/reset signals are synthesized into the top-level explicit interface when ordinary signal declarations do not already carry them.
- This lets infrastructure signals like `HCLK` and `HRESETN` contribute honest canonical interface direction/width coverage, and it allows reset polarity grounded only through system-contract text to surface as `resolved_polarity` in both `SemanticIR` and `IntentIR`.
- Added focused regressions in [semantic.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/semantic.rs) and [validate.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/commands/validate.rs) covering the exact system-contract-only clock/reset case at semantic and intent validation time.

### Changed: AHB now reports resolved polarity in the live baseline
- Rebuilt AHB `SemanticIR` and `IntentIR` sequentially from the current `EvidenceIR`, re-validated the artifact, and refreshed the tracked four-document projection.
- AHB now reports `with_resolved_polarity: 1` and the live AMBA polarity line is now `1 / 1 / 1 / 1`; the overall AHB score stays `84/100 GOOD`, but the infrastructure reset polarity is now represented honestly in the canonical interface surface.
- A full original-PDF `converge` rerun was attempted first, but the local environment currently lacks an importable `docling` runtime for `python3`, so authoritative fresh-ingest reruns remain blocked until `docling` is installed or `SPECFORGE_DOCLING_PYTHON` points at a working interpreter.

### Validation
- `cargo test --manifest-path Cargo.toml system_contract_signals_become_explicit_interface_records -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml validate_semantic_ir_counts_system_contract_resolved_polarity -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml validate_intent_ir_counts_system_contract_resolved_polarity -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json` → passed (`84/100 GOOD`, `with_resolved_polarity: 1`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (resolved signal polarity now lives on canonical interface records)

### Added: canonical interface records now carry resolved polarity directly
- Extended [semantic.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/semantic.rs) so each `InterfaceSignalRecord` can now carry `resolved_polarity` directly instead of forcing downstream consumers to reconstruct polarity only from the carried top-level side list.
- [intent.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/intent.rs) now preserves that same per-signal polarity surface into `IntentIR`.
- [validate.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/commands/validate.rs) now reports `with_resolved_polarity` for both `SemanticIR` and `IntentIR`.

### Changed: the live corpus now shows the gap honestly
- Rebuilt the live AMBA `SemanticIR` / `IntentIR` artifacts, re-validated the four-document projection, and refreshed the tracked snapshot docs.
- The canonical polarity surface is now present in the live corpus too: AXI, APB, AHB, and AXI-Stream each currently report `with_resolved_polarity: 1`, so the remaining polarity work is broader non-reset control coverage rather than carry-through plumbing.
- That refresh also replaced a stale optimistic validation snapshot; after the later current-`SourceIR` / current-`EvidenceIR` rebuild and the follow-on AXI field-table truthfulness fix, the tracked live baseline now stands at AXI `85/100 GOOD`, APB `94/100 EXCELLENT`, AHB `94/100 EXCELLENT`, and AXI-Stream `90/100 EXCELLENT`.

### Validation
- `cargo test --manifest-path Cargo.toml carries_resolved_signal_polarity_into_interface_records -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml carries_resolved_signal_polarity_into_intent_ir -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml validate_semantic_ir_counts_resolved_signal_polarity -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml validate_intent_ir_counts_resolved_signal_polarity -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/semantic_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`90/100 EXCELLENT`, `with_resolved_polarity: 1`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (temporal conflict comparison is now polarity-aware)

### Fixed: asserted/deasserted temporal semantics now respect signal polarity
- Extended [semantic.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/semantic.rs), [evidence.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/evidence.rs), and [intent.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/intent.rs) so resolved signal polarity now survives into canonical IR and can guide temporal-conflict comparison.
- `ASSERTED` and `DEASSERTED` are now treated as polarity-relative assertion semantics, not as fixed synonyms for `HIGH` and `LOW`.
- When the current document grounds polarity, conflict detection now maps assertion semantics through that local polarity:
  - active-high: `ASSERTED -> HIGH`, `DEASSERTED -> LOW`
  - active-low: `ASSERTED -> LOW`, `DEASSERTED -> HIGH`
- When polarity is still unknown, `ASSERTED` stays abstract instead of manufacturing or suppressing a level conflict.

### Changed: AXI-Stream timing semantics are now cleaner again without changing the score
- Rebuilt `SemanticIR` and `IntentIR` for AXI-Stream from the current `EvidenceIR`, re-validated the artifact, and refreshed the tracked four-document projection.
- The score stayed at `90/100 EXCELLENT`, but AXI-Stream now carries `0` typed temporal conflicts instead of `1`.
- The remaining dominant honest gaps are now:
  - the infrastructure-sourcing/system-contract note for `ACLK` and `ARESETN`
  - the carried `semantic_interface_grouping` residual decision

### Validation
- `cargo test --manifest-path Cargo.toml derives_typed_temporal_conflicts_from_conflicting_value_rules -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml asserted_and_high_do_not_form_temporal_conflicts_without_known_polarity -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml asserted_and_high_form_temporal_conflict_for_active_low_signal -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/semantic_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`90/100 EXCELLENT`, `temporal_conflicts: 0`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (same-cycle timing language now lands as bounded temporal semantics)

### Fixed: same-cycle timing phrases now recover explicit `0`-cycle windows
- Extended [semantic.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/semantic.rs) so `extract_cycle_window_from_text()` now recognizes bounded same-cycle language such as `in the same ACLK cycle`, `in the same tick`, and `on the current rising edge`.
- Added focused semantic regressions that lock both layers of the behavior:
  - direct phrase recovery from same-cycle timing language
  - end-to-end temporal-rule derivation from a same-cycle signal constraint

### Changed: AXI-Stream timing semantics are now more explicit without changing the score
- Re-ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- Re-validated the rebuilt artifact and refreshed the tracked four-document projection.
- The score stayed at `90/100 EXCELLENT`, but six AXI-Stream temporal rules now carry explicit `0`-cycle windows for same-cycle handshake/timing language, so the old `intent_temporal_rules_missing_cycle_windows` warning is gone.
- The timing surface also got cleaner as a side effect: AXI-Stream now carries `1` typed temporal conflict instead of `2`.
- The remaining dominant honest gaps are now:
  - the dedicated infrastructure-sourcing note for `ACLK` / `ARESETN`
  - the single remaining typed temporal conflict
  - the carried `semantic_interface_grouping` residual decision

### Validation
- `cargo test --manifest-path Cargo.toml extracts_zero_cycle_window_from_same_cycle_phrases -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml derives_zero_cycle_window_from_same_cycle_constraint_text -- --nocapture` → passed
- full `specforge converge` on AXI-Stream with Ollama VLM + NLP Level 3 → converged in `2` iterations
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`90/100 EXCELLENT`, `temporal_rules_with_cycle_window: 6`, `temporal_conflicts: 1`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (clock/reset connectivity now validates as infrastructure)

### Changed: clock/reset connectivity is now classified as infrastructure in canonical IR
- Extended [semantic.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/semantic.rs) so `SignalConnectivityRecord` now carries an explicit `connectivity_class`, with `SystemClock` and `SystemReset` derived from the local system contract instead of flattening those signals into ordinary protocol connectivity.
- Extended [intent.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/intent.rs) so that infrastructure classification survives into `IntentIR` unchanged.
- Extended [validate.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/commands/validate.rs) so missing producers on infrastructure connectivity no longer emit the generic `[warning:signal_connectivity]` finding; they now surface as a dedicated `[info:system_contract]` note that keeps canonical sourcing on the system-contract side of the model.

### Changed: AXI-Stream still validates at `90/100 EXCELLENT`, but the remaining gap is now represented more honestly
- Re-ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- Re-validated the rebuilt artifact and refreshed the tracked four-document projection.
- The score stayed at `90/100 EXCELLENT`, with declared graph-direction and width coverage still at `22/22`, but `ACLK` and `ARESETN` now surface under `infrastructure_signal_connectivity: 2` with an `[info:system_contract]` finding instead of a generic missing-producer warning.
- The dominant remaining honest gaps are now:
  - typed temporal rules that still have no explicit cycle-window bounds
  - the two carried temporal conflicts
  - the remaining interface-grouping residual decision

### Validation
- `cargo test --manifest-path Cargo.toml clock_and_reset_gain_input_actor_ports_for_relation_actors -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml carries_infrastructure_signal_connectivity_class_into_intent_ir -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml validate_intent_ir_treats_clock_and_reset_as_infrastructure_connectivity -- --nocapture` → passed
- full `specforge converge` on AXI-Stream with Ollama VLM + NLP Level 3 → converged in `2` iterations
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`90/100 EXCELLENT`, `infrastructure_signal_connectivity: 2`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (corpus knowledge base plane added to roadmap)

### Added: explicit `R15g` workstream for a corpus knowledge base layer
- Logged a new roadmap slice in [ROADMAP.md](/Users/richarddje/Documents/github/specforge/ROADMAP.md) for a persistent corpus knowledge base that sits beside the per-document IR pipeline and the typed `CorpusMemory` prior store.
- The design boundary is explicit:
  - per-document canonical truth stays in `SourceIR` / `EvidenceIR` / `SemanticIR` / `IntentIR`
  - typed machine-usable reuse stays in `CorpusMemory`
  - the new corpus knowledge base becomes the human+LLM synthesis layer for recurring motifs, failures, contradiction summaries, table/figure families, and protocol-family notes

### Changed: live architecture guidance now targets three cross-document planes, not one
- Updated [DEVELOPMENT_NOTES.md](/Users/richarddje/Documents/github/specforge/DEVELOPMENT_NOTES.md), [README.md](/Users/richarddje/Documents/github/specforge/README.md), and [LIVE_ACHIEVEMENT_STATUS.md](/Users/richarddje/Documents/github/specforge/LIVE_ACHIEVEMENT_STATUS.md) so future work treats the long-term shape as:
  - document-local canonical IR
  - typed cross-document priors
  - corpus-level compiled knowledge base
- The docs also now make the safety boundary explicit: the corpus knowledge base may guide humans, LLM synthesis, benchmark design, and prior-candidate generation, but it must not directly author canonical IR truth.

## 2026-04-08 (AXI-Stream parity-check width semantics now survive end to end)

### Fixed: parity-check rows now recover bounded width hints from their local table semantics
- Extended [evidence.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/evidence.rs) so `Check Signal / Signals Covered / Width / Granularity / Check Enable` rows can recover width hints from the `Signals Covered` cell when the literal `Width` cell is only a range placeholder like `1-8`.
- Added a bounded fallback from `Check Enable` / `Granularity` to the grounded base signal when `Signals Covered` only carries a width expression, so the structural and width semantics stay tied to local evidence instead of remaining partially orphaned.
- Tightened graph-derived declaration synthesis so width-only statements no longer block stronger relation-grounded `Signal X is output width ...` declarations for the same signal.

### Changed: AXI-Stream now validates at `90/100 EXCELLENT`
- Re-ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- The run still converged in `2` pipeline iterations, but the carried `*CHK` surface is now complete enough to count honestly: declared signal records rose from `21` to `22`, actor-signal relations rose from `38` to `40`, actor ports rose from `42` to `44`, signal connectivity rose from `21` to `22`, compatibility direction hints reached `22/22`, width coverage reached `22/22`, and the projected score improved from `88/100 GOOD` to `90/100 EXCELLENT`.
- The remaining dominant gaps are now:
  - unresolved producer attribution for infrastructure signals `ACLK` and `ARESETN`
  - typed temporal rules that still have no explicit cycle-window bounds
  - the two carried temporal conflicts

### Validation
- `cargo test --manifest-path Cargo.toml check_signal_tables_inherit_relations_from_covered_signals -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml source_table_relations_infer_unique_complementary_reads -- --nocapture` → passed
- full `specforge converge` on AXI-Stream with Ollama VLM + NLP Level 3 → converged in `2` iterations
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`90/100 EXCELLENT`, declared graph-direction and width coverage `22/22`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (AXI-Stream parity-check table now restores structural ownership)

### Fixed: parity-check tables now recover actor-signal relations from covered base signals
- Extended [evidence.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/evidence.rs) with a bounded second-pass relation recovery path for `Check Signal / Signals Covered` tables.
- When a local parity-check row explicitly ties a check signal to a covered base signal that already has grounded actor relations, the check signal now inherits those local `drives` / `reads` edges instead of remaining structurally orphaned.
- Added the focused regression `check_signal_tables_inherit_relations_from_covered_signals`.

### Changed: AXI-Stream now validates at `88/100 GOOD`
- Re-ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- The run still converged in `2` pipeline iterations, but parity-check ownership now survives end to end: declared signal records rose from `16` to `21`, graph-direction coverage rose from `12/16` to `21/21`, actor ports rose from `24` to `42`, signal connectivity rose from `12` to `21`, and the projected score improved from `84/100 GOOD` to `88/100 GOOD`.
- The remaining dominant gaps are now:
  - missing widths on `TDESTCHK`, `TIDCHK`, `TSTRBCHK`, `TUSERCHK`, and `TWAKEUPCHK`
  - unresolved producer attribution for infrastructure signals `ACLK` and `ARESETN`

### Validation
- `cargo test --manifest-path Cargo.toml check_signal_tables_inherit_relations_from_covered_signals -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml source_table_relations_infer_unique_complementary_reads -- --nocapture` → passed
- full `specforge converge` on AXI-Stream with Ollama VLM + NLP Level 3 → converged in `2` iterations
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`88/100 GOOD`, graph-direction coverage `21/21`)

## 2026-04-08 (clock/reset semantics logged as infrastructure-first steering)

### Changed: design steering now treats clocks and resets as infrastructure semantics, not ordinary protocol edges
- Logged the implementation doctrine in [DEVELOPMENT_NOTES.md](/Users/richarddje/Documents/github/specforge/DEVELOPMENT_NOTES.md): clocks and resets should remain first-class infrastructure semantics with conservative sourcing/distribution modeling, not flattened into ordinary protocol producer/consumer behavior.
- Updated [ROADMAP.md](/Users/richarddje/Documents/github/specforge/ROADMAP.md) so `R15b` now explicitly carries that requirement forward into the clock-tick temporal-model workstream.
- This locks an important architectural boundary for future work on `ACLK`, `ARESETN`, and similar infrastructure signals: graph carry-through is allowed as a local aid, but long-term canonical truth should prefer dedicated infrastructure semantics over false graph completeness.

## 2026-04-08 (AXI-Stream graph-direction coverage rises after width-symbol cleanup)

### Fixed: width-only `_WIDTH` declarations no longer masquerade as interface signals
- Hardened [semantic.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/semantic.rs) so synthesized declarations like `Signal TDATA_WIDTH is width LOW.` no longer become canonical interface-signal records when they carry width metadata but no real port direction.
- Added focused regressions for both sides of the boundary:
  - `width_only_width_parameter_declarations_do_not_become_interface_signal_records`
  - `width_only_signal_declarations_become_interface_signal_records`

### Fixed: relation-grounded actors now inherit clock/reset input ports from explicit system contracts
- Extended [semantic.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/semantic.rs) so actors already grounded by structural KG evidence now receive `input` actor ports for the explicit clock and reset signals instead of leaving `ACLK` / `ARESETN` outside the graph-backed port surface.
- Added the regression `clock_and_reset_gain_input_actor_ports_for_relation_actors`, which locks that actor-relative clock/reset carry-through path.

### Changed: AXI-Stream now validates at `84/100 GOOD`
- Re-ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- The run still converged in `2` pipeline iterations, but the canonical denominator is now more honest: declared interface signals dropped from `20` to `16`, graph-direction coverage rose from `10/20` to `12/16`, and the projected score improved from `80/100 GOOD` to `84/100 GOOD`.
- The remaining dominant gaps are now narrower and clearer:
  - the four `*CHK` signals still lack graph-derived direction coverage
  - `ACLK` and `ARESETN` still lack resolved producer actors even though they now have graph-backed consumer ports

### Validation
- `cargo test --manifest-path Cargo.toml width_only_width_parameter_declarations_do_not_become_interface_signal_records -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml clock_and_reset_gain_input_actor_ports_for_relation_actors -- --nocapture` → passed
- full `specforge converge` on AXI-Stream with Ollama VLM + NLP Level 3 → converged in `2` iterations
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`84/100 GOOD`, graph-direction coverage `12/16`)

## 2026-04-08 (AXI-Stream consumer-side connectivity now survives from source tables)

### Fixed: source-column signal tables can now recover the opposite-side reader when it is uniquely grounded
- Extended [evidence.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/evidence.rs) so a `Source` / `Driver` column no longer stops at `(actor, Drives, signal)` when the current document already exposes exactly one opposite actor role locally.
- The new helper path builds a small local actor-role inventory from signal-description tables and section headings, then adds the complementary `Reads` edge only when the opposite requester-like/completer-like actor is unique.
- Added focused regressions for both the positive case and the ambiguity guard:
  - `source_table_relations_infer_unique_complementary_reads`
  - `source_table_relations_skip_complementary_reads_when_opposite_actor_is_ambiguous`
- Updated the tracked KG fixtures whose expected graph shape now honestly includes these complementary consumer edges.

### Changed: AXI-Stream now keeps consumer-side structural connectivity without changing its score
- Re-ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- The run still converged in `2` pipeline iterations and still validates at `80/100 GOOD`, but `IntentIR` now carries the missing consumer-side structural KG edges:
  - `Receiver reads TVALID`
  - `Transmitter reads TREADY`
  - `Receiver reads TDATA/TSTRB/TKEEP/TLAST/TID/TDEST/TUSER/TWAKEUP`
- The AXI-Stream validation finding for missing consumer actors is now gone; the remaining dominant gap is graph-derived direction coverage, not missing connectivity.

### Validation
- `cargo test --manifest-path Cargo.toml source_table_relations_infer_unique_complementary_reads -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml source_table_relations_skip_complementary_reads_when_opposite_actor_is_ambiguous -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml actor_taxonomy_priors_guide_source_column_direction_inference -- --nocapture` → passed
- full `specforge converge` on AXI-Stream with Ollama VLM + NLP Level 3 → converged in `2` iterations
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`80/100 GOOD`, consumer-gap finding removed)

## 2026-04-08 (learning plane now rejects bogus actor vocabulary)

### Fixed: actor-taxonomy learning no longer harvests payload nouns as actors
- Hardened [learn_priors.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/commands/learn_priors.rs) so actor-taxonomy priors now skip non-actor payload/event terms like `control information`, even if an earlier document-local bug let that text survive into `IntentIR`.
- Added a shared actor-term hygiene guard in [prior_memory.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/prior_memory.rs) so actor-taxonomy prior lookup also ignores those bogus terms if an older local `CorpusMemory` still contains stale entries.
- Reused the same guard in [evidence.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/evidence.rs), so live relation extraction and cross-document learning now reject the same class of bogus actor terms instead of drifting apart.

### Changed: local `CorpusMemory` is now cleaned of the stale AXI-Stream actor prior
- Re-ran `specforge learn-priors` across AXI/APB/AHB/AXI-Stream `IntentIR` artifacts.
- The local prior store now yields `16` actor-taxonomy priors, `5` semantic phrase priors, `4` semantic modality-reliability priors, `266` temporal phrase priors, and `99` table-shape priors.
- The stale `control information -> requester_like` actor-taxonomy prior is now gone from local `generated/prior_memory/corpus_memory.json`.

### Validation
- `cargo test --manifest-path Cargo.toml learn_priors_skips_payload_like_actor_terms -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- learn-priors generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`16 / 5 / 4 / 266 / 99`)
- `bash scripts/run_ci.sh` → passed (`209/209` tests)

## 2026-04-08 (AXI-Stream bogus prose actor extraction fixed)

### Fixed: prose KG extraction no longer promotes payload nouns into actors
- Hardened [evidence.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/evidence.rs) so prose actor extraction now normalizes candidate actor phrases through the same non-actor guard used by table extraction and rejects generic payload/event nouns like `control information`, `data`, and `transfer`.
- Tightened active-clause subject recovery so coordinated prose like `the Transmitter presents ... and asserts TVALID` keeps the real actor subject instead of capturing trailing payload phrases or clause verbs.
- Added the regression `coordinated_active_drive_extracts_real_actor_not_payload_phrase`, which locks the AXI-Stream-style sentence shape that previously leaked `control information` into the structural KG.

### Changed: AXI-Stream structural truthfulness improved without score inflation
- Re-ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- The artifact still converges in `2` pipeline iterations and still validates at `80/100 GOOD`, but the carried multi-producer conflict on `TVALID` is now gone: `control information` no longer appears as an actor in `EvidenceIR`, `SemanticIR`, or `IntentIR`, and `signal_connectivity_conflicts` for AXI-Stream dropped from `1` to `0`.
- The remaining honest AXI-Stream gap is now clearer: unresolved consumer actors and graph-direction coverage, not bogus producer attribution.

### Validation
- `cargo test --manifest-path Cargo.toml coordinated_active_drive_extracts_real_actor_not_payload_phrase -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed
- `bash scripts/run_ci.sh` → passed (`208/208` tests)

## 2026-04-07 (AXI-Stream unseen-protocol run populates semantic priors)

### Added: first unseen-protocol full converge + learning refresh
- Ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- The AXI-Stream artifact converged in `2` pipeline iterations and validates at `80/100 GOOD`; it is now included in the tracked [VALIDATION_SNAPSHOT.md](/Users/richarddje/Documents/github/specforge/VALIDATION_SNAPSHOT.md) projection and the managed validation block in [LIVE_ACHIEVEMENT_STATUS.md](/Users/richarddje/Documents/github/specforge/LIVE_ACHIEVEMENT_STATUS.md).
- Refreshing `specforge learn-priors` across AXI/APB/AHB/AXI-Stream now yields `16` actor-taxonomy priors, `5` semantic phrase priors, `4` semantic modality-reliability priors, `266` temporal phrase priors, and `99` table-shape priors in local `CorpusMemory`.

### Fixed: multi-signal table-row semantic-role leakage
- Extended [evidence.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/evidence.rs) so signal-description table rows now sanitize against the full local known-signal set before semantic-role inference, preventing a secondary signal mention like `TREADY` from leaking a ready-like role onto a row subject like `TVALID`.
- Added the regression `signal_table_descriptions_ignore_other_handshake_signal_mentions`, which locks the AXI-Stream-style case where `TVALID` should stay valid-like even when its row also describes the handshake condition involving `TREADY`.

### Why this matters
- This is the first concrete proof that a new unseen protocol document can both expose an extraction flaw and then materially strengthen the learning plane once the artifact is repaired enough to be harvested.
- It also marks the first real-corpus point where semantic phrase priors and semantic modality-reliability priors become nonzero instead of remaining only architecturally possible.

## 2026-04-07 (learning-plane structure documented as first-class architecture)

### Added: explicit doctrine for how `R15f` should learn
- Logged in [DEVELOPMENT_NOTES.md](/Users/richarddje/Documents/github/specforge/DEVELOPMENT_NOTES.md) that the cross-document learning plane should not try to "mimic humans completely," but should instead borrow the useful structural properties of human learning:
  - accumulate experience across many documents
  - abstract patterns from repeated successful cases
  - keep confidence graded
  - remember failures and false positives
  - use prior experience to guide attention
  - still require local evidence before promoting a fact
- Logged the non-negotiable architectural properties for the learning plane:
  - separation between document truth and learned priors
  - typed, inspectable memory
  - bounded influence
  - validation-gated feedback
  - negative learning
  - provenance on learned priors

### Why this matters
- This frames `R15f` as an epistemology layer, not just a bigger cache.
- It makes explicit that the design risk is not "too little learning," but poorly structured learning that can poison canonical truth.

## 2026-04-07 (table-shape timing-table benchmark added)

### Added: second table-shape gold/negative pair
- Added [table_shape_prior_guided_timing_table_gold](/Users/richarddje/Documents/github/specforge/crates/specforge/test_data/kg_quality/table_shape_prior_guided_timing_table_gold/fixture.json) and [table_shape_prior_guided_timing_table_without_prior_negative](/Users/richarddje/Documents/github/specforge/crates/specforge/test_data/kg_quality/table_shape_prior_guided_timing_table_without_prior_negative/fixture.json).
- The pair proves a locally `unknown` `Parameter | Min | Max | Unit` table stays inert without prior memory and yields `timing_constraints = 1` across `EvidenceIR`, `SemanticIR`, and `IntentIR` only when a matching table-shape prior is staged.

### Why this matters
- It broadens the first table-shape prior family beyond signal-description recovery and shows that the same bounded consumer already generalizes to timing-table interpretation.

## 2026-04-07 (semantic modality-reliability priors landed as the fifth bounded learning slice)

### Added: first semantic modality-reliability prior family in `CorpusMemory`
- Extended [prior_memory.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/prior_memory.rs) with typed `semantic_modality_reliability_priors` plus advisory lookup helpers that score how reliable a given semantic source kind has been for a given role and protocol family.
- Extended [learn_priors.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/commands/learn_priors.rs) so `specforge learn-priors` now harvests those priors from decisive, non-alias-dependent semantic consensus records rather than from raw guesses.
- The latest local AMBA run over APB/AHB/AXI still yields `0` semantic modality-reliability priors, which is the honest current state: the family is landed, but the real canonical artifacts are not yet surfacing enough promoted semantic consensus to populate it automatically.

### Added: bounded semantic arbitration consumption and benchmark coverage
- Extended [semantic.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/semantic.rs) so `SemanticIR` can advisory-adjust local semantic arbitration using modality-reliability priors, but only when the current PDF already contains multiple locally grounded semantic candidates; the original conflict remains visible either way.
- Extended [validate.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/commands/validate.rs) so semantic and intent validation now report prior-guided semantic arbitration and prior-guided semantic consensus explicitly instead of hiding that path inside the canonical result.
- Added the tracked gold/negative fixture pair [semantic_modality_reliability_prior_guided_conflict_gold](/Users/richarddje/Documents/github/specforge/crates/specforge/test_data/kg_quality/semantic_modality_reliability_prior_guided_conflict_gold/fixture.json) and [semantic_modality_reliability_prior_guided_conflict_without_prior_negative](/Users/richarddje/Documents/github/specforge/crates/specforge/test_data/kg_quality/semantic_modality_reliability_prior_guided_conflict_without_prior_negative/fixture.json), which prove locally conflicted role evidence stays contested without the staged prior and becomes decisively resolved only when the matching modality-reliability prior is present.

### Why this matters
- This is the first learning slice that improves semantic arbitration itself instead of only widening local phrase, actor-vocabulary, timing, or table-shape interpretation.
- It stays within the project doctrine: the learning plane can guide which locally grounded evidence should carry more weight, but it still cannot author canonical facts that the current document did not expose.

### Validation
- `cargo test --manifest-path Cargo.toml modality_reliability_priors_can_resolve_local_semantic_conflicts -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality semantic_modality_reliability_prior_guided_conflict_gold semantic_modality_reliability_prior_guided_conflict_without_prior_negative` → passed
- `bash scripts/run_ci.sh` → passed (`207/207` tests)

## 2026-04-07 (table-shape priors landed as the fourth bounded learning slice)

### Added: first table-shape prior family in `CorpusMemory`
- Extended [prior_memory.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/prior_memory.rs) with typed `table_shape_priors`, normalized structured-table header signatures, and advisory lookup helpers that can resolve a local table kind only when the signature matches uniquely.
- Extended [learn_priors.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/commands/learn_priors.rs) so `specforge learn-priors` now harvests table-shape priors from validated document chains by walking `IntentIR -> SemanticIR -> EvidenceIR -> SourceIR`.
- The latest local AMBA run over APB/AHB/AXI now yields `94` table-shape priors in addition to the existing actor-taxonomy and temporal families.

### Added: bounded table-shape prior consumption and benchmark coverage
- Extended [evidence.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/evidence.rs) so `EvidenceIR` can advisory-recover a local table kind from prior memory, but only when the current table is still `unknown`; explicit local `SourceIR.table_kind` values remain authoritative.
- Added the tracked gold/negative fixture pair [table_shape_prior_guided_signal_table_gold](/Users/richarddje/Documents/github/specforge/crates/specforge/test_data/kg_quality/table_shape_prior_guided_signal_table_gold/fixture.json) and [table_shape_prior_guided_signal_table_without_prior_negative](/Users/richarddje/Documents/github/specforge/crates/specforge/test_data/kg_quality/table_shape_prior_guided_signal_table_without_prior_negative/fixture.json), which prove a locally `unknown` `Name | Direction | Width` table stays inert without prior memory and gains signal-description recovery only when a matching learned prior is staged.

### Why this matters
- This is the first cross-document learning slice that improves table interpretation directly, not just actor vocabulary or phrase interpretation.
- It stays fully within the project doctrine: prior memory widens local interpretation, but it does not rewrite `SourceIR` or override explicit local classifications.

## 2026-04-07 (Learning-plane growth model clarified)

### Added: explicit note on what grows to materialize learning
- Logged in [DEVELOPMENT_NOTES.md](/Users/richarddje/Documents/github/specforge/DEVELOPMENT_NOTES.md) that the learning capability is defined in code, while the thing that actually grows over time is the typed prior store, typically [corpus_memory.json](/Users/richarddje/Documents/github/specforge/generated/prior_memory/corpus_memory.json).
- Added the matching short entry-point note in [README.md](/Users/richarddje/Documents/github/specforge/README.md), so future sessions do not confuse `R15f` with neural-network-style hidden-weight learning.

### Why this matters
- This makes the learning model explicit: `specforge` uses symbolic, inspectable, typed memory rather than opaque learned weights.
- It also clarifies the safety boundary between:
  - code that defines how learning works
  - data that stores what has been learned
  - canonical per-document IR that remains provenance-pure

## 2026-04-07 (Actor-taxonomy priors now recover structural KG from section headings)

### Added: prior-guided structural KG recovery for width-only section-guided signal tables
- Extended [evidence.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/evidence.rs) so actor-taxonomy priors can now lift width-only `Signal | Width` tables under headings like `Issuer signals` or `Acceptor signals` into structural `ActorSignalRelation::Drives` edges, not just flat compatibility directions.
- Strengthened the tracked gold fixture [actor_taxonomy_prior_guided_section_direction_gold](/Users/richarddje/Documents/github/specforge/crates/specforge/test_data/kg_quality/actor_taxonomy_prior_guided_section_direction_gold/fixture.json) so it now requires graph-backed recovery too: `actor_signal_relations = 3`, `actor_ports = 3`, and `with_graph_direction = 3`.

### Why this matters
- This closes an important quality gap in the first learning-plane consumer: prior-guided section headings now improve the canonical structural KG, not just the compatibility hint surface.
- It directly supports the graph-first roadmap because learned actor vocabulary can now produce actor-relative port structure from locally grounded width-only tables.

### Validation
- `cargo test --manifest-path Cargo.toml actor_taxonomy_priors_guide_section_heading_direction_inference -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality actor_taxonomy_prior_guided_section_direction_gold actor_taxonomy_prior_guided_section_direction_without_prior_negative` → passed
- `bash scripts/run_ci.sh` → passed (`204/204` tests)

## 2026-04-07 (KG fixtures now lock prior-guided visual semantic recovery)

### Added: prior-guided visual semantic gold/negative pair
- Added [visual_semantic_prior_guided_caption_gold](/Users/richarddje/Documents/github/specforge/crates/specforge/test_data/kg_quality/visual_semantic_prior_guided_caption_gold/fixture.json), which proves the unseen local visual-caption phrase `XACK can sink the transfer` gains ready-like semantic recovery only when a matching `visual_caption` semantic prior is staged into the fixture.
- Added [visual_semantic_prior_guided_caption_without_prior_negative](/Users/richarddje/Documents/github/specforge/crates/specforge/test_data/kg_quality/visual_semantic_prior_guided_caption_without_prior_negative/fixture.json), which locks the honest fallback behavior that the same local caption stays semantically unresolved when prior memory is absent.

### Why this matters
- This broadens the first benchmark surface into a second modality without inventing a new unsafe prior family prematurely.
- It proves the bounded semantic-prior doctrine is not prose-only: prior memory can widen interpretation of local visual-caption phrasing, but it still cannot manufacture a role when the matching prior is absent.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality visual_semantic_prior_guided_caption_gold` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality visual_semantic_prior_guided_caption_without_prior_negative` → passed
- `bash scripts/run_ci.sh` → passed (`204/204` tests)

## 2026-04-07 (KG fixtures now lock prior-guided actor-taxonomy recovery)

### Added: prior-guided actor-taxonomy gold/negative pair
- Added [actor_taxonomy_prior_guided_section_direction_gold](/Users/richarddje/Documents/github/specforge/crates/specforge/test_data/kg_quality/actor_taxonomy_prior_guided_section_direction_gold/fixture.json), which proves width-only `Issuer signals` / `Acceptor signals` sections gain canonical signal directions only when matching actor-taxonomy priors are staged into the fixture.
- Added [actor_taxonomy_prior_guided_section_direction_without_prior_negative](/Users/richarddje/Documents/github/specforge/crates/specforge/test_data/kg_quality/actor_taxonomy_prior_guided_section_direction_without_prior_negative/fixture.json), which locks the honest fallback behavior that the same local section headings stay directionless when prior memory is absent.

### Why this matters
- This completes the first benchmark triangle for the three initial bounded prior families: actor-taxonomy, semantic-role phrases, and temporal-language phrases.
- It proves the local-grounding doctrine for actor vocabulary too: prior memory can widen how the extractor interprets explicit local actor terms, but it cannot manufacture direction when the matching prior is absent.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality actor_taxonomy_prior_guided_section_direction_gold` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality actor_taxonomy_prior_guided_section_direction_without_prior_negative` → passed
- `bash scripts/run_ci.sh` → passed (`204/204` tests)

## 2026-04-07 (KG fixtures now lock prior-guided semantic recovery)

### Added: prior-guided unseen-phrase semantic gold/negative pair
- Added [semantic_prior_guided_phrase_gold](/Users/richarddje/Documents/github/specforge/crates/specforge/test_data/kg_quality/semantic_prior_guided_phrase_gold/fixture.json), which proves the unseen local phrase `XACK can receive the transfer` gains ready-like semantic recovery only when a matching semantic prior is staged into the fixture.
- Added [semantic_prior_guided_phrase_without_prior_negative](/Users/richarddje/Documents/github/specforge/crates/specforge/test_data/kg_quality/semantic_prior_guided_phrase_without_prior_negative/fixture.json), which locks the honest fallback behavior that the same local phrase stays semantically unresolved when prior memory is absent.

### Why this matters
- This is the second tracked benchmark proof that the cross-document learning plane can strengthen analysis of an unseen local phrase without leaking canonical facts across documents.
- It shows the same “local text required, prior only widens interpretation” contract now holds for both temporal-language priors and semantic-role priors.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality semantic_prior_guided_phrase_gold` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality semantic_prior_guided_phrase_without_prior_negative` → passed
- `bash scripts/run_ci.sh` → passed (`204/204` tests)

## 2026-04-07 (KG fixtures now lock prior-guided temporal recovery)

### Added: fixture-owned prior-memory patching in `specforge kg-bench`
- Extended [kg_bench.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/commands/kg_bench.rs) so tracked fixtures can now stage a local `CorpusMemory` before `EvidenceIR` is built.
- The new patch surface is generic across actor-taxonomy, semantic, and temporal priors, so later `R15f` benchmark slices can exercise more prior families without depending on a shared mutable prior file.

### Added: prior-guided unseen-phrase temporal gold/negative pair
- Added [temporal_prior_guided_cycle_window_gold](/Users/richarddje/Documents/github/specforge/crates/specforge/test_data/kg_quality/temporal_prior_guided_cycle_window_gold/fixture.json), which proves the unseen local phrase `PREADY must be asserted one beat later` gains a one-cycle `cycle_window` only when a matching temporal prior is staged into the fixture.
- Added [temporal_prior_guided_cycle_window_without_prior_negative](/Users/richarddje/Documents/github/specforge/crates/specforge/test_data/kg_quality/temporal_prior_guided_cycle_window_without_prior_negative/fixture.json), which locks the honest fallback behavior that the same local phrase still yields a temporal rule but no bounded `cycle_window` without prior memory.

### Why this matters
- This is the first tracked benchmark evidence that the cross-document learning plane can improve analysis of an unseen local phrase without leaking canonical facts across documents.
- It upgrades `R15f` from “consumers exist” to “consumers are benchmarked against honest before/after behavior.”

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality temporal_prior_guided_cycle_window_gold` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality temporal_prior_guided_cycle_window_without_prior_negative` → passed
- `bash scripts/run_ci.sh` → passed (`204/204` tests)

## 2026-04-06 (bounded temporal prior consumption landed)

### Added: prior-guided cycle-window recovery in `SemanticIR`
- Extended [prior_memory.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/prior_memory.rs) with a typed temporal phrase lookup that can resolve a unique learned `CycleWindowRecord` from locally grounded timing text.
- Extended [semantic.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/semantic.rs) so `SemanticIR` now loads advisory prior memory from the persisted upstream `prior_memory_path` and uses temporal phrase priors only as a fallback when direct cycle-window parsing cannot recover the local timing window.
- Added a direct semantic regression proving `PREADY must be asserted one beat later` still yields no built-in cycle window on its own, but does recover a one-cycle temporal rule when a validated temporal prior is present.

### Why this matters
- The cross-document learning plane now has a third real bounded consumer, and it lives in the temporal model instead of only in evidence extraction.
- This lets the extractor become stronger on previously unseen local timing phrase shapes without weakening the rule that canonical timing still has to be justified by the current PDF.
- The temporal prior path is still honest: without the local timing sentence there is no rule, and without a unique learned prior there is no learned cycle-window fallback.

### Validation
- `cargo test --manifest-path Cargo.toml derives_cycle_window_from_temporal_phrase_prior_when_builtin_parser_cannot -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml extracts_single_cycle_window_from_idiomatic_clock_tick_phrases -- --nocapture` → passed
- `bash scripts/run_ci.sh` → passed (`204/204` tests)

## 2026-04-06 (bounded semantic prior consumption landed)

### Added: prior-guided semantic hint recovery in `EvidenceIR`
- Extended [prior_memory.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/prior_memory.rs) with shared semantic-phrase normalization and lookup helpers, so the same phrase-shape logic now powers both `learn-priors` and runtime prior consumption.
- Extended [evidence.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/evidence.rs) so `EvidenceIR` can now use semantic phrase priors to recover local signal-role hints from non-hardcoded grounded phrases.
- `EvidenceIR` now also persists the consulted `prior_memory_path`, so later `refresh_signal_semantic_hints()` calls during NLP loopback keep the same advisory prior guidance instead of silently dropping it.

### Why this matters
- The cross-document learning plane now has a second real bounded consumer, beyond actor-taxonomy direction guidance.
- This lets the extractor become stronger on phrases it has learned from prior validated documents without breaking the local-grounding rule.
- The semantic prior path is still honest: without the local phrase, there is no semantic promotion; with the local phrase, the prior only helps interpret it.

### Validation
- `cargo test --manifest-path Cargo.toml semantic_phrase_priors_guide_local_semantic_hint_recovery -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml actor_taxonomy_priors_guide_source_column_direction_inference -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml learn_priors_harvests_semantic_and_temporal_priors -- --nocapture` → passed
- `bash scripts/run_ci.sh` → passed (`203/203` tests)

## 2026-04-06 (first bounded prior-consumption path landed)

### Added: advisory prior-guided direction recovery in `EvidenceIR`
- Extended [prior_memory.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/prior_memory.rs) with reusable actor-taxonomy lookup helpers, normalized actor-term matching, and protocol-family inference so the learning plane can be queried safely during extraction.
- Extended [evidence.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/evidence.rs) with `build_with_prior_memory(...)` plus the first bounded prior consumer:
  - section-heading direction inference can now use actor-taxonomy priors
  - `Source` / `Destination` column direction inference can now use actor-taxonomy priors
  - prior guidance still requires explicit local actor terms already present in the current document
- Extended [cli.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/cli.rs), [evidence.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/commands/evidence.rs), and [converge.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/commands/converge.rs) so `specforge evidence` and `specforge converge` now consult `--prior-memory generated/prior_memory/corpus_memory.json` by default.

### Why this matters
- The cross-document learning plane is no longer just storing priors; it now has its first real bounded consumer in the staged pipeline.
- This replaces another brittle hardcoded actor-vocabulary heuristic with typed reusable memory while preserving the project’s truthfulness rule: priors may guide local interpretation, but they must not author canonical facts on their own.
- It gives the extractor a safe path to improve on PDF `N+1` from validated experience on PDFs `1..N` without letting document pipelines contaminate one another.

### Validation
- `cargo test --manifest-path Cargo.toml actor_taxonomy_priors_guide_source_column_direction_inference -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml actor_taxonomy_priors_guide_section_heading_direction_inference -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml converge_defaults_to_ollama_for_vlm_and_nlp -- --nocapture` → passed
- `bash scripts/run_ci.sh` → passed (`202/202` tests)

## 2026-04-06 (typed prior memory now learns actor taxonomy too)

### Added: actor-taxonomy priors for `R15f`
- Extended [prior_memory.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/ir/prior_memory.rs) with a typed `actor_taxonomy_priors` family plus query helpers by protocol family and taxonomy role.
- Extended [learn_priors.rs](/Users/richarddje/Documents/github/specforge/crates/specforge/src/commands/learn_priors.rs) so `specforge learn-priors` now harvests actor-taxonomy priors from:
  - decisive, non-alias-dependent actor-grounded handshake-role evidence
  - conservative self-identifying actor vocabulary such as `requester`, `completer`, `manager`, and `subordinate`

### Why this matters
- The cross-document learning plane can now accumulate reusable protocol-role vocabulary, not just timing language.
- This is the first prior family that directly teaches the extractor how actor terminology varies across specs while still keeping canonical per-document truth local and validated.
- The latest live AMBA prior-memory run is now materially richer: `16` actor-taxonomy priors and `222` temporal phrase priors, while semantic phrase priors remain honestly at `0`.

### Validation
- `cargo test --manifest-path Cargo.toml learn_priors -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- learn-priors generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json --output generated/prior_memory/corpus_memory.json` → passed (`16` actor-taxonomy priors, `222` temporal phrase priors)
- `bash scripts/run_ci.sh` → passed (`199/199` tests)

## 2026-04-06 (local and hosted CI now share one entrypoint)

### Added: checked-in local CI runner
- Added [run_ci.sh](/Users/richarddje/Documents/github/specforge/scripts/run_ci.sh), a repository-local CI entrypoint that runs the canonical Rust quality gate from the repo root:
  - `cargo fmt --all --check`
  - `cargo test --manifest-path Cargo.toml`

### Changed: GitHub Actions now reuses the local runner
- Updated [.github/workflows/ci.yml](/Users/richarddje/Documents/github/specforge/.github/workflows/ci.yml) so GitHub Actions calls `./scripts/run_ci.sh` instead of duplicating the commands inline.

### Why this matters
- The full Rust CI path can now be run locally before push, which makes CI breakage easier to catch on the developer machine instead of waiting for GitHub.
- Using one checked-in entrypoint removes local-versus-hosted drift and makes future CI expansion safer.

### Validation
- `./scripts/run_ci.sh` → passed
- `cargo fmt --all --check` → passed
- `cargo test --manifest-path Cargo.toml` → passed

## 2026-04-06 (GitHub Actions CI baseline established)

### Added: repo-hosted Rust CI on `push` / `pull_request`
- Added [.github/workflows/ci.yml](/Users/richarddje/Documents/github/specforge/.github/workflows/ci.yml), a GitHub Actions workflow that installs Rust `1.89.0`, caches Cargo artifacts, and runs the same baseline Rust gate used locally:
  - `cargo fmt --all --check`
  - `cargo test --manifest-path Cargo.toml`

### Why this matters
- The project now has a real hosted validation path instead of relying only on local discipline before commits and pushes.
- The CI contract stays intentionally narrow and trustworthy by mirroring the commands already used as the canonical local gate.
- This also closes a repository-bootstrap gap: the new GitHub repo now validates Rust changes automatically on every push and pull request.

### Validation
- `cargo fmt --all --check` → passed
- `cargo test --manifest-path Cargo.toml` → passed

## 2026-04-06 (first typed cross-document prior store landed)

### Added: `specforge learn-priors <intent_ir>...`
- Added a new CLI command that builds the first local typed `CorpusMemory` prior store under `generated/prior_memory/corpus_memory.json`.
- The command only learns from validated `IntentIR` artifacts and skips artifacts whose latest validation report carries error findings, so the new learning plane stays downstream of validation instead of becoming a shortcut around it.

### Added: first typed prior families for `R15f`
- Added `crates/specforge/src/ir/prior_memory.rs` with a typed `CorpusMemory` schema, explicit update-policy record, protocol-family scoping, and advisory query helpers.
- The first semantic prior family learns reusable semantic-role phrases only from decisive, non-alias-dependent canonical semantic consensus plus preserved observation text.
- The first temporal prior family learns reusable timing/constraint language from canonical `temporal_rules` plus validated canonical `signal_constraints` / `conditional_rules`, which makes the learning plane immediately useful even while real-document temporal-rule lift remains conservative.

### Why this matters
- This is the first real implementation of the separate cross-document learning plane captured in the roadmap and development notes.
- It keeps the document plane provenance-pure while finally giving the extractor a place to accumulate reusable knowledge about how chip specifications express meaning.
- The first live AMBA run is already informative: AXI/APB/AHB `IntentIR` artifacts currently yield `222` temporal phrase priors and `0` semantic phrase priors, which is exactly the kind of honest signal the project needs while the canonical semantic-consensus surface on real PDFs is still strengthening.

### Validation
- `cargo test --manifest-path Cargo.toml learn_priors -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- learn-priors generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json --output generated/prior_memory/corpus_memory.json` → passed (`222` temporal phrase priors)

## 2026-04-06 (KG fixtures now lock AHB wait-state timing recovery)

### Added: AHB-style wait-state timing gold fixture
- Added a tracked staged fixture proving that AHB-style `Manager signals` / `Subordinate signals` section-heading context, `Destination`-column signal tables, and explicit actor relations can recover wait-state timing semantics in addition to the earlier section-heading direction path.
- The fixture locks `HREADY must be asserted on the next cycle when HSEL is HIGH` together with waited-transfer hold rules on `HTRANS` and `HADDR`, and expects canonical actor-relative ports, one bounded `cycle_window`, multi-predicate temporal guards, actor-grounded temporal predicates, and no false handshake completion to survive through both `SemanticIR` and `IntentIR`.

### Why this matters
- The earlier AHB gold fixture proved that family-specific section headings can recover truthful per-signal direction. This follow-on slice proves the same AHB evidence path can also recover real wait-state timing without falling back to generic protocol heuristics.
- It closes an important family gap between “AHB direction works” and “AHB wait-state timing works,” which is necessary if the KG benchmark suite is going to be honest about protocol behavior rather than only port orientation.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality ahb_wait_state_timing_gold` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all --check` → passed
- `cargo test --manifest-path Cargo.toml` → passed

## 2026-04-06 (KG fixtures now lock APB setup/access timing recovery)

### Added: APB-style setup/access timing gold fixture
- Added a tracked staged fixture proving that APB-style `Signal | Source | Width | Description` tables plus guarded constraints can recover setup/access timing semantics in addition to the earlier requester/completer handshake path.
- The fixture locks `PENABLE must be asserted on the next cycle when PSEL is HIGH` together with wait-state and completion hold rules on `PADDR`, and expects canonical actor-relative ports, one bounded `cycle_window`, multi-predicate temporal guards, actor-grounded temporal predicates, and typed handshake completion to survive through both `SemanticIR` and `IntentIR`.

### Why this matters
- The earlier APB gold fixture proved that requester/completer roles plus one guarded constraint can recover handshake completion. This follow-on slice proves the same APB vocabulary can also recover setup-to-access timing and wait-state stability without losing actor grounding or collapsing guarded temporal structure.
- It closes an important protocol-family gap between “APB handshake meaning works” and “APB access timing works,” which is necessary if the KG benchmark suite is going to be honest about behavioral protocol semantics rather than just request/accept roles.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality apb_setup_access_timing_gold` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all --check` → passed
- `cargo test --manifest-path Cargo.toml` → passed

## 2026-04-06 (KG fixtures now lock AXI next-cycle timing recovery)

### Added: AXI-style next-cycle timing gold fixture
- Added a tracked staged fixture proving that AXI-style width-only channel tables plus prose `Manager` / `Subordinate` drive-sample relations can recover next-cycle timing semantics in addition to direction and signal inventory.
- The fixture locks `AWREADY must be asserted on the next cycle` together with `AWADDR must not change when AWVALID is HIGH and AWREADY is HIGH`, and expects canonical actor-relative ports, one bounded `cycle_window`, actor-grounded temporal predicates, and typed handshake completion to survive through both `SemanticIR` and `IntentIR`.

### Why this matters
- The first AXI gold fixture proved that width-only tables plus prose can recover truthful actor-relative direction. This follow-on slice proves the same family of evidence can also recover temporal meaning instead of stopping at static ports.
- It closes an important roadmap gap between “AXI direction works” and “AXI timing works,” which is necessary if the KG benchmark suite is going to be honest about protocol semantics rather than only signal inventory.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_next_cycle_timing_gold` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → passed

## 2026-04-06 (KG fixtures now lock AXI width-only prose-direction recovery)

### Added: AXI-style width-only plus prose-direction gold fixture
- Added a tracked staged fixture proving that AXI-style `Name | Width | Description` signal tables still recover truthful actor-relative ports when prose drive/sample relations provide the missing directionality.
- The fixture locks AXI write-address-channel recovery end-to-end: table-grounded widths for `AWVALID`, `AWREADY`, and `AWADDR`; prose-grounded `Manager` / `Subordinate` `Drives` and `Reads` relations; request/accept semantic grounding; and typed handshake completion from one guarded `AWADDR must not change when AWVALID is HIGH and AWREADY is HIGH` constraint.

### Fixed: width-only synthesized declarations now survive into canonical signal records
- Widened the semantic explicit-signal parser so synthesized statements like `Signal AWVALID is width 1.` are treated as real interface-signal declarations even without an immediate `input` / `output` token.
- This closes the AXI-family gap where width-only channel tables previously stopped at actor relations and connectivity instead of becoming canonical `SemanticIR` / `IntentIR` signal records.

### Why this matters
- AXI-family specs are a real stress case because the signal tables often omit direction columns entirely. If that mixed table-plus-prose recovery path regresses, the generic AMBA/APB/AHB fixture suite can still look healthy while AXI truth quietly drifts.
- This turns that family-specific extraction pattern into executable benchmark coverage instead of leaving it protected only by aggregate PDF scores.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_width_only_prose_direction_gold` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → passed

## 2026-04-06 (KG fixtures now lock APB Requester/Completer semantics)

### Added: APB-style `Requester` / `Completer` gold fixture
- Added a tracked staged fixture proving that APB-style `Source`-column signal tables recover `Requester` / `Completer` actor roles canonically instead of only being covered indirectly by broader AMBA fixtures.
- The fixture locks `(Requester, drives, PSEL)` and `(Completer, drives, PREADY)`, the corresponding actor-relative output ports, table-grounded request/accept semantics, and a typed handshake-completion temporal rule from one guarded `PADDR must not change when PSEL is HIGH and PREADY is HIGH` constraint.

### Why this matters
- APB-family specs use `Completer` as real protocol vocabulary. If that family-specific role word regresses, the broader AMBA fixture set can still look healthy while APB truth quietly degrades.
- This turns APB-specific table vocabulary into executable benchmark coverage rather than assuming it is already protected by the generic `Requester` / `Subordinate` path.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality apb_requester_completer_handshake_gold` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → passed

## 2026-04-06 (KG fixtures now lock AHB section-heading direction recovery)

### Added: AHB-style section-heading gold fixture
- Added a tracked staged fixture proving that `Manager signals` / `Subordinate signals` section context recovers per-signal direction and width correctly for AHB-style signal tables.
- The fixture locks canonical direction on `HADDR`, `HWRITE`, `HTRANS`, `HREADYOUT`, and `HRESP` through both `SemanticIR` and `IntentIR`.

### Improved: `kg-bench` can now patch `document_sections` and assert per-signal direction directly
- Added fixture support for patching `SourceIR.document_sections`, so section-heading-driven extraction paths are benchmarkable without needing heavyweight source documents.
- Added canonical per-signal direction expectations, so tracked fixtures can lock actual signal direction instead of inferring it through aggregate validation metrics.

### Why this matters
- AHB extraction quality genuinely depends on section-heading context in some real specs. If that path regresses, the pipeline can still look healthy at a coarse metric level while silently losing signal truth.
- This turns that protocol-family-specific path into executable benchmark coverage instead of leaving it as a fragile unit-test-only behavior.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality ahb_section_heading_direction_gold` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 192/192 passed

## 2026-04-06 (KG fixtures now lock AMBA destination-column receiver semantics)

### Added: AMBA-style `Destination`-column gold fixture
- Added a tracked staged fixture proving that AMBA-style `Destination` signal-description tables recover consumer-side actor-signal relations canonically as `Reads`.
- The fixture locks that `Subordinate` reads `XREQ` and `Requester` reads `XRESP`, and that those same relations survive downstream as actor-relative input ports in both `SemanticIR` and `IntentIR`.

### Improved: `kg-bench` can now assert canonical actor-signal relations directly
- Added canonical fixture expectations for actor-signal relations, so tracked truthfulness checks can lock `Drives` versus `Reads` semantics directly.
- This makes protocol-grade relation benchmarks stronger than relying only on actor-port projections or relation counts.

### Why this matters
- `Destination`-oriented AMBA tables carry receiver semantics, not producer semantics. If the KG flattens those into output-side relations, downstream truthfulness quietly drifts.
- Locking the relation itself, not just derived port shape, makes the benchmark harness more faithful to the graph-first architecture.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality amba_destination_column_reads_gold` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 192/192 passed

## 2026-04-06 (KG fixtures now lock spurious timing-annotation rejection)

### Added: spurious timing-annotation negative fixture
- Added a tracked staged fixture proving that low-value VLM timing-diagram labels like `T0`, `Addr 1`, and `Cycle 2` remain visible as timing-diagram extraction at the evidence stage but do not synthesize canonical timing constraints or temporal rules downstream.
- The fixture locks `timing_diagram_extractions = 1` together with `timing_constraints = 0` and `temporal_rules = 0`, so the pipeline keeps the observation without overclaiming semantics.

### Fixed: semantic timing lift now rejects label-only waveform noise
- Added a narrow semantic-stage filter so label-only VLM timing annotations are treated as waveform labels instead of timing semantics.
- This keeps the `EvidenceIR` timing observation honest while preventing `SemanticIR` / `IntentIR` from fabricating timing meaning from low-value annotation fragments alone.

### Why this matters
- Chip-spec timing diagrams often contain a mix of true behavioral annotations and low-value figure labels. The KG should learn from the former without hallucinating meaning from the latter.
- This closes a real false-positive path in the multimodal timing lift and makes that truthfulness boundary executable in the tracked benchmark suite.

### Validation
- `cargo test --manifest-path Cargo.toml vlm_timing_diagram_observation_rejects_label_only_noise -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_spurious_annotation_negative` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 192/192 passed

## 2026-04-06 (KG fixtures now lock field-table misclassification rejection)

### Added: field-table misclassification negative fixture
- Added a tracked staged fixture proving that a misclassified `Bits | Name | Description` table does not synthesize fake top-level signals or semantic roles from field names like `REQ` and `ACK`.
- The fixture locks that only the real declared top-level signal survives in the canonical semantic/intent surface, while evidence-stage semantic hints stay at zero.

### Fixed: table-driven top-level signal synthesis now rejects field-like layouts
- Added a shared table-level sanity gate so field-like `Bits | Name | Description` layouts and `... signal fields` captions are filtered before they can generate fake signal declarations, semantic hints, or related top-level table-derived facts.
- This guard now protects the table-driven name/semantic paths together instead of relying on one-off downstream cleanup.

### Why this matters
- This closes another KG false-positive path: register-field tables and bit-field tables often contain uppercase names that look like signals, but they are not top-level interface ports.
- It also makes table misclassification a benchmarked truthfulness property instead of an implicit hope in the upstream classifier.

### Validation
- `cargo test --manifest-path Cargo.toml misclassified_field_table_does_not_synthesize_fake_signal_semantics -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality table_misclassification_field_table_negative` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 191/191 passed

## 2026-04-06 (KG fixtures now lock bogus source-column actor rejection)

### Added: bogus actor-attribution negative fixture
- Added a tracked staged fixture proving that AMBA-style `Source`-column infrastructure rows like `Clock` and `Reset` do not become protocol actors in the KG.
- The fixture locks that only the true `Requester` / `Subordinate` rows survive as actor-signal relations and actor-relative ports while table-grounded semantic request/accept meaning still remains recoverable.

### Fixed: table relation extraction now distinguishes source vs destination semantics
- `Source` / `Driver` columns now yield `Drives` relations.
- `Destination` columns now yield `Reads` relations.
- Direction and infrastructure placeholders like `input`, `Clock`, and `Reset` are filtered instead of being promoted into fake actor names.

### Why this matters
- This closes a real KG false-positive path: chip-spec signal tables often mix protocol rows with clock/reset/infrastructure rows, and the graph should not silently invent actors from those metadata labels.
- It also hardens a subtle semantics boundary: destination-oriented tables are receiver facts, not disguised driver facts.

### Validation
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 190/190 passed

## 2026-04-06 (KG fixtures now lock multimodal conflict behavior too)

### Added: cross-modality semantic-conflict negative fixture
- Added a tracked staged fixture proving that table evidence and visual-caption evidence can disagree about the same signal role without collapsing into false cross-modality consensus.
- The fixture locks that `XCTRL`:
  - carries a semantic conflict
  - keeps multiple candidates and non-decisive arbitration
  - does not gain a resolved role or consensus

### Added: validation-metric expectations for contested multimodal grounding
- The new fixture also locks an important validator nuance:
  - `with_visual_semantic_grounding` stays non-zero because visual evidence is still present
  - `with_cross_modality_semantic_grounding` stays zero because the multimodal evidence never resolved into consensus

### Why this matters
- This protects against a subtle multimodal failure mode: “two modalities spoke” must not be mistaken for “two modalities agreed.”
- The benchmark harness now guards both the positive and negative sides of multimodal grounding quality.

### Validation
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 188/188 passed

## 2026-04-06 (KG fixtures now lock cross-modality grounding metrics)

### Added: validation-metric expectations in `specforge kg-bench`
- KG fixtures can now assert persisted validation metric values directly, not only finding ids.
- This lets the tracked harness lock quantitative truthfulness surfaces like:
  - `with_cross_modality_semantic_grounding`
  - `with_visual_semantic_grounding`
  - decisive semantic-arbitration counts

### Added: visual-asset patching in `SourceIR` fixtures
- `specforge kg-bench` fixtures can now patch `SourceIR.visual_assets` directly in addition to structured tables.
- That makes tracked multimodal regressions practical without needing a heavyweight external PDF for each case.

### Added: cross-modality semantic-grounding gold fixture
- Added a tracked staged fixture proving that `XREQ` can become valid-like through joint signal-description-table evidence plus visual-caption evidence.
- The fixture locks both canonical outcomes and validator metrics, proving the resulting role is:
  - resolved
  - decisive
  - cross-modally grounded
  - visually grounded

### Why this matters
- This extends `R15e` from canonical-shape assertions into quantitative grounding checks.
- The benchmark harness can now protect multimodal semantic-strength behavior directly instead of leaving it to crate-local unit tests or manual inspection.

### Validation
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 188/188 passed

## 2026-04-06 (cross-document learning plane captured in steering docs)

### Added: explicit roadmap target for cross-document extractor learning
- Logged the architecture for a separate cross-document learning plane that can improve extraction on PDF `N+1` using reusable priors learned from PDFs `1..N`.
- Made the safety boundary explicit:
  - per-document `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR` truth stays local and provenance-pure
  - cross-document memory learns extraction priors, not undocumented facts

### Added: detailed engineering note for prior-guided extraction
- Captured the full doctrine in `DEVELOPMENT_NOTES.md`, including:
  - the two-plane architecture
  - examples of good priors versus bad fact leakage
  - candidate memory shapes like `CorpusMemory`, `PriorGraph`, and `ExperienceIR`
  - the retrieval / grounding / validation / feedback loop
  - the rule that only validated/promoted outcomes should feed the learning plane

### Why this matters
- This is the clean path to making the extractor progressively more expert across many chip-spec PDFs without breaking the truthfulness contract of the canonical IR.
- It steers future implementation toward learning reusable extraction intelligence rather than contaminating document-local truth.

## 2026-04-06 (KG fixtures now lock semantic arbitration state directly)

### Added: canonical semantic-arbitration expectations in `specforge kg-bench`
- KG fixtures can now assert:
  - signals with any semantic candidates
  - signals with multiple semantic candidates
  - signals carrying semantic arbitration
  - signals with decisive semantic arbitration
  - signals with non-decisive semantic arbitration

### Added: direct arbitration checks to the staged handshake fixtures
- The contested handshake fallback fixture now proves that `XVALID` stays canonically contested while `XACK` stays decisively grounded.
- The alias-dependent handshake caveat fixture now proves that `XREQ` and `XACK` stay decisively grounded even though their accepted handshake meaning remains explicitly caveated as alias-dependent.

### Why this matters
- This upgrades `R15e` from benchmarking arbitration side effects to benchmarking arbitration state directly.
- The harness now checks the canonical truth model itself instead of inferring arbitration quality only from blocked heuristic fallback, residual packets, or validator findings.

### Validation
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 188/188 passed

## 2026-04-06 (KG fixtures now cover alias-dependent semantic caveats)

### Added: richer canonical expectations in `specforge kg-bench`
- KG fixtures can now assert:
  - alias-dependent semantic consensus by signal
  - alias-dependent semantic candidates by signal
  - alias-dependent handshake-completion counts
- `EvidenceIR` fixture patches can now seed alias maps and refresh semantic hints before downstream stages run.

### Added: alias-dependent handshake-completion caveat fixture
- Added a stage-patched tracked fixture proving that alias-grounded handshake recovery remains canonical only when its weaker grounding stays explicit through:
  - semantic residual decisions
  - intent assumptions
  - validator findings

### Why this matters
- This extends `R15e` from benchmarking only hard rejection cases to also benchmarking “useful but caveated” semantic recovery.
- The harness now protects both sides of the truthfulness contract:
  - unsafe heuristic promotion must stay blocked
  - weak but acceptable semantics must retain their caveat trail

### Validation
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 188/188 passed

## 2026-04-06 (KG fixtures can now patch staged inputs)

### Added: stage-patched KG fixtures
- `specforge kg-bench` fixtures can now patch `SourceIR` and `EvidenceIR` inputs directly before downstream stages run.
- This lets the tracked benchmark harness express richer structured/semantic conditions than plain markdown prose alone.

### Added: contested handshake-name fallback negative fixture
- Added a stage-patched tracked fixture proving that contested meaning for a handshake-shaped signal like `XVALID` blocks typed `HandshakeComplete` recovery.
- The fixture injects:
  - a structured signal-description table
  - a typed signal constraint guard
  - validation expectations showing the blocked fallback and preserved semantic-role conflict

### Why this matters
- This strengthens `R15e` from “benchmark simple markdown cases” into “benchmark real staged semantics.”
- The harness now protects a high-value truthfulness invariant:
  - contested semantic evidence must outrank handshake-name heuristics

### Validation
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 188/188 passed

## 2026-04-06 (KG benchmark now includes a first AMBA-style gold fixture)

### Added: representative AMBA-style handshake gold fixture
- Added `crates/specforge/test_data/kg_quality/amba_source_column_handshake_gold/`.
- The fixture patches an AMBA-style `Signal | Source | Width | Description` table with `Requester` / `Subordinate` source roles and one guarded `PAYLOAD must not change when XREQ is HIGH and XACK is HIGH` constraint.
- It proves:
  - `EvidenceIR` recovers `actor_signal_relations = 2`
  - `SemanticIR` / `IntentIR` recover driver-side actor ports for `Requester -> XREQ` and `Subordinate -> XACK`
  - table-grounded semantic role consensus survives for both handshake signals
  - the guarded constraint lifts into one typed temporal rule with `HandshakeComplete`

### Why this matters
- This is the first tracked benchmark step from seed synthetic truthfulness checks toward representative APB/AHB/AXI-style gold coverage.
- It locks an important real-doc pattern: AMBA-style `Source` columns can now be benchmarked end-to-end instead of only being covered by ad hoc unit tests.

## 2026-04-06 (KG benchmark now locks caption-vs-VLM visual semantic conflicts)

### Added: visual-source semantic-conflict negative fixture
- Added `crates/specforge/test_data/kg_quality/visual_sources_semantic_conflict_negative/`.
- The fixture patches one timing-diagram visual asset with:
  - a caption that implies valid-like meaning
  - a `vlm_timing_diagram_extraction` note that implies ready-like meaning
- It proves:
  - `EvidenceIR` reports one visual-caption semantic hint and one VLM timing-annotation semantic hint
  - downstream `SemanticIR` / `IntentIR` preserve a semantic conflict, multiple candidates, and non-decisive arbitration
  - `with_visual_semantic_grounding = 1` while `with_multi_source_semantic_grounding = 0`

### Why this matters
- The benchmark harness now locks an important same-asset arbitration nuance: two conflicting visual sub-sources must stay visibly grounded without being overpromoted into same-modality consensus.

## 2026-04-06 (KG benchmark now locks VLM-note semantic-noise rejection)

### Added: direct VLM timing-note semantic-noise negative fixture
- Added `crates/specforge/test_data/kg_quality/vlm_timing_name_only_semantic_noise_negative/`.
- The fixture patches a timing-diagram `VisualAsset.note` with `vlm_timing_diagram_extraction` content that only describes waveform motion around `XVALID`.
- It proves:
  - `EvidenceIR` reports `timing_diagram_extractions = 1`
  - `EvidenceIR` reports `signal_semantic_hints_from_vlm_timing_annotations = 0`
  - downstream `SemanticIR` / `IntentIR` keep semantic-role candidates, arbitration, and consensus at zero

### Why this matters
- The benchmark harness now locks both sides of direct VLM-note truthfulness:
  - real timing extraction should survive
  - semantic-role meaning must not leak from handshake-shaped signal spelling alone

## 2026-04-06 (KG benchmark now locks direct VLM-note semantics)

### Added: evidence-stage validation expectations in `specforge kg-bench`
- `specforge kg-bench` fixtures can now assert persisted validation metrics at the `EvidenceIR` stage, not only at `SemanticIR` and `IntentIR`.
- This lets tracked regressions prove exactly where a semantic hint came from when downstream visual-grounding metrics would be too coarse.

### Added: direct VLM timing-note semantic-grounding gold fixture
- Added `crates/specforge/test_data/kg_quality/vlm_timing_semantic_grounding_gold/`.
- The fixture patches a timing-diagram `VisualAsset.note` with `vlm_timing_diagram_extraction` content and proves:
  - `EvidenceIR` reports `signal_semantic_hints_from_vlm_timing_annotations = 1`
  - `EvidenceIR` reports `signal_semantic_hints_from_visual_captions = 0`
  - downstream `SemanticIR` / `IntentIR` still resolve the signal meaning with visual grounding

### Why this matters
- The benchmark harness now locks direct image-note-derived meaning explicitly instead of only checking downstream visual-grounding side effects.
- This closes an important quality gap in `R15e`: tracked fixture coverage now reaches caption grounding, caption-versus-table arbitration, and direct VLM timing-note grounding.

## 2026-04-06 (tracked KG-quality benchmark harness landed)

### Added: `specforge kg-bench` command
- Added a new `specforge kg-bench` CLI command that runs tracked KG-quality fixtures through the staged `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR` pipeline.
- The command can assert canonical IR expectations and persisted validation findings, and it fails the run when any gold or negative fixture drifts.

### Added: first tracked KG-quality fixture pack
- Added tracked fixtures under `crates/specforge/test_data/kg_quality/` for:
  - actor-relative port recovery
  - rejection of name-only semantic role noise
  - multi-producer structural conflict surfacing through validation
  - actor-boundary residual quality

### Why this matters
- This starts `R15e` as a real executable benchmark surface instead of leaving KG-quality evaluation as roadmap text or scalar scores alone.
- The benchmark harness now protects:
  - canonical graph truthfulness
  - false-positive control
  - conflict surfacing
  - residual-quality honesty

### Validation
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 188/188 passed

## 2026-04-06 (polarity conflicts now survive into canonical IR)

### Added: canonical carry-through for polarity disagreement
- `SemanticIR` now carries `signal_polarity_conflicts` forward from `EvidenceIR`.
- `IntentIR` now carries the same polarity-conflict surface forward from `SemanticIR`.

### Added: semantic and intent validation for polarity conflicts
- `specforge validate` now reports `signal_polarity_conflicts` for both `SemanticIR` and `IntentIR`, not only for `EvidenceIR`.
- Contradictory active-high/active-low evidence now stays visible all the way to the canonical artifacts instead of disappearing after the evidence stage.

### Added: regression coverage for polarity-conflict carry-through
- Added semantic-stage and intent-stage carry-through regressions for `signal_polarity_conflicts`.
- Added validator regressions proving polarity conflicts are flagged at both canonical stages.

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 186/186 passed

## 2026-04-05 (alias-dependent handshake completion is now canonical residual state)

### Added: canonical residual and assumption carry-through for alias-dependent handshake semantics
- `SemanticIR` now emits `semantic_alias_dependent_handshake_completion` when typed `HandshakeComplete` predicates still depend on alias-grounded semantic role consensus.
- `IntentIR` now carries that caution forward as `assumption_alias_dependent_handshake_completion`, so the weaker grounding remains inspectable even before validation runs.

### Why this matters
- Validator findings are useful, but SOTA-quality continuity needs the canonical artifacts themselves to preserve important semantic caveats.
- Alias-grounded transfer-progress structure remains usable, while still being marked as weaker than directly grounded or corroborated handshake semantics.

### Added: regression coverage for canonical alias-dependent handshake caveats
- Added semantic-stage coverage proving alias-grounded handshake completion emits a residual packet.
- Added intent-stage coverage proving that residual packet becomes an explicit canonical assumption.

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 182/182 passed

## 2026-04-05 (alias-dependent handshake completion is now explicit in validation)

### Added: validation visibility for alias-dependent temporal handshake semantics
- `specforge validate` now reports:
  - `temporal_rules_with_alias_dependent_handshake_completion`
- It also emits an explicit finding when typed `HandshakeComplete` predicates depend on alias-dependent semantic role consensus.

### Why this matters
- Alias-grounded handshake recovery remains useful, but it no longer looks as grounded as directly supported handshake semantics.
- The temporal layer now makes that weaker grounding visible instead of blending it into the generic handshake-completion count.

### Added: regression coverage for alias-dependent handshake validation
- Added a validation regression proving that alias-grounded semantic role consensus feeding a typed handshake predicate is reported explicitly.

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 181/181 passed

## 2026-04-05 (alias-dependent semantic role meaning is now explicit)

### Added: explicit alias-dependence on semantic candidates and consensus summaries
- `SemanticIR` and `IntentIR` now mark semantic role candidates and consensus summaries as `alias_dependent` when the current meaning still depends only on alias-grounded evidence.
- This keeps alias-grounded meaning usable while making that dependency explicit in the canonical IR instead of hiding it inside source-kind lists.

### Added: validation reporting for alias-dependent resolved roles
- `specforge validate` now reports:
  - `with_alias_dependent_semantic_consensus`
  - `alias_dependent_semantic_candidates`
- It also emits an explicit finding when resolved semantic roles still depend only on alias-grounded evidence.

### Added: regression coverage for alias-dependent canonical role visibility
- Added assertions proving:
  - alias-grounded semantic consensus is marked `alias_dependent`
  - direct visual/table grounded semantic consensus is not marked `alias_dependent`
  - intent validation reports alias-dependent resolved roles explicitly

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 180/180 passed

## 2026-04-05 (explicit signal names now outrank aliases for semantic grounding)

### Changed: alias-grounded semantic hints no longer double-count when direct signal names are present
- `EvidenceIR` now suppresses alias-grounded targeting for a signal when the same prose sentence or visual caption already contains an explicit mention of that signal.
- This means aliases stay a rescue path for implicit references, not an extra vote when the document is already explicit.

### Why this matters
- A sentence like `The request phase XREQ indicates that address and control information are valid for transfer.` now produces exactly one semantic hint for `XREQ`, grounded as direct prose rather than both direct and alias-grounded support.
- That keeps semantic-role arbitration honest and prevents artificial support inflation.

### Added: regression coverage for direct-name precedence over aliases
- Added an evidence-stage regression proving that explicit signal mentions outrank alias-grounding for the same statement.

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 179/179 passed

## 2026-04-05 (multi-signal prose and captions now ground per-signal semantic roles)

### Changed: semantic-role hint extraction now decomposes multi-signal text into per-signal context windows
- `EvidenceIR` no longer requires a whole prose statement or visual caption to resolve to exactly one signal before it can contribute a semantic role hint.
- When multiple known signals appear in the same sentence or caption, the extractor now carves out clause-local context windows around each signal mention and infers role meaning from that local description.

### Why this matters
- Text like `XVALID indicates request pending and XREADY indicates the subordinate can accept the transfer` can now produce:
  - a valid-like hint for `XVALID`
  - a ready-like hint for `XREADY`
- The previous weaker behavior either dropped that region entirely or would have required unsafe whole-text attribution.

### Added: regression coverage for multi-signal prose and caption grounding
- Added evidence-stage regressions proving that:
  - one prose sentence can contribute different semantic role hints to different signals
  - one visual caption can contribute different semantic role hints to different signals

### Cleanup
- removed the dead single-target semantic-role helper after the stronger per-signal path replaced it

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 178/178 passed

## 2026-04-05 (signal names no longer self-justify semantic role hints)

### Changed: semantic-role hint inference now strips signal identifiers from prose/visual text
- `EvidenceIR` now removes explicit signal tokens before semantic-role tag inference on prose descriptions, alias-grounded prose, visual captions, VLM timing annotations, and signal-description row text.
- This means identifiers like `AWVALID` / `AWREADY` no longer create valid-like or ready-like consensus by themselves.

### Added: regression coverage for declaration-only handshake-shaped names
- Added an evidence-stage regression proving that plain declarations such as `Signal AWVALID is input width 1.` do not create semantic handshake hints without descriptive language.

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 176/176 passed

## 2026-04-05 (provisional semantic roles no longer drive typed handshake recovery)

### Changed: handshake-role recovery now requires observation-backed consensus
- `SemanticIR` no longer lets fallback-only resolved semantic roles populate the canonical handshake-role context by themselves.
- Typed handshake-role recovery now trusts observation-backed `semantic_consensus` instead of any resolved role value that still lacks preserved grounding.

### Changed: provisional fallback-only role state now blocks handshake name fallback too
- Handshake-shaped signals with fallback-only provisional roles now block literal `VALID` / `READY` name fallback, not only signals with contested semantic arbitration.
- `specforge validate` now reports those blocked fallback cases under the existing handshake-fallback metric/finding surface.

### Added: regression coverage for provisional-role handshake blocking
- Added assertions proving:
  - provisional fallback-only semantic roles do not populate canonical handshake-role context
  - handshake-shaped provisional-role signals are reported as blocked name-fallback cases in validation

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 175/175 passed

## 2026-04-05 (fallback-only semantic roles are now explicit provisional state)

### Added: residual surfacing for resolved semantic roles without consensus
- `SemanticIR` now emits a `semantic_resolved_role_without_consensus` residual decision when a signal still carries a resolved semantic role but lacks preserved observation-backed consensus.

### Added: explicit IntentIR assumption for provisional semantic meaning
- `IntentIR` now turns that carried residual into `assumption_semantic_role_without_consensus` so provisional role meaning stays visible in the canonical artifact.

### Added: regression coverage for provisional semantic-role visibility
- Added assertions proving:
  - the semantic residual packet is emitted for fallback-only resolved roles
  - the intent stage carries that packet forward
  - `IntentIR` emits the matching provisional-role assumption

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 173/173 passed

## 2026-04-05 (blocked handshake fallback is now explicit residual state)

### Added: residual surfacing for intentionally blocked handshake promotion
- `SemanticIR` now emits a `semantic_handshake_name_fallback_blocked` residual decision when a signal looks handshake-shaped by name but preserved semantic arbitration is still contested.
- `IntentIR` carries that residual packet forward unchanged.

### Added: validation visibility for blocked handshake fallback
- `specforge validate` now reports:
  - `with_blocked_handshake_name_fallback`
- It also emits explicit semantic and intent findings when handshake-shaped signals intentionally block literal `VALID` / `READY` fallback.

### Added: regression coverage for blocked fallback visibility
- Added assertions proving:
  - the semantic residual packet is emitted
  - the intent stage carries it forward
  - semantic and intent validation both report the blocked-fallback state

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 171/171 passed


## 2026-04-05 (contested semantic evidence outranks handshake-name heuristics)

### Changed: temporal handshake derivation now respects contested semantic arbitration
- `SemanticIR` now builds a richer handshake-role context instead of relying only on a resolved-role map plus raw signal-name fallback.
- Signals with non-decisive `semantic_arbitration` now block literal `VALID` / `READY` name fallback during typed `HandshakeComplete` derivation.

### Why this matters
- A signal like `XVALID` can now stay honestly unresolved when preserved evidence disagrees about whether it is valid-like or ready-like.
- Literal spelling no longer overrides explicit contested semantic evidence in the temporal layer.

### Added: regression coverage for blocked handshake-name fallback
- Added a semantic regression proving that contested role evidence suppresses typed `HandshakeComplete` derivation even when the signal name looks handshake-shaped.

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 168/168 passed


## 2026-04-05 (canonical semantic arbitration summaries)

### Added: explicit semantic arbitration summaries on canonical interface signals
- `SemanticIR` now carries `semantic_arbitration` on `InterfaceSignalRecord`.
- `IntentIR` carries that same semantic-arbitration surface forward unchanged.

### Added: lead-vs-runner-up visibility without unsafe role forcing
- Each arbitration summary currently records:
  - candidate count
  - leading role
  - leading evidence weight
  - runner-up role and evidence weight when present
  - lead margin over the runner-up
  - decisive vs non-decisive status
- Multiple candidates still do not force a resolved semantic role; the arbitration surface is preserved for inspection while the canonical winner remains `None`.

### Added: validation reporting for decisive vs contested semantic roles
- `specforge validate` now reports:
  - `with_semantic_arbitration`
  - `with_decisive_semantic_arbitration`
  - `with_non_decisive_semantic_arbitration`
- It also emits an explicit finding when canonical semantic-role arbitration remains non-decisive.

### Added: regression coverage for semantic arbitration summaries
- Added assertions for:
  - decisive arbitration on single-candidate role meaning
  - non-decisive arbitration on conflicting role meaning
  - arbitration carry-through into `IntentIR`
  - validation metrics and findings for contested semantic arbitration

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 167/167 passed
## 2026-04-04 (canonical semantic candidate arbitration surface)

### Added: explicit semantic candidates on canonical interface signals
- `SemanticIR` now carries `semantic_candidates` on `InterfaceSignalRecord`.
- `IntentIR` carries that same candidate-arbitration surface forward unchanged.

### Added: typed candidate profiles for competing role meanings
- Each semantic candidate now records:
  - role
  - grounding strength
  - supporting source kinds
  - supporting observation count
  - strongest supporting automation confidence
  - deterministic evidence weight

### Changed: resolved semantic roles now build from canonical candidates
- Observation-backed resolved roles and consensus summaries are now built from the canonical candidate layer.
- When multiple role candidates exist, the signal keeps those candidates explicit instead of flattening the situation into only a conflict record.

### Added: validation metrics for canonical semantic arbitration
- `specforge validate` now reports:
  - `semantic_candidates`
  - `with_semantic_candidates`
  - `with_multiple_semantic_candidates`

### Added: regression coverage for canonical semantic candidates
- Added tests for:
  - candidate details on single-source, same-modality multi-source, and cross-modality role meanings
  - conflicting role meanings producing multiple canonical candidates without a resolved role
  - candidate carry-through into `IntentIR`
  - validation counts for multiple semantic candidates

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 167/167 passed

## 2026-04-04 (canonical semantic consensus summaries)

### Added: explicit semantic consensus summaries on canonical interface signals
- `SemanticIR` now carries `semantic_consensus` on `InterfaceSignalRecord` when a resolved role is backed by preserved observations.
- `IntentIR` now carries that same consensus summary forward unchanged.

### Added: semantic consensus profile details
- `semantic_consensus` currently records:
  - winning role
  - grounding strength
  - supporting source kinds
  - supporting observation count
  - strongest supporting automation confidence

### Changed: validation now surfaces fallback-only resolved roles
- `specforge validate` now reports:
  - `with_semantic_consensus`
  - `with_high_confidence_semantic_consensus`
  - `resolved_semantic_roles_without_consensus`
- It also emits an explicit finding when a resolved semantic role still lacks canonical consensus metadata.

### Added: regression coverage for canonical semantic consensus
- Added tests for:
  - consensus details on single-source, same-modality multi-source, and cross-modality semantic grounding
  - consensus carry-through into `IntentIR`
  - validation reporting and findings for resolved roles without consensus

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 166/166 passed

## 2026-04-04 (modality-aware semantic grounding strength)

### Changed: semantic grounding strength now distinguishes cross-modality reinforcement
- `SemanticIR` now derives `semantic_grounding_strength` as:
  - `single_source`
  - `multi_source`
  - `cross_modality`
- `IntentIR` carries that stronger distinction forward unchanged.

### Changed: repeated same-modality evidence no longer overclaims cross-modality support
- Repeated observations from one modality family now stay `multi_source`.
- Support spanning more than one modality family across table/prose/visual evidence now upgrades to `cross_modality`.

### Added: validation metric for cross-modality semantic grounding
- `specforge validate` now reports:
  - `with_cross_modality_semantic_grounding`

### Added: regression coverage for modality-aware role grounding
- Added tests for:
  - cross-modality semantic grounding on interface signals
  - same-modality multi-source semantic grounding on interface signals
  - cross-modality grounding carry-through into `IntentIR`
  - validation counts for both cross-modality and same-modality multi-source grounding

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 165/165 passed

## 2026-04-04 (canonical semantic-role consensus from preserved observations)

### Added: resolved semantic-role consensus on canonical interface signals
- `SemanticIR` now resolves `resolved_semantic_role` on `InterfaceSignalRecord` from canonical `semantic_observations` before falling back to merged `semantic_tags`.
- `IntentIR` carries that same canonical resolved-role surface forward unchanged.

### Added: grounding-strength visibility for semantic roles
- `InterfaceSignalRecord` now also carries `semantic_grounding_strength`.
- The canonical layers can now distinguish single-source grounding from multi-source grounding for resolved role meaning.

### Changed: validation now exposes semantic grounding quality directly
- `specforge validate` now reports:
  - `with_resolved_semantic_role`
  - `with_single_source_semantic_grounding`
  - `with_multi_source_semantic_grounding`
- This makes it visible when canonical role meaning is merely present versus reinforced by multiple preserved observations.

### Added: regression coverage for observation-backed role consensus
- Added tests for:
  - single-source resolved semantic roles on interface signals
  - multi-source semantic grounding on interface signals
  - validation counts for multi-source semantic grounding

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 162/162 passed

## 2026-04-04 (canonical semantic-role observation carry-through)

### Added: per-signal semantic observations in canonical IR
- `SemanticIR` now carries `semantic_observations` on `InterfaceSignalRecord`.
- `IntentIR` now carries the same role-observation surface forward.
- These observations preserve source kind, source text, and statement/table/visual provenance instead of flattening everything into merged `semantic_tags`.

### Changed: validation now exposes canonical role-grounding depth
- `specforge validate` now reports:
  - `semantic_observations`
  - `with_visual_semantic_grounding`
- This makes it visible when canonical signal meaning is actually grounded in preserved provenance rather than only implied by merged tags.

### Added: regression coverage for canonical observation preservation
- Added tests for:
  - carrying semantic observations into `SemanticIR` interface records
  - carrying semantic observations into `IntentIR`
  - exposing canonical semantic-observation counts in validation

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 160/160 passed

## 2026-04-04 (initial multimodal semantic-role grounding)

### Added: visual semantic-role hints in `EvidenceIR`
- `EvidenceIR` now mines `signal_semantic_hints` from grounded visual captions and VLM timing-diagram annotations.
- The visual path stays conservative: it only promotes hints when the text implies a role meaning and resolves to exactly one known signal.

### Added: explicit visual provenance for semantic-role hints
- `SignalSemanticHintRecord` now carries `supporting_visual_evidence_ids`.
- This keeps caption/VLM-derived role hints tied to concrete visual evidence instead of degrading into anonymous strings.

### Changed: validation now reports multimodal role-hint sources
- `specforge validate` now emits:
  - `signal_semantic_hints_from_visual_captions`
  - `signal_semantic_hints_from_vlm_timing_annotations`

### Added: end-to-end proof that visual grounding affects semantics
- Added tests for:
  - caption-grounded semantic-role hints
  - VLM timing-annotation-grounded semantic-role hints
  - semantic handshake completion derived from caption-grounded role hints

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 159/159 passed

## 2026-04-04 (semantic-role conflict carry-through into canonical IR)

### Added: carried semantic-role conflicts in `SemanticIR` and `IntentIR`
- `SemanticIR` now carries `signal_semantic_conflicts` forward from `EvidenceIR`.
- `IntentIR` now carries the same explicit role-conflict surface into the canonical endpoint.

### Changed: validation now reports semantic-role disagreement end-to-end
- `specforge validate` now prints and flags `signal_semantic_conflicts` for `SemanticIR` and `IntentIR`, not only for `EvidenceIR`.
- This keeps unresolved role disagreement visible to downstream consumers instead of letting it disappear after the evidence stage.

### Added: regression coverage for canonical conflict carry-through
- Added tests for:
  - carrying semantic-role conflicts into `SemanticIR`
  - carrying semantic-role conflicts into `IntentIR`
  - flagging those conflicts from semantic-stage and intent-stage validation

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 155/155 passed

## 2026-04-04 (explicit semantic-role conflict surfacing)

### Added: typed semantic-role conflicts in `EvidenceIR`
- `EvidenceIR` now persists `signal_semantic_conflicts` when meaning-based role evidence assigns incompatible roles to the same signal.
- This keeps role disagreement explicit instead of leaving it hidden inside a dual-tag ambiguity.

### Changed: validation now reports semantic-role disagreement clearly
- `specforge validate` now prints a dedicated semantic-role-conflict section for `EvidenceIR`.
- Validation now emits a `signal_semantic_conflicts` metric and a warning finding when incompatible role evidence is present.

### Added: regression coverage for semantic-role conflict surfacing
- Added tests for:
  - surfacing a role conflict when one source makes a signal look valid-like and another makes it look ready-like
  - flagging that conflict explicitly in `specforge validate`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 151/151 passed

## 2026-04-04 (SourceIR and ingest maturity guidance logged)

### Added: explicit steering on SourceIR maturity and remaining Tier 1 work
- Logged a durable implementation note clarifying that `specforge ingest` and `SourceIR` are different responsibilities:
  - ingest is the stage/command
  - `SourceIR` is the typed artifact/model
- Recorded the current maturity boundary:
  - architecturally strong and high leverage
  - not yet proven universal against arbitrary chip-spec PDFs

### Changed: roadmap now treats remaining Tier 1 work as robustness hardening
- `ROADMAP.md` now says the remaining `SourceIR` / ingest work should be:
  - robustness benchmarking
  - failure-mode detection
  - better fallback behavior
  - stronger source-level validation
- It also makes explicit that broad new Tier 1 expansion should stay secondary unless real PDFs expose a capture bottleneck.

### Validation
- docs-only change; Rust tests were not run

## 2026-04-04 (prose and alias-grounded semantic handshake roles)

### Added: prose and alias-grounded semantic role hints in `EvidenceIR`
- `EvidenceIR` now refreshes `signal_semantic_hints` from direct prose descriptions and alias-grounded prose descriptions, not only from `SignalDescription` tables.
- This means learned aliases can now contribute to typed semantic role grounding instead of only helping constraint reclassification.

### Changed: `nlp-enrich` now refreshes role hints before persistence
- `specforge nlp-enrich` now calls `refresh_signal_semantic_hints()` before writing updated `EvidenceIR`.
- Alias learning and backannotation can therefore feed the same loop-backed semantic-role surface immediately.

### Changed: validation now exposes where semantic role hints came from
- `specforge validate` now reports:
  - `signal_semantic_hints_from_tables`
  - `signal_semantic_hints_from_prose`
  - `signal_semantic_hints_from_alias_grounded_prose`

### Added: regression coverage for prose / alias-grounded role inference
- Added tests for:
  - alias-grounded prose descriptions producing semantic handshake hints in `EvidenceIR`
  - deriving a typed handshake predicate from alias-grounded semantic hints in `SemanticIR`
  - reporting alias-grounded semantic-hint counts in `specforge validate`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 149/149 passed

## 2026-04-04 (meaning-grounded handshake roles from signal descriptions)

### Added: typed semantic role hints in `EvidenceIR`
- `EvidenceIR` now persists `signal_semantic_hints` mined from `SignalDescription` table descriptions when the text establishes handshake-like request/valid or accept/ready meaning.
- The new records preserve source text, supporting table ids, and automation confidence instead of collapsing immediately into opaque downstream behavior.

### Changed: handshake detection now prefers grounded meaning before literal naming
- `SemanticIR` now carries per-signal `semantic_tags`, and `IntentIR` preserves the same surface at the canonical endpoint.
- Typed `HandshakeComplete` derivation now consults those meaning-grounded semantic tags before falling back to literal `VALID` / `READY` signal-name heuristics.

### Changed: validation now reports the new meaning-grounded role surface
- `specforge validate` now reports `signal_semantic_hints` for `EvidenceIR`.
- `specforge validate` now reports `with_semantic_tags` for `SemanticIR` and `IntentIR`.

### Added: regression coverage for meaning-grounded handshake-role carry-through
- Added tests for:
  - mining handshake-role semantic hints from signal-description tables in `EvidenceIR`
  - deriving a typed handshake predicate from semantic signal hints in `SemanticIR`
  - carrying signal semantic tags into `IntentIR`
  - reporting the new validation metrics in `specforge validate`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 146/146 passed

## 2026-04-04 (semantic-programming doctrine logged)

### Changed: the live roadmap now encodes how semantic intent should be programmed
- Logged a cross-cutting implementation doctrine in `ROADMAP.md`:
  - deterministic extraction where the source is crisp
  - typed protocol-world modeling as the semantic target
  - evidence aggregation and convergence instead of one-shot interpretation
  - bounded AI hypotheses rather than end-to-end black-box AI
  - explicit uncertainty, conflicts, and residual decisions as part of the truthfulness contract

### Added: detailed engineering guidance for programming semantics without full-pipeline AI
- Logged the full steering rationale in `DEVELOPMENT_NOTES.md` as a durable implementation note for future sessions.
- Refreshed `README.md` so the project objective explicitly states that `specforge` is building a typed protocol compiler, not an unrestricted English reader.

### Validation
- docs-only change; no Rust tests were run
## 2026-04-04 (typed ready/valid handshake completion)

### Added: protocol-native handshake predicates in the temporal layer
- `SemanticIR` now derives `TemporalPredicateRecord::HandshakeComplete` when a temporal rule contains grounded asserted `VALID` and `READY` signals in the same phase.
- This keeps ready/valid transfer completion visible as a first-class protocol event instead of only as two separate scalar guard clauses.

### Changed: validation now reports handshake-predicate coverage
- `specforge validate` now reports `temporal_rules_with_handshake_completion` for `SemanticIR` and `IntentIR`.
- This makes handshake-semantic coverage visible in the live validation surface instead of hiding it inside raw temporal-rule counts.

### Added: regression coverage for handshake temporal lift
- Added tests for:
  - deriving a typed handshake predicate from a valid/ready guard in `SemanticIR`
  - carrying that predicate into `IntentIR`
  - reporting handshake-predicate coverage in `specforge validate`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 141/141 passed

## 2026-04-04 (idiomatic one-cycle temporal language)

### Added: idiomatic one-cycle latency recovery in the temporal-rule layer
- `SemanticIR` temporal derivation now recognizes common protocol phrases such as `next cycle`, `next clock cycle`, `next tick`, and `next rising edge`.
- `following` and `subsequent` one-cycle variants now also map onto the canonical `CycleWindowRecord` surface instead of being left as unbounded prose.

### Changed: the explicit clock-tick model now covers both numeric and idiomatic latency language
- One-cycle prose no longer needs an explicit numeral like `within 1 cycle` to become a bounded temporal rule.
- This keeps the temporal model aligned with how real chip-design PDFs often describe synchronous behavior.

### Added: regression coverage for idiomatic one-cycle phrases
- Added tests for:
  - direct parser recovery of a single-cycle window from `next cycle`, `next tick`, and `next rising edge`
  - end-to-end temporal-rule derivation from a `next tick` signal constraint in `SemanticIR`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 138/138 passed

## 2026-04-04 (interface-signal conflict surfacing for conflicting declarations)

### Added: typed interface-signal conflicts in the canonical IR layers
- `SemanticIR` now persists `interface_signal_conflicts` when conflicting declarations disagree on a signal's direction or width.
- `IntentIR` now carries the same conflict surface forward so canonical interface-shape disagreement remains explicit downstream.

### Changed: validation now clearly reports interface-shape disagreement
- `specforge validate` now prints a dedicated interface-signal-conflict section for `SemanticIR` and `IntentIR`.
- Validation now emits a warning finding and metric when conflicting direction/width evidence is still unresolved in the canonical interface surface.

### Added: regression coverage for interface-signal conflict surfacing
- Added tests for:
  - deriving direction and width conflicts from contradictory explicit declarations in `SemanticIR`
  - carrying those conflicts into `IntentIR`
  - flagging them in `specforge validate`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 136/136 passed

## 2026-04-04 (structural KG conflict surfacing for multi-producer ambiguity)

### Added: typed structural connectivity conflicts in the canonical IR layers
- `SemanticIR` now persists `signal_connectivity_conflicts` when the structural KG resolves more than one producer for the same signal.
- `IntentIR` now carries the same conflict surface forward so unresolved producer ambiguity remains explicit at the canonical endpoint.

### Changed: validation now clearly reports structural producer ambiguity
- `specforge validate` now prints a dedicated signal-connectivity-conflict section for `SemanticIR` and `IntentIR`.
- Validation now emits a warning finding and metric when the structural KG still has unresolved multi-producer ambiguity.

### Added: regression coverage for structural KG conflict surfacing
- Added tests for:
  - deriving a signal-connectivity conflict from two producer claims in `SemanticIR`
  - carrying that conflict into `IntentIR`
  - flagging that carried conflict in `specforge validate`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 133/133 passed

## 2026-04-04 (explicit polarity-conflict surfacing in EvidenceIR validation)

### Added: typed polarity-conflict records in `EvidenceIR`
- `EvidenceIR` now persists `signal_polarity_conflicts` when prose and signal-description tables disagree on active-high/active-low semantics for the same anchored signal.
- This keeps contradictory polarity inspectable instead of only letting it disappear into a polarity-neutral derived constraint.

### Changed: validation now clearly reports polarity disagreement
- `specforge validate` now prints a dedicated polarity-conflict section for `EvidenceIR`.
- Validation now emits a warning finding and metric when signal polarity evidence disagrees across sources.

### Added: regression coverage for polarity-conflict reporting
- Added tests for:
  - persisting a polarity conflict while keeping the derived constraint neutral
  - flagging that persisted conflict in `specforge validate`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 130/130 passed

## 2026-04-04 (signal-table polarity refinement in convergent evidence)

### Added: signal-description tables now contribute polarity facts
- The convergent `EvidenceIR` loop now scans `SignalDescription` tables for active-high/active-low signal facts using known signals as anchors.
- This lets table rows refine asserted/deasserted constraints even when the polarity never appears in prose.

### Changed: polarity merging is now cross-modality and conservative
- Prose polarity and signal-table polarity are now merged before constraint refinement.
- Conflicting polarity across prose and tables cancels the refinement instead of forcing a wrong `MustBeHigh` / `MustBeLow` conversion.

### Added: regression coverage for table-driven polarity refinement
- Added end-to-end tests for:
  - table-driven active-low polarity refinement
  - preserving polarity neutrality when prose and table evidence disagree

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 129/129 passed

## 2026-04-04 (typed temporal conflict records)

### Added: explicit temporal conflict records in the canonical IR layers
- `SemanticIR` now derives `temporal_conflicts: Vec<TemporalConflictRecord>` from contradictory typed temporal value obligations.
- `IntentIR` now carries the same conflict surface forward so disagreement remains explicit downstream.

### Changed: validation now reports and flags typed temporal conflicts
- `specforge validate` now reports `temporal_conflicts` for `SemanticIR` and `IntentIR`.
- Validation now emits a dedicated warning when contradictory temporal value obligations are present in the typed rule set.

### Added: regression coverage for temporal contradiction surfacing
- Added end-to-end tests for:
  - deriving a typed temporal conflict from contradictory value obligations in `SemanticIR`
  - carrying the conflict into `IntentIR`
  - validating that the contradiction is surfaced as a temporal-conflict finding

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 127/127 passed

## 2026-04-04 (compound temporal antecedents in typed temporal rules)

### Added: conjunctive temporal guards now survive as multiple typed antecedents
- `parse_temporal_condition_predicates()` now preserves compound guards like `when HREADY is LOW and HSEL is HIGH` as multiple antecedent predicates when each clause grounds to a known signal.
- This means the typed temporal layer no longer drops half of a conjunctive protocol precondition during semantic lift.

### Changed: validation now reports multi-predicate temporal guard coverage
- `specforge validate` now reports `temporal_rules_with_multi_predicate_antecedents` for `SemanticIR` and `IntentIR`.
- This gives the live validation surface an explicit signal that conjunctive temporal guards are surviving into the canonical IR.

### Added: regression coverage for compound temporal guards
- Added end-to-end tests for:
  - deriving multi-predicate antecedents from a compound temporal guard in `SemanticIR`
  - carrying those antecedents into `IntentIR`
  - validation metrics for multi-predicate temporal antecedents

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 124/124 passed

## 2026-04-04 (actor-grounded stability semantics in temporal rules)

### Added: actor-relative stability predicates in the temporal layer
- `TemporalPredicateRecord` now includes `ActorMaintainsSignalStable`.
- Stable/hold-style temporal consequents now emit actor-grounded stability predicates when the structural KG resolves a unique producer for the signal.
- This means the temporal layer can now express not just that a signal remains stable, but which actor is responsible for maintaining that stability across the tick window.

### Changed: actor-grounding validation now counts actor-grounded stability too
- `specforge validate` now treats `ActorMaintainsSignalStable` as actor-grounded temporal evidence alongside `ActorDrivesSignal` and `ActorSamplesSignal`.

### Added: regression coverage for actor-grounded stability lift
- Added end-to-end tests for:
  - deriving `ActorMaintainsSignalStable` from a stable constraint with a unique producer
  - carrying actor-grounded stability rules into `IntentIR`
  - validation metrics for actor-grounded stability rules

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 121/121 passed

## 2026-04-04 (actor-grounded temporal drive events)

### Added: actor-relative drive predicates in typed temporal rules
- `TemporalPredicateRecord` now includes `ActorDrivesSignal`.
- Temporal-rule derivation now emits actor-relative drive predicates when the structural KG provides a unique producer for the constrained signal.
- This keeps the temporal layer aligned with the structural graph instead of representing every bounded/value rule as a signal-only event.

### Changed: validation now counts actor-grounded temporal rules
- `specforge validate` now reports `temporal_rules_with_actor_grounding` for `SemanticIR` and `IntentIR`.
- Validation now flags temporal-rule sets that exist alongside a non-empty actor-signal graph but still carry no actor-relative drive/sample grounding at all.

### Added: regression coverage for actor-grounded temporal lift
- Added end-to-end tests for:
  - deriving `ActorDrivesSignal` from a value constraint with a unique producer in the KG
  - carrying actor-grounded temporal rules into `IntentIR`
  - validation metrics for actor-grounded temporal rules

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 118/118 passed

## 2026-04-04 (cycle-window recovery in temporal rules)

### Added: bounded latency windows in the temporal-rule layer
- `SemanticIR` temporal-rule derivation now recovers `CycleWindowRecord` bounds from prose such as:
  - `within 2 cycles`
  - `for 2 cycles`
  - `at least 1 cycle`
  - `at most 3 cycles`
  - `between 1 and 3 cycles`
- Timing rows whose unit is already `cycles` now also project their numeric min/max/typ values into `cycle_window`.

### Changed: validation now counts bounded temporal rules
- `specforge validate` now reports `temporal_rules_with_cycle_window` for `SemanticIR` and `IntentIR`.
- Validation now flags when typed temporal rules exist but none of them currently carry explicit cycle-window bounds.

### Added: regression coverage for cycle-window carry-through
- Added tests for:
  - cycle-window derivation from a cycle-bounded signal constraint
  - cycle-window carry-through into `IntentIR`
  - validation metrics for bounded temporal rules

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 115/115 passed

## 2026-04-04 (typed temporal-rule surface in SemanticIR / IntentIR)

### Added: initial clock-tick temporal rules in the canonical IR layers
- `SemanticIR` now carries `temporal_rules: Vec<TemporalRuleRecord>` alongside legacy timing/constraint records.
- `IntentIR` now carries the same `temporal_rules` surface forward so downstream consumers can target a typed temporal layer instead of only free-form timing text.
- The first predicate set covers:
  - signal value predicates at explicit tick phases
  - signal stability across `pre_tick -> post_tick`
  - signal sampling on clock edges, with optional actor grounding

### Changed: temporal grounding no longer depends on a full reset contract
- Temporal-rule derivation now reuses an explicit clock declaration even when the spec has not yet surfaced a full `SystemContractRecord`.
- This lets timing/constraint semantics ground to a real clock as soon as `Clock <signal>.` is known, instead of waiting for both clock and reset declarations.

### Changed: validation now reports temporal-rule presence and grounding gaps
- `specforge validate` now reports `temporal_rules` and `temporal_rules_missing_clock_grounding` for `SemanticIR` and `IntentIR`.
- Validation findings now explicitly call out:
  - when typed temporal rules exist but still lack clock/edge grounding
  - when timing/constraint evidence exists but no typed temporal rules were derived

### Added: regression coverage for the new temporal layer
- Added end-to-end tests for:
  - temporal-rule derivation from a conditioned signal constraint plus explicit clock context
  - temporal-rule carry-through from `SemanticIR` into `IntentIR`
  - validation diagnostics for ungrounded temporal rules

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 114/114 passed

## 2026-04-04 (graph-first direction scoring in validation)

### Changed: `specforge validate` now scores direction coverage from the actor-relative graph first
- `validate_semantic_ir()` and `validate_intent_ir()` now treat actor-relative `actor_ports` coverage as the primary signal-direction surface and only fall back to flat `direction_hint` values as a compatibility layer.
- Validation metrics now split direction coverage into:
  - `with_resolved_direction`
  - `with_graph_direction`
  - `with_compat_direction_hint`
- Compatibility lag still surfaces as an informational finding, but flat `direction_hint` absence no longer lowers direction coverage when the actor-relative graph already resolves the signal.

### Added: regression coverage for graph-first validation behavior
- Added `validate_intent_ir_scores_direction_from_graph_before_compat_hints`.
- The regression locks the expected behavior: removing flat compatibility hints from an `IntentIR` fixture with intact actor-relative ports must not lower the direction score.

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 111/111 passed

## 2026-04-04 (roadmap retuned around semantic truthfulness)

### Changed: roadmap priorities now explicitly favor KG quality over adapter breadth
- Logged the roadmap reassessment in `DEVELOPMENT_NOTES.md`: the existing four-layer architecture is still right, but the near-term program must be semantic-truthfulness hardening rather than adapter expansion.
- Updated `ROADMAP.md` so the next named milestones are:
  - graph-first downstream semantics
  - explicit clock-tick temporal modeling
  - KG-guided multimodal rescans
  - cross-modality evidence arbitration
  - KG-quality evaluation with gold and negative fixtures
- Demoted SystemVerilog/Verilog/VHDL adapter expansion and adapter validation to horizon work until the semantic pipeline is materially harder to fool.

### Changed: continuity docs now steer future sessions toward the truth-model program
- Updated `LIVE_ACHIEVEMENT_STATUS.md`, `RUST_CODEBASE_ANALYSIS.md`, `README.md`, and `USER_GUIDE.md` so they no longer imply that adapter validation is the next priority.
- The repo now consistently states that adapters should consume truth, not compensate for missing truth in the KG and temporal model.

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
