# 0006 — Doctrine-enforcement architecture: every doctrine becomes a mechanically-gated check

- Date: 2026-06-22
- Status: accepted
- Tags: process, doctrine, enforcement, continuity

## Context

SpecForge's written doctrines (no code change without an owning task-tree leaf; signoff quality;
zero ROADMAP↔code↔mdBook drift; ADR-0006 genericity; honest-residual-over-fabrication) are
*discoverable* but, for most of them, not *mechanically enforced* — they relied on the agent reading
prose and complying. Two existing doctrines were already mechanized end-to-end (memory-architecture
via `scripts/check_memory_architecture.sh`; the knowledge-map via
`knowledge-map/scripts/check_knowledge_map.sh`), each wired through `.githooks/pre-commit` (E3) and
`scripts/run_ci.sh` (E4) — but as a hand-rolled, ad-hoc check stack with no shared registry, no
per-doctrine report, and no single seam to add the next doctrine to.

The owner directed (`2026-06-22`): "adopt this doctrine enforcement system" → `DOCTRINE_ENFORCEMENT.md`,
the portable, project-agnostic standard (sibling of `MEMORY_ARCHITECTURE.md`) that turns every rule
into `rule + a deterministic check that exits nonzero on any breach`, run from one registry/driver and
gated by the same E1→E4 defense-in-depth.

## Decision

Adopt the **Doctrine-Enforcement architecture** as the **4th portable architecture** in this repo
(alongside task-trees, memory-architecture, knowledge-map). Concretely:

1. Land the portable standard `DOCTRINE_ENFORCEMENT.md` at the repo root (§0–§9/§11 are the adopted
   standard; §10 is SpecForge's live registry).
2. Add the registry+driver `scripts/check_doctrines.sh`: it runs every registered `check_*.sh`,
   prints a per-doctrine PASS/FAIL report, exits nonzero on any breach, and **meta-checks** that
   every registered enforcer exists + is executable (no dangling promise).
3. **Register the existing structural checks** (`MEMORY-ARCH`, `KNOWLEDGE-MAP`) into the driver and
   route `.githooks/pre-commit` (E3) and `scripts/run_ci.sh` (E4) through it — unify, do not
   duplicate; each check stays the single source of truth for its own invariant.
4. Mechanize SpecForge's **flagship doctrine** — *no code change without an owning task-tree leaf
   first* (decision `0003`) — as a native EVIDENCE check `scripts/check_task_acceptance.sh`: a staged
   Rust code change (`crates/**/*.rs`, `crates/**/test_data/**`) must have a staged owning
   `docs/tasks/*.md` leaf whose acceptance checklist carries ROOT CAUSE + ADDRESSED + NO REGRESSION,
   ticked and backed by SpecForge tool signatures (`kg-bench 156/156`, `WIRE-BASED-100 1.000`,
   `run_ci`/`cargo test`, byte-identical, `validate`/`adapt` findings). The checklist template lives
   in `TOOLBOX.md`.
5. Keep the **deterministic oracles** (`kg-bench` 156/156, WIRE-BASED-100 golds, `cargo
   fmt`/`clippy`/`test`/`doc`, byte-identical evidence/`.isf`) on the `run_ci.sh` / CI path — they are
   heavy and are the strongest leg; pre-commit runs only the cheap structural + evidence checks
   (`DOCTRINE_ENFORCEMENT.md` §4.7 / §6.1 leg-3).

## Consequences

- A non-compliant change is blocked locally (E3) and un-mergeable in CI (E4) by **one** driver, and a
  new doctrine is added by writing one `check_*.sh` + one registry line + one §10 row.
- The "no code change without task-tree ownership" doctrine is now mechanically enforced, not just
  prose — a Rust code change with no owning leaf / no evidence cannot commit.
- Honest limit (stated, not hidden): local hooks are bypassable (`--no-verify`); SpecForge's hosted CI
  is currently manual-only (`workflow_dispatch`), so the un-fakeable oracle re-run happens at the next
  `run_ci.sh` / dispatched-CI run, not instantly. Re-enabling an auto CI doctrine-gate is the true "no
  matter what" backstop.
- A dedicated structural ADR-0006 check (flagging NEW hardcoded chip/signal-name literals in
  extraction code) is a tracked future candidate, deferred until it ships without false-blocking
  legitimate test fixtures; ADR-0006 stays enforced meanwhile by the `kg-bench` negative fixtures and
  the per-slice golds.

## Links

- Standard: `DOCTRINE_ENFORCEMENT.md`; driver: `scripts/check_doctrines.sh`; evidence check:
  `scripts/check_task_acceptance.sh`; checklist template + toolbox: `TOOLBOX.md`.
- Owning task-tree: `docs/tasks/DOCTRINE-ENFORCEMENT-ADOPT.md`.
- Sibling standards: `MEMORY_ARCHITECTURE.md` §9 (same E1→E4 model), decision `0003` (the doctrine
  this mechanizes), `knowledge-map/KNOWLEDGE_MAP_ARCHITECTURE.md`.
