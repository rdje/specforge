# SpecForge toolbox — stage replay, gates, build, and host (§7)

> A part of SpecForge's diagnostic toolbox. The standing directive, the enforcement and
> acceptance-checklist contract, the quick chooser, the first-reach tools (§1-§4), and the
> diagnosis protocols stay in the landing, [`TOOLBOX.md`](../../TOOLBOX.md). Section numbering
> is continuous with it, so a citation like `TOOLBOX.md` §7.7 still names this entry.

## 7. Stage replay, build, and host

### 7.1 `scripts/run_ci.sh` — the full gate
- **WHAT:** the canonical local/hosted gate: the doctrine driver (memory-arch + knowledge-map +
  task-acceptance), `cargo fmt --all --check`, warning-deny clippy, warning-deny tests, rustdoc, mdBook.
- **WHEN:** before committing any Rust code change (the NO-REGRESSION oracle leg).
- **HOW:** `bash scripts/run_ci.sh`

### 7.2 `scripts/check_doctrines.sh` — the doctrine gate only
- **WHAT:** the registry/driver alone. The default `gate` tier is fast (structural + evidence checks,
  no heavy build) and reports every `ci`-tier doctrine as `DEFER` so none is silently absent; `--all`
  also runs the CI-tier doctrines and is what `run_ci.sh` invokes.
- **WHEN:** before any commit; what the pre-commit hook runs.
- **HOW:** `bash scripts/check_doctrines.sh` / `bash scripts/check_doctrines.sh --all`
- **ONE DOCTRINE:** `--only ID[,ID...]` (and `--list` for the ids). **Use this instead of invoking a
  gate script by hand** — the driver runs each enforcer exactly as the hook does, with no arguments,
  and asserts the exit status, so no caller has to know a per-script flag. An unrecognised id is
  refused rather than matching nothing, every unselected doctrine is reported `SKIP` rather than
  omitted, and a subset run prints `SUBSET ONLY … this is NOT the gate` so it can never be mistaken
  for a full one (`COMMIT-GATE-SINGLE-RUN.1`).
- **THE EARLY SIGNAL:** `--fast` runs the gate tier minus the measured costliest four (`LIVE-DOC-SIZE`,
  `PROJECT-DATA-LOCALITY`, `PROOF-SEAL-CURRENCY`, `PRODUCTION-GENERICITY`). It is what `COMMIT.md` step 8
  runs on EVERY slice; the full driver is never run by hand, because with `G` the cost of one gate a manual
  full run costs `G+G` on pass against `G` for letting the hook be the only full run, and `G+fix+G` either
  way on failure (`COMMIT-GATE-SINGLE-RUN.2`/`.3`). A Rust slice adds the oracles this driver never runs —
  it neither compiles nor tests — and `--only PRODUCTION-GENERICITY` when the producer graph could move.
  The subset is declared as an **exclusion** in the driver, so a newly registered doctrine is in the fast
  set automatically, and the list is meta-checked against the registry on every run, so a renamed doctrine
  cannot leave a dangling exclusion. `--fast` **refuses when the pre-commit hook is not active**, because
  the premise of a subset run is that the hook pays for the rest. **A green `--fast` is not a green gate:**
  of the five doctrines evidenced blocking a commit on `2026-09-17` it runs three and omits
  `LIVE-DOC-SIZE` and `PROJECT-DATA-LOCALITY`, and it prints the omitted list on every run for that reason.

### 7.2-i `scripts/measure_doctrine_cost.sh` — what the gate costs, per doctrine
- **WHAT:** wall-clock cost of every registered doctrine, run one at a time. It DERIVES its population
  by parsing the driver's `DOCTRINES=(...)` registry, so a newly registered doctrine cannot be missing
  from the table. Emits a Markdown table on stdout and per-doctrine progress on stderr.
- **WHEN:** before changing what the commit path runs. It **refuses above load average 2.0** (override with
  `IDLE_MAX`/`ALLOW_CONTENDED`, and say so wherever the numbers go) because a contended run times
  contention: three runs of the same tree at load 12.95 gave 335s / 429s / 524s, one doctrine moving 2.4x
  while a single-threaded one held at 1.02x. **Read the verdict column too:** a doctrine that FAILS is timed
  at the cost of its first error, so a table with any FAIL row is not a measurement
  (`COMMIT-GATE-SINGLE-RUN.0` measured `CLAIM-VERIFICATION` at 0.8s that way; it is ~31.8s). **Quote
  membership, not shares** — across three runs the costliest four were the same four, but their shares and
  their internal ordering were not.
