---
id: a-section-defining-the-documents-notation-is-apparatus
title: Every sentence of a section that defines the document's own notation — its conventions, a notation, its typography or its pseudocode — is apparatus; a numbered such heading covers the headings numbered under it
answers:
  - "why are ADIv6 appendix E3 Pseudocode Definition sentences apparatus"
  - "what is document_notation_sections / names_document_notation / outline_number"
  - "why is 'The block of lines consisting of elsif and its indented statements is optional' not an optional feature"
  - "why is a Conventions, Typographic conventions, RTL notation or Signal Naming Conventions section apparatus"
  - "why is AXI 'Pseudocode description of the transfers' not apparatus (pseudocode put to use)"
  - "how does SpecForge tell which flat section headings belong to an appendix (outline numbers)"
  - "does the collective-name reader still read AxCACHE from the conventions section"
  - "what is EXTRACTION-REPRESENTATION-GAPS.51 and .55"
date: 2026-10-21
status: current
tags: [evidence-ir, statement-class, apparatus, notation, conventions, pseudocode, section-headings, adr-0006, extraction-representation-gaps]
evidence: crates/specforge/src/ir/evidence.rs (StatementClass::Apparatus; outline_number; names_document_notation; document_notation_sections; the statement loop's class); docs/archive/tasks/extraction-representation-gaps/hint-and-notation-leaves.md (.51, .55)
reverify: "cargo test --offline -p specforge-core --lib a_heading_names_the_documents_notation_or_puts_it_to_use a_numbered_notation_heading_covers_the_headings_numbered_under_it a_sentence_of_a_notation_section_is_apparatus"
---

A document explains its own notation — the pseudocode language it writes behaviour in, its typefaces, its keywords,
how it names signals and numbers bits — and those sentences were read as protocol content: ADIv6's appendix E3
(*Pseudocode Definition*) gave an optional feature (*The block of lines consisting of elsif and its indented statements
is optional*), normative statements (*This statement indicates a special case that replaces the behavior defined by
the current pseudocode*) and an equivalence rule (*Hexadecimal integer constants are treated as positive*).

Since `EXTRACTION-REPRESENTATION-GAPS.51` (`2026-10-21`) every sentence of a **notation section** is `apparatus`,
the class no protocol reader consumes:

- **A heading names the document's notation** (`names_document_notation`) when it holds *convention(s)*,
  *notation(s)*, *typographic* or *typographical* (*Typographic conventions*, *1.5 Signal Naming Conventions*, *RTL
  notation*, *8.12.1.1 State Machine Notation Information*), or names pseudocode as its topic (*Pseudocode
  Definition*, *About ARM pseudocode*, *Pseudo Code*) — not pseudocode put to use, which an *of*/*for* after the word
  marks (*A3.1.7 Pseudocode description of the transfers*).
- **Headings are flat** (a converter can give every heading level 1), so the outline number decides membership
  (`outline_number`: `E3.2.1`, `1.6.2.3`, a chapter's `2`, an appendix's `E3`). A numbered notation heading covers
  the headings numbered under it and the unnumbered ones among them (*E3.2.1 General data type rules*, *UNDEFINED*,
  *if … then … else …*), until a heading numbered outside it. An unnumbered one covers only itself: nothing says where
  its subject ends.

Readers that interpret notation read every class, so nothing they read is lost: the collective-name reader still
reads *AxCACHE refers to both ARCACHE and AWCACHE* from AXI's conventions. **Measured `2026-10-21` (31 chains):** 447
statements in 20 documents become apparatus (416 source facts, 23 normative, 5 conditional, 2 derived, 1 design
decision); 8 records leave — ADIv6's `elsif` option, its equivalence and conditional rule from *Integer constants*,
two conditional rules each from USB 3.2's and USB4 inter-domain's *Precedence* and *Reserved* keyword definitions,
and Wishbone's *Array limits may also be shown within the parenthesis* permission. Kept where they are: a
pseudocode function that describes hardware (*The pseudocode function EnterDormantState describes the function of
the JTAG-to-DS sequence detector*). The obligation a keyword definition carries (*a reserved bit … shall be set to
zero by the sender*) is owned by `.55`.
