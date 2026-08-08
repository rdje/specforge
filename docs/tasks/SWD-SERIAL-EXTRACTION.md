# SWD-SERIAL-EXTRACTION: serial-protocol/architecture extraction for SWD/ADI → WIRE-BASED-100 100%

## Metadata

- Tree ID: `SWD-SERIAL-EXTRACTION`
- Status: `active` (research → implementation program)
- Roadmap lane: `R15e`/`R16` (extraction quality / signoff)
- Created: `2026-06-07`
- Parent: opened by owner decision (a) from `WIRE-BASED-100.5j` — SWD/ADI (IHI0074) is a serial/architecture
  spec the parallel-bus signal-table model does not fit (KM `swd-adi-not-signal-table-spec`). This tree is
  the genuine path to SWD 100% (not a faked cherry-picked gold).

## Goal

Recover SWD/ADI implementation-relevant intent — the serial interface signals, the serial-frame protocol,
and the DP/AP register-access interface — to the owner's WIRE-BASED-100 100% bar (constraints + relations +
temporal, real fresh-evidence `eval-extraction`, per-fact, no faking), via a serial/architecture-aware
extraction approach distinct from the parallel-bus signal-table path.

## Non-goals

- Do NOT regress the parallel-bus specs: APB/AHB/AXI must stay 100% on all 3 aspects + kg-bench green.
- Not full HDL/`.isf` lowering of SWD (out of scope per the project boundary).
- Not modeling every DP/AP register field exhaustively — focus on the implementation-relevant interface.

## Acceptance criteria

- `SWCLK` and `SWDIO` (the 2-wire serial interface) are captured as canonical interface signals (today they
  are prose/figure-only → never declared).
- A faithful SWD constraint + relation + temporal gold (`seed_swd*.json`) is built from real ADI prose and
  measured; the extractor reaches `P=R=F1=1.000` on it (per-fact, no cherry-picking).
- The serial-frame protocol (start/stop/park/turnaround/parity bits, WAIT/FAULT ACK responses) is captured
  as typed facts (a sequence/temporal surface), not lost.
- No parallel-bus regression; full `scripts/run_ci.sh` green; KM cards for each durable finding.

## Task tree

- ID: `SWD-SERIAL-EXTRACTION` · Status: `active` · Children: `.1`–`.7`
- ID: `SWD-SERIAL-EXTRACTION.1` · Status: `done` · Goal: research/characterize SWD's extractable serial
  intent. **Findings (`2026-06-07`, from the ingested ADI evidence):** (1) SWCLK/SWDIO ARE describable from
  prose — "The SWD interface uses a single bidirectional data pin, **SWDIO**"; "The SWD interface … requires
  a clock pin, **SWCLK**" (pattern: "a `<role>` pin, `<SIGNAL>`" / "`<SIGNAL>` is the `<role>` pin"). (2) The
  serial frame IS described: start bit, single stop bit (`0b0`), Park bit (driven HIGH), turnaround period,
  parity, WAIT/FAULT/OK ACK responses. (3) Source tables: 160 unknown, 39 encoding, 5 signal_description, 3
  register_map, 1 timing, 2 feature_matrix — the DP/AP register interface is in register_map/encoding tables;
  the bulk is architecture prose. Conclusion: SWD extraction is feasible with a serial/architecture-aware
  path; the decomposition below follows.
- ID: `SWD-SERIAL-EXTRACTION.2` · Status: `done` · Goal: capture `SWCLK`/`SWDIO` (and the JTAG pins) as
  canonical interface signals from the prose "`<role>` pin, `<SIGNAL>`" pattern. **Done:** added
  `synthesize_signal_declarations_from_prose` (scans statements for the noun "pin" immediately naming a
  signal across a comma — fused "pin," SIG or separate "pin" "," SIG; emits a width-1 declaration), wired
  into `EvidenceIr` build alongside the table-declaration path (additive; dupes dedupe downstream).
  ADR-0006 (grammar, not names). **Achieved on fresh evidence: `SWCLK`, `SWDIO`, and `NSRST` (a real ADI
  system-reset) now declared** (were prose/figure-only); cross-ref "pin, see Figure" garbage suppressed by
  adding `SEE` to `is_signal_synthesis_non_signal`. No parallel-bus regression (APB/AHB/AXI all still
  1.000 on constraints + relations); +3 hermetic tests; full `scripts/run_ci.sh` green. KM
  `[[prose-pin-appositive-signal-capture]]`.
