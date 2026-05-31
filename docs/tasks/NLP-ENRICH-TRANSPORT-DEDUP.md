# NLP-ENRICH-TRANSPORT-DEDUP: complete the text-transport consolidation (route nlp_enrich through llm_text)

## Metadata

- Tree ID: `NLP-ENRICH-TRANSPORT-DEDUP`
- Status: `active`
- Roadmap lane: `R0` (code-quality / DRY debt paydown)
- Created: `2026-05-31`
- Last updated: `2026-05-31`
- Owner: repo-local workflow

## Goal

Finish the DRY consolidation that `LLM-TEXT-TRANSPORT-DEDUP` explicitly flagged
as a follow-up: route `commands/nlp_enrich.rs`'s `call_llm_for_sentence` through
the shared `commands/llm_text.rs::call_text_provider` and delete its duplicated
private `build_text_chat_request` / `extract_chat_content`. This takes the
OpenAI-compatible text transport from **2 copies → 1** (the last duplicate),
finishing the 3→2→1 reduction.

The fold must be **behavior-preserving**. The only behavioral difference between
the two transports today is `max_tokens`: `nlp_enrich` uses **256** (a
deliberate per-sentence cost choice — it scans every `NormativeStatement`),
while the shared `call_text_provider` uses **512** (its current callers
`extract-contracts` / `signal-resolve`). So the shared transport gains a
`max_tokens` parameter; the existing two callers pass `512`, `nlp_enrich` passes
`256`. No other path changes (helper-env hook shape, curl flags, OpenAI auth,
tempfile request, `extract_chat_content` parsing are already identical).

## Non-Goals

- NOT changing any prompt, classifier, response-parsing, or extraction outcome
  of `nlp-enrich`, `extract-contracts`, or `signal-resolve`.
- NOT changing `max_tokens` for any command (preserved exactly via the param).
- NOT touching `enrich` (image/VLM transport — a different, multimodal path).

## Acceptance Criteria

- `nlp_enrich::call_llm_for_sentence` acquires its raw response via
  `llm_text::call_text_provider` (with `max_tokens = 256`); its private
  `build_text_chat_request` / `extract_chat_content` are removed.
- `call_text_provider` takes a `max_tokens` parameter; `extract_contracts` +
  `signal_resolve` pass `512` (unchanged behavior).
- Existing `nlp_enrich` tests (mock via `SPECFORGE_VLM_HELPER`) pass unchanged;
  full `scripts/run_ci.sh` green.
- Book note: the `commands/quality-and-learning.md` shared-transport paragraph
  reflects that all four text/LLM commands now share one transport.

## Task Tree

- ID: `NLP-ENRICH-TRANSPORT-DEDUP`
  Status: `active`
  Goal: complete the text-transport consolidation
  Children: `.1`, `.2`

- ID: `NLP-ENRICH-TRANSPORT-DEDUP.1`
  Status: `done`
  Goal: own + design the behavior-preserving fold (this file); register in `docs/TASK_TREE.md`. Docs-only.
  Acceptance: tree created with the max_tokens-parameter design; registered; committed.
  Verification: pending
  Commit: `see Commit Log`

- ID: `NLP-ENRICH-TRANSPORT-DEDUP.2`
  Status: `pending`
  Goal: >
    Implement: add `max_tokens: usize` to `call_text_provider` (thread into
    `build_text_chat_request`); update `extract_contracts` + `signal_resolve` to
    pass `512`; route `nlp_enrich::call_llm_for_sentence` through it with `256`;
    remove `nlp_enrich`'s dead `build_text_chat_request` / `extract_chat_content`.
    Verify `nlp_enrich` mock tests unchanged; full CI green; book note; close.
  Acceptance: 2→1 transport copies; behavior-preserving; full `scripts/run_ci.sh` green; book updated; tree CLOSED.
  Verification: pending
  Commit: pending

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `NLP-ENRICH-TRANSPORT-DEDUP.1` | `done` | tree + design landed |
| 2 | `NLP-ENRICH-TRANSPORT-DEDUP.2` | `pending` | implement the behavior-preserving fold + verify + book + close — next |

## Decisions

- `2026-05-31`: preserve `nlp_enrich`'s `max_tokens=256` exactly by
  parameterizing the shared transport rather than unifying to 512 — keeps the
  consolidation provably behavior-neutral on the in-use production NLP-L3
  command (which the earlier transport-dedup deliberately did not touch).

## Open Questions

- None. (Considered unifying `max_tokens` to 512; rejected to keep zero behavior
  change on `nlp-enrich`.)

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-31` | `.1` | tree created (behavior-preserving max_tokens-param design); registered in `docs/TASK_TREE.md`; docs-only (CI invariant) | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `NLP-ENRICH-TRANSPORT-DEDUP.1` | `NLP-ENRICH-TRANSPORT-DEDUP.1 — own + design the nlp_enrich→llm_text transport fold` | docs-only |

## Changelog

- `2026-05-31`: Created — own + design the final text-transport consolidation
  (route `nlp_enrich` through `llm_text::call_text_provider`; 2→1 copies;
  behavior-preserving via a `max_tokens` parameter). Frontier → `.2` (implement).
