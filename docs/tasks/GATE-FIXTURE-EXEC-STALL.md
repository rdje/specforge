# GATE-FIXTURE-EXEC-STALL: the commit gate's runtime is hostage to the host's executable-assessment daemon

## Metadata

- Tree ID: `GATE-FIXTURE-EXEC-STALL`
- Status: `active`
- Roadmap lane: `R16` (doctrine enforcement / developer throughput)
- Created: `2026-09-19`
- Last updated: `2026-09-19`
- Owner: repo-local workflow

## Goal

Make the doctrine gate's wall-clock cost a function of the work it does, not of how many **new**
executable files it asks the operating system to assess. The gate must stay honest — no check is
weakened, skipped, or made conditional — while ceasing to pay a per-fixture security-assessment toll
that has no relationship to what it is proving.

## Non-Goals

- Changing the host's security posture. Excluding a path from Gatekeeper/XProtect is the machine
  owner's decision, is not repository-portable, and would hide the cost rather than remove it.
- Weakening `test_live_document_size.pl`. Its 113 checks are the reason `LIVE-DOC-SIZE` is trusted;
  this tree must not trade coverage for speed.
- Making the gate conditional on the environment. A gate that runs a different set of checks on a
  slow host is not the same gate.

## The finding (measured `2026-09-19` during `CORPUS-CHAIN-CURRENCY.10a`)

A `git commit` sat in `.githooks/pre-commit` for **41 minutes** and was aborted. The whole tree —
`git commit` → `pre-commit` → `check_doctrines.sh` → `test_live_document_size.pl` →
`check_live_document_size.pl` → `bash <fixture>/scripts/currency-ok.sh` — was sleeping at a combined
**0.03 s of CPU**. The innermost blocked process was a fixture script whose entire body is
`#!/usr/bin/env bash` + `exit 0`.

**It is not a deadlock, and it is not this repository's logic.** Measured directly:

| exec | latency |
| --- | ---: |
| first exec of a brand-new script | **127,254 ms** |
| next brand-new script | 223 ms |
| next brand-new script | 177 ms |
| re-exec of an already-assessed script | **8 ms** |

Concurrently `XprotectService` held 75.1% CPU and `syspolicyd` 47.3% (152 hours accumulated). macOS
assesses an executable on first exec; when that daemon is backed up, the assessment blocks the caller
for minutes. The repository's processes were waiting, not working.

**The cost is quantified by the gate's own numbers.** A full `scripts/check_doctrines.sh` run on the
same tree, immediately afterwards: **18m07s wall against 3m55s user CPU**. All 16 executed doctrines
PASS. Roughly 14 of those 18 minutes bought nothing.

**Why the repository owns a share of it.** `new_fixture` builds each fixture with
`tempdir('live-document-size-tests.XXXXXX', DIR => $generated, CLEANUP => 1)` and writes its verifier
scripts inside. Every fixture therefore presents the operating system with a **new** executable at a
**new** path — about 113 of them per run — so the gate requests ~113 first-exec assessments per run
for three distinct scripts whose content never varies. That multiplier is ours, and it is the part we
can remove without touching anyone's security settings.

**Second-order damage, and it is the reason this is not merely slow.** The stall is
indistinguishable from a hang: zero CPU, no output, no progress marker. It cost this session an
aborted commit, a killed process tree, and a wrong first hypothesis (a pipe deadlock). It is also
intermittent — once the daemon drains, the same command finishes in 1m37s — so it will be
re-diagnosed from scratch by the next session that meets it unless this tree exists.

## Acceptance Criteria

- Fixture verifier scripts are presented to the operating system as a bounded, reused set rather than
  one new executable per fixture, with no change to what any check asserts.
- `test_live_document_size.pl` still reports its full declared check count and still goes RED on every
  case it goes RED on today.
- The gain is measured the way the loss was: wall clock **and** user CPU, cold and warm, before and
  after, on this host.
- The stall is made legible where a future session will meet it, so the next occurrence is recognised
  in seconds rather than re-derived over 40 minutes.

## Task Tree

- ID: `GATE-FIXTURE-EXEC-STALL`
  Status: `active`
  Goal: make gate wall-clock a function of gate work
  Children: `.1`, `.2`, `.3`

