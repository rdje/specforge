---
id: a-hint-role-counts-only-for-the-signal-that-bears-it
title: A semantic hint gives a signal a handshake role only when its text makes that signal the role's bearer — the agent of an indicating predicate, the subject of a verb-phrase role, or a description's label — in prose, tables and captions alike
answers:
  - "why is a signal tagged handshake_valid_like or handshake_ready_like"
  - "what is retain_borne_semantic_roles / SemanticRoleSubject / SemanticRoleReading / semantic_role_phrase_spans"
  - "why is a signal not valid-like when the sentence says valid data (presents valid data on its DAT_O lines)"
  - "why is PNSE must be valid when PSEL is asserted not a valid-like hint"
  - "why does invalid or already or validity no longer tag a signal"
  - "why did RTY_I, STALL_I and HREADYOUT lose a ready-like hint (a denied readiness)"
  - "why is WSTRB or RCHUNKSTRB not valid-like (indicates which byte lanes, a choice)"
  - "why is Valid indicator a valid-like hint for AWVALID even though AXI declares a signal VALID"
  - "does corpus prior memory still give a hint a role (a remembered role must pass the same text)"
  - "what is EXTRACTION-REPRESENTATION-GAPS.50"
  - "why is a table whose first column is headed Code or Key not a signal inventory (table_keys_codes_rather_than_signals, EXTRACTION-REPRESENTATION-GAPS.53)"
date: 2026-10-21
status: current
tags: [evidence-ir, semantic-hints, handshake-roles, prior-memory, prose, signal-description-tables, visual-captions, adr-0006, extraction-representation-gaps]
evidence: crates/specforge/src/ir/evidence.rs (VALID_ROLE_PHRASES; READY_ROLE_PHRASES; INDICATING_VERBS; ASSERTION_WORDS; is_role_label_word; semantic_role_phrase_spans; SemanticRoleSubject; retain_borne_semantic_roles; SemanticRoleReading; infer_signal_semantic_tags_from_description; synthesize_signal_semantic_hints_from_tables/_prose/_visual_evidence; table_keys_codes_rather_than_signals); docs/tasks/extraction-representation-gaps/hint-and-notation-leaves.md (.50, .53, .54)
reverify: "cargo test --offline -p specforge-core --lib a_prose_role_counts_only_for_the_signal_that_indicates_it a_description_names_its_signal_by_the_role_or_indicates_it a_remembered_role_holds_only_where_the_signal_states_it"
---

The three hint readers (signal-description tables, prose statements, visual captions and annotations) shared one word
test: `contains_any(" valid", "valid ", …)` on the text with signal names stripped. So a role inside a longer word
counted (*CleanInvalid*, *validity*, *already*, *AFVALID*), and any role word anywhere tagged whichever signal stood
near it. Since `EXTRACTION-REPRESENTATION-GAPS.50` (`2026-10-21`) `semantic_role_phrase_spans` matches the role
phrases as whole words, and `retain_borne_semantic_roles` keeps a role only when the text makes the signal its
**bearer**.

**The principle.** A handshake signal *indicates* that something is valid or that a receiver is ready; data simply
*is* valid. A role phrase therefore counts for a signal when the signal is:

- the agent of an **indicating predicate** whose complement holds it — *STB_O indicates a valid data transfer cycle*;
  *keeps ACK_I asserted to indicate that it is ready*; *REQ_VALID is set high, indicating … a valid request is
  pending*; *indicates that it has placed valid data … by asserting ACK_I*; *asserted when … is valid*. The agent is
  the nearest signal before the verb in its sentence that is not a preposition's object (*in response to STB_O* is
  not), and a finite verb's agent stands in its own clause;
- the **subject of a verb-phrase role** — *XREADY can accept the transfer* — but never the bare `valid` or `ready`
  said of the signal itself (*PNSE must be valid when PSEL is asserted* states its data's validity);
- a **description's label** — a description cell, or a table row naming exactly one signal, whose role word is
  followed by *signal*/*signals*/*indicator* (*Ready indicator*, *The ready signal used by …*) or closes a verbless
  first sentence (*Write address valid.*, *Ready.*).

It does not count for the data a validity qualifies (*presents valid data on its DAT_O lines*; *Number of valid bytes
on ATDATA*), for a signal named beside the bearer (*If there is valid data, that is, ATVALID HIGH*), when denied
within three words (*not ready*, *unable to complete the transfer*, *do not contain valid data*), or when what is
indicated is a choice (*indicates which byte lanes …*, *indicates the read data chunks that are valid*).

**A role label written as a word is the word.** AXI declares a generic `VALID`; *Valid indicator* case-folded to it,
so the stripping removed every *valid* from AXI's descriptions and its valid indicators came from corpus memory
alone. `is_role_label_word`: only the signal's own spelling names the signal (`VALID`), the word (*Valid*) is the
role.

**Corpus memory.** A recalled phrase (`semantic_phrase_role_in_text`) has no position, so it holds only when the text
has no phrase of that role of its own and the signal indicates, or opens a sentence as its subject (*XACK can receive
the transfer*). The local memory was learned from the old readings; with the new reader it adds one hint across the 31
chains (AXI `AWTAGOP` ready-like, from *already*) — owned by `.54`.

**Measured `2026-10-21` (31 chains):** 136 -> 97 hints (67 kept, 68 withdrawn, 29 gained — AXI's valid indicators
now from their own words); conflicts 4 -> 1 (Wishbone `ACK_I`, which the document gives both roles). AXI `Y`, a code
of a key table read as a signal, kept ready-like from *| Ready | Y |* until `.53` (`2026-10-21`) stopped reading a
table whose first column is headed *Code* or *Key* as a signal inventory; with the codes gone AXI's presence rows
name one signal each, and a value compared in an expression (*AXI_Transport == Ready*) is not a label.
