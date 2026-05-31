# Intent-Capture Completeness — Research Framework

> Status: **living research artifact** (owned by `INTENT-COMPLETENESS-RESEARCH`).
> Purpose: think the accuracy problem through from first principles BEFORE
> coding, so that what we build is provably aimed at the right target. This
> document is theory + design; it does not claim implemented capability. Each
> instrument it specifies becomes its own implementation task-tree.

## 0. The problem, stated precisely

**Goal.** Capture *all* design intent expressed in a digital-chip design PDF —
in prose, tables, figures, captions, and structure — into the typed IR / KG
pipeline (`SourceIR → EvidenceIR → SemanticIR → IntentIR → .isf`), at an
accuracy approaching 100%.

**Why "approaching".** 100% is asymptotic for an open-ended natural-language +
visual artifact. The achievable, *honest* engineering goal is stronger than a
vague "high accuracy":

> Every miss the system is *capable of detecting* is surfaced as an explicit
> residual, and the residual recall is *estimated and reported* so the user
> knows the confidence. A miss that is detected and surfaced is no longer a
> silent miss — it becomes a decision the user, a rescan, an LLM, or a human can
> resolve. This is the residual-honesty doctrine scaled from precision to
> completeness.

So the research target is not "perfect extraction" but **"extraction whose gaps
are known, bounded, and surfaced, with a calibrated completeness estimate."**

## 1. Two axes, asymmetric difficulty

| Axis | Question | Failure | Detectable by… |
| --- | --- | --- | --- |
| **Precision** | Is everything we captured true to the doc? | hallucination / wrong fact | going back to the **source** (provenance) — *easy* |
| **Recall** | Did we capture everything the doc states? | **a miss** (false negative) | comparing to "truth" — *hard, no oracle* |

The user's question is overwhelmingly about **recall / misses**. Precision is
largely handled (see §9). The intellectual work is recall.

**The recall trap:** you cannot find a miss by inspecting the output, because the
miss is precisely what is *not* there. You need an external notion of "what
should be there." There are only four sources of that notion, and the whole
research program is about exploiting them:

1. **The source itself** — every meaning-bearing input region should be
   explained (§3, §4). *(No ground truth needed.)*
2. **Domain invariants** — well-formedness/closure laws of digital design that,
   when violated, prove something is missing (§5, §6). *(No ground truth needed.)*
3. **Redundancy** — independent extractors / modalities that should agree;
   disagreement or unique finds reveal the unseen (§8 capture–recapture).
   *(No ground truth needed; gives a statistical estimate.)*
4. **Curated ground truth** — gold fixtures + competency questions (§8).
   *(Precise but small.)*

## 2. The denominator problem — a closed ontology of design intent

You cannot measure "did we capture everything" without defining **everything**.
Recall = captured / (captured + missed); the denominator is undefined until the
*universe of intent categories* is explicit and closed.

**Deliverable:** a **closed, typed ontology of digital-design intent** — the
finite set of intent categories a chip spec can express. SpecForge already
embodies most of it implicitly across the IR types; the research makes it
*explicit and exhaustive*:

- **Structural:** signals/ports (name, direction, width, polarity), buses,
  interfaces, actors (manager/subordinate/initiator/target), connectivity
  (drives/reads/produces/consumes), hierarchy/instantiation.
- **State:** registers (offset, fields, bit-ranges, reset value, access RW/RO/WO/W1C),
  enumerations/encodings, parameters/constants, value atoms.
- **Temporal/behavioral:** clocks (domains, edges), resets (polarity, sync/async),
  timing constraints (setup/hold, cycle windows, latency bounds), handshake/protocol
  semantics, normative rules (must/shall/until/while), state machines
  (states, transitions, guards), stability/sampling obligations.
- **Cross-cutting:** assumptions, residuals, conflicts, provenance.

For each category, a **coverage matrix** records *where it is typically
expressed* (prose / table-kind / figure-kind / caption / section) and *which
extractor owns it*. The matrix is the spec of "what we must attempt." A category
× modality cell with **no owning extractor** is a *systematic blind spot* — the
most dangerous kind of miss (silent and total). Finding empty cells is itself a
research output.

