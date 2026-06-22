---
id: doctrine-enforcement-adoption
title: SpecForge enforces every mechanizable doctrine via one registry/driver (scripts/check_doctrines.sh) gated E1→E4; a Rust code change is blocked unless its task leaf carries an evidence-backed acceptance checklist
answers:
  - "how are doctrines enforced in specforge"
  - "what is scripts/check_doctrines.sh / the doctrine driver"
  - "how do I add a new enforced doctrine / doctrine check"
  - "what blocks a Rust code change from committing in specforge"
  - "where is the acceptance checklist a code change must satisfy"
  - "what is the 4th portable architecture (doctrine enforcement)"
  - "why does the pre-commit hook run check_doctrines.sh"
  - "what does the TASK-ACCEPTANCE check verify / why was my commit blocked"
  - "what debug/diagnostic tools does specforge have (TOOLBOX.md)"
  - "which doctrines are registered (MEMORY-ARCH, KNOWLEDGE-MAP, TASK-ACCEPTANCE)"
  - "how do I waive or range-scope the task-acceptance check"
date: 2026-06-22
tags: [doctrine-enforcement, process, governance, pre-commit, ci, task-tree-doctrine, toolbox, adr-0003, portable-architecture]
evidence: DOCTRINE_ENFORCEMENT.md (the standard; §10 = live registry); scripts/check_doctrines.sh (registry+driver, DOCTRINES array); scripts/check_task_acceptance.sh (evidence check); scripts/check_memory_architecture.sh + knowledge-map/scripts/check_knowledge_map.sh (the two structural checks); TOOLBOX.md (tool catalog + acceptance-checklist template); .githooks/pre-commit + scripts/run_ci.sh (E3/E4 wiring); docs/decisions/0006-doctrine-enforcement-architecture.md; docs/tasks/DOCTRINE-ENFORCEMENT-ADOPT.md; docs/book/src/reference/doctrine-enforcement.md
reverify: "bash scripts/check_doctrines.sh  # prints a per-doctrine report, exits 0 with 'ALL 3 enforced doctrines PASS' (MEMORY-ARCH, KNOWLEDGE-MAP, TASK-ACCEPTANCE). grep -n 'DOCTRINES=' -A4 scripts/check_doctrines.sh shows the registry. A staged crates/**/*.rs change with no staged owning docs/tasks/*.md leaf carrying ROOT-CAUSE/ADDRESSED/NO-REGRESSION (ticked+evidence-backed) makes scripts/check_task_acceptance.sh exit 1."
---

**SpecForge adopted the Doctrine-Enforcement architecture (`DOCTRINE_ENFORCEMENT.md`) on `2026-06-22`
as its 4th portable standard** (alongside task-trees, `MEMORY_ARCHITECTURE.md`, and the knowledge-map),
on the owner directive "adopt this doctrine enforcement system". Tree `DOCTRINE-ENFORCEMENT-ADOPT`,
decision `0006`.

**The model: doctrine = a rule + a deterministic check that exits nonzero on any breach.** One driver
runs them all and the git gates run the driver.

- **`scripts/check_doctrines.sh`** is the registry+driver. Its `DOCTRINES=(…)` array is the source of
  truth for "which doctrine is enforced by which `check_*.sh`". It runs every registered check
  (collecting all results, not stopping at the first failure), prints a per-doctrine PASS/FAIL report,
  exits nonzero iff any failed, and **meta-checks** that each registered enforcer exists + is executable
  (so a registry entry can never be a dangling promise).
- **Registered today (3):** `MEMORY-ARCH` (`scripts/check_memory_architecture.sh`, structural),
  `KNOWLEDGE-MAP` (`knowledge-map/scripts/check_knowledge_map.sh`, structural), `TASK-ACCEPTANCE`
  (`scripts/check_task_acceptance.sh`, evidence).
- **Wiring:** `.githooks/pre-commit` (E3, activate once with `git config core.hooksPath .githooks`) and
  `scripts/run_ci.sh` (E4) both invoke the driver. The pre-commit regenerates + stages the derived
  knowledge map BEFORE the driver validates it (so map drift is structurally impossible).

**TASK-ACCEPTANCE mechanizes the flagship doctrine (decision 0003): no code change without an owning
task-tree leaf first.** A staged Rust behavioral change (`crates/**/*.rs` or `crates/**/test_data/**`)
must have a staged owning `docs/tasks/*.md` leaf whose acceptance checklist carries **ROOT CAUSE /
ADDRESSED / NO REGRESSION**, each ticked `[x]` AND backed by a SpecForge tool signature (a `validate`
finding/metric, an `adapt --target isf` `blocking_reason`, a `kg-bench` diagnostic, a `cargo test`, a
`--dry-run` measurement, `file:line`; and for NO-REGRESSION an oracle — `kg-bench 156/156`,
`WIRE-BASED-100 1.000`, byte-identical golds, `run_ci`). Docs-only / scripts-only / mdBook-only / hook
changes are **EXEMPT** (they carry their own gates) so continuity work is never false-blocked. Knobs:
`SPECFORGE_TASK_ACCEPTANCE_RANGE=<git-range>` (check a range instead of the staged set),
`SPECFORGE_TASK_ACCEPTANCE_WAIVER="<reason>"` (loud, never-silent exception; CI still re-checks). The
check is bash-3.2-safe (no `mapfile`).

**Earned, not ticked.** The local hook proves only that the boxes are ticked and a matching signature is
present (presence, leg 1). The cited oracles RE-RUN in `scripts/run_ci.sh` / CI (leg 3) — so a
self-ticked-but-false NO-REGRESSION box passes the hook and dies when `kg-bench` / the golds are re-run.
**Honest limit:** local hooks are bypassable (`--no-verify`) and SpecForge's hosted CI is currently
manual-only (`workflow_dispatch`), so the un-fakeable re-run happens at the next CI/`run_ci.sh` run, not
instantly. Re-enabling an auto CI doctrine-gate is the true "no matter what" backstop.

**To add a doctrine:** write `scripts/check_<id>.sh` obeying the check-script contract
(`DOCTRINE_ENFORCEMENT.md` §4 — exit nonzero on breach, deterministic, reads-the-repo-mutates-nothing,
scope-aware, path-agnostic), add one line to the driver's `DOCTRINES` array, and add a §10 row. A
candidate-but-deferred one is a structural ADR-0006 check (flag NEW hardcoded chip/signal-name literals
in extraction code), held back until it can ship without false-blocking legitimate test fixtures.

**`TOOLBOX.md`** is SpecForge's own diagnostic catalog (the evidence the checklist cites comes from
these): `doctor`, `inspect`, `validate`, `adapt --target isf` (`blocking_reasons`), FSMGen
`--strict --check`, `kg-bench`, the WIRE-BASED-100 golds, `--dry-run` byte-identical orthogonality,
`nli-verify`, `grits-consensus`, `measure_isf_completeness.py`, `corpus-cluster`, `run_ci.sh` /
`check_doctrines.sh`, plus 3 diagnostic protocols.
