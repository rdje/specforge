# COMMIT-GATE-SINGLE-RUN: every slice pays the full doctrine gate twice

## Metadata

- Tree ID: `COMMIT-GATE-SINGLE-RUN`
- Status: `active` (`2026-09-20`; `.0a` and `.8` open; `.9` closed the step-8 clippy oracle that could not fail, `.10` the CI rustdoc leg that nothing ran, `.11` restored the hosted push trigger)
- Roadmap lane: process / continuity (commit workflow)
- Created: `2026-09-15`
- Last updated: `2026-09-20`
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

- ID: `COMMIT-GATE-SINGLE-RUN` · Status: `active` (`2026-09-20`) · Children: `.0`, `.0a`, `.1`, `.2`, `.3`, `.3a`, `.4`, `.5`, `.6`, `.7`, `.8`, `.9`, `.10`, `.11`, `.11`

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

- ID: `COMMIT-GATE-SINGLE-RUN.3` · Status: `done` (`2026-09-17`) · Goal: **`.2`'s Rust
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
  **Done `2026-09-17`.** Step 8 now says **never run the full driver by hand**, states the arithmetic that
  makes that true regardless of what the slice touched, and replaces the Rust branch with the oracle for
  the risk: `cargo fmt --all -- --check` / `cargo clippy` / `cargo test -p specforge --lib` — **which the
  doctrine gate never runs at all, because it neither compiles nor tests** — plus `--only
  PRODUCTION-GENERICITY` when the producer graph could move, and `--only LIVE-DOC-SIZE` /
  `--only PROJECT-DATA-LOCALITY` for a governed-surface or seam change. The Rust branch was not merely
  expensive, it was aimed at the wrong thing: it paid for `PROOF-SEAL-CURRENCY` and `LIVE-DOC-SIZE` on a
  slice that could not move either, while buying **no compile and no test**.
  **Four restatements were corrected with it, and that set is the point.** The old branch was echoed in
  `scripts/check_doctrines.sh`'s own header comment, the routed toolbox §7.2, the mdBook chapter, and the
  fact card. A rule restated in five places drifts in four of them the moment one changes — which is the
  same defect class `SIGNAL-DECLARATION-ROW-DROP.2i` closed one commit earlier, in a Rust doc comment.
  **The book subsection gained the honest example** rather than the abstract claim: `--fast` went green on
  `.2`'s own tree and the hook's full run then failed `LIVE-DOC-SIZE`, one of the four `--fast` skips.
  Declared under `COMMIT-GATE-SINGLE-RUN.3-BOOK-MANUAL-RUN-ARITHMETIC`, +11 lines / +1,036 bytes over
  `.2`'s expected 18,935 / 1,233,603, re-derived over the 42 book files.
  **This slice ran under its own new rule** and is the first evidence it works: `--fast` for the early
  signal plus `--only LIVE-DOC-SIZE` because it edits governed surfaces and a registered enforcer — no
  manual full driver, where `.2`'s rule would have demanded ~7-10 minutes of one.
  Verification: `2026-09-17` — `scripts/check_doctrines.sh --fast` PASS 12 of 18 in **52s** at load 7.50;
  `--only LIVE-DOC-SIZE` PASS in **87s** at load 7.35, and **96s** at load 5.79 earlier in the same slice;
  the hook's complete driver PASS at commit.
  Commit: `COMMIT-GATE-SINGLE-RUN.3 — the manual full gate is never cheaper, whatever the slice touched`

- ID: `COMMIT-GATE-SINGLE-RUN.4` · Status: `done` (`2026-09-17`) · Goal: **the driver threw away every
  warning a PASSING check emitted, so containment could only ever speak by refusing.**
  **Measured `2026-09-17`.** `scripts/check_live_document_size.sh` run directly emits **44** warning lines,
  including `surface 'task_evidence' lines_each is at or above rollover (99.9%) — 2 below its 3000 ceiling`.
  Through `scripts/check_doctrines.sh` — which is how `.githooks/pre-commit`, `scripts/run_ci.sh` and
  `COMMIT.md` step 8 ALL run it — exactly **0** were visible: `out="$(...)"` captured stdout+stderr and
  printed it only on failure.
  **This is the governance defect behind the day's containment stop, and the director named it.** The
  `EXTRACTION-QUALITY-GAUGE` tree went 95.9% (recorded by `LIVE-DOCUMENT-PRESSURE-HEADROOM.30`) -> 98.5% ->
  a refused commit, and at no point did the gate say so to anyone running it the supported way. The author
  then met the bound as a STOP mid-slice and compacted evidence to land the work. **Being forced to shrink
  evidence is a policy failure, not an author problem**: evidence is layer B, the record of why a decision
  was made, and a bound whose only remedy at the moment it speaks is deletion is corroding the memory
  architecture it is supposed to protect.
  **Fixed at the delivery seam, not by weakening anything.** No ceiling moved, no check changed its verdict,
  no warning threshold was touched. The driver now forwards warning lines from checks that PASSED, under a
  `PRESSURE (N)` section that says what they are for, and closes with the rule the director stated: *a
  remedy that requires DELETING evidence is a policy defect, not an author problem.*
  Verification: `2026-09-17` — GREEN: `--only LIVE-DOC-SIZE` prints `PRESSURE (44)` on a PASSING run, and
  **44 equals the 44 the enforcer emits directly**, so nothing is dropped in forwarding. RED: `--only
  README-POLICY`, a check that emits no warnings, prints no section at all. Exit statuses unchanged in both.
  Prerequisite: none; found on the director's challenge after the third containment refusal in one commit.
  Commit: `COMMIT-GATE-SINGLE-RUN.4 — containment must speak before it refuses`

