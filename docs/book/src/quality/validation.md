# Validation And Learning

This chapter explains how `specforge` judges its own output quality and how it improves over time without letting learned memory overwrite document truth.

There are two separate concerns here:

- validation:
  deciding how strong, complete, and internally consistent a current artifact is
- learning:
  accumulating reusable extraction priors from earlier validated artifacts

Those concerns are connected, but they are not the same thing.

## Validation is about present-document truth

`specforge validate` operates on one artifact and reports how well that artifact currently holds up.

It measures things like:

- declared-signal direction coverage
- graph-derived direction coverage
- compatibility-direction findings that distinguish flat-hint lag from signals that still lack both flat direction and actor-relative graph coverage
- table-backed signal coverage
- width coverage
- temporal rule cycle-window grounding
- temporal rule actor grounding
- handshake-completion temporal predicates
- infrastructure-signal source and distribution status
- VLM readiness and visual enrichment coverage
- visual semantic grounding
- cross-modality semantic grounding
- negative-knowledge prior matches
- semantic conflicts
- connectivity conflicts
- temporal conflicts
- residual decisions
- overall score and grade

The validator is important because `specforge` is not trying to sound plausible.
It is trying to be inspectable and honest about what was recovered, what remains ambiguous, and where the current artifact is still weak.

## Scores are useful, but not the whole truth

The quality score is a compact summary, not a replacement for the detailed findings.

That matters because:

- a score can stay flat while the artifact becomes more truthful
- a score can even go down when the pipeline stops cheating
- removing false structure is often a real improvement, even if a coarse coverage metric gets stricter

One useful example is `with_table_support` on `SemanticIR` and `IntentIR`.
It reports how many canonical signal records still carry `supporting_table_ids` from structured `SourceIR` tables.
That number is a coverage and explainability signal, not a new truth source: exact signal-to-table provenance is still checked by canonical IR shape and benchmark expectations.
The earlier EvidenceIR metric `table_signal_declaration_provenance` plays the same role one stage earlier: it counts table-synthesized declaration links before canonical signal records exist.

So the right way to read a validation result is:

1. inspect the findings
2. inspect the conflict and residual surfaces
3. then use the score as a compact rollup

## `project-validation`

`project-validation` exists because single-artifact validation is not enough for project steering.

It validates selected artifacts and refreshes the tracked live projection docs, especially:

- `VALIDATION_SNAPSHOT.md`
- the managed validation block in `LIVE_ACHIEVEMENT_STATUS.md`

That command belongs to the continuity plane, but users still benefit from understanding that it is how the project keeps its published local baseline honest.

The tracked baseline is specifically the **last reviewed projection**. Running `project-validation`
mutates the selected artifacts and local rescan plan as well as the two tracked docs, so doctrine does
not invoke it as a freshness check. Instead, a read-only contract verifies the reviewed commit, report
fingerprints and scores, recommendation counts, snapshot/live-block identities, and exact producer
regions. New local results remain review candidates until that tracked boundary is updated atomically.

## Residuals and conflicts are first-class

One of the central design choices in `specforge` is that unresolved ambiguity must stay visible.

That is why the IR and validator preserve things like:

- residual decisions
- interface conflicts
- connectivity conflicts
- semantic-role conflicts
- temporal conflicts

This is not an implementation detail.
It is part of the product philosophy.

