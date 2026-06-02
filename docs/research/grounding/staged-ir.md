# Literature grounding — Staged IR & progressive lowering

*Aspect 2 of `LITERATURE-GROUNDING`. SpecForge's architectural spine: lifting intent
through staged typed IRs `SourceIR → EvidenceIR → SemanticIR → IntentIR → .isf`, each a
distinct typed representation, lowered semantics-preservingly with provenance. All
citations web-verified; none guessed.*

## Prior art (verified)

- **Chris Lattner, Vikram Adve — "LLVM: A Compilation Framework for Lifelong Program
  Analysis & Transformation"**, CGO 2004, pp. 75–86. DOI
  [10.1109/CGO.2004.1281665](https://doi.org/10.1109/CGO.2004.1281665). An SSA,
  language-independent low-level IR carried across compile/link/run-time.
- **Chris Lattner, Mehdi Amini, Uday Bondhugula, Albert Cohen, et al. — "MLIR: Scaling
  Compiler Infrastructure for Domain Specific Computation"**, CGO 2021, pp. 2–14. DOI
  [10.1109/CGO51591.2021.9370308](https://doi.org/10.1109/CGO51591.2021.9370308);
  arXiv:[2002.11054](https://arxiv.org/abs/2002.11054). Multi-level IR with *dialects* and
  *progressive lowering* between abstraction levels.
- **Dipanwita Sarkar, Oscar Waddell, R. Kent Dybvig — "A Nanopass Infrastructure for
  Compiler Education"**, ICFP 2004, pp. 201–212. DOI
  [10.1145/1016848.1016878](https://doi.org/10.1145/1016848.1016878). Many small
  single-task passes over *formally specified* intermediate languages.
- **Andrew W. Keep, R. Kent Dybvig — "A Nanopass Framework for Commercial Compiler
  Development"**, ICFP 2013, pp. 343–350. DOI
  [10.1145/2500365.2500618](https://doi.org/10.1145/2500365.2500618). Nanopass at
  production scale (Chez Scheme).

Standard terminology: *multi-level IR* / *progressive lowering* (MLIR), per-pass
*intermediate language* (nanopass), *SSA* / language-independent typed IR (LLVM).

## Alignment (where SpecForge already matches the literature)

SpecForge's `SourceIR → EvidenceIR → SemanticIR → IntentIR → .isf` chain is exactly MLIR's
*progressive lowering* across descending abstraction levels, and nanopass's *many typed
intermediate languages* rather than monolithic stages. Each stage being a distinct typed
IR mirrors nanopass's formally specified per-pass languages — the structure the literature
credits for understandability and isolated reasoning. ".isf as the sole adapter target"
echoes LLVM/MLIR's stable low-level IR as the single backend boundary.

## Adopt (proven techniques worth borrowing)

- **Nanopass differential language definitions** — define each SpecForge IR as a *delta*
  from the prior one (node forms added/removed), so the type system enforces what each
  lowering must produce and eliminate. Cheap, mechanical, and it catches dropped facts.
- **MLIR verifier-per-level** — attach a structural validity check (invariant verifier) to
  each IR so an ill-formed lowering fails at the stage boundary, not downstream.
- **Lowering legalization with a "fully-lowered" check** (MLIR's legality/conversion
  model) — assert that no higher-level construct survives into `.isf`.

## Extend / genuine novelty (the out-of-the-box part)

SpecForge lowers **design intent extracted from natural-language PDF prose**, not source
code. Its highest IR is born from evidence/NLP, so early stages are *inherently lossy and
uncertain* — unlike compiler IRs lowered from a precise input language. And
**residualization of unresolved facts as first-class decisions** has no analog in
LLVM/MLIR/nanopass, which assume total, faithful translation; SpecForge makes
incompleteness an explicit typed output rather than a translation failure.

## Gaps / opportunities → candidate future trees

1. **Formalize each IR boundary as a nanopass-style differential grammar** and auto-derive
   a stage verifier (catch silent drops between stages).
2. **Add an MLIR-style "legalization" pass** asserting `.isf` contains no un-lowered
   Semantic/Intent constructs.
3. **Provenance as a typed attribute threaded through every lowering** (à la MLIR
   location/attribute propagation) so any `.isf` fact traces to a source span.
4. **Model lowering confidence/uncertainty explicitly** so residual decisions are emitted
   by rule, not ad hoc.

## Links

- Task-tree: `LITERATURE-GROUNDING`. Related SpecForge trees: `ISF-TEMPORAL-LOWERING`,
  `ISF-RULE-CONFLICT-RESIDUAL`, `LLM-TEXT-TRANSPORT-DEDUP`.
