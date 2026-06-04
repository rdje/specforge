# Knowledge Map

> **AUTO-GENERATED — DO NOT EDIT.** Regenerate with `knowledge-map/scripts/gen_knowledge_map.sh`.
> Source of truth = YAML front-matter in: `docs/knowledge docs/decisions`. Edit the fact files, never this map.
> A fact is any `.md` whose front-matter has a non-empty `answers:` list.
> **8** facts · **41** question keys.

## Questions → fact

- "Cannot convert a MPS Tensor to float64" -> [docling-device-cpu](docs/knowledge/docling-device-cpu.md) · 2026-06-01 · reverify: `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- "are the degenerate PSEL-header or WIDTH-subject temporal rules a live bug" -> [temporal-eval-residual-fps-are-stale](docs/knowledge/temporal-eval-residual-fps-are-stale.md) · 2026-06-02 · reverify: `grep -n " when " crates/specforge/src/ir/evidence.rs`
- "default model for the ollama provider" -> [llm-vlm-provider-default](docs/knowledge/llm-vlm-provider-default.md) · 2026-06-01 · reverify: `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- "does SpecForge detect implementation-defined or TBD or and/or" -> [ambiguity-weak-phrase-detector](docs/knowledge/ambiguity-weak-phrase-detector.md) · 2026-06-04 · reverify: `grep -n "fn weak_phrase_findings" crates/specforge/src/ir/ambiguity.rs`
- "does SpecForge model-check temporal properties" -> [temporal-logic-choice](docs/knowledge/temporal-logic-choice.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "does SpecForge use LTL CTL or TLA+" -> [temporal-logic-choice](docs/knowledge/temporal-logic-choice.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "how are temporal rules expressed as LTL or MTL" -> [temporal-rule-ltl-rendering](docs/knowledge/temporal-rule-ltl-rendering.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "how do I run a Docling ingest or re-ingest on this machine" -> [docling-device-cpu](docs/knowledge/docling-device-cpu.md) · 2026-06-01 · reverify: `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- "how does SpecForge flag vague or ambiguous spec language" -> [ambiguity-weak-phrase-detector](docs/knowledge/ambiguity-weak-phrase-detector.md) · 2026-06-04 · reverify: `grep -n "fn weak_phrase_findings" crates/specforge/src/ir/ambiguity.rs`
- "how does SpecForge relate to GoldMine Texada Pnueli Ammons" -> [spec-mining-framing](docs/knowledge/spec-mining-framing.md) · 2026-06-04 · reverify: `grep -rn "forward specification mining" README.md docs/book/src/architecture-rationale.md`
- "is SpecForge specification mining" -> [spec-mining-framing](docs/knowledge/spec-mining-framing.md) · 2026-06-04 · reverify: `grep -rn "forward specification mining" README.md docs/book/src/architecture-rationale.md`
- "is the LLM/VLM provider missing or not wired up" -> [llm-vlm-provider-default](docs/knowledge/llm-vlm-provider-default.md) · 2026-06-01 · reverify: `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- "is the eval-extraction temporal precision 0.6 a real defect" -> [temporal-eval-residual-fps-are-stale](docs/knowledge/temporal-eval-residual-fps-are-stale.md) · 2026-06-02 · reverify: `grep -n " when " crates/specforge/src/ir/evidence.rs`
- "is there a PSL or SVA export of temporal rules" -> [temporal-rule-ltl-rendering](docs/knowledge/temporal-rule-ltl-rendering.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "qwen2.5vl vs qwen3-vl which model" -> [llm-vlm-provider-default](docs/knowledge/llm-vlm-provider-default.md) · 2026-06-01 · reverify: `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- "should I fix the PSEL valid when PSEL asserted temporal rule" -> [temporal-eval-residual-fps-are-stale](docs/knowledge/temporal-eval-residual-fps-are-stale.md) · 2026-06-02 · reverify: `grep -n " when " crates/specforge/src/ir/evidence.rs`
- "temporal rule eval false positives root cause" -> [temporal-eval-residual-fps-are-stale](docs/knowledge/temporal-eval-residual-fps-are-stale.md) · 2026-06-02 · reverify: `grep -n " when " crates/specforge/src/ir/evidence.rs`
- "temporal rule predicate atom vocabulary" -> [temporal-rule-ltl-rendering](docs/knowledge/temporal-rule-ltl-rendering.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "torch MPS float64 error during ingest" -> [docling-device-cpu](docs/knowledge/docling-device-cpu.md) · 2026-06-01 · reverify: `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- "what does DOCLING_DEVICE do" -> [docling-device-cpu](docs/knowledge/docling-device-cpu.md) · 2026-06-01 · reverify: `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- "what does SpecForge defer from conformal prediction NLI Dempster Snorkel NoRBERT" -> [adopt-defer-ledger](docs/knowledge/adopt-defer-ledger.md) · 2026-06-04 · reverify: `ls docs/research/grounding/adopt-defer-ledger.md`
- "what does SpecForge take from Docling OpenIE LayoutLM Chao Chow LLVM MLIR GoldMine Texada Pnueli" -> [adopt-defer-ledger](docs/knowledge/adopt-defer-ledger.md) · 2026-06-04 · reverify: `ls docs/research/grounding/adopt-defer-ledger.md`
- "what does SpecForge take from a grounded author" -> [adopt-defer-ledger](docs/knowledge/adopt-defer-ledger.md) · 2026-06-04 · reverify: `ls docs/research/grounding/adopt-defer-ledger.md`
- "what does SpecForge take from the spec-mining literature and what does it leave out" -> [spec-mining-framing](docs/knowledge/spec-mining-framing.md) · 2026-06-04 · reverify: `grep -rn "forward specification mining" README.md docs/book/src/architecture-rationale.md`
- "what is SpecForge doing in academic or research terms" -> [spec-mining-framing](docs/knowledge/spec-mining-framing.md) · 2026-06-04 · reverify: `grep -rn "forward specification mining" README.md docs/book/src/architecture-rationale.md`
- "what is the LTL form of a temporal_rule" -> [temporal-rule-ltl-rendering](docs/knowledge/temporal-rule-ltl-rendering.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "what is the ambiguous_statements metric in validate" -> [ambiguity-weak-phrase-detector](docs/knowledge/ambiguity-weak-phrase-detector.md) · 2026-06-04 · reverify: `grep -n "fn weak_phrase_findings" crates/specforge/src/ir/ambiguity.rs`
- "what model do converge / enrich / nlp-enrich use by default" -> [llm-vlm-provider-default](docs/knowledge/llm-vlm-provider-default.md) · 2026-06-01 · reverify: `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- "what parts of the literature are deferred or flagged as future work" -> [adopt-defer-ledger](docs/knowledge/adopt-defer-ledger.md) · 2026-06-04 · reverify: `ls docs/research/grounding/adopt-defer-ledger.md`
- "what research did SpecForge leave out and why" -> [adopt-defer-ledger](docs/knowledge/adopt-defer-ledger.md) · 2026-06-04 · reverify: `ls docs/research/grounding/adopt-defer-ledger.md`
- "what temporal logic backs temporal_rules" -> [temporal-logic-choice](docs/knowledge/temporal-logic-choice.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "where is the LTL renderer for temporal rules" -> [temporal-rule-ltl-rendering](docs/knowledge/temporal-rule-ltl-rendering.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "where is the per-author adopt-vs-defer provenance" -> [adopt-defer-ledger](docs/knowledge/adopt-defer-ledger.md) · 2026-06-04 · reverify: `ls docs/research/grounding/adopt-defer-ledger.md`
- "where is the weak-phrase / NASA ARM ambiguity detector" -> [ambiguity-weak-phrase-detector](docs/knowledge/ambiguity-weak-phrase-detector.md) · 2026-06-04 · reverify: `grep -n "fn weak_phrase_findings" crates/specforge/src/ir/ambiguity.rs`
- "which LLM or VLM does SpecForge use" -> [llm-vlm-provider-default](docs/knowledge/llm-vlm-provider-default.md) · 2026-06-01 · reverify: `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- "why are modal verbs must shall should may not flagged as ambiguous" -> [ambiguity-weak-phrase-detector](docs/knowledge/ambiguity-weak-phrase-detector.md) · 2026-06-04 · reverify: `grep -n "fn weak_phrase_findings" crates/specforge/src/ir/ambiguity.rs`
- "why does Docling re-ingest fail on Apple Silicon" -> [docling-device-cpu](docs/knowledge/docling-device-cpu.md) · 2026-06-01 · reverify: `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- "why doesn't SpecForge use TLA+" -> [temporal-logic-choice](docs/knowledge/temporal-logic-choice.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "why doesn't the temporal_rule eval reach precision 1.0" -> [temporal-eval-residual-fps-are-stale](docs/knowledge/temporal-eval-residual-fps-are-stale.md) · 2026-06-02 · reverify: `grep -n " when " crates/specforge/src/ir/evidence.rs`
- "why is SpecForge called forward specification mining" -> [spec-mining-framing](docs/knowledge/spec-mining-framing.md) · 2026-06-04 · reverify: `grep -rn "forward specification mining" README.md docs/book/src/architecture-rationale.md`
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

### docling-device-cpu
_Docling ingest must run on CPU on this stack (torch MPS lacks float64)_

- **answers:** why does Docling re-ingest fail on Apple Silicon | Cannot convert a MPS Tensor to float64 | how do I run a Docling ingest or re-ingest on this machine | what does DOCLING_DEVICE do | torch MPS float64 error during ingest
- **date:** 2026-06-01 · **status:** current
- **evidence:** `docs/decisions/0001-docling-device-cpu.md; crates/specforge/src/ir/source/docling_backend.rs:444`
- **reverify:** `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- **source:** [`docs/knowledge/docling-device-cpu.md`](docs/knowledge/docling-device-cpu.md)

### llm-vlm-provider-default
_SpecForge ships a production Ollama+Qwen2.5VL provider (the default LLM/VLM)_

- **answers:** which LLM or VLM does SpecForge use | is the LLM/VLM provider missing or not wired up | what model do converge / enrich / nlp-enrich use by default | qwen2.5vl vs qwen3-vl which model | default model for the ollama provider
- **date:** 2026-06-01 · **status:** current
- **evidence:** `docs/decisions/0002-llm-vlm-provider-default.md; crates/specforge/src/commands/llm_text.rs:35`
- **reverify:** `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- **source:** [`docs/knowledge/llm-vlm-provider-default.md`](docs/knowledge/llm-vlm-provider-default.md)

### spec-mining-framing
_SpecForge is forward specification mining (spec -> intent, not implementation -> spec)_

- **answers:** what is SpecForge doing in academic or research terms | is SpecForge specification mining | how does SpecForge relate to GoldMine Texada Pnueli Ammons | what does SpecForge take from the spec-mining literature and what does it leave out | why is SpecForge called forward specification mining
- **date:** 2026-06-04 · **status:** current
- **evidence:** `docs/research/grounding/adopt-defer-ledger.md; docs/book/src/architecture-rationale.md; README.md`
- **reverify:** `grep -rn "forward specification mining" README.md docs/book/src/architecture-rationale.md`
- **source:** [`docs/knowledge/spec-mining-framing.md`](docs/knowledge/spec-mining-framing.md)

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
