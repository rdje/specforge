---
id: a-document-stated-collective-name-stands-for-its-members
title: A collective signal name the document defines (AxSIZE for AWSIZE and ARSIZE) is a typed record, read from its sentences and its stated lowercase-letter rule, and never from a prefix list
answers:
  - "what is collective_signal_names / CollectiveSignalNameRecord"
  - "how does SpecForge expand a generic Ax name to AW and AR"
  - "why is AxSNOOP's absent value bound to AWSNOOP and ARSNOOP now"
  - "what is collective_signal_name_surface / collective_marker / is_collective_member"
  - "how is 'Lowercase x at the second letter denotes a collective term' read"
  - "why are USB 3.2's RxD and TxEQ not expanded"
  - "does a collective name's sentence have to name declared signals (no, the sentence declares them)"
  - "what is collective_name on a StatedAbsentValue"
  - "which naming outranks which for an absent value (the signal itself, then a collective name, then a class)"
  - "are rules stated on a collective name applied to each member (.47: a polarity reaches each member that states none; the rest are descriptions members already state)"
date: 2026-10-20
status: current
tags: [evidence-ir, collective-name, generic-name, signal-presence, absent-value, carried-evidence, adr-0006, adr-0016]
evidence: crates/specforge/src/ir/evidence.rs (CollectiveSignalNameRecord; collective_signal_name_surface; collective_marker; is_collective_member; StatedAbsentValueReading::rank); crates/specforge/src/ir/semantic.rs and intent.rs (collective_signal_names); docs/archive/tasks/extraction-representation-gaps/unread-intent-leaves.md (.3.d.ii.a.ii)
reverify: "cargo test --offline -p specforge-core --lib collective_signal_name && python3 -c \"import json; e=json.load(open('generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json')); c=e['collective_signal_names']; print(len(c), [r for r in c if r['collective_name']=='AxSNOOP'])\"   # expect 37 records; AxSNOOP -> ARSNOOP, AWSNOOP citing statement_0446"
---

SpecForge keeps generic names literal (`AxBURST` enums, `AxMMUATST` conditions, a presence row `AxVALID`), and
its alias map is empty for AXI, so a sentence stated on `AxSNOOP` reached neither `AWSNOOP` nor `ARSNOOP`. The
document itself defines what the name means, in two shapes, and since `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.a.ii`
(`2026-10-03`) EvidenceIR carries each definition as a `CollectiveSignalNameRecord { collective_id,
collective_name, member_names, supporting_statement_ids }` in `evidence.declaration`, carried unchanged by
SemanticIR and IntentIR.

**Shape one, a sentence that names the members.** An identifier with exactly one run of lowercase letters
(`collective_marker`: `Ax` -> 1..2, `VAx` -> 2..3), a defining verb (`indicates`, `refers`, `denotes`, `means`,
`represents`), then two or more identifiers that are the name with its run replaced by uppercase letters
(`is_collective_member`). *In this specification, AxSIZE indicates AWSIZE and ARSIZE*; *the term VAxQOSACCEPT
refers collectively to the VAWQOSACCEPT and VARQOSACCEPT signals*; *AxMMU indicates AWMMU or ARMMU*. The members
are as written and need not be declared — the sentence declares them.

**Shape two, a stated rule.** A statement containing the word *collective* whose example is read by shape one
(*Lowercase x - At the second letter of a signal name denotes a collective term for both Read and Write. For
example, AxCACHE refers to both the ARCACHE and AWCACHE signals*) states that every name with the same run at
the same position stands for the same replacements. Each such name the document uses is derived — in the
example's member order — only when all its members are known signals (declared, or a presence-matrix row), and
cites the rule statement. No rule, no derivation: USB 3.2's `RxD`/`TxEQ` stay names (the negative control).

**Binding.** The absent-value reader resolves a statement that names no presence row but exactly one collective
name to each member that has a row, recording the name in `StatedAbsentValue.collective_name`. The closest
naming wins for a signal: the signal itself (rank 0), a collective name (1), a class through a check-coverage
table (2); within one rank, disagreeing statements still bind nothing.

**Measured `2026-10-03`.** Two of the 31 chains define collective names: AXI 37 records (6 from sentences, 31
from the rule; `AxCACHE` cites both) and ADIv6 2 (`CxxxPWRUPREQ`, `CxxxPWRUPACK`: *refers to either CSYSPWRUPREQ or
CDBGPWRUPREQ*). `AWSNOOP` and `ARSNOOP` take `LOW` from `statement_2207` through `AxSNOOP`; the unread
`assumed_by_word` family falls 107 -> 106. The shape's trap, found on Wishbone: *When the gap indicates a
wait state, the symbols '-WSM-' or '-WSS-' …* reads as `When` -> `WSM`, `WSS` unless a name must carry an uppercase
letter after its run — `collective_marker` requires one.

**`.47` (`2026-10-20`): measured, the gap was a polarity.** Of AXI's 250 statements naming a collective, the
normative and conditional ones are already read (their consequents are descriptions, not drives), no signal
constraint comes from one, and making collective-shaped tokens subjects of the convergent loop added only junk
(`SubstreamID`, `Num_RP`). The records keyed on a collective name have members carrying their own, except one
polarity: `extend_polarities_through_collective_names` gives `AWMMUATST` and `ARMMUATST` the active-high polarity
stated on `AxMMUATST`, only to a member that states none; enum definitions stay on the collective name.

Links: [[a-signals-stated-absent-value-rides-its-presence-record]],
[[a-check-coverage-table-binds-the-class-of-covered-signals]], [[signal-presence-matrix-capture]].
