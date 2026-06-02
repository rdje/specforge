# Literature grounding — Information-extraction evaluation methodology

*Aspect 9 of `LITERATURE-GROUNDING`. SpecForge scores extraction with labeled
precision/recall/F1 (closed-world per labeled statement) and estimates recall via
capture–recapture (Lincoln–Petersen) across two independent extractor tiers. All citations
web-verified; none guessed.*

## Prior art (verified)

- **C.J. van Rijsbergen — *Information Retrieval*, 2nd ed.**, Butterworths, 1979.
  ISBN 9780408709293. Origin of the effectiveness measure *E*; the **F-measure** is
  *F = 1 − E* — the precision/recall trade-off underpinning all IE scoring. (Pre-DOI book;
  ISBN is the resolvable id.)
- **N. Chinchor, B. Sundheim — "MUC-5 Evaluation Metrics"**, Proc. 5th Message Understanding
  Conference, ACL, 1993. DOI [10.3115/1072017.1072026](https://doi.org/10.3115/1072017.1072026).
  The MUC lineage that standardized **slot-based precision/recall/F against a human answer
  key**.
- **J. Cohen — "A Coefficient of Agreement for Nominal Scales"**, Educational and
  Psychological Measurement 20(1):37–46, 1960. DOI
  [10.1177/001316446002000104](https://doi.org/10.1177/001316446002000104). Cohen's **κ**.
- **K. Krippendorff — *Content Analysis: An Introduction to Its Methodology***, SAGE
  (1st ed. 1980; 4th ed. 2018, ISBN 9781506395661). Source of **Krippendorff's α**.
- **R. Artstein, M. Poesio — "Inter-Coder Agreement for Computational Linguistics"**,
  Computational Linguistics 34(4):555–596, 2008. DOI
  [10.1162/coli.07-034-R2](https://doi.org/10.1162/coli.07-034-R2). Canonical NLP guide to
  κ/π/α.
- **A. Chao — "Estimating the Population Size for Capture-Recapture Data with Unequal
  Catchability"**, Biometrics 43(4):783–791, 1987. DOI
  [10.2307/2531532](https://doi.org/10.2307/2531532). The **Chao estimator**, robust to
  heterogeneous catchability.
- **S.G. Eick, C.R. Loader, M.D. Long, L.G. Votta, S.A. Vander Wiel — "Estimating Software
  Fault Content Before Coding"**, Proc. 14th ICSE, 1992, pp. 59–65. DOI
  [10.1145/143062.143090](https://doi.org/10.1145/143062.143090). First **capture–recapture
  for software inspection** (estimating *undiscovered* faults).
- **H. Petersson, T. Thelin, P. Runeson, C. Wohlin — "Capture–recapture in software
  inspections after 10 years research"**, Journal of Systems and Software 72(2):249–264,
  2004. DOI [10.1016/S0164-1212(03)00090-6](https://doi.org/10.1016/S0164-1212(03)00090-6).
  Survey of estimators (Lincoln–Petersen *Mₜ*, Chao).

## Alignment (where SpecForge already matches the literature)

SpecForge already matches the tradition: `eval.rs` computes **closed-world
precision/recall/F1** per labeled statement (MUC/van Rijsbergen line); `ir/completeness.rs`
runs a **2-source Lincoln–Petersen** capture–recapture across the Pattern and Nlp tiers
(N̂ = |a|·|b|/m), correctly returning `None` when m=0 (undefined) rather than fabricating
"0 misses" — exactly the honesty Petersson et al. demand.

## Adopt (proven techniques worth borrowing)

- **Chao's estimator** as a second N̂ alongside Lincoln–Petersen: the two tiers share prose
  input (positive dependence → LP under-estimates), and Chao tolerates unequal catchability;
  report both as a range.
- **Inter-annotator agreement (κ or Krippendorff's α)** on the agent-drafted gold labels —
  quantify and report label reliability, since the gold is machine-drafted, not human
  consensus.
- **MUC partial-match credit** for approximately-correct slots, instead of strict exact-key
  matching.

## Extend / genuine novelty (the out-of-the-box part)

SpecForge applies capture–recapture across **automated extractor tiers**, not human
inspectors (Eick/Petersson assume independent human reviewers) — a defensible reframing,
though tier independence is weaker and must be stated. Its **closed-world-per-labeled-
statement** scoring is a deliberate departure from MUC's per-document answer key, sidestepping
open-world recall penalties for unlabeled facts.

## Gaps / opportunities → candidate future trees

1. **Confidence intervals on N̂** (Chao gives a variance) instead of a point estimate.
2. **Quantify tier dependence**; LP/Chao both assume independence — add a 3rd source (e.g.
   the VLM tier) to enable closed-population *Mₜ* models and detect violations.
3. **Add κ/α agreement** to the eval harness and a **partial-match scorer**.

## Links

- Task-tree: `LITERATURE-GROUNDING`. Related SpecForge trees: `LLM-EXTRACTION-EVAL`,
  `INTENT-COMPLETENESS-RESEARCH` (CLOSED), `COMPLETENESS-RECALL-RELATIONS`. Key files:
  `src/eval.rs`, `ir/completeness.rs`.
