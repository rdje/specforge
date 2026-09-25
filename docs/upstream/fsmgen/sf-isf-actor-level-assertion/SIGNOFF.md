# Signoff — sf-isf-actor-level-assertion

Every box is ticked only with the evidence it names. `scripts/check_fsmgen_request_bundles.pl` refuses a
bundle whose `request.json` status is `signed_off` or later while any box here is open.

- [x] **CLASSIFIED** — the README (§3) cites 13d and 13k *Immediate verification checks* (*"a transaction-level
  clause"*): FSMGen follows its contract, so a capability is missing.
- [x] **REPRODUCED ON THE PIN AND ON UPSTREAM** — `request.json` `verified_on` records `54ac1c22` (pinned) and
  `354844fe1` (upstream) with identical strict-check results.
- [x] **STANDALONE** — `commands.sh` runs from the FSMGen root using only files inside this bundle, and
  `observed/` is the output of that script, not of a hand-run command.
- [x] **MINIMIZED** — five hand-written inputs, each stating one placement (README §4); no SPECFORGE corpus content
  is needed.
- [x] **EXPECTED IS TESTABLE** — `expected/expected-notes.md` states four outcomes a regression test can assert.
- [x] **NO DUPLICATE** — searched `docs/upstream/fsmgen/`, `docs/FSMGEN_FEEDBACK.md` with its archive, and FSMGen's
  `docs/SPECFORGE_FEEDBACK_RESPONSE.md` at both revisions; FSMGen's 2026-06-04 answer replaced the former
  `(contract …)` clause with the transaction-level verification family and did not address actor-level placement.
- [x] **PORTABLE** — no machine-local absolute path in any file; every redaction is listed in README §10.
- [x] **OWNED** — `request.json` names `EXTRACTION-REPRESENTATION-GAPS.15` and the open record
  `actor-level-assertion`.
- [x] **SPECFORGE SIDE SEPARATED** — lowering obligations as assertions is `EXTRACTION-REPRESENTATION-GAPS.11`,
  which can host them in existing transactions meanwhile; nothing here depends on it.

Signed off: `2026-09-25`, `EXTRACTION-REPRESENTATION-GAPS`.
