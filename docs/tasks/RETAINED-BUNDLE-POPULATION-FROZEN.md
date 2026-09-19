# RETAINED-BUNDLE-POPULATION-FROZEN: the retained normalized-bundle set can neither grow nor shrink

## Metadata

- Tree ID: `RETAINED-BUNDLE-POPULATION-FROZEN`
- Status: `active`
- Roadmap lane: `R15e`/`R16` (corpus currency / doctrine enforcement)
- Created: `2026-09-10`
- Last updated: `2026-09-19`
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
  Status: `done` (`2026-09-19`, CODE/DOC)
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
  **SCOPED `2026-09-19` by `.4`, and it is larger than the one-line framing above.** Two findings, both
  from reading the qualification rather than the checker:
  **(a) The candidate shape is the right one, and it is the same class of error ADR 0049 just named.**
  `held_out_policy.prospective_definition` defines the population as the documents held out *"at the
  selection boundary"*, and the contract pins `selection_boundary_commit`. It is a **snapshot at a
  commit**, not a live invariant — so requiring the live `retained` set to equal it forever conflates
  "the population we qualified" with "the set we currently retain". ADR 0049 retired a pre-repair
  contract for exactly this confusion; here the remedy is subset-plus-report rather than retirement,
  because the frozen selection still carries real evidence and only its *equality* is wrong.
  **(b) It is not one comparison.** The set-equality join at
  `scripts/check_behavioral_genericity_contract.py` (~line 1392) is the visible half. The contract also
  pins `population_assertions.current_documents: 24` and a `frozen_census`, and those are frozen
  boundary values too. Each needs adjudicating individually — which stay exact because they are
  release evidence, which become subset-or-reported because they describe a moving set — and that is a
  design question, not an edit.
  **What is NOT a blocker any more.** This leaf was recorded as needing "the alignment tree's owner in
  the room". `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.v` is **`done`** — the signoff closed, so there is no
  pending owner to wait for. What remains is the adjudication in (b), and a decision record for it.
  Prerequisite: `.1` (done).

  **SHIPPED as ADR 0050: subset floor plus declared residual.** The frozen population must remain a
  SUBSET of live `retained` (a frozen row that is no longer retained is RED, at the same strength
  equality gave it), and every retained key above that floor must be declared in the new
  `doctrine/production_genericity/post_boundary_retention.json` naming the held-out relations it owes
  and the leaf that owes them. An undeclared key above the floor is RED: growth is admitted, silence is
  not. The gate now reports the debt on every run — `… ; 0 retained post-boundary and unqualified`.

  **`.4`'s scoping finding (b) was re-derived and is CORRECTED, which shrank this slice.** `.4` recorded
  that `population_assertions.current_documents: 24` and `frozen_census` are frozen boundary values
  joined the same way and each needing its own adjudication. **Measurement says otherwise: it is one
  comparison after all.** Declaring the three gold keys (`retained` 24 → 27) with the frozen TSV
  untouched produces **exactly one** problem — the set-equality join. Both `current_documents` and
  `frozen_census.aggregate.documents` stay **green**, because both are compared against `len(rows)` from
  `behavioral_population.tsv`, not against `retained`. They are frozen values checked against the frozen
  thing they describe, which is self-consistent and correct. **Adjudicated: they stay exact**, and
  relaxing them would have weakened a binding that was never the problem. `.4`'s finding (a) — that this
  is ADR 0049's class of error with subset-plus-report as the remedy — was confirmed and is the shipped
  design.

  **The declaration deliberately does NOT live in `behavioral_qualification.json`, and the reason is
  measured.** That was the first implementation, and it was reverted. The contract's digest is pinned by
  `behavioral_holdout_evidence.json` at the top level **and in each of the 35 completed held-out attempt
  identities**, so amending it re-pins **36** digests — and `unqualified_keys` GROWS with every
  retention, so that cost recurs forever and each recurrence rewrites the identity records of a closed
  qualification. That is ADR 0050's own error one level down: a moving set inside a frozen artifact. The
  declaration therefore lives in its own live file, referenced by the checker as a module constant
  rather than through the digest-pinned `declarations` map. **`behavioral_qualification.json` is
  byte-identical and the change costs zero re-pins.** Recorded as a corollary in the ADR.
  (`BEHAVIORAL_TOOL_PATH` pins the Rust conformance harness, not this checker, so editing the checker
  moves no digest either.)

  **What `.3` now owes, measured rather than assumed.** At 27 retained:
  `validate_residual_actionability_contract.py` reports two problems, both the designed count/digest
  binding `.1` installed (`affected_chain_count` and `affected_chain_ids_sha256` must be updated to the
  new membership — bookkeeping, not a freeze); `check_corpus_frontier_census.pl` is unaffected; and the
  behavioral gate is **green**.
  Verification: end-to-end on the real files — `retained` grown to 27 with the three golds declared runs
  `check_behavioral_genericity_contract.py` **green**, reporting `3 retained post-boundary and
  unqualified`; both files restored to their committed digests afterwards. `--self-test` **23/23 RED +
  1/1 admissible** (6 new boundary cases: vanished frozen key, unreported post-boundary key, declaration
  for a non-retained key, qualified key declared unqualified, declaration without an owing leaf,
  declaration owing an ineligible relation). Three perturbations with byte-identical restore: dropping
  the subset-floor check MISSES `vanished frozen key`; dropping the undeclared-excess check MISSES
  `unreported post-boundary key`; **restoring the original set-equality join REJECTS the admissible
  grown set, and the equality is the only problem it reports** — the direct evidence that the equality,
  and not the contract's design, was what made ADR 0025's mandated operation unreachable.
  Commit: `RETAINED-BUNDLE-POPULATION-FROZEN.2 — a frozen population is a floor, not a fence`

