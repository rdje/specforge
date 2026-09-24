# Signoff — sf-tool-issue-bundle-foreign-git-head

Every box is ticked only with the evidence it names. `scripts/check_fsmgen_request_bundles.pl` refuses a
bundle whose `request.json` status is `signed_off` or later while any box here is open.

- [x] **CLASSIFIED** — the README states the kind and cites the FSMGen public-contract text that makes it a
  bug (FSMGen contradicts its contract) or a feature request (FSMGen follows it; a capability is missing).
- [x] **REPRODUCED ON THE PIN AND ON UPSTREAM** — `request.json` `verified_on` records both revisions and
  the result on each.
- [x] **STANDALONE** — `commands.sh` runs from the FSMGen repository root using only files inside this
  bundle, and `observed/` is the output of that script, not of a hand-run command.
- [x] **MINIMIZED** — every input is hand-written and differs from its companion in one stated place; no
  SPECFORGE corpus content is required to reproduce.
- [x] **EXPECTED IS TESTABLE** — `expected/expected-notes.md` states outcomes a regression test can assert.
- [x] **NO DUPLICATE** — searched `docs/fsmgen-issues/`, `docs/FSMGEN_FEEDBACK.md` and FSMGen's
  `docs/SPECFORGE_FEEDBACK_RESPONSE.md`; no earlier record covers this.
- [x] **PORTABLE** — no machine-local absolute path in any file; every redaction is listed in the README.
- [x] **OWNED** — `request.json` names the owning task leaf and the open `docs/FSMGEN_FEEDBACK.md` record.
- [x] **SPECFORGE SIDE SEPARATED** — any SPECFORGE defect involved is owned by its own leaf
  (`request.json` `specforge_side`), and nothing here depends on it.

Signed off: `2026-09-24`, `FSMGEN-REQUESTS`.
