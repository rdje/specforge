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
  Children: `.1`, `.2`, `.3`, `.4`

- ID: `RETAINED-BUNDLE-POPULATION-FROZEN.1`
  Status: `done` (`2026-09-19`, CODE/DOC)
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
  **Shipped in both validators.** Mechanism 1 (`len(...) != 24`) and mechanism 2 (`reclamations != []`)
  are retired; what remains is what those lines were written to mean — the declared
  `affected_chain_count` must equal the live retained count, and `affected_chain_ids_sha256` must equal
  the live membership digest. Structural validity is still required: `retained` and `reclamations` must
  both be lists, and the recovery contract still pins `schema_version == 1`.
  **The digest is strictly stronger than the literal it replaces, which is why this is a relaxation and
  not a loosening.** A size check cannot see a same-size SUBSTITUTION; the digest can. Proved by
  perturbation rather than argued: dropping the count binding lets both `partial-chain-set` and the new
  `grown-retained-count-stale` through, and dropping the digest binding lets `grown-retained-digest-stale`
  through. Each new case is caught by exactly the binding it was written for.
  **The admissible case is the one this tree existed for.** `--self-test` now builds a 25-key retained
  set carrying a recorded reclamation, with `affected_chain_count` and the digest updated as a
  compliant refresh would update them, and requires it to be **ACCEPTED**. Restoring either frozen
  mechanism rejects it — which is the direct evidence that those two lines, and not the contract's
  design, were what made ADR 0025's mandated operations unreachable.
  `residual-actionability-contract: self-test 40/40 … RED cases pass, with 1/1 mandated retained-set
  operations admitted.`
  **`validate_canonical_recovery_contract.py` received the same relaxation and could not receive the
  same test, and that is reported rather than papered over.** Its `--self-test` cannot be extended
  because its BASELINE is already red: the script reports **10 errors** on the live tree, about witness
  key drift and a frozen population summary, none of them in this tree's scope. Measured identical
  **before and after** this change (10 errors, byte-identical diff), so the relaxation altered nothing
  in its verdict. Nothing invokes the script, so no gate has ever surfaced this. Routed to `.4`.
  Verification: `--self-test` 40/40 + 1/1 admissible; the three perturbations above each observed RED
  with the producer restored byte-identically; the frozen mechanisms restored once to observe the
  admissible case rejected; live `validate_residual_actionability_contract.py` PASS; recovery validator
  error set diffed pre/post as identical.
  Commit: `RETAINED-BUNDLE-POPULATION-FROZEN.1 — retire the freeze, keep the digest that was the real binding`

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

- ID: `RETAINED-BUNDLE-POPULATION-FROZEN.4`
  Status: `pending` (opened `2026-09-19` by `.1`)
  Goal: **decide the owner and disposition of `scripts/validate_canonical_recovery_contract.py`, which
  is red and invoked by nothing.** `.1` measured it at **10 errors** on the live tree — witness
  `evidence_ir`/`semantic_ir`/`intent_ir` keys differing from the frozen witness, witness scores no
  longer `3/0/1`, a downstream boundary no longer `3/3/0`, and a drifted frozen current-population
  summary. The error set is byte-identical before and after `.1`'s relaxation, so none of it belongs to
  this tree; it is simply invisible, because nothing in `scripts/check_doctrines.sh` or
  `scripts/run_ci.sh` runs the script.
  The decision is not "fix it": a contract that no gate runs and whose frozen witness has drifted may be
  **stale by design** (its boundary has moved on) or may be an **unregistered doctrine** that should be
  wired and made green. Those are different outcomes with different costs, and picking one needs the
  owner of the canonical-recovery boundary, not a guess from here.
  Acceptance: the script is either registered in the doctrine driver and green, or retired with a
  decision record saying what replaced it; either way no red-and-unrun validator remains in `scripts/`.
  Prerequisite: none. Blocks: nothing.
  Verification: `pending`
  Commit: `pending`

