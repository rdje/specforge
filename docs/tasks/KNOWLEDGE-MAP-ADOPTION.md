# KNOWLEDGE-MAP-ADOPTION: adopt the portable Knowledge Map retrieval layer

## Metadata

- Tree ID: `KNOWLEDGE-MAP-ADOPTION`
- Status: `active` (`.1` design done; `.2` = adopt + seed + wire + close)
- Roadmap lane: `R0` (durable memory / live-doc continuity / governance)
- Created: `2026-06-04`
- Owner: repo-local workflow
- Parent context: the user authored a portable, harness-agnostic standard
  **`KNOWLEDGE_MAP_ARCHITECTURE.md`** (the `knowledge-map/` bundle, built in a sibling repo)
  and directed adopting it across their repos — the same pattern as `MEMORY_ARCHITECTURE.md`
  ("you will implement everything that this document recommends"). The KM is an **additive
  retrieval layer** that makes *archaeology* — a fresh agent re-deriving a fact from code or
  runtime that was **already logged once** — a structural impossibility, by deriving a
  question-keyed index (`KNOWLEDGE_MAP.md`) from small front-mattered fact cards. It composes
  with SpecForge's existing `MEMORY_ARCHITECTURE.md` + task-trees + `docs/decisions/`; it
  **replaces nothing and converts nothing**.

## Goal

Adopt the `knowledge-map/` bundle in SpecForge per its §8 checklist, wired into SpecForge's
**existing** enforcement (no new paradigm): copy the self-contained bundle, derive the first
map, add the KM gate to the existing `.githooks/pre-commit` (beside the memory-arch check)
and to `scripts/run_ci.sh` (CI backstop) + `.github/workflows/ci.yml`, register it in the
bootstrap surfaces so the read path is discoverable, and **seed** a handful of high-value
fact cards for facts already established that would otherwise force re-derivation. Going
forward the map grows **lazily** — one card whenever a durable fact is established or
archaeology is caught — never a migration sweep.

## How the bundle maps onto SpecForge (no config override needed)

| KM default | SpecForge reality | Action |
|---|---|---|
| `KM_SCAN_DIRS = docs/knowledge docs/decisions` | `docs/decisions/` exists (ADRs); `docs/knowledge/` is new | create `docs/knowledge/`; ADRs lack front-matter so are ignored until optionally folded in |
| `KM_OUTPUT = KNOWLEDGE_MAP.md` (repo root) | free | use as-is |
| pre-commit hook (regenerate+stage+check) | `.githooks/pre-commit` `exec`s `check_memory_architecture.sh` | rewrite the hook to run **both** gates (no `exec`) |
| CI gate | `scripts/run_ci.sh` (first step = memory-arch) + `.github/workflows/ci.yml` | add a KM step beside the memory-arch step |
| `core.hooksPath .githooks` | already set | none |

