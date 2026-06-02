# Literature grounding — Spec → hardware (RTL / assertions)

*Aspect 11 of `LITERATURE-GROUNDING`. SpecForge produces a backend-independent `IntentIR`
lowered to `.isf`, consumed downstream by FSMGen (which owns scheduling, `.fsm`, and HDL
generation). SpecForge itself stops at `.isf`; it does NOT generate RTL. All citations
web-verified; none guessed.*

## Prior art (verified)

- **IEEE Std 1850 — *Property Specification Language (PSL)*** (1850-2005; 1850-2010; also
  IEC 62531). Formal temporal-logic notation for design intent — assertions, constraints,
  coverage — bound to VHDL/Verilog/SystemVerilog/SystemC.
  <https://standards.ieee.org/ieee/1850/4383/>
- **IEEE Std 1800 — *SystemVerilog***. SystemVerilog Assertions (SVA, introduced 1800-2005;
  latest 1800-2023): sequences, properties, `assert`/`assume`/`cover`. Terminology:
  *concurrent vs immediate assertions*, *sequence*, *property*.
  <https://standards.ieee.org/ieee/1800/7743/>
- **"AssertLLM: Generating and Evaluating Hardware Verification Assertions from Design
  Specifications via Multi-LLMs"**, W. Fang et al., ASP-DAC 2025. DOI
  [10.1145/3658617.3697756](https://doi.org/10.1145/3658617.3697756);
  arXiv:[2402.00386](https://arxiv.org/abs/2402.00386). Multi-LLM pipeline: structural-spec
  extraction → signal mapping → SVA generation from spec text (and waveforms); 89%
  syntactically+functionally accurate on a 23-I/O design.
- **"Hybrid-NL2SVA: Integrating RAG and Finetuning for LLM-based NL2SVA"**, W. Xiao, D.
  Ekberg, S. Garg, R. Karri, MLCAD 2025. arXiv:[2506.21569](https://arxiv.org/abs/2506.21569).
  RAG + fine-tuning for NL → SVA; operator-aware retrieval and rechecking.
- **"QiMeng-CodeV-SVA: Training Specialized LLMs for Hardware Assertion Generation via
  RTL-Grounded Bidirectional Data Synthesis"**, 2026.
  arXiv:[2603.14239](https://arxiv.org/abs/2603.14239). RTL-grounded synthetic data;
  NL2SVA-Human / NL2SVA-Machine benchmarks. (Very recent; SOTA-trend, not seminal.)
- **J. Cong, B. Liu, S. Neuendorffer, J. Noguera, K. Vissers, Z. Zhang — "High-Level
  Synthesis for FPGAs: From Prototyping to Deployment"**, IEEE TCAD 30(4):473–491, 2011. DOI
  [10.1109/TCAD.2011.2110592](https://doi.org/10.1109/TCAD.2011.2110592). Canonical HLS
  survey (the algorithm/spec → RTL boundary).

## Alignment (where SpecForge already matches the literature)

SpecForge occupies the *upstream* half of this boundary: it produces a typed,
backend-independent `IntentIR` (contracts over actors/ports/temporal interaction) from spec
PDFs, then lowers to `.isf` for FSMGen. This mirrors PSL/SVA's role as the *intent formalism
between spec and implementation* — but SpecForge stops at intent and hands off, rather than
emitting RTL. Notably, AssertLLM's decomposition (structure extraction → signal mapping →
property emission) parallels SpecForge's existing extract → resolve stages.

## Adopt (proven techniques worth borrowing)

- **Align IntentIR temporal contracts with PSL/SVA property forms** (sequence/property,
  overlapping/non-overlapping implication `|->`/`|=>`, `always`/`eventually`/`until`) so
  contracts are nameable in an established vocabulary (reinforces the LTL/MTL grounding from
  aspect 4).
- **Borrow AssertLLM's decomposition** (structure extraction → signal mapping → typed-
  property emission) — it matches SpecForge's staged extract/resolve.
- **Classify each recovered obligation as `assert`/`assume`/`cover`** — the canonical IEEE
  1800/1850 classification of a property's role.

## Extend / genuine novelty (the out-of-the-box part)

NL2SVA/AssertLLM work generates assertions *directly* (often from RTL, or as terminal text)
with no audit trail. SpecForge instead **recovers intent from the spec to FEED a downstream
HW generator**, carrying *provenance* (PDF source spans) and *residual honesty* (e.g.
surfaced dropped rule-conflicts) — a typed, inspectable IR rather than free-floating SVA
strings.

## Gaps / opportunities → candidate future trees

1. **`.isf` → PSL/SVA assertion export** so IntentIR contracts become directly checkable in
   FSMGen/sim — closes the loop with the IEEE formalisms.
2. **Adopt NL2SVA-style benchmarks** (functional + syntactic accuracy) to score
   IntentIR → property fidelity.
3. **Classify each contract as assume/assert/cover** to map cleanly onto IEEE 1800/1850
   semantics.

## Links

- Task-tree: `LITERATURE-GROUNDING`. Related SpecForge trees: `ISF-TEMPORAL-LOWERING`,
  `ISF-RULE-CONFLICT-RESIDUAL`. Downstream: FSMGen (see the FSMGen handoff contract). See
  also aspect 4 (`protocol-temporal-semantics.md`).
