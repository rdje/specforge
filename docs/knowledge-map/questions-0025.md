# Knowledge questions — shard 0025

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound](../decisions/0027-fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound.md)
  > why is 198 exactly the largest fact-card maximum the current projection allows
- [a-register-width-is-stated-never-inferred](../knowledge/a-register-width-is-stated-never-inferred.md)
  > why is ADIv6 ABORT emitted 5 bits wide
- [a-register-field-binding-to-a-wire](../knowledge/a-register-field-binding-to-a-wire.md)
  > why is ADIv6 Table B2-7's bit-to-signal mapping not read
- [decision-capacity-is-rederived-without-moving-stable-records](../decisions/0041-decision-capacity-is-rederived-without-moving-stable-records.md)
  > why is ADR 0038 not split or rewritten
- [isf-parameter-width-contract](../knowledge/isf-parameter-width-contract.md)
  > why is AHB ADDR_WIDTH declared as 10
- [an-operation-kind-a-level-states-is-a-guard](../knowledge/an-operation-kind-a-level-states-is-a-guard.md)
  > why is AHB's HWRITE direction sentence not read
- [property-table-is-not-a-signal-inventory](../knowledge/property-table-is-not-a-signal-inventory.md)
  > why is APB Check signal descriptions not refused as a property table
