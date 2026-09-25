# Signoff — sf-isf-optional-signal

Every box is ticked only with the evidence it names. `scripts/check_fsmgen_request_bundles.pl` refuses a
bundle whose `request.json` status is `signed_off` or later while any box here is open.

- [x] **CLASSIFIED** — the README (§3) states the kind and cites 13a *Interface* and 13k *Interface ports*: FSMGen
  follows its contract (zero widths fail closed; no optional port), so a capability is missing.
- [x] **REPRODUCED ON THE PIN AND ON UPSTREAM** — `request.json` `verified_on` records `54ac1c22` (pinned) and
  `354844fe1` (upstream) with identical strict-check results.
- [x] **STANDALONE** — `commands.sh` runs from the FSMGen root using only files inside this bundle, and
  `observed/` is the output of that script, not of a hand-run command.
- [x] **MINIMIZED** — five hand-written inputs, each differing from `baseline-good.isf` in one stated place
  (README §4); no SPECFORGE corpus content is needed to reproduce.
- [x] **EXPECTED IS TESTABLE** — `expected/expected-notes.md` states five outcomes a regression test can assert.
- [x] **NO DUPLICATE** — searched `docs/upstream/fsmgen/`, `docs/FSMGEN_FEEDBACK.md` with its archive, and FSMGen's
  `docs/SPECFORGE_FEEDBACK_RESPONSE.md` at both revisions; no earlier record asks for optional ports.
- [x] **PORTABLE** — no machine-local absolute path in any file; every redaction is listed in README §10.
- [x] **OWNED** — `request.json` names `EXTRACTION-REPRESENTATION-GAPS.4` and the open record `optional-signal`.
- [x] **SPECFORGE SIDE SEPARATED** — SPECFORGE's half (reading presence properties) is
  `KG-ISF-COMPLETENESS.2a.vii.c.vi`; nothing here depends on it. The strict-mode observation is kept apart as a
  SPECFORGE question (README §4).

Signed off: `2026-09-25`, `EXTRACTION-REPRESENTATION-GAPS`.