Because the defaults already match, **no `.knowledge_map.conf` override** is written and the
bundle files are **copied verbatim, never edited** (per the bundle's own rule).

## Seeding plan (lazy, high-value — not a sweep)

Write `docs/knowledge/*.md` cards only for durable, structural/causal facts that are genuine
archaeology traps. Initial seed (3):
- `docling-device-cpu` — Docling ingest must run on CPU on this stack (torch MPS lacks
  float64); points to `docs/decisions/0001` + the device-selection block. (Prevents
  re-debugging the `Cannot convert a MPS Tensor to float64` failure.)
- `llm-vlm-provider-default` — SpecForge HAS a production Ollama+Qwen2.5VL provider (default
  for converge/enrich/nlp); points to `docs/decisions/0002`. (Prevents "the LLM provider is
  missing" re-derivation.)
- `temporal-eval-residual-fps-are-stale` — the `TEMPORAL-RULE-EVAL` residual 2 FPs
  (degenerate `PSEL`-as-subject header rule; `*_WIDTH` subject) are **stale-artifact only,
  NOT a live bug** — current `evidence.rs` already strips `" when "` + excludes `*_WIDTH`. A
  prime archaeology trap (a future agent could chase them as a live bug). Points to the two
  temporal trees + the `evidence.rs` helpers.

(Folding existing ADRs into the map in-place — adding `answers:` front-matter — is the
bundle's *optional* future bridge, done when convenient, NOT in this tree.)

## Non-Goals

- NOT converting the book / live docs / task-trees into cards (the bundle's #1 anti-pattern).
- NOT editing the bundle files (override only via `.knowledge_map.conf`, not needed here).
- NOT a migration sweep / mass-folding the ADRs.
- NOT a crate code change — this is repo infra + docs only (no `cargo` rebuild).

## Acceptance Criteria

- `.1` design owned (this file), registered in `docs/TASK_TREE.md`.
- `.2`: bundle copied to repo root (scripts +x); `docs/knowledge/` created; the 3 seed cards
  written; `KNOWLEDGE_MAP.md` generated (deterministic, populated); the pre-commit hook runs
  **both** the memory-arch check and `check_knowledge_map.sh`; `run_ci.sh` + `ci.yml` gain
  the KM check; the KM registered in `MEMORY_ARCHITECTURE.md` + `README.md` + the bootstrap
  pointers (`AGENTS.md`/`CLAUDE.md`/`.cursorrules`/copilot). Verified: `gen` + `check` pass,
  the derive-and-diff gate is green, the hook bites on a malformed fact (negative test), full
  `scripts/run_ci.sh` GREEN; book note where appropriate; tree CLOSED.

## Task Tree

- ID: `KNOWLEDGE-MAP-ADOPTION`
  Status: `active`
  Children: `.1` (design) · `.2` (copy + wire + seed + verify + close)

- ID: `KNOWLEDGE-MAP-ADOPTION.1`
  Status: `done`
  Goal: own + design (this file) — read the bundle end-to-end, map it onto SpecForge's
    existing enforcement, fix the seeding plan + verification. Docs-only.
  Acceptance: design recorded; registered.
  Verification: passed (`2026-06-04`) — bundle read in full (`KNOWLEDGE_MAP_ARCHITECTURE.md`,
    `install.sh`, `gen`/`check`/`conf`, template, hook snippet, CI yaml, README, FAQ);
    SpecForge infra confirmed (`core.hooksPath=.githooks`; pre-commit `exec`s the memory-arch
    check → must de-`exec` to run both; `run_ci.sh` first step = memory-arch; `docs/decisions/`
    ADRs have no front-matter → ignored; `ci.yml` present). Defaults match → no config
    override; bundle copied verbatim. Seed set (3) chosen as real archaeology traps.
  Commit: `see Commit Log`

- ID: `KNOWLEDGE-MAP-ADOPTION.2`
  Status: `pending`
  Goal: copy the bundle (scripts +x); create `docs/knowledge/` + the 3 seed cards; generate
    `KNOWLEDGE_MAP.md`; rewrite `.githooks/pre-commit` to run both gates; add the KM check to
    `run_ci.sh` + `ci.yml`; register in `MEMORY_ARCHITECTURE.md` + `README.md` + bootstrap
    pointers; verify (gen/check/derive-and-diff, negative hook test, full CI); close.
  Acceptance: as in the Acceptance Criteria above.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `KNOWLEDGE-MAP-ADOPTION.1` | `done` | owned + designed (bundle read; mapped to SpecForge infra) |
| 2 | `KNOWLEDGE-MAP-ADOPTION.2` | `pending` | copy + wire + seed + verify + close |

## Decisions

- `2026-06-04`: copy the bundle **verbatim** (project-agnostic; defaults already fit) — no
  `.knowledge_map.conf`. Wire the KM gate **beside** the memory-arch gate (compose, don't
  replace) in both the pre-commit hook and `run_ci.sh`/`ci.yml`. Seed lazily (3 archaeology
  traps); do not fold the ADRs in this tree.

## Blockers

- None. Repo-infra + docs only; no crate rebuild.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-04` | `.1` | bundle read end-to-end; SpecForge enforcement infra mapped; seeding + verification plan fixed; docs-only | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `KNOWLEDGE-MAP-ADOPTION.1` | `KNOWLEDGE-MAP-ADOPTION.1 — own + design adoption of the portable Knowledge Map retrieval layer` | docs-only |

## Changelog

- `2026-06-04`: Created — adopt the user's portable `KNOWLEDGE_MAP_ARCHITECTURE.md` bundle in
  SpecForge: an additive, derived, question-keyed retrieval index over durable facts that
  makes archaeology structurally impossible, composed with the existing memory architecture +
  enforcement. Seed lazily; no conversion, no migration.
