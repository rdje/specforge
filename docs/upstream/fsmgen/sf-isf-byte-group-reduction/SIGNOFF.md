# Signoff — sf-isf-byte-group-reduction

Every box is ticked only with the evidence it names. `scripts/check_fsmgen_request_bundles.pl` refuses a
bundle whose `request.json` status is `signed_off` or later while any box here is open.

- [x] **CLASSIFIED** — the README (§3) cites 13k *Rules and trigger fan-in* and *Data manipulation*, and the parser's
  arity diagnostic: FSMGen refuses a one-operand `^` as documented, so a capability is missing.
- [x] **REPRODUCED ON THE PIN AND ON UPSTREAM** — `request.json` `verified_on` records `54ac1c22` (pinned) and
  `82f0a5661` (upstream) with identical strict-check exit codes and diagnostics for all three inputs.
- [x] **STANDALONE** — `commands.sh` runs from the FSMGen root using only files inside this bundle, and
  `observed/` is the output of that script, not of a hand-run command.
- [x] **MINIMIZED** — three hand-written inputs over one 16-bit bus (README §4); no SPECFORGE corpus content is
  needed to reproduce.
- [x] **EXPECTED IS TESTABLE** — `expected/expected-notes.md` states four outcomes a regression test can assert,
  each on a named input and a named report or HDL value.
- [x] **NO DUPLICATE** — searched `docs/upstream/fsmgen/`, `docs/FSMGEN_FEEDBACK.md` with its archive, and FSMGen's
  `docs/SPECFORGE_FEEDBACK_RESPONSE.md` and book at `82f0a5661`: no request asks for a reduction; the book's
  *multibit reduction* is a guard's truthiness in `.fsm`; `sf-isf-derived-width-parameter` is related and sidestepped.
- [x] **PORTABLE** — no machine-local absolute path in any file; every redaction is listed in README §10.
- [x] **OWNED** — `request.json` names `FSMGEN-REQUESTS.12` and the open record `byte-group-reduction`.
- [x] **SPECFORGE SIDE SEPARATED** — the one-bit checks are lowered by `EXTRACTION-REPRESENTATION-GAPS.46.b`; the 49
  byte-granular ones wait on this under `EXTRACTION-REPRESENTATION-GAPS.46.c`.

Signed off: `2026-10-20`, `FSMGEN-REQUESTS`.
