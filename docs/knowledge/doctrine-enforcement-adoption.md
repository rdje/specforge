---
id: doctrine-enforcement-adoption
title: SpecForge enforces every mechanizable doctrine via one registry/driver gated E1→E4, including published-claim provenance
answers:
  - "how are doctrines enforced in specforge"
  - "what is scripts/check_doctrines.sh / the doctrine driver"
  - "how do I add a new enforced doctrine / doctrine check"
  - "what blocks a Rust code change from committing in specforge"
  - "where is the acceptance checklist a code change must satisfy"
  - "what is the 4th portable architecture (doctrine enforcement)"
  - "which doctrines are registered in the SpecForge doctrine driver"
  - "why does the pre-commit hook run check_doctrines.sh"
  - "what does the TASK-ACCEPTANCE check verify / why was my commit blocked"
  - "what debug/diagnostic tools does specforge have (TOOLBOX.md)"
  - "which doctrines are registered (MEMORY-ARCH, KNOWLEDGE-MAP, TASK-ACCEPTANCE)"
  - "how do I waive or range-scope the task-acceptance check"
date: 2026-06-22
tags: [doctrine-enforcement, process, governance, pre-commit, ci, task-tree-doctrine, toolbox, adr-0003, portable-architecture]
evidence: DOCTRINE_ENFORCEMENT.md (the standard; §10 = live registry); scripts/check_doctrines.sh (registry+driver, DOCTRINES array); scripts/check_task_acceptance.sh (evidence check); scripts/check_memory_architecture.sh + knowledge-map/scripts/check_knowledge_map.sh (the two structural checks); TOOLBOX.md (tool catalog + acceptance-checklist template); .githooks/pre-commit + scripts/run_ci.sh (E3/E4 wiring); docs/decisions/0006-doctrine-enforcement-architecture.md; docs/tasks/DOCTRINE-ENFORCEMENT-ADOPT.md; docs/book/src/reference/doctrine-enforcement.md
reverify: "bash scripts/check_doctrines.sh; sed -n '/^DOCTRINES=(/,/^)/p' scripts/check_doctrines.sh; perl scripts/check_claim_verification.pl --report"
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
- **The registered set is the `DOCTRINES` array itself**, and its size is not carried here: the driver prints
  `(N registered, tier=…)` on every run, and `DOCTRINE_ENFORCEMENT.md` §10 mirrors the rows in prose. Enumerate
  it with `grep -c '^  "[A-Z]' scripts/check_doctrines.sh`. The gate-tier rows — `MEMORY-ARCH`,
  `KNOWLEDGE-MAP`, `TASK-ACCEPTANCE`, `README-POLICY`, `LIVE-DOC-SIZE`, `PROJECT-DATA-LOCALITY`,
  `PRODUCTION-GENERICITY`, `CORPUS-FRONTIER`, `CLAIM-VERIFICATION`, `PROOF-SEAL-CURRENCY` — run on the default
  invocation; CI-tier `CHAIN-CURRENCY` is reported as `DEFER` and runs under `--all`. This card carried a count
  and a short list until `CLAIM-VERIFICATION-ADOPTION.11`: `PROOF-SEAL-CURRENCY` was registered by
  `SOURCE-IR-REPRODUCIBILITY.16` and the enumeration was never extended, which is a set claim whose own
  enumeration was short by one member — refutable, and refuted, by a single counterexample.
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
scope-aware, path-agnostic), add one line to the driver's `DOCTRINES` array, and add a §10 row. ADR 0006 is now
enforced by the compositional `PRODUCTION-GENERICITY` row rather than a vocabulary denylist.

`CLAIM-VERIFICATION` is the fifth portable architecture's enforcement row. Its dedicated self-bounded registry
and checker execute verified claims' argv-form source/RED controls, authenticate exact tracked artifact digests,
require complete stale-check membership, and resolve the prepared commit message or `HEAD` against known current
claim IDs. Twenty-seven controlled cases keep malformed, stale, untracked, missing/stale/misdirected exact-RED,
and ignored/untracked scratch-producer provenance fail-closed. The live audit derives seven cited controls, seven
exact RED regions, and six governed producers with zero ignored or untracked candidates; record validity is not
treated as proof that the underlying assertion is semantically correct.

**`TOOLBOX.md`** is SpecForge's own diagnostic catalog (the evidence the checklist cites comes from
these): `doctor`, `inspect`, `validate`, `adapt --target isf` (`blocking_reasons`), FSMGen
`--strict --check`, `kg-bench`, the WIRE-BASED-100 golds, `--dry-run` byte-identical orthogonality,
`nli-verify`, `grits-consensus`, `measure_isf_completeness.py`, `corpus-cluster`, `run_ci.sh` /
`check_doctrines.sh`, plus 3 diagnostic protocols.