Prior memory can make some of those conflict surfaces more informative, but it must not erase them.
For example, `EvidenceIR` validation can report `negative_knowledge_prior_matches` when a current signal-semantic conflict matches a learned caution pattern.
`SemanticIR` and `IntentIR` validation can report the same metric when carried conflict or residual packet patterns match learned negative knowledge.
Those exact matches also surface `negative_knowledge_rescan_recommendations`, `negative_knowledge_corroboration_requirements`, and stage-specific `*_negative_knowledge_rescan_guidance` findings.
That is guidance for targeted rescans and stronger local corroboration, not a correction: the current conflict or residual remains present and still has to be resolved by local evidence and arbitration.
The KG benchmark fixture schema can now assert the related ids on those validation findings, which keeps negative-knowledge guidance tied to the exact conflict or residual packet that matched the learned caution pattern.
`project-validation` is the first consumer for this guidance: it projects those findings into the validation snapshot and writes a generated `generated/validation/rescan_plan.json` target list for later rescan/extractor-selection loops.
It now consumes the EvidenceIR form too, so `evidence_negative_knowledge_rescan_guidance` becomes a bounded `SourceIR -> EvidenceIR -> validate` replay target keyed by the exact conflict or residual ids that triggered the learned caution, while semantic/intent forms keep their downstream rebuild lanes.
The same rescan-guidance channel also covers prior-classified visual-motif evidence that became normative but still needs VLM/multimodal corroboration.
When a matching existing plan already carries executed recommendation summaries, `project-validation` preserves them and projects the review-relevant verdict/delta summary into the tracked validation docs.
That target list is versioned and replay-oriented: each recommendation carries typed replay inputs plus structured command hints for targeted local enrichment, stage rebuild, and follow-up validation, while remaining `planned_not_executed` until an explicit rescan consumer chooses to execute it.
The compact live-status queue now also surfaces the replay-input kind chain and a concise action summary beside each recommendation, so replay scope is visible even from the first-line tracked projection.
Non-decisive semantic-role arbitration is now part of that same guidance surface: when competing role evidence is still non-decisive, validation emits a replay-oriented recommendation to rerun local NLP enrichment on `EvidenceIR` and rebuild the downstream canonical stages before re-validating.
Fallback-only resolved semantic roles now join that same bounded loop too: if canonical role meaning survived without observation-backed `semantic_consensus`, validation emits `semantic_role_consensus_surface_rescan_guidance` so operators can replay the local evidence lane instead of treating provisional meaning as settled truth.
Alias-dependent semantic consensus now joins it as well: if a resolved role still depends only on alias-grounded evidence, validation emits `semantic_alias_dependent_semantic_consensus_surface_rescan_guidance` so the next step is to seek stronger direct or corroborating non-alias grounding, not to silently accept the alias-dependent state as good enough.
Prior-guided final consensus now joins it too: if a resolved role converged with help from learned modality-reliability priors, validation emits `semantic_prior_guided_semantic_consensus_surface_rescan_guidance` so operators can look for stronger current-document corroboration rather than treating the prior-guided state as fully self-sufficient.
At the evidence stage, carried semantic-role conflicts now join it too: validation emits `evidence_signal_semantic_conflict_surface_rescan_guidance` so preserved `semantic_conflict_*` ids become explicit evidence-local NLP replay targets instead of passive upstream disagreement markers.
Canonical signal surfaces that still lack actor-relative graph direction coverage now join it too: validation emits `semantic_graph_direction_coverage_surface_rescan_guidance` so the next step is to revisit local actor/role language and see whether those same signal ids can be rebuilt with graph direction instead of remaining graph-uncovered.
Carried graph-direction self-conflicts now join it as well: validation emits `semantic_graph_direction_conflict_surface_rescan_guidance` so preserved actor-aware conflict ids become explicit replay targets instead of passive same-actor direction disagreement markers.
When one artifact emits both graph-coverage and graph-conflict guidance for the same signal, `project-validation` preserves both replay recommendations at SemanticIR and IntentIR stages: the signal id remains a coverage target, the actor-aware conflict id remains a disagreement target, and each recommendation keeps the expected local NLP enrichment, downstream rebuild, and validation command lane.
Protocol connectivity endpoint gaps now join it too: validation emits `semantic_connectivity_missing_producer_surface_rescan_guidance` or `semantic_connectivity_missing_consumer_surface_rescan_guidance` so signals with only a consumer side or only a producer side become explicit local replay targets instead of passive structural debt.
Infrastructure clock/reset sourcing is intentionally excluded from that replay family, because protocol PDFs often define boundary-visible clock/reset contracts without naming the eventual physical producer.
Carried interface-signal conflicts now join it as well: validation emits `semantic_interface_signal_conflict_surface_rescan_guidance` so preserved direction/width conflict ids become explicit replay targets instead of passive interface disagreement markers.
Carried typed temporal conflicts now join it as well: validation emits `semantic_temporal_conflict_surface_rescan_guidance` so preserved contradiction ids become explicit replay targets instead of passive timing-conflict markers.
Carried signal-connectivity conflicts now join it as well: validation emits `semantic_signal_connectivity_conflict_surface_rescan_guidance` so preserved producer-ambiguity conflict ids become explicit replay targets instead of passive conflict markers.
Carried signal-polarity conflicts now join it as well: validation emits `semantic_signal_polarity_conflict_surface_rescan_guidance` so preserved active-level conflict ids become explicit replay targets instead of passive polarity-disagreement markers.
Carried actor-port gaps now join it as well: validation emits `semantic_actor_port_gap_surface_rescan_guidance` so preserved actor-signal relation ids become explicit replay targets instead of passive relation-only graph debt.
At the source stage, missing VLM enrichment now joins it too: validation emits `source_vlm_enrichment_missing_surface_rescan_guidance` so timing/state diagram asset ids become explicit SourceIR visual replay targets instead of passive source-side enrichment debt.
At the evidence stage, missing VLM observations now join it too: validation emits `evidence_missing_vlm_observations_surface_rescan_guidance` so visual ids without timing/state extraction become explicit source-side visual replay targets instead of passive enrichment debt.
At the evidence stage, structural-KG gaps now join it too: validation emits `evidence_structural_kg_missing_surface_rescan_guidance` so stranded behavioral ids become explicit evidence-local NLP replay targets instead of passive missing-graph debt.
At the evidence stage, partially structured normative residuals now join it too: validation emits `evidence_normative_residual_surface_rescan_guidance` so preserved residual statement ids become explicit evidence-local NLP replay targets instead of passive upstream extraction debt.
At the evidence stage, carried polarity conflicts now join it too: validation emits `evidence_signal_polarity_conflict_surface_rescan_guidance` so preserved `polarity_conflict_*` ids become explicit evidence-local NLP replay targets instead of passive upstream disagreement markers.
Carried signal-semantic conflicts now join it as well: validation emits `semantic_signal_semantic_conflict_surface_rescan_guidance` so preserved semantic-conflict ids become explicit replay targets instead of passive semantic-role disagreement markers.
Typed temporal rules that still lack actor-relative drive/sample grounding now join it too: validation emits `semantic_temporal_actor_grounding_surface_rescan_guidance` so the next step is to revisit local timing language and see whether those same rule ids can be rebuilt with explicit actor grounding instead of remaining actorless.
Typed temporal rules that still lack explicit clock or edge grounding now join it too: validation emits `semantic_temporal_clock_grounding_surface_rescan_guidance` so the next step is to revisit local timing language and see whether those same rule ids can be rebuilt with explicit clock context instead of remaining clockless.
Typed temporal rules that still lack explicit cycle-window bounds now join it as well: validation emits `semantic_temporal_cycle_window_surface_rescan_guidance` so the next step is to revisit local timing language and see whether those same rule ids can be rebuilt with explicit bounds instead of remaining unbounded.
For visual-motif corroboration, the replay sequence is `enrich_source_ir`, rebuild `EvidenceIR`, then validate the current artifact.
The generated enrichment hint now uses a local provider policy: `auto-local` prefers ready Ollama and falls back to ready LM Studio, while explicit `ollama`, `lm-studio`, `skip`, and model overrides remain available on `project-validation`.
`rescan-plan` is the first explicit consumer for that schema: it dry-runs by default, prints artifact path, extractor lane, replay inputs, related ids, and a readable action summary before the command hints, with empty fields rendered as explicit `none` values.
With `--execute`, it dispatches only whitelisted local enrichment/stage rebuild/validate commands from the structured args rather than trusting shell text, and the structured intent must be known and match the SpecForge subcommand lane.
Malformed provider options, including missing providers, repeated provider/model/classification flags, missing provider or model values, flag-shaped provider or model values, unsupported provider values, and unsupported args, are rejected before execution.
It can also scope a multi-document queue with `--document-key <key>`.
Scoped queues still select only pending `planned_not_executed` recommendations; executed matches and missing document keys produce no pending dry-run work.
Limits are applied after that pending/document filter, so unrelated documents cannot consume a scoped queue's limit.
Execution validates before and after the rebuild and records only neutral changed/no-change status, not an improvement claim.
Executed recommendations also persist an `execution_summary` containing before/after validation snapshots, score/finding deltas, added/removed finding ids, and a conservative verdict that distinguishes possible improvement from regression or neutral artifact drift while still requiring review.
Sparse before/after score labels render as score plus grade, score only, grade only, or `n/a` when neither field exists.
Added and removed finding-id lists are sorted and deduplicated before persistence so review diffs do not depend on raw validation-report ordering.
If a delta contains both added and removed finding ids, added findings take regression-review precedence even when the score and total finding count stay flat.
That summary also carries an explicit promotion gate: no-change executions remain `not_promoted_no_change`, and changed executions remain `not_promoted_review_required` until current-document evidence and arbitration policy approve any canonical change.
Score or grade presence-only changes stay in neutral review-required arbitration because they changed the validation surface without proving directional quality movement.
It now also carries `promotion_review`, which records whether human review and an approval record are required, the policy that would have to be satisfied, the required decisions, and the fact that canonical mutation is still not allowed by the rescan executor itself.
That review record is not the approval artifact.
Future approval artifacts remain local/generated by default until a deliberate canonical mutation workflow defines tracked approval evidence with current-document support, validation-delta review, exact mutation scope, prior-memory non-authority, reviewer intent, artifact fingerprints, and replayable provenance.
`converge --rescan-plan <plan>` now reuses that same consumer after the fixed-point loop stabilizes.
It filters the queue to the current source document key, can dry-run the queue, or execute it only when `--execute-rescan-plan` is present, and reports changed outcomes as review-required arbitration state rather than promoting facts.

## Learning depends on validation gates

The learning plane should only absorb reusable knowledge from artifacts that are strong enough to trust.

That means validation is not just a reporting layer.
It is also part of the project’s epistemology:

- weak or unresolved output should not become future memory by default
- stronger, validated output can become advisory prior knowledge

The next chapters explain the benchmark harness and the prior-memory layer in more detail.

