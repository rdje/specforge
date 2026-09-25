# Expected behavior

Run from any directory that is not the root of an FSMGen git checkout — a release tarball, a `git archive`
export, or a copy nested inside another repository — `bin/fsmgen-issue-bundle` must not attribute the
bundle to a foreign revision. Either of these satisfies the report:

1. It records the FSMGen revision it can prove (for example from the capability manifest's producer
   identity when that is known), and otherwise writes `unknown` for the head in both `env.txt` and
   `observed/command-logs/fsmgen-git-head.txt`, as the capability manifest already does
   (`producer.git_commit: unknown`); or
2. it asks for the revision explicitly (for example a `--fsmgen-revision` option) when it is not running
   at the top level of an FSMGen git checkout.

`commands.sh` then prints `NOT REPRODUCED`.