- ID: `SWD-SERIAL-EXTRACTION.3b` · Status: `done` · Goal: enrich the frame surface — named single-bit
  request fields, ACK response values, field ordering. **Done:** added `order` + `response_values` to
  `SerialFrameField`; `parse_named_bit_list` mines "the N bits X, Y and Z" (the prose explicitly labels
  them "bits" → grammar, not names) so the mixed-case `APnDP`/`RnW` request bits (which fail
  `is_hardware_signal_token`) are captured at width 1; `extract_ack_response_values` reads the ACK
  responses from the "`<value>` response to a DPACC/APACC access" grammar (handles "OK or FAULT"),
  gated to DP/AP-access statements so the broad "`<X>` response" noise (DP/CTI/ACK) is excluded; a final
  pass assigns `order` by phase rank (request → acknowledge → data) then appearance. **Achieved on fresh
  ADI evidence: 7 ordered frame fields** — `A`(2)/`DATAIN`(32)/`APnDP`(1)/`RnW`(1) [request],
  `ACK`(3, resp=`[FAULT, OK, WAIT]`) [acknowledge], `WDATA`(32)/`RDATA`(32) [data]. Parallel buses still
  emit 0 + stay 100%; +3 hermetic tests; full `scripts/run_ci.sh` green. KM `[[swd-serial-frame-surface]]`.
- ID: `SWD-SERIAL-EXTRACTION.3` · Status: `done` · Goal: model the SWD serial-frame protocol as a
  NEW typed fact surface (owner decision (ii)). **Done:** added the `SerialFrameField` record +
  `SerialFramePhase` enum (Request/Acknowledge/Data) to `EvidenceIr` (serde-skip-if-empty → zero churn
  for non-serial docs); `extract_serial_frame_fields` mines `NAME[hi:lo]` bit-ranges (width = |hi-lo|+1),
  **double-gated** — (1) document-level serial markers ("serial wire"/"packet request"/"shift-dr"/SWDIO/
  SWCLK) so parallel buses produce 0, (2) per-statement frame phase (request/ack/data) so unrelated
  bit-fields (register fields, bridged-bus `AxCACHE[3:0]`/`HMASTER[3:0]`) are dropped. Protocol
  vocabulary, not chip names (ADR 0006). **Achieved on fresh ADI evidence: 5 clean frame fields** —
  `A`(2, request), `ACK`(3, acknowledge), `DATAIN`/`WDATA`/`RDATA`(32, data) — zero noise; parallel
  buses produce 0 `serial_frame_fields` and stay 100% on all 3 aspects. +4 hermetic tests; full
  `scripts/run_ci.sh` green. KM `[[swd-serial-frame-surface]]`.
- ID: `SWD-SERIAL-EXTRACTION.4` · Status: `done` (FSM states; transitions → `.4b`) · Goal: **extract the
  protocol FSM (states).** Owner insight (`2026-06-07`): "SWD like JTAG is also described using a FSM …
  that FSM is critical to the proper understanding and implementation of SWD/JTAG" — correct, and it is the
  heart of SpecForge's purpose (IntentIR → `.isf` → FSMGen builds the `.fsm`). **Done:** added the typed
  `ProtocolStateRecord` surface (machine_name + state_name + per-state action + supporting statements) to
  `EvidenceIr` (serde-skip-if-empty); `extract_protocol_states` recognizes states by the "`<StateName>`
  state" grammar (hyphen/slash-joined capitalized tokens — `looks_like_state_name`; grammar, not names,
  ADR 0006), gated to documents describing a state machine ("state machine"/DBGTAPSM/"TAP controller");
  captures the per-state action clause via `find_states_with_actions`. **Achieved on fresh ADI evidence:
  the DBGTAPSM with 9 named TAP states + actions** — Capture-/Shift-/Update-IR, Capture-/Shift-/Update-DR,
  Run-Test/Idle ("no special actions occur"), Test-Logic-Reset ("is the reset condition"). Parallel buses
  emit 0 protocol_states and stay 100%; +4 hermetic tests; full `scripts/run_ci.sh` green. KM
  `[[swd-protocol-fsm-surface]]`. Known minor artifact: a `Test-Logic/Reset` docling separator variant of
  `Test-Logic-Reset` (the JTAG name uses hyphens) — dedup is a `.4b` refinement. NOTE: ISF must be able to
  model an explicit FSM (states + transitions) elegantly for `.5` lowering — see the ISF-abstraction
  feature-request check (owner: no hacks; raise an ISF feature request if a gap exists).
