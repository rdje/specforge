# Adopt / defer ledger — what SpecForge takes from each author, and what it leaves out

*Tree: `SPEC-MINING-PROVENANCE`. This sharpens the per-aspect grounding
(`docs/research/grounding/*.md`) into per-**author** provenance. For each leveraged
author/work: **Take** (the abstraction SpecForge adopts), **Leave out (+why)** (what of their
work SpecForge deliberately does not use, and the reason — so it is a recorded decision, not
an oversight), and **Instantiated at** (where the adopted idea lives in SpecForge). Every
citation is reused from the verified per-aspect grounding docs; none is invented.*

## SpecForge is *specification mining* — run forward

**Specification mining** (Ammons, Bodík & Larus, *Mining Specifications*, POPL 2002,
DOI [10.1145/503272.503275](https://doi.org/10.1145/503272.503275)) is the discipline of
**automatically discovering the formal specification a system obeys**, motivated by the fact
that engineers rarely write formal specs by hand. The literature runs it **backward**:
recover a spec *from an implementation* — execution traces, RTL, or source code.

**SpecForge runs it forward.** Its input is the *human-authored specification document*
(PDF prose + tables + figures); it mines typed design **intent** *from the spec itself*,
before any implementation exists. So SpecForge is **forward specification mining** —
spec → intent, not implementation → spec. That direction is the genuine novelty; the
abstractions below are what SpecForge borrows from the (mostly backward) spec-mining
literature and recombines for the forward case.

---

## Temporal trio — the `G(antecedent → consequent)` property template

### Pnueli — *The Temporal Logic of Programs*, 18th IEEE FOCS, 1977 (DOI [10.1109/SFCS.1977.32](https://doi.org/10.1109/SFCS.1977.32))

- **Take.** **LTL** as the formalism for temporal/protocol behaviour — the operators `G`
  (globally/always), `F` (eventually), `X` (next), and the `G(antecedent → consequent)`
  invariant shape. SpecForge's temporal contracts *are* LTL/MTL-shaped properties.
- **Leave out (+why).** Full LTL with arbitrary operator nesting (`U` until, `W` weak-until,
  `R` release) and, above all, temporal-logic **model checking / satisfiability**. SpecForge
  uses a *restricted* `G(ante → X / F[min,max] cons)` template and does **not** check or prove
  anything. *Why:* the restricted template covers the dominant chip-spec timing-obligation
  shape and stays grounded in what the spec actually states; proving properties is the
  downstream verifier's (FSMGen / sim) job, not the miner's.
- **Instantiated at.** `TemporalRuleRecord` (`crates/specforge/src/ir/semantic.rs`); the LTL
  renderer `crates/specforge/src/ir/temporal_ltl.rs` (`G` / `X` / `F[min,max]`);
  `TemporalPredicateRecord`. See `protocol-temporal-semantics.md`, `temporal-rule-ltl-rendering`.

### Vasudevan, Sheridan, Patel, Tcheng, Tuohy & Johnson — *GoldMine*, DATE 2010

- **Take.** The `G(antecedent → consequent)` LTL **assertion template with holes** as the unit
  of mined behaviour; automatic high-coverage assertion generation; and **precision/recall
  evaluation of mined assertions** against a reference.
- **Leave out (+why).** GoldMine mines from **RTL + simulation traces** via *data mining*
  (decision trees) + static analysis of the design. SpecForge mines from the **spec**
  (forward), with no RTL/traces and no machine-learning-over-runs — deterministic patterns +
  a bounded LLM over prose. *Why:* at SpecForge's stage no implementation/traces exist; the
  novelty is forward (spec → property).
- **Instantiated at.** `temporal_rules` as `G(ante→cons)` templates; the precision/recall
  methodology is realized as the `TEMPORAL-RULE-EVAL` tree (supervised P/R/F1 of the mined
  temporal rules).

### Lemieux, Park & Beschastnikh — *Texada: General LTL Specification Mining*, ASE 2015

- **Take.** **LTL property templates with "holes"** instantiated from a corpus — mining
  *general* LTL properties via templates rather than a handful of fixed patterns.
- **Leave out (+why).** Texada mines from execution **traces/logs** with **user-supplied,
  arbitrary** LTL templates + support/confidence thresholds. SpecForge uses a **fixed,
  domain-typed** template set (drive / stable / valid / sample / handshake) over spec
  **prose** — not arbitrary user templates over traces, and no support/confidence over a
  trace corpus. *Why:* forward (spec, not traces); a fixed protocol-domain ontology keeps the
  mined properties typed and grounded; the "corpus" is one document's prose, not a log set.
- **Instantiated at.** The typed `TemporalPredicateRecord` template set + the
  `G(ante→cons)`-with-holes shape; `ir/temporal_ltl.rs`.

### Ammons, Bodík & Larus — *Mining Specifications*, POPL 2002 (method) · Daikon (Ernst et al., ICSE 1999 / IEEE TSE 2001)

- **Ammons (method).** *Take:* the discipline + framing (above). *Leave out (+why):* the
  *method* — learning an automaton from dynamic execution **traces** via ML. SpecForge has no
  traces; it reads the static spec. *Instantiated at:* the whole forward framing.
- **Daikon.** *Take:* the **"likely invariant"** stance — propose invariants *with confidence*,
  not as proven truth. *Leave out (+why):* dynamic invariant detection from program runs (no
  runs at SpecForge's stage). *Instantiated at:* `automation_confidence` + residual-honesty
  (propose, don't assert) across the IR.

---

## Document understanding — the PDF → structured-evidence front end

*(per-aspect grounding: `document-extraction.md`)*

- **Docling** (Auer et al. 2024, arXiv:2408.09869 / 2501.17887). *Take:* the production
  PDF→structured-document toolkit (layout analysis + table-structure recovery), pinned
  `docling==2.84.0`, as the ingest backbone. *Leave out (+why):* nothing of its core —
  SpecForge does **not** reinvent document AI; it uses the leading open toolkit as-is.
  *Instantiated at:* `ir/source/docling_backend.rs`, `bootstrap_docling.sh` (+ KM
  `docling-device-cpu`).
- **TableFormer** (Nassar et al., CVPR 2022, arXiv:2203.01017). *Take:* transformer
  table-structure recovery (borderless/empty cells, spans, hierarchical headers), via Docling.
  *Leave out (+why):* training/fine-tuning it (consumed through Docling). *Instantiated at:*
  `structured_tables`.
- **DocLayNet** (Pfitzmann et al., KDD 2022, arXiv:2206.01062). *Take:* the layout-class
  taxonomy + the dataset Docling's layout model trains on. *Leave out (+why):* re-training a
  layout model. *Instantiated at:* via Docling layout analysis.
- **PubTables-1M / GriTS / TEDS** (Smock et al., CVPR 2022, arXiv:2110.00061). *Take:* the
  **functional-analysis** framing (header vs data roles) informing table classification.
  *Leave out (for now, +why):* adopting **TEDS/GriTS** as an ingest-stage table-fidelity
  metric — flagged future tree (no supervised table-structure metric yet). *Instantiated at:*
  informs `classify_table_kind`; the metric is deferred.

## Knowledge-graph & relation extraction

*(per-aspect grounding: `knowledge-graph-relation-extraction.md`)*

- **Banko et al. — OpenIE / TextRunner** (IJCAI 2007). *Take:* the **(arg1, relation, arg2)
  triple** as the unit of extraction. *Leave out (+why):* the **open** relation vocabulary —
  SpecForge uses a *closed, behavioral* relation set (`Drives`/`Reads`); protocol relations are
  a fixed ontology, not open. *Instantiated at:* `ActorSignalRelation` (`R14-SIGNAL-RESOLVE`).
- **Fader et al. — ReVerb** (EMNLP 2011). *Take:* verb-mediated lexical/syntactic relation
  patterns. *Leave out (+why):* open relation phrases. *Instantiated at:* the Tier-2 verb-pattern
  relation extractor.
- **Mintz et al. — distant supervision** (ACL 2009). *Take:* the distant-supervision idea
  (KB facts → auto-labels) as the stated Level-4 path. *Leave out (for now, +why):* training a
  distant-supervised model — deferred until enough human-reviewed triples exist.
  *Instantiated at:* planned; the seed corpus framed as distant-supervision data.
- **Zeng et al. — PCNN** (EMNLP 2015). *Take:* (reference) neural multi-instance RE.
  *Leave out (+why):* neural RE entirely — SpecForge is deterministic + bounded-LLM, with no
  trained RE model (no training data/infra at this stage). *Instantiated at:* none yet.
- **Ji & Grishman — KBP** (ACL 2011). *Take:* per-relation slot-filling P/R/F1 + provenance
  discipline. *Leave out (for now, +why):* the full KBP pipeline + a per-relation gold scorer
  (flagged). *Instantiated at:* relation provenance (`source_statement_ids`).
- **Hogan et al. — Knowledge Graphs** (ACM CSUR 2021). *Take:* KG schema/ontology hygiene +
  provenance practice. *Leave out (+why):* KG embeddings/deductive reasoning. *Instantiated at:*
  the typed protocol ontology (`R16-KG-PROTOCOL-ONTOLOGY`).

## Multimodal evidence fusion

*(per-aspect grounding: `multimodal-fusion.md`)*

- **LayoutLM v1–v3** (Xu / Huang et al., KDD 2020 / ACL 2021 / ACM MM 2022). *Take:* the
  principle that text + layout (+ image) must be modelled jointly; layout-aware grounding as a
  *reference* for figure-region→signal grounding. *Leave out (for now, +why):* adopting a
  LayoutLM model — SpecForge uses Docling layout + a VLM, not LayoutLM (a 2D-grounding pass is a
  flagged future tree). *Instantiated at:* multimodal `EvidenceModality`.
- **Donut** (Kim et al., ECCV 2022, arXiv:2111.15664). *Take:* (reference) the OCR-free VDU
  idea (avoid OCR-error propagation). *Leave out (+why):* adopting Donut. *Instantiated at:*
  none (reference).
- **DocVQA** (Mathew et al., WACV 2021, arXiv:2007.00398). *Take:* (context) document-structural
  QA framing. *Leave out (+why):* SpecForge is not QA. *Instantiated at:* none (context).
- **Dempster** (Dempster–Shafer, 1967, DOI 10.1214/aoms/1177698950). *Take:* (reference)
  principled belief combination + conflict mass for fusing modalities. *Leave out (for now,
  +why):* replacing the `min()` confidence with a Dempster-rule combiner (flagged; needs a
  high-conflict-pathology guard). *Instantiated at:* cross-modal conflict is surfaced as a
  residual today; the combiner is deferred.

## Neuro-symbolic / bounded-LLM extraction

*(per-aspect grounding: `neuro-symbolic-bounded-llm.md`)*

- **Grammar-Constrained Decoding** (Geng et al., EMNLP 2023, arXiv:2305.13971) + **Outlines**
  (Willard & Louf, arXiv:2307.09702). *Take:* schema/grammar-constrained LLM output (validity
  by construction). *Leave out (for now, +why):* moving grammar enforcement *into* decoding —
  SpecForge does post-hoc schema rejection (fail-closed) today (flagged). *Instantiated at:* the
  JSON-schema-constrained CVE surface (`R16-CONSTRAINED-VERIFIED-EXTRACTION`).
- **RAG** (Lewis et al., NeurIPS 2020, arXiv:2005.11401). *Take:* grounding generated claims in
  retrieved source spans (provenance). *Leave out (+why):* a full retrieval-augmented
  pipeline. *Instantiated at:* every LLM claim bound to its source statement.
- **SNLI / NLI** (Bowman et al., EMNLP 2015, arXiv:1508.05326). *Take:* the entailment framing
  (premise=source, hypothesis=claim → keep only ENTAILMENT). *Leave out (for now, +why):* an
  actual NLI model — the grounding gate is rule/string-based; an NLI verifier is a flagged
  future tree. *Instantiated at:* the entailment/grounding gate.
- **Ji et al. — hallucination survey** (ACM CSUR 2023, DOI 10.1145/3571730). *Take:* the
  intrinsic-vs-extrinsic taxonomy → "drop ungrounded to residual". *Leave out (for now, +why):*
  calibrating the drop threshold against a measured hallucination rate (flagged).
  *Instantiated at:* fail-closed-to-residual.
- **SelfCheckGPT** (Manakul et al., EMNLP 2023, arXiv:2303.08896). *Take:* (reference)
  sample-consistency hallucination detection. *Leave out (for now, +why):* multi-sample
  self-consistency (deferred). *Instantiated at:* none yet.
- **Garcez & Lamb — neurosymbolic** (AI Review 2023, DOI 10.1007/s10462-023-10448-w). *Take:*
  the neural-propose + symbolic-verify architecture. *Leave out (+why):* heavier neuro-symbolic
  reasoning. *Instantiated at:* the bounded-LLM + deterministic-backbone split (the core
  design).

## Cross-document learning

*(per-aspect grounding: `cross-document-learning.md`)*

- **Yarowsky** (ACL 1995) + **Riloff & Jones** (AAAI 1999). *Take:* confidence-gated
  bootstrapping ("retain only reliable entries"); one-sense-per-discourse. *Leave out (+why):*
  unrestricted self-training (semantic drift). *Instantiated at:* confidence-gated prior
  harvesting (`CorpusMemory`, `learn-priors`).
- **NELL** (Carlson et al., AAAI 2010). *Take:* never-ending KB accumulation. *Leave out (for
  now, +why):* coupled cross-type constraints for drift control (flagged) — SpecForge gates
  per-prior-type. *Instantiated at:* `CorpusMemory` typed priors (incl. negative-knowledge).
- **Snorkel / data programming** (Ratner et al., PVLDB 2017, arXiv:1711.10160). *Take:*
  (reference) labeling-function denoising. *Leave out (for now, +why):* learning LF
  accuracies — SpecForge's confidence ladder is hand-set (flagged). *Instantiated at:* the
  prior-family confidence ladder.
- **Parisi et al. — continual learning** (arXiv:1802.07569). *Take:* the catastrophic-forgetting
  framing. *Leave out (for now, +why):* prior decay/revision — SpecForge only accretes (flagged
  as the named gap). *Instantiated at:* none yet.

## Extraction-evaluation methodology

*(per-aspect grounding: `extraction-evaluation.md`)*

- **van Rijsbergen — *Information Retrieval*** (1979, ISBN 9780408709293). *Take:* precision /
  recall / F-measure. *Leave out:* nothing (foundational). *Instantiated at:* `eval.rs` P/R/F1.
- **MUC-5** (Chinchor & Sundheim, 1993, DOI 10.3115/1072017.1072026). *Take:* slot-based P/R/F
  against an answer key. *Leave out (for now, +why):* **partial-match** credit — SpecForge does
  exact-key matching (flagged). *Instantiated at:* closed-world per-labeled-statement scoring.
- **Cohen κ / Krippendorff α / Artstein-Poesio** (Comp. Ling. 2008, DOI 10.1162/coli.07-034-R2).
  *Take:* (reference) inter-annotator agreement. *Leave out (for now, +why):* κ/α on the gold —
  the gold is single-source (agent-drafted); κ needs a second annotator first. *Instantiated
  at:* none yet (flagged).
- **Chao** (Biometrics 1987, DOI 10.2307/2531532). *Take:* the heterogeneity-robust richness
  estimator as a **second** recall N̂. *Leave out (for now, +why):* variance / confidence
  intervals (flagged). *Instantiated at:* **`RECALL-CHAO-ESTIMATOR`** (`recall_estimate` Chao
  fields) — **adopted**.
- **Eick et al.** (ICSE 1992, DOI 10.1145/143062.143090) + **Petersson et al.** (JSS 2004,
  DOI 10.1016/S0164-1212(03)00090-6). *Take:* capture–recapture for estimating *undiscovered*
  items + the honesty (undefined when no overlap). *Leave out (+why):* the human-inspector
  independence assumption (SpecForge's "inspectors" are extractor tiers — weaker independence,
  stated). *Instantiated at:* the capture–recapture recall gauge (`completeness.rs`).

## Uncertainty / residual-honesty

*(per-aspect grounding: `uncertainty-residual-honesty.md`)*

- **Chow** (IEEE Trans. IT 1970, DOI 10.1109/TIT.1970.1054406). *Take:* the optimum
  **reject-option** / error–reject tradeoff. *Leave out (+why):* a scalar reject *score* —
  SpecForge abstains into **typed residual packets**, not a thresholded label.
  *Instantiated at:* `ResidualDecisionPacket`, fail-closed.
- **El-Yaniv & Wiener** (JMLR 2010) + **Geifman & El-Yaniv** (NeurIPS 2017, arXiv:1705.08500).
  *Take:* selective classification / risk–coverage framing. *Leave out (for now, +why):*
  guaranteed-risk coverage targeting (flagged). *Instantiated at:* abstain-to-residual.
- **Vovk et al. — conformal prediction** (2005, ISBN 9783031066481) + **Angelopoulos & Bates**
  (arXiv:2107.07511). *Take:* (reference) distribution-free calibrated prediction sets.
  *Leave out (for now, +why):* conformal calibration of the LLM/VLM tier — a flagged future
  tree (needs a held-out calibration set). *Instantiated at:* none yet.
- **Guo et al. — calibration** (ICML 2017, arXiv:1706.04599). *Take:* (reference) temperature
  scaling / reliability diagrams. *Leave out (for now, +why):* calibrating the confidence tiers
  to empirical accuracy (flagged). *Instantiated at:* ordinal `AutomationConfidence` (currently
  uncalibrated).
- **Scheirer et al. — open-set recognition** (TPAMI 2013, DOI 10.1109/TPAMI.2012.256). *Take:*
  the known-unknowns / open-world stance. *Leave out (+why):* open-set classifier machinery.
  *Instantiated at:* closure-invariant miss detection + capture–recapture (the open-world
  surfaces).

## Requirements / normative extraction

*(per-aspect grounding: `requirements-extraction.md`)*

- **RFC 2119 / RFC 8174**. *Take:* the MUST/SHALL/SHOULD/MAY modal vocabulary. *Leave out (for
  now, +why):* an uppercase-only normativity knob + modelling SHOULD/MAY *defeasibility*
  distinctly from MUST (flagged). *Instantiated at:* `SignalConstraintKind` / `Obligation`
  (MUST-level).
- **NLP4RE** (Zhao et al., ACM CSUR 2021, arXiv:2004.01099). *Take:* the NLP-for-requirements
  framing + task taxonomy. *Leave out (+why):* generic FR/NFR classification — SpecForge does
  *typed protocol-semantic lowering*. *Instantiated at:* normative-statement classification.
- **PROMISE-NFR** (Cleland-Huang et al., 2007) + **NoRBERT** (Hey et al., RE 2020). *Take:*
  (reference) supervised requirements classification + the *unseen-project* F1 discipline.
  *Leave out (for now, +why):* a PROMISE-style labelled corpus + a NoRBERT baseline (flagged).
  *Instantiated at:* none yet.
- **ACE — Attempto Controlled English** (Fuchs & Schwitter, arXiv:cmp-lg/9603004). *Take:* the
  controlled-NL → logic idea (precise, unambiguous specs). *Leave out (+why):* requiring authors
  to write in a controlled subset — SpecForge recovers structure from *uncontrolled* vendor
  prose (the input is given, not authorable). *Instantiated at:* n/a (contrast / novelty).
- **Berry-Kamsties handbook** + **Wilson, Rosenberg & Hyatt — NASA ARM** (ICSE 1997,
  DOI 10.1145/253228.253258). *Take:* the **weak-phrase / ambiguity** indicators. *Leave out
  (for now, +why):* routing flagged statements into typed residual packets (deferred).
  *Instantiated at:* **`AMBIGUITY-PHRASE-DETECTOR`** (`ir/ambiguity.rs`) — **adopted**.

## Staged IR / progressive lowering

*(per-aspect grounding: `staged-ir.md`)*

- **LLVM** (Lattner & Adve, CGO 2004, DOI 10.1109/CGO.2004.1281665). *Take:* a stable,
  language-independent IR as the single backend boundary. *Leave out (+why):* SSA /
  optimization machinery (SpecForge lowers *intent*, not code). *Instantiated at:* `.isf` as the
  sole adapter target.
- **MLIR** (Lattner et al., CGO 2021, arXiv:2002.11054). *Take:* multi-level IR + **progressive
  lowering** across descending abstraction levels; per-level verifiers (reference). *Leave out
  (for now, +why):* dialect/legalization infra + an MLIR-style stage verifier asserting no
  un-lowered construct survives (flagged). *Instantiated at:* `SourceIR→EvidenceIR→SemanticIR→
  IntentIR→.isf`.
- **Nanopass** (Sarkar/Waddell/Dybvig ICFP 2004; Keep & Dybvig ICFP 2013). *Take:* many small
  typed per-pass languages. *Leave out (for now, +why):* auto-derived *differential grammars* +
  a generated stage verifier (flagged). *Instantiated at:* the staged typed IRs.

## Spec → hardware (PSL/SVA, HLS)

*(per-aspect grounding: `spec-to-hardware.md`)*

- **IEEE 1850 PSL** + **IEEE 1800 SVA**. *Take:* the standard property/assertion *formalism*
  (sequence/property, `assert`/`assume`/`cover`) as the target vocabulary the LTL rendering
  aims at. *Leave out (for now, +why):* an actual `.isf`→PSL/SVA **export** — that touches the
  FSMGen handoff contract (a separate downstream tree; the LTL renderer is the foundation).
  *Instantiated at:* `ir/temporal_ltl.rs` (LTL) → the PSL/SVA export is deferred.
- **AssertLLM** (ASP-DAC 2025, arXiv:2402.00386) + **Hybrid-NL2SVA** (MLCAD 2025,
  arXiv:2506.21569) + **QiMeng-CodeV-SVA** (2026, arXiv:2603.14239). *Take:* (reference) the
  decomposition structure-extraction → signal-mapping → property-emission; NL2SVA benchmark
  framing. *Leave out (+why):* direct LLM assertion generation with no audit trail — SpecForge
  produces a *typed, provenance-carrying* intent IR upstream of any assertion generation.
  *Instantiated at:* the extract→resolve staging (contrast/novelty).
- **HLS** (Cong et al., IEEE TCAD 2011, DOI 10.1109/TCAD.2011.2110592). *Take:* (context) the
  algorithm/spec→RTL boundary. *Leave out (+why):* HLS itself (downstream of `.isf`/FSMGen).
  *Instantiated at:* n/a (boundary context).

---

## Synthesis — the pattern across all authors

Read top-to-bottom, SpecForge takes and defers the *same kinds* of thing from almost every
author — which is itself the design thesis:

- **What SpecForge consistently TAKES:** the **abstraction / unit of extraction** (the triple,
  the `G(ante→cons)` template, the staged typed IR, the reject-option), the **standard
  vocabulary** (LTL/MTL, PSL/SVA, "specification mining", "weak phrases"), the **evaluation
  methodology** (P/R/F1, capture–recapture, inter-annotator agreement), and the **honesty
  stance** (Daikon's *likely* invariants; Chow's reject option; the hallucination taxonomy).
- **What SpecForge consistently LEAVES OUT (for now), and why:** anything that needs **data or
  signals SpecForge does not have** — execution traces, RTL, runs (GoldMine, Texada, Ammons,
  Daikon, Zeng), large labelled corpora or trained models (NoRBERT, PCNN, LayoutLM, NLI,
  Snorkel, conformal calibration); the **backward direction** itself (mining a spec from an
  implementation); and **full-power machinery** beyond the task (LTL model checking,
  open-vocabulary IE, controlled-authoring, KG reasoning, HLS). These are deferred, not
  rejected — many are **flagged future trees** (TEDS/GriTS metric, Dempster combiner, NLI
  verifier, per-relation gold scorer, κ/α agreement, conformal calibration, MLIR-style stage
  verifier, `.isf`→PSL/SVA export, prior decay).
- **Already adopted from the ledger** (closed trees): the Chao estimator
  (`RECALL-CHAO-ESTIMATOR`), the weak-phrase detector (`AMBIGUITY-PHRASE-DETECTOR`), the LTL
  rendering (`TEMPORAL-RULE-LTL-RENDER`), and the temporal-rule eval (`TEMPORAL-RULE-EVAL`) —
  the literature-grounding program turning into shipped, verified code.

The throughline: **SpecForge borrows the abstractions, vocabulary, and honesty of the
spec-mining / IE / RE literature, applies them *forward* (spec → intent), and defers the
parts that assume an implementation or training data it does not have.**

## Links

- Task-tree: `SPEC-MINING-PROVENANCE`. Per-aspect grounding: `docs/research/grounding/*.md`
  (esp. `protocol-temporal-semantics.md`). KM: `spec-mining-framing`,
  `temporal-rule-ltl-rendering`.
