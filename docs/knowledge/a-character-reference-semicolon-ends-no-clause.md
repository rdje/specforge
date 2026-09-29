---
id: a-character-reference-semicolon-ends-no-clause
title: The converter writes < > & | as &lt; &gt; &amp; &#124;, and a clause splitter must not end a clause at the reference's semicolon
answers:
  - "why did TMC have an unconditional FULL must be HIGH constraint"
  - "why do statements contain &gt; and &lt;"
  - "what is split_clauses"
  - "does evidence split a clause at a semicolon inside &gt;"
  - "what is closes_character_reference"
  - "why does TMC's BUFWM sentence yield a conditional FULL constraint"
  - "what is fronted_condition_split"
date: 2026-09-29
status: current
tags: [evidence-ir, clause-splitting, converter-escaping, constraints, extraction-representation-gaps, measured]
evidence: crates/specforge/src/ir/evidence.rs (closes_character_reference, is_clause_boundary, split_clauses, find_clause_boundary, rfind_clause_boundary, fronted_condition_split); docs/tasks/extraction-representation-gaps/recent-leaves.md (.42, .42.a); docs/book/src/pipeline/evidence-failure-modes.md (A semicolon that belongs to a >)
reverify: "cargo test --offline --workspace --exclude specforge-production-graph --lib a_character_reference_semicolon_ends_no_clause; then grep -c '&gt;' over generated/evidence_ir/*/evidence_ir.json statement texts is nonzero (the IR keeps the converter's escaping), and TMC's evidence_ir.json holds exactly one signal_constraints record for FULL, whose condition_text is 'the space in the FIFO is less than or equal to this value'"
---

SourceIR and EvidenceIR keep the converter's markdown escaping, so a printed `<`, `>`, `&` and a table cell's `|`
reach every reader as `&lt;`, `&gt;`, `&amp;`, `&#124;`. On `2026-09-29` 281 statements on the 31 current chains
held one (ADIv6 173, mostly `SYSPSR&lt;n&gt;`; USB 3.2 47). Decoding them upstream is not a local fix: `&#124;`
stands for a pipe inside a table cell, and serialized table rows use `|` as their cell separator.

Twelve readers in `ir/evidence.rs` end a clause or a sentence at `;`, and each cut a reference in two. CoreSight TMC's
*When the space in the FIFO is less than or equal to this value, that is, Fill level &gt;= MEM_SIZE - BUFWM, the FULL
output is pulled HIGH* gave the level-binding path a clause that began after `&gt;`. The binding lost its *When*
and became an unconditional `FULL` must-be-HIGH constraint.

Since `EXTRACTION-REPRESENTATION-GAPS.42`, `closes_character_reference` recognises the `;` of `&name;` or `&#digits;`,
and every splitter goes through `split_clauses`, `find_clause_boundary` or `rfind_clause_boundary`. The converter
escapes every literal `&`, so any `&…;` in the IR is a reference. Replayed over the 31 chains, only that TMC
constraint moved: it is gone, along with an unconditional temporal rule on the junk actor `DEVID register`. Since
`.42.a` the level path reads the main clause after a fronted condition (`fronted_condition_split`) and keeps the
condition, so TMC now states *FULL must be HIGH when the space in the FIFO is less than or equal to this value*, which
the adapter lists rather than drives.
See also [[a-condition-marker-after-a-sentence-end-is-fronted]].
