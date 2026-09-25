# FSMGEN-REQUESTS: raise signed-off bug reports and feature requests to FSMGen, and track them to closure

## Metadata

- Tree ID: `FSMGEN-REQUESTS`
- Status: `active`
- Roadmap lane: `R0` (downstream contract hygiene) and the `KG-ISF-COMPLETENESS` lowering it unblocks
- Created: `2026-09-24`
- Last updated: `2026-09-24`
- Owner: repo-local workflow

## Goal

Every change SPECFORGE asks of FSMGen is classified against FSMGen's public contract, reproduced on both the
pinned and the current upstream FSMGen, packaged as a standalone bundle FSMGen can reproduce on its side,
signed off against a written and mechanically enforced standard, owned by a leaf here, tracked locally from
drafting to integration, and forwarded by the owner. The process is
[`docs/fsmgen-issues/PROCESS.md`](../fsmgen-issues/PROCESS.md).

## Non-Goals

- Patching, inspecting the implementation of, or re-pinning `subs/fsmgen` (standing doctrine: submodules are
  black boxes). Verification against upstream uses a read-only `git archive` export under `generated/tmp/`.
- Forwarding to FSMGen. The owner forwards; this tree records that it happened.
- Fixing the SPECFORGE half of any finding; each has its own owner, named in the request.

## Acceptance Criteria

- `PROCESS.md` defines classification, verification, bundle build, README shape for both kinds,
  `request.json`, lifecycle, signoff and closure.
- `scripts/check_fsmgen_request_bundles.pl` enforces the signoff standard inside `LIVE-DOC-SIZE`, with a
  refused-then-accepted demonstration for each rule class it checks.
- Each request in this tree reproduces from its own `commands.sh` on the pin and on upstream HEAD, and has an
  open record in `docs/FSMGEN_FEEDBACK.md`.
- The mdBook tells the owner what is pending to forward and how the process works.

## Task Tree

- ID: `FSMGEN-REQUESTS` · Status: `active` · Children: `.0`, `.1`, `.2`, `.3`, `.4`

- ID: `FSMGEN-REQUESTS.0` · Status: `done` (`2026-09-24`; PROCESS + CHECK) · Goal: **a signoff-level
  request process, enforced.** `docs/fsmgen-issues/PROCESS.md` builds on FSMGen's published
  `DOWNSTREAM_ISSUE_REPORTING.md` and adds what it leaves open: a classification table (bug / feature /
  SPECFORGE defect / question), verification on the pin **and** on upstream HEAD, a README shape per kind, a
  machine-readable `request.json`, an eight-state lifecycle mapped onto the `FSMGEN_FEEDBACK.md` statuses,
  a nine-box `SIGNOFF.md`, and forwarding/closure steps. `scripts/check_fsmgen_request_bundles.pl` (run by
  `scripts/check_live_document_size.sh`, so every commit pays it) refuses a bundle whose layout, summary
  fields, `request.json`, signoff boxes, pin+upstream verification, owner leaf, open feedback record or
  portability is wrong. The three pre-process bundles are exempt as closed history.
  **Why upstream as well as the pin**: the pin trails upstream HEAD by 951 commits at `2026-09-24`, and the
  withdrawn `sf-isf-explicit-fsm-declaration` is what filing against a stale pin costs.
  **Found and fixed while landing**: the existing `check_fsmgen_feedback_protocol.pl --self-test` rendered its
  fixture's open region as a hard-coded "None" while reading the open ids from the real contract, so its
  "valid migrated contract" case failed the first time any record was open. The fixture now renders one
  minimal record per declared id (11/11 pass; an empty ledger still renders "None"). Also: the checker's own
  `--self-test` runs an 11-case refusal matrix and only counts a refusal that names the mutated bundle; deleting
  one rule from a copy of the checker makes it fail.
  **Scratch hygiene**: the upstream export (4.2 GB) and probe directories under `generated/tmp/` were removed
  once the bundles held everything; the locality gate had flagged the export's own absolute paths.

- ID: `FSMGEN-REQUESTS.1` · Status: `open` (`2026-09-24`; forwarded and acknowledged — FSMGen is working on it) ·
  Goal: **FSMGen can state a derived width.** Bundle
  [`sf-isf-derived-width-parameter`](../fsmgen-issues/sf-isf-derived-width-parameter/README.md), a
  `feature_request`. FSMGen's `.isf` actor surface refuses every spelling of a width derived from a
  parameter — a param expression, a constant expression, an inline width, a ceiling-division param — while
  its `.fsm` `+size` / `+params` surfaces already evaluate such expressions (book 04, 06; six stable
  `FSMGEN_LANGUAGE_*SIZE*` diagnostics in its capability manifest). 56 live SPECFORGE signals need it
  (`DATA_WIDTH/8`, `ceil(X/8)`, `clog2(N)`, …). Requested: R1 derived actor parameters that stay symbolic
  through instantiation, R2 ceiling division, R3 `clog2`, R4 (optional) inline width expressions.
  Reproduced on `c0d8b668` (pin) and `5a44ad49` (upstream). SPECFORGE integration once shipped:
  `KG-ISF-COMPLETENESS.2a.vii.d`. Closes at `integrated`, `rejected` or `withdrawn`.

