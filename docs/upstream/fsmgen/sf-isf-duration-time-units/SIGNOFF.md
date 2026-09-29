# Signoff — sf-isf-duration-time-units

Every box is ticked only with the evidence it names. `scripts/check_fsmgen_request_bundles.pl` refuses a
bundle whose `request.json` status is `signed_off` or later while any box here is open.

- [x] **CLASSIFIED** — the README (§3) cites 13k *Single-clock timing*, *Await and latency* and *Static and dynamic
  waits*, and 13a *Clock*: FSMGen enforces cycle-count bounds as documented, so a capability is missing.
- [x] **REPRODUCED ON THE PIN AND ON UPSTREAM** — `request.json` `verified_on` records `54ac1c22` (pinned) and
  `8513cef42` (upstream) with identical strict-check exit codes and diagnostics for all four inputs.
- [x] **STANDALONE** — `commands.sh` runs from the FSMGen root using only files inside this bundle, and
  `observed/` is the output of that script, not of a hand-run command.
- [x] **MINIMIZED** — four hand-written inputs, each differing from `baseline-good.isf` in one place (README §4);
  no SPECFORGE corpus content is needed to reproduce.
- [x] **EXPECTED IS TESTABLE** — `expected/expected-notes.md` states four outcomes a regression test can assert,
  each on a named input and a named report value.
- [x] **NO DUPLICATE** — searched `docs/upstream/fsmgen/`, `docs/FSMGEN_FEEDBACK.md` with its archive, and FSMGen's
  `docs/SPECFORGE_FEEDBACK_RESPONSE.md` and book at `8513cef42`: no record asks for durations in time units or a
  declared clock frequency.
- [x] **PORTABLE** — no machine-local absolute path in any file; every redaction is listed in README §10.
- [x] **OWNED** — `request.json` names `FSMGEN-REQUESTS.7` and the open record `duration-time-units`.
- [x] **SPECFORGE SIDE SEPARATED** — SPECFORGE carries the durations in its IR today; lowering them, and
  representing USB 3.2's link layer at all, are SPECFORGE work named in `request.json` `specforge_side`.

Signed off: `2026-09-29`, `FSMGEN-REQUESTS`.
