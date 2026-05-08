# KG Bench And Fixtures

`specforge kg-bench` is the extraction-truthfulness regression harness.

It exists because aggregate scores are not enough.
The project also needs executable examples that prove specific recovery paths and specific negative guards.

## What it checks

The fixture set is used to lock:

- gold paths that must keep working
- negative paths that must stay blocked
- conflict surfacing
- residual quality
- prior-guided before/after behavior
- graph-backed direction coverage without falling back to flat compatibility hints
- compatibility-direction gaps that have no actor-relative graph coverage

This lets the project protect individual truthfulness properties instead of relying only on broad integration runs.

## Why fixtures matter

Without fixture coverage, the pipeline can drift in subtle ways:

- a false actor might quietly reappear
- a coordinated active drive/read clause might only recover the first signal
- a semantic role might start resolving from name noise again
- integration vocabulary like `VIP`, `PLL`, or `DFT` might become fake signals
- a learned prior might start overreaching
- a table-shape rule might begin misclassifying field tables as real signal tables

`kg-bench` exists to catch exactly that kind of drift.

## Graph-Backed Direction Expectations

Fixtures can assert canonical signal inventories directly.
`signal_names_include` checks that required interface signals survive, while `signal_names_exclude` checks that tempting non-signals stay out.
That second field is especially useful for protocol-PDF scope tests: uppercase engineering words such as `PDF`, `RTL`, `VIP`, `PLL`, `DFT`, or `SoC` may be important document context, but they are not automatically interface signals.
The table-shape prior family-mismatch fixture uses the same exclusion surface for table rows: an AXI-local unknown table with `XREQ` and `XACK` rows must stay inert when the only matching `Name | Direction | Width` prior is APB-scoped, so unrelated table-shape memory cannot mint canonical signals or graph directions.
That mismatch guard also checks `EvidenceIR` directly now: the unrelated APB prior must leave table-signal declaration provenance at zero before canonical inventory is even built.
The no-prior table-shape signal-table fixture locks the before side even more directly: without learned shape memory, a local `Name | Direction | Width` table stays out of table-signal provenance and canonical `XREQ`/`XACK` inventory.
The prior-guided table-shape signal-table fixture locks the after side: learned shape memory can classify the same table, but recovered `XREQ`/`XACK` declarations must retain exact `table_0001` provenance and canonical table support.
The actor-taxonomy no-prior section-heading fixture applies the same per-signal graph discipline to local vocabulary: `Issuer` and `Acceptor` section titles can preserve `XADDR`, `XCMD`, and `XRESP` inventory, but without learned actor memory those signals must stay graph-direction empty.
The actor-taxonomy protocol-family mismatch fixture extends that fail-closed check: an APB-scoped `issuer` prior must not give AXI-local `XADDR` graph-backed direction.
The actor-taxonomy prior-guided section-heading fixture locks the positive side: learned `Issuer` and `Acceptor` vocabulary must produce graph-backed direction coverage for `XADDR`, `XCMD`, and `XRESP`.

The `signal_table_inventory_authority_negative` fixture locks the positive side of that same boundary.
It provides real signals only through a structured `Signal | Direction | Width | Description` table, then surrounds that table with uppercase implementation vocabulary in prose.
The expected result is intentionally sharp: `XREQ`, `XACK`, and `PAYLOAD` become canonical signals with table-derived directions, while terms such as `PDF`, `RTL`, `VIP`, `PLL`, `DFT`, `CDC`, `CTS`, `ECO`, and `SoC` remain document context.

Fixtures can also assert table provenance for canonical interface signals.
`signal_supporting_table_ids_include` checks `InterfaceSignalRecord.supporting_table_ids`, proving that a recovered signal remains tied to the structured `SourceIR` table that authored its synthesized declaration.
The harness self-tests wrong-support and absent expected-signal diagnostics at both `SemanticIR` and `IntentIR`, so these failures identify the fixture, stage, signal-specific expectation field, missing table id or signal, and actual support or signal set.
That matters for signal-table-heavy protocol PDFs: the pipeline should be able to explain that `XREQ` came from a specific signal-description table, not merely that a synthetic `Signal XREQ is output` sentence happened to exist somewhere downstream.
Fixtures can also assert the EvidenceIR side directly with `table_signal_declaration_provenance_include`.
That checks the table-synthesized declaration before canonical carry-through: the signal name, source table id, and optionally the generated statement text must match.
For negative fixtures, `table_signal_declaration_provenance_count` can require zero evidence-stage provenance records, which is how field-table false positives stay blocked before they reach canonical signal inventory.
The field-table misclassification fixture also locks table-sourced semantic hints at zero, proving those rows cannot invent roles while provenance stays empty.
Ordinary prose hints are zero too, so text recovery cannot backfill roles from field names while provenance stays empty.
Alias-grounded prose is excluded as well, preventing phrase-alias recovery from turning field names into semantic roles.
Visual-caption hints are zero too, keeping captions from backfilling roles for field names while provenance stays empty.
VLM timing-annotation hints are zero as well, keeping timing notes from turning field names into semantic roles while provenance stays empty.
The harness has focused regression tests for the count, missing-record, and synthesized-statement mismatch diagnostics around this EvidenceIR provenance surface.
The same fixture family can also assert persisted validation metrics such as EvidenceIR `table_signal_declaration_provenance` and canonical `with_table_support`, so the user-visible validator surface stays aligned with both the evidence-stage table bridge and the exact canonical provenance checks.

Fixtures can also assert validation findings directly.
The validation expectation surface supports coarse `finding_ids_include` / `finding_ids_exclude` checks, and a more precise `findings_include` form that can match a finding id plus optional severity, category, summary substring, required related ids, and excluded related ids.
That matters for caution and rescan guidance: a fixture can prove not only that validation emitted `semantic_negative_knowledge_prior_matches`, but also that the finding points at the exact local conflict or residual packet that triggered the caution.

Actor-relative direction is a graph surface, not just a flat signal annotation.

Fixtures can therefore assert graph direction coverage directly with `graph_direction_signal_names_include` and `graph_direction_signal_names_exclude`.
Those fields inspect canonical `actor_ports` for non-`unknown` direction evidence by signal name.

