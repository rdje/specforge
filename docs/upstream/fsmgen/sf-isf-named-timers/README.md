# FSMGen Feature Request: several named timers, whose expiry is behaviour

Title: sf-isf-named-timers — an `.isf` actor has one watchdog counter per transaction, and a watchdog timeout is an error path; a controller whose states end on their own timers, with other timers running beside them, cannot be written
FSMGen command or public API entrypoint: ./bin/fsmgen --strict --check --json sources/fsmgen-input/named-timers.isf
FSMGen-facing primary artifact path: sources/fsmgen-input/named-timers.isf
Failure class: rejected-input
FSMGen commit or capability-manifest producer commit: 8513cef4254db4572ab8dd3c94f3953df358f0a9 (upstream HEAD, 2026-09-28); also reproduced on SPECFORGE's pin 54ac1c22d5314943027494f4c270f9c612e62704 — see "Revision note"
SPECFORGE version/commit: specforge 6d2eb6e60
Expected behavior: an actor may declare several named timers, start, restart and stop them in transaction bodies, and test their expiry as an ordinary condition, so that a timer running beside other behaviour can lead to a stated next state rather than to an error; several timers may run at once
Observed behavior: --strict --check --json refuses (timers (timer pending_hp 1250) (timer credit_hp 625000)) with "unknown actor clause 'timers'"; two awaits with distinct watchdog limits in one transaction fail closed with "multiple distinct await watchdog limits are not supported by the current single watchdog counter"; the accepted single-watchdog baseline lowers its timeout to a state that pulses done and sets last_error
First failing command or API call: original (./bin/fsmgen --strict --check --json CASE)
Exit status or exception: 1 (observed/command-logs/original.exit); JSON diagnostic in observed/stdout/original.stdout
Is the attached artifact bundle minimized: yes
Does the minimized/redacted bundle still reproduce: yes

Request kind: **feature request** (FSMGen follows its current public contract; the contract lacks a capability).
SPECFORGE owner: `FSMGEN-REQUESTS.8`; ledger record `named-timers`.
Local status: see `request.json` (`signed_off`; waiting for the owner to forward it).

## 1. What SPECFORGE needs

A link-layer controller keeps several timers at once, and their expiry is normal behaviour, not an error. USB 3.2
Revision 1.0 states:

- **Link-layer timers** (*Table 7-7*, *Table 7-8*): `PENDING_HP_TIMER` (10 µs), `CREDIT_HP_TIMER` (5000 µs),
  `PM_LC_TIMER` (4 µs in x1 operation, 8 µs in x2), `PM_ENTRY_TIMER`, `Ux_EXIT_TIMER`, `U1_MIN_RESIDENCY_TIMER`.
  A header packet whose acknowledgement (`LGOOD_n`) is not received when `PENDING_HP_TIMER` expires sends the
  link to Recovery.
- **State timeouts** (*Table 7-12. LTSSM State Transition Timeouts*): 25 rows, each naming the state a timer runs
  in, the state its expiry leads to, and its value (`tPollingActiveTimeout`: in `Polling.Active`, 12 ms, then
  `Rx.Detect`, …).

So one actor needs several timers running beside its state machine, each started on entry to some state or on
sending some packet, and each expiry moving the machine to a stated state. Today's `.isf` offers one watchdog per
transaction, bound to an await, whose expiry ends the transaction with `last_error`.

## 2. Measured need (SPECFORGE corpus, 2026-09-29)

