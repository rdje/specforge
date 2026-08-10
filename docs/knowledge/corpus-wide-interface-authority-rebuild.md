---
id: corpus-wide-interface-authority-rebuild
title: The corpus-wide rebuild collapsed 14 documents' false interfaces, taking emitted ISFs from 57 to 44
answers:
  - "how many emitted .isf files does the corpus have and are they FSMGen-strict clean"
  - "why did the emitted ISF count drop from 57 to 44"
  - "did retiring generic gates and phases cost ISF renderability"
  - "what made 14 corpus documents stop emitting an .isf"
  - "why is an emitted-ISF count from an earlier refresh slice not the current number"
  - "can the downstream corpus chain be rebuilt without re-ingesting documents"
date: 2026-08-10
status: current
tags: [corpus, isf-adapter, interfaces, semantic-ir, currency, measurement]
evidence: docs/tasks/CORPUS-CHAIN-CURRENCY.md (.3); scripts/check_chain_currency.sh; crates/specforge/src/ir/adapters.rs
reverify: bash scripts/check_chain_currency.sh
---

The first corpus-wide downstream rebuild (`CORPUS-CHAIN-CURRENCY.3`) re-ran `semantic` → `intent` →
`adapt --target isf` for every document from its unchanged persisted EvidenceIR. The emitted-`.isf`
population moved **57 → 44**, and all 44 pass `fsmgen --strict --check --json` with zero diagnostics.

**The cause is interface authority, not the gate/phase retirements.** Exactly 14 documents' SemanticIR
`interfaces[]` collapsed from a heuristic bulk (929, 737, 530, 480, 426, 211, 195, 135, 131, 121, 82, 78,
74, 7) to **zero** under current authority, and all 14 now block with `no signals declared in interface`
and emit nothing. Not one of them still emits. This confirms rather than contradicts
[[legacy-generic-gates-are-audit-only]] and [[legacy-generic-section-phases-are-audit-only]]: the retired
generic gates and phases carried no renderability value; the false heuristic interfaces did.

Each `.2.4x` refresh leaf removed stale heuristic interfaces for **its own** document only, so every
un-refreshed document kept its false interface set — and its `.isf` — until this rebuild applied current
authority to the whole corpus at once.

**An emitted-ISF count taken before this rebuild is a mixture of code vintages, not a measurement of the
current binary.** The per-slice figures in the refresh cards and ledger (66, 67, 61, 60, 59, 58, 57 …) are
exact history for their own slice; the reproducible current number is 44, and `CHAIN-CURRENCY` is what
keeps it reproducible (see [[chain-currency-doctrine]]).

**No re-ingest was needed.** Only the evidence stage reads a document's normalized markdown bundle;
`SemanticIr::build`, `IntentIr::build`, and `AdapterArtifact::build` read only the persisted upstream JSON.
So all 78 downstream chains were rebuildable even though 56 documents have no bundle. The rebuild plus
re-validation of the previously-validated set took under three minutes.

ISF lowering blocks on exactly the two conditions in `assess_isf_renderability`
(`crates/specforge/src/ir/adapters.rs`): no signals in any interface, and no behavioral content — where
behavioral content means `temporal_rules`, `conditional_rules`, `signal_constraints`, or `control_blocks`
only. IntentIR `behaviors`, `constraints`, and `temporal_invariants` do **not** satisfy it, which is why a
document can carry 65 temporal invariants and still block honestly.
