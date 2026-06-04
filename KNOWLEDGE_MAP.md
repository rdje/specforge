# Knowledge Map

> **AUTO-GENERATED — DO NOT EDIT.** Regenerate with `knowledge-map/scripts/gen_knowledge_map.sh`.
> Source of truth = YAML front-matter in: `docs/knowledge docs/decisions`. Edit the fact files, never this map.
> A fact is any `.md` whose front-matter has a non-empty `answers:` list.
> **4** facts · **20** question keys.

## Questions → fact

- "Cannot convert a MPS Tensor to float64" -> [docling-device-cpu](docs/knowledge/docling-device-cpu.md) · 2026-06-01 · reverify: `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- "are the degenerate PSEL-header or WIDTH-subject temporal rules a live bug" -> [temporal-eval-residual-fps-are-stale](docs/knowledge/temporal-eval-residual-fps-are-stale.md) · 2026-06-02 · reverify: `grep -n " when " crates/specforge/src/ir/evidence.rs`
- "default model for the ollama provider" -> [llm-vlm-provider-default](docs/knowledge/llm-vlm-provider-default.md) · 2026-06-01 · reverify: `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- "how are temporal rules expressed as LTL or MTL" -> [temporal-rule-ltl-rendering](docs/knowledge/temporal-rule-ltl-rendering.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "how do I run a Docling ingest or re-ingest on this machine" -> [docling-device-cpu](docs/knowledge/docling-device-cpu.md) · 2026-06-01 · reverify: `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- "is the LLM/VLM provider missing or not wired up" -> [llm-vlm-provider-default](docs/knowledge/llm-vlm-provider-default.md) · 2026-06-01 · reverify: `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- "is the eval-extraction temporal precision 0.6 a real defect" -> [temporal-eval-residual-fps-are-stale](docs/knowledge/temporal-eval-residual-fps-are-stale.md) · 2026-06-02 · reverify: `grep -n " when " crates/specforge/src/ir/evidence.rs`
- "is there a PSL or SVA export of temporal rules" -> [temporal-rule-ltl-rendering](docs/knowledge/temporal-rule-ltl-rendering.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "qwen2.5vl vs qwen3-vl which model" -> [llm-vlm-provider-default](docs/knowledge/llm-vlm-provider-default.md) · 2026-06-01 · reverify: `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- "should I fix the PSEL valid when PSEL asserted temporal rule" -> [temporal-eval-residual-fps-are-stale](docs/knowledge/temporal-eval-residual-fps-are-stale.md) · 2026-06-02 · reverify: `grep -n " when " crates/specforge/src/ir/evidence.rs`
- "temporal rule eval false positives root cause" -> [temporal-eval-residual-fps-are-stale](docs/knowledge/temporal-eval-residual-fps-are-stale.md) · 2026-06-02 · reverify: `grep -n " when " crates/specforge/src/ir/evidence.rs`
- "temporal rule predicate atom vocabulary" -> [temporal-rule-ltl-rendering](docs/knowledge/temporal-rule-ltl-rendering.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "torch MPS float64 error during ingest" -> [docling-device-cpu](docs/knowledge/docling-device-cpu.md) · 2026-06-01 · reverify: `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- "what does DOCLING_DEVICE do" -> [docling-device-cpu](docs/knowledge/docling-device-cpu.md) · 2026-06-01 · reverify: `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- "what is the LTL form of a temporal_rule" -> [temporal-rule-ltl-rendering](docs/knowledge/temporal-rule-ltl-rendering.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "what model do converge / enrich / nlp-enrich use by default" -> [llm-vlm-provider-default](docs/knowledge/llm-vlm-provider-default.md) · 2026-06-01 · reverify: `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- "where is the LTL renderer for temporal rules" -> [temporal-rule-ltl-rendering](docs/knowledge/temporal-rule-ltl-rendering.md) · 2026-06-04 · reverify: `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- "which LLM or VLM does SpecForge use" -> [llm-vlm-provider-default](docs/knowledge/llm-vlm-provider-default.md) · 2026-06-01 · reverify: `grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs`
- "why does Docling re-ingest fail on Apple Silicon" -> [docling-device-cpu](docs/knowledge/docling-device-cpu.md) · 2026-06-01 · reverify: `grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs`
- "why doesn't the temporal_rule eval reach precision 1.0" -> [temporal-eval-residual-fps-are-stale](docs/knowledge/temporal-eval-residual-fps-are-stale.md) · 2026-06-02 · reverify: `grep -n " when " crates/specforge/src/ir/evidence.rs`

## Facts (by id)

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

### temporal-eval-residual-fps-are-stale
_The temporal-rule eval's residual false positives are a stale artifact, not a live bug_

- **answers:** why doesn't the temporal_rule eval reach precision 1.0 | are the degenerate PSEL-header or WIDTH-subject temporal rules a live bug | should I fix the PSEL valid when PSEL asserted temporal rule | temporal rule eval false positives root cause | is the eval-extraction temporal precision 0.6 a real defect
- **date:** 2026-06-02 · **status:** current
- **evidence:** `crates/specforge/src/ir/evidence.rs:2838; crates/specforge/src/ir/evidence.rs:5756; docs/tasks/TEMPORAL-RULE-EVAL.md`
- **reverify:** `grep -n " when " crates/specforge/src/ir/evidence.rs`
- **source:** [`docs/knowledge/temporal-eval-residual-fps-are-stale.md`](docs/knowledge/temporal-eval-residual-fps-are-stale.md)

### temporal-rule-ltl-rendering
_Temporal rules render to standard LTL/MTL via ir/temporal_ltl.rs_

- **answers:** how are temporal rules expressed as LTL or MTL | where is the LTL renderer for temporal rules | what is the LTL form of a temporal_rule | is there a PSL or SVA export of temporal rules | temporal rule predicate atom vocabulary
- **date:** 2026-06-04 · **status:** current
- **evidence:** `crates/specforge/src/ir/temporal_ltl.rs; docs/book/src/domain/temporal-semantics.md`
- **reverify:** `grep -n "fn temporal_rule_to_ltl" crates/specforge/src/ir/temporal_ltl.rs`
- **source:** [`docs/knowledge/temporal-rule-ltl-rendering.md`](docs/knowledge/temporal-rule-ltl-rendering.md)
