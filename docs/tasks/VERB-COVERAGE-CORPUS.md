# VERB-COVERAGE-CORPUS: mine a comprehensive normative-verb vocabulary from a real chip-spec corpus

## Metadata

- Tree ID: `VERB-COVERAGE-CORPUS`
- Status: `active` (`.1` corpus mine + gap report; `.2` curate + integrate)
- Roadmap lane: `R16`/`R15e` (extraction quality — grammar coverage)
- Created: `2026-06-05`
- Owner: repo-local workflow
- Parent context: owner request (2026-06-05). Because extraction is **pattern-matching** (not the
  parked pure-NLP model, `[[PURE-NLP-INTENT-EXTRACTION]]`), the normative-verb list IS the engine's
  vocabulary — a constraint phrased with an unlisted verb is silently dropped. Goal: a verb list
  large enough to cover the **vast majority** of digital chip-spec PDFs, derived from a real corpus.
  Verbs are **grammar = the "how"** (ADR `0006`/`[[feedback-no-hardcoded-chip-spec-names]]`), so a
  big corpus-derived verb list is exactly the kind of list we *want* — it never memorizes names.

## Corpus (confirmed available)

`/Users/richarddje/Documents/livework/chipdoc` — **82 PDFs, 564 MB**, cross-vendor: I²C (NXP),
Wishbone (OpenCores), CCIX, OpenCAPI, Intel VT-d/SDM, AMD IOMMU, Avalon (Intel/Altera), … Tooling
present: `pdftotext`, `docling`, `pymupdf`. **PoC done (2026-06-05):** mined two specs and the
normative-predicate vocabulary clearly exceeds the current lists (`reset`, `driven`, `generate`,
`qualify`, `initiate`, `recognize`, … beyond `high/low/stable/asserted/change/hold`).

## Design

- **`.1` mine + report (read-only):** `pdftotext` all 82 PDFs (+`docling` fallback for hard ones);
  extract normative predicate patterns (`must/shall/should/may/will (not) (be/remain/stay/driven/
  held/…) <predicate>`); tally frequency with **per-spec provenance**. **Filter** to the
  *signal-behavioral* subset (what a *signal* does — asserted/driven/sampled/stable/reset/toggled/…)
  versus *device/system requirements* (`must conform/support/describe` — out of scope for signal
  constraints). Produce a gap report: corpus verbs **not** covered by the current extractor lists
  (`classify_statement`, `is_signal_value_constraint`, the constraint-kind classifier,
  `extract_action_phrase`). Output a reviewed artifact under `docs/research/` — **owner reviews
  before any wiring**.
- **`.2` curate + integrate:** add the confirmed signal-behavioral verbs, mapping each to a
  constraint kind (or a new kind where warranted); ideally **centralize** the scattered
  normative-grammar vocabulary into one place so it is maintainable and corpus-traceable. Re-verify
  with the extraction suite + a re-extraction; **no names** enter the lists. Then close.

## Non-Goals / guardrails

- Harvest **grammar only** (verbs/predicates) — never signal/value/protocol **names** (ADR 0006).
- Not the pure-NLP model (parked) — this strengthens the *pattern* engine, which must be proven
  first.
- Huge PDFs (Intel SDM, thousands of pages) are sampled/streamed, not held whole in memory.

## Task Tree

- ID: `VERB-COVERAGE-CORPUS` · Status: `active` · Children: `.1` · `.2`
- ID: `VERB-COVERAGE-CORPUS.1` · Status: `pending` · Goal: mine the 82-PDF corpus → provenance-
  tagged verb frequency + a gap report vs the current lists (read-only artifact for owner review).
- ID: `VERB-COVERAGE-CORPUS.2` · Status: `pending` · Goal: curate + integrate the signal-behavioral
  gaps (mapped to constraint kinds); centralize the grammar vocabulary; re-verify; close.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `VERB-COVERAGE-CORPUS.1` | `pending` | extract + mine + gap report (read-only, for review) |
| 2 | `VERB-COVERAGE-CORPUS.2` | `pending` | curate + integrate the confirmed gaps |

## Decisions

- `2026-06-05`: build the verb list from the owner's real 82-PDF corpus; mine read-only first and
  let the owner review the gap report before integrating; grammar only, no names; strengthens the
  pattern engine (the pure-NLP model stays parked).

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |

## Changelog

- `2026-06-05`: Created — owner request; PoC confirmed feasible on I²C + Wishbone; plan to mine all
  82 specs into a provenance-tagged, owner-reviewed normative-verb vocabulary.
