# Knowledge Map

> **AUTO-GENERATED — DO NOT EDIT.** Regenerate with `knowledge-map/scripts/gen_knowledge_map.sh`.
> Source of truth = YAML front-matter in: `docs/knowledge docs/decisions`. Edit the fact files, never this map.
> A fact is any `.md` whose front-matter has a non-empty `answers:` list.
> **41** facts · **214** question keys.

## Questions → fact

- "Cannot convert a MPS Tensor to float64" -> [docling-device-cpu](docs/knowledge/docling-device-cpu.md) · 2026-06-01 · reverify: `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- "are APB tables 0016 0017 0018 a real catalog miss" -> [apb-signal-catalog-fully-extracted](docs/knowledge/apb-signal-catalog-fully-extracted.md) · 2026-06-06 · reverify: `./target/debug/specforge validate generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json 2>/dev/null | sed -n '/Region Accounting/,/convergence:/p'`
- "are SWCLK and SWDIO extracted / declared" -> [swd-adi-not-signal-table-spec](docs/knowledge/swd-adi-not-signal-table-spec.md) · 2026-06-07 · reverify: `python3 -c "import json,re; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print(sorted({re.match(r'Signal (\w+)',s['text']).group(1) for s in e['extracted_statements'] if s['text'].startswith('Signal ')}))`
- "are SignalStable obligations representable in .isf" -> [stable-obligation-phase-scoped-residual](docs/knowledge/stable-obligation-phase-scoped-residual.md) · 2026-06-04 · reverify: `grep -n "bare stability across tick phases" crates/specforge/src/ir/contract.rs`
- "are temporal rules silently dropped when lowering IntentIR to .isf" -> [isf-temporal-lowering-no-silent-drop](docs/knowledge/isf-temporal-lowering-no-silent-drop.md) · 2026-06-04 · reverify: `grep -n "temporal_residuals" crates/specforge/src/ir/isf_ir.rs crates/specforge/src/ir/adapters.rs`
- "are the APB signals PCLK PADDR PWDATA the parity-check PADDRCHK extracted" -> [apb-signal-catalog-fully-extracted](docs/knowledge/apb-signal-catalog-fully-extracted.md) · 2026-06-06 · reverify: `./target/debug/specforge validate generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json 2>/dev/null | sed -n '/Region Accounting/,/convergence:/p'`
- "are the degenerate PSEL-header or WIDTH-subject temporal rules a live bug" -> [temporal-eval-residual-fps-are-stale](docs/knowledge/temporal-eval-residual-fps-are-stale.md) · 2026-06-02 · reverify: `grep -n " when " crates/specforge/src/ir/evidence.rs`
- "can ISF model an explicit state machine / FSM (proven)" -> [isf-fsm-via-switch-select](docs/knowledge/isf-fsm-via-switch-select.md) · 2026-06-07 · reverify: `write a state machine as (storage (var st ...)) + (transaction step (on start) (switch st (S (select st input A B))...) (complete done)) + (rule tick start (trigger step)); run subs/fsmgen/bin/fsmgen --strict --check --json FILE → success:true`
- "can SpecForge use FSMGen's (stable …) sampled-value predicate" -> [stable-obligation-phase-scoped-residual](docs/knowledge/stable-obligation-phase-scoped-residual.md) · 2026-06-04 · reverify: `grep -n "bare stability across tick phases" crates/specforge/src/ir/contract.rs`
- "can WIRE-BASED-100 reach 100% on SWD the same way as the parallel buses" -> [swd-adi-not-signal-table-spec](docs/knowledge/swd-adi-not-signal-table-spec.md) · 2026-06-07 · reverify: `python3 -c "import json,re; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print(sorted({re.match(r'Signal (\w+)',s['text']).group(1) for s in e['extracted_statements'] if s['text'].startswith('Signal ')}))`
- "default model for the ollama provider" -> [llm-vlm-provider-default](docs/knowledge/llm-vlm-provider-default.md) · 2026-06-01 · reverify: `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- "did a second annotator validate seed_apb.json" -> [eval-gold-interannotator-kappa](docs/knowledge/eval-gold-interannotator-kappa.md) · 2026-06-05 · reverify: `true  # MANUAL: re-run a blind second annotation per EVAL-GOLD-INTERANNOTATOR-AGREEMENT.md Method and recompute kappa (not an automatable grep)`
- "do any chip-spec PDFs need a real password (no)" -> [pdf-encryption-and-read-access](docs/knowledge/pdf-encryption-and-read-access.md) · 2026-06-07 · reverify: `.venv-docling/bin/python -c \"from pypdf import PdfReader; r=PdfReader('<pdf>'); print('enc',r.is_encrypted); r.decrypt('') if r.is_encrypted else 0; print(len(r.pages))\"`
- "do text-reasoning tasks need a vision model" -> [local-llm-for-text-reasoning](docs/knowledge/local-llm-for-text-reasoning.md) · 2026-06-05 · reverify: `ollama list  # qwen2.5:14b-instruct (text) + qwen2.5vl:7b (vision); re-run the NLI/kappa probes`
- "do the Pattern and Nlp extraction tiers find the same constraints" -> [conformal-tier-agreement-degenerate](docs/knowledge/conformal-tier-agreement-degenerate.md) · 2026-06-06 · reverify: `grep -n "tier_count_by_fact_key\|nli_conformal_pass" crates/specforge/src/ir/nli_verify.rs`
- "does SpecForge cycle-schedule the FSM (no — FSMGen does)" -> [isf-fsm-via-switch-select](docs/knowledge/isf-fsm-via-switch-select.md) · 2026-06-07 · reverify: `write a state machine as (storage (var st ...)) + (transaction step (on start) (switch st (S (select st input A B))...) (complete done)) + (rule tick start (trigger step)); run subs/fsmgen/bin/fsmgen --strict --check --json FILE → success:true`
- "does SpecForge detect implementation-defined or TBD or and/or" -> [ambiguity-weak-phrase-detector](docs/knowledge/ambiguity-weak-phrase-detector.md) · 2026-06-04 · reverify: `grep -n "fn weak_phrase_findings" crates/specforge/src/ir/ambiguity.rs`
- "does SpecForge model-check temporal properties" -> [temporal-logic-choice](docs/knowledge/temporal-logic-choice.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "does SpecForge revise or decay priors" -> [contested-priors](docs/knowledge/contested-priors.md) · 2026-06-04 · reverify: `grep -n "fn contested_priors" crates/specforge/src/ir/prior_memory.rs`
- "does SpecForge use LTL CTL or TLA+" -> [temporal-logic-choice](docs/knowledge/temporal-logic-choice.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "does agreement between sources boost confidence" -> [dempster-fusion](docs/knowledge/dempster-fusion.md) · 2026-06-04 · reverify: `grep -n "fn dempster_corroborate_confidence" crates/specforge/src/ir/fusion.rs`
- "does corpus prior memory only accrete" -> [contested-priors](docs/knowledge/contested-priors.md) · 2026-06-04 · reverify: `grep -n "fn contested_priors" crates/specforge/src/ir/prior_memory.rs`
- "does encryption block the VLM from reading tables (no)" -> [vlm-table-strategy](docs/knowledge/vlm-table-strategy.md) · 2026-06-07 · reverify: `./target/debug/specforge enrich generated/source_ir/<key>/source_ir.json --vlm-provider ollama --vlm-model qwen2.5vl:7b   # prints tables_reclassified_by_vlm`
- "does eval-extraction rebuild evidence or load the persisted file" -> [eval-scores-persisted-evidence](docs/knowledge/eval-scores-persisted-evidence.md) · 2026-06-07 · reverify: `./target/debug/specforge evidence generated/source_ir/<doc_key>/source_ir.json   # errors if normalized was reclaimed`
- "does every temporal_rule reach the .isf or a residual" -> [isf-temporal-lowering-no-silent-drop](docs/knowledge/isf-temporal-lowering-no-silent-drop.md) · 2026-06-04 · reverify: `grep -n "temporal_residuals" crates/specforge/src/ir/isf_ir.rs crates/specforge/src/ir/adapters.rs`
- "does the NLI verifier actually catch real extraction errors" -> [nli-gate-real-apb-validation](docs/knowledge/nli-gate-real-apb-validation.md) · 2026-06-05 · reverify: `cargo run -p specforge --quiet -- nli-verify generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json --vlm-provider ollama --model qwen2.5:14b-instruct`
- "how are ACK WDATA RDATA DATAIN bit-widths extracted" -> [swd-serial-frame-surface](docs/knowledge/swd-serial-frame-surface.md) · 2026-06-07 · reverify: `python3 -c "import json; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print([(f['name'],f.get('bit_width')) for f in e.get('serial_frame_fields',[])])`
- "how are NAME[hi:lo] bit-ranges parsed into frame fields" -> [swd-serial-frame-surface](docs/knowledge/swd-serial-frame-surface.md) · 2026-06-07 · reverify: `python3 -c "import json; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print([(f['name'],f.get('bit_width')) for f in e.get('serial_frame_fields',[])])`
- "how are SWCLK and SWDIO captured if they are not in a signal table" -> [prose-pin-appositive-signal-capture](docs/knowledge/prose-pin-appositive-signal-capture.md) · 2026-06-07 · reverify: `python3 -c "import json,re; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); d={re.match(r'Signal (\w+)',s['text']).group(1) for s in e['extracted_statements'] if s['text'].startswith('Signal ')}; print('SWCLK',('SWCLK' in d),'SWDIO',('SWDIO' in d))`
- "how are TAP states (Shift-DR, Run-Test/Idle, Test-Logic-Reset) extracted" -> [swd-protocol-fsm-surface](docs/knowledge/swd-protocol-fsm-surface.md) · 2026-06-07 · reverify: `python3 -c "import json; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print([s['state_name'] for s in e.get('protocol_states',[])])`
- "how are per-instance indexed signals (PSELx HSELx) referenced in prose handled" -> [indexed-signal-family-canonicalization](docs/knowledge/indexed-signal-family-canonicalization.md) · 2026-06-06 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_apb_temporal.json --provider skip 2>/dev/null | sed -n '/Extraction eval/,/source-tolerant/p'`
- "how are per-state actions captured" -> [swd-protocol-fsm-surface](docs/knowledge/swd-protocol-fsm-surface.md) · 2026-06-07 · reverify: `python3 -c "import json; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print([s['state_name'] for s in e.get('protocol_states',[])])`
- "how are serial/architecture spec interface signals added to the catalog" -> [prose-pin-appositive-signal-capture](docs/knowledge/prose-pin-appositive-signal-capture.md) · 2026-06-07 · reverify: `python3 -c "import json,re; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); d={re.match(r'Signal (\w+)',s['text']).group(1) for s in e['extracted_statements'] if s['text'].startswith('Signal ')}; print('SWCLK',('SWCLK' in d),'SWDIO',('SWDIO' in d))`
- "how are temporal rules expressed as LTL or MTL" -> [temporal-rule-ltl-rendering](docs/knowledge/temporal-rule-ltl-rendering.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "how are unknown tables reclassified by the VLM" -> [vlm-table-strategy](docs/knowledge/vlm-table-strategy.md) · 2026-06-07 · reverify: `./target/debug/specforge enrich generated/source_ir/<key>/source_ir.json --vlm-provider ollama --vlm-model qwen2.5vl:7b   # prints tables_reclassified_by_vlm`
- "how did APB temporal reach 100% (WIRE-BASED-100.4)" -> [indexed-signal-family-canonicalization](docs/knowledge/indexed-signal-family-canonicalization.md) · 2026-06-06 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_apb_temporal.json --provider skip 2>/dev/null | sed -n '/Extraction eval/,/source-tolerant/p'`
- "how do I file an FSMGen bug report or feature request" -> [fsmgen-feedback-channel](docs/knowledge/fsmgen-feedback-channel.md) · 2026-06-04 · reverify: `ls docs/FSMGEN_FEEDBACK.md`
- "how do I make the NLI verifier actively change extraction / demote claims" -> [nli-intent-gate](docs/knowledge/nli-intent-gate.md) · 2026-06-05 · reverify: `grep -n "fn apply_nli_gate\|fn nli_gate_contracts\|fn obligation_claim_text" crates/specforge/src/ir/nli_verify.rs`
- "how do I run a Docling ingest or re-ingest on this machine" -> [docling-device-cpu](docs/knowledge/docling-device-cpu.md) · 2026-06-01 · reverify: `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- "how do you audit registers/signals against the table image with the VLM" -> [extraction-audit-vlm](docs/knowledge/extraction-audit-vlm.md) · 2026-06-08 · reverify: `./target/debug/specforge audit-extraction generated/source_ir/1_0_risc_v_debug_specification/source_ir.json --sample 8           # plan-only: lists 8 sampled intent-bearing tables, no VLM calls`
- "how does SpecForge capture signals that are in prose not tables (I2C SDA/SCL)" -> [prose-signal-capture](docs/knowledge/prose-signal-capture.md) · 2026-06-07 · reverify: `./target/debug/specforge evidence generated/source_ir/um10204_rev7_0_2021_i2c_bus_specification/source_ir.json && python3 -c "import json,re;e=json.load(open('generated/evidence_ir/um10204_rev7_0_2021_i2c_bus_specification/evidence_ir.json'));print(sorted({re.match(r'Signal (\w+)',x['text']).group(1) for x in e['extracted_statements'] if x['text'].startswith('Signal ')}))`
- "how does SpecForge combine confidence across modalities or sources" -> [dempster-fusion](docs/knowledge/dempster-fusion.md) · 2026-06-04 · reverify: `grep -n "fn dempster_corroborate_confidence" crates/specforge/src/ir/fusion.rs`
- "how does SpecForge detect contradicting or conflicting priors" -> [contested-priors](docs/knowledge/contested-priors.md) · 2026-06-04 · reverify: `grep -n "fn contested_priors" crates/specforge/src/ir/prior_memory.rs`
- "how does SpecForge emit temporal rules or a bounded-eventually into .isf" -> [fsmgen-temporal-isf-form](docs/knowledge/fsmgen-temporal-isf-form.md) · 2026-06-04 · reverify: `grep -n "assert (monitor (within" crates/specforge/src/ir/isf_ir.rs`
- "how does SpecForge extract register fields from tables" -> [register-field-table-extraction](docs/knowledge/register-field-table-extraction.md) · 2026-06-08 · reverify: `./target/debug/specforge evidence generated/source_ir/1_0_risc_v_debug_specification/source_ir.json && python3 -c "import json;e=json.load(open('generated/evidence_ir/1_0_risc_v_debug_specification/evidence_ir.json'));print(len(e['register_records']),'regs',sum(len(r['fields']) for r in e['register_records']),'fields')`
- "how does SpecForge flag vague or ambiguous spec language" -> [ambiguity-weak-phrase-detector](docs/knowledge/ambiguity-weak-phrase-detector.md) · 2026-06-04 · reverify: `grep -n "fn weak_phrase_findings" crates/specforge/src/ir/ambiguity.rs`
- "how does SpecForge relate to GoldMine Texada Pnueli Ammons" -> [spec-mining-framing](docs/knowledge/spec-mining-framing.md) · 2026-06-04 · reverify: `grep -rn "forward specification mining" README.md docs/book/src/architecture-rationale.md`
- "how does SpecForge verify an extracted claim semantically / catch hallucination" -> [nli-entailment-verifier](docs/knowledge/nli-entailment-verifier.md) · 2026-06-05 · reverify: `grep -n "fn verify_entailment" crates/specforge/src/ir/nli_verify.rs`
- "how does specforge declare a signal mentioned only in prose" -> [prose-pin-appositive-signal-capture](docs/knowledge/prose-pin-appositive-signal-capture.md) · 2026-06-07 · reverify: `python3 -c "import json,re; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); d={re.match(r'Signal (\w+)',s['text']).group(1) for s in e['extracted_statements'] if s['text'].startswith('Signal ')}; print('SWCLK',('SWCLK' in d),'SWDIO',('SWDIO' in d))`
- "how does specforge handle PSEL vs PSELx (or HSEL vs HSELx)" -> [indexed-signal-family-canonicalization](docs/knowledge/indexed-signal-family-canonicalization.md) · 2026-06-06 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_apb_temporal.json --provider skip 2>/dev/null | sed -n '/Extraction eval/,/source-tolerant/p'`
- "how does specforge handle a signal table whose name column is not first" -> [rotated-signal-table-extraction](docs/knowledge/rotated-signal-table-extraction.md) · 2026-06-07 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_ahb_temporal.json --provider skip 2>/dev/null | grep temporal_rule`
- "how does specforge model the JTAG TAP / SWD state machine (FSM)" -> [swd-protocol-fsm-surface](docs/knowledge/swd-protocol-fsm-surface.md) · 2026-06-07 · reverify: `python3 -c "import json; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print([s['state_name'] for s in e.get('protocol_states',[])])`
- "how does specforge model the SWD serial frame / packet" -> [swd-serial-frame-surface](docs/knowledge/swd-serial-frame-surface.md) · 2026-06-07 · reverify: `python3 -c "import json; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print([(f['name'],f.get('bit_width')) for f in e.get('serial_frame_fields',[])])`
- "how does specforge reject non-signal constraint subjects (LICENSEE, AXI, RME, MPAM)" -> [axi-constraint-subject-must-be-declared](docs/knowledge/axi-constraint-subject-must-be-declared.md) · 2026-06-07 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_axi.json --provider skip 2>/dev/null | grep -A1 source-tolerant`
- "how does the VLM understand tables / can a VLM read PDF tables" -> [vlm-table-strategy](docs/knowledge/vlm-table-strategy.md) · 2026-06-07 · reverify: `./target/debug/specforge enrich generated/source_ir/<key>/source_ir.json --vlm-provider ollama --vlm-model qwen2.5vl:7b   # prints tables_reclassified_by_vlm`
- "how flexible is the register model / what register-table shapes are handled" -> [register-field-table-extraction](docs/knowledge/register-field-table-extraction.md) · 2026-06-08 · reverify: `./target/debug/specforge evidence generated/source_ir/1_0_risc_v_debug_specification/source_ir.json && python3 -c "import json;e=json.load(open('generated/evidence_ir/1_0_risc_v_debug_specification/evidence_ir.json'));print(len(e['register_records']),'regs',sum(len(r['fields']) for r in e['register_records']),'fields')`
- "how good is prose signal capture / .3a quality" -> [prose-signal-capture-i2c-precision](docs/knowledge/prose-signal-capture-i2c-precision.md) · 2026-06-08 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_i2c_signals.json --provider skip 2>&1 | grep -A2 "declared-signal surface`
- "how is AXI organized / what are the AXI channels" -> [axi-channel-structure](docs/knowledge/axi-channel-structure.md) · 2026-06-07 · reverify: `python3 -c "import json,re; e=json.load(open('generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json')); d={re.match(r'Signal (\w+)',s['text']).group(1) for s in e['extracted_statements'] if s['text'].startswith('Signal ')}; print(sorted(x for x in d if x.startswith('AW'))[:10])`
- "how is a claim's grounding checked beyond a string match" -> [nli-entailment-verifier](docs/knowledge/nli-entailment-verifier.md) · 2026-06-05 · reverify: `grep -n "fn verify_entailment" crates/specforge/src/ir/nli_verify.rs`
- "how is a fused contract's automation_confidence computed" -> [dempster-fusion](docs/knowledge/dempster-fusion.md) · 2026-06-04 · reverify: `grep -n "fn dempster_corroborate_confidence" crates/specforge/src/ir/fusion.rs`
- "how is a register-field mnemonic recovered when the name column is a bit-range (NVMe)" -> [register-field-table-extraction](docs/knowledge/register-field-table-extraction.md) · 2026-06-08 · reverify: `./target/debug/specforge evidence generated/source_ir/1_0_risc_v_debug_specification/source_ir.json && python3 -c "import json;e=json.load(open('generated/evidence_ir/1_0_risc_v_debug_specification/evidence_ir.json'));print(len(e['register_records']),'regs',sum(len(r['fields']) for r in e['register_records']),'fields')`
- "how is an unless/except exception clause handled in a temporal condition" -> [rotated-signal-table-extraction](docs/knowledge/rotated-signal-table-extraction.md) · 2026-06-07 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_ahb_temporal.json --provider skip 2>/dev/null | grep temporal_rule`
- "how is prose signal over-capture prevented (no garbage)" -> [prose-signal-capture](docs/knowledge/prose-signal-capture.md) · 2026-06-07 · reverify: `./target/debug/specforge evidence generated/source_ir/um10204_rev7_0_2021_i2c_bus_specification/source_ir.json && python3 -c "import json,re;e=json.load(open('generated/evidence_ir/um10204_rev7_0_2021_i2c_bus_specification/evidence_ir.json'));print(sorted({re.match(r'Signal (\w+)',x['text']).group(1) for x in e['extracted_statements'] if x['text'].startswith('Signal ')}))`
- "how is register-field extraction quality measured / scored" -> [register-field-eval-measure-and-surface](docs/knowledge/register-field-eval-measure-and-surface.md) · 2026-06-08 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_riscv_debug_registers.json --provider skip 2>&1 | grep -A3 "register-field surface`
- "how is the NLI gate tested without Ollama" -> [nli-intent-gate](docs/knowledge/nli-intent-gate.md) · 2026-06-05 · reverify: `grep -n "fn apply_nli_gate\|fn nli_gate_contracts\|fn obligation_claim_text" crates/specforge/src/ir/nli_verify.rs`
- "how is the SWD FSM/frame derivation scored (not constraints/relations/temporal)" -> [swd-derivation-scored-100](docs/knowledge/swd-derivation-scored-100.md) · 2026-06-07 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_swd_derivation.json --provider skip 2>/dev/null | sed -n '/source-tolerant/,/document-level/p'`
- "how is the precision of the broadened (non-gold) extraction measured / estimated" -> [extraction-audit-vlm](docs/knowledge/extraction-audit-vlm.md) · 2026-06-08 · reverify: `./target/debug/specforge audit-extraction generated/source_ir/1_0_risc_v_debug_specification/source_ir.json --sample 8           # plan-only: lists 8 sampled intent-bearing tables, no VLM calls`
- "how many signals does each AXI channel have" -> [axi-channel-structure](docs/knowledge/axi-channel-structure.md) · 2026-06-07 · reverify: `python3 -c "import json,re; e=json.load(open('generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json')); d={re.match(r'Signal (\w+)',s['text']).group(1) for s in e['extracted_statements'] if s['text'].startswith('Signal ')}; print(sorted(x for x in d if x.startswith('AW'))[:10])`
- "how much intent does SpecForge extract across the whole corpus" -> [corpus-coverage-sweep](docs/knowledge/corpus-coverage-sweep.md) · 2026-06-08 · reverify: `python3 over generated/evidence_ir/*/evidence_ir.json — count Signal-decls / register_records+fields / protocol_actors / signal_constraints / actor_signal_relations per doc-key`
- "how should an AXI gold or extraction be structured (per channel)" -> [axi-channel-structure](docs/knowledge/axi-channel-structure.md) · 2026-06-07 · reverify: `python3 -c "import json,re; e=json.load(open('generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json')); d={re.match(r'Signal (\w+)',s['text']).group(1) for s in e['extracted_statements'] if s['text'].startswith('Signal ')}; print(sorted(x for x in d if x.startswith('AW'))[:10])`
- "how to express the JTAG TAP / SWD FSM in .isf" -> [isf-fsm-via-switch-select](docs/knowledge/isf-fsm-via-switch-select.md) · 2026-06-07 · reverify: `write a state machine as (storage (var st ...)) + (transaction step (on start) (switch st (S (select st input A B))...) (complete done)) + (rule tick start (trigger step)); run subs/fsmgen/bin/fsmgen --strict --check --json FILE → success:true`
- "how to get a fresh eval-extraction baseline for a spec" -> [eval-scores-persisted-evidence](docs/knowledge/eval-scores-persisted-evidence.md) · 2026-06-07 · reverify: `./target/debug/specforge evidence generated/source_ir/<doc_key>/source_ir.json   # errors if normalized was reclaimed`
- "how to re-score SWD derivation" -> [swd-derivation-scored-100](docs/knowledge/swd-derivation-scored-100.md) · 2026-06-07 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_swd_derivation.json --provider skip 2>/dev/null | sed -n '/source-tolerant/,/document-level/p'`
- "how to read a chip-spec PDF when the Read tool refuses it" -> [pdf-encryption-and-read-access](docs/knowledge/pdf-encryption-and-read-access.md) · 2026-06-07 · reverify: `.venv-docling/bin/python -c \"from pypdf import PdfReader; r=PdfReader('<pdf>'); print('enc',r.is_encrypted); r.decrypt('') if r.is_encrypted else 0; print(len(r.pages))\"`
- "how was AHB HREADY recovered for the temporal antecedent" -> [rotated-signal-table-extraction](docs/knowledge/rotated-signal-table-extraction.md) · 2026-06-07 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_ahb_temporal.json --provider skip 2>/dev/null | grep temporal_rule`
- "how was AXI constraint precision fixed" -> [axi-constraint-subject-must-be-declared](docs/knowledge/axi-constraint-subject-must-be-declared.md) · 2026-06-07 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_axi.json --provider skip 2>/dev/null | grep -A1 source-tolerant`
- "how was the eval gold checked for idiosyncrasy" -> [eval-gold-interannotator-kappa](docs/knowledge/eval-gold-interannotator-kappa.md) · 2026-06-05 · reverify: `true  # MANUAL: re-run a blind second annotation per EVAL-GOLD-INTERANNOTATOR-AGREEMENT.md Method and recompute kappa (not an automatable grep)`
- "is SWD at 100% and on what metric" -> [swd-derivation-scored-100](docs/knowledge/swd-derivation-scored-100.md) · 2026-06-07 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_swd_derivation.json --provider skip 2>/dev/null | sed -n '/source-tolerant/,/document-level/p'`
- "is SpecForge specification mining" -> [spec-mining-framing](docs/knowledge/spec-mining-framing.md) · 2026-06-04 · reverify: `grep -rn "forward specification mining" README.md docs/book/src/architecture-rationale.md`
- "is SpecForge's constraint extraction over-generating" -> [nli-gate-real-apb-validation](docs/knowledge/nli-gate-real-apb-validation.md) · 2026-06-05 · reverify: `cargo run -p specforge --quiet -- nli-verify generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json --vlm-provider ollama --model qwen2.5:14b-instruct`
- "is qwen2.5:14b-instruct good enough for NLI" -> [local-llm-for-text-reasoning](docs/knowledge/local-llm-for-text-reasoning.md) · 2026-06-05 · reverify: `ollama list  # qwen2.5:14b-instruct (text) + qwen2.5vl:7b (vision); re-run the NLI/kappa probes`
- "is the APB PSTRB must be LOW constraint extracted" -> [apb-signal-catalog-fully-extracted](docs/knowledge/apb-signal-catalog-fully-extracted.md) · 2026-06-06 · reverify: `./target/debug/specforge validate generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json 2>/dev/null | sed -n '/Region Accounting/,/convergence:/p'`
- "is the APB signal catalog extracted" -> [apb-signal-catalog-fully-extracted](docs/knowledge/apb-signal-catalog-fully-extracted.md) · 2026-06-06 · reverify: `./target/debug/specforge validate generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json 2>/dev/null | sed -n '/Region Accounting/,/convergence:/p'`
- "is the LLM/VLM provider missing or not wired up" -> [llm-vlm-provider-default](docs/knowledge/llm-vlm-provider-default.md) · 2026-06-01 · reverify: `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- "is the SWD FSM the same as the JTAG TAP DBGTAPSM (no)" -> [swd-intent-is-the-fsm-driving-swdio](docs/knowledge/swd-intent-is-the-fsm-driving-swdio.md) · 2026-06-07 · reverify: `python3 — dump source_ir content_elements for page_id page_0110..page_0128 (Chapter B4); the PDF itself is password-protected so the Read tool cannot open it — use the docling content_elements text.`
- "is the WIRE-BASED-100.5a AHB 0.364 baseline real" -> [eval-scores-persisted-evidence](docs/knowledge/eval-scores-persisted-evidence.md) · 2026-06-07 · reverify: `./target/debug/specforge evidence generated/source_ir/<doc_key>/source_ir.json   # errors if normalized was reclaimed`
- "is the eval gold / answer key trustworthy or reliable" -> [eval-gold-interannotator-kappa](docs/knowledge/eval-gold-interannotator-kappa.md) · 2026-06-05 · reverify: `true  # MANUAL: re-run a blind second annotation per EVAL-GOLD-INTERANNOTATOR-AGREEMENT.md Method and recompute kappa (not an automatable grep)`
- "is the eval-extraction temporal precision 0.6 a real defect" -> [temporal-eval-residual-fps-are-stale](docs/knowledge/temporal-eval-residual-fps-are-stale.md) · 2026-06-02 · reverify: `grep -n " when " crates/specforge/src/ir/evidence.rs`
- "is the extraction audit chip-spec-PDF agnostic (yes)" -> [extraction-audit-vlm](docs/knowledge/extraction-audit-vlm.md) · 2026-06-08 · reverify: `./target/debug/specforge audit-extraction generated/source_ir/1_0_risc_v_debug_specification/source_ir.json --sample 8           # plan-only: lists 8 sampled intent-bearing tables, no VLM calls`
- "is there a PSL or SVA export of temporal rules" -> [temporal-rule-ltl-rendering](docs/knowledge/temporal-rule-ltl-rendering.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "is there a lowering-completeness check for temporal rules" -> [isf-temporal-lowering-no-silent-drop](docs/knowledge/isf-temporal-lowering-no-silent-drop.md) · 2026-06-04 · reverify: `grep -n "temporal_residuals" crates/specforge/src/ir/isf_ir.rs crates/specforge/src/ir/adapters.rs`
- "is tier-agreement a good confidence axis for conformal calibration" -> [conformal-tier-agreement-degenerate](docs/knowledge/conformal-tier-agreement-degenerate.md) · 2026-06-06 · reverify: `grep -n "tier_count_by_fact_key\|nli_conformal_pass" crates/specforge/src/ir/nli_verify.rs`
- "qwen2.5vl vs qwen3-vl which model" -> [llm-vlm-provider-default](docs/knowledge/llm-vlm-provider-default.md) · 2026-06-01 · reverify: `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- "should I build an isf lowering-completeness verifier for temporal rules" -> [isf-temporal-lowering-no-silent-drop](docs/knowledge/isf-temporal-lowering-no-silent-drop.md) · 2026-06-04 · reverify: `grep -n "temporal_residuals" crates/specforge/src/ir/isf_ir.rs crates/specforge/src/ir/adapters.rs`
- "should I fix the PSEL valid when PSEL asserted temporal rule" -> [temporal-eval-residual-fps-are-stale](docs/knowledge/temporal-eval-residual-fps-are-stale.md) · 2026-06-02 · reverify: `grep -n " when " crates/specforge/src/ir/evidence.rs`
- "temporal rule eval false positives root cause" -> [temporal-eval-residual-fps-are-stale](docs/knowledge/temporal-eval-residual-fps-are-stale.md) · 2026-06-02 · reverify: `grep -n " when " crates/specforge/src/ir/evidence.rs`
- "temporal rule predicate atom vocabulary" -> [temporal-rule-ltl-rendering](docs/knowledge/temporal-rule-ltl-rendering.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "torch MPS float64 error during ingest" -> [docling-device-cpu](docs/knowledge/docling-device-cpu.md) · 2026-06-01 · reverify: `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- "was the (contract ... eventually ...) ISF clause removed" -> [fsmgen-temporal-isf-form](docs/knowledge/fsmgen-temporal-isf-form.md) · 2026-06-04 · reverify: `grep -n "assert (monitor (within" crates/specforge/src/ir/isf_ir.rs`
- "what ISF form does SpecForge use for a bounded-eventually contract" -> [fsmgen-temporal-isf-form](docs/knowledge/fsmgen-temporal-isf-form.md) · 2026-06-04 · reverify: `grep -n "assert (monitor (within" crates/specforge/src/ir/isf_ir.rs`
- "what ISF idiom describes states and input-driven transitions" -> [isf-fsm-via-switch-select](docs/knowledge/isf-fsm-via-switch-select.md) · 2026-06-07 · reverify: `write a state machine as (storage (var st ...)) + (transaction step (on start) (switch st (S (select st input A B))...) (complete done)) + (rule tick start (trigger step)); run subs/fsmgen/bin/fsmgen --strict --check --json FILE → success:true`
- "what are APB's remaining completeness candidate misses" -> [apb-signal-catalog-fully-extracted](docs/knowledge/apb-signal-catalog-fully-extracted.md) · 2026-06-06 · reverify: `./target/debug/specforge validate generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json 2>/dev/null | sed -n '/Region Accounting/,/convergence:/p'`
- "what are scripts/pdf_text.py and scripts/decrypt_pdf.py" -> [pdf-encryption-and-read-access](docs/knowledge/pdf-encryption-and-read-access.md) · 2026-06-07 · reverify: `.venv-docling/bin/python -c \"from pypdf import PdfReader; r=PdfReader('<pdf>'); print('enc',r.is_encrypted); r.decrypt('') if r.is_encrypted else 0; print(len(r.pages))\"`
- "what are the SWD packet phases and per-phase SWDIO direction" -> [swd-intent-is-the-fsm-driving-swdio](docs/knowledge/swd-intent-is-the-fsm-driving-swdio.md) · 2026-06-07 · reverify: `python3 — dump source_ir content_elements for page_id page_0110..page_0128 (Chapter B4); the PDF itself is password-protected so the Read tool cannot open it — use the docling content_elements text.`
- "what confidence axis correlates with extracted-constraint correctness" -> [conformal-tier-agreement-degenerate](docs/knowledge/conformal-tier-agreement-degenerate.md) · 2026-06-06 · reverify: `grep -n "tier_count_by_fact_key\|nli_conformal_pass" crates/specforge/src/ir/nli_verify.rs`
- "what did running nli-verify on a real spec find" -> [nli-gate-real-apb-validation](docs/knowledge/nli-gate-real-apb-validation.md) · 2026-06-05 · reverify: `cargo run -p specforge --quiet -- nli-verify generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json --vlm-provider ollama --model qwen2.5:14b-instruct`
- "what does DOCLING_DEVICE do" -> [docling-device-cpu](docs/knowledge/docling-device-cpu.md) · 2026-06-01 · reverify: `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- "what does SpecForge defer from conformal prediction NLI Dempster Snorkel NoRBERT" -> [adopt-defer-ledger](docs/knowledge/adopt-defer-ledger.md) · 2026-06-04 · reverify: `ls docs/research/grounding/adopt-defer-ledger.md`
- "what does SpecForge take from Docling OpenIE LayoutLM Chao Chow LLVM MLIR GoldMine Texada Pnueli" -> [adopt-defer-ledger](docs/knowledge/adopt-defer-ledger.md) · 2026-06-04 · reverify: `ls docs/research/grounding/adopt-defer-ledger.md`
- "what does SpecForge take from a grounded author" -> [adopt-defer-ledger](docs/knowledge/adopt-defer-ledger.md) · 2026-06-04 · reverify: `ls docs/research/grounding/adopt-defer-ledger.md`
- "what does SpecForge take from the spec-mining literature and what does it leave out" -> [spec-mining-framing](docs/knowledge/spec-mining-framing.md) · 2026-06-04 · reverify: `grep -rn "forward specification mining" README.md docs/book/src/architecture-rationale.md`
- "what does intent --nli-verify do" -> [nli-intent-gate](docs/knowledge/nli-intent-gate.md) · 2026-06-05 · reverify: `grep -n "fn apply_nli_gate\|fn nli_gate_contracts\|fn obligation_claim_text" crates/specforge/src/ir/nli_verify.rs`
- "what does resolve_indexed_signal_family do" -> [indexed-signal-family-canonicalization](docs/knowledge/indexed-signal-family-canonicalization.md) · 2026-06-06 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_apb_temporal.json --provider skip 2>/dev/null | sed -n '/Extraction eval/,/source-tolerant/p'`
- "what does signal_table_covered_by_inventory do" -> [apb-signal-catalog-fully-extracted](docs/knowledge/apb-signal-catalog-fully-extracted.md) · 2026-06-06 · reverify: `./target/debug/specforge validate generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json 2>/dev/null | sed -n '/Region Accounting/,/convergence:/p'`
- "what does synthesize_signal_declarations do when the body is rotated" -> [rotated-signal-table-extraction](docs/knowledge/rotated-signal-table-extraction.md) · 2026-06-07 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_ahb_temporal.json --provider skip 2>/dev/null | grep temporal_rule`
- "what does uncaptured_normative_statement_ids do" -> [apb-signal-catalog-fully-extracted](docs/knowledge/apb-signal-catalog-fully-extracted.md) · 2026-06-06 · reverify: `./target/debug/specforge validate generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json 2>/dev/null | sed -n '/Region Accounting/,/convergence:/p'`
- "what eval-extraction tasks score the SWD surfaces" -> [swd-derivation-scored-100](docs/knowledge/swd-derivation-scored-100.md) · 2026-06-07 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_swd_derivation.json --provider skip 2>/dev/null | sed -n '/source-tolerant/,/document-level/p'`
- "what extraction approach does SWD/ADI need" -> [swd-adi-not-signal-table-spec](docs/knowledge/swd-adi-not-signal-table-spec.md) · 2026-06-07 · reverify: `python3 -c "import json,re; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print(sorted({re.match(r'Signal (\w+)',s['text']).group(1) for s in e['extracted_statements'] if s['text'].startswith('Signal ')}))`
- "what field holds the constrained signal name (signal_name vs subject_signal)" -> [apb-signal-catalog-fully-extracted](docs/knowledge/apb-signal-catalog-fully-extracted.md) · 2026-06-06 · reverify: `./target/debug/specforge validate generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json 2>/dev/null | sed -n '/Region Accounting/,/convergence:/p'`
- "what fsmgen pin does SpecForge target for temporal properties" -> [fsmgen-temporal-isf-form](docs/knowledge/fsmgen-temporal-isf-form.md) · 2026-06-04 · reverify: `grep -n "assert (monitor (within" crates/specforge/src/ir/isf_ir.rs`
- "what happens when the NLI provider is down" -> [nli-entailment-verifier](docs/knowledge/nli-entailment-verifier.md) · 2026-06-05 · reverify: `grep -n "fn verify_entailment" crates/specforge/src/ir/nli_verify.rs`
- "what is PDF-VARIANT-DIGESTION.3 prose entity capture" -> [prose-signal-capture](docs/knowledge/prose-signal-capture.md) · 2026-06-07 · reverify: `./target/debug/specforge evidence generated/source_ir/um10204_rev7_0_2021_i2c_bus_specification/source_ir.json && python3 -c "import json,re;e=json.load(open('generated/evidence_ir/um10204_rev7_0_2021_i2c_bus_specification/evidence_ir.json'));print(sorted({re.match(r'Signal (\w+)',x['text']).group(1) for x in e['extracted_statements'] if x['text'].startswith('Signal ')}))`
- "what is ProtocolStateRecord / protocol_states / DBGTAPSM" -> [swd-protocol-fsm-surface](docs/knowledge/swd-protocol-fsm-surface.md) · 2026-06-07 · reverify: `python3 -c "import json; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print([s['state_name'] for s in e.get('protocol_states',[])])`
- "what is SWD's actual intent / protocol (from the spec)" -> [swd-intent-is-the-fsm-driving-swdio](docs/knowledge/swd-intent-is-the-fsm-driving-swdio.md) · 2026-06-07 · reverify: `python3 — dump source_ir content_elements for page_id page_0110..page_0128 (Chapter B4); the PDF itself is password-protected so the Read tool cannot open it — use the docling content_elements text.`
- "what is SerialFrameField / serial_frame_fields / SerialFramePhase" -> [swd-serial-frame-surface](docs/knowledge/swd-serial-frame-surface.md) · 2026-06-07 · reverify: `python3 -c "import json; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print([(f['name'],f.get('bit_width')) for f in e.get('serial_frame_fields',[])])`
- "what is SpecForge doing in academic or research terms" -> [spec-mining-framing](docs/knowledge/spec-mining-framing.md) · 2026-06-04 · reverify: `grep -rn "forward specification mining" README.md docs/book/src/architecture-rationale.md`
- "what is a contested prior" -> [contested-priors](docs/knowledge/contested-priors.md) · 2026-06-04 · reverify: `grep -n "fn contested_priors" crates/specforge/src/ir/prior_memory.rs`
- "what is audit-extraction / PDF-VARIANT-DIGESTION.4b" -> [extraction-audit-vlm](docs/knowledge/extraction-audit-vlm.md) · 2026-06-08 · reverify: `./target/debug/specforge audit-extraction generated/source_ir/1_0_risc_v_debug_specification/source_ir.json --sample 8           # plan-only: lists 8 sampled intent-bearing tables, no VLM calls`
- "what is content-based name-column detection / rotation offset remapping" -> [rotated-signal-table-extraction](docs/knowledge/rotated-signal-table-extraction.md) · 2026-06-07 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_ahb_temporal.json --provider skip 2>/dev/null | grep temporal_rule`
- "what is declared_signal_complete_gold_precision" -> [prose-signal-capture-i2c-precision](docs/knowledge/prose-signal-capture-i2c-precision.md) · 2026-06-08 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_i2c_signals.json --provider skip 2>&1 | grep -A2 "declared-signal surface`
- "what is in seed_swd_derivation.json" -> [swd-derivation-scored-100](docs/knowledge/swd-derivation-scored-100.md) · 2026-06-07 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_swd_derivation.json --provider skip 2>/dev/null | sed -n '/source-tolerant/,/document-level/p'`
- "what is index-family signal canonicalization" -> [indexed-signal-family-canonicalization](docs/knowledge/indexed-signal-family-canonicalization.md) · 2026-06-06 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_apb_temporal.json --provider skip 2>/dev/null | sed -n '/Extraction eval/,/source-tolerant/p'`
- "what is register_field_name_recall / register_field_completeness / register_bit_structure_recall" -> [register-field-eval-measure-and-surface](docs/knowledge/register-field-eval-measure-and-surface.md) · 2026-06-08 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_riscv_debug_registers.json --provider skip 2>&1 | grep -A3 "register-field surface`
- "what is synthesize_register_field_tables" -> [register-field-table-extraction](docs/knowledge/register-field-table-extraction.md) · 2026-06-08 · reverify: `./target/debug/specforge evidence generated/source_ir/1_0_risc_v_debug_specification/source_ir.json && python3 -c "import json;e=json.load(open('generated/evidence_ir/1_0_risc_v_debug_specification/evidence_ir.json'));print(len(e['register_records']),'regs',sum(len(r['fields']) for r in e['register_records']),'fields')`
- "what is synthesize_signal_declarations_from_prose / the pin-appositive pattern" -> [prose-pin-appositive-signal-capture](docs/knowledge/prose-pin-appositive-signal-capture.md) · 2026-06-07 · reverify: `python3 -c "import json,re; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); d={re.match(r'Signal (\w+)',s['text']).group(1) for s in e['extracted_statements'] if s['text'].startswith('Signal ')}; print('SWCLK',('SWCLK' in d),'SWDIO',('SWDIO' in d))`
- "what is the AXI signal naming convention (channel prefix)" -> [axi-channel-structure](docs/knowledge/axi-channel-structure.md) · 2026-06-07 · reverify: `python3 -c "import json,re; e=json.load(open('generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json')); d={re.match(r'Signal (\w+)',s['text']).group(1) for s in e['extracted_statements'] if s['text'].startswith('Signal ')}; print(sorted(x for x in d if x.startswith('AW'))[:10])`
- "what is the Dempster combiner in fusion" -> [dempster-fusion](docs/knowledge/dempster-fusion.md) · 2026-06-04 · reverify: `grep -n "fn dempster_corroborate_confidence" crates/specforge/src/ir/fusion.rs`
- "what is the FSMGen issue bundle protocol" -> [fsmgen-feedback-channel](docs/knowledge/fsmgen-feedback-channel.md) · 2026-06-04 · reverify: `ls docs/FSMGEN_FEEDBACK.md`
- "what is the I2C declared-signal recall / precision" -> [prose-signal-capture-i2c-precision](docs/knowledge/prose-signal-capture-i2c-precision.md) · 2026-06-08 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_i2c_signals.json --provider skip 2>&1 | grep -A2 "declared-signal surface`
- "what is the LTL form of a temporal_rule" -> [temporal-rule-ltl-rendering](docs/knowledge/temporal-rule-ltl-rendering.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "what is the NLI entailment verifier" -> [nli-entailment-verifier](docs/knowledge/nli-entailment-verifier.md) · 2026-06-05 · reverify: `grep -n "fn verify_entailment" crates/specforge/src/ir/nli_verify.rs`
- "what is the NVMe register-field recall / precision" -> [register-field-eval-measure-and-surface](docs/knowledge/register-field-eval-measure-and-surface.md) · 2026-06-08 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_riscv_debug_registers.json --provider skip 2>&1 | grep -A3 "register-field surface`
- "what is the PDF-VARIANT-DIGESTION whole-corpus coverage / re-triage" -> [corpus-coverage-sweep](docs/knowledge/corpus-coverage-sweep.md) · 2026-06-08 · reverify: `python3 over generated/evidence_ir/*/evidence_ir.json — count Signal-decls / register_records+fields / protocol_actors / signal_constraints / actor_signal_relations per doc-key`
- "what is the RISC-V Debug register-field recall / precision" -> [register-field-eval-measure-and-surface](docs/knowledge/register-field-eval-measure-and-surface.md) · 2026-06-08 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_riscv_debug_registers.json --provider skip 2>&1 | grep -A3 "register-field surface`
- "what is the SWD line state machine (reset/operating/protocol-error/lockout)" -> [swd-intent-is-the-fsm-driving-swdio](docs/knowledge/swd-intent-is-the-fsm-driving-swdio.md) · 2026-06-07 · reverify: `python3 — dump source_ir content_elements for page_id page_0110..page_0128 (Chapter B4); the PDF itself is password-protected so the Read tool cannot open it — use the docling content_elements text.`
- "what is the VLM table strategy / PDF-VARIANT-DIGESTION.2b" -> [vlm-table-strategy](docs/knowledge/vlm-table-strategy.md) · 2026-06-07 · reverify: `./target/debug/specforge enrich generated/source_ir/<key>/source_ir.json --vlm-provider ollama --vlm-model qwen2.5vl:7b   # prints tables_reclassified_by_vlm`
- "what is the ambiguous_statements metric in validate" -> [ambiguity-weak-phrase-detector](docs/knowledge/ambiguity-weak-phrase-detector.md) · 2026-06-04 · reverify: `grep -n "fn weak_phrase_findings" crates/specforge/src/ir/ambiguity.rs`
- "what is the completeness gauge over-counting on APB" -> [apb-signal-catalog-fully-extracted](docs/knowledge/apb-signal-catalog-fully-extracted.md) · 2026-06-06 · reverify: `./target/debug/specforge validate generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json 2>/dev/null | sed -n '/Region Accounting/,/convergence:/p'`
- "what is the constraint-subject-must-be-declared filter" -> [axi-constraint-subject-must-be-declared](docs/knowledge/axi-constraint-subject-must-be-declared.md) · 2026-06-07 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_axi.json --provider skip 2>/dev/null | grep -A1 source-tolerant`
- "what is the declared-signal eval surface / EvalTask::DeclaredSignal" -> [prose-signal-capture-i2c-precision](docs/knowledge/prose-signal-capture-i2c-precision.md) · 2026-06-08 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_i2c_signals.json --provider skip 2>&1 | grep -A2 "declared-signal surface`
- "what is the inter-annotator agreement of the eval gold" -> [eval-gold-interannotator-kappa](docs/knowledge/eval-gold-interannotator-kappa.md) · 2026-06-05 · reverify: `true  # MANUAL: re-run a blind second annotation per EVAL-GOLD-INTERANNOTATOR-AGREEMENT.md Method and recompute kappa (not an automatable grep)`
- "what is the parenthetical noun-phrase head rule / EXTRACTION-GAP-FIX.1" -> [prose-signal-capture-i2c-precision](docs/knowledge/prose-signal-capture-i2c-precision.md) · 2026-06-08 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_i2c_signals.json --provider skip 2>&1 | grep -A2 "declared-signal surface`
- "what is the register-field eval surface (EvalTask::RegisterField)" -> [register-field-eval-measure-and-surface](docs/knowledge/register-field-eval-measure-and-surface.md) · 2026-06-08 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_riscv_debug_registers.json --provider skip 2>&1 | grep -A3 "register-field surface`
- "what is the table-kind precision estimate and the flagged-mismatch list" -> [extraction-audit-vlm](docs/knowledge/extraction-audit-vlm.md) · 2026-06-08 · reverify: `./target/debug/specforge audit-extraction generated/source_ir/1_0_risc_v_debug_specification/source_ir.json --sample 8           # plan-only: lists 8 sampled intent-bearing tables, no VLM calls`
- "what model do converge / enrich / nlp-enrich use by default" -> [llm-vlm-provider-default](docs/knowledge/llm-vlm-provider-default.md) · 2026-06-01 · reverify: `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- "what model does the NLI verifier use" -> [nli-entailment-verifier](docs/knowledge/nli-entailment-verifier.md) · 2026-06-05 · reverify: `grep -n "fn verify_entailment" crates/specforge/src/ir/nli_verify.rs`
- "what must SpecForge derive to fully capture SWD; what are the gaps" -> [swd-intent-is-the-fsm-driving-swdio](docs/knowledge/swd-intent-is-the-fsm-driving-swdio.md) · 2026-06-07 · reverify: `python3 — dump source_ir content_elements for page_id page_0110..page_0128 (Chapter B4); the PDF itself is password-protected so the Read tool cannot open it — use the docling content_elements text.`
- "what parts of the literature are deferred or flagged as future work" -> [adopt-defer-ledger](docs/knowledge/adopt-defer-ledger.md) · 2026-06-04 · reverify: `ls docs/research/grounding/adopt-defer-ledger.md`
- "what research did SpecForge leave out and why" -> [adopt-defer-ledger](docs/knowledge/adopt-defer-ledger.md) · 2026-06-04 · reverify: `ls docs/research/grounding/adopt-defer-ledger.md`
- "what signals belong to which AXI channel" -> [axi-channel-structure](docs/knowledge/axi-channel-structure.md) · 2026-06-07 · reverify: `python3 -c "import json,re; e=json.load(open('generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json')); d={re.match(r'Signal (\w+)',s['text']).group(1) for s in e['extracted_statements'] if s['text'].startswith('Signal ')}; print(sorted(x for x in d if x.startswith('AW'))[:10])`
- "what temporal logic backs temporal_rules" -> [temporal-logic-choice](docs/knowledge/temporal-logic-choice.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "where are cross-document prior contradictions surfaced" -> [contested-priors](docs/knowledge/contested-priors.md) · 2026-06-04 · reverify: `grep -n "fn contested_priors" crates/specforge/src/ir/prior_memory.rs`
- "where did SpecForge suggest LTL/MTL support in ISF" -> [fsmgen-feedback-channel](docs/knowledge/fsmgen-feedback-channel.md) · 2026-06-04 · reverify: `ls docs/FSMGEN_FEEDBACK.md`
- "where do I log feedback or a suggestion to FSMGen" -> [fsmgen-feedback-channel](docs/knowledge/fsmgen-feedback-channel.md) · 2026-06-04 · reverify: `ls docs/FSMGEN_FEEDBACK.md`
- "where do the APB signal declarations come from (which table)" -> [apb-signal-catalog-fully-extracted](docs/knowledge/apb-signal-catalog-fully-extracted.md) · 2026-06-06 · reverify: `./target/debug/specforge validate generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json 2>/dev/null | sed -n '/Region Accounting/,/convergence:/p'`
- "where does .isf record dropped temporal obligations" -> [isf-temporal-lowering-no-silent-drop](docs/knowledge/isf-temporal-lowering-no-silent-drop.md) · 2026-06-04 · reverify: `grep -n "temporal_residuals" crates/specforge/src/ir/isf_ir.rs crates/specforge/src/ir/adapters.rs`
- "where does the NLI gate route a not-entailed contract" -> [nli-intent-gate](docs/knowledge/nli-intent-gate.md) · 2026-06-05 · reverify: `grep -n "fn apply_nli_gate\|fn nli_gate_contracts\|fn obligation_claim_text" crates/specforge/src/ir/nli_verify.rs`
- "where is the I2C signal gold seed" -> [prose-signal-capture-i2c-precision](docs/knowledge/prose-signal-capture-i2c-precision.md) · 2026-06-08 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_i2c_signals.json --provider skip 2>&1 | grep -A2 "declared-signal surface`
- "where is the LTL renderer for temporal rules" -> [temporal-rule-ltl-rendering](docs/knowledge/temporal-rule-ltl-rendering.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "where is the SpecForge FSMGen feedback or handoff channel" -> [fsmgen-feedback-channel](docs/knowledge/fsmgen-feedback-channel.md) · 2026-06-04 · reverify: `ls docs/FSMGEN_FEEDBACK.md`
- "where is the declared-signal gate applied (pattern + dynamic constraint paths)" -> [axi-constraint-subject-must-be-declared](docs/knowledge/axi-constraint-subject-must-be-declared.md) · 2026-06-07 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_axi.json --provider skip 2>/dev/null | grep -A1 source-tolerant`
- "where is the per-author adopt-vs-defer provenance" -> [adopt-defer-ledger](docs/knowledge/adopt-defer-ledger.md) · 2026-06-04 · reverify: `ls docs/research/grounding/adopt-defer-ledger.md`
- "where is the register-field gold seed" -> [register-field-eval-measure-and-surface](docs/knowledge/register-field-eval-measure-and-surface.md) · 2026-06-08 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_riscv_debug_registers.json --provider skip 2>&1 | grep -A3 "register-field surface`
- "where is the weak-phrase / NASA ARM ambiguity detector" -> [ambiguity-weak-phrase-detector](docs/knowledge/ambiguity-weak-phrase-detector.md) · 2026-06-04 · reverify: `grep -n "fn weak_phrase_findings" crates/specforge/src/ir/ambiguity.rs`
- "which LLM or VLM does SpecForge use" -> [llm-vlm-provider-default](docs/knowledge/llm-vlm-provider-default.md) · 2026-06-01 · reverify: `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- "which corpus PDFs are password/permission protected" -> [pdf-encryption-and-read-access](docs/knowledge/pdf-encryption-and-read-access.md) · 2026-06-07 · reverify: `.venv-docling/bin/python -c \"from pypdf import PdfReader; r=PdfReader('<pdf>'); print('enc',r.is_encrypted); r.decrypt('') if r.is_encrypted else 0; print(len(r.pages))\"`
- "which corpus docs still yield nothing (the VLM frontier)" -> [corpus-coverage-sweep](docs/knowledge/corpus-coverage-sweep.md) · 2026-06-08 · reverify: `python3 over generated/evidence_ir/*/evidence_ir.json — count Signal-decls / register_records+fields / protocol_actors / signal_constraints / actor_signal_relations per doc-key`
- "which docs fail to ingest (giants / timeouts)" -> [corpus-coverage-sweep](docs/knowledge/corpus-coverage-sweep.md) · 2026-06-08 · reverify: `python3 over generated/evidence_ir/*/evidence_ir.json — count Signal-decls / register_records+fields / protocol_actors / signal_constraints / actor_signal_relations per doc-key`
- "which local model should SpecForge use for NLI or entailment verification" -> [local-llm-for-text-reasoning](docs/knowledge/local-llm-for-text-reasoning.md) · 2026-06-05 · reverify: `ollama list  # qwen2.5:14b-instruct (text) + qwen2.5vl:7b (vision); re-run the NLI/kappa probes`
- "which local models are pulled and what are they for" -> [local-llm-for-text-reasoning](docs/knowledge/local-llm-for-text-reasoning.md) · 2026-06-05 · reverify: `ollama list  # qwen2.5:14b-instruct (text) + qwen2.5vl:7b (vision); re-run the NLI/kappa probes`
- "why are modal verbs must shall should may not flagged as ambiguous" -> [ambiguity-weak-phrase-detector](docs/knowledge/ambiguity-weak-phrase-detector.md) · 2026-06-04 · reverify: `grep -n "fn weak_phrase_findings" crates/specforge/src/ir/ambiguity.rs`
- "why are stability obligations residuals" -> [stable-obligation-phase-scoped-residual](docs/knowledge/stable-obligation-phase-scoped-residual.md) · 2026-06-04 · reverify: `grep -n "bare stability across tick phases" crates/specforge/src/ir/contract.rs`
- "why can't specforge evidence rebuild the evidence (normalized missing)" -> [eval-scores-persisted-evidence](docs/knowledge/eval-scores-persisted-evidence.md) · 2026-06-07 · reverify: `./target/debug/specforge evidence generated/source_ir/<doc_key>/source_ir.json   # errors if normalized was reclaimed`
- "why do APB tables 0016 0017 0018 produce no signal records" -> [apb-signal-catalog-fully-extracted](docs/knowledge/apb-signal-catalog-fully-extracted.md) · 2026-06-06 · reverify: `./target/debug/specforge validate generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json 2>/dev/null | sed -n '/Region Accounting/,/convergence:/p'`
- "why do FPs appear in eval that the current code does not produce" -> [eval-scores-persisted-evidence](docs/knowledge/eval-scores-persisted-evidence.md) · 2026-06-07 · reverify: `./target/debug/specforge evidence generated/source_ir/<doc_key>/source_ir.json   # errors if normalized was reclaimed`
- "why do I2C/CCIX/USB4 have 0 table signals and how are they recovered" -> [prose-signal-capture](docs/knowledge/prose-signal-capture.md) · 2026-06-07 · reverify: `./target/debug/specforge evidence generated/source_ir/um10204_rev7_0_2021_i2c_bus_specification/source_ir.json && python3 -c "import json,re;e=json.load(open('generated/evidence_ir/um10204_rev7_0_2021_i2c_bus_specification/evidence_ir.json'));print(sorted({re.match(r'Signal (\w+)',x['text']).group(1) for x in e['extracted_statements'] if x['text'].startswith('Signal ')}))`
- "why does Docling re-ingest fail on Apple Silicon" -> [docling-device-cpu](docs/knowledge/docling-device-cpu.md) · 2026-06-01 · reverify: `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- "why does a temporal antecedent use PSELX not PSEL" -> [indexed-signal-family-canonicalization](docs/knowledge/indexed-signal-family-canonicalization.md) · 2026-06-06 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_apb_temporal.json --provider skip 2>/dev/null | sed -n '/Extraction eval/,/source-tolerant/p'`
- "why does condition_text matter for the NLI claim" -> [nli-gate-real-apb-validation](docs/knowledge/nli-gate-real-apb-validation.md) · 2026-06-05 · reverify: `cargo run -p specforge --quiet -- nli-verify generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json --vlm-provider ollama --model qwen2.5:14b-instruct`
- "why does the ADI spec produce so few signals and so much garbage" -> [swd-adi-not-signal-table-spec](docs/knowledge/swd-adi-not-signal-table-spec.md) · 2026-06-07 · reverify: `python3 -c "import json,re; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print(sorted({re.match(r'Signal (\w+)',s['text']).group(1) for s in e['extracted_statements'] if s['text'].startswith('Signal ')}))`
- "why does the Claude Read tool refuse some PDFs / report password-protected" -> [pdf-encryption-and-read-access](docs/knowledge/pdf-encryption-and-read-access.md) · 2026-06-07 · reverify: `.venv-docling/bin/python -c \"from pypdf import PdfReader; r=PdfReader('<pdf>'); print('enc',r.is_encrypted); r.decrypt('') if r.is_encrypted else 0; print(len(r.pages))\"`
- "why doesn't SpecForge emit (contract eventually) anymore" -> [fsmgen-temporal-isf-form](docs/knowledge/fsmgen-temporal-isf-form.md) · 2026-06-04 · reverify: `grep -n "assert (monitor (within" crates/specforge/src/ir/isf_ir.rs`
- "why doesn't SpecForge lower stability obligations to (assert (stable sig))" -> [stable-obligation-phase-scoped-residual](docs/knowledge/stable-obligation-phase-scoped-residual.md) · 2026-06-04 · reverify: `grep -n "bare stability across tick phases" crates/specforge/src/ir/contract.rs`
- "why doesn't SpecForge use TLA+" -> [temporal-logic-choice](docs/knowledge/temporal-logic-choice.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "why doesn't fusion use the minimum confidence" -> [dempster-fusion](docs/knowledge/dempster-fusion.md) · 2026-06-04 · reverify: `grep -n "fn dempster_corroborate_confidence" crates/specforge/src/ir/fusion.rs`
- "why doesn't the NLI-oracle conformal calibration produce a threshold" -> [conformal-tier-agreement-degenerate](docs/knowledge/conformal-tier-agreement-degenerate.md) · 2026-06-06 · reverify: `grep -n "tier_count_by_fact_key\|nli_conformal_pass" crates/specforge/src/ir/nli_verify.rs`
- "why doesn't the temporal_rule eval reach precision 1.0" -> [temporal-eval-residual-fps-are-stale](docs/knowledge/temporal-eval-residual-fps-are-stale.md) · 2026-06-02 · reverify: `grep -n " when " crates/specforge/src/ir/evidence.rs`
- "why don't parallel buses get serial_frame_fields" -> [swd-serial-frame-surface](docs/knowledge/swd-serial-frame-surface.md) · 2026-06-07 · reverify: `python3 -c "import json; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print([(f['name'],f.get('bit_width')) for f in e.get('serial_frame_fields',[])])`
- "why is NVMe register field_name a bit-range and how is the mnemonic found in the description" -> [register-field-table-extraction](docs/knowledge/register-field-table-extraction.md) · 2026-06-08 · reverify: `./target/debug/specforge evidence generated/source_ir/1_0_risc_v_debug_specification/source_ir.json && python3 -c "import json;e=json.load(open('generated/evidence_ir/1_0_risc_v_debug_specification/evidence_ir.json'));print(len(e['register_records']),'regs',sum(len(r['fields']) for r in e['register_records']),'fields')`
- "why is SWD/ADI hard / different from APB AHB AXI" -> [swd-adi-not-signal-table-spec](docs/knowledge/swd-adi-not-signal-table-spec.md) · 2026-06-07 · reverify: `python3 -c "import json,re; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print(sorted({re.match(r'Signal (\w+)',s['text']).group(1) for s in e['extracted_statements'] if s['text'].startswith('Signal ')}))`
- "why is SpecForge called forward specification mining" -> [spec-mining-framing](docs/knowledge/spec-mining-framing.md) · 2026-06-04 · reverify: `grep -rn "forward specification mining" README.md docs/book/src/architecture-rationale.md`
- "why is conformal calibration still blocked at CHI scale" -> [conformal-tier-agreement-degenerate](docs/knowledge/conformal-tier-agreement-degenerate.md) · 2026-06-06 · reverify: `grep -n "tier_count_by_fact_key\|nli_conformal_pass" crates/specforge/src/ir/nli_verify.rs`
- "why is statement_0223 still a normative statement" -> [apb-signal-catalog-fully-extracted](docs/knowledge/apb-signal-catalog-fully-extracted.md) · 2026-06-06 · reverify: `./target/debug/specforge validate generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json 2>/dev/null | sed -n '/Region Accounting/,/convergence:/p'`
- "why is the AHB eval baseline wrong or stale" -> [eval-scores-persisted-evidence](docs/knowledge/eval-scores-persisted-evidence.md) · 2026-06-07 · reverify: `./target/debug/specforge evidence generated/source_ir/<doc_key>/source_ir.json   # errors if normalized was reclaimed`
- "why is the FSM important for SWD/JTAG" -> [swd-protocol-fsm-surface](docs/knowledge/swd-protocol-fsm-surface.md) · 2026-06-07 · reverify: `python3 -c "import json; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print([s['state_name'] for s in e.get('protocol_states',[])])`
- "why is the NLI framing better than free-form labeling" -> [local-llm-for-text-reasoning](docs/knowledge/local-llm-for-text-reasoning.md) · 2026-06-05 · reverify: `ollama list  # qwen2.5:14b-instruct (text) + qwen2.5vl:7b (vision); re-run the NLI/kappa probes`
- "why is the PSEL antecedent dropped in a temporal rule" -> [indexed-signal-family-canonicalization](docs/knowledge/indexed-signal-family-canonicalization.md) · 2026-06-06 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_apb_temporal.json --provider skip 2>/dev/null | sed -n '/Extraction eval/,/source-tolerant/p'`
- "why is the strict register-field per-fact score 0 on RISC-V Debug / NVMe" -> [register-field-eval-measure-and-surface](docs/knowledge/register-field-eval-measure-and-surface.md) · 2026-06-08 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_riscv_debug_registers.json --provider skip 2>&1 | grep -A3 "register-field surface`
- "why not CTL for temporal behavior" -> [temporal-logic-choice](docs/knowledge/temporal-logic-choice.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "why was a property like RME_Support or MPAM_WIDTH extracted as a signal constraint" -> [axi-constraint-subject-must-be-declared](docs/knowledge/axi-constraint-subject-must-be-declared.md) · 2026-06-07 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_axi.json --provider skip 2>/dev/null | grep -A1 source-tolerant`
- "why was a signal not extracted from a signal table (e.g. AHB HREADY)" -> [rotated-signal-table-extraction](docs/knowledge/rotated-signal-table-extraction.md) · 2026-06-07 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_ahb_temporal.json --provider skip 2>/dev/null | grep temporal_rule`
- "why was the ISF explicit-FSM feature request withdrawn" -> [isf-fsm-via-switch-select](docs/knowledge/isf-fsm-via-switch-select.md) · 2026-06-07 · reverify: `write a state machine as (storage (var st ...)) + (transaction step (on start) (switch st (S (select st input A B))...) (complete done)) + (rule tick start (trigger step)); run subs/fsmgen/bin/fsmgen --strict --check --json FILE → success:true`
- "why were ACK / NACK / DDC / SDR extracted as I2C signals (and how was it fixed)" -> [prose-signal-capture-i2c-precision](docs/knowledge/prose-signal-capture-i2c-precision.md) · 2026-06-08 · reverify: `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_i2c_signals.json --provider skip 2>&1 | grep -A2 "declared-signal surface`
- "why were RISC-V/TRM register tables unextracted (unknown table_kind)" -> [register-field-table-extraction](docs/knowledge/register-field-table-extraction.md) · 2026-06-08 · reverify: `./target/debug/specforge evidence generated/source_ir/1_0_risc_v_debug_specification/source_ir.json && python3 -c "import json;e=json.load(open('generated/evidence_ir/1_0_risc_v_debug_specification/evidence_ir.json'));print(len(e['register_records']),'regs',sum(len(r['fields']) for r in e['register_records']),'fields')`

## Facts (by id)

### adopt-defer-ledger
_Per-author adopt/defer ledger — what SpecForge takes from / leaves out of each grounded author_

- **answers:** what does SpecForge take from a grounded author | what does SpecForge take from Docling OpenIE LayoutLM Chao Chow LLVM MLIR GoldMine Texada Pnueli | what research did SpecForge leave out and why | what parts of the literature are deferred or flagged as future work | where is the per-author adopt-vs-defer provenance | what does SpecForge defer from conformal prediction NLI Dempster Snorkel NoRBERT
- **date:** 2026-06-04 · **status:** current
- **evidence:** `docs/research/grounding/adopt-defer-ledger.md; docs/tasks/SPEC-MINING-PROVENANCE.md`
- **reverify:** `ls docs/research/grounding/adopt-defer-ledger.md`
- **source:** [`docs/knowledge/adopt-defer-ledger.md`](docs/knowledge/adopt-defer-ledger.md)

### ambiguity-weak-phrase-detector
_validate flags vague spec prose via the weak-phrase detector (ir/ambiguity.rs)_

- **answers:** how does SpecForge flag vague or ambiguous spec language | what is the ambiguous_statements metric in validate | where is the weak-phrase / NASA ARM ambiguity detector | does SpecForge detect implementation-defined or TBD or and/or | why are modal verbs must shall should may not flagged as ambiguous
- **date:** 2026-06-04 · **status:** current
- **evidence:** `crates/specforge/src/ir/ambiguity.rs; docs/book/src/quality/validation.md`
- **reverify:** `grep -n "fn weak_phrase_findings" crates/specforge/src/ir/ambiguity.rs`
- **source:** [`docs/knowledge/ambiguity-weak-phrase-detector.md`](docs/knowledge/ambiguity-weak-phrase-detector.md)

### apb-signal-catalog-fully-extracted
_APB signal catalog is fully extracted (35/35); tables 0016/0017/0018 are duplicate views, not misses_

- **answers:** is the APB signal catalog extracted | are the APB signals PCLK PADDR PWDATA the parity-check PADDRCHK extracted | why do APB tables 0016 0017 0018 produce no signal records | what is the completeness gauge over-counting on APB | are APB tables 0016 0017 0018 a real catalog miss | where do the APB signal declarations come from (which table) | what does signal_table_covered_by_inventory do | is the APB PSTRB must be LOW constraint extracted | why is statement_0223 still a normative statement | what field holds the constrained signal name (signal_name vs subject_signal) | what does uncaptured_normative_statement_ids do | what are APB's remaining completeness candidate misses
- **date:** 2026-06-06 · **status:** current
- **evidence:** `docs/tasks/WIRE-BASED-100.md; crates/specforge/src/ir/completeness.rs (unexplained_intent_bearing_tables, signal_table_covered_by_inventory)`
- **reverify:** `./target/debug/specforge validate generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json 2>/dev/null | sed -n '/Region Accounting/,/convergence:/p'`
- **source:** [`docs/knowledge/apb-signal-catalog-fully-extracted.md`](docs/knowledge/apb-signal-catalog-fully-extracted.md)

### axi-channel-structure
_AXI is channel-organized — each channel (AW/W/B/AR/R/AC) has its own VALID/READY + payload signals_

- **answers:** how is AXI organized / what are the AXI channels | what signals belong to which AXI channel | how should an AXI gold or extraction be structured (per channel) | what is the AXI signal naming convention (channel prefix) | how many signals does each AXI channel have
- **date:** 2026-06-07 · **status:** current
- **evidence:** `corpus/arm/amba/core/axi/current/IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf; generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification (310 declared signals)`
- **reverify:** `python3 -c "import json,re; e=json.load(open('generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json')); d={re.match(r'Signal (\w+)',s['text']).group(1) for s in e['extracted_statements'] if s['text'].startswith('Signal ')}; print(sorted(x for x in d if x.startswith('AW'))[:10])`
- **source:** [`docs/knowledge/axi-channel-structure.md`](docs/knowledge/axi-channel-structure.md)

### axi-constraint-subject-must-be-declared
_A signal-constraint subject must be a DECLARED signal (drops property/config/doc-meta noise)_

- **answers:** why was a property like RME_Support or MPAM_WIDTH extracted as a signal constraint | how does specforge reject non-signal constraint subjects (LICENSEE, AXI, RME, MPAM) | what is the constraint-subject-must-be-declared filter | how was AXI constraint precision fixed | where is the declared-signal gate applied (pattern + dynamic constraint paths)
- **date:** 2026-06-07 · **status:** current
- **evidence:** `docs/tasks/WIRE-BASED-100.md (.5i); crates/specforge/src/ir/evidence.rs (extract_signal_constraints, extract_dynamic_signal_constraints)`
- **reverify:** `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_axi.json --provider skip 2>/dev/null | grep -A1 source-tolerant`
- **source:** [`docs/knowledge/axi-constraint-subject-must-be-declared.md`](docs/knowledge/axi-constraint-subject-must-be-declared.md)

### conformal-tier-agreement-degenerate
_Tier-agreement is a degenerate conformal axis — the extraction tiers complement, they don't corroborate_

- **answers:** why doesn't the NLI-oracle conformal calibration produce a threshold | is tier-agreement a good confidence axis for conformal calibration | do the Pattern and Nlp extraction tiers find the same constraints | what confidence axis correlates with extracted-constraint correctness | why is conformal calibration still blocked at CHI scale
- **date:** 2026-06-06 · **status:** current
- **evidence:** `docs/tasks/TABLE-GRITS-CONFORMAL.md`
- **reverify:** `grep -n "tier_count_by_fact_key\|nli_conformal_pass" crates/specforge/src/ir/nli_verify.rs`
- **source:** [`docs/knowledge/conformal-tier-agreement-degenerate.md`](docs/knowledge/conformal-tier-agreement-degenerate.md)

### contested-priors
_SpecForge detects contested priors (same key, conflicting values across docs) — read-only_

- **answers:** how does SpecForge detect contradicting or conflicting priors | does SpecForge revise or decay priors | what is a contested prior | where are cross-document prior contradictions surfaced | does corpus prior memory only accrete
- **date:** 2026-06-04 · **status:** current
- **evidence:** `crates/specforge/src/ir/prior_memory.rs; crates/specforge/src/commands/learn_priors.rs`
- **reverify:** `grep -n "fn contested_priors" crates/specforge/src/ir/prior_memory.rs`
- **source:** [`docs/knowledge/contested-priors.md`](docs/knowledge/contested-priors.md)

### corpus-coverage-sweep
_Whole-corpus coverage sweep (2026-06-08) — Lever A+B uplift measured across the 82-PDF library_

- **answers:** how much intent does SpecForge extract across the whole corpus | what is the PDF-VARIANT-DIGESTION whole-corpus coverage / re-triage | which corpus docs still yield nothing (the VLM frontier) | which docs fail to ingest (giants / timeouts)
- **date:** 2026-06-08 · **status:** current
- **evidence:** `docs/corpus_coverage_2026-06-08.md (per-doc matrix); /tmp/corpus_coverage.tsv (raw)`
- **reverify:** `python3 over generated/evidence_ir/*/evidence_ir.json — count Signal-decls / register_records+fields / protocol_actors / signal_constraints / actor_signal_relations per doc-key`
- **source:** [`docs/knowledge/corpus-coverage-sweep.md`](docs/knowledge/corpus-coverage-sweep.md)

### dempster-fusion
_SpecForge fuses agreeing-source confidence via Dempster corroboration (not min)_

- **answers:** how does SpecForge combine confidence across modalities or sources | does agreement between sources boost confidence | why doesn't fusion use the minimum confidence | what is the Dempster combiner in fusion | how is a fused contract's automation_confidence computed
- **date:** 2026-06-04 · **status:** current
- **evidence:** `crates/specforge/src/ir/fusion.rs`
- **reverify:** `grep -n "fn dempster_corroborate_confidence" crates/specforge/src/ir/fusion.rs`
- **source:** [`docs/knowledge/dempster-fusion.md`](docs/knowledge/dempster-fusion.md)

### docling-device-cpu
_Docling ingest must run on CPU on this stack (torch MPS lacks float64)_

- **answers:** why does Docling re-ingest fail on Apple Silicon | Cannot convert a MPS Tensor to float64 | how do I run a Docling ingest or re-ingest on this machine | what does DOCLING_DEVICE do | torch MPS float64 error during ingest
- **date:** 2026-06-01 · **status:** current
- **evidence:** `docs/decisions/0001-docling-device-cpu.md; crates/specforge/src/ir/source/docling_backend.rs:444`
- **reverify:** `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- **source:** [`docs/knowledge/docling-device-cpu.md`](docs/knowledge/docling-device-cpu.md)

### eval-gold-interannotator-kappa
_Eval gold is reliable — Cohen's kappa 0.90 (almost-perfect) on the constraint task_

- **answers:** is the eval gold / answer key trustworthy or reliable | what is the inter-annotator agreement of the eval gold | did a second annotator validate seed_apb.json | how was the eval gold checked for idiosyncrasy
- **date:** 2026-06-05 · **status:** current
- **evidence:** `crates/specforge/test_data/llm_eval/seed_apb.json; docs/tasks/EVAL-GOLD-INTERANNOTATOR-AGREEMENT.md`
- **reverify:** `true  # MANUAL: re-run a blind second annotation per EVAL-GOLD-INTERANNOTATOR-AGREEMENT.md Method and recompute kappa (not an automatable grep)`
- **source:** [`docs/knowledge/eval-gold-interannotator-kappa.md`](docs/knowledge/eval-gold-interannotator-kappa.md)

### eval-scores-persisted-evidence
_eval-extraction scores the PERSISTED evidence_ir.json — rebuild before trusting a baseline (it can be stale)_

- **answers:** why is the AHB eval baseline wrong or stale | does eval-extraction rebuild evidence or load the persisted file | why do FPs appear in eval that the current code does not produce | how to get a fresh eval-extraction baseline for a spec | why can't specforge evidence rebuild the evidence (normalized missing) | is the WIRE-BASED-100.5a AHB 0.364 baseline real
- **date:** 2026-06-07 · **status:** current
- **evidence:** `docs/tasks/WIRE-BASED-100.md (.5a/.5b); crates/specforge/src/commands/eval_extraction.rs`
- **reverify:** `./target/debug/specforge evidence generated/source_ir/<doc_key>/source_ir.json   # errors if normalized was reclaimed`
- **source:** [`docs/knowledge/eval-scores-persisted-evidence.md`](docs/knowledge/eval-scores-persisted-evidence.md)

### extraction-audit-vlm
_audit-extraction — VLM proposer/verifier precision estimate over the broadened table-driven extraction_

- **answers:** how is the precision of the broadened (non-gold) extraction measured / estimated | what is audit-extraction / PDF-VARIANT-DIGESTION.4b | how do you audit registers/signals against the table image with the VLM | what is the table-kind precision estimate and the flagged-mismatch list | is the extraction audit chip-spec-PDF agnostic (yes)
- **date:** 2026-06-08 · **status:** current
- **evidence:** `crates/specforge/src/commands/audit_extraction.rs (audited_kind, select_sample, build_audit_prompt, parse_audit_verdict, aggregate); reuses commands/enrich.rs::vlm_image_query + ir/evidence.rs::is_register_field_header`
- **reverify:** `./target/debug/specforge audit-extraction generated/source_ir/1_0_risc_v_debug_specification/source_ir.json --sample 8           # plan-only: lists 8 sampled intent-bearing tables, no VLM calls`
- **source:** [`docs/knowledge/extraction-audit-vlm.md`](docs/knowledge/extraction-audit-vlm.md)

### fsmgen-feedback-channel
_SpecForge -> FSMGen feedback channel is docs/FSMGEN_FEEDBACK.md (+ issue bundles for bugs)_

- **answers:** where do I log feedback or a suggestion to FSMGen | where is the SpecForge FSMGen feedback or handoff channel | how do I file an FSMGen bug report or feature request | where did SpecForge suggest LTL/MTL support in ISF | what is the FSMGen issue bundle protocol
- **date:** 2026-06-04 · **status:** current
- **evidence:** `docs/FSMGEN_FEEDBACK.md; subs/fsmgen/docs/DOWNSTREAM_ISSUE_REPORTING.md; docs/fsmgen-issues/`
- **reverify:** `ls docs/FSMGEN_FEEDBACK.md`
- **source:** [`docs/knowledge/fsmgen-feedback-channel.md`](docs/knowledge/fsmgen-feedback-channel.md)

### fsmgen-temporal-isf-form
_SpecForge emits bounded-eventually as (assert (monitor (within s N))) into .isf (fsmgen pin 43b29f5c)_

- **answers:** how does SpecForge emit temporal rules or a bounded-eventually into .isf | what ISF form does SpecForge use for a bounded-eventually contract | was the (contract ... eventually ...) ISF clause removed | what fsmgen pin does SpecForge target for temporal properties | why doesn't SpecForge emit (contract eventually) anymore
- **date:** 2026-06-04 · **status:** current
- **evidence:** `crates/specforge/src/ir/isf_ir.rs; subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md`
- **reverify:** `grep -n "assert (monitor (within" crates/specforge/src/ir/isf_ir.rs`
- **source:** [`docs/knowledge/fsmgen-temporal-isf-form.md`](docs/knowledge/fsmgen-temporal-isf-form.md)

### indexed-signal-family-canonicalization
_Un-indexed prose signal refs resolve to the declared indexed family member (PSEL → PSELx)_

- **answers:** why is the PSEL antecedent dropped in a temporal rule | how does specforge handle PSEL vs PSELx (or HSEL vs HSELx) | what is index-family signal canonicalization | what does resolve_indexed_signal_family do | why does a temporal antecedent use PSELX not PSEL | how are per-instance indexed signals (PSELx HSELx) referenced in prose handled | how did APB temporal reach 100% (WIRE-BASED-100.4)
- **date:** 2026-06-06 · **status:** current
- **evidence:** `docs/tasks/WIRE-BASED-100.md; crates/specforge/src/ir/semantic.rs (resolve_indexed_signal_family, temporal_clause_value, parse_temporal_condition_predicates)`
- **reverify:** `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_apb_temporal.json --provider skip 2>/dev/null | sed -n '/Extraction eval/,/source-tolerant/p'`
- **source:** [`docs/knowledge/indexed-signal-family-canonicalization.md`](docs/knowledge/indexed-signal-family-canonicalization.md)

### isf-fsm-via-switch-select
_ISF CAN describe a state machine — proven idiom is storage-var + switch + select + rule-trigger (FSMGen lowers it)_

- **answers:** can ISF model an explicit state machine / FSM (proven) | how to express the JTAG TAP / SWD FSM in .isf | what ISF idiom describes states and input-driven transitions | does SpecForge cycle-schedule the FSM (no — FSMGen does) | why was the ISF explicit-FSM feature request withdrawn
- **date:** 2026-06-07 · **status:** current
- **evidence:** `docs/fsmgen-issues/sf-isf-explicit-fsm-declaration/README.md (WITHDRAWN); subs/fsmgen @ d31b0b91; empirical fsmgen --strict --check --json probes`
- **reverify:** `write a state machine as (storage (var st ...)) + (transaction step (on start) (switch st (S (select st input A B))...) (complete done)) + (rule tick start (trigger step)); run subs/fsmgen/bin/fsmgen --strict --check --json FILE → success:true`
- **source:** [`docs/knowledge/isf-fsm-via-switch-select.md`](docs/knowledge/isf-fsm-via-switch-select.md)

### isf-temporal-lowering-no-silent-drop
_temporal_rules are never silently dropped in the IntentIR->.isf lowering (already guaranteed)_

- **answers:** are temporal rules silently dropped when lowering IntentIR to .isf | does every temporal_rule reach the .isf or a residual | is there a lowering-completeness check for temporal rules | where does .isf record dropped temporal obligations | should I build an isf lowering-completeness verifier for temporal rules
- **date:** 2026-06-04 · **status:** current
- **evidence:** `crates/specforge/src/ir/isf_ir.rs; crates/specforge/src/ir/adapters.rs`
- **reverify:** `grep -n "temporal_residuals" crates/specforge/src/ir/isf_ir.rs crates/specforge/src/ir/adapters.rs`
- **source:** [`docs/knowledge/isf-temporal-lowering-no-silent-drop.md`](docs/knowledge/isf-temporal-lowering-no-silent-drop.md)

### llm-vlm-provider-default
_SpecForge ships a production Ollama+Qwen2.5VL provider (the default LLM/VLM)_

- **answers:** which LLM or VLM does SpecForge use | is the LLM/VLM provider missing or not wired up | what model do converge / enrich / nlp-enrich use by default | qwen2.5vl vs qwen3-vl which model | default model for the ollama provider
- **date:** 2026-06-01 · **status:** current
- **evidence:** `docs/decisions/0002-llm-vlm-provider-default.md; crates/specforge/src/commands/llm_text.rs:35`
- **reverify:** `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- **source:** [`docs/knowledge/llm-vlm-provider-default.md`](docs/knowledge/llm-vlm-provider-default.md)

### local-llm-for-text-reasoning
_Text-reasoning gates (NLI, semantic checks) want a strong TEXT LLM, not a VLM; qwen2.5:14b-instruct is viable for NLI_

- **answers:** which local model should SpecForge use for NLI or entailment verification | do text-reasoning tasks need a vision model | is qwen2.5:14b-instruct good enough for NLI | why is the NLI framing better than free-form labeling | which local models are pulled and what are they for
- **date:** 2026-06-05 · **status:** current
- **evidence:** `docs/knowledge/eval-gold-interannotator-kappa.md; docs/tasks/EVAL-GOLD-INTERANNOTATOR-AGREEMENT.md`
- **reverify:** `ollama list  # qwen2.5:14b-instruct (text) + qwen2.5vl:7b (vision); re-run the NLI/kappa probes`
- **source:** [`docs/knowledge/local-llm-for-text-reasoning.md`](docs/knowledge/local-llm-for-text-reasoning.md)

### nli-entailment-verifier
_NLI entailment verifier — a semantic "does the source entail this claim?" grounding gate_

- **answers:** how does SpecForge verify an extracted claim semantically / catch hallucination | what is the NLI entailment verifier | how is a claim's grounding checked beyond a string match | what model does the NLI verifier use | what happens when the NLI provider is down
- **date:** 2026-06-05 · **status:** current
- **evidence:** `crates/specforge/src/ir/nli_verify.rs`
- **reverify:** `grep -n "fn verify_entailment" crates/specforge/src/ir/nli_verify.rs`
- **source:** [`docs/knowledge/nli-entailment-verifier.md`](docs/knowledge/nli-entailment-verifier.md)

### nli-gate-real-apb-validation
_NLI gate validated on the real AMBA APB spec — it works, and it exposes constraint over-generation_

- **answers:** does the NLI verifier actually catch real extraction errors | what did running nli-verify on a real spec find | is SpecForge's constraint extraction over-generating | why does condition_text matter for the NLI claim
- **date:** 2026-06-05 · **status:** current
- **evidence:** `crates/specforge/src/ir/nli_verify.rs`
- **reverify:** `cargo run -p specforge --quiet -- nli-verify generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json --vlm-provider ollama --model qwen2.5:14b-instruct`
- **source:** [`docs/knowledge/nli-gate-real-apb-validation.md`](docs/knowledge/nli-gate-real-apb-validation.md)

### nli-intent-gate
_NLI intent gate — active demote-to-residual of un-entailed contracts (intent --nli-verify)_

- **answers:** how do I make the NLI verifier actively change extraction / demote claims | what does intent --nli-verify do | where does the NLI gate route a not-entailed contract | how is the NLI gate tested without Ollama
- **date:** 2026-06-05 · **status:** current
- **evidence:** `crates/specforge/src/ir/nli_verify.rs`
- **reverify:** `grep -n "fn apply_nli_gate\|fn nli_gate_contracts\|fn obligation_claim_text" crates/specforge/src/ir/nli_verify.rs`
- **source:** [`docs/knowledge/nli-intent-gate.md`](docs/knowledge/nli-intent-gate.md)

### pdf-encryption-and-read-access
_12/82 corpus PDFs are permission-encrypted (open w/ empty password); docling+pypdf read all 82; the Read tool is unreliable_

- **answers:** which corpus PDFs are password/permission protected | do any chip-spec PDFs need a real password (no) | why does the Claude Read tool refuse some PDFs / report password-protected | how to read a chip-spec PDF when the Read tool refuses it | what are scripts/pdf_text.py and scripts/decrypt_pdf.py
- **date:** 2026-06-07 · **status:** current
- **evidence:** `scripts/pdf_text.py; scripts/decrypt_pdf.py; pypdf+cryptography in .venv-docling; pdfinfo`
- **reverify:** `.venv-docling/bin/python -c \"from pypdf import PdfReader; r=PdfReader('<pdf>'); print('enc',r.is_encrypted); r.decrypt('') if r.is_encrypted else 0; print(len(r.pages))\"`
- **source:** [`docs/knowledge/pdf-encryption-and-read-access.md`](docs/knowledge/pdf-encryption-and-read-access.md)

### prose-pin-appositive-signal-capture
_Interface signals declared in prose ("a clock pin, SWCLK") are captured via the pin-appositive pattern_

- **answers:** how are SWCLK and SWDIO captured if they are not in a signal table | how does specforge declare a signal mentioned only in prose | what is synthesize_signal_declarations_from_prose / the pin-appositive pattern | how are serial/architecture spec interface signals added to the catalog
- **date:** 2026-06-07 · **status:** current
- **evidence:** `docs/tasks/SWD-SERIAL-EXTRACTION.md (.2); crates/specforge/src/ir/evidence.rs (synthesize_signal_declarations_from_prose)`
- **reverify:** `python3 -c "import json,re; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); d={re.match(r'Signal (\w+)',s['text']).group(1) for s in e['extracted_statements'] if s['text'].startswith('Signal ')}; print('SWCLK',('SWCLK' in d),'SWDIO',('SWDIO' in d))`
- **source:** [`docs/knowledge/prose-pin-appositive-signal-capture.md`](docs/knowledge/prose-pin-appositive-signal-capture.md)

### prose-signal-capture
_Signals introduced in PROSE (not tables) are captured — pin appositive + parenthetical abbreviation, as a sparse-catalog fallback_

- **answers:** how does SpecForge capture signals that are in prose not tables (I2C SDA/SCL) | what is PDF-VARIANT-DIGESTION.3 prose entity capture | why do I2C/CCIX/USB4 have 0 table signals and how are they recovered | how is prose signal over-capture prevented (no garbage)
- **date:** 2026-06-07 · **status:** current
- **evidence:** `crates/specforge/src/ir/evidence.rs (synthesize_signal_declarations_from_prose)`
- **reverify:** `./target/debug/specforge evidence generated/source_ir/um10204_rev7_0_2021_i2c_bus_specification/source_ir.json && python3 -c "import json,re;e=json.load(open('generated/evidence_ir/um10204_rev7_0_2021_i2c_bus_specification/evidence_ir.json'));print(sorted({re.match(r'Signal (\w+)',x['text']).group(1) for x in e['extracted_statements'] if x['text'].startswith('Signal ')}))`
- **source:** [`docs/knowledge/prose-signal-capture.md`](docs/knowledge/prose-signal-capture.md)

### prose-signal-capture-i2c-precision
_Prose-signal capture on I2C — measured 0.600 precision then FIXED to 1.000 via the noun-phrase head rule_

- **answers:** how good is prose signal capture / .3a quality | what is the I2C declared-signal recall / precision | why were ACK / NACK / DDC / SDR extracted as I2C signals (and how was it fixed) | what is the declared-signal eval surface / EvalTask::DeclaredSignal | what is declared_signal_complete_gold_precision | where is the I2C signal gold seed | what is the parenthetical noun-phrase head rule / EXTRACTION-GAP-FIX.1
- **date:** 2026-06-08 · **status:** current
- **evidence:** `crates/specforge/src/eval.rs (EvalTask::DeclaredSignal, GoldFact::DeclaredSignal, declared_signal_record_key, index_declared_signal_predictions, declared_signal_complete_gold_precision); crates/specforge/src/commands/eval_extraction.rs (declared-signal surface); crates/specforge/test_data/llm_eval/seed_i2c_signals.json; docs/tasks/PDF-VARIANT-DIGESTION.md (.4a.4/.4a.5)`
- **reverify:** `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_i2c_signals.json --provider skip 2>&1 | grep -A2 "declared-signal surface`
- **source:** [`docs/knowledge/prose-signal-capture-i2c-precision.md`](docs/knowledge/prose-signal-capture-i2c-precision.md)

### register-field-eval-measure-and-surface
_Register-field extraction quality is measured per-fact, with a "measure & surface" decomposition (RISC-V Debug = 0.588 field-name recall)_

- **answers:** how is register-field extraction quality measured / scored | what is the register-field eval surface (EvalTask::RegisterField) | what is the RISC-V Debug register-field recall / precision | what is the NVMe register-field recall / precision | why is the strict register-field per-fact score 0 on RISC-V Debug / NVMe | what is register_field_name_recall / register_field_completeness / register_bit_structure_recall | where is the register-field gold seed
- **date:** 2026-06-08 · **status:** current
- **evidence:** `crates/specforge/src/eval.rs (EvalTask::RegisterField, GoldFact::RegisterField, register_field_record_key, register_field_name_recall, register_field_completeness); crates/specforge/src/commands/eval_extraction.rs (register-field surface block); crates/specforge/test_data/llm_eval/seed_riscv_debug_registers.json; docs/tasks/PDF-VARIANT-DIGESTION.md (.4a.1/.4a.2)`
- **reverify:** `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_riscv_debug_registers.json --provider skip 2>&1 | grep -A3 "register-field surface`
- **source:** [`docs/knowledge/register-field-eval-measure-and-surface.md`](docs/knowledge/register-field-eval-measure-and-surface.md)

### register-field-table-extraction
_Register-FIELD tables (Field|…|Access|Reset) the classifier left "unknown" are recovered into RegisterRecords_

- **answers:** how does SpecForge extract register fields from tables | why were RISC-V/TRM register tables unextracted (unknown table_kind) | what is synthesize_register_field_tables | how flexible is the register model / what register-table shapes are handled | how is a register-field mnemonic recovered when the name column is a bit-range (NVMe) | why is NVMe register field_name a bit-range and how is the mnemonic found in the description
- **date:** 2026-06-08 · **status:** current
- **evidence:** `crates/specforge/src/ir/evidence.rs (synthesize_register_field_tables, is_register_field_header, register_name_from_caption); docs/tasks/PDF-VARIANT-DIGESTION.md`
- **reverify:** `./target/debug/specforge evidence generated/source_ir/1_0_risc_v_debug_specification/source_ir.json && python3 -c "import json;e=json.load(open('generated/evidence_ir/1_0_risc_v_debug_specification/evidence_ir.json'));print(len(e['register_records']),'regs',sum(len(r['fields']) for r in e['register_records']),'fields')`
- **source:** [`docs/knowledge/register-field-table-extraction.md`](docs/knowledge/register-field-table-extraction.md)

### rotated-signal-table-extraction
_Misaligned signal tables (name column rotated to last) are extracted by content-based column detection_

- **answers:** why was a signal not extracted from a signal table (e.g. AHB HREADY) | how does specforge handle a signal table whose name column is not first | what is content-based name-column detection / rotation offset remapping | how was AHB HREADY recovered for the temporal antecedent | what does synthesize_signal_declarations do when the body is rotated | how is an unless/except exception clause handled in a temporal condition
- **date:** 2026-06-07 · **status:** current
- **evidence:** `docs/tasks/WIRE-BASED-100.md (.5h); crates/specforge/src/ir/evidence.rs (synthesize_signal_declarations); crates/specforge/src/ir/semantic.rs (parse_temporal_condition_predicates)`
- **reverify:** `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_ahb_temporal.json --provider skip 2>/dev/null | grep temporal_rule`
- **source:** [`docs/knowledge/rotated-signal-table-extraction.md`](docs/knowledge/rotated-signal-table-extraction.md)

### spec-mining-framing
_SpecForge is forward specification mining (spec -> intent, not implementation -> spec)_

- **answers:** what is SpecForge doing in academic or research terms | is SpecForge specification mining | how does SpecForge relate to GoldMine Texada Pnueli Ammons | what does SpecForge take from the spec-mining literature and what does it leave out | why is SpecForge called forward specification mining
- **date:** 2026-06-04 · **status:** current
- **evidence:** `docs/research/grounding/adopt-defer-ledger.md; docs/book/src/architecture-rationale.md; README.md`
- **reverify:** `grep -rn "forward specification mining" README.md docs/book/src/architecture-rationale.md`
- **source:** [`docs/knowledge/spec-mining-framing.md`](docs/knowledge/spec-mining-framing.md)

### stable-obligation-phase-scoped-residual
_SpecForge stability obligations stay .isf residuals — phase-scoped, not FSMGen's unconditional (stable s)_

- **answers:** why doesn't SpecForge lower stability obligations to (assert (stable sig)) | can SpecForge use FSMGen's (stable …) sampled-value predicate | are SignalStable obligations representable in .isf | why are stability obligations residuals
- **date:** 2026-06-04 · **status:** current
- **evidence:** `crates/specforge/src/ir/contract.rs (contract_from_temporal_rule, Obligation::Stable); subs/fsmgen/docs/knowledge/isf-sampled-value-predicates.md`
- **reverify:** `grep -n "bare stability across tick phases" crates/specforge/src/ir/contract.rs`
- **source:** [`docs/knowledge/stable-obligation-phase-scoped-residual.md`](docs/knowledge/stable-obligation-phase-scoped-residual.md)

### swd-adi-not-signal-table-spec
_SWD/ADI (IHI0074) is an architecture/serial spec — the parallel-bus signal-table model doesn't fit_

- **answers:** why is SWD/ADI hard / different from APB AHB AXI | are SWCLK and SWDIO extracted / declared | why does the ADI spec produce so few signals and so much garbage | can WIRE-BASED-100 reach 100% on SWD the same way as the parallel buses | what extraction approach does SWD/ADI need
- **date:** 2026-06-07 · **status:** current
- **evidence:** `corpus/arm/debug/interfaces/adi/current/IHI0074_A_2017-03-09_Arm_Debug_Interface_v6_Architecture_Specification.pdf; generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification`
- **reverify:** `python3 -c "import json,re; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print(sorted({re.match(r'Signal (\w+)',s['text']).group(1) for s in e['extracted_statements'] if s['text'].startswith('Signal ')}))`
- **source:** [`docs/knowledge/swd-adi-not-signal-table-spec.md`](docs/knowledge/swd-adi-not-signal-table-spec.md)

### swd-derivation-scored-100
_SWD intent derivation is scored 100% — eval-extraction gained serial_frame_field / swd_operation / protocol_state tasks_

- **answers:** how is the SWD FSM/frame derivation scored (not constraints/relations/temporal) | what eval-extraction tasks score the SWD surfaces | what is in seed_swd_derivation.json | is SWD at 100% and on what metric | how to re-score SWD derivation
- **date:** 2026-06-07 · **status:** current
- **evidence:** `docs/tasks/SWD-SERIAL-EXTRACTION.md (.5); crates/specforge/src/eval.rs; crates/specforge/src/commands/eval_extraction.rs; crates/specforge/test_data/llm_eval/seed_swd_derivation.json`
- **reverify:** `./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_swd_derivation.json --provider skip 2>/dev/null | sed -n '/source-tolerant/,/document-level/p'`
- **source:** [`docs/knowledge/swd-derivation-scored-100.md`](docs/knowledge/swd-derivation-scored-100.md)

### swd-intent-is-the-fsm-driving-swdio
_SWD's intent = its packet protocol + line state machine on SWDIO (read from spec Chapter B4)_

- **answers:** what is SWD's actual intent / protocol (from the spec) | what are the SWD packet phases and per-phase SWDIO direction | what is the SWD line state machine (reset/operating/protocol-error/lockout) | what must SpecForge derive to fully capture SWD; what are the gaps | is the SWD FSM the same as the JTAG TAP DBGTAPSM (no)
- **date:** 2026-06-07 · **status:** current
- **evidence:** `corpus/.../IHI0074_A Chapter B4 (read directly via docling content_elements, pages 110-128); generated/source_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification`
- **reverify:** `python3 — dump source_ir content_elements for page_id page_0110..page_0128 (Chapter B4); the PDF itself is password-protected so the Read tool cannot open it — use the docling content_elements text.`
- **source:** [`docs/knowledge/swd-intent-is-the-fsm-driving-swdio.md`](docs/knowledge/swd-intent-is-the-fsm-driving-swdio.md)

### swd-protocol-fsm-surface
_The SWD/JTAG protocol FSM is a typed surface (ProtocolStateRecord) — states extracted from "<State> state" grammar_

- **answers:** how does specforge model the JTAG TAP / SWD state machine (FSM) | what is ProtocolStateRecord / protocol_states / DBGTAPSM | how are TAP states (Shift-DR, Run-Test/Idle, Test-Logic-Reset) extracted | why is the FSM important for SWD/JTAG | how are per-state actions captured
- **date:** 2026-06-07 · **status:** current
- **evidence:** `docs/tasks/SWD-SERIAL-EXTRACTION.md (.4); crates/specforge/src/ir/evidence.rs (extract_protocol_states, find_states_with_actions, looks_like_state_name, ProtocolStateRecord)`
- **reverify:** `python3 -c "import json; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print([s['state_name'] for s in e.get('protocol_states',[])])`
- **source:** [`docs/knowledge/swd-protocol-fsm-surface.md`](docs/knowledge/swd-protocol-fsm-surface.md)

### swd-serial-frame-surface
_SWD serial-frame fields are a distinct typed surface (SerialFrameField), double-gated to serial docs + frame phases_

- **answers:** how does specforge model the SWD serial frame / packet | what is SerialFrameField / serial_frame_fields / SerialFramePhase | how are ACK WDATA RDATA DATAIN bit-widths extracted | why don't parallel buses get serial_frame_fields | how are NAME[hi:lo] bit-ranges parsed into frame fields
- **date:** 2026-06-07 · **status:** current
- **evidence:** `docs/tasks/SWD-SERIAL-EXTRACTION.md (.3); crates/specforge/src/ir/evidence.rs (extract_serial_frame_fields, parse_bit_range_fields, SerialFrameField)`
- **reverify:** `python3 -c "import json; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print([(f['name'],f.get('bit_width')) for f in e.get('serial_frame_fields',[])])`
- **source:** [`docs/knowledge/swd-serial-frame-surface.md`](docs/knowledge/swd-serial-frame-surface.md)

### temporal-eval-residual-fps-are-stale
_The temporal-rule eval's residual false positives are a stale artifact, not a live bug_

- **answers:** why doesn't the temporal_rule eval reach precision 1.0 | are the degenerate PSEL-header or WIDTH-subject temporal rules a live bug | should I fix the PSEL valid when PSEL asserted temporal rule | temporal rule eval false positives root cause | is the eval-extraction temporal precision 0.6 a real defect
- **date:** 2026-06-02 · **status:** current
- **evidence:** `crates/specforge/src/ir/evidence.rs:2838; crates/specforge/src/ir/evidence.rs:5756; docs/tasks/TEMPORAL-RULE-EVAL.md`
- **reverify:** `grep -n " when " crates/specforge/src/ir/evidence.rs`
- **source:** [`docs/knowledge/temporal-eval-residual-fps-are-stale.md`](docs/knowledge/temporal-eval-residual-fps-are-stale.md)

### temporal-logic-choice
_SpecForge captures temporal behavior in LTL/MTL, not CTL or TLA+_

- **answers:** does SpecForge use LTL CTL or TLA+ | why not CTL for temporal behavior | why doesn't SpecForge use TLA+ | what temporal logic backs temporal_rules | does SpecForge model-check temporal properties
- **date:** 2026-06-04 · **status:** current
- **evidence:** `docs/decisions/0005-temporal-logic-ltl-mtl-not-ctl-tla.md; crates/specforge/src/ir/temporal_ltl.rs`
- **reverify:** `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- **source:** [`docs/knowledge/temporal-logic-choice.md`](docs/knowledge/temporal-logic-choice.md)

### temporal-rule-ltl-rendering
_Temporal rules render to standard LTL/MTL via ir/temporal_ltl.rs_

- **answers:** how are temporal rules expressed as LTL or MTL | where is the LTL renderer for temporal rules | what is the LTL form of a temporal_rule | is there a PSL or SVA export of temporal rules | temporal rule predicate atom vocabulary
- **date:** 2026-06-04 · **status:** current
- **evidence:** `crates/specforge/src/ir/temporal_ltl.rs; docs/book/src/domain/temporal-semantics.md`
- **reverify:** `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- **source:** [`docs/knowledge/temporal-rule-ltl-rendering.md`](docs/knowledge/temporal-rule-ltl-rendering.md)

### vlm-table-strategy
_VLM (Qwen2.5VL) reads table images to reclassify "unknown" tables — the second, best-wins table strategy_

- **answers:** how does the VLM understand tables / can a VLM read PDF tables | what is the VLM table strategy / PDF-VARIANT-DIGESTION.2b | how are unknown tables reclassified by the VLM | does encryption block the VLM from reading tables (no)
- **date:** 2026-06-07 · **status:** current
- **evidence:** `crates/specforge/src/commands/enrich.rs (vlm_image_query, build_table_classify_prompt, parse_vlm_table_kind, classify_unknown_tables_via_vlm)`
- **reverify:** `./target/debug/specforge enrich generated/source_ir/<key>/source_ir.json --vlm-provider ollama --vlm-model qwen2.5vl:7b   # prints tables_reclassified_by_vlm`
- **source:** [`docs/knowledge/vlm-table-strategy.md`](docs/knowledge/vlm-table-strategy.md)
