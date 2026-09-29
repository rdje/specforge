---
id: a-register-field-binding-to-a-wire
title: A register field bound to a wire carries it as signal_bindings (drives / reflects), read from the body under the field's own heading
answers:
  - "where does specforge record that a register bit controls a signal"
  - "what is RegisterFieldRecord.signal_bindings"
  - "what is fill_field_signal_bindings"
  - "which ADIv6 fields are bound to wires"
  - "why is ADIv6 Table B2-7's bit-to-signal mapping not read"
date: 2026-09-29
status: current
tags: [evidence-ir, registers, register-fields, signals, adiv6, extraction-representation-gaps, measured]
evidence: crates/specforge/src/ir/source.rs (RegisterFieldRecord.signal_bindings, RegisterFieldSignalBinding, RegisterFieldSignalRelation); crates/specforge/src/ir/evidence.rs (fill_field_signal_bindings, field_signal_bindings_in); docs/tasks/extraction-representation-gaps/recent-leaves.md (.41, .41.a); docs/book/src/pipeline/register-tables.md (A field bound to a wire)
reverify: "cargo test --offline --workspace --exclude specforge-production-graph --lib a_field_heading_body_binds_the_field_to_its_wire; then python3 over generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json: 10 register fields carry signal_bindings, 6 under CTRL/STAT and 4 under CSW@JTAG-AP"
---

Architecture specifications give each register field a `.10g` heading (`CSYSPWRUPREQ, bit[30]`) and say in its body
what the field is bound to. *This bit controls the CSYSPWRUPREQ signal* and *This field drives the nSRSTOUT signal*
bind a field that `drives` the wire. *Indicates the status of the CSYSPWRUPACK signal* and *returns the logical AND of
the PORTCONNECTED signals* bind one that `reflects` it. Since `EXTRACTION-REPRESENTATION-GAPS.41` (`2026-09-29`),
`fill_field_signal_bindings` walks the content in reading order. It reads the body under such a heading into the one
field of that name and bit range, and a field two registers hold at the same bits gets nothing. The binding is stored
as `RegisterFieldRecord.signal_bindings` (the signal as printed, e.g. `TRST*`).

Before `.41` these sentences made the relation reader publish `bit` and `field` as actors
([[a-storage-or-value-noun-heads-no-agent]]). Its first replay attached the power-up bindings to `ABORT`, because the
`.10g` reader had read `CTRL/STAT`'s fields into `ABORT` (a `/` in the container name, `.41.a`). On the 31 current
chains only ADIv6 states such bindings: 10 fields. Nothing lowers them yet, and `.isf` is unchanged. Table B2-7 (`Signal
| Programmers' model`) maps the same wires to `CTRL/STAT` bits, but its columns are converter-swapped and it is not
read.
