# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey.
> History lives in `git log` + `CHANGES.md`; work state lives in the task-trees
> (`docs/tasks/`); durable facts/decisions live in `docs/decisions/`. Do **not** append
> session narration here — overwrite the "Current state" block instead.
> (The prior ever-growing MEMORY.md is preserved in git history before commit `6d668eb2`.)

## How to resume (any AI, any harness)
- Read `MEMORY_ARCHITECTURE.md` (the memory system) and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`);
  follow `COMMIT.md` after every slice (unit id in the commit subject).
- Non-negotiable doctrine: see `docs/decisions/0003-task-tree-and-commit-doctrine.md`
  (no code change without an owning task-tree first; signoff quality; zero
  ROADMAP↔code↔mdBook drift; push ~every 200 commits; artifact cleanup ≥ every 24h).
- Durable cross-cutting facts: `docs/decisions/` (e.g. Docling CPU device 0001,
  LLM/VLM provider default 0002).

## Current state (OVERWRITE this block each update — do not append)
- latest_commit: **`EXTRACTION-QUALITY-GAUGE.3f`** = alphabetic-value word-boundary gate on the deterministic value binder (CODE; lib 1628; full CI GREEN; kg-bench 156/156); **~140 ahead** of `origin/main` (push at ~200). Session `2026-06-15` (fresh) = owner sent the standard bootstrap; resumed PNT. Owner-chosen direction (AskUserQuestion) = **"EQG constraint precision"** (autonomous, one narrow structural gate at a time, wire docs held at 1.000, rest as honest residuals).
- **THIS-SESSION (PNT, active):** `EXTRACTION-QUALITY-GAUGE.3f` DONE — the value binder `extract_discovered_state_value_from_text` matched an enum value behind `shall be`/`must be` with a PLAIN substring, fabricating NVMe `SANICAP must_be_value NO` off "shall be **no**n-zero". Fix = `lead_binds_value` requires a whole-word boundary after the value ONLY when it ends in a LETTER (numeric stays lenient → "shall be 0h" still binds; blanket rule REJECTED by measurement). Universal grammar, ADR-0006, DRY root-cause. **Verified:** NVMe 20→19 (only SANICAP NO), wire Pattern builds byte-identical (0 records altered, proven), wire gold ×4 1.000 (constraints/relations/temporal/SWD-deriv) on a non-destructive temp-evidence-root eval, `seed_nvme_registers` 28/29 unchanged. KM `value-binder-alphabetic-whole-word`; book `pipeline/evidenceir.md` `.3f`.
- **RAM-SAFETY (owner, non-negotiable): 90→93% used RAM = reboot DANGER ZONE; monitor every background job + swap, autonomously kill at ≥85% used; serialize Docling vs the 14B model (`ollama stop` before ingests); QUALITY INVARIANT under restriction — only SPEED flexes** (`feedback_ram_ceiling_monitor` / `project_big_pdf_memory_bounded_ingest`). Push ~every 200 commits or on ask. (This slice was RAM-safe: deterministic, no 14B.)
- **DOCTRINE (non-negotiable): RUNTIME code is PDF-AGNOSTIC (ADR-0006)** — concrete chip names ONLY in gold keys + `#[cfg(test)]`, never runtime; NO deny/filter LISTS (prefer structural gates); goal = #1 structured+multimodal+bounded-LLM intent mining w/ honest residuals; honest RESIDUAL over fabrication. **`scripts/run_ci.sh` before declaring any CODE slice green**; `cargo test` does NOT rebuild the bin — `cargo build --release` before live measurement. **WRITE-PATH GOTCHA:** commands persist by the artifact's `artifact_layout` (canonical path), NOT the input path; for non-destructive measurement build into a TEMP evidence-root (`evidence --dry-run` strips a 4-line header before the JSON) and eval there. `subs/fsmgen` pinned `c0b7eaa7`.
- **EQG `.3` precision program now spans `.3a`–`.3f`.** Candidate-future residuals on the persisted NVMe constraint surface (each a SEPARATE measured slice, RAM-safe, no denylist): (a) wrong-prose-subject class (`NVM`/`FFFF`/`LBA` lifted from descriptive command sentences — these passed the catalog gate because the doc has no real wire catalog); (b) spurious-digit-value class on descriptive field cells (`HMDLLA 1`, `MPS 0` — value not grounded in a "shall be V"). Both need a structural discriminator; measure per-item before building.
- next_action: **PNT CONTINUING (owner direction = EQG constraint precision).** Next pickable RAM-safe slice = a structural gate for one of the two NVMe residual classes above (measure-first per the `.3f`/`.3e` pattern: scan persisted `signal_constraints`, confirm clean+wire-safe, build, verify wire gold 1.000 + kg-bench 156/156 + run_ci). The one big NON-EQG lever (`CORPUS-PATTERN-REUSE.4`, offline miner) stays RAM-heavy 14B + gated → ask before launching. 24 promoted canonical docs still await the `R7-VALIDATION` review/approval workflow. **NOT pushed** (~140 ahead, < 200).
- in_flight_uncommitted: **none after this commit** — `EXTRACTION-QUALITY-GAUGE.3f` is a complete slice (code + tests + tree + book + CHANGES/DEV-NOTES/LIVE-STATUS + KM card + this pointer). Untracked (stays): `.claude/settings.json`. blockers: **none**. **Repo HANDOFF-READY.**
