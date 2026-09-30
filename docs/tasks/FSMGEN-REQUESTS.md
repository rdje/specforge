# FSMGEN-REQUESTS: raise signed-off bug reports and feature requests to FSMGen, and track them to closure

## Metadata

- Tree ID: `FSMGEN-REQUESTS`
- Status: `active`
- Roadmap lane: `R0` (downstream contract hygiene) and the `KG-ISF-COMPLETENESS` lowering it unblocks
- Created: `2026-09-24`
- Last updated: `2026-09-26`
- Owner: repo-local workflow

## Goal

Every change SPECFORGE asks of FSMGen is classified against FSMGen's public contract, reproduced on both the
pinned and the current upstream FSMGen, packaged as a standalone bundle FSMGen can reproduce on its side,
signed off against a written and mechanically enforced standard, owned by a leaf here, tracked locally from
drafting to integration, and forwarded by the owner. The process is
[`docs/upstream/fsmgen/PROCESS.md`](../upstream/fsmgen/PROCESS.md).

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

- ID: `FSMGEN-REQUESTS` · Status: `active` · Children: `.0`, `.1`, `.2`, `.3`, `.4`, `.5`, `.6`, `.7`, `.8`, `.9`,
  `.10`, `.11`

- ID: `FSMGEN-REQUESTS.0` · Status: `done` (`2026-09-24`; PROCESS + CHECK) · Goal: **a signoff-level
  request process, enforced.** `docs/upstream/fsmgen/PROCESS.md` builds on FSMGen's published
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
  [`sf-isf-derived-width-parameter`](../upstream/fsmgen/sf-isf-derived-width-parameter/README.md), a
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
  [`sf-isf-undeclared-rule-operand`](../upstream/fsmgen/sf-isf-undeclared-rule-operand/README.md), a
  `bug_report` (`accepted-invalid-input`). `(rule r_a (A VALID))` with `VALID` declared nowhere passes
  `--strict --check`; FSMGen promotes `VALID` to an undeclared module input (`input wire VALID` in the
  generated SystemVerilog; semantic JSON identical to the declared baseline), sized from its first
  assignee, so widening one driven output turns it into a width mismatch reported on the wrong rule. Book 10
  lists "undeclared internal operands" among the errors pre-generation validation exists to catch.
  **Classification of the owner's question**: the finding is two defects. SPECFORGE lowering *"X must be
  valid"* to `(X VALID)` is SPECFORGE's (`KG-ISF-COMPLETENESS.2a.vii.e`); FSMGen accepting it against its
  contract is FSMGen's, and is this bundle. Reproduced on `c0d8b668` and `5a44ad49`.
  **Fixed in FSMGen's development line, not yet published** (`2026-09-25`, exports): refused from `c6b8bc783` on, for this bundle's value case and for the
  same read in a guard (`EXTRACTION-REPRESENTATION-GAPS.23`); closes with `FSMGEN-REFRESH-INTEGRATE-10`.

- ID: `FSMGEN-REQUESTS.3` · Status: `open` (`2026-09-24`; forwarded and **acknowledged** the same day —
  FSMGen keeps it as its revision-attribution ticket) ·
  Goal: **the issue-bundle helper never records a foreign revision.** Bundle
  [`sf-tool-issue-bundle-foreign-git-head`](../upstream/fsmgen/sf-tool-issue-bundle-foreign-git-head/README.md),
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

- ID: `FSMGEN-REQUESTS.5` · Status: `open` (`2026-09-26`; REQUEST — classify, reproduce, bundle, sign off; found by
  `EXTRACTION-GAP-FIX.7`) · Goal: **a storage register can state its own access.** FSMGen's `(field …)` takes
  `(access ro)`, but its storage `(var …)` does not: on the pin `54ac1c22`, TMC's emitted `.isf` with
  `(var sts (width 32))` rewritten to `(var sts (width 32) (access ro))` fails `--strict --check --json` with
  *"storage 'sts' has unsupported option 'access'"*. SPECFORGE now carries a register-level access for TMC's 50 of
  50 registers and ADIv6's 51 of 82 (`EXTRACTION-GAP-FIX.7`), 22 of TMC's with no field to carry it, and it never
  copies a register's access into an invented field, so the fact stops at IntentIR. Acceptance: classify against
  FSMGen's published contract (a feature request, or an existing spelling SPECFORGE should use); reproduce on the pin
  and on upstream HEAD; bundle and sign off per `PROCESS.md`; open the `FSMGEN_FEEDBACK.md` record; the owner
  forwards. SPECFORGE's lowering, once FSMGen ships a form, is a leaf of its own.
  **Signed off `2026-09-26`:** bundle
  [`sf-isf-storage-register-access`](../upstream/fsmgen/sf-isf-storage-register-access/README.md), a
  `feature_request`, classified against 13a *Declarative Storage Fields* and 13k *Actor-owned scalar storage*
  (identical at both revisions), with FSMGen's `2026-06-22` field-access answer as precedent. Five inputs: the
  primary `(var status (width 8) (reset 0) (access ro))`, the same beside `(fields …)`, an unknown token that must
  stay refused, the accepted whole-word-field workaround SPECFORGE declines, and the baseline. They give identical
  exit codes on the pin `54ac1c22` and upstream `5be6a3c08`, each export known-good-checked first. Measured need: 101
  registers (TMC 50 of 50, ADIv6 51 of 82), 98 with an FSMGen token and 61 with no field. Requested: R1 word access
  with the field token set, report-only; R2 independent of fields; R3 reported in `inferred_storage[]`; R4
  (optional) banks. Open record `storage-register-access`. **Forwarded and acknowledged `2026-09-26`**: FSMGen
  (relayed) finds it consistent with its contract, is reproducing the controls, and tracks scalar support apart
  from the optional bank extension; no task id yet. The leaf stays open until integration, which includes lowering SPECFORGE's
  `RegisterRecord.access_type` to `(access …)`.

