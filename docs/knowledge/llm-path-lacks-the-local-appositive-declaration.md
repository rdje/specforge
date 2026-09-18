---
id: llm-path-lacks-the-local-appositive-declaration
title: A same-clause appositive is a local declaration — on the deterministic path only, so the LLM path re-refuses the very fact ADR 0037's remedy recovers
answers:
  - "why does the LLM constraint path refuse APB PSEL when the deterministic path recovers it (the same-clause appositive local declaration is implemented at ONE call site, is_same_clause_signal_appositive inside parse_inference_antecedent_signal_constraint at ir/evidence.rs:10107, and the LLM path types against the global catalog alone)"
  - "may a resolver alias PSEL to PSELx or PSELX (no — ADR 0037 §1 says suffix and substring carry no semantic authority and §3 permits only exact and unique case-folded resolution; the recovery route is the appositive local declaration, not an alias)"
  - "what would a one-character-truncation resolver cost (372 one-character stems exist across five persisted documents that are not themselves declared, 4 of them ambiguous in AXI alone — 372 admissions to recover one real name)"
  - "how does SpecForge tell a parameterised declaration template from a truncation (it does not, and deliberately: ADR 0037 forbids reading the suffix at all, so PSELx and ATBYTES are both simply declared names that a shorter proposal does not match)"
  - "which resolution modes does ADR 0037 authorize for a model proposal (exactly two: an exact match, and a case-folded match that yields exactly one current-document identity, with an exact match winning and collisions failing closed)"
  - "is a persisted llm_sigcon_ record with a subject the catalog refuses necessarily wrong (no — APB llm_sigcon_0000 PSEL|must_be_asserted is the canonical fact SPEC-TO-INTENT-ALIGNMENT.7a exists to recover, refused by an asymmetry between the two paths rather than by ADR 0037's principle)"
date: 2026-09-18
status: current
tags: [extraction-quality-gauge, entity-typing, adr-0037, llm-primary, spec-to-intent-alignment, apb, adjudication]
evidence: docs/decisions/0037-identifiers-are-opaque-and-one-way-grounded.md (§1, §3, §7); crates/specforge/src/ir/evidence.rs:10107 (is_same_clause_signal_appositive, single call site); crates/specforge/src/ir/evidence.rs:31806 (exact_witness_establishes_local_psel_without_aliasing_pselx); crates/specforge/src/commands/extract_constraints_llm.rs (the LLM path's catalog); docs/tasks/extraction-quality-gauge/llm-path-family.md (.3j.2.b)
reverify: "grep -rn is_same_clause_signal_appositive crates/ — expect exactly one call site, inside parse_inference_antecedent_signal_constraint; and cargo test -p specforge-core --lib exact_witness_establishes_local_psel_without_aliasing_pselx"
---

Two paths mint signal constraints from the same document, and only one of them knows that a document can
declare an identifier **in the sentence that uses it**.

## The sentence, and what each path does with it

APB writes: *"The select signal, PSEL, is asserted, which means that PADDR, PWRITE and PWDATA must be
valid."* Its declaration table declares `PSELx`, so the catalog holds `PSELX` and `PSELXCHK` — never the
bare `PSEL`.

**The deterministic path recovers `PSEL|must_be_asserted`.** `SPEC-TO-INTENT-ALIGNMENT.7a` ruled that the
same-clause appositive *"The select signal, PSEL"* **is a local declaration of the opaque identifier
`PSEL`**, distinct from `PSELX`, and `is_same_clause_signal_appositive` implements it.

**The LLM path proposes the same fact and it is dropped.** `promote_constraints` types a subject through
`resolve_unique_document_identifier` against `declared_signal_catalog(ir)` — the global catalog — so
`PSEL` is `EntityType::Unknown` and the whole proposal goes. The persisted `llm_sigcon_0000` carries it
only because that record predates catalog grounding
(`[[persisted-llm-constraint-corpus-predates-catalog-grounding]]`).

**The asymmetry is one call site wide.** `is_same_clause_signal_appositive` is used once, inside
`parse_inference_antecedent_signal_constraint`, which is reachable only from the deterministic extractor.
Nothing exposes it to the catalog the LLM path types against.

## Why the obvious fix is forbidden

Aliasing `PSEL` to `PSELX` is not available, and this is decided rather than open. **ADR 0037 §1**: an
identifier's *"length, case, prefix, suffix, substring, resemblance to a conventional name"* carries no
semantic authority. **§3**: a model proposal resolves *exactly*, or by a case fold that yields exactly one
identity — an exact match wins and collisions fail closed. Those are the only two modes.

The measurement agrees with the doctrine rather than merely deferring to it. The one rule that would
separate `PSEL` → `PSELX` from `ATB` → `ATBYTES` — resolve to the unique declared name you extend by
**exactly one character** — has an admission surface of the whole catalog minus one character: **372
one-character stems across five persisted documents** are not themselves declared, and four are ambiguous
in AXI alone (`ARCTLCHK` → `ARCTLCHK0..3`, `A` → `AC`/`AR`/`AW`). That is 372 admissions to recover one
name.

## The lesson that outlives the instance

**A refusal is not evidence of a wrong record.** `PSEL` was refused, the record was right, and the two
facts are compatible because the refusal came from an *incomplete notion of "declared"*, not from a wrong
subject. When a census reports a subject the catalog rejects, read the record against its source before
classifying it — and check whether another path in the same codebase already accepts it.

Links: [[inference-antecedent-state-loss]],
[[persisted-llm-constraint-corpus-predates-catalog-grounding]],
[[indexed-signal-family-canonicalization]]