## Document class — and why a guide is reported as a guide, not a silent failure

SpecForge is built to digest **any** chip-spec PDF, and the corpus is genuinely mixed: bus
protocols, register/CSR manuals, signal-interface specifications, and a long tail of
programming guides, overviews, and optimization notes. Those last ones are not failures —
they simply carry little *structured design intent*. The danger is that a guide that yields
no signals and no registers looks **identical** to a real specification we failed to extract
from. Both come out as "0", and a bare 0 quietly reads as "nothing here" when the honest
statement is "this document is a narrative, not a contract."

So `validate` now reports a **document class**, inferred purely from *which typed intent
surfaces the extraction produced* — never from the document's name or vendor (the project's
non-negotiable agnostic rule). Four classes:

- **protocol** — the document encodes behaviour: signal constraints (*"PSTRB must be LOW"*),
  or an explicit state machine / serial frame. APB, AHB, AXI, CHI, I²C, and SWD land here.
- **register** — register/CSR records dominate; the primary design intent is a register map.
  The RISC-V Debug spec and the NVMe base spec land here.
- **interface** — a signal inventory plus actor-signal connectivity, but no behavioural
  obligations. Avalon, Wishbone, and TileLink land here.
- **guide** — the **honest floor**: no reliable structured-intent surface is present. A
  programming guide, an architecture overview, an optimization note, or an image-only
  datasheet. Reported as such — *"low structured design-intent … reported honestly, not a
  silent miss"* — with its signal/register/relation/constraint counts shown so you can see it
  was a genuine low-yield document, not a dropped one.

One detail is worth calling out because it shaped the design. SpecForge's "conditional rule"
extractor (*"if X then Y"*) is **over-produced** on narrative prose — a programming guide can
carry a dozen conditional-rule sentences while containing zero signals, registers, or
constraints. Counting those as "behaviour" would mislabel guides as protocols. So the
classifier deliberately **excludes conditional rules from the decision** and routes only on
the low-noise surfaces (registers, signal constraints, actor-signal relations, the
declared-signal inventory, and the FSM / frame surfaces). The conditional-rule count is still
shown in a guide's rationale, flagged as *"narrative … not class-determining"*, so the
reasoning is transparent.

The classifier (`crate::ir::completeness::classify_document`) is a pure, unit-tested function
of a small structural census, with only generic numeric floors as constants — no chip,
vendor, or protocol vocabulary. `validate` prints a **Document Class** section, adds a
`document_class` metric, and records an `evidence_document_class` Info finding, so the class
travels in the persisted validation report.

### Reading what the document says it is — and catching a spec we under-extracted

