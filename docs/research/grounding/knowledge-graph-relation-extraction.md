# Literature grounding — Knowledge-graph & relation extraction

*Aspect 5 of `LITERATURE-GROUNDING`. SpecForge builds a knowledge graph of actors
(Manager/Subordinate/Requester/Completer), signals, and their relations (drives/reads)
from technical text and tables, with provenance, surfacing conflicts when evidence
disagrees. All citations web-verified; none guessed.*

## Prior art (verified)

- **M. Banko, M. Cafarella, S. Soderland, M. Broadhead, O. Etzioni — "Open Information
  Extraction from the Web"**, IJCAI 2007. Coins **Open IE**: a single self-supervised pass
  extracting **(arg1, relation, arg2) tuples** with no pre-specified relation vocabulary;
  the **TextRunner** system ([ACL N07-4013](https://aclanthology.org/N07-4013/)).
- **A. Fader, S. Soderland, O. Etzioni — "Identifying Relations for Open Information
  Extraction"**, EMNLP 2011 ([ACL D11-1142](https://aclanthology.org/D11-1142/)). **ReVerb**:
  syntactic + lexical constraints on **verb-mediated** relations to cut incoherent
  extractions.
- **M. Mintz, S. Bills, R. Snow, D. Jurafsky — "Distant supervision for relation extraction
  without labeled data"**, ACL-IJCNLP 2009 ([ACL P09-1113](https://aclanthology.org/P09-1113/)).
  **Distant supervision**: align a KB to unlabeled text to auto-label training pairs.
- **D. Zeng, K. Liu, Y. Chen, J. Zhao — "Distant Supervision for Relation Extraction via
  Piecewise Convolutional Neural Networks"**, EMNLP 2015
  ([ACL D15-1203](https://aclanthology.org/D15-1203/)). Neural RE (**PCNN + multi-instance**)
  handling the distant-supervision wrong-label problem.
- **H. Ji, R. Grishman — "Knowledge Base Population: Successful Approaches and Challenges"**,
  ACL 2011 ([ACL P11-1115](https://aclanthology.org/P11-1115/)); the **TAC-KBP**
  slot-filling/entity-linking evaluations (<https://tac.nist.gov>).
- **C. Niklaus, M. Cetto, A. Freitas, S. Handschuh — "A Survey on Open Information
  Extraction"**, COLING 2018 ([ACL C18-1326](https://aclanthology.org/C18-1326/);
  arXiv:[1806.05599](https://arxiv.org/abs/1806.05599)).
- **A. Hogan et al. — "Knowledge Graphs"**, ACM Computing Surveys 54(4):71, 2021. DOI
  [10.1145/3447772](https://doi.org/10.1145/3447772). KG data models, schema/ontology,
  deductive + inductive extraction.

## Alignment (where SpecForge already matches the literature)

SpecForge is textbook Open IE specialized to a domain: the core unit is the **(actor,
relation, signal) triple** with `RelationKind::{Drives, Reads}` — directly the Open IE tuple
form. Tier-2 prose extraction uses **ReVerb-style verb-mediated lexical/syntactic patterns**
(active/passive drive/read verb lists). Every relation carries provenance
(`source_statement_ids`) and `AutomationConfidence`, matching KBP/Hogan provenance practice.
The Tier-2 → Tier-3 (deterministic → LLM) escalation mirrors the OIE-survey precision/recall
layering.

## Adopt (proven techniques worth borrowing)

- **Distant supervision (Mintz)** as the stated Level-4 path: human-reviewed triples are
  auto-labels. Frame the seed corpus as a distant-supervision dataset keyed off the closed
  protocol ontology.
- **KBP-style RE evaluation** — report **precision/recall/F1 per relation type** against a
  gold slot-set (the eval harness exists for extraction; extend it to relations), and adopt
  **PR-curve** reporting (ReVerb).
- **Hogan ontology hygiene** — explicit schema-validation counts on KG node/edge types
  (already present in the `Channel/Transaction/Phase/HandshakePair` ontology work).

## Extend / genuine novelty (the out-of-the-box part)

Unlike open-vocabulary Open IE, SpecForge uses a **closed, domain-typed protocol ontology**
(behavioral actor roles Manager/Subordinate; `Channel`/`Phase`/`HandshakePair`; `TickPhase`
vs `ProtocolPhase`) — actor identity is *behavioral, not lexical*, which generic systems
lack. It fuses **multimodal evidence** (table signal names + prose relations + figures via
VLM), beyond the text-only seminal work. And it **fails closed**: value-conflicting
relations/rules are not silently dropped but emitted as `ResidualDecisionPacket` residuals —
a provenance-and-conflict discipline stronger than KBP confidence scoring.

## Gaps / opportunities → candidate future trees

1. **Gold relation set + per-relation P/R/F1** KBP-style scorer (currently only
   extraction-level eval exists).
2. **Distant-supervision Level-4 NER+RE model** once enough reviewed triples exist;
   benchmark against the Tier-2/3 patterns.
3. **Cross-sentence / coreference RE** (a noted Open IE limitation) for actor antecedents
   spanning sentences.
4. **Canonicalization layer** (Hogan) merging actor/signal aliases before triple assertion
   to reduce conflict noise.

## Links

- Task-tree: `LITERATURE-GROUNDING`. Related SpecForge trees: `R14-SIGNAL-RESOLVE`,
  `R16-KG-PROTOCOL-ONTOLOGY`, `COMPLETENESS-RECALL-RELATIONS`, `ISF-RULE-CONFLICT-RESIDUAL`.
