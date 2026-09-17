# COMMIT-GATE-SINGLE-RUN: every slice pays the full doctrine gate twice

## Metadata

- Tree ID: `COMMIT-GATE-SINGLE-RUN`
- Status: `active` (`2026-09-17`; `.0a`/`.3` open)
- Roadmap lane: process / continuity (commit workflow)
- Created: `2026-09-15`
- Last updated: `2026-09-17`
- Owner: repo-local workflow

## Goal

`COMMIT.md` step 8 tells the agent to run `scripts/check_doctrines.sh` before committing, and
`.githooks/pre-commit` ends by running the same driver. Both are correct in isolation and the
combination is redundant: **every slice pays the complete gate twice**, and the second run is the one
that can block, so the first buys only an earlier failure signal.

Make the cost paid once without weakening the gate. The un-bypassable leg is the hook (a local run can
simply be skipped), so whatever changes, the hook keeps running the full driver.

## Why this tree exists at all

The director approved a change on `2026-08-31`, in response to a steer to stop burning time on
administrative work: *for a slice that touches no Rust, run only a focused subset by hand and let the
hook run the full driver once at commit.* Their words: *"Move on with your proposition … we will amend
later if need be."*

**It was never implemented, because no tree owned it.** The obvious candidates were already closed
(`DOCTRINE-ENFORCEMENT-ADOPT` is `done`), and doctrine forbids a change without an owning leaf, so the
approval sat in an agent memory note across sessions instead of in the repository. That is the failure
this tree fixes first: an approved change with no owner is not tracked, it is remembered.

## Measured cost — and the earlier figures were wrong

**Re-measured `2026-09-15` over one long session (10 commits), wall clock from task-file birth to
mtime:**

| run | count | mean | range |
| --- | ---: | ---: | --- |
| standalone `check_doctrines.sh` (the `COMMIT.md` step-8 run) | 11 | **5m57s** | 5m14s – 7m13s |
| `git commit` (hook gate + commit) | 10 | **5m49s** | 5m06s – 8m02s |

**The redundant half cost this session about 65 minutes.** At ~6 minutes a run it is roughly one hour
per ten-commit session, which is the scale that justifies the work — not per-commit urgency.

**Two earlier figures are withdrawn, and how they were wrong matters more than the numbers.**
`2026-08-31` recorded *"LIVE-DOC-SIZE alone ran over 8 minutes and was still going"* and a session
asserted *"~15 min each / ~30 min per commit"*. Both were invented from the fact that a run exceeded a
2-minute foreground timeout, never derived. A `2026-09-01` measurement (n=4) already put the gate at
~5 minutes; this session's n=21 confirms it. **Do not seed any policy from a figure that was not
measured** — the whole point of the focused subset is that it is chosen by per-doctrine timing.

## Non-Goals

- Weakening the hook. It is the un-bypassable leg; it keeps running the complete driver.
- Composing a "focused subset" from a guess. No doctrine enters or leaves the fast set without its own
  measured cost — the same bar this repository applies to every other rule.
- Removing the agent's ability to run the gate early. An early signal is worth having when it is
  *chosen*, not when it is mandatory.

## Acceptance Criteria

- A slice that touches no Rust pays the full gate **once**, not twice.
- The hook still runs every registered doctrine, and a non-compliant change still cannot land.
- Whatever `COMMIT.md` ends up instructing is derived from per-doctrine timings, with the measurement
  committed.
- `scripts/check_doctrines.sh` green; no ceiling, milestone, or contract widened.

## Task Tree

- ID: `COMMIT-GATE-SINGLE-RUN` · Status: `active` (`2026-09-17`) · Children: `.0`, `.0a`, `.1`, `.2`, `.3`