This is intentionally separate from `signal_directions_include`.
The older field still checks the flat compatibility `direction_hint` on interface signal records.
Keeping both surfaces separate prevents the harness from flattening producer/consumer actor roles into a fake single perspective while still letting graph-first recovery be tested explicitly.
The tracked fixtures now also separate graph-backed compatibility lag from genuinely unresolved direction gaps.
When a declared signal has no flat `direction_hint` and no actor-relative graph direction, validation must report the `*_compat_direction_hints_incomplete` finding ids rather than the graph-backed `*_compat_direction_hints_lag_graph` ids.
The mixed fixture keeps both states in one artifact so related ids stay signal-specific when graph-backed lag and unresolved direction evidence appear together.
A conflicted graph-direction fixture proves same-actor disagreement is not credited as graph coverage and therefore cannot produce a graph-backed compatibility-lag finding.
The flat-hint-missing version also asserts both replay-guidance findings: graph coverage guidance related to the signal id and graph-conflict guidance related to the actor-aware conflict id.
The flat-hint-present conflict fixture locks the converse boundary: graph coverage remains unresolved, compatibility-direction debt stays absent while the flat direction hint exists, and the same paired replay-guidance payloads remain explicit.

Fixtures can assert canonical signal-connectivity conflicts directly.
`signal_connectivity_conflicts_include` checks the conflicted signal, conflict kind, conflicting actor ids or names, and optional supporting statement ids.
That lets graph-conflict fixtures prove the actual multi-producer actor set rather than reducing the test to "one connectivity conflict exists."

Fixtures can assert canonical temporal rules directly too.
`temporal_rules_include` checks typed clock/edge grounding, cycle windows, supporting statement ids, antecedent predicates, and consequent predicates.
That lets APB/AHB/AXI timing fixtures prove actor-grounded drive or stability predicates, compound guards, and `HandshakeComplete` predicates as IR shape rather than only as validation-count side effects.
The `temporal_cycle_window_surface_negative` fixture locks the replay side of the same surface: a rule can already have clock and actor grounding while still lacking a latency bound, and validation must keep emitting cycle-window rescan guidance for that exact rule id instead of treating the rule as settled.
The companion `temporal_cycle_window_grounded_gold` fixture locks the no-rescan side: once a local rule carries `within 2 cycles`, clock grounding, and actor grounding, it must keep the explicit cycle window and avoid cycle-window replay findings.
The unbounded fixture also locks replay lane separation: cycle-window debt must not masquerade as actor-grounding or clock-grounding debt when those fields are already grounded.
The actor-grounding negative fixture locks the mirror separation: when clock grounding and the cycle window are already present, missing actor grounding must not masquerade as clock or cycle-window replay debt.
The clock-grounding negative fixture completes that separation: when actor grounding and the cycle window are already present, missing clock grounding must not masquerade as actor or cycle-window replay debt.
That clock-grounding fixture also asserts exact temporal-rule shape, so the clockless rule keeps unknown edge, actor-stability and signal-stability predicates, max-cycle window, and supporting statement id through `SemanticIR` and `IntentIR`.
The actor-grounding fixture now asserts exact temporal-rule shape too, so the actorless rule keeps clock signal, edge, asserted-value predicate, max-cycle window, and supporting statement id through both canonical layers.
The cycle-window fixture now asserts exact temporal-rule shape as well, so the unbounded rule keeps clock signal, edge, actor-drive predicate, asserted-value predicate, and supporting statement id while validation still reports the missing window.
The bounded fully grounded fixture now also excludes every temporal replay lane, proving no cycle-window, actor-grounding, or clock-grounding guidance appears after all three surfaces are present.
The tracked suite now includes AXI write-address, write-address-id-stability, write-address-control-sideband-stability, write-response, write-response-id-stability, address-response-user-sideband-stability, read-address, read-address-id-stability, read-address-control-sideband-stability, address-qos-region-sideband-stability, read-address-sideband-stability, read-data, read-data-id-stability, read-data-response-stability, read-data-last-stability, data-user-sideband-stability, write-data, write-data-last-stability, sideband-stability, and write-address-sideband-stability timing gold paths, so the same width-only table plus prose actor-relation pattern is checked across `AWVALID` / `AWREADY` / `AWADDR` / `AWID` / `AWPROT` / `AWCACHE` / `AWLOCK` / `AWQOS` / `AWREGION` / `AWUSER`, `BVALID` / `BREADY` / `BRESP` / `BID` / `BUSER`, `ARVALID` / `ARREADY` / `ARADDR` / `ARLEN` / `ARID` / `ARPROT` / `ARCACHE` / `ARLOCK` / `ARQOS` / `ARREGION` / `ARUSER`, `RVALID` / `RREADY` / `RDATA` / `RRESP` / `RID` / `RUSER`, `WVALID` / `WREADY` / `WDATA` / `WSTRB` / `WUSER`, explicit sideband hold obligations for `ARLEN` and `WSTRB`, read-address sideband holds for `ARSIZE` and `ARBURST`, write-address sideband holds for `AWLEN`, `AWSIZE`, and `AWBURST`, write-address control sideband holds for `AWPROT`, `AWCACHE`, and `AWLOCK`, read-address control sideband holds for `ARPROT`, `ARCACHE`, and `ARLOCK`, address QoS/region holds for `AWQOS`, `AWREGION`, `ARQOS`, and `ARREGION`, address/response USER sideband holds for Manager-owned `AWUSER` / `ARUSER` and Subordinate-owned `BUSER`, data USER sideband holds for `WUSER` and `RUSER`, manager-owned write-address ID stability for `AWID`, manager-owned read-address ID stability for `ARID`, subordinate-owned write-response ID stability for `BID`, subordinate-owned read-data ID stability for `RID`, subordinate-owned read-data response stability for `RRESP`, subordinate-owned read-data last-beat stability for `RLAST`, and manager-owned write-data last-beat stability for `WLAST`.
The AXI read-address sideband fixture now also keeps prose-sourced semantic hints at zero, proving `ARSIZE` and `ARBURST` role hints are table-only in that gold path.
Alias-grounded prose hints are zero there too, so phrase-alias recovery cannot masquerade as table-backed sideband evidence.
Visual-caption hints are zero there too, so diagram captions cannot masquerade as table-backed sideband evidence.
VLM timing-annotation hints are zero there as well, so timing notes cannot masquerade as table-backed sideband evidence.
The AXI read-data ID fixture also keeps prose-sourced semantic hints at zero, proving `RID` role hints are table-only in that gold path.
Alias-grounded prose hints are zero there too, so phrase-alias recovery cannot masquerade as table-backed ID evidence.
Visual-caption hints are zero there too, so diagram captions cannot masquerade as table-backed ID evidence.
VLM timing-annotation hints are zero there as well, so timing notes cannot masquerade as table-backed ID evidence.
The AXI read-data last fixture keeps prose-sourced semantic hints at zero too, proving `RLAST` role hints are table-only in that gold path.
Alias-grounded prose hints are zero there too, so phrase-alias recovery cannot masquerade as table-backed last-beat evidence.
Visual-caption hints are zero there too, so diagram captions cannot masquerade as table-backed last-beat evidence.
VLM timing-annotation hints are zero there as well, so timing notes cannot masquerade as table-backed last-beat evidence.
The AXI sideband stability fixture keeps prose-sourced semantic hints at zero too, proving its sideband role hints are table-only in that gold path.
The APB timing family now also includes a write-control stability path: `PWRITE`, `PWDATA`, and `PSTRB` must remain requester-owned stable outputs while `PSEL` and `PENABLE` are high and `PREADY` is low, without treating that wait-state guard as a completed handshake.
Its address/protection wait-state mirror checks `PADDR` and `PPROT` as Requester-owned stable outputs under the same `PSEL HIGH` / `PENABLE HIGH` / `PREADY LOW` stalled-access guard, again without promoting the wait state to handshake completion.
Its response-stability mirror checks the completed-access side: `PRDATA` and `PSLVERR` must remain completer-owned stable outputs when `PSEL`, `PENABLE`, and `PREADY` are high, and the same rule must preserve `HandshakeComplete(PSEL, PREADY)`.
The AHB timing family now includes a control-stability wait-state path too: `HADDR`, `HWRITE`, `HSIZE`, `HBURST`, and `HPROT` must remain manager-owned stable outputs while `HREADY` is low and `HSEL` is high, with no false handshake-completion predicate.
Its transfer/lock mirror checks `HTRANS` and `HMASTLOCK` as manager-owned stable outputs under the same `HREADY LOW` / `HSEL HIGH` wait-state guard, proving transfer-type and locked-transfer attributes remain owned protocol obligations without promoting the stall to handshake completion.
Its exclusive/security mirror checks `HEXCL` and `HNONSEC` as manager-owned stable controls plus `HEXOKAY` as a subordinate-owned stable response under the same wait-state guard, proving optional AHB5-style fields retain producer ownership without promoting the stall to handshake completion.
Its response-side mirror checks `HRDATA` and `HRESP` as subordinate-owned stable outputs under the same `HREADY LOW` / `HSEL HIGH` wait-state guard, again without promoting the stalled transfer to handshake completion.
The write-data path adds the data-phase guard: `HWDATA` must remain manager-owned stable while `HREADY` is low, `HSEL` is high, and `HWRITE` is high, proving three-predicate wait-state stability without a false completion event.

