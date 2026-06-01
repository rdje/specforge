# MEMORY-ARCHITECTURE-DOC: author a portable, harness-agnostic durable-memory standard

## Metadata

- Tree ID: `MEMORY-ARCHITECTURE-DOC`
- Status: `active`
- Roadmap lane: `R0` (process / continuity infrastructure)
- Created: `2026-06-01`
- Owner: repo-local workflow
- Parent context: user asked for a git-tracked, NOT-project-specific document, usable
  directly in their other projects, that demotes an ever-growing `MEMORY.md` and
  implements the four durability properties + task-trees so any information that
  enters survives across harness, AI model, session loss, and crashes — applicable to
  any project already using task-trees.

## Discovery / rationale

`~/.<harness>/…` memory is harness-specific (lost on a switch to Codex/Cursor/etc.)
and untracked (lost on machine loss). An append-only `MEMORY.md` grows unbounded,
conflates current-state/facts/history, and is hard for a *different* model to parse.
The durable, portable answer is: memory must be **in-repo + git-tracked + structured +
self-describing + reachable from a tool-neutral entrypoint** — properties the
task-tree system already has and these other stores lack.

## Goal

Produce a single, thorough, accurate, **project-agnostic** Markdown standard
(`MEMORY_ARCHITECTURE.md` at repo root) that a user can copy into any task-tree repo
to: (a) demote `MEMORY.md` to a bounded resume pointer; (b) define the memory layers
(pointer / task-trees / decision records / git audit trail); (c) specify the write
path (how info enters + is guaranteed to survive) and read path (deterministic
resume in any harness); (d) define the tool-neutral bootstrap entrypoint; (e) give
templates + an adoption checklist + a durability matrix.

## Non-Goals

- NOT SpecForge-specific *in the standard itself* (`MEMORY_ARCHITECTURE.md` must port
  cleanly — no SpecForge nouns).
- NOT re-specifying the task-tree system itself (assumed present; referenced as
  layer B).

## Scope expansion (user directives `2026-06-01`)

The user added: (1) "shall be extremely difficult for a harness NOT to follow what is
recommended" → the standard MUST include mechanical **enforcement** (not just advice);
(2) "you will implement everything that this document will say we do or recommend" →
after authoring, **apply the standard to SpecForge** (run its own §11 adoption
checklist here). So this tree now both authors the portable standard AND implements it
in-repo.

## Acceptance Criteria

- `MEMORY_ARCHITECTURE.md` exists at repo root, project-agnostic, self-contained:
  problem → 4 properties → 4 layers (lifecycle table) → write path → read path →
  resume-pointer spec + template → decision-record spec + template → tool-neutral
  bootstrap + `AGENTS.md` template → git backbone → maintenance/compaction → adoption
  checklist → anti-patterns → durability matrix. README doc-map pointer added. Commit
  via COMMIT.md workflow. Tree CLOSED.

## Task Tree

- ID: `MEMORY-ARCHITECTURE-DOC`
  Status: `active`
  Children: `.1`–`.5`

