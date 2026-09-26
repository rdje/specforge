# Signoff — sf-isf-storage-register-access

Every box is ticked only with the evidence it names. `scripts/check_fsmgen_request_bundles.pl` refuses a
bundle whose `request.json` status is `signed_off` or later while any box here is open.

- [x] **CLASSIFIED** — the README (§3) cites 13a *Declarative Storage Fields* and 13k *Actor-owned scalar
  storage*: FSMGen enforces the documented `var` options, so a capability is missing.
- [x] **REPRODUCED ON THE PIN AND ON UPSTREAM** — `request.json` `verified_on` records `54ac1c22` (pinned) and
  `5be6a3c08` (upstream) with identical strict-check exit codes for all five inputs.
- [x] **STANDALONE** — `commands.sh` runs from the FSMGen root using only files inside this bundle, and
  `observed/` is the output of that script, not of a hand-run command.
- [x] **MINIMIZED** — five hand-written inputs, each differing from `baseline-good.isf` in one storage entry
  (README §4); no SPECFORGE corpus content is needed to reproduce.
- [x] **EXPECTED IS TESTABLE** — `expected/expected-notes.md` states four outcomes a regression test can assert,
  each on a named input and a named report field.
- [x] **NO DUPLICATE** — searched `docs/upstream/fsmgen/`, `docs/FSMGEN_FEEDBACK.md` with its archive, and FSMGen's
  `docs/SPECFORGE_FEEDBACK_RESPONSE.md` at `5be6a3c08`; the `2026-06-22` field-structured storage exchange asked
  for and got access on fields only, and no record asks for a register's own access.
- [x] **PORTABLE** — no machine-local absolute path in any file; every redaction is listed in README §10.
- [x] **OWNED** — `request.json` names `FSMGEN-REQUESTS.5` and the open record `storage-register-access`.
- [x] **SPECFORGE SIDE SEPARATED** — SPECFORGE already carries the register access (`EXTRACTION-GAP-FIX.7`);
  lowering it once this ships is `FSMGEN-REQUESTS.5`'s integration step.

Signed off: `2026-09-26`, `FSMGEN-REQUESTS`.
