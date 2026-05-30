# R14-SIGNAL-RESOLVE: Tier-3 LLM `signal_relation` extraction (the `signal-resolve` command)

## Metadata

- Tree ID: `R14-SIGNAL-RESOLVE`
- Status: `done`
- Roadmap lane: `R14` (Actor-signal relation extraction: Tier 3 LLM — *Not Started* → now started)
- Created: `2026-05-31`
- Last updated: `2026-05-31`
- Owner: repo-local workflow

## Goal

Deliver R14's stated goal — "implement `specforge signal-resolve` (or
equivalent integrated relation-resolution flow)" + "add a `signal_relation`
extraction type" — as a new **additive** `signal-resolve` command that mines
actor→signal relations (`drives` / `reads`) from the hard prose that the
Tier-1 (tables) and Tier-2 (verb patterns) extractors miss, using the
production-default Ollama + Qwen2.5VL provider.

The target surface already exists: `EvidenceIR.actor_signal_relations:
Vec<ActorSignalRelation>` (`source.rs:314` — `{actor_name, signal_name,
relation: RelationKind::{Drives,Reads}, source_statement_ids,
automation_confidence}`) is populated today by Tier-1/2 and is already
consumed by `SemanticIR` (cloned in `SemanticIr::build`). So R14 is a *new
producer for an existing field* — no new IR types, no `SemanticIr::build`
wiring, no carry. Tier-3 relations simply join the KG the graph-first model
already scores.

## Non-Goals

- Does NOT modify the live `nlp_enrich` command or its prompt (zero behavior
  change to the existing `signal_constraint`/`conditional_rule` extraction).
  A separate command is the roadmap's own suggested shape and keeps the
  working command untouched (it is in use by converge).
- No accuracy tuning: the value is the wired, safe, grounded Tier-3 path.
  Yield improves iteratively, measured by `validate`'s graph-direction
  coverage.
- Not the CVE contract extractor (separate, done: `CVE-PROSE-EXTRACTION`).

## Acceptance Criteria

- New `specforge signal-resolve <evidence-ir> [--provider ollama] [--model …]
  [--max-statements N] [--dry-run] [--grounding-signals …]` command, mirroring
  the `extract-contracts` structure.
- For each `NormativeStatement` prose sentence it prompts the provider for one
  `{actor, signal, relation:"drives"|"reads"}` JSON object (or `none`), via a
  PURE `classify_relation_response`: `none`/unparseable/ungrounded → skip;
  valid + grounding-gates pass → an `ActorSignalRelation` appended to
  `EvidenceIR.actor_signal_relations`.
- **Grounding gates (anti-fabrication):** signal must be uppercase
  alphanumeric (the `nlp_enrich` signal-name discipline); actor non-empty;
  relation ∈ {drives, reads}; and (when a grounding-signal list is supplied)
  the signal must be a known declared signal — never invent a signal/actor
  the sentence + grounding do not support. Provenance set to the real
  statement; `relation_id` namespaced (`r14:<statement_id>`).
- Survivors deduped against existing `actor_signal_relations` (same
  actor+signal+relation) so Tier-3 never double-counts a Tier-1/2 edge.
- Mock unit tests (via the same `SPECFORGE_VLM_HELPER` hook / pure
  `classify_relation_response`) over: none→skip, malformed→skip, valid
  drives→Accepted, valid reads→Accepted, ungrounded-signal→skip.
- CLI command + dispatch + `commands/mod.rs`; `scripts/run_ci.sh` green.
- Verified end-to-end on a real EvidenceIR (`--provider skip` candidate
  selection); book method-doc subsection; tree CLOSED.

## Architecture (data flow)

```
NormativeStatement prose  ── signal-resolve ──▶ Qwen (Ollama)
   │ "{actor, signal, relation}" or "none"
   ▼ classify_relation_response (PURE, testable)
   none / unparseable / ungrounded ........▶ skip
   valid + grounding-gates pass ...........▶ ActorSignalRelation{Drives|Reads}
   │ dedup vs existing actor_signal_relations
   ▼ EvidenceIR.actor_signal_relations.push(...)  + write
   ▼ SemanticIr::build  (ALREADY clones evidence_ir.actor_signal_relations)
   → actor_ports / signal_connectivity / graph-first direction (existing)
   ▼ validate: graph-direction coverage improves; ungrounded debt shrinks
```

