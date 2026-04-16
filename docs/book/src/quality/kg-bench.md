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

This lets the project protect individual truthfulness properties instead of relying only on broad integration runs.

## Why fixtures matter

Without fixture coverage, the pipeline can drift in subtle ways:

- a false actor might quietly reappear
- a semantic role might start resolving from name noise again
- a learned prior might start overreaching
- a table-shape rule might begin misclassifying field tables as real signal tables

`kg-bench` exists to catch exactly that kind of drift.

## Graph-Backed Direction Expectations

Actor-relative direction is a graph surface, not just a flat signal annotation.

Fixtures can therefore assert graph direction coverage directly with `graph_direction_signal_names_include` and `graph_direction_signal_names_exclude`.
Those fields inspect canonical `actor_ports` for non-`unknown` direction evidence by signal name.

This is intentionally separate from `signal_directions_include`.
The older field still checks the flat compatibility `direction_hint` on interface signal records.
Keeping both surfaces separate prevents the harness from flattening producer/consumer actor roles into a fake single perspective while still letting graph-first recovery be tested explicitly.

Fixtures can assert canonical signal-connectivity conflicts directly.
`signal_connectivity_conflicts_include` checks the conflicted signal, conflict kind, conflicting actor ids or names, and optional supporting statement ids.
That lets graph-conflict fixtures prove the actual multi-producer actor set rather than reducing the test to "one connectivity conflict exists."

Fixtures can assert canonical temporal rules directly too.
`temporal_rules_include` checks typed clock/edge grounding, cycle windows, supporting statement ids, antecedent predicates, and consequent predicates.
That lets APB/AHB/AXI timing fixtures prove actor-grounded drive or stability predicates, compound guards, and `HandshakeComplete` predicates as IR shape rather than only as validation-count side effects.
The tracked suite now includes AXI write-address, write-address-id-stability, write-response, write-response-id-stability, read-address, read-address-id-stability, read-address-sideband-stability, read-data, read-data-id-stability, read-data-response-stability, read-data-last-stability, write-data, write-data-last-stability, sideband-stability, and write-address-sideband-stability timing gold paths, so the same width-only table plus prose actor-relation pattern is checked across `AWVALID` / `AWREADY` / `AWADDR` / `AWID`, `BVALID` / `BREADY` / `BRESP` / `BID`, `ARVALID` / `ARREADY` / `ARADDR` / `ARLEN` / `ARID`, `RVALID` / `RREADY` / `RDATA` / `RRESP` / `RID`, `WVALID` / `WREADY` / `WDATA` / `WSTRB`, explicit sideband hold obligations for `ARLEN` and `WSTRB`, read-address sideband holds for `ARSIZE` and `ARBURST`, write-address sideband holds for `AWLEN`, `AWSIZE`, and `AWBURST`, manager-owned write-address ID stability for `AWID`, manager-owned read-address ID stability for `ARID`, subordinate-owned write-response ID stability for `BID`, subordinate-owned read-data ID stability for `RID`, subordinate-owned read-data response stability for `RRESP`, subordinate-owned read-data last-beat stability for `RLAST`, and manager-owned write-data last-beat stability for `WLAST`.
The APB timing family now also includes a write-control stability path: `PWRITE`, `PWDATA`, and `PSTRB` must remain requester-owned stable outputs while `PSEL` and `PENABLE` are high and `PREADY` is low, without treating that wait-state guard as a completed handshake.
Its response-stability mirror checks the completed-access side: `PRDATA` and `PSLVERR` must remain completer-owned stable outputs when `PSEL`, `PENABLE`, and `PREADY` are high, and the same rule must preserve `HandshakeComplete(PSEL, PREADY)`.
The AHB timing family now includes a control-stability wait-state path too: `HADDR`, `HWRITE`, `HSIZE`, `HBURST`, and `HPROT` must remain manager-owned stable outputs while `HREADY` is low and `HSEL` is high, with no false handshake-completion predicate.
Its response-side mirror checks `HRDATA` and `HRESP` as subordinate-owned stable outputs under the same `HREADY LOW` / `HSEL HIGH` wait-state guard, again without promoting the stalled transfer to handshake completion.
The write-data path adds the data-phase guard: `HWDATA` must remain manager-owned stable while `HREADY` is low, `HSEL` is high, and `HWRITE` is high, proving three-predicate wait-state stability without a false completion event.

Fixtures can assert canonical temporal conflicts directly too.
`temporal_conflicts_include` checks the contradiction signal, phase, clock/edge context, antecedent predicates, conflicting values, supporting rule ids, and supporting statement ids.
That keeps conflict-surfacing benchmarks honest: a prior can add caution guidance, but it cannot make the current-document contradiction disappear.

Fixtures can assert canonical signal-semantic conflicts directly as well.
`signal_semantic_conflicts_include` checks the conflicted signal and included observations by semantic tag, source kind, source text, and supporting evidence ids.
That lets multimodal disagreement fixtures prove, for example, that visual-caption evidence and VLM timing-diagram annotation evidence support different handshake roles without reducing the test to "one conflict exists."

