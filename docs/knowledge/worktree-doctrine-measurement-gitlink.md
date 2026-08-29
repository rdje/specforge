---
id: worktree-doctrine-measurement-gitlink
title: A per-revision doctrine measurement in a git worktree fails closed until the fsmgen gitlink is populated
answers:
  - "how do I measure a doctrine checker across many revisions"
  - "how do I re-derive a published count per revision instead of at two endpoints"
  - "why does check_current_claim_census.pl exit 1 in a fresh git worktree"
  - "why does the fsmgen_correspondence_projection derived-state contract fail in a worktree"
  - "why is subs/fsmgen empty in a git worktree"
  - "how do I populate the fsmgen gitlink in a detached measurement worktree"
date: 2026-08-29
status: current
tags: [measurement, worktree, doctrine, claim-verification, continuity, fsmgen]
evidence: docs/tasks/CLAIM-VERIFICATION-ADOPTION.md (.6a); scripts/check_current_claim_census.pl; scripts/check_derived_state_contracts.pl; scripts/check_fsmgen_feedback_protocol.pl
reverify: git worktree add --detach .project-data/tmp/gitlink-probe/wt HEAD && (cd .project-data/tmp/gitlink-probe/wt && perl scripts/check_current_claim_census.pl --report; echo "empty-gitlink exit=$?") && git -C subs/fsmgen archive "$(git rev-parse HEAD:subs/fsmgen)" | tar -x -C .project-data/tmp/gitlink-probe/wt/subs/fsmgen && (cd .project-data/tmp/gitlink-probe/wt && perl scripts/check_current_claim_census.pl --report; echo "populated exit=$?") && git worktree remove --force .project-data/tmp/gitlink-probe/wt
---

Answering "when did this published number actually change?" honestly needs each revision measured with **that
revision's own checker**, not the current checker replayed over old content and not two endpoints joined by an
assumption. The repository-local way to do that is a detached `git worktree` under `.project-data/tmp/`, stepping
it through `git rev-list --reverse <base>^..HEAD` and running the checker from inside the worktree at each stop.

`git worktree add` does **not** populate submodules. `subs/fsmgen` is a gitlink, so a fresh worktree gets an
empty `subs/fsmgen/` directory. That single gap fails the measurement closed rather than loudly:

- `scripts/check_fsmgen_feedback_protocol.pl` reports five records whose evidence
  `subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md` is missing and exits 1;
- `scripts/check_derived_state_contracts.pl` therefore fails its `fsmgen_correspondence_projection` verifier;
- every checker that *executes* that verifier as declared evidence — `scripts/check_current_claim_census.pl`
  among them — collects the nonzero exit as a violation and exits **before printing its report**.

So a naive rig records "no result" at every revision and looks like the checker never worked, when the only
defect is the empty gitlink. Populate it from the parent checkout, using the commit each revision pins:

```sh
rm -rf "$WT/subs/fsmgen" && mkdir -p "$WT/subs/fsmgen"
git -C subs/fsmgen archive "$(git rev-parse "$rev:subs/fsmgen")" | tar -x -C "$WT/subs/fsmgen"
```

`git archive | tar -x` is used rather than a copy because it writes the pinned tree without the submodule's own
`.git` pointer file, which would otherwise resolve back into the parent repository's module store.

Two further rig properties are worth carrying, both learned by getting them wrong:

- **Exactly one runner at a time.** Two loops sharing one worktree check out different commits under each
  other, and the symptom is a scatter of "exact region is stale" hash mismatches that look like real drift.
- **Do not background the loop with `nohup … &` inside a harness background call.** The outer shell exits
  immediately, the harness reports success, and the loop is killed after its first write; run the script as the
  background command itself.

A revision-by-revision trace is what separates a *stale constant* (wrong once, correctable) from a *counter
ordinary work moves* (withdraw it and route the reader to the producer). `CLAIM-VERIFICATION-ADOPTION.6a` used
this rig to show that four counts `TOOLBOX.md` published as "confirmed unchanged" were already false in the very
commit that published them. See [[current-claim-census-freeze]].
