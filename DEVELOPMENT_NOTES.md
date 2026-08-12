# DEVELOPMENT_NOTES
## SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.iii (`2026-08-13`) — replayable proposals, not semantic self-attestation

EvidenceIR is the first stage where deterministic extraction, current-document text, scoped priors, and bounded
model-assisted enrichment meet. Its schema-3 proof context therefore captures the exact normalized Markdown, the
exact validated prior if one was used, and every approved post-build mutation as an ordered typed proposal. The
verifier reconstructs the complete artifact from verified SourceIR plus those inputs and compares each registered
conclusion exactly; the proposal is evidence for replay, never a claim that a model response is inherently true.

The cumulative ledger composes rather than restarts. All verified SourceIR claims are retained byte-for-byte as
the prefix, then 39 field roots across 11 EvidenceIR families and all populated record/alias claims are appended.
Registered derivations are exact relations: each claim binds its output bytes to the already verified global
stage replay, and nested derivations must name an existing dependency with matching input/output digests. Registry
validation is once per ledger, keeping verification linear in the number of claims rather than recursively
replaying the whole artifact for every record.

Mutation authority is closed by kind and field set. The application commands that enrich NLP facts, conditions,
constraints, contracts, signal relations, register bits, NLI quality, or validation reports must reconstruct an
exact predecessor, cite existing source support, apply only their allowed patch, and extend proof before writing.
The former carry-forward behavior that silently imported selected fields from an older EvidenceIR was removed;
unrecorded state cannot regain canonical authority.

Only 24 documents retain verifiable SourceIR capture inputs. Their schema-3 EvidenceIR migration changed no
extracted field, and their SemanticIR replay is likewise unchanged. The 54 remaining schema-2 EvidenceIRs stay
inspection-only until owned source recapture is possible. Chain currency treats this closed legacy/proofless
condition as an explicit unmeasurable EvidenceIR-to-SemanticIR frontier, never as current and never as an
exception for a stale proof. IntentIR and adapter replay remain 78/78 stage-locally reproducible from the
persisted SemanticIRs because those stages have not crossed their proof migrations; that is not an end-to-end
proof-current claim.

## SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.ii (`2026-08-12`) — a hash is not a derivation

SourceIR is the first canonical stage to cross the proof boundary. Its proof context records the exact
pre-classification field projection, while its ledger records field-root and per-record conclusions. Each rule
id resolves through the current closed registry to an executable verifier and a digest of that implementation.
Reload therefore checks the actual premise-to-conclusion relation against exact bytes. An editor who changes a
field and recomputes its conclusion hash has not supplied the missing derivation and is rejected.

Classification makes the distinction concrete. The captured visual, table, or section kind is neutralized to
`unknown`; the current structural Rust classifier must reconstruct the persisted conclusion. Table records also
carry their exact cells as typed premises, visual/page records carry typed regions, and grounded model refinements
carry the exact response with direct typed grounding. Model output can propose a refinement but cannot validate
itself or overwrite an unrelated record. Validation is downstream evaluation over already proved claims, not a
new semantic oracle.

The migration did not bless 78 historical artifacts. Only the 24 SourceIR documents with declared retained
bundles were eligible. The maintenance-only feature verified their source size, promoted capture products,
page/visual manifests, and referenced assets before applying current generic classifications and proofs. The
other 54 stay proofless and inspection-only. All 24 eligible chains were rebuilt through the adapter; exact
currency is 24/24 measurable EvidenceIR and 78/78 downstream, with zero stale artifacts.

Conformance required a separate authority class. Tests may contain named specifications and may patch synthetic
records, but those values now live in typed noncanonical overlays. The overlays expose no canonical serializer or
writer, and production EvidenceIR/SemanticIR/IntentIR writers reload their canonical upstream files. This keeps
specific examples useful as adversarial evidence without letting them become production extraction policy.

This is stage-one closure, not whole-pipeline proof closure. EvidenceIR, SemanticIR, IntentIR, and ISF rule
migrations remain `.e.iv.iii`–`.e.iv.vi`; the AST-aware information-flow doctrine in `.e.v` remains the final
structural answer. The finite forbidden-vocabulary census remains a useful diagnostic only.

## SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.i (`2026-08-12`) — temporary storage is not a trust boundary

A constructor census alone would leave canonical bypasses. Existing artifacts can be carried forward,
deduplicated, reclassified, provider-enriched, register-recovered, signal-resolved, or validation-backannotated
after the principal builder returns. Conformance code can also patch temporary stage structs and then call an
ordinary production writer or downstream builder. The exact reviewed graph therefore owns producers, mutators,
and sinks together: 38 families, 168 field-root rules, 113 current producer/mutator entrypoints, 39 canonical
seams, and four explicit conformance-only bypass obligations.

The proof design is cumulative rather than one disconnected ledger per stage. SourceIR proves normalized capture
before classification. Each later stage verifies the exact upstream artifact and ledger, retains those claims as
an ordered prefix, and appends its own conclusions. Every top-level field receives a root proof even when empty;
non-empty record collections also receive stable per-record proofs. This gives completeness a finite denominator
without pretending that the ledger metadata must prove itself recursively.

Canonical authority is checked when a stage loads, serializes, writes, feeds a downstream production build, or
lowers to ISF. Direct public mutation makes the recorded conclusion digests stale until a registered mutator
extends the ledger. Historical proofless input stays inspectable and can rebuild or residualize, but cannot be
wrapped in synthetic proof. Conformance fixtures will use noncanonical typed overlays that cannot call canonical
sinks; an evaluation-only storage rebase must preserve proof bytes and convey no semantic permission.

The TSV graph is a reviewed high-level migration contract, deliberately not the final static proof. The later
AST/information-flow doctrine must derive raw decision/helper closure, fail on parse or macro uncertainty, and
prove no unregistered construction path exists. This division makes the stage migrations reviewable without
weakening the eventual closed-world gate.

## SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iii (`2026-08-12`) — authority is an unforgeable verification witness

Persisted provenance is not authority. Any public record or deserializable ledger can be copied, edited, or
forged, so a schema-valid proof envelope cannot be allowed to authorize itself. The new kernel returns a
non-deserializable `VerifiedProofLedger` only after checking the closed ruleset, exact current-document premise
catalog, upstream proof order, unique claim address, allowed symbol operations, and canonical conclusion digest
together. Legacy/stale envelopes request rebuild or residualization; malformed/future envelopes reject.

Opaque input identity follows the same rule. `OpaqueSymbol` retains private spelling for persistence,
presentation, and target encoding, but ordinary semantics receives only scope plus source ordinal and exact
identity. It implements neither display/string conversion, spelling order, nor Serde. Four sealed capability
tokens are unconstructible outside the kernel, and external compile-fail tests prove the forbidden surface stays
absent. This constrains information flow rather than trying to enumerate every possible protocol word or alias.

This slice deliberately does not wrap old claims in synthetic proofs. The kernel and schema are live, but no
current artifact gains proof authority until `.e.iv` registers and migrates every producer/carry/residual/lowering
family. A temporary, documented dead-code allowance covers only those sealed internal entrypoints and must leave
when the first production registry lands. That sequencing preserves the difference between installing a checker
and claiming the existing pipeline already passed it.

## SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.ii (`2026-08-12`) — dependency direction is a type-system boundary

The neutral core cannot be protected by a directory naming convention inside one crate: Rust privacy cannot stop
core modules from importing an oracle that shares their crate. The workspace now gives that rule to Cargo. The
core package compiles the staged IR and lowering surfaces with no SpecForge-internal dependency; conformance owns
measurement, category characterization, replay, trajectory, and reviewed fixtures and depends only one way on
core. The application is the sole composition layer and re-exports legacy paths, so user behavior remains stable.

The physical source move is deliberately smaller than the semantic move. Existing large modules are path-linked
into their owning package while the compiler graph changes immediately; later proof/rule slices can reorganize
files without mixing that churn with dependency authority. Public visibility grew only where application or
conformance consumers cross the new crate boundary. A manifest/module checker tests direct and aliased reverse
edges, an application cycle, and oracle reinsertion; Cargo compilation remains the independent authority.

This makes conformance feedback unrepresentable as a core import, but it does not authenticate core conclusions.
The next slices must still seal identity observation, require kernel-checked derivations, register each inference,
and enforce the resulting information-flow topology. That distinction is why the vocabulary diagnostic can be
empty while whole-core genericity remains honestly open.

## SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.i (`2026-08-12`) — freeze the denominator before the migration

The neutrality repair is intentionally not starting as a mass code move. Moving modules first would change the
surface being audited and make it possible to mistake migration churn for complete coverage. The accepted design
therefore freezes two independent joins before any production edit: every compiled Rust module has one target
plane/lane, and every top-level canonical artifact field has one claim family/proof class/lane. A live-set checker
rejects additions, omissions, duplicates, malformed rows, and stale rows; its self-test proves both primary drift
classes fail and cleans its repository-volume fixture on success or failure.

The chosen enforcement unit is derivation authority, not vocabulary. A sealed capability controls who may observe
source spelling; a registered rule turns typed evidence into a grounded proposal; one small kernel validates the
proof and returns a proved claim. The AST scan planned for `.e.v` is defense in depth around those compiler/type
boundaries. ADR 0038 owns the full architecture and makes explicit that linguistic completeness remains a
qualification problem rather than something a proof ledger can pretend to solve.

## SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.iv (`2026-08-12`) — qualification is compositional

The three identity-remediation children change different authority boundaries, so their closure cannot be inferred
from a token census or from any one unit test. The bounded qualification composes four properties: document identity
cannot select prior memory; symbol spelling cannot select semantics or lowering; provider policy can use current-
document symbols only as opaque grounded data; and corpus organization derives only typed structural facets.
Current-binary chain currency proves those policies describe the persisted product, while focused alpha, identity,
prompt, spelling, name-only, and fail-closed proposal controls exercise the cross-path invariants.

The expected recall loss is evidence, not failure: 37 stale prior-created timings, 53 evidence constraints, 2,891
invariants, 42 transactions, and 27 formerly renderable adapters depended on authority the source did not justify.
None is restored through a named exception; source statements and explicit residuals preserve the recovery frontier.
This closes `.d`, not the complete neutrality program. `.e` must make the rule structurally unrepresentable to violate,
and `.f` must qualify the complete population under renaming, identity perturbation, paraphrase, and held-out inputs.

## SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.iii (`2026-08-12`) — executable policy is structural

Prompts are production policy even when their outputs are independently validated. The model-facing surfaces
now describe typed digital-hardware relations, constraints, diagrams, tables, registers, and entailment while
treating every document-owned symbol as opaque. Text prompts receive only current-document evidence and closed
declaration catalogs; visual prompts admit only visible labels and structure. The complete constructor census
found that entity typing still exposed the raw identifier and invoked model world knowledge. It now replaces that
identifier in context and helper transport with `<OPAQUE_IDENTIFIER>`, leaving typed evidence and grammar only.
Contract signals outside the independent catalog become residuals, and the LLM-primary constraint proposal cannot
validate its own subject. Catalog order follows declaration/provenance order rather than spelling, because sorting
opaque names would make prompt order change under alpha-renaming. No real vendor/protocol example remains.

The corpus-KB command had a stronger identity leak: fixture-name substrings selected a protocol-family page and
classified candidate evidence as gold or negative. `KgBenchFixtureStructureProfile` now derives capabilities
solely from populated typed fixture fields, ignoring every string value. The ten managed Markdown outputs group
the same 156 tracked conformance fixtures into seven structural capability pages; the named AMBA page is gone.
Manifest schema 2 separately reports typed-prior presence and typed control surfaces, which may overlap. Exact
fixture names remain provenance/display data and cannot decide membership or readiness.

This is one bounded repair, not whole-core signoff. `.d.iv` qualifies the combined identity work; `.e` still
owns the proof-carrying production/conformance module boundary and `.f` owns behavioral metamorphic replay.

## SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.ii (`2026-08-12`) — neutrality requires positive authority

Deleting a word denylist or uppercase check is not sufficient. During retained-corpus replay, two prose readers
started treating the first word of an aside as a signal, and a descriptive invariant reader started treating
ordinary “state”/“timing” prose as canonical. Those were latent grammar defects previously hidden by case-based
rejection. The repair is positive and structural: a parenthetical name must fill the parentheses, a pin
appositive must be bounded, a parenthesis cannot rename an already marked signal, and descriptive behavior must
mention an exactly declared signal. No replacement vocabulary list was introduced.

The same principle holds downstream. A model proposal cannot author the declaration that later validates it;
roles cannot be reconstructed from fragments; and a target grammar's mandatory clock/reset fields cannot justify
inventing design intent. Diagnostic placeholders make the absence inspectable while renderability stays blocked.
This explains the deliberate 44→17 renderability change and is the architectural reason alpha-renaming is a
semantic obligation rather than a token-census metric.

## SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.i (`2026-08-12`) — prior identity is provenance, never policy

The prior path violated the neutral-core boundary at its selector. `ProtocolFamily::infer` inspected a document
key and display name, then scoped seven EvidenceIR/SemanticIR prior families. This was stronger than an unfortunate
identifier: renaming an identical input could change the lookup set.

Schema 7 removes that selector. `PriorScope` has only `Global`; loaders receive only the persisted prior path,
learning groups normalized evidence and typed/structural keys globally, and document identity remains provenance.
The current deserializer rejects any other scope. Legacy schemas are not harmlessly compatible: their family
partition is semantic authority, so schemas 1–6 clear the seven scoped prior arrays before typed load. The
identity-independent structural-profile array may survive. This prevents a migration from converting old
family-conditioned knowledge into apparently neutral global truth.

The compatibility work exposed a separate feedback-integrity defect. The old schema-6 store declared 14 accepted
IntentIR sources, one already deleted. Its 13 remaining current sources had no embedded persisted validation, so
the store could not be rebuilt under its own `requires_validated_intent_ir` policy. The correct disposition was
not to preserve the 1.49-MiB authority. The current sources were validated, the store relearned, every affected
chain replayed, and the changed learning inputs validated again.

The loop converged in two iterations: the first neutral replay removed one masked residual, changing the learned
negative set from 12 to 11; the next learn and a third confirmation were byte-identical at SHA-256
`a416cc8b7b6bd84947e4b6780d5253b13c2985c57ca4e8f1c02d6daf56239633`. The final store contains 29 actor, 83
semantic, four modality, 443 temporal, zero table, 1,251 visual, 11 negative, and two profile priors. A global
`manager` role contradiction is surfaced as contested rather than hidden by family partitions.

ADR 0025 replay is exact at 24/24 measurable EvidenceIR and 78/78 downstream. Compared with the original
family-scoped state, the neutral validated store increases globally applicable prior metadata only where current
normalized evidence matches. One source chain loses 37 timing constraints that existed solely through the stale
table prior; its 37 statements remain available for later grounded recovery. This leaf does not claim whole-core
neutrality: signal spelling and prompt/corpus paths remain owned by the next children.

## FSMGEN-REFRESH-INTEGRATE-8.1 (`2026-08-12`) — pin-only upstream compatibility

The exact FSMGen fast-forward is four commits, from `a51dcdad0a7e752e638abfe3ab414f7f3911889d` to
`c0d8b668db2527108d1c23c20d184400c51efea6`. The changed-path audit covers 15 files: hosted mdBook workflow
repair, two AHB regression-corpus count restorations, terminal-failure inventory closure, and the upstream
memory/task/knowledge projections that record those changes.

No path in FSMGen's public ISF specification, downstream integration specification, parser, IR, lowering,
SpecForge response, or release-contract surfaces changes. This makes the correct downstream integration a pin
refresh, not a speculative compatibility edit. The seven SpecForge real-binary strict canaries pass at the new
detached, clean pin. Current pin copies move with the gitlink; historical hashes remain evidence of their own
review boundaries.

## SPEC-TO-INTENT-ALIGNMENT.6d.ii.b (`2026-08-12`) — SourceIR fails closed before semantics

The first genericity remediation seam is SourceIR because every downstream extractor consumes its visual,
table, and section labels. The old embedded Python mixed useful document grammar with reviewed-corpus cues:
operation/burst captions could become timing diagrams, participant/interface phrases could name diagram and
section kinds, binary/field-reference contents could become encodings without a declared mapping role, and a
named packet-layout denylist attempted to patch register overclassification. Substring matching also let a
generic fragment inside an unrelated word create authority.

The replacement is deliberately narrower. Caption classification recognizes only explicitly named generic
visual forms. Section grammar matches whole normalized phrases. Table classification requires typed role
conjunctions: signal identity plus direction (or explicit signal identity plus width), value plus meaning,
register identity plus access plus address/range, scalar timing roles plus context, or feature plus support.
Operation, participant, filename, vendor/protocol, and arbitrary symbol spelling never participate. Ambiguous
forms remain `unknown`; no negative vocabulary is needed to enumerate what a table is not.

Changing only the writer would have left a bypass: 78 retained schema-1 SourceIR files already contain the old
semantic labels. SourceIR schema 2 therefore treats those files as lossless source capture but not current type
authority. Load neutralizes every legacy diagram/table/section label to `unknown`/`normative`, records why, and
requires re-ingest to regain typed labels; a future schema is rejected.

A read-only application of the new writer policy to all 78 retained SourceIRs measured the tradeoff before the
blanket legacy downgrade: 2,293 diagram labels, 2,123 section labels, and 3,484 table labels would change. The
largest removals are 1,918 register-bitfield and 317 timing-diagram guesses, 1,954 encoding guesses, 218 feature
guesses, and 136 timing guesses. Conversely, 754 unknown tables acquire complete structural register-map
authority because column order no longer controls recognition. This is a large, intentional recall reset—not
evidence against feasibility. The source structure was always present; universal or grounded recovery can be
rebuilt without preserving corpus-specific shortcuts.

The first all-tier currency gate then made the compatibility effect concrete: ten of 24 EvidenceIR chains with
retained normalized inputs differed, while all 78 later stages were still current against their old inputs.
ADR 0025 therefore required same-leaf reconciliation. A repository-volume backup captured the exact 80 old
files (39,847,014 bytes; aggregate SHA-256 `d5a59914b9c6b56f266f5959b1d396024decf3ce9919bd37ff547474da1e3a5f`),
then each affected chain was rebuilt as EvidenceIR → SemanticIR → IntentIR → ISF adapter and all 30 typed IR
artifacts were validated. The two renderable ISFs pass the pinned FSMGen strict checker with zero diagnostics.

The attributed changes are deliberately not hidden behind aggregate currency. Four chains change only visual
roles. Four documents lose 194 old table-derived timing records (55 + 60 + 5 + 74) because their tables do not
satisfy the new timing-role conjunction. The debug-architecture chain loses weak table-derived actor/register
authority (21→5 actor relations, 45→33 registers, 195→125 contracts) while adding four grounded section-register
records. One discovery/configuration chain replaces one weak register guess with 16 structurally complete
field-layout registers; one USB chain loses six weak register guesses; another gains one message-field record
while losing one weak register guess and eleven figure-label-derived invariants. These are exact temporary
recall/precision consequences, not a claim that the missing intent is absent. `.6d.ii.c` and `.6d.ii.d` own
generic EvidenceIR recovery and identity-routing removal; restoring any result through a document-specific cue
would violate the north star.

After rebuild, exact replay currency is 24/24 measurable EvidenceIR and 78/78 SemanticIR, IntentIR, and ISF
adapter chains. The 54 EvidenceIR chains whose normalized bundles were reclaimed remain explicitly
unmeasurable until their owned re-ingest; no stale measurable artifact remains.

Signoff is complete for this seam: all eight doctrines, formatting, warning-deny Clippy, 1,873 tests with six
ignored and zero failures, warning-deny rustdoc, mdBook test/build, and final project-data locality pass. The
exact rollback root and temporary comparison script are removed and absent. This closes SourceIR only; it does
not weaken or pre-approve the protocol-specific EvidenceIR surfaces owned by `.6d.ii.c`.

## SPEC-TO-INTENT-ALIGNMENT.6d.ii.a (`2026-08-12`) — replay truth exposes the production-genericity breach

The clean replay starts only after `.6d.i` commits, so all 12 sources and 48 stages bind production revision
`b977a51f`. Four source authorities are repository-relative; eight read-only authorities are hash-equal, reside
on the repository's SSD filesystem, and enter only through an untracked runtime map. Durable evidence retains
portable ids, digests, byte counts, and repository-relative scratch paths—never a host path.

Fresh root/revision metadata changes stage identities, but the complete 14-cell reviewed projection localizes
semantic causality exactly. Only the two OpenCAPI analog cells change. `IL(f)|21 dB`,
`ILD(f)|0.45 dB_RMS`, and `IL(f)|30 dB` disappear from all three canonical projections and reappear unchanged
as source-linked residual keys at SemanticIR and IntentIR. Each residual carries non-empty reason, boundary, and
replay data, so both reviewed cells are actionable and accounted. The four UI digital-skew facts, every other
reviewed cell, all 24 prior true positives, and 29/29 canonical provenance remain exact.

TP/FP/FN become 24/2/16. The evaluator counts residual actionability per required cell/stage observation, hence
the two repaired cells contribute four met observations (4/24), not three key-level observations. Source
disposition becomes 6/14 and required-modality document accounting 4/12; conservation remains 72/88 and the 16
unexplained canonical drops are unchanged. Physical-link is now `supported`; the other five categories remain
`incomplete` for their own measured defects.

Controller v6 therefore cannot jump to `.7`. Its replay and provenance hard gates are clear, but the zero-
fabrication gate retains two unrelated violations: one AMD IOMMU packed-layout key and one GIC-400 summary-
caption key. The hard-first owner is new pending `.6e`, followed by `.7`'s 16 source losses, `.8`'s 20 remaining
inactionable residual observations, and `.9`'s five omitted capability islands. The promoted manifest, result,
input, and report hashes are `81f1758b…2c855`, `78e27515…5075`, `274311e6…0c04`, and `ccb8a112…b5d8`.
After promotion, the exact 3,913-file / 1,090,948-KiB replay root and runtime source map were removed and proved
absent.

The owner review of the exact OpenCAPI assertions triggered a complete production-pipeline audit rather than a
local vocabulary cleanup. The denominator is all 71 Rust sources, including embedded Docling Python, optional
VLM/text/NLI paths, prior learning/use, validation/evaluation, corpus commands, trajectory composition, and both
adapters. The audit confirms that the generic stage architecture is reusable, but the compiled production
surface is not neutral: document identity selects named prior families; EvidenceIR exposes and produces a named
serial protocol schema; signal spellings infer semantic roles; PDF classifiers contain corpus-calibrated
phrases; prompts narrow the input domain; and production comments/public names normalize specific examples.

A finite token denylist cannot close that class of failure: it misses unseen aliases, hashes, thresholds,
translated fragments, schema coupling, and neutral-looking copied phrases. Signoff therefore requires a one-way
production-core/conformance boundary, opaque input identities and symbols, audited grammar APIs for raw text,
source-grounded promotion/residuals, AST/dependency mutation tests, and behavioral invariance under symbol
renaming, identity perturbation, paraphrase, and held-out documents. The reviewed snapshot composer is the first
boundary correction: it now exists only in `#[cfg(test)]` support, while the generic evaluator/controller stays
in production.

The audit does not imply that neutral extraction is theoretically impossible. Universal digital-intent types
and document grammar are legitimate priors; document identity, named signals, and corpus phrases are not.
Perfect recovery from absent, contradictory, or unreadable source is impossible, so the signoff contract is one
identity-independent engine that promotes justified intent and makes every unsupported or undecidable item an
explicit residual. Remediation may reduce benchmark recall temporarily; keeping a named shortcut is not an
acceptable way to preserve it.

## SPEC-TO-INTENT-ALIGNMENT.6c.ii (`2026-08-12`) — provenance closure exposes the next honesty family

The clean replay was intentionally run only after `.6c.i` committed. Its production revision is therefore the
exact `74a658b3` boundary, not a dirty-tree approximation. All 12 reviewed source hashes match their prior
authority, including the eight caller-authorized read-only inputs on the repository's SSD volume; the runner
copies those inputs into repository-local scratch before use and persists no host path.

