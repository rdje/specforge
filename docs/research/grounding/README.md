# Literature grounding — synthesis map & reach-full-potential backlog

*Synthesis leaf (`.13`) of `LITERATURE-GROUNDING`. This is the unifying "SpecForge ↔ the
literature" map across all grounded aspects, plus the prioritized backlog of what to adopt,
what genuine novelty to claim, and which research-suggested improvements become future
owned task-trees. Every per-aspect doc cites only web-verified sources (resolvable id);
nothing is guessed.*

## Why this program exists

SpecForge does many things — document AI, staged IR lowering, requirements/protocol
extraction, knowledge-graph building, multimodal fusion, bounded-LLM extraction,
cross-document learning, evaluation, uncertainty handling, and the spec→hardware handoff.
Each of those is an **established research field**. This program maps what SpecForge does
onto that prior art so we (1) **do not reinvent the wheel**, (2) **adopt the right
vocabulary and proven techniques**, and (3) **claim only the novelty that is genuinely
ours** — while keeping the out-of-the-box parts deliberate, not accidental.

Findings here are *advisory research*, not code. Any technique SpecForge decides to adopt
becomes a separate, code-owning task-tree downstream (per the doctrine).

## The aspect → literature map

| # | Aspect (doc) | Literature anchor(s) | SpecForge alignment | Claimed novelty |
| --- | --- | --- | --- | --- |
| 1 | [Document extraction](document-extraction.md) | Docling, TableFormer, DocLayNet, PubTables-1M; TEDS/GriTS | Ingest layer *is* the SOTA doc-AI stack | Lifts protocol *intent* above the structured doc |
| 2 | [Staged IR](staged-ir.md) | LLVM, MLIR (progressive lowering), nanopass | Staged typed IRs = MLIR/nanopass shape | Lowers *intent from prose*, residualizes the unresolved |
| 3 | [Requirements extraction](requirements-extraction.md) | RFC 2119/8174, NLP4RE, PROMISE-NFR, NoRBERT, ACE | Modal/normative prose → typed constraints | Typed protocol-semantic lowering, multimodal |
| 4 | [Protocol & temporal semantics](protocol-temporal-semantics.md) | Pnueli LTL, spec mining (Ammons/Daikon/Texada/GoldMine) | `temporal_rules` = `G(ante→cons)` templates | Mines from the *spec* (forward), not traces/RTL |
| 5 | [KG & relation extraction](knowledge-graph-relation-extraction.md) | OpenIE, ReVerb, distant supervision, KBP, Hogan | `(actor, relation, signal)` = Open IE tuple | Closed, behavioral, domain-typed protocol ontology |
| 6 | [Multimodal fusion](multimodal-fusion.md) | LayoutLM v1–3, Donut, DocVQA; Dempster–Shafer | Typed multimodal `EvidenceModality` | Fail-closed cross-modal **conflict residuals** |
| 7 | [Neuro-symbolic / bounded-LLM](neuro-symbolic-bounded-llm.md) | GCD/Outlines, RAG, NLI, hallucination survey, SelfCheckGPT | Constrained decode + grounding gate + fail-closed | Residual into a *typed IR*, recall-bounded LLM tier |
| 8 | [Cross-document learning](cross-document-learning.md) | Yarowsky, Riloff-Jones, NELL, Snorkel, continual | `CorpusMemory` = confidence-gated bootstrap KB | Advisory-only priors + **negative-knowledge** priors |
| 9 | [Extraction evaluation](extraction-evaluation.md) | van Rijsbergen, MUC, Cohen/Krippendorff, Chao, inspection C-R | P/R/F1 + Lincoln–Petersen recall | Capture–recapture across *extractor tiers* |
| 10 | [Uncertainty / residual-honesty](uncertainty-residual-honesty.md) | Chow reject-option, selective classification, conformal, open-set | Abstain-to-residual + confidence tiers | Residuals as first-class typed IR; closure-invariant misses |
| 11 | [Spec → hardware](spec-to-hardware.md) | IEEE 1850 PSL, IEEE 1800 SVA, AssertLLM/NL2SVA, HLS | `IntentIR`→`.isf` = upstream intent formalism | Provenance + residual-honest intent *feeding* HW gen |

*(The **completeness** aspect was grounded earlier by `INTENT-COMPLETENESS-RESEARCH`
— KG completeness/LCWA, Chao-Mh capture–recapture, ontology competency questions — and is
referenced, not repeated; see `docs/research/intent-capture-completeness.md` and
`docs/research/literature-grounding.md`.)*

## The throughline (SpecForge's genuine novelty, claimed deliberately)

Reading across all aspects, the same handful of out-of-the-box choices recur — these are
worth claiming:

