---
id: chomp-is-a-no-op-under-a-callers-slurp
title: A caller's local $/ turns chomp into a no-op, and the guard it feeds then disables a whole check silently
answers:
  - "why did my Perl checker stop reporting after I moved a call inside another function"
  - "why does git_top return a path that does not equal the repository root"
  - "why is chomp not removing the newline in this Perl script"
  - "what does local $/ do to a callee in the SpecForge gate scripts"
  - "why did registry discovery return zero registries while every doctrine reported PASS"
  - "how do I read a file line by line inside a function whose caller slurps"
date: 2026-09-16
status: current
tags: [perl, doctrine, live-document-size, gates, debugging, silent-failure]
evidence: scripts/check_live_document_size.pl (git_top, bounded_registry_paths, validate_bounded_registry_population); docs/tasks/LIVE-DOCUMENT-PRESSURE-HEADROOM.md (.22f)
reverify: perl scripts/check_live_document_size.pl 2>&1 | grep -c "^live-document-size: warning: registry '"
---

`chomp` does not remove a newline. It removes whatever `$/` currently holds — and when `$/` is
`undef`, which is how Perl slurps, **it removes nothing and returns 0**. `$/` is a global, and
`local $/` in a caller is still in effect inside every function that caller invokes. So a helper that
reads one line and chomps it is correct when called at the top level and quietly wrong when called
from inside a slurping function.

**What that cost here, measured.** `scripts/check_live_document_size.pl` has
`git_top()`, which reads `git rev-parse --show-toplevel`, chomps it, and returns it; several callers
guard on `return if git_top() ne $root`. `validate_ceiling_history` slurps the previous registry with
`local $/;`. Moving the bounded-registry observer inside that function put it under that `local`, so
`git_top()` returned the repository path **with a trailing newline**, the guard compared unequal, and
registry discovery returned **zero registries**. The pressure band over ten registries and a newly
added authority check both went silent — while every doctrine still reported PASS, because nothing was
wrong, there was simply nothing being looked at.

**Why it is worth a card rather than a comment.** The failure mode is silence, and the check that
disappeared was itself a check for silence. There was no diagnostic, no exception and no test failure;
the only symptom was that a probe which had failed a minute earlier now passed. A green gate that has
stopped looking is indistinguishable from a green gate that looked and found nothing, unless something
independent asserts the looking happened.

**The rule.** Fix it at the source, not at the call site. A helper that parses command output or reads
lines sets its **own** separator (`local $/ = "\n";`) and strips terminators with an explicit
substitution (`s/\r?\n\z//`) rather than `chomp`, so no caller can change what it means. Reserve
`chomp` for code that owns `$/` itself. The same applies to any `while (my $line = <$fh>)` loop in a
helper: under a caller's slurp it reads the entire file as one record, and a per-line count silently
becomes 1 or 0.

**How to notice it.** When a checker goes quiet after a refactor, assert the population it walked, not
just its exit code: print or test the number of things discovered. A count of zero is a bug; an exit
code of zero is not evidence.

Links: [[live-surface-edit-bookkeeping-chain]].
