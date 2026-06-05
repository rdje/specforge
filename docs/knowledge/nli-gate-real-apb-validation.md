---
id: nli-gate-real-apb-validation
title: NLI gate validated on the real AMBA APB spec — it works, and it exposes constraint over-generation
answers:
  - "does the NLI verifier actually catch real extraction errors"
  - "what did running nli-verify on a real spec find"
  - "is SpecForge's constraint extraction over-generating"
  - "why does condition_text matter for the NLI claim"
date: 2026-06-05
tags: [nli, grounding, validation, extraction-quality, apb]
evidence: crates/specforge/src/ir/nli_verify.rs
reverify: cargo run -p specforge --quiet -- nli-verify generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json --vlm-provider ollama --model qwen2.5:14b-instruct
---

Running `nli-verify` on the **real** AMBA APB EvidenceIR (`ihi0024_e`, 42 `signal_constraints`,
qwen2.5:14b-instruct, ~42 s) flagged **36/42** not-entailed on first run, **33/42** after the
`condition_text` fix (`[[nli-entailment-verifier]]`). Two conclusions:

1. **The gate works on real data** — most flags are *correct* catches of real extraction noise:
   - **non-signals constrained:** `ACCESS`/`SETUP` (protocol *states*), `USER_REQ_WIDTH`/
     `USER_DATA_WIDTH`/`USER_RESP_WIDTH` (*width parameters*), `PCLK` (*the clock*) all got
     spurious "must be stable/VALID" records;
   - **condition signals constrained:** "PSEL must be VALID" extracted from *"when PSEL is
     asserted, the following signals must be valid"* — PSEL is the condition, the obligation is on
     the *other* signals (the condition-vs-obligation error the gate targets).
   So the run **validates the gate AND exposes that the constraint extractor over-generates** —
   that is a real extraction-quality finding worth its own future investigation (the deterministic
   backbone keeps it from reaching `.isf`, but the EvidenceIR `signal_constraints` are noisy).

2. **A precision fix it surfaced:** `constraint_claim_text` originally dropped the constraint's
   `condition_text`, so a *legitimately*-conditional constraint ("PSTRB must be LOW" from *"for
   read transfers, drive PSTRB LOW"*) read as not-entailed purely because the claim was
   unconditional. Carrying `condition_text` into the claim ("PSTRB must be LOW for read transfers")
   raised precision (36→33) without losing the correct catches (a mis-attributed condition signal
   still fails). `NLI-CLAIM-CONDITION`.