- ID: `RETAINED-BUNDLE-POPULATION-FROZEN.4`
  Status: `done` (`2026-09-19`, CODE/DOC)
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
  **DECIDED: retired, and the reason inverts the question.** The script is red **because the repair it
  was gating succeeded.** The contract froze two key sets side by side — `current_canonical_keys`, what
  the pipeline produced at freeze time, and `expected_canonical_keys`, what it should produce once
  repaired — and the live tree now matches the **expected** column.

  | | frozen `current` | frozen `expected` | live |
  | --- | --- | --- | --- |
  | witness canonical keys | 3, no `PSEL\|must_be_asserted` | 4, with it | **4, with it** |
  | intent TP / FN | 39 / 1 | — | **40 / 0** |
  | canonical provenance | 42/42 | — | **45/45** |
  | conservation | 117/117 | — | **120/120** |

  So the two obvious repairs are both wrong, and for different reasons. **Wiring it** would gate the
  repository on the APB canonical loss *still being present* — a green build would require the defect.
  **Regenerating its witness** would overwrite `SPEC-TO-INTENT-ALIGNMENT.7a`'s completion evidence with
  post-repair values; a freeze whose witness is refreshed whenever it disagrees with production is not
  a freeze, it is a mirror.
  **What replaced it, and it already runs in CI:** `crates/specforge/src/ir/evidence.rs`,
  `mod canonical_inference_antecedent_recovery` — **6 passing tests**, including
  `frozen_contract_matrix_executes_against_the_production_sibling` and
  `full_build_recovers_the_exact_witness_without_borrowing_the_consequence`. The frozen matrix is still
  executed against production; only the redundant, unrun, misleading checker is gone.
  `doctrine/spec_to_intent/canonical_recovery_contract.json` **stays** — it is `7a`'s evidence and the
  Rust tests' input. Recorded as **ADR 0049**, whose general rule is that a contract frozen *before* a
  repair is a pre-repair snapshot, must go red when the repair lands, and has exactly two honest end
  states: migrate its obligations into a test that runs against production, or retire it with the
  reason recorded.
  **A second defect fell out of it.** `docs/knowledge/inference-antecedent-state-loss.md` was
  `status: current` and asserted the false negative as present, with a `reverify` command that ran the
  now-deleted checker — a Knowledge Map card that would have sent the next session to re-derive a
  closed defect from a red command. Corrected in the same slice.
  Prerequisite: none. Blocks: nothing.
  Verification: live snapshot compared field-by-field against both frozen columns; `git log -S` over
  `check_doctrines.sh` and `run_ci.sh` empty, proving it was never registered;
  `cargo test -p specforge-core --lib canonical_inference_antecedent_recovery` **6 passed** before and
  after the deletion; `validate_residual_actionability_contract.py` PASS after it.
  Commit: `RETAINED-BUNDLE-POPULATION-FROZEN.4 — the validator is red because the repair landed`

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
  Prerequisite: `.2` — **satisfied `2026-09-19`; this leaf is unblocked and is the tree's last.**
  **Its exact remaining obligation is measured, not assumed.** With `retained` at 27:
  `check_behavioral_genericity_contract.py` is **green** provided the three keys are declared in
  `doctrine/production_genericity/post_boundary_retention.json` (each naming its owed relations and
  `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii` as the owing leaf);
  `validate_residual_actionability_contract.py` reports two problems, both the count/digest binding
  `.1` installed — `reconciliation.affected_chain_count` must become 27 and
  `affected_chain_ids_sha256` must be recomputed over the new membership;
  `check_corpus_frontier_census.pl` is unaffected. `check_chain_currency.sh` is the gate that must then
  turn the three EvidenceIR replays from UNMEASURABLE to measurable.
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `RETAINED-BUNDLE-POPULATION-FROZEN.3` | `pending` | **unblocked** — all three freeze mechanisms are retired, and the 27-key end state is measured green on the behavioral gate. Restore the three bundles, declare them, update the residual-actionability count/digest, re-run the currency gate |
| — | `RETAINED-BUNDLE-POPULATION-FROZEN.2` | `done` | ADR 0050: the frozen population is a subset floor, the excess is declared debt in its own live file, and the gate reports it instead of blocking. `.4`'s "not one comparison" scoping was re-derived and corrected — it was one comparison |
| — | `RETAINED-BUNDLE-POPULATION-FROZEN.4` | `done` | retired: it was red because the repair landed, and its coverage already runs as 6 Rust tests (ADR 0049) |
| — | `RETAINED-BUNDLE-POPULATION-FROZEN.1` | `done` | the literal and the `reclamations` freeze are gone; the count/digest binding they hid behind is proved stronger than what it replaced |

