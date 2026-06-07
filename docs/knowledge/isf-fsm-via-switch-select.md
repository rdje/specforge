---
id: isf-fsm-via-switch-select
title: ISF CAN describe a state machine — proven idiom is storage-var + switch + select + rule-trigger (FSMGen lowers it)
answers:
  - "can ISF model an explicit state machine / FSM (proven)"
  - "how to express the JTAG TAP / SWD FSM in .isf"
  - "what ISF idiom describes states and input-driven transitions"
  - "does SpecForge cycle-schedule the FSM (no — FSMGen does)"
  - "why was the ISF explicit-FSM feature request withdrawn"
date: 2026-06-07
tags: [isf, fsmgen, swd, jtag, fsm, state-machine, proven, no-feature-request]
evidence: docs/fsmgen-issues/sf-isf-explicit-fsm-declaration/README.md (WITHDRAWN); subs/fsmgen @ d31b0b91; empirical fsmgen --strict --check --json probes
reverify: "write a state machine as (storage (var st ...)) + (transaction step (on start) (switch st (S (select st input A B))...) (complete done)) + (rule tick start (trigger step)); run subs/fsmgen/bin/fsmgen --strict --check --json FILE → success:true"
---

**ISF (IAL1) can accurately describe a finite state machine** — proven empirically against the current
FSMGen (`subs/fsmgen` @ `d31b0b91`; the submodule had been 312 commits stale, which caused a premature,
now-WITHDRAWN feature request). SpecForge expresses the FSM *intent* in `.isf`; **FSMGen owns
cycle-scheduling and lowers `.isf` → `.fsm`** — SpecForge does NOT cycle-schedule.

**The working idiom** (a 6-state JTAG TAP-DR FSM lowers clean: `--strict --check --json` → `success:true`):

```lisp
(actor jtag_tap_dr
  (clock tck) (reset (rst_n async active_low)) (watchdog 65536)
  (interface (input start) (input tms) (output shifting))
  (storage (var tap (width 3) (reset 0)))          ;; the state register (state-name encoding)
  (transaction step
    (on start)
    (switch tap                                     ;; dispatch on current state
      (0 (select tap tms 1 0))                      ;; per-state input-driven transition:
      (1 (select tap tms 1 2))                      ;;   (select STATE GUARD NEXT_IF_1 NEXT_IF_0)
      (2 (select tap tms 4 3))
      (3 (select tap tms 4 3))
      (4 (select tap tms 5 3))
      (5 (select tap tms 1 0)))
    (complete done))
  (rule tick start (trigger step)))                 ;; perpetual: re-trigger every active cycle
```

**Empirical boundary found (use the idiom above; avoid these):**
- `switch` INSIDE a `while`/loop body → unsupported ("unsupported `(switch ...)` clause in while body").
- One rule per transition writing a shared state var → `isf_conflicting_rule_writes` (the conflict checker
  does NOT prove disjoint state/input guards, even for fully disjoint `st==0` vs `st==1`). So do NOT model
  transitions as competing rules.
- `cond`/nested conditions inside a `switch` branch → unsupported nested body.
- `(select …)` is a **transaction** data-op action, NOT a rule action (rules need `(port expr)`).
- The reset must use a supported spelling (`(rst_n async active_low)`); `(trst …)` → strict rejects the
  `(areset …)` mapping.

Conclusion: **no ISF/FSMGen feature request is warranted.** The SWD/JTAG TAP FSM is expressed with shipped
constructs (storage var + `switch` + `select` + rule `trigger`); see `[[swd-protocol-fsm-surface]]`. Lesson:
`[[feedback_verify_fsmgen_before_fr]]` — update the submodule and EMPIRICALLY test before claiming any
FSMGen capability gap (`[[feedback_isf_no_hacks]]`).