No fabrication: an ungrounded or malformed answer yields no edge; a real edge
joins the same typed KG Tier-1/2 build, scored graph-first by `validate`.

## Task Tree

- ID: `R14-SIGNAL-RESOLVE`
  Status: `done`
  Goal: Tier-3 LLM signal_relation extraction via a `signal-resolve` command
  Children: `.1`, `.2`, `.3`

- ID: `R14-SIGNAL-RESOLVE.1`
  Status: `done`
  Goal: design fixed + tree created (this file)
  Acceptance: architecture/data-flow/grounding-gates/test-plan recorded; tree registered; docs-only.
  Verification: `docs-only; tree + design recorded; no code/book touched (CI invariant)`
  Commit: `see Commit Log`

- ID: `R14-SIGNAL-RESOLVE.2`
  Status: `done`
  Goal: >
    Producer + command: `commands/signal_resolve.rs` mirroring
    `extract_contracts` (curl + `SPECFORGE_VLM_HELPER`); relation-extraction
    prompt; PURE `classify_relation_response` (none/malformed/ungrounded→skip;
    valid drives|reads→`ActorSignalRelation`); grounding gates + dedup vs
    existing relations; append to `EvidenceIR.actor_signal_relations` + write.
    CLI `SignalResolveArgs` + `Commands::SignalResolve` + dispatch + mod.
    Mock unit tests over all outcome paths.
  Acceptance: command builds + dispatches; pure classifier tested (5 paths incl. ungrounded-skip + drives/reads); dedup proven; `scripts/run_ci.sh` green.
  Verification: >
    passed (`2026-05-31`) — new `commands/signal_resolve.rs` producer +
    `SignalResolveArgs` / `Commands::SignalResolve` / `lib.rs` dispatch /
    `commands/mod.rs`. PURE `classify_relation_response` tested over: none→skip,
    malformed→skip, valid drives→Accepted, valid reads→Accepted,
    non-uppercase-signal / bad-relation / empty-actor→skip, ungrounded-signal→
    skip (+ grounded→accept) — 5 tests. Grounding gates (uppercase signal,
    non-empty actor, relation∈{drives,reads}, optional declared-signal
    grounding); provenance set to the real statement; `relation_id` namespaced
    `r14:<id>`; dedup vs existing edges + within-run. Transport mirrors
    `extract_contracts`/`nlp_enrich` (DRY follow-up flagged in-module). Lib
    `1159 → 1164`; full `scripts/run_ci.sh` green.
  Commit: `see Commit Log`

- ID: `R14-SIGNAL-RESOLVE.3`
  Status: `done`
  Goal: >
    Verify end-to-end on a real EvidenceIR (`--provider skip` candidate
    selection); book method-doc subsection in
    `domain/actor-connectivity.md` (the KG/actor-relation chapter); close the
    tree (live-doc sync). Live `--provider ollama` run recorded (honestly
    server-gated if the shared Ollama is still busy).
  Acceptance: skip-mode verified on real artifact; book subsection added; tree CLOSED; full `scripts/run_ci.sh` green.
  Verification: >
    passed (`2026-05-31`) — `signal-resolve
    generated/evidence_ir/readme/evidence_ir.json --provider skip` loaded the
    artifact (0 existing relations), selected 12 `NormativeStatement`
    candidates, clean exit (load + candidate-selection proven, no network).
    Book method-doc subsection added to `domain/actor-connectivity.md`
    (Tier-3 relation extraction, positioned as the third source after the
    Tier-1 table + Tier-2 verb-pattern paths). The live `--provider ollama`
    run remains **server-gated** — the 5 user `converge` jobs still hold the
    single-model Ollama queue, and killing them needs user authorization (the
    safety classifier correctly blocked me from terminating processes I didn't
    create). Wiring + decision logic proven by the `.2` pure
    `classify_relation_response` unit tests (5 paths) + skip-mode. Full
    `scripts/run_ci.sh` green.
  Commit: `see Commit Log`

## Current Frontier

