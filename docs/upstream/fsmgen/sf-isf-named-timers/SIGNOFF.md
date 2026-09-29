# Signoff — sf-isf-named-timers

Every box is ticked only with the evidence it names. `scripts/check_fsmgen_request_bundles.pl` refuses a
bundle whose `request.json` status is `signed_off` or later while any box here is open.

- [x] **CLASSIFIED** — the README (§3) cites 13a *Watchdog*, 13b's await semantics and 13k *Await and latency*:
  FSMGen enforces one watchdog per transaction with error semantics as documented, so a capability is missing.
- [x] **REPRODUCED ON THE PIN AND ON UPSTREAM** — `request.json` `verified_on` records `54ac1c22` (pinned) and
  `8513cef42` (upstream) with identical strict-check exit codes and diagnostics for all three inputs.
- [x] **STANDALONE** — `commands.sh` runs from the FSMGen root using only files inside this bundle, and
  `observed/` is the output of that script, not of a hand-run command.
- [x] **MINIMIZED** — three hand-written inputs (README §4); no SPECFORGE corpus content is needed to reproduce.
- [x] **EXPECTED IS TESTABLE** — `expected/expected-notes.md` states four outcomes a regression test can assert,
  each on a named input and a named report value or simulated behaviour.
- [x] **NO DUPLICATE** — searched `docs/upstream/fsmgen/`, `docs/FSMGEN_FEEDBACK.md` with its archive, and FSMGen's
  `docs/SPECFORGE_FEEDBACK_RESPONSE.md` and book at `8513cef42`: no record asks for author-declared timers; the book
  defers per-await limits (R6) without a request behind it.
- [x] **PORTABLE** — no machine-local absolute path in any file; every redaction is listed in README §10.
- [x] **OWNED** — `request.json` names `FSMGEN-REQUESTS.8` and the open record `named-timers`.
- [x] **SPECFORGE SIDE SEPARATED** — carrying each timer's next state is `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.g`;
  lowering timers once this ships is `FSMGEN-REQUESTS.8`'s integration step.

Signed off: `2026-09-29`, `FSMGEN-REQUESTS`.