- ID: `FSMGEN-REQUESTS.2` · Status: `open` (`2026-09-24`; forwarded and acknowledged — FSMGen is working on it) ·
  Goal: **strict mode rejects an undeclared rule operand.** Bundle
  [`sf-isf-undeclared-rule-operand`](../fsmgen-issues/sf-isf-undeclared-rule-operand/README.md), a
  `bug_report` (`accepted-invalid-input`). `(rule r_a (A VALID))` with `VALID` declared nowhere passes
  `--strict --check`; FSMGen promotes `VALID` to an undeclared module input (`input wire VALID` in the
  generated SystemVerilog; semantic JSON identical to the declared baseline), sized from its first
  assignee, so widening one driven output turns it into a width mismatch reported on the wrong rule. Book 10
  lists "undeclared internal operands" among the errors pre-generation validation exists to catch.
  **Classification of the owner's question**: the finding is two defects. SPECFORGE lowering *"X must be
  valid"* to `(X VALID)` is SPECFORGE's (`KG-ISF-COMPLETENESS.2a.vii.e`); FSMGen accepting it against its
  contract is FSMGen's, and is this bundle. Reproduced on `c0d8b668` and `5a44ad49`.
  **Fixed upstream** (`2026-09-25`, exports): refused from `c6b8bc783` on, for this bundle's value case and for the
  same read in a guard (`EXTRACTION-REPRESENTATION-GAPS.23`); closes with `FSMGEN-REFRESH-INTEGRATE-10`.

- ID: `FSMGEN-REQUESTS.3` · Status: `open` (`2026-09-24`; forwarded and **acknowledged** the same day —
  FSMGen keeps it as its revision-attribution ticket) ·
  Goal: **the issue-bundle helper never records a foreign revision.** Bundle
  [`sf-tool-issue-bundle-foreign-git-head`](../fsmgen-issues/sf-tool-issue-bundle-foreign-git-head/README.md),
  a `bug_report` (`wrong-output`). Found while building `.1` and `.2`: run from a non-git copy of FSMGen inside
  another repository, `bin/fsmgen-issue-bundle` writes that repository's commit as "FSMGen git HEAD" (both
  bundles first recorded SPECFORGE's `343bd7fa`), while the capability manifest correctly says `unknown`.
  The protocol's own triage contract (§9) depends on the revision being right. `commands.sh` builds the
  scratch enclosing repository itself; reproduced on `c0d8b668` and `67c8b36d` (upstream HEAD moved during
  the session). Until fixed, `PROCESS.md` step 4 corrects the head by hand.
  All three requests were re-verified on the new pin `54ac1c22` (`FSMGEN-REFRESH-INTEGRATE-9`) and reproduce
  unchanged. FSMGen's reply also set a standing rule for comparing revisions (complete installations, a
  known-good check on both first), adopted as `PROCESS.md` §2.

- ID: `FSMGEN-REQUESTS.4` · Status: `done` (`2026-09-24`; DETERMINATION) · Goal: **decide whether "checks can
  pass for the wrong reason" is an FSMGen report.** It is not; no bundle. Both instances were SPECFORGE's own
  verification tooling, and FSMGen was not involved in either. (1) `specforge eval-extraction` prints the
  evidence path of a fresh randomly named temporary directory (`.project-data/tmp/specforge-XXXXXX/…`), so
  two runs of the same oracle differ textually and a byte comparison reports a difference that is not a
  score change. That is a SPECFORGE reproducibility defect, owned by `WIRE-BASED-100.11`. (2) The HEAD binary
  refused a chain rebuilt by the new binary (`topology is stale`) and scored nothing; that refusal is correct
  proof-carrying behaviour, and the method rule — score each binary against the chain it produced — is
  already the resume pointer's standing instruction and `KG-ISF-COMPLETENESS.2a.vii.c.i`'s record.

## Current Frontier

`.1`, `.2`, `.3` — all forwarded and acknowledged (`2026-09-24`); FSMGen is working on them. Each advances when
a published FSMGen revision contains its change: pin it (`FSMGEN-REFRESH-INTEGRATE` procedure), re-run the
bundle's `commands.sh`, write `RESOLUTION.md`, close. The current pin `54ac1c22` predates all three.

## Changelog

- `2026-09-24`: tree opened on the owner's request; `.0` process and checker, `.1`-`.3` bundles signed off,
  `.4` determined.
- `2026-09-24`: the owner forwarded all three; FSMGen acknowledged them and is working on them.