Fixtures can assert canonical temporal conflicts directly too.
`temporal_conflicts_include` checks the contradiction signal, phase, clock/edge context, antecedent predicates, conflicting values, supporting rule ids, and supporting statement ids.
That keeps conflict-surfacing benchmarks honest: a prior can add caution guidance, but it cannot make the current-document contradiction disappear.

Fixtures can assert canonical signal-semantic conflicts directly as well.
`signal_semantic_conflicts_include` checks the conflicted signal and included observations by semantic tag, source kind, source text, and supporting evidence ids.
That lets multimodal disagreement fixtures prove, for example, that visual-caption evidence and VLM timing-diagram annotation evidence support different handshake roles without reducing the test to "one conflict exists."
The cross-modality semantic-conflict fixture also locks total evidence semantic hints at two, matching its table and visual-caption source splits.
It also locks the prose split at zero, so the conflict stays attributable to the intended non-prose modalities.
The alias-grounded prose split is zero too, preventing phrase-alias recovery from becoming a hidden third source.
The VLM timing-annotation split is zero in that fixture as well, keeping the visual source caption-only.
Timing-diagram extractions are zero there too, keeping timing extraction support out of the table-plus-caption conflict.

Fixtures can assert canonical signal-polarity conflicts directly as well.
`signal_polarity_conflicts_include` checks the conflicted signal and included observations by polarity, source kind, and supporting statement or table ids.
That lets polarity-disagreement fixtures prove that active-high prose and active-low table evidence both survived into the canonical conflict surface, rather than only counting that some polarity conflict exists.

Fixtures can assert canonical resolved signal polarity directly as well.
`signal_polarities_include` checks the signal name and expected active-high or active-low polarity on the canonical interface signal record.
That lets polarity gold fixtures prove `CS_N` is active-low or `ENABLE` is active-high as typed IR shape, rather than only proving that the number of resolved-polarity signals increased.