- **HOW:** `bash scripts/measure_doctrine_cost.sh` / `--all` to include the CI tier

### 7.2-ii `scripts/probe_exec_assessment_latency.sh` — is the gate stalled, or is the host?

**Symptom.** A doctrine step produces no output for minutes and the whole process tree sits at ~0%
CPU. That is indistinguishable from a hang, and on `2026-09-19` it cost a session forty minutes, an
aborted commit, and a wrong first diagnosis (a pipe deadlock in the driver).

**First, read the driver's own progress.** `check_doctrines.sh` names each doctrine on stderr before
running it, and after a notice interval (`SPECFORGE_DOCTRINE_STALL_SECONDS`, default 120) prints which
one is still going and what to do about it. A run that finishes names every step that passed the
interval under `---- SLOW ----` with its measured cost, so "the gate is slow" becomes "this doctrine
took N seconds" without re-running anything.

**Then decide between the two causes, because they have different owners:**

```bash
bash scripts/probe_exec_assessment_latency.sh
```

It times the exec of brand-new scripts against the re-exec of one already assessed. A large gap is the
host assessing newly created executables — the gate creates fixture scripts by the hundred, and each
first exec can block for minutes while a security daemon inspects it. Confirm with
`ps -Ao pid,pcpu,time,comm -r | head -5` and look for `XprotectService` / `syspolicyd`. A small gap
**excludes** that cause, and the slowness belongs to the step itself.

The probe measures rather than asserts, on purpose: the numbers move with the daemon's backlog, which
is exactly why the condition is intermittent and why a pinned figure would be the wrong thing to
carry. `--self-test` covers the timer, the freshness of each sample, the verdict, and residue.

Owner: `GATE-FIXTURE-EXEC-STALL`. `.1`/`.2` remove the repository's share of the cost — the gate asks
the operating system to assess a new executable per fixture for a handful of script contents that
never change.

### 7.2a `scripts/check_chain_currency.sh` — the CHAIN-CURRENCY oracle (CI-tier)
- **WHAT:** replays every persisted corpus artifact `--dry-run` from its persisted input (evidence,
  semantic, intent, `.isf` adapter, plus each emitted `.isf` against the adapter's rendered
  `source_text`) and fails when the persisted artifact is not what the current binary produces.
  `validation_reports` is excluded exactly as the product's `*_ir_fingerprint` helpers exclude it.
  Its second leg compares `doctrine/chain_currency/retained_bundles.json` with the normalized bundles
  actually on disk: a declared bundle that is gone, or a bundle no leaf declared, fails closed.
- **WHEN:** after any shared-extractor or stage change, and before signing off a refresh — it is the
  measurement ADR 0025 decision 1 requires before attributing a delta. Skips loudly with no corpus.
- **HOW:** `bash scripts/check_chain_currency.sh`
- **SELF-TEST:** `--self-test` runs 22 fail-closed cases before any PASS is trusted.

### 7.2a-i `scripts/check_proof_seal_currency.sh` — the PROOF-SEAL-CURRENCY gate (gate-tier)
- **WHAT:** reads the `ruleset_sha256` every persisted artifact records at all five chain stages for the
  proof-carrying stratum — a **total** census — then asks the current build's own canonical loader whether
  it still accepts that seal. The probe's scope is **per stage**: `semantic` and `intent` are probed
  TOTALLY (every censused artifact, active by default since `CORPUS-CHAIN-CURRENCY.9`), while `source-ir`
  and `evidence` — whose probes replay extraction and cost an order of magnitude more — get one
  **representative** per *distinct* seal and the check says it is blind there. The probe is
  the CONSUMING stage in `--dry-run`; never `specforge validate`, which is not idempotent and would
  invalidate the chain it claims to read. The terminal `adapters/isf` stage has no consumer, so it is
  censused and reported UNPROBED rather than counted as a pass.
- **COST:** **1m12.7s** over the 27-document stratum, of which ~65 s is the two TOTAL stages; the whole
  gate-tier driver is **5m24.5s-5m30.1s** (it was 7.4 s and 4m13.0s with the TOTAL stages sampled). `--total`,
  which probes the extraction-replaying stages too, is **1m59.2s** and stays CI tier.
