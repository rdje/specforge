# COMMIT.md

Last updated: 2026-05-05

## Purpose
Define the exact commit workflow for this project so a new AI instance can apply it consistently without re-reading chat history.
The workflow exists to preserve full operational continuity across session loss, crashes, and handoffs.

## When To Run
Run this workflow after each completed task/activity.

## Batch Runs
When the user explicitly authorizes an automatic batch of `N` tasks, slices, or lanes:
- still run this full commit workflow after every completed task, slice, or lane in the batch
- do not defer task-scoped commits until the end of the batch
- push only after the full defined `N`-item batch is complete, unless the user explicitly gives a different push instruction or the branch is around `30` local commits ahead and the current active-run policy allows checkpoint pushes
- if the user-defined batch ends early because of a blocker, do not push automatically unless the user explicitly approves that early-batch push
- record the active batch size, completed count, and push deferral status in `MEMORY.md` during the batch so crash recovery is explicit

When the user authorizes PNT (`Pick the Next Task`) mode:
- treat PNT as an open-ended batch with no fixed `BWFSC`
- still run the full commit workflow after every completed slice
- continue selecting bounded roadmap-aligned slices until no task, slice, or lane remains to pick from or the user explicitly pauses/stops
- treat the post-commit report as a continuity checkpoint, not a pause; after reporting, immediately pick the next slice and roll with it
- push around every `30` local commits since the last push, unless the user gives a different push instruction
- record the active PNT completed count, current ahead count when known, and push threshold in `MEMORY.md`

## Files Involved
- `README.md` (tracked)
  - Single project entrypoint and navigation hub.
  - Must be updated whenever objective, canonical flow, key paths, standard commands, or doc map changes.
- `LIVE_ACHIEVEMENT_STATUS.md` (tracked)
  - Authoritative live progress tracker.
  - Must use only `Done`, `Mostly Done`, `In Progress`, and `Not Started`.
  - Must be reviewed and updated before every commit whenever actual closure or remaining scope changes.
  - The current live-status snapshot must be summarized in every user-facing completion message produced by the commit workflow.
  - If any live-status row changes, the completion message must also state how the task affected that snapshot.
- `git_message_brief.txt` (must remain untracked)
  - Short, concise commit message file.
  - Used with `git commit -F git_message_brief.txt`.
  - Must be cleared to 0 bytes after commit.
- `CHANGES.md` (tracked)
  - Changelog-style summary of completed work and validation.
- `DEVELOPMENT_NOTES.md` (tracked)
  - Detailed technical notes: root cause, implementation, validation.
- `MEMORY.md` (tracked)
  - Live continuity file for resume/handoff.
  - Must be updated after completed tasks and at meaningful mid-task checkpoints during long-running work so crash recovery remains possible before commit.
  - Must capture the actionable summary of key user instructions, current repo state, files in flight, recent decisions, and exact next steps.
  - Must contain the latest committed Git hash and corresponding brief commit message known at the time of update, or explicitly state that no commit exists yet.
  - Must stay compact, precise, and operational rather than turning into a verbatim session transcript.
- `RUST_CODEBASE_ANALYSIS.md` (tracked)
  - Live Rust architecture/state assessment.
  - Must remain a deep and current analysis of the Rust codebase, even when the Rust codebase is still only partially built.
  - Must be reviewed and updated whenever a task materially changes Rust architecture, major subsystem boundaries, public integration seams, or the current high-level risk/steering picture.
- `questions_keep_untracked.txt` (must remain untracked)
  - User backlog/questions for future UG work.
- Markdown path policy
  - File paths mentioned in tracked `.md` files and mdBook sources must be repo-root-relative when they point inside the repository, never checkout-specific absolute paths.
  - Host-local or private external input paths must not be recorded as absolute filesystem paths; use non-host-specific placeholders such as `<local AXI protocol PDF>` or a repo-local generated artifact path when available.