## Decisions

- `2026-09-10`: **Hold, do not reclaim, and do not force the declaration.** `WIRE-BASED-100.9b`
  preserved the APB bundle on the repository volume rather than deleting it, and left the declaration
  untouched. Reason: a wire-protocol leaf must not enlarge a frozen, in-flight, release-blocking
  qualification population as a side effect of re-ingesting one document, and must not delete evidence
  the current binary cannot regenerate. The hold costs one document's EvidenceIR replay measurability
  and nothing else; it is reversed by `.3`.
- `2026-09-19`: **A frozen population is a floor, not a fence (ADR 0050).** The behavioral population is
  a snapshot at `selection_boundary_commit`; the retained set is required by ADR 0025 to move. Equality
  between them conflated "a qualified row vanished" (a real defect) with "a key was retained after the
  boundary" (unqualified debt, not drift). The floor stays gated at full strength; the excess is
  declared with its owed relations and owing leaf, and reported on every run.
- `2026-09-19`: **A declaration that grows does not belong in an artifact that is frozen.** The
  `unqualified_keys` list was first placed inside `behavioral_qualification.json` and reverted after
  measuring the cost: 36 digest re-pins per amendment (top level plus 35 held-out attempt identities),
  recurring on every future retention, each one rewriting a closed qualification's identity records. It
  lives in `doctrine/production_genericity/post_boundary_retention.json` instead, at zero re-pin cost.
- `2026-09-19`: **`population_assertions.current_documents` and `frozen_census` stay exact.** Measured:
  both compare against `len(rows)` from the frozen TSV, not against `retained`, so neither moves when
  the retained set grows. They were never part of the freeze.
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

