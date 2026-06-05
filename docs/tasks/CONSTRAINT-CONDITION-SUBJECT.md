# CONSTRAINT-CONDITION-SUBJECT: never let a condition-clause signal become a constraint subject

## Metadata

- Tree ID: `CONSTRAINT-CONDITION-SUBJECT`
- Status: `done` (CLOSED `2026-06-05`; `.1` — fallback excludes condition-clause signals)
- Roadmap lane: `R16`/`R15e` (extraction quality)
- Created: `2026-06-05`
- Owner: repo-local workflow
- Parent context: the real-APB `nli-verify` run (`[[nli-gate-real-apb-validation]]`) exposed
  constraint over-generation. The most *general* current-code cause: in `extract_signal_constraints`
  the primary subject scan correctly strips the trailing condition clause, but when the subject
  part has no concrete signal (e.g. *"The following signals must be valid when PSEL is asserted:"*)
  it **falls back to scanning the full text**, which re-introduces the condition signal → a bogus
  "PSEL must be VALID" record (PSEL is the *condition*, not the subject).

## Design (principled, protocol-agnostic, low-risk)

- In `extract_signal_constraints`, when the empty-subject **fallback** scans the full text,
  **exclude any token that appears in the condition clause** (`&sentence[before.len()..]`, the part
  `text_before_condition_marker` cut off). So a signal that appears *only* in the "when …"/"until
  …"/"if …" clause can never become the subject; if that empties the subject set, no constraint is
  emitted (correct — the real subject is a forward-referenced list the extractor can't resolve).
- Applies to the **fallback path only** — the primary path already strips the condition, so legit
  subjects are untouched. General across protocols (no denylist of protocol-specific names).

## Non-Goals / documented follow-ups

- Filtering **non-signal subjects** (FSM states `ACCESS`/`SETUP`, the clock `PCLK`, width params)
  needs a signal/state/clock registry threaded into the extractor — a separate tree. Note the
  generated EvidenceIR artifacts may **predate** the existing `_WIDTH` filter, so some over-
  generation the run showed may already be fixed; confirming needs a re-extraction.

## Acceptance Criteria

- `.1`: condition-clause tokens excluded from the fallback subject; unit test (the "following
  signals … when PSEL is asserted" sentence yields NO PSEL constraint; a normal "PADDR must be
  stable when HREADY is LOW" still yields PADDR); book/KM note; full CI GREEN; tree CLOSED.

## Task Tree

- ID: `CONSTRAINT-CONDITION-SUBJECT` · Status: `active` · Children: `.1`
- ID: `CONSTRAINT-CONDITION-SUBJECT.1` · Status: `done` · Goal: the exclusion + test + close.
  Verification: passed (`2026-06-05`) — `extract_signal_constraints`' empty-subject fallback now
  excludes any token that appears in the condition clause
  (`&sentence[text_before_condition_marker(sentence).len()..]`), so a forward-reference subject
  ("The following signals must be valid when PSEL is asserted") no longer mis-attributes to PSEL.
  Test `constraint_subject_excludes_condition_signal_on_forward_reference` added to the existing
  `CONSTRAINT-SUBJECT-PRECISION` section. **Important finding while implementing:** that section
  *already* fixes width-params, cross-sentence sweeping, and until/if conditions — so the generated
  EvidenceIR artifacts the NLI run used are **stale**, and much of the "33/42 over-generation" is
  *already addressed* in current code; this leaf closes the one remaining (forward-reference)
  fallback gap. Truly measuring current extraction quality needs a **re-extraction** + re-run.
  Full `scripts/run_ci.sh` GREEN (1255→1256).

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `CONSTRAINT-CONDITION-SUBJECT.1` | `done` | fallback excludes condition signals → **tree CLOSED** |

**Tree CLOSED `2026-06-05`.** Fallback no longer attributes a constraint to a condition-clause
signal. **Honest correction:** the generated artifacts the NLI run used are *stale* — current code
(`CONSTRAINT-SUBJECT-PRECISION`) already handles width-params / cross-sentence / until-if, so most
of the apparent over-generation is already fixed; this closes the remaining forward-reference gap.
A re-extraction is the way to measure *current* extraction quality (noted follow-up).

## Decisions

- `2026-06-05`: fix the *general* cause (condition-clause signal leaking into the subject via the
  full-text fallback); leave non-signal/registry filtering to a follow-up tree.

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `CONSTRAINT-CONDITION-SUBJECT.1` | `CONSTRAINT-CONDITION-SUBJECT.1 — fallback excludes condition-clause signals from constraint subjects; close` | +1 test; CI green 1256 |

## Changelog

- `2026-06-05`: Created — from the real-APB NLI finding; stop the fallback from attributing a
  constraint to a condition-clause signal.
- `2026-06-05`: **Tree CLOSED.** Fallback condition-exclusion + test. Found the NLI-run artifacts
  are stale (much over-generation already fixed); re-extraction noted as the way to re-measure.
