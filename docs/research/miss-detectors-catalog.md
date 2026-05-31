# Miss-Detector Catalog — completeness invariants & the miss taxonomy

> Owned by `INTENT-COMPLETENESS-RESEARCH.4`. Turns the miss taxonomy
> ([`intent-capture-completeness.md`](intent-capture-completeness.md) §5–§7) into
> a concrete catalog of **detectors**, each specified by inputs / check / emitted
> residual / kind. Incorporates the `.6` literature corrections — every detector
> is tagged **EXACT** (pure structural check, no false-negatives by construction)
> or **GATED** (PCA-style heuristic, valid only under a stated
> cardinality/functionality precondition, can under-count misses if mis-applied).
> Design only — each detector becomes (or joins) an implementation tree.

## Preconditions (run before any detector)

- **Canonicalization (HNEN, `.6`):** normalize signal/parameter synonyms + units
  to a canonical form, so terminology/unit drift neither masquerades as a miss
  nor hides one. Closure invariants and competency questions silently assume
  canonical names; this is a hard precondition.
- **Provenance completeness:** every fact must carry a resolvable source-region id
  (precondition for region accounting `.3` and the conservation ledger). Where the
  current code links to a `table_id` but not a row/cell, that is itself a gap to
  close first.

## A. Region-level detectors (input-side; ground-truth-free)

| # | Detector | Inputs | Check | Residual | Class |
| --- | --- | --- | --- | --- | --- |
| A1 | **Unexplained intent-bearing region** | region inventory (`.3`) + backward-traceability index | intent-bearing region with empty backward index | `UnexplainedRegionResidual{region_id, excerpt, signals}` | EXACT (given the classifier) |
| A2 | **Mutation / sensitivity coverage** (Chockler-Kupferman-Vardi) | region + the emitted fact set | perturb the region; if **no** emitted fact changes, the region is fact-*insensitive* | `FactInsensitiveRegionResidual` | EXACT, orthogonal to A1 (catches a region with a forward link that nonetheless drives no fact) |
| A3 | **Deferred-region ledger** | region classifier | region tagged `deferred` (e.g. undecoded figure) | `DeferredRegionResidual{reason}` | EXACT (a *declared* gap; counts against coverage, never silent) |

A1's classifier recall is the ceiling (a region wrongly tagged non-intent is
still a silent miss) → keep the intent gate **high-recall** and periodically
audit the non-intent bucket (LLM completeness critic, §8.4).

## B. Field-level detectors (schema completeness; mostly GATED by cardinality)

