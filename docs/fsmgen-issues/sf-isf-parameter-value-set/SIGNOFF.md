# Signoff — sf-isf-parameter-value-set

Every box is ticked only with the evidence it names. `scripts/check_fsmgen_request_bundles.pl` refuses a
bundle whose `request.json` status is `signed_off` or later while any box here is open.

- [x] **CLASSIFIED** — the README (§3) cites 13k *Actor report metadata and params*: FSMGen enforces
  `(NAME value)` as documented, so a capability is missing.
- [x] **REPRODUCED ON THE PIN AND ON UPSTREAM** — `request.json` `verified_on` records `54ac1c22` (pinned) and
  `354844fe1` (upstream) with identical strict-check results.
- [x] **STANDALONE** — `commands.sh` runs from the FSMGen root using only files inside this bundle, and
  `observed/` is the output of that script, not of a hand-run command.
- [x] **MINIMIZED** — five hand-written inputs, each differing from `baseline-good.isf` only in the `DATA_WIDTH`
  entry (README §4); no SPECFORGE corpus content is needed to reproduce.
- [x] **EXPECTED IS TESTABLE** — `expected/expected-notes.md` states five outcomes a regression test can assert.
- [x] **NO DUPLICATE** — searched `docs/fsmgen-issues/`, `docs/FSMGEN_FEEDBACK.md` with its archive, and FSMGen's
  `docs/SPECFORGE_FEEDBACK_RESPONSE.md` at both revisions; no earlier record asks for parameter value sets.
- [x] **PORTABLE** — no machine-local absolute path in any file; every redaction is listed in README §10.
- [x] **OWNED** — `request.json` names `EXTRACTION-REPRESENTATION-GAPS.5` and the open record `parameter-value-set`.
- [x] **SPECFORGE SIDE SEPARATED** — SPECFORGE already carries the values (`KG-ISF-COMPLETENESS.2a.vii.c.ii`,
  `.c.iii`); emitting them once this ships is `EXTRACTION-REPRESENTATION-GAPS.5`'s integration step.

Signed off: `2026-09-25`, `EXTRACTION-REPRESENTATION-GAPS`.
