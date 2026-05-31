# Literature Grounding — verified references, corrections, and adopted techniques

> Owned by `INTENT-COMPLETENESS-RESEARCH.6`. Product of an 8-discipline parallel
> research workflow (`intent-completeness-lit-survey`, 9 agents) that verified
> references against DBLP/ACM/arXiv/Springer and synthesized them against the
> framework ([`intent-capture-completeness.md`](intent-capture-completeness.md)).
> Citations are reproduced as returned and marked with the surveyor's confidence;
> treat `high` as verified-locatable, `medium` as real-method-but-no-single-
> canonical-paper. This turns framework §11 from "starting points" into grounded,
> corrected design.

## Verdict: the core reframe is the literature's consensus

The survey **confirms** the framework's foundational moves:

- **No output-side recall oracle under open-world** → detect misses input-side /
  via structure / via redundancy. This is exactly the stance of the authoritative
  recent survey: *Razniewski, Arnaout, Ghosh, Suchanek, "Completeness, Recall, and
  Negation in Open-World Knowledge Bases: A Survey," ACM Computing Surveys 56(6)
  Art. 145, 2024 (arXiv:2305.05403).*
- **The four sources of a completeness notion** (source / invariants / redundancy
  / curated truth, framework §1) is a faithful decomposition — each maps to a real
  discipline (region accounting ↔ requirements traceability + document AI;
  invariants ↔ PCA/local-closed-world; redundancy ↔ capture–recapture; curated ↔
  competency questions + gold fixtures).
- **The denominator problem** (§2) is real: *Stanovsky & Dagan, "Creating a Large
  Benchmark for Open IE," EMNLP 2016* — Open IE could not report recall until an
  enumerated target set existed.
- **Residual-honesty / relative recall** (§0, §13) is the field's stance: report
  *relative* recall + explicit negative statements, never claim absolute recall.

## Verified key references (by instrument)

**Region accounting / backward traceability (§3):**
- *Gotel & Finkelstein, "An Analysis of the Requirements Traceability Problem,"
  ICRE 1994* — foundational **bidirectional** traceability; an orphan/dangling
  link in either direction = a coverage gap (the framework's forward+backward
  accounting).
- *Arora, Sabetzadeh, Briand, "…domain models for completeness checking of
  requirements," Empirical Software Engineering 24(4), 2019* — a domain model used
  as an **input-side completeness oracle** (closest RE analog to our §2 ontology).
- *Pfitzmann et al., "DocLayNet," KDD 2022 (arXiv:2206.01062)* + *Docling Technical
  Report (arXiv:2408.09869)* — a **closed, mutually-exclusive** layout taxonomy:
  every page region gets exactly one class. Directly licenses §3's "every region
  is intent / non-intent / deferred, no silent fourth bucket."
- *Smock, Pesala, Abraham, "PubTables-1M," CVPR 2022 (arXiv:2110.00061)* + *"GriTS,"
  ICDAR 2023 (arXiv:2203.12555)* — recall **denominated by the input/reference
  inventory**; PubTables' "blank cell is an explicit annotation" = our "a region
  must produce a fact OR an explicit no-intent tag."

**Closure invariants / local completeness (§6):**
- *Galárraga, Teflioudi, Hose, Suchanek, "AMIE," WWW 2013* (+ AMIE+ VLDBJ 2015,
  AMIE 3 ESWC 2020) — the **Partial Completeness Assumption (PCA)**: if a KB knows
  *some* object for r(x,·), assume it knows *all* — manufacturing negative evidence
  per-(subject,relation) without global closed-world.
- *Galárraga, Razniewski, Amarilli, Suchanek, "Predicting Completeness in Knowledge
  Bases," WSDM 2017 (arXiv:1612.05786)* — ground-truth-free per-(entity,relation)
  completeness prediction via **completeness signals**: PCA, the **star-pattern
  oracle** (co-occurring relations predict which facts should be present —
  e.g. a register row implies a reset value *and* an access type), cardinality/
  functionality, popularity.
