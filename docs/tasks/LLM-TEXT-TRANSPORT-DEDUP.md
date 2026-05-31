# LLM-TEXT-TRANSPORT-DEDUP: consolidate the duplicated text chat transport (extract-contracts + signal-resolve)

## Metadata

- Tree ID: `LLM-TEXT-TRANSPORT-DEDUP`
- Status: `done`
- Roadmap lane: `R0` (code-quality / infra)
- Created: `2026-05-31`
- Last updated: `2026-05-31`
- Owner: repo-local workflow

## Goal

Pay down the duplication introduced (and flagged in-module) by
`CVE-PROSE-EXTRACTION.3` and `R14-SIGNAL-RESOLVE.2`: both
`commands/extract_contracts.rs` and `commands/signal_resolve.rs` copy the same
OpenAI-compatible text-chat transport from `nlp_enrich` — provider→endpoint/
model selection, the `SPECFORGE_VLM_HELPER` test hook, the curl request build,
and the response-content extraction. Extract one shared
`commands/llm_text.rs` helper and route both new commands through it.

## Non-Goals

- Does NOT refactor the in-use `nlp_enrich` command onto the shared helper
  (it has its own inline copy + a different result type; touching it is higher
  risk while it is the command the running converge jobs exercise). Recorded
  as a follow-up. This tree takes the duplication from 3 copies → 2 (the two
  new commands now share one), not all the way to 1.
- No behavior change: the prompts (`build_contract_prompt` /
  `build_relation_prompt`) and the pure classifiers stay command-specific and
  untouched; only the transport plumbing is shared.

## Acceptance Criteria

- New `commands/llm_text.rs` with `pub(crate)` `provider_name`,
  `default_model`, `api_url`, `call_text_provider` (hook + curl) + private
  request-build / content-extract; `VLM_HELPER_ENV` lives there.
- `extract_contracts` + `signal_resolve` delete their copies and call the
  shared helper; their command-specific prompt builders + pure classifiers are
  unchanged.
- Behavior identical: the existing pure-classifier unit tests still pass
  unchanged; `extract-contracts`/`signal-resolve` `--provider skip` still load
  + select candidates identically; full `scripts/run_ci.sh` green.

## Task Tree

- ID: `LLM-TEXT-TRANSPORT-DEDUP`
  Status: `done`
  Goal: consolidate the duplicated text chat transport
  Children: `.1`

- ID: `LLM-TEXT-TRANSPORT-DEDUP.1`
  Status: `done`
  Goal: >
    Add `commands/llm_text.rs` (shared transport) + route
    `extract_contracts` and `signal_resolve` through it (delete their
    duplicated `provider_name`/`default_model`/`api_url`/
    `build_text_chat_request`/`extract_chat_content`/`call_provider_for_*`);
    keep prompts + classifiers command-specific. A focused unit test for the
    shared request-shape + content-extract.
  Acceptance: 3→2 transport copies; no behavior change; existing tests green; `scripts/run_ci.sh` green.
  Verification: >
    passed (`2026-05-31`) — added `commands/llm_text.rs` (`pub(crate)`
    `provider_name`/`default_model`/`api_url`/`call_text_provider` + private
    `build_text_chat_request`/`extract_chat_content` + `VLM_HELPER_ENV`; 3 new
    transport unit tests — request-JSON shape/escaping, string+array content
    extraction, fails-closed on garbage/no-choices). `extract_contracts` +
    `signal_resolve` deleted their duplicated copies and route through it;
    their command-specific prompts (`build_contract_prompt` /
    `build_relation_prompt`) + pure classifiers (`classify_response` /
    `classify_relation_response`) + all their unit tests are unchanged. 3→2
    transport copies (`nlp_enrich` inline left as a tracked follow-up). No
    behavior change: existing classifier tests green; `--provider skip` still
    selects candidates identically (re-verified). Full `scripts/run_ci.sh`
    green.
  Commit: `see Commit Log`

## Current Frontier

**Tree CLOSED `2026-05-31`** — the single refactor leaf is `done`.

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `LLM-TEXT-TRANSPORT-DEDUP.1` | `done` | shared `llm_text` helper landed; 3→2 transport copies; CI green |

## Decisions

- `2026-05-31`: scoped to the two NEW commands only (leave `nlp_enrich`
  inline) — bounded + zero-risk to the command the running converge jobs use.
  Chosen as a server-independent continuation while the live confirmations are
  gated on the busy Ollama server.

## Open Questions

- None.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-31` | `LLM-TEXT-TRANSPORT-DEDUP.1` | shared `commands/llm_text.rs` (3 transport tests); `extract_contracts` + `signal_resolve` route through it (prompts/classifiers/tests unchanged); 3→2 copies; `--provider skip` re-verified; full `scripts/run_ci.sh` | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `LLM-TEXT-TRANSPORT-DEDUP.1` | `LLM-TEXT-TRANSPORT-DEDUP.1 — shared commands/llm_text transport; de-dup extract-contracts + signal-resolve` | 3→2 transport copies; no behavior change; nlp_enrich follow-up noted |

## Changelog

- `2026-05-31`: `.1` — added shared `commands/llm_text.rs`; routed
  `extract_contracts` + `signal_resolve` through it (deleted their duplicated
  transport); 3→2 copies; +3 transport unit tests; no behavior change;
  **TREE CLOSED**. (`nlp_enrich` onto the shared helper = tracked follow-up.)
- `2026-05-31`: Created — consolidate the duplicated text chat transport from
  `extract_contracts` + `signal_resolve` into a shared `commands/llm_text.rs`.
