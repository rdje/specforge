# Signoff — sf-isf-aggregate-interface-ports

Every box is ticked only with the evidence it names. `scripts/check_fsmgen_request_bundles.pl` refuses a
bundle whose `request.json` status is `signed_off` or later while any box here is open.

- [x] **CLASSIFIED** — the README (§3) cites 13j and 13k *Interface ports* and its residual list: FSMGen refuses
  aggregate ports as documented, so a capability is missing.
- [x] **REPRODUCED ON THE PIN AND ON UPSTREAM** — `request.json` `verified_on` records `54ac1c22` (pinned) and
  `8513cef42` (upstream) with identical strict-check exit codes and diagnostics for all three inputs.
- [x] **STANDALONE** — `commands.sh` runs from the FSMGen root using only files inside this bundle, and
  `observed/` is the output of that script, not of a hand-run command.
- [x] **MINIMIZED** — three hand-written inputs sharing one record type (README §4); no SPECFORGE corpus content
  is needed to reproduce.
- [x] **EXPECTED IS TESTABLE** — `expected/expected-notes.md` states four outcomes a regression test can assert,
  each on a named input and a named report or HDL value.
- [x] **NO DUPLICATE** — searched `docs/upstream/fsmgen/`, `docs/FSMGEN_FEEDBACK.md` with its archive, and FSMGen's
  `docs/SPECFORGE_FEEDBACK_RESPONSE.md` and book at `8513cef42`: the `2026-06-22` exchange left packet layouts an
  open sibling of storage fields, and no request asks for record-typed ports.
- [x] **PORTABLE** — no machine-local absolute path in any file; every redaction is listed in README §10.
- [x] **OWNED** — `request.json` names `FSMGEN-REQUESTS.10` and the open record `aggregate-interface-ports`.
- [x] **SPECFORGE SIDE SEPARATED** — reading layout tables into records is `DOC-INTENT-TAXONOMY.4b`; lowering them
  once this ships is `FSMGEN-REQUESTS.10`'s integration step.

Signed off: `2026-09-29`, `FSMGEN-REQUESTS`.
