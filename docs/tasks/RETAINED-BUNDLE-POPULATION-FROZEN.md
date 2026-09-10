# RETAINED-BUNDLE-POPULATION-FROZEN: the retained normalized-bundle set can neither grow nor shrink

## Metadata

- Tree ID: `RETAINED-BUNDLE-POPULATION-FROZEN`
- Status: `active`
- Roadmap lane: `R15e`/`R16` (corpus currency / doctrine enforcement)
- Created: `2026-09-10`
- Last updated: `2026-09-10`
- Owner: repo-local workflow

## Goal

Make `doctrine/chain_currency/retained_bundles.json` movable in the two directions ADR 0025 already
mandates — **grow** when a refresh keeps its normalized bundle, and **shrink** when a reclamation is
deliberate and task-owned — without any gate becoming a stop that compliant work cannot pass.

## The finding (measured `2026-09-10` by `WIRE-BASED-100.9b`)

ADR 0025 decision 3 says a refresh **keeps** its bundle ("retention is what makes a document
replayable"), that the 54 missing bundles are "backfilled at each document's own refresh", and that
reclamation stays possible as a "deliberate, task-owned" recorded operation. Three independent
mechanisms freeze the set against exactly those two operations:

| # | Mechanism | Where | Forbids |
| --- | --- | --- | --- |
| 1 | `len(retained_ids) != 24` literal | `scripts/validate_residual_actionability_contract.py` (gate-tier) and `scripts/validate_canonical_recovery_contract.py` | GROWTH and SHRINK — any size but 24 |
| 2 | `retained.reclamations != []` | the same two validators | SHRINK — the mechanism ADR 0025 provides for recording a deliberate reclamation |
| 3 | behavioral population **set equality** with `retained` | `scripts/check_behavioral_genericity_contract.py` (gate-tier, `PRODUCTION-GENERICITY`) | GROWTH — a new key must also arrive as a fully qualified held-out row |

Mechanism 3 is the substantive one. The behavioral contract declares a `selection_boundary_commit`
and a frozen 24-row population with a 7/17 calibration/holdout split, `declared_attempts: 51`
(17 prospective × 3 relations) and per-document held-out evidence including vendor/family novelty.
A newly retained document therefore cannot be admitted by bookkeeping: it needs three executed
held-out relations and an amendment to a release-blocking qualification whose `final_signoff` is
`deferred_to_SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.v`.

**How it was found.** `WIRE-BASED-100.9b` re-ingested the APB gold. The re-ingest restored its
normalized bundle, so `check_chain_currency.sh` required the declaration to grow (an undeclared
bundle on disk fails closed as "a refresh that did not record what it retained"). Declaring it turned
`PRODUCTION-GENERICITY` and `RESIDUAL-ACTIONABILITY` red. Declaring a reclamation instead would have
turned the same two red through mechanism 2. The first refresh to exercise ADR 0025 after the freeze
had no compliant move.

**Interim position taken by `.9b`, deliberately reversible.** The APB bundle was moved to
`generated/preserved/WIRE-BASED-100.9b/apb-normalized-bundle-held-out/` (repository volume, 25 MB,
byte-identical, never deleted) and the declaration left at 24 with `reclamations: []`. Consequences,
stated rather than hidden: APB's SourceIR→EvidenceIR replay is reported UNMEASURABLE by
`check_chain_currency.sh` until the bundle returns, while its SemanticIR, IntentIR and adapter stages
are measurable and current, and its EvidenceIR is canonical schema 3 and scoreable. Nothing was
deleted and no gate was weakened; the choice is a hold, not a reclamation.

## Non-Goals

- Executing the held-out behavioral qualification of any newly retained document. That is
  `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f`'s frozen, release-blocking program; this tree makes the
  population movable and states what the new document still owes, it does not qualify it.
- Weakening the exact-set binding. `affected_chain_ids_sha256` pins the retained set exactly and must
  keep doing so; only the redundant magic size and the `reclamations` freeze are in question.
- Re-opening ADR 0025.

## Acceptance Criteria

- A refresh that keeps its bundle can declare it and pass every gate, with any behavioral-qualification
  debt reported explicitly rather than as a hard stop.
- A deliberate, task-owned reclamation can be recorded in `reclamations` and pass every gate.
- The exact-set digest binding is preserved, and each change is proven by a controlled RED case.
- `WIRE-BASED-100.9b`'s held-out APB bundle is restored to `generated/source_ir/` and declared.

## Task Tree

- ID: `RETAINED-BUNDLE-POPULATION-FROZEN`
  Status: `active`
  Goal: make the retained-bundle population movable in both mandated directions
  Children: `.1`, `.2`, `.3`

- ID: `RETAINED-BUNDLE-POPULATION-FROZEN.1`
  Status: `pending`
  Goal: **retire the redundant `24` literal and the `reclamations != []` freeze in both contract
  validators, keeping the exact-set digest as the real binding.** The digest already pins the exact
  membership, so the size literal adds no guarantee the digest does not already give — it only makes a
  policy-mandated operation unreachable. `reclamations != []` is stronger still: it forbids the exact
  mechanism ADR 0025 names for a deliberate reclamation. Both contracts must instead require that the
  declared `affected_chain_count`/`affected_chain_ids_sha256` agree with the live authority, which is
  what they were written to mean.
  Acceptance: `scripts/validate_residual_actionability_contract.py` and
  `scripts/validate_canonical_recovery_contract.py` accept a 25-key retained set and a recorded
  reclamation while still rejecting a count/digest that disagrees with the authority; each relaxation
  ships a controlled RED case in the script's `--self-test`; `scripts/check_doctrines.sh` green.
  Verification: `pending`
  Commit: `pending`

- ID: `RETAINED-BUNDLE-POPULATION-FROZEN.2`
  Status: `pending`
  Goal: **decide, with `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f`, what a newly retained document owes the
  frozen behavioral population, and make the gate say it instead of blocking.** The candidate shape:
  the frozen selection must remain a SUBSET of the live retained set (nothing selected may silently
  vanish — the guarantee worth keeping), and any retained key outside the frozen selection is reported
  as an explicitly unqualified residual naming the leaf that owes its three held-out relations. The
  competing shape — every retained key must be qualified before it may be retained — is what exists
  today and is what produced a stop with no remedy.
  Acceptance: the chosen rule is recorded as a decision record, implemented in
  `scripts/check_behavioral_genericity_contract.py` with a controlled RED case for a vanished frozen
  key AND for an unreported new key, and `PRODUCTION-GENERICITY` green with the APB key retained.
  Prerequisite: `.1`.
  Verification: `pending`
  Commit: `pending`

- ID: `RETAINED-BUNDLE-POPULATION-FROZEN.3`
  Status: `pending`
  Goal: **restore the held-out APB bundle and close the interim position.** Move
  `generated/preserved/WIRE-BASED-100.9b/apb-normalized-bundle-held-out/` back to
  `generated/source_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/normalized/`, declare it in
  `retained_bundles.json`, and re-run the currency gate so APB's EvidenceIR replay becomes measurable.
  Acceptance: `check_chain_currency.sh` reports 25 retained bundles and 25 measurable EvidenceIR
  replays with zero stale; the book's retention census and `WIRE-BASED-100.9b`'s interim note are
  corrected where they were published.
  Prerequisite: `.2`.
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `RETAINED-BUNDLE-POPULATION-FROZEN.1` | `pending` | it is the mechanical half — a redundant literal and a freeze on a documented operation — and it unblocks `.9c`/`.9d` for the residual contract without touching a qualification |
| 2 | `RETAINED-BUNDLE-POPULATION-FROZEN.2` | `pending` | the substantive half; needs the alignment tree's owner and a decision record |
| 3 | `RETAINED-BUNDLE-POPULATION-FROZEN.3` | `pending` | restoration is only meaningful once both gates accept a 25th key |

## Decisions

- `2026-09-10`: **Hold, do not reclaim, and do not force the declaration.** `WIRE-BASED-100.9b`
  preserved the APB bundle on the repository volume rather than deleting it, and left the declaration
  untouched. Reason: a wire-protocol leaf must not enlarge a frozen, in-flight, release-blocking
  qualification population as a side effect of re-ingesting one document, and must not delete evidence
  the current binary cannot regenerate. The hold costs one document's EvidenceIR replay measurability
  and nothing else; it is reversed by `.3`.
- `2026-09-10`: **The exact-set digest is the binding worth keeping; the size literal is not.**
  `affected_chain_ids_sha256` already forces the contract to be updated whenever membership changes,
  so `len(ids) != 24` cannot catch a drift the digest misses. It can only stop compliant growth.

## Open Questions

- Does `scripts/validate_canonical_recovery_contract.py` still have an owner? Nothing in
  `scripts/check_doctrines.sh` or `scripts/run_ci.sh` invokes it, and it reports four further failures
  unrelated to this tree (witness boundary and frozen population-summary drift) that no gate would
  ever surface. Owner and disposition are unresolved; it does not block `.1`, which must keep the
  script correct whether or not anything runs it.

## Blockers

- None. `.2` needs the alignment tree's owner in the room, which is scheduling, not a blocker.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-10` | finding | `scripts/check_doctrines.sh` with the APB key declared | `PRODUCTION-GENERICITY` and `RESIDUAL-ACTIONABILITY` FAIL; with it undeclared and the bundle held out, all gate-tier doctrines PASS |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| finding | `WIRE-BASED-100.9b — …` | opened by the leaf that hit the stop |
| `RETAINED-BUNDLE-POPULATION-FROZEN.1` | `pending` | `pending` |

## Changelog

- `2026-09-10`: Created. `WIRE-BASED-100.9b`'s APB re-ingest found that the retained normalized-bundle
  population is frozen against both operations ADR 0025 mandates, by a redundant size literal, a
  `reclamations` freeze, and a set-equality join to a frozen behavioral qualification.
