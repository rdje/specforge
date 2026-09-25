# Signoff — sf-isf-known-value-predicate

Every box is ticked only with the evidence it names. `scripts/check_fsmgen_request_bundles.pl` refuses a
bundle whose `request.json` status is `signed_off` or later while any box here is open.

- [x] **CLASSIFIED** — the README (§3) cites 13k *Immediate verification checks*, which lists the supported
  predicates; `known` is not among them, so a capability is missing.
- [x] **REPRODUCED ON THE PIN AND ON UPSTREAM** — `request.json` `verified_on` records `54ac1c22` (pinned) and
  `354844fe1` (upstream) with identical exit codes.
- [x] **STANDALONE** — `commands.sh` runs from the FSMGen root using only files inside this bundle, and
  `observed/` is the output of that script, not of a hand-run command.
- [x] **MINIMIZED** — three hand-written inputs differing only in the predicate (README §4); no SPECFORGE corpus
  content is needed.
- [x] **EXPECTED IS TESTABLE** — `expected/expected-notes.md` states four outcomes a regression test can assert.
- [x] **NO DUPLICATE** — searched `docs/fsmgen-issues/`, `docs/FSMGEN_FEEDBACK.md` with its archive, and FSMGen's
  `docs/SPECFORGE_FEEDBACK_RESPONSE.md` at both revisions; `sf-isf-undeclared-rule-operand` is related (validity
  written as a value) and is not this request (README §3).
- [x] **PORTABLE** — no machine-local absolute path in any file; every redaction is listed in README §10.
- [x] **OWNED** — `request.json` names `EXTRACTION-REPRESENTATION-GAPS.16` and the open record
  `known-value-predicate`.
- [x] **SPECFORGE SIDE SEPARATED** — never writing validity as a value is `KG-ISF-COMPLETENESS.2a.vii.e`; nothing
  here depends on it.

Signed off: `2026-09-25`, `EXTRACTION-REPRESENTATION-GAPS`.
