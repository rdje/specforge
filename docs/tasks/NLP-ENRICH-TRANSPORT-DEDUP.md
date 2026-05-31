# NLP-ENRICH-TRANSPORT-DEDUP: complete the text-transport consolidation (route nlp_enrich through llm_text)

## Metadata

- Tree ID: `NLP-ENRICH-TRANSPORT-DEDUP`
- Status: `done`
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
  Status: `done`
  Goal: complete the text-transport consolidation
  Children: `.1`, `.2`

- ID: `NLP-ENRICH-TRANSPORT-DEDUP.1`
  Status: `done`
  Goal: own + design the behavior-preserving fold (this file); register in `docs/TASK_TREE.md`. Docs-only.
  Acceptance: tree created with the max_tokens-parameter design; registered; committed.
  Verification: pending
  Commit: `see Commit Log`

- ID: `NLP-ENRICH-TRANSPORT-DEDUP.2`
  Status: `done`
  Goal: >
    Implement: add `max_tokens: usize` to `call_text_provider` (thread into
    `build_text_chat_request`); update `extract_contracts` + `signal_resolve` to
    pass `512`; route `nlp_enrich::call_llm_for_sentence` through it with `256`;
    remove `nlp_enrich`'s dead `build_text_chat_request` / `extract_chat_content`.
    Verify `nlp_enrich` mock tests unchanged; full CI green; book note; close.
  Acceptance: 2→1 transport copies; behavior-preserving; full `scripts/run_ci.sh` green; book updated; tree CLOSED.
  Verification: >
    passed (`2026-05-31`) — `llm_text::call_text_provider` gained a
    `max_tokens: usize` param (threaded into `build_text_chat_request`);
    `extract_contracts` + `signal_resolve` pass `512` (unchanged);
    `nlp_enrich::call_llm_for_sentence` now delegates to it with a
    `NLP_MAX_TOKENS = 256` const (exact prior budget preserved), keeping only its
    `build_nlp_prompt` + `parse_nlp_response`; deleted `nlp_enrich`'s duplicated
    `build_text_chat_request` / `extract_chat_content` and the now-unused
    `std::fs` / `std::process::Command` imports; `cfg(test)`-gated the
    test-only `VLM_HELPER_ENV` const. Text transport is now **2→1** (the last
    duplicate). All 22 `commands::nlp_enrich` tests pass (incl. the
    `SPECFORGE_VLM_HELPER` mock-helper end-to-end paths) — behavior preserved.
    `cargo clippy --all-targets` clean; full `scripts/run_ci.sh` GREEN (fmt /
    clippy -D / lib tests / rustdoc -D / mdBook). Added a `max_tokens` request-
    body unit test in `llm_text`. **Residual-honesty catch:** corrected a book
    overclaim — `enrich` is the image/VLM command and does NOT share the
    `llm_text` text transport (it keeps its own base64-image transport); it only
    shares the `SPECFORGE_VLM_HELPER` hook + provider-flag convention. The book's
    shared-transport note now scopes the unified transport to the **three text
    commands** and footnotes `enrich` accurately.
  Commit: `see Commit Log`

## Current Frontier

**Tree CLOSED `2026-05-31`** — the OpenAI-compatible text transport is now a
single copy (`llm_text::call_text_provider`), shared by the three text commands;
`nlp_enrich`'s duplicate is gone; behavior preserved; full CI green.

| Order | Leaf | Status | Why |
| --- | --- | --- | --- |
| 1 | `NLP-ENRICH-TRANSPORT-DEDUP.1` | `done` | tree + design landed |
| 2 | `NLP-ENRICH-TRANSPORT-DEDUP.2` | `done` | fold implemented + verified (22 tests) + book + closed |

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
| `2026-05-31` | `.2` | `call_text_provider` gained `max_tokens` param; `nlp_enrich` routed through it (256) + dead `build_text_chat_request`/`extract_chat_content` + unused `fs`/`Command` imports removed; 2→1 transport copies; 22 nlp_enrich tests pass (behavior preserved); +1 llm_text unit test; book overclaim re: `enrich` corrected; full `scripts/run_ci.sh` GREEN | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `NLP-ENRICH-TRANSPORT-DEDUP.1` | `NLP-ENRICH-TRANSPORT-DEDUP.1 — own + design the nlp_enrich→llm_text transport fold` | docs-only |
| `NLP-ENRICH-TRANSPORT-DEDUP.2` | `NLP-ENRICH-TRANSPORT-DEDUP.2 — route nlp_enrich through shared llm_text transport (2→1); close` | code + book; behavior-preserving; CI green |

## Changelog

- `2026-05-31`: `.2` — implemented the fold (route `nlp_enrich` through
  `llm_text::call_text_provider`, `max_tokens` parameterized so nlp_enrich keeps
  256; removed the duplicated transport + dead imports). 2→1 transport copies;
  22 nlp_enrich tests pass (behavior preserved); corrected a book overclaim re:
  `enrich` (image transport, not shared). Full CI green. **Tree CLOSED.**
- `2026-05-31`: Created — own + design the final text-transport consolidation
  (route `nlp_enrich` through `llm_text::call_text_provider`; 2→1 copies;
  behavior-preserving via a `max_tokens` parameter). Frontier → `.2` (implement).