- None. All three freeze mechanisms are retired; `.3` is unblocked and is the last leaf.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-10` | finding | `scripts/check_doctrines.sh` with the APB key declared | `PRODUCTION-GENERICITY` and `RESIDUAL-ACTIONABILITY` FAIL; with it undeclared and the bundle held out, all gate-tier doctrines PASS |
| `2026-09-19` | `.1` | `validate_residual_actionability_contract.py --self-test`; three perturbations, producer restored byte-identically after each | **40/40 RED + 1/1 admissible**. Dropping the count binding admits `partial-chain-set` and `grown-retained-count-stale`; dropping the digest binding admits `grown-retained-digest-stale`; restoring either frozen mechanism REJECTS the admissible 25-key-plus-reclamation case |
| `2026-09-19` | `.1` | `validate_canonical_recovery_contract.py`, error sets diffed pre/post | **10 errors both ways, identical** — the relaxation altered nothing in its verdict, and the failures are pre-existing, unowned, and invisible because nothing runs the script. Routed to `.4` |
| `2026-09-19` | `.4` | live snapshot vs both frozen columns; `git log -S` over the driver and CI; `cargo test … canonical_inference_antecedent_recovery` | live matches the frozen **expected** column (TP 39→40, FN 1→0, provenance 42→45, conservation 117→120); the script was **never** registered; the frozen matrix runs as **6 passing Rust tests**. Retired per ADR 0049 |
| `2026-09-19` | `.3` scope | `specforge evidence --dry-run` + `compare_stage_artifact` per gold, each restored to pre-state | all three held-out bundles replay **CONTENT SAME** (APB 0.30 s, AHB 0.67 s, AXI 3.28 s); measured by `CORPUS-CHAIN-CURRENCY.10a` |
| `2026-09-19` | `.2` | `retained` perturbed 24 → 27 with the frozen TSV untouched, restored byte-identically | **exactly one** problem — the set-equality join. `population_assertions.current_documents` and `frozen_census.aggregate.documents` stay **green**, correcting `.4`'s "not one comparison" scoping |
| `2026-09-19` | `.2` | `check_behavioral_genericity_contract.py --self-test` | **23/23 RED cases pass, 1/1 mandated retained-set growth admitted** (6 new boundary cases) |
| `2026-09-19` | `.2` | three perturbations of the checker, producer restored byte-identically after each (`95ef0bbf…`) | dropping the subset-floor check MISSES `vanished frozen key`; dropping the undeclared-excess check MISSES `unreported post-boundary key`; **restoring the set-equality join REJECTS the admissible grown set with the equality as its only problem** |
| `2026-09-19` | `.2` | end-to-end on the real files: `retained` = 27 with the three golds declared, then restored to committed digests | behavioral gate **green**, reporting `3 retained post-boundary and unqualified`. `validate_residual_actionability_contract.py` reports 2 problems, both the designed count/digest binding (`.3`'s bookkeeping); `check_corpus_frontier_census.pl` unaffected |
| `2026-09-19` | `.2` | committed vs working `behavioral_qualification.json` compared key-by-key | **byte-identical** — the declaration was rehomed out of the frozen contract, so the change costs **zero** of the 36 digest re-pins an amendment would have forced |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| finding | `WIRE-BASED-100.9b — …` | opened by the leaf that hit the stop |
| `RETAINED-BUNDLE-POPULATION-FROZEN.1` | `RETAINED-BUNDLE-POPULATION-FROZEN.1 — retire the freeze, keep the digest that was the real binding` | mechanisms 1 and 2 retired in both validators; `.4` opened for the unowned red one |
| `RETAINED-BUNDLE-POPULATION-FROZEN.4` | `RETAINED-BUNDLE-POPULATION-FROZEN.4 — the validator is red because the repair landed` | retired per ADR 0049; coverage already runs as 6 Rust tests |
| `RETAINED-BUNDLE-POPULATION-FROZEN.2` | `RETAINED-BUNDLE-POPULATION-FROZEN.2 — a frozen population is a floor, not a fence` | mechanism 3 retired per ADR 0050; the last freeze is gone and `.3` is unblocked |

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
- `2026-09-19`: `.4` closed. `validate_canonical_recovery_contract.py` is retired, not wired and not
  regenerated: it was red because the repair it froze had landed, it was never registered with any
  gate, and its frozen matrix already executes against production as 6 Rust tests. ADR 0049 records the
  general rule. A stale `status: current` fact card asserting the closed defect, with a `reverify` that
  ran the deleted checker, was corrected in the same slice.
- `2026-09-19`: **`.2` closed — the last of the three freeze mechanisms is retired.** ADR 0050 replaces
  the behavioral set-equality join with a **subset floor plus declared residual**: the frozen population
  must stay a subset of `retained`, and every retained key above it is declared in
  `doctrine/production_genericity/post_boundary_retention.json` with the relations it owes and the leaf
  that owes them. Undeclared growth is RED; declared growth is green and reported on the gate's summary
  line. Measured end state: 27 retained with the three golds declared is **green**. Two findings worth
  keeping. `.4`'s scoping said this was "not one comparison" and that `current_documents` and
  `frozen_census` each needed adjudicating — **re-derivation shows it was one comparison**; both are
  compared against the frozen TSV, not against `retained`, and they stay exact. And the declaration was
  first written into `behavioral_qualification.json` and **reverted**: that contract's digest is pinned
  by 35 held-out attempt identities, so housing a list that grows with every retention there would cost
  36 re-pins per retention and rewrite a closed qualification's records — ADR 0050's own error one level
  down. Rehomed to its own live file at zero re-pin cost.
- `2026-09-19`: `.2` scoped by `.4` without being started. The behavioral qualification defines its
  population *at a selection boundary* and pins that commit, so it is a snapshot wired as a live
  equality invariant — ADR 0049's class of error, with subset-plus-report as the remedy instead of
  retirement. It is not one comparison: `population_assertions.current_documents: 24` and
  `frozen_census` are frozen boundary values too and each needs its own adjudication. No longer blocked
  on an owner; `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.v` is closed.