Fixtures can assert canonical resolved semantic roles directly as well.
`resolved_semantic_roles_include` checks the signal name and expected valid-like or ready-like role on the canonical interface signal record.
That lets semantic gold fixtures prove `XREQ` is valid-like or `XACK` is ready-like as typed IR shape, rather than only proving that some semantic role was resolved.
The semantic phrase prior gold uses this for `XACK can receive the transfer`: prior-guided recovery must resolve exactly to ready-like with single-source grounding.
That gold now also asserts table-sourced semantic hints stay zero, keeping the recovery prose-only.
Alias-grounded prose stays zero there too, keeping alias recovery out of the direct phrase-prior proof.
Visual-caption hints stay zero as well, keeping the direct phrase-prior proof prose-only.
VLM timing-annotation hints stay zero too, keeping timing-note evidence out of the direct phrase-prior proof.
Timing-diagram extractions stay zero too, keeping the direct phrase-prior proof free of timing extraction support.
The protocol-family mismatch semantic phrase fixture complements that positive case by requiring the local `XREQ` output declaration to survive exactly while an unrelated APB phrase prior stays silent.
That family-mismatch guard now also asserts table-sourced semantic hints stay zero while the APB prior remains silent.
Alias-grounded prose stays zero there too, keeping alias recovery out of the family-mismatch guard.
Visual-caption hints stay zero as well, keeping the family-mismatch guard non-visual.
VLM timing-annotation hints stay zero too, keeping timing-note evidence out of the family-mismatch guard.
Timing-diagram extractions stay zero too, keeping the family-mismatch guard free of timing extraction support.
The source-kind mismatch companion applies the same signal-shape discipline to `XSTAGE`: a visual-caption-only phrase prior stays silent for prose evidence while the local output declaration remains canonical.
That source-kind guard now also asserts table-sourced semantic hints stay zero while the visual-caption-only prior remains silent for prose evidence.
Alias-grounded prose stays zero there too, keeping alias recovery out of the source-kind guard.
Visual-caption hints stay zero as well, keeping the source-kind guard free of visual semantic hints.
VLM timing-annotation hints stay zero too, keeping timing-note evidence out of the source-kind guard.
Timing-diagram extractions stay zero too, keeping the source-kind guard free of timing extraction support.
The exact-phrase mismatch companion does the same for `XFLOW`: an unmatched learned phrase prior stays silent while the local output declaration remains canonical.
That exact-phrase mismatch guard now also asserts table-sourced semantic hints stay zero while the unmatched prior remains silent.
Alias-grounded prose stays zero there too, keeping alias recovery out of the exact-phrase mismatch guard.
Visual-caption hints stay zero there as well, keeping visual evidence out of the exact-phrase mismatch guard.
VLM timing-annotation hints stay zero there too, keeping timing-note evidence out of the exact-phrase mismatch guard.
Timing-diagram extractions stay zero there too, keeping timing extraction support out of the exact-phrase mismatch guard.
The broad-phrase guard adds the one-token case: a generic `transfer` prior stays inert while the same `XFLOW` declaration remains canonical.
That broad-phrase guard now also asserts table-sourced semantic hints stay zero while the one-token prior remains silent.
Alias-grounded prose stays zero there too, keeping alias recovery out of the broad-phrase guard.
Visual-caption hints stay zero there as well, keeping visual evidence out of the broad-phrase guard.
VLM timing-annotation hints stay zero there too, keeping timing-note evidence out of the broad-phrase guard.
Timing-diagram extractions stay zero there too, keeping timing extraction support out of the broad-phrase guard.
The conflicting-role guard adds the ambiguity case: equal valid-like and ready-like priors for one phrase stay inert while the local `XCTRL` input declaration remains canonical.
That conflicting-role guard now also asserts table-sourced semantic hints stay zero while the equal priors fail closed.
Alias-grounded prose stays zero there too, keeping alias recovery out of the conflicting-role guard.
Visual-caption hints stay zero there as well, keeping visual evidence out of the conflicting-role guard.
VLM timing-annotation hints stay zero there too, keeping timing-note evidence out of the conflicting-role guard.
Timing-diagram extractions stay zero there too, keeping timing extraction support out of the conflicting-role guard.
The no-prior mirror keeps the before side exact too: `XACK can receive the transfer` stays unresolved without learned memory while the local input declaration remains canonical.
That no-prior mirror now also asserts table-sourced semantic hints stay zero while learned memory is absent.
Alias-grounded prose stays zero there too, keeping the no-prior semantic phrase mirror free of alias recovery.
Visual-caption hints stay zero as well, keeping the no-prior semantic phrase mirror non-visual.
VLM timing-annotation hints stay zero too, keeping timing-note evidence out of the no-prior semantic phrase mirror.
Timing-diagram extractions stay zero too, keeping the no-prior semantic phrase mirror free of timing extraction support.
The ready-sink no-prior mirror extends that before-side check to `XACK can sink the transfer`, again preserving the local input declaration while learned memory is absent.
That ready-sink no-prior mirror now also asserts table-sourced semantic hints stay zero while learned memory is absent.
Alias-grounded prose stays zero there too, keeping the no-prior ready-sink mirror free of alias recovery.
Visual-caption hints stay zero as well, keeping the no-prior ready-sink mirror non-visual.
VLM timing-annotation hints stay zero too, keeping timing-note evidence out of the no-prior ready-sink mirror.
Timing-diagram extractions stay zero too, keeping the no-prior ready-sink mirror free of timing extraction support.
The valid-like no-prior mirror does the same for `XREQ can publish the beat`, preserving the local output declaration while learned memory is absent.
That valid-like no-prior mirror now also asserts table-sourced semantic hints stay zero while learned memory is absent.
Alias-grounded prose stays zero there too, keeping the no-prior valid-like mirror free of alias recovery.
Visual-caption hints stay zero as well, keeping the no-prior valid-like mirror non-visual.
VLM timing-annotation hints stay zero too, keeping timing-note evidence out of the no-prior valid-like mirror.
Timing-diagram extractions stay zero too, keeping the no-prior valid-like mirror free of timing extraction support.
The valid-like prior-guided gold locks the after side: learned memory resolves `XREQ can publish the beat` as valid-like while preserving the local output declaration.
That valid-like gold now also asserts table-sourced semantic hints stay zero, keeping the valid-like phrase recovery prose-only.
Alias-grounded prose stays zero there too, keeping alias recovery out of the direct valid-like proof.
Visual-caption hints stay zero as well, keeping the direct valid-like proof prose-only.
VLM timing-annotation hints stay zero too, keeping timing-note evidence out of the direct valid-like proof.
Timing-diagram extractions stay zero too, keeping the direct valid-like proof free of timing extraction support.
The ready-sink prior-guided gold mirrors that after-side check: learned memory resolves `XACK can sink the transfer` as ready-like while preserving the local input declaration.
That ready-sink gold now also asserts table-sourced semantic hints stay zero, keeping the ready-sink phrase recovery prose-only.
Alias-grounded prose stays zero there too, keeping alias recovery out of the direct ready-sink proof.
Visual-caption hints stay zero as well, keeping the direct ready-sink proof prose-only.
VLM timing-annotation hints stay zero too, keeping timing-note evidence out of the direct ready-sink proof.
Timing-diagram extractions stay zero too, keeping the direct ready-sink proof free of timing extraction support.
The receive prior-guided gold keeps the older ready-like phrase equally exact: learned memory resolves `XACK can receive the transfer` while preserving the local input declaration.
The visual-caption no-prior mirror carries the same discipline across modalities: the caption phrase stays unresolved without learned visual memory while the local `XACK` input declaration remains canonical.
That no-prior caption mirror also keeps table-sourced semantic hints at zero, so table evidence cannot rescue the missing visual prior.
Ordinary prose hints are zero too, so text evidence cannot backfill the missing visual prior.
Alias-grounded prose is excluded as well, keeping phrase-alias recovery from bypassing the missing visual prior.
VLM timing-annotation hints stay zero too, so timing notes cannot stand in for absent visual prior memory.
The visual-caption prior-guided gold locks the after side for that modality, resolving the caption phrase as ready-like while preserving the local input declaration.
The visual-motif no-prior mirror keeps diagram-classification recovery separate from table truth: the unknown motif stays ambiguous while table-derived `XREQ` output remains canonical.
That no-prior motif mirror also keeps table-sourced semantic hints at zero, so table evidence cannot rescue absent motif memory.
Ordinary prose hints are zero too, so text evidence cannot backfill absent motif memory.
Alias-grounded prose is excluded too, keeping phrase-alias recovery from bypassing absent motif memory.
VLM timing-annotation hints stay zero too, keeping timing notes from standing in for absent motif memory.
The visual-motif prior-guided gold locks the after side for diagram classification: learned memory classifies the motif while table-derived `XREQ` output remains canonical and no semantic role is invented.
That motif gold also keeps table-sourced semantic hints at zero, proving learned classification does not create table-derived roles.
Ordinary prose hints are zero too, proving learned motif classification does not invent text-derived roles.
Alias-grounded prose hints are zero too, proving learned motif classification cannot create phrase-alias roles.
VLM timing-annotation hints are zero as well, keeping timing-note evidence from masquerading as semantic roles.
That visual-motif gold now also checks the evidence-stage corroboration finding payload, so prior-classified `visual_0001` remains targeted for VLM/multimodal follow-up instead of being silently promoted.
The visual-motif family-mismatch negative keeps table-sourced semantic hints at zero too, so an APB-scoped motif prior cannot add table-derived roles to AXI-local evidence.
Ordinary prose hints stay zero there as well, preventing unrelated motif memory from creating text-derived roles.
Alias-grounded prose hints stay zero there too, keeping phrase-alias recovery from bypassing protocol-family scoping.
VLM timing-annotation hints stay zero there as well, so timing-note evidence cannot bypass protocol-family scoping.

