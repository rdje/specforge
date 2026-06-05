# NLI-ENTAILMENT-VERIFIER: a semantic "does the source actually say this?" gate

## Metadata

- Tree ID: `NLI-ENTAILMENT-VERIFIER`
- Status: `active` (`.1` design; `.2` module; `.3` live wiring)
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

- ID: `NLI-ENTAILMENT-VERIFIER` · Status: `active` · Children: `.1` · `.2` · `.3`
- ID: `NLI-ENTAILMENT-VERIFIER.1` · Status: `done` · Goal: own + design (this file).
  Verification: passed (`2026-06-05`) — architecture fixed (reuse `call_text_provider` + the
  `SPECFORGE_VLM_HELPER` hermetic hook; `NliVerdict`; fail-closed parse; the additive gate
  semantics where `Unknown`→Abstain so an outage never breaks extraction; `qwen2.5:14b-instruct`
  validated viable). Docs-only.
- ID: `NLI-ENTAILMENT-VERIFIER.2` · Status: `pending` · Goal: implement the module + tests + book
  + KM.
- ID: `NLI-ENTAILMENT-VERIFIER.3` · Status: `pending` · Goal: live wiring + metric + close.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `NLI-ENTAILMENT-VERIFIER.1` | `done` | design + the validated model + hermetic-test plan |
| 2 | `NLI-ENTAILMENT-VERIFIER.2` | `pending` | the pure+mock-tested module |
| 3 | `NLI-ENTAILMENT-VERIFIER.3` | `pending` | live gate → residual; close |

## Decisions

- `2026-06-05`: entailment framing (not labeling); additive gate (`Unknown`→Abstain, never break
  extraction); reuse the existing text-provider transport + hermetic helper hook; text model
  (`qwen2.5:14b-instruct`), not the VLM.

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `NLI-ENTAILMENT-VERIFIER.1` | `NLI-ENTAILMENT-VERIFIER.1 — own + design the semantic entailment grounding gate` | docs-only |

## Changelog

- `2026-06-05`: Created — a semantic NLI entailment gate (premise=source, hypothesis=claim → keep
  only ENTAILED), built on the validated `qwen2.5:14b-instruct`, additive + fail-safe + hermetic.
