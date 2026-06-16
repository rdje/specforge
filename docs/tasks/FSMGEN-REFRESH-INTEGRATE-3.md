# FSMGEN-REFRESH-INTEGRATE-3: bump FSMGen submodule to the phase-membership-response tip

## Metadata

- Tree ID: `FSMGEN-REFRESH-INTEGRATE-3`
- Status: `done` (CLOSED `2026-06-16`)
- Roadmap lane: `R6` (`.isf` adapter — FSMGen is the downstream `.isf` consumer)
- Created: `2026-06-16`
- Last updated: `2026-06-16`
- Owner: repo-local workflow
- Predecessor: `FSMGEN-REFRESH-INTEGRATE-2` (`done`, `2026-06-16`, bump `d31b0b91 → 8c39827f`).

## Goal

Per explicit owner direction (`2026-06-16`): *"Please update FSMGEN submodule and read its
response `fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md`."* This refresh exists because **FSMGen
answered SpecForge's `2026-06-16` transaction-phase-membership question** (raised in
`docs/FSMGEN_FEEDBACK.md` under `KG-ISF-TRANSACTIONS.2i`). Update the pinned `subs/fsmgen`
gitlink to the upstream tip that carries the answer, verify SpecForge's emitted `.isf` is still
strict-valid on the new binary, record the answer + how SpecForge will act, and reconcile the
current-facing pin references.

## Measured baseline (`2026-06-16`, read-only)

- Committed gitlink at superproject HEAD `6fd927eb`: `8c39827f`.
- Upstream `origin/main` tip after fetch: **`030f8c273`** — **+9 commits** over `8c39827f`,
  including the two direct response commits `ISF-SPECFORGE-PHASE-MEMBERSHIP-RESPONSE.1`
  (`24b23db3c`, select) + `.2` (`030f8c273`, answer), plus `ISF-VERIFICATION-OBSERVATION-METADATA.1`
  (shipped actor-level `(observe …)` passive-monitor metadata), an IAL1 verification-code-generation
  frontier, an import-tree refresh, a legacy `fx` removal, and an Accellera standards reference import.

## FSMGen's answer (read from `subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md`, `2026-06-16` section)

Short answer: **do not fabricate either value or order**; no immediate FSMGen code change is
needed to say so; the right *future* feature is **checked transaction phase-group metadata in ISF**,
under its own FSMGen task tree before any parser/report behavior change. Per question:

1. **Value-less output participation → NO.** `(drive S)` is value-bearing behavior; accepting a
   bare/don't-care/default drive "would be the same fabrication SpecForge is trying to avoid." Keep
   value-less participation in SpecForge IntentIR metadata/residual; emit a `drive` only when the
   source grounds participation AND value.
2. **Unordered / partial-order body → NO.** ISF bodies are intentionally source-ordered; a body is
   "the wrong container for a set of membership facts whose cross-phase ordering is not asserted."
   Keep such facts outside the body.
3. **Phase-group metadata → YES (agreed future ISF shape).** First version = **checked metadata,
   not generated behavior**: transaction name; authored phase/group names; member signals per phase;
   actor-relative role/direction; optional provenance/residual notes. Hard negative rule: must not
   imply drive values, schedule order, HDL, or SV/UVM/VHDL output until those have their own
   contracts; validation stays real (names resolve, directions match, malformed fails closed).
   Adjacent to but distinct from the now-shipped actor-level `(observe …)` metadata. Needs its own
   FSMGen tree — **not shipped yet.**
4. **Ordering — constraint vs body → confirmed division.** body clauses = grounded behavior + source
   order; verification family (`assert`/`assume`/`cover`, `after`/`next`/`within`, monitors,
   sampled-value predicates) = grounded temporal obligations; future phase-group metadata =
   membership/phase/role facts, no generated behavior.
- **`.isf` stays the source of truth** for the synthesizable path; do NOT target a hypothetical
  `.val` (a possible future verification-abstraction interchange, not a replacement, not needed here).

**Consequence for SpecForge:** this **validates the honest-residual stance and confirms
`KG-ISF-TRANSACTIONS.2i` option (a)** — ship the grounded per-phase membership grouping as IntentIR
**metadata** (not ordered ISF steps); keep value/order as honest residual; `.isf` byte-identical.
The cross-`.isf` phase-group metadata surface is a recorded *future FSMGen* feature that would later
carry the membership across the `.isf` boundary (FSMGen owns building it).

## Non-Goals

- Do NOT modify the `subs/fsmgen` submodule from this repository (read-only upstream reference).
- Do NOT adopt the shipped `(observe …)` metadata or build the future phase-group surface in this
  tree — assessment + pin bump only; `.2i` (metadata grouping) is its own owned slice.