There is one failure the structural class cannot tell apart on its own: a document that comes
out `guide` because it *is* a guide, versus one that comes out `guide` because it is a real
specification we **failed to extract from** (a spec that is mostly diagrams and rendered
tables — the part the deterministic pipeline can't yet read). Both are structurally empty;
both would be reported as low-yield guides. The second one is a genuine miss hiding as an
honest "guide", and silently calling it a guide would be the exact over-confidence we are
trying to avoid.

A chip-spec PDF almost always **states what it is, in plain words, in its early pages** — the
title and the first-chapter headings. So `validate` reads that front-matter and infers a
self-declared document type from generic vocabulary: *guide / tutorial / "learn the
architecture" / application note* on one side, *specification / architecture / protocol /
standard / datasheet / reference manual* on the other. (Guide phrasings deliberately win over
spec words — Arm's "Learn the architecture …" series is a guide even though it contains
"architecture" — and "overview" / "introduction" are excluded because every specification has
an introduction chapter.) As always, the vocabulary is generic document-type words, never a
chip, vendor, or protocol name.

That self-declared type does two things. For a doc that already has a real structural class
(protocol / register / interface) it is shown as corroboration in the rationale. For a doc
that came out `guide`, it is the deciding signal:

- declares a **guide** (or nothing recognizable) → a confirmed, honest low-intent guide.
- declares a **specification / architecture / standard** → this is **not** a true guide. It is
  a real design document we under-extracted, so `validate` raises a **Warning**
  (`evidence_document_underextracted_spec`) routing it to the VLM frontier instead of letting
  it pass as low-intent.

Across the corpus this split is real: of the 16 documents that come out `guide` structurally,
the front-matter confirms 11 as genuine guides (a CPU software-optimization guide, "Learn the
architecture" notes, …) and flags 5 as under-extracted specifications — the RISC-V Advanced
Interrupt *Architecture*, a *JEDEC STANDARD* HBM DRAM spec, the CoreSight Base System
*Architecture*, and two more. Those five are now visible candidates for deeper (image-aware)
extraction rather than silent zeros. The hint (`front_matter_doc_type_hint`) is a pure,
unit-tested function over the front-matter text, and `validate` surfaces it as a
`document_type_declared` metric.

### What is the document *about*? — the purpose category

The structural class answers "what *shape* of intent did we extract?" — but two documents with the
same shape can have very different **purposes**. A wire bus protocol and a CPU register manual are both
"contracts", yet what it means to capture them *completely* — and how they lower to `.isf` — is entirely
different. So alongside the class, `validate` now reports a richer **document intent category**: the
six-category purpose taxonomy from the [Chip-Spec Document Categories](../document-categories.md) chapter,
naming what the document is *about*.

The six categories are `wire-protocol`, `register-or-platform`, `cpu-isa`, `physical-link`,
`methodology-guide`, and an honest `unresolved`. It is reported as a `document_intent_category` metric (plus
`document_intent_category_confidence`) and an `evidence_document_intent_category` Info finding, beside the
structural class — it does not replace `document_class`, it **consumes it as one input**.

Two design choices make it trustworthy rather than a confident-sounding guess:

- **It only claims high confidence where the evidence is unambiguous.** A clean wire-behavioural shape →
  `wire-protocol`, and a front-matter guide self-declaration → `methodology-guide`, are reported at **high**
  confidence. Everything else is **low** confidence with an explicit **residual** — the honest sentence
  saying *why* it could not decide. The corpus census proved this is necessary: typed-surface counts simply
  cannot separate a register-IP (category 2) from a platform/system-IP (category 3) — both are
  register/structure-dominant — so the recognizer reports the combined `register-or-platform` with a
  residual rather than inventing a split. Likewise a CPU ISA has no distinct structural signature, and a
  physical-layer spec is indistinguishable from a guide by structure alone, so both rest on a generic
  front-matter self-declaration and otherwise fall through to an honest `unresolved`.
- **A register count never vetoes a real wire protocol.** Many bus protocols also carry a register map
  (AXI has 71 registers — and 348 actor-signal relations). Classifying on the *dominant* surface, the
  recognizer weighs the wire surface against the register/structure surface, so AXI is correctly
  `wire-protocol` while a register manual that merely mentions a few signals is not. The subtle case is
  message/packet **fields**: they are wire (flit) intent for a packet protocol like CHI or DTI, but
  in-memory *structure* intent for a register IP like NVMe or AMD-IOMMU. The measured, name-list-free
  discriminator is the presence of a register map — flit fields count toward wire intent only when the
  document declares no registers. A register-heavy protocol whose wire shape is outweighed is reported as
  `register-or-platform`, but its residual says so explicitly (*"may be a register-heavy wire protocol"*),
  so nothing is silently misfiled.

Across the corpus this lands as 21 `wire-protocol` and 8 `methodology-guide` at high confidence — every one
verified genuinely correct, **zero** high-confidence mislabels — with the register/platform, ISA, PHY, and
genuinely-ambiguous documents reported honestly at low confidence with their residuals. Like every other
classifier here, `classify_document_intent_category` is a pure, unit-tested function over the structural
census plus generic document-type vocabulary — never a chip, vendor, or protocol name.

### How complete is what we extracted? — the class-aware completeness gauge

Knowing a document's *class* answers "what kind of thing is this?". The next honest question
is "of the design intent we *did* pull out, how much is fully formed?" A register the pipeline
found but for which it never captured the field breakdown, a signal with no resolved
direction, an intent-bearing table that produced no record — each is a real, locatable gap.
Left unsaid, they make a partial extraction look finished. So `validate` reports a **per-document
completeness gauge**: a short, honest tally of how complete the produced intent is.

The crucial design choice is that the gauge is **class-aware**. It would be meaningless — and
actively misleading — to hold a programming guide to "every register must have fields" when a
guide has no registers by nature. So a document classed `guide` is reported as **not
applicable**: there is no design surface whose completeness to gauge, and that is stated
plainly rather than scored as a 0%. (If a "guide" is actually an under-extracted spec, the
`.5c` warning above already flags it.) For the three design-document classes — protocol,
register, interface — the gauge measures each dimension only when that dimension actually
exists in the document. A protocol with no registers shows no register gap; a register
document *is* held to "every register has fields and a resolved width". Nothing is invented:
every counted item is one the extraction itself produced but left incomplete.

The dimensions are:

- **`registers_without_fields`** — registers captured as a bare entry with no field breakdown.
- **`registers_unresolved_width`** — the mandatory-width flag: a register is physical
  bit-storage, so an unresolved width is a real gap (parametric, e.g. XLEN, or defined in
  another document), not an optional attribute.
- **`signals_without_direction`** — declared signals with no resolved direction (input/output).
- **`unexplained_intent_bearing_tables`** — register/signal/timing tables the classifier
  recognized that nonetheless produced no record.

Each gap reports `missing / total` plus a small sample of the affected names so a reviewer can
go straight to them. `validate` prints a **Document Completeness Gauge** block and adds a
`document_completeness_gaps` metric (and `document_completeness_applicable`). Across the corpus
the gauge reads true to each document's character: AMBA APB comes out a protocol with a single
signal-direction gap and every table explained; the RISC-V Debug *register* spec is held to its
60 registers and honestly shows all 60 widths unresolved (they are XLEN-parametric — the same
gap the per-fact gold measured); a GIC overview *guide* reads "not applicable". The gauge is a
pure, unit-tested function (`document_completeness_gauge`) keyed off the document class, so it
adds no extraction behavior — it only makes the existing incompleteness visible instead of
silent.

Two structural quirks of real PDFs used to distort that table accounting, and both are now
handled honestly. First, **header-trapped data rows**: the PDF backend sometimes marks every
row-label cell of a table as a header cell, leaving the table's `body_rows` empty — common in
the multi-page *signal presence* matrices bus specs use to summarize which signals exist in
which interface class. Such a matrix usually restates signals that are already declared
elsewhere, so the coverage check now reads those recovered rows (the same structural rule the
timing extractor uses — one shared definition, so the two can never drift) and recognizes a
fully-redundant presentation as *covered* instead of flagging it forever. The strictness is
unchanged: one signal in the matrix that is *not* already declared keeps the table flagged as a
real candidate miss. Second, **continuation fragments that lost their classification**: a table
split across pages appears as a captioned head (*"Table B2.2: Summary of signal presence…"*)
plus `Continued from previous page` fragments, and the backend occasionally drops a fragment's
table kind to `unknown` — which silently removed it from the accounting altogether. A fragment
now inherits its head's kind when two independent facts ground the join: the fragment's own
caption states the parent table reference, *and* its first header row matches the head's
exactly. On the AMBA AXI spec this honesty pass closed seven of the flagged presence-matrix
tables (all their signals were already declared), brought four previously invisible fragments
into the accounting, and on AMBA APB it explained the last unexplained table — while on the LTI
spec the same pass *raised* the count by one, because a garbled presence fragment that had been
invisible genuinely carries uncaptured signal-presence content. Both directions are the same
property: the gauge reports what is actually there.

Those presence matrices now also have their own typed surface (`signal_presence_records` — see
the [EvidenceIR chapter](../pipeline/evidenceir.md)), and the accounting consumes it directly: a
matrix whose every identifier-led row was captured into presence records counts as *explained*
even when its signals appear nowhere else (the AXI generic-name family `AxVALID`/`AxADDR`… has
no literal declaration anywhere, yet its matrix is no longer a candidate miss because its
content now lives in typed records). The strictness mirrors the inventory rule: one row the
capture refused — a fused cell, a garbled fragment — keeps the whole table flagged, so partial
capture never hides a miss. Measured on the canonical artifacts without any rebuild, this
closed the AXI generic family (32→31 unexplained), one ACE snoop-channel fragment (36→35), and
both LTI presence fragments (6→4), while the garble-class fragments stay honestly flagged. The
report itself gains four metrics — `signal_presence_records`, `signal_presence_signals`,
`signal_presence_conditioned`, `signal_presence_variant_labels` — and an
`evidence_signal_presence_inventory` Info finding emitted only when the surface is non-empty
(absence is not an event).

The typed **message-field inventory** (the in-memory structure and packet-field surface built
by the bit-position and field-titled table readers) is also on the report: five metrics —
`message_field_records`, `message_field_containers`, `message_fields_with_bit_range`,
`message_fields_with_byte_offset`, `message_field_constraints` — plus an
`evidence_message_field_inventory` Info finding when the surface is non-empty (an NVMe rebuild
reports *"216 field(s) across 113 container(s), 216 with literal bit positions"*). Two
deliberate non-integrations keep the report honest: the inventory does **not** join the
document-class decision (measured on the real field-bearing documents, neither would change
class, and generalizing from two data points would be exactly the kind of overfitting the
class rules avoid), and it does **not** add a completeness-gauge dimension (a width-only field
table states no bit positions — counting "fields without positions" as a gap would mislabel
the document's own honest absence as an extraction failure).

### How *correct* is what we extracted? — the standing extraction-quality gauge

Completeness asks "did we get everything?"; the natural sibling question is "**is what we got
actually right?**" SpecForge answers it with the NLI-oracle extraction-quality gauge (the
entailment check described in the [architecture rationale](../architecture-rationale.md)): for
every extracted signal constraint, a text model is asked whether the constraint's own source
sentence *entails* the constraint-as-a-claim. The fraction judged **not entailed** is a cheap,
automatic estimate of how erroneous the constraint surface is — and it is now a **standing,
persisted measurement**, not a one-off terminal report:

- **`specforge nli-verify <evidence_ir.json>`** runs the measurement and back-annotates an
  `extraction_quality_gauge` record into the artifact (model, counts, and the exact ids of the
  not-entailed constraints).
- **`specforge converge`** re-runs the same measurement automatically after the pipeline
  stabilizes, so every full run ends with a fresh per-document quality report in its summary.
- **`specforge validate <evidence_ir.json>`** then reports the persisted gauge **without
  needing any model**: an `Extraction-Quality Gauge` block, the metrics
  `extraction_quality_labeled`, `extraction_quality_not_entailed`,
  `extraction_quality_abstained`, and `extraction_quality_not_entailed_pct` (all `n/a` until a
  measurement exists — never a fabricated verdict), and an Info finding whose `related_ids` are
  the not-entailed constraint ids, so review goes straight to the items.

Two warnings keep the report honest:

- **`evidence_extraction_quality_majority_not_entailed`** fires when *more than half* of the
  labeled constraints are not entailed — a scale-free "more wrong than right" line, not a
  corpus-tuned threshold. Live on the persisted corpus this separates documents exactly as the
  hand-validated measurements did: AMBA APB sits at 4/14 (28.6%, Info only), while the
  pattern-extracted AXI surface reads 91/100 (91%) and is flagged as majority-erroneous — the
  honest statement that its canonical constraint surface still carries the old pattern
  extractor's quality.
- **`evidence_extraction_quality_gauge_stale`** fires when the constraint surface changed since
  the measurement (different count, or a measured constraint id that no longer exists — which
  is exactly what happens when `extract-constraints-llm` replaces the surface). A stale gauge
  is still shown, but never allowed to masquerade as a current one.

Like every gauge in this chapter, it observes and reports — it never mutates extraction truth.
A measurement that labeled nothing (model unreachable, every verdict unknown) is not persisted
at all, so a dead provider can never erase a real prior measurement. The whole reporting path
is provider-free and fixture-locked in the tracked KG benchmark
(`extraction_quality_gauge_persisted_gold`), and the NLI oracle itself remains a *noisy*
estimate — treat the number as a signal for review priority, not as ground truth.
*Authoritative tracking:* `docs/tasks/EXTRACTION-QUALITY-GAUGE.md` (leaf `.0`).

### Catching a stale downstream stage — when an artifact silently drops what its upstream found

The pipeline runs in stages: `evidence → semantic → intent`. If you rebuild one stage but
forget to cascade the next (the per-stage commands don't auto-cascade — only `converge` rebuilds
the whole chain), the downstream artifact goes **stale**: it still holds the *old* result while
its upstream has moved on. The sharpest form of that is silent loss — an `intent_ir.json` that
carries **zero** actor-signal relations while its `semantic_ir.json` upstream carries dozens.
Nothing errors; the relations just quietly vanish from the canonical surface.

`validate` now catches exactly this. When an IntentIR (or SemanticIR) carries no actor-signal
relations, it loads its upstream (the artifact records the path it was built from) and, if the
upstream *does* carry relations, raises a **Warning** — `intent_stale_relations_dropped` or
`semantic_stale_relations_dropped` — telling you to re-run that stage (or `converge`) to recover
them. It is deliberately a **zero-versus-some** test, not a count comparison, because that makes
it free of false alarms: the agent-identity gates that clean up the actor surface
(consolidating, splitting, dropping phantom roles) only ever *re-attribute* relations, they never
empty a non-empty set — so an empty-downstream/non-empty-upstream split can only be staleness. A
register or command protocol that genuinely has no wire-signal relations (its intent lives in its
register and message-field surfaces) has an empty upstream too, so it is correctly left silent —
honest absence is not a stale drop. The check costs nothing on healthy documents (it only reads
the upstream when the downstream is suspiciously empty). *Authoritative tracking:*
`docs/tasks/CORPUS-COVERAGE.md` (leaf `.1`).

## Closed task trees — how each was implemented and verified

### `PROVENANCE-HARDENING` — provenance fields always have assertions

Every provenance-like field — `supporting_*_ids`,
`automation_confidence`, `strongest_automation_confidence`,
provenance vectors, evidence-span ids — on every IR record type
must have at least one non-empty test assertion where the field
is populated. The tree audited the IR surface, identified
provenance fields with no assertion, and added regression-only
assertions per field. The doctrine recorded: *provenance is what
makes residuals honest; an unasserted provenance field is a
silent fabrication surface*. Verified by per-field audit + the
full `scripts/run_ci.sh`. *Authoritative tracking:*
`docs/tasks/PROVENANCE-HARDENING.md`.

### `R7-VALIDATION` — what `specforge validate` does for you today

If you run `specforge validate <artifact.json>`, this section
tells you what kinds of observations you should expect to see
in the report — and what they mean. The `R7-VALIDATION` tree
is the work that landed the additional findings and metrics
beyond the originals; reading this gives you the full
inventory of validate-time surfaces grouped by what each one
helps you catch.

#### The user-facing guarantee

> **Whenever you run `specforge validate`, you get a typed
> report (findings + metrics + an additive
> `applied_mutations` field, currently empty by design — see
> below) — covering temporal-rule capture gaps, KG-quality
> baselines, and adapter-artifact structure. Every finding
> tells you what was observed, where, and whether you should
> act on it.**

`Info` means "we noticed this; here's the signal." `Warning`
means "this is likely worth your attention." `Error` means
"this is a hard correctness issue." The validator is
read-only — it never silently rewrites your IR (see the
"Tracked approval evidence" section below for the design
that makes the read-only-by-default contract structural even
if canonical IR mutation is ever introduced).

#### The four findings/metrics surfaces this tree delivered

**Temporal handshake completion gap** — when your
`interfaces[].signal_records` carry the
`HandshakeValidLike` / `HandshakeReadyLike` semantic roles
but no temporal rule expresses a `HandshakeComplete`
predicate over them, you see an `Info` finding listing the
affected signal names. *"Your spec has the signals; the
extracted temporal rules don't yet bind them into a
completion predicate."* The rescan-guidance pointer aims at
the temporal-grounding surface so the next pass knows where
to look.

**Temporal multi-predicate antecedent flag** — when a
temporal rule's antecedent carries more than one predicate
(`signal A high AND signal B held value V` rather than just
one), an `Info` finding flags the count and lists the rule
ids. It isn't a correctness error — multi-predicate
antecedents are valid — but it's a coverage signal: a rule
that fires only when several conditions co-occur has fewer
witnesses than a rule that fires on a single trigger, so
the extractor's confidence in it should be lower until you
confirm.

**KG-quality benchmark findings** — three measurable signals
about how well the knowledge graph is grounded:

- **Graph-direction coverage**: what fraction of signals
  have a non-`Unknown` actor-relative direction.
- **Semantic-role resolution rate**: what fraction of
  signals carry a typed semantic role (clock / reset /
  data / handshake).
- **Consensus coverage rate**: what fraction of signals
  have agreement across the contributing extractors.

Each surfaces with its current rate and the conservative 50%
floor threshold. Below the threshold ⇒ `Info` finding with
the threshold and the actual rate in the message; you can
see *"we expect at least 50% here; we're at 32%"* without
reading source.

**`.isf` adapter validation target** — when you pass
`specforge validate` an adapter artifact (the shape the build
pipeline emits), the command auto-detects the adapter kind
from the artifact's `stage` field and runs the per-adapter
checks. `.isf` is SpecForge's sole adapter target (the
downstream FSMGen tool owns `.fsm`/HDL lowering beyond it), so
the auto-detected validator is the ISF coverage that shipped in
`R6-ISF-ADAPTER`: six findings covering payload presence
(`isf_adapter_artifact_missing_isf_payload`), schema-version
freshness (`isf_adapter_schema_version_unexpected`),
renderability (`isf_adapter_not_renderable`), a non-empty
signal inventory (`isf_adapter_empty_signal_inventory`), a
non-empty behavioral surface — transactions or rules
(`isf_adapter_no_behavior`), and residual-decision visibility
(`isf_adapter_residual_decisions_present`).

You don't need to know which validator to invoke — the
auto-detection picks it for you. Failing checks surface as
typed findings in the same `ValidationReportRecord` format
as every other validate output.

#### What this buys you, as a SpecForge user

- **Capture gaps are visible.** When the temporal extractor
  hasn't yet bound a handshake or a multi-predicate
  antecedent, you see it in the report; you don't have to
  read the IR yourself to find the gap.
- **KG quality has a baseline.** The three benchmark metrics
  give you a "is this getting better or worse?" number per
  document, against a stable threshold; drift is observable
  not anecdotal.
- **Adapter validation is one command.** `specforge
  validate <adapter.json>` auto-detects the adapter kind from
  the artifact and runs the matching surface — today that is
  `.isf` (SpecForge's sole adapter target); any future target
  would slot into the same auto-detected, one-command flow.
- **Validation never silently mutates your IR.** The
  report is additive; nothing about your input artifact
  changes. (The "Tracked approval evidence" section below
  designs the only path through which that could ever
  change — and that path is opt-in, signed by you, and fully
  audited.)

Every leaf in this tree landed under the per-leaf signoff
discipline — `scripts/run_ci.sh` green; idiomatic clippy /
fmt fixes only, per the `SIGNOFF-REMEDIATION` doctrine
(see `reference/live-docs.md`). *Authoritative tracking:*
`docs/tasks/R7-VALIDATION.md`.

### Tracked approval evidence for canonical IR mutation (`R7-VALIDATION.5` design)

This section explains a design — not yet implemented — for the
single case in which `specforge` would ever modify your IR
after the build stages have produced it. Reading this end-to-end
should leave you confident about two things: (1) **today**,
`specforge` never modifies your IR during validation, full stop;
and (2) **if** that ever changes, the change will be opt-in,
explicit, signed by you, and fully audited.

#### What the IR contract looks like today

Today the validation pipeline is **read-only**. When you run
`specforge validate <artifact.json>`, the validator inspects the
IR and emits two kinds of additive observations:

- **Findings** (`ValidationFindingRecord`) — things the
  validator noticed. *"You have ready/valid signals but no
  `HandshakeComplete` temporal rule covering them."* Info,
  Warning, or Error severity. They do not change the IR.
- **Metrics** (`ValidationMetricRecord`) — counts and rates the
  validator measured. *"Graph-direction coverage = 73%."* They
  do not change the IR either.

Findings and metrics flow back into the report you read; the IR
that the build stages emitted stays exactly as it was. That
read-only contract is what makes a `specforge` pipeline
**reproducible** — run validation twice on the same input, you
get the same report; the IR you built upstream is never silently
edited under your feet.

#### What "canonical IR mutation" would mean — and why it's risky

Imagine the validator finds something it thinks it can fix:

> "Contract `c-37` is marked `Residual{reason="under-determined
> delay"}`, but I see a follow-up table on page 14 that pins
> the bound to `min=2, max=4`. I could flip this to `Lowerable`
> with that bound."

Letting the validator just *do that* would be **canonical IR
mutation** — the validator writing back to the IR it was meant
only to observe. It would be tempting because it would close a
real gap automatically. It would also be dangerous, for three
reasons every `specforge` user should know:

1. **Reproducibility breaks.** Two validation runs against the
   same input could now produce different IRs, depending on
   which mutations the validator decided to apply.
2. **Fabrication becomes possible.** The whole point of the
   three structural honesty doctrines `R16` introduced —
   *fidelity Fail → Residual*, *fusion disagreement →
   Residual*, *entailment Fail → Residual* — is that
   `specforge` refuses to silently invent a contract it can't
   ground. A validator that's allowed to "fix" the IR
   unilaterally is exactly the surface those doctrines were
   built to prevent.
3. **Provenance gets opaque.** A reader of the final IR has no
   way to know *what* the validator changed, *why*, *when*, or
   *who* sanctioned it. The audit trail vanishes.

This design's job is to make the useful case (a human-approved
correction) possible **without** opening the door to any of the
three risks above.

#### The mental model: every change comes with an approval card

The whole design comes down to one rule, and you can hold it in
your head as a single sentence:

> **The IR can only be changed if you hand the validator a
> signed "approval card" that says exactly what to change and
> why — and the validator double-checks the card matches the
> change before and after.**

Everything below is the typed shape of that approval card
(`ApprovalRecord`), the single function that's allowed to read
the card and apply the change (`apply_approved_mutation`), the
place the cards live in your repo (`ApprovalStore`), the
permanent receipt the validator writes after applying a card
(`ValidationReportRecord.applied_mutations`), and the four
safety rules that make sure no card can ever be forged, lost,
or applied to the wrong IR.

#### `ApprovalRecord` — what an approval card carries

Every change carries an `ApprovalRecord`. Here are the fields,
in plain language:

- `approval_id` — a stable string you can refer to in
  conversation and in tickets (`"approval-2026-axi-aw-stage"`).
- `approver` — who signed the card. Either a `Human { userid,
  evidence }` (you, with `evidence: SignedCommit { commit_sha }`
  or `SignedFile { path, sig }` or `PullRequestApproval {
  pr_url, approver_login }` — whichever your team's process
  uses to prove an approval is real), or a `SystemProcess {
  process_id, parent_approval }` for the cases where one
  approval explicitly chains to another (e.g. a batch
  remediation script that was itself human-approved).
- `scope: MutationScope` — exactly what changes:
  - `ir_stage` — which stage's IR (`SemanticIR`, `IntentIR`,
    `IsfAdapter`, `FsmAdapter`).
  - `ir_path` — a serde-JSON path into the IR
    (`actor_contracts[3].lowering`).
  - `value_before` — the JSON value that *must* be at that path
    before the change.
  - `value_after` — the JSON value that *must* be at that path
    after the change.
