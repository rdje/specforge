# Signoff — sf-isf-generic-port-widths

Every box is ticked only with the evidence it names. `scripts/check_fsmgen_request_bundles.pl` refuses a
bundle whose `request.json` status is `signed_off` or later while any box here is open.

- [x] **CLASSIFIED** — the README (§3) cites 13k *Interface ports* (*"resolved positive integer widths"*) and
  *Reusable ISF libraries* (*"parameter-driven shape elaboration … remain backlog"*): FSMGen follows its contract,
  so a capability is missing; the unused emitted parameter is left to FSMGen to classify.
- [x] **REPRODUCED ON THE PIN AND ON UPSTREAM** — `request.json` `verified_on` records `54ac1c22` (pinned, strict
  checks) and `354844fe1` (upstream, `commands.sh` with generation).
- [x] **STANDALONE** — `commands.sh` runs from the FSMGen root using only files inside this bundle, and
  `observed/` is the output of that script, not of a hand-run command.
- [x] **MINIMIZED** — three hand-written inputs: the fixed-width baseline and two parameter-sized variants that
  differ only in the default (README §4); no SPECFORGE corpus content is needed.
- [x] **EXPECTED IS TESTABLE** — `expected/expected-notes.md` states four outcomes a regression test can assert.
- [x] **NO DUPLICATE** — searched `docs/upstream/fsmgen/`, `docs/FSMGEN_FEEDBACK.md` with its archive, and FSMGen's
  `docs/SPECFORGE_FEEDBACK_RESPONSE.md` at both revisions; `sf-isf-derived-width-parameter` presumes this base case
  and is related, not a duplicate (README §3).
- [x] **PORTABLE** — no machine-local absolute path in any file; every redaction is listed in README §10.
- [x] **OWNED** — `request.json` names `EXTRACTION-REPRESENTATION-GAPS.6` and the open record `generic-port-widths`.
- [x] **SPECFORGE SIDE SEPARATED** — SPECFORGE already emits `(width P)` with a declared default
  (`KG-ISF-COMPLETENESS.2a.vii.b`); it needs no change for R1-R2 beyond re-verifying once shipped.

Signed off: `2026-09-25`, `EXTRACTION-REPRESENTATION-GAPS`.
