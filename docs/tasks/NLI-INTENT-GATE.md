# NLI-INTENT-GATE: make the NLI verifier an active IntentIR gate (demote NotEntailed → residual)

## Metadata

- Tree ID: `NLI-INTENT-GATE`
- Status: `done` (CLOSED `2026-06-05`; `.1` design + `.2` active gate + `intent --nli-verify`)
- Roadmap lane: `R16`/`R15e` (neuro-symbolic grounding)
- Created: `2026-06-05`
- Owner: repo-local workflow
- Parent context: user directive ("→ (b)") — the active-in-pipeline follow-up to
  `NLI-ENTAILMENT-VERIFIER`. Path (b): run the gate at the **IntentIR stage**, where
  `residual_decisions` already exist, instead of adding a residuals home to EvidenceIR.

## Design (fidelity-first, demote-not-delete, hermetic)

A **post-build pass** on a built `IntentIr` (no `IntentIr::build` signature change):

- `obligation_claim_text(&ActorContract) -> Option<String>` — render an obligation as an NLI
  hypothesis ONLY when we can phrase it faithfully: `Drive{signal,value}` → "`signal` must be
  `value`"; `Stable{signal,..}` → "`signal` must be stable"; `Eventually{Level/Edge signal,
  Within max}` → "`signal` must occur within `max` cycles"; `HandshakeBarrier{v,r}` → "the
  `v`/`r` handshake must complete"; `Mutex{a,b}` → "`a` and `b` are mutually exclusive". The
  rest (`Observe`, `Persist`, `Sequence`, `OrderedBefore`, non-signal `Eventually`) → `None`
  (**not gated** — never NLI-check a claim we cannot state cleanly).
- `nli_gate_contracts(contracts, verify: impl Fn(&str,&str)->NliVerdict) -> (kept, new_residuals)`
  — premise = `contract.provenance.source_text`, hypothesis = `obligation_claim_text`. A
  `NotEntailed` contract is **demoted**: removed from `actor_contracts`, added to
  `residual_decisions` as a `ResidualDecisionPacket` (`packet_id = nli_unentailed_<id>`,
  `why_unresolved` names the claim). `Entailed`/`Unknown`/un-renderable → **kept** (the existing
  pipeline stands). Verifier injected → fully testable, no provider.
- `apply_nli_gate(&mut IntentIr, verify)` — thin wrapper (partition contracts, extend residuals).
- **Demote, never delete** — a NotEntailed claim becomes a residual decision (preserved for
  review), so a verifier error costs a review item, not a lost fact. Opt-in flag (LLM calls).

## Wiring

`intent` command gains `--nli-verify` (+ `--vlm-provider` / `--model`): after `IntentIr::build`,
if set and the provider isn't `skip`, call `apply_nli_gate` with a real `verify_entailment`
closure before writing. Default off. No CI test needs Ollama (the gate logic is verifier-injected;
the flag path is thin glue).

## Acceptance Criteria

- `.1`: design (this file); registered.
- `.2`: `obligation_claim_text` + `nli_gate_contracts` + `apply_nli_gate` (verifier injected) +
  unit tests (claim render per obligation incl. `None` cases; NotEntailed→demoted-to-residual;
  Entailed/Unknown/un-renderable→kept); `intent --nli-verify` flag wiring; book subsection +
  KM; full CI GREEN; tree CLOSED.

## Task Tree

- ID: `NLI-INTENT-GATE` · Status: `done` (CLOSED `2026-06-05`) · Children: `.1` · `.2`
- ID: `NLI-INTENT-GATE.1` · Status: `done` · Goal: own + design (this file).
  Verification: passed (`2026-06-05`) — verified `IntentIr` has `actor_contracts` (with
  `provenance.source_text` + `obligation`) and `residual_decisions`; designed the post-build
  demote-not-delete pass with `obligation_claim_text` (Option → only gate phrasable obligations)
  and an injected verifier (hermetic); the `intent --nli-verify` opt-in flag. Docs-only.
- ID: `NLI-INTENT-GATE.2` · Status: `done` · Goal: implement + wire + tests + book + KM + close.
  Verification: passed (`2026-06-05`) — added to `ir/nli_verify.rs`: `obligation_claim_text`
  (phrasable obligations → claim; `Observe`/`Persist`/`Sequence`/`OrderedBefore`/non-signal
  `Eventually` → `None`/not-gated), `nli_gate_contracts(contracts, verify)` (premise =
  `provenance.source_text`; `NotEntailed` → demoted to a `ResidualDecisionPacket`
  `nli_unentailed_<id>`; Entailed/Unknown/un-phrasable kept), `apply_nli_gate(&mut IntentIr)`.
  Verifier **injected** → 3 hermetic tests (claim render incl. `None`; demote-to-residual;
  keep-un-phrasable/Unknown). Wired opt-in **`intent --nli-verify`** (`--vlm-provider`/`--model`;
  one `rescan_plan` ctor site updated to `Skip`). Book subsection refreshed (active gate, per the
  BOOK-METHOD-DOC rule); KM card `nli-intent-gate`. Full `scripts/run_ci.sh` GREEN (1250→1253; +3).
  Demote-not-delete: a wrong verdict costs a review item, not a lost fact.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `NLI-INTENT-GATE.1` | `done` | design (post-build demote-to-residual; phrasable-only; injected verifier) |
| 2 | `NLI-INTENT-GATE.2` | `done` | gate + `intent --nli-verify` + 3 tests + book + KM → **tree CLOSED** |

**Tree CLOSED `2026-06-05`.** The NLI verifier is now an *active* IntentIR gate: `intent
--nli-verify` demotes contracts whose source sentence doesn't entail them into `residual_decisions`
(demote-not-delete, phrasable-only, opt-in, hermetic). CI green 1253; book + KM in sync.

## Decisions

- `2026-06-05`: gate at the IntentIR stage (residuals exist there) as a post-build pass;
  demote-not-delete (NotEntailed → residual decision); only gate phrasable obligations; opt-in
  flag; verifier injected for hermetic tests.

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `NLI-INTENT-GATE.1` | `NLI-INTENT-GATE.1 — own + design the active IntentIR NLI gate (demote to residual)` | docs-only |
| `NLI-INTENT-GATE.2` | `NLI-INTENT-GATE.2 — active IntentIR NLI gate (obligation_claim_text + nli_gate_contracts + apply_nli_gate) + intent --nli-verify; book + KM; close` | +3 tests; CI green 1253 |

## Changelog

- `2026-06-05`: Created — make the NLI verifier an *active* IntentIR gate (path b): demote
  NotEntailed contracts into `residual_decisions`, demote-not-delete, phrasable-only, opt-in.
- `2026-06-05`: **Tree CLOSED.** `.2` shipped `obligation_claim_text` + `nli_gate_contracts` +
  `apply_nli_gate` (verifier injected → 3 hermetic tests) and the opt-in `intent --nli-verify`
  flag; book + KM refreshed. CI green 1253.
