# Signoff — sf-isf-unstated-reset

Every box is ticked only with the evidence it names. `scripts/check_fsmgen_request_bundles.pl` refuses a
bundle whose `request.json` status is `signed_off` or later while any box here is open.

- [x] **CLASSIFIED** — a question: the README (§1, §2) cites 13a *Reset* and *Multi-Clock Domains*; FSMGen behaves
  as documented in both cases, and the book is silent on a specification that states no reset.
- [x] **REPRODUCED ON THE PIN AND ON UPSTREAM** — `request.json` `verified_on` records `54ac1c22` (pinned) and
  `8513cef42` (upstream): all three inputs accepted on both.
- [x] **STANDALONE** — `commands.sh` runs from the FSMGen root using only files inside this bundle, and
  `observed/` is the output of that script, not of a hand-run command.
- [x] **MINIMIZED** — three hand-written inputs differing in their reset only (README §3); no SPECFORGE corpus
  content is needed to reproduce.
- [x] **EXPECTED IS TESTABLE** — `expected/expected-notes.md` states four outcomes an answer lets a regression test
  assert.
- [x] **NO DUPLICATE** — searched `docs/upstream/fsmgen/`, `docs/FSMGEN_FEEDBACK.md` with its archive, and FSMGen's
  `docs/SPECFORGE_FEEDBACK_RESPONSE.md` and book at `8513cef42`: no record discusses an unstated reset or the
  default's provenance.
- [x] **PORTABLE** — no machine-local absolute path in any file; every redaction is listed in README §6.
- [x] **OWNED** — `request.json` names `FSMGEN-REQUESTS.9` and the open record `unstated-reset`.
- [x] **SPECFORGE SIDE SEPARATED** — SPECFORGE's refusal to lower an unstated reset is its own adapter policy
  (README §4); acting on the answer is `FSMGEN-REQUESTS.9`'s integration step.

Signed off: `2026-09-29`, `FSMGEN-REQUESTS`.