- ID: `FSMGEN-REQUESTS.6` · Status: `done` (`2026-09-26`; INTAKE) · Goal: **record FSMGen's `2026-09-26` answer
  to the six feature requests and the question, and act on what it found.** FSMGen's
  `docs/SPECFORGE_FEEDBACK_RESPONSE.md` (commit `5be6a3c08`, section *2026-09-26: Configuration, Verification And
  Phase-Metadata Requests*) accepts all six requests. Each is owned by a leaf of its
  `SEMANTIC-INTROSPECTION-MCP-FRONTIER` tree: generic port widths `.33.3`, parameter value sets `.33.4`, optional
  signals `.33.5`, actor-level assertions `.33.6`, known-value predicates `.33.7`, phase groups `.33.8`. It answers
  the question: silently accepting an unknown or unimplemented port option is a defect it will fix (`.33.2`). Each
  of the seven packets is now `acknowledged`, with an `upstream_reference`, and its record `accepted`.
  **Two findings acted on here.** (1) FSMGen showed the optional-signal packet's criterion 2 was vacuous: its
  positive control never drives `AWMECID`, and an undriven output is dropped from the HDL (`.33.9`), so a
  byte-identical comparison could not show presence. SPECFORGE confirmed this on `54ac1c22` (semantic export) and
  `5be6a3c08` (generated module) and corrected the criterion to a port-list check; the forwarded inputs are
  unchanged. `present-when` drives `AWPROT`, so criterion 3 stands. (2) The same class of loss reaches SPECFORGE's
  own output. On complete read-only exports, SystemVerilog generated from the five current chains' emitted `.isf`
  files lacks **123 of 126** declared inputs on the pin `54ac1c22` (for example APB `PRDATA` and `PREADY`, AHB
  `HRDATA`, Wishbone `ACK_I`), though every file passes `--strict`. On `5be6a3c08`, which holds FSMGen's input repair
  `a7bf398ac`, none are missing. Outputs are complete on both, 255 of 255. `FSMGEN-REFRESH-INTEGRATE-10` now requires
  `a7bf398ac` in the tip and a realized-port inventory (`scripts/measure_isf_realized_ports.py`). SPECFORGE emits
  only `(width …)` on ports, a documented option, so FSMGen's `.33.2` repair will refuse nothing SPECFORGE writes.
  `git ls-remote` shows FSMGen's published `main` is still `54ac1c22`.
  **Found and fixed while landing:** the bundle checker's self-test case *status disagrees with its record* set the
  first packet to `acknowledged`. Once every packet was acknowledged, that mutation changed nothing, and the case
  failed on data rather than on a rule. It now moves the packet to a status whose record status differs from its own.

- ID: `FSMGEN-REQUESTS.7` · Status: `open` (`2026-09-29`; REQUEST — classify, reproduce, bundle, sign off; found by
  `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.c`) · Goal: **a duration stated in time units stays a duration.** USB 3.2
  states its link-training timeouts in ms and µs (`tPollingActiveTimeout` 12 ms, `tU0LTimeout` 10 µs), and I2C and I2S
  state bus timing in µs and ns. On the 31 chains 172 timing records are in time units. `.isf` accepts only cycle counts
  in `wait`, `watchdog` and `latency`, so SPECFORGE would have to invent a clock frequency. A static wait also lowers to
  one state per cycle: 234 cycles check in 3 s, 2,345 in 21 s, and 23,456 not in 90 s, while 12 ms at 125 MHz is
  1,500,000 cycles. Acceptance: bundle `sf-isf-duration-time-units`, reproduced on the pin and upstream HEAD, signed
  off, open ledger record; the owner forwards.

- ID: `FSMGEN-REQUESTS.8` · Status: `open` (`2026-09-29`; REQUEST; found by `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.c`)
  · Goal: **several named timers, whose expiry is behaviour.** USB 3.2's link layer runs timers concurrently with its
  state machine (`PENDING_HP_TIMER`, `CREDIT_HP_TIMER`, `PM_LC_TIMER`, `PM_ENTRY_TIMER`, `Ux_EXIT_TIMER`), and each of
  Table 7-12's 25 rows names the state a timer runs in and the state its expiry leads to. `.isf` has one watchdog
  counter per transaction (two distinct await limits fail closed), and a watchdog timeout is an error path
  (`last_error`), not a transition. Acceptance: bundle `sf-isf-named-timers`, reproduced, signed off, open record; the
  owner forwards.