Every stage artifact has a new identity because the isolated JSON carries the fresh replay root and the current
production revision. The reviewed projection provides the causal comparison: only five cells change. I2S moves
from 0/5/5/5 to 5/0/0/0 true-positive/false-positive/false-negative/unprovenanced counts because the five exact
values now retain `ns` and `table_0004`. Four correct OpenCAPI digital-skew records and three incorrect analog
channel-loss records gain table provenance only. No other reviewed cell changes, and all 19 prior true positives
survive.

The new aggregate is 24/5/16 TP/FP/FN, 29/29 provenance, 72/88 conservation, 4/14 source disposition, and 2/12
required-modality document accounting. The result is important structurally: provenance is no longer mixed into
the next decision. The controller still fails the zero-fabrication hard gate on five exact records and selects a
new owned `.6d` leaf. Its largest bounded family is three OpenCAPI analog channel-loss records; AMD IOMMU and
GIC-400 account for one remaining fabricated register key each. Recall recovery under `.7` stays second.

The replay orchestrator and projection now require caller-provided portable replay, owner, and dataset identities,
so a later qualification cannot silently publish stale `.6b.ii.b` names. After portable evidence and the result
were promoted, the exact 3,913-file / 1,090,908-KiB root and runtime source map were removed and proved absent.

## SPEC-TO-INTENT-ALIGNMENT.6c.i (`2026-08-12`) — the unit existed before the carrier

I2S `table_0004` was already recognized as timing and its five scalar rows were already exact. The missing value
was table-wide metadata: SourceIR retained `all values in ns` in the caption, but
`synthesize_timing_constraints` read only a dedicated unit column. The same producer initialized statement
support empty even though each record came directly from a structured table, and `TimingConstraintRecord` had
no separate home for that authority. SemanticIR and IntentIR then correctly cloned the incomplete record.

The repair is structural and closed. `timing_caption_unit` accepts only the complete `all value(s) in <unit>` cue
and a bounded timing-unit vocabulary. It preserves source spelling, refuses unrelated unit mentions, and is used
only when the row has no explicit unit. Every table-derived timing row now records its table in
`supporting_table_ids`, distinct from `supporting_statement_ids`. The field is default-empty and omitted while
absent, so pre-carrier artifacts and timing observations from other modalities remain compatible.

The fixed-input retained-corpus replay identified exactly five stale EvidenceIR artifacts: OpenCAPI 25G,
OpenCAPI 32G, I2C, I2S, and USB 3.2. Their 55/60/37/5/74 timing cardinalities do not change. All 231 rows gain
one existing SourceIR table id; only the five I2S rows also change unit from absent to `ns`. The exact 41-file /
25,308-KiB pre-change cascades were copied within the repository volume before replacement. After rebuild and
validation, all 15 EvidenceIR/SemanticIR/IntentIR artifacts equal those backups after neutralizing only the new
carrier and the five intended units. All five adapter JSON files and the sole emitted `.isf` are byte-identical.
Whole-chain replay is current at EvidenceIR 24/24 plus SemanticIR/IntentIR/adapters 78/78.

The dirty-tree real-source probe and the retained SourceIR reconciliation answer different questions. The probe
proved current ingest bytes can carry all five facts through the stages; rebuilding from the retained SourceIR
kept ADR 0025 isolation sharp and avoided importing unrelated current-ingest classifier changes. Clean committed-
revision replay of all 12 reviewed sources and publication of the exact quality/controller delta remain owned
only by `.6c.ii`.

After hash-verifying all 41 rollback files, the complete 89-file / 89,012-KiB reconciliation workspace and the
64-file / 11,532-KiB real-source probe were removed. Both exact roots and the standalone initial-currency log are
absent; no background result remains to consume.

Signoff passes all eight doctrines including whole-chain currency, formatting, warning-deny Clippy, 1,866 Rust
tests with five ignored and zero failures, warning-deny rustdoc, mdBook, and final project-data locality.

## SPEC-TO-INTENT-ALIGNMENT.6b.iii (`2026-08-12`) — activation is a resource decision

The 400-page failure did not come from page-range conversion or the autonomous RAM guard. Rust already adapted
the size of an active batch from total RAM, but bounded *activation* remained a separate embedded-Python default:
512 pages. Worse, failure of the cheap `pypdfium2` page count silently selected the same single-pass path. When
the backend was killed by the operating system, Rust collapsed the missing exit code into an ordinary external
command error. The safety mechanisms existed, but the selection and error seams did not compose them.

The repaired default observes total physical RAM once and uses it for both decisions. Activation budgets 40% of
that fixed capacity against the measured 64-page / roughly 4.8-GiB Docling working set—75 MB per page—and clamps
the result to 1..399 pages. This is deliberately conservative: unnecessary batching costs time, while an unsafe
single pass loses the process. The known 400-page risk remains capped even on a larger host; the current 24-GiB
host resolves 131 pages and the existing adaptive ladder resolves 64 pages per batch. Fixed total RAM, rather
than momentary free RAM, keeps the choice deterministic. Explicit nonnegative threshold overrides remain exact;
bad or absent values return to policy rather than disabling it.

The embedded helper now refuses conversion when it cannot count pages, receives both resolved values explicitly,
and records threshold, batch size, and batched state in its metadata. On Unix, signal termination becomes typed
`IngestTerminatedBySignal`; its diagnostic says truthfully that a signal may reflect external resource
enforcement but does not prove OOM. The existing in-process guard remains the only path that can assert measured
memory breach through `IngestAbortedForMemory`. Both failure paths discard staging and retain the last good bundle.

The live proof used no threshold override. Arm Debug selected 131/64 and reproduced 400 pages, 210 tables, 176
figures, 6,784 elements, and 1,321 sections. Profile, structured tables, content elements, and sections are
byte-identical to the retained SourceIR; page artifacts and visual assets are identical after neutralizing their
root path. EvidenceIR, SemanticIR, and IntentIR are likewise byte-identical after path normalization and removal
of validation backannotations. The exact 795-file / 184,164-KiB replay root is removed and absent.

Focused policy, helper-contract, process-status, environment-propagation, and staged-swap tests pass. Full CI
passes all eight doctrines including chain currency, formatting, warning-deny Clippy, 1,866 Rust tests with five
ignored and zero failures, warning-deny rustdoc, mdBook, and final project-data locality. The leaf is ready for
its closure commit; `.6c` remains the next semantic work.

## LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.0 (`2026-08-08`) — measured adoption decision

The repository's bounded `MEMORY.md` succeeded locally but displaced growth into adjacent mandatory
surfaces. Independent line/byte/max-line measurements show that several root docs now combine current
state with chronology, while the commit workflow requires broad review or refresh on essentially every
slice. That is the exact failure mode the FSMGEN adoption guide describes: containment at one entrypoint
without transitive control of its destinations.

Decision: adopt the neutral contracts as SpecForge-owned policies, derive all local values from this
repository, and migrate by information role rather than arbitrary line shards. The work is split into
separately committable leaves so no historical removal precedes identity, retrieval, consumer, and route
proof. The same umbrella owns the external-SSD locality rule because doctrine registries, generated
projections, archives, caches, and temporary workspaces must share one portable repository-root path
contract. `~/.rustup` and `~/.cargo` remain explicit shared toolchain dependencies; they are not
project-owned artifact destinations. The pinned FSMGEN submodule remains under FSMGEN authority, so
SpecForge constrains invocation environment at the boundary instead of editing submodule files inline.

The ramp-up also found two current-state contradictions in the mdBook (actor-relative ISF direction and
the delivered constrained-extraction provider). They are tracked under `.5a`; this slice does not edit
them before focused source/record verification.

## PDF-VARIANT-DIGESTION.10i (`2026-06-24`) — CODE: block-qualified recovery of the genuinely-different register class

**What / why.** `.10h` recovered the *collapsible* half of the `.10g` reused-mnemonic residual —
identical/nested views of one register collapse to a single record by field-set containment — but
left the GENUINELY-DIFFERENT half a full residual: when a mnemonic's occurrences have disjoint field
sets (ARM-Debug MEM-AP `CSW` 11 fields vs JTAG-AP `CSW` 7 fields), there is no common superset to
collapse to, so both were dropped. That is lossy: each access port really does have its own `CSW`.
The `.10h` write-up worried the flattened heading hierarchy gave no block name to tell the two apart.

**Measurement (probe-first).** The `.10h` `#[ignore]` block probe
(`section_header_register_block_probe`) already prints, for every reused register-routed mnemonic,
its dotted number, its resolved parent-section title, and its field set. Reading that output proved
the `.10h` worry too pessimistic: the headings lose their *nesting*, but each register's parent
section still exists as its own line, and its dotted number is the parent of the register's
(`C2.6` is the parent of `C2.6.7`). Every reused register sits under a parent titled
`<dotted-num> <BLOCK> register descriptions` — `C2.6 MEM-AP register descriptions`,
`C3.5 JTAG-AP register descriptions`, `C1.4 AP Register Descriptions` — except `CLAIMSET`'s fourth
occurrence under a block-less `D4.5 Register descriptions`. The probe also confirmed the
genuinely-different class is in EXACTLY 1 doc (ARM-Debug `ihi0074`); CoreSight `ihi0029`'s only reused
name (`AUTHSTATUS`, all field sets ⊆ the 5-field `B2.3.1`) is `.10h`-collapsible and never reaches
`.10i`.

**Implementation.** `extract_section_header_registers` (`ir/evidence.rs`) now builds a `by_number` map
(dotted section number → heading title) over `document_sections`, and threads each container's own
dotted number onto a new additive `SectionHeaderFieldContainer.dotted` field (computed via the new
`section_dotted_number` helper at the point `parse_dotted_container_heading` already succeeds; `.10f`
ignores it). Each candidate is tagged with its parent block via `dotted_parent` +
`derive_register_block_name`, the latter requiring the parent title's remainder (after its number) to
be exactly `<single-token BLOCK> register description(s)` (case-insensitive) — a multi-word lead-in or
a missing tail yields `None`. When the `.10h` `collapse_section_header_register_identity` returns
`None` (the genuinely-different class), `block_qualify_register_occurrences` groups the occurrences by
block and emits each block-named one as `<NAME>@<BLOCK>` through the shared
`push_section_header_register` helper; a no-block occurrence stays residual, and ≥2 still-disjoint
occurrences sharing one block re-run `.10h` containment within the block (else residual — never a
conflated record). The candidate type became a small struct (`name`/`block`/`fields`) for clarity;
the unique-name and `.10h`-collapse paths are byte-identical to before. Grouping is fully
deterministic (first-appearance order via `or_insert_with`).

**`.isf` safety.** A qualified name flows EvidenceIR → SemanticIR → IntentIR → the `.isf` storage
lowering, which applies `sanitize_isf_name(register_name.to_lowercase())` → a valid identifier:
`CSW@MEM-AP` → `csw_mem_ap`, distinct from `csw_jtag_ap`. The `@` and `-` never reach FSMGen unescaped.

**Verification.** `section_header_register_corpus_sweep`: ARM-Debug 15→20 registers / 69→93 fields;
CoreSight 6/29, GIC 73/468, SMMU 88/381, ACC 2/4 byte-identical. Full `evidence` register_records
ARM-Debug 40→45 — the 5 additions are exactly `CSW@MEM-AP`{11}, `CSW@JTAG-AP`{7},
`CLAIMSET@AP`/`@MEM-AP`/`@JTAG-AP`{2}, with all 40 baseline records byte-identical (a `git stash`
baseline-vs-change full-`evidence` diff over the 8 `.10h` golds + CoreSight `ihi0029` is byte-identical;
only `ihi0074` changes, ADD-only, zero removals). The ARM-Debug `debugger.isf` re-emit carries 45
storage vars (5 new, sanitized + distinct), 0 blocking_reasons, and passes FSMGen `--strict --check
--json` with 0 diagnostics / 0 errors. +3 hermetic tests (block-qualify disjoint / skip no-block
occurrence / `derive_register_block_name` grammar). `kg-bench` 156/156; full `run_ci.sh` GREEN.
ADR-0006 (universal `<NUM> <BLOCK> register descriptions` grammar, no chip-name list). KM
`section-header-register-block-qualification`; book `pipeline/evidenceir.md` `.10i`.

## PDF-VARIANT-DIGESTION.10h (`2026-06-24`) — CODE: block-ambiguous register-mnemonic recovery by field-set containment

**What / why.** `.10g` reads register fields laid out as section headings (`<NAME>, bits [hi:lo]`)
into `register_records`, but its per-document name-uniqueness gate DROPPED every register mnemonic
reused across ≥2 register-routed containers — ARM-Debug reuses `AUTHSTATUS`/`CSW`/`IDR`/`CLAIMSET`/
`DEVARCH` once per access-port block, the dotted heading carrying only the short name. A blind merge
would conflate genuinely-different registers (MEM-AP `CSW` vs JTAG-AP `CSW`, disjoint fields) or
over-count identical cross-references, so `.10g` held the whole reused-name class as a residual.

**Measurement (probe-first).** A local `#[ignore]` probe (`section_header_register_block_probe`)
replayed the `.10g` container walk over the persisted `source_ir` with the real predicates. Two
facts decided the design: (1) the reused mnemonics live in EXACTLY 2 docs (ARM-Debug `ihi0074`,
CoreSight `ihi0029`); (2) the PDF backend flattens every heading to `heading_level` 1, so there is
NO ancestor-block heading to qualify a register with — only the dotted-number hierarchy and the
field lists are intact. The per-occurrence field-set audit cleanly separated three sub-classes:
identical cross-reference (`IDR` C1.4 ≡ C2.6), nested views (`AUTHSTATUS` {2}⊂{3}⊂{4}⊂{5} across
chapters), and genuinely-different (`CSW` 11-field MEM-AP vs 7-field JTAG-AP, disjoint).

**Implementation.** `extract_section_header_registers` (`ir/evidence.rs`) now groups the candidate
register-routed containers by name and resolves each reused name through the new pure helper
`collapse_section_header_register_identity`: it returns the SINGLE maximal occurrence (largest field
set, earliest in document order among ties) iff every occurrence's field set — compared by
uppercased field name — is a subset of it, else `None`. A returned occurrence collapses to one
record carrying that fullest occurrence's REAL layout (never a fabricated union of fields the
document never co-listed); `None` keeps the name as an honest residual. Grouping is fully
deterministic (first-appearance order via `or_insert_with`, no hash iteration reaches output —
EVIDENCE-DETERMINISM). The unique-name path is byte-identical to `.10g`.

**Verification.** `section_header_register_corpus_sweep`: ARM-Debug 12→15 registers / 57→69 fields,
CoreSight 5→6 / 24→29; GIC 73 / SMMU 88 / ACC 2 byte-identical. Full `evidence` register_records:
ARM-Debug 37→40, CoreSight 26→27. A `git stash` baseline-vs-change `evidence` diff over 8 golds
(CCIX r1.0, NVMe, AXI, AHB, GIC, SMMU, ACC, DTI) is byte-identical; the only 2 changed docs ADD
records with ZERO removals and every baseline record byte-identically preserved. +3 hermetic tests +
the existing duplicate-name test re-scoped to the disjoint residual; `kg-bench` 156/156; full
`run_ci.sh` GREEN (lib 1718→1721). Residual narrowed to the genuinely-different (`CSW`/`CLAIMSET`)
class, deferred to a future block-qualified sub-lever.

## KG-ISF-COMPLETENESS.5.iii (`2026-06-24`) — CODE: per-member `_WIDTH` parameter-leak enum gate

**What / why.** The `.5.iii` measurement found a width-PARAMETER leak polluting the AXI gold `.isf` enum
surface: a configuration row (`Enum BRESP BRESP_WIDTH = 0.`) admitted into a value enum, so `(BRESP
(BRESP_WIDTH 0)(OKAY 0)…)` duplicated value `0` and `(RRESP (RRESP_WIDTH 0))` replaced the real codes. This
slice lands the gate.

**Change (`ir/evidence.rs`).** `is_width_parameter_leak_member(member_name, enum_name, known_signals)`
returns true when the member is `<X>_WIDTH` and `X` is the enum's own name OR a declared signal
(`known_signals`, case-insensitive). The member loop in `synthesize_encoding_declarations_for_enum`
`continue`-skips such a member right after the `.5.ii` spine gate (one seam). `known_signals` is threaded
from the signal-match caller (`evidence.rs:4708 Some(known_signals)`); the `synthesize_encoding_declarations`
`None` caller covers the enum-self case. A pure-parameter enum empties → no statements → no
`SymbolDefinition` (honest residual), the same contract `.5.ii` uses.

**Why document-grounded, not a denylist.** The discriminator is the document's own evidence (the enum's
name + the declared-signal set), so a genuine width-VALUE — a link-width enum's `FULL_WIDTH`/`HALF_WIDTH`/
`QUARTER_WIDTH` — is KEPT (its prefix is neither the enum name nor a declared signal). The corpus
false-positive set is EMPTY (`scripts/measure_enum_width_leak.py`), and a unit test pins `FULL_WIDTH` kept.

**Verification (per-item).** AXI evidence rebuild (baseline-vs-new binary) drops EXACTLY the 7 leaks
(`BRESP/RRESP/RCHUNKNUM/RCHUNKSTRB/AWSNOOP/ARSNOOP/AWCMO _WIDTH`; statements 6414→6407; the non-Enum
statement TEXT set byte-identical). The rebuilt `manager.isf` emits `(BRESP (OKAY 0)(EXOKAY 1)(SLVERR 2)
(DECERR 3)(DEFER 4)(TRANSFAULT 5)(RESERVED 6)(UNSUPPORTED 7))` + `(AWCMO (CLEAN_AND_INVALIDATE 0)
(CLEAN_ONLY 1))`; the false `RRESP`/`RCHUNKNUM`/`RCHUNKSTRB`/`AXSNOOP` `_WIDTH`-only enums are gone; FSMGen
`--strict --check --json` success / 0 diagnostics. WIRE-BASED-100 = 1.000 before==after (AXI before/after
eval identical; APB/AHB/SWD/i2c evidence byte-identical → gate inert). `kg-bench` 156/156; `run_ci.sh` GREEN
(lib 1718, +2 tests).

**Residual.** The `SECSID_WIDTH`/`SID_WIDTH`/`SSID_WIDTH` self-named pseudo-enums (the enum NAME itself ends
`_WIDTH`, prefix not a declared signal) are out of this gate's per-member scope — a name-level case
overlapping `.5.i`; left as an honest residual.

## KG-ISF-COMPLETENESS.5.iii (`2026-06-24`) — MEASUREMENT: the `_WIDTH` parameter-leak deeper-enum residual (GO)

**What / why.** `.5.ii` cleaned the prose-SENTENCE-fragment enum members and deferred five deeper
member-quality classes as honest residuals. The resume pointer flagged them as the re-open candidate
"needing their own measurement before code". This slice measures them per-item (read-only, docs-only) and
isolates the one buildable, material lever. It stays in the FIRST active tree (the north star) and is a
surfaced extraction-precision lever — the preferred PNT category per `[[feedback_not_complete_attack_substantive_gaps]]`.

**Method.** Tracked deterministic reproducer `scripts/measure_enum_width_leak.py` over the 78 persisted
IntentIR docs: replicate the `.5.ii` sentence-spine filter, classify the survivors, check each candidate
structural gate for false positives against real enum members, and confirm whether each class reaches a
real-signal-named (`.5.i`-surviving) enum that actually emits to `.isf`.

**Findings.**
- The corpus is a MIX (AXI/APB/CCIX re-emitted post-`.5.ii`, the rest pre-`.5.i`) so the spine-flag tally
  reads 3375/11721 here vs the `.5.ii` 3781/12509 — immaterial to the residual-class question.
- **Subsumption:** 47 of 54 `_WIDTH` members and the bulk of the 319 leading-section-number survivors sit
  in generic-named enums (`TABLE`/`TRANSLATION`/`DEBUG`) that `.5.i` already drops whole → no `.isf` reach.
- **Material leak:** the 7 remaining `_WIDTH` members are in real-signal-named enums in the AXI **gold**
  `ihi0022_l` and reach `manager.isf`: `(BRESP (BRESP_WIDTH 0)(OKAY 0)…)` dup-value, `(RRESP (RRESP_WIDTH 0))`
  replaces real codes, `(AXSNOOP (AWSNOOP_WIDTH 0)(ARSNOOP_WIDTH 1))` pure junk, `(AWCMO (AWCMO_WIDTH 0)…)`
  dup. Root cause confirmed in `generated/evidence_ir/ihi0022_l*`: synthesized statement
  `Enum BRESP BRESP_WIDTH = 0.` — a width PARAMETER row admitted as an encoding value.
- **Discriminator (FP-free, document-grounded, ADR 0006):** drop `<X>_WIDTH` iff `X` is a declared signal
  OR the enum's own name. All 6 distinct caught prefixes are TRUE declared signals; corpus FP set EMPTY
  (no `FULL_WIDTH`/`HALF_WIDTH` value exists, and the declared-signal arm would never catch one — `FULL`/
  `HALF` are not signals, so a real link-width enum is preserved). Per-member, not per-enum.

**Decision.** GO on the `_WIDTH` parameter-leak gate (the code slice; thread the existing `known_signals`
set in scope at the signal-match caller `evidence.rs:4700`, enum-self-name needs no plumbing; before/after
WIRE-BASED-100 eval required as it byte-changes the AXI gold). NO-GO on section-caption (leading
`[A-Z]?digit` collides with real codes `D1`/`L2`), restart-of-clean (no defect — `.5.ii` proved restart is
not junk), glossary/front-matter (tiny + name-ish) → honest residuals.

**Verification.** No code → all oracles orthogonal by construction; `scripts/check_doctrines.sh` GREEN;
`KNOWLEDGE_MAP.md` regenerated (126 facts / 932 keys, in sync). Report
`docs/research/generic-enum-conflation-measurement.md` §`.5.iii measurement`; KM `[[generic-enum-conflation]]`.

## KG-ISF-COMPLETENESS.5.ii (`2026-06-24`) — CODE: per-member sentence-spine enum-fragment gate
- **Slice:** the `.5.ii` code, landed in the same fresh focused session as its measurement (high-stakes — it changes wire-gold `.isf` bytes). Before/after-proven (`[[feedback_scoring_rigor]]`).
- **The edit (`ir/evidence.rs`).** New `PROSE_SENTENCE_SPINE_WORDS` const (38 lowercase copula/aux/modal/article/demonstrative/relativizer/subordinator words; collisions `a`/`i`/`its`/`can`/`may`/`am` excluded) + `is_prose_fragment_member_name(name)` (splits on `_`, exact-lowercased token compare). The member loop in `synthesize_encoding_declarations_for_enum` `continue`-skips a fragment member BEFORE pushing its `Enum X = V` statement. One seam → both call paths (`synthesize_encoding_declarations` `None` + the signal-anchor `Some(known_signals)` path). An all-fragment table yields zero statements, so `build_symbol_definitions` (`semantic.rs`) mints no `SymbolDefinition` — the existing no-statements → no-enum contract, an honest residual.
- **Why per-member (not the recorded per-enum plan).** The surviving junk enums are conflations of a clean code table and a prose-description table sharing a signal name (AXI `BRESP` = `OKAY`/`EXOKAY`/`SLVERR`/`DECERR`/… fused with whole sentences). A whole-enum drop loses the codes; value-restart (AHB `HPROT`) flags all-clean conflations. The grammatical NAME shape is the only false-positive-free signal — see the measurement note below.
- **Verified (before/after, all gated by the preserved baseline = post-`.5.i` binary vs the gated = post-`.5.ii` binary):** WIRE-BASED-100 = **1.000 before==after** over all 10 seeds (gold evidence rebuilt with both binaries via `evidence --dry-run`, scored with `eval-extraction --provider skip`; the scored surface is byte-identical, only `evidence_root`/temp-path lines differ). Landed wire-gold enum-member-statement census: AXI 148→91 (`BRESP` recovers its 8 codes — confirmed in the emitted `(BRESP …)`; `ARTAGOP`/`AWTAGOP`/`RLAST`/`WLAST`→0), APB 7→4; AHB/SWD/nvme/i2c unchanged. CCIX `TABLE` stays 27 clean-identifier members (correctly untouched — the cross-table-merge-of-clean residual). Rebuilt AXI + APB `.isf` pass FSMGen `--strict --check --json` (`success: true`). `kg-bench` 156/156; `run_ci.sh` GREEN (lib **1716**, +4 tests).
- **Tests:** `prose_sentence_spine_words_excludes_identifier_collisions` (drift-guard the 6 collision exclusions), `prose_fragment_member_predicate` (fragments flagged / identifiers + collisions kept), `encoding_member_synthesis_drops_prose_fragments_keeps_codes` (mixed BRESP-like table → only codes survive), `encoding_member_synthesis_all_prose_yields_no_statements` (pure-prose table → 0 members, counter unadvanced).
- **Honest residuals deferred:** glossary `SEE…`/`SEE_ALSO`, front-matter/ToC (`NON_CONFIDENTIAL_PROPRIETARY_NOTICE`), section-caption (`B2_3_1_READ_TRANSACTIONS`), `_WIDTH` parameter leaks (`BRESP_WIDTH`), value-restart conflations with all-clean members (`HPROT`/CCIX `TABLE` — sub-enum splitting is a separate, riskier refinement). Report §`.5.ii LANDED`; KM `[[generic-enum-conflation]]` → LANDED. **`.5` enum-surface fidelity is now built.**