> **Concrete, code-grounded ontology + coverage matrix:**
> [`intent-ontology-coverage.md`](intent-ontology-coverage.md) (leaf `.2`). It
> maps ~20 categories to their IR fields + producers + modalities, with gap
> claims verified against the code. Confirmed (A) ontology gaps so far: **clock
> domains** (no first-class type) and **enumerations as a queryable record**.
> (B) single-modality blind spots are recorded there as hypotheses the §3/§4
> detectors will confirm empirically.

## 3. Where intent lives — the source model and region accounting

Intent is distributed across modalities, each with characteristic expression:

- **Prose:** normative sentences ("X must remain stable until Y is asserted").
- **Tables:** signal-description, register-map, timing-parameter, encoding tables.
- **Figures:** timing diagrams, state diagrams, block/connectivity diagrams.
- **Captions & labels:** often carry the role/meaning a figure encodes.
- **Section structure:** scoping (which actor/interface a section is about).
- **Equations/constants:** parametric widths, address maps.

**Region accounting (the key recall instrument).** Partition the document into
regions (text blocks, table cells, figures, captions). Classify each region as
exactly one of:

- **intent-bearing** → must link forward to ≥1 extracted fact;
- **non-intent** (boilerplate, prose explanation, legal, TOC) → tagged with a
  reason;
- **deferred/unsupported** (e.g., a figure we cannot yet decode) → tagged as a
  *known* gap.

> **A region that is intent-bearing but links to zero facts is a candidate
> miss.** This is detectable purely on the input side — no oracle.