Fixtures can assert canonical semantic grounding strength directly as well.
`semantic_grounding_strengths_include` checks the signal name and expected single-source, multi-source, or cross-modality grounding strength on the canonical interface signal record.
That lets semantic gold fixtures prove a multimodal `XREQ` role is really cross-modality grounded, or a VLM-only `XACK` role is really single-source grounded, rather than only proving that a validation counter changed.
The cross-modality grounding gold also locks total evidence semantic hints at two, tying decisive grounding to the table and visual-caption observations.
It also locks one table-sourced semantic hint, so the aggregate count cannot drift away from its structured-table contribution.
The same gold fixture locks one visual-caption semantic hint as the other half of that cross-modality evidence.
It excludes ordinary prose hints too, preserving the fixture as a table plus visual-caption proof rather than a hidden text-only shortcut.
Alias-grounded prose hints are excluded as well, so alias recovery cannot silently satisfy the positive multimodal case.
VLM timing-annotation hints are excluded too, keeping the fixture's visual side caption-grounded rather than timing-note-grounded.
Timing-diagram extractions are zero there too, keeping timing extraction support out of the positive caption/table proof.
The VLM timing semantic-grounding gold also checks `timing_diagram_extractions`, so ready-like `XACK` recovery from a timing annotation must remain tied to an evidence-stage timing diagram extraction.

Fixtures can assert canonical interface-signal conflicts directly as well.
`interface_signal_conflicts_include` checks the conflicted signal, conflict kind, and included observation values plus supporting statement ids.
That lets direction/width disagreement fixtures prove that the canonical layers preserved the exact contested shape, such as `input` versus `output` or width `8` versus `16`, rather than only counting that some interface conflict exists.

Fixtures can also assert canonical clock/reset infrastructure records directly.
`infrastructure_signals_include` checks the typed infrastructure signal surface, and `infrastructure_topologies_include` checks explicit current-document topology such as clock-gated branches, reset synchronizer stages, and reset-tree targets.
That keeps infrastructure doctrine executable instead of leaving it as prose-only guidance or aggregate validation metrics.

## Gold fixtures versus negative fixtures

Gold fixtures prove that a wanted path works.

Negative fixtures prove that a dangerous path stays blocked.

Both are equally important.
For a provenance-first extractor, “did not hallucinate a fact” is often just as valuable as “recovered the intended fact.”

One actor-connectivity negative fixture now protects relative-clause actor hygiene directly.
`relative_clause_actor_noise_negative` proves that AXI-style prose such as `An interconnect which connects to components with a mixture of chunking support can drive ARCHUNKEN and RCHUNKV` recovers `interconnect` as the producer for both signals, that `The Manager samples ARCHUNKEN and RCHUNKV` recovers the consumer side for both signals, keeps `mixture of` out of the graph, and preserves zero signal-connectivity conflicts through `SemanticIR` and `IntentIR`.

## Prior-guided fixtures

The benchmark harness can also stage fixture-local `CorpusMemory`.

That matters because the learning plane must be tested with the same discipline as the document pipeline:

- without the prior, the unseen local phrase or table should stay unresolved
- with the matching prior, the current document should recover the meaning locally
- the prior must widen interpretation, not author facts on its own
- caution priors must not suppress local conflicts or residuals

