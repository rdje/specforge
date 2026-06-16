# FSMGEN-REFRESH-INTEGRATE-2: refresh the FSMGen submodule (2026-06) + re-assess adoptable ISF features

## Metadata

- Tree ID: `FSMGEN-REFRESH-INTEGRATE-2`
- Status: `done` (CLOSED `2026-06-16`)
- Roadmap lane: `R6` (`.isf` adapter — FSMGen is the downstream `.isf` consumer)
- Created: `2026-06-16`
- Last updated: `2026-06-16`
- Owner: repo-local workflow
- Predecessor: `FSMGEN-REFRESH-INTEGRATE` (`done`, `2026-05-29`, bump `9bfb9a20 → 88a7af9c`); this
  is the second refresh cycle requested by the owner after a fresh upstream push.

## Goal

Per explicit owner direction (`2026-06-16`): *"FSMGEN recently pushed. Please update FSMGEN
submodule. then go through FSMGEN book/handoff/contract."* Update the pinned `subs/fsmgen`
submodule to the current upstream tip, then read its handoff, integration, and
public-API-contract docs plus its mdBook thoroughly, and re-assess which FSMGen features /
contract changes SpecForge's `.isf` adapter must track or can adopt. FSMGen is the downstream
consumer of SpecForge's emitted `.isf`; staying current keeps the emitted-`.isf` contract honest
and surfaces new lowering capability or breaking changes early.

This cycle ALSO reconciles a measured live-doc drift: the committed gitlink is `d31b0b91`
("ARCHITECTURE-DEBT-FRONTIER.3: defer ISF extraction"), but `README.md` / `MEMORY`-era references
still cite the pin as `c0b7eaa7` — stale by several intervening re-pins
(`88a7af9c → c0b7eaa7 → 43b29f5c → 92d7036b → d31b0b91`, made under `ISF-SYMBOL-SURFACE-EMIT.1`,
`FSMGEN-ASSERT-MIGRATE.2`, `FSMGEN-ASSERT-LOWERING.1`, `SWD-SERIAL-EXTRACTION.6`).

## Measured baseline (`2026-06-16`, read-only)

- Committed gitlink at superproject HEAD `1985cdb4`: `d31b0b91` (`subs/fsmgen` clean checkout).
- Upstream `origin/main` tip after fetch: `8c39827f` — **300 commits ahead** of `d31b0b91`.
  New upstream work spans `BACKEND-LANGUAGE-PORTABILITY-CONTRACT-FRONTIER`,
  `IAL2-FEATURE-COMPLETENESS-FRONTIER`, `SEMANTIC-INTROSPECTION-MCP-FRONTIER`, and more.
- The fsmgen binary is the Perl script `subs/fsmgen/bin/fsmgen`; SpecForge tests invoke it via
  `run_fsmgen_strict_check` (`ir/mod.rs`) — so the contract canary is `cargo test`'s
  `*_passes_fsmgen_strict_validation` family (`ir/adapters.rs`, `ir/isf_ir.rs`), NOT a Rust rebuild
  of fsmgen.

## Non-Goals

- Do NOT modify the `subs/fsmgen` submodule from this repository (read-only upstream reference);
  upstream misbehavior is tracked locally via the `DOWNSTREAM_ISSUE_REPORTING.md` bundle protocol.
- Do NOT adopt a new FSMGen feature into the SpecForge adapter in this tree beyond what is
  verifiably safe; adapter changes that alter emitted `.isf` are scoped as their own follow-up
  trees (this tree assesses + recommends).
- Respect `[[feedback_fsmgen_contract]]`: parser acceptance ≠ support — the public-interface
  contract / integration spec is the authority, not empirical binary behavior alone.
- Respect `[[feedback_verify_fsmgen_before_fr]]`: update the submodule + read current docs +
  empirically test before claiming any FSMGen/ISF gap or filing a feature request.

## Acceptance Criteria

