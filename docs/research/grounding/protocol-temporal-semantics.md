# Literature grounding — Protocol & temporal semantics

*Aspect 4 of `LITERATURE-GROUNDING`. SpecForge's most domain-central layer: recovering
typed temporal/protocol behavior — handshakes, `temporal_rules` (antecedent→consequent,
`cycle_window`, `HandshakeComplete`, stability/eventually), FSMs. All citations
web-verified; none guessed.*

## Prior art (verified)

- **Temporal logic foundation** — A. Pnueli, *The Temporal Logic of Programs*, 18th IEEE
  FOCS, 1977 (DOI 10.1109/SFCS.1977.32). The basis of LTL; SpecForge's contract operators
  (`Eventually`, `Within`, `Stable`, `HandshakeComplete`) are LTL/MTL-shaped properties.
- **Specification mining (origin of the term)** — G. Ammons, R. Bodík, J. R. Larus,
  *Mining Specifications*, POPL 2002 (DOI 10.1145/503272.503275). Machine-learning to
  discover the formal **protocol** a system must obey, motivated by the fact that
  engineers rarely write formal specs by hand.
- **Likely-invariant detection** — M. Ernst et al., *Daikon* — *Dynamically Discovering
  Likely Program Invariants…*, ICSE 1999 / IEEE TSE 2001. Pattern-based inference of
  *likely* (not proven) invariants — the "propose with confidence, don't assert" stance.
- **LTL specification mining** — C. Lemieux, D. Park, I. Beschastnikh, *General LTL
  Specification Mining*, ASE 2015 (Texada). Mines LTL properties from traces using
  **`G(antecedent → consequent)` templates with "holes."**
- **Hardware assertion mining** — S. Vasudevan, D. Sheridan, S. Patel, D. Tcheng,
  B. Tuohy, D. Johnson, *GoldMine: Automatic assertion generation using data mining and
  static analysis*, DATE 2010. Generates high-coverage RTL assertions from `G(...)` LTL
  templates by combining data mining with static analysis.
- W. Li, *Specification Mining: New Formalisms, Algorithms and Applications*, UC Berkeley
  tech report EECS-2014-20. *Mining Secure Behavior of Hardware Designs*, arXiv:2108.09249
  (security-specific LTL templates).

## Alignment (where SpecForge already matches the literature)

SpecForge's `temporal_rules` surface — typed `antecedent → consequent` predicates with a
`cycle_window` bound and `HandshakeComplete` — is **exactly** the
`G(antecedent → consequent)`-with-holes property-template shape that GoldMine and Texada
formalized, and its operators are LTL/MTL (Pnueli). Its `automation_confidence` +
residual-honesty (propose, don't assert) is the Daikon "*likely* invariant" stance. So
SpecForge is, unknowingly, instantiating a well-established formalism — it should *adopt
the vocabulary* rather than keep an ad hoc one.

## Adopt (proven techniques worth borrowing)

- **Name SpecForge's temporal contract kinds as LTL/MTL templates** (Texada/GoldMine
  "holes"): a principled basis, and a path to interoperability with verification tools and
  the `.isf`/FSMGen downstream (standard property forms).
- **MTL for `cycle_window`**: SpecForge's "within N cycles" is bounded-until in Metric
  Temporal Logic — grounding `cycle_window` in MTL makes the bound formally precise.
- **Daikon's likely-vs-proven framing** validates SpecForge's confidence/residual model:
  mined properties are hypotheses, surfaced with confidence, not asserted as truth.
- **Spec-mining evaluation methodology** (precision/recall of mined properties vs gold):
  SpecForge's new `LLM-EXTRACTION-EVAL` already does this for constraints/relations —
  extend it to temporal rules.

## Extend / genuine novelty (the out-of-the-box part)

All of the above mine specifications **from traces, RTL, or code** — i.e. from an
*implementation* (dynamic execution or source). **SpecForge inverts the direction: it
mines protocol/temporal intent from the human-authored *specification* itself** (natural-
language PDF + tables + diagrams, multimodal), *before* any implementation exists. That
forward, spec→property recovery — and doing it fail-closed (unverifiable → residual,
never a fabricated assertion) — is the novel contribution; the trace/RTL mining literature
is complementary prior art for the *property formalism*, not for the *source*.

## Gaps / opportunities → candidate future trees

1. **Express `temporal_rules` in a standard LTL/MTL template form** (vs the current ad hoc
   typed shape) — verifiability + tool/FSMGen interoperability. Highest-leverage adopt.
2. **Ground `cycle_window` in MTL** bounded operators.
3. **Temporal-rule eval**: extend `LLM-EXTRACTION-EVAL` with a `temporal_rule` fact kind so
   mined temporal properties get a precision/recall score (mirrors spec-miner evaluation).

## Links

- Task-tree: `LITERATURE-GROUNDING`. Related SpecForge trees: `R16-WAVEFORM-CONTRACT-MINING`,
  `ISF-TEMPORAL-LOWERING`, `LLM-EXTRACTION-EVAL`, `R14-SIGNAL-RESOLVE`.