The temporal prior-guided fixture also asserts exact canonical rule shape: a learned `one beat later` phrase can recover a one-cycle window, but the resulting rule must still preserve the current document's local clock, edge, asserted-value predicate, and supporting statement id.
The no-prior mirror fixture asserts the same local rule shape without the recovered window, so the benchmark distinguishes prior-enabled improvement from ordinary local temporal evidence.
The prior-guided fixture also excludes cycle-window and clock-grounding replay findings after the recovered rule is fully bounded and locally clocked.
The no-prior mirror fixture keeps clock-grounding replay absent while cycle-window replay remains present, so the missing prior cannot be misreported as a missing clock.
The protocol-family mismatch temporal fixture applies the same rule-shape and replay-lane discipline to an AXI-local `XREADY` sentence with an unrelated APB prior: the local clocked asserted-value rule remains canonical, the APB prior cannot add the cycle window, and only cycle-window replay guidance remains.
The visual-motif protocol-family mismatch fixture locks the neighboring multimodal boundary: the local table-derived `XREQ` signal keeps its output direction and table support, but an unrelated APB visual motif creates no semantic candidate or consensus.
The semantic-modality protocol-family mismatch fixture locks semantic arbitration itself: the local table valid-like and prose ready-like observations stay an explicit conflict, and an unrelated APB modality-reliability prior cannot make that conflict decisive.
That semantic-modality mismatch fixture also locks two aggregate evidence semantic hints, matching the local table and prose observations before arbitration.
It now locks one table-sourced semantic hint too, keeping the structured-table side explicit.
It locks one prose-sourced semantic hint as well, keeping the text side explicit.
Alias-grounded prose is explicitly zero there, so alias recovery cannot satisfy the family-scoped mismatch case.
Visual-caption hints are zero there too, preserving the fixture as a table/prose conflict.
VLM timing-annotation hints are also zero, keeping timing-note extraction from silently satisfying the family-scoped mismatch case.
Timing-diagram extractions stay at zero as well, so the guard remains a non-visual table/prose conflict.
That mismatch fixture now also checks the non-decisive arbitration and rescan-guidance finding payloads directly, so a scoped-out APB prior cannot erase actionable AXI-local `XCTRL` and `semantic_conflict_0001` follow-up debt.
The source-kind mismatch companion applies the same exact conflict-shape check when the prior is learned for visual captions but the current evidence is table plus prose.
That source-kind guard now checks the same non-decisive arbitration and rescan-guidance payloads directly, proving a visual-caption-only prior cannot clear table/prose `XCTRL` follow-up debt.
It also locks two aggregate evidence semantic hints, so the source-kind guard stays tied to the local table/prose observations rather than prior memory.
The same source-kind guard now locks one table-sourced semantic hint, keeping the structured-table side explicit before arbitration.
It locks one prose-sourced semantic hint too, keeping the text side explicit before arbitration.
Alias-grounded prose is explicitly zero for the source-kind guard, so phrase-alias recovery cannot masquerade as the intended table/prose evidence.
Visual-caption hints are zero there too, so the visual-only prior cannot be confused with current-document visual evidence.
VLM timing-annotation hints are also zero, keeping the source-kind guard free of timing-note evidence.
Timing-diagram extractions stay at zero as well, keeping the source-kind guard a non-visual table/prose conflict.
The weak-prior companion applies the same check when the prior record is underpowered: weak memory stays non-decisive while the local conflict remains explicit and typed.
It now locks two aggregate evidence semantic hints too, so weak memory cannot change the local table/prose evidence count before arbitration.
The weak-prior guard now locks one table-sourced semantic hint as well, keeping the structured-table side explicit.
It locks one prose-sourced semantic hint too, keeping the text side explicit.
Alias-grounded prose stays zero in that guard, so alias recovery cannot hide the weak-prior boundary.
Visual-caption hints are zero there too, preserving the guard as a table/prose conflict.
VLM timing-annotation hints are also zero, keeping timing-note evidence out of the weak-prior guard.
Timing-diagram extractions stay at zero as well, keeping the weak-prior guard a non-visual table/prose conflict.
That weak-prior fixture now also locks the non-decisive arbitration and rescan-guidance payloads, so insufficient support cannot hide the unresolved `XCTRL`/`semantic_conflict_0001` review target.
The no-prior companion locks the before side of the same pair: absent modality memory means the local table/prose conflict remains explicit and non-decisive.
It now locks two aggregate evidence semantic hints as well, proving absent memory cannot change the local table/prose evidence count before arbitration.
It locks one table-sourced semantic hint too, keeping the structured-table side explicit before arbitration.
It locks one prose-sourced semantic hint too, keeping the text side explicit before arbitration.
Alias-grounded prose stays zero in the no-prior mirror, so alias recovery cannot hide the absent-memory boundary.
Visual-caption hints stay zero there too, preserving the no-prior mirror as a table/prose-only conflict.
VLM timing-annotation hints stay zero as well, keeping timing-note evidence out of the no-prior mirror.
Timing-diagram extractions stay zero too, so the no-prior mirror remains a non-visual table/prose conflict.
That no-prior fixture now also checks the non-decisive arbitration and rescan-guidance finding payloads directly, so absent memory still leaves actionable `XCTRL` and `semantic_conflict_0001` validation debt.
The prior-guided gold fixture locks the positive side without overstating provenance: the learned modality prior makes `XCTRL` resolve as valid-like, but canonical grounding strength remains `single_source` because the current document still supplies the grounded table observation.
It now locks two aggregate evidence semantic hints as well, proving prior-guided arbitration starts from the same local table/prose conflict rather than extra hidden evidence.
It locks one table-sourced semantic hint too, keeping the structured-table side explicit before learned arbitration.
It locks one prose-sourced semantic hint too, keeping the text side explicit before learned arbitration.
Alias-grounded prose stays zero in the prior-guided gold too, keeping alias recovery out of the positive modality-prior proof.
Visual-caption hints stay zero there too, preserving the positive modality-prior proof as a table/prose arbitration.
VLM timing-annotation hints stay zero as well, keeping timing-note evidence out of the positive modality-prior proof.
Timing-diagram extractions stay zero too, preserving the positive modality-prior proof as a non-visual table/prose arbitration.
The AMBA-generic fallback gold applies that same exact shape to a generic prior used by an AXI-family document, proving generic memory can guide arbitration without widening grounding claims.
It now locks two aggregate evidence semantic hints too, proving AMBA-generic fallback starts from the same local table/prose conflict rather than extra hidden evidence.
It locks one table-sourced semantic hint too, keeping the structured-table side explicit before generic-prior arbitration.
It locks one prose-sourced semantic hint too, keeping the text side explicit before generic-prior arbitration.
Alias-grounded prose stays zero in the fallback gold too, keeping alias recovery out of the generic fallback proof.
Visual-caption hints stay zero there too, preserving the generic fallback proof as a table/prose arbitration.
VLM timing-annotation hints stay zero as well, keeping timing-note evidence out of the generic fallback proof.
Timing-diagram extractions stay zero too, preserving the generic fallback proof as a non-visual table/prose arbitration.
That fallback fixture now also checks the prior-guided arbitration, consensus, and rescan-guidance finding payloads directly, so generic memory must produce the same `XCTRL`-related validation details as exact-family memory.