1. **Forward intent recovery** — mine protocol/temporal/normative intent *from the
   human-authored specification*, before any implementation exists. The mining/assertion
   literature (Daikon, GoldMine, Texada, AssertLLM/NL2SVA) almost always works *backward*
   from traces, RTL, or code. (aspects 2, 3, 4, 11)
2. **Fail-closed residuals as first-class typed IR objects** — when evidence is not
   decisive, SpecForge emits a structured, provenance-carrying `ResidualDecisionPacket`
   rather than guessing or dropping. The reject-option / selective-classification /
   hallucination literature stops at a scalar score or a dropped sample. (aspects 6, 7, 10)
3. **Capture–recapture recall bounds across automated extractor tiers** — a quantitative
   estimate of the *unseen* misses, not just located ones. (aspects 9, 10)
4. **A closed, behavioral, domain-typed protocol ontology** — actor identity is behavioral
   (Manager/Subordinate), not lexical, unlike open-vocabulary Open IE. (aspect 5)
5. **Advisory-only cross-document priors that cannot invent ungrounded facts**, plus
   **negative-knowledge priors** (what *not* to trust). (aspect 8)

## Reach-full-potential backlog (prioritized → future owned trees)

Each item is advisory until it becomes its own code-owning tree. Ordered by leverage.

### Tier 1 — high-leverage adopt-now
- **LTL/MTL property-template vocabulary for `temporal_rules` + `.isf`→PSL/SVA export**
  (aspects 4, 11). Names SpecForge's ad-hoc temporal shapes in an established formalism and
  makes `IntentIR` contracts directly checkable downstream (FSMGen/sim). Highest
  interoperability payoff.
- **Extend the eval harness: per-relation P/R/F1 + a `temporal_rule` fact kind**
  (aspects 5, 4, 9). The supervised loop already caught a real precision bug for
  constraints; widen it to relations and temporal rules.
- **Conformal calibration + temperature scaling of the LLM/VLM tier; publish a reliability
  diagram / ECE** (aspect 10). Turns the ordinal `AutomationConfidence` tiers into
  distribution-free, calibrated guarantees.

### Tier 2 — principled replacements for current heuristics
- **NLI-based entailment verifier** for each LLM claim vs its source span (aspect 7),
  replacing the current rule/string grounding.
- **Dempster-rule (belief-mass) confidence combiner** replacing `min()` in fusion, with a
  guard for the high-conflict pathology (aspect 6).
- **Inter-annotator agreement (κ / Krippendorff's α) on gold labels + Chao estimator +
  partial-match scorer** (aspect 9).
- **PubTables-1M functional analysis (header vs data) + TEDS/GriTS table-fidelity metric**
  for the ingest stage (aspect 1), re-grounding the `classify_table_kind` heuristics.

### Tier 3 — research-grade / structural
- **Nanopass-style differential grammar per IR boundary + auto-derived stage verifier +
  provenance as a typed attribute threaded through every lowering** (aspect 2).
- **NELL-style coupled cross-type prior constraints + Snorkel-learned labeling-function
  accuracies + prior decay/revision** (aspect 8).
- **Weak-phrase/ambiguity classifier (Berry-Kamsties/ARM) routing to residuals; model
  SHOULD/MAY (defeasible) vs MUST distinctly** (aspect 3).
- **Cross-sentence / coreference relation extraction + an alias canonicalization layer**
  (aspect 5).

## Methodology & discipline (how this was produced)

- Per-aspect research executed via parallel research agents (one per aspect), mirroring the
  proven `INTENT-COMPLETENESS-RESEARCH` workflow.
- **Anti-hallucination is mandatory and was enforced**: every citation carries a resolvable
  id; the orchestrator personally spot-verified the highest-fabrication-risk recent works
  (AssertLLM, Hybrid-NL2SVA, QiMeng-CodeV-SVA, the MLLM-VRDU survey) and **dropped** one
  secondary survey (arXiv:2408.01287) that could not be independently confirmed, rather than
  guess it. A citation that cannot be verified is dropped, not guessed.
- Each aspect doc separates *adopt-from-literature* from *claimed-novelty* so grounding does
  not flatten SpecForge into "only what already exists."

## Links

- Task-tree: `LITERATURE-GROUNDING` (this is `.13`, the closing synthesis).
- Per-aspect docs: the 11 files in this directory (see the map above).
- Completeness grounding (separate, earlier): `docs/research/intent-capture-completeness.md`,
  `docs/research/literature-grounding.md` (`INTENT-COMPLETENESS-RESEARCH`, CLOSED).
- Book mirror: `docs/book/src/architecture-rationale.md` ("Why the design is grounded in
  published research").