- ID: `GATE-FIXTURE-EXEC-STALL.1`
  Status: `done` (`2026-09-19`, PROBE/DOC)
  Goal: **measure the multiplier before changing anything.** Count the distinct executable paths the
  gate creates and execs in one full run, and the distinct *contents* behind them. The finding above
  asserts ~113 paths for 3 contents from one test; confirm it, and enumerate every other gate-tier
  check that writes-then-execs a fixture, because `test_live_document_size.pl` is unlikely to be the
  only one and a fix that lands on one of several is a fix that does not show up in the wall clock.
  Acceptance: an exact path/content census per producing check, plus a cold/warm wall-clock and user-
  CPU baseline for the full driver on this host.
  **MEASURED, and this tree's own estimate was wrong in both directions.** The finding above said
  "~113 paths for 3 contents". Instrumenting `write_text` to log every executable-mode fixture write
  (producer restored byte-identically afterwards) gives **228 writes, 226 distinct paths, and 2
  distinct contents** — twice the paths for two-thirds the contents. The two contents are
  `#!/usr/bin/env bash\nexit 0\n` (226 of them) and the same with `exit 1` (2).
  **One producer owns it, which is the answer `.2` needed.**

  | producer | fresh executable paths per gate run | gate-tier |
  | --- | ---: | --- |
  | `scripts/test_live_document_size.pl` | **226** | yes |
  | `scripts/check_proof_seal_currency.sh` | 5 | yes |
  | `scripts/test_knowledge_map_projection.sh` | 2 | yes, via `LIVE-DOC-SIZE` |
  | `scripts/rebuild_stage_cascade.sh` | 1 | no — a remedy, not a gate step |

  ≈**233 per gate run**, 97% of them from one file. `.2` therefore has a single target rather than a
  scattered one, and a fix landing only on the others would not move the wall clock.
  **The tax does not require a stall, and that changes the case for `.2`.** Eight samples on a QUIET
  host: fresh first exec **111 ms**, already-assessed re-exec **12 ms** — a 99 ms delta. At 233 fresh
  paths that is **≈23 s of every gate run**, permanently, spent having the operating system assess
  scripts whose content is one of two fixed strings. No episode needed.
  **Baseline, quiet host against the stalled episode — same work, 3.1x the clock.**

  | measurement | quiet (`2026-09-19`) | stalled episode (`2026-09-19`) |
  | --- | ---: | ---: |
  | `check_doctrines.sh` wall | **5m49.7s** | 18m07s |
  | user CPU | **3m59.2s** | 3m55s |
  | sys | 1m14.9s | — |
  | verdict | ALL 16 executed PASS | ALL 16 executed PASS |

  User CPU is within 4 seconds across the two. The work is identical; **12m18s of the difference is
  waiting**, which is ≈3.7 s averaged over the 233 execs — a degraded daemon rather than a maximally
  backed-up one. `--fast` on the same tree: 51.8 s.
  **Frequency, stated as what was observed rather than as a rate.** One day: present from roughly
  `00:50` to `02:50` (two episodes — a 41-minute commit and an 18-minute gate), absent from ~`03:00`
  across many runs with `probe_exec_assessment_latency.sh` reporting quiet each time. So the condition
  is **episodic, lasting tens of minutes**. It is not continuous, and a single day does not support a
  rate; what it does support is that a slow gate must be *measured* rather than attributed, which is
  what `.3` shipped.
  Verification: instrumented census over `test_live_document_size.pl --quiet` reporting 113/113 with
  the producer restored byte-identically; `chmod`/shebang enumeration across every gate-tier enforcer;
  `probe_exec_assessment_latency.sh` at 8 samples; full `check_doctrines.sh` timed on a quiet host.
  Commit: `GATE-FIXTURE-EXEC-STALL.1 — census the multiplier, and correct this tree's own estimate`

