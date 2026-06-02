# Literature grounding — Cross-document learning plane

*Aspect 8 of `LITERATURE-GROUNDING`. SpecForge's `CorpusMemory`: a typed prior store that
accumulates reusable extraction knowledge harvested ONLY from validated outputs, consulted
advisory-only on new documents — it can widen interpretation but never invent a fact not
grounded in the current document. All citations web-verified; none guessed.*

## Prior art (verified)

- **D. Yarowsky — "Unsupervised Word Sense Disambiguation Rivaling Supervised Methods"**,
  ACL 1995, pp. 189–196 ([ACL P95-1026](https://aclanthology.org/P95-1026/)). Iterative
  **bootstrapping** from seeds; "one sense per collocation / per discourse."
- **E. Riloff, R. Jones — "Learning Dictionaries for Information Extraction by Multi-Level
  Bootstrapping"**, AAAI 1999, pp. 474–479. **Mutual/meta-bootstrapping**: alternate
  pattern ↔ lexicon, retain only the most-reliable entries.
- **M. Mintz, S. Bills, R. Snow, D. Jurafsky — "Distant Supervision for Relation Extraction
  without Labeled Data"**, ACL-IJCNLP 2009 ([ACL P09-1113](https://aclanthology.org/P09-1113/)).
  KB facts as **distant supervision** over an unlabeled corpus.
- **A. Carlson, J. Betteridge, B. Kisiel, B. Settles, E.R. Hruschka, T. Mitchell — "Toward
  an Architecture for Never-Ending Language Learning" (NELL)**, AAAI 2010, pp. 1306–1313.
  **Coupled** semi-supervised learning; cross-category/relation constraints curb semantic
  drift.
- **A. Ratner, S. Bach, H. Ehrenberg, J. Fries, S. Wu, C. Ré — "Snorkel: Rapid Training Data
  Creation with Weak Supervision"**, PVLDB 11(3), 2017.
  arXiv:[1711.10160](https://arxiv.org/abs/1711.10160). **Data programming**: noisy
  **labeling functions** denoised by a generative model.
- **G.I. Parisi, R. Kemker, J.L. Part, C. Kanan, S. Wermter — "Continual Lifelong Learning
  with Neural Networks: A Review"**, Neural Networks, 2019.
  arXiv:[1802.07569](https://arxiv.org/abs/1802.07569). Lifelong accumulation vs
  **catastrophic forgetting**.

## Alignment (where SpecForge already matches the literature)

SpecForge's `CorpusMemory` (`ir/prior_memory.rs`, `commands/learn_priors.rs`) is a
**knowledge-base-accumulation** plane in the NELL lineage: typed priors (actor-taxonomy,
semantic-phrase, temporal, table-shape, visual-motif, negative-knowledge) accreted across a
corpus. Harvesting is **confidence-gated bootstrapping** — `assess_intent_for_learning`
admits only validated IntentIR (`requires_validated_intent_ir`, `rejects_error_findings`),
with `support_count`/`strongest_automation_confidence` echoing Riloff & Jones's "retain only
reliable entries." Priors are consumed **advisory-only** (`advisory_only: true`), widening
interpretation like distant-supervision seeds without dictating labels.

## Adopt (proven techniques worth borrowing)

- **NELL-style coupled constraints** — cross-check actor-taxonomy ↔ semantic-role ↔
  protocol-family priors so mutually-incompatible promotions are blocked, hardening drift
  control beyond per-type `support_count`.
- **Snorkel denoising** — treat each prior family as a labeling function and learn per-source
  accuracies/correlations (currently a hand-set `support_count >= {2,3}` ladder) to weight
  advice.
- **Yarowsky one-sense-per-discourse** — enforce intra-document consistency when a prior
  widens a local reading.

## Extend / genuine novelty (the out-of-the-box part)

Unlike NELL/Snorkel, SpecForge priors are **strictly advisory and cannot invent an
ungrounded fact** — `local_grounding_required_for_canonical_promotion` keeps per-document
truth immutable, so corpus knowledge never overwrites the current spec. Distinctive
**negative-knowledge priors** (`NegativeKnowledgeKind`: known conflict/residual patterns)
accumulate *what not to trust* — an under-explored complement to positive-fact KBs.
Harvesting only from **validated** outputs sidesteps the drift that unrestricted
self-training (Yarowsky/NELL) suffers.

## Gaps / opportunities → candidate future trees

1. **No forgetting/decay** — priors only accrete; add staleness or revision when a later
   validated doc contradicts a prior (Parisi-style plasticity).
2. **No learned labeling-function accuracies** — the confidence ladder is fixed; a Snorkel
   generative step would calibrate it.
3. **Coupled cross-type constraints (NELL) are absent** — promotions are judged per-prior-
   type independently.
4. **Provenance-weighted trust** — corroboration from independent protocol families could
   outrank repeated single-source support.

## Links

- Task-tree: `LITERATURE-GROUNDING`. Related SpecForge trees: `CORPUS-MEMORY`,
  `LEARN-PRIORS`, `CORPUS-HARDENING`.
