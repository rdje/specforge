---
id: nli-intent-gate
title: NLI intent gate — active demote-to-residual of un-entailed contracts (intent --nli-verify)
answers:
  - "how do I make the NLI verifier actively change extraction / demote claims"
  - "what does intent --nli-verify do"
  - "where does the NLI gate route a not-entailed contract"
  - "how is the NLI gate tested without Ollama"
date: 2026-06-05
tags: [nli, grounding, residual, intent-ir, hallucination]
evidence: crates/specforge/src/ir/nli_verify.rs
reverify: grep -n "fn apply_nli_gate\|fn nli_gate_contracts\|fn obligation_claim_text" crates/specforge/src/ir/nli_verify.rs
---

The **active** form of the NLI verifier runs at the **IntentIR stage** (path b — that is where
`residual_decisions` already exist; EvidenceIR has no residuals home). It is a **post-build pass**
(no `IntentIr::build` signature change):

- `obligation_claim_text(&ActorContract) -> Option<String>` renders a contract's obligation as an
  NLI hypothesis **only when phrasable** (`Drive`→"S must be V", `Stable`→"S must be stable",
  `Eventually{signal,Within max}`→"S must occur within N cycles", `HandshakeBarrier`, `Mutex`);
  `Observe`/`Persist`/`Sequence`/`OrderedBefore`/non-signal `Eventually` → `None` = **not gated**
  (never NLI-check a claim we cannot state cleanly).
- `nli_gate_contracts(contracts, verify) -> (kept, new_residuals)` — premise =
  `contract.provenance.source_text`. A `NotEntailed` contract is **demoted**: removed from
  `actor_contracts`, returned as a `ResidualDecisionPacket` (`packet_id = nli_unentailed_<id>`).
  `Entailed`/`Unknown`/un-phrasable → kept.
- `apply_nli_gate(&mut IntentIr, verify)` extends `residual_decisions` and returns the demote
  count. Verifier **injected** → hermetic (3 tests, no provider).

Wired as opt-in **`specforge intent <semantic_ir.json> --nli-verify`** (`--vlm-provider`,
`--model`; `skip` no-ops). **Demote-not-delete:** a wrong verdict costs a review item, not a lost
fact; the gate can demote a borderline contract but can never invent one. Default model
`qwen2.5:14b-instruct`. Builds on `[[nli-entailment-verifier]]` /
`[[local-llm-for-text-reasoning]]`. See `docs/tasks/NLI-INTENT-GATE.md`.
