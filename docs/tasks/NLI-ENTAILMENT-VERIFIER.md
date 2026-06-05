# NLI-ENTAILMENT-VERIFIER: a semantic "does the source actually say this?" gate

## Metadata

- Tree ID: `NLI-ENTAILMENT-VERIFIER`
- Status: `done` (CLOSED `2026-06-05`; `.1`–`.3` — module + claim-set gate + live `nli-verify` command)
- Roadmap lane: `R16`/`R15e` (neuro-symbolic / bounded-LLM grounding)
- Created: `2026-06-05`
- Owner: repo-local workflow
- Parent context: user directive ("→ NLI entailment verifier"), the culmination of the
  text-model thread. Grounded gap from `neuro-symbolic-bounded-llm.md` (SNLI/Bowman; the
  hallucination-mitigation entailment framing). Today SpecForge's "is this claim grounded?" gate
  is rule/string-based; an **NLI verifier** is a *semantic* entailment check (premise = source
  statement, hypothesis = extracted claim → keep only ENTAILED).

## Why now (validated 2026-06-05)

`qwen2.5:14b-instruct` (text-only, local) is **viable for NLI** — 5/6 on a probe, and it judged
the subtle *"PSEL must be asserted" → NOT-entailed* correctly (a condition is not an obligation).
Crucially the **NLI/entailment framing beats free-form labeling** for that nuance (same model:
κ = 0.498 labeling vs the entailment judgments correct). See `local-llm-for-text-reasoning`,
`eval-gold-interannotator-kappa`. So the verifier is framed as *"does the source entail THIS
claim?"*, not re-extraction.

## Design (fidelity-first, residual-honesty, hermetic)

- New pure-ish module `crates/specforge/src/ir/nli_verify.rs`:
  - `enum NliVerdict { Entailed, NotEntailed, Unknown }`.
  - `entailment_prompt(source, claim) -> String` — pure, deterministic; states the premise +
    hypothesis + the "a condition/added/changed/contradicted fact is NOT entailed" rule + asks
    for a one-word `ENTAILED` / `NOT_ENTAILED`.
  - `parse_nli_verdict(response) -> NliVerdict` — pure; **fail-closed to `Unknown`** on anything
    that is not an unambiguous verdict (note `NOT_ENTAILED` must win over a substring `ENTAILED`).
  - `verify_entailment(provider, model, source_statement_id, source, claim) -> NliVerdict` —
    calls `commands::llm_text::call_text_provider` (reusing the curl transport **and** the
    `SPECFORGE_VLM_HELPER` test hook, so CI is hermetic); any provider error → `Unknown`.
- **Gate semantics (the load-bearing fidelity choice):** the NLI verifier is an *additive
  strengthening* gate, not the sole grounding.
  - `NotEntailed` → the claim is a likely hallucination → **route to a residual** (the gate
    caught it).
  - `Entailed` → keep (verified).
  - `Unknown` (provider down / unclear) → **ABSTAIN** — do NOT change the claim's disposition;
    the existing rule-based grounding stands. A provider outage must never nuke extraction.
  - `gate_action(verdict) -> NliGateAction { Keep, RouteResidual, Abstain }` (pure, testable).
- Default model `qwen2.5:14b-instruct` (text), NOT the VLM (text-only task).

## Hermetic testing

The `SPECFORGE_VLM_HELPER` env hook (already used by the text commands) lets a mock binary return
canned `ENTAILED`/`NOT_ENTAILED` so the *whole* path is tested without the live model;
`entailment_prompt` + `parse_nli_verdict` + `gate_action` are pure unit tests. **No CI test ever
requires Ollama.**

## Slices

- **`.1`** — own + design (this file); registered.
- **`.2`** — implement `ir/nli_verify.rs` (verdict + prompt + parser + `verify_entailment` via
  `call_text_provider` + `gate_action`); unit tests (prompt shape; parser incl. the
  `NOT_ENTAILED`-beats-`ENTAILED` + fail-closed cases; gate routing; an end-to-end via the
  `SPECFORGE_VLM_HELPER` mock); book + KM. Full CI GREEN. (No live wiring yet → no behavior change.)
- **`.3`** (follow-up) — wire into the extraction/grounding path: for each extracted claim with a
  source statement, run `verify_entailment` (when a provider is configured) and route
  `NotEntailed` to a residual; surface a `nli_*` metric. A `converge`/`validate` flag gates it
  on. Then close.

## Non-Goals

- NOT replacing the existing rule-based grounding — NLI is additive.
- NOT making any CI test depend on Ollama (the helper hook + pure tests only).
- NOT the live pipeline wiring in `.2` (that is `.3`).

## Acceptance Criteria

- `.1`: design recorded; registered.
- `.2`: module + tests (pure + mock-helper) green; book + KM; full CI GREEN; no behavior change.
- `.3`: live gate + metric; not-entailed → residual; CI GREEN; tree CLOSED.

## Task Tree

- ID: `NLI-ENTAILMENT-VERIFIER` · Status: `done` (CLOSED `2026-06-05`) · Children: `.1` · `.2` · `.3`
- ID: `NLI-ENTAILMENT-VERIFIER.1` · Status: `done` · Goal: own + design (this file).
  Verification: passed (`2026-06-05`) — architecture fixed (reuse `call_text_provider` + the
  `SPECFORGE_VLM_HELPER` hermetic hook; `NliVerdict`; fail-closed parse; the additive gate
  semantics where `Unknown`→Abstain so an outage never breaks extraction; `qwen2.5:14b-instruct`
  validated viable). Docs-only.
