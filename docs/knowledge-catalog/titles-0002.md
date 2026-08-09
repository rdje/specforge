# Knowledge fact-card titles — part 0002

> **AUTO-GENERATED — DO NOT EDIT.** Canonical facts remain in `docs/knowledge/`.
> Return to the [bounded fact-card landing](../knowledge/INDEX.md).

| Fact | Established | Status | Title |
| --- | --- | --- | --- |
| [eval-gold-interannotator-kappa](../knowledge/eval-gold-interannotator-kappa.md) | 2026-06-05 | `current` | Eval gold is reliable — Cohen's kappa 0.90 (almost-perfect) on the constraint task |
| [eval-scores-persisted-evidence](../knowledge/eval-scores-persisted-evidence.md) | 2026-06-07 | `current` | eval-extraction scores the PERSISTED evidence_ir.json — rebuild before trusting a baseline (it can be stale) |
| [evidence-build-nondeterminism](../knowledge/evidence-build-nondeterminism.md) | 2026-06-09 | `current` | EvidenceIR build determinism — two HashSet-iteration leaks (relations + enum name) found & FIXED (EVIDENCE-… |
| [evidence-signal-declaration-utf8-boundary-panic](../knowledge/evidence-signal-declaration-utf8-boundary-panic.md) | 2026-08-09 | `current` | Evidence signal-declaration catalogs panic when a match follows a multi-byte bullet |
| [extraction-audit-vlm](../knowledge/extraction-audit-vlm.md) | 2026-06-08 | `current` | audit-extraction — VLM proposer/verifier precision estimate over the broadened table-driven extraction |
| [extraction-quality-gauge-standing](../knowledge/extraction-quality-gauge-standing.md) | 2026-06-10 | `current` | The NLI extraction-quality gauge is persisted on EvidenceIR and re-measured by converge |
| [extractor-path-architecture](../knowledge/extractor-path-architecture.md) | 2026-06-09 | `current` | The EvidenceIR extractor path is a flat bank of ~60 free functions wired in one build() — coherent IR targe… |
| [fact-card-catalog](../knowledge/fact-card-catalog.md) | 2026-08-08 | `current` | Fact cards have a bounded derived human catalog distinct from question retrieval |
| [fsmgen-feedback-channel](../knowledge/fsmgen-feedback-channel.md) | 2026-06-04 | `current` | FSMGen feedback uses a bounded current channel and exact correspondence history |
| [fsmgen-ignores-signal-direction](../knowledge/fsmgen-ignores-signal-direction.md) | 2026-06-17 | `current` | FSMGen --strict --check does NOT validate or use a signal's declared direction (input/output) — driving a d… |
| [fsmgen-temporal-isf-form](../knowledge/fsmgen-temporal-isf-form.md) | 2026-06-04 | `current` | SpecForge emits bounded-eventually as (assert (monitor (within s N))) into .isf (fsmgen pin 43b29f5c) |
| [full-page-capture-gap](../knowledge/full-page-capture-gap.md) | 2026-06-14 | `current` | SpecForge captures a page's intent-bearing content; the residual outside Docling regions is decoration/furnit… |
| [generic-enum-conflation](../knowledge/generic-enum-conflation.md) | 2026-06-24 | `current` | The `.isf` generic-`TABLE` (and `FIGURE`/`DATA`/…) mega-enum is an EXTRACTION-born conflation — `derive_e… |
| [header-trapped-signal-table-recovery](../knowledge/header-trapped-signal-table-recovery.md) | 2026-06-11 | `current` | Header-trapped SIGNAL tables — shared trapped-row rule, continuation-kind inheritance, inventory-gated gap-… |
| [indexed-signal-family-canonicalization](../knowledge/indexed-signal-family-canonicalization.md) | 2026-06-06 | `current` | Un-indexed prose signal refs resolve to the declared indexed family member (PSEL → PSELx) |
| [ingest-adaptive-batch-sizing](../knowledge/ingest-adaptive-batch-sizing.md) | 2026-06-14 | `current` | ingest sizes each page-range batch to the host's total physical RAM so a small machine completes |
| [ingest-disk-preflight](../knowledge/ingest-disk-preflight.md) | 2026-06-14 | `current` | ingest disk pre-flight refuses before launching when free disk is below a source-size-scaled requirement |
| [ingest-ram-guard](../knowledge/ingest-ram-guard.md) | 2026-06-14 | `current` | ingest has a built-in autonomous RAM guard that aborts cleanly before the host crosses a danger ceiling |
| [isf-enum-value-literal-emit-gate](../knowledge/isf-enum-value-literal-emit-gate.md) | 2026-06-23 | `current` | The `.isf` emitter drops an enum whose member value is a bare binary-looking token (only `0`/`1` digits, leng… |
| [isf-fsm-via-switch-select](../knowledge/isf-fsm-via-switch-select.md) | 2026-06-07 | `current` | ISF CAN describe a state machine — proven idiom is storage-var + switch + select + rule-trigger (FSMGen low… |
| [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md) | 2026-06-18 | `current` | The emitted `.isf` interface now lowers grounded actor-relative signal DIRECTION from the protocol's INITIATO… |
| [isf-lowering-fidelity-gauge](../knowledge/isf-lowering-fidelity-gauge.md) | 2026-06-17 | `current` | Which IntentIR surfaces reach the .isf vs are silently dropped (KG-ISF-COMPLETENESS.2 measurement) — the ba… |
| [isf-module-name-hdl-sanitization](../knowledge/isf-module-name-hdl-sanitization.md) | 2026-06-21 | `current` | The emitted `.isf` module name (and every internal `.isf` identifier) is HDL-sanitized by `sanitize_isf_name`… |
| [isf-temporal-lowering-no-silent-drop](../knowledge/isf-temporal-lowering-no-silent-drop.md) | 2026-06-04 | `current` | temporal_rules are never silently dropped in the IntentIR->.isf lowering (already guaranteed) |
| [isf-unconditional-rule-overlap-conflict](../knowledge/isf-unconditional-rule-overlap-conflict.md) | 2026-06-23 | `current` | The `.isf` emitter drops a rule that conflicts with an UNCONDITIONAL driver on the same signal (an empty-guar… |
| [isf-unrenderable-rule-value-residual](../knowledge/isf-unrenderable-rule-value-residual.md) | 2026-06-23 | `current` | The `.isf` emitter drops a rule whose drive VALUE is not a renderable ISF value expression (free prose, not a… |
| [isf-value-width-operand-contract](../knowledge/isf-value-width-operand-contract.md) | 2026-06-21 | `current` | FSMGen strict rejects an ISF value literal whose notation width ≠ the target signal width (OperandContract,… |
| [knowledge-map-architecture-location](../knowledge/knowledge-map-architecture-location.md) | 2026-08-08 | `current` | Knowledge-map architecture lives inside the knowledge-map bundle |
| [knowledge-map-shard-contract](../knowledge/knowledge-map-shard-contract.md) | 2026-08-08 | `current` | The Knowledge Map shard migration is locked by a bounded executable contract |
| [live-document-containment-fixture-gate](../knowledge/live-document-containment-fixture-gate.md) | 2026-08-08 | `current` | Live-document lifecycle and control-plane proofs run on the repository volume |
| [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md) | 2026-08-08 | `current` | Parent Git index defines SpecForge live-Markdown coverage |
| [live-document-derived-state-contract-gap](../knowledge/live-document-derived-state-contract-gap.md) | 2026-08-09 | `current` | Derived-state field and copy plane is independently closed |
| [llm-primary-condition-subject-gate](../knowledge/llm-primary-condition-subject-gate.md) | 2026-06-10 | `current` | Condition-only-subject gate — APB/AHB/AXI constraint task all P=R=F1=1.000 (FPs 3→0) |
| [llm-primary-constraint-dedup](../knowledge/llm-primary-constraint-dedup.md) | 2026-06-10 | `current` | LLM-primary constraint dedup — provenance-merging, condition-aware (AXI 54→50 live) |
| [llm-primary-must-be-value-recall](../knowledge/llm-primary-must-be-value-recall.md) | 2026-06-10 | `current` | LLM-primary must_be_value recall gap CLOSED — 10/16 → 16/16 gold facts (APB/AHB/AXI) |
| [llm-primary-permissive-frame-gate](../knowledge/llm-primary-permissive-frame-gate.md) | 2026-06-10 | `current` | Permissive-only frame gate is SUBJECT-SENTENCE-scoped — block-scoped modals over-kill |
| [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md) | 2026-06-10 | `current` | converge --promote-constraints-llm replaces the Pattern constraint surface post-stability |
| [llm-vlm-provider-default](../knowledge/llm-vlm-provider-default.md) | 2026-06-01 | `current` | SpecForge ships a production Ollama+Qwen2.5VL provider (the default LLM/VLM) |
| [local-llm-for-text-reasoning](../knowledge/local-llm-for-text-reasoning.md) | 2026-06-05 | `current` | Text-reasoning gates (NLI, semantic checks) want a strong TEXT LLM, not a VLM; qwen2.5:14b-instruct is viable… |
| [mdbook-current-truth-drift-lock](../knowledge/mdbook-current-truth-drift-lock.md) | 2026-08-08 | `current` | Two mdBook current-state facts are mechanically locked to their code seams |
| [mdbook-doctest-gap](../knowledge/mdbook-doctest-gap.md) | 2026-08-08 | `current` | The live book classifies illustrative fences explicitly and passes mdBook doctests |
| [message-field-constraints-surface](../knowledge/message-field-constraints-surface.md) | 2026-06-10 | `current` | message_field_constraints — field-subject obligations are routed, not dropped (CHI TagOp/PBHA live) |
| [message-field-records-surface](../knowledge/message-field-records-surface.md) | 2026-06-10 | `current` | message_field_records — the typed home for packet/flit message fields (CHI 106, C2C ≤189, CCIX ~50) |
| [message-field-validate-integration](../knowledge/message-field-validate-integration.md) | 2026-06-11 | `current` | validate reports the message-field surfaces (5 metrics + inventory finding); class census and completeness ga… |
| [model-misspelled-subject-snap](../knowledge/model-misspelled-subject-snap.md) | 2026-06-10 | `current` | A subject absent from its own source sentence snaps to the sentence's declared token (edit distance 1, unambi… |
| [nli-entailment-verifier](../knowledge/nli-entailment-verifier.md) | 2026-06-05 | `current` | NLI entailment verifier — a semantic "does the source entail this claim?" grounding gate |
| [nli-gate-real-apb-validation](../knowledge/nli-gate-real-apb-validation.md) | 2026-06-05 | `current` | NLI gate validated on the real AMBA APB spec — it works, and it exposes constraint over-generation |
| [nli-intent-gate](../knowledge/nli-intent-gate.md) | 2026-06-05 | `current` | NLI intent gate — active demote-to-residual of un-entailed contracts (intent --nli-verify) |
| [nlp-coordination-already-handled](../knowledge/nlp-coordination-already-handled.md) | 2026-06-15 | `current` | Coordinated drive/read objects + relative-clause distribution are already implemented in the production hand… |
| [offset-suffixed-dword-relative-bit-cells](../knowledge/offset-suffixed-dword-relative-bit-cells.md) | 2026-06-11 | `current` | Offset-suffixed bit cells (31:28 +04) are dword-relative — capture the literal bit_range + byte_offset, nev… |
| [packet-field-table-declaration](../knowledge/packet-field-table-declaration.md) | 2026-06-10 | `current` | Packet/flit protocols declare message FIELDS in field-titled tables — the header vocabulary types the rows |
| [page-image-disk-bounding](../knowledge/page-image-disk-bounding.md) | 2026-06-14 | `current` | Per-page full-res PNGs are not read downstream — ingest skips persisting them for large PDFs |
| [pdf-encryption-and-read-access](../knowledge/pdf-encryption-and-read-access.md) | 2026-06-07 | `current` | 12/82 corpus PDFs are permission-encrypted (open w/ empty password); docling+pypdf read all 82; the Read tool… |
| [persisted-path-origin-and-rebase-contract](../knowledge/persisted-path-origin-and-rebase-contract.md) | 2026-08-09 | `current` | Persisted paths separate repository ownership from authorized external inputs |
| [prior-phrase-utf8-byte-as-char](../knowledge/prior-phrase-utf8-byte-as-char.md) | 2026-06-14 | `current` | prior-phrase normalization UTF-8 byte-as-char mangling — exponential OOM found & FIXED (PDF-VARIANT-DIGESTI… |
| [project-data-locality-enforcement](../knowledge/project-data-locality-enforcement.md) | 2026-08-08 | `current` | Project-owned temp, cache, dependency, and subprocess data resolves from the current repository |
