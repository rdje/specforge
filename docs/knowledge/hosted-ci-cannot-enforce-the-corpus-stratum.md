---
id: hosted-ci-cannot-enforce-the-corpus-stratum
title: Hosted CI enforces 13 of 18 doctrines; the 5 quantified over the persisted corpus have no subject on a runner and are reported NOT GOVERNED
answers:
  - "does GitHub CI run the full doctrine gate"
  - "why does check_doctrines.sh skip CHAIN-CURRENCY on CI"
  - "what does SPECFORGE_CORPUS_ABSENT do"
  - "why does the gate refuse when generated/source_ir is missing"
  - "can the corpus be rebuilt on a CI runner"
  - "why does actions/checkout need fetch-depth 0 and submodules"
  - "what does a green hosted CI run actually prove"
date: 2026-09-20
status: current
tags: [commit-gate-single-run, doctrine-enforcement, hosted-ci, corpus, coverage]
evidence: .github/workflows/ci.yml; scripts/check_doctrines.sh; scripts/check_chain_currency.sh (absent-corpus skip); docs/tasks/COMMIT-GATE-SINGLE-RUN.md (.13/.14/.14a)
reverify: "count generated/source_ir entries whose source.requested_path is a tracked file — expect 21 of 78, the other 57 path_origin=external_input; in a fresh clone, SPECFORGE_CORPUS_ABSENT=1 bash scripts/check_doctrines.sh --all — expect 'ALL 13 executed doctrines PASS (18 registered)' and a NOT GOVERNED line naming 5."
---

A green hosted CI run proves **13 of 18 doctrines**, not the gate. The other five are quantified over
the persisted corpus, and a runner cannot have one. `PRODUCTION-GENERICITY` is among the thirteen
because it declares corpus dependency per COMPONENT: 9 of its 11 components run on a corpus-free
tree, and only `BEHAVIORAL-CONTRACT` and `BEHAVIORAL-CONTRACT-MUTATIONS` are reported NOT GOVERNED.

`/generated/` is gitignored, so a fresh checkout has no artifacts at all. It also cannot rebuild
them: `generated/source_ir` holds **78** documents, of which only **21** were built from a source
tracked in `corpus/`; the other **57** record `path_origin: external_input` under
`.cache/local-references/` and have no in-repo source at any price. (The repository tracks 22 PDFs,
one of which — NVMe 2.0a — has no persisted SourceIR, so 22 and 21 count different things.) The
corpus stratum — `CHAIN-CURRENCY`,
`PROOF-SEAL-CURRENCY`, `PROOF-SEAL-TOTAL`, `CORPUS-FRONTIER`, and the corpus components of
`PRODUCTION-GENERICITY` and `CLAIM-VERIFICATION` — is enforced on a developer machine only.

The two failure modes that hides behind are opposite, and both were live on `2026-09-20`:

| stratum | on a corpus-free tree | what the report used to say |
| --- | --- | --- |
| `CHAIN-CURRENCY`, both `PROOF-SEAL-*`, `CORPUS-FRONTIER` | enforcer skips loudly, exits 0 | **`PASS`** — the skip lines were dropped |
| `PRODUCTION-GENERICITY`, `CLAIM-VERIFICATION` | enforcer fails on missing artifacts | `FAIL`, unfixable on a runner |

The enforcers were already honest — `check_chain_currency.sh` prints *"Nothing was measured; nothing
is claimed"* and `check_proof_seal_currency.sh` pins that in self-test case 16. The driver lost it:
its relay of a passing enforcer's output filtered for `warning`, so a declared non-applicability
never reached the one report the hook, `run_ci.sh` and COMMIT.md step 8 all print.

Two things follow, and both are mechanical now:

- **An absent corpus is a refusal.** `scripts/check_doctrines.sh` exits 2 when
  `generated/source_ir` is empty unless the environment declares itself corpus-free with
  `SPECFORGE_CORPUS_ABSENT=1` (set in `.github/workflows/ci.yml`). The declaration is a permission,
  not an override — where a corpus is present it changes nothing.
- **The gate is history- and submodule-dependent**, so `actions/checkout` defaults are both wrong.
  `fetch-depth: 0` because `PUBLISHED-ASSERTIONS` anchors values to named revisions and
  `LIVE-DOC-SIZE` reads boundary commits; `submodules: recursive` because `feedback-protocol` reads
  evidence under `subs/fsmgen`. At the defaults these produce *"revision … does not resolve in this
  repository"* and *"git exit 128"*, which look like content failures and are not.

Whether the corpus stratum should get a hosted subject — a small tracked fixture corpus whose
sources are in `corpus/` — is open as `COMMIT-GATE-SINGLE-RUN.14a`. Do not close it by widening the
skip. See [[gate-and-replay-costs-are-affordable]].
