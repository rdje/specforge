---
id: parenthetical-data-head-requires-wire-qualifier
title: Parenthetical data heads require an adjacent wire qualifier
answers:
  - "when does data (ACRONYM) declare a one-bit signal"
  - "why is Vital Product Data (VPD) not a signal"
  - "why is Wishbone memory output data (DO) not a port"
  - "why do I2C SDA USDA and SDAH remain signals"
  - "why does I2S SD remain a signal"
  - "what is the complete parenthetical data-head declaration census"
  - "where is parenthetical single-wire authority enforced"
  - "did the parenthetical data-head repair change real serial signals"
date: 2026-08-10
status: current
tags: [evidence-ir, signal-authority, parenthetical, sparse-catalog, i2c, i2s, wishbone, opencapi, adr-0006]
evidence: crates/specforge/src/ir/evidence.rs; docs/tasks/CORPUS-COVERAGE.md (.2.46a); generated/evidence_ir/opencapi_discovery_configuration_v201; generated/evidence_ir/wbspec_b4_wishbone_b4_specification; generated/evidence_ir/um10204_rev7_0_2021_i2c_bus_specification; generated/evidence_ir/um11732_v3_2022_02_17_i2s_bus_specification
reverify: "Build the release binary; census all generated/evidence_ir artifacts for synthetic width-one declarations whose source phrase has immediate head data; expect exactly six candidates across four documents. Rebuild EvidenceIR through adapter for OpenCAPI Discovery, Wishbone B4, I2C, and I2S; require VPD and DO absent, I2C declarations exactly SCL/SCLH/SDA/SDAH/USCL/USDA, I2S declarations exactly SCK/SD, I2C renderable at six signals/26 rules, and two complete replay manifests byte-identical. Then run cargo test -p specforge ir::evidence, the nine provider-free WIRE/I2C/SWD eval datasets, specforge kg-bench, and every emitted ISF through FSMGen strict."
---

**Established `2026-08-10` (`CORPUS-COVERAGE.2.46a`).** The complete 80-EvidenceIR census finds exactly six
synthetic width-one declarations admitted through a parenthetical whose immediate noun head is `data`. Four are
real wires with an adjacent electrical/transport qualifier: I2C `serial data (SDA)`, `serial data (USDA)`,
`high-speed data (SDAH)`, and I2S `Serial Data (SD)`. Two are properties or example payloads: OpenCAPI `Vital
Product Data (VPD)` and Wishbone example-memory `output data (DO)`. The Wishbone sentence itself names the real
bus output as `DAT_O`; `DO` is only the example memory's payload abbreviation.

The causal seam is the sparse-catalog (`<8` declarations) fallback in
`crates/specforge/src/ir/evidence.rs`. Parenthetical uppercase tokens gain a synthetic one-bit declaration only
when the immediately preceding noun phrase has single-wire authority. `line`, `signal`, `clock`, `wire`, and
`pin` are intrinsically wire-shaped in this grammar, but bare `data` is not: it also names register contents,
memory values, product metadata, and structure fields. Accepting bare `data` produced synthetic `Signal VPD is
width 1.` and `Signal DO is width 1.`, which survived canonical interface construction and reached emitted ISF
ports despite having no independent declaration authority.

The shared repair therefore requires only a `data` head to carry adjacent `serial` or `high-speed` authority,
case-insensitively. Other accepted heads, pin appositives, definitional signal prose, and table declarations are
unchanged; production policy contains no vendor, document, or signal name. Paired unit tests pin both sides.
Real-corpus replay removes `VPD` and `DO`, while I2C retains exactly
`SCL/SCLH/SDA/SDAH/USCL/USDA` and its six-signal/26-rule adapter, and I2S retains exactly `SCK/SD` plus both
canonical signals. I2S's current adapter blocks on the independent absence of behavioral content, not signal
loss.

Two complete four-document cascades plus validation reproduce all 33 downstream hashes. All 351 EvidenceIR
tests pass (346 executable, five intentional local measurements ignored); nine provider-free WIRE/I2C/SWD gold
datasets retain their declared filtered 1.000 gates; KG remains 156/156; and all 61 current emitted ISFs pass
FSMGen strict with zero diagnostics.