Fixtures can assert canonical signal-polarity conflicts directly as well.
`signal_polarity_conflicts_include` checks the conflicted signal and included observations by polarity, source kind, and supporting statement or table ids.
That lets polarity-disagreement fixtures prove that active-high prose and active-low table evidence both survived into the canonical conflict surface, rather than only counting that some polarity conflict exists.

Fixtures can assert canonical resolved signal polarity directly as well.
`signal_polarities_include` checks the signal name and expected active-high or active-low polarity on the canonical interface signal record.
That lets polarity gold fixtures prove `CS_N` is active-low or `ENABLE` is active-high as typed IR shape, rather than only proving that the number of resolved-polarity signals increased.

Fixtures can assert canonical resolved semantic roles directly as well.
`resolved_semantic_roles_include` checks the signal name and expected valid-like or ready-like role on the canonical interface signal record.
That lets semantic gold fixtures prove `XREQ` is valid-like or `XACK` is ready-like as typed IR shape, rather than only proving that some semantic role was resolved.

Fixtures can assert canonical semantic grounding strength directly as well.
`semantic_grounding_strengths_include` checks the signal name and expected single-source, multi-source, or cross-modality grounding strength on the canonical interface signal record.
That lets semantic gold fixtures prove a multimodal `XREQ` role is really cross-modality grounded, or a VLM-only `XACK` role is really single-source grounded, rather than only proving that a validation counter changed.

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

## Prior-guided fixtures

The benchmark harness can also stage fixture-local `CorpusMemory`.

That matters because the learning plane must be tested with the same discipline as the document pipeline:

- without the prior, the unseen local phrase or table should stay unresolved
- with the matching prior, the current document should recover the meaning locally
- the prior must widen interpretation, not author facts on its own
- caution priors must not suppress local conflicts or residuals

Negative-knowledge fixtures make that last rule executable.
The suite now has prior-guided caution fixtures for signal-semantic conflicts, temporal conflicts, residual packets, signal-connectivity conflicts, and interface-signal conflicts.
Those fixtures require the matched local conflict or residual to remain present while validation only adds `negative_knowledge_prior_matches`, rescan recommendations, corroboration requirements, and stage-specific rescan-guidance findings.

Timing-annotation fixtures also cover polarity-sensitive multimodal evidence.
For example, `vlm_timing_active_low_assertion_equivalence_gold` proves that an active-low reset observed by a VLM timing diagram as both `asserted` and `LOW` becomes typed temporal evidence without creating a false temporal conflict.
The companion `vlm_timing_active_low_deassertion_equivalence_gold` fixture proves the reset-release mirror case: `deasserted` and `HIGH` are equivalent for the same active-low reset.
That matters because `ASSERTED` and `DEASSERTED` are polarity-relative, not synonyms for fixed logic levels.

Timing-annotation negative fixtures also protect against visual overreach.
Low-value labels such as `T0`, `Addr 1`, `Cycle 2`, `D0`, `A1`, `DATA0`, `0xAA`, `D[0]`, `A[1]`, `DATA[3]`, and `ADDR[7]` do not become timing constraints, and waveform motion states such as `rising`, `stable`, `falling`, `UNCHANGED`, `RISING_EDGE`, `LOW_TO_HIGH`, `POS_EDGE`, `risingedge`, and `LOW2HIGH` do not become symbolic signal values.
Motion-only annotation prose such as `XREQ rises, remains stable, then falls` is filtered the same way unless it carries real timing/constraint indicators.
Concrete sampled values such as `HIGH` can still become typed temporal evidence when the signal itself is document-grounded.

State-machine visual fixtures protect the FSM side of the same boundary.
`vlm_state_machine_label_noise_negative` proves identifier labels such as `IDLE` and `BUSY` survive into canonical state records and transition endpoints, while prose/OCR labels such as `IDLE state` and `ACCESS phase` stay out.
`vlm_state_machine_undeclared_transition_negative` adds the graph-grounding guard: transition endpoints such as `DONE` and `RESET` stay out unless the same VLM observation also declared them as accepted states.
`vlm_state_machine_duplicate_initial_gold` locks the complementary positive case: duplicate state labels are merged, a later `is_initial: true` marker still makes `IDLE` the canonical initial state, and validation reports exactly one `initial_regular_states` record.
`vlm_state_machine_multiple_initial_negative` locks the companion warning path: when VLM marks more than one canonical state initial, the graph remains inspectable but semantic and intent validation report state-machine initial-cardinality findings.
`vlm_state_machine_missing_initial_negative` locks the other side of the same warning path: when VLM marks no canonical state initial, states and transitions remain inspectable while validation still reports unsafe initial-state cardinality.

The polarity fixture family also covers non-reset controls.
Gold fixtures prove explicit asserted-when-level prose, collective active-level prose, and safe clause-local mixed prose can recover active-low or active-high facts.
The detached mixed-polarity negative fixture proves a phrase like `CS_N is active LOW and active HIGH` stays unresolved instead of borrowing an implicit subject.

The infrastructure fixture family now includes explicit clock/reset topology coverage too.
`clock_reset_topology_gold` proves that current-document clock-gate, reset-synchronizer, and reset-tree phrases become typed infrastructure topology, while generic guidance such as glitch-avoidance advice or possible synchronizer usage does not add extra topology records or ordinary protocol actor ports.

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