- `justification` — the human-readable reason. *"Confirmed
  pinned bound 2..4 from the §3.4 timing table; original
  Residual reason no longer applies."*
- `timestamp_utc` — RFC 3339, recorded once when you sign the
  card. Doesn't update on re-application.
- `related_finding_ids` — the validator's own finding ids that
  motivated this change. If your `value_before` came from an
  Info finding, that finding's id goes here. This is the audit
  thread back to *why* the change exists.
- `content_hash` — a SHA-256 over the load-bearing fields
  (`scope`, `justification`, `timestamp_utc`,
  `related_finding_ids`). If anyone tampers with the card,
  the hash stops matching, and the validator refuses to apply
  it. (A later phase can upgrade this to a detached PGP or
  ed25519 signature bound to `approver.evidence`; the algorithm
  swaps under the same field name.)

#### `ApprovalStore` — where the cards live

The cards live in **your repo**, under version control, in an
append-only JSONL file (default: `.specforge/approvals.jsonl`).

- **Append-only** is the structural contract. You add a card;
  you never delete one. If a previously-applied mutation turns
  out to be wrong, the fix is a *new* approval whose
  `value_before` is the post-state of the wrong one and whose
  `value_after` reverses it. Both cards stay in the store.
  The audit trail is the full history, not a "current view"
  that could lie.
