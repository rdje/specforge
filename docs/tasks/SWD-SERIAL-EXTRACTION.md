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

- ID: `SWD-SERIAL-EXTRACTION` · Status: `active` · Children: `.1`–`.5`
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
- ID: `SWD-SERIAL-EXTRACTION.5` · Status: `pending` · Goal: build the SWD constraint/relation/temporal golds
  from real prose, measure, and fix to `P=R=F1=1.000` (per-fact), no parallel-bus regression. Lower the
  captured FSM (`protocol_states`) to `.isf` using the PROVEN idiom (storage state var + `switch` +
  `select` per-state transition + `rule trigger`; KM `[[isf-fsm-via-switch-select]]`) and confirm it lowers
  through FSMGen — no feature request needed.
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

## Reframe (owner `2026-06-07`): SWD's intent IS the FSM walking SWDIO

SWD's full intent comes from its FSM — the host walks the FSM to **drive commands onto SWDIO** and
**sample returned data from SWDIO** (KM `[[swd-intent-is-the-fsm-driving-swdio]]`). So the classic
constraint/relation/temporal scores are NOT where SWD's intent lives (they are sparse; `seed_swd.json`
shows constraint + relation `P=R=F1=1.000` on the small clean set, and the lone temporal rule is noise —
not pursued). The work is to FULLY derive the FSM + per-state SWDIO I/O.

What SpecForge derives today: SWD signals (`.2`); packet FIELDS with phase+width (`.3`/`.3b`); JTAG TAP
states (`.4`, the JTAG-DP FSM — NOT the SWD-on-SWDIO packet FSM). Gaps → `.4c`/`.4b`/`.4d` below.

## Current frontier

- `SWD-SERIAL-EXTRACTION.4c` (NEW, central) — **per-state SWDIO direction**: each frame field/phase needs
  `drive` vs `sample` + actor (host drives request + write-data; target drives ack + read-data, host
  samples). This is "drive commands / capture data on SWDIO via the FSM". Add a direction to
  `SerialFrameField` (or the FSM states).
- `SWD-SERIAL-EXTRACTION.4b` — the **SWD packet FSM** + transitions (request → turnaround → ack →
  turnaround → data → park), distinct from the JTAG TAP FSM; + dedup the `Test-Logic/Reset` variant.
- `SWD-SERIAL-EXTRACTION.4d` — line state machine (reset / operating / protocol-error) + edge timing
  (sample on rising SWCLK, drive on falling — `statement_1948`).
- `SWD-SERIAL-EXTRACTION.5` — DONE for the measurable scores: `seed_swd.json` constraint + relation
  `P=R=F1=1.000` (sparse clean set; garbage actors filtered). Temporal not pursued (SWD intent is the FSM).

## Decisions

- Opened per owner decision (a) on `2026-06-07`; the parallel-bus playbook does not fit SWD (KM
  `swd-adi-not-signal-table-spec`) — this is a distinct serial/architecture path.
- No-faking: SWD 100% must be earned on a faithful gold, never a cherry-picked one.

## Open questions

- Should the serial-frame protocol be a new typed fact surface, or fit into the existing temporal-rule model?
- How much of the DP/AP register interface is implementation-relevant for the `IntentIR` consumer?

## Blockers

- None (the ADI PDF is in `corpus/`, ingested; normalized present after `.5j`).

## Verification log

- `.1`: characterization done from the ingested ADI evidence (SWCLK/SWDIO prose, serial-frame prose, table-kind census).
- `.2`: fresh ADI evidence rebuild shows SWCLK/SWDIO/NSRST declared, SEE suppressed; APB/AHB/AXI constraints+relations all 1.000; +3 hermetic tests; full `scripts/run_ci.sh` green (1323 lib tests).
- `.3`: fresh ADI evidence yields 5 clean frame fields (A/ACK/DATAIN/WDATA/RDATA with correct widths); parallel buses produce 0 serial_frame_fields and stay 100% on constraints+relations+temporal; +4 hermetic tests; full `scripts/run_ci.sh` green (1327 lib tests).
- `.3b`: fresh ADI evidence yields 7 ordered frame fields (adds APnDP/RnW request bits + ACK resp=[FAULT,OK,WAIT] + order); parallel buses still 0 + 100%; +3 hermetic tests; full `scripts/run_ci.sh` green (1330 lib tests).
- `.4`: fresh ADI evidence yields the DBGTAPSM with 9 named TAP states + per-state actions; parallel buses emit 0 protocol_states and stay 100%; +4 hermetic tests; full `scripts/run_ci.sh` green (1334 lib tests).

## Commit log

- `.1`: see the `SWD-SERIAL-EXTRACTION.1` commit (tree opened + research leaf).
- `.2`: see the `SWD-SERIAL-EXTRACTION.2` commit (prose pin-appositive signal capture).
- `.3`: see the `SWD-SERIAL-EXTRACTION.3` commit (typed serial-frame field surface).
- `.3b`: see the `SWD-SERIAL-EXTRACTION.3b` commit (named request bits + ACK values + ordering).
- `.4`: see the `SWD-SERIAL-EXTRACTION.4` commit (typed protocol-FSM state surface).

## Changelog

- `2026-06-07`: Created (owner decision (a) from `WIRE-BASED-100.5j`). `.1` research/characterization done.
  `.2` done — SWCLK/SWDIO/NSRST captured from the prose "`<role>` pin, `<SIGNAL>`" appositive.
  `.3` done (owner decision (ii)) — new typed `SerialFrameField` surface (bit-widths from `NAME[hi:lo]`,
  double-gated to serial docs + frame phases); 5 clean SWD frame fields; frontier = `.3b`.