- ID: `COMMIT-GATE-SINGLE-RUN.0` · Status: `done` (`2026-09-17`) · Goal: **time each doctrine
  individually before any policy is written.** The tree-level totals above are the case for doing the
  work; they cannot choose the fast set. Produce a per-doctrine wall-clock table from the registry in
  `scripts/check_doctrines.sh`, on an idle machine, and commit it.
  **Run it alone.** A standing hazard in this repository is that a heavy suite run concurrently with the
  locality gate distorts both; this session's own outlier (13m34s against a 5m57s mean) is what a
  contended measurement looks like.
  **What the table must decide, and neither answer is assumed:** whether a cheap subset exists that
  catches the ordinary docs-only failure early (this session's two real blocks were LIVE-DOC-SIZE and
  PRODUCTION-GENERICITY, both of which the hook would have caught anyway), or whether the honest change
  is simply to delete step 8's manual run and let the hook be the only gate.
  Non-goal: writing the `COMMIT.md` change in this leaf. Measure first.
  Prerequisite: none.
  **CORRECTION `2026-09-17`, on the director's challenge to re-derive rather than restate. Every share and
  total below is WITHDRAWN.** They were published as *alone on an idle machine*; the machine was never
  measured and was at **load average 12.95**. Three clean runs of the same tree gave totals **335.0s ->
  429.0s -> 523.8s**, rising in step with load, while the one single-threaded doctrine stayed flat
  (`PROJECT-DATA-LOCALITY` **1.02x** across all three) and the others moved **2.0x-2.5x**. A contended run
  times contention. The tool now measures load itself and **refuses above a 2.0 threshold** — it refuses on
  this machine today, which is the control that should have existed first. `.0a` owns the re-measurement.
  **Three further defects in the same tool and record, all mine, all found by re-deriving:** `fmt_ms`
  **truncated** to a tenth, so every aggregate summed from the formatted column was biased LOW — published
  *86.9%* against a true **(87.0%, 87.1%)**, and *42.6s / 12.7%* against **(43.1-43.6s) / (12.9-13.0%)**,
  which was also an addition error since even the truncated sum is 42.9. Published *3 of 4 doctrines that
  blocked*; the session's evidenced set is **five** — `KNOWLEDGE-MAP` was never counted — so it is **3 of 5**.
  And the fix for the truncation silently **dropped the cheapest row** (16 timed, 15 printed) and broke its
  own footer. The tool now emits exact milliseconds, derives every aggregate in integers, and **asserts
  printed rows == measured rows**.
  **What survives, and it is the part the decision needs.** Across all three contended runs the MEMBERSHIP of
  the costliest four is identical — `LIVE-DOC-SIZE`, `PROJECT-DATA-LOCALITY`, `PROOF-SEAL-CURRENCY`,
  `PRODUCTION-GENERICITY` — and so is the free set. Their combined share was **87.0% / 89.8% / 90.7%**, so
  *the heavy four carry at least 87%* is supportable as a band; no point value is. Ordering WITHIN the four
  is not stable: `PROOF-SEAL-CURRENCY` ranked 3rd, 1st, 3rd and `LIVE-DOC-SIZE` 1st, 2nd, 1st.
  **Measured `2026-09-17` at `474de7f2`, run 2 of three, CONTENDED**, by
  `scripts/measure_doctrine_cost.sh`, which DERIVES its population by parsing the driver's `DOCTRINES=(...)`
  registry rather than listing it, so a doctrine added there cannot be missing from the table. Gate tier,
  **16 of 18 registered** (two are CI-tier), every one PASS:

| Doctrine | Wall clock | Share | Enforcer |
| --- | ---: | ---: | --- |
| `LIVE-DOC-SIZE` | 1m25.9s | 25.6% | `scripts/check_live_document_size.sh` |
| `PROJECT-DATA-LOCALITY` | 1m22.8s | 24.7% | `scripts/check_project_data_locality.sh` |
| `PROOF-SEAL-CURRENCY` | 1m12.0s | 21.5% | `scripts/check_proof_seal_currency.sh` |
| `PRODUCTION-GENERICITY` | 0m50.8s | 15.1% | `scripts/check_production_genericity.sh` |
| `CLAIM-VERIFICATION` | 0m31.8s | 9.5% | `scripts/check_claim_verification.pl` |
| `KNOWLEDGE-MAP` | 0m09.9s | 2.9% | `knowledge-map/scripts/check_knowledge_map.sh` |
| `PUBLISHED-ASSERTIONS` | 0m00.6s | 0.1% | `scripts/check_published_assertions.pl` |
| `README-POLICY` | 0m00.2s | 0.0% | `scripts/check_readme_policy.sh` |
| `SECTION-ANCHORS` | 0m00.1s | 0.0% | `scripts/check_section_anchors.pl` |
| `TASK-NODE-RETENTION` | 0m00.1s | 0.0% | `scripts/check_task_node_retention.py` |
| `CONSTRAINT-PART-SPAN` | 0m00.1s | 0.0% | `scripts/check_constraint_part_span.sh` |
| `CORPUS-FRONTIER` | 0m00.1s | 0.0% | `scripts/check_corpus_frontier.sh` |
| `MEMORY-ARCH` | 0m00.0s | 0.0% | `scripts/check_memory_architecture.sh` |
| `TASK-ACCEPTANCE` | 0m00.0s | 0.0% | `scripts/check_task_acceptance.sh` |
| `RESIDUAL-ACTIONABILITY` | 0m00.0s | 0.0% | `scripts/validate_residual_actionability_contract.py` |
| `OWNERSHIP-CITATIONS` | 0m00.0s | 0.0% | `scripts/check_ownership_citations.pl` |
| **total** | **5m35.0s** | **100.0%** | — |

  **The distribution is extremely skewed, and that is what survives the correction.** In this run the heavy
  four carried **(87.0%, 87.1%)** and everything else **(43.1-43.6s), (12.9%, 13.0%)** — bands, because the
  displayed column is rounded; the exact-millisecond column added later removes the need for bands, and the
  later runs read 89.8%/10.2% and 90.7%/9.3%.
  **An earlier table was thrown away, and how it was wrong is the durable part.** That run recorded
  `CLAIM-VERIFICATION` at **0m00.8s**; the true figure is **0m31.8s**, a **40x** understatement, because the
  checker exited early on an unrelated failure — `scripts/measure_doctrine_cost.sh` was itself untracked at
  the time, which `CLAIM-VERIFICATION` refuses as an *untracked producer-shaped path under governed source
  roots* and `PROJECT-DATA-LOCALITY` refuses for missing its `project_data_env.sh` contract text. Both
  refusals were correct and caught a real omission. **A cost measured on a failing tree is not that check's
  cost, it is the cost of its first error**, so a timing table is only valid when every row reads PASS.
  **What the table decides, and it does not decide it the way the leaf guessed.** A cheap subset does exist
  arithmetically — the ten free doctrines cost 1.2s — but it would be worthless: of the **four** doctrines
  that actually blocked a commit in this session, `LIVE-DOC-SIZE`, `PROJECT-DATA-LOCALITY`,
  `CLAIM-VERIFICATION` and `PUBLISHED-ASSERTIONS`, only the last is in the free ten. The useful cut is
  different and membership names it: **everything except the four heaviest** — between **9.3% and 13.0%** of
  the gate across the three runs — and it contains **three of the session's five** evidenced blocks
  (`CLAIM-VERIFICATION`, `PUBLISHED-ASSERTIONS`, `KNOWLEDGE-MAP`), while **missing the two heavy ones that
  actually fired**, `LIVE-DOC-SIZE` and `PROJECT-DATA-LOCALITY`. That is a real trade, not a free win, and
  the earlier record overstated it as three of four.
  Verification: `2026-09-17` — four runs, three with all 16 PASS; population derived from the driver registry
  and cross-checked at 18 entries / 16 gate tier. **The verification leg that was MISSING is the one that
  mattered**: no run measured whether the machine was idle, which the leaf required, and it was not
  Commit: `COMMIT-GATE-SINGLE-RUN.0 — measure what the doctrine gate actually costs, per doctrine`