- *Paulheim, "Knowledge Graph Refinement: A Survey," Semantic Web Journal 8(3),
  2017* — evaluation **without full ground truth**: silver standards, retrospective
  known-fact removal, sampling-with-judgment (the discipline for validating our
  miss-detector itself).

**Capture–recapture recall estimation (§8.1):**
- *Eick, Loader, Long, Votta, Vander Wiel, "Estimating Software Fault Content
  Before Coding," ICSE 1992* — the seminal transfer of biological capture–recapture
  to software inspection (our precedent: extractors as "inspectors").
- *Vander Wiel & Votta, "Assessing Software Designs Using Capture-Recapture
  Methods," IEEE TSE 19(11), 1993.*
- *Briand, El Emam, Freimut, Laitenberger, "A Comprehensive Evaluation of
  Capture-Recapture Models for Estimating Software Defect Content," IEEE TSE 26(6),
  2000* — which estimator, how many passes, and the **systematic underestimation**
  finding.
- *Petersson, Thelin, Runeson, Wohlin, "Capture-Recapture in Software Inspections
  after 10 Years Research," JSS 72(2), 2004* — best single entry point; model
  taxonomy.
- *Chao, "Estimating the Population Size for Capture-Recapture Data with Unequal
  Catchability," Biometrics 43(4), 1987* — the robust nonparametric **Mh
  estimator** `N̂ = D + f₁²/(2·f₂)` + CI for the **singleton-heavy** regime.

**Competency questions (§8.2):**
- *Grüninger & Fox, "Methodology for the Design and Evaluation of Ontologies,"
  IJCAI-95 Workshop* — canonical origin of **competency questions** *and*
  **completeness theorems** (formal conditions for when a CQ's answer set is
  complete).

**Information extraction recall (§5):**
- *Stanovsky & Dagan, EMNLP 2016* (denominator); *Ritter, Zettlemoyer, Mausam,
  Etzioni, "Modeling Missing Data in Distant Supervision," TACL 2013* — a latent-
  variable separation of **present-but-missed vs genuinely-absent** (a principled
  model for our region-accounting miss-vs-non-intent decision); *Sigletos et al.,
  JMLR 2005* — union of extractors maximizes recall, overlap feeds capture–recapture.

**Spec/property mining (§5 normative, §6 invariants):**
- *Ernst et al., "The Daikon system for dynamic detection of likely invariants,"
  Science of Computer Programming 2007* — template-instantiation-with-confidence.
- *Vasudevan et al., "GoldMine: Automatic assertion generation…," DATE 2010* —
  **mine-then-formally-filter** (mine candidates, keep only model-checker-proven).
- *Chockler, Kupferman, Vardi, "Coverage Metrics for Formal Verification," CHARME
  2003 (LNCS 2860)* — **mutation/sensitivity coverage** (see new techniques).

**Datasheet / closest prior art:**
- *Chen, Xu, Zhang, "D2S-FLOW: Automated Parameter Extraction from Datasheets for
  SPICE Model Generation Using LLMs," arXiv:2502.16540, 2025* — closest modern
  datasheet→machine-model work, and a **foil**: high reported accuracy but
  ground-truth-based, no input-side oracle. Contributes **HNEN** synonym/unit
  canonicalization (a precondition for our checks).
- *Bai, Bany Hamad, Suhaib, Ren (NVIDIA), "AssertionForge…," arXiv:2503.19174,
  2025* — **two-source (spec-KG + RTL-KG) fusion** recovers misses a spec-only pass
  makes — a genuinely *independent* second extractor.

## Corrections the literature forces on the framework (folded into the framework doc)

1. **Capture–recapture: bias direction + estimator.** The framework named only
   Lincoln–Petersen `N=ab/m`. Corrections: (a) correlated extractors (two LLM
   passes on one backbone) are *positively* correlated → **over-estimate recall /
   under-count misses** — the dangerous direction; require **≥3 heterogeneous**
   extractors (rule-based vs VLM vs different model family), not reseeds. (b) In the
   singleton-heavy regime chip extraction will hit, use **Chao 1987 Mh** + CI, not
   L-P. (c) Empirically capture–recapture *underestimates* content (Briand/El Emam
   2000; Petersson/Wohlin 2004) → report `recall_hat` as an **optimistic ceiling /
   misses as a lower bound**, assumptions printed.
