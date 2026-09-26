---
id: an-operation-kind-a-level-states-is-a-guard
title: A kind of operation a declared signal's level states (APB's PWRITE indicates an APB write access when HIGH and an APB read access when LOW) is a SemanticIR operation_kinds record carried into IntentIR, and a condition that names only that kind of operation (For read transfers) reads as that level in every reader; a transaction named without such a statement, or identified only by an opcode table, stays listed; since 2026-09-26 (EXTRACTION-REPRESENTATION-GAPS.10.g.ii)
answers:
  - "why does requester.isf drive PSTRB 0 when PWRITE is 0"
  - "what is operation_kinds / OperationKindRecord / operation_kind_condition"
  - "how is For read transfers read as a guard"
  - "why are AXI's for a WriteZero transaction conditions still listed"
  - "why is AHB's HWRITE direction sentence not read"
date: 2026-09-26
tags: [semantic, intent, isf, conditions, transactions, extraction-representation-gaps, measured]
evidence: crates/specforge/src/ir/semantic.rs (OperationKindRecord, document_operation_kind_records, operation_kind_condition, build_temporal_rules); crates/specforge/src/ir/intent.rs (synthesize_temporal_invariants); crates/specforge/src/ir/isf_ir.rs (lower_obligations); crates/specforge/src/ir/evidence.rs (is_operation_noun); docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md (.10.g, .10.g.ii, .10.g.iii)
reverify: "cargo test --offline -p specforge-core --lib -- a_signal_level_that_states_an_operation_kind_is_its_guard a_condition_naming_an_operation_kind_is_the_level_that_states_it. grep -c '(== PWRITE 0)' generated/adapters/isf/ihi0024_e_2023_02_amba_5_apb_protocol_specification/requester.isf -> 5 on 2026-09-26."
---

**Measured and landed `2026-09-26` (`EXTRACTION-REPRESENTATION-GAPS.10.g` → `.10.g.ii`).** On the 27 live adapters, 19
obligations were listed because their condition is not `SIGNAL is LEVEL`: AXI 17 and APB-E 2. A transaction's
*name* identifies nothing that `.isf` can test. AXI's IntentIR carries 14 named transactions, but none carries its
identifying values, and `manager.isf` emits none. What identifies one is the document's own statement, and there
are two kinds:

- **A signal's level.** APB's *PWRITE indicates an APB write access when HIGH and an APB read access when LOW*
  (`0170`) becomes `operation_kinds` records `read` → `PWRITE` LOW and `write` → `PWRITE` HIGH. The kind is the word
  before a closed-class operation noun (the class `.10.c` uses). A kind that two sentences give different levels
  or signals is dropped. `operation_kind_condition` turns a condition naming only that kind of operation (*For
  read transfers*, *a read transfer*) into `PWRITE is LOW` before each reader parses it: the temporal rule, the
  IntentIR invariant, and the adapter's constraint lowering. APB-E's `requester.isf` gains 5 rules `(== PWRITE 0)
  → (PSTRB 0)`, strict-clean. The sentence states no transfer scope, so none is added: `PSTRB` is also driven 0
  in idle cycles with `PWRITE` LOW, which the document leaves unconstrained.
- **An opcode table.** AXI's `| 0b00111 | NSH, SH, SYS | WriteZero | … |` keys WriteZero by `AWSNOOP` *and*
  `AWDOMAIN`, so one signal's value is not the key. InvalidateHint, UnstashTranslation, Prefetch, *non-stash*,
  Atomic (`AWATOP`) and *DVM Complete* (`ARSNOOP`) are the same kind. All stay listed (`.10.g.iii`). Seven also
  need presence (`.4`), and the *DVM Complete* rows are implications between outputs, not drives (`.11`).

Not read: AHB's *When HIGH this signal indicates a write transferandwhenLOWa read transfer* (`0265`). It fronts its
level, names its signal as *this signal*, and its text layer glues the words. No AHB obligation names a read or
write transfer, so nothing waits on it.
`[[a-per-target-family-is-typed-intent]]` · `[[an-uncited-obligation-is-a-candidate-miss]]`.
