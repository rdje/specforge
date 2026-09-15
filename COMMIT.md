# COMMIT.md

Last updated: 2026-08-08

## Purpose
Define the exact commit workflow for this project so a new AI instance can apply it consistently without re-reading chat history.
The workflow exists to preserve full operational continuity across session loss, crashes, and handoffs.

## Task-Tree Commit Rule
- If a completed activity belongs to a task-tree leaf, update the owning `docs/tasks/*.md` file (node status, verification log, commit log, frontier) before or alongside the source commit.
- Identify the leaf ID (e.g. `PROVENANCE-HARDENING.4`) in the commit subject or first body line.
- Commit every completed leaf before selecting another leaf from any task tree.

## Published-Claim Declaration

Every commit body and pull-request description must contain exactly one of:

```text
Published-claims: none
Published-claims: <claim-id>[, <claim-id>...]
```

Use `none` only when the slice publishes or changes no current actionable assertion as defined by
`CLAIM_VERIFICATION.md`. A docs-only change is not automatically exempt: changing a current score, count,
capacity, compatibility statement, or status requires claim IDs. For each ID, the owning task/claim record must
name re-derivation, a dimensionally different falsification oracle, tracked durability/staleness, and any missing
leg. `scripts/check_claim_verification.pl` now resolves the declaration against the bounded registry and rejects
missing, duplicate, unknown, superseded, untracked, or stale evidence.

## When To Run
Run this workflow after each completed task/activity.

## Batch Runs
When the user explicitly authorizes an automatic batch of `N` tasks, slices, or lanes:
- still run this full commit workflow after every completed task, slice, or lane in the batch
- do not defer task-scoped commits until the end of the batch
- push only after the full defined `N`-item batch is complete, unless the user explicitly gives a different push instruction or the branch is around `400` local commits ahead and the current active-run policy allows checkpoint pushes
- if the user-defined batch ends early because of a blocker, do not push automatically unless the user explicitly approves that early-batch push
- record batch progress in `MEMORY.md` when it changes the concrete resume action or leaves work in flight;
  do not edit the pointer merely to acknowledge a commit

When the user authorizes PNT (`Pick the Next Task`) mode:
- treat PNT as an open-ended batch with no fixed `BWFSC`
- still run the full commit workflow after every completed slice
- continue selecting bounded roadmap-aligned slices until no task, slice, or lane remains to pick from or the user explicitly pauses/stops
- treat the post-commit report as a continuity checkpoint, not a pause; after reporting, immediately pick the next slice and roll with it
- push around every `400` local commits since the last push, unless the user gives a different push instruction (30 → 200 per user directive `2026-06-04`; 200 → 400 per user directive `2026-09-13`, and it is fixed there)
- keep the active leaf and next concrete action in `MEMORY.md`; obtain ahead/behind and HEAD from Git
  when needed instead of copying them into a hand-maintained shadow field

## Documentation routing contract

Review documentation by impact, then update only the canonical surfaces whose truth changed. A path
must never be edited merely to prove that it was reviewed.

- `README.md` is the bounded landing page. Update it only when purpose, first-use path, top-level
  architecture, or canonical navigation changes.
- `MEMORY.md` is the bounded resume pointer. Update it when the active unit, next action, in-flight
  uncommitted state, or blocker changes. Query `git rev-parse HEAD` and Git status on resume; do not
  copy HEAD into a `latest_commit` shadow field.
- The owning `docs/tasks/*.md` file is mandatory for every completed slice and owns its frontier,
  decisions, verification, and commit subject/reference.
- `LIVE_ACHIEVEMENT_STATUS.md` changes only when its current product-status snapshot changes.
- `CHANGES.md` changes only when its still-live ledger contract calls for an entry; its replacement
  lifecycle and rollover are owned by `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4`.
- `DEVELOPMENT_NOTES.md` changes only for unique, durable technical rationale not already owned by a
  task-tree, decision, fact card, book chapter, or source comment; its replacement lifecycle is owned
  by the same adoption tree.
