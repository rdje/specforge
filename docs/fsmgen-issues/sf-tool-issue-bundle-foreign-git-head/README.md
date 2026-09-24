# FSMGen Bug Report: the issue-bundle helper attributes a bundle to an enclosing repository's commit

Title: sf-tool-issue-bundle-foreign-git-head — `bin/fsmgen-issue-bundle`, run from a non-git copy of FSMGen that sits inside another git repository, records that repository's commit as FSMGen's revision
FSMGen command or public API entrypoint: ./bin/fsmgen-issue-bundle --case ../tiny.isf --issue-id helper-head --no-probes -- --strict --check --json
FSMGen-facing primary artifact path: sources/fsmgen-input/tiny.isf (a strict-valid 9-line actor; its content is irrelevant to the defect)
Failure class: wrong-output
FSMGen commit or capability-manifest producer commit: reproduced on c0d8b668db2527108d1c23c20d184400c51efea6 (SPECFORGE pin) and 67c8b36d0a8080c59daf1449be00be4628b65af9 (upstream HEAD at capture)
SPECFORGE version/commit: see env.txt
Expected behavior: the bundle's env.txt "FSMGen git HEAD:" line and observed/command-logs/fsmgen-git-head.txt name the FSMGen revision the helper actually ran, or say unknown; they never name a commit of a different repository.
Observed behavior: both name the HEAD commit of the unrelated repository that encloses the FSMGen copy (observed/command-logs/recorded-fsmgen-head.txt equals enclosing-repo-head.txt, not fsmgen-actual-head.txt); the helper exits 0 and prints no warning.
First failing command or API call: step 4 of commands.sh (the helper invocation)
Exit status or exception: 0 (observed/command-logs/helper.exit) — the defect is a silently wrong record
Is the attached artifact bundle minimized: yes
Does the minimized/redacted bundle still reproduce: yes

Request kind: **bug report** (tooling). SPECFORGE owner: `FSMGEN-REQUESTS.3`. Local status: see `request.json`.

## Why it matters

`docs/DOWNSTREAM_ISSUE_REPORTING.md` §9 makes a report actionable only when "the bundle identifies the
FSMGen revision or capability manifest producer identity". A bundle captured this way identifies the
wrong revision with full confidence, so a maintainer bisecting or checking out that "revision" is sent to
a commit that does not exist in FSMGen. SPECFORGE hit it while filing `sf-isf-derived-width-parameter` and
`sf-isf-undeclared-rule-operand`: to verify against FSMGen's current HEAD without touching its own pinned
submodule, SPECFORGE exported FSMGen with `git archive` into its own working tree, and both bundles first
recorded SPECFORGE's commit `343bd7fa` as FSMGen's head (corrected by hand before filing; see their
section "Revision note"). The capability manifest in the same bundles correctly says
`producer.git_commit: unknown`, so the helper's two identity records disagree with each other.

## Reproduction

From the root of an FSMGen git checkout, with this directory unpacked there as
`fsmgen-issue-sf-tool-issue-bundle-foreign-git-head/`:

```bash
bash fsmgen-issue-sf-tool-issue-bundle-foreign-git-head/commands.sh
```

The script needs git, perl and tar, and writes only into `REPRO_WORKDIR` (default: a fresh `mktemp -d`)
and this bundle's `observed/`. It:

1. records the checkout's real revision (`observed/command-logs/fsmgen-actual-head.txt`);
2. creates an unrelated git repository with one empty commit (`enclosing-repo-head.txt`);
3. extracts `git archive HEAD` of FSMGen (the files the helper and CLI need) into a subdirectory of it;
4. runs the documented helper from that copy on `tiny.isf`;
5. copies what the bundle recorded (`recorded-fsmgen-head.txt`, `recorded-env-head.txt`) and prints
   `REPRODUCED` when the recorded head is the enclosing repository's commit.

## Observed (both runs)

| FSMGen revision | actual head | enclosing repo head | recorded "FSMGen git HEAD" | verdict |
| --- | --- | --- | --- | --- |
| `c0d8b668` (pin) | `c0d8b668…` | `a50b39e0…` | `a50b39e0…` | REPRODUCED |
| `67c8b36d` (upstream) | `67c8b36d…` | `759d6e6d…` | `759d6e6d…` | REPRODUCED |

Evidence: `observed/pinned-c0d8b668/` and `observed/command-logs/`. The enclosing commit hashes differ
between runs because each run creates a fresh repository.

## Expected

See `expected/expected-notes.md`. In short: record a revision the helper can prove, or `unknown`, and
never a foreign one.

## Notes

- SPECFORGE did not read the helper's implementation (it treats FSMGen as a black box); the diagnosis
  above is inferred from behaviour only: the recorded value always equals the enclosing repository's
  `HEAD`, which is what `git rev-parse HEAD` returns from a subdirectory of that repository.
- No redactions: the scratch repository lives outside the bundle, and the captured files contain commit
  hashes only.
