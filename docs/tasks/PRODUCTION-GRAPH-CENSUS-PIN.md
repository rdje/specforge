# PRODUCTION-GRAPH-CENSUS-PIN: four repository-wide census numbers drifted under a fully green gate

## Metadata

- Tree ID: `PRODUCTION-GRAPH-CENSUS-PIN`
- Status: `active` (`2026-09-12`; `.0`-`.2` done, `.2a` corrects `.2`; `.3` open)
- Roadmap lane: `process / doctrine enforcement`
- Created: `2026-09-12`
- Last updated: `2026-09-12`
- Owner: repo-local workflow

## Goal

`tools/production-genericity-graph` derives a repository-wide census — analyzed functions, helper
edges, decision sites, semantic macros — and `current_repository_flow_is_complete_local_and_deterministic`
pins all four exactly. When `PROSE-NAME-CELL-DECLARATION.2` ran `cargo test` it found that test red,
and red **before** its own change: the pins had drifted by **+18 functions, +179 helper edges, +226
decision sites, +3 semantic macros** while every commit reported a fully green doctrine gate.

Restore the pins to measured truth, and close the gap that let them drift silently.

## How it was found

`PROSE-NAME-CELL-DECLARATION.2` added one production function and three tests, ran `cargo test`, and
saw `left: 2399, right: 2380`. Stashing the change and re-running gave `left: 2398` — so 18 of the 19
predated the slice entirely. The same measurement across all four pins:

| pin | pinned in the test | at `4bb6c1c8` (pre-existing drift) | with `.2`'s change | `.2`'s own contribution |
| --- | ---: | ---: | ---: | ---: |
| `analyzed_functions` | 2,380 | 2,398 (+18) | 2,399 | **+1** |
| `helper_edges` | 14,763 | 14,942 (+179) | 14,946 | **+4** |
| `decision_sites` | 12,705 | 12,931 (+226) | 12,943 | **+12** |
| `semantic_macros` | 1,466 | 1,469 (+3) | 1,469 | **0** |

The drift is not the interesting part. **Two mechanisms had to both be true for it to stay invisible**,
and they are a textbook instance of `CLAIM_VERIFICATION.md` §2 — *what class of defect does this check
still permit?*

1. `scripts/check_production_genericity_flow.sh` is a **gate-tier doctrine that runs on every commit**,
   and it *prints* all four numbers:
   `production-genericity-flow: … 2399 functions; 14946 helper edges; 12943 decision sites; … 1469 semantic macros`.
   It compares none of them. A census that reports is not a check: it permits every drift, and it
   reported the true value while the pin beside it said something else.
2. The only comparison lives in a `cargo test`, which the repository's CI policy runs **before a push,
   not per commit**. That policy is deliberate and correct for throughput — but it means the one
   comparing oracle for this census fires at push cadence while the drift accrues at commit cadence.

The last time the pins were set is `0d4860ba` (`WIRE-BASED-100.8b`). The **29 commits since** have
been able to move them without any local signal.

## Non-Goals

- Do not weaken or re-scope the CI policy. Running the full suite per commit is the throughput cost
  that policy exists to avoid, and a 90-second graph analysis is precisely the kind of check it was
  written to keep out of the per-commit path.
- Do not delete the pins. A drifting census is not an argument for having no census; the numbers are
  the genericity boundary's own denominator and a silent change in them is exactly what
  `PRODUCTION-GENERICITY` exists to catch.
- Do not audit which historical commit contributed which unit of the drift. The pins are a whole-tree
  derivation, the commits that moved them are ordinary feature work, and attributing 226 decision
  sites across 29 commits buys nothing a re-pin does not.

## Acceptance Criteria

- The four pins equal the measured current derivation, and `cargo test` is green.
- A change to any of the four is detectable **at commit cadence**, not only before a push — or the
  decision not to make it detectable is recorded with the cost that was weighed.
- The distinction between a census that prints and a check that compares is stated where the next
  author will meet it, not only here.