Negative-knowledge fixtures make that last rule executable.
The suite now has prior-guided caution fixtures for signal-semantic conflicts, temporal conflicts, residual packets, signal-connectivity conflicts, and interface-signal conflicts.
Those fixtures require the matched local conflict or residual to remain present while validation only adds `negative_knowledge_prior_matches`, rescan recommendations, corroboration requirements, and stage-specific rescan-guidance findings.
They also assert the related ids carried by those validation findings, so a caution prior cannot silently drift from "this exact conflict/residual needs attention" into an ungrounded aggregate warning.
The positive semantic-conflict caution fixture now checks the conflict shape directly too: `XCTRL` must still contain visual-caption valid-like evidence and VLM timing-annotation ready-like evidence after negative-knowledge guidance is applied.
It also excludes table semantic hints, keeping that caution-guided conflict visual-only.
Ordinary prose hints are excluded too, so text-only evidence cannot satisfy the caution case.
Alias-grounded prose hints are excluded as well, keeping alias recovery out of the caution-guided conflict.
The negative-knowledge protocol-family mismatch fixture locks the other side of that contract: an unrelated APB caution prior stays silent while the local AXI `XCTRL` semantic conflict remains exact, including its visual-caption valid-like and VLM-annotation ready-like observations.
That mismatch fixture also locks one visual-caption semantic hint, so protocol-family scoping cannot make the local AXI caption evidence disappear.
It locks one VLM timing-annotation semantic hint too, preserving the other side of the local conflict while the APB caution prior remains scoped out.
The same fixture excludes table semantic hints, keeping that local conflict visual-only.
It excludes ordinary prose hints as well, so the mismatch case cannot be satisfied by text evidence.
Alias-grounded prose is excluded too, keeping phrase-alias recovery out of this scoped-family guard.
The fixture also locks one timing-diagram extraction, tying the VLM-side semantic hint to extracted visual evidence.
It also keeps arbitration explicit: `XCTRL` still enters semantic arbitration, but the unrelated prior must not make that arbitration decisive.

Timing-annotation fixtures also cover polarity-sensitive multimodal evidence.
For example, `vlm_timing_active_low_assertion_equivalence_gold` proves that an active-low reset observed by a VLM timing diagram as both `asserted` and `LOW` becomes typed temporal evidence without creating a false temporal conflict.
That assertion fixture also checks zero VLM timing-annotation semantic hints, so reset assertion prose cannot accidentally become handshake valid/ready-like evidence.
It now locks the total semantic-hint count at zero too, closing off alternate hint sources for the same reset evidence.
It also checks zero table-sourced semantic hints, keeping active-low reset assertion evidence out of table-driven role recovery.
Ordinary prose hints stay zero as well, so text around the reset assertion cannot backfill a handshake role.
Alias-grounded prose is excluded too, keeping phrase-alias recovery from inventing a reset semantic role.
Visual-caption hints are zero too, so captions cannot turn reset polarity evidence into a handshake role.
The companion `vlm_timing_active_low_deassertion_equivalence_gold` fixture proves the reset-release mirror case: `deasserted` and `HIGH` are equivalent for the same active-low reset.
That deassertion fixture now carries the same zero-hint guard, so reset-release prose stays temporal/polarity evidence only.
It also locks the total semantic-hint count at zero, matching the assertion fixture's no-role-hint boundary.
The deassertion fixture now checks zero table-sourced semantic hints too, keeping reset-release evidence out of table-driven role recovery.
Ordinary prose hints are zero there as well, so text around reset release cannot backfill a handshake role.
Alias-grounded prose is excluded too, preventing phrase-alias recovery from inventing a reset-release semantic role.
Visual-caption hints are zero too, matching the assertion-side caption exclusion for reset-release evidence.
That matters because `ASSERTED` and `DEASSERTED` are polarity-relative, not synonyms for fixed logic levels.