- ID: `COMMIT-GATE-SINGLE-RUN.1` · Status: `done` (`2026-09-17`) · Goal: **make a hand-run
  focused check impossible to get silently wrong.** This tree's whole remedy is *run a focused subset by
  hand*, and hand-running is exactly where a check turns into a no-op. Censused `2026-09-17` over the 42
  `scripts/check_*.pl|sh` gates:
  **(a) there is no flag convention.** The opening census said *21 accept `--check` and 21 do not*; that was
  a **grep for the string**, and re-running it as an execution test gives **27 accept, 15 reject**. Either
  way the correct invocation is per-script knowledge with nothing to check it against.
  **(b) the two families fail differently, and one of them does not fail.** Given an unsupported
  `--check`, the Perl gates die with usage and a non-zero status (`check_live_document_size.pl` 255,
  `check_derived_state_contracts.pl` 255, `check_rolling_ledger_protocol.pl` 25) — loud. But
  `check_readme_policy.sh --check` and `check_memory_architecture.sh --check` **ignore the unknown
  argument, run, and exit 0**: a wrong flag is accepted rather than refused, so a near-miss on a script
  that has a meaningful mode (`--apply`, `--all`, `--apply-rollover`, `--execute-stale-gates`) selects the
  wrong mode silently. **`accepts` conflates two different things**, and separating them is what sizes the
  hazard: run every gate with a flag that cannot be a mode (`--zzz-not-a-real-mode`) and **8 of 42 ignored
  it and ran anyway**. Three of those eight are the ones named here; the other five
  (`check_book_current_truth.sh`, `check_production_genericity_flow.sh`, `check_production_genericity_graph.sh`,
  `check_project_data_locality.sh`, `check_task_acceptance.sh`) were missed by generalising from the three
  that happened to be in front of me instead of censusing the population.
  **(c) the caller amplifies it.** `cmd | grep -i warning | head` reports `$?` from `head`, so a
  non-zero gate reads as success, and usage text on stderr looks nothing like a failure to a reader
  grepping for findings. Observed **twice in one session** (`2026-09-17`): a "no live-document warnings"
  reading that was a usage error hiding **26** warnings, and a commit-completion check that matched the
  PREVIOUS commit's subject and led to the message file being cleared mid-commit.
  **Decide between**: a single focused-subset entry point that owns every script's flags and asserts each
  exit status, so no caller picks flags at all; or normalizing `--check` as an accepted alias across all
  42 plus unknown-argument refusal in the shell gates. The first changes the shape; the second changes 42
  numbers. Do not seed the fast set from this leaf — that is `.0`'s job and it must measure first.
  Prerequisite: none; found while closing `LIVE-DOCUMENT-PRESSURE-HEADROOM.28`.
  **Done `2026-09-17`, and the census above needed one correction that makes the finding sharper.** (b)
  claimed `check_readme_policy.sh` *takes no arguments and ignores any*. It takes exactly one:
  `--self-test`, handled at line 61 — and `scripts/check_live_document_size.sh:26` **calls it with that
  flag**. So the hazard was not hypothetical and not about an unused script: a near-miss on that one real
  mode ran the ORDINARY check and exited 0, and the caller would have recorded a self-test that never
  executed. The other two, `check_memory_architecture.sh` and `check_constraint_part_span.sh`, do take no
  arguments; all three accepted `--bogus-flag-xyz` and exited **0**.
  **Both options in the leaf were taken, because they fix different halves.** Shape first: the driver gains
  **`--only ID[,ID...]`** and **`--list`**, so a caller never invokes a gate script by hand at all — the
  driver runs each enforcer exactly as the hook does, with no arguments, and asserts the exit status itself.
  Selection is validated **against the registry**, so an unknown id is refused (exit 2) rather than
  selecting nothing and reporting success; every unselected doctrine is reported **`SKIP`** rather than
  omitted; and a subset run prints **`SUBSET ONLY — N of 18 … this is NOT the gate`**, so it cannot be read
  as a complete run. Then the population: **all 8** permissive gates now refuse any argument they do not
  implement, verified as **42 of 42 REFUSE** on the bogus-mode sweep.
  **Controls, all on the real tree.** RED: `--only NOPE` exits 2 naming the id; `--only` with no value exits
  2; a registry entry pointed at a deliberately failing enforcer reports `FAIL` under `--only` and exits
  **1**, so a subset still blocks (entry and script removed, driver restored byte-for-byte from a
  pre-change copy). GREEN: `--list` prints **18** rows; `--only MEMORY-ARCH,PUBLISHED-ASSERTIONS` runs
  exactly those two and prints the SUBSET sentence; each guarded gate exits **2** on `--bogus-flag-xyz` and
  **0** with no arguments; `check_readme_policy.sh --self-test` still passes, which is the arm the guard
  could most easily have broken — and it is the arm that caught the guard's first draft, written on the
  false premise that the script took no arguments while `check_live_document_size.sh:26` calls it with one.
  Population re-swept after the guards: **42 of 42 refuse** `--zzz-not-a-real-mode`, from 34 of 42 before.
  **The guard found a live instance on its first run, in the claim registry itself.** Turning it on failed
  `CURRENT-CLAIM-CENSUS`: the evidence record
  `evidence-readme-entrypoint-maintained-references-route-8c0d2795de7c` declares its verifier as
  `bash scripts/check_readme_policy.sh --check`, **and `--check` is not a flag that script has ever
  implemented**. It was silently ignored, the ordinary check ran, stdout matched and the exit was 0, so the
  record passed for its whole life on an invocation that did not exist. Corrected to the argv that does.
  **Swept the rest rather than assuming it was the only one**: all **34** distinct `argv` arrays declared
  across every `doctrine/**/*.jsonl` were enumerated and each flag-bearing one executed. Exactly **one** was
  a phantom — this one. The other flag users (`--self-test`, `--report`, `--probe`, `--contract`, and the
  Perl `--check`s) all resolve to modes their targets implement.
  Verification: `2026-09-17` — six control arms above, the 34-argv sweep, plus `scripts/check_doctrines.sh`
  green at 16/16
  Commit: `COMMIT-GATE-SINGLE-RUN.1 — make the driver the only way to run one doctrine`