- `scripts/check_doctrines.sh` green; no ceiling, milestone, or contract widened.

## Task Tree

- ID: `PRODUCTION-GRAPH-CENSUS-PIN` · Status: `active` (`2026-09-12`) · Children: `.0`-`.2`

- ID: `PRODUCTION-GRAPH-CENSUS-PIN.0` · Status: `done` (`2026-09-12`) · Goal: **re-pin at measured
  truth, and say what the drift was.** The four pins are set to the values the current tree derives,
  with the pre-existing / this-slice split stated above so no earlier drift is absorbed silently. The
  test's doc comment now carries why the pins could move at all.
  Verification: `cargo test -p specforge-production-graph --lib` → `4 passed; 0 failed` (89.03s);
  the derivation is compared against `scripts/check_production_genericity_flow.sh`'s own printed line,
  which is an independent execution path to the same analyzer.
  Commit: `PROSE-NAME-CELL-DECLARATION.2 / PRODUCTION-GRAPH-CENSUS-PIN.0`

- ID: `PRODUCTION-GRAPH-CENSUS-PIN.1` · Status: `done` (`2026-09-12`) · Goal: **make the printing gate compare.**
  `check_production_genericity_flow.sh` already derives all four numbers on every commit; the missing
  half is a declaration to compare them against. Put the expected census in a tracked contract under
  `doctrine/` — where every other doctrine's expectations already live — and have the flow check fail
  closed on a mismatch, naming the field and both values. The analysis is already being paid for, so
  this adds a comparison, not a run.
  **The open question is answered and the design is chosen** (`2026-09-12`, four-for-four measurement
  above): neither an exact pin nor a band, but the repository's own `aggregate_change` idiom — the same
  one `doctrine/live_document_size/surfaces.jsonl` already uses for the book's byte total, another
  derived number that legitimately moves on ordinary work. A contract under `doctrine/` carries
  `baseline` + `delta` + `owner` + `rationale`; the flow check compares `baseline + delta` against what
  it already derives and fails closed naming the field and both values. The per-commit edit is not
  removed — it is converted from *re-pinning a literal in a test that cannot see it at commit cadence*
  into *attributing a census change to the leaf that caused it*, which is what the other doctrines
  already require. **`tools/production-genericity-graph`'s test then reads the same contract instead of
  carrying its own four literals**, which is the actual root cause: the pin and the deriver were two
  places that could disagree, and they did, for 29 commits.
  Prerequisite: `.0`. Verification: observed RED against a perturbed count in EACH of the four fields,
  and against a contract whose `baseline + delta` is right but whose `owner` is absent; green on the
  true tree; the Rust test carries no census literal afterwards.
  **Shipped.** `doctrine/production_genericity/flow_census.json` + `--check-census` in the flow gate +
  `census::disagreements`; the test's eighteen literals are gone and it reads the same contract.
  Commit: `PRODUCTION-GRAPH-CENSUS-PIN.1`

- ID: `PRODUCTION-GRAPH-CENSUS-PIN.2` · Status: `done` (`2026-09-12`) · Goal: **find the other censuses that print
  without comparing.** This one was found by accident, by a slice that happened to run `cargo test`.
  The general shape — a doctrine check that derives a number, reports it, and compares nothing —
  is mechanical to search for: every registered check in `scripts/check_doctrines.sh` that emits a
  numeral in its success line, adjudicated by hand against whether anything fails when that numeral
  changes. Report the population; open leaves only for the ones where a silent change would matter.
  Prerequisite: `.1` (its design decides what "comparing" should look like here).
  Verification: the census is the deliverable; a count with no adjudication is not.
  **Answered, and the population is not where the leaf expected to find it.** The registry's checks do
  compare what they measure about the REPOSITORY. What none of them compares is what they report about
  THEMSELVES: 12 of the 16 that print a self-test coverage count cannot detect a change in it. Result
  below; the fix is `.3`. Producer: `python3 scripts/measure_self_test_coverage_reports.py`.
  Commit: `PRODUCTION-GRAPH-CENSUS-PIN.2`