## Required Commit Workflow (Exact Order)
1. Ensure task is complete and tested.
3. Update tracked docs as needed (`CHANGES.md`, `DEVELOPMENT_NOTES.md`, `MEMORY.md`, `RUST_CODEBASE_ANALYSIS.md`, `README.md`, `LIVE_ACHIEVEMENT_STATUS.md`, others touched by task).
   - Treat markdown synchronization as systematic, not optional:
     - always review the tracked continuity/workflow markdown surface before commit,
     - always update every relevant tracked `.md` file touched by the task or affected by its workflow/policy/command/documentation impact,
     - do not leave a relevant markdown file stale just because code/tests already passed.
   - Treat current-state drift as a blocker before commit:
     - explicitly compare the codebase and just-completed work against `README.md`, `ROADMAP.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `MEMORY.md`, `RUST_CODEBASE_ANALYSIS.md`, and the mdBook source,
     - reconcile active/in-progress/not-started terminology, batch state, push state, command surfaces, adapter targets, and validation claims before staging,
     - do not proceed to commit while a current-facing live doc or live-book page still describes a stale project state.
   - For long-running or multi-step tasks:
     - do not wait until the very end to refresh live docs,
     - update `MEMORY.md` and any other impacted live documents at meaningful checkpoints,
     - preserve enough in-progress context that a new session can resume cleanly after a crash or interruption.
   - `MEMORY.md` review/update is mandatory before each commit, and must include:
     - the latest committed Git hash known at the time of update,
     - the corresponding brief commit message,
     - or an explicit `no commit yet` state for repositories without a committed `HEAD`.
   - Because the hash of a just-created commit is only known after the commit exists:
     - the pre-commit `MEMORY.md` update should reflect the latest already-existing committed baseline,
     - the post-commit user-facing message must report the new commit ID and message,
     - and the next live-document refresh checkpoint must update `MEMORY.md` so it reflects that newly-created committed baseline.
   - `RUST_CODEBASE_ANALYSIS.md` review/update is mandatory before each commit when the task materially changes:
     - Rust architecture,
     - major subsystem boundaries,
     - public integration surfaces,
     - or the current high-level implementation/risk assessment of the Rust codebase.
   - `LIVE_ACHIEVEMENT_STATUS.md` review/update is mandatory before each commit whenever the task changes:
     - what is `Done`,
     - what is `Mostly Done`,
     - what is `In Progress`,
     - what is `Not Started`,
     - or what the next most important remaining gap is.
   - In every commit-workflow completion message:
     - display the current live-status snapshot from `LIVE_ACHIEVEMENT_STATUS.md`,
     - make it clear whether the task changed that snapshot or left it unchanged.
   - When any live-status row changes:
     - update `LIVE_ACHIEVEMENT_STATUS.md` before commit,
     - summarize the changed status snapshot in the user-facing completion message,
     - explicitly state the effect of the completed task on the tracker.
   - When no live-status row changes:
     - still display the current live-status snapshot in the completion message,
     - explicitly say that the tracker is unchanged rather than implying a status update happened.
   - In every commit-workflow completion message, also display:
     - the commit ID,
     - the exact commit message,
     - the full list of tracked files included in that commit.
   - `README.md` sync is required when:
     - project objective/scope changes,
     - canonical generation flow changes,
     - key project paths or standard commands change,
     - markdown documentation map/ramp-up order changes.
   - While reviewing/updating markdown docs:
     - convert any repo-internal absolute checkout path to a relative path before commit,
     - replace host-local absolute external input paths with non-host-specific placeholders when a repo-relative path does not exist,
     - do not leave checkout-specific absolute filesystem paths in tracked `.md` files or mdBook sources.
4. Write concise commit message to `git_message_brief.txt`.
5. Stage only intended tracked files (`git add <files>`).
6. Commit with:
   - `git commit -F git_message_brief.txt`
7. Clear message file:
   - `: > git_message_brief.txt`
8. Confirm post-conditions:
   - `git ls-files --error-unmatch git_message_brief.txt` must fail (untracked).
   - `wc -c git_message_brief.txt` must be `0`.
   - `git status --short` must show expected state only.
9. In the user-facing completion message after commit, report:
   - the commit ID,
   - the exact commit message,
   - the list of tracked files included in the commit,
   - the current live-status snapshot,
   - and whether that snapshot changed or stayed unchanged.

## Pre-Commit Safety Rules
- Do not add `git_message_brief.txt` to git.
- Do not track `questions_keep_untracked.txt`.
- Do not use destructive git commands unless explicitly requested.

## Command Template
```bash
# 1) write concise commit message
cat > git_message_brief.txt <<'EOF'
<concise title>

- <brief bullet 1>
- <brief bullet 2>
EOF

# 2) stage intended files only
git add <tracked-file-1> <tracked-file-2> ...

# 3) commit
git commit -F git_message_brief.txt

# 4) clear message file
: > git_message_brief.txt

# 5) verify
wc -c git_message_brief.txt
git ls-files --error-unmatch git_message_brief.txt >/dev/null 2>&1; echo $?
git status --short
```