- ID: `SWD-SERIAL-EXTRACTION.5` · Status: `done` (owner decision (a): score the SWD surfaces to 100%).
  Built a **SWD-derivation gold + scorer**: extended `eval`/`eval-extraction` with three new tasks —
  `serial_frame_field`, `swd_operation`, `protocol_state` (new `EvalTask`/`GoldFact` variants, canonical
  keys, record-key + index fns, deterministic extractor reading the EvidenceIR surfaces). Gold
  `seed_swd_derivation.json` = **28 spec-verified facts** (11 frame fields, 4 operations, 13 FSM states),
  each checked against spec B4.2 / B3.2.3. **Two precision fixes to reach 100%:** DATAIN phase
  request→data (data-keyword precedence — its statement has both RnW and DATAIN); and separator dedup of
  `Test-Logic/Reset` into `Test-Logic-Reset` (docling '/' variant). **Achieved: source-tolerant (the
  WIRE-BASED-100 metric) `P=R=F1=1.000` on all three** — `serial_frame_field` (tp=11), `swd_operation`
  (tp=4), `protocol_state` (tp=13). (Strict per-statement is lower only because some statements support
  multiple doc-level facts — the source-tolerant scorer is the bar used for every spec.) +N tests; parallel
  buses + their evals unchanged; full `scripts/run_ci.sh` green. **SWD now scores 100% on its FSM/frame
  surfaces — "100% on all fronts" reached.** `.isf` lowering (gated on 100%) is now unblocked → next:
  lower the FSM via the PROVEN enum-state + `switch`+`select`+`rule trigger` idiom (KM
  `[[isf-fsm-via-switch-select]]`); SpecForge emits intent, FSMGen lowers.