- Respect `[[feedback_fsmgen_contract]]` (contract is authority) + `[[feedback_verify_fsmgen_before_fr]]`
  (empirically verify on the new binary).

## Acceptance Criteria

- `subs/fsmgen` pinned to `030f8c273`; gitlink staged + committed; full `scripts/run_ci.sh` green on
  the new pin — the `*_passes_fsmgen_strict_validation` canaries still accept SpecForge's emitted
  `.isf`, or any divergence recorded honestly (never forced).
- FSMGen's answer recorded; the `2026-06-16` question in `docs/FSMGEN_FEEDBACK.md` marked ANSWERED with
  SpecForge's planned action; reviewed-baseline pin reconciled `8c39827f → 030f8c273`.

## Task Tree

- ID: `FSMGEN-REFRESH-INTEGRATE-3` · Status: `done` · Children: `.1`
- ID: `FSMGEN-REFRESH-INTEGRATE-3.1` · Status: `done` (`2026-06-16`) · Goal: bump `subs/fsmgen`
  `8c39827f → 030f8c273` (+9); verify on the new binary; read + record FSMGen's phase-membership
  answer; mark the `docs/FSMGEN_FEEDBACK.md` question ANSWERED; reconcile pin references; commit.
  **DONE:** checked out `030f8c273` (clean working tree). Re-ran SpecForge's emitted APB `.isf`
  through the new `bin/fsmgen --strict --check --json` → `success:true, 0 diagnostics`; the 6
  fsmgen-strict canary tests + full `scripts/run_ci.sh` GREEN (`1645 passed / 0 failed / 2 ignored`;
  fmt + warning-deny clippy + rustdoc + mdBook) → **the +9-commit bump does NOT break SpecForge's
  emitted `.isf` contract.** FSMGen's answer read + recorded (above + in `docs/FSMGEN_FEEDBACK.md`
  ANSWERED callout); reviewed-baseline `8c39827f → 030f8c273`; current-facing pin references reconciled
  (README, MEMORY). No strict-behavior divergence. Unblocks `KG-ISF-TRANSACTIONS.2i` (option (a),
  FSMGen-confirmed).

## Current Frontier

Empty — **tree CLOSED `2026-06-16`** (`.1` done). Next: `KG-ISF-TRANSACTIONS.2i` (unparked — implement
the FSMGen-confirmed metadata-only per-phase membership grouping).

## Decisions

- `2026-06-16`: created by explicit owner direction (update + read the response). The bump's whole
  point is FSMGen's phase-membership answer; it confirms `.2i` option (a) (metadata-only grouping) and
  records the future ISF phase-group metadata surface as the cross-`.isf` carry path (FSMGen-owned).

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-16` | `.1` | checkout `030f8c273` clean; emitted APB `.isf` strict `success:true`/0 diag on new binary; 6 fsmgen-strict canaries pass; full `scripts/run_ci.sh` (`1645/0/2`; fmt+clippy+rustdoc+mdBook); pins reconciled; FSMGen answer recorded | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `FSMGEN-REFRESH-INTEGRATE-3.1` | `FSMGEN-REFRESH-INTEGRATE-3.1 — bump subs/fsmgen 8c39827f -> 030f8c273 (+9); record phase-membership answer; CI green` | gitlink change (tracked); SpecForge `.isf` still strict-valid; canary 6/6 + CI 1645/0; confirms `.2i` option (a) |

## Changelog

- `2026-06-16`: **`.1` DONE — tree CLOSED.** Owner asked to update FSMGen + read its response. Bumped
  `subs/fsmgen` `8c39827f → 030f8c273` (+9) — the tip carries FSMGen's answer to SpecForge's
  `2026-06-16` transaction-phase-membership question (`ISF-SPECFORGE-PHASE-MEMBERSHIP-RESPONSE.1/.2`).
  **Answer:** don't fabricate value or order; keep value-less participation + unordered membership as
  IntentIR metadata/residual (NOT transaction-body drives/steps); checked transaction phase-group
  metadata is the right *future* ISF surface (its own FSMGen tree, not shipped); `.isf` stays the
  source of truth (no `.val`). This **confirms `KG-ISF-TRANSACTIONS.2i` option (a)** — metadata-only
  grouping. Verified on the new binary (emitted APB `.isf` strict `success:true`; 6 canaries + full
  `run_ci.sh` green `1645/0/2`); reviewed-baseline `8c39827f → 030f8c273`; pins reconciled. Next:
  implement `.2i` (unparked). `[[project_kg_isf_transactions]]` / `[[feedback_isf_no_hacks]]` /
  `[[feedback_verify_fsmgen_before_fr]]`.
