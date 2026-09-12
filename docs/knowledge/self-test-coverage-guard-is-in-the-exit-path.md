---
id: self-test-coverage-guard-is-in-the-exit-path
title: A self-test coverage guard lives in the exit code, not the printed ratio — reading the print line got the census wrong three times
answers:
  - "does a check notice when one of its own self-test cases is deleted"
  - "is $passed/$total a coverage guard"
  - "why did PRODUCTION-GRAPH-CENSUS-PIN.2's census have to be corrected twice"
  - "how do I tell whether a doctrine check guards its own self-test coverage"
  - "what does self-test 15/15 passed actually prove"
  - "why is a hardcoded self-test count worse than a tautological one"
  - "how many registry checks guard their own coverage"
  - "what is the behavioural oracle for a coverage guard"
date: 2026-09-12
status: current
tags: [doctrine-enforcement, self-test, claim-verification, production-graph-census-pin, method]
evidence: scripts/measure_self_test_coverage_reports.py; docs/tasks/PRODUCTION-GRAPH-CENSUS-PIN.md (.2, .2a, .3); DOCTRINE_ENFORCEMENT.md section 3; scripts/check_proof_seal_currency.sh (total=19, line 352); scripts/check_book_quantitative_claims.pl
reverify: "pick any check with a self-test mode, delete one self-test case, run it, and read the EXIT CODE. Exit 0 means the check cannot see its own coverage shrink. Before PRODUCTION-GRAPH-CENSUS-PIN.3, twelve of sixteen exited 0; after it, none do."
---

**Established `2026-09-12` (`PRODUCTION-GRAPH-CENSUS-PIN.2` / `.2a` / `.3`).** The question "does this
check notice when one of its own self-test cases is deleted?" **cannot be answered by reading the line
it prints.** Three consecutive attempts to answer it textually were wrong.

## The only reliable test is behavioural

```bash
# delete one self-test case, then:
perl scripts/check_<name>.pl --self-test >/dev/null; echo $?
```

`0` means the check is blind to its own coverage shrinking. Everything else is inference.

## What the printed ratio does not tell you

| printed | actually |
| --- | --- |
| `self-test 13/13 passed.` — a hardcoded literal | **guarded**: `$passed != 13` forty lines earlier |
| `self-test $total/$total passed.` — looks tautological | **guarded**: `total=19` is a declared literal compared with `[ "$passed" -eq "$total" ]` |
| `self-test $passed/$total` — looks independently derived | **NOT guarded** when `$total++` sits in the same case loop |
| `self-test $passed/$passed` | not guarded, and honest about it |

**`$passed/$total` is the trap.** It reads as two independent values and is usually not:

```perl
my ($passed, $total) = (0, 0);
for my $case (@cases) { $total++; ... }
```

Delete a case and both drop; the ratio stays `N/N` and the exit code stays `0`. Measured on
`check_book_quantitative_claims.pl`: `19/19` became `18/18`, exit 0. That form detects a **failing**
case — a real and different property.

## The three failures, because the method matters more than the table

1. `.2` classified by the shape of the print line. It filed three checks that print a literal as
   unguarded and named the four `$passed/$total` checks as "the remedy that already exists".
   Its counts (4 guarded / 12 not) happened to be right; **its membership was completely wrong.**
2. `.2a` corrected the membership and got the count wrong (3/13), by reading
   `check_proof_seal_currency.sh`'s `$total/$total` as tautological without checking that `$total` is a
   literal `19`.
3. The producer written to replace the classifier then missed the shell increment
   `passed=$((passed + 1))` and silently dropped a real guard from its evidence.

Each time a textual proxy returned an answer that looked clean. This is `CLAIM_VERIFICATION.md` §2's
own subject — a check whose agreement with the thing it checks carries no information — and it landed
inside the leaf whose whole purpose is to name that defect.

`scripts/measure_self_test_coverage_reports.py` therefore **emits evidence and renders no verdict**:
the report lines and every comparison involving a counted variable, for a human to adjudicate.

## The remedy, and what it is not

Declare the expected count as a **literal beside the suite**, and `die` on a mismatch. Not a contract
file — the declaration belongs where the cases are. Not `$passed/$total`.

```perl
my $expected_cases = 19;
die "...: self-test ran $total cases, declaration expects $expected_cases ...\n"
    if $total != $expected_cases;
```

After `.3`, all sixteen registry checks that report a self-test count fail closed when a case is
removed. The per-commit cost is one literal edited by whoever adds or removes a case — the same
attribution trade `[[production-graph-census-is-declared-not-pinned]]` makes for the production census.