- ID: `SWD-SERIAL-EXTRACTION.6` · Status: `done` (re-done correctly; feature request WITHDRAWN) · Goal:
  ensure ISF can model these protocols ELEGANTLY (owner: no hacks). **Initial mistake:** filed a feature
  request claiming ISF can't declare an FSM — but off a `subs/fsmgen` submodule **312 commits stale** and
  from reading alone. Owner corrected (3×): thoroughly check what FSMGen offers; focus on `.isf` not `.fsm`
  (`.fsm` is too low-level); SpecForge doesn't cycle-schedule — FSMGen lowers `.isf` → `.fsm`; and "make
  sure you really can't use the existing ISF … first". **Re-done:** updated the submodule to `d31b0b91`
  (SpecForge's 41 isf/fsmgen tests still pass against it), read the current ISF book/contract/handoff, then
  **EMPIRICALLY tested**. **PROVEN: ISF accurately describes an FSM** — a 6-state JTAG TAP-DR FSM (correct
  TMS edges) lowers clean (`fsmgen --strict --check --json` → `success:true`) via the idiom `storage` state
  var + `switch` on it + `(select st input NEXT_IF NEXT_ELSE)` per state + `(rule tick start (trigger
  step))` for recurrence. Boundaries found: switch-in-`while` unsupported; competing per-transition rules
  trip `isf_conflicting_rule_writes`; `select` is a transaction action not a rule action; `cond` nested in
  a switch branch unsupported. → **Feature request WITHDRAWN** (`docs/fsmgen-issues/sf-isf-explicit-fsm-declaration/`
  marked WITHDRAWN); serial frame also NOT a gap (ISF has shift registers / serial fixtures). KM
  `[[isf-fsm-via-switch-select]]`; lessons `[[feedback_verify_fsmgen_before_fr]]`, `[[feedback_isf_no_hacks]]`.
- ID: `SWD-SERIAL-EXTRACTION.4e` · Status: `done` (`2026-08-08`) · Goal: capture the one
  still-missing Chapter B4 clocking fact as a typed, scored interface-edge timing record: the target
  samples SWDIO and changes whether it drives SWDIO on the rising edge of SWCLK. Derive actor, data
  signal, clock signal, edge, and both operations from the document's own timing-class prose; no
  protocol-name or signal-name constants in production. Extend the real SWD derivation gold from 28 to
  29 spec-verified facts and keep every existing wire/corpus gate green.
- ID: `SWD-SERIAL-EXTRACTION.7` · Status: `active` (container; depends on `.4e`; opened `2026-08-08`;
  activated `2026-08-09`) · Goal:
  audit and close the protocol-surface projection boundary. `serial_frame_fields`, `swd_operations`, and
  `protocol_states` currently live only on `EvidenceIR`; `eval-extraction` scores them by reading that
  layer directly, while `SemanticIR::build`, `IntentIR::build`, and the `.isf` adapter consume none of
  them. After `.4e`, design and implement the honest typed Evidence→Semantic→Intent projection and only
  the ISF lowering that the proven FSM/serial idioms can represent without fabrication.
- ID: `SWD-SERIAL-EXTRACTION.7a` · Status: `done` (`2026-08-09`) · Goal: audit the exact four-surface schemas,
  downstream typed models, validation, and current canonical artifact state; record the lossless-projection
  and honest-lowering decision; root-cause the missing canonical `.4e` timing record; and decompose `.7`
  before any product-schema change.
- ID: `SWD-SERIAL-EXTRACTION.7b` · Status: `done` (`2026-08-09`) · Goal: project all four typed protocol surfaces
  losslessly from EvidenceIR into SemanticIR with additive, backward-compatible serialization, exact
  cross-stage parity tests, and validation counts.
- ID: `SWD-SERIAL-EXTRACTION.7c` · Status: `pending` · Goal: project the same surfaces losslessly from
  SemanticIR into canonical IntentIR, with exact parity/round-trip tests and validation counts.
- ID: `SWD-SERIAL-EXTRACTION.7d` · Status: `pending` · Goal: make the `.isf` adapter account explicitly
  for every protocol record: lower only a fully licensed representable subset and preserve every
  under-specified record as a typed residual, with FSMGen-strict and generic-adapter regressions.
- ID: `SWD-SERIAL-EXTRACTION.7e` · Status: `pending` (container) · Goal: restore protocol-aware convergence
  accounting, then rebuild/promote the tracked ADI pipeline and close the program from current artifacts.
- ID: `SWD-SERIAL-EXTRACTION.7e.i` · Status: `pending` · Goal: extend Evidence/Semantic/Intent convergence
  snapshots and fact counts across all four protocol collections, with protocol-only change detection tests,
  before a fresh canonical run relies on convergence deltas.
- ID: `SWD-SERIAL-EXTRACTION.7e.ii` · Status: `pending` · Goal: rebuild the tracked ADI pipeline from a fresh
  repository-local CPU ingest, promote the complete current chain, prove canonical 29/29 scoring plus
  downstream parity/residual/convergence accounting, run full wire/KG/CI gates, and close `.7` and the parent.

### Acceptance Checklist (enforced) — `SWD-SERIAL-EXTRACTION.7a`

- [x] **REPRODUCE / MEASURE** — inventory each EvidenceIR record's exact semantics and prove the four
  surfaces are absent from SemanticIR, IntentIR, and the typed ISF adapter; measure the canonical SWD
  artifact separately from the fresh `.4e` proof.
- [x] **ROOT CAUSE (WHY + WHERE)** — identify the exact missing bindings that make direct behavioral ISF
  lowering unsafe, and establish why canonical `interface_edge_timings` is empty despite the green fresh
  `.4e` extraction proof.
- [x] **ADDRESSED (verified)** — accept one durable architecture decision, one bounded implementation
  decomposition, and Knowledge Map facts whose `reverify` commands reproduce both boundaries.
- [x] **NO REGRESSION** — documentation/doctrine/index checks pass; no code, product schema, generated
  artifact, or user-owned workspace file changes in this design leaf.
- [x] **LOCKSTEP** — update the task frontier, decision index, topically correct mdBook section, bounded
  resume pointer, and derived Knowledge Map only where the audited public truth changed.

### Acceptance Checklist (enforced) — `SWD-SERIAL-EXTRACTION.7b`

- [x] **REPRODUCE / MEASURE** — prove SemanticIR has no fields for the four non-empty EvidenceIR protocol
  collections and validation cannot report them, while unrelated legacy artifacts load without those keys.
- [x] **ROOT CAUSE (WHY + WHERE)** — localize the loss to `SemanticIr` schema/build assembly and validation
  metrics; confirm no semantic filter or reinterpretation is licensed by ADR 0016.
- [x] **ADDRESSED (verified)** — add serde-default/skip-empty SemanticIR collections using the exact EvidenceIR
  record types, clone each collection unchanged, and report deterministic counts.
- [x] **NO REGRESSION** — focused schema/build/load/validation tests, warning-deny formatting/Clippy, relevant
  wire/KG checks, doctrines, and the broader gate warranted by the central IR schema all pass.
- [x] **GENERICITY / HONESTY** — no protocol/signal name list, inferred field, record filtering, reordering,
  or provenance rewrite enters the projection.
- [x] **LOCKSTEP** — update this leaf, live technical truth, and the SemanticIR/mdBook product contract; leave
  the still-unimplemented IntentIR/adapter boundary explicit under `.7c`/`.7d`.

### Surfaced portability finding (handoff after `.4e`)

The fresh live build exposed a project-wide locality defect outside this slice: 335 generated JSON/Markdown
artifacts still embed `/Users/richarddje/Documents/github/specforge`, and the Evidence→Semantic→Intent stage
builders call `canonicalize_existing_path` before persisting their upstream-path fields. The old tree is gone,
so these are stale references rather than live cross-volume reads, but absolute stage pointers violate the
repository-root-relative persistence contract and will break after the next move. Per the dirty-tree pivot
rule, `.4e` records but does not absorb this broader repair. Immediately after the clean `.4e` commit, open a
dedicated task-tree that owns portable cross-stage paths, backward-compatible loading, a generated-artifact
census/migration, and fail-closed locality coverage before resuming `.7`.

## Acceptance Checklist (enforced) — `SWD-SERIAL-EXTRACTION.4e`

- [x] **REPRODUCE / MEASURE** — `statement_1948` is a `timing_constraint` that explicitly binds target
  sampling and drive-state changes on SWDIO to the rising edge of SWCLK; current EvidenceIR, SemanticIR,
  and IntentIR contain no typed timing record or temporal rule supported by that statement.
- [x] **ROOT CAUSE (WHY + WHERE)** — the existing timing synthesis reads parameter tables only, while
  SWD protocol surfaces cover frame fields, response branches, and states but not prose-defined interface
  edge timing. The current SWD derivation gold therefore scores 28 facts without this known B4 fact.
- [x] **ADDRESSED (verified)** — add one document-derived `InterfaceEdgeTimingRecord`, a registered
  extraction surface, one agent-verified gold item, and deterministic scoring of the full semantic tuple.
- [x] **NO REGRESSION** — prove positive and fail-closed grammar tests, fresh SWD evidence/gold 29/29,
  existing APB/AHB/AXI and SWD scores, `kg-bench`, doctrines, full CI, and repository-local residue.
- [x] **GENERICITY (ADR 0006)** — production logic keys only on timing statement class, universal
  sample/drive/edge grammar, and names already present in the document; fixtures may use SWD names.
- [x] **LOCKSTEP** — update this tree, the topically correct mdBook chapter, the SWD timing/projection
  Knowledge Map facts, live ledgers, and the bounded resume pointer; regenerate derived indexes only.

## Reframe (owner `2026-06-07`, GROUNDED IN SPEC CHAPTER B4 read directly)

Owner: "do not trust what I am saying, read and understand SWD's chip-spec PDF thoroughly." Done — read all
of Chapter B4 (B4.1 About, B4.2 Operation, B4.3 Interface) via docling `content_elements` (the PDF is
password-protected so the Read tool can't open it). **SWD's intent = its packet protocol + line state
machine on SWDIO** (KM `[[swd-intent-is-the-fsm-driving-swdio]]`). The classic constraint/relation/temporal
scores are NOT where SWD's intent lives (sparse; `seed_swd.json` constraint+relation `P=R=F1=1.000`, lone
temporal rule = noise, not pursued). Two corrections to the loose framing: the SWD FSM is NOT the JTAG
`DBGTAPSM` (`.4` captured the JTAG TAP machine — a different thing); and the target samples AND drives SWDIO
on the RISING SWCLK edge (B4.3.1).

**SWD intent precisely (B4):** packet micro-sequence — request(8b: Start/APnDP/RnW/A[2:3]/Parity/Stop/Park,
host drives) → Trn → ack(3b ACK[0:2]: OK=0b001/WAIT=0b010/FAULT=0b100, target drives) → [write: Trn →
WDATA[0:31]+par host drives; read: no Trn, RDATA[0:31]+par target drives, then Trn]; response branching
(OK→3-phase, WAIT/FAULT→2-phase); even parity over request + over data; LSB-first. Line FSM:
reset/operating/protocol-error/lockout/dormant/deselected (line reset = ≥50 SWDIO-HIGH + ≥2 idle).

**What SpecForge derives today:** SWCLK/SWDIO signals (`.2`); SOME packet bit-fields (`.3`: A/ACK/APnDP/RnW/
WDATA/RDATA — MISSING Start/Parity/Stop/Park, no direction/sequence/turnaround/branch); JTAG TAP `DBGTAPSM`
(`.4`, not the SWD FSM). **SpecForge does NOT yet fully derive SWD's intent.**

## Current frontier (spec-grounded gaps)

- `SWD-SERIAL-EXTRACTION.4e` — **DONE.** The explicit target/SWDIO/SWCLK rising-edge statement is one
  typed, registered, provenance-carrying interface-edge record and the 29th independently verified SWD
  gold fact. Fresh extraction and complete-tuple scoring are 1.000; projection remains outside this leaf.
- **Completed clean-tree handoff:** `ARTIFACT-PATH-PORTABILITY` is closed at `97916b95`; all canonical
  paths, dormant serialized schemas, present artifacts, and moved-root workflows now pass the locality contract.
- `SWD-SERIAL-EXTRACTION.7a` — **DONE.** ADR 0016 freezes exact Evidence→Semantic→Intent carry-through and
  explicit adapter residual accounting; direct behavioral lowering remains empty until records supply complete
  bindings. The canonical 11/4/13/0 surface split is the deliberately unpromoted pre-`.4e` cache, and `.7e`
  owns its fresh tracked-PDF replacement.
- `SWD-SERIAL-EXTRACTION.7b` — **DONE.** SemanticIR carries the exact four EvidenceIR record collections through
  additive empty-compatible fields and unfiltered clones; validation reports all four counts. Full CI passes.
  Next pickable leaf: `.7c` canonical IntentIR projection.

- `SWD-SERIAL-EXTRACTION.4c` — **DONE.** Added `SwdioDirection {HostDrives, TargetDrives}` + `swdio_direction`
  on `SerialFrameField`; `extract_serial_frame_fields` derives it from the spec's own "from the `<A>` to the
  `<B>`" / "`<A>` to `<B>`, following a read/write request" prose (`swdio_source_actor`), field-level for the
  data phase (WDATA host / RDATA target) and phase-level for request/acknowledge. Achieved on fresh ADI
  evidence: A/DATAIN/APnDP/RnW=`host_drives`, ACK=`target_drives`, WDATA=`host_drives`, RDATA=`target_drives`
  (exactly the spec). Parallel buses emit 0 serial_frame_fields (no pollution); +3 hermetic tests; full
  `scripts/run_ci.sh` green. This is "drive commands / sample data on SWDIO via the FSM".
- `SWD-SERIAL-EXTRACTION.4b` — **partly DONE (missing fields).** Added `parse_control_bit_fields`: the
  request-frame control bits **Start / Parity / Stop / Park** are now captured (1-bit, request phase,
  host-driven) from two high-precision phrasings — "A single `<name>` bit …" (Start/Stop/Parity) and "the
  `<Name>` bit is not 0b…" (Stop/Park protocol-error, B4.2.5) — the broad "`<Word>` bit" form is rejected
  (it over-matched ~30 register names). **SWD packet request frame now complete:** A/DATAIN/APnDP/RnW/Start/
  Parity/Stop/Park (host) · ACK (target) · WDATA (host)/RDATA (target), ordered, with SWDIO direction. +2
  hermetic tests; parallel buses still 0; CI green. **`.4b` response branching — DONE:** added the
  `SwdOperation` surface + `extract_swd_operations` deriving the response-branched phase sequences from
  "a successful `<read|write>` operation consists of three phases" / "A `<WAIT|FAULT>` response … consists
  of two phases" (B4.2) + the turnaround model from the write/read turnaround prose. On fresh ADI evidence:
  **OK/write → 3-phase, Trn-before-data=true; OK/read → 3-phase, Trn-before-data=false; WAIT → 2-phase
  no-data; FAULT → 2-phase no-data** (exactly the spec). +2 hermetic tests; parallel buses emit 0
  swd_operations; CI green. The SWD packet protocol (fields + widths + phase + order + SWDIO direction +
  response branching + turnaround) is now fully derived. **`.4b` DONE.**
- `SWD-SERIAL-EXTRACTION.4d` — **DONE (line states).** Added `extract_swd_line_states` (extends
  `protocol_states` with `machine_name="SWD line state machine"`): the SWD LINE states are lowercase 1–2-word
  names introduced by a transition verb — "(enter|enters|into|leave|leaves) [the] `<name>` state". On fresh
  ADI evidence: **Reset, Operating, Protocol error, Lockout, Dormant** (all 5 line states; the
  "Line reset" near-dup is now collapsed into Reset via leading-qualifier stripping, and `operating` is
  captured via the "to the `<adj>` operating state" phrasing — logic-level/verb garbage rejected). Verb-gated + a
  per-statement SWD-context gate (swd/sw-dp/line/target/interface/protocol) drops the processor "Debug
  state" (execution mode, not a line state). +2 hermetic tests; parallel buses emit 0 SWD-line states; CI
  green. **Edge timing** (target samples & drives SWDIO on the rising SWCLK edge, B4.3.1) is derivable from
  `statement_1948` (documented; a typed timing surface deferred as a single fact). `operating` (the implicit
  normal state) is phrased "transition to …" and not verb-captured — minor.
- `SWD-SERIAL-EXTRACTION.5` — DONE for the (minor) measurable scores: `seed_swd.json` constraint + relation
  `P=R=F1=1.000` (sparse clean set; garbage actors filtered). Temporal not pursued (SWD intent is the FSM).

## Decisions

- Opened per owner decision (a) on `2026-06-07`; the parallel-bus playbook does not fit SWD (KM
  `swd-adi-not-signal-table-spec`) — this is a distinct serial/architecture path.
- No-faking: SWD 100% must be earned on a faithful gold, never a cherry-picked one.
- ADR 0016 (`SWD-SERIAL-EXTRACTION.7a`): project all four protocol collections and provenance exactly through
  SemanticIR and IntentIR; the adapter lowers only records carrying every required behavioral binding and
  residualizes the rest. The current directly lowerable protocol subset is empty; independently licensed ISF
  may still render alongside explicit protocol residuals.

## Open questions

- Resolved by `.3`: serial-frame structure is its own typed surface; `.4e` likewise uses a typed
  interface-edge record because the exact clock signal and both sample/drive-state operations are part of
  identity, while the current generic temporal-score key does not include `clock_signal`.
- How much of the DP/AP register interface is implementation-relevant for the `IntentIR` consumer?

## Blockers

- None. The path-portability defect is closed. The tracked ADI PDF and repository-local Docling environment
  remain available for `.7e`; the older canonical SourceIR's reclaimed normalized Markdown means freshness
  requires that planned full ingest rather than an unsafe stage-only rebuild.

## Verification log

- `.1`: characterization done from the ingested ADI evidence (SWCLK/SWDIO prose, serial-frame prose, table-kind census).
- `.2`: fresh ADI evidence rebuild shows SWCLK/SWDIO/NSRST declared, SEE suppressed; APB/AHB/AXI constraints+relations all 1.000; +3 hermetic tests; full `scripts/run_ci.sh` green (1323 lib tests).
- `.3`: fresh ADI evidence yields 5 clean frame fields (A/ACK/DATAIN/WDATA/RDATA with correct widths); parallel buses produce 0 serial_frame_fields and stay 100% on constraints+relations+temporal; +4 hermetic tests; full `scripts/run_ci.sh` green (1327 lib tests).
- `.3b`: fresh ADI evidence yields 7 ordered frame fields (adds APnDP/RnW request bits + ACK resp=[FAULT,OK,WAIT] + order); parallel buses still 0 + 100%; +3 hermetic tests; full `scripts/run_ci.sh` green (1330 lib tests).
- `.4`: fresh ADI evidence yields the DBGTAPSM with 9 named TAP states + per-state actions; parallel buses emit 0 protocol_states and stay 100%; +4 hermetic tests; full `scripts/run_ci.sh` green (1334 lib tests).
- `.4e`: four focused extraction/key tests pass; fresh repository-local CPU re-ingest (400 pages, 386
  visual assets, high confidence, zero residual decisions) produces exactly one complete timing record
  from `statement_1948`, with manifest eligible/produced/kept = 1/1/1. Fresh 29-item SWD scoring is
  source-tolerant P/R/F1 1.000 for frame 11, operation 4, state 13, and edge timing 1. Seven neighboring
  WIRE suites retain their expected gates (the known SWD promotion-only constraint miss unchanged),
  `kg-bench` is 156/156, all six doctrines pass, and full CI is green: formatting, warning-deny Clippy,
  1,735 passed / 5 ignored, rustdoc, 36-file mdBook, locality, and residue. Only `.gitkeep` remains in
  `.project-data/tmp`; the two exact `.4e` workspaces plus empty compiler/Docling scratch were removed.
- `.7a` architecture reproduction (`2026-08-09`): source inspection finds all four fields only on EvidenceIR
  and the evaluator, not SemanticIR/IntentIR/IsfIR. The exact schemas expose no state transitions/guards/initial
  state/encoding and no complete serial wire/value/activation/storage bindings; `IsfTxnStep` has `Switch`,
  `Set`, shift, and sample forms but no `select` expression. Canonical artifact measurement is frame 11 /
  operation 4 / state 13 / edge 0, no edge-manifest row; adapter is renderable with 0 transactions, 1 rule,
  and 3 unrelated residuals. `statement_1948` retains the exact rising-edge clause. A correct EvidenceIR
  dry-run fails closed on the absent normalized Markdown leaf, while the 2,925,300-byte tracked source PDF is
  present. ADR 0016, the updated projection fact, and the canonical-staleness fact preserve the root cause.
  Knowledge Map generation writes 153 facts / 1,066 unique question keys across eight bounded shards; the
  fact-card catalog writes 152 routes. mdBook doctests/build and all six composed doctrines pass.
- `.7b`: the exact projection test passes with two ordered frame records plus operation/state/edge records,
  preserving every optional member and supporting statement id; clearing all four collections omits the keys,
  and that legacy JSON shape decodes to empty defaults. SemanticIR validation reports 2/1/1/1 in its focused
  test. The complete SemanticIR module passes 394/394 and the matching validation subset 18/18; warning-deny
  Clippy passes. A live current-ADI SemanticIR dry-run projects 11 frame / 4 operation / 13 state / 0 edge
  records and preserves the first field/state ids and statement provenance. Full `scripts/run_ci.sh` is green:
  all six doctrines, formatting, Clippy, 1,768 passed / 5 ignored, rustdoc, mdBook doctests/build, 156/156 KG
  fixtures through the suite, and final project-data locality. After the convergence finding is recorded,
  Knowledge Map generation writes 154 facts / 1,073 unique question keys across eight shards; the fact catalog
  writes 153 routes, and all six doctrines pass again.
- `.7b` cold-read follow-up: `EvidenceSnapshot`, `SemanticSnapshot`, and `IntentSnapshot` in `converge.rs`
  omit all four protocol collections from their typed fields and `fact_count`; therefore a protocol-only
  change can be invisible to the aggregate convergence delta. This does not invalidate exact `.7b` projection,
  but it blocks final fresh-pipeline closure until dedicated `.7e.i` makes all three snapshots protocol-aware.

## Commit log

- `.1`: see the `SWD-SERIAL-EXTRACTION.1` commit (tree opened + research leaf).
- `.2`: see the `SWD-SERIAL-EXTRACTION.2` commit (prose pin-appositive signal capture).
- `.3`: see the `SWD-SERIAL-EXTRACTION.3` commit (typed serial-frame field surface).
- `.3b`: see the `SWD-SERIAL-EXTRACTION.3b` commit (named request bits + ACK values + ordering).
- `.4`: see the `SWD-SERIAL-EXTRACTION.4` commit (typed protocol-FSM state surface).
- `.4e`: see the `SWD-SERIAL-EXTRACTION.4e` commit (typed/scored interface-edge timing).
- `.7a`: see the `SWD-SERIAL-EXTRACTION.7a` commit (projection/lowering architecture and freshness root cause).
- `.7b`: see the `SWD-SERIAL-EXTRACTION.7b` commit (lossless EvidenceIR→SemanticIR projection and counts).

## Changelog

- `2026-06-07`: Created (owner decision (a) from `WIRE-BASED-100.5j`). `.1` research/characterization done.
  `.2` done — SWCLK/SWDIO/NSRST captured from the prose "`<role>` pin, `<SIGNAL>`" appositive.
  `.3` done (owner decision (ii)) — new typed `SerialFrameField` surface (bit-widths from `NAME[hi:lo]`,
  double-gated to serial docs + frame phases); 5 clean SWD frame fields; frontier = `.3b`.
- `2026-08-08`: `.4e` done — added and freshly verified the complete interface-edge timing record,
  expanded SWD gold 28→29 at 1.000 on all four protocol tasks, kept every WIRE/KG gate green, and
  recorded the EvidenceIR projection plus cross-stage absolute-path boundaries without widening scope.
- `2026-08-09`: `.7a` audited the four downstream gaps, accepted exact typed carry-through plus explicit
  residual accounting, root-caused canonical edge staleness as deliberate `.4e` nonpromotion, and decomposed
  implementation/fresh-ingest closure into `.7b`–`.7e`.
- `2026-08-09`: `.7b` added exact additive SemanticIR carry-through for all four protocol collections,
  validation counts, legacy-empty compatibility, and full record/order/provenance parity tests; full CI green.
  Its cold read also split protocol-aware convergence accounting into `.7e.i` ahead of fresh `.7e.ii` closure.