## KG-ISF-COMPLETENESS.5.ii (`2026-06-24`) — MEASUREMENT/CALIBRATION: member-quality gate is PER-MEMBER (design corrected, GO)
- **Slice:** read-only calibration of the residual junk-enum gate (the `.5.i` name-gate cannot reach enums whose NAME is a real signal but whose MEMBERS are junk). Measurement-first, per the `.5.ii` "calibration-gated" requirement (`[[feedback_scoring_rigor]]`). No code.
- **Method.** `member_final.py` over all 78 `generated/intent_ir/*/intent_ir.json` (561 enums / 12 509 members): split each synthesis-sanitized `member_name` on `_`, test each token against an English sentence-spine lexicon.
- **Finding 1 — the recorded `.5` whole-enum rule is DISPROVEN.** (a) A whole-enum drop destroys real codes: AXI-gold `BRESP` (`ihi0022_l`) is **16 members** = 7 prose fragments (`NON_EXCLUSIVE_WRITE__THE_TRANSACTION_WAS_SUCCESSFUL…`) + a leaked `BRESP_WIDTH` **fused with** the 8 genuine codes `OKAY/EXOKAY/SLVERR/DECERR/DEFER/TRANSFAULT/RESERVED/UNSUPPORTED`. The surviving junk enums are conflations of a junk table and a clean table → the correct granularity is **per-member**. (b) Value-restart is NOT a junk signal: AHB-gold `HPROT` has `restarts=2` (3 fused HPROT sub-encodings) yet all 15 members are clean identifiers (`DATA_INST`/`PRIVILEGED`/`WRITE_BACK__SHAREABLE`/…) — a restart-gate would be a false positive.
- **Finding 2 — the signal is per-member NAME shape.** The synthesis (`synthesize_encoding_declarations_for_enum`) sanitizes a name-cell to `[A-Z0-9_]` uppercased, so a prose sentence becomes one `_`-joined member name. A real symbol never carries the **spine** of an English sentence — a copula/aux/modal (`IS`/`ARE`/`WAS`/`BE`/`HAS`/`MUST`/`SHALL`/`WILL`/…), article/demonstrative (`THE`/`AN`/`THIS`/`THAT`/…), relativizer/subordinator (`WHICH`/`WHEN`/`IF`/`BECAUSE`/`WHILE`/…). Drop a member carrying a spine token; an emptied enum is not minted (existing no-statements → no-`SymbolDefinition` contract → honest residual). **Collisions EXCLUDED** (the `.1a` discipline): `A` (article vs. single-letter suffix), `I`, `ITS` (GIC ITS), `CAN` (CAN-bus), `MAY` (month), `AM`.
- **Finding 3 — precision/recall (per-item).** Clean anchor (wire-gold `HTRANS`/`HSIZE`/`HRESP`/`HPROT`/`TKEEP` members + canonical codes) **0/115 flagged → precision 1.000**; junk anchor (hand-picked prose fragments) **269/269 caught → recall 1.000**; corpus-wide 3 781/12 509 (30.2 %) drop. The 8 "clean-anchor FP" my first pass reported were a mislabeled anchor (`ARCACHE`/`AWCACHE` in the dense-prose AXI+ACE `ihi0022_h_c`, genuinely prose → true positives).
- **Finding 4 — wire-gold blast radius.** `BRESP` 16→9 (recovers codes), `RRESP` 8→1 (`_WIDTH` leak residual), `PPROT` 3→0, AXI+ACE `ARCACHE`/`AWCACHE`/`AWTAGOP`/`READ`/`RLAST`/`WLAST`→0, `HSIZE`/`HTRANS`/`HRESP` untouched. Byte-changing (strict improvement) → code slice needs before/after WIRE-BASED-100 eval.
- **Honest residuals deferred** (deeper member-quality classes, FP risk too high for v1): glossary `SEE…`/`SEE_ALSO` refs, front-matter/ToC members (`NON_CONFIDENTIAL_PROPRIETARY_NOTICE`), section-caption members (`B2_3_1_READ_TRANSACTIONS`), `_WIDTH` parameter leaks, value-restart conflations with all-clean members (`HPROT` kept intact; sub-enum splitting is a separate refinement).
- **Decision: GO** — land at `synthesize_encoding_declarations_for_enum` (`ir/evidence.rs`). Oracles on the code slice: WIRE-BASED-100 before/after + `kg-bench` + FSMGen `--strict` + `run_ci.sh`. Report `docs/research/generic-enum-conflation-measurement.md` §`.5.ii measurement`; KM `[[generic-enum-conflation]]`.

