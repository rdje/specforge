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

Fixtures can assert canonical temporal rules directly too.
`temporal_rules_include` checks typed clock/edge grounding, cycle windows, supporting statement ids, antecedent predicates, and consequent predicates.
That lets APB/AHB/AXI timing fixtures prove actor-grounded drive or stability predicates, compound guards, and `HandshakeComplete` predicates as IR shape rather than only as validation-count side effects.

Fixtures can assert canonical temporal conflicts directly too.
`temporal_conflicts_include` checks the contradiction signal, phase, clock/edge context, antecedent predicates, conflicting values, supporting rule ids, and supporting statement ids.
That keeps conflict-surfacing benchmarks honest: a prior can add caution guidance, but it cannot make the current-document contradiction disappear.

Fixtures can assert canonical signal-semantic conflicts directly as well.
`signal_semantic_conflicts_include` checks the conflicted signal and included observations by semantic tag, source kind, source text, and supporting evidence ids.
That lets multimodal disagreement fixtures prove, for example, that visual-caption evidence and VLM timing-diagram annotation evidence support different handshake roles without reducing the test to "one conflict exists."

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