- **WHEN:** automatically, on every commit through the doctrine driver — that is the point. Run it by hand
  after editing a stage root or `derivation.rs` if you want the answer before the hook gives it to you.
  Skips loudly and passes with no corpus.
- **LIMIT:** a current seal is **not** content currency. Whether a persisted artifact is still what the
  current binary reproduces stays `CHAIN-CURRENCY`'s question at CI tier.
- **HOW:** `bash scripts/check_proof_seal_currency.sh`.
- **SELF-TEST:** `--self-test` runs 21 fail-closed cases, the last of which pins the shipped
  TOTAL-stage default itself, so emptying it goes RED.
- **REMEDY SELF-TEST:** `bash scripts/rebuild_stage_cascade.sh --self-test` runs 14 fail-closed cases.
  Every count on these three lines is re-derived on every commit from the script that declares it
  (`perl scripts/report_self_test_totals.pl`), because all three were carried by hand and all three
  went stale: `CLAIM-VERIFICATION-ADOPTION.7.3`.
- **ON A STALE SEAL:** `source_proof_migrate --write` for SourceIR (proof-only), or
  `scripts/rebuild_stage_cascade.sh --write` for every stage below it (a real content rebuild).
- **WHICH BUILD ANSWERS:** this check, `check_chain_currency.sh` and `rebuild_stage_cascade.sh` all
  replay the persisted corpus against the current build, so they share one binary predicate —
  `scripts/lib/corpus_replay_binary.sh`, the **release** profile since `CORPUS-CHAIN-CURRENCY.8`. Always
  say which profile a probe cost was measured with: one `intent --dry-run` over a 39.7 MB artifact is
  **6.5 s** at release and **63.1 s** at debug, and the whole sampled check is 7.4 s against 15.9 s. The
  verdict is the same either way — measured byte-identical, same SHA-256, over a 43 MB IntentIR.

### 7.2b `scripts/check_corpus_frontier.sh` — the CORPUS-FRONTIER derive-and-diff gate
- **WHAT:** derives the corpus cohort from each document's persisted SourceIR and diffs it against the explicit,
  disjoint `refreshed`/`remaining` partition in `doctrine/corpus_frontier/census.json`. It checks exact identity,
  lifecycle membership, retained-bundle agreement, whole-cohort omission, and the counts stated by the root task.
  Source locations never classify lifecycle state, so a library move cannot impersonate a current-binary refresh.
  A refresh must move the declaration in the same transaction or this fails closed.
- **WHEN:** to answer "how many documents are left, really" — never read a carried number. Also the fastest
  way to confirm a refresh's bookkeeping landed. Skips loudly with no corpus.
- **HOW:** `bash scripts/check_corpus_frontier.sh`
  (`perl scripts/check_corpus_frontier_census.pl --report` for the JSON census;
  `--self-test` for its ten fail-closed cases)

### 7.2c `scripts/check_production_genericity.sh` — the PRODUCTION-GENERICITY structural gate
- **WHAT:** composes the compiler-visible package-direction check, exact production module/claim inventory,
  exact rule/field/producer/seam/bypass joins, and the compiled graph's fixed-point raw/identity flow plus
  proof-only canonical-authority analysis. All four components run and report even when one fails.
- **WHEN:** before any production extraction, proof, persistence, or package-boundary change; it also runs
  unconditionally through the doctrine driver and pre-commit hook.
- **HOW:** `bash scripts/check_production_genericity.sh` for the fast clean-tree doctrine;
  `bash scripts/check_production_genericity.sh --self-test` for the CI qualification matrix and exact
  inventory-to-runtime alpha-obligation join over all 170 rules.
- **LIMIT:** structural alpha qualification proves declared capability/premise/topology invariants, not the
  population renaming/paraphrase/held-out behavior owned by `.f`.

### 7.3 `scripts/check_task_tree_archive.pl`
- **WHAT:** validates the contract-driven terminal task lifecycle. `source_locked` pins the still-live source and
  rejects premature archive paths; `migrated` verifies the exact capsule, bounded closed root/index, manifest,
  provenance, routes, milestones, and ceilings.
- **WHEN:** before or during a terminal task-tree migration; use `--report` to inspect the selected source and
  metrics, and `--self-test` to exercise all 15 fail-closed cases.
