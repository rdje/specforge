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

- **`.1` mine + report (read-only):** `pdftotext` all 82 PDFs (+`docling` fallback for hard ones).
  **Owner-flagged refinement (2026-06-05):** a single modal-seed regex
  (`must/shall/should/may/will + <predicate>`) only finds the *modal-reachable* slice — it misses
  (a) other normative phrasings (`is required to`, `needs to`, `has to`, `is to`, `is forbidden`,
  imperatives), (b) **descriptive** behavior with no modal at all ("PSEL **is asserted**", "the
  Manager **drives** HTRANS"), and (c) imperatives ("**Set** bit 3"). So mine from **two ends and
  iterate**: anchor on a *broadened* normative-marker seed AND on the **behavioral verbs
  themselves** (scan `asserted/driven/stable/sampled/latched/…` wherever they occur), then let
  verbs found in one pass seed the next (co-occurrence). The seed-dependence is the inherent
  pattern-matching ceiling — the parked pure-NLP model is what removes it. Tally frequency with
  **per-spec provenance**.
  - First (naive, modal-only) pass done `2026-06-05`: 45,224 occurrences / 1,867 distinct
    predicates across all 82 PDFs; modal-reachable behavioral gap = `set`/`reset`/`cleared`/
    `enabled`/`disabled`/`aligned`/`ignored`/`assert`/`updated`/`loaded`/… — useful but partial.
  - **METHOD CHANGED (owner, `2026-06-05`): use the LLM (`qwen2.5:14b-instruct`) to mine verbs
    seedlessly — OFFLINE/build-time only (the runtime stays deterministic pattern-matching; this is
    the parked pure-NLP model in a bounded list-building role, fully on the bounded-LLM doctrine).**
    Sample head-to-head (5 specs — I²C/Wishbone/OpenCAPI/Avalon/AMD IOMMU, 532 candidate sentences,
    146s): LLM found 8 verbs the seed also found AND **23 the modal regex structurally could not**
    (`driven`/`drives`/`asserts`/`sample`/`sampled`/`holds`/`pulls`/`released`/`remains`/`detects`/
    `generates`/`matched`/`tested`/`transferred`/…) — the active-voice/descriptive forms that live
    in sentences with no `must`/`shall`. Some noise (`defines`/`formed`/`reversed`) → curation +
    owner review. Conclusion: LLM-mining clearly out-recalls the seed regex; it is the `.1` method.
    Pipeline: `pdftotext` → broad signal-presence sentence pre-filter (not a verb seed) → batched
    LLM verb extraction → ground each verb in the corpus → dedup + provenance → owner review. **Filter** to the
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
- ID: `VERB-COVERAGE-CORPUS.1` · Status: `done` · Goal: mine the 82-PDF corpus → provenance-
  tagged verb frequency + a gap report vs the current lists (read-only artifact for owner review).
  Verification: passed (`2026-06-05`) — full seedless LLM mine of all 82 PDFs (qwen2.5:14b, ~39 min)
  → 640 grounded behavioral verbs; **Claude curated** (cross-model) into KEEP/BORDERLINE/REJECT and
  diffed vs the engine. Reviewed artifact: `docs/research/verb-coverage-corpus.md`. Shortlist of
  **new** behavioral verbs the engine lacks: `set`/`clear`/`reset`/`sample`/`release`/`mask`/`gate`/
  `enable`/`disable`/`toggle`/`latch`/`pull`/`capture`/`load`/`store`/`invalidate`/`activate`/… Read-
  only — nothing wired in; awaiting owner sanity pass (esp. the Rejected pile) before `.2`.
- ID: `VERB-COVERAGE-CORPUS.2` · Status: `in-progress` · Goal: integrate the verbs; centralize the
  grammar vocabulary; re-verify; close.
  Done (`2026-06-05`): **owner reframed the model** — the extraction path is "recognize actors +
  signals + normative verbs, capture the `(actor —verb→ signal)` relations between them" = the KG;
  the signal-vs-actor split is not the axis, the **relation kind** (Drives/Reads) is. First
  integration slice: added the corpus-mined relationship verbs to `extract_actor_signal_relations`'
  `ACTIVE_DRIVES_VERBS` (`set`/`clear`/`reset`/`toggle`/`negate`/`release`/`enable`/`disable`/`mask`/
  `gate`/`pull`/`load`/`store`/`write`/`send`/`transmit`/`forward`/`respond`/`request`/`acknowledge`/
  `grant`/`control`/`determine`/…) and `ACTIVE_READS_VERBS` (`poll`/`poll`). Rescued verbs the report
  had wrongly rejected (`provide`/`apply`/`control`/`determine` — they ARE actor→signal edges).
  +1 test (`corpus_mined_actor_verbs_extract_relations`: clears→Drives, polls→Reads). CI green 1258.
  Remaining: value-pinning verbs → signal-constraint side; centralize the vocabulary.

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
