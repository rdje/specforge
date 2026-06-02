# Literature grounding — Uncertainty representation & residual honesty

*Aspect 10 of `LITERATURE-GROUNDING`. SpecForge's residual-honesty doctrine: rather than
guess, it represents what it cannot confidently resolve as explicit residual-decision
packets, attaches confidence levels, fails closed, and never claims completeness. All
citations web-verified; none guessed.*

## Prior art (verified)

- **C.K. Chow — "On Optimum Recognition Error and Reject Tradeoff"**, IEEE Trans.
  Information Theory 16(1):41–46, 1970. DOI
  [10.1109/TIT.1970.1054406](https://doi.org/10.1109/TIT.1970.1054406). Founding
  *reject-option* rule; the *error–reject tradeoff*.
- **R. El-Yaniv, Y. Wiener — "On the Foundations of Noise-free Selective Classification"**,
  JMLR 11:1605–1641, 2010.
  <https://jmlr.org/papers/v11/el-yaniv10a.html>. Formalizes *selective classification* and
  the *risk–coverage tradeoff*.
- **Y. Geifman, R. El-Yaniv — "Selective Classification for Deep Neural Networks"**,
  NeurIPS 2017. arXiv:[1705.08500](https://arxiv.org/abs/1705.08500). Guaranteed-risk
  *selective classifier* at a chosen coverage.
- **V. Vovk, A. Gammerman, G. Shafer — *Algorithmic Learning in a Random World***, Springer,
  2005 (2nd ed. ISBN 9783031066481). *Conformal prediction*: distribution-free valid
  prediction *sets* with an explicit error rate.
- **A. Angelopoulos, S. Bates — "A Gentle Introduction to Conformal Prediction and
  Distribution-Free Uncertainty Quantification"**, 2021.
  arXiv:[2107.07511](https://arxiv.org/abs/2107.07511). Practical *split-conformal*
  calibration on any pretrained model.
- **C. Guo, G. Pleiss, Y. Sun, K. Weinberger — "On Calibration of Modern Neural Networks"**,
  ICML 2017. arXiv:[1706.04599](https://arxiv.org/abs/1706.04599). Modern nets are
  miscalibrated; *temperature scaling* fixes it.
- **W. Scheirer, A. Rocha, A. Sapkota, T. Boult — "Toward Open Set Recognition"**, IEEE
  TPAMI 35(7):1757–1772, 2013. DOI
  [10.1109/TPAMI.2012.256](https://doi.org/10.1109/TPAMI.2012.256). *Open-set / known-
  unknowns* formalization.

## Alignment (where SpecForge already matches the literature)

SpecForge embodies the reject-option philosophy: rather than force a label, it **abstains
into explicit residual-decision packets** — exactly Chow's "reject" region and El-Yaniv's
"non-coverage." Its `AutomationConfidence::{High, Medium, Low}` tiers are a discretized
confidence estimate, and `ir/completeness.rs` runs **capture–recapture** to bound remaining
misses — an open-world stance matching Scheirer's known-unknowns. Fail-closed +
never-claim-perfection mirrors selective classification's coverage-for-accuracy contract.

## Adopt (proven techniques worth borrowing)

- **Split-conformal calibration (Angelopoulos & Bates)** of the LLM/VLM tier: on a held-out
  labeled spec set, convert raw scores into a per-fact-kind threshold so "High confidence"
  carries a *distribution-free* error guarantee instead of an ordinal label.
- **Temperature scaling (Guo)** on the LLM tier's probability outputs before thresholding —
  cheap post-hoc recalibration.
- **Explicit risk–coverage curves (El-Yaniv/Geifman)** — report residual rate vs extraction
  error as a tunable knob, letting users set a target risk.
- **Chow framing** in the docs — name the residual rule the "optimum error–reject tradeoff."

## Extend / genuine novelty (the out-of-the-box part)

SpecForge goes beyond the literature's *scalar* reject/score: residuals are **first-class
typed IR objects (decision packets)** carrying provenance and the unresolved alternatives —
a structured, auditable abstention, not a dropped sample. Its **domain closure-invariant
miss detection** (signals must close against port/table universes) is a domain-specific
open-world test absent from generic open-set recognition, and **capture–recapture recall
lower bounds** quantify *unseen* misses rather than only rejecting seen-but-uncertain inputs.

## Gaps / opportunities → candidate future trees

1. **Calibrate confidence tiers to empirical accuracy** and publish a reliability diagram /
   ECE (per Guo) — tiers are currently heuristic.
2. **Wrap the LLM relation/signal tiers in a conformal layer** emitting *prediction sets*
   that feed residual packets.
3. **Coverage targeting** — let users pick a max error and auto-tune the abstention
   threshold (Geifman's guaranteed-risk selection).
4. **Validate capture–recapture assumptions** (Chao-style estimators) against held-out
   ground-truth specs.

## Links

- Task-tree: `LITERATURE-GROUNDING`. Related SpecForge trees: `INTENT-COMPLETENESS-RESEARCH`
  (CLOSED), `ISF-RULE-CONFLICT-RESIDUAL`, `COMPLETENESS-RECALL-RELATIONS`. Key files:
  `ir/evidence.rs` (confidence tiers), `ir/completeness.rs` (recall bounds). See also
  aspect 7 (`neuro-symbolic-bounded-llm.md`).