- `subs/fsmgen` pinned to current upstream `origin/main` tip (`8c39827f`); gitlink staged +
  committed; full `scripts/run_ci.sh` green on the new pin — critically the
  `*_passes_fsmgen_strict_validation` tests still accept SpecForge's emitted `.isf`, or any
  divergence is understood + recorded honestly (never forced).
- The handoff / integration / public-API-contract / mdBook docs are read thoroughly and a concrete
  feature-adoption / contract-delta assessment is recorded (what SpecForge must track, what it can
  adopt now, what needs a follow-up tree, what stays out of scope).
- Stale pin references in tracked docs (README, RUST_CODEBASE_ANALYSIS, FSMGEN_FEEDBACK baseline,
  MEMORY) reconciled to the new pin.

## Task Tree

- ID: `FSMGEN-REFRESH-INTEGRATE-2` · Status: `active` · Children: `.1`, `.2`
- ID: `FSMGEN-REFRESH-INTEGRATE-2.1` · Status: `done` (`2026-06-16`) · Goal: bump `subs/fsmgen`
  `d31b0b91 → 8c39827f` (+300); smoke-test `bin/fsmgen`; run full `scripts/run_ci.sh` green on the
  new binary (esp. the fsmgen-strict ISF canary tests); reconcile the stale pin references in
  `README.md` / `RUST_CODEBASE_ANALYSIS.md` / `docs/FSMGEN_FEEDBACK.md` / `MEMORY.md`. Commit
  (gitlink is a tracked change). Any strict-behavior divergence recorded honestly.
  **DONE:** checked out `8c39827f` (clean working tree; `perl -c bin/fsmgen` syntax-OK). The 7
  fsmgen-strict canary tests (`isf_output_passes_fsmgen_strict_validation` +
  `bounded_contract`/`guarded_windowed_eventual`/`symbol_surface`/`temporal_rule`/`temporal_stage`
  `_passes_fsmgen_strict_validation` + `transaction_steps_use_fsmgen_contract_grammar`) all PASS on
  the new binary → **the +300-commit bump does NOT break SpecForge's emitted `.isf` contract**.
  Full `scripts/run_ci.sh` GREEN (`1641 passed / 0 failed / 2 ignored`; fmt + warning-deny clippy +
  rustdoc + mdBook). `kg-bench` 156/156 (orthogonal, run for rigor). Gitlink staged → `8c39827f`.
  Reconciled the CURRENT-facing pin references (`README.md` ×2, `MEMORY.md`); the dated historical
  pin snapshots in `CHANGES.md`/`DEVELOPMENT_NOTES.md`/`LIVE_ACHIEVEMENT_STATUS.md`/
  `RUST_CODEBASE_ANALYSIS.md` are left as point-in-time record (no-rewrite-dated-history, per the
  `FSMGEN-REFRESH-INTEGRATE.1` precedent); `KNOWLEDGE_MAP.md` is auto-derived (never hand-edited).
  `docs/FSMGEN_FEEDBACK.md` reviewed-baseline update deferred to `.2` (set once the new docs are
  actually read). No strict-behavior divergence to record.
