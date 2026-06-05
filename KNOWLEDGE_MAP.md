# Knowledge Map

> **AUTO-GENERATED — DO NOT EDIT.** Regenerate with `knowledge-map/scripts/gen_knowledge_map.sh`.
> Source of truth = YAML front-matter in: `docs/knowledge docs/decisions`. Edit the fact files, never this map.
> A fact is any `.md` whose front-matter has a non-empty `answers:` list.
> **17** facts · **84** question keys.

## Questions → fact

- "Cannot convert a MPS Tensor to float64" -> [docling-device-cpu](docs/knowledge/docling-device-cpu.md) · 2026-06-01 · reverify: `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- "are SignalStable obligations representable in .isf" -> [stable-obligation-phase-scoped-residual](docs/knowledge/stable-obligation-phase-scoped-residual.md) · 2026-06-04 · reverify: `grep -n "bare stability across tick phases" crates/specforge/src/ir/contract.rs`
- "are temporal rules silently dropped when lowering IntentIR to .isf" -> [isf-temporal-lowering-no-silent-drop](docs/knowledge/isf-temporal-lowering-no-silent-drop.md) · 2026-06-04 · reverify: `grep -n "temporal_residuals" crates/specforge/src/ir/isf_ir.rs crates/specforge/src/ir/adapters.rs`
- "are the degenerate PSEL-header or WIDTH-subject temporal rules a live bug" -> [temporal-eval-residual-fps-are-stale](docs/knowledge/temporal-eval-residual-fps-are-stale.md) · 2026-06-02 · reverify: `grep -n " when " crates/specforge/src/ir/evidence.rs`
- "can SpecForge use FSMGen's (stable …) sampled-value predicate" -> [stable-obligation-phase-scoped-residual](docs/knowledge/stable-obligation-phase-scoped-residual.md) · 2026-06-04 · reverify: `grep -n "bare stability across tick phases" crates/specforge/src/ir/contract.rs`
- "default model for the ollama provider" -> [llm-vlm-provider-default](docs/knowledge/llm-vlm-provider-default.md) · 2026-06-01 · reverify: `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- "did a second annotator validate seed_apb.json" -> [eval-gold-interannotator-kappa](docs/knowledge/eval-gold-interannotator-kappa.md) · 2026-06-05 · reverify: `python3 -c "see EVAL-GOLD-INTERANNOTATOR-AGREEMENT.md Method — re-run a blind second annotation and recompute`
- "do text-reasoning tasks need a vision model" -> [local-llm-for-text-reasoning](docs/knowledge/local-llm-for-text-reasoning.md) · 2026-06-05 · reverify: `ollama list  # qwen2.5:14b-instruct (text) + qwen2.5vl:7b (vision); re-run the NLI/kappa probes`
- "does SpecForge detect implementation-defined or TBD or and/or" -> [ambiguity-weak-phrase-detector](docs/knowledge/ambiguity-weak-phrase-detector.md) · 2026-06-04 · reverify: `grep -n "fn weak_phrase_findings" crates/specforge/src/ir/ambiguity.rs`
- "does SpecForge model-check temporal properties" -> [temporal-logic-choice](docs/knowledge/temporal-logic-choice.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "does SpecForge revise or decay priors" -> [contested-priors](docs/knowledge/contested-priors.md) · 2026-06-04 · reverify: `grep -n "fn contested_priors" crates/specforge/src/ir/prior_memory.rs`
- "does SpecForge use LTL CTL or TLA+" -> [temporal-logic-choice](docs/knowledge/temporal-logic-choice.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "does agreement between sources boost confidence" -> [dempster-fusion](docs/knowledge/dempster-fusion.md) · 2026-06-04 · reverify: `grep -n "fn dempster_corroborate_confidence" crates/specforge/src/ir/fusion.rs`
- "does corpus prior memory only accrete" -> [contested-priors](docs/knowledge/contested-priors.md) · 2026-06-04 · reverify: `grep -n "fn contested_priors" crates/specforge/src/ir/prior_memory.rs`
- "does every temporal_rule reach the .isf or a residual" -> [isf-temporal-lowering-no-silent-drop](docs/knowledge/isf-temporal-lowering-no-silent-drop.md) · 2026-06-04 · reverify: `grep -n "temporal_residuals" crates/specforge/src/ir/isf_ir.rs crates/specforge/src/ir/adapters.rs`
- "how are temporal rules expressed as LTL or MTL" -> [temporal-rule-ltl-rendering](docs/knowledge/temporal-rule-ltl-rendering.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "how do I file an FSMGen bug report or feature request" -> [fsmgen-feedback-channel](docs/knowledge/fsmgen-feedback-channel.md) · 2026-06-04 · reverify: `ls docs/FSMGEN_FEEDBACK.md`
- "how do I run a Docling ingest or re-ingest on this machine" -> [docling-device-cpu](docs/knowledge/docling-device-cpu.md) · 2026-06-01 · reverify: `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- "how does SpecForge combine confidence across modalities or sources" -> [dempster-fusion](docs/knowledge/dempster-fusion.md) · 2026-06-04 · reverify: `grep -n "fn dempster_corroborate_confidence" crates/specforge/src/ir/fusion.rs`
- "how does SpecForge detect contradicting or conflicting priors" -> [contested-priors](docs/knowledge/contested-priors.md) · 2026-06-04 · reverify: `grep -n "fn contested_priors" crates/specforge/src/ir/prior_memory.rs`
- "how does SpecForge emit temporal rules or a bounded-eventually into .isf" -> [fsmgen-temporal-isf-form](docs/knowledge/fsmgen-temporal-isf-form.md) · 2026-06-04 · reverify: `grep -n "assert (monitor (within" crates/specforge/src/ir/isf_ir.rs`
- "how does SpecForge flag vague or ambiguous spec language" -> [ambiguity-weak-phrase-detector](docs/knowledge/ambiguity-weak-phrase-detector.md) · 2026-06-04 · reverify: `grep -n "fn weak_phrase_findings" crates/specforge/src/ir/ambiguity.rs`
- "how does SpecForge relate to GoldMine Texada Pnueli Ammons" -> [spec-mining-framing](docs/knowledge/spec-mining-framing.md) · 2026-06-04 · reverify: `grep -rn "forward specification mining" README.md docs/book/src/architecture-rationale.md`
- "how does SpecForge verify an extracted claim semantically / catch hallucination" -> [nli-entailment-verifier](docs/knowledge/nli-entailment-verifier.md) · 2026-06-05 · reverify: `grep -n "fn verify_entailment" crates/specforge/src/ir/nli_verify.rs`
- "how is a claim's grounding checked beyond a string match" -> [nli-entailment-verifier](docs/knowledge/nli-entailment-verifier.md) · 2026-06-05 · reverify: `grep -n "fn verify_entailment" crates/specforge/src/ir/nli_verify.rs`
- "how is a fused contract's automation_confidence computed" -> [dempster-fusion](docs/knowledge/dempster-fusion.md) · 2026-06-04 · reverify: `grep -n "fn dempster_corroborate_confidence" crates/specforge/src/ir/fusion.rs`
- "how was the eval gold checked for idiosyncrasy" -> [eval-gold-interannotator-kappa](docs/knowledge/eval-gold-interannotator-kappa.md) · 2026-06-05 · reverify: `python3 -c "see EVAL-GOLD-INTERANNOTATOR-AGREEMENT.md Method — re-run a blind second annotation and recompute`
- "is SpecForge specification mining" -> [spec-mining-framing](docs/knowledge/spec-mining-framing.md) · 2026-06-04 · reverify: `grep -rn "forward specification mining" README.md docs/book/src/architecture-rationale.md`
- "is qwen2.5:14b-instruct good enough for NLI" -> [local-llm-for-text-reasoning](docs/knowledge/local-llm-for-text-reasoning.md) · 2026-06-05 · reverify: `ollama list  # qwen2.5:14b-instruct (text) + qwen2.5vl:7b (vision); re-run the NLI/kappa probes`
- "is the LLM/VLM provider missing or not wired up" -> [llm-vlm-provider-default](docs/knowledge/llm-vlm-provider-default.md) · 2026-06-01 · reverify: `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- "is the eval gold / answer key trustworthy or reliable" -> [eval-gold-interannotator-kappa](docs/knowledge/eval-gold-interannotator-kappa.md) · 2026-06-05 · reverify: `python3 -c "see EVAL-GOLD-INTERANNOTATOR-AGREEMENT.md Method — re-run a blind second annotation and recompute`
- "is the eval-extraction temporal precision 0.6 a real defect" -> [temporal-eval-residual-fps-are-stale](docs/knowledge/temporal-eval-residual-fps-are-stale.md) · 2026-06-02 · reverify: `grep -n " when " crates/specforge/src/ir/evidence.rs`
- "is there a PSL or SVA export of temporal rules" -> [temporal-rule-ltl-rendering](docs/knowledge/temporal-rule-ltl-rendering.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "is there a lowering-completeness check for temporal rules" -> [isf-temporal-lowering-no-silent-drop](docs/knowledge/isf-temporal-lowering-no-silent-drop.md) · 2026-06-04 · reverify: `grep -n "temporal_residuals" crates/specforge/src/ir/isf_ir.rs crates/specforge/src/ir/adapters.rs`
- "qwen2.5vl vs qwen3-vl which model" -> [llm-vlm-provider-default](docs/knowledge/llm-vlm-provider-default.md) · 2026-06-01 · reverify: `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- "should I build an isf lowering-completeness verifier for temporal rules" -> [isf-temporal-lowering-no-silent-drop](docs/knowledge/isf-temporal-lowering-no-silent-drop.md) · 2026-06-04 · reverify: `grep -n "temporal_residuals" crates/specforge/src/ir/isf_ir.rs crates/specforge/src/ir/adapters.rs`
- "should I fix the PSEL valid when PSEL asserted temporal rule" -> [temporal-eval-residual-fps-are-stale](docs/knowledge/temporal-eval-residual-fps-are-stale.md) · 2026-06-02 · reverify: `grep -n " when " crates/specforge/src/ir/evidence.rs`
- "temporal rule eval false positives root cause" -> [temporal-eval-residual-fps-are-stale](docs/knowledge/temporal-eval-residual-fps-are-stale.md) · 2026-06-02 · reverify: `grep -n " when " crates/specforge/src/ir/evidence.rs`
- "temporal rule predicate atom vocabulary" -> [temporal-rule-ltl-rendering](docs/knowledge/temporal-rule-ltl-rendering.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "torch MPS float64 error during ingest" -> [docling-device-cpu](docs/knowledge/docling-device-cpu.md) · 2026-06-01 · reverify: `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- "was the (contract ... eventually ...) ISF clause removed" -> [fsmgen-temporal-isf-form](docs/knowledge/fsmgen-temporal-isf-form.md) · 2026-06-04 · reverify: `grep -n "assert (monitor (within" crates/specforge/src/ir/isf_ir.rs`
- "what ISF form does SpecForge use for a bounded-eventually contract" -> [fsmgen-temporal-isf-form](docs/knowledge/fsmgen-temporal-isf-form.md) · 2026-06-04 · reverify: `grep -n "assert (monitor (within" crates/specforge/src/ir/isf_ir.rs`
- "what does DOCLING_DEVICE do" -> [docling-device-cpu](docs/knowledge/docling-device-cpu.md) · 2026-06-01 · reverify: `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- "what does SpecForge defer from conformal prediction NLI Dempster Snorkel NoRBERT" -> [adopt-defer-ledger](docs/knowledge/adopt-defer-ledger.md) · 2026-06-04 · reverify: `ls docs/research/grounding/adopt-defer-ledger.md`
- "what does SpecForge take from Docling OpenIE LayoutLM Chao Chow LLVM MLIR GoldMine Texada Pnueli" -> [adopt-defer-ledger](docs/knowledge/adopt-defer-ledger.md) · 2026-06-04 · reverify: `ls docs/research/grounding/adopt-defer-ledger.md`
- "what does SpecForge take from a grounded author" -> [adopt-defer-ledger](docs/knowledge/adopt-defer-ledger.md) · 2026-06-04 · reverify: `ls docs/research/grounding/adopt-defer-ledger.md`
- "what does SpecForge take from the spec-mining literature and what does it leave out" -> [spec-mining-framing](docs/knowledge/spec-mining-framing.md) · 2026-06-04 · reverify: `grep -rn "forward specification mining" README.md docs/book/src/architecture-rationale.md`
- "what fsmgen pin does SpecForge target for temporal properties" -> [fsmgen-temporal-isf-form](docs/knowledge/fsmgen-temporal-isf-form.md) · 2026-06-04 · reverify: `grep -n "assert (monitor (within" crates/specforge/src/ir/isf_ir.rs`
- "what happens when the NLI provider is down" -> [nli-entailment-verifier](docs/knowledge/nli-entailment-verifier.md) · 2026-06-05 · reverify: `grep -n "fn verify_entailment" crates/specforge/src/ir/nli_verify.rs`
- "what is SpecForge doing in academic or research terms" -> [spec-mining-framing](docs/knowledge/spec-mining-framing.md) · 2026-06-04 · reverify: `grep -rn "forward specification mining" README.md docs/book/src/architecture-rationale.md`
- "what is a contested prior" -> [contested-priors](docs/knowledge/contested-priors.md) · 2026-06-04 · reverify: `grep -n "fn contested_priors" crates/specforge/src/ir/prior_memory.rs`
- "what is the Dempster combiner in fusion" -> [dempster-fusion](docs/knowledge/dempster-fusion.md) · 2026-06-04 · reverify: `grep -n "fn dempster_corroborate_confidence" crates/specforge/src/ir/fusion.rs`
- "what is the FSMGen issue bundle protocol" -> [fsmgen-feedback-channel](docs/knowledge/fsmgen-feedback-channel.md) · 2026-06-04 · reverify: `ls docs/FSMGEN_FEEDBACK.md`
- "what is the LTL form of a temporal_rule" -> [temporal-rule-ltl-rendering](docs/knowledge/temporal-rule-ltl-rendering.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "what is the NLI entailment verifier" -> [nli-entailment-verifier](docs/knowledge/nli-entailment-verifier.md) · 2026-06-05 · reverify: `grep -n "fn verify_entailment" crates/specforge/src/ir/nli_verify.rs`
- "what is the ambiguous_statements metric in validate" -> [ambiguity-weak-phrase-detector](docs/knowledge/ambiguity-weak-phrase-detector.md) · 2026-06-04 · reverify: `grep -n "fn weak_phrase_findings" crates/specforge/src/ir/ambiguity.rs`
- "what is the inter-annotator agreement of the eval gold" -> [eval-gold-interannotator-kappa](docs/knowledge/eval-gold-interannotator-kappa.md) · 2026-06-05 · reverify: `python3 -c "see EVAL-GOLD-INTERANNOTATOR-AGREEMENT.md Method — re-run a blind second annotation and recompute`
- "what model do converge / enrich / nlp-enrich use by default" -> [llm-vlm-provider-default](docs/knowledge/llm-vlm-provider-default.md) · 2026-06-01 · reverify: `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- "what model does the NLI verifier use" -> [nli-entailment-verifier](docs/knowledge/nli-entailment-verifier.md) · 2026-06-05 · reverify: `grep -n "fn verify_entailment" crates/specforge/src/ir/nli_verify.rs`
- "what parts of the literature are deferred or flagged as future work" -> [adopt-defer-ledger](docs/knowledge/adopt-defer-ledger.md) · 2026-06-04 · reverify: `ls docs/research/grounding/adopt-defer-ledger.md`
- "what research did SpecForge leave out and why" -> [adopt-defer-ledger](docs/knowledge/adopt-defer-ledger.md) · 2026-06-04 · reverify: `ls docs/research/grounding/adopt-defer-ledger.md`
- "what temporal logic backs temporal_rules" -> [temporal-logic-choice](docs/knowledge/temporal-logic-choice.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "where are cross-document prior contradictions surfaced" -> [contested-priors](docs/knowledge/contested-priors.md) · 2026-06-04 · reverify: `grep -n "fn contested_priors" crates/specforge/src/ir/prior_memory.rs`
- "where did SpecForge suggest LTL/MTL support in ISF" -> [fsmgen-feedback-channel](docs/knowledge/fsmgen-feedback-channel.md) · 2026-06-04 · reverify: `ls docs/FSMGEN_FEEDBACK.md`
- "where do I log feedback or a suggestion to FSMGen" -> [fsmgen-feedback-channel](docs/knowledge/fsmgen-feedback-channel.md) · 2026-06-04 · reverify: `ls docs/FSMGEN_FEEDBACK.md`
- "where does .isf record dropped temporal obligations" -> [isf-temporal-lowering-no-silent-drop](docs/knowledge/isf-temporal-lowering-no-silent-drop.md) · 2026-06-04 · reverify: `grep -n "temporal_residuals" crates/specforge/src/ir/isf_ir.rs crates/specforge/src/ir/adapters.rs`
- "where is the LTL renderer for temporal rules" -> [temporal-rule-ltl-rendering](docs/knowledge/temporal-rule-ltl-rendering.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "where is the SpecForge FSMGen feedback or handoff channel" -> [fsmgen-feedback-channel](docs/knowledge/fsmgen-feedback-channel.md) · 2026-06-04 · reverify: `ls docs/FSMGEN_FEEDBACK.md`
- "where is the per-author adopt-vs-defer provenance" -> [adopt-defer-ledger](docs/knowledge/adopt-defer-ledger.md) · 2026-06-04 · reverify: `ls docs/research/grounding/adopt-defer-ledger.md`
- "where is the weak-phrase / NASA ARM ambiguity detector" -> [ambiguity-weak-phrase-detector](docs/knowledge/ambiguity-weak-phrase-detector.md) · 2026-06-04 · reverify: `grep -n "fn weak_phrase_findings" crates/specforge/src/ir/ambiguity.rs`
- "which LLM or VLM does SpecForge use" -> [llm-vlm-provider-default](docs/knowledge/llm-vlm-provider-default.md) · 2026-06-01 · reverify: `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- "which local model should SpecForge use for NLI or entailment verification" -> [local-llm-for-text-reasoning](docs/knowledge/local-llm-for-text-reasoning.md) · 2026-06-05 · reverify: `ollama list  # qwen2.5:14b-instruct (text) + qwen2.5vl:7b (vision); re-run the NLI/kappa probes`
- "which local models are pulled and what are they for" -> [local-llm-for-text-reasoning](docs/knowledge/local-llm-for-text-reasoning.md) · 2026-06-05 · reverify: `ollama list  # qwen2.5:14b-instruct (text) + qwen2.5vl:7b (vision); re-run the NLI/kappa probes`
- "why are modal verbs must shall should may not flagged as ambiguous" -> [ambiguity-weak-phrase-detector](docs/knowledge/ambiguity-weak-phrase-detector.md) · 2026-06-04 · reverify: `grep -n "fn weak_phrase_findings" crates/specforge/src/ir/ambiguity.rs`
- "why are stability obligations residuals" -> [stable-obligation-phase-scoped-residual](docs/knowledge/stable-obligation-phase-scoped-residual.md) · 2026-06-04 · reverify: `grep -n "bare stability across tick phases" crates/specforge/src/ir/contract.rs`
- "why does Docling re-ingest fail on Apple Silicon" -> [docling-device-cpu](docs/knowledge/docling-device-cpu.md) · 2026-06-01 · reverify: `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- "why doesn't SpecForge emit (contract eventually) anymore" -> [fsmgen-temporal-isf-form](docs/knowledge/fsmgen-temporal-isf-form.md) · 2026-06-04 · reverify: `grep -n "assert (monitor (within" crates/specforge/src/ir/isf_ir.rs`
- "why doesn't SpecForge lower stability obligations to (assert (stable sig))" -> [stable-obligation-phase-scoped-residual](docs/knowledge/stable-obligation-phase-scoped-residual.md) · 2026-06-04 · reverify: `grep -n "bare stability across tick phases" crates/specforge/src/ir/contract.rs`
- "why doesn't SpecForge use TLA+" -> [temporal-logic-choice](docs/knowledge/temporal-logic-choice.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "why doesn't fusion use the minimum confidence" -> [dempster-fusion](docs/knowledge/dempster-fusion.md) · 2026-06-04 · reverify: `grep -n "fn dempster_corroborate_confidence" crates/specforge/src/ir/fusion.rs`
- "why doesn't the temporal_rule eval reach precision 1.0" -> [temporal-eval-residual-fps-are-stale](docs/knowledge/temporal-eval-residual-fps-are-stale.md) · 2026-06-02 · reverify: `grep -n " when " crates/specforge/src/ir/evidence.rs`
- "why is SpecForge called forward specification mining" -> [spec-mining-framing](docs/knowledge/spec-mining-framing.md) · 2026-06-04 · reverify: `grep -rn "forward specification mining" README.md docs/book/src/architecture-rationale.md`
- "why is the NLI framing better than free-form labeling" -> [local-llm-for-text-reasoning](docs/knowledge/local-llm-for-text-reasoning.md) · 2026-06-05 · reverify: `ollama list  # qwen2.5:14b-instruct (text) + qwen2.5vl:7b (vision); re-run the NLI/kappa probes`
- "why not CTL for temporal behavior" -> [temporal-logic-choice](docs/knowledge/temporal-logic-choice.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`

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

### contested-priors
_SpecForge detects contested priors (same key, conflicting values across docs) — read-only_

- **answers:** how does SpecForge detect contradicting or conflicting priors | does SpecForge revise or decay priors | what is a contested prior | where are cross-document prior contradictions surfaced | does corpus prior memory only accrete
- **date:** 2026-06-04 · **status:** current
- **evidence:** `crates/specforge/src/ir/prior_memory.rs; crates/specforge/src/commands/learn_priors.rs`
- **reverify:** `grep -n "fn contested_priors" crates/specforge/src/ir/prior_memory.rs`
- **source:** [`docs/knowledge/contested-priors.md`](docs/knowledge/contested-priors.md)

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
- **reverify:** `python3 -c "see EVAL-GOLD-INTERANNOTATOR-AGREEMENT.md Method — re-run a blind second annotation and recompute`
- **source:** [`docs/knowledge/eval-gold-interannotator-kappa.md`](docs/knowledge/eval-gold-interannotator-kappa.md)

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
