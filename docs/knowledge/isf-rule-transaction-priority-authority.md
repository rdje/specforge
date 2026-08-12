---
id: isf-rule-transaction-priority-authority
title: SpecForge emits no blanket ISF rule-over-transaction priorities; IntentIR has no precedence carrier, so a rule overlapping a uniquely owned named drive becomes an explicit residual instead of an invented winner
answers:
  - "why did FSMGen pin a51dcdad0 reject SpecForge manager.isf with isf_ambiguous_rule_transaction_drive_priority on AWSNOOP"
  - "why did SpecForge remove every generated (priority RULE over TRANSACTION) line"
  - "did IntentIR or SemanticIR ever contain rule transaction priority authority (no — the ISF emitter fabricated the Cartesian product)"
  - "what does drop_ungrounded_rule_transaction_conflicts do"
  - "why does an isf_rule_transaction_conflict_<name> residual appear in adapter.json"
  - "how does SpecForge handle one transaction and one rule writing the same named-drive target"
  - "why is a multi-caller named drive kept without actor priority"
  - "how many current emitted ISFs pass FSMGen after retiring fabricated priorities (44 of 44, zero diagnostics at pin a51dcdad0)"
date: 2026-08-12
tags: [isf, fsmgen, priority, transaction, named-drive, residual, semantic-authority, no-fabrication, fsmgen-refresh-integrate-6]
evidence: crates/specforge/src/ir/isf_ir.rs (drop_ungrounded_rule_transaction_conflicts and no IsfPriority storage/rendering); docs/tasks/FSMGEN-REFRESH-INTEGRATE-6.md; subs/fsmgen/docs/ISF_RULE_TRANSACTION_NAMED_DRIVE_PRIORITY_BEHAVIOR.md; FSMGen upstream commit 1dbff8fc6
reverify: "Build the current SpecForge binary; rebuild adapters from generated/intent_ir; rg '^  [(]priority ' generated/adapters/isf -g '*.isf' must find zero; run subs/fsmgen/bin/fsmgen --strict --check --json over all 44 emitted .isf files and require success=true plus diagnostic_count=0; AHB adapter must contain five isf_rule_transaction_conflict_* residuals."
---

FSMGen refresh cycle 6 exposed a downstream authority bug rather than an upstream regression. The old
`IsfIr::from_intent_ir` created every combination of rule and transaction and rendered it as
`(priority RULE over TRANSACTION)`. Neither SemanticIR nor IntentIR carries that relation, so the adapter was
inventing execution semantics.

FSMGen pin `a51dcdad0` correctly made the fabrication observable. A named drive with several local transaction
callers has no unique transaction owner, so an applicable actor priority fails closed with
`isf_ambiguous_rule_transaction_drive_priority`. Removing the unsupported priority made that artifact clean.
Across the full corpus, deletion alone left one honest AHB conflict: `idle_transfer` uniquely owns the `HTRANS`
named drive while five rules also write `HTRANS`; FSMGen requires an explicit winner and the source provides none.

The emitter therefore has no `IsfPriority` model or render path. It recursively gathers named-drive calls. When
exactly one distinct local transaction calls a drive and a rule writes the same target, the transaction remains
executable and the rule becomes an `isf_rule_transaction_conflict_<name>` residual. Multi-caller drives remain
priority-free because no single transaction owns them and current FSMGen accepts that shape. This is structural
and protocol-independent: no document, signal, or vendor exception exists. The rebuilt current corpus contains
zero priority lines and all 44 emitted artifacts pass FSMGen strict with zero diagnostics.

Related: [[isf-unconditional-rule-overlap-conflict]], [[feedback_isf_no_hacks]].
