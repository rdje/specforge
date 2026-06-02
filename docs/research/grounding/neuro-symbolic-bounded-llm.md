# Literature grounding — Neuro-symbolic / bounded-LLM extraction

*Aspect 7 of `LITERATURE-GROUNDING`. SpecForge's core principle: use the LLM/VLM as a
bounded, fail-closed hypothesis generator — never an end-to-end black box. Outputs pass
schema constraints + a grounding/entailment gate; anything malformed or ungrounded drops
to a residual, never asserted. All citations web-verified; none guessed.*

## Prior art (verified)

- **S. Geng, M. Josifoski, M. Peyrard, R. West — "Grammar-Constrained Decoding for
  Structured NLP Tasks without Finetuning"**, EMNLP 2023.
  arXiv:[2305.13971](https://arxiv.org/abs/2305.13971). Masks tokens to guarantee output
  conforms to a context-free grammar; matches/beats finetuned task-specific models.
- **B.T. Willard, R. Louf — "Efficient Guided Generation for Large Language Models"**, 2023.
  arXiv:[2307.09702](https://arxiv.org/abs/2307.09702) (Outlines). Reformulates constrained
  decoding as FSM transitions with a precomputed vocabulary index for regex/CFG/JSON-schema
  constraints — basis of the GBNF-style grammars in llama.cpp/Ollama.
- **P. Lewis et al. — "Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks"**,
  NeurIPS 2020. arXiv:[2005.11401](https://arxiv.org/abs/2005.11401). Parametric LM +
  non-parametric retrieval; provenance and reduced fabrication.
- **S. Bowman, G. Angeli, C. Potts, C. Manning — "A large annotated corpus for learning
  natural language inference"** (SNLI), EMNLP 2015.
  arXiv:[1508.05326](https://arxiv.org/abs/1508.05326). Foundational
  entailment/contradiction/neutral framing (NLI).
- **Z. Ji et al. — "Survey of Hallucination in Natural Language Generation"**, ACM Computing
  Surveys 55(12), 2023. DOI [10.1145/3571730](https://doi.org/10.1145/3571730)
  (arXiv:[2202.03629](https://arxiv.org/abs/2202.03629)). Defines *intrinsic* (contradicts
  source) vs *extrinsic* (unverifiable) hallucination.
- **P. Manakul, A. Liusie, M. Gales — "SelfCheckGPT"**, EMNLP 2023.
  arXiv:[2303.08896](https://arxiv.org/abs/2303.08896). Zero-resource consistency check:
  sampled responses diverge on hallucinated facts.
- **A. d'Avila Garcez, L. Lamb — "Neurosymbolic AI: The 3rd Wave"**, Artificial Intelligence
  Review, 2023. DOI [10.1007/s10462-023-10448-w](https://doi.org/10.1007/s10462-023-10448-w)
  (arXiv:[2012.05876](https://arxiv.org/abs/2012.05876)).

## Alignment (where SpecForge already matches the literature)

SpecForge instantiates this canon: GBNF/schema-constrained decoding via Ollama+Qwen =
GCD/Outlines; the entailment/grounding verifier maps to NLI (intrinsic-contradiction
detection) and SelfCheckGPT-style consistency gating; "drop ungrounded to residual" directly
enforces Ji et al.'s extrinsic-hallucination boundary. The deterministic-backbone-plus-
bounded-generator split is textbook neuro-symbolic (Garcez & Lamb): neural hypothesis,
symbolic verification.

## Adopt (proven techniques worth borrowing)

- Frame the schema/JSON gate explicitly as **FSM-indexed grammar-constrained decoding**
  (Willard & Louf) — guarantees validity *at generation time*, not just post-hoc rejection.
- **NLI-style entailment scoring** of each LLM claim against its source span
  (premise = PDF text, hypothesis = extracted claim); keep only ENTAILMENT.
- **SelfCheckGPT self-consistency** — multi-sample a claim; contradiction across samples =
  drop, cheaply, with no extra source.
- **RAG provenance** — bind every accepted claim to a retrieved source span for
  auditability.

## Extend / genuine novelty (the out-of-the-box part)

SpecForge's differentiators are real and unclaimed by the above: (1) fail-closed routing
into a **typed protocol IR residual** rather than free-text generation (the verification
target is a formal datatype, not prose); (2) deterministic typed extraction is primary, the
LLM strictly recovers misses; (3) **capture–recapture recall estimation on the LLM tier** —
a quantitative completeness bound none of these works provide.

## Gaps / opportunities → candidate future trees

1. **Add an explicit NLI verifier model** to the pipeline (grounding is currently
   rule/string-based); benchmark against an SNLI-trained checker.
2. **Calibrate the residual drop threshold** against a hallucination-rate metric (Ji et al.
   taxonomy) — measure intrinsic vs extrinsic drop separately.
3. **Move grammar enforcement *into* decoding** (Outlines/GCD) where the runtime allows,
   reducing wasted generations vs post-hoc schema rejection.

## Links

- Task-tree: `LITERATURE-GROUNDING`. Related SpecForge trees: `R14-SIGNAL-RESOLVE`,
  `LLM-TEXT-TRANSPORT-DEDUP`, `R16-VLM-ENRICH`, `LLM-EXTRACTION-EVAL`. See also aspect 10
  (`uncertainty-residual-honesty.md`).