- ID: `COMMIT-GATE-SINGLE-RUN.3a` · Status: `done` (`2026-09-17`) · Goal: **one
  `--only LIVE-DOC-SIZE` run took more than 600s where a green one takes 96s, and load does not explain
  it.** Observed `2026-09-17` while working `.3`: the doctrine exceeded a 600s foreground timeout at load
  ~10 on a tree where it FAILED (`fact_index` freshness — `KNOWLEDGE_MAP.md` was stale because the slice
  had edited a fact card without regenerating), then passed in **96s** at load 5.79 once regenerated.
  `.0` measured this doctrine at **1m25.9s** contended, so 96s is the normal figure and the outlier is the
  failing run. **Load is not a sufficient explanation**: `.0` measured multi-process doctrines moving
  2.0-2.5x with load, which puts the worst case near 240s, not 600+.
  **Two hypotheses were checked and BOTH are refuted, so neither should be re-derived.** (a) *The nested
  freshness verifier is invoked per surface*: exactly **one** surface (`fact_index`) declares
  `knowledge-map/scripts/check_knowledge_map.sh`, invoked once at `check_live_document_size.pl:986`.
  (b) *The nested verifier is itself slow*: measured **13s** standalone and green. What remains unmeasured
  is whether the FAILING path is the expensive one — at that moment both `KNOWLEDGE_MAP.md` and the
  fact-card catalog projection were stale, and `check_live_document_size.sh` runs a large self-test suite
  (113 + 47 + 45 + 49 + 60 + 64 fixture cases) whose generators re-render projections.
  **The controlled experiment, so the next session does not re-derive the framing**: make exactly one input
  stale (regenerate-then-dirty the fact card), time `--only LIVE-DOC-SIZE`, restore, time it again, at a
  load measured at both ends — the same precondition discipline `.0a` owns. One variable, two readings.
  **Why it matters beyond curiosity**: `.2`'s `FAST_EXCLUDE` membership and `.0a`'s re-measurement both
  assume a doctrine's cost is a property of the doctrine. If a stale input multiplies one doctrine 6x, the
  cost table is a function of tree state as well as load, and a timing run on a dirty tree measures neither.
  Prerequisite: none. Related: `.0a` (both are about what a gate timing actually measures).
  **Done `2026-09-17`. THE LEAF'S OWN HYPOTHESIS IS REFUTED, and the real answer is worse than the one it
  guessed.** A stale input is not the expensive path, and the doctrine has no stable cost to explain: the
  same command on the same unchanged tree varies nearly **6x**.
  **The matrix, all `bash scripts/check_doctrines.sh --only LIVE-DOC-SIZE`, load read at each run's start:**

| tree state | wall clock | load at start |
| --- | ---: | ---: |
| green | 87s | 7.35 |
| green | 89s | 5.36 |
| green | 96s | 5.79 |
| green | 214s | 4.90 |
| green | 220s | 5.51 |
| green | 266s | 5.12 |
| stale | 87s | 4.63 |
| stale | 256s | 4.46 |
| stale | 506s | 6.49 |

  **Read the table for what it refutes.** The four consecutive green runs at load **4.90-5.51** — a 12%
  band — span **89s to 266s, 3.0x**. The stale and green populations *overlap completely*: the fastest
  run of all (87s) and the slowest (506s) are both stale, and the second-fastest is green. Load does not
  order the readings either; the 87s green run was taken at the HIGHEST load in the set. **Nothing about
  the tree or the machine's load average predicts the number.**
  **Two mechanisms were eliminated by direct measurement, not by argument.** The nested freshness verifier
  `knowledge-map/scripts/check_knowledge_map.sh` costs **10s stale / 13s green**, so it cannot carry a
  170s delta; and `scripts/check_live_document_size.sh` invoked DIRECTLY on the stale tree finished in
  **96.4s**, indistinguishable from green, so neither staleness nor the driver wrapper is the variable.
  A phase timing of that run put the largest single gap at 44.6s in the 113-case lifecycle suite, with no
  gap large enough to explain the outliers.
  **Consequence, and it reaches past this leaf.** `.0`'s table recorded `LIVE-DOC-SIZE` at **1m25.9s**,
  which sits at the very BOTTOM of the observed range — against a median near **217s** for the green
  same-load group, the table understates this doctrine by roughly **2.5x**. `.0` withdrew its shares for
  contention at load 12.95; this is a wider withdrawal, because the instability is present at load 5 on an
  unchanged tree. **A single wall-clock reading of a gate doctrine on this machine is not a measurement.**
  **It also explains `.0`'s own split result** — membership of the costliest four stable across three runs
  while ordering WITHIN the four was not. Noise of this size reorders neighbours freely but cannot lift a
  sub-second doctrine past a 90-500s one, so membership survives exactly the noise that scrambles rank.
  `.2`'s `FAST_EXCLUDE` rests on the part that survives; no share anywhere rests on the part that does not.
  **Not chased further, deliberately**: the remaining candidate is host I/O contention, which the load
  average does not measure and this repository has no instrument for. `.0a` owns the remedy, and it is now
  a different remedy — repeat and publish a spread, rather than one run behind a load threshold.
  Verification: `2026-09-17` — nine timed runs above (6 green / 3 stale), each with its start load; the
  10s/13s nested-verifier pair; the 96.4s direct stale run with per-phase timestamps. The transient probe
  line appended to the fact card was restored from a pre-change copy and verified byte-identical to `HEAD`
  (`git diff` empty), and the projection regenerated.
  Commit: `COMMIT-GATE-SINGLE-RUN.3a — a gate doctrine has no stable cost: 6x on one unchanged tree`

- ID: `COMMIT-GATE-SINGLE-RUN.5` · Status: `done` (`2026-09-17`, DOC) · Goal: **step 8's Rust oracle
  named a crate that does not contain the code a Rust slice changes.** Found while running it for
  `EXTRACTION-QUALITY-GAUGE.3j.1.a`, which edits `crates/specforge/src/ir/constraint_extract_llm.rs`:
  `cargo test -p specforge --lib constraint_extract_llm` reported **0 tests run, 473 filtered out** and
  exited 0. A passing command that runs none of the relevant tests is worse than a missing one, because
  it is indistinguishable from a green run.
  **Root cause, and it is a workspace shape rather than a typo.** `crates/specforge-core/src/lib.rs:9`
  declares ``#[path = "../../specforge/src/ir/mod.rs"] pub mod ir;``, and `crates/specforge/src/lib.rs:7`
  re-exports `specforge_core::ir::*`. So every `ir/` module — where every extraction rule lives — compiles
  into **specforge-core**, and `-p specforge --lib` runs only the **473** CLI-facade tests. The suite that
  actually covers an `ir/` change is **specforge-core's 1,551**, and step 8 never named it. The doctrine
  that a subset is only safe while something else pays for the rest (`.2`/`.3`) is unaffected; what was
  wrong is WHICH subset.
  **Fixed in `COMMIT.md` step 8**: the Rust oracle now reads `cargo test -p specforge-core --lib` for a
  change under `crates/specforge/src/ir/` (compiled into core by `#[path]`) and `-p specforge --lib` for
  the CLI, with the reason stated inline so the next reader does not have to re-derive the include.
  Verification: `cargo test -p specforge --lib constraint_extract_llm` -> `0 passed; 473 filtered out`,
  against `cargo test -p specforge-core --lib clause` -> `46 passed; 1505 filtered out`; the four new
  `.3j.1.a` tests appear only in the second. Prerequisite: none; found by `.3j.1.a`
  Commit: `EXTRACTION-QUALITY-GAUGE.3j.1.a — carry the obligation clause, and refuse one the span never stated`

