# Signoff — sf-isf-strict-unknown-port-clause

Every box is ticked only with the evidence it names. `scripts/check_fsmgen_request_bundles.pl` refuses a
bundle whose `request.json` status is `signed_off` or later while any box here is open.

- [x] **CLASSIFIED** — the README (§2) cites book 10 *Strict Mode* and 13k *Interface ports*: the contract is
  silent on an unrecognised port clause, so this is a question (`PROCESS.md` §1, ADR 0054).
- [x] **REPRODUCED ON THE PIN AND ON UPSTREAM** — `request.json` `verified_on` records `54ac1c22` (pinned) and
  `354844fe1` (upstream) with identical results.
- [x] **STANDALONE** — `commands.sh` runs from the FSMGen root using only files inside this bundle, and
  `observed/` is its output.
- [x] **MINIMIZED** — three 13-line inputs differing in one port clause (README §3).
- [x] **EXPECTED IS TESTABLE** — `expected/expected-notes.md` states the assertion for either answer.
- [x] **NO DUPLICATE** — the only earlier mention is this question's own ledger record; FSMGen's
  `docs/SPECFORGE_FEEDBACK_RESPONSE.md` has no answer on port clauses.
- [x] **PORTABLE** — no machine-local absolute path in any file; redactions in README §6.
- [x] **OWNED** — `request.json` names `EXTRACTION-REPRESENTATION-GAPS.19` and the open record
  `strict-unknown-port-clause`.
- [x] **SPECFORGE SIDE SEPARATED** — SPECFORGE emits only documented port clauses (README §4); nothing waits on it.

Signed off: `2026-09-25`, `UPSTREAM-CHANNEL-UNIFICATION`.