- ID: `PRODUCTION-GRAPH-CENSUS-PIN.3` · Status: `pending` (opened `2026-09-12` by `.2`) · Goal: **give
  the twelve unguarded self-test reports a denominator something compares.** The remedy is already
  written and already in the repository — but **`.2a` corrects which shape it is**. `$passed/$total` is
  NOT it: four checks use that form with `$total` incremented in the same case loop, so deleting a case
  drops both and the ratio stays `N/N`. The remedy is a LITERAL expected total, declared beside the
  suite, which is what the only three genuinely guarded checks do (`13`, `10`, `22`). No contract file.
  **Do the two remaining literal-print checks first and expect to find a wrong number.** Those print a literal
  (`"self-test 15/15 passed."`) that is derived from nothing, so it cannot be checked against the suite
  and is the only class where the published figure may already be false. A hand count of
  `check_persisted_artifact_paths.pl` does not obviously reproduce its 15 — 12 entries in `@cases` plus
  4 standalone assertions — but `.2` deliberately did not settle it, because *"nobody can tell"* is the
  finding and guessing the true number would paper over it. Settle each by making the script count.
  Prerequisite: `.2`. Verification: observed RED per class — delete one self-test case from one check of
  each class and confirm the check now fails; and state, per decorative check, what the number actually
  was.

## `.1` — result (`2026-09-12`)

### One contract, read by the gate and by the test

The defect `.0` recorded was not that the numbers drifted. It was that **the pin and the deriver were
two places that could disagree** — the gate derived all eighteen counts every commit and printed them,
while the only comparison sat in a `cargo test` that runs before a push. Keeping literals in the test
and adding a second declaration elsewhere would have reproduced exactly that shape. So there is one
file, and both read it:

```text
doctrine/production_genericity/flow_census.json
    ├── scripts/check_production_genericity_flow.sh   (gate-tier, every commit, --check-census)
    └── tests::current_repository_flow_is_complete_local_and_deterministic
```

The test keeps what only it can prove — that the derivation is deterministic across two runs and emits
no absolute path — and has no census literal left.

### Boundaries and sizes are different numbers and get different declarations

This is the substance of the design, and it is what `.1`'s open question was actually asking.

| | fields | declaration | why |
| --- | ---: | --- | --- |
| **boundary** | 14 | exact pin | which types are sources, which roots are rules, which regions are trusted. Measured **unchanged across all six** slices that moved the volume counts. A change here is a real change to the product's authority structure. |
| **volume** | 4 | `baseline` + `delta` + `owner` + `rationale` | functions, helper edges, decision sites, macros are SIZES. **Six consecutive slices moved them**, each having to edit a literal to land. |

A declared band was rejected on the argument, not on taste: the census only ever grows, so any band
either admits a jump worth noticing or has to be re-based on the same cadence an exact pin would. The
`aggregate_change` shape is the repository's own answer to the identical problem for the book's byte
total, and it makes the per-commit edit informative rather than ceremonial — it records **who** moved
the census and **why**, which is precisely what was missing across the 29 silent commits.

### Observed RED, one probe per failure mode

```text
boundary.pinned.rule_roots            RED  'rule_roots' is 120, contract declares 121
volume.baseline.analyzed_functions    RED  'analyzed_functions' is 2409, contract declares 2410
volume.delta.helper_edges             RED  'helper_edges' is 14988, contract declares 14989
volume.aggregate_change.owner = ""    RED  owner names no leaf; a census change must be attributed
```

The delta probe is the one that matters for the design: it proves `baseline + delta` is the compared
total, so a slice cannot land by editing the baseline and leaving its own change unattributed. Four
unit controls additionally pin the comparison itself — a disagreement names the field and both values,
a census field the contract does not declare is a breach, and a declared field the census stopped
reporting is a breach rather than a silent pass. That last pair is what stops the contract quietly
ceasing to cover the surface it claims, which is the same failure mode in a different dress.

### The cost, stated