- ID: `GATE-FIXTURE-EXEC-STALL.2`
  Status: `pending`
  Goal: **present a bounded set of verifier executables instead of one per fixture**, and prove the
  checks are unchanged. The shape to evaluate first is a stable per-run script directory the fixtures
  reference, so identical content is written once and assessed once; the constraint is that fixtures
  must stay isolated from each other, which is why this is a leaf and not an edit.
  **`.1` narrowed the target to one file.** `scripts/test_live_document_size.pl` writes **226** of the
  **233** fresh executable paths a gate run creates, for **2** distinct contents. Two shared scripts
  would replace 226 assessments with 2. At the measured quiet-host delta of 99 ms per fresh exec that
  recovers **≈23 s of every gate run**; during an assessment episode it recovers far more, but the
  quiet-host figure alone is the justification and does not depend on the episode recurring.
  The isolation constraint is the real design question: fixtures must not be able to influence one
  another through a shared script, and the two contents differ only by exit status, so the shared pair
  is `exit 0` and `exit 1` and nothing fixture-specific may ever be written into them.
  Acceptance: identical declared check count, identical RED behaviour on every self-test case, and a
  measured drop in first-exec assessments; the before/after numbers recorded here.
  Prerequisite: `.1`.
  Verification: `pending`
  Commit: `pending`

- ID: `GATE-FIXTURE-EXEC-STALL.3`
  Status: `done` (`2026-09-19`, CODE/DOC)
  Goal: **make the stall self-identifying.** A gate that blocks at zero CPU with no output is
  indistinguishable from a hang, and that ambiguity is what cost this session 40 minutes. Emit a
  progress marker per doctrine and, when a step exceeds a generous threshold, name the host condition
  and how to confirm it (`XprotectService`/`syspolicyd` CPU, the fresh-vs-cached exec probe) so the
  next session reaches the answer in seconds. Route the diagnostic into `TOOLBOX.md`.
  Acceptance: an induced stall produces a message that names the cause; the toolbox entry reproduces
  the exec-latency probe.
  **Shipped.** `check_doctrines.sh` now names each doctrine on stderr before running it, and a
  per-step watchdog prints, after `SPECFORGE_DOCTRINE_STALL_SECONDS` (default 120), which doctrine is
  still going, that ~0% CPU means waiting rather than hanging, and the one command that settles it. A
  completed run lists every step past the interval under `---- SLOW ----` with its measured cost. The
  report on stdout is byte-unchanged; all of this is stderr.
  `scripts/probe_exec_assessment_latency.sh` is the reproducer: fresh-script first exec against
  already-assessed re-exec, with a verdict either way. It **measures rather than asserts**, because the
  numbers move with the daemon's backlog — that is what makes the condition intermittent, and a pinned
  figure would be the wrong thing to carry. `--self-test` 6/6 over timer, freshness, verdict and
  residue.
  **The instrument built to reveal a stall introduced one, and that is the most useful thing this leaf
  produced.** The first watchdog backgrounded itself and returned its pid through a command
  substitution — `stall_pid="$(stall_watch "$id")"`. A command substitution does not return until its
  stdout pipe closes, and a process backgrounded inside it inherits that pipe and holds it open for the
  whole sleep, so the driver waited the FULL notice interval before every doctrine. `--fast` went from
  ~54 s to over ten minutes.
  **It was caught by this leaf's own probe, and only because the probe is allowed to say no.** The
  progress stream showed steps stalling at exactly 120 s while
  `probe_exec_assessment_latency.sh` reported *no assessment stall* — fresh exec 134 ms against 13 ms
  cached. The diagnostic excluded the host, which left the change. A probe that could only ever confirm
  the hypothesis it was built for would have agreed, and the regression would have shipped behind the
  story it was designed to tell. Fixed by letting the caller background it (`stall_watch "$id" &`);
  `--fast` measured **17.3 s** afterwards, and the mechanism is recorded in the function's comment
  rather than only here.
  Verification: watchdog fired on demand at `SPECFORGE_DOCTRINE_STALL_SECONDS=1` naming the doctrine
  and the remedy; `---- SLOW (1) ---- CLAIM-VERIFICATION 1s` emitted with its measured cost; default
  run 12 progress lines and **0** stall notices in 17.3 s; `probe_exec_assessment_latency.sh
  --self-test` 6/6 with RED observed for the orphaned-workspace case (5/6 plus the orphan directory
  appearing) and the producer restored byte-identically.
  Commit: `GATE-FIXTURE-EXEC-STALL.3 — make the stall say what it is, and catch the one it caused`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `GATE-FIXTURE-EXEC-STALL.2` | `pending` | the only leaf left, and `.1` has narrowed it to one file: replace 226 fresh executable paths with 2 shared ones in `test_live_document_size.pl`, recovering ≈23 s of every gate run on a quiet host without weakening a single check. The isolation constraint is the design question, not the saving |
