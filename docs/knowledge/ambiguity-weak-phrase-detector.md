---
id: ambiguity-weak-phrase-detector
title: validate flags vague spec prose via the weak-phrase detector (ir/ambiguity.rs)
answers:
  - "how does SpecForge flag vague or ambiguous spec language"
  - "what is the ambiguous_statements metric in validate"
  - "where is the weak-phrase / NASA ARM ambiguity detector"
  - "does SpecForge detect implementation-defined or TBD or and/or"
  - "why are modal verbs must shall should may not flagged as ambiguous"
date: 2026-06-04
tags: [validate, ambiguity, residual-honesty, requirements]
evidence: crates/specforge/src/ir/ambiguity.rs; docs/book/src/quality/validation.md
reverify: grep -n "fn weak_phrase_findings" crates/specforge/src/ir/ambiguity.rs
---

`crate::ir::ambiguity::weak_phrase_findings(&[ExtractedStatement])` flags statements whose
prose carries vagueness / under-specification markers — NASA ARM "weak phrases"
("as appropriate", "if necessary", "and/or", "to be determined"/"TBD", "but not limited to",
"as a minimum") plus the chip-spec idioms "implementation-defined" / "vendor-specific". The
lexicon is `WEAK_PHRASES` (lowercase; case-insensitive `contains`). `validate` (EvidenceIR)
surfaces an **Ambiguity / Weak Phrases** section, an `ambiguous_statements` metric, and an
`evidence_ambiguous_statements` Info finding. Flag-only / extraction-neutral — nothing is
dropped, only annotated.

**Modal verbs (MUST/SHALL/SHOULD/MAY) are deliberately NOT flagged** — they carry normative
*strength*, not vagueness, and are handled by the constraint/obligation extraction. Grounded
in Wilson/Rosenberg/Hyatt (NASA ARM, ICSE 1997) + Berry-Kamsties. Routing flagged statements
into typed residual packets is a deferred follow-up. See
`docs/tasks/AMBIGUITY-PHRASE-DETECTOR.md` and `docs/research/grounding/requirements-extraction.md`.