The per-commit edit is not removed. A slice that adds production code still edits one file — but it
edits a doctrine contract that something compares, under an owner and a rationale, instead of four
literals in a test that could not see them until push.

## Acceptance Checklist (enforced) — `.1`

- [x] **REPRODUCE / MEASURE** — six consecutive slices moved the volume counts and none moved a
  boundary count: `ACTOR-NOUN-RELATION-DECLARATION.1`, `INVARIANT-SHAPE-ADMISSION.1`,
  `PROSE-NAME-CELL-DECLARATION.2`, `INVARIANT-SHAPE-ADMISSION.3` (+6/+32/+29/+2),
  `INVARIANT-SHAPE-ADMISSION.5` (+2/+4/+4/0), `EXTRACTION-QUALITY-GAUGE.3i` (0/+1/0/0). That 6-for-6
  split is the measurement `.1` was required to take before choosing a design.
- [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/check_production_genericity_flow.sh` ran
  `--flow`, which prints `InformationFlowReport::summary()` and compares nothing;
  `tools/production-genericity-graph/src/lib.rs` held the only comparison, as eighteen
  `assert_eq!` literals in a test the repository's CI policy runs before a push.
- [x] **ADDRESSED (verified)** — the contract, `--check-census`, `census::disagreements`, and the test
  rewritten to read the contract. Observed RED on all four probes above; the flow gate green on the
  true tree; `cargo test -p specforge-production-graph` 8 passed (4 pre-existing + 4 new controls).
- [x] **NO REGRESSION** — `cargo test` green; `cargo fmt --check` and
  `cargo clippy --all-targets -D warnings` green (two findings from the new test helpers fixed rather
  than allowed); `scripts/check_doctrines.sh` green. No corpus artifact is touched: this leaf changes
  no production rule, and `tools/` is outside the analyzed production surface, so the census it
  declares is unmoved by the code that declares it.
- [x] **GENERICITY (ADR 0006)** — a JSON contract and a map comparison; no document, protocol, vendor,
  or name list. The census field set is derived from the report struct's own fields rather than
  restated, so a new field cannot escape the contract silently.
- [x] **LOCKSTEP** — `DOCTRINE_ENFORCEMENT.md` §3 gains "A census that reports is not a check", with
  the measured instance and the boundary-vs-size rule, which is this tree's acceptance criterion that
  the distinction be stated where the next author will meet it. Not a book change: this is doctrine
  machinery, not user-visible product behaviour.

## `.2` — result (`2026-09-12`)

### The shape is there, one level up from where the leaf looked

`.2` was opened to sweep the registry for `.0`'s shape — a check that derives a number, reports it, and
compares nothing. The checks turn out to compare what they measure about the **repository** rather well.
What none of them compares is what they report about **themselves**.

Nearly every registered check ends with a reassuring self-test line. `python3
scripts/measure_self_test_coverage_reports.py` classifies how that number is produced, over 16 checks:

| class | checks | how the number is produced | delete a self-test case → |
| --- | ---: | --- | --- |
| `derived` | **4** | `$passed/$total`, computed independently | they differ, the check **fails** |
| `TAUTOLOGY` | **4** | `$passed/$passed` | prints `59/59`; exit 0 |
| `DECORATIVE` | **5** | a literal in the message: `"self-test 15/15 passed."` | prints `15/15` forever |
| `BARE` | **3** | a running counter, no declared total | the counter moves; nothing reads it |

**12 of 16 cannot detect a change in their own coverage.**

```text
TAUTOLOGY   check_active_task_evidence.pl  check_fact_card_catalog.pl
            check_proof_seal_currency.sh   check_task_tree_archive.pl
DECORATIVE  check_chain_currency.sh        check_corpus_kb_currentness.pl
            check_persisted_artifact_paths.pl
            check_source_pdf_registry_currentness.pl
            check_validation_snapshot_currentness.pl
BARE        test_derived_state_authorities.pl  test_derived_state_contracts.pl
            test_live_document_size.pl
```

### The limit, stated, because the headline would otherwise overclaim

**A self-test that FAILS still fails every one of these checks.** They `die` on a failing case; the
verdict is real. What is unguarded is **coverage** — a case silently removed, or one never added, moves
the reported number and nothing compares it. That is a weaker defect than `.0`'s and it is still the
same shape: the number is in the output, and nothing reads it.

### Why `DECORATIVE` is the worst of the three

A tautology at least moves with the suite. A hardcoded literal does not move at all:

```perl
if ($mode eq 'self-test') {
    run_self_test();
    print "persisted-artifact-paths: self-test 15/15 passed.\n";
}
```

`15` is a string. It is not derived from the suite, cannot be compared against it, and will keep
asserting `15` whatever `run_self_test` contains. A reader meets `15/15` and reads coverage evidence.

**This leaf deliberately does not settle whether that 15 is currently right.** A hand count finds 12
entries in `@cases` plus 4 standalone assertions, which does not obviously reproduce it — but resolving
it by hand would replace an underivable number with a number derived by me, and *"nobody can tell"* is
the finding. `.3` settles each by making the script count.

### What the sweep did NOT find, stated so the absence is evidence

Every check that publishes a number about the repository — `278 canonical cards`, `301 facts / 2475
question keys`, `158 task trees`, `946 files / 57 surfaces`, `22 tracked PDFs`, `156/156 KG fixtures` —
**is compared**, by a derive-and-diff, a contract, or a gold score. `.0`'s defect was genuinely the
exception among repository-facing numbers rather than the rule, which is worth knowing: the sweep's
value is that it bounds the problem as well as locating it.

## Acceptance Checklist (enforced) — `.2`

- [x] **REPRODUCE / MEASURE** — `python3 scripts/measure_self_test_coverage_reports.py`: 16 checks
  report a self-test coverage count; 4 `derived`, 4 `TAUTOLOGY`, 5 `DECORATIVE`, 3 `BARE`. Each class is
  named by how the number is produced, not by whether it looks plausible.
- [x] **ROOT CAUSE (WHY + WHERE)** — the report line itself. `check_fact_card_catalog.pl` prints
  `"$passed/$passed …"`, so the ratio is structurally incapable of being anything but `N/N`;
  `check_persisted_artifact_paths.pl` prints the literal `"self-test 15/15 passed."` after calling
  `run_self_test()`, so the figure is independent of the suite entirely.
- [x] **ADDRESSED (verified)** — the census IS this leaf's deliverable, per its own stated verification
  ("a count with no adjudication is not"). All 16 are classified individually and the four `derived`
  checks are named as the working precedent the fix should copy, so `.3` has a remedy that already
  exists in the repository rather than a new mechanism.
- [x] **NO REGRESSION** — read-only: the census reads the check scripts as text and does not execute
  them. No production rule, contract, ceiling, or corpus artifact is touched.
- [x] **GENERICITY (ADR 0006)** — classification is by the shape of the report expression, not by script
  name; a new check is classified by the same four patterns without being listed anywhere.
- [x] **LOCKSTEP** — `DOCTRINE_ENFORCEMENT.md` §3 already carries "A census that reports is not a check"
  from `.1`; this leaf adds the self-coverage corollary to it. Not a book change: doctrine machinery,
  not user-visible product behaviour.

## `.2a` — `.2`'s census was wrong, and wrong in the direction that matters (`2026-09-12`)

### What `.2` published, and what is actually true

`.2` classified 16 checks, reported **12 unguarded**, and named `$passed/$total` in four checks as
"the remedy that already exists in the repository" for `.3` to copy. Re-derived by hand, both halves
are wrong:

| | `.2` published | actually |
| --- | ---: | ---: |
| coverage-guarded | 4 | **3** |
| unguarded | 12 | **13** |
| the named "working precedent" | `$passed/$total`, 4 checks | **not a coverage guard at all** |

**`$passed/$total` does not guard coverage.** Read in full:

```perl
my ($passed, $total) = (0, 0);
for my $case (@cases) {
    $total++;
    ...
}
```

`$total` counts the same loop as `$passed`. Delete a case and BOTH drop; the ratio stays `N/N`. That
form detects a **failing** case — a real property, and a different, weaker one than the question this
tree asks. All four checks `.2` held up as the precedent are in this shape.

Conversely, three checks `.2` filed as `DECORATIVE` — because their print line carries a literal — do
compare a counted variable against a literal elsewhere in the script, and are the only genuinely
coverage-guarded members of the population:

```text
check_source_pdf_registry_currentness.pl   die "... expected 13 cases, passed $passed" if $passed != 13;
check_validation_snapshot_currentness.pl  die "... expected 10 cases, passed $passed" if $passed != 10;
check_chain_currency.sh                   if [ "$passed" -ne 22 ]; then ...
```

### The corrected rule, which is simpler than `.2`'s

**A self-test count is coverage-guarded only when the expected total is declared INDEPENDENTLY of the
suite — a literal.** Anything co-derived from the same loop cannot see a case leave it. By that single
test, hand-adjudicated over the 16:

| | checks | shape |
| --- | ---: | --- |
| **guarded** | **3** | `$passed` compared to a literal (13, 10, 22) |
| unguarded — co-derived total | 4 | `$passed/$total`, `$total++` inside the case loop |
| unguarded — tautology | 4 | `$passed/$passed` |
| unguarded — literal print, no comparison | 2 | `"self-test 15/15 passed."` |
| unguarded — bare counter | 3 | `all $test_number checks pass` |

### How `.2` got it wrong, because the mechanism is this tree's own subject

`.2`'s producer classified by the **shape of the print line** and inferred the guard from how the
number was formatted there. A script can print a literal and compare elsewhere; three did. The repair
attempt — "find a comparison involving a counter anywhere in the file" — then produced false positives
(`$lines != $parts`, `$count != 1`) and a false negative on a check already verified by hand.

**The property is not regex-decidable, and `.2` published a number a regex had guessed.** That is
exactly the defect this tree exists to name. `.2`'s own acceptance criterion said so — *"the census is
the deliverable; a count with no adjudication is not"* — and its producer was doing the adjudicating.

`scripts/measure_self_test_coverage_reports.py` is rewritten to **emit evidence and refuse to
classify**: per check, its self-test report lines and every comparison involving a counted variable.
The verdict lives here. Broadening it to stop under-capturing also raised the population from 16 to
**31 report lines**, so the 16 `.2` adjudicated were not the whole surface either; enumerating the rest
is `.3`'s.

### One worked remediation, because a correction should show the corrected remedy

`check_corpus_kb_currentness.pl` now counts in its own `assert_valid`/`assert_invalid` helpers and
compares against a declared literal beside the suite. **Observed RED**: deleting one `assert_invalid`
case yields `self-test ran 14 assertions, declaration expects 15` and **exit 2**, where before it
printed `15/15` and exited 0. Its 15 was correct all along; what it lacked was anything that would
notice if it stopped being.

## Acceptance Checklist (enforced) — `.2a`

- [x] **REPRODUCE / MEASURE** — hand-adjudication of all 16 checks `.2` censused, by the single test
  "does deleting one self-test case make this check fail?". 3 guarded, 13 not; `.2` said 4 and 12.
- [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/measure_self_test_coverage_reports.py` as `.2` shipped it
  classified on the print line's format. `check_source_pdf_registry_currentness.pl` prints `13/13` and
  compares `$passed != 13` at line 538; the producer never looked there. Its replacement regex then
  mis-fired in both directions, which is the evidence that the property is not regex-decidable.
- [x] **ADDRESSED (verified)** — the producer emits evidence and no verdict; this leaf carries the
  adjudication; `.3`'s brief is corrected from `$passed/$total` to a declared literal; and
  `check_corpus_kb_currentness.pl` is remediated as the worked example, with an observed RED at exit 2.
- [x] **NO REGRESSION** — `perl -c` clean on the edited script; its self-test prints `15/15` from a
  derived count; `scripts/check_doctrines.sh` green. No production rule, contract, ceiling or corpus
  artifact touched.
- [x] **GENERICITY (ADR 0006)** — the corrected rule is a property of the code shape (is the expected
  total independent of the suite?), not a list of script names.
- [x] **LOCKSTEP** — `DOCTRINE_ENFORCEMENT.md` §3's self-coverage corollary carried `.2`'s wrong table
  and its wrong remedy; both corrected there. Not a book change: doctrine machinery.

## Current Frontier

Ordered; PNT selects the first eligible leaf.

1. `PRODUCTION-GRAPH-CENSUS-PIN.3` — give the twelve unguarded self-test reports a denominator
   something compares. The remedy already exists in four sibling checks; start with the decorative
   five, where the published figure may already be false.

## Decisions

- `2026-09-12` — **re-pinned in the slice that found it, rather than in a separate commit.** The
  finding surfaced with `PROSE-NAME-CELL-DECLARATION.2` already uncommitted in the tree, and this
  repository's pivot rule forbids switching trees across a dirty tree. Landing both leaves in one
  commit with the per-pin attribution stated is the honest version of that constraint; stashing
  half-finished production work to slip a repair underneath it is not.
- `2026-09-12` — **the drift is not audited commit-by-commit.** See Non-Goals. What matters is the
  mechanism, and the mechanism is fully established without knowing which of the 29 commits added
  which decision site.
- `2026-09-12` — **the fix is not "run the full suite per commit".** That would answer this defect by
  reversing a throughput decision the owner made deliberately. The cheaper answer is that the check
  which *already runs and already derives these numbers* should compare them.
- `2026-09-12` — **the design is `aggregate_change`, not a band, and the reason is precedent not
  taste.** A declared band has to answer "how wide", and any answer is arbitrary: the census only ever
  grows, so a band either admits a jump worth noticing or has to be re-based on the same cadence an
  exact pin would. This repository already solved the identical problem for the book's byte total —
  a derived number that moves on ordinary work — with `baseline` + `delta` + `owner` + `rationale`, and
  that idiom makes the per-commit edit informative rather than ceremonial: it records *who* moved the
  census and *why*, which is exactly what was missing across the 29 silent commits.

## Open Questions

- ~~How often do the four numbers legitimately move?~~ **Answered `2026-09-12`: every slice that adds a
  production predicate, four for four.** `ACTOR-NOUN-RELATION-DECLARATION.1`,
  `INVARIANT-SHAPE-ADMISSION.1`, `PROSE-NAME-CELL-DECLARATION.2` and now
  `INVARIANT-SHAPE-ADMISSION.3` (+6 functions / +32 helper edges / +29 decision sites / +2 semantic
  macros) each had to edit the pins to land — and then `INVARIANT-SHAPE-ADMISSION.5` made it **five for
  five**, which is the sharpest case of all: that slice added ONE condition to an existing gate,
  removed two fabricated records and introduced no new behaviour, and still had to edit four literals
  in a test that cannot observe them until push. So an exact pin in a push-cadence test is the worst of
  both: it taxes every ordinary commit AND detects nothing until push. `.1` now has its measurement
  and a design to build; see the decision below.

## Blockers

None.

## Verification Log

- `2026-09-12` — `.2a`. All 16 checks `.2` censused re-adjudicated BY HAND against the single question
  "does deleting one self-test case make this check fail?", by reading each script's count guard rather
  than its print line. 3 guarded, 13 not, where `.2` published 4 and 12 — and the four it named as the
  working precedent are the shape that does not work. Two automated classifiers were written and both
  were wrong (the first on print format, the second with false positives `$lines != $parts` /
  `$count != 1` and a false negative on a hand-verified case), which is the evidence recorded for the
  "not regex-decidable" claim rather than an assertion of it.
  **Observed RED** on the one remediation: `check_corpus_kb_currentness.pl` with one `assert_invalid`
  deleted exits **2** with `self-test ran 14 assertions, declaration expects 15`; before the change the
  same deletion printed `15/15` and exited 0.
  `perl -c` clean; `scripts/check_doctrines.sh` green. Read-only otherwise; no corpus rebuild.

- `2026-09-12` — `.2`. Read-only; no artifact written, executed or mutated — the census reads the check
  scripts as text. All 16 self-test report lines classified individually by how the number is produced:
  4 `derived`, 4 `TAUTOLOGY`, 5 `DECORATIVE`, 3 `BARE`. The classification was verified by reading the
  printing statement in each case rather than by pattern-matching a name; an early grep using an ERE
  backreference silently matched nothing and would have reported 0 tautologies, so the producer uses a
  real regex engine and the counts were cross-checked against the printed lines from a full
  `check_doctrines.sh` run. The question of whether any decorative literal is currently WRONG is
  deliberately left open for `.3` — see the result section.

- `2026-09-12` — `.1`. Observed RED on four probes, one per failure mode: a perturbed boundary pin, a
  perturbed volume baseline, a perturbed volume **delta** (which proves `baseline + delta` is the
  compared total, so a slice cannot land by editing the baseline and leaving its change unattributed),
  and an emptied `owner` (refused — a census change must name a leaf). Green on the true tree.
  `cargo test -p specforge-production-graph` 8 passed, of which 4 are new controls on the comparison
  itself: a disagreement names the field and both values; a census field the contract does not declare
  is a breach; a declared field the census stopped reporting is a breach rather than a silent pass.
  `cargo fmt --check` and `cargo clippy --all-targets -D warnings` green; `scripts/check_doctrines.sh`
  green. No corpus rebuild: this leaf changes no production rule.

- `2026-09-12` — `.0`. Baseline established by measurement, not inference: `git stash push` of the
  slice's only modified source file, then `scripts/check_production_genericity_flow.sh` at
  `4bb6c1c8` → `2398 functions; 14942 helper edges; 12931 decision sites; 1469 semantic macros`;
  `git stash pop`, re-run → `2399 / 14946 / 12943 / 1469`. `cargo test -p specforge-production-graph
  --lib` green at the re-pinned values. Full `cargo test` green (`472` / `168` / `1423` / `4`).

## Commit Log

- `.0` — `PROSE-NAME-CELL-DECLARATION.2 / PRODUCTION-GRAPH-CENSUS-PIN.0`.

## Changelog

- `2026-09-12` — `.2a` corrects `.2`. The census was 3 guarded / 13 unguarded, not 4 / 12, and the
  `$passed/$total` form `.2` named as the remedy is not a coverage guard at all — `$total` counts the
  same loop, so a deleted case drops both. The producer no longer classifies: it emits the report lines
  and candidate guards as evidence and the adjudication lives in the leaf, because two regexes tried
  and both were wrong. One worked remediation ships with the correction.

- `2026-09-12` — `.2` closed, and the population is not where the leaf expected it. The registry's
  repository-facing numbers are compared; what is unguarded is what each check reports about its own
  coverage — 12 of 16 self-test counts cannot detect a case being removed. `.3` opened for the fix,
  which already exists in four sibling checks as `$passed/$total`.

- `2026-09-12` — `.1` closed. The census moved out of eighteen test literals into
  `doctrine/production_genericity/flow_census.json`, which the gate-tier flow check now COMPARES
  against on every commit and the test reads instead of restating. Boundary counts (14) are pinned
  exactly; volume counts (4) are declared as `baseline + delta + owner + rationale`, because six
  consecutive slices moved them and none moved a boundary count. `DOCTRINE_ENFORCEMENT.md` §3 states
  the distinction so `.2`'s sweep has a definition to match against.

- `2026-09-12` — tree created. A `cargo test` pin of four repository-wide census numbers was found
  stale by +18 / +179 / +226 / +3 while the gate-tier doctrine that derives the same numbers on every
  commit printed the true values and compared none of them.
