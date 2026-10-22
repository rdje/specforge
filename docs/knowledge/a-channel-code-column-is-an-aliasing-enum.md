---
id: a-channel-code-column-is-an-aliasing-enum
title: A code table whose signal-titled columns name one collective's members gives each channel its whole enum, members sharing codes; only its reader's marking sentence exempts that enum from SemanticIR's collision rule
answers:
  - "why does AXI's ARCACHE enum have 12 members with repeated codes (A4.4; .3.g.iii.c.i)"
  - "what is read_channel_code_tables / channel_code_and_alternative"
  - "what does 'Enum ARCACHE shares a code between members.' mean (the collision-rule exemption)"
  - "what does 'Enum ARCACHE WRITE_THROUGH_READ_ALLOCATE also takes 6.' mean (a bracketed permitted alternative)"
  - "when may two enum members share a code (KG-ISF-COMPLETENESS.5.xiv.a and its one exemption)"
  - "why is the bracketed A4.4 code not emitted to .isf"
  - "what is EXTRACTION-REPRESENTATION-GAPS.3.g.iii.c and .3.g.iii.c.i"
date: 2026-10-22
status: current
tags: [evidence-ir, semantic-ir, encodings, enums, aliasing, collectives, extraction-representation-gaps]
evidence: crates/specforge/src/ir/evidence.rs (read_channel_code_tables; channel_code_and_alternative; the convergence loop's channel_tables skip set); crates/specforge/src/ir/semantic.rs (build_symbol_definitions: shared_code_enums and the collision rule); docs/tasks/extraction-representation-gaps/encoding-and-state-leaves.md (.3.g.iii.c, .3.g.iii.c.i)
reverify: "cargo test --offline -p specforge-core --lib a_channel_code_table_gives_each_channel_its_whole_enum members_sharing_a_code_are_kept_where_the_table_reader_states_it"
---

AXI *Table A4.4* gives each of 12 memory types an `ARCACHE` and an `AWCACHE` code. Each column repeats four codes:
on reads `0b1010` is both Write-Through No-Allocate and Write-Allocate, because a read cannot carry the
write-allocate hint. The document adds *Values in brackets are permitted but not preferred* (`0b1110 ( 0b0110 )`).
SemanticIR's collision rule (`KG-ISF-COMPLETENESS.5.xiv.a`, "no encoding assigns one code to two members") dropped
both members of every repeated code, so `ARCACHE` kept 6 members and `AWCACHE` none. The rule's default still holds
for the misreads it was written against: a split code, one bit of a field, a joint key.

**The reader (`EXTRACTION-REPRESENTATION-GAPS.3.g.iii.c.i`, `2026-10-22`).** `read_channel_code_tables` takes an
encoding table whose two or more signal-titled code columns (a signal or a slice of it, never one bit) name members
of one collective: one key written per channel. It also needs a member column of distinct names and a code in every
channel cell.

- **Members.** Each channel gets one `Enum SIGNAL MEMBER = CODE.` per row, with the preferred code.
- **Alternatives.** A bracketed code becomes `Enum SIGNAL MEMBER also takes CODE.`
  (`channel_code_and_alternative`). It stays in EvidenceIR and is not emitted, because an `.isf` member has one value
  and a second name for it would be invented.
- **The mark.** A channel whose codes repeat ends with `Enum SIGNAL shares a code between members.`
- **Skips.** The single-column scan and `.3.g.iii.a`'s column reader both skip the table.

**The exemption.** SemanticIR collects the enums named by such a sentence, provided it cites a table. A document
writing the same words exempts nothing. Those enums keep members that share a value. FSMGen strict accepts an enum
with two members on one value.

**Measured `2026-10-22` (31 chains):** only AXI moves. `ARCACHE` grows from 6 members to 12, `AWCACHE` from 0 to 12,
both typed 4 bits (`.59`), with 4 alternatives stated. `manager.isf` is FSMGen strict-clean, and the other 30
chains are unchanged.
