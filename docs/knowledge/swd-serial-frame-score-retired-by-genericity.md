---
id: swd-serial-frame-score-retired-by-genericity
title: The SWD 29/29 protocol signoff is retired, not current — it was scored by a protocol-name-bound extractor that ADR 0006 enforcement removed
answers:
  - "is SWD at 100% / does the SWD derivation gold still score 29/29 (NO. Re-derived 2026-09-01 with the restored oracle: serial_frame_field 0/11 and protocol_state 0/13, protocol_operation 4/4 and interface_edge_timing 1/1 — document-level 5/29. The 29/29 is retired, not current)"
  - "why did the SWD serial_frame_field score go from 11/11 to 0/11 (because SPEC-TO-INTENT-ALIGNMENT.6d.ii.c at 89d8dee7 on 2026-08-12 retired the protocol-name-bound frame extractor and its fixed phase enum on ADR 0006 genericity grounds; its own ledger entry says the exact comparison retires 22 fixed-phase frame and four named-operation records)"
  - "is the SWD frame/state score drop an extraction regression (NO. check_chain_currency.sh reports 24/24 current, so the persisted artifact is exactly what the current binary produces; the loss is surface-selective and corpus-wide; and the 2026-08-12 ledger's own counts re-derive exactly from today's corpus)"
  - "what did the retired SWD frame extractor key on (literal protocol identity: extract_serial_frame_fields at 89d8dee7^ gated the whole document on the strings serial wire / packet request / shift-dr / swdio / swclk, then assigned a fixed three-value SerialFramePhase enum from wdata / rdata / datain / ack[ — exactly what ADR 0006 forbids in production)"
  - "how many serial frame fields does the current producer emit across the corpus (ZERO, across all 24 rebuildable schema-3 documents; protocol_states carry a machine_name on 0 of 40. protocol_operations still produce 5 corpus-wide — 4 SWD plus 1 Wishbone — which is exactly the five the 2026-08-12 ledger said the generic producer retains)"
  - "why does the current generic frame grammar produce nothing on SWD (extract_serial_frame_fields admits a field only from a statement that itself carries a document-stated phase name AND a bit-range or named-bit parse. SWD states its phases in 61 statements and writes its fields — A[3:2], WDATA[31:0] — in different ones, so the two never coincide)"
  - "was the SWD gold wrong (NO. seed_swd_derivation.json's 29 facts are real, independently verified statements of the ADI spec. The gold is faithful; what changed is that no production extractor may reach them through protocol identity)"
  - "can the SWD frame facts be recovered generically (not yet — it needs the phase stated in a section or paragraph to bind the fields in its scope rather than only within one sentence. That is document grammar, not protocol identity, so it is ADR-0006-admissible; owned by WIRE-BASED-100.8d)"
date: 2026-09-01
status: current
tags: [swd, adi, wire-based-100, eval, genericity, adr-0006, claim-verification, serial-frame]
evidence: "./target/release/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_swd_derivation.json --provider skip; git show 89d8dee7^:crates/specforge/src/ir/evidence.rs (extract_serial_frame_fields identity gate + enum SerialFramePhase); git show 89d8dee7 -- CHANGES.md (the retirement entry); crates/specforge/src/ir/evidence.rs (stated_phase_name is the current admission gate); scripts/check_chain_currency.sh --check"
reverify: "./target/release/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_swd_derivation.json --provider skip 2>&1 | grep -E 'serial_frame_field|protocol_state|protocol_operation|interface_edge_timing'"
---

# A retired score, not a regression

`SWD-SERIAL-EXTRACTION` published a 29/29 protocol signoff at `1.000` on `2026-08-09`: 11 frame fields, four
operations, 13 states, and one interface-edge timing fact. Re-derived on `2026-09-01`, the first time the
scoring oracle could run since (`WIRE-BASED-100.8a`), the same gold scores:

```text
protocol_operation     P=1.000 R=1.000 F1=1.000  (tp=4 fp=0 fn=0)
interface_edge_timing  P=1.000 R=1.000 F1=1.000  (tp=1 fp=0 fn=0)
serial_frame_field     P=0.000 R=0.000 F1=0.000  (tp=0 fp=0 fn=11)
protocol_state         P=0.000 R=0.000 F1=0.000  (tp=0 fp=9 fn=13)
```

Document-level recall is **5/29**.

## Why this is not an extraction regression

Three independent legs separate the two competing explanations.

**The artifact is not stale.** `check_chain_currency.sh --check` replays all 24 rebuildable documents to
24 current / 0 stale at evidence, semantic, intent, and the ISF adapter. The persisted SWD EvidenceIR is
exactly what the current binary rebuilds from its SourceIR, so the zeros are the current producer's real output.

**The loss partitions along the retirement boundary, corpus-wide.** Across all 24 measurable documents the
current producer emits **zero** `serial_frame_fields` and binds a `machine_name` on **0 of 40**
`protocol_states`, while `protocol_operations` still produce **5** — four on SWD and one on Wishbone. A
regression does not partition that cleanly. `SPEC-TO-INTENT-ALIGNMENT.6d.ii.c`'s ledger entry from
`2026-08-12` predicted exactly this shape: *"Exact comparison retires 22 fixed-phase frame and four
named-operation records; the generic producer retains five operations and 40 structurally admitted states."*
Five operations and forty states re-derive from today's corpus on the nose.

**The producer's own code says so, per revision.** At `89d8dee7^`, `extract_serial_frame_fields` gated the
whole document on literal protocol identity —

```rust
let is_serial_doc = statements.iter().any(|s| {
    let l = s.text.to_ascii_lowercase();
    l.contains("serial wire") || l.contains("packet request")
        || l.contains("shift-dr") || l.contains("swdio") || l.contains("swclk")
});
```

— and then assigned a fixed `SerialFramePhase { Request, Acknowledge, Data }` from `wdata` / `rdata` /
`datain` / `ack[`. ADR 0006 forbids exactly that in production. The enum no longer exists.

## What this means for the number

The 29/29 was never evidence of generic capability: it measured an extractor that recognised one protocol by
name. Enforcing the genericity doctrine was correct and the trade was published — but the retired score was
left standing as current on the roadmap, the status ledger, the book, and four fact cards for twenty days,
because the oracle that would have caught it was itself down
(`[[evidence-proof-binds-artifact-location]]`). A genericity trade may retire a score; it may not leave the
retired score published as current.

The gold is untouched and stays faithful: `seed_swd_derivation.json`'s 29 facts are real statements of the ADI
specification, and they remain the target. What is now honestly measured is that generic extraction reaches 5
of them.

## The residual, stated precisely

Today's `extract_serial_frame_fields` admits a field only from a statement that itself carries a
document-stated phase name (`stated_phase_name`) **and** parses a bit range or named bit list. SWD carries 61
statements with a stated phase name, and writes `A[3:2]`, `WDATA[31:0]`, `RDATA[31:0]` in different statements,
so the conjunction never holds. The generic repair is scope binding — a phase named in a section or paragraph
binds the fields within its scope — which is document grammar rather than protocol identity, so ADR 0006
admits it. Owned by `WIRE-BASED-100.8d`.

Links: [[swd-derivation-scored-100]], [[swd-canonical-protocol-artifact-is-current]],
[[swd-adi-not-signal-table-spec]], [[swd-protocol-surfaces-reach-intentir]],
[[evidence-proof-binds-artifact-location]], [[production-genericity-boundary]].
