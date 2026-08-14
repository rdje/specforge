# Behavioral-genericity qualification design

Date: `2026-08-14`
Owner: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.i`
Machine authority: `doctrine/production_genericity/behavioral_qualification.json` and
`doctrine/production_genericity/behavioral_population.tsv`

## Decision summary

Behavioral genericity is a layered relational proof over two source representations, not one equality test.
Byte-identical PDF replay and adversarial filename/document identity perturbation qualify the full rich-capture
path. Symbol alpha-renaming and reviewed text variants use the retained normalized Markdown projection and may
qualify only its downstream text-extraction behavior. They do not qualify PDF conversion, page geometry,
figures, structured tables, or another rich-capture surface absent from direct Markdown SourceIR.

The comparison denominator is closed across SourceIR, EvidenceIR, SemanticIR, IntentIR, and the ISF adapter:
every serialized top-level field and every proof claim is classified by the relation. Permitted presentation,
path, source-span, digest, and copied-symbol changes are declared per transform. Any other delta fails. A
meaning-changing control must produce its declared fact or residual delta while preserving the complete
unaffected complement; an invariant-only comparison must reject it.

## Authority and population census

The proof-current population is the exact 24-key chain-currency retention set. Every key has a ready PDF
SourceIR, a retained normalized Markdown view, and proof-bearing artifacts through adapter lowering. At the
selection boundary all 24 PDF sources were live, hash-verified, and on the repository volume. Three sources are
repository-owned. Twenty-one are authorized external inputs and remain measurable only when a runtime map
resolves each portable basename to the frozen digest on the repository volume; a missing external file is
`unmeasurable`, never passed.

The rich PDF captures total 1,822 pages, 1,995 visual assets, 906 structured tables, 22,088 content elements,
and 3,899 sections. The 24 retained Markdown views total 5,455,286 bytes. Their provider-free replay produced
27,330 EvidenceIR statements, 6,313 SemanticIR semantic records, and 9,543 IntentIR semantic records. The
temporary 24-run census was inspected and its exact `.project-data/tmp/spec-to-intent-f-i-text-census` root was
removed; no census output is retained as undeclared authority.

| Population fact | Exact result |
| --- | ---: |
| proof-current documents | 24 |
| repository-owned / external PDF authority | 3 / 21 |
| reviewed-current overlap / current-only | 7 / 17 |
| normalized-text behaviorally non-vacuous / vacuous | 23 / 1 |
| vendor / family / layout strata | 5 / 15 / 3 |
| prospectively vendor-novel / family-novel documents | 4 / 13 |
| prospective category strata / reviewed category strata | 4 / 6 |

The single text-vacuous document is recorded in the machine population rather than silently removed. Its direct
Markdown replay carries source/evidence envelope and 106 extracted statements but yields zero SemanticIR and
zero IntentIR semantic records. It is therefore eligible for unchanged-PDF and adversarial-identity relations,
but a text alpha or paraphrase pass would be vacuous and is `unmeasurable` until a nonempty supported surface
exists.

## Honest held-out boundary

The 12-document reviewed vertical dataset cannot be called historically unseen: its reviewed labels already
informed earlier source-to-Intent remediation. Seven of those documents overlap the 24 current chains. Those
seven are the reviewed calibration stratum for authoring transform machinery. The other 17 current retained
documents are prospectively held out from behavioral recipe calibration at commit
`8307100e96a7ed6b9f04315949fd2f1c3c232bbf`.

The prospective set spans all three page-layout bands and four categories. Four documents come from two vendors
absent from calibration; 13 belong to families absent from calibration. It has no current-only `register-ip` or
`cpu-isa` denominator, so those prospective category claims are explicitly unmeasurable. The existing reviewed
dataset still supplies six-category evaluation authority, but it is reported as historically exposed review,
not relabeled as a prospective holdout.

Layout is derived without document-specific exceptions:

- `compact`: 1–32 pages;
- `medium`: 33–96 pages;
- `long`: 97 or more pages.

Vendor, family, category, layout, review role, source identity, retained Markdown identity, rich-capture counts,
and text-projection record counts are data rows. The checker contains none of those document labels.

## Input planes and transform relations

### Full PDF capture

`unchanged_source` runs one exact PDF twice in isolated roots and requires byte equality after only scratch-root
substitution. `adversarial_identity` copies identical PDF bytes to deterministic misleading repository-local
filenames. Document identity, source/artifact paths, and identity-bound proof premises must change; rich capture,
canonical facts, residual disposition, validation metrics, and lowering status must not.

This is the only current plane that can qualify Docling conversion, page artifacts, visual assets, structured
tables, content elements, and document sections.

### Normalized text projection

`symbol_alpha` constructs a source-bound bijection only when the baseline has nonempty downstream semantics and
every selected occurrence is unambiguous. Inverse-renaming must make all canonical decisions and proof topology
exact. The declaration may normalize copied spellings, symbol-derived stable ids, source excerpts, and source
content digests. It may not normalize record admission, semantic roles, conflicts, residual disposition,
validation, or lowering.

The retained Markdown is a lossy projection. Direct Markdown SourceIR deliberately has zero page, visual,
structured-table, content-element, and document-section records; EvidenceIR still reads the promoted text. The
baseline and transformed runs therefore compare the same input plane, avoiding a PDF-versus-Markdown
representation confound. Results must carry the explicit rich-capture exclusion.

### Reviewed variants

`structure_preserving_paraphrase` and `harmless_layout` require checked-in review recipes. Each recipe names
every changed source span, preserved conclusion, allowed provenance replacement, and complete unaffected
comparison complement. An unsupported paraphrase or ambiguous parser boundary is invalid, not assumed
equivalent.

`semantic_negative` names one meaning-changing omission, contradiction, relation reversal, value/timing change,
undeclared symbol, misleading identity, proof corruption, or disabled-stage control. It also names the required
changed fact or residual and the unaffected complement. Failure to observe the required change fails the gate;
an undeclared change also fails it.

## Complete stage comparison

Each run must carry all five stages:

| Stage | Closed comparison surface |
| --- | --- |
| SourceIR | every field, selected-plane captures, normalization state, residuals, validation, every proof claim |
| EvidenceIR | every field, capture/conflict/residual/provenance/validation surface, every proof claim |
| SemanticIR | every canonical/conflict/residual/provenance/validation surface, every proof claim |
| IntentIR | every canonical/conflict/residual/provenance/validation surface, every proof claim |
| ISF adapter | lowering status, target model, residuals, validation, emitted bytes, every proof claim |

Array order and cardinality remain part of the relation unless a transform explicitly declares and normalizes a
stable-id substitution. Proof comparison includes claim address, rule id, premise topology, confidence, symbol
use topology, and normalized conclusion. Removing proof fields before comparison is forbidden.

## Evidence and failure semantics

Every document/relation attempt ends in exactly one state:

- `pass`: the complete declared relation holds;
- `fail`: an expected delta is absent, an undeclared semantic/proof/provenance/validation/lowering delta appears,
  or a negative control is not detected;
- `unmeasurable`: exact source authority or a required local provider is unavailable, or the baseline is
  behaviorally vacuous for that relation;
- `invalid`: the population/contract is stale, a transform is ambiguous/nonbijective/escaped, or any run/stage/
  field/proof/delta/complement coverage is partial.

Evidence pins contract, production revision, source, recipe, prior-memory, and tool digests. It publishes exact
declared, attempted, completed, and unmeasurable document sets plus every field/proof/expected-delta/unaffected-
complement denominator. A summary cannot turn an invalid or unmeasurable row into a pass.

## Mechanical enforcement

`scripts/check_behavioral_genericity_contract.py` derives the exact 24-key join among the population table,
chain-currency declaration, reviewed dataset, live SourceIR captures, all five proof-bearing artifact stages,
repository-local Markdown files, and any available PDF authorities. It validates portable paths, source and
Markdown hashes, capture counts, layout bands, review roles, historical exposure, prospective novelty counts,
complete relations, all-stage proof coverage, and the closed failure taxonomy.

Its self-test injects eight controlled faults: a population omission, absolute authority locator, Markdown hash
drift, reviewed-label leakage, vacuity laundering, missing relation, missing comparison stage, and false
population assertion. All eight must fail. The production-genericity wrapper now runs the contract in its
baseline and the mutations in `--self-test`; this gates design currency without claiming that the future
behavioral runs already pass.

## Implementation handoff

`.f.ii.a` may now implement deterministic full-PDF unchanged/identity pairs and text-projection symbol-alpha
pairs in conformance-owned code. It must emit the contract-defined five-stage evidence and preserve the plane
exclusions above. `.f.ii.b` adds only reviewed paraphrase/layout recipes. `.f.ii.c` proves comparator sensitivity
with semantic and authority negatives. `.f.iii` consumes the frozen 17-document prospective split without
using its labels to steer production. `.f.iv` reconciles every measurable and unmeasurable row; `.f.v` alone may
make the final behavioral genericity decision.