- ID: `COMMIT-GATE-SINGLE-RUN.2` · Status: `done` (`2026-09-17`) · Goal: **make a slice pay the
  gate once**, now that `.0` has measured what it costs. The approval this tree exists to carry is the
  director's `2026-08-31` one — *for a slice that touches no Rust, run only a focused subset by hand and
  let the hook run the full driver once at commit* — and `.0`'s table finally says what that subset is.
  **The candidate, with its own numbers**: keep the manual pass at everything except `LIVE-DOC-SIZE`,
  `PROJECT-DATA-LOCALITY`, `PROOF-SEAL-CURRENCY` and `PRODUCTION-GENERICITY` — **42.6s against 5m35.0s,
  12.7%** — which still covers three of the four doctrines that blocked a commit in the `2026-09-17`
  session. The alternative `.0` was asked to weigh, deleting step 8's manual run outright, is cheaper
  still and loses the early signal entirely; the hook blocks either way, so this is a question about
  iteration cost, not about enforcement.
  **Do not implement either by editing `COMMIT.md` prose alone.** A subset that lives only in an
  instruction is a subset each session re-derives from memory, which is how `.1`'s wrong-flag class
  happens; whatever is chosen needs an executable entry point that names its own membership, and that
  entry point must derive from the driver registry the way `scripts/measure_doctrine_cost.sh` does, so a
  newly registered doctrine is never silently outside the fast set.
  **Sequenced after `.1`**, deliberately: `.1` decides whether a hand-run check can be trusted at all, and
  a fast set that can silently no-op is worse than no fast set.
  Prerequisite: `.0` (done); `.1` should land first.
  **Done `2026-09-17`.** The manual leg is now `scripts/check_doctrines.sh --fast`, and `COMMIT.md` step 8
  no longer mandates a full manual run: the hook runs the complete driver once at commit.
  **The subset is an EXCLUSION, and that is the whole design.** `FAST_EXCLUDE=(LIVE-DOC-SIZE,
  PROJECT-DATA-LOCALITY, PROOF-SEAL-CURRENCY, PRODUCTION-GENERICITY)` sits beside the registry in the
  driver, so a doctrine added to `DOCTRINES=(...)` is in the fast set **automatically**; an inclusion list
  would have put every new doctrine silently outside it, which is exactly the drift this leaf was told to
  prevent. The exclusion is meta-checked **against the registry on every run**, not only under `--fast`, so
  renaming a doctrine and forgetting the list refuses at the next gate instead of quietly widening the
  subset. Membership is `.0`'s one surviving finding — the same four were costliest in all three of its
  runs — and **no share is quoted anywhere**, in the driver, in `COMMIT.md`, or in the book; `.0a` can move
  the membership by editing four lines and nothing else.
  **`--fast` refuses when the pre-commit hook is not active**, and this is the control the whole change
  rests on. Dropping step 8's mandatory run is only safe because `.githooks/pre-commit` runs the complete
  driver, and that leg is a per-clone opt-in (`git config core.hooksPath .githooks`). In a clone where it
  was never wired, a subset run would be the only doctrine enforcement that ever happened **and it would
  report PASS**. The driver now resolves `core.hooksPath`, requires an executable `pre-commit` under it,
  and requires that file to invoke `check_doctrines.sh`.
  **It cannot be read as a full run.** Unselected doctrines report `SKIP`, never omission; a `--fast` run
  prints `FAST SUBSET — N of 18 … this is NOT the gate` plus a second line naming every doctrine it did
  not run, so the caveat travels with any line copied into a task record.
  **The honest cost of the trade, stated at the call site and in the book:** of the five doctrines
  evidenced blocking a commit on `2026-09-17`, `--fast` runs `CLAIM-VERIFICATION`, `PUBLISHED-ASSERTIONS`
  and `KNOWLEDGE-MAP`, and omits `LIVE-DOC-SIZE` and `PROJECT-DATA-LOCALITY`. So `COMMIT.md` routes a slice
  that touches Rust, a registered enforcer, or anything under `scripts/` to the **full** driver, and only a
  Markdown/task/doctrine-record slice to `--fast`.
  **It paid for itself on its first real run.** Executed against this leaf's own working tree, `--fast`
  failed `CLAIM-VERIFICATION` in **16s** on the two stale `scripts/check_doctrines.sh` digests in
  `claims.jsonl` that editing the driver had just created — a block the full gate would have reported
  minutes later. That 16s is **not** the subset's cost: per `.0`, a run that fails is timed at the cost of
  its first error. The green figure is below.
  **One alignment-review correction landed in the same commit, because `COMMIT.md` step 3 makes a known
  current-facing contradiction a blocker.** Its routing contract said editing a governed file re-pins
  regions in *"all THREE claim registries"*; `scripts/repin_claim_regions.py` has processed **four** since
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.22e` added `claims.jsonl`, and `MEMORY.md` already said four — so the
  commit workflow contradicted the resume pointer and the tool. Corrected to four, named, and widened to
  say what the count hides: a **checker script** carries pins too (`control.red_case` /
  `red_evidence.source_region` pin line ranges inside the gate scripts), which is precisely the class this
  leaf triggered by editing the driver.
  Verification: `2026-09-17` — eight control arms, all on the real tree.
  **RED**: `--fast --only MEMORY-ARCH` exits **2** (two different selections); `--only NOPE` exits **2**
  naming the id; `--only` with no value exits **2**; an unrecognised flag exits **2**; with
  `core.hooksPath` pointed at a `pre-commit` that does not run the driver, `--fast` exits **2** naming the
  resolved path; with `core.hooksPath` unset, `--fast` exits **2** reporting `<unset>`; a copy of the
  driver with `PROOF-SEAL-CURRENCY` renamed in `FAST_EXCLUDE` only exits **2** on `--list`, proving the
  exclusion meta-check is unconditional rather than `--fast`-only. Every temporary artifact removed;
  `core.hooksPath` restored to `.githooks` and re-read.
  **GREEN**: `--list` prints **18** rows; `--fast` executes **12 of 18** — 4 `SKIP` by exclusion, 2 `DEFER`
  CI-tier — and prints both subset lines; on a green tree it costs **63s**, against **625s** for the full
  driver run back-to-back on the same tree. Both CONTENDED (load 10.43 and 10.30), so they are a
  same-session PAIR, not shares, and the 625s is a LOWER bound on a green gate because that run ended
  with one doctrine FAILING.
  **That failing run is the leaf's best evidence, because it is the trade happening rather than being
  described.** `--fast` was GREEN on this very tree at 12 of 18; the full driver then FAILED
  `LIVE-DOC-SIZE` — a doctrine `--fast` skips — on the book surface's `aggregate_change` authority, which
  this leaf's own 33-line chapter subsection had made stale. Declared under a new authority id
  (`COMMIT-GATE-SINGLE-RUN.2-BOOK-SUBSET-SELECTORS`, +33 lines / +2,128 bytes over `.28`'s expected
  18,902 / 1,231,475, re-derived independently over the 42 book files rather than read from the error
  text — the checker refuses a REUSED authority id across an aggregate change, so attributing these lines
  to `.28` was not an option). **This is exactly why `COMMIT.md` routes a `scripts/`-touching slice to the
  full driver and why the banner names what it skipped:** a green `--fast` would have carried this slice
  into a commit the hook would have blocked.
  Commit: `COMMIT-GATE-SINGLE-RUN.2 — pay the doctrine gate once, and make the subset unable to pass for the gate`

- ID: `COMMIT-GATE-SINGLE-RUN.3` · Status: `pending` (opened `2026-09-17` by `.2`) · Goal: **`.2`'s Rust
  branch is wrong by `.2`'s own argument, and the arithmetic is the proof.** `COMMIT.md` step 8 now sends a
  slice that touches Rust, a registered enforcer, or anything under `scripts/` to the FULL driver by hand.
  Price that branch the way `.2` priced the docs branch, with `G` the cost of one full gate: manual-first
  costs `G + G` when it passes and `G + fix + G` when it fails; no-manual costs `G` when it passes and
  `G + fix + G` when it fails. **The manual full run is never cheaper and is usually 2x, whatever the slice
  touches** — which is the identical argument `.2` used to delete the mandatory run for docs slices, applied
  to the branch `.2` left standing. It was written to be conservative about Rust and was not derived.
  **What the branch should say instead**, and it is a stronger check, not a weaker one: the doctrine gate
  does not compile or test anything, so for a Rust slice the signal that matters is the one the gate never
  provides — `cargo fmt`/`clippy`/`cargo test -p specforge --lib` — plus `--only PRODUCTION-GENERICITY` when
  the producer graph could move (`flow_census.json`), which `--fast` omits. That is targeted at the real
  failure mode instead of paying for `PROOF-SEAL-CURRENCY` and `LIVE-DOC-SIZE` twice on a slice that cannot
  move either.
  **Do not fix this by loosening the sentence.** The reason the manual run keeps coming back is that an
  early signal feels safer than it measures; whatever replaces it must name the specific oracle for the
  specific risk, or the next session re-derives "run everything" from first principles again.
  **Found while working `SIGNAL-DECLARATION-ROW-DROP.2i`**, the first Rust slice after `.2` landed: the new
  branch demanded ~10 minutes of manual gate on a documentation-comment edit that provably cannot move a
  producer. That slice paid it rather than bend a one-commit-old rule, which is the right order — fix the
  rule, then rely on the fix.
  Prerequisite: none. `.0a` does not block it: this is an arithmetic correction, not a measurement.
  Verification: pending
  Commit: pending

- ID: `COMMIT-GATE-SINGLE-RUN.0a` · Status: `pending` (opened `2026-09-17`) · Goal: **re-measure the gate on
  a machine proved idle**, because `.0`'s table was taken at load average 12.95 and its shares are
  withdrawn. The tool now refuses above a 2.0 load threshold and prints the load at both ends of the run, so
  the precondition is asserted rather than claimed; what is missing is a run that passes it.
  **What the re-measurement must settle**, and only the first is already supported: the costliest-four
  MEMBERSHIP (identical across three contended runs), their share (observed 87.0% / 89.8% / 90.7% — a band,
  not a value), and the ordering within them (unstable: `PROOF-SEAL-CURRENCY` ranked 3rd, 1st, 3rd).
  **Take at least three passing runs and publish the spread**, not one run's numbers. A single measurement
  of this gate has now been shown twice to be unreproducible, once by 2.4x on one doctrine.
  Prerequisite: `.0`; blocks nothing — `.2` can decide on membership alone, which is what it needs.
  Verification: pending
  Commit: pending

## Current Frontier

Ordered; PNT selects the first eligible leaf.

0. `COMMIT-GATE-SINGLE-RUN.3` — correct `.2`'s Rust branch, which costs 2x by `.2`'s own arithmetic and
   was not derived. Prerequisite-free, and it should land before many more Rust slices pay for it.
1. `COMMIT-GATE-SINGLE-RUN.0a` — re-measure on a machine proved idle, and publish a spread. **Not runnable
   on the current machine**: `scripts/measure_doctrine_cost.sh` refuses above load average 2.0 and this one
   has held near 9 all session. That refusal is the control `.0` lacked; do not override it to close a leaf.
   `.0a` can move `FAST_EXCLUDE` without touching anything else if the idle membership differs.

`COMMIT-GATE-SINGLE-RUN.0`, `.1` and `.2` are `done` (`2026-09-17`), `.0` with its shares withdrawn: the same four doctrines carried at least 87% in each of
three runs, so the subset question had an answer in MEMBERSHIP even though every share is withdrawn — and `.2` spent exactly that,
quoting membership and no share.

## Decisions

- `2026-09-15` (tree opened): **the hook is the leg that must not move.** `MEMORY_ARCHITECTURE.md` §9
  layers discovery / self-check / git hook / CI, and notes the hosted CI is manual-only. With CI not
  firing automatically, the pre-commit hook is the only un-bypassable enforcement in practice, so the
  redundancy must be removed from the manual side.
- `2026-09-15`: **this is background hygiene, not feature work.** Measured at ~1 hour per ten-commit
  session it pays off across a long run and not within one slice; schedule it accordingly rather than
  ahead of product work.