- ID: `FSMGEN-REFRESH-INTEGRATE-2.2` · Status: `done` (`2026-06-16`) · Goal: read thoroughly the handoff
  (`SPECFORGE_FEEDBACK_RESPONSE.md`), integration (`ISF_DOWNSTREAM_INTEGRATION_SPEC.md`), public
  API contract (`ISF_PUBLIC_INTERFACE_CONTRACT.md`), feature-support matrix (book `13k`), and the
  mdBook ISF chapters (`13*`); record a concrete contract-delta + feature-adoption assessment;
  update `DEVELOPMENT_NOTES.md` + `docs/FSMGEN_FEEDBACK.md`; scope any adoption as named follow-up
  trees; close the tree. Docs-only.
  **DONE** (contract-is-authority + empirical canary from `.1`; full assessment in `DEVELOPMENT_NOTES.md`
  "FSMGEN-REFRESH-INTEGRATE-2.2"). Findings: **(a) CONTRACT DELTA = NONE** affecting SpecForge's emitted
  `.isf` — every emitted form still `shipped` in `13k`; 7/7 strict canaries + `run_ci.sh` green; the removed
  `(contract … (eventually …))` / transaction `(assign …)` / deprecated `(handshake …)` are forms SpecForge
  doesn't emit (already migrated/never emitted); no SpecForge-directed ask is dated after `2026-06-04`.
  **(b) +300 commits are FSMGen-internal** (compositional control-flow acceptance widening, ATL diagnostics,
  IAL2 feature-completeness, backend-language portability, a semantic-introspection MCP server) — no new ISF
  surface SpecForge must change. **(c) ALREADY ADOPTED:** the assert/verification family (monitor/within
  MIN MAX/stable/changed/rose/fell/=>) — closed `FSMGEN-ASSERT-LOWERING`/`FSMGEN-ASSERT-MIGRATE`. **(d) "Do we
  need NEW ISF features?" → NO, not now** — the refreshed surface is sufficient for everything SpecForge
  extracts AND for the deferred composed multi-phase transaction body (already expressible; a BUILD gap, not
  an ISF gap; the control-flow widening makes the downstream MORE ready). Per `[[feedback_isf_no_hacks]]` no
  missing abstraction is being hacked around → no FR warranted. **(e) ONE grounded NEW adopt candidate** →
  proposed follow-up tree `ISF-REGISTER-RESET-EMIT` (register/CSR reset values `(storage (var … (reset V)))`
  — SpecForge extracts register-field `reset_value` + has a `(storage)` emit surface not yet fed from it;
  measurement-first, WIRE-BASED-100-gated, ISF-strict-validated). Conditional FUTURE ISF candidates recorded
  (raise only after SpecForge extracts the intent + empirical probe): ID-based out-of-order outstanding-txn
  correlation; first-class phase-group typing. Reviewed-baseline in `docs/FSMGEN_FEEDBACK.md` updated
  `88a7af9c → 8c39827f`. No user-facing SpecForge behavior changed → mdBook unchanged (the BOOK-METHOD-DOC
  close-rule is N/A for a refresh/assessment tree that ships no user-facing capability).

## Current Frontier

Empty — **tree CLOSED `2026-06-16`** (`.1`+`.2` both `done`). One adoptable item recorded as the proposed
follow-up tree `ISF-REGISTER-RESET-EMIT` (created when promoted, per the "captured + owned, execution on
promotion" pattern). Next: resume `KG-ISF-TRANSACTIONS.2d` (owner-ordered).

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `FSMGEN-REFRESH-INTEGRATE-2.1` | `done` | bumped `d31b0b91 → 8c39827f`; CI green 1641/0; canary green; pins reconciled |
| 2 | `FSMGEN-REFRESH-INTEGRATE-2.2` | `done` | contract delta = none; no new ISF FR needed; adopt candidate → proposed `ISF-REGISTER-RESET-EMIT`; tree closed |

## Decisions

- `2026-06-16`: created by explicit owner direction. Upstream is **+300 commits**
  (`d31b0b91 → 8c39827f`). Bump to `origin/main` tip, then read the *updated* docs (checkout
  first, then read) so the assessment reflects current upstream, per the owner's "update … then go
  through" sequence. The owner also directed: after this tree, resume `KG-ISF-TRANSACTIONS.2d`.

## Open Questions

- Whether the +300-commit bump changes fsmgen `--strict` behavior in a way that affects
  SpecForge's emitted `.isf` contract. To be resolved empirically in `.1` (the strict ISF tests
  are the canary) cross-checked against the public-interface contract in `.2`.

## Blockers