Timing-annotation negative fixtures also protect against visual overreach.
Low-value labels such as `T0`, `Addr 1`, `Cycle 2`, `D0`, `A1`, `DATA0`, `0xAA`, `D[0]`, `A[1]`, `DATA[3]`, and `ADDR[7]` do not become timing constraints, and waveform motion states such as `rising`, `stable`, `falling`, `UNCHANGED`, `RISING_EDGE`, `LOW_TO_HIGH`, `POS_EDGE`, `risingedge`, and `LOW2HIGH` do not become symbolic signal values.
The waveform-motion fixture now locks total semantic hints at zero too, so rejected motion states cannot surface as semantic-role hints through another path.
It also excludes table-sourced semantic hints, so table evidence cannot rescue rejected motion-state labels.
Ordinary prose hints are zero for waveform-motion labels too, so text evidence cannot backfill a semantic role for rejected motion states.
Alias-grounded prose is excluded as well, keeping phrase-alias recovery from inventing roles for rejected motion-state labels.
Visual-caption hints are zero too, so captions cannot backfill a semantic role for rejected motion states.
The spurious-annotation fixture now also locks total semantic hints at zero, so low-value labels and value-like annotations cannot invent roles through another evidence channel.
It also excludes table-sourced semantic hints, so low-value annotation noise cannot be rescued through table evidence.
Ordinary prose hints are zero for spurious annotations too, so text evidence cannot backfill a semantic role for label/value noise.
Alias-grounded prose is excluded as well, keeping phrase-alias recovery from inventing roles for low-value annotation noise.
Visual-caption hints are zero too, so captions cannot backfill a role for low-value annotation noise.
The indexed signal-value fixture carries the same total zero-hint guard for labels such as `XREQ[0] HIGH`.
It now also excludes table-sourced semantic hints, keeping indexed label noise from being rescued through table evidence.
Ordinary prose hints are zero for indexed labels too, so text evidence cannot backfill a role for bit-select label noise.
Alias-grounded prose is excluded as well, keeping phrase-alias recovery from inventing roles for indexed label noise.
Visual-caption hints are zero too, so captions cannot backfill a role for bit-select label noise.
The cycle-qualified signal-value fixture carries that guard for labels such as `XREQ HIGH at T1`.
It now also excludes table-sourced semantic hints, so cycle-scoped label noise cannot be rescued through table evidence.
Ordinary prose hints are zero too, keeping text evidence from inventing a role for cycle-scoped label noise.
Alias-grounded prose is excluded too, preventing phrase-alias recovery from turning cycle-scoped labels into roles.
Visual-caption hints are zero as well, keeping captions from backfilling roles for cycle-scoped label noise.
Motion-only annotation prose such as `XREQ rises, remains stable, then falls` is filtered the same way unless it carries real timing/constraint indicators.
The motion-only fixture now locks total semantic hints at zero, so motion prose cannot invent valid/ready-like roles through another evidence channel.
It also excludes table-sourced semantic hints, so table evidence cannot rescue rejected motion-only prose.
Ordinary prose hints are zero too, keeping text evidence from inventing roles for rejected motion-only prose.
Alias-grounded prose is excluded too, preventing phrase-alias recovery from inventing roles for rejected motion-only prose.
Visual-caption hints are zero as well, so captions cannot backfill roles for rejected motion-only prose.
Concrete sampled values such as `HIGH` can still become typed temporal evidence when the signal itself is document-grounded.

State-machine visual fixtures protect the FSM side of the same boundary.
`vlm_state_machine_label_noise_negative` proves identifier labels such as `IDLE` and `BUSY` survive into canonical state records and transition endpoints, while prose/OCR labels such as `IDLE state` and `ACCESS phase` stay out.
That label-noise fixture now also checks canonical initial-state shape, proving accepted `IDLE` stays initial and `BUSY` stays non-initial after prose/OCR label filtering.
`vlm_state_machine_undeclared_transition_negative` adds the graph-grounding guard: transition endpoints such as `DONE` and `RESET` stay out unless the same VLM observation also declared them as accepted states.
That undeclared-transition fixture now also checks the canonical initial-state shape, proving accepted `IDLE` stays initial and `BUSY` stays non-initial after endpoint filtering.
`vlm_state_machine_duplicate_initial_gold` locks the complementary positive case: duplicate state labels are merged, a later `is_initial: true` marker still makes `IDLE` the canonical initial state, and validation reports exactly one `initial_regular_states` record.
That duplicate-initial gold now also excludes the initial-cardinality validation findings directly, proving the merge path stays warning-clean.
`vlm_state_machine_multiple_initial_negative` locks the companion warning path: when VLM marks more than one canonical state initial, the graph remains inspectable but semantic and intent validation report state-machine initial-cardinality findings.
That multiple-initial fixture now also checks exact warning payloads at both canonical stages, including the `state_machine` category and related `IDLE`/`BUSY` ids.
`vlm_state_machine_missing_initial_negative` locks the other side of the same warning path: when VLM marks no canonical state initial, states and transitions remain inspectable while validation still reports unsafe initial-state cardinality.
That missing-initial fixture now also checks exact warning payloads at both canonical stages, including the `state_machine` category and related `IDLE`/`BUSY` ids.

The polarity fixture family also covers non-reset controls.
Gold fixtures prove explicit asserted-when-level prose, collective active-level prose, and safe clause-local mixed prose can recover active-low or active-high facts.
The detached mixed-polarity negative fixture proves a phrase like `CS_N is active LOW and active HIGH` stays unresolved instead of borrowing an implicit subject.

The infrastructure fixture family now includes explicit clock/reset topology coverage too.
`clock_reset_topology_gold` proves that current-document clock-gate, reset-synchronizer, and reset-tree phrases become typed infrastructure topology.
`clock_reset_contract_scope_negative` proves that protocol-PDF contract language can mention RTL/VIP scope and integration-owned physical clock/reset trees without authoring topology records.
`clock_reset_generic_advice_negative` proves the companion guard: generic guidance such as glitch-avoidance advice, no-glue reset-tree advice, possible synchronizer usage, or async-assert/sync-release discipline does not add topology records or ordinary protocol actor ports without current-document topology evidence.

## Practical role in the project

If `validate` tells us how strong one artifact is, `kg-bench` tells us whether the extraction logic is still behaving correctly across targeted truthfulness cases.

In practice, it is one of the main ways `specforge` stays honest as the pipeline grows more capable.
Fixture-local validation checks run through the same validator as `specforge validate`, but the harness suppresses the full validation report text so benchmark output stays focused on fixture pass/fail status.

The current fixture run can also be projected into the corpus knowledge base:

```bash
cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality
```

That projection writes `corpus_kb/benchmarks/kg-fixtures.md`.
It is a reviewable synthesis page, not a replacement for the executable `kg-bench` gate.
It also includes a fixture-family summary table so coverage by truthfulness family is visible beside the per-fixture pass/fail list.
The same refresh now populates dedicated semantic/truthfulness pattern, typed-prior-memory, table, visual, state-machine, timing, infrastructure/polarity, and AMBA-family corpus-KB pages from the same fixture outcomes.
It also populates a review-only prior-candidate page plus a JSON readiness manifest with family-level schema, fixture, harvest, and consumer gate rows; those artifacts can guide future typed-prior work but cannot mutate `CorpusMemory`.