- ID: `FSMGEN-REQUESTS.9` · Status: `open` (`2026-09-29`; QUESTION; found by `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.c`)
  · Goal: **learn which `.isf` form states "the specification names no reset pin".** USB 3.2 states no reset pin; its
  resets are protocol states (Warm Reset, Hot Reset) and its power-on state is a state machine state. An omitted `(reset
  …)` is accepted and silently becomes `(rst_n async active_low)`, and a single `(clock-domains …)` domain with no reset
  is accepted too; FSMGen's book does not say which of these, if either, states an unspecified reset honestly.
  Acceptance: question packet `sf-isf-unstated-reset`, reproduced, signed off, open record.

- ID: `FSMGEN-REQUESTS.10` · Status: `open` (`2026-09-29`; REQUEST; found by `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.c`)
  · Goal: **a packet-typed interface port.** USB 3.2's interfaces carry packets and ordered sets whose headers are field
  layouts (75 of the 191 untyped layout tables on the 31 chains are USB 3.2's). `.isf` accepts a record type on
  actor-owned storage only; a record-typed interface port fails closed. FSMGen named packet/flit layouts as a possible
  sibling of storage field maps (`2026-06-22`); this asks for the port half, with a consumer. Acceptance: bundle
  `sf-isf-aggregate-interface-ports`, reproduced, signed off, open record; the owner forwards.

- ID: `FSMGEN-REQUESTS.11` · Status: `done` (`2026-09-29`; PROCESS; found by `.7`-`.10`) · Goal: **the upstream
  export no longer blocks commits.** `PROCESS.md` §2 exported FSMGen's HEAD into `generated/tmp/`. The locality
  doctrine scans `generated/` for absolute paths, and FSMGen's own JSON files carry them, so the first commit of
  `.7`-`.10` failed `PROJECT-DATA-LOCALITY` with 290 values in the export alone. The export now goes under
  `.project-data/tmp/` and is removed once its bundle is copied; the 4.2 GB export of `8513cef42` was removed and the
  commit passed. Verification: `check_fsmgen_request_bundles.pl` and the full gate driver pass with no export
  present.

## Current Frontier

`.7`-`.10` — four requests found while measuring what USB 3.2's link layer needs from `.isf` (`2026-09-29`):
durations in time units, named timers, the unstated-reset question, packet-typed ports. Signed off here; the owner
forwards. The ledger's open-record cap moved 12 -> 16 for them (as 8 -> 12 did for `EXTRACTION-REPRESENTATION-GAPS.19`),
and its line target is owned by `LIVE-DOCUMENT-PRESSURE-HEADROOM.45`.

`.5` — `sf-isf-storage-register-access` forwarded and acknowledged `2026-09-26`; FSMGen is reproducing it.
The six requests and the question forwarded `2026-09-25` (owned in `EXTRACTION-REPRESENTATION-GAPS`) were accepted
by FSMGen on `2026-09-26` (`.6`).
`.1`, `.2`, `.3` — all forwarded and acknowledged (`2026-09-24`); FSMGen is working on them. Each advances when
a published FSMGen revision contains its change: pin it (`FSMGEN-REFRESH-INTEGRATE` procedure), re-run the
bundle's `commands.sh`, write `RESOLUTION.md`, close. The current pin `54ac1c22` predates all three.

## Changelog

- `2026-09-24`: tree opened on the owner's request; `.0` process and checker, `.1`-`.3` bundles signed off,
  `.4` determined.
- `2026-09-24`: the owner forwarded all three; FSMGen acknowledged them and is working on them.
- `2026-09-26`: `.5` opened by `EXTRACTION-GAP-FIX.7` (FSMGen's storage `var` refuses an `access` option).
- `2026-09-26`: `.6` — FSMGen accepted the six requests and answered the question; the optional-signal criterion
  corrected; 123 of 126 emitted inputs found missing from the pin's HDL and routed to `FSMGEN-REFRESH-INTEGRATE-10`.
- `2026-09-26`: `.5` signed off (`sf-isf-storage-register-access`); waits for the owner to forward it.
- `2026-09-26`: `.5` forwarded by the owner and acknowledged by FSMGen (relayed; scalar tracked apart from banks).
- `2026-09-29`: `.7`-`.10` opened and signed off: four packets for USB 3.2's link layer.
- `2026-09-29`: `.11` — the upstream export moves to `.project-data/tmp/` and is removed after bundling.
- `2026-09-30`: `.7`-`.10`'s rows added to the book's *Current packets* table, which their sign-off had missed (found by
  `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.g`); SpecForge now carries the transitions `sf-isf-named-timers` asks to lower.