- **Version-controlled** means `git blame` works on the
  approvals themselves — who added a card, in which commit,
  is recoverable from your normal repo history. No new tooling
  needed.
- **Per-record content hash** plus **per-store git history**
  give you two independent tamper-detection layers.

#### `apply_approved_mutation` — the single function that's allowed to change the IR

Every canonical IR mutation flows through one function. Any
other code path that writes to the IR in the validation context
is, by definition, a bug. Here's the algorithm in plain
English:

1. **Look up the card in the store.** If `approval_id` isn't
   present, **refuse** (`MutationError::UnknownApproval`).
2. **Re-compute the card's `content_hash`.** If it doesn't
   match what's written on the card, **refuse**
   (`MutationError::TamperedApproval`).
3. **Read the IR at `scope.ir_path`.** If the value there
   doesn't match `scope.value_before`, **refuse**
   (`MutationError::DriftedSource`). This catches the case
   where the IR has moved since the card was signed —
   e.g. an upstream re-extraction changed the contract you
   were approving a fix for.
4. **Apply the change.** This is the *only* moment any IR
   field gets written. It's a small closure provided by the
   caller that knows how to set the typed field at `ir_path`.
5. **Read the IR at `scope.ir_path` again.** If the value
   there doesn't now match `scope.value_after`, **refuse**
   (`MutationError::PostStateMismatch`) — meaning the closure
   did something different from what the card promised.
