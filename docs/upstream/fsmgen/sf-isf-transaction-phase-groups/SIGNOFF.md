# Signoff — sf-isf-transaction-phase-groups

Every box is ticked only with the evidence it names. `scripts/check_fsmgen_request_bundles.pl` refuses a
bundle whose `request.json` status is `signed_off` or later while any box here is open.

- [x] **CLASSIFIED** — the README (§3) cites FSMGen's documented transaction clauses and its 2026-06-16 answer,
  which names this as a future feature: a capability is missing.
- [x] **REPRODUCED ON THE PIN AND ON UPSTREAM** — `request.json` `verified_on` records `54ac1c22` (pinned) and
  `354844fe1` (upstream) with identical strict-check results.
- [x] **STANDALONE** — `commands.sh` runs from the FSMGen root using only files inside this bundle, and
  `observed/` is the output of that script, not of a hand-run command.
- [x] **MINIMIZED** — four hand-written inputs around one APB write transfer (README §4); no SPECFORGE corpus
  content is needed.
- [x] **EXPECTED IS TESTABLE** — `expected/expected-notes.md` states three outcomes a regression test can assert.
- [x] **NO DUPLICATE** — the 2026-06-16 exchange (SPECFORGE archive `docs/archive/fsmgen-feedback/`, FSMGen
  `docs/SPECFORGE_FEEDBACK_RESPONSE.md`) was a question answered with the direction and a condition (*"if this
  becomes load-bearing"*); no request was filed. This is that request, with the measured need.
- [x] **PORTABLE** — no machine-local absolute path in any file; every redaction is listed in README §10.
- [x] **OWNED** — `request.json` names `EXTRACTION-REPRESENTATION-GAPS.20` and the open record
  `transaction-phase-groups`.
- [x] **SPECFORGE SIDE SEPARATED** — SPECFORGE already extracts the membership (`KG-ISF-TRANSACTIONS.2i`); emitting
  it once shipped is `EXTRACTION-REPRESENTATION-GAPS.20`'s integration step.

Signed off: `2026-09-25`, `EXTRACTION-REPRESENTATION-GAPS`.
