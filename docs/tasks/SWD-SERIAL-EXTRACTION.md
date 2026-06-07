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
- ID: `SWD-SERIAL-EXTRACTION.2` · Status: `pending` · Goal: capture `SWCLK`/`SWDIO` (and the JTAG pins) as
  canonical interface signals from the prose "`<role>` pin, `<SIGNAL>`" pattern (general, ADR-0006 — grammar
  not names), so they enter the declared catalog and become available to relations/temporal.
- ID: `SWD-SERIAL-EXTRACTION.3` · Status: `pending` · Goal: model the SWD serial-frame protocol
  (start/stop/park/turnaround/parity bits + WAIT/FAULT/OK ACK) as typed sequence/temporal facts.
- ID: `SWD-SERIAL-EXTRACTION.4` · Status: `pending` · Goal: recover the DP/AP register-access interface from
  the register_map/encoding tables (the architecture-side intent).
- ID: `SWD-SERIAL-EXTRACTION.5` · Status: `pending` · Goal: build the SWD constraint/relation/temporal golds
  from real prose, measure, and fix to `P=R=F1=1.000` (per-fact), no parallel-bus regression.

## Current frontier

- `SWD-SERIAL-EXTRACTION.2` — capture SWCLK/SWDIO from prose (the foundation; relations/temporal depend on it).

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

## Commit log

- `.1`: see the `SWD-SERIAL-EXTRACTION.1` commit (tree opened + research leaf).

## Changelog

- `2026-06-07`: Created (owner decision (a) from `WIRE-BASED-100.5j`). `.1` research/characterization done;
  frontier = `.2` (capture SWCLK/SWDIO from prose).
