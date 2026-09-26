---
id: wishbone-clock-and-reset-come-from-prose
title: Wishbone's system contract (CLK_I, RST_I synchronous active-high) is read from its prose since 2026-09-26 — a clock input named in a sentence, the reset's kind from a relative clause on the reset signal, and its polarity from a rule over a class of signals the document places the reset in — and its ports from its per-role signal sections, so it emits master.isf with exactly the MASTER's ports (EXTRACTION-REPRESENTATION-GAPS.7.c, .32)
answers:
  - "where does Wishbone's clock and reset come from (it has no signal table)"
  - "how is a polarity stated as a rule over a class of signals read (All WISHBONE interface signals MUST use active high logic)"
  - "why does I2C's serial clock signal SCL not become the system clock (only a clock input is read; .7.e)"
  - "why did CLOCK GENERATION refuse Wishbone's contract (a heading parsed as a second clock; a clock is now declared only by a sentence)"
  - "why did Wishbone's slave.isf declare ACK_I as an output (The SLAVE asserts either the [ACK_I] ...; fixed by EXTRACTION-REPRESENTATION-GAPS.32)"
  - "where do Wishbone's ports come from (its 2.2.x MASTER/SLAVE Signals sections)"
  - "why are the system clock and reset not counted when choosing the initiator"
  - "how many .isf files are emitted (AXI, AHB, APB-E, Wishbone)"
date: 2026-09-26
tags: [isf, reset, clock, system-contract, wishbone, extraction-representation-gaps, measured]
evidence: crates/specforge/src/ir/evidence.rs (synthesize_system_clock_prose_statements, relative_clause_protocol_kind, synthesize_reset_class_rule_polarity_statements, stated_reset_descriptor); crates/specforge/src/ir/semantic.rs (build_system_contract); docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md (.7.c, .7.e, .32)
reverify: "cargo test --offline -p specforge-core --lib -- a_reset_kind_stated_in_a_relative_clause_on_the_signal a_clock_input_named_in_prose a_reset_polarity_stated_over_a_class a_heading_is_no_second_clock. jq '.system_contract' generated/semantic_ir/wbspec_b4_wishbone_b4_specification/semantic_ir.json -> CLK_I, RST_I synchronous active_high (2026-09-26)."
---

**Landed `2026-09-26` (`EXTRACTION-REPRESENTATION-GAPS.7.c`).** Wishbone has no signal table, so the table readers
that give AXI, AHB and APB-E their clock and reset found nothing, and its adapter was blocked. It states all three
facts in prose, and each is now read from its own sentence:

- **Clock:** *The clock input [CLK_I] coordinates all activities …* gives `Clock CLK_I.`. Only a clock *input* counts.
  I2C's *serial clock signal SCL* is a line its initiator drives, which `.7.e` keeps out of the contract. Nothing is
  added when the document already declares a clock.
- **Kind:** *the WISHBONE reset [RST_I] signal that uses a purely synchronous protocol* names the reset (`reset [X]
  signal`) and states its kind in a relative clause.
- **Polarity:** *All WISHBONE interface signals MUST use active high logic* reaches `RST_I` only through the document's
  own links: *the SLAVE interface MUST include the following signals: … [RST_I]* and *WISHBONE MASTER and SLAVE
  interfaces*. Without either link, nothing is stated.

A heading, *CLOCK GENERATION*, had parsed as a second clock and refused the contract. A clock is now declared only by a
sentence. A first draft required the clock to be a declared signal. It broke the markdown fixtures that declare their
clock with `Clock clk.` alone, and was replaced.

Wishbone emits `slave.isf`, `(reset (RST_I sync active_high))`, strict-clean. Its port set is `.32`'s: *The SLAVE
asserts either the [ACK_I], [ERR_I] or [RTY_I] signals* puts the MASTER's port names on the SLAVE.
`[[an-unstated-reset-kind-is-a-listed-choice]]` · `[[isf-initiator-perspective-direction]]`.

## Ports from the role sections (`2026-09-26`, `.32`)

Wishbone assigns every port to a role in its own sections: *2.2.3 MASTER Signals*, *2.2.4 SLAVE Signals*, *2.2.2
Signals Common to MASTER and SLAVE Interfaces*. An *input* or *output* naming a bracketed signal under such a heading
relates the role to it. A prose relation that puts a signal on a role whose section does not name it is dropped as the
other side of the wire (*The SLAVE asserts either the [ACK_I] …*). The initiator is chosen without the system clock and
reset, which every actor reads; counted, they tied the MASTER at 7 and 7. Wishbone emits `master.isf` with exactly the
MASTER's 12 ports.