**Tree CLOSED `2026-05-31`** — all three leaves `done`; the live Qwen
confirmation is server-gated (the user's hung converge jobs hold the queue;
killing them needs user authorization), recorded honestly, not blocking.

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R14-SIGNAL-RESOLVE.2` | `done` | producer + `signal-resolve` command landed (lib 1159→1164; CI green) |
| 2 | `R14-SIGNAL-RESOLVE.3` | `done` | skip-mode verified + book method-doc; tree CLOSED |

## Decisions

- `2026-05-31`: Created after the user delegated the next pick. Implemented as
  a SEPARATE `signal-resolve` command (the roadmap's own suggested shape),
  NOT by editing `nlp_enrich`'s prompt — additive, zero-risk to the in-use
  command, mirrors the just-shipped `extract-contracts` pattern. No new IR
  types: `EvidenceIR.actor_signal_relations` already exists + flows to
  SemanticIR, so Tier-3 is a new producer for an existing field (3 leaves, not
  4 — no typed-surface leaf needed).

## Open Questions

- Candidate pool = `NormativeStatement` (Tier-3 = the prose Tier-1/2 missed),
  consistent with `nlp_enrich`/`extract-contracts`. Broadening to `source_fact`
  descriptive prose is a possible future enhancement, out of scope here.

## Blockers

- None. (Live confirmation may be server-gated while the user's converge jobs
  hold the shared Ollama server — recorded honestly, as in CVE-PROSE-EXTRACTION.)

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-31` | `R14-SIGNAL-RESOLVE.1` | design/data-flow/grounding-gates/test-plan recorded; tree registered; docs-only (no code/book ⇒ CI invariant) | `passed` |
| `2026-05-31` | `R14-SIGNAL-RESOLVE.2` | `commands/signal_resolve.rs` producer + CLI/dispatch/mod; pure `classify_relation_response` tested over 5 outcome paths (incl. ungrounded-skip + drives/reads) + grounding gates + dedup; lib `1159 → 1164`; full `scripts/run_ci.sh` | `passed` |
| `2026-05-31` | `R14-SIGNAL-RESOLVE.3` | `signal-resolve --provider skip` on real EvidenceIR (12 candidates, clean exit); book method-doc subsection in `domain/actor-connectivity.md`; live `--provider ollama` server-gated (recorded); tree CLOSED; full `scripts/run_ci.sh` | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `R14-SIGNAL-RESOLVE.1` | `R14-SIGNAL-RESOLVE.1 — design + tree: Tier-3 LLM signal_relation extraction (signal-resolve command)` | docs-only design leaf |
| `R14-SIGNAL-RESOLVE.2` | `R14-SIGNAL-RESOLVE.2 — producer + signal-resolve command (Qwen via Ollama); pure classify_relation_response (5 paths) + CLI wiring` | lib 1159→1164; grounding-gated + deduped; DRY-transport follow-up noted |
| `R14-SIGNAL-RESOLVE.3` | `R14-SIGNAL-RESOLVE.3 — verify skip-mode + book method-doc + close` | tree CLOSED; live run server-gated (recorded); R14 delivered |

## Changelog

- `2026-05-31`: `.3` — verified skip-mode on a real EvidenceIR (12
  candidates); book method-doc subsection added to
  `domain/actor-connectivity.md`; live `--provider ollama` run server-gated
  (recorded honestly); **TREE CLOSED**. R14 (Tier-3 LLM relation extraction)
  delivered as the `signal-resolve` command.
- `2026-05-31`: `.2` — producer + `signal-resolve` command
  (`commands/signal_resolve.rs`) + CLI/dispatch/mod wiring; pure
  `classify_relation_response` (none/malformed/ungrounded→skip; valid
  drives|reads→`ActorSignalRelation`) + grounding gates + dedup + 5 unit
  tests; transport mirrors `extract_contracts` (DRY follow-up flagged);
  lib `1159 → 1164`; full CI green. Frontier → `.3`.
- `2026-05-31`: Created — design fixed for the Tier-3 `signal-resolve`
  command (R14); additive producer for the existing
  `EvidenceIR.actor_signal_relations` KG field; 3-leaf plan.
