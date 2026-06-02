# Literature grounding — Requirements & specification intent extraction

*Aspect 3 of `LITERATURE-GROUNDING`. SpecForge recovers normative requirements/constraints
from natural-language spec prose (e.g. "PADDR must remain stable until the transfer
completes") into typed constraints. All citations web-verified; none guessed.*

## Prior art (verified)

- **S. Bradner — "Key words for use in RFCs to Indicate Requirement Levels"**, RFC 2119,
  BCP 14, IETF, 1997 — updated by **RFC 8174** (2017, restricts keyword force to
  uppercase). Canonical MUST/SHALL/SHOULD/MAY modal-keyword vocabulary.
  <https://www.rfc-editor.org/rfc/rfc2119.html>
- **L. Zhao, W. Alhoshan, A. Ferrari, K. Letsholo, et al. — "Natural Language Processing
  for Requirements Engineering: A Systematic Mapping Study"**, ACM Computing Surveys 54(3),
  2021. DOI [10.1145/3444689](https://doi.org/10.1145/3444689); preprint
  arXiv:[2004.01099](https://arxiv.org/abs/2004.01099). The 404-study **NLP4RE** landscape.
- **J. Cleland-Huang, R. Settimi, X. Zou, P. Solc — "Automated classification of
  non-functional requirements"**, Requirements Engineering 12(2):103–120, 2007. DOI
  [10.1007/s00766-007-0045-1](https://doi.org/10.1007/s00766-007-0045-1). Origin of the
  **PROMISE NFR** dataset and IR-based FR/NFR classification.
- **T. Hey, J. Keim, A. Koziolek, W. Tichy — "NoRBERT: Transfer Learning for Requirements
  Classification"**, IEEE RE 2020, pp. 169–179. DOI
  [10.1109/RE48521.2020.00028](https://doi.org/10.1109/RE48521.2020.00028). BERT
  fine-tuning, ~94% F1 on PROMISE incl. unseen projects.
- **N. Fuchs, R. Schwitter — "Attempto Controlled English (ACE)"**, 1996.
  arXiv:[cmp-lg/9603004](https://arxiv.org/abs/cmp-lg/9603004). Controlled NL → first-order
  logic for unambiguous, executable specs.
- **D. Berry, E. Kamsties, M. Krieger — "From Contract Drafting to Software Specification:
  Linguistic Sources of Ambiguity — A Handbook"**, Univ. of Waterloo, 2003.
  <https://cs.uwaterloo.ca/~dberry/handbook/ambiguityHandbook.pdf>. Ambiguity taxonomy.
- **W. Wilson, L. Rosenberg, L. Hyatt — "Automated Analysis of Requirement
  Specifications"**, ICSE 1997, pp. 161–171. DOI
  [10.1145/253228.253258](https://doi.org/10.1145/253228.253258). NASA ARM tool: imperatives
  (shall/must) as quality indicators; weak-phrase detection.

## Alignment (where SpecForge already matches the literature)

SpecForge already implements the core NLP4RE pattern: it lifts modal/imperative prose into
typed normative constraints. `SignalConstraintKind` (`MustBeStable`, `MustNotChange`,
`MustHoldData`, `MustBeAsserted/Deasserted`) and `TemporalInvariantKind` (`MustRemainStable`,
`MustMatch`) are the chip-domain analog of RFC 2119 MUST-level obligations; the `Obligation`
enum and `EvidenceModality` mirror deontic modality. The residual-decision mechanism
(`normative_visual` interpretation; `ISF-RULE-CONFLICT-RESIDUAL`) is exactly the
ambiguity-surfacing Berry & Kamsties advocate — flag, don't silently resolve.

## Adopt (proven techniques worth borrowing)

- **RFC 8174's uppercase-only rule** as an explicit knob — distinguish normative `MUST`
  from prose "must" to cut false positives.
- **NoRBERT-style transfer-learning evaluation discipline** — measure F1 on *unseen* specs
  (cross-protocol generalization), not just held-out sentences.
- **Wilson/ARM weak-phrase indicators** ("as appropriate", "if possible", "etc.") as an
  ambiguity-flag lexicon feeding residual decisions.

## Extend / genuine novelty (the out-of-the-box part)

SpecForge goes beyond generic FR/NFR labeling: it performs **typed protocol-semantic
lowering** (drive/stable/handshake-barrier/eventually with temporal windows) over
signals/ports, and is **multimodal** (VLM diagram + prose fusion). PROMISE/NoRBERT stop at a
category label; ACE requires authors to write in a controlled subset — SpecForge instead
*recovers* structure from uncontrolled vendor prose.

## Gaps / opportunities → candidate future trees

1. **PROMISE-style labeled corpus of chip-spec normative sentences**; report
   precision/recall vs. a NoRBERT baseline (extends the new eval harness).
2. **Berry-Kamsties weak-phrase/ambiguity classifier** that routes flagged sentences to
   residual decisions rather than silently dropping them.
3. **Model SHOULD/MAY (defeasible) vs MUST distinctly** in the IR — current `Obligation`
   treats most constraints as hard.

## Links

- Task-tree: `LITERATURE-GROUNDING`. Related SpecForge trees: `CONSTRAINT-SUBJECT-PRECISION`,
  `ISF-RULE-CONFLICT-RESIDUAL`, `LLM-EXTRACTION-EVAL`. See also aspect 4
  (`protocol-temporal-semantics.md`).
