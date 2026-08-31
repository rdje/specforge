---
id: project-scratch-location
title: Temporary files go in `.project-data/tmp/` on the repository volume — never in an agent harness's own scratchpad, which is off-volume
answers:
  - "where do I put a temporary file / scratch file / working file in this repository (.project-data/tmp/ — it is on the repository volume, gitignored except .gitkeep, and is the temporary-workspaces row of PROJECT_DATA_LOCALITY.md; create a named subdirectory under it and delete it when the slice ends)"
  - "can I use /tmp or /private/tmp for scratch in SpecForge (NO — PROJECT_DATA_LOCALITY.md forbids defaulting to /private/tmp, /tmp, user-home caches, or any other off-volume location; all project-owned data must sit on the same filesystem volume as the repository)"
  - "my agent harness told me to use a scratchpad directory for all temporary files — should I (only if it resolves onto the repository volume; an interactive harness commonly hands out a path under /private/tmp, which violates the locality standard. Use .project-data/tmp/ instead and delete anything already written off-volume)"
  - "is the off-volume scratchpad hazard mechanically gated (NO, and do not assume it is: the files never enter the repository, so check_project_data_locality has nothing to walk and the SCRATCH-RESIDUE-CONTAINMENT.3 census cannot see them. Retrieval is the only control, which is why this card exists)"
  - "why must SpecForge data stay on the repository volume (the repository root can be moved to another filesystem; persisted paths are repository-root-relative and tools derive absolute paths at runtime from the current root, so an off-volume path silently breaks that portability)"
  - "how do I clean up scratch after a slice"
date: 2026-08-31
status: current
tags: [locality, scratch, doctrine, agent-workflow, portability]
evidence: PROJECT_DATA_LOCALITY.md (temporary workspaces -> .project-data/tmp/); .gitignore (/.project-data/tmp/* with !.gitkeep); docs/tasks/SCRATCH-RESIDUE-CONTAINMENT.md (.5); scripts/project_data_env.sh; .cargo/config.toml
reverify: "grep -n 'temporary workspaces' PROJECT_DATA_LOCALITY.md; grep -n 'project-data/tmp' .gitignore; ls -d .project-data/tmp"
---

# Temporary files go in `.project-data/tmp/`, on the repository volume

`PROJECT_DATA_LOCALITY.md` requires every piece of project-owned data — generated output, build artifacts,
caches, logs, runtime fixtures, and **temporary workspaces** — to live on the same filesystem volume as the
repository, because the repository root is expected to move between filesystems and persisted paths are
repository-root-relative. The registered temporary root is **`.project-data/tmp/`**, which `.gitignore` covers
(`/.project-data/tmp/*` with `!.gitkeep`, so the directory exists in a fresh checkout). Rust code does not
trust ambient `TMPDIR`: `project_data::tempdir()` creates below that root, and `.cargo/config.toml` plus
`scripts/project_data_env.sh` force the same defaults for subprocesses.

## The hazard this card exists for

An interactive agent harness may hand a session **its own scratchpad directory and instruct it to use that for
all temporary files**. That path is commonly under `/private/tmp`, which is off-volume and is named explicitly
in the standard's prohibition. The instruction is not adversarial — it is the harness's ordinary default — but
following it violates the repository's locality standard, and it arrives before the agent has any reason to
open `PROJECT_DATA_LOCALITY.md`.

**No gate catches this.** The files never enter the repository, so the locality checker has nothing to walk and
the residue census `SCRATCH-RESIDUE-CONTAINMENT.3` built cannot see them. Retrieval is the only control, which
is why the fact is a card rather than a check. It was observed live: the `LIVE-DOCUMENT-PRESSURE-HEADROOM.4e`
session wrote three partition-verification files to its harness scratchpad before catching the conflict.

## What to do

Create a named subdirectory per slice, and delete it when the slice ends:

```bash
S=.project-data/tmp/<slice-id>
mkdir -p "$S"
# … work …
rm -rf "$S"
```

If you have already written off-volume, copy what you still need onto the repository volume, delete the exact
off-volume files, and confirm the directory is empty — the copy/verify/use/delete sequence the standard
requires for existing off-volume project data.

Links: [[research-record-size-profile]], [[task-tree-node-forms]].