6. Return an `AppliedMutationRecord` capturing the approval
   id, the scope, and the apply-time timestamp.

Notice what this gives you: a "refuse to act" outcome from
*any* of the four checks. Nothing about your IR can change
unless every one of (present, untampered, matching-before,
matching-after) is true. Validation that runs with no cards in
the store — today's default — never enters this function at
all.

#### `ValidationReportRecord.applied_mutations` — what you see afterwards

After validation runs, the report you read carries a new
additive field: `applied_mutations: Vec<AppliedMutationRecord>`.

- **Empty** in the default read-only case. *Your report looks
  byte-for-byte the same as before this design ever existed.*
  This is the zero-artifact-churn discipline we've applied
  across every recent `specforge` typed addition: the surface
  only takes up space when it has content.
- **Non-empty** when one or more mutations were applied. Each
  record carries the approval id (so you can find the card in
  the store), the scope (so you can see the exact change),
  and the apply-time timestamp. Re-reading the report tells
  you everything about what the validator did to your IR.

#### When an approval is required

Any change to any typed IR field through the validation
pipeline requires a matching `ApprovalRecord`. That's the
default policy, and it's enforced *structurally*: the only
code path that writes to the IR is the function above, and
that function won't run without a present, untampered,
matched-before-and-after card.

The validator's normal response to "I think the IR could be
improved here" is **not** to change anything. It's to emit:

- a `Finding { severity: Warning }` that says what it would
  change, and
- a **draft `ApprovalRecord` template** the operator can review,
  fill in (the `justification`, the `approver.evidence`), and
  add to the store.

Nothing happens until you, the operator, sign the card. This
is the read-only-by-default property preserved verbatim — what
changes is that you now have a typed, auditable path to opt
in to a specific correction, one at a time.

#### How this slots into the existing validation pipeline

The functions you already use (`validate_semantic_ir`,
`validate_intent_ir`, `validate_isf_adapter`,
`validate_fsm_adapter`) keep their existing signatures and
keep returning the same `ValidationReportRecord`. The design
adds **one** optional pass-through hook on each:

> `with_approved_mutations(&ApprovalStore)`

When you pass `Some(store)`, the validator runs the approved
mutations through `apply_approved_mutation` **before** the
read-only check pass runs. That ordering matters: findings
should observe the *post-mutation* IR, so a fix you've
approved removes the finding it was approved against — not the
other way around. Every successful application gets appended
to `ValidationReportRecord.applied_mutations`.

When you pass `None` (or omit the hook entirely), the pipeline
is byte-for-byte identical to today's behaviour. The opt-in is
explicit, per-run.

#### Audit-by-absence — how we'll prove no one snuck around the single entry point

A future tree (sketch id: `R7-MUTATION-PATHWAY-IMPL`) will add
a **static audit** to the test suite: a build-time check that
no `ir.<field> = …` assignment, no `.push(…)`/`.insert(…)`/
`.remove(…)` on a tracked IR collection, exists *outside*
`apply_approved_mutation`. The check is mechanical (grep- or
syn-tree-based) and runs in CI. Combined with the single-entry-
point algorithm above, this gives you a structural guarantee:
*"if validation changed an IR field, it went through the
audited path."*

#### The four protections you get, framed as user benefits

The same four properties expressed as what they buy you:

1. **Refuse-by-default** — *You never lose work to a silent
   automated rewrite.* If the validator can't verify a change
   end-to-end against an approval card you signed, it doesn't
   happen.
2. **Fully diff-able** — *You can always see exactly what
   changed.* The `value_before` and `value_after` are right
   there on the card, in JSON, beside the `ir_path` they apply
   to. No "the validator did something." Always: *this* field,
   *this* before, *this* after.
3. **Provenance-bearing** — *You can always trace a change
   back to why.* Every applied mutation links to the
   `Finding`s the validator emitted that motivated it. The
   audit thread runs all the way back to the observation that
   started the conversation.
4. **Append-only** — *You can't lose the history.* The audit
   trail is the full history of approvals + applied mutations.
   Reversing a wrong change is a new approval, not a deletion;
   even the "undo" is on the record.

Together with `R16`'s three structural honesty doctrines —
*fidelity Fail → Residual*, *fusion disagreement → Residual*,
*entailment Fail → Residual* — these four properties extend
the "fabrication is mechanically impossible" guarantee across
the validation pipeline, even after canonical IR mutation is
introduced.

#### What this design does **not** do (and what comes next)

To set expectations honestly:

- **No code ships from `R7-VALIDATION.5`.** This is a design
  deliverable. Today's `specforge validate` behaves exactly
  as it did before this section was written — read-only, no
  approval store, no `applied_mutations` field in the report.
- **The default never flips.** Even after a future
  implementation tree (sketch id: `R7-MUTATION-PATHWAY-IMPL`)
  lands the typed `approval` module + the `with_approved_mutations`
  hook, running validation without an approval store leaves the
  IR untouched. The opt-in is explicit and per-run.