| # | Detector | Inputs | Check | Residual | Class |
| --- | --- | --- | --- | --- | --- |
| B1 | **Missing required attribute** | entity + per-type required-attribute schema | a captured entity lacks an attribute its type *requires* | `MissingAttributeResidual{entity, attr}` | GATED (only attributes that are *mandatory* for the type: signal→width, signal→direction, register-field→{bits,access}) |
| B2 | **Star-pattern co-occurrence** (Galárraga/Razniewski WSDM'17) | observed facts in a region | co-occurring facts predict a fact that should be present but isn't (register row ⇒ reset value *and* access type) | `PredictedMissingFactResidual` | GATED (engineered co-occurrence rules from the domain ontology, not mined — per-doc data is sparse) |

B1/B2 are the PCA family: "produced some facts of type T ⇒ should have all of
T." Valid **only** where the relation is functional/fixed-cardinality. The schema
declares which attributes are mandatory; open/optional ones are never flagged.

## C. Relation- & structure-level detectors (mostly EXACT closure laws)

| # | Detector | Inputs | Check | Residual | Class |
| --- | --- | --- | --- | --- | --- |
| C1 | **Symbol closure (dangling reference)** | signal inventory + all references (prose/table/figure) | a referenced signal ∉ inventory | `DanglingReferenceResidual{signal, where}` (missed declaration OR doc defect) | EXACT |
| C2 | **Register bit-tiling** | register fields | fields don't partition `[0,width)` (gap/overlap) | `RegisterFieldGapResidual{reg, gap_bits}` | EXACT |
| C3 | **Handshake pairing** | signal inventory + role hints | a `*VALID`/request with no matching `*READY`/accept (or vice-versa) | `UnpairedHandshakeResidual` | GATED (naming/role heuristic; gate on confirmed handshake role) |
| C4 | **Encoding coverage** | enum + declared bit-width | codes covered `< 2ⁿ` and remainder not marked reserved | `UncoveredEncodingResidual` | EXACT (given width) |
| C5 | **Producer/consumer closure** | actor-signal graph | a signal with no driver, or a non-output with no reader | `OrphanSignalResidual` | GATED (some signals legitimately external) |
| C6 | **Clock/reset completeness** | clocks, resets | a clock with no edge/domain; a reset with no polarity/kind | `IncompleteInfraResidual` | GATED (per-attribute mandatory-ness) |

## D. Cross-modal detectors (redundancy; ground-truth-free)

| # | Detector | Inputs | Check | Residual | Class |
| --- | --- | --- | --- | --- | --- |
| D1 | **Cross-modal presence** | per-modality fact sets (prose/table/figure) | an entity present in modality X absent from modality Y where it should also appear (signal in a timing figure ∉ signal table) | `CrossModalGapResidual{entity, present_in, missing_from}` | GATED (where co-presence is expected) |
| D2 | **Two-source fusion** (AssertionForge, `.6`) | spec-KG + an independent structural KG (e.g. RTL when available) | fact in one source absent in the other | `CrossSourceGapResidual` | EXACT vs the other source (also a genuinely independent capture–recapture occasion) |

## E. Pipeline-conservation detectors (we control both sides → highly detectable, EXACT)

| # | Detector | Inputs | Check | Residual | Class |
| --- | --- | --- | --- | --- | --- |
| E1 | **Inter-stage conservation** | fact identities at stage N and N+1 | a fact at N is absent at N+1 with **no** accompanying residual/decision | `DroppedFactResidual{fact, from_stage, to_stage}` | EXACT |
| E2 | **Symbol-surface conservation** | recovered symbols vs emitted (.isf) | a recovered symbol/constant/enum dropped at emit without a residual | (generalizes the shipped `ISF-RULE-CONFLICT-RESIDUAL` + symbol-count work) | EXACT |

## F. Systematic-blind-spot detector (ontology-level; the only one capture–recapture can't see)

| # | Detector | Inputs | Check | Residual | Class |
| --- | --- | --- | --- | --- | --- |
| F1 | **Coverage-matrix empty cell** | the ontology × modality × extractor matrix (`.2`) | an intent category × modality with **no owning extractor** | `SystematicBlindSpotResidual{category, modality}` | EXACT (the most dangerous miss — silent & total; invisible to A–E and to capture–recapture) |

Current confirmed F1 hits (from `.2`): clock-domains (no first-class type),
enumerations-as-a-queryable-record. (B) single-modality rows are F1/D1 candidates.

## Detector → residual → report

Every detector emits a typed residual into the unifying **`CompletenessReport`**
(framework §10), surfaced by `validate`. The honest headline aggregates: "N
detectors run; K candidate misses surfaced (by kind); estimated residual recall
R% (lower bound, Chao Mh, assumptions printed)."

**Mine-then-formally-filter (GoldMine, `.6`):** where a detector is heuristic
(GATED), candidates may be passed through a stricter confirmation (a checker, a
targeted LLM critic, or a rescan) before being promoted from "candidate miss" to
"confirmed gap" — keeping the report's precision honest.

## Implementation ordering (feeds `.7`)

Cheapest-and-exact first: **C1 symbol closure, C2 register tiling, E1/E2
conservation, F1 empty-cell** (no ML, exact, high-signal) → then **A1 region
accounting** (foundational, needs the classifier + backward index) → then **A2
mutation coverage, B1/B2 schema/star-pattern, D1 cross-modal** → then **D2
two-source fusion** and the capture–recapture gauge (needs heterogeneous
extractors). Each is its own owning implementation tree; several share the
`CompletenessReport` plumbing.