- ID: `COMMIT-GATE-SINGLE-RUN.6` · Status: `done` (`2026-09-17`, DOC) · Goal: **stop naming a crate
  under test in step 8 at all.** `.5` found that the named crate was the wrong one and replaced it with the
  right one. The director asked the better question — *why would `COMMIT.md` name specific crates in the
  first place?* — and the answer is that it should not. **Which crate a module compiles into is a fact about
  the workspace, not about the slice**, so encoding it in a process document guarantees the document goes
  stale silently; `.5` fixed the number and left the shape that produced it.
  **The only defensible reason to name anything is cost, so it was measured** (`2026-09-17`, warm target):
  `-p specforge-core --lib` **12s**; `--workspace --lib` **3m13s**, of which the `specforge-production-graph`
  enforcement tool is **117s for the 8 tests it owns**; `--workspace --lib --exclude
  specforge-production-graph` **1m17s for 2,187 tests across all three product crates**. So the whole
  workspace is 16x a single crate, but excluding ONE enforcement tool recovers most of it.
  **The rule that follows**: name the EXCLUSION, never the crate under test. An exclusion is a cost decision
  that is true regardless of where a module lives, and it is self-describing: the graph tool's own unit
  tests cannot be moved by a product change, and `PRODUCTION-GENERICITY` already runs that graph against the
  product on every commit. A slice now pays 1m17s and cannot run zero relevant tests.
  Verification: the three timings above, each on a warm target; `--workspace --lib --exclude
  specforge-production-graph` reports 473 + 168 + 1,546 passing, i.e. every product crate
  Commit: `COMMIT-GATE-SINGLE-RUN.6 — name the exclusion, never the crate under test`
  Prerequisite: `.5`, whose fix this supersedes in shape rather than in direction

- ID: `COMMIT-GATE-SINGLE-RUN.7` · Status: `done` (`2026-09-18`, DOC) · Goal: **the defect `.5` named in
  step 8 was still live in five documented run commands, and a sixth is a command the PRODUCT prints.**
  `.5` established that `crates/specforge/src/ir/**` compiles into `specforge-core` by `#[path]`, so
  `cargo test -p specforge --lib <filter>` over that tree reports *0 passed, N filtered out* and **exits 0**
  — a passing command that runs none of the relevant tests. `.6` fixed step 8. It did not sweep for the
  same string elsewhere, and it was there: **five `--ignored` measurement harnesses in
  `ir/evidence.rs` documented their own run command as `-p specforge`.** A developer following any of them
  gets a green run that measured nothing, which is exactly the failure mode `.5` called indistinguishable
  from success. Verified empirically per harness rather than by inspection, with `-- --ignored --list`
  under both crates: `-p specforge` lists **0 tests** for all five; `-p specforge-core` lists 1, 2, 1, 1, 1.
  All five now name `specforge-core`, and each was re-listed after the edit.
  **The sweep is what closes this, not the five edits.** `grep -rn 'cargo test -p specforge --lib'` over
  `crates/` and `docs/` now returns no command that names a filter living in `ir/**`; the remaining hits
  are `commands::converge::tests::…`, which **is** correct — `commands/` is not `#[path]`-included, and
  `--list` confirms it at 1 test under `-p specforge` and 0 under `-p specforge-core`. The crate name is
  not uniformly wrong; it is wrong for one tree, which is why this had to be checked rather than replaced.
  Found while writing `EXTRACTION-QUALITY-GAUGE.3j.2.a.ii`, which caught the same defect in ADR 0037's own
  `reverify`. Acceptance: every documented run command over `ir/**` resolves to a non-empty test set under
  the crate it names, proved by `--list`. Prerequisite: `.5`. Verification: the ten `--list` runs above
  Commit: `COMMIT-GATE-SINGLE-RUN.7 — sweep the crate name out of every run command that names it wrongly`

### Acceptance Checklist (enforced) — `COMMIT-GATE-SINGLE-RUN.7`