- **HOW:** `perl scripts/check_task_tree_archive.pl --check`

### 7.4 `project-validation <artifact>...` / `rescan-plan [--plan <p>] [--execute]`
- **WHAT:** `project-validation` validates + refreshes the tracked validation snapshot + writes the local
  schema-v2 rescan plan with typed replay hints; `rescan-plan` inspects/executes the whitelisted local
  replay hints (dry-run by default, `--execute` only for repository-local `cargo run … --` hints).
- **WHEN:** projecting validation state into a crash-safe snapshot; running a bounded, gated rescan loop.
  The tracked snapshot is the **last reviewed** projection, not ambient git-ignored artifact state:
  review producer output before committing it, and verify the declared boundary read-only with
  `perl scripts/check_validation_snapshot_currentness.pl --check`.

### 7.7 `scripts/repin_claim_regions.py`

- **WHAT:** re-pins every `line_range_sha256` region in the four claim registries
  (`current_claim_census.jsonl`, `book_quantitative_claims.jsonl`, `published_assertions.jsonl`,
  `claims.jsonl`) BY CONTENT after a governed file changes — and **refuses rather than guessing** when
  a region's digest matches more than one location, printing every candidate.
- **WHEN:** every slice that edits a governed file, prepends to a rolling ledger, **or edits a checker
  script**. All three shift pinned rows below the edit. The covered region total is a per-commit
  counter, so read it from `--check` rather than from this page.
- **THREE SHAPES, and the last two were invisible until `LIVE-DOCUMENT-PRESSURE-HEADROOM.22e`:**
  `{"path", "region"}` pins a governed document; `control.red_case` pins a RED case **inside a checker
  script** and carries no `kind`; `red_evidence.source_region` pins the same kind of line range with
  the file named by its control's `producer` or `inputs[0]`. Editing a gate script moves the last two,
  and the tool used to report `unchanged` while they were displaced.
- **HOW:** `python3 scripts/repin_claim_regions.py --check` (report only), then `--apply`;
  `--path <file>` scopes one governed file; `--self-test` runs the RED matrix, whose case total is
  declared in the script beside the suite so it cannot silently shrink.
- **WHY IT REFUSES, AND WHY THAT IS THE POINT.** A re-pin that lands on the WRONG line is invisible,
  because the digest it was moved to match is the digest it now has. `sha256("\n")` =
  `01ba4719…546b` matches **every blank line** in a file, and `docs/book/src/reference/live-docs.md`
  holds **280** of them behind **163** pins; `CLAIM-VERIFICATION-ADOPTION.8` recorded two rows that had
  already drifted this way. So `locate()` returns every candidate, never the first, and on any refusal
  the run writes **nothing** — all-or-nothing per registry.
- **OUTPUT:** `moved` / `unchanged` counts, one line per move, and `REFUSED AMBIGUOUS|ABSENT|NO FILE`
  with the candidate lines. Exit 1 on any refusal. A clean tree reports every region as `unchanged` and
  exits 0; the total moves with the tree, so run it rather than quoting it.

### 7.5 Build & host (RAM-bounded)
- **WHAT:** builds are RAM-constrained on this host. Monitor with `memory_pressure` (macOS); cap parallel
  jobs and kill at the danger line.
- **HOW:** `CARGO_BUILD_JOBS=2 cargo test --manifest-path Cargo.toml`; watch RAM, kill background work at
  ≥85% used (`feedback_ram_ceiling_monitor`); serialize a heavy Docling ingest vs a loaded VLM model.

### 7.6 `clean [--scope …] [--execute]`
- **WHAT:** first-class local artifact reclamation (dry-run by default): `--scope source-normalized` keeps
  `source_ir.json` but drops the heavy `normalized/` bundle; `--scope document`/`all-generated` for deeper
  sweeps. **Never** delete a `source_ir` before a re-ingest succeeds (`project_docling_mps_cpu`).
- **WHEN:** never as refresh routine. A refresh **keeps** its normalized bundle (ADR 0025 decision 3) —
  that retention is what keeps the document's evidence stage replayable. Reclaiming is a deliberate,
  separately owned decision that must also drop the key from
  `doctrine/chain_currency/retained_bundles.json` and record a `reclamations` row, or `CHAIN-CURRENCY`
  reddens.
- **HOW:** `cargo run --manifest-path Cargo.toml -- clean --scope source-normalized --document-key <key>`