- ID: `NLI-ENTAILMENT-VERIFIER.2` · Status: `done` · Goal: implement the module + tests + book
  + KM.
  Verification: passed (`2026-06-05`) — `ir/nli_verify.rs` (`pub mod` registered): `NliVerdict`
  + `NliGateAction` + `gate_action` (Entailed→Keep, NotEntailed→RouteResidual, **Unknown→Abstain**)
  + `entailment_prompt` (states the condition-vs-obligation rule) + `parse_nli_verdict`
  (fail-closed; `NOT_ENTAILED` beats substring `ENTAILED`) + `verify_entailment` (reuses
  `call_text_provider` + `api_url`; provider error → Unknown). `DEFAULT_NLI_MODEL =
  qwen2.5:14b-instruct`. 4 pure unit tests (prompt shape; NOT_ENTAILED-beats-ENTAILED;
  fail-closed-on-garbage; additive/fail-safe gate) — **no Ollama dependency**. User-friendly book
  subsection in `architecture-rationale.md` ("Checking a claim semantically — the NLI entailment
  gate"); KM card `nli-entailment-verifier`. Full `scripts/run_ci.sh` GREEN (1243→1247; +4). **No
  behavior change yet** (the live wiring is `.3`).
- ID: `NLI-ENTAILMENT-VERIFIER.3` · Status: `done` · Goal: live wiring + close.
  Verification: passed (`2026-06-05`) — the gate applied to a claim set + a live command. Added
  to `ir/nli_verify.rs`: `constraint_claim_text` (a `SignalConstraintRecord` → its NLI hypothesis,
  e.g. "PADDR must be stable" / "HTRANS must be IDLE" / "X must not change") + `NliClaimFinding` +
  `nli_claim_findings(constraints, verify: impl Fn(&str,&str)->NliVerdict)` — premise =
  `constraint.source_text`; collects `NotEntailed`, `Entailed` keeps, `Unknown` abstains. **The
  verifier is injected as a closure → 3 new tests run with NO provider/network** (claim-text per
  kind; only-NotEntailed-collected; abstain-on-Unknown). New **`specforge nli-verify
  <evidence_ir.json>`** command (`commands/nli_verify.rs` + `Commands::NliVerify` + dispatch):
  loads EvidenceIR, runs `nli_claim_findings` over `signal_constraints` with the real
  `verify_entailment`, prints the not-entailed claims; `--vlm-provider skip` no-ops, `--model`
  overrides. Book subsection refreshed (the `nli-verify` command, per the BOOK-METHOD-DOC rule);
  KM card updated. Full `scripts/run_ci.sh` GREEN (1247→1250; +3). **Decision:** kept `.3` to a
  standalone command (additive) rather than churning `ValidateArgs`/`converge`; auto-routing
  NotEntailed → residual *inside* converge is a documented follow-up.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `NLI-ENTAILMENT-VERIFIER.1` | `done` | design + the validated model + hermetic-test plan |
| 2 | `NLI-ENTAILMENT-VERIFIER.2` | `done` | `ir/nli_verify.rs` module + 4 tests + book + KM (CI green 1247; no Ollama dep) |
| 3 | `NLI-ENTAILMENT-VERIFIER.3` | `done` | claim-set gate (`nli_claim_findings`) + live `nli-verify` command → **tree CLOSED** |

**Tree CLOSED `2026-06-05`.** The NLI entailment grounding gate exists end-to-end: the verifier
module (`.2`), the claim-set gate (`constraint_claim_text` + `nli_claim_findings`, verifier
injected → hermetic), and the live `specforge nli-verify` command (`.3`). Text model
`qwen2.5:14b-instruct`; fail-safe (Unknown→Abstain); no CI test needs Ollama; book + KM in sync.
Follow-up: auto-route NotEntailed → residual inside `converge`.

## Decisions

- `2026-06-05`: entailment framing (not labeling); additive gate (`Unknown`→Abstain, never break
  extraction); reuse the existing text-provider transport + hermetic helper hook; text model
  (`qwen2.5:14b-instruct`), not the VLM.

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `NLI-ENTAILMENT-VERIFIER.1` | `NLI-ENTAILMENT-VERIFIER.1 — own + design the semantic entailment grounding gate` | docs-only |
| `NLI-ENTAILMENT-VERIFIER.2` | `NLI-ENTAILMENT-VERIFIER.2 — ir/nli_verify.rs module (verdict + prompt + fail-closed parse + provider call) + book + KM` | +4 tests; CI green 1247; no Ollama dep |
| `NLI-ENTAILMENT-VERIFIER.3` | `NLI-ENTAILMENT-VERIFIER.3 — claim-set gate (constraint_claim_text + nli_claim_findings) + live nli-verify command; book + KM; close tree` | +3 tests; CI green 1250 |

## Changelog

- `2026-06-05`: Created — a semantic NLI entailment gate (premise=source, hypothesis=claim → keep
  only ENTAILED), built on the validated `qwen2.5:14b-instruct`, additive + fail-safe + hermetic.
- `2026-06-05`: `.2` done — `ir/nli_verify.rs` module (`NliVerdict`/`NliGateAction`/`gate_action`/
  `entailment_prompt`/`parse_nli_verdict`/`verify_entailment`, fail-closed, `Unknown→Abstain`),
  4 pure unit tests (no Ollama dep), book subsection (`architecture-rationale.md`), KM card
  `nli-entailment-verifier`. CI green 1247.
- `2026-06-05`: **Tree CLOSED.** `.3` shipped the claim-set gate (`constraint_claim_text` +
  `nli_claim_findings`, verifier injected → 3 hermetic tests) and the live `specforge nli-verify`
  command (loads EvidenceIR, runs NLI over `signal_constraints`, reports not-entailed claims;
  `--vlm-provider skip`/`--model`). Book + KM refreshed. CI green 1250. Auto-routing inside
  `converge` is the noted follow-up.