- [x] **REPRODUCE / MEASURE** — per harness, `cargo test -p specforge --lib <filter> -- --ignored --list`
  reports **0 tests** for all five; the same filters under `-p specforge-core` report **1, 2, 1, 1, 1**.
  Ten `--list` runs, one pair per harness, before and after.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` doc comments at the five
  `--ignored` harnesses. `crates/specforge-core/src/lib.rs` pulls `crates/specforge/src/ir/**` in by
  `#[path]`, so a filter over that tree resolves in `specforge-core` and in no other crate, while
  `cargo test` still exits 0 when a filter matches nothing (`COMMIT-GATE-SINGLE-RUN.5`).
- [x] **ADDRESSED (verified)** — each corrected command was re-run with `--list` and now names a
  non-empty test set. The sweep is the closing evidence, not the five edits:
  `grep -rn 'cargo test -p specforge --lib' crates/ docs/ *.md` returns no command over `ir/**`, and the
  `commands::converge::tests::…` hits that remain were **checked, not assumed** — `--list` gives 1 test
  under `-p specforge` and 0 under `-p specforge-core`, so naming `specforge` there is correct.
- [x] **NO REGRESSION** — `cargo test --workspace --lib --exclude specforge-production-graph`
  **2,194 passed / 10 ignored / 0 failed**, unmoved; `cargo fmt --all -- --check` clean. The change is
  doc comments only: no production rule, signature or behaviour is touched, so `PRODUCTION-GENERICITY`
  re-derives with every count unmoved and `flow_census.json` is not edited.
- [x] **GENERICITY (ADR 0006)** — N/A: no rule, vocabulary or predicate changed. The edit names crates in
  developer instructions.
- [x] **LOCKSTEP** — these are developer-facing run instructions for `--ignored` local harnesses, not
  user-visible behaviour, so no book text changes and no production rule was deleted. The durable lesson
  already has a home in `COMMIT.md` step 8 (`.6`) and in ADR 0037's corrected `reverify` (`ADR 0047`);
  this leaf adds the sweep that those two point fixes did not do.
  Verification: the ten `--list` runs, the repository-wide sweep, and the workspace oracle

- ID: `COMMIT-GATE-SINGLE-RUN.8` · Status: `pending` (opened `2026-09-18` by `.7`) · Goal: **SpecForge
  prints a reproduction command that cannot reproduce anything.** `ir/trajectory.rs:1120` emits
  `reproduction: "cargo test -p specforge --lib ir::trajectory"` on a trajectory gap record — a
  **user-facing** instruction, not a comment. Measured `2026-09-18`: that filter matches **0 tests in
  `specforge` and 0 in `specforge-core`**, so it is not `.7`'s crate-name defect but a worse one — the
  filter resolves nowhere at all, and a user who follows it sees a green run. It is the only
  `reproduction:` literal in production (`grep -rn 'reproduction: "' crates/specforge/src/{ir,commands}`
  returns exactly one), so the population is one and the fix is bounded. Decide what the record should
  name: a test filter that exists, or a command that actually re-derives the gap it describes — and add a
  control, because an emitted command nothing executes is precisely the class of claim
  `CLAIM_VERIFICATION.md` §2 asks *"what does this check still permit?"* about. Prerequisite: none.
  Verification: pending
  Commit: pending

- ID: `COMMIT-GATE-SINGLE-RUN.9` · Status: `done` (`2026-09-20`; opened the same day by
  `SIGNAL-DECLARATION-ROW-DROP.2h.2`, which ran the step-8 oracle with the flags CI uses and found it
  already red) · Goal: **step 8 prescribed a Rust oracle that cannot fail, and the workspace had
  drifted behind it into a state `scripts/run_ci.sh` would refuse.**
  `COMMIT.md` step 8 named `cargo clippy`. A bare `cargo clippy` **prints its findings and exits 0** —
  measured, not assumed — so a slice can run the prescribed command, read a green exit, and declare
  the oracle clean while the workspace carries findings. `scripts/run_ci.sh:30` has always run
  `cargo clippy --manifest-path Cargo.toml --all-targets -- -D warnings`, so the two legs of the same
  check disagreed, and the disagreement was silent in the direction that matters: the cheap one that
  runs every slice could not fail, and the expensive one that runs at push could.
  **This is the same defect class as `.5`/`.6`** — a command that reports green for the wrong
  reason — and it had already cost something: at `16bc8e72` the workspace carried **5 findings**, so
  the branch was **un-pushable** and had been for at least three commits whose records describe
  clippy as clean.
  Fixed both halves. The five findings: four in `evidence.rs`'s `EXTRACTION-GAP-FIX.5` diagnostic —
  a counter (`subject_after_modal`) incremented in exactly the branch that already increments
  `no_signal_before_modal`, never read and never printed, plus its `+= 0` no-op arm and a
  single-pattern `match` — and one in `constraint_extract_llm.rs`: `ground_constraint_typed` takes
  eight parameters while the extractor directly above it already carries the allow for the same
  reason. The counter is **deleted** rather than printed: it duplicated a reported number, so
  printing it would have published the same measurement twice under two names.
  Non-goal: reducing `ground_constraint_typed`'s arity. Five identities plus three injected decisions
  is the shape a reader of a grounding function needs to see, and bundling them behind a struct would
  hide which injection points a call site supplies; the allow carries that reason in the source.
  Prerequisite: none.
  Verification: `cargo clippy --offline --all-targets -- -D warnings` **exit 1 (5 errors) -> exit 0**;
  `bash scripts/run_ci.sh`'s clippy leg reproduced green; `cargo fmt --all -- --check` exit 0;
  `cargo test --workspace --lib --exclude specforge-production-graph` **2,209 passed / 0 failed**;
  `scripts/check_doctrines.sh` green. The deleted counter is provably redundant: it was incremented
  on exactly the `!precedes` branch that increments `no_signal_before_modal`, and on no other.
  Commit: `COMMIT-GATE-SINGLE-RUN.9 — a clippy that exits 0 on its own findings is not an oracle`

## Acceptance Checklist (enforced) — `COMMIT-GATE-SINGLE-RUN.9`

- [x] **REPRODUCE / MEASURE** — at `16bc8e72`: `cargo clippy --offline --all-targets -- -D warnings`
  exits **1** with **5 errors**, while the step-8-prescribed `cargo clippy` exits **0** on the same
  tree. Reproduced against a pristine `HEAD` checkout of the file the slice had touched, so the
  finding is the repository's and not the slice's.
- [x] **ROOT CAUSE (WHY + WHERE)** — `COMMIT.md` step 8 prescribed the flagless form; clippy's default
  lint level is `warn`, which does not set a nonzero exit. The five findings:
  `crates/specforge/src/ir/evidence.rs:41127/41267/41270` (`subject_after_modal` assigned, never read;
  two of its assignments never read) and `:41261` (single-pattern `match`), plus
  `crates/specforge/src/ir/constraint_extract_llm.rs:421` (`ground_constraint_typed`, 8 arguments,
  where `:399` already carries `#[allow(clippy::too_many_arguments)]`).
- [x] **ADDRESSED (verified)** — findings **5 -> 0**, exit **1 -> 0**. `COMMIT.md` step 8 now
  prescribes `cargo clippy --offline --all-targets -- -D warnings` and states why each flag is load-
  bearing (`--all-targets` because four of the five were in test code).
- [x] **NO REGRESSION** — `cargo fmt --all -- --check` exit 0; `cargo test --workspace --lib
  --exclude specforge-production-graph` **2,209 passed / 0 failed / 15 ignored**; the diagnostic's
  own printed output is unchanged, because the deleted counter was never printed;
  `scripts/check_doctrines.sh` green including `PRODUCTION-GENERICITY` (the `#[allow]` attribute and
  the `if let` add no function and no decision site the census had not already counted).
- [x] **GENERICITY (ADR 0006)** — N/A: no production rule added, changed or deleted. One diagnostic
  counter removed, one `match` rewritten as `if let`, one lint allow added with its reason.
- [x] **LOCKSTEP** — `COMMIT.md` step 8 is the surface whose truth changed, and it is corrected in
  place with the measurement that forced it. No user-visible behaviour and no book text: the mdBook
  documents the product, not the commit workflow. No production rule is deleted or replaced.

- ID: `COMMIT-GATE-SINGLE-RUN.10` · Status: `done` (`2026-09-20`; opened the same day by the first
  `scripts/run_ci.sh` run of the session) · Goal: **`run_ci.sh` was RED, and had been, because
  nothing runs it between pushes.**
  Hitting the 400-commit push threshold triggered the first full `scripts/run_ci.sh` of the session.
  It failed at the rustdoc leg — `RUSTDOCFLAGS="-D warnings" cargo doc --manifest-path Cargo.toml
  --no-deps`, exit **101**, *"could not document `specforge-core`"* — on **four** intra-doc links:
  one unresolved (`is_grounded_in_source`) and three from PUBLIC documentation to a PRIVATE item
  (`carries_canonical_source_classifications` -> `neutralize_legacy_source_classifications`,
  `locally_declared_signal_identifiers` -> `is_same_clause_signal_appositive`,
  `replay_persisted_signal_declarations` -> `read_explicit_signal_declaration`).
  **All four predate this session**, verified against `ce3049bf`, its first parent: every one of the
  four link forms is present there. This was not introduced by the eleven slices that preceded it;
  it was *found* by them, because reaching the push threshold is the only thing that runs this gate.
  **This is `.9`'s finding one layer out, and the more expensive layer.** `.9` fixed a step-8 oracle
  whose exit code could not express its own findings. This is a gate whose exit code is fine and
  which **nothing executes between pushes** — the same failure `EXTRACTION-QUALITY-GAUGE.3k.2j`
  names in another corner of the repository: *a fail-closed check nothing executes is not a check.*
  The fix is the minimal honest one: each link is demoted to a plain code span naming the item as
  private. **The targets stay private** — three of them are internals a public doc may legitimately
  mention but must not link, and widening their visibility to satisfy rustdoc would trade a real
  encapsulation for a formatting convenience.
  Non-goal: adding a rustdoc leg to the per-commit hook. `.0` measured the gate's cost and the
  arithmetic that keeps `--fast` cheap applies here too; what this leaf establishes is that the leg
  exists, is fail-closed, and had gone unrun — the cadence question is `.0a`'s.
  Prerequisite: none.
  Verification: `RUSTDOCFLAGS="-D warnings" cargo doc --manifest-path Cargo.toml --no-deps` exit
  **101 -> 0**, 4 errors -> 0; then `scripts/run_ci.sh` end to end.
  Commit: `COMMIT-GATE-SINGLE-RUN.10 — run_ci.sh was red, and nothing had run it`

## Acceptance Checklist (enforced) — `COMMIT-GATE-SINGLE-RUN.10`

- [x] **REPRODUCE / MEASURE** — `bash scripts/run_ci.sh` exit **101** after 1,400 s, failing at
  *"building Rust docs with warnings denied"* with 4 rustdoc errors and *"could not document
  `specforge-core`"*.
- [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/run_ci.sh:36` runs `cargo doc` under
  `RUSTDOCFLAGS="-D warnings"`, which promotes rustdoc's `private_intra_doc_links` and
  `broken_intra_doc_links` to errors. Four doc comments use the linking form `[` `` `item` `` `]`
  for an item that is private or absent:
  `crates/specforge/src/ir/source.rs:2295`, `crates/specforge/src/ir/evidence.rs:10074`,
  `crates/specforge/src/ir/semantic.rs:6730`, `crates/specforge/src/ir/constraint_extract_llm.rs:390`.
  **Age established, not assumed**: all four link forms are present in `ce3049bf`'s copies of those
  files, so the defect predates the session that found it.
- [x] **ADDRESSED (verified)** — 4 errors -> **0**; `cargo doc` under the CI flags exits **0**. Each
  link became a plain code span naming the item as private, so the prose still points a reader at
  the right function while claiming no link rustdoc must resolve.
- [x] **NO REGRESSION** — **comment-only**: no executable line changes, so no artifact, gold, seal
  or score can move, and the producer graph is untouched. `cargo fmt --all -- --check` exit 0;
  `cargo clippy --offline --all-targets -- -D warnings` exit 0; workspace lib tests green;
  `scripts/check_doctrines.sh` green; `scripts/run_ci.sh` green end to end, which is the oracle this
  leaf exists for.
- [x] **GENERICITY (ADR 0006)** — N/A: no rule, no vocabulary, no identity; four doc comments.
- [x] **LOCKSTEP** — no user-visible behaviour and no public contract changes, so the book is
  unchanged; the four edits remove a link, not a description, and no production rule is deleted or
  replaced. The durable record is this node.

- ID: `COMMIT-GATE-SINGLE-RUN.11` · Status: `done` (`2026-09-20`) · Goal: **hosted CI had not run on
  a push since April, and the push cadence assumed it had.**
  The director's standing instruction — *monitor the GitHub CI after a push, fix and re-push until
  it passes* — was given on the belief that a push triggers it. It does not. `.github/workflows/ci.yml`
  carried `on: workflow_dispatch:` alone, made manual-only by `bc110c3d` *"to conserve GitHub Actions
  minutes"*, with its own re-enable condition written beside it: *"re-enable push/pull_request
  triggers when hosted CI minutes are available again."* Measured: the most recent hosted run before
  today was **2026-04-12**, and the 401-commit push of `dee0740f..cdeda606` produced **none**.
  **The re-enable condition is now met, and it was verified rather than assumed.** The repository was
  made public, which makes hosted minutes free; `gh api repos/rdje/specforge --jq .private` returns
  **`false`** and `.visibility` returns **`public`**. It is worth recording that the first reading was
  the other way — the check returned `PRIVATE` on two independent calls (`gh repo view` and the REST
  API) before the flip, and the change was held until it returned `public`, because enabling a
  per-push trigger on a private repository spends billable minutes and is precisely what `bc110c3d`
  switched off. **A premise about someone's spending is worth one API call.**
  The triggers are restored **exactly as `bc110c3d^` had them** — `push:` and `pull_request:`, no
  branch or path filters — recovered from that revision rather than reinvented, with
  `workflow_dispatch:` kept so a run can still be forced without a push.
  Non-goal: changing what CI runs. The job body is untouched; only when it runs changes.
  Prerequisite: none.
  Verification: `git show bc110c3d^:.github/workflows/ci.yml` for the restored block; repository
  visibility `public`. **The oracle this leaf named is WITHDRAWN, because it was wrong**: it claimed
  its own push would be the first push-triggered hosted run since `2026-04-12`, and that push
  produced no run at all. Restoring the trigger was necessary and NOT sufficient — see `.12`.
  Commit: `COMMIT-GATE-SINGLE-RUN.11 — hosted CI runs on push again, now that the minutes are free`

- ID: `COMMIT-GATE-SINGLE-RUN.12` · Status: `done` (`2026-09-20`) · Goal: **the trigger was restored
  onto an engine that was switched off.** `.11`'s own push produced no run, and a manual dispatch sat
  `queued` for **26 minutes** with no runner. Neither is a CI failure and neither is a pass.
  Measured cause: `gh api repos/rdje/specforge/actions/permissions` returned **`{"enabled":false}`** —
  Actions was disabled for the entire repository, so no trigger of any kind could start a job.
  Addressed: a `PUT` to that endpoint with `enabled=true` returns
  `{"enabled":true,"allowed_actions":"all"}`, and a fresh dispatch then reached `in_progress` on
  `4c962cd5` within seconds — the control the previous state could not pass.
  **This is the fourth gate defect of one shape in one day**: `.9` an oracle whose exit code could not
  express its findings, `.10` a gate nothing executed, `.11` a trigger switched off, `.12` the engine
  switched off. Each fix exposed the layer beneath it. **"It is configured" is not "it runs", at every
  layer of the stack, and only an observed run discharges the difference.**
  Non-goal: changing what CI runs. Prerequisite: `.11`.
  Verification: run `35497529879` reached `in_progress` and then returned a real verdict — `failure`,
  owned by `.13`. A verdict of any kind is precisely what `{"enabled":false}` made impossible.
  Commit: `COMMIT-GATE-SINGLE-RUN.12/.13/.14 — the hosted gate can start, can pass a clean checkout, and says what it did not measure`

- ID: `COMMIT-GATE-SINGLE-RUN.13` · Status: `done` (`2026-09-20`) · Goal: **the first hosted run in
  five months was RED, and the gate could not have passed a default checkout.** Run `35497529879` on
  `4c962cd5` failed with **four** doctrines FAIL that all PASS locally: `LIVE-DOC-SIZE`,
  `PUBLISHED-ASSERTIONS`, `CLAIM-VERIFICATION`, `PRODUCTION-GENERICITY`.
  Root cause — two, both in `.github/workflows/ci.yml` and neither in the doctrines:
  `actions/checkout@v4` is invoked with no inputs, so it takes its defaults, **`fetch-depth: 1`** and
  **`submodules: false`**. This gate is history-dependent and submodule-dependent. **26
  of** `PUBLISHED-ASSERTIONS`' 27 violations read *"revision ... does not resolve in this
  repository"* (the 27th is `fact-card-catalog-count`, whose producer exited 1 for the same reason);
  `LIVE-DOC-SIZE` and `CLAIM-VERIFICATION` report *"boundary commit lookup failed (git exit 128)"*;
  and `feedback-protocol` cannot find `subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md`, which lives
  in the `subs/fsmgen` submodule.
  **Bisected locally rather than guessed**, in an on-volume clone at `generated/tmp/ci-repro`: at
  `--depth 1` the `git exit 128` failures reproduce exactly; after `git fetch --unshallow` they
  vanish and `LIVE-DOC-SIZE` fails on the missing submodule file alone; after
  `git submodule update --init --recursive`, **`LIVE-DOC-SIZE` and `PUBLISHED-ASSERTIONS` both PASS**.
  Depth and submodules are each necessary, and together sufficient, for those two.
  **The deeper point: this gate has never run hosted.** The last push-triggered run was `2026-04-12`,
  before most of these doctrines existed, so "CI is green" has been a statement about a configuration
  no doctrine had ever been executed under. A gate is only proved by the environment it actually runs
  in, and a default `actions/checkout` is not the environment this one needs.
  Non-goal: weakening any doctrine to fit a shallow checkout. The checkout is what is wrong.
  Prerequisite: `.12`.
  Verification: each bisection step above, re-run in `generated/tmp/ci-repro`; and the hosted run on
  this leaf's own push, which is the only oracle that counts and is recorded here once it is green.
  Commit: `COMMIT-GATE-SINGLE-RUN.12/.13/.14 — the hosted gate can start, can pass a clean checkout, and says what it did not measure`

- ID: `COMMIT-GATE-SINGLE-RUN.14` · Status: `done` (`2026-09-20`) · Goal: **the aggregate report
  turns "nothing was measured" into PASS.** Chasing `.13`'s hosted failures into the corpus-dependent
  doctrines looked at first like four vacuous passes. It is not, and the real defect is narrower and
  worse placed. **The enforcers are honest.** With no `generated/`, `check_chain_currency.sh`,
  `check_proof_seal_currency.sh`, `check_proof_seal_total.sh` and `check_corpus_frontier.sh` each
  print a loud skip — *"this doctrine does not govern this tree. Nothing was measured; nothing is
  claimed"* — and exit 0 deliberately, a decision `check_proof_seal_currency.sh`'s own self-test case
  16 pins end to end. **`scripts/check_doctrines.sh` then reports `PASS  CHAIN-CURRENCY — every
  persisted corpus artifact is exactly what the current binary reproduces`, and drops the skip
  lines**, because the output it relays from a PASSING enforcer is filtered to lines matching
  `warning` (`.4`). The single output the hook, `run_ci.sh` and COMMIT.md step 8 all show is the one
  that loses the qualification.
  **This is `.4` again with a different word.** That leaf found a passing check's warnings discarded
  and surfaced them; the same filter silently decided a declared non-applicability was not worth
  relaying. The driver already refuses to let a subset run read as a complete one under `--only`, and
  an unmeasured doctrine must not read as an enforced one either.
  Addressed in two parts, both in `scripts/check_doctrines.sh`. **First**, a declared skip is no
  longer reported as a pass: the driver recognises an enforcer's own `SKIP:` / `SKIPPED -` line and
  reports `SKIP  <id> — enforcer declared it does not govern this tree`, relaying the reasons under
  a new `NOT GOVERNED` section, instead of `PASS` and silence. The match is deliberately narrow,
  because a false positive would downgrade a doctrine that DID measure — the opposite failure, and
  the worse one.
  **Second**, the corpus is asserted rather than assumed, because the other two corpus doctrines do
  not skip — they FAIL, and not for a reason a runner can fix: `CLAIM-VERIFICATION` asserts
  `stdout_contains` against producers that have nothing to derive from, and
  `check_behavioral_genericity_contract.py` reads `generated/source_ir/<key>/source_ir.json` per
  document. An absent corpus is now a REFUSAL (exit 2) unless the environment declares itself
  corpus-free with `SPECFORGE_CORPUS_ABSENT=1`, and under that declaration all six are reported not
  discharged and named in the summary sentence. **The declaration is a permission, never an
  override**: where a corpus is present it changes nothing, so it cannot duck enforcement that is
  actually possible.
  Measured in the clean clone: `ALL 12 executed doctrines PASS (18 registered)`, with
  `NOT GOVERNED here, so NOT enforced by this run: PRODUCTION-GENERICITY, CORPUS-FRONTIER,
  CLAIM-VERIFICATION, PROOF-SEAL-CURRENCY, PROOF-SEAL-TOTAL, CHAIN-CURRENCY` — the same tree that
  previously reported four of those as PASS. Undeclared, that run now exits 2.
  The corpus genuinely cannot be rebuilt on a runner: `generated/source_ir` holds **78** documents
  and `/generated/` is gitignored, and only **21** of the 78 were built from a source tracked in
  `corpus/` — the other **57** record `path_origin: external_input` under `.cache/local-references/`.
  (22 PDFs are tracked; one, NVMe 2.0a, has no persisted SourceIR, so 22 and 21 count different
  things, and the first draft of this leaf published their difference as though it were the gap.) Hosted enforcement of the corpus stratum is unavailable at any price; saying so
  plainly is the only honest option, and is what this leaf makes the report do.
  Non-goal: giving the corpus stratum a hosted subject — `.14a`.
  Prerequisite: `.13`.
  Verification: in the clean clone, the undeclared run exits 2; the declared run reports 12 PASS and
  6 NOT GOVERNED where four of those six previously read PASS; and with the corpus present the local
  gate still executes and passes all 18, so the declaration changed nothing where it must not.
  Commit: `COMMIT-GATE-SINGLE-RUN.12/.13/.14 — the hosted gate can start, can pass a clean checkout, and says what it did not measure`

- ID: `COMMIT-GATE-SINGLE-RUN.15` · Status: `done` (`2026-09-20`) · Goal: **the gate is green and
  the step after it is not.** `.13` and `.14` are confirmed hosted: push-triggered run
  `35502186694` on `65e2b4d2` — the first push-triggered run since `2026-04-12` — printed
  `ALL 12 executed doctrines PASS (18 registered)` and named the six NOT GOVERNED, which is exactly
  the designed report. The job still failed, on the NEXT line of `scripts/run_ci.sh`, for two causes
  and neither is a doctrine.
  **First: `run_ci.sh` steps around its own driver.** Its second step is
  `./scripts/check_production_genericity.sh --self-test`, invoked directly rather than through
  `check_doctrines.sh --only`, so the corpus declaration `.14` put in the driver never reached it and
  its corpus components ran against nothing — *"current SourceIR is missing"* for every document,
  *"frozen census prior_memory is missing"*. **A rule the driver enforces is not enforced on the path
  that walks around the driver**, and `.1` added `--only` precisely so that no caller would ever have
  to invoke a gate script by hand. The gate's own pipeline was the caller still doing it.
  **Second: `--offline` cannot resolve on a cold registry.** `check_production_genericity.sh` and its
  `_flow`/`_graph` siblings run `cargo --locked --offline`; with nothing fetched, resolution failed
  with `no matching package named 'zmij' found`. That was checked before being treated as
  environmental: `zmij 1.0.21` is a legitimate transitive dependency of `serde_json 1.0.149`, in
  `Cargo.lock` with its checksum and already in the local cache — the runner had simply never
  fetched it. **The hermeticity `--offline` buys is real and is KEPT**; the cache is primed before
  the suite instead of the flag being dropped.
  **The two causes were entangled in the hosted log and were separated before either was fixed.**
  Re-run corpus-free with a WARM cargo cache, in an on-volume clone outside the repository: **9 of
  the 11 components PASS**, including `INFORMATION-FLOW`, `FLOW-MUTATIONS` and `ALPHA-OBLIGATIONS`,
  which the hosted run had reported as failures. Only `BEHAVIORAL-CONTRACT` and
  `BEHAVIORAL-CONTRACT-MUTATIONS` actually need the corpus. Every other hosted genericity failure was
  the cold cache wearing a doctrine's name.
  **Third, found by simulating the whole suite rather than waiting for the runner to find it.** The
  hosted job had never once reached `cargo test`, so nothing yet said whether the suite needs the
  corpus. Run corpus-free in the clone, it does: **6 tests** in `specforge-conformance`'s
  `behavioral_genericity` module fail with `authority_unavailable: behavioral source authority is
  unavailable`, replaying a normalized bundle that a runner cannot have. They are now skipped BY
  NAME when the environment is declared corpus-free — one by one, because `--skip
  behavioral_genericity` would also drop that module's alpha-transform, comparator and classifier
  tests, which need no corpus and are real coverage a runner CAN collect. **A renamed test stops
  being skipped and the suite goes red, which is the safe direction; an over-broad filter would go
  quiet instead.** Finding this locally cost one clone run and saved a third hosted iteration.
  Addressed: `.github/workflows/ci.yml` primes the registry with `cargo fetch --locked` before the
  suite; `check_production_genericity.sh` makes the same corpus assertion the driver makes and
  reports its two corpus components as `SKIP` when the environment is declared corpus-free, refusing
  outright when it is not. **Because that skip is per COMPONENT, `PRODUCTION-GENERICITY` is removed
  from the driver's coarse `CORPUS_DEPENDENT` set** — hosted now enforces 13 of 18 doctrines and 9 of
  11 genericity components instead of skipping all 11. That is most of what `.14a` was opened to
  recover, bought by evidence that was already in hand.
  **And `.14`'s own comment came true inside the hour.** That relay is matched on a text pattern, and
  the comment beside it says a false positive "would silently downgrade a doctrine that DID measure,
  which is the opposite failure and a worse one". Adding per-component skip lines here produced
  exactly that: `PRODUCTION-GENERICITY` emitted `SKIP:` for its two corpus components, the driver
  read one and reported the whole doctrine as measuring NOTHING while nine components had passed.
  Caught by the control run, not by reading. The relay now ignores any enforcer that prints its own
  `PASS`/`FAIL` component report: **a composite enforcer adjudicates its own coverage and says so in
  its own summary, and the driver must not re-decide that from a substring.** The lesson is narrower
  than "be careful": a heuristic that reads another tool's prose is only safe while that tool's prose
  is unchanging, and this one changed because the same commit changed it.
  Non-goal: the same treatment for `CLAIM-VERIFICATION`, which needs registry semantics for a
  corpus-dependent command rather than a component list — still `.14a`.
  Prerequisite: `.14`.
  Verification: the isolation run above; `scripts/run_ci.sh` completing corpus-free in the clone;
  the corpus-free driver reporting
  `ALL 13 executed doctrines PASS (18 registered)` with `PRODUCTION-GENERICITY` among the executed
  and five named NOT GOVERNED, where before this leaf it read 12 and six; and the hosted run on this
  leaf's own push.
  Commit: `COMMIT-GATE-SINGLE-RUN.15 — the driver's rule did not reach the pipeline that walks around it`

- ID: `COMMIT-GATE-SINGLE-RUN.14a` · Status: `pending` (opened `2026-09-20`, narrowed by `.15`) ·
  Goal: **decide whether the corpus stratum should have a hosted subject at all.** `.15` already
  recovered `PRODUCTION-GENERICITY` by declaring corpus dependency per component, so what remains
  ungoverned hosted is five doctrines — `CORPUS-FRONTIER`, `CLAIM-VERIFICATION`, both
  `PROOF-SEAL-*`, `CHAIN-CURRENCY` — plus two genericity components. That is the correct report and
  still a real hole: the strongest guarantees in the project are enforced on one machine.
  **The cheap option is measured and it is cheap.** A hosted subject does NOT need Docling: the
  corpus doctrines replay a persisted `source_ir.json` with the Rust binary, never the PDF. 21
  documents have a source tracked in `corpus/`, and their full artifact chains run from **1.3 MB**
  (`ihi0032_c`, AMBA Trace Bus) — the three smallest total **4.2 MB**, against 386.7 MB for AXI. So
  one or two tracked fixture documents would give five doctrines a real hosted subject for
  single-digit megabytes.
  **The cost is coupling, not size, and it is the reason this is a decision and not a chore**: once
  those artifacts are tracked, every binary change that moves them makes hosted CI red until they are
  regenerated. That is exactly what the doctrine is for, and it is a standing commitment the director
  should take deliberately.
  The alternative is to accept the hole and state it in `DOCTRINE_ENFORCEMENT.md` as a declared
  boundary. **Do not close this by widening the skip.**
  `CLAIM-VERIFICATION` needs separate work either way: it is coarse-skipped because a claim command
  asserts `stdout_contains` against a corpus-derived producer, so the registry needs a way to declare
  a command corpus-dependent — a component list cannot express it.
  Prerequisite: `.14`. Verification: pending. Commit: pending

- ID: `COMMIT-GATE-SINGLE-RUN.0a` · Status: `pending` (opened `2026-09-17`) · Goal: **re-measure the gate on
  a machine proved idle**, because `.0`'s table was taken at load average 12.95 and its shares are
  withdrawn. The tool now refuses above a 2.0 load threshold and prints the load at both ends of the run, so
  the precondition is asserted rather than claimed; what is missing is a run that passes it.
  **What the re-measurement must settle**, and only the first is already supported: the costliest-four
  MEMBERSHIP (identical across three contended runs), their share (observed 87.0% / 89.8% / 90.7% — a band,
  not a value), and the ordering within them (unstable: `PROOF-SEAL-CURRENCY` ranked 3rd, 1st, 3rd).
  **Take at least three passing runs and publish the spread**, not one run's numbers. A single measurement
  of this gate has now been shown twice to be unreproducible, once by 2.4x on one doctrine.
  **AMENDED by `.3a` (`2026-09-17`), which changes what this leaf has to do.** The load threshold is
  necessary and NOT sufficient: `--only LIVE-DOC-SIZE` was measured at **89s, 214s, 220s, 266s** across
  four consecutive runs on one unchanged tree at load **4.90-5.51**, and the full nine-run matrix spans
  **87s-506s**. So three runs is too few to characterise a 3x-at-constant-load distribution, and a single
  reading per doctrine — which is what `scripts/measure_doctrine_cost.sh` still emits — cannot be made
  honest by any precondition.
  **What that means concretely, so this leaf is not re-scoped from scratch:** the tool must repeat each
  doctrine N times and publish min/median/max per row rather than one number, and the table must be read
  as a distribution. Membership of the costliest four can survive this (noise reorders neighbours but
  cannot lift a sub-second doctrine past a 90-500s one — `.3a`); a share cannot, and none is published.
  If an idle machine turns out to collapse the spread, that is itself the finding and worth recording.
  Prerequisite: `.0`; blocks nothing — `.2` can decide on membership alone, which is what it needs.
  Verification: pending
  Commit: pending

## Current Frontier

Ordered; PNT selects the first eligible leaf.

1. `COMMIT-GATE-SINGLE-RUN.15` — **CLOSED `2026-09-20`.** `.13`/`.14` are confirmed hosted, and the
   job still failed one line later: `run_ci.sh` calls `check_production_genericity.sh` DIRECTLY, so
   the corpus rule the driver enforces never reached it, and `--offline` could not resolve on a cold
   runner registry. Separated before either was fixed — corpus-free with a warm cache, **9 of 11**
   genericity components pass, so every other hosted genericity failure was the cold cache wearing a
   doctrine's name. Per-component skipping took hosted enforcement from 12 to **13 of 18**.
   **`.14`'s own warning came true inside the hour**: its text-matched relay read one of the new
   per-component `SKIP:` lines and downgraded a doctrine that had measured nine things. A heuristic
   that reads another tool's prose is safe only until that prose changes — and the same commit
   changed it.

2. `COMMIT-GATE-SINGLE-RUN.13` — **CLOSED `2026-09-20`.** The first hosted run in five months was
   **RED**: four doctrines that pass locally failed on the runner, because `actions/checkout@v4` was
   taking its defaults — `fetch-depth: 1` and `submodules: false` — and this gate needs full history
   and `subs/fsmgen`. Bisected in an on-volume clone, both inputs now set. **This gate had never run
   hosted**: the last push-triggered run predates most of the doctrines, so "CI is green" described a
   configuration not one of them had ever been executed under.
3. `COMMIT-GATE-SINGLE-RUN.12` — **CLOSED `2026-09-20`.** `.11` restored the trigger onto an engine
   that was switched off — `actions/permissions` read `{"enabled":false}`, so its push produced no
   run and a manual dispatch sat queued 26 minutes. Enabled; a dispatch then started in seconds.
   **Seven gate defects in one day.** Five are one shape — nothing ran the gate: `.9` an oracle whose
   exit code could not express its findings, `.10` a gate nothing executed, `.11` a trigger switched
   off, `.12` the engine switched off, `.13` the gate running where it cannot pass. Two are a second
   shape — the gate ran and the report lied about it: `.14` a declared skip printed as `PASS`, `.15`
   the driver's rule not reaching the pipeline that calls around the driver. Each fix exposed the
   layer beneath it. **"It is configured" is not "it runs", only an observed verdict separates them —
   and a verdict is only worth what its report says honestly.**
4. `COMMIT-GATE-SINGLE-RUN.11` — **CLOSED `2026-09-20`.** Hosted CI had not run on a push since
   **2026-04-12**: the workflow was `workflow_dispatch`-only to conserve minutes, and the 401-commit
   push produced no run at all. The repository is now public so the re-enable condition its own
   comment named is met, and `push:`/`pull_request:` are restored exactly as `bc110c3d^` had them.
   Necessary, and by itself not sufficient — `.12` and `.13` are the rest of it.
5. `COMMIT-GATE-SINGLE-RUN.10` — **CLOSED `2026-09-20`.** `run_ci.sh` was RED and had been: its
   rustdoc leg failed on four intra-doc links that all predate the session, found only because
   reaching the 400-commit push threshold ran the gate for the first time. **`.9` fixed an oracle
   whose exit code could not express its findings; this one's exit code was fine and nothing
   executed it.** Both are the same lesson at different layers, and the second is the more expensive
   one — it blocks a push rather than a commit.
6. `COMMIT-GATE-SINGLE-RUN.9` — **CLOSED `2026-09-20`.** Step 8 prescribed `cargo clippy`, which
   prints its findings and exits **0**; `scripts/run_ci.sh` has always denied warnings. The cheap leg
   that runs every slice could not fail and the expensive leg at push could, so the workspace drifted
   to **5 findings** and the branch was un-pushable across at least three commits whose records call
   clippy clean. Both halves fixed. **The lesson generalises past clippy: a step-8 oracle whose exit
   code cannot express its own findings is not an oracle**, and the other step-8 commands are worth
   re-reading with that question.
7. `COMMIT-GATE-SINGLE-RUN.8` — SpecForge prints a reproduction command that resolves to **no test in
   either crate**. `.7` swept the crate name out of every documented run command over `ir/**`; this one is
   not a comment but a `reproduction:` field the product emits on a trajectory gap record, and its filter
   `ir::trajectory` matches 0 tests in `specforge` and 0 in `specforge-core`. Population is one. Decide
   whether the record should name a filter that exists or a command that re-derives the gap, and control it.

0. `COMMIT-GATE-SINGLE-RUN.0a` — re-measure on a machine proved idle, and publish a spread. **Runnability is
   a property of the MOMENT, not of the machine — check it, never assert it.** An earlier revision of this row
   read *"not runnable on the current machine … has held near 9 all session"*; on the director's challenge the
   load was re-read at **3.61** the same day, having fallen from 10.5, so the second half was already false and
   the first half was the wrong KIND of claim. `scripts/measure_doctrine_cost.sh` adjudicates it at the instant
   it runs and refuses above 2.0; ask it. That refusal is the control `.0` lacked; do not override it.
   `.0a` can move `FAST_EXCLUDE` without touching anything else if the idle membership differs. **`.3a`
   amended its scope**: repeat each doctrine and publish min/median/max, because one unchanged tree at
   constant load already gave 89s-266s on a single doctrine.

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