- ID: `RETAINED-BUNDLE-POPULATION-FROZEN.3`
  Status: `pending`
  Goal: **restore the three held-out gold bundles and close the interim position.** Move
  `generated/preserved/WIRE-BASED-100.10/{apb,ahb,axi}-normalized-bundle-held-out/` back to the normalized
  root each document's own SourceIR declares, declare all three in `retained_bundles.json`, and re-run the
  currency gate so their EvidenceIR replays become measurable.
  **WIDENED `2026-09-19` from APB-only to all three, by `CORPUS-CHAIN-CURRENCY.10a`.** That leaf was sent to
  re-ingest AXI, APB and AHB from PDF on the premise that they were unrebuildable, and found instead that
  `WIRE-BASED-100.9c`/`.9d` had held out AHB and AXI exactly as `.9b` held out APB, and that
  `WIRE-BASED-100.10` re-ingested and held out all three again on `2026-09-11`. So this leaf's interim
  position is not one document's cost; it is the whole gold trio, and it is what made a destructive
  re-ingest look necessary to another tree.
  **The restore is now evidenced rather than hoped, which changes this leaf's risk.** `.10a` measured each
  bundle replaying `evidence --dry-run` **CONTENT SAME** against the persisted EvidenceIR — APB 0.30 s, AHB
  0.67 s, AXI 3.28 s. Content identity is what makes the restore safe to perform: every downstream stage
  reads the persisted EvidenceIR, and an identical replay moves no downstream input, so no `WIRE-BASED-100`
  gold can move as a consequence of installing these bundles.
  Rollback for the restore: `.project-data/tmp/pre-reingest-snapshot-2026-09-19/` (21 files, 193,457,740
  bytes, digest `5a5dffa2865f67ad`), built by `CORPUS-CHAIN-CURRENCY.10` and re-censused `2026-09-19`. It
  covers the full source/evidence/semantic/intent chain for exactly these three documents, and `generated/`
  is git-ignored, so it is the only way back.
  Pre-flight: `CORPUS-CHAIN-CURRENCY.10b` ships `scripts/probe_held_out_bundle_replay.sh` so this leaf can
  confirm the answer before mutating the corpus instead of after.
  Acceptance: `check_chain_currency.sh` reports 27 retained bundles and 27 measurable EvidenceIR replays
  with zero stale; the book's retention census and `WIRE-BASED-100.9b`'s interim note are corrected where
  they were published.
  Prerequisite: `.2`.
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `RETAINED-BUNDLE-POPULATION-FROZEN.2` | `pending` | the substantive half and now the only thing between this tree and `.3`: mechanisms 1 and 2 are retired, so the behavioral set-equality join is the single remaining freeze. Needs the alignment tree's owner and a decision record |
| 2 | `RETAINED-BUNDLE-POPULATION-FROZEN.4` | `pending` | independent of `.2`/`.3`, and cheap to decide: a validator that is 10 errors red and invoked by nothing should not keep sitting in `scripts/` |
| — | `RETAINED-BUNDLE-POPULATION-FROZEN.1` | `done` | the literal and the `reclamations` freeze are gone; the count/digest binding they hid behind is proved stronger than what it replaced |
| 3 | `RETAINED-BUNDLE-POPULATION-FROZEN.3` | `pending` | restoration is only meaningful once both gates accept the 25th–27th keys; widened to all three golds `2026-09-19` by `CORPUS-CHAIN-CURRENCY.10a`, which found AHB and AXI held out alongside APB and measured all three replaying CONTENT SAME |

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

- **ANSWERED as far as this tree can answer it, and promoted to a leaf (`2026-09-19`, `.1`).** Nothing
  in `scripts/check_doctrines.sh` or `scripts/run_ci.sh` invokes
  `scripts/validate_canonical_recovery_contract.py`, and it is not "four further failures" but **10**,
  measured on the live tree and byte-identical before and after `.1`'s relaxation. `.1` did what its
  goal required — kept the script correct whether or not anything runs it — and the remaining question
  is ownership, not correctness. It is now `.4` rather than an open question, because a red validator
  nobody runs is a defect to dispose of, not a note to carry.

## Blockers

- None. `.2` needs the alignment tree's owner in the room, which is scheduling, not a blocker.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-10` | finding | `scripts/check_doctrines.sh` with the APB key declared | `PRODUCTION-GENERICITY` and `RESIDUAL-ACTIONABILITY` FAIL; with it undeclared and the bundle held out, all gate-tier doctrines PASS |
| `2026-09-19` | `.1` | `validate_residual_actionability_contract.py --self-test`; three perturbations, producer restored byte-identically after each | **40/40 RED + 1/1 admissible**. Dropping the count binding admits `partial-chain-set` and `grown-retained-count-stale`; dropping the digest binding admits `grown-retained-digest-stale`; restoring either frozen mechanism REJECTS the admissible 25-key-plus-reclamation case |
| `2026-09-19` | `.1` | `validate_canonical_recovery_contract.py`, error sets diffed pre/post | **10 errors both ways, identical** — the relaxation altered nothing in its verdict, and the failures are pre-existing, unowned, and invisible because nothing runs the script. Routed to `.4` |
| `2026-09-19` | `.3` scope | `specforge evidence --dry-run` + `compare_stage_artifact` per gold, each restored to pre-state | all three held-out bundles replay **CONTENT SAME** (APB 0.30 s, AHB 0.67 s, AXI 3.28 s); measured by `CORPUS-CHAIN-CURRENCY.10a` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| finding | `WIRE-BASED-100.9b — …` | opened by the leaf that hit the stop |
| `RETAINED-BUNDLE-POPULATION-FROZEN.1` | `RETAINED-BUNDLE-POPULATION-FROZEN.1 — retire the freeze, keep the digest that was the real binding` | mechanisms 1 and 2 retired in both validators; `.4` opened for the unowned red one |

## Changelog

- `2026-09-10`: Created. `WIRE-BASED-100.9b`'s APB re-ingest found that the retained normalized-bundle
  population is frozen against both operations ADR 0025 mandates, by a redundant size literal, a
  `reclamations` freeze, and a set-equality join to a frozen behavioral qualification.
- `2026-09-19`: `.3` widened from the APB bundle to all three wire-based gold bundles.
  `CORPUS-CHAIN-CURRENCY.10a` established that AHB and AXI are held out on the same terms as APB, that
  `WIRE-BASED-100.10` re-ingested and held out all three, and that each replays CONTENT SAME — so this
  tree's freeze is what made another tree plan a destructive re-ingest of the project's own golds.
- `2026-09-19`: `.1` closed. Two of the three mechanisms that froze the retained-bundle population are
  retired, and the count/digest binding that replaces them is proved stronger than the size literal it
  replaces — a digest catches a same-size substitution a size check never could. Only mechanism 3, the
  behavioral set-equality join, still stands between this tree and `.3`. `.4` opened for
  `validate_canonical_recovery_contract.py`, which `.1` measured at 10 errors and which nothing runs.