- **CLI / UX is not part of this design.** How you'll *propose*
  an approval (the validator emitting a draft card),
  *interactively review* it, *sign* it (the bridge to your
  team's signing process), and *commit* it to the store is a
  downstream UX leaf when the implementation lands.
- **Phase 1 tamper-evidence uses `content_hash` (SHA-256).**
  Phase 2 can upgrade to detached PGP or ed25519 signatures
  bound to `approver.evidence`. The `content_hash` field name
  stays; the algorithm behind it swaps.

*Authoritative tracking:* `docs/tasks/R7-VALIDATION.md` — the
"Design (`.5` output, 2026-05-20)" section is the full
specification this chapter explains.

### `AMBIGUITY-PHRASE-DETECTOR` — flagging vague prose so it isn't mistaken for precise

Specifications are not all equally precise. A sentence like *"PADDR must remain stable until
the transfer completes"* pins down exactly what has to happen; *"the reset value is
implementation-defined"* or *"…handled as appropriate"* does the opposite — it tells you the
spec is **leaving something open**. SpecForge would happily extract from both and, without
help, present the vague one with the same confidence as the precise one. That is the kind of
quiet over-confidence the residual-honesty doctrine exists to prevent.

So `validate` now runs a small, grounded **weak-phrase detector**. Requirements-engineering
research (the NASA Automated Requirement Measurement work — Wilson, Rosenberg & Hyatt, ICSE
1997 — and Berry & Kamsties' ambiguity handbook) catalogued the phrases that mark
under-specified language: *"as appropriate"*, *"if necessary"*, *"and/or"*, *"to be
determined" / "TBD"*, *"but not limited to"*, and — for chip specs especially —
*"implementation-defined"* and *"vendor-specific"*. The detector scans the extracted
statements for these and surfaces an **Ambiguity / Weak Phrases** section plus an
`ambiguous_statements` metric and an `evidence_ambiguous_statements` finding listing the
flagged statements.

Two deliberate choices keep it honest and quiet:

- **It flags, it does not delete.** Nothing is dropped or down-ranked; the finding is an
  `Info` signpost ("review these before relying on extraction precision"). Extraction is
  completely unchanged — this only adds a note to the validation report.
- **Modal verbs are *not* weak phrases.** MUST / SHALL / SHOULD / MAY carry *normative
  strength* (handled by the constraint and obligation extraction), not vagueness, so flagging
  every "may" would be noise. The lexicon excludes them on purpose.

The detector itself (`crate::ir::ambiguity::weak_phrase_findings`) is a pure, unit-tested
function over the statement text. Routing flagged statements into typed residual decision
packets — rather than only noting them — is a deliberate future step.

*Authoritative tracking:* `docs/tasks/AMBIGUITY-PHRASE-DETECTOR.md`.

### `EXTRACTOR-ARCHITECTURE` — the unified extractor framework and its run manifest

SpecForge derives each typed surface (FSM states, semantic hints, registers, actors, serial-frame fields,
packet operations, signal polarity, actor–signal relations) by running a small set of independent
**extractor** strategies and merging their results.
As of `EXTRACTOR-ARCHITECTURE`, those strategies are first-class units driven by one shared engine — and that
engine records a **run manifest**: for every surface, which extractors were *eligible*, which *fired*, and
how many records each produced and kept. `validate <evidence>` surfaces it as an Info finding plus two
metrics (`extraction_manifest_surfaces`, `extraction_extractors_fired`), for example:

```
- [info:extraction_manifest] extraction run manifest (8 framework surface(s), 5 extractor(s) fired):
  register_records[registers.field_table] signal_polarities[] actor_signal_relations[relations.prose]
  protocol_states[] protocol_actors[actors.prose]
  signal_semantic_hints[semantic_hints.tables,semantic_hints.prose] serial_frame_fields[] swd_operations[]
```

Read that as: on this document, register records came from the field-table strategy (not the register-map
one), no FSM strategy fired, actors came from prose, the actor–signal relations were grounded in prose
sentences (no signal table carried a `Source`/`Destination` column), signal meaning came from tables and
prose (not from visual/VLM annotations), and the polarity, serial-frame, and packet-operation surfaces ran
but found nothing — this is a register document, not a polarity-rich bus protocol or a serial-protocol spec.
On a serial-debug spec the same line shows
`serial_frame_fields[serial_frame.bit_range] swd_operations[operations.prose]`, on a CAN-style spec the
frame comes from the prose composition list: `serial_frame_fields[serial_frame.composition]`, and on an
AMBA-style bus protocol the polarity and relation surfaces show where those facts came from:
`signal_polarities[signal_polarity.prose,signal_polarity.tables]
actor_signal_relations[relations.prose,relations.tables]` — on the AXI specification, for example, the
manifest reveals that relations are overwhelmingly table-driven (hundreds from the channel signal tables,
a couple of dozen from prose), while on a serial-debug spec they are pure prose. "Ran and found
nothing" is deliberately distinct from "never ran" — both are honest, but they tell you different things.

The polarity and relation surfaces are also the first **convergence-loop** surfaces on the engine: SpecForge
re-extracts them on every pass of its fixed-point evidence loop (each pass can discover new signals whose
prose then yields new polarity facts and new relation edges), and the manifest keeps exactly the *final,
converged* pass's run — so the counts you see describe the evidence that actually shipped, not an
intermediate pass.

**Why this matters to you.** It answers a question that used to require reading the code — *"which part of
the pipeline actually produced these facts, and which strategies found nothing here?"* — turning the
extraction into something you can inspect per document. It is also a compact **behavioural fingerprint** of a
document: two specs that fire the same strategies in the same proportions tend to be organised the same way.
That is exactly the signal SpecForge's cross-document pattern-reuse plane clusters on, so a new spec can be
recognised as "like ones we've seen" and benefit from what worked on them. The manifest is a faithful
observation — it reports what ran, never inflates it — and it is deterministic for a given document.

**How it was built, and how you know it changed nothing.** The framework did not rewrite any extraction
logic — the grammars that read your PDFs are exactly the ones that earned the wire-based 100% scores. What
changed is the *wiring*: each strategy became a named, registered unit with an explicit applicability gate,
run by one shared engine with two merge modes (first-wins key-merge, and ordered concatenation for surfaces
whose post-passes need the full merged list). Surfaces that re-run inside SpecForge's fixed-point evidence
loop (signal polarity, actor–signal relations) record on every pass, and the manifest keeps the final,
converged run. A few producers stay deliberately off the engine where their behaviour is inherently
sequential — for example the constraint family, whose record ids are minted across three extractors in
order — because forcing them through a merge driver would change behaviour. Every migration step was
verified the same way: rebuild the evidence for the full set of repo-backed specification PDFs before and
after the change and require the output to be **byte-identical** (the tracked KG benchmark and the per-fact
evaluation suites run on top of that). So the inspectability you get here cost nothing in extraction
behaviour — the facts are the same facts.

*Authoritative tracking:* `docs/tasks/EXTRACTOR-ARCHITECTURE.md` (tree complete: framework `.2`, eight
migrated surfaces `.3`–`.9c`, the three-category model `.9d`, and the orchestrator consolidation `.10a`);
see also `docs/tasks/CORPUS-PATTERN-REUSE.md`.