The classifier need not be perfect: even a high-recall "is this region likely to
carry intent?" gate converts silent total misses into *visible* residuals
("this normative-looking sentence / this table / this timing figure produced no
fact"). That visibility is the whole game.

## 4. The central reframe — what *is* a miss, operationally

> **A miss is either (a) an intent-bearing source region that produced no fact
> (§3), or (b) a violated completeness invariant of the domain or pipeline
> (§5–§7).**

Both are *checkable conditions*, not oracle comparisons. This converts the
impossible question "what did we miss?" into a finite battery of detectors, each
of which emits an explicit residual when it fires. We never claim "nothing was
missed"; we claim "no detector fired," with the detector set itself being the
audited, growing measure of how thorough we are.

## 5. Miss taxonomy — kinds of misses, each with a detector

| Miss kind | Example | Detector (no ground truth) |
| --- | --- | --- |
| **Region-level** | a normative sentence / table / figure yields no fact | backward-traceability: intent-bearing region with 0 linked facts (§3) |
| **Field-level** | a signal with no width; a register field with no reset value | schema-completeness invariant: expected-attribute checklist per entity type |
| **Relation-level** | a VALID with no READY; a signal with no producer | graph-closure invariant (§6) |
| **Reference (dangling)** | prose names `HREADY`, never declared anywhere | symbol-closure: referenced ⇒ must be declared locally |
| **Cross-modal** | a signal in a timing figure absent from the signal table | cross-modal agreement: union-of-modalities vs per-modality |
| **Structural-closure** | register fields don't tile [0,width); enum doesn't cover 2ⁿ | domain tiling/coverage invariant (§6) |
| **Pipeline-conservation** | a fact in EvidenceIR vanishes by SemanticIR with no residual | inter-stage conservation (§7) |
| **Systematic blind spot** | an ontology category with no owning extractor | coverage-matrix empty cell (§2) |

This table is the spine of the program: **each row is a future implementation
tree** that adds a detector. Completeness improves monotonically as detectors
are added — and the *list of detectors* is the auditable definition of "how hard
we look."

## 6. Completeness invariants — domain closure laws

Borrowed from formal methods (well-formedness + closure properties). When an
invariant is violated, *something is missing or wrong* — either a miss or a
genuine document defect; either way it must be surfaced. Examples specific to
digital design:

- **Symbol closure:** every signal referenced in prose/table/figure appears in
  the canonical signal inventory (else: missed declaration or doc defect).
- **Handshake closure:** every `*VALID`/request has a matching `*READY`/accept
  (protocol pairing) — a lone half is a likely miss.
- **Register tiling:** a register's fields partition `[0, width)` with no gap or
  overlap — a gap is a missed field.
- **Encoding coverage:** an enum over `n` bits either covers `2ⁿ` codes or marks
  the remainder reserved — uncovered codes are a likely miss.
- **Direction/producer closure:** every signal has ≥1 driver and (for non-output)
  ≥1 reader; isolated signals are suspicious.
- **Clock/reset completeness:** every clock has a domain; every reset a polarity
  and sync/async classification.
- **Temporal closure:** every timing constraint binds to a declared clock edge
  and named signals.

These are *local closed-world* assumptions: within one document, the doc is
expected to define what it references. (See §11 / `.6`: AMIE / Local Closed World
Assumption / Partial Completeness Assumption.)

**Two classes of invariant (corrected by the `.6` survey).** Some are genuinely
*exact* (register bit-tiling, symbol closure, handshake pairing — pure structural
checks). Others are **PCA-style heuristics, NOT exact**: "if a region produced
*some* fact of type T it produced *all* of them" can **under-count** misses when
a region is only partially extracted. These must be **gated by explicit
per-relation cardinality/functionality**: safe where the relation is functional
or fixed-cardinality (port→direction, signal→width, register-row→{reset,access});
*unsafe* for open relations (cross-references, "related constraints") where PCA
would silently hide a miss. The catalog (`.4`) tags each detector exact-vs-gated,
and a synonym/unit **canonicalization precondition** (HNEN, `.6`) runs first so
terminology drift neither masquerades as nor hides a miss.

## 7. Inter-stage conservation — misses the pipeline introduces

A miss can enter at **any** stage transition, independently of ingestion:

`PDF → SourceIR` (Docling drops a table) · `→ EvidenceIR` (a pattern misses a
constraint) · `→ SemanticIR` (fusion silently drops a fact) · `→ IntentIR`
(canonicalization loses a record) · `→ .isf` (lowering drops a rule — already
partly guarded by the rule-conflict residual work).

**Conservation principle:** a fact present at stage N must appear at stage N+1
**or** be accompanied by an explicit residual/decision explaining its
disappearance. Track fact identity/counts across stages; an unexplained drop is
a *pipeline-introduced miss* — a distinct, highly detectable class (we control
both sides of the transition). This generalizes the "metric == emitted content"
and residual-honesty doctrines into a cross-stage **conservation ledger**.

## 8. Recall estimation without full ground truth

Detectors (§5–§7) find *specific* misses. To estimate the *residual* recall
(misses no detector caught), use:

1. **Capture–recapture (mark–recapture).** Run independent extractors over the
   same content (e.g. SpecForge's pattern Tier-1/2 vs the LLM Tier-3). Overlap of
   what they each find lets you estimate the unseen population. Lincoln–Petersen
   `N̂ ≈ a·b/m` is the 2-pass intuition (a=100, b=90, m=80 ⇒ N̂≈112 ⇒ ~12 estimated
   misses with no gold answer). **Corrected/sharpened by the literature survey
   (`.6`, see [`literature-grounding.md`](literature-grounding.md)):**
   - **Independence is the load-bearing — and dangerous — assumption.** Two LLM
     passes on the *same* backbone are *positively correlated* → they co-miss the
     same hard regions → capture–recapture **over-estimates recall / under-counts
     misses** (the unsafe direction). Require **≥3 heterogeneous** extractors
     (rule-based vs VLM vs a different model family / a different *source* — see
     two-source fusion below), NOT reseeds of one model.
   - Use **Chao's 1987 Mh estimator** `N̂ = D + f₁²/(2·f₂)` (+ CI) — not plain
     Lincoln–Petersen — for the singleton-heavy regime chip extraction hits
     (most facts found by only one extractor).
   - Capture–recapture *empirically underestimates* content → report `recall_hat`
     as an **optimistic ceiling** and misses as a **lower bound**, assumptions
     printed. It is a calibrated gauge, never a guarantee.
   - **It cannot see systematic blind spots** (a fact missed by *every* pass
     contributes nothing to the overlap) — those are caught only by the ontology
     coverage matrix (§2) and independent-source fusion. The instruments are
     complementary; none alone suffices.
2. **Competency questions.** From ontology engineering: define the set of
   questions a complete chip-spec KG must answer ("all inputs of actor X?",
   "reset value of field Y?", "what must be stable during a write?", "latency
   REQ→ACK?"). If the document addresses a competency question but the KG cannot
   answer it, that is a *measured* miss. Completeness becomes query-answerability.
3. **Gold + negative fixtures (already R15e).** Small, fully-annotated specs give
   exact precision/recall on the covered surfaces; negative fixtures guard
   against false positives. The `kg-bench` harness already exists for this.
4. **LLM completeness critic.** A separate adversarial pass: "list everything in
   this region that the extraction did not capture." High-recall, lower-precision
   — its output is *candidate* residuals, gated by the same grounding rules.

The honest output is a **calibrated completeness estimate**, not a single number:
per-category detector results + a capture–recapture interval + competency-question
pass-rate + gold-fixture recall.

## 9. Safeguards already in place (precision) — build on these

SpecForge already enforces the precision side; completeness work must preserve
these and never trade precision for recall:

- **Fails-closed parsing** — malformed LLM output → `schema_reject`, never a
  fabricated fact (`parse_constrained_contract`).
- **Entailment gating** — claims unsupported by evidence → **Residual**, not
  canonical fact (`apply_entailment_to_contract`).
- **Grounding gates** — extracted relations must reference *declared* signals
  (R14 `signal-resolve`).
- **Conflict surfacing** — contradictions are preserved as typed conflict records
  (polarity / semantic / interface / connectivity / temporal), never flattened
  (R15d).
- **Provenance required** for any promotion to canonical IR.
- **Residual-honesty doctrine** — uncertain/withheld → explicit residual/assumption.
- **Convergence reporting** (just shipped, `R15C-CONVERGENCE-REPORT`) — the
  anchored-rescan loop reports whether it stabilized or stopped at its cap. This
  is the *first* completeness instrument and the seed of §10.

Completeness adds the **recall** detectors on top; the safeguards ensure those
detectors raise *residuals* (visible gaps) rather than *fabrications*.

## 10. The completeness report — the unifying artifact

Generalize the just-shipped `EvidenceConvergenceReport` into a typed
**`CompletenessReport`** carried on the IR and surfaced by `validate`:

- per-category coverage (from the §2 matrix): attempted? yielded facts?
- region accounting (§3): intent-bearing regions total / linked / **unexplained**;
- invariant results (§6): which closure checks passed/failed, with the offending
  entities;
- conservation ledger (§7): facts dropped between stages without a residual;
- recall estimate (§8): capture–recapture interval + competency-question pass-rate;
- a single honest headline: *"N detectors run; K candidate misses surfaced;
  estimated residual recall R% ± e."*

Every entry is a residual the user can act on. This is the deliverable that makes
accuracy *visible and improvable*, iteration over iteration.

## 11. Academic grounding (starting points — verify/expand in `.6`)

> **Verified, corrected grounding now lives in
> [`literature-grounding.md`](literature-grounding.md)** (leaf `.6` — an
> 8-discipline research workflow). It confirms the core reframe as the field's
> consensus, supplies DBLP/ACM/arXiv-checked references, and **corrects** this
> framework (capture–recapture → Chao + heterogeneity + bias direction; PCA is
> heuristic-and-gated, not exact; competency questions bound schema not
> population; systematic blind spots are invisible to capture–recapture) and
> **extends** it (mutation/sensitivity coverage; star-pattern oracle; two-source
> KG fusion; HNEN canonicalization; the **STOP-OR-REINSPECT** rule that gives the
> already-shipped `R15C-CONVERGENCE-REPORT` loop a *principled* statistical
> stopping criterion). The list below is the original orienting map.

Concepts and fields to ground the design in (the literature-survey leaf will
confirm exact references; listed here as established starting points, not as
verified citations):

- **Knowledge-graph completeness:** Local Closed World Assumption / Partial
  Completeness Assumption; rule mining under incompleteness (AMIE — Galárraga,
  Suchanek et al.). Directly motivates §6's local-closed-world invariants.
- **Recall estimation:** capture–recapture / Lincoln–Petersen (ecology);
  its application to **software-inspection defect/recall estimation** (Petersson
  & Wohlin survey; Briand, El Emam, Freimut, Laitenberger). Motivates §8.1.
- **Ontology engineering:** **competency questions** (Grüninger & Fox, 1995) as
  the completeness acceptance instrument. Motivates §8.2.
- **Requirements engineering:** bidirectional **traceability matrices**; NLP for
  requirements (ambiguity, completeness checking). Motivates §3 backward links.
- **Information extraction:** Open IE, slot filling, relation extraction;
  precision/recall methodology; distant supervision. General IE recall practice.
- **Specification / invariant mining:** Daikon dynamic invariant detection
  (Ernst) — analog for "expected invariants"; hardware **assertion/property
  generation from natural language** (NL→SVA) — analog for normative-rule capture.
- **Document understanding:** layout analysis, **table-structure recognition**
  (PubTabNet, TableBank), figure/diagram understanding; Docling (the ingestion
  backend SpecForge already uses).
- **Datasheet/hardware-spec extraction:** emerging work on register-map and
  pin/signal extraction from datasheets — closest prior art to the target domain.

This is *not* "just web search" — it is a map of the disciplines whose results we
adapt. The survey leaf turns each bullet into verified references + the specific
result we borrow.

## 12. From research to code — prioritized instrument backlog

Each becomes its own implementation task-tree (no code without one). Ordered by
*value ÷ cost* and dependency:

1. **Region accounting + backward traceability** (§3) — highest value; converts
   silent misses to visible residuals; foundational for §10. Builds on existing
   provenance (forward links exist; add backward index + unexplained-region tag).
2. **Domain closure invariants** (§6) — cheap, exact, high-value: symbol closure
   (dangling references), register tiling, handshake pairing. No ML needed.
3. **Inter-stage conservation ledger** (§7) — we control both sides; catches
   pipeline-introduced misses deterministically.
4. **Completeness report** (§10) — unify §1–§3 outputs (extend the convergence
   report pattern) + surface in `validate`.
5. **Capture–recapture recall gauge** (§8.1) — reuse the existing dual extractors
   (pattern vs LLM); statistical recall estimate.
6. **Competency-question battery** (§8.2) — query-answerability completeness;
   pairs with the ontology (§2).
7. **Ontology + coverage matrix formalization** (§2) — make the denominator
   explicit; find empty cells (systematic blind spots). *(May lead 1–6 if we
   want the denominator first.)*
8. **Cross-modal agreement** (§5 cross-modal) + **LLM completeness critic** (§8.4)
   — higher-cost, ML-assisted; later.

## 13. Honest accuracy framing — what "99.999…%" means here

A bare percentage is meaningless without (a) the denominator (§2 ontology) and
(b) the recall instrument (§8). What we *can* deliver and defend:

- **Per-category recall on gold fixtures** (exact, small denominator).
- **Capture–recapture residual-recall interval** (statistical, full document).
- **Zero unexplained intent-bearing regions** (a *binary* completeness property
  on the source side — achievable and checkable).
- **All closure invariants satisfied or their violations surfaced** (binary).
- **Competency-question pass-rate** (functional completeness).

"Approaching 100%" then has a precise operational meaning: *the unexplained-region
count trends to zero, all closure invariants hold or surface, the
capture–recapture interval tightens near 100%, and the competency battery passes
— for a defined ontology, with every residual visible.* That is a target we can
measure, report, and drive down — which is exactly what accuracy-of-utmost-importance
demands.