- ID: `MEMORY-ARCHITECTURE-DOC.1`
  Status: `done`
  Goal: own (this file) + author the portable `MEMORY_ARCHITECTURE.md` standard
    (problem → 4 properties → 4 layers → write/read paths → resume-pointer spec →
    bootstrap → git backbone → **enforcement (E1–E4) + agnostic reproduce-anywhere
    kit** → maintenance → adoption checklist → anti-patterns → durability matrix +
    templates) + README doc-map pointer. Commit.
  Acceptance: standard exists, project-agnostic, enforcement + agnostic kit included.
  Verification: passed (`2026-06-01`) — `MEMORY_ARCHITECTURE.md` authored at repo root,
    fully project-agnostic (no SpecForge nouns); §§0–14 incl. the 4 durability
    properties, 4 layers (lifecycle table), write/read paths, resume-pointer + AGENTS
    templates, §9 enforcement E1–E4 (ubiquitous bootstrap, self-check script, git
    hooks, CI gate) with reference implementations, §9.1 agnostic reproduce-anywhere
    kit (the user's "other projects follow your footsteps" requirement), durability
    matrix. README doc-map points at it + reframes `MEMORY.md` as the resume pointer.
  Commit: `see Commit Log`

- ID: `MEMORY-ARCHITECTURE-DOC.2`
  Status: `done`
  Goal: implement layer C in-repo — `docs/decisions/` + `INDEX.md`; seed dated
    decision records by migrating durable, not-already-tracked facts (e.g. Docling
    device/MPS operational fact; the production LLM/VLM provider default; a pointer
    record for the task-tree + commit doctrine). Link from related task-trees.
  Acceptance: `docs/decisions/` + index + ≥3 seeded records; index lists them.
  Verification: passed (`2026-06-01`) — `docs/decisions/INDEX.md` + 3 ADR records:
    `0001-docling-device-cpu` (migrated the Docling MPS→CPU operational fact + the
    don't-delete-source_ir-before-reingest process note), `0002-llm-vlm-provider-default`
    (Ollama+qwen2.5vl:7b production default; never claim missing; qwen3-vl:8b candidate),
    `0003-task-tree-and-commit-doctrine` (pointer record to the non-negotiable doctrine
    + COMMIT.md). Index table lists all three; each links its related task-trees.

- ID: `MEMORY-ARCHITECTURE-DOC.3`
  Status: `pending`
  Goal: demote this repo's `MEMORY.md` to the §6 bounded resume pointer (≤ cap;
    overwrite-only; current-state block). History stays in git; durable facts now in
    layer C. (Done before enforcement so the cap check passes.)
  Acceptance: `MEMORY.md` ≤ cap and matches the resume-pointer template.

- ID: `MEMORY-ARCHITECTURE-DOC.4`
  Status: `pending`
  Goal: install the agnostic enforcement kit in-repo — `scripts/check_memory_architecture.sh`
    (+x), `.githooks/pre-commit` + `.githooks/commit-msg` (+x), `git config
    core.hooksPath .githooks`, bootstrap pointer files (`AGENTS.md`, `CLAUDE.md`,
    `.cursorrules`, `.github/copilot-instructions.md`), and wire the self-check into
    `scripts/run_ci.sh` (E4). README references the system.
  Acceptance: self-check passes; hooks active; CI runs the check; bootstrap files point
    at the standard.

- ID: `MEMORY-ARCHITECTURE-DOC.5`
  Status: `pending`
  Goal: verify end-to-end — `check_memory_architecture.sh` green, full `run_ci.sh`
    green (incl. the new check), bootstrap/README/live-docs synced; close the tree.
  Acceptance: all green; tree CLOSED.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `MEMORY-ARCHITECTURE-DOC.1` | `pending` | author the standard (w/ enforcement + agnostic kit) + README pointer |
| 2 | `MEMORY-ARCHITECTURE-DOC.2` | `pending` | layer C: docs/decisions + seed migrated facts |
| 3 | `MEMORY-ARCHITECTURE-DOC.3` | `pending` | demote MEMORY.md to the bounded resume pointer (before enforcement, so the cap passes) |
| 4 | `MEMORY-ARCHITECTURE-DOC.4` | `pending` | install enforcement kit (check script + hooks + CI + bootstrap files) |
| 5 | `MEMORY-ARCHITECTURE-DOC.5` | `pending` | verify all-green + close |

## Decisions

- `2026-06-01`: keep the standard strictly project-agnostic + at repo root so it
  copies cleanly into other repos; own the SpecForge authoring activity here.
- `2026-06-01`: do NOT also refactor SpecForge's `MEMORY.md` in this tree — offer it
  as a separate follow-on so the portable standard ships independently reviewable.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-01` | `.1` | portable standard authored (project-agnostic; enforcement E1–E4 + agnostic kit + durability matrix + templates); README doc-map pointer; MEMORY.md reframed as resume pointer | `passed` |
| `2026-06-01` | `.2` | layer C in-repo: `docs/decisions/INDEX.md` + 3 ADR records (docling-cpu, provider-default, doctrine-pointer) migrated from `~/.claude`/MEMORY.md; indexed + cross-linked | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `MEMORY-ARCHITECTURE-DOC.1` | `MEMORY-ARCHITECTURE-DOC.1 — author portable harness-agnostic durable-memory standard (+enforcement +agnostic kit)` | standard + README pointer |
| `MEMORY-ARCHITECTURE-DOC.2` | `MEMORY-ARCHITECTURE-DOC.2 — implement layer C: docs/decisions + seed migrated decision records` | docs/decisions/ + 3 ADRs |

## Changelog

- `2026-06-01`: Created — author a portable harness-agnostic durable-memory standard
  (`MEMORY_ARCHITECTURE.md`).
