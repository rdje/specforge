---
id: a-keyword-definition-is-typed-with-the-obligations-it-states
title: A keyword the document defines (Reserved is a keyword …) is a keyword_definitions record with each shall/must/should obligation its definition states, carried to IntentIR and never lowered
answers:
  - "what is keyword_definitions / KeywordDefinitionRecord / KeywordObligation / KeywordObligationModality"
  - "where does 'a reserved bit shall be set to zero by the sender and ignored by the receiver' go"
  - "why is a keyword definition apparatus but still typed (ADR 0033 decision 6)"
  - "what is isf_keyword_obligations_not_bound"
  - "why does 'Shall is a keyword' state no obligation"
  - "why is 'by this specification' not an agent"
  - "what is EXTRACTION-REPRESENTATION-GAPS.55.a and .55.b"
  - "which records cite the Reserved keyword definition (reserved_spans.keyword_definition_id)"
date: 2026-10-21
status: current
tags: [evidence-ir, keyword-definitions, notation, obligations, reserved-fields, non-executable, carried-evidence, isf-adapter, adr-0033, extraction-representation-gaps]
evidence: crates/specforge/src/ir/evidence.rs (KeywordDefinitionRecord; KeywordObligation; KeywordObligationModality; keyword_definition_surface; keyword_obligations; keyword_definition_tests); crates/specforge/src/ir/semantic.rs and intent.rs (keyword_definitions); crates/specforge/src/ir/adapters.rs (keyword_definition_residual_packet); docs/tasks/extraction-representation-gaps/hint-and-notation-leaves.md (.55, .55.a, .55.b); docs/tasks/extraction-representation-gaps/packet-and-memory-leaves.md (.55.b.iv.a)
reverify: "cargo test --offline -p specforge-core --lib keyword_definition"
---

A document can define a keyword that carries a rule on everything it marks — USB's *Reserved is a keyword indicating
reserved bits, bytes, words, fields, and code values … A reserved bit, byte, word or field shall be set to zero by the
sender and shall be ignored by the receiver* — and since `.51` such definitions are apparatus, as the conventions
section they stand in. ADR 0033 decision 6 keeps non-executable knowledge typed, so since
`EXTRACTION-REPRESENTATION-GAPS.55.a` (`2026-10-21`) EvidenceIR carries each definition as a `KeywordDefinitionRecord
{ keyword_id, keyword, definition, obligations, supporting_statement_ids }` (family `evidence.normative`), which
SemanticIR and IntentIR carry unchanged.

**Reading (`keyword_definition_surface`).** A statement of any class whose first word (past list markers) is
keyword-shaped (capital first, letters, digits, `/`, `-`) followed by *is a keyword*. Each *shall*, *must* or
*should* in the definition is one `KeywordObligation { text, modality, agents }` — `obligation`, `prohibition`
(*shall not*), `recommendation`, `discouragement` — except the keyword opening its own definition (*Shall is a
keyword …*). A clause runs from the last *and* before its modal to the last *and* before the next modal (*shall be set
to zero by the sender and shall be ignored by the receiver* is two); its agents are the words after *by* (*the
sender*), never a reference to the document (*by this specification*). The same keyword twice is one record.

**Disposition.** Since `.55.b.iv.a` (`2026-10-21`) the space a captured layout reserves cites the *Reserved*
definition: EvidenceIR's `reserved_spans` (card `a-layouts-reserved-space-is-a-typed-span-citing-the-keyword`). No
field record carries a keyword and the spans stop at EvidenceIR, so every adapter still names a document's
definitions in one `isf_keyword_obligations_not_bound` residual listing the obliging ones.

**Measured `2026-10-21` (31 chains):** USB 3.2 and USB4 inter-domain each define 8 keywords with 7 obligations
(*Reserved* 5, *N/A* 1, *Optional* 1); no other chain defines one. Every `.isf` text byte-identical.