- `RUST_CODEBASE_ANALYSIS.md` changes only for a material Rust architecture, subsystem-boundary,
  public-integration, or current-risk change.
- The mdBook changes whenever user-visible behavior or the public understanding of SpecForge changes.
  The `BOOK-METHOD-DOC` closing-leaf rule remains mandatory.
- `ROADMAP.md` changes only when program direction, milestones, or roadmap-level status changes.
- A durable structural/causal fact gets a Knowledge Map fact card; generated `KNOWLEDGE_MAP.md` and
  `docs/knowledge-map/questions-*.md` must never be hand-edited.
- **Editing any governed file re-pins the `line_range_sha256` regions below the edit, in all THREE
  claim registries.** Do not do it by hand and do not write a throwaway: run
  `python3 scripts/repin_claim_regions.py --check`, then `--apply`
  (`CLAIM-VERIFICATION-ADOPTION.12`). It resolves each region by CONTENT and **refuses** when more
  than one location matches — a blank-line region matches every blank line in the file, and a re-pin
  that lands on the wrong one is invisible because the digest it was moved to match is the digest it
  now has. A refusal is yours to decide; the tool writes nothing until it is resolved.

All repository-internal Markdown paths are repository-root-relative. Host-local inputs use portable
placeholders. `git_message_brief.txt` and `questions_keep_untracked.txt` remain untracked; the former
must be cleared to zero bytes after each commit.

## Rolling-ledger rollover

When a root rolling ledger reaches its declared health-target rollover, stop ordinary appends and create a
task-owned, repository-relative JSONL plan that pins the committed opening blob and exact whole-record cut. Run
the plan without `--apply-rollover` first; this is a read-only identity, chronology, capacity, and warning-safe
dry run. Apply only the exact green plan:

```bash
perl scripts/check_rolling_ledger_protocol.pl --rollover-plan docs/research/<plan>.jsonl
perl scripts/check_rolling_ledger_protocol.pl --rollover-plan docs/research/<plan>.jsonl --apply-rollover
```

The writer stages on the repository volume, installs sealed segments then manifests/indexes, writes live roots
last, validates the complete result, and restores exact preflight bytes on failure. Run the generic live-size
gate afterward; the focused checker also requires its ceilings/milestones to agree with the generic surface
authority. Never edit a segment, widen a control, or hand-cut a root to bypass this transaction.

## Required Commit Workflow (Exact Order)

1. Finish and verify the bounded slice.
2. Update the owning task-tree leaf (status, evidence, frontier, and commit subject/reference).
3. Perform an impact-based alignment review across roadmap, codebase, current status, architecture
   analysis, and mdBook. Update each canonical surface whose truth changed; leave unrelated surfaces
   byte-identical. Any known current-facing contradiction is a blocker.
4. If resumable state changed, overwrite `MEMORY.md` with one active unit, concise current state, one
   concrete next action, in-flight work, and blockers. Do not narrate chronology or mirror HEAD.
5. Apply the `BOOK-METHOD-DOC` close rule when this commit closes a tree. For long-running work, commit
   completed verified units promptly and record genuinely in-flight state before a handoff.
6. Write the concise message to `git_message_brief.txt` and stage only intended files.
7. Add exactly one `Published-claims:` declaration to the commit body per `CLAIM_VERIFICATION.md`.
8. Run `scripts/check_doctrines.sh` and all risk-proportionate focused/broader gates.
9. Commit with:
   - `git commit -F git_message_brief.txt`
10. Clear the message file:
   - `: > git_message_brief.txt`
11. Confirm post-conditions:
   - `git ls-files --error-unmatch git_message_brief.txt` must fail (untracked).
   - `wc -c git_message_brief.txt` must be `0`.
   - `git status --short` must show expected state only.
12. In a user-facing completion or handoff message, report:
   - the commit ID,
   - the exact commit message,
   - the list of tracked files included in the commit,
   - and any current product-status change caused by the slice.

No-op documentation review is recorded by leaving the canonical file unchanged, not by appending
ceremonial prose. Git history plus the task leaf prove the review path.

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

Published-claims: none
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