- [isf-value-width-operand-contract](../knowledge/isf-value-width-operand-contract.md)
  > why is ATID emitted (width 1) when the IntentIR grounds width 7 (the emitter's first-seen signal dedup isf_ir.rs:696-700 takes the first signal_records entry (w=None→1) and skips the later w=7 record; the .2a.i recovery only falls back to actor_ports, and ATID has none)
- [a-condition-never-contains-its-obligation](../knowledge/a-condition-never-contains-its-obligation.md)
  > why is AWLOCK no longer driven in AXI manager.isf (For Atomic transactions, the following restrictions apply:)
- [enum-member-name-column-needs-adjudication-not-a-score](../knowledge/enum-member-name-column-needs-adjudication-not-a-score.md)
  > why is AWSNOOP Table A7.3 not published as an enum
- [subject-resolution-admits-a-width-verified-full-width-slice](../decisions/0047-subject-resolution-admits-a-width-verified-full-width-slice.md)
  > why is AWSNOOP[3] not resolved to AWSNOOP
- [a-section-defining-the-documents-notation-is-apparatus](../knowledge/a-section-defining-the-documents-notation-is-apparatus.md)
  > why is AXI 'Pseudocode description of the transfers' not apparatus (pseudocode put to use)
- [a-hold-keeps-its-release](../knowledge/a-hold-keeps-its-release.md)
  > why is AXI AWAKEUP's hold guard unchanged (the condition already excludes the release)
- [agent-pure-inferred-phantom-drop](../knowledge/agent-pure-inferred-phantom-drop.md)
  > why is AXI transmitter / SWD host / GIC arbiter kept but APB controller / AHB agent dropped
- [assume-sentences-fall-into-six-shapes](../knowledge/assume-sentences-fall-into-six-shapes.md)
  > why is AXI's 'CMO transactions are assumed to be Non-shareable' not AWDOMAIN's absent value (it holds for a class of requests only)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > why is AXI/SWD per-signal phase membership empty (document absence — AXI 0/4, SWD 0/63 signal↔phase co-occurrence; phases described abstractly/by-packet, not per declared wire — not an extraction gap)
- [a-document-stated-collective-name-stands-for-its-members](../knowledge/a-document-stated-collective-name-stands-for-its-members.md)
  > why is AxSNOOP's absent value bound to AWSNOOP and ARSNOOP now
- [corpus-reuse-activate-only-no-current-consumer](../knowledge/corpus-reuse-activate-only-no-current-consumer.md)
  > why is CORPUS-PATTERN-REUSE.3b.3b (activate-only consume) deferred / not built
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > why is CPU-ISA / PHY only recognized from front-matter (no distinct structural signature; cat 5 vs cat 6 indistinguishable by structure)
- [isf-parameter-width-contract](../knowledge/isf-parameter-width-contract.md)
  > why is DATA_WIDTH declared as 8 in the AXI isf
- [semantic-interface-authority-empty-fallback](../knowledge/semantic-interface-authority-empty-fallback.md)
  > why is DTI DOWNSTREAM not preserved as a heuristic only wire
- [a-width-offset-packet-layout-reaches-message-field-records](../knowledge/a-width-offset-packet-layout-reaches-message-field-records.md)
  > why is Delayed (DL) named DL
- [a-gate-doctrine-has-no-stable-wall-clock-on-this-host](../knowledge/a-gate-doctrine-has-no-stable-wall-clock-on-this-host.md)
  > why is FAST_EXCLUDE membership quoted but never a share
- [agent-surface-defect-taxonomy](../knowledge/agent-surface-defect-taxonomy.md)
  > why is For / Then it / is recommended / next / HPROT bit / TREADY input minted as an actor
- [a-storage-or-value-noun-heads-no-agent](../knowledge/a-storage-or-value-noun-heads-no-agent.md)
  > why is HPROT bit not an actor
- [table-row-obligation-binds-to-the-token-before-its-modal](../knowledge/table-row-obligation-binds-to-the-token-before-its-modal.md)
  > why is HPROT constrained to 0 in the persisted AHB evidence
- [enum-member-name-column-needs-adjudication-not-a-score](../knowledge/enum-member-name-column-needs-adjudication-not-a-score.md)
  > why is HSIZE BYTE equal to HALFWORD
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > why is KG-ISF-TRANSACTIONS.2i parked/blocked (waiting on FSMGEN to say how to lower phase membership without fabricating drive values or step order)
- [a-dropped-declaration-row-is-usually-not-a-signal](../knowledge/a-dropped-declaration-row-is-usually-not-a-signal.md)
  > why is LOOP_W_WIDTH not declared as a signal
- [a-width-cell-that-is-a-sentence-is-not-a-width](../knowledge/a-width-cell-that-is-a-sentence-is-not-a-width.md)
  > why is LTI_GPC == True ? 2:1 not refused as prose
- [source-ir-size-scaling](../knowledge/source-ir-size-scaling.md)
  > why is MEMORY-BOUNDED-INGEST.5 summary streaming deferred
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > why is NVMe register field_name a bit-range and how is the mnemonic found in the description
- [a-hint-role-counts-only-for-the-signal-that-bears-it](../knowledge/a-hint-role-counts-only-for-the-signal-that-bears-it.md)
  > why is PNSE must be valid when PSEL is asserted not a valid-like hint
- [declared-spelling-is-the-document-spelling](../knowledge/declared-spelling-is-the-document-spelling.md)
  > why is PSELx not affected by the interior lower-case rule
- [alpha-variant-placeholder-is-not-a-wire](../knowledge/alpha-variant-placeholder-is-not-a-wire.md)
  > why is PSELx not treated as a metavariable
- [protocol-state-machine-binding](../knowledge/protocol-state-machine-binding.md)
  > why is ProtocolStateRecord machine_name always None
- [constraint-scope-carrier-is-lost](../knowledge/constraint-scope-carrier-is-lost.md)
  > why is RACK must be LOW published without during reset
- [arithmetic-width-drops-the-declaration](../knowledge/arithmetic-width-drops-the-declaration.md)
  > why is RUSERCHK missing from the AXI catalog
- [legacy-generic-section-phases-are-audit-only](../knowledge/legacy-generic-section-phases-are-audit-only.md)
  > why is Reset value not a semantic phase
- [a-layouts-reserved-space-is-a-typed-span-citing-the-keyword](../knowledge/a-layouts-reserved-space-is-a-typed-span-citing-the-keyword.md)
  > why is Route String/Reserved, RSVDC or RsvdZero not a reserved span (is_reserved_field_name)
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > why is SWD per-signal phase membership degenerate / empty
- [swd-frame-phase-binding-lives-in-the-figure](../knowledge/swd-frame-phase-binding-lives-in-the-figure.md)
  > why is SWD serial_frame_field left at 0/11 instead of being partially recovered (because every candidate rule that fires often enough to help also mis-assigns the phase on most fields, and a wrong phase is fabrication. A measured zero is the honest result; WIRE-BASED-100.8d is deferred with that consequence stated)
- [swd-adi-not-signal-table-spec](../knowledge/swd-adi-not-signal-table-spec.md)
  > why is SWD/ADI hard / different from APB AHB AXI
- [definitional-signal-capture](../knowledge/definitional-signal-capture.md)
  > why is SWIO not captured as a third SWP signal
- [spec-mining-framing](../knowledge/spec-mining-framing.md)
  > why is SpecForge called forward specification mining
- [agent-interface-block-consolidation](../knowledge/agent-interface-block-consolidation.md)
  > why is Subordinate interface merged to Subordinate but GIC CPU interface kept intact
- [a-signal-from-one-actor-to-another-names-both-ends](../knowledge/a-signal-from-one-actor-to-another-names-both-ends.md)
  > why is TCK / SWCLK is the Test Clock signal from ... refused
- [task-tree-catalog](../knowledge/task-tree-catalog.md)
  > why is TEMPLATE excluded from the task catalog
- [logic-level-walk-stops-at-eleven-unrelated-words](../knowledge/logic-level-walk-stops-at-eleven-unrelated-words.md)
  > why is TLAST cannot be tied LOW not extracted
- [an-untyped-signal-table-is-read-by-its-direction-cells](../knowledge/an-untyped-signal-table-is-read-by-its-direction-cells.md)
  > why is TMC's memory interface table not read
- [field-value-table-fills-register-field](../knowledge/field-value-table-fills-register-field.md)
  > why is Table C2-2 ERRNPASS ERRSTOP not read as field values
- [a-hint-role-counts-only-for-the-signal-that-bears-it](../knowledge/a-hint-role-counts-only-for-the-signal-that-bears-it.md)
  > why is Valid indicator a valid-like hint for AWVALID even though AXI declares a signal VALID
- [parenthetical-data-head-requires-wire-qualifier](../knowledge/parenthetical-data-head-requires-wire-qualifier.md)
  > why is Vital Product Data (VPD) not a signal
- [arithmetic-width-drops-the-declaration](../knowledge/arithmetic-width-drops-the-declaration.md)
  > why is WSTRB missing from AXI's SemanticIR interface catalog while WSTRBCHK is in it
- [a-hint-role-counts-only-for-the-signal-that-bears-it](../knowledge/a-hint-role-counts-only-for-the-signal-that-bears-it.md)
  > why is WSTRB or RCHUNKSTRB not valid-like (indicates which byte lanes, a choice)
- [parenthetical-data-head-requires-wire-qualifier](../knowledge/parenthetical-data-head-requires-wire-qualifier.md)
  > why is Wishbone memory output data (DO) not a port
- [section-header-register-block-qualification](../knowledge/section-header-register-block-qualification.md)
  > why is a CSW@MEM-AP / CLAIMSET@AP qualified register name emitted, and is it .isf-safe
- [a-section-defining-the-documents-notation-is-apparatus](../knowledge/a-section-defining-the-documents-notation-is-apparatus.md)
  > why is a Conventions, Typographic conventions, RTL notation or Signal Naming Conventions section apparatus
- [a-layouts-reserved-space-is-a-typed-span-citing-the-keyword](../knowledge/a-layouts-reserved-space-is-a-typed-span-citing-the-keyword.md)
  > why is a Reserved row of a packet or descriptor layout not a message field
- [subject-resolution-admits-a-width-verified-full-width-slice](../decisions/0047-subject-resolution-admits-a-width-verified-full-width-slice.md)
  > why is a bit slice not a spelling heuristic
- [bounded-task-root-accretes-in-its-frontier](../knowledge/bounded-task-root-accretes-in-its-frontier.md)
  > why is a bounded active root near its line ceiling
- [declared-population-is-not-the-candidate-row-population](../knowledge/declared-population-is-not-the-candidate-row-population.md)
  > why is a comma-separated signal family safe when a space-separated one is not
- [a-split-name-cell-does-not-say-which-join-it-wants](../knowledge/a-split-name-cell-does-not-say-which-join-it-wants.md)
  > why is a concatenating join more dangerous than an underscore join
- [a-condition-never-contains-its-obligation](../knowledge/a-condition-never-contains-its-obligation.md)
  > why is a condition like 'X is asserted, Y must remain asserted' wrong
- [semantic-grounding-filter-is-catalog-independent](../knowledge/semantic-grounding-filter-is-catalog-independent.md)
  > why is a conditional rule in EvidenceIR but not in SemanticIR
- [temporal-condition-read-whole](../knowledge/temporal-condition-read-whole.md)
  > why is a conditional rule with an unread condition listed instead of emitted
- [evidence-statement-markdown-escape-truncates-identifiers](../knowledge/evidence-statement-markdown-escape-truncates-identifiers.md)
  > why is a constraint subject CLK when the document says CLK_I
- [enum-member-name-column-needs-adjudication-not-a-score](../knowledge/enum-member-name-column-needs-adjudication-not-a-score.md)
  > why is a content score not enough to pick the member name column
- [decimal-before-binary-needs-a-notational-witness](../knowledge/decimal-before-binary-needs-a-notational-witness.md)
  > why is a corpus cell census not a measure of a producer defect
- [one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample](../knowledge/one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample.md)
  > why is a debug probe 14 seconds and a release probe 1.2
- [published-assertion-gate](../knowledge/published-assertion-gate.md)
  > why is a digest binding not enough to watch a number in prose
- [fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy](../decisions/0029-fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy.md)
  > why is a fact-card aggregate ceiling the file bound times the per-file bound
- [bit-location-register-field-vocabulary](../knowledge/bit-location-register-field-vocabulary.md)
  > why is a field name accepted or rejected from a description cell
- [captured-region-residual-carrier](../knowledge/captured-region-residual-carrier.md)
  > why is a figure caption not coverage for the figure
- [a-bounded-snapshot-bounds-its-sections-not-just-its-file](../decisions/0031-a-bounded-snapshot-bounds-its-sections-not-just-its-file.md)
  > why is a file-level line ceiling not enough for a bounded snapshot
- [published-assertion-gate](../knowledge/published-assertion-gate.md)
  > why is a gate's own numeral grammar part of its contract
- [llm-primary-condition-subject-gate](../knowledge/llm-primary-condition-subject-gate.md)
  > why is a gerund after while/when not a condition (action coordination)
- [published-assertion-gate](../knowledge/published-assertion-gate.md)
  > why is a glob list not enough to scope a claim gate
- [doctrine-driver-runs-no-cargo-gate](../knowledge/doctrine-driver-runs-no-cargo-gate.md)
  > why is a green doctrine report not evidence that the branch builds cleanly
- [qualified-role-header-proves-no-role](../knowledge/qualified-role-header-proves-no-role.md)
  > why is a green wire-protocol score not evidence that extraction is intact
- [a-dropped-declaration-row-is-usually-not-a-signal](../knowledge/a-dropped-declaration-row-is-usually-not-a-signal.md)
  > why is a grounding test against the declared catalog circular
- [a-frozen-qualification-population-is-a-subset-floor-not-an-equality](../decisions/0050-a-frozen-qualification-population-is-a-subset-floor-not-an-equality.md)
  > why is a growing list kept out of a frozen contract
- [self-test-coverage-guard-is-in-the-exit-path](../knowledge/self-test-coverage-guard-is-in-the-exit-path.md)
  > why is a hardcoded self-test count worse than a tautological one
- [a-hold-keeps-its-release](../knowledge/a-hold-keeps-its-release.md)
  > why is a hold listed with 'the hold's release … was not read'
- [a-keyword-definition-is-typed-with-the-obligations-it-states](../knowledge/a-keyword-definition-is-typed-with-the-obligations-it-states.md)
  > why is a keyword definition apparatus but still typed (ADR 0033 decision 6)
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > why is a membership-derived (sample) body NOT faithful even though it is FSMGen-accepted
- [declaration-replay-reads-the-legacy-stratum](../knowledge/declaration-replay-reads-the-legacy-stratum.md)
  > why is a missing-signal census over generated/ not a statement about the current reader
- [isf-rule-transaction-priority-authority](../knowledge/isf-rule-transaction-priority-authority.md)
  > why is a multi-caller named drive kept without actor priority
- [partitioning-a-task-tree-has-a-fixed-registration-price](../knowledge/partitioning-a-task-tree-has-a-fixed-registration-price.md)
  > why is a partitioned tree's archived capsule not counted in expected_current_surfaces (its state is terminal, so only index/parts/route_parts are current)
- [dotted-cross-reference-subject-gate](../knowledge/dotted-cross-reference-subject-gate.md)
  > why is a pure-hex-literal subject filter unsafe (CBA, BADD)
- [quoted-literal-notation-carries-the-radix](../knowledge/quoted-literal-notation-carries-the-radix.md)
  > why is a quoted binary literal not parsed
- [subject-phrase-that-swallowed-its-verb-is-refused](../knowledge/subject-phrase-that-swallowed-its-verb-is-refused.md)
  > why is a refused subject phrase not re-attributed to an earlier determiner
- [normalized-bundle-retention-is-declared](../knowledge/normalized-bundle-retention-is-declared.md)
  > why is a retained bundle count not the same as corpus refresh progress
- [reviewed-residual-gold-key-law](../knowledge/reviewed-residual-gold-key-law.md)
  > why is a reviewed residual cell unmet no matter what production does
- [legacy-generic-gates-are-audit-only](../knowledge/legacy-generic-gates-are-audit-only.md)
  > why is a sentence containing when not necessarily a semantic gate
- [live-surface-edit-bookkeeping-chain](../knowledge/live-surface-edit-bookkeeping-chain.md)
  > why is a shipped_behavior aggregate_change rationale rejected (it is capped at 512 bytes; the whole surfaces.jsonl record is bounded by max_scalar_bytes)
- [a-hint-role-counts-only-for-the-signal-that-bears-it](../knowledge/a-hint-role-counts-only-for-the-signal-that-bears-it.md)
  > why is a signal not valid-like when the sentence says valid data (presents valid data on its DAT_O lines)
- [a-hint-role-counts-only-for-the-signal-that-bears-it](../knowledge/a-hint-role-counts-only-for-the-signal-that-bears-it.md)
  > why is a signal tagged handshake_valid_like or handshake_ready_like
- [timing-observation-to-verified-figure-contract](../knowledge/timing-observation-to-verified-figure-contract.md)
  > why is a stable span after tick zero residual
- [unread-intent-records-fall-into-six-families](../knowledge/unread-intent-records-fall-into-six-families.md)
  > why is a stated default like the DTF value assumed 0 an assumption
- [base-name-template-table-is-not-a-catalogue](../knowledge/base-name-template-table-is-not-a-catalogue.md)
  > why is a suffix-of-other-declared-names rule not enough to detect a naming template
- [captured-region-residual-carrier](../knowledge/captured-region-residual-carrier.md)
  > why is a table listed uncarried though its rows produced constraints
- [an-absent-value-may-be-a-label-and-hold-for-a-class](../knowledge/an-absent-value-may-be-a-label-and-hold-for-a-class.md)
  > why is a table row not read as an absent-value sentence
- [a-hint-role-counts-only-for-the-signal-that-bears-it](../knowledge/a-hint-role-counts-only-for-the-signal-that-bears-it.md)
  > why is a table whose first column is headed Code or Key not a signal inventory (table_keys_codes_rather_than_signals, EXTRACTION-REPRESENTATION-GAPS.53)
- [unattached-table-captions](../knowledge/unattached-table-captions.md)
  > why is a table's caption_text null when the caption is printed on the page
- [claim-verification-task-evidence-migrated](../knowledge/claim-verification-task-evidence-migrated.md)
  > why is a task-evidence part allowed to own several non-adjacent source regions
- [per-row-column-drift-rule](../knowledge/per-row-column-drift-rule.md)
  > why is a tie between two direction columns not an anchor
- [timing-table-structural-authority](../knowledge/timing-table-structural-authority.md)
  > why is a timeout value a typ value
- [a-timeouts-next-state-is-a-stated-transition](../knowledge/a-timeouts-next-state-is-a-stated-transition.md)
  > why is a timer's expiry not a temporal rule or a StateTransitionRecord
- [assume-sentences-fall-into-six-shapes](../knowledge/assume-sentences-fall-into-six-shapes.md)
  > why is a timing-diagram key sentence an IntentIR assumption
- [subject-phrase-that-swallowed-its-verb-is-refused](../knowledge/subject-phrase-that-swallowed-its-verb-is-refused.md)
  > why is a two-word relation actor ending in an -s word refused
- [a-layouts-reserved-space-is-a-typed-span-citing-the-keyword](../knowledge/a-layouts-reserved-space-is-a-typed-span-citing-the-keyword.md)
  > why is a value sub-row 0000b | Reserved not a reserved span
- [arithmetic-width-drops-the-declaration](../knowledge/arithmetic-width-drops-the-declaration.md)
  > why is an AXI obligation about WSTRB demoted to a residual
- [behavioral-held-out-oracle-defects](../knowledge/behavioral-held-out-oracle-defects.md)
  > why is an ambiguous derived id pairing a fail rather than an invalid alpha transform
- [fact-plane-capacity-is-funded-not-declared](../knowledge/fact-plane-capacity-is-funded-not-declared.md)
  > why is an eighth fact-card title part infeasible
- [corpus-wide-interface-authority-rebuild](../knowledge/corpus-wide-interface-authority-rebuild.md)
  > why is an emitted-ISF count from an earlier refresh slice not the current number
- [caption-label-is-not-table-identity](../knowledge/caption-label-is-not-table-identity.md)
  > why is an enum called TABLE or FIGURE
- [corpus-task-evidence-containment-design](../knowledge/corpus-task-evidence-containment-design.md)
  > why is an exact archive alone insufficient for the active corpus task
- [heading-as-declaration-needs-a-direction-bearing-body](../knowledge/heading-as-declaration-needs-a-direction-bearing-body.md)
  > why is an identifier-shaped section title not enough to declare a wire
- [retained-bundle-population-is-frozen](../knowledge/retained-bundle-population-is-frozen.md)
  > why is an in-repo gold document re-ingest blocked by PRODUCTION-GENERICITY
- [local-repair-closes-the-caption-decision](../knowledge/local-repair-closes-the-caption-decision.md)
  > why is arm C excluded on caption_admission
- [local-repair-closes-the-caption-decision](../knowledge/local-repair-closes-the-caption-decision.md)
  > why is arm C excluded on declaration_row
- [a-layouts-reserved-space-is-a-typed-span-citing-the-keyword](../knowledge/a-layouts-reserved-space-is-a-typed-span-citing-the-keyword.md)
  > why is bReserved a field and also a reserved span (field_id)
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > why is bus not a valid parenthetical single-wire head
- [a-relational-predicate-is-not-a-value](../knowledge/a-relational-predicate-is-not-a-value.md)
  > why is by excluded from the relation complement markers
- [split-identifier-name-cell-joins-only-from-the-document](../knowledge/split-identifier-name-cell-joins-only-from-the-document.md)
  > why is c opcode read as the signal c
- [cat3-topology-isf-lowering-decision](../knowledge/cat3-topology-isf-lowering-decision.md)
  > why is category 3 (platform/system-IP) topology not lowered to .isf (ISF has no static-topology construct + the emit is single-initiator-actor; cross-component topology is structurally absent from the emit by design — KG-ISF-COMPLETENESS.2a.ii)
- [cat4-isa-csr-lowering-decision](../knowledge/cat4-isa-csr-lowering-decision.md)
  > why is category 4 (CPU ISA) ISF-thin (the only lowerable cat-4 intent is CSRs, which are registers; their bit-fields are UNLOCATED so they don't reach .isf — an extraction-recall gap, not a missing ISF construct)
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > why is channel membership metadata-only and not lowered to .isf
- [chomp-is-a-no-op-under-a-callers-slurp](../knowledge/chomp-is-a-no-op-under-a-callers-slurp.md)
  > why is chomp not removing the newline in this Perl script
- [conformal-tier-agreement-degenerate](../knowledge/conformal-tier-agreement-degenerate.md)
  > why is conformal calibration still blocked at CHI scale
- [semantic-empty-catalog-disables-grounding-filter](../knowledge/semantic-empty-catalog-disables-grounding-filter.md)
  > why is consequent_signal NOTICE or PDF or IMPLEMENTATION or MUST in SemanticIR
- [an-llm-constraint-record-cannot-be-judged-by-a-positional-gate](../knowledge/an-llm-constraint-record-cannot-be-judged-by-a-positional-gate.md)
  > why is constraint_bearing_sentence the wrong narrowing for a model proposal
- [the-binding-bearing-clause](../knowledge/the-binding-bearing-clause.md)
  > why is constraint_bearing_sentence the wrong narrowing for the dynamic constraint path
- [host-library-route-remains-on-boot-volume](../knowledge/host-library-route-remains-on-boot-volume.md)
  > why is corpus coverage refresh 34 blocked before ingest
- [retrospective-baseline-current-replay-boundary](../knowledge/retrospective-baseline-current-replay-boundary.md)
  > why is current binary replay coverage 1 of 12
- [bounded-decision-frozen-baseline](../knowledge/bounded-decision-frozen-baseline.md)
  > why is declarations_emitted not the reader's final decision
- [definitional-signal-capture](../knowledge/definitional-signal-capture.md)
  > why is descriptor apposition (signal NAME / NAME signal) NOT used to capture signal names
- [task-evidence-route-catalogs-shard-by-lifecycle](../decisions/0046-task-evidence-route-catalogs-shard-by-lifecycle.md)
  > why is docs/tasks/spec-to-intent-alignment/routes-0001.md auto-generated
- [isf-emission-blocked-by-clock-reset-contract](../knowledge/isf-emission-blocked-by-clock-reset-contract.md)
  > why is every isf adapter blocked
- [source-ir-ingest-not-reproducible](../knowledge/source-ir-ingest-not-reproducible.md)
  > why is exact source region capture 13 of 14
- [claim-registries-pin-two-digest-families](../knowledge/claim-registries-pin-two-digest-families.md)
  > why is file_digest uncached in repin_claim_regions
- [relation-verb-after-an-article-is-a-noun](../knowledge/relation-verb-after-an-article-is-a-noun.md)
  > why is its not a determiner for relation verbs (GIC ITS agent)
- [enum-member-values-have-no-oracle](../knowledge/enum-member-values-have-no-oracle.md)
  > why is kg-bench 156/156 not enough for an enum change
- [isf-emission-blocked-by-clock-reset-contract](../knowledge/isf-emission-blocked-by-clock-reset-contract.md)
  > why is lowering_status blocked
- [a-register-layout-table-sizes-the-register-its-heading-names](../knowledge/a-register-layout-table-sizes-the-register-its-heading-names.md)
  > why is msi_cfg_tbl not sized
- [a-relational-predicate-is-not-a-value](../knowledge/a-relational-predicate-is-not-a-value.md)
  > why is must be greater than 0 not a value binding
- [passive-binding-subject-authority](../knowledge/passive-binding-subject-authority.md)
  > why is must have its WSTRB input tied HIGH still extracted after the pre-bind subject repair