USB 3.2's current chain carries **34 timing records** from those three tables: 26 state timeouts from the 24 rows
that state a duration (two of them per link width; the 25th row's value refers to a packet field), each conditioned
on the state the timer runs in, and 8 link-layer timer values.
Re-derive in a SPECFORGE checkout:

```bash
python3 -c 'import json
e = json.load(open("generated/evidence_ir/usb_3_2_revision_1_0_2017_09/evidence_ir.json"))
rows = [t for t in e["timing_constraints"] if set(t.get("supporting_table_ids", [])) & {"table_0087", "table_0088", "table_0094"}]
print(len(rows), sum(1 for t in rows if t.get("row_conditions")))'
```

## 3. Current public contract (why this is a feature request, not a bug)

- `docs/book/src/13a-actor-interface.md`, *Watchdog*: *"Global timeout for every `(await ...)` in this actor."*
- `docs/book/src/13b-transactions.md` (await semantics): *"One transaction currently has one watchdog counter. If a
  transaction contains multiple awaits with distinct effective watchdog limits, FSMGen fails closed until per-await
  counter reset semantics are selected"*, and on timeout: *"request the delayed `done` pulse, set `last_error=1`,
  return to idle"*.
- `docs/book/src/13k-isf-feature-support-matrix.md`, *Await and latency*: *"One transaction has one watchdog
  counter, so distinct per-await limits in one transaction fail closed."* No row names a timer the author declares.

All three passages are identical at the pin and at upstream HEAD. The watchdog is the precedent: FSMGen already
generates and reports a down-counter per transaction (`{transaction}_wd`); this asks for counters the author names
and whose expiry the author gives a meaning.

## 4. Reproduction

Run from the FSMGen root: `bash <bundle>/commands.sh`.

| Input | Difference from `baseline-good.isf` | `--strict --check` | Result |
| --- | --- | --- | --- |
| `baseline-good.isf` | one `(await ack (watchdog 1250))` | exit 0 | accepted; its timeout state `pending_timeout` pulses `done` and sets `last_error` (`observed/stdout/baseline-good-schedule.stdout`) |
| `named-timers.isf` (primary) | `(timers (timer pending_hp 1250) (timer credit_hp 625000))`, `(start-timer pending_hp)`, `(when (expired pending_hp) …)` | exit 1 | *unknown actor clause 'timers'* |
| `two-await-limits.isf` | a second `(await credit (watchdog 625000))` in the same transaction | exit 1 | *multiple distinct await watchdog limits are not supported by the current single watchdog counter* |

The spellings in `named-timers.isf` are illustrative; the syntax is FSMGen's choice.

## 5. Requested capability

- **R1 (required) — declared timers.** An actor declares named timers, each with a limit in cycles (and, if
  `sf-isf-duration-time-units` ships, a duration). Failure cases: a duplicate name, a zero limit, or a reference to
  an undeclared timer fails closed naming it.
- **R2 (required) — control.** Transaction bodies and rules may start, restart and stop a timer.
- **R3 (required) — expiry as a condition.** A timer's expiry is a condition usable wherever a condition is (`when`,
  `switch`, rule guards, await alternatives), so it can lead to any stated next step. It is not an error unless the
  author says so.
- **R4 (required) — concurrency.** Several timers may run at once and independently of the transaction that
  started them, including across transactions of the same actor.
- **R5 (required) — reported.** The schedule report lists each timer, its limit, its counter width and where it is
  started, stopped and tested.
- **R6 (optional) — per-await limits.** Distinct watchdog limits for awaits in one transaction, the case the book
  defers today, if R1–R4 do not already cover it.

## 6. Acceptance criteria (proposed, for FSMGen's regression tests)

1. `named-timers.isf`, in FSMGen's selected spelling, passes `--strict --check`; its schedule report lists
   `pending_hp` (1250) and `credit_hp` (625000) with their counter widths.
2. A body that starts both timers and tests each expiry reaches the tested branch when that timer's count runs out,
   and the other timer keeps counting.
3. Referring to an undeclared timer, or declaring one twice, fails under `--strict` naming the timer.
4. `baseline-good.isf` keeps its current result and schedule report; its watchdog timeout still sets `last_error`.

## 7. Alternatives SPECFORGE considered and why they are not faithful

- **The await watchdog**: one per transaction, bound to one await, and its expiry is an error that ends the
  transaction; the specification's expiry is a normal transition.
- **Explicit counters in storage with increment and compare rules**: invents storage and arithmetic the
  specification does not state, and nothing reports them as timers.
- **One transaction per timer**: splits one state machine's behaviour across transactions with no stated
  coordination, and still yields error semantics on expiry.
- **Keep timers in SPECFORGE's IR** (what SPECFORGE does today): no FSMGen artifact carries them.

## 7a. What SPECFORGE does until this ships

SPECFORGE carries each timer's value, unit and the state it runs in as timing records, and lowers none of them.
USB 3.2 is classed `physical-link` in SPECFORGE's document categories, whose default is not to generate executable
output; its owner asked on `2026-09-29` for these link-layer requests. Carrying the state each timer's expiry leads
to is SPECFORGE work of its own (`EXTRACTION-REPRESENTATION-GAPS.3.b.iii.g`).

## 8. Compatibility

Additive: the watchdog keeps its meaning; only an actor clause FSMGen refuses today gains one.

## 9. Revision note

Reproduced on FSMGen upstream HEAD `8513cef4254db4572ab8dd3c94f3953df358f0a9` (a read-only `git archive` export,
where `commands.sh` ran and generated `observed/`) and on SPECFORGE's pin
`54ac1c22d5314943027494f4c270f9c612e62704` (strict checks of the same three inputs: identical exit codes and
diagnostics). The contract text cited in §3 is identical at the two revisions. Each installation passed a
known-good strict check before anything was compared.

## 10. Redactions and simplifications

- Absolute paths in captured outputs are replaced by `<FSMGEN_ROOT>` (the FSMGen export the helper ran in),
  `<SPECFORGE_ROOT>`, `<FSMGEN_CHECKOUT>` and `<HOME>`. No other byte of any captured output changed.
- `env.txt`'s *FSMGen git HEAD* and `observed/command-logs/fsmgen-git-head.txt` are corrected by hand: the helper
  recorded SPECFORGE's commit, the defect reported as `sf-tool-issue-bundle-foreign-git-head`.
- `commands.sh` is the helper's script plus the SPECFORGE-added companion loop at its end.
- The USB 3.2 values in §1 are copied from SPECFORGE's SourceIR of the tables; no corpus content is needed to
  reproduce.
