# Literature grounding — Multimodal evidence fusion

*Aspect 6 of `LITERATURE-GROUNDING`. SpecForge fuses evidence from multiple modalities —
prose text, structured tables, and figures/diagrams (timing diagrams, state machines)
read by a VLM — and represents cross-modal agreement and conflict explicitly. All
citations web-verified; none guessed.*

## Prior art (verified)

- **LayoutLM** — Y. Xu, M. Li, L. Cui, S. Huang, F. Wei, M. Zhou — "Pre-training of Text
  and Layout for Document Image Understanding", KDD 2020.
  arXiv:[1912.13318](https://arxiv.org/abs/1912.13318); DOI
  [10.1145/3394486.3403172](https://doi.org/10.1145/3394486.3403172). First joint
  *text+layout* document pre-training; term "Document AI."
- **LayoutLMv2** — Y. Xu et al. — "Multi-modal Pre-training for Visually-Rich Document
  Understanding", ACL-IJCNLP 2021. arXiv:[2012.14740](https://arxiv.org/abs/2012.14740).
  Adds an image stream + *text-image alignment/matching* cross-modal tasks.
- **LayoutLMv3** — Y. Huang, T. Lv, L. Cui, Y. Lu, F. Wei — "Pre-training for Document AI
  with Unified Text and Image Masking", ACM MM 2022.
  arXiv:[2204.08387](https://arxiv.org/abs/2204.08387); DOI
  [10.1145/3503161.3548112](https://doi.org/10.1145/3503161.3548112). MLM + MIM +
  *Word-Patch Alignment*.
- **Donut** — G. Kim et al. — "OCR-free Document Understanding Transformer", ECCV 2022.
  arXiv:[2111.15664](https://arxiv.org/abs/2111.15664). OCR-free VDU; avoids OCR-error
  propagation.
- **DocVQA** — M. Mathew, D. Karatzas, C.V. Jawahar, WACV 2021.
  arXiv:[2007.00398](https://arxiv.org/abs/2007.00398). Document VQA requiring structural
  understanding.
- **"A Survey on MLLM-based Visually Rich Document Understanding"**, 2025.
  arXiv:[2507.09861](https://arxiv.org/abs/2507.09861). Current MLLM-era VRDU landscape
  (terminology: *KIE*, *VRD-QA*, multimodal feature fusion).
- **A.P. Dempster — "Upper and Lower Probabilities Induced by a Multivalued Mapping"**,
  Annals of Mathematical Statistics 38(2):325–339, 1967. DOI
  [10.1214/aoms/1177698950](https://doi.org/10.1214/aoms/1177698950). Belief/plausibility
  intervals + a *rule for combining independent sources* (Dempster–Shafer theory of
  evidence).

## Alignment (where SpecForge already matches the literature)

SpecForge already implements the multimodal-fusion thesis these works motivate.
`ir/evidence.rs` defines a typed `EvidenceModality` (Text, TableTranscription, timing-diagram
/ state-machine VLM extraction) — directly the text+table+figure triad. Crucially it
surfaces *cross-modal disagreement* explicitly via `SignalPolarityConflictRecord` and
`SignalSemanticConflictRecord`, which stay visible instead of silently collapsing.
`ir/fusion.rs` clusters candidates by a typed `FusionKey` and, on incompatible
obligation/guard/kind, routes to a `Residual` rather than picking a winner.

## Adopt (proven techniques worth borrowing)

- **Layout-aware token embeddings** (LayoutLM 2D position) and **Word-Patch-Alignment**
  (v3) to tighten figure-region-to-signal-name grounding in VLM extraction.
- **Principled belief combination** (Dempster 1967 / belief–plausibility intervals) to
  compute `automation_confidence` as a fused belief mass rather than the current `min()`,
  with conflict mass *K* as a calibrated disagreement signal.

## Extend / genuine novelty (the out-of-the-box part)

SpecForge does *typed protocol-fact fusion with fail-closed conflict residuals*: it
produces no single fused answer when modalities disagree, emitting an auditable
`Residual{reason="disagreement:…"}`. LayoutLM*/Donut/DocVQA optimize a single most-probable
output and have no first-class conflict object. SpecForge's honesty doctrine inverts that —
unresolved cross-modal conflict is a preserved, gated artifact.

## Gaps / opportunities → candidate future trees

1. **Replace `min()`-confidence with a Dempster-rule combiner**; expose conflict mass *K*
   in `validate fusion:`.
2. **Mitigate Dempster's high-conflict pathology** (the Zadeh counterexample) via
   weighted/divergence combination before adopting the classic rule.
3. **Layout-aware figure-region → signal grounding pass** so VLM evidence carries 2D
   provenance, enabling spatial agreement checks against table cells.

## Links

- Task-tree: `LITERATURE-GROUNDING`. Related SpecForge trees: `R16-VLM-ENRICH`,
  `FUSION-CONFLICT-RESIDUAL`, `SIGNAL-POLARITY-CONFLICT`. Key files: `ir/fusion.rs`,
  `ir/evidence.rs`.
