# FSMGen Feature Request: a duration stated in time units, lowered to a counter

Title: sf-isf-duration-time-units — `.isf` takes durations only as cycle counts, and a static wait lowers to one state per cycle; a controller specified in milliseconds cannot be written without inventing a clock frequency
FSMGen command or public API entrypoint: ./bin/fsmgen --strict --check --json sources/fsmgen-input/wait-in-ms.isf
FSMGen-facing primary artifact path: sources/fsmgen-input/wait-in-ms.isf
Failure class: rejected-input
FSMGen commit or capability-manifest producer commit: 8513cef4254db4572ab8dd3c94f3953df358f0a9 (upstream HEAD, 2026-09-28); also reproduced on SPECFORGE's pin 54ac1c22d5314943027494f4c270f9c612e62704 — see "Revision note"
SPECFORGE version/commit: specforge 6d2eb6e60
Expected behavior: a duration stated in a time unit, (wait (time 12 ms)), is accepted wherever a cycle count is (wait, await watchdog, latency) once the actor declares its clock frequency; it lowers to a counter of the derived cycle count, not to one state per cycle, and the schedule report carries the stated duration, the frequency and the derived count
Observed behavior: --strict --check --json refuses (wait (time 12 ms)) as an unsupported runtime wait-count expression, (await ready (watchdog (time 10 us))) with "await watchdog override requires '(watchdog N)'", and (clock clk (frequency CLK_HZ)) with "(clock ...) requires exactly one name"; the accepted cycle-count form lowers a 234-cycle static wait to 236 states with no counter
First failing command or API call: original (./bin/fsmgen --strict --check --json CASE)
Exit status or exception: 1 (observed/command-logs/original.exit); JSON diagnostic in observed/stdout/original.stdout
Is the attached artifact bundle minimized: yes
Does the minimized/redacted bundle still reproduce: yes

Request kind: **feature request** (FSMGen follows its current public contract; the contract lacks a capability).
SPECFORGE owner: `FSMGEN-REQUESTS.7`; ledger record `duration-time-units`.
Local status: see `request.json` (`signed_off`; waiting for the owner to forward it).

## 1. What SPECFORGE needs

Link-layer and bus specifications state a controller's timers in physical time, not in cycles of a clock they do
not define. USB 3.2 Revision 1.0, *Table 7-12. LTSSM State Transition Timeouts*, states for example:

| Name | Initial State | Timeout to Next State | Timeout Values |
| --- | --- | --- | --- |
| `tRxDetectQuietTimeoutDFP` | `Rx.Detect.Quiet` | `Rx.Detect.Active` | 12 ms (min) 120 ms (max) |
| `tPollingActiveTimeout` | `Polling.Active` | `Rx.Detect` / `Polling.PortMatch` / … | 12 ms (x1) 24 ms (x2) |
| `tU0LTimeout` | `U0` | `U0` | 10 µs |

*Table 7-7* and *Table 7-8* state the link-layer timers in µs (`PENDING_HP_TIMER` 10 µs, `Ux_EXIT_TIMER` 6000 µs).
The specification states no clock frequency for any of them: the frequency is the implementation's choice. A
faithful `.isf` must therefore keep "12 ms" as 12 ms and let the integrator's clock turn it into cycles.

## 2. Measured need (SPECFORGE corpus, 2026-09-29)

On SPECFORGE's 31 current chains, **172 timing records** are stated in time units (ns 87, µs 44, ms 36, ps 4,
s 1): I2C 85, USB 3.2 71, I2S 11, OpenCAPI PHY 5. USB 3.2's 34 timeouts and timers (*Tables 7-7, 7-8, 7-12*) are
controller behaviour: a state is left when its timer expires. Re-derive in a SPECFORGE checkout:

```bash
python3 -c 'import json,collections
units = collections.Counter()
for k in json.load(open("doctrine/chain_currency/retained_bundles.json"))["retained"]:
    for t in json.load(open(f"generated/evidence_ir/{k}/evidence_ir.json")).get("timing_constraints", []):
        u = (t.get("unit") or "").strip()
        if u in ("s", "ms", "us", "µs", "μs", "ns", "ps"): units[u] += 1
print(sum(units.values()), dict(units))'
```

## 3. Current public contract (why this is a feature request, not a bug)

- `docs/book/src/13k-isf-feature-support-matrix.md`, rows *Single-clock timing* (`(clock clk)` plus optional
  `(reset …)` and `(watchdog N)`), *Await and latency* and *Static and dynamic waits*: every bound is a positive
  integer, a declared constant, a parameter default or a package constant; none takes a unit.
- The same row *Static and dynamic waits*: *"Static waits create explicit wait states unless zero-count bypass
  semantics apply. Runtime waits use generated counters."* The baseline confirms it (§4): a 234-cycle wait is
  236 states and `counter_signal: null`.
- `docs/book/src/13a-actor-interface.md`, *Clock*: `(clock clk)` names one clock signal; nothing states its
  frequency or period.

Both sections are identical at the pin and at upstream HEAD. FSMGen's existing parameters are the precedent this
builds on: a wait count may already be an actor parameter, so the integrator can set it at instantiation; this asks
FSMGen to derive that count from a stated duration instead of from a number the integrator computes by hand.

## 4. Reproduction

Run from the FSMGen root: `bash <bundle>/commands.sh`. Each input differs from `baseline-good.isf` in one place.