2. **PCA / local-closed-world is a HEURISTIC, not exact.** Framework §6 called
   closure invariants "cheap, exact." Correction: PCA can **under-count** misses
   when a region is only partially extracted; it must be **gated by explicit
   per-relation cardinality/functionality** (port→direction, signal→width are
   functional ⇒ safe; "related constraints"/cross-references are open ⇒ unsafe,
   PCA would hide misses). *Some* invariants (register bit-tiling, symbol closure)
   remain exact; the PCA-style ones are heuristic-and-gated.
3. **Competency questions bound schema-answerability, not population.** §8.2:
   Grüninger–Fox completeness theorems assume the instance data is present; they
   detect "can the schema answer X," not "did we extract the data." **Pair CQs with
   population-completeness** (region accounting + PCA).
4. **Soft coverage scores can mask misses.** GriTS-style partial-credit matching
   can hide a miss as a low-similarity match. Keep a **hard binary "unaccounted
   region" flag** separate from any soft coverage percentage.
5. **Systematic blind spots are invisible to capture–recapture.** A fact missed by
   *all* passes contributes nothing to the overlap statistics — capture–recapture
   *cannot see it*. Only the **ontology coverage matrix** (empty cells, `.2`) and
   independent-source fusion catch systematic blind spots. The instruments are
   **complementary; none alone suffices** — this is now an explicit design law.

## New techniques to adopt (beyond the original framework)

- **Chao 1987 Mh estimator + CI** as the default recall gauge (L-P only as a 2-pass
  fallback); model-selection over the Mh/Mt/Mth/M0 hierarchy.
- **Mutation / sensitivity coverage** (Chockler-Kupferman-Vardi): a region is
  "covered" iff perturbing it changes ≥1 emitted fact. **Orthogonal** to backward-
  traceability — catches a region that has a forward link yet is fact-insensitive
  (a degenerate/ignored extraction). A new region-accounting detector.
- **Star-pattern completeness oracle** (WSDM 2017): co-occurring extracted facts
  predict which facts a region *should* yield (register row ⇒ reset value + access
  type) — a learnable/engineered version of our closure invariants.
- **Relative recall**: compare a region to similar peers; an under-populated region
  vs its siblings = likely miss. Distinct from capture–recapture and closure.
- **Two-source KG fusion** (AssertionForge): build an independent structural-source
  KG (e.g. from RTL when available) and fuse with the spec-KG — disagreements/
  unique-finds recover misses *and* supply a genuinely independent capture–recapture
  occasion (the cleanest answer to the independence problem).
- **Mine-then-formally-filter** (GoldMine): mine candidate invariants, keep only
  the ones a checker proves — raises invariant precision.
- **HNEN canonicalization** (D2S-FLOW): normalize synonyms/units *before* closure
  invariants and CQs, so terminology/unit drift neither masquerades as a miss nor
  hides one. A **precondition** step for §6/§8.
- **QuARS span-level defect markers** (Fabbrini et al., REFSQ 2001): deterministic
  word-level markers for weak/vague language ("may", "optionally") — finer-grained
  than region-level residuals.
- **STOP-OR-REINSPECT rule** (Briand/El Emam 2000): the capture–recapture estimate
  *drives a decision* — re-extract if estimated remaining misses exceed a threshold.
  **This gives the shipped `R15C-CONVERGENCE-REPORT` loop a principled statistical
  stopping criterion** instead of a fixed pass cap. A direct, high-value bridge from
  research to the code already in the repo.

## Net effect on the program

The framework survives the literature intact in its core reframe, is **sharpened**
on capture–recapture (Chao + heterogeneity + bias direction), **corrected** on PCA
exactness and CQ scope, and **extended** with mutation-coverage, star-pattern,
two-source fusion, and the STOP-OR-REINSPECT bridge. The prioritized backlog (`.7`)
will fold these in; the convergence loop already in the repo is the first place a
research result (STOP-OR-REINSPECT) lands.