## KG-ISF-COMPLETENESS.5.i (`2026-06-24`) — CODE: extraction-side enum fallback name-gate + emitter orphan-`(type)` fix
- **Slice:** the `.5` decision's first code lever, on a fresh focused session per the high-stakes gate-code rule (it changes wire-gold `.isf` bytes). Measurement-first, before/after-proven (`[[feedback_scoring_rigor]]`).
- **The two edits.** (1) `derive_encoding_enum_name`'s fallback (`ir/evidence.rs`) now finds the first `is_hardware_signal_token` caption token, then keeps it **only when independently evidenced** — `known_signals.eq_ignore_ascii_case` (declared signal) OR `contains_reference_token(header, cand)` / `header.contains("{cand}[")` (a column-header reference token of the table) — else returns `None`. (2) the emitter (`ir/isf_ir.rs`) builds `emitted_enum_names` from `emitted_enums()` and filters `self.types` by it, so the `(types)` block only declares a type whose `(enums)` family is emitted.
- **Design grounded by a read-only probe (not assumed).** HBM2's genuinely-named enums (`UPDATEWR`/`EXTEST_RX`/`DWORD_MISR`/…) are returned by the signal-match LOOP that runs BEFORE the fallback, so the fallback gate cannot touch them (byte-identical). The fallback only ever produced document-structure/caption words (`TABLE`/`COLUMN`/`AMBA`/`BYTE`/`READ`/`CACHE`/`RELEASE`), none of which is a declared signal or a column header → all dropped. So the structural gate is *strictly stronger* than the name-only gate the `.5` measurement envisioned — it also catches the fallback-origin subset of the "271 real-named-but-junk" enums, leaving `.5.ii` precisely the signal-match-origin junk-member enums + the 8 document-evidenced survivors.
- **A second, beneficial side effect (corrects the `.5` reasoning).** The dropped `Enum X = V` statements also leave `discovered_values`, which feeds value-constraint extraction; so off-gold junk constraints whose value was a junk-enum member also vanish (AXI: `ACTIVATEACK A` → the grounded reset value `ACTIVATEACK 1`; spurious `ARDOMAIN SHAREABLE`/`ARMMU* 0B0` removed). The AXI *distinct* `signal_constraints` fact set is identical (count drop 64→51 = duplicate records); WIRE-BASED-100 is unchanged. So the `.5` claim "orthogonal because enums are unscored" is corrected to "orthogonal because the affected constraints are off-gold + already `.6/.7`-filtered" — proven by the before/after eval, not assumed.
- **Measured (per-item, before/after via baseline vs gated binary; method: symlink read-only `source_ir`+`prior_memory` into a temp CWD, rebuild `evidence→semantic→intent→adapt`, diff).** Corpus census (33 rebuildable docs): generic-named enums **82→8**, total enum records **422→105**. The 8 survivors are document-evidenced column-header/declared-signal tokens (CCIX has 48 "Table of Contents" header cells → keeps a junk `TABLE`; gic_600 `DATA` is a real signal) — honest `.5.ii` residuals. Wire golds: APB `PPROT`+`TABLE`→`PPROT`; AHB drop `TABLE`/`AMBA`; AXI drop 6 caption words / keep 22 real; SWD drop 9 / keep `TDI`; SWP `TABLE`/`CLT`/`ANNEX`→none; AXI-Stream `TKEEP` unchanged. Every affected `.isf` FSMGen `--strict --check --json` **0 diagnostics**.
- **NO REGRESSION.** WIRE-BASED-100 = **1.000 before==after** (rebuilt each gold doc's evidence with both binaries, ran `eval-extraction --provider skip` — APB/AHB/AXI signal_constraint+relation, APB/AHB/AXI temporal, SWD relation all identical; SWD lone constraint the documented promotion-only 0/1). nvme-registers + i2c golds identical. `kg-bench` 156/156. `run_ci.sh` GREEN (lib 1712 passed, +4 tests). Genericity (ADR 0006): structural, no name list.
- **Tests.** `encoding_enum_name_fallback_drops_document_structure_keyword`, `encoding_enum_name_fallback_keeps_column_header_candidate`, `encoding_enum_name_keeps_declared_signal_but_drops_unevidenced_caption_word`, and the strengthened `binary_looking_enum_value_is_excluded_and_recorded_as_residual` (now asserts the orphan `(type table)` is dropped).
- **Frontier → `.5.ii`** (per-table member-quality gate for the 8 survivors + signal-match-origin junk-member enums; calibration-gated).

## KG-ISF-COMPLETENESS.5 (`2026-06-24`) — generic-`TABLE` mega-enum conflation MEASURED (read-only, docs-only)
- **Slice:** PNT pivot. After re-ingest #32 confirmed the `CORPUS-COVERAGE.2` tail is low-value (a legal-exhibit excerpt), the high-value move per `[[feedback_not_complete_attack_substantive_gaps]]` is to ACT on a surfaced upstream extraction-precision lever rather than grind more bundle-restorations. Measurement-first (`[[feedback_scoring_rigor]]`), read-only, no code.
- **Method:** delegated the heavy corpus characterization (reading the 26K-line `evidence.rs`) to a focused read-only agent; cross-verified its key claims myself (`intent.rs:1490` "TABLE" guard comment; the emitted `(type TABLE (bits 6))` in HBM2's `hbm.isf`).
- **Finding — EXTRACTION-born, not the emitter.** `derive_encoding_enum_name` (`evidence.rs:4457-4461`): when the known-signal match fails, the fallback names the table after the first caption token passing `is_hardware_signal_token` (`evidence.rs:7106`, which accepts `Table`→`TABLE` — it only requires len≥2 / uppercase-first / `[A-Z0-9_]`). HBM2's captions all read "Table N - …", so every unmatched encoding table is named `TABLE`, and `build_symbol_definitions` (`semantic.rs:2782-2789`) accumulates members **by enum_name key** → all `TABLE`-named tables fuse into ONE `SymbolDefinitionRecord`, copied verbatim to IntentIR (`intent.rs:189`) and lowered faithfully (`isf_ir.rs:889-912`). The emitter is a faithful pass-through.
- **Corpus:** 56/78 docs / 96 generic enums (vs 493 real) / ~95 reach `.isf`. HBM2 `TABLE` = 57 members across 7 value-restart runs (lane-remap binary codes + microbump geometry + IEEE-1500 opcodes + IDD currents + a footnote + a mode-register caption + …) — 29 dup values, 30 fragment names.
- **Decisive insight (changes the fix shape):** a name-only gate catches 96 but MISSES 271 real-named-but-junk enums (`COMMAND`/`DWORD_MISR`/`AMBA` carry sentence-fragment member names + restarting values); only 222 of 493 real enums are clean. The real defect is member quality; the generic name is its most visible symptom.
- **Decision: GO, extraction-side, decomposed.** `.5.i` (the safe, higher-leverage first step): gate `derive_encoding_enum_name`'s fallback to return `None` unless the candidate token is independently a declared signal (a `None` fallback is the existing contract → no enum minted → honest residual), which kills all 96 generic enums AND the conflation (the merge keys on the shared name); + the emitter orphan-`(type)` fix (`isf_ir.rs:403-409` emits all `self.types` unconditionally, leaving an orphan `(type TABLE)` even when Lever F drops the members — gate by `emitted_enums()`). `.5.ii` (calibration-gated): a per-table member-quality gate for the 271 real-named junk enums.
- **WIRE-BASED-100 safety:** the scored constraint/relation/temporal surfaces are ORTHOGONAL (generic enums are in no `eval-extraction` gold). BUT all 4 wire golds emit a junk `TABLE` enum into their `.isf` (AHB's fuses HTRANS+HSIZE values that already have correctly-named enums), so a fix CHANGES `.isf` bytes — a strict improvement, but not byte-identical, requiring a deliberate per-doc snapshot refresh. For that reason `.5.i` is the next code slice but is **deferred to a fresh/focused session** for signoff quality (the gate-code high-stakes rule), not rushed at the tail of a long measurement session.
- **Gates:** no code / no canonical mutation → WIRE-BASED-100 + register/wire golds + `kg-bench` orthogonal by construction; `check_doctrines.sh` PASS; knowledge map regenerated in sync (125→126). Report `docs/research/generic-enum-conflation-measurement.md`; KM `[[generic-enum-conflation]]`. Book unchanged (no behavior change yet — the book update lands with `.5.i` when the emitted `.isf` actually changes).

## CORPUS-COVERAGE.2 re-ingest #32 (`2026-06-24`) — JEDEC HBM-gen1 (`jesd235`): bundle-restoration + corpus-provenance finding
- **Slice:** PNT continuation — the strict frontier (first active tree `KG-ISF-COMPLETENESS` has no buildable leaf) advances to `CORPUS-COVERAGE.2`. RAM 83% free, no ollama model loaded → re-ingest viable (a perishable RAM-gated opportunity). Picked `jesd235` as the best remaining real-delta candidate (HBM gen1 = a register/timing-table DRAM spec; its stale evidence was essentially empty — 0 registers / 69 statements from a degraded 35KB Jun-7 `source_ir`).
- **Finding (the actual value of this slice):** the library PDF `JESD235_2013-10_HBM_DRAM.pdf` is **only 6 pages** and is a **legal-exhibit cover** — the normalized markdown is `DOCKET`/`ALARM`/`JEDEC STANDARD … High Bandwidth Memory (HBM) DRAM … OCTOBER 2013`/`Netlist Inc.`/`Netlist Ex 2021` plus 12 cover images, with **0 tables**. It is NOT the full HBM gen1 standard. So the thin extraction (0 registers, no behavioral content) is **honest source-driven absence**, not an extraction failure — and the real HBM gen1 spec content is simply not present in the corpus under this key (a corpus-quality flag for whoever curates the library).
- **Method:** Docling ingest (`DOCLING_DEVICE=cpu`, current release binary, `.4a` RAM guard armed, staged-swap with the stale `source_ir.json` backed up first) → deterministic cascade `evidence → semantic → intent → adapt --target isf`. No `converge`/VLM/NLP (deterministic, no RAM contention, reproducible).
- **Result:** deterministic surfaces byte-near-identical to stale (register/message-field/relations/constraints/conditional all 0; statements 69→68; intent actors 2→2 / interfaces 1→7 minor regroup). `adapt` recovers 16 signals on actor `channel` but blocks honestly with the single reason "no behavioral content" (1 residual) — the correct `CORPUS-COVERAGE.0` intent-no-isf class; no `.isf` is fabricated, so no FSMGen `--strict` run applies. Normalized bundle restored (was missing).
- **Forward note:** after #32, 24 real chip-spec docs remain normalized-missing, and the tail is overwhelmingly thin/degraded (OpenCAPI×13 + USB guides + debug guides). This empirically confirms the survey's read that the re-ingest tail is low-value; recorded in MEMORY's next_action that the next PNT slice should pivot to a surfaced upstream extraction-precision lever (generic-`TABLE` mega-enum conflation or signal-inventory prose-acronym noise) per `[[feedback_not_complete_attack_substantive_gaps]]`.
- **Gates:** no code change → WIRE-BASED-100 + register/wire golds + `kg-bench` orthogonal by construction; `generated/` git-ignored (durable trace = the `.2` ledger); `check_doctrines.sh` PASS.

## BOOK-COMMAND-COVERAGE.3 (`2026-06-24`) — complete the Commands-overview "Current surface" list (21→28)
- **Slice:** fresh-session PNT pick — a concrete mdBook↔CLI drift surfaced by the session-start book survey and then RE-DERIVED by hand (the surfacing agent's count is never trusted; the `BOOK-COMMAND-COVERAGE.1` precedent had a delegated "5 missing" claim that an objective grep cut to the real 2). Owned by re-opening `BOOK-COMMAND-COVERAGE` (R0, the command-surface tree) with a `.3` leaf.
- **Root cause:** `docs/book/src/commands/overview.md` is the CLI's operational map; its "Current surface:" bullet list is meant to be the complete command set. As the EXTRACTION-QUALITY-GAUGE / GRITS commands landed, each got a dedicated detail section (`.1`/`.2`'s bar) but the overview list was never extended — so it enumerated 21 of the 28 `cli.rs` subcommands. A user scanning the operational map for "what can I run?" saw 75% of the surface.
- **Implementation:** added the 7 missing commands (`nli-verify`, `entity-type`, `extract-conditions`, `extract-constraints-llm`, `eval-extraction`, `audit-extraction`, `grits-consensus`) after `signal-resolve` in README order. Each flag signature was cross-checked against the actual `cli.rs` clap arg struct (the source of truth — `EvalExtractionArgs`/`NliVerifyArgs`/`GritsConsensusArgs`/`EntityTypeArgs`/`ExtractConditionsArgs`/`ExtractConstraintsLlmArgs`/`AuditExtractionArgs`), not copied from `README.md`. Added a navigation pointer to the two detail chapters.
- **Rigor catch:** the navigation pointer first asserted "every one of them defaults to a CI-safe no-op." Re-reading the clap defaults disproved it — the extraction/judge commands default to live `ollama` (no-op only when passed `skip`); only `eval-extraction`/`audit-extraction` default to `skip`, and `grits-consensus` is an offline measurement. The wording was corrected to describe the actual per-command behavior, honoring the "book describes what the code does" doctrine (no over-claim).
- **Validation:** `mdbook build docs/book` exit 0; awk over the "Current surface" block = 28 bullets (matches the 28 `cli.rs` commands); all 7 grep `PRESENT`. Docs-only → lib unchanged; `check_doctrines.sh` PASS (MEMORY-ARCH / KNOWLEDGE-MAP / TASK-ACCEPTANCE).
- **Boundary:** the separate `ROADMAP.md` lane-hierarchy drift (Open Question in the tree) is still out of scope. This leaf only reconciles the book overview list.

## AUDIT-DOC-RECONCILE.3 (`2026-06-24`) — refresh `RUST_CODEBASE_ANALYSIS.md` size/command/module/test counts
- **Slice:** fresh-session PNT pick — the second concrete live-doc drift surfaced by the session-start codebase survey (after the `nli-verify` book gap). `RUST_CODEBASE_ANALYSIS.md` is a tracked live continuity doc that `SESSION_BOOTSTRAP` says to keep current; its `2026-06-08` currency section stated counts that drifted as the EXTRACTION-QUALITY-GAUGE / PDF-VARIANT-DIGESTION / KG-ISF-* / MEMORY-BOUNDED-INGEST work landed. Owned by re-opening `AUDIT-DOC-RECONCILE` (the doc-reconciliation tree, R0) with a `.3` leaf.
- **Measurement-first (verified at HEAD, deterministic — did NOT trust the survey agent's numbers):** `find crates/specforge/src -name '*.rs' -exec cat {} + | wc -l` → **128,742** lines across **65** files (doc: ≈103,200); `grep -oE 'Commands::[A-Za-z]+' lib.rs | sort -u | wc -l` → **28** subcommands (doc: 25); `ls crates/specforge/src/ir/*.rs | wc -l` → **28** IR modules (doc: 25); `cargo test -p specforge --lib` → **1709 passed / 0 failed / 4 ignored** (doc: 1435; ran fresh, 6.86s — confirming no Rust change this session left it at the `.2a.vi` count).
- **Edit pattern:** added a new top `## Session update (2026-06-24 ramp-up currency correction …)` section carrying the four refreshed counts (each with its reproducible command) + a largest-modules table; left every older dated section verbatim (they are historical record, explicitly superseded where they cite smaller counts); reversed no architecture claim (four IR stages, `.isf`-only adapter, R16 typed layer, Ollama+Qwen2.5VL production-default all reaffirmed).
- **A caught inaccuracy (the value of the slice):** I first wrote "the three subcommands added since 2026-06-08 are audit-extraction / recover-register-bits / corpus-cluster", then noticed the `2026-06-08` entry itself describes `recover-register-bits` in prose (line ~506) while omitting it from its 25-count list — so that entry's enumeration was an undercount and the exact per-command delta can't be reconstructed reliably. I **withdrew** the sub-claim and state only the verified live total (28). A currency-correction doc must not introduce a new unverified claim while fixing old ones.
- **Gates:** docs-only (no Rust change) → lib unchanged (1709), `mdbook`/WIRE-BASED-100/`kg-bench` orthogonal; `scripts/check_doctrines.sh` GREEN. Both discovered live-doc drifts (the `nli-verify` Commands-chapter gap, `BOOK-COMMAND-COVERAGE.2`; and this codebase-analysis staleness) are now reconciled.

## BOOK-COMMAND-COVERAGE.2 (`2026-06-24`) — dedicated `nli-verify` section in the Commands chapter
- **Slice:** fresh-session PNT lane-switch. After re-ingest #31 exhausted the CHI-C2C family marquees and left a mostly-thin re-ingest tail, I switched to the most concrete doc gap the session-start mdBook survey surfaced — honoring the user's #1 non-negotiable (the mdBook is the only window into the tool and must reflect it). Owned by re-opening `BOOK-COMMAND-COVERAGE` with a `.2` leaf.
- **The gap (honestly framed):** this is NOT book-contradicts-code drift. `BOOK-COMMAND-COVERAGE.1` (`2026-06-14`) deliberately judged `nli-verify` *covered* under its bar ("≥1 substantive treatment anywhere"), because `architecture-rationale.md` carries a rich honesty-layer section on it. The real gap is **commands-chapter completeness/navigability**: of the 15 quality commands, `nli-verify` was the only one without a dedicated `##` section in `commands/quality-and-learning.md`, so a user browsing the Commands reference to look it up found nothing. `.2` raises the bar to "every command has a dedicated Commands-chapter section."
- **What I wrote:** a `## nli-verify` section in the chapter's why-before-what style, beside its sibling `extract-constraints-llm`. It covers: the invocation `nli-verify <evidence-ir> [--vlm-provider …] [--model …]`; the NLI premise/hypothesis framing (source sentence entails the constraint-as-a-claim — catching a *condition* read as an *obligation*, the failure a string-match grounding waves through); the only-ever-strengthens + abstain-on-unavailable property (CI-safe; a model outage never deletes an extraction); text-model (not vision) + the mock test hook; `--vlm-provider skip` default-safe no-op; the persisted `extraction_quality_gauge` and the "a vacuous all-abstained pass is never persisted" honesty rule; and the active-gate path (`intent --nli-verify` demotes non-entailed contracts to residuals, surfaced by `validate` as `nli_demoted_contracts`).
- **Fact-grounding:** every claim cross-checked against `crates/specforge/src/commands/nli_verify.rs` (the `measure_and_persist_gauge` / `gauge_summary_line` / `run` flow, the `persisted = entailed + not_entailed > 0` rule, the split-conformal calibration print) and `cli.rs` (the `NliVerifyArgs` flags). No model name hardcoded (said "the default text model") to avoid future drift.
- **Verify:** `mdbook build docs/book` exit 0; `grep '^## ' commands/quality-and-learning.md` shows `## nli-verify` (sections 14 → 15). The richer `architecture-rationale.md` narrative is untouched — the new section is the Commands-reference entry that complements it. Docs-only (no Rust change) → lib unchanged, WIRE-BASED-100 / `kg-bench` orthogonal; `scripts/check_doctrines.sh` GREEN.

## CORPUS-COVERAGE.2 (`2026-06-24`) — re-ingest #31: AMBA CHI C2C 2026 variant (`ihi0098_a_b`), marquee message-field refresh 0→143
- **Slice:** fresh-session PNT pick continuing the `.2` sweep (binary current from #29, no rebuild). Picked the CHI C2C 2026 variant `ihi0098_a_b` (122pp) deliberately as a hypothesis test: #29 (the 2024 `ihi0098_a`) yielded a 0→149 message-field marquee, so the sibling — also `message_field_records=0` in stale evidence — should confirm the CHI-C2C family is a real marquee, not a one-off.
- **Result — hypothesis CONFIRMED:** `message_field_records` **0 → 143** across **12 containers** (vs #29's 0→149 — the small difference is the 2026 revision's content, not a regression). transactions 2→3 (recognition-only; the 2026 variant names one more); the single stale `actor_signal_relations` 1→0 (a fragment relation the current `.1a`/`.1b` agent-identity gates correctly drop — CHI is a coherency/message protocol whose intent lives in message fields, so 0 relations is honest per `KG-ISF-COMPLETENESS.3`); conditional_rules 25 held; 0 registers (CHI has no register tables); 2364 statements; intent 10 actors / 0 rel / 3 txns; normalized bundle RESTORED (122pp / 96 visual / 0 residuals).
- **Emit + verify:** `agent.isf` renderable (69 interface ports / 3 enums / 0 rules); **FSMGen `--strict --check --json` → success / 0 diagnostics**.
- **Surfaced (same as #29, not fixed in-slice — upstream extraction-precision residuals):** generic `(type TABLE …)` mega-enum conflation + signal-inventory prose-acronym noise. These recur across the CHI-C2C family, reinforcing that they're standing upstream levers (an enum-extraction-precision lever + the `KG-ISF-COMPLETENESS.4` signal-precision lever), not emitter bugs.
- **Gates:** no code change (data refresh) → WIRE-BASED-100 / `kg-bench` orthogonal; `generated/` git-ignored (durable trace = the `.2` log); `scripts/check_doctrines.sh` GREEN. After #31: 31 docs re-ingested, 25 real chip-spec docs still normalized-missing.

## CORPUS-COVERAGE.2 (`2026-06-24`) — re-ingest #30: RISC-V AIA (`1_0_2025_03_12`), bundle-restoration + confirmation
- **Slice:** fresh-session PNT pick continuing the `.2` re-ingest sweep (CORPUS-COVERAGE remained the first active tree with a buildable frontier after re-ingest #29). The release binary was already current from #29, so no rebuild.
- **Candidate selection:** RISC-V AIA (89pp) — its persisted evidence was `register_records=0` / `message_field_records=0` despite 1193 statements, the pre-`.10`-stale signature for a register/ISA spec, so it was picked as value-likely (the `.10g`/`.10c` register families fire on RISC-V Debug/IOMMU).
- **Result — the value-likely hypothesis did NOT pan out; it is the bundle-restoration class instead:** the deterministic cascade is byte-near-identical to the stale evidence (statements 1193→1193 identical, register_records 0→0, message_field_records 0, actor_signal_relations 0, conditional_rules 39 held; intent 8 actors / 0 rel / 0 txns). So AIA's evidence was already rebuilt from the retained `source_ir.json` and is current — re-ingest value = **normalized-bundle RESTORED** (was reclaimed) + current-binary confirmation, the #21/#22 phase class.
- **Why registers stayed 0 (honest, characterized):** AIA is a memory-mapped interrupt-controller ISA (APLIC/IMSIC). Its register-layout/structure tables sit in the **12 `unknown`-classified** structured tables (of 22; the other 8 are `encoding` tables that feed the 5 emitted enums, 2 are `timing_parameter`) — they don't match the `.10b`/`.10c`/`.10g` two-column / section-heading register families, so `register_records` is honestly 0. This **re-confirms the #21 RISC-V-IOMMU Lever-D structure-table recall opportunity** (RISC-V in-memory structure/register tables are a recognized recall gap, not a regression). The `.10g` register family is deliberately gated to ARM-architecture docs by per-document name-uniqueness, so it does not fire on RISC-V by design.
- **Emit + verify:** `agent.isf` renderable (91 interface ports / 5 enums / 22 rules); **FSMGen `subs/fsmgen/bin/fsmgen --strict --check --json` → success / 0 diagnostics** (strict-clean).
- **Gates:** no code change (data refresh), so WIRE-BASED-100 / `kg-bench` orthogonal by construction; `generated/` git-ignored → durable trace = the `.2` log row + changelog; `scripts/check_doctrines.sh` GREEN. After #30: 30 docs re-ingested, 26 real chip-spec docs still normalized-missing. **Phase note: the tail interleaves marquee jumps (#29 CHI, 0→149 message fields) with bundle-restoration confirmations (#30 AIA) — consistent with the #22 finding that marquee jumps are now the exception, not the rule.**

## CORPUS-COVERAGE.2 (`2026-06-24`) — re-ingest #29: AMBA CHI C2C (`ihi0098_a`), marquee message-field refresh 0→149
- **Slice:** fresh-session PNT pick. `KG-ISF-COMPLETENESS` (first active tree) had no immediately-buildable leaf after `.3` closed (its remaining `.1c.ii`/`.2b` are deferred/out-of-frontier), so per the PNT selection rules PNT advances to the next active tree — `CORPUS-COVERAGE`, whose `.2` standing frontier is the RAM-guarded current-binary re-ingest of the 57 normalized-missing docs.
- **Candidate selection:** chose AMBA CHI C2C `ihi0098_a` (108pp) — its persisted evidence was `message_field_records=0` / `register_records=0` despite 2239 statements, the signature of pre-`.10` staleness on a packet/flit protocol whose intent is exactly message fields. A bounded, high-value pick (vs the thin OpenCAPI PHY tail or the 548pp USB 3.2).
- **Pre-flight (binary currency):** the persisted `target/release/specforge` was dated 18:44 but the `.2a.vi` code commit `c9d82fd3` landed 20:57 and the `isf_rule_value` symbol was absent — the binary was STALE. Rebuilt release (`CARGO_BUILD_JOBS=4`, RAM ~22% used); confirmed the symbol present post-build. The re-ingest's `adapt` stage depends on `ir/isf_ir.rs`, so a current binary is mandatory for a faithful result.
- **Procedure:** `DOCLING_DEVICE=cpu specforge ingest <pdf>` (the `.4a` RAM guard as the host-safety net) → 108 page artifacts / 88 visual / 0 residuals, normalized bundle RESTORED (was reclaimed) → deterministic `evidence → semantic → intent → adapt` (no LLM, no VLM).
- **Result (before → after):** `message_field_records` **0 → 149** across **13 containers** (the marquee win — the current `.10` message-field families fire on CHI's packet/flit field tables that the stale pre-`.10` evidence never carried); `register_records` 0 (CHI has no register tables — honest); `actor_signal_relations` 0 (coherency/message protocol — intent on message fields, the `KG-ISF-COMPLETENESS.3` honest-absence pattern, and `validate` stays correctly silent on 0-vs-0); `conditional_rules` 17 held; intent 10 actors / 0 relations / 2 transactions held; 2239 statements.
- **Emit + verify:** `agent.isf` renderable (69 interface ports / 3 enums / 1 rule / 0 transaction bodies — the 2 recognized transactions are recognition-only); **FSMGen `subs/fsmgen/bin/fsmgen --strict --check --json` → success / 0 diagnostics** (strict-clean).
- **Surfaced (honest residuals — extraction-precision, NOT the emitter; deliberately NOT fixed inside a re-ingest slice per the `.2` doctrine):** (1) the emitter built a generic `(type TABLE (bits 8))` mega-enum conflating many distinct field-value tables, with duplicate member values and sentence-fragment member names — the same generic-`TABLE` conflation class as #28 HBM2 (an upstream enum-extraction-precision concern; FSMGen still accepts it 0-diagnostics, so it's not a strict-FAIL); (2) a few interface ports are prose-acronym noise (`AES`/`AMBA`/`ARM`) — the `KG-ISF-COMPLETENESS.4` signal-inventory prose-noise class. Both recur the standing upstream levers; recorded, not patched.
- **Gates:** no code change (a data refresh + the binary rebuild), so WIRE-BASED-100 / `kg-bench` are orthogonal by construction (the 4 gold docs are not re-ingested); `generated/` is git-ignored so the durable trace is the `.2` log row + changelog; `scripts/check_doctrines.sh` GREEN. After #29: 29 docs re-ingested, 27 real chip-spec docs still normalized-missing (plus the project README).

## KG-ISF-COMPLETENESS.3 (`2026-06-24`) — CLOSED: bar #2 relation-completeness resolved (verification-only, no code change)
- **Slice:** fresh-session PNT pick — the first eligible leaf of the first active tree (`KG-ISF-COMPLETENESS`), per the PNT selection rules in `docs/TASK_TREE.md`. `.3`'s remaining open leaves above it (`.1c.ii`, `.2b`) are both `deferred` (out of the frontier), so `.3` (`active`) was first eligible.
- **What `.3` is:** the owner-directed north-star bar #2 investigation (`2026-06-17`) of why whole docs carry actors+constraints but ZERO `actor_signal_relations`. It established two cleanly-separated causes — **(A) STALE IntentIR** (the canonical `intent_ir.json` lagged its `evidence_ir.json` because per-stage commands don't auto-cascade; only `converge` rebuilds the whole chain) and **(B) HONEST ABSENCE** (register/command/coherency protocols declare ~0 wire signals; their intent lives on `register_records`/`message_field_records`/`transactions`). It left two frontier sub-steps.
- **(i) Corpus refresh — verified DONE.** Re-census over all 78 persisted `intent_ir.json` vs `evidence_ir.json` (Python, `actor_signal_relations` array length both stages): **0 stale docs** (zero with `evidence>0 & intent==0`). The `CORPUS-COVERAGE.2` re-ingest sweep — which rebuilds documents via `converge`, cascading the whole pipeline — already landed the recovered relations canonically. Concrete (each `intent`==`evidence`): `tilelink_1_7_1` 33, `tilelink_1_8_0` 34, `um10204` I2C **17**, `gic_600` 101, `mmu_700` 25, ATS `ihi0082` 9, DTI 1, opencapi-TL 15, USB4 13. `wbspec` now 0/0 (re-ingest reclassified to honest-absence). 33 register/PHY/command docs at 0/0 = the (B) honest-absence class.
- **(ii) Stage-staleness detector — verified DONE (shipped as `CORPUS-COVERAGE.1`).** `crates/specforge/src/commands/validate.rs::stage_staleness_relation_finding` emits `semantic_stale_relations_dropped` (line ~4451) / `intent_stale_relations_dropped` (line ~6247), category `stage_staleness`, when a downstream artifact carries 0 relations while its loaded upstream carries some. Three unit tests confirm: fires on emptied-vs-nonempty-upstream; silent when downstream carries relations or when upstream is also empty (so honest absence is never flagged). This makes the staleness recoverable-and-surfaced rather than silent — exactly the north-star "the KG must be COMPLETE, and a silent drop must be surfaced" requirement.
- **Why this is an honest closure, not victory-declaring:** the recoverable stale class is genuinely empty corpus-wide (measured, not assumed), AND there is a code-level detector preventing silent recurrence. The honest-absence docs are deliberately LEFT at 0 relations — minting relations for register/coherency protocols would fabricate (the north-star caution). Relation-completeness is the wrong bar dimension there; register-/message-field-completeness is the right one (those surfaces are non-empty for exactly those docs). Wire protocols stay held at WIRE-BASED-100 = 1.000.
- **Gates:** no code change in this closure (a measurement + verification of an already-shipped detector and an already-cascaded refresh), so `kg-bench`/WIRE-BASED-100/`run_ci` are orthogonal by construction; `scripts/check_doctrines.sh` GREEN (memory-arch + knowledge-map + task-acceptance). Report closure appended to `docs/research/relation-completeness-measurement.md` (with a re-census reverify block); KM cards `relation-completeness-staleness-vs-absence` (closure addendum) and `stage-staleness-validate-detector`.

## KG-ISF-COMPLETENESS.2a.vi (`2026-06-23`) — CODE: ISF rule-drive-value validity gate → renderable corpus 70/70 FSMGen-strict-clean
- **Slice:** fresh-session PNT continuation — closing the LAST open ISF-emit strict-FAIL (the spun-out `ISF-VALUE-WIDTH-EMIT` `(port expr)` Non-Goal, surfaced as the lone FAIL after Lever C `.2a.v`). Owned as `KG-ISF-COMPLETENESS.2a.vi`.
- **Reproduce (real FSMGen):** AXI+ACE `ihi0022_h_c` `manager.isf` → `fsmgen --strict --check` `Error: rule 'constraint_48' assignment actions require '(port expr)'`.
- **Root cause (WHY + WHERE):** the rule's drive VALUE is free PROSE — `(RLOOP the value that was presented on the ARLOOP signal)` / `(BLOOP … AWLOOP …)`, an AXI+ACE loopback-tag obligation the extractor captured as a sentence rather than a literal. FSMGen requires a rule assignment-action RHS to be a renderable value expression (`(port expr)`); the emitter (`ir/isf_ir.rs`) renders rule drive values verbatim with no value-validity gate (the existing gates cover enum members `.2a.iv` and value width `ISF-VALUE-WIDTH-EMIT`, not the rule's own scalar).
- **Fix:** new pass `drop_unrenderable_rule_values` (`isf_ir.rs`, BEFORE the width/dedup passes so a prose rule never reaches value processing) drops a rule whose any drive value fails `is_safe_isf_scalar_value` (non-empty, whitespace-free) + records an `isf_rule_value_<name>` residual. The prose value is unrecoverable — "the value that was *presented* on ARLOOP" is a temporal loopback, NOT the current `(port ARLOOP)`, so recovering a `(port expr)` would fabricate the timing — honest residual over fabrication (`[[feedback_isf_no_hacks]]`). ADR-0006: a structural value-shape test, no name list; every legitimate drive (scalar `0`/`1`/`0b01`, width-cast `7'd125`, enum symbol `VALID`) passes.
- **Verified:** read-only scan of all 70 current-emit `.isf` → EXACTLY 4 prose-valued drives, ALL in `ihi0022_h_c` (`constraint_48`/`_49`/`tinv_sc_llm_sigcon_0061`/`_0062`); 0 elsewhere → byte-identical on every other doc by construction. AXI+ACE re-emit FSMGen success / 0 diagnostics (4 `isf_rule_value_*` residuals). **Full FSMGen sweep over all 70 current emits → 70/70 strict-clean** (was 69/70; 0 PASS→FAIL). +1 unit test; `kg-bench` 156/156; `run_ci.sh` GREEN (lib **1709**). WIRE-BASED-100 orthogonal by construction (the only Rust file changed is the `.isf` emitter; `eval-extraction` reads the IR). **The ISF-emit strict-FAIL frontier (Levers A/B/C/F + this value gate) is now fully CLOSED.**

## KG-ISF-COMPLETENESS.2a.v (`2026-06-23`) — CODE: ISF unconditional-rule-overlap conflict residual (Lever C)
- **Slice:** fresh-session PNT, the last open ISF strict-FAIL lever. Owned as `KG-ISF-COMPLETENESS.2a.v` (the `.2a.*` ISF-lowering-fidelity series, sibling of the same-guard `ISF-RULE-CONFLICT-RESIDUAL` drop and `.2a.iv` Lever F).
- **Reproduce (real FSMGen):** a fresh re-emit of the AMBA Low Power Interface (`ihi0068_d`) `controller.isf` fails `subs/fsmgen/bin/fsmgen --strict --check --json` with `isf_conflicting_rule_writes` on `PREQ` (`rule_5` unconditional ←1 vs guarded `temporal_..._dyn_sigcon_0012` `(== PACCEPT 0)` ←0; a twin `PACCEPT` conflict is masked behind it — FSMGen confesses one at a time). A full corpus sweep showed the SAME class also failing the AXI/AHB/AXI-Stream wire golds + LTI + NVMe; the resume-pointer "27/28 clean, 1 FAIL (LPI)" tally was stale (the cached wire-gold `.isf` are byte-identical to fresh AND already FAIL).
- **Root cause (WHY + WHERE):** SpecForge's `dedup_conflicting_rules` (`crates/specforge/src/ir/isf_ir.rs`) keys conflict detection on `(signal, condition)` — the same-guard model — so an unconditional rule (`IsfRule.condition == ""`) and a guarded rule hash to different keys and the overlap is never detected. FSMGen flags it: its `_condition_terms_prove_disjoint` (`subs/fsmgen/perl/FSM/Scheduler/ISF/LoweringIR.pm:10458`) returns "not disjoint" for an absent condition (`condition_terms` undef), so an unconditional rule's firing set ⊇ every guard and conflicts with any different-value rule on the target (`_build_conflict_issues:10888`).
- **Design choice (drop+residual, NOT priority):** FSMGen offers three conflict escape-hatches — same value (`_compatible_record_pair`), provably-disjoint guards, and a `(priority …)` (`_priority_resolved_record_pair:10953`). The `(priority …)` hatch was empirically tested (`priority rule_5 over ..._0012`) and **rejected**: it cleared that one pair but `..._0012` then conflicted with the next unconditional `PREQ←1` rule (`rule_6`) — keeping the guarded minority rule would require asserting an ungrounded precedence over EVERY same-value unconditional rule, i.e. fabrication (`[[feedback_isf_no_hacks]]`). So the fix extends the established same-guard drop+residual pattern to the cross-guard unconditional case.
- **Implementation:** new `drop_unconditional_overlap_conflicts` + `unconditional_overlap_residual_packet` (`isf_ir.rs`), wired in `from_intent_ir` AFTER `dedup_conflicting_rules` (so ≤1 unconditional value per signal) and BEFORE priority emit (so a dropped rule gets no dangling `(priority …)`). Per signal with an unconditional driver value `V`, drop every other rule driving it to `≠ V`; record an `isf_unconditional_overlap_<name>` `ResidualDecisionPacket` (the unconditional value kept; dropped obligation explicit). Precise — it drops exactly FSMGen's flagged overlap, so a strict-clean doc (which cannot contain such a config) re-emits byte-identical.
- **Verified:** LPI `controller.isf`+`channel.isf` FSMGen success / 0 diagnostics; `..._0011`+`..._0012` dropped + residualized (adapter `residual_decision_count` 4→6). Re-emit diff (current binary WITH vs WITHOUT the change, via `git stash` of `isf_ir.rs`): exactly **7 `.isf` differ** (all current primary emits), every other emit byte-identical; FSMGen on the 7 → **6 FAIL→PASS** (AXI `ihi0022_l` / AHB `ihi0033_c` / AXI-Stream `ihi0051_b` wire golds + LPI + LTI `ihi0089_d` + NVMe), **0 PASS→FAIL**, AXI+ACE `ihi0022_h_c` stays FAIL on the orthogonal `(port expr)` grammar (a spun-out `ISF-VALUE-WIDTH-EMIT` Non-Goal). +2 unit tests; `kg-bench` 156/156; `run_ci.sh` GREEN (lib **1708**). WIRE-BASED-100 orthogonal by construction (the only Rust file changed is the `.isf` emitter; `eval-extraction` reads the IR).
- **Tally correction (same slice, `2026-06-23`):** the per-`.isf` tally was first reported over a stale-inclusive 107-file cache. `adapt` emits only ONE primary-actor `.isf` per doc — the other 37 were STALE cruft from older binary versions (e.g. AHB `address_decoder.isf`, HBM2 `bit_fields.isf`, AXI-Stream `agent.isf`, the un-sanitized GIC-600 `redistributor→…`). Cleaned them (git-ignored cache; serves the artifact-cleanup directive) and re-swept the **70 current emits**: **69/70 PASS**, the lone FAIL = `ihi0022_h_c` `(port expr)`. So after Lever C the ISF-emit strict-FAIL frontier is effectively CLOSED (Levers A/B/C/F all resolved; the remaining `(port expr)` is a spun-out Non-Goal).

## KG-ISF-COMPLETENESS.2a.iv (`2026-06-23`) — CODE: ISF enum value-literal emit-gate (Lever F) + Lever A drift correction
- **Slice:** fresh-session PNT. A probe re-verified the `CORPUS-COVERAGE.2` "3 strict-FAIL" tally (DTI/LPI/HBM2) against the current binary + the real FSMGen, then landed the one open emitter lever confirmed (Lever F). Owned as `KG-ISF-COMPLETENESS.2a.iv` (the `.2a.*` ISF-lowering-fidelity series, sibling of `.2a.iii` module-name sanitization).
- **Measurement-first correction (a wrong hypothesis caught — the value of diffing + verifying FSMGen):** the first-cut root cause was "count-derived enum width overflow" — `IsfIr::from_intent_ir` (`ir/isf_ir.rs:878`) derives the backing `(type … (bits B))` width from member COUNT (`ceil(log2(count))`), so a member value `>= 2^B` overflows. A width-fits gate was coded; a before/after `.isf` diff DISPROVED it (it wrongly dropped legitimate AXI `AWATOP`(49)/`AWSNOOP`/`ARSNOOP` and AHB `TABLE`(64) enums), and a value sweep against the pinned FSMGen showed FSMGen accepts bare decimals of ANY magnitude (GIC-600's `TABLE` carries `69152` and is strict-clean). So width is NOT the rule.
- **The actual FSMGen rule (verified by value sweep, `[[feedback_verify_fsmgen_before_fr]]`):** the package-symbol parser rejects a **bare token of only binary digits (`0`/`1`) with length >= 4** — an un-qualified binary literal. `1000`/`1010`/`1111`/`10000` fail; `0`/`1`/`10`/`111` (<= 3 binary digits) and any value with a 2-9 digit (`999`/`1020`/`69152`) pass; a width/radix-qualified literal (`4'b1000`/`16'd1000`/`8'hA5`) passes at any magnitude. HBM2's `TABLE.REPAIR_LANE` values (`1000,1001,1110,1111`) are exactly that shape — binary codes the extractor mis-read as bare decimals in a mega-conflated `TABLE`.
- **Root cause (WHERE):** `emitted_enums()` (`ir/isf_ir.rs`) gated only on `is_safe_isf_scalar_value` (whitespace-free), so a bare `1000` reached FSMGen and broke the whole `.isf`.
- **Change:** new `isf_enum_value_is_emittable_literal(value)` (FALSE only for a bare `[01]`-only token of length >= 4); free fn `isf_enum_is_emittable(e)` (non-empty + every member a safe scalar AND emittable-literal); `emitted_enums()` filters on it; `enum_residuals(&self)` records an `isf_enum_value_literal_<name>` packet for an enum dropped specifically by this gate (all members safe-scalar but >= 1 binary token) — pre-existing operator-expression drops keep their prior silent exclusion (no new residual → byte-identical adapter surface for those docs); `adapters.rs` extends `residual_decisions`. +2 unit tests.
- **Verified (per item):** HBM2 `hbm.isf` → FSMGen `--strict --check --json` **success / 0 diagnostics**; only `TABLE` dropped (residual `isf_enum_value_literal_table`); `EXTEST_RX`(212)/`DWORD_MISR`(19) KEPT; 0 `TABLE.<member>` references so dropping strands nothing.
- **No regression (oracles):** fresh re-emit + `diff` of ALL `generated/adapters/isf/*/*.isf` vs a pre-change baseline → the ONLY changed file is HBM2 `hbm.isf`; wire golds (APB/AHB/AXI/SWD) + Avalon + CoreSight SoC-600 + GIC-600 (`69152`) + ARM-Debug (`3360`) BYTE-IDENTICAL (the binary-token criterion never flags a legit decimal). `run_ci.sh` GREEN (lib 1706, +2; clippy/fmt/rustdoc warning-deny + mdBook); `kg-bench` 156/156; WIRE-BASED-100 orthogonal (emitter-only); doctrines GREEN.
- **Lever A drift corrected:** DTI is already strict-clean (`ATST` = `1'd1`, 0 diagnostics) — the `CORPUS-COVERAGE.2` #8 `2'b1` strict-FAIL was a stale log entry (`ISF-VALUE-WIDTH-EMIT.2` fixed it; never re-verified). Tally corrected to 27/28 clean, 1 FAIL (LPI Lever C).
- **Honest residual:** the upstream mega-enum conflation + binary-as-decimal mis-read (generic `TABLE` sweeping ~10 tables) stays a future extraction-precision lever; the emitter cannot recover the lost radix without fabricating. KM `[[isf-enum-value-literal-emit-gate]]`. `[[feedback_isf_no_hacks]]`.

## KG-ISF-COMPLETENESS.1c.ii (`2026-06-23`) — MEASURED: no clean within-doc gate for the bulk phantom class → bounded residual (read-only, no code)
- **Slice:** read-only measurement of the bulk dense-prose phantom class (multi-word, single-`REL-INFERRED`, leading-noun relation subjects — eMMC carries 109 multi-word actors). Same-session continuation after `.1c.i` landed the clean trailing-strip win.
- **Candidate tested + disproven:** a `.1b.ii`-style connectivity fold — rewrite a multi-word relation subject `A … Z` onto its last content token `Z` when `Z` is an independently-connected single-word agent in the doc. Measured on eMMC (19 candidates) + the wire golds + AXI+ACE `ihi0022_h_c` (15 candidates). **The rule mishandles the real cases:** on AXI+ACE it correctly folds `caching Manager`/`initiating Manager`/`snooped Manager`/`originating Manager`→`Manager` (descriptive references to the real Manager) but WRONGLY folds `Manager component`/`intermediate component`/`participating component`→`component` (the agent is the modifier `Manager`, not the noun-phrase head `component`). A fold-on-FIRST-token rule instead breaks `caching Manager`→`caching`. The agent token's position in the noun phrase varies (modifier vs head), so no fixed structural position is safe; and `full AXI`→`AXI`, `AxDOMAIN signal`→`signal`, `read barrier`→`barrier` fold onto non-agent tails.
- **Root cause (why no structural gate exists):** within one document a real descriptive reference (`caching Manager`) and a phantom fragment (`basic bus`) are structurally INDISTINGUISHABLE — same single-relation participation, same `SECTION-PHASE`/`PROSE-PARA` responsibility provenance (a relation's statement always sits in some section, so the phantom inherits section-phase grounding), same multi-word noun-phrase shape. The eMMC EvidenceIR actor surface is ONLY `actor_signal_relations` (no `ProtocolActorRecord` agent-definition records), so there is no grounding signal to separate them. A name-shape/connectivity DROP is forbidden by the genericity guardrail (loses real agents); a participation threshold by the completeness north star (a rare-but-real agent can appear in exactly one relation).
- **Outcome:** `.1c.ii` is a **bounded honest residual**, not a downstream gate. The genuine fix is upstream relation-subject extraction precision on descriptive prose — reading the actual grammatical subject of a who-acts-on-what sentence rather than a noun phrase near the signal — i.e. the owner-directed in-Rust shallow-parse direction (`project_nlp_shallow_parse_direction`, `docs/tasks/NLP-SHALLOW-PARSE.md`). Until that lands the phantoms stay a residual; they never reach `.isf` (the adapter lowers the renderable initiator's signals/behaviours, never raw `actors[]`), so the bounded cost is IntentIR `actors[]` precision (bar #1) on dense-prose specs. **Lever E fully scoped: clean structural win `.1c.i` shipped; remainder upstream-NLP-gated.**
- **Gates:** no code change → WIRE-BASED-100 + register/wire golds + `kg-bench` orthogonal by construction; `scripts/check_doctrines.sh` GREEN. Report `docs/research/agent-identity-prose-class-measurement.md` §8; KM `agent-identity-prose-class-measurement` updated.

## KG-ISF-COMPLETENESS.1c.i (`2026-06-23`) — CODE: dense-prose trailing preposition/auxiliary strip
- **Slice:** same-session continuation of the `.1c` probe (fresh session, measurement loaded). Lands the measurement-clean first gate of Lever E — the safe, corpus-verified extension of the `.1b.i` trailing-fragment strip to trailing prepositions + auxiliaries/modals.
- **Root cause (WHY+WHERE):** `consolidate_trailing_fragment` (`crates/specforge/src/ir/evidence.rs`) stripped a trailing token only when it was a `NON_ACTOR_LEADING_VERB` or a `NON_ACTOR_TRAILING_DISCOURSE_MARKER`. `.1b.i` deliberately excluded prepositions (for the conjunction/`.1b.iii` case), so dense-prose relation subjects `host has`/`host to`/`host is`/`cache in` survived as separate phantom actors at the `normalize_relation_actor_name` seam (leading token a noun → `.1a` passes; carries a relation → `.1b.iv` can't touch).
- **Change:** added `NON_ACTOR_TRAILING_FUNCTION_WORDS` — a closed class of prepositions (`of`/`to`/`for`/`with`/`in`/…, NOT `before`/`after`/`until` which are already discourse markers) + auxiliaries/copulas/modals (`is`/`are`/`has`/`have`/`do`/`will`/`shall`/`can`/`may`/`must`/…) — and a third OR-clause in the strip condition. The const is a deliberate SUBSET of `NON_ACTOR_LEADING_FUNCTION_WORDS` and EXCLUDES conjunctions (a trailing `and`/`or` is a coordinated-subject remnant `.1b.iii` splits, never strips). Two drift guards: `trailing_function_words_are_known_leading_non_conjunctions` (subset-of-leading + no conjunction) + a `.1c.i` consolidation test (`host has`/`host to`/`cache in`→head, `advantage of`→`advantage` normalized-not-recovered, `Subordinate and` NOT stripped, real agents byte-identical).
- **Verified (per item):** new-binary `evidence→semantic→intent --dry-run` cascade on `jesd84_b50_2013_09_emmc_5_0` → **actors 153→138** (−15), relations 349→341; host variants 11→7 (`host has`/`host is`/`host to`/`host with` fold onto `host`); 29 phantom names removed (`cache in`/`cache is`→cache, `device to`→device, `CMD to`/`CMD in`, `advantage of`/`any of`/`cases of` normalized). `host.isf` (62 signals) renders + `subs/fsmgen/bin/fsmgen --strict --check --json` success / 0 diagnostics (emitter-safe — the change feeds the actor graph but the merged `host` edges lower cleanly).
- **No regression (oracles):** WIRE-BASED-100 = 1.000 on fresh-Pattern new-binary evidence (4 gold docs rebuilt into a temp evidence-root via `evidence --dry-run`; `eval-extraction seed_{apb,ahb,axi,swd}{,_temporal} --provider skip --evidence-root <temp>` → filtered F1 constraints 6/6·6/6·3/3, relations 5/5·6/6·6/6·1/1, temporal 3/3·4/4·3/3; SWD lone constraint 0/1 documented promotion-only). `kg-bench` 156/156. `run_ci.sh` GREEN (lib 1704, +2; clippy/fmt/rustdoc warning-deny + mdBook).
- **Genericity (ADR 0006):** universal closed-class grammar, no name list; corpus-wide safety MEASURED — 0 ≥8-port actors are `X <aux/prep>` across all 78 docs, so no real high-participation agent is renamed; wire golds carry no such actor (byte-identical).
- **Deferred:** the bulk of the dense-prose explosion (single-relation noun-phrase phantoms with a leading noun, `basic bus`/`actual sector`) is the `.1c.ii` participation/grounding-precision problem — a name-shape drop is disproven unsafe (AMBA real agents share the shape). Frontier → `.1c.ii` measurement.

## KG-ISF-COMPLETENESS.1c (`2026-06-23`) — Lever E agent-identity precision for the DENSE-PROSE doc class: OWNED + PROBE (read-only, no code)
- **Slice:** fresh-session PNT pivot off `CORPUS-COVERAGE.2`. Two consecutive re-ingests (#27 eMMC → lever E, #28 HBM2 → lever F) each surfaced a NEW substantive lever rather than a clean refresh, so per the owner's "attack substantive gaps, not easy incremental" directive (`feedback_not_complete_attack_substantive_gaps`) the high-value move is ACTING on a surfaced lever. This is the measurement-first PROBE that owns + scopes Lever E (the established discipline: every `.1`/`.1a`/`.1b` gate was preceded by a read-only measurement slice).
- **Method:** read-only python over `generated/intent_ir/<key>/intent_ir.json` (78 persisted docs), current binary unchanged. Three probes: (1) actor port/relation participation + provenance shape on eMMC vs HBM2 vs the AMBA gold docs; (2) structural name-shape classification across the explosion + the gold docs; (3) corpus-wide safety + reach of a trailing aux/prep strip.
- **Finding 1 — relation-subject seam, not actors[] prose-mint:** eMMC has 153 actors; joining `actor_ports` (by `actor_id`) + counting `actor_signal_relations` (by `actor_name`) → **148 CONNECTED** (≥1 port AND ≥1 relation), each minted from a single `REL-INFERRED` relation subject (`"advantage of" reads BACKGROUND`, `"basic bus"`, `"B write"`, `"actual sector"`); **5 pure-unconnected** (`channel`/`controller`/`source`/`target`/`transmitter`, all `PROSE-PARA`+`SECTION-PHASE` grounded). Of the 5, **0** carry ONLY a pure-inferred marker → `.1b.iv` correctly drops 0. So the explosion lives at the `normalize_relation_actor_name` / `actor_signal_relation_surface` seam (where `.1a`/`.1b.i`/`.1b.iii` operate): the leading token is a noun so `.1a` (first-token POS) passes it; it carries a relation/port so `.1b.iv` (0/0-only) can't touch it.
- **Finding 2 — shape-only drop disproven unsafe:** classifying actor names by structural shape, AMBA's REAL agents occupy the SAME classes as eMMC's phantoms — AHB/AXI `agent`/`bus`/`controller`/`decoder`/`device` are real single-lowercase-noun agents, `address decoder`/`Exclusive Access Monitor` real multiword agents — so a shape drop would destroy them and fail WIRE-BASED-100. The differentiator must be grammatical normalization (rewrite) or participation/grounding, never shape (the genericity guardrail).
- **Finding 3 — clean safe first gate (`.1c.i`), measured:** extend the `.1b.i` trailing-strip from verbs/discourse to a closed class of trailing prepositions + auxiliaries (`advantage of`→`advantage`, `host has`→`host`, `cache in`→`cache`). **Safety CLEAN:** across all 78 docs, **0** actors with ≥8 ports are `X <aux/prep>` shaped → the strip renames no real high-participation agent (the `.1a`/`.1b.i` bar); the 4 wire golds carry no such actor. **Reach:** 138 names corpus-wide (`...of` 39, `...to` 28, `...is` 23, `...in`/`...has` 7, …) across ~17 docs; the dense AXI+ACE `ihi0022_h_c` (189 actors) and CHI `ihi0050_g` (87) explode too → the doc class is **dense-prose**, not "non-AMBA". Necessary-not-sufficient (eMMC 29 of 153).
- **Deferred (`.1c.ii`):** the bulk ≈120 are single-relation noun-phrase phantoms with a leading noun; needs a participation+grounding discriminator + its own measurement (a rare-but-real agent can also appear in one relation — completeness forbids dropping it). Deferred-with-trigger, like `.1b.ii`/`.1b.iv` were.
- **Artifacts:** report `docs/research/agent-identity-prose-class-measurement.md`; KM `agent-identity-prose-class-measurement`; task leaf `KG-ISF-COMPLETENESS.1c`; book caveat `pipeline/evidenceir.md`.
- **Gates:** no code change → WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal by construction; `scripts/check_doctrines.sh` GREEN; `mdbook build` GREEN. Frontier → `.1c.i` (best on a fresh session per the high-stakes gate-code rule).

## CORPUS-COVERAGE.2 (`2026-06-23`) — re-ingest #28 JEDEC HBM2 DRAM (MIXED: confirms prose-specificity of lever E; surfaces lever F)
- **Slice:** fresh-session PNT, register/TRM phase, **deliberate diagnostic pick** — a DRAM register/timing-table spec chosen as the structured contrast to #27's descriptive prose, to test whether the phantom-actor explosion is prose-specific. Doc: JEDEC HBM2 DRAM (`jesd235a_2015_11_hbm2_dram`, 172pp / 224 visual), via `.cache/local-references/chipdoc/jedec/hbm/current/JESD235A_2015-11_HBM2_DRAM.pdf`.
- **Method:** `DOCLING_DEVICE=cpu specforge ingest` → cascade on the current release binary (no rebuild). ~2 min, RAM 81% free.
- **DIAGNOSTIC RESULT — the phantom explosion is PROSE-SPECIFIC (hypothesis confirmed):** HBM2 actors **52→38 CONSOLIDATED DOWN** (the `.1a`/`.1b`/`.1b.iv` agent-identity gates fold fragment/phantom actors, exactly like the structured AMBA/CoreSight class), vs #27 eMMC's 20→153 explosion. The difference is doc style: eMMC is heavily descriptive prose ("the host shall…/the device responds…") yielding many relation-subject fragments; HBM2 is timing-parameter/command-truth-table oriented with far fewer prose subjects. So lever E (agent-identity precision) is correctly scoped to descriptive-prose / non-AMBA specs, not register/timing-table specs.
- **BEFORE → AFTER (genuine refresh on the structured surfaces):** register_records 17 held; `message_field_records` absent→0 (HBM2 carries no bit-position structure tables — honest); signal_constraints 20→12; **transactions 0→3** [`read_operation`, `write_operation`, `trr_mode_operation`, recognition-only]; semantic `transaction_anchors` 0→3; interfaces 72→31; actors 52→38; actor_signal_relations 89→55; extracted_statements 2843→2801; intent constraints 540→480.
- **NEW lever F (ISF-emitter, kin to Lever A — the strict-FAIL):** `fsmgen --strict --check --json` on `hbm.isf` returns success=false / 1 diagnostic: *"Package 'ISF actor 'hbm'' contains '+enums' entry for enum member 'TABLE.REPAIR_LANE_8' with value token '1000', but package symbol values currently must resolve to literal scalar values such as '0', '8'h3', '8'hA5', or 'const_8b0'."* Inspecting the `.isf`: `(type TABLE (bits 6))` is a generic-named mega-enum with members `(REPAIR_LANE_0 0) (REPAIR_LANE_1 1) (REPAIR_LANE_2 10) (REPAIR_LANE_3 11) … (REPAIR_LANE_8 1000) (REPAIR_LANE_9 1001) … (DEFAULT___NO_REPAIR 1111)` PLUS ~50 more members from entirely DIFFERENT doc tables (microbump-pitch descriptions, a test-operation list, IDD currents `IDD0..IDD4W`, mode-register refs) — values RESTART (multiple `0`/`1`/`2`), so it conflates ~10 distinct tables into one enum named `TABLE`. Two defects: (1) **enum-value literal format** — the REPAIR_LANE values are BINARY codes whose radix was stripped, emitted as bare decimal-looking tokens, so FSMGen can't resolve `1000` as a scalar literal (needs `4'b1000`), AND the bare token is semantically WRONG (binary 1000 = 8, not decimal 1000); (2) **generic-`TABLE` mega-enum precision** — a captioned-`Table N` heading was used as the enum NAME and many tables collapsed together (an enum analogue of the #27 phantom-actor precision gap).
- **Why honest, not fixed in-slice:** per the tree rule, re-ingest slices surface + scope levers but don't fix them. Lever F belongs in the `ISF-*-EMIT` value-literal family (with Lever A); the binary-radix-preservation + enum-naming-precision parts may need both an emitter fix and an upstream enum-extraction fix. Owned future leaf; WIRE-BASED-100 + register/wire golds + `kg-bench` gating required.
- **Verification:** `hbm.isf` (15 signals / 17 storage(reset) / 8 enums / 28 rules / 2 txns) → `subs/fsmgen/bin/fsmgen --strict --check --json` = **success=false / 1 diagnostic (Lever F)**. `validate` no stage-staleness, 55/100 ADEQUATE, transaction inventory 3 recognition-only.
- **Gates:** no code change → WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal by construction; `scripts/check_doctrines.sh` GREEN; no book change (HBM2 adds 0 message-fields). Coverage 28 of 57; 25/28 strict-clean (3 FAIL: DTI/A, LPI/C, HBM2/F).

## CORPUS-COVERAGE.2 (`2026-06-23`) — re-ingest #27 JEDEC eMMC 5.0 (MIXED: transaction win + phantom-actor explosion → lever E)
- **Slice:** fresh-session PNT, register/TRM phase, **first descriptive-prose register/protocol spec re-ingested**. Doc: JEDEC eMMC 5.0 (`jesd84_b50_2013_09_emmc_5_0`, 296pp / 344 visual), via `.cache/local-references/chipdoc/jedec/emmc/current/JESD84-B50_2013-09_eMMC_5.0.pdf`.
- **Method:** `DOCLING_DEVICE=cpu specforge ingest` → cascade on the current release binary (no rebuild). ~5 min, RAM 80–82% free, `.4a` guard armed, Ollama idle.
- **BEFORE → AFTER (MIXED — genuine transaction win + register hold, BUT a phantom-actor explosion):** register_records 17 held (exact); `message_field_records` absent→still 0 (eMMC carries no bit-position structure tables — honest); signal_constraints 0→2; **transactions 0→6** [`boot_operation`, `alternative_boot_operation`, `device_lock_unlock_operation`, `dual_data_rate_mode_operation`, `background_operation`, `h_w_reset_operation` — 5 with grounded signal set (9 members), all REAL eMMC operations; one carries a 2-phase grouping]; semantic `transaction_anchors` 0→6; interfaces 9→38; **actors 20→153**; **actor_signal_relations 23→349**; extracted_statements 6544→7017; intent constraints 939→1071, behaviors 1203→1314.
- **The phantom-actor explosion (honest root-cause — NOT papered over):** of the 153 intent actors, only ~10–15 are real agents (`host`, `device`, `controller`, `adapter`, `master`, `slave`, `transmitter`, `RPMB`, `e MMC`, `target`, `source`, `channel`, `partition`). The rest are phantom sentence-fragments — `host has`/`host is`/`host to`/`host with`/`host tries`/`host wants`/`host selects` (real agent + trailing auxiliary/preposition/verb), `cache in`/`cache is`, `B write`/`CMD to`/`CMD in`, `device behaves`/`device validates`, plus bare-noun fragments `following`/`value`/`field is`/`requested`. The fresh Docling extraction produced 7017 statements (vs the retained stale 6544), and eMMC's *descriptive prose* (vs AMBA's structured signal tables) yields far more relation-subject candidates that slip the `.1a` (leading function-word/verb reject) and `.1b.i` (trailing-verb/discourse-marker strip) gates — those gates were measured to reject ZERO ≥8-port actors across the AMBA-style persisted corpus, but eMMC's trailing AUXILIARIES (`has`/`is`) and PREPOSITIONS (`to`/`with`) are not in the trailing-strip vocabulary, so `host has`/`host to` survive as separate phantoms instead of consolidating onto `host`.
- **Why this is a KG-fidelity gap, not a strict-FAIL:** the ISF emitter lowers signals/behaviors for the single renderable actor (`host`), never the raw `actors[]` list, so `host.isf` is strict-clean (`fsmgen --strict --check --json` success / 0 diagnostics). The damage is to IntentIR's `actors[]` precision (north-star bar #1: every actor a real agent) and the inflated `actor_signal_relations` (many are phantom-actor edges). This is exactly the substantive precision gap the owner wants attacked — but per the tree rule it is NOT fixed inside a re-ingest slice.
- **NEW spun-out lever E — agent-identity precision for descriptive-prose / non-AMBA specs:** owned future leaf under `KG-ISF-COMPLETENESS`; probe-first + structural gates (NOT a fragment denylist — `feedback_avoid_denylists_prefer_structural`). Candidate structural rules to measure: extend the `.1b.i` trailing-strip to trailing closed-class AUXILIARIES/PREPOSITIONS (consolidate `host has`/`host to`→`host`); a participation/port-count floor for minting a relation-subject actor on prose docs; a head-noun-must-be-a-known-role-or-have-a-port gate. HIGH-VALUE: the remaining prose tail (USB4/USB3.2, JEDEC HBM, the ARM guides) will likely recur this.
- **Verification:** `host.isf` (62 signals / 17 storage(reset) / 28 enums / 16 rules / 0 txns) → `subs/fsmgen/bin/fsmgen --strict --check --json` = **success / 0 diagnostics**. `validate` no stage-staleness (relations 349-vs-349 non-zero — silent), quality 51/100 ADEQUATE, transaction inventory 6 (5 grounded, 9 members, 1 phase-grouped), plus signal_connectivity warnings (10 no producer / 12 no consumer) and 5 same-actor graph-direction conflicts (some on phantom actors like `actor_high_priority`).
- **Gates:** no code change → WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal by construction; `scripts/check_doctrines.sh` GREEN; `mdbook build` GREEN (no book number changed — eMMC adds 0 message-fields). Coverage 27 of 57; 25/27 strict-clean.

## CORPUS-COVERAGE.2 (`2026-06-23`) — re-ingest #26 Intel VT-d 5.0 (genuine refresh; corrects the #22 survey)
- **Slice:** fresh-session PNT, register/TRM phase, first distinct-vendor (Intel) re-ingest of the tail. Doc: Intel VT-d 5.0 (`5_0_2024_08_intel_virtualization_technology_for_directed_io_specification`, 354pp / 810 visual), via the git-ignored `.cache/local-references/chipdoc` symlink (`intel/system-ip/vt-d/current/5.0_2024-08_…pdf`).
- **Method:** `DOCLING_DEVICE=cpu specforge ingest` → cascade on the current release binary (no rebuild — no Rust source newer than the binary). ~4 min, RAM 80–83% free, `.4a` guard armed, Ollama idle (idle `serve` only).
- **BEFORE → AFTER (GENUINE refresh — the #22 one-pass survey was WRONG to call VT-d "already current-binary-equivalent"):** register_records 103 held (exact); **`message_field_records` key ABSENT (stale evidence predates the surface) → 15 fields / 4 containers** (the `.10c` structure-field family fires on VT-d's `Root-Entry Format`/context-table structures); **transactions 0→2** [`device_tlb_operation`, `set_root_table_pointer_operation`, both recognition-only]; semantic `transaction_anchors` absent→2; intent actors 14→13 (agent-identity consolidation); signal_constraints 1→4; semantic temporal_rules 1→4; actor_signal_relations 0→0 (memory-mapped register spec — honest absence). intent constraints/behaviors held (1200/1164).
- **Root-cause of the survey miss:** the #22 survey predicted VT-d current by reading only `register_records=103`. But a doc's stale evidence sits at whatever binary last rebuilt it; VT-d's evidence had been rebuilt from the retained `source_ir.json` at a binary that had the `.10c`/`.10g` *register* families but predated `message_field_records` (`.10b`/`.10c` structure routing) AND the section-heading transaction recognizer. So register-count parity ≠ full-surface parity — the survey under-counts refresh value, the key phase correction from this slice.
- **Verification:** `agent.isf` (510 signals / 103 storage(reset) / 4 enums / 44 rules / 0 txns) → `subs/fsmgen/bin/fsmgen --strict --check --json` = **success / 0 diagnostics** (strict-clean; fsmgen is the Perl `bin/fsmgen`, no cargo build). `validate` no stage-staleness (intent relations 0-vs-0 honest absence — the `.1` detector correctly silent), quality 8/100 INCOMPLETE (honest), transaction inventory = 2 recognition-only, doc-completeness gauge registers_without_fields 0/103, registers_unresolved_width 2/103, unexplained_intent_bearing_tables 7/17 (Lever-D opportunity).
- **Book sync (measured, no fabrication):** ran a corpus-wide count over `generated/evidence_ir/*/` → in-memory-structure `message_field_records` total = **1,235 fields across 12 documents** (was exactly 1,220/11 before VT-d; VT-d is the 12th). Updated `docs/book/src/document-categories.md:73` accordingly; this reconciles a latent inconsistency — `evidenceir.md:674` already claimed "VT-d gains 15 typed structure fields" from the `.10c` implementation, but the persisted corpus total never counted them because VT-d had not been re-ingested at HEAD until now.
- **Gates:** no code change → WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal by construction; `scripts/check_doctrines.sh` GREEN. Coverage 26 of 57; 24/26 strict-clean.

## CORPUS-COVERAGE.2 (`2026-06-23`) — re-ingest #25 CoreSight SoC-600 0100 TRM (completes SoC-600 cluster)
- **Slice:** fresh-session PNT, register/TRM phase. Doc: CoreSight SoC-600 0100 TRM (`100806_0100_00_2017_08_01_coresight_soc_600_technical_reference_manual`, 702pp / 1157 visual), via the git-ignored `.cache/local-references/chipdoc` symlink. **Last of the SoC-600 family** (0701=#23, 0200=#24, 0100=#25) → cluster complete.
- **Method:** `DOCLING_DEVICE=cpu specforge ingest` → cascade on the current release binary (no rebuild). ~5 min, RAM 67–77% free, `.4a` guard armed, Ollama idle.
- **BEFORE → AFTER (GENUINE refresh, same #23/#24/#17 class):** register_records 597 held (exact); **actor_signal_relations 41→31 / actors 44→34 / interfaces 12→5** (consolidation gates fold fragment/phantom actors); **transactions 0→1** (recognizer fires); conditional_rules 46 held; intent constraints 759→745; 0 message-fields (honest).
- **Verification:** `dp.isf` (597 storage(reset) / 12 enums / 3 rules / 4 signals) → `subs/fsmgen/bin/fsmgen --strict --check --json` = **success / 0 diagnostics** (strict-clean). `validate` no stage-staleness (relations 31-vs-31), quality 52/100 ADEQUATE.
- **Honest residuals:** register bit-fields + field-resets not lowered (UNLOCATED); 1 `isf_temporal_unrepresentable` (residualized, not emitted); 0 message-fields.
- **Cluster conclusion:** all 3 CoreSight SoC-600 versions (0701/0200/0100) are UNIFORMLY the consolidation+transaction refresh class — stale evidence had `.10c` registers but predated the `.1a`/`.1b` consolidation gates + the section-heading transaction recognizer, so re-ingest delivers a real KG cleanup (actor/relation/interface consolidation) + a recognized transaction, not a marquee table jump.
- **Gates:** no code change → WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal by construction. Durable trace: `docs/tasks/CORPUS-COVERAGE.md` `.2` log (#25 row + changelog). Coverage: 25 of 57.

## CORPUS-COVERAGE.2 (`2026-06-23`) — re-ingest #24 CoreSight SoC-600 0200 TRM (genuine consolidation+transaction refresh)
- **Slice:** fresh-session PNT, register/TRM phase. Doc: CoreSight SoC-600 0200 TRM (`100806_0200_00_2017_12_08_coresight_soc_600_technical_reference_manual`, 761pp), via the git-ignored `.cache/local-references/chipdoc` symlink. Second of the SoC-600 family (after #23 = 0701; 0100 pending).
- **Method:** `DOCLING_DEVICE=cpu specforge ingest` → `evidence`→`semantic`→`intent`→`adapt --target isf` on the current release binary (no rebuild). Ingest ~5 min (761 page artifacts / 1302 visual / 0 residuals / high). RAM 74–79% free, `.4a` guard armed, Ollama idle ([[feedback_ram_ceiling_monitor]]).
- **BEFORE → AFTER (GENUINE refresh, same #23/#17 class):** register_records 631 held (exact — no fresh-Docling table variance this time, unlike #23's 833→828); **actor_signal_relations 47→35 / actors 46→37 / interfaces 13→6** (the current `.1a`/`.1b`/`.1b.iv` consolidation gates fold fragment/phantom actors the older evidence carried); **transactions 0→1** (section-heading recognizer now fires); conditional_rules 51 held; intent constraints 877→861; 0 message-fields (honest). Stale evidence sat at an intermediate binary (had `.10c` registers, predated consolidation + transaction recognizer).
- **Verification (real tools):** emitted `dp.isf` (631 storage(reset) / 9 enums / 3 rules / 4 signals) → `subs/fsmgen/bin/fsmgen --strict --check --json` from the fsmgen root = **success: true / 0 diagnostics** (strict-clean). `validate <intent-ir>` = no `stage_staleness` (relations 35-vs-35), quality 52/100 ADEQUATE.
- **Honest residuals (no fabrication):** register bit-fields + field-resets not lowered (UNLOCATED, the located-fields-only rule); 1 `isf_temporal_unrepresentable` (a temporal_signal_constraint FSMGen can't represent — residualized, not emitted); 0 message-fields.
- **Confirms #23's phase finding:** the CoreSight SoC-600 family (0701, 0200) is uniformly the consolidation+transaction refresh class — its stale evidence predated those two gates, so re-ingest delivers a real KG cleanup + a recognized transaction, not a marquee table jump.
- **Gates:** no code change → WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal by construction. Durable trace: `docs/tasks/CORPUS-COVERAGE.md` `.2` log (#24 row + changelog). Coverage: 24 of 57.

## CORPUS-COVERAGE.2 (`2026-06-23`) — re-ingest #23 CoreSight SoC-600 0701 TRM (genuine consolidation+transaction refresh)
- **Slice:** fresh-session PNT, register/TRM phase. Doc: CoreSight SoC-600 0701 TRM (`100806_0701_17_2025_06_30_coresight_soc_600_technical_reference_manual`, 842pp), via the git-ignored `.cache/local-references/chipdoc` symlink.
- **Method:** `DOCLING_DEVICE=cpu specforge ingest <pdf>` → `evidence` → `semantic` → `intent` → `adapt --target isf` on the current release binary (no rebuild). Ingest ~9 min (842 page artifacts / 1935 visual / 0 residuals / high confidence). RAM monitored 71–80% free throughout, `.4a` guard armed, Ollama idle, swap stable ([[feedback_ram_ceiling_monitor]]). (A transient outage of the opus-4-8[1m] Bash safety-classifier briefly blocked the cascade after the re-ingest completed; re-ran cleanly on recovery — the deterministic cascade is fully re-runnable, generated/ is git-ignored, so no risk.)
- **BEFORE → AFTER (a GENUINE refresh, NOT byte-identical like #22):** evidence register_records 833→828 (fresh-Docling table-boundary variance, minor); **actor_signal_relations 64→41 / actors 60→47 / interfaces 20→8** (the current `.1a`/`.1b`/`.1b.iv` agent-identity consolidation gates — which POSTDATE this doc's stale evidence — fold the fragment/phantom actors the older evidence carried, like #17 Avalon); **transactions 0→2** (the section-heading transaction recognizer, also newer than the stale evidence, now fires); conditional_rules 74 held; intent constraints 1186→1155, behaviors 790→787; 0 message-fields (no packet/structure tables — honest). So this doc's stale evidence sat at an INTERMEDIATE binary (already had the `.10c` registers, but predated the `.1a`/`.1b` consolidation + transaction recognizer).
- **Verification (real tools):** emitted `dp.isf` (828 storage(reset) / 19 enums / 4 rules / 5 signals) → `subs/fsmgen/bin/fsmgen --strict --check --json` from the fsmgen root = **success: true / 0 diagnostics** (strict-clean). Notably the 3 `isf_rule_conflict` adapter residuals on signal `ATB` (the ISF-RULE-CONFLICT-RESIDUAL family) RESIDUALIZE rather than emit conflicting drives, so unlike LPI #7 the `.isf` stays strict-valid. `specforge validate <intent-ir>` = no `stage_staleness` (semantic+intent relations both 41 → non-zero, silent), quality 53/100 ADEQUATE.
- **Honest residuals (no fabrication):** 791 bit-fields + 164 field-resets not lowered (UNLOCATED, the DOC-INTENT-TAXONOMY.4a.ii located-fields-only rule); the 3 `ATB` rule-conflicts (Lever C family); 0 message-fields.
- **Phase refinement (vs #22's claim):** "already current-binary-equivalent" is NOT uniform — a doc's stale evidence sits at whatever binary last rebuilt it. #22 Cortex-A76 was fully current (byte-identical); #23 CoreSight SoC-600 predated two later gates and got a real KG refresh (the #17 Avalon class). So the remaining tail mixes pure confirmation (#21/#22) and consolidation/recognition refreshes (#17/#23) + bundle restoration; marquee table-family jumps are now the exception.
- **Gates:** no code change → WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal by construction. Durable trace: `docs/tasks/CORPUS-COVERAGE.md` `.2` log (#23 row + changelog). Coverage: 23 of 57.

## CORPUS-COVERAGE.2 (`2026-06-23`) — re-ingest #22 Cortex-A76 TRM (already current-binary-equivalent; bundle restored)
- **Slice:** fresh-session PNT, register/CPU-core-TRM phase of the `.2` re-ingest batch. Doc: Cortex-A76 TRM (`100798_0401_00_2020_07_31_cortex_a76_technical_reference_manual`, 620pp), reached via the git-ignored `.cache/local-references/chipdoc` symlink (host-local path never tracked, COMMIT.md path policy).
- **Method:** `DOCLING_DEVICE=cpu specforge ingest <pdf>` ([[project_docling_mps_cpu]]) → `evidence` → `semantic` → `intent` → `adapt --target isf`, all on the current release binary (no rebuild; binary mtime newer than all sources). Ollama idle, built-in `.4a` RAM guard armed (abort ≥85% used); RAM monitored 73–83% free throughout, swap stable ([[feedback_ram_ceiling_monitor]]). Ingest ~3.5 min (620 page artifacts / 476 visual / 0 residuals / automation_confidence high; the `.3` no-page-image-on-disk optimization keeps disk O(assets)).
- **BEFORE (stale evidence + prior intent) vs AFTER (this re-ingest):** byte-identical on the deterministic surfaces — evidence register_records 42, conditional_rules 26, message_field_records 0, interfaces 0, actor_signal_relations 0, transaction_anchors 0; intent actors 9 / interfaces 7 / behaviors 707 / constraints 393 / relations 0. So the prior evidence was already rebuilt from the retained `source_ir.json` with a binary that carried the `.10c`/`.10g` register families (the #21 RISC-V IOMMU pattern). The substantive deliverable is the **restored `normalized/` bundle** (was missing) + current-binary confirmation + the honest data point.
- **Verification (real tools):** emitted `agent.isf` (48,099 bytes; 189 signals / 42 storage(reset) / 13 enums / 0 txns / 0 rules) → `subs/fsmgen/bin/fsmgen --strict --check --json` run from the fsmgen root = **success: true / 0 diagnostics** (strict-clean). `specforge validate <intent-ir>` = no `stage_staleness` finding (semantic+intent relations both 0 → 0-vs-0 honest absence, the `.1` detector correctly silent), quality 24/100 INCOMPLETE (honest for a register/behavior CPU-core TRM lacking wire direction/width/clock/reset grounding).
- **Honest residuals (no fabrication):** 0 evidence relations/interfaces (CPU-core TRM — register/behavioral intent, `KG-ISF-COMPLETENESS.3`); 0 message-fields (no packet/structure tables); 42 registers' bit-fields largely UNLOCATED → adapter residuals `isf_register_fields_not_lowered` (469 bit-fields) + `isf_storage_reset_not_lowered` (17 field-resets), the DOC-INTENT-TAXONOMY.4a.ii located-fields-only rule.
- **Phase correction:** a one-pass survey of the 35 remaining normalized-missing docs (read-only over persisted `evidence_ir.json`) found the register-heavy candidates the frontier had flagged "genuinely pre-`.10`-stale" (CoreSight SoC-600 ×3 = 597/631/833 regs, AMD-IOMMU 217 msg-fields, VT-d 103 regs) are ALSO already current-binary-equivalent → the marquee-jump phase is effectively over; remaining slices are the #21/#22 confirmation+restoration class.
- **Gates:** no code change → WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal by construction (the 4 gold docs are not re-ingested; the binary is unchanged). Generated tree is git-ignored → durable trace is `docs/tasks/CORPUS-COVERAGE.md` `.2` log (#22 row + changelog). Coverage: 22 of 57.

## DOC-INTENT-TAXONOMY.4d.i (`2026-06-23`) — cat-4 RISC-V CSR bit-position recovery measured non-viable (read-only, docs-only)

The `.4d.i` leaf was queued as "the genuine buildable cat-4 lever": a deterministic parser for RISC-V CSR
bit-layout tables → `bits_high`/`bits_low`, auto-lowering via `.4a.ii`. Before writing a line of that
regression-sensitive code (it would touch the shared register path 24 docs / 6,570 emitted fields depend on),
the premise was tested against the real source and the human-reviewed bit gold. It does not hold:

- **Modality.** 53 of 56 RISC-V Debug register-with-address headings carry the bit layout as an `![Image]`
  diagram, not a text table — including the cleanest gold register `dmcontrol` (`![Image]` only). The bit
  positions genuinely live in a non-text modality. `EXTRACTION-GAP-FIX.4` already built the right tool for
  that: the VLM reader `ir/register_bits.rs` + `recover-register-bits` (reads name/order/width off the diagram
  image, reconstructs positions by MSB→LSB tiling, gated to never fabricate).
- **The few flattened tables lie.** Only 7/56 diagrams were flattened to text. `dmstatus`'s explicit high-bit
  row is off by ~8 vs gold (`ndmresetpending` at 16, gold 24), it drops a 7-field middle band, and it mixes
  doubled cells + two stacked half-rows requiring contradictory decode rules. `tdata1`'s positions are symbolic
  XLEN-relative (`XLEN-1`, `XLEN-5`) — RISC-V CSRs are XLEN-parameterized, so the bits are not concrete.
- **Gate hole.** The tiling gates validate width-sum + name-multiset but **not field order**. The VLM reads in
  visual order so that is safe; a row-jumbled Docling table could present a wrong order whose widths still sum
  to 32 and whose names still match — passing both gates while emitting wrong bits. A deterministic-table reader
  is therefore *strictly more dangerous* than the VLM front-end.

Conclusion: do not build the deterministic parser (it would fabricate or recover ~0). Honest residual; the
genuine lever is a sharper VLM read for the existing gated path (stronger/cloud model, upscaling, voting, tighter
prompt), owned outside the `.4` ISF-lowering program. RISC-V AIA (the second sub-lever) is blocked on
RAM/Docling-gated re-ingest (`CORPUS-COVERAGE`) and its CSR intent is prose, not register tables. No FSMGen FR
(ISF already expresses register fields via `.4a.ii`). Reproducer: `scripts/measure_cat4_csr_bit_recovery.py`.
Packet: `docs/research/cat4-csr-bit-position-recovery-measurement.md`. The lasting value of this leaf is the
*avoided* regression — a measured "do not build this" on the shared register path.

## DOC-INTENT-TAXONOMY.4c.i (`2026-06-23`) — cat-3 topology-capture recall measurement (read-only, docs-only)

**Context.** `.4c` decided cat-3's distinctive intent (component topology / connectivity / clock-reset distribution) is
*captured* as a typed surface (`signal_connectivity`, `infrastructure_signals`) but ISF has no declarative static-topology
construct, and filed no FSMGen FR yet — deferring the construct decision (verified FR vs honest non-target) to a
capture-recall measurement, because the capture looked sparse/noisy and ISF is a per-actor format.

**Method.** Read-only structural gauge over the persisted IntentIR corpus (tracked reproducer
`scripts/measure_cat3_topology_recall.py`). For the full 15-doc cat-3 set — and the 4 cat-1 wire gold docs as a reference
baseline that proves the same surface is *capable* of dense, clean topology — computed: `signal_connectivity` edges per
actor (density), the fraction of edges with both a producer AND a consumer (a half edge is an unusable stub), the
fraction of clean (non-`None`/non-`\`-escaped) endpoint names, and `infrastructure_signals` fan-out / source-resolution.
The cat-3 set is the `.1`-census reconstruction enumerated in the reproducer (closing the gap that `.1` never persisted
the per-doc labels). ADR-0006: every metric is a structural count; the only per-document input is the one-off cat-3/cat-1
measurement labeling (a label like `.1`, not runtime code).

**Measured.** Cat-3 (15 docs): 380 actors / 135 `signal_connectivity` edges = **0.355 edges/actor**; **33 (24%)**
both-endpoint; **457/480 (95%)** clean endpoints; 10 `infrastructure_signals`, 6 with a fan-out list, **0** with a
resolved source. Cat-1 wire baseline (4 docs): 65 actors / 267 edges = **4.108 edges/actor**; **227 (85%)** both-endpoint;
100% clean; 8/8 infra with distribution, 1 with source. Robust to dropping the 4 borderline docs (cat-3 core: 0.395
edges/actor, 20% both-endpoint, 0 resolved source).

**Root cause.** The topology surface is *capable* of dense, fully-connected, rooted topology — it achieves exactly that on
wire docs, where the protocol's small signal graph IS the topology and is fully declared. On cat-3 platform TRMs the same
surface is ~12× too sparse, three-quarters half-connected, and the clock/reset tree has no captured root, because the
extractor recovers only a sliver of the multi-component interconnect from TRM integration prose. The bottleneck is
upstream **capture-recall** (EvidenceIR/SemanticIR connectivity + infrastructure extraction), NOT the missing ISF
static-topology construct `.4c` confirmed.

**Verdict.** Capture-recall-gated, not abstraction-gated → **no FSMGen FR** (premature on an unfaithful capture —
`feedback_verify_fsmgen_before_fr`); cat-3 topology stays an **honest residual**; the buildable lever (if pursued) is
upstream extraction-recall owned OUTSIDE the `.4` ISF-lowering program (mirrors `.4d.i` + the cat-2 structure-recall
frontier), recorded as a cross-reference, not a `.4` gap. Even with faithful capture, lowering would also need a
multi-actor ISF emit + a construct FSMGen's behavioral ATL frontier does not subsume — resolved WITH FSMGen only after
capture clears this bar. Refined `.4c`'s "sparse + noisy" to **sparse + half-connected + rootless-infra with minor name
noise**.

**Gates.** No Rust code / no canonical mutation → wire golds / `kg-bench` / emitted `.isf` byte-identical by construction
(WIRE-BASED-100 orthogonal). `scripts/check_doctrines.sh` green; `mdbook build` green; knowledge-map derive-and-diff in
sync (118 → 119 facts). Report `docs/research/cat3-topology-capture-recall-measurement.md`; KM
`docs/knowledge/cat3-topology-capture-recall.md`.

## DOC-INTENT-TAXONOMY.4e (`2026-06-23`) — conditional-rule lowering triage (`.2` Result 3, read-only, docs-only)

**Context.** `.2` Result 3 flagged that the `constraints + temporal + conditional → (rule)` lowering ratio is low (cat 1
~41% / cat 2 ~45% / cat 3 ~11% / cat 4 ~22%), dominated by `conditional_rules`, and deferred the honest-vs-lever verdict
to a dedicated `.4+` triage. This leaf does that triage so the shortfall is classified per item, never assumed.

**Method (reproducible, read-only).** Over 9 representative docs (cat-1 AHB/APB/AXI, cat-2 NVMe/RISC-V IOMMU, cat-3
GIC-600/CoreSight SoC-600, cat-4 RISC-V Debug/AIA), classified every `conditional_rules` entry against the document's
declared-signal inventory (union of `interfaces[].signals`, `interfaces[].signal_records[].signal_name`,
`actor_ports[].signal_name`) and the consequent's quality (no signal / undeclared / placeholder / bare-modal /
concrete-value cue).

**Measured (603 conditional_rules, 9 docs).** A no-consequent prose 392 (65%) / B undeclared signal 14 (2%) / C
declared+placeholder 33 (6%) / D declared+non-placeholder action 164 (27%). The D bucket's `consequent_action`
distribution: `shall` 45, `must` 35, `shall not` 14, `shall be cleared` 10, `must not` 7, `must be 4` 2, `shall be 1` 2,
… — **161/164 bare deontic modals, only 3/603 with a concrete value/level cue**.

**Decision.** A conditional lowers to an ISF `(rule)` only when it names a declared signal AND a concrete obligation. A/B/C
(73%) fail that by construction; the D bucket fails because the captured `consequent_action` is a deontic modal, not the
concrete value the document states only in prose — rendering it would fabricate the obligation. So the conditional-rule
shortfall is **honest residual, not an ISF-completeness gap**: no ISF lever, no FSMGen FR (the adapter already lowers the
516 cleanly-grounded conditional obligations corpus-wide). The only improvement path is **upstream extraction quality**
(recover the concrete obligation from the modal conditionals' `source_text`), owned by the extraction-quality program,
distinct from `.4` and lower-leverage than register/structure/topology — a cross-reference, NOT a `.4` gap
(`feedback_scoring_rigor`). Closes `.2` Result 3; the `.2` per-category measurement phase is complete (remaining `.4` work
is CODE: `.4d.i`, `.4c.i`, `.4b`).

**Gates.** No Rust code / no canonical mutation → wire golds / `kg-bench` / emitted `.isf` byte-identical by construction.
`scripts/check_doctrines.sh` green; `mdbook build` green; knowledge-map derive-and-diff in sync (117→118 facts). Report
`docs/research/conditional-rule-lowering-triage.md`; KM `docs/knowledge/conditional-rule-lowering-triage.md`.

## DOC-INTENT-TAXONOMY.4c (`2026-06-23`) — cat-3 platform/system-IP topology lowering decision packet (read-only, docs-only)

**Context.** `.2` scored category-3 (platform / system-IP topology & integration) as **PARTIAL** ("infrastructure signals
+ actor ports lower; topology stays at the hint level"). `.4a.ii` then shipped register bit-field lowering, and cat-3
carries the corpus's largest register-field volume. This leaf decides whether cat-3's *distinctive* intent — topology /
connectivity / clock-reset distribution — maps onto an existing ISF construct or needs a new one, before any cat-3 code
or FR.

**Method (reproducible, read-only).** Cat-3 = 15 docs (`.1` census). Profiled 3 representative docs off persisted IR:
CoreSight SoC-600 (`100806_0701_17…`), GIC-600 (`100336_0106_00…`), CoreSight Base System Arch (`den0068…`). Re-verified
the FSMGen ISF on pin `d327129b7` for cross-component composition / topology / connectivity.

**Measured.**
- Cat-3 docs **do** carry a typed topology surface — refining `.2`'s "hint-level" to **captured-but-sparse-and-unlowered**:
  `signal_connectivity` (producer→consumer graph; GIC-600 66 edges e.g. `DATA` from `MISC ignores` → `cache several`/
  `pmu_int`; SoC-600 6) and `infrastructure_signals` (clock/reset distribution: `infrastructure_topology` /
  `distributed_to_actor_ids`; GIC-600 2).
- But the capture is **sparse and noisy**: SoC-600 has 6 connectivity edges across 60 actors; canonical actor names come
  back `None`; connectivity endpoints carry extraction noise (escaped `pmu\_int`, generic `MISC ignores`).
- FSMGen ISF (`d327129b7`) has **no declarative static-topology construct**: composition is transaction-level only
  (`(do child)`, `13f-composition.md`); the backlog's multi-actor "ATL" frontier wires children *generated from
  transaction composition* (behavioral orchestration), not a declarative IP-interconnect netlist. The SpecForge emit is
  single-initiator-actor (`KG-ISF-COMPLETENESS.2a.ii`), so a 60-component doc emits one actor — cross-component topology is
  structurally absent by design.

**Decision.** (1) Cat-3's register half already lowers (same road as cat-2). (2) Topology is captured but unlowerable
today; ISF has no static-topology construct and FSMGen's ATL frontier is not a home. (3) **No FSMGen FR is filed yet** —
premature because the capture is too sparse/noisy to lower faithfully AND ISF is a per-actor format where static topology
may deliberately be the integrator's concern above per-module synthesis (resolve WITH FSMGen —
`feedback_verify_fsmgen_before_fr`). Topology stays an honest residual; the buildable next step is the measurement `.4c.i`
(topology-capture recall across all 15 cat-3 docs), after which the construct decision (verified FR vs honest-non-target)
becomes real.

**Gates.** No Rust code / no canonical mutation → wire golds / `kg-bench` / emitted `.isf` byte-identical by construction.
`scripts/check_doctrines.sh` green; `mdbook build` green; knowledge-map derive-and-diff in sync (116→117 facts). Report
`docs/research/cat3-topology-isf-lowering-decision.md`; KM `docs/knowledge/cat3-topology-isf-lowering-decision.md`.

## DOC-INTENT-TAXONOMY.4d (`2026-06-23`) — cat-4 ISA/CSR lowering decision packet (read-only, docs-only)

**Context.** `.2` scored category-4 (CPU ISA / privileged architecture) as **THIN**. `.4a.ii` then shipped register
**bit-field** lowering. This leaf resolves the standing cat-4 Open Question — *do CSR-fields / instructions / privilege
intent map onto the existing register/storage abstraction, or need a new ISF construct?* — from measured evidence, before
any cat-4 code.

**Method (reproducible, read-only).** The corpus has exactly 2 cat-4 docs: `1_0_risc_v_debug_specification` and
`1_0_2025_03_12_risc_v_advanced_interrupt_architecture` (the RISC-V IOMMU doc is cat-2). Profiled the persisted
`generated/<stage>/<key>/<stage>.json` surfaces, counted register_records + located fields (the `.4a.ii` gate:
`bits_high`/`bits_low`, or `bits_low`+`bit_width`), and re-verified the FSMGen ISF on pin `d327129b7`.

**Measured.**
- **RISC-V Debug** — `register_records=44` (the Debug-Module CSR block, captured by the `field_table` strategy), all 44
  with a field list, **179 fields total but 0 located**: every field carries `field_name` + `access_type` + `reset_value`
  + `description` but none carries a bit position, and the register `name`/`offset_address`/`size_bits` are `None`. The
  bit-layout column was never parsed into the field record, so under the `.4a.ii` located-fields gate **0 of 179 fields
  reach `.isf`**. (`conditional_rules=62`, `temporal_invariants=216`, `behaviors=784`, `transactions=3`.)
- **RISC-V AIA** — `register_records=0` despite being CSR-dense (IMSIC/APLIC). Its register/interrupt-controller intent
  is in prose (`conditional_rules=39`: *"each RISC-V hart must have an IMSIC"*, *"the system will normally still contain
  an APLIC"*; `behaviors=493`). `interfaces=211` is empty prose noise (signal records all `None`; 0 constraints / 0
  relations). No current register strategy matches RISC-V's CSR layout — the `.10g` `<NAME>, bits [hi:lo]` section-heading
  family fires only on ARM `ihiXXXX` arch specs.
- **FSMGen ISF (`d327129b7`)** — `(storage (var … (fields …)))` is titled the "register map / CSR" construct
  (`13a-actor-interface.md:419`/`:468`); no instruction/privilege/exception/memory-ordering construct exists; its listed
  future ISF directions are storage/structure abstractions (`:511`), not ISA semantics.

**Decision (cat-4 splits into three).** (1) **CSR / register intent reuses the existing register/storage abstraction** —
no new ISF construct, no FSMGen FR (CSRs are structurally registers). (2) **The cat-4 register gap is EXTRACTION RECALL**
— spun out as `.4d.i` (locate RISC-V Debug fields + capture AIA CSR blocks → auto-lower via `.4a.ii`, no emitter change;
must key off structural RISC-V CSR-layout shape, ADR 0006). (3) **Instruction / privilege / exception / memory-ordering →
honest non-target** (software-visible ISA semantics, not synthesizable hardware intent; no FR per
`feedback_verify_fsmgen_before_fr` / `feedback_isf_no_hacks`; conditional-future only if FSMGen's SV/UVM path scopes
ISA-model verification).

**Gates.** No Rust code / no canonical mutation → wire golds / `kg-bench` / emitted `.isf` byte-identical by construction.
`scripts/check_doctrines.sh` green; `mdbook build` green; knowledge-map derive-and-diff in sync (115→116 facts). Report
`docs/research/cat4-isa-csr-lowering-decision.md`; KM `docs/knowledge/cat4-isa-csr-lowering-decision.md`.

## DOC-INTENT-TAXONOMY.4a.ii (`2026-06-22`) — emit register bit-fields into ISF field-structured storage

**Context.** Gap A of the per-category ISF-completeness scorecard: register **bit-fields** reached the emitted `.isf`
zero times (12,638 captured fields across 35 register-bearing docs) — the emitter built `IsfStorageVar { name, width,
reset }` and rendered an opaque `(var NAME (width N) [(reset V)])`, discarding every field's bit range, access, reset,
and enum. `.4a` proved this was not a SpecForge carry gap (the field map reaches `IntentIr.register_records` intact) but
a missing ISF abstraction, filed it as a verified FSMGen FR, and FSMGen shipped the declarative field-structured-storage
construct (pin `d327129b7`). This slice does the lowering.

**Measure first (TOOLBOX discipline).** Before coding, a read-only sweep of `generated/intent_ir/*` established the
surface: 8,708 of 12,638 fields carry a concrete bit range (68%); 130 registers have a sanitized-name collision
(dominated by reserved gaps `res0`×115 / `reserved`×53, plus mis-extraction dups like `size`×9); 36 have a located-field
overlap; access is dominated by mappable tokens (RO/RW/WO/WARL/R/WPRI/RW1C ≈ 88%); enums are rare (6 members). The
collision composition was the decisive measurement: it justified per-field admission with a structural collision-drop
(not all-or-nothing), since "gaps are allowed" by FSMGen and the collisions are overwhelmingly reserved gaps.

**Verify the contract empirically.** A hand-authored `(fields …)` block was run through the real pinned `fsmgen
--strict --check --json` (`success=true`), `--emit-schedule-json` (`inferred_storage[].fields[]` round-trips
name/msb/lsb/width/access/reset/enum), and a deliberately-bad out-of-width field (`fsmgen` fails closed:
`field 'big' bits [9:0] exceed parent width 8`). So the emitter is built to exactly what FSMGen accepts.

**Implementation (`crates/specforge/src/ir/isf_ir.rs`).** New `IsfStorageField` struct + `IsfStorageVar.fields`; the
storage render emits the nested `(fields (field FNAME (bits HI LO) [(access …)] [(reset V)] [(enum (M V)…)]) …)` block
when present and the opaque single-line form (byte-identical to before) when not. Field derivation is the pure
`register_storage_fields(r, var_width, parent_reset)`: located fields only (mirrors `classify_register_reset`'s u64
tiling bound: `hi`/width < 64, in parent width); drop the whole group of any sanitized-name collision; require the
survivors non-overlapping else fail the register's block closed; `normalize_field_access` maps to FSMGen's 10-token set
plus unambiguous synonyms (`r→ro`, `w→wo`, `r/w→rw`, `rw1c→w1c`), omitting the unmapped; a field `(reset)` only when
`parent_reset` is `Some` (FSMGen requires an explicit parent reset for a field reset), emitted as that value's own bit
slice so it matches by construction; `(enum)` keeps width-fitting numeric members (`meaning`→sanitized member, deduped).
The unlowered count becomes the `isf_register_fields_not_lowered` adapter residual (wired in `ir/adapters.rs`; folds in
`.4a.i`). A test-only `run_fsmgen_schedule_json` helper (`ir/mod.rs`) exposes the round-trip oracle.

**Key correctness subtlety — field reset matches the parent slice by construction.** `classify_register_reset` returns
`Emit(V)` only when every field is located, has a parseable reset, and the fields don't overlap, tiling
`V = OR(value_i << lo_i)`. Computing each field's `(reset)` as `(V >> lo) & ((1<<width)-1)` therefore equals that
field's own value (disjoint bits) and satisfies FSMGen's "field reset must match the parent slice" rule even when some
fields (reserved gaps) are dropped from the emitted block — their bits in `V` are disjoint and unreferenced.

**Measured live (real emitter).** 6,570 register bit-fields now reach `.isf` across 2,531 registers in 24 docs (was 0):
CoreSight SoC-600 1,166/1,118/974, GIC arch `ihi0069` 424, CCIX 380, SMMU `ihi0070` 332, CHI-C2C 276, … The real count
is lower than the read-only replica's 7,905 because the emitter dedups duplicate-named *registers* (`seen_storage_names`,
required so FSMGen never sees duplicate storage vars) — caught precisely because I verified against the tool, not the
replica.

**No regression.** The 4 WIRE-BASED-100 golds emit 0 fields → emitted `.isf` byte-identical (old-vs-new `adapt` diff
empty); metadata-only / schedule-safe; RISC-V IOMMU (122) / GIC (424) / CoreSight SoC-600 (974) keep `fsmgen --strict`
`success / 0 diagnostics` before AND after (0 new); `kg-bench 156/156`; `cargo test 1702/0` (warning-deny, +6 tests);
`cargo fmt`/`clippy -D warnings` clean; `run_ci.sh` green.

## FSMGEN-REFRESH-INTEGRATE-5.1 (`2026-06-22`) — FSMGen SHIPPED declarative storage fields; un-gate DOC-INTENT-TAXONOMY.4a.ii

**Context.** One refresh cycle after FSMGen *accepted* the field-structured-storage FR (cycle 4), the owner reported
FSMGen had pushed again and pointed me at the new declarative-storage-fields spec/book sections. FSMGen ran its promised
`ISF-FIELD-STRUCTURED-STORAGE-FRONTIER.1` contract audit and `.2` **shipped** the construct — exactly the abstraction my
`.4a` FR requested.

**What shipped, and why it's clean to adopt.** `(storage (var NAME (width N) [(reset V)] (fields (field FNAME
(bits HI LO) [(access …)] [(reset V)] [(enum …)]) …)))`. The decisive property: it is **metadata-only and
schedule-safe** — FSMGen states (and the book example shows) the scheduled `.fsm` is *byte-identical* with vs without
`(fields …)`, and the HDL reset still comes only from the parent `(reset V)`. So SpecForge can emit the field map
*behind* the existing `(var …)` with zero risk to the wire golds. The field map maps 1:1 onto `RegisterFieldRecord`
(name/bits/access/reset/enum), the report exposes it as `inferred_storage[].fields[]` (our verification oracle), and
FSMGen's fail-closed rules (overlap/out-of-width/unsupported-access/field-reset-must-match-parent-slice/enum-fits)
line up exactly with the gating `classify_register_reset` already does — so `.4a.ii` can reuse that tiling gate.

**Verification (didn't trust the commit subjects).** Checked out `d327129b7`, ran the 6
`*_passes_fsmgen_strict_validation` canaries (PASS — emitted `.isf` still strict-valid), then full `run_ci.sh` + `kg-bench
156/156`. No SpecForge Rust code changed, so the emitted `.isf` is byte-identical and WIRE-BASED-100 is orthogonal; the
+2 commits are purely the additive field-storage feature. Ownership: `FSMGEN-REFRESH-INTEGRATE-5` created before the
gitlink bump.

**Consequence for the Gap-A program.** `.4a.ii` (the register bit-field emit) is **un-gated and is now the
highest-leverage buildable lever** — it turns the largest measured ISF intent-loss (12,638 fields / 32 docs at zero)
into faithful synthesis. It **supersedes** `.4a.i` (the honest residual): with a real lowering target the faithful move
is to emit the fields, not just record they were dropped (`.4a.i` stays a cheap fallback). I captured the exact grammar,
the `RegisterFieldRecord`→ISF mapping, the tiling-gate reuse, and the `inferred_storage[].fields[]` verification oracle
in the `.4a.ii` node so the next (fresh) session can implement it directly. Gap B (`.4b`, packet/flit) stays gated —
FSMGen explicitly deferred packet/flit layouts.

## FSMGEN-REFRESH-INTEGRATE-4.1 (`2026-06-22`) — refresh the FSMGen pin + integrate the accepted field-structured-storage FR answer

**Context.** Right after `DOC-INTENT-TAXONOMY.4a` filed the field-structured-storage FR, the owner reported FSMGen had
answered and asked me to update the submodule and read `subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md`. This is the
fourth FSMGen refresh cycle.

**What FSMGen answered.** FSMGen **ACCEPTED** the FR — "yes, FSMGen accepts this as a real ISF representational gap and a
valid future direction" — and accepted the *exact* shape I proposed (a storage var with an optional declarative field
partition: per-field name, bit range, optional access/reset/enum/provenance; first version = checked metadata with real
fail-closed validation). Two things matter most: (1) it is **not shipped** — FSMGen's own next step is a readiness/contract
audit `ISF-FIELD-STRUCTURED-STORAGE-FRONTIER.1` before any parser/lowering code; and (2) FSMGen **independently
confirmed the no-hack stance** I reasoned to in `.4a` — do not use `set-field`/`when-field`/`extract`/`assemble`, fake
drives, or comments as a substitute, because "would fabricate behavior." It even spelled out the sanctioned near-term
SpecForge posture: keep recovered register/CSR + packet/flit field maps as IntentIR metadata/residuals, keep emitting
opaque storage, fabricate nothing. That is a direct endorsement of `.4a.i` (the adapter honest residual) and keeps
`.4a.ii` (the actual field-structured emit) gated on FSMGen shipping the construct.

**How I verified the bump (didn't trust the commit subjects).** I checked out `5ce0335c5` in the submodule working tree
and ran the contract canary — the 6 `*_passes_fsmgen_strict_validation` tests, which emit `.isf` and run the real
`subs/fsmgen/bin/fsmgen --strict --check` — they PASS, so every form SpecForge emits is still strict-valid on the new
binary. Then full `run_ci.sh` (1696 tests / 0 failed, warning-deny clippy/rustdoc, mdBook) + `kg-bench 156/156`. No
SpecForge Rust code changed, so the emitted `.isf` is byte-identical and WIRE-BASED-100 is orthogonal; the contract delta
across the 106 commits is none for SpecForge (the rest are FSMGen-internal IAL2 frontier work + a doctrine adoption).
Ownership: created `FSMGEN-REFRESH-INTEGRATE-4` before committing the gitlink bump (no change without a task tree).

**Lockstep.** FR marked RESOLVED in `docs/FSMGEN_FEEDBACK.md`; README pin updated; `DOC-INTENT-TAXONOMY.4a.ii`/`.4a.i`
notes carry FSMGen's acceptance; KM card `register-bit-field-isf-lowering-gap` updated with the answer.

## DOC-INTENT-TAXONOMY.4a (`2026-06-22`) — Gap A register bit-field ISF lowering: verified FSMGen FR (measurement/design)

**Context.** `.2` measured the two dominant ISF-completeness gaps; Gap A (register bit-fields: 12,638 fields / 32 docs
reaching `.isf` zero times) is the highest-leverage lever. The owner's standing doctrine forbids guessing the fix:
measure WHERE the loss is, and verify the downstream submodule empirically before claiming an ISF gap or filing an FR
(`[[feedback_verify_fsmgen_before_fr]]`), never hack the emitter (`[[feedback_isf_no_hacks]]`). So `.4a` is a
measurement/design leaf, docs-only.

**What the two probes found.** (1) The bit-field intent is *not* lost in SpecForge — `RegisterFieldRecord`
(`source.rs:414`) carries the full field map and IntentIR carries it unchanged (a direct
`semantic_ir.register_records.clone()` at `intent.rs:193`). The loss is a single emit-time discard: the per-register loop
at `isf_ir.rs:852` builds `IsfStorageVar { name, width, reset }`, reading `r.fields` only to compose a register-wide
reset, then renders the opaque `(var NAME (width N) [(reset V)])` (`:391`). So there is **no carry gap** to build — the
fields are already in IntentIR. (2) The reason they can't be emitted is upstream: on the pinned `subs/fsmgen`
(`030f8c273`) the ISF `(storage …)` grammar is opaque width-only (`(var)`/`(variable)`/`(bank)`), with **no construct to
declare named bit-fields**. The shipped `set-field`/`extract` operators are runtime read-modify-write on an opaque
register, not a static field-map declaration — emitting them to "represent" a documented layout would fabricate runtime
behaviour the spec never states. Field-structured storage isn't on the FSMGen backlog either.

**The decision, and why it's not a hack.** I considered three emitter-only paths and rejected all as fabrication or
loss: per-field `(var REG_FIELD …)` invents N separate storage units and loses the register grouping + absolute bit
positions; runtime `(extract REG as FIELD…)` fabricates behaviour; field-name comments aren't intent (FSMGen ignores
them). There is genuinely no faithful emitter-only path — Gap A is a real missing ISF abstraction. So I filed a verified
FSMGen FR (`docs/FSMGEN_FEEDBACK.md`, `2026-06-22`) for a declarative field-structured-storage construct
(`(var NAME (width N) (fields (field … (bits hi lo) (access ..) (reset ..) (enum ..))))`), framed to also serve the
Gap-B packet/structure layout family. The timing is right: the owner noted FSMGen is adding a verification-oriented
SV/UVM + VHDL path and anticipates new ISF abstractions (memory banks, single/dual-port memory) — a declarative
field-structured register is the same family.

**Follow-on (recorded, not built).** `.4a.i` = an explicit adapter honest residual `isf_register_fields_not_lowered`
(today the field drop is silent; only the *reset* drop is recorded) — a RAM-light CODE slice with no FSMGen dependency,
the natural next build. `.4a.ii` = the actual field-structured emit, gated on FSMGen shipping the construct. `.4b` = the
Gap-B `Evidence→Intent` carrier. Report `docs/research/register-bit-field-isf-lowering-design.md`; KM
`[[register-bit-field-isf-lowering-gap]]`. No code → golds/`kg-bench` orthogonal.

## DOC-INTENT-TAXONOMY.3c (`2026-06-22`) — recognizer fixtures + book + KM (the `.3` recognizer is complete)

**Context.** `.3b` implemented + wired the recognizer and locked its per-category logic with 13 in-file unit tests.
`.3c` adds the two things the pinned decomposition deferred: an end-to-end regression lock for the *validate surface*,
and the user-facing documentation (the book is the user's only window into the tool).

**The integration test, and what it taught me.** I first wrote the test expecting a markdown titled "# Widget User
Guide" to classify as `methodology-guide` (high) — but it came out `unresolved`. That was the recognizer being *correct*,
not a bug: the validate front-matter signal is `document_profile.title` + the early `document_sections` titles, and a
minimal one-heading markdown does not populate them the way a real PDF's front-matter does, so there was no decisive cue
and the recognizer honestly fell through to `unresolved` rather than guessing. I retargeted the test to lock that
honest-residual fall-through path end to end (metric + confidence + finding + the residual text). The per-category gold
cases are already locked at the unit level (where the census is controlled directly), so the integration test's job is to
lock the *wiring*, which it now does. Lesson reaffirmed: assert what the pipeline actually does, not what I assumed.

**ISA/PHY vocab calibration (precision-first).** Before shipping any front-matter vocabulary I probed all 78 docs'
front-matter: the ISA vocabulary (`instruction set` / `privileged architecture` / `isa` / …) matches **0** corpus docs —
so the 2 corpus ISA docs (RISC-V Debug, RISC-V AIA) honestly fall through to a register-or-platform / unresolved residual,
exactly as `.1` predicted (ISA has no structural signature). The PHY vocabulary recovers all 4 OpenCAPI PHY docs once
`"physical signaling"` was added (precision-checked: that phrase appears only in the 2 PHY signaling specs). I deliberately
did NOT widen further — the honest residual is better than a false positive.

**Book framing.** The new validation.md section leads with *why* (two same-shape documents can have different purposes, so
"complete" means different things) before the *what*, and is explicit about the two trust properties: high confidence only
where the evidence is unambiguous, and a register map never vetoes a real wire protocol. `document-categories.md` flips
from describing the recognizer as a "target" to "now reported by the CLI", keeping the conceptual taxonomy as the map.

## DOC-INTENT-TAXONOMY.3b (`2026-06-22`) — implement the 6-category document PURPOSE recognizer

**Context.** `.3a` pinned the recognizer design; `.3b` implements it. This is the FIRST Rust slice gated by the new
`TASK-ACCEPTANCE` doctrine, so the owning leaf carries the evidence-backed acceptance checklist.

**Why the `.3a` flit clause had to be refined (measure-before-code).** The `.3a` design proposed a cat-1 cue of "a
substantial `message_field_records` flit surface co-present with behavioral/relation shape". Before coding, I measured the
real per-document census off the persisted corpus and found that cue is falsified by its own data: NVMe (216 message
fields + 20 incidental constraints, 0 relations), AMD-IOMMU (98 relations + 217 fields), and GIC-600 (101 relations + 293
register fields) are register/memory-mapped or platform IP (cat 2/3) yet would satisfy a naive co-presence gate and be
mislabelled wire (cat 1). The honest-residual doctrine + `feedback_scoring_rigor` (measure per item) require the cue the
data actually supports.

**The discriminators that work (all measured).**
- **Flit ⇒ cat-1 only without a register map.** `message_field_records` are ambiguous: cat-1 flit/packet fields for
  CHI (106) and DTI (159), but cat-2 in-memory structures for NVMe (216) / AMD-IOMMU (217) / CCIX (92). The clean split
  in the corpus is `registers == 0`: CHI/DTI carry zero registers, every structure-IP doc carries some. So flit fields
  count as a wire cue, and as wire weight, only when there is no register map.
- **Wire-weight vs register/structure-weight dominance.** A register/field count must NOT veto a clean wire shape (AXI:
  71 registers but 348 relations + 50 constraints). The dominance test (`wire_weight = relations + constraints +
  flit_fields + fsm + frame + presence` vs `struct_weight = registers + register_fields + struct_fields`) rescues AXI
  (401 ≥ 229 → wire) while routing GIC-600 (110 < 326 → register-or-platform) and AMD-IOMMU (98 < 225) to the honest
  combined category, whose residual names the outweighed wire cue ("may be a register-heavy WIRE protocol").
- **Confidence is the signoff lever.** Only a clean wire shape and a front-matter guide self-declaration are HIGH
  confidence; cat 2↔3 (combined, `.1` proved counts can't split them), cat 4 (ISA, no structural signature), cat 5 (PHY),
  and the near-empty fall-through are all LOW + an explicit residual. Verified: across all 78 docs, every one of the 21
  wire + 8 guide HIGH-confidence calls is genuinely correct — zero high-confidence mislabels.

**Genericity (ADR 0006).** Every cue is a structural typed-surface count or generic front-matter doc-type vocabulary
(guide / spec / instruction-set / privileged-architecture / physical-layer / "physical signaling"). Precision-verified on
the corpus: the ISA vocabulary matches 0 docs (the 2 corpus ISA docs honestly fall through to a residual — `.1` predicted
this), and `"physical signaling"` matches ONLY the 2 PHY signaling specs. No chip/vendor/protocol-instance name list.

**Deferred to `.3c` (pinned).** The user-facing mdBook chapter (in `quality/validation.md` beside `document_class`, and
`document-categories.md` flipped from "target" to "now CLI-reported"), the KM fact card, gold/negative + honest-residual
fixtures, and empirical calibration/widening of the ISA/PHY front-matter vocabulary against the real front-matter strings.

## DOCTRINE-ENFORCEMENT-ADOPT.2 (`2026-06-22`) — closing the tree: docs + KM, book-method-doc

**Context.** Closing leaf of the doctrine-enforcement adoption. The book is the user-facing surface
(zero-drift doctrine), so the enforcement system — which is *how the project keeps its own promises* —
must be explained there, not only in the root standard.

**Engineering notes.**
- **Framed for a user, not a contributor.** The chapter answers the question a careful reader actually
  asks ("how do I know these promises are kept?"), then connects it to a concrete user payoff: when the
  project reports `kg-bench 156/156` or a wire gold at `1.000`, those are oracles CI re-runs, not numbers
  typed into a doc. Why-before-what per the book-doc style.
- **No dead links in the book.** The standard, driver, and `TOOLBOX.md` live at the repo root, not in the
  book tree — so they are referenced as plain code-spans, never as markdown links to a non-book path
  (which would render as dead links / trip a link check). `mdbook build` is green.
- **The KM card carries a real `reverify`.** The fact card's `reverify` re-runs `scripts/check_doctrines.sh`
  (expects "ALL 3 … PASS") and describes the TASK-ACCEPTANCE block, so a future agent confirms the fact
  by execution rather than trust — consistent with the architecture the card documents.
- **Tree closed cleanly.** All three leaves are scripts/docs only; no extraction or emitter Rust was
  touched across the whole tree, so WIRE-BASED-100 + `kg-bench` (156/156) are orthogonal by construction
  and were never at risk.

## DOCTRINE-ENFORCEMENT-ADOPT.1 (`2026-06-22`) — making the task-tree-ownership doctrine un-self-tickable, without false-blocking

**Context.** Decision 0003's "no code change without an owning task-tree leaf first" was prose the agent
was trusted to follow. `.1` turns it into a gate (`scripts/check_task_acceptance.sh`) of the EVIDENCE
archetype (`DOCTRINE_ENFORCEMENT.md` §3/§6). The owner also clarified that `TOOLBOX.md` must be
SpecForge's OWN tool catalog, not a copy of the reference project's.

**Engineering notes.**
- **Scope chosen to enforce the real doctrine, not annoy.** "Code change" = `crates/**/*.rs` +
  `crates/**/test_data/**` (behavioral Rust + gold fixtures). Shell/hook/docs/mdBook commits are EXEMPT —
  they carry their own gates, and gating them would false-block continuity work (including this very
  adoption commit, which is scripts+docs). This is why `.0` and `.1` commit cleanly through the new gate.
- **Signatures are SpecForge's real verification vocabulary.** The DIAGNOSIS signature matches the strings
  SpecForge's own tools emit (`validate` `evidence_*`/`semantic_*`/`intent_*` findings, `document_class`,
  `rationale:`, `adapt` `blocking_reason`, `kg-bench`, `*.rs:<line>`, a measured `N->M`); the NO-REGRESSION
  signature matches the project's oracle vocabulary (`kg-bench 156/156`, `WIRE-BASED-100`, `1.000`,
  `byte-identical`, `run_ci`, `cargo test|clippy|fmt`, `orthogonal`). So a genuine signoff-quality leaf
  passes and an empty/`trust me` checklist fails — verified with a `trust me` probe (TEST 5 blocked).
- **Earned, not ticked.** Presence (the hook) is leg 1; the cited oracles re-run in `run_ci.sh` / CI
  (leg 3) — a self-ticked-but-false NO-REGRESSION box dies when `kg-bench`/the golds are re-run. The hook
  honestly only proves the boxes are ticked + a matching signature is present.
- **Portability (bash 3.2).** The reference check used `mapfile` (bash 4+). macOS ships bash 3.2 as
  `/bin/bash`, and `#!/usr/bin/env bash` can resolve to it on a fresh clone / CI runner — so the check was
  written `mapfile`-free (`while IFS= read` over `git diff --cached`), `bash -n`-clean, version-independent.
- **Tested before wiring.** All 5 gate paths (exempt / block-no-leaf / pass / block-unticked /
  block-unbacked) were exercised with throwaway staged files and fully reverted before `TASK-ACCEPTANCE`
  was registered — so the gate's behavior was proven before it could block a real commit.

## DOCTRINE-ENFORCEMENT-ADOPT.0 (`2026-06-22`) — adopting the 4th portable architecture without breaking the existing gates

**Context.** The owner directed adopting `DOCTRINE_ENFORCEMENT.md` (the portable doctrine-enforcement standard).
SpecForge already enforced two doctrines (memory-architecture, knowledge-map) through a hand-rolled
`.githooks/pre-commit` + `scripts/run_ci.sh` stack. The adoption had to (a) land the unifying registry+driver, (b)
register the existing checks without weakening them, and (c) not break the live commit gate at any point.

**Engineering notes.**
- **Driver-first, test-before-wire.** `scripts/check_doctrines.sh` was written and run standalone (`bash
  scripts/check_doctrines.sh` → 2/2 PASS) BEFORE editing `.githooks/pre-commit`, so the on-disk commit gate was
  never pointed at an unverified or missing driver. The git hook runs from the working tree, so the driver had to
  exist + pass on disk at commit time — verified first.
- **No duplication — the checks stay the single source of truth.** The driver does not re-implement the
  memory-arch / knowledge-map invariants; it invokes the existing `check_*.sh` scripts and aggregates exit codes.
  Each script remains the one place its rule is defined (`DOCTRINE_ENFORCEMENT.md` §5).
- **Derived-artifact ordering preserved.** The knowledge map is derived; the pre-commit must regenerate + stage it
  BEFORE the driver's `check_knowledge_map.sh` validates sync. The new pre-commit keeps that exact ordering, then
  calls the driver (which validates, never regenerates) — so map drift stays structurally impossible.
- **Meta-check guards the registry.** A registered enforcer that is missing or non-executable fails the driver
  (not silently skipped), so a future one-line registry add that points at a not-yet-written script fails loudly —
  the reason `.1`'s `TASK-ACCEPTANCE` line is added only once `check_task_acceptance.sh` exists.
- **Pre-commit stays cheap; oracles stay in CI.** The driver runs only the structural + (soon) evidence checks
  locally; the heavy deterministic oracles (`kg-bench` 156/156, WIRE-BASED-100 golds, `cargo fmt/clippy/test/doc`)
  remain on the `run_ci.sh` / CI path — the strongest leg (`DOCTRINE_ENFORCEMENT.md` §4.7 / §6.1).

## DOC-INTENT-TAXONOMY.2 (`2026-06-22`) — per-category ISF-completeness gauge: how the measurement was made honest

**Context.** `.1` established the per-category denominator (36/7/15/2/4/14). `.2` had to turn the `.0` maturity
column (MATURE/PARTIAL/THIN/non-target) from estimate into measurement — "what fraction of each document's intent
reaches `.isf`" — without fabricating a score (`feedback_scoring_rigor`).

**Engineering notes — how the gauge avoids a gamed number.**
- **Per-surface ledger, not one blended %.** The intent surfaces are heterogeneous (signals, registers, fields,
  enums, rules, transactions, structures). A single weighted percentage is trivially gameable; eight separate
  present→lowered ratios are not. The gauge reports each surface's verdict (lowered / partial / true-gap /
  honest-residual / absent) and only then rolls up per category.
- **Numerator from the real artifacts.** "Lowered" is read straight off `adapter.json.isf` (`signal_count`,
  `storage_count`, `enum_count`, `rule_count`, `transaction_count`) — the emitter's own counts, not a re-derivation.
- **Two unmaterialized adapters handled read-only.** `1_0_risc_v_debug_specification` (cat 4) and
  `den0068_…coresight_base_system_architecture` (cat 3) had no `adapter.json`; `adapt … --dry-run` prints the
  computed adapter JSON without writing (verified the adapter dirs stayed absent), so the gauge stays read-only.
- **Signals are deliberately NOT scored as a ratio.** First-cut accounting flagged "signals present < lowered"
  for cat 3/5/6, which looked impossible. Root cause: the `.isf` signal set is built from a *different basis* than
  the IntentIR `interfaces` array (union of interface + actor-port graph + relation/constraint refs), so it can be
  larger (Cortex-A76 TRM 7→189) or smaller (AXI `ihi0022_h_c` 1026→242). Forcing a present/lowered ratio on signals
  would be a meaningless number, so the gauge reports the signal *path* as mature and quarantines the real
  signal-side concern (cat-6 over-extraction) as a separate precision finding.
- **The two true gaps are root-caused, not just counted.** Register fields: the `.isf` storage block is
  `(var register_table_XXXX (width N))` with no field substructure — registers lower as opaque width-only vars.
  Structures: `intent_ir.json` has no `message_field_records` key at all (`has_msgfld_key=false`) — the surface
  exists at EvidenceIR and is simply not carried into IntentIR. Both are concrete, reproducible, and point at the
  same FSMGen ISF-abstraction need rather than a SpecForge bug to silently patch.
- **Label robustness.** Category labels are one-off measurement labels reconstructing the un-persisted `.1` ground
  truth (and now persisted in `scripts/measure_isf_completeness.py`, fixing a `.1` continuity gap); the distribution
  checksums to 36/7/15/2/4/14. The conclusions (Gap A, Gap B) are objective per-document facts, robust to the five
  borderline labels flagged in `.1`.

**Outcome.** Scorecard measured: cat 1 MATURE, cat 2/3 PARTIAL, cat 4 THIN, cat 5/6 honest non-targets. Highest-
leverage lever is Gap A (register bit-field lowering — 32 docs, 12,638 fields), then Gap B (structure carry +
lowering). No code, no canonical mutation → golds/`kg-bench` orthogonal; memory-arch + knowledge-map (112 facts)
gates green; mdBook builds.

## CORPUS-COVERAGE.2 (`2026-06-22`) — re-ingest #19: OpenCAPI 4.0 TL Arch (table-recognition-gap finding + regression-ruled-out method)

**Context.** Second consecutive thin protocol result (after Wishbone). A 0-typed-surface result on TWO protocol specs in a
row demands a regression check before it can be honestly called "doc-style absence" (`feedback_scoring_rigor`).

**Engineering notes — how I ruled out a regression.**
- **Measure the right field.** My first probe counted `interfaces` (= 0), but I then saw `interfaces=0` even for the AHB
  WIRE GOLD (which has 66 relations) — so `interfaces` at the evidence top level isn't the signal-inventory home. The
  load-bearing metric is `actor_signal_relations` (and the source-stage `structured_tables` kinds).
- **Cross-check known-rich docs on the SAME binary.** DTI (159 message-fields), MMU-700 (63 registers), AHB (66 relations)
  all still hold → the binary extracts richly when the doc fits a recognized shape. The thin OpenCAPI/Wishbone results are
  therefore doc-specific, not a code regression.
- **Pin the gap at the table-classification layer.** `structured_tables` kinds: AHB = 13 `signal_description`/40;
  OpenCAPI = **0**/246 (219 `unknown`); Wishbone = 1/25 (22 `unknown`). So Docling DID capture the tables; the extractor
  just can't classify the non-AMBA packet/command/signal-list shapes as signal/field surfaces. That is a precise,
  well-scoped upstream lever (D) — table-recognition for non-AMBA styles — for its own future leaf, not a re-ingest fix.
- **Process lesson.** Re-running `evidence` overwrites the stale `evidence_ir.json`, so capture stale COUNTS before the
  cascade (I did) — but the stale CONTENT is then gone. For a suspected regression, the structured_tables-kind histogram
  on the FRESH source_ir is the durable, content-grounded discriminator (it survives the cascade).

**Decision.** Honest thin refresh recorded; phase pivots to register/TRM/ISA docs (where `.10` families fire) for the next
substantive gains, circling back to the OpenCAPI/USB tail (expected thin) later.

## CORPUS-COVERAGE.2 (`2026-06-22`) — re-ingest #18: Wishbone B4 (honest signal-recall-gap finding)

**Context.** Wishbone B4 is a clean classic bus protocol, so a naive expectation is "re-ingest → rich signals/relations."
The fresh current-binary extraction instead yields **0 interfaces / 0 signal_records → 0 relations** and a 1-signal
`.isf` — a genuine recall gap, not a re-ingest bug (the stale build was already ~0-1 relations).

**Engineering notes.**
- **Diagnose before logging a surprising number.** A 0-relation bus protocol could be (a) a broken cascade, (b) a
  stage-staleness drop, or (c) a true extraction gap. I ruled out (a)/(b) by reading the fresh evidence directly:
  `interfaces=0`, `signal_records=0`, yet `extracted_statement_count=2041` and IntentIR `constraints=215` / transactions=2
  / actors=8 — so the pipeline ran fine and captured the obligations; only the typed *wire* surface is empty. That makes
  it (c): a signal-INVENTORY recall gap, upstream of relations (no signals → no relations possible).
- **Why Wishbone specifically.** Its signals use the `SIGNAL_O()`/`SIGNAL_I()` suffix notation and are introduced in
  prose signal-list sections, not in the `Signal | Direction | Width | Description` table shape the current extractors
  key on. This is exactly the doc-style variance the `PDF-VARIANT-DIGESTION` program exists to chase; it's kin to the
  parked `.9.10` prose-bus-line lever (which is blocked behind the shallow-parser, no denylists).
- **Discipline: surface, don't fix in-slice.** The re-ingest tree explicitly forbids fixing a surfaced lever inside a
  re-ingest slice (it would be a code change without its own owned leaf + gate). Logged as Lever D for a future leaf;
  committed the honest refresh. The `.isf` is thin but valid (FSMGen strict-clean) — honest residual over fabrication.

## CORPUS-COVERAGE.2 (`2026-06-22`) — re-ingest #17: Avalon Interface Spec (current-binary refresh)

**Context.** Continuation of the corpus re-ingest batch (now 17 of 57 normalized-missing docs). Avalon reached IntentIR
in `.0` but on stale EvidenceIR (Jun 7), predating the `.10`/`.12`/`.2` extractor families and the `.1a`/`.1b`
agent-identity gates. This slice re-runs the existing deterministic pipeline with the current binary — no code change.

**Engineering notes.**
- **Binary freshness check matters.** Before any cascade I compared `find crates -name '*.rs' -newer target/release/specforge`:
  `isf_ir.rs` was newer than the prior build, so the cached release binary did NOT contain HEAD's `adapt` code. Rebuilt
  release (`CARGO_BUILD_JOBS=2`, RAM-safe) so the `.isf` lowering ran HEAD. A re-ingest that skipped this would have
  cascaded through a stale emitter — a silent signoff hazard. Worth doing every re-ingest slice when HEAD has code commits.
- **Re-ingest value is doc-shaped, not uniform.** For register/message-heavy docs (DTI 0→159 message fields, SMMU 1→89
  registers) the win is marquee table-family surfaces. For Avalon — a prose/diagram interface spec with no register-field,
  message-field, or presence tables — the families correctly produce nothing; the win is instead (a) current-binary
  freshness and (b) KG cleanup via the consolidation gates (relations 126→111, actors 64→50, folding fragment/phantom
  actors). Reporting "honest absence" rather than forcing a surface is the ADR-0006 / no-fabrication discipline.
- **Verification.** `.isf` renders (`source.isf`, 26 signals); real `fsmgen --strict --check --json` → success / 0
  diagnostics; `validate` → no stage-staleness warning. RAM steady 77–78% free, `.4a` guard armed, Ollama idle.

## ISF-VALUE-WIDTH-EMIT.2 (`2026-06-21`) — emit value-width-aligned ISF literals + complete width recovery (CODE; TREE CLOSED)

**Context.** The `.0/.1` measurement designed the fix and returned GO; `.2` was compile-gated, held until the
host had RAM headroom. It cleared (`memory_pressure` 81% free, well inside the project's own RAM ceiling), so
the emit slice was compiled (debug, single-job, RAM-monitored), verified end-to-end, and committed.

**The two fixes (both in `crates/specforge/src/ir/isf_ir.rs`, ADR-0006 numeric/width arithmetic only).**

1. **Width recovery completeness — `interface_widths`.** `from_intent_ir` builds, alongside the existing
   `.2a.i` `port_widths`, a `BTreeMap<String, u32>` that maps each signal to the *single unambiguous* concrete
   width across **all** of its `intent_ir.interfaces[].signal_records` (collect every `WidthHint::Numeric(n)`
   with `n > 1` into a `BTreeSet`; keep the signal only when the set has exactly one element — a conflict keeps
   the honest width-1 default, mirroring `actor_port_concrete_widths`). The signal-width fallback that used to
   read `port_widths.get(name).unwrap_or(1)` now chains `interface_widths → port_widths → 1`. Root cause this
   closes: the first-seen signal dedup keeps only the first `signal_records` entry, so a width declared in a
   *later* record (trace-bus `ATID`, width 7 in its 3rd record, and no actor-port for `.2a.i` to fall back to)
   was lost to the width-1 default. Live: `ATID` now emits `(output ATID (width 7))`.

2. **Value-literal width-alignment — `align_rule_drive_widths`.** A post-pass inserted right after
   `rules.extend(temporal_isf_rules)` and **before** `dedup_conflicting_rules` (so an aligned value participates
   in conflict detection on its final form). For each rule it builds `signal_widths` from the emitted signal set
   and, per drive `(sig, val)`:
   - `parse_sized_literal(val)` recognises a based literal — `0b…` (notation width = binary-digit count) or
     `0x…` (notation width = hex-digit count × 4, exactly FSMGen's operand-width reading) — and returns `None`
     for a bare decimal (FSMGen-unsized, fits any width), an enum symbol, a reference, or an already-cast
     `W'…` literal, all left untouched. A literal wider than 128 bits overflows `u128::from_str_radix` → `None`
     → safely untouched.
   - `align_value_to_width(val, w)` returns `Keep` (no based literal, or its notation width already equals `w`),
     `Replace("{w}'d{v}")` when the value fits (`w >= 128 || v < 1u128 << w` — the `>= 128` guard avoids the
     `1u128 << 128` shift overflow), or `Residualize` when it does not.
   - A rule with **any** residualizing drive is dropped whole and yields a `value_width_residual_packet`
     (`packet_id = isf_value_width_<sanitized rule name>`, mirroring `rule_conflict_residual_packet`); otherwise
     fitting-but-over-wide literals are rewritten in place and the rule is kept. Order is preserved so the
     downstream dedup still keeps the first rule. The residuals join `temporal_residuals`.

**Why truncation was rejected.** `ATID 0x7D` is the genuine 7-bit value 125. Chopping it to width 1 would
fabricate a wrong value; FSMGen's OperandContract exists precisely to block that. So the honest options are
exactly two: width-align when the value fits, or surface a residual when it does not.

**Verification.** Debug build clean (22.6 s, single-job); 47/47 `isf_ir` tests (3 new: `parse_sized_literal_*`,
`align_value_to_width_*`, `align_rule_drive_widths_*`). Regenerated the affected `.isf` and ran the real
`subs/fsmgen/bin/fsmgen --strict --check --json`:
- DTI `ihi0088_g`: `(ATST 1'd1)` → `has_diagnostics: false` (was: *"assignment to 'ATST' uses RHS '2'b1' with
  incompatible width 2 for LHS width 1"*).
- Trace-bus `ihi0032_c`: `(output ATID (width 7))` + `(ATID 7'd125)` → `has_diagnostics: false` (was width 1 +
  `8'h7D`).
- 4 wire golds regenerated and re-checked: **0 NEW** diagnostics. APB `apb_protocol/requester` 0/0 and SWD
  `agent/debugger` 0/0 unchanged; AHB `address_decoder/manager` 1/1 (`isf_conflicting_rule_writes` on `HAUSER`)
  and AXI `agent`/`manager` 1/1 (`constraint_33` `(port expr)` / `isf_conflicting_rule_writes` on `ASKSTOP`) are
  the documented orthogonal issues, count unchanged.

Full `scripts/run_ci.sh` GREEN: memory-arch + knowledge-map + `cargo fmt` + clippy `-D warnings` + test suite
**1682 passed / 0 failed** + rustdoc `-D warnings` + mdBook. `kg-bench` 156/156. RAM held 81–82% free throughout
(`CARGO_BUILD_JOBS=2`). One `cargo fmt` autoformat (a long `assert!(matches!(…))` wrapped) was the only post-write
change. AXI `(port expr)` and DTI ATST mis-attribution remain spun out (tree Non-Goals).

## ISF-VALUE-WIDTH-EMIT.0/.1 (`2026-06-21`) — measure ISF value-literal width-alignment (measurement-first, docs-only, GO)

**Context.** Resuming the PNT loop on a 6.3 GB host at ~16% free RAM, where the other open frontier thread (the
`CORPUS-COVERAGE.2` long-tail Docling re-ingest) is unsafe per the owner's non-negotiable RAM ceiling. So the
RAM-light spun-out ISF-emitter lever ("Lever A" — value width-alignment) is the right slice, and a
measurement-first (read-only) phase needs no compile at all.

**The defect.** The ISF emitter lowers a `(rule … (SIGNAL value))` clause by copying the constraint/temporal
value literal verbatim from the IntentIR (`render_isf_control_expression` →
`ControlExpressionRecord::Literal { literal } => literal.clone()`, `ir/isf_ir.rs:1493-1495`) and never reconciles
the literal's width against the target signal's emitted `(width N)`. FSMGen's strict `--check`
(`OperandContractValidationSupport.pm`, `validate_pre_generation_operand_contract`) rejects a literal whose
**notation width** (digit count — `0x7D` = 8 bits, `0b00` = 2 bits, `0B01` = 2 bits; *not* the value) differs
from the LHS signal width, demanding *"an explicit width-aligned source expression"* (it blocks implicit
truncation/extension). DTI's `(ATST 0B01)` on `(output ATST (width 1))` renders `2'b1` (width 2) onto width 1 →
fail.

**Measurement (read-only + real FSMGen probes; Perl binary, RAM-safe).** Over the 86 persisted `.isf`: 4 docs /
13 clauses — DTI ATST ×3, AXI+ACE ARTAGOP/BTAGMATCH ×6, AXI-gold AWCMO ×1, trace-bus ATID ×3. DTI + TRACE
confirmed FAIL on OperandContract; the AXI docs fail FIRST on the orthogonal `(port expr)` rule-assignment
grammar (masked). Probe table established FSMGen's semantics: a width-cast `W'<radix><digits>` matching the LHS
width PASSes (`7'd125`, `2'b00`, `1'b1`); a bare decimal is unsized and PASSes any width; a based literal whose
notation width ≠ LHS width FAILs.

**The decisive nuance — truncation would be dishonest.** All four signals are emitted at `(width 1)` *even when
the IntentIR grounds a wider width* (ATID 7 / ARTAGOP 2). Root cause: the emitter's first-seen signal dedup
(`isf_ir.rs:696-700`) keeps the first `signal_records` entry (often `width=None`→1) and skips a later record that
carries the concrete width; the `.2a.i` recovery only falls back to `actor_ports` (ATID has none, so its width-7
third record is lost). So the over-width literal is a *symptom* of an under-emitted width, and ATID `0x7D` = 125
is a genuine 7-bit value — truncating it would fabricate.

**GO — fix design for `.2` (compile-gated, HELD on RAM):** (1) recover the grounded width across **all** interface
`signal_records` + `actor_ports`; (2) re-render the value literal width-aligned as `W'<radix><digits>` when
`value < 2^W`, else **residualize** the clause (the `ISF-RULE-CONFLICT-RESIDUAL` honest-residual pattern), never
truncate. ADR-0006 numeric-only. Spun out: DTI ATST is an upstream **mis-attribution** (the source text's `0b01`
belongs to `ATTR_OVR.SHCFG`; ATST is a value of FLOW), and the AXI `(port expr)` grammar is a separate lever.
Wire-gold orthogonal: only AXI `ihi0022_l` carries one (AWCMO) and it already fails on `(port expr)`;
WIRE-BASED-100 measures extraction F1, not `.isf` bytes. Report
`docs/research/isf-value-width-alignment-measurement.md`; KM `isf-value-width-operand-contract`.