| Input | Difference | `--strict --check` | Result |
| --- | --- | --- | --- |
| `baseline-good.isf` | `(params (QUIET_CYCLES 234))`, `(wait QUIET_CYCLES)` | exit 0 | accepted; lowers to 236 states, no counter |
| `wait-in-ms.isf` (primary) | `(params (CLK_HZ 125000000))`, `(wait (time 12 ms))` | exit 1 | *runtime dynamic wait count expression '(time 12 ms)' must use a supported expression operator shape* |
| `watchdog-in-us.isf` | `(await ready (watchdog (time 10 us)))` | exit 1 | *await watchdog override requires '(watchdog N)'* |
| `clock-frequency.isf` | `(clock clk (frequency CLK_HZ))` with `(wait (time 12 ms))` | exit 1 | *(clock ...) requires exactly one name* |

Scale, measured on the pin with the baseline's shape: a static wait of 234 cycles checks in 3 s, 2,345 in 21 s,
and 23,456 did not finish in 90 s. USB 3.2's 12 ms at a 125 MHz clock is 1,500,000 cycles.

## 5. Requested capability

- **R1 (required) — durations in time units.** A duration literal, for example `(time 12 ms)` (the spelling is
  FSMGen's choice), accepted wherever a cycle count is accepted: `(wait …)`, an await `(watchdog …)`,
  `(latency (min …) (max …))` and a temporal monitor's bound. Units: `s`, `ms`, `us`, `ns`, `ps`. Failure cases: an
  unknown unit, a zero or negative duration, or a non-numeric value fails closed naming the clause.
- **R2 (required) — a declared frequency.** An actor (or clock domain) may declare its clock frequency or period,
  as a literal or an actor parameter so the integrator sets it at instantiation. A duration in an actor without a
  declared frequency fails closed naming the duration.
- **R3 (required) — a stated conversion.** The book states how a duration becomes cycles. A wait or timeout must
  never be shorter than stated, so it rounds up; a `max` bound must never be longer than stated, so it rounds down.
  The schedule report carries the stated duration, the frequency and the derived cycle count for each one.
- **R4 (required) — a counter, not states.** A duration lowers to a counter of the derived width, so its cost does
  not grow with the cycle count.
- **R5 (optional) — a stated range.** A duration given as a range (`12 ms` min, `120 ms` max) as an
  implementation-chosen value within the range, for example a parameter with a legal range (compare
  `sf-isf-parameter-value-set`).

## 6. Acceptance criteria (proposed, for FSMGen's regression tests)

1. `wait-in-ms.isf`, with a frequency declared as R2 specifies, passes `--strict --check`; its schedule report
   carries `12 ms`, `125000000` and `1500000` for the wait, and the scheduled `.fsm` uses a counter, not 1,500,000
   states.
2. `watchdog-in-us.isf` with the same declaration passes; the await's watchdog limit is reported as 10 µs and
   1,250 cycles.
3. A duration in an actor with no declared frequency, and an unknown unit, each fail under `--strict` with a
   diagnostic naming the clause.
4. `baseline-good.isf` keeps its current result and schedule report.

## 7. Alternatives SPECFORGE considered and why they are not faithful

- **Pick a frequency and emit cycle counts**: states a clock the specification never states, and the result is
  wrong on any other clock.
- **Emit cycle-count parameters with no unit**: the integrator must redo the conversion by hand, nothing checks it,
  and the stated 12 ms is gone from the artifact.
- **A runtime wait on a stored count**: a counter, but its value is still a hand-made cycle count, and it adds
  storage the specification does not state.
- **Keep durations in SPECFORGE's IR** (what SPECFORGE does today): no FSMGen artifact carries them.

## 7a. What SPECFORGE does until this ships

SPECFORGE keeps every timing record in its IR with its stated unit (`TimingConstraintRecord`) and lowers none of
them. USB 3.2 is classed `physical-link` in SPECFORGE's document categories, whose default is not to generate
executable output; its owner asked on `2026-09-29` for these link-layer requests, which ask only for the
controller's behavioural timers, not for electrical parameters.

## 8. Compatibility

Additive: every currently accepted input keeps its meaning; only forms FSMGen refuses today gain one.

## 9. Revision note

Reproduced on FSMGen upstream HEAD `8513cef4254db4572ab8dd3c94f3953df358f0a9` (a read-only `git archive` export,
where `commands.sh` ran and generated `observed/`) and on SPECFORGE's pin
`54ac1c22d5314943027494f4c270f9c612e62704` (strict checks of the same four inputs: identical exit codes and
diagnostics). The contract text cited in §3 is identical at the two revisions. Each installation passed a
known-good strict check (`sf-isf-storage-register-access/expected/baseline-good.isf`: exit 0) before anything was
compared. The scale timings in §4 were measured on the pin.

## 10. Redactions and simplifications

- Absolute paths in captured outputs are replaced by `<FSMGEN_ROOT>` (the FSMGen export the helper ran in),
  `<SPECFORGE_ROOT>`, `<FSMGEN_CHECKOUT>` and `<HOME>`. No other byte of any captured output changed.
- `env.txt`'s *FSMGen git HEAD* and `observed/command-logs/fsmgen-git-head.txt` are corrected by hand: the helper
  recorded SPECFORGE's commit, the defect reported as `sf-tool-issue-bundle-foreign-git-head`.
- `commands.sh` is the helper's script plus the SPECFORGE-added companion loop at its end.
- The USB 3.2 rows in §1 are copied from SPECFORGE's SourceIR of the tables; no corpus content is needed to
  reproduce.