- None (network fetch succeeded; upstream tip `8c39827f` identified).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-16` | `.1` | checkout `8c39827f` clean + `perl -c` OK; 7 fsmgen-strict canary tests pass on new binary; full `scripts/run_ci.sh` (`1641/0/2`; fmt+clippy+rustdoc+mdBook); `kg-bench` 156/156; pins reconciled | `passed` |
| `2026-06-16` | `.2` | thorough doc read (contract/integration/handoff/matrix `13k`/`13b-13m`) cross-referenced vs `ir/isf_ir.rs` + closed adoption trees; contract delta = none; no new ISF FR needed; assessment recorded (DEVELOPMENT_NOTES + FSMGEN_FEEDBACK baseline `88a7af9c → 8c39827f`); follow-up `ISF-REGISTER-RESET-EMIT` named; memory-arch gate ok | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `FSMGEN-REFRESH-INTEGRATE-2.1` | `FSMGEN-REFRESH-INTEGRATE-2.1 — bump subs/fsmgen d31b0b91 -> 8c39827f (+300); CI green; reconcile pins` | gitlink change (tracked); SpecForge `.isf` still strict-valid; canary 7/7 + CI 1641/0 + kg-bench 156/156 |
| `FSMGEN-REFRESH-INTEGRATE-2.2` | `FSMGEN-REFRESH-INTEGRATE-2.2 — ISF feature-adoption assessment; no new ISF FR needed; close` | docs-only; contract delta = none; adopt candidate → proposed `ISF-REGISTER-RESET-EMIT`; reviewed-baseline `88a7af9c → 8c39827f` |

## Changelog

- `2026-06-16`: **`.2` DONE — tree CLOSED.** Read the new upstream's contract/integration/handoff/feature-matrix
  (`13k`)/ISF chapters (`13b`–`13m`) and cross-referenced vs `ir/isf_ir.rs` + the closed adoption trees.
  **Contract delta = NONE** (every emitted form still `shipped`; 7/7 strict canaries + `run_ci.sh` green; removed/
  deprecated forms aren't emitted; no ask after `2026-06-04`). **+300 commits are FSMGen-internal** (compositional
  control-flow widening, ATL diagnostics, IAL2, backend portability, semantic-introspection MCP). **"Need new ISF
  features?" → NO** — the surface is sufficient for current + deferred transaction work (composed multi-phase body
  is a BUILD gap, not an ISF gap; the control-flow widening makes the downstream MORE ready); per
  `[[feedback_isf_no_hacks]]` no missing abstraction → no FR. **ONE grounded adopt candidate** → proposed tree
  `ISF-REGISTER-RESET-EMIT` (register/CSR `(reset V)` storage; SpecForge extracts register-field `reset_value` +
  has a `(storage)` surface). Conditional future candidates recorded (ID-based out-of-order outstanding-txn
  correlation; phase-group typing) — not filed. Assessment in `DEVELOPMENT_NOTES.md`; reviewed-baseline in
  `docs/FSMGEN_FEEDBACK.md` `88a7af9c → 8c39827f`. Docs-only; no user-facing change → mdBook unchanged.
- `2026-06-16`: **`.1` DONE** — bumped `subs/fsmgen` `d31b0b91 → 8c39827f` (+300). All 7
  fsmgen-strict canary tests pass on the new binary (SpecForge's emitted `.isf` stays strict-valid);
  full `scripts/run_ci.sh` green (`1641/0/2`); `kg-bench` 156/156. Reconciled the current-facing pin
  references (README ×2, MEMORY); dated history left intact; `KNOWLEDGE_MAP.md` not hand-edited.
  Frontier → `.2` (read the handoff/integration/contract/mdBook of the new upstream + record the
  feature-adoption assessment + close).
- `2026-06-16`: Created by explicit owner direction (update FSMGen submodule + go through
  book/handoff/contract). Recorded the measured baseline (committed gitlink `d31b0b91`; upstream
  `origin/main` `8c39827f`, +300; README pin reference stale at `c0b7eaa7`). Frontier → `.1`.