| — | `GATE-FIXTURE-EXEC-STALL.1` | `done` | 226 paths / 2 contents / 97% from one producer; quiet 5m49.7s wall vs 3m59.2s CPU against the episode's 18m07s vs 3m55s |
| — | `GATE-FIXTURE-EXEC-STALL.3` | `done` | the stall now names itself, and the probe that does it already excluded a wrong hypothesis once |

## Blockers

- None. The host condition is not a blocker: the repository-side multiplier is removable without it.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-19` | finding | exec-latency probe (fresh vs cached), `ps` CPU accounting on the blocked tree, full `check_doctrines.sh` timed | fresh first exec **127,254 ms** vs cached **8 ms**; blocked tree at **0.03 s CPU / 41 min**; full gate **18m07s wall / 3m55s CPU**, all 16 executed doctrines PASS |
| `2026-09-19` | `.3` | induced stall at `SPECFORGE_DOCTRINE_STALL_SECONDS=1`; default `--fast`; `probe_exec_assessment_latency.sh --self-test` with a reverted perturbation | watchdog named the doctrine and the remedy; `---- SLOW (1) ---- CLAIM-VERIFICATION 1s`; default run 12 progress lines / 0 notices / **17.3 s**; self-test **6/6** with RED observed at 5/6 for the orphaned workspace |
| `2026-09-19` | `.3` self-inflicted | the probe run against the instrumented driver | driver steps stalling at exactly 120 s while the probe reported **no assessment stall** (134 ms fresh vs 13 ms cached) — the diagnostic excluded the host and localised the regression to the watchdog's command substitution |
| `2026-09-19` | `.1` | instrumented `write_text` census over `test_live_document_size.pl --quiet`, producer reverted byte-identically | **228 executable writes, 226 distinct paths, 2 distinct contents**; the tree's own "~113 paths / 3 contents" estimate was wrong in both directions |
| `2026-09-19` | `.1` | `chmod`/shebang enumeration across every gate-tier enforcer | ≈**233** fresh executable paths per gate run: 226 from `test_live_document_size.pl`, 5 from `check_proof_seal_currency.sh`, 2 from `test_knowledge_map_projection.sh`; `rebuild_stage_cascade.sh` is a remedy, not a gate step |
| `2026-09-19` | `.1` | `probe_exec_assessment_latency.sh` at 8 samples, quiet host | fresh **111 ms** vs cached **12 ms** — a 99 ms delta, so ≈**23 s of every gate run** is assessment of two fixed script contents, with no episode required |
| `2026-09-19` | `.1` | full `check_doctrines.sh` timed, quiet host | **5m49.7s wall / 3m59.2s user / 1m14.9s sys**, ALL 16 executed PASS, 0 stall notices — against the episode's 18m07s wall / 3m55s user. Same CPU, **12m18s of pure waiting** |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| finding | `CORPUS-CHAIN-CURRENCY.10a` | found while committing that leaf; the gate stall is not that tree's subject |
| `GATE-FIXTURE-EXEC-STALL.3` | `GATE-FIXTURE-EXEC-STALL.3 — make the stall say what it is, and catch the one it caused` | shipped the progress stream, the watchdog, the probe, and the toolbox route |
| `GATE-FIXTURE-EXEC-STALL.1` | `GATE-FIXTURE-EXEC-STALL.1 — census the multiplier, and correct this tree's own estimate` | 226 paths / 2 contents / one producer at 97%; quiet-host baseline paired against the episode |

## Changelog

- `2026-09-19`: Created. A 41-minute pre-commit stall was root-caused to host executable assessment,
  not to repository logic — and to a repository-owned multiplier of ~113 fresh executables per gate
  run for three unchanging script contents.
- `2026-09-19`: `.3` closed. The gate now says which doctrine is running and, past a notice interval,
  what a zero-CPU stall usually is and how to settle it. Its own probe immediately earned its keep by
  refusing to confirm the hypothesis it was built for, which is how the watchdog's command-substitution
  regression was found before it shipped.
- `2026-09-19`: `.1` closed. The multiplier is **226 fresh executable paths for 2 distinct contents**
  from a single producer, not ~113 for 3 spread across several — and it costs ≈23 s of every gate run
  on a quiet host, before any assessment episode. The quiet baseline (5m49.7s wall / 3m59.2s CPU)
  against the episode (18m07s / 3m55s) shows identical work and 12m18s of waiting.
