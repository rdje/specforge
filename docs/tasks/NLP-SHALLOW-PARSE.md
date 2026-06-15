# NLP-SHALLOW-PARSE: a deterministic in-Rust shallow-parse tier (subject–verb–object understanding)

## Metadata

- Tree ID: `NLP-SHALLOW-PARSE`
- Status: `active` (BUILD frontier measured-EXHAUSTED `2026-06-15` — standing; `.2h`+`.2f` NO-GO, `.3` consolidation-only deferred)
- Roadmap lane: `R16`/`R15e` (extraction)
- Created: `2026-06-09`
- Last updated: `2026-06-15`
- Owner: repo-local workflow

## Goal

Replace the proliferating per-PDF pattern-match grammars (the `PDF-VARIANT-DIGESTION.9.x` treadmill — one
hand-written grammar per phrasing of "this is a signal" / "X drives Y") with ONE general, **deterministic,
pure-Rust shallow-parse layer** that reads a sentence, recovers its **subject–verb–object** structure
(who-does-what), and feeds the recovered triples into the EXISTING document-grounding gates (the declared
signal/actor catalog — ADR 0006). The layer generalizes PHRASING variation; the grounding gate stays the
single source of truth, so nothing is fabricated and the extractor stays chip-agnostic.

This is the deterministic **text** arm of SpecForge's orchestrator role (owner framing `2026-06-09`):
SpecForge is the typed-IR orchestrator that uses NLP (text) and the VLM (figures/tables/timing/FSMs) as
**bounded hypothesis generators**, never black-box end-to-end readers (README; `project_scope`). The VLM arm
is already wired (Qwen2.5-VL via Ollama: `enrich`, `recover-register-bits`). This tree builds the NLP arm.

## Non-Goals

- NOT replacing the grounding gates. The declared-signal/actor catalog is what makes extraction agnostic and
  garbage-free; the shallow parser PROPOSES, the catalog DECIDES. (ADR 0006, `feedback_no_hardcoded_chip_spec_names`.)
- NOT a full syntactic/semantic parser, treebank, or ML dependency model with downloaded weights — that breaks
  determinism (`feedback_scoring_rigor`; the EVIDENCE-DETERMINISM tree) and the offline/no-heavy-dep posture.
- NOT the LLM tier. The Ollama+Qwen tier (`signal-resolve`/`nlp-enrich`/`extract-contracts`,
  `project_llm_provider_ollama_qwen`) stays for enrichment + the hard residual, not the deterministic core.
- NOT touching the VLM/vision arm (separate orchestrated modality).

## Component map (a shallow-understanding text pipeline — each component is an OWNED leaf)

The pipeline is the harness; each component below is task-tree owned (owner directive `2026-06-09`: track
each piece, ensure SpecForge harnesses all of them elegantly). "Tool" is the size-conscious choice (owner
directive `2026-06-09`: download what's needed but be careful with size — see the Tooling decision section).

| # | Component | Owning leaf | Why | Status today | Tool (size-conscious) |
| --- | --- | --- | --- | --- | --- |
| 1 | domain-aware tokenizer | `.2a` | keep `ARVALID`/`S1`/`WDATA[31:0]`/`1'b1`/`C6` as single tokens | partial (`is_hardware_signal_token`) | hand-roll (+ `unicode-segmentation`, tiny) |
| 2 | sentence segmenter | `.2b` | specs break naively on `e.g.`/`1.2.3`/bit-ranges | crude (statement split) | hand-roll (spec-aware) |
| 3 | POS tagger | `.2c` | label noun/verb/aux/det/prep so chunking + SVO can work | none (implicit only) | hand-roll lexicon+rule tagger (seeded by the catalog) |
| 4 | lemmatizer / morphology | `.2d` | `drives/driven/drive` → one edge label | **exists**: `normative_vocab.rs` (VERB-COVERAGE-CORPUS) | extend lists **+ `rust-stemmers`** (Snowball, tiny) for unseen inflections |
| 5 | NP/VP chunking | `.2e` | group "the host debugger" / "can drive" | scattered in relation grammars | hand-roll over POS tags |
| 6 | clause split + coordination | `.2f` | "X, which connects to Y, drives Z and W" → distribute | crude (relative-clause stripping) | hand-roll |
| 7 | negation / modality | `.2g` | `must not`/`shall`/`may` | partial (constraint negation) | hand-roll (extend existing) |
| 8 | light dependency (passive + verb sense) | `.2h` | "Y is driven by X" ↔ "X drives Y"; fix the spike's direction errors | **none — the new capability** | hand-roll |
| 9 | grounding NER gate | `.2i` | is this token actually a signal/actor? | **strong** (declared-signal catalog) | exists — formalize as the gate |
| — | SVO assembler + Extractor-tier integration | `.3` | wire 1–9 into ONE `Extractor`; prove ≥ hand grammars | — | the `EXTRACTOR-ARCHITECTURE` framework |

## Tooling decision (size-conscious — owner directive `2026-06-09`)

Honest verdict: **this pipeline needs almost no download, and that is by design.** The spike (`.1`) showed the
**grounding gate does the heavy lifting a big NER/parser model would otherwise do** (only declared signals/
actors survive → the tagger-choke risk evaporated). So the deterministic, offline path is **mostly hand-rolled
+ at most two TINY pure-Rust crates** (verified to resolve, no model files, KB-scale):
- `unicode-segmentation` v1.13.3 — Unicode word/sentence boundaries (`.2a`/`.2b`).
- `rust-stemmers` v1.2.0 — Snowball stemmer for unseen verb inflections (`.2d`).

**Deliberately REJECTED** (violate the size caution AND the determinism/offline doctrine — `feedback_scoring_rigor`):
- spaCy + `en_core_web_sm` (~12 MB model + heavy Python runtime; non-deterministic, off-process).
- `rust-bert` / ONNX / BERT taggers (100s of MB + libtorch).
- `nlprule` (LanguageTool English rules binary, ~15 MB+ data).

Each tiny crate is added **only when its owning leaf is built** (not pre-added, so nothing unused sits in
`Cargo.lock`) — the most faithful reading of "be careful with size."

## Acceptance Criteria

- A measured decision (the spike) precedes any production code; the spike's numbers gate the build.
- Any built tier slots into the `EXTRACTOR-ARCHITECTURE` framework as a strategy tier feeding the grounding gates.
- Wire-based APB/AHB/AXI/SWD stay at 100% on all scored surfaces (non-regression is a hard gate).
- Deterministic (double-run byte-identical) and offline (no model download, no runtime LLM dependency).
- Live docs + book updated; each leaf committed via `COMMIT.md`.

## Task Tree

- ID: `NLP-SHALLOW-PARSE`
  Status: `active`
  Goal: deterministic in-Rust shallow-parse tier feeding the grounding gates
  Children: `.1` (spike/measure), `.2`+ (build, gated on `.1`)

- ID: `NLP-SHALLOW-PARSE.1`
  Status: `done` (`2026-06-09`)
  Goal: **SPIKE — measure the upside before building.** Prototype a general SVO pipeline (no per-PDF rules),
  ground its (subject, verb→edge, object) triples against each document's own catalog, and measure recall vs
  the current Rust hand grammars + the tagger-choke risk. Two taggers compared: (v1) a hand-rolled
  lexicon+rule tagger; (v2) **nltk's real statistical POS tagger** (`averaged_perceptron_tagger_eng`,
  installed in a throwaway 54 MB `.venv-nlp` — size-conscious vs spaCy's ~200 MB+; deleted after).
  **Findings (honest, scoring-rigor):**
  | doc | declared | rust hand-grammar rels | general SVO (nltk) | agree | recall vs rust | svo-only |
  | --- | --- | --- | --- | --- | --- | --- |
  | APB | 32 | 65 | 8 | 5 | **8%** | 3 |
  | AXI | 305 | 348 | 13 | 10 | **3%** | 3 |
  | SWD | 14 | 26 | 2 | 2 | 8% | 0 |
  | SWP | 2 | 5 | 3 | 2 | 40% | 1 |
  - **The architecture is sound but it is NOT a recall revolution.** A general SVO extractor — even with a
    real statistical tagger — recovers only **3–8%** of what the mature hand grammars already produce. The
    hand-roll→nltk jump was ~zero, so **POS quality is not the bottleneck**: most relation recall lives in
    TABLES and domain-specific constructions (multi-signal coordination, section-scoped "Issuer signals",
    compatibility/lag inference, alias grounding, the 640-verb lexicon applied across many constructions),
    NOT free-prose subject-verb-object. The hand grammars are effectively a domain-tuned shallow extractor
    that out-recalls generic NLP here.
  - **Tagger-choke risk NEUTRALIZED:** the grounding gate (only declared signals/actors survive) removed all
    ALL-CAPS / table-text garbage — the heaviest unknown going in. Grounding does the work a big NER model
    would otherwise do (which is WHY we need almost no download).
  - **Precision caveat:** SVO-only triples included direction errors (e.g. AXI `manager Reads ARID` — ARID is
    manager-DRIVEN). Grounding answers "is it a signal/actor?" but not "is the drives/reads direction right?"
    → that needs the `.2h` passive-voice + verb-sense work.
  **Conclusion / recommendation:** the shallow-parse tier is worth building as a **consolidation + unseen-
  phrasing-robustness** play (one maintainable parser replacing N scattered prose grammars, feeding the same
  grounding), NOT as a recall multiplier — and it does NOT replace table/structure extraction, where the bulk
  of intent lives. The bigger "digest any PDF" lever remains the table-kind classifier + the VLM arm
  (`PDF-VARIANT-DIGESTION` levers A/B). Build incrementally (`.2a`–`.3`), measured against the hand grammars
  on the 100% specs; retire a grammar only when SVO provably ≥ it.
  Verification: spike measured on real APB/AXI/SWD/SWP persisted evidence; numbers above; throwaway `.venv-nlp`
  deleted after (production stays pure-Rust). No production code changed.
  Commit: pending (this slice).

The build leaves below own each pipeline component (owner directive `2026-06-09`); they are `proposed` pending
the owner's go after the tempered spike result, then built incrementally and measured against the hand grammars
(wire-based APB/AHB/AXI/SWD stay 100%). See the Component map for the tool per leaf.

- ID: `NLP-SHALLOW-PARSE.2a` · Status: `deferred` (scope-bounded `2026-06-15` — mostly exists; not the spike bottleneck) · Goal: **domain-aware tokenizer** — keep `ARVALID`/`S1`/
  `WDATA[31:0]`/`1'b1`/`C6` as single tokens; hand-roll + tiny `unicode-segmentation` for boundaries.
- ID: `NLP-SHALLOW-PARSE.2b` · Status: `deferred` (scope-bounded `2026-06-15`) · Goal: **sentence segmenter** — spec-aware splitting that
  survives `e.g.`/`1.2.3`/bit-ranges; hand-roll.
- ID: `NLP-SHALLOW-PARSE.2c` · Status: `deferred` (scope-bounded `2026-06-15` — spike: heavy tagger adds ~0) · Goal: **POS tagger** — deterministic lexicon+rule tagger
  seeded by the declared-signal/actor/verb catalogs (the spike showed a heavy statistical tagger adds ~nothing
  here); hand-roll, no model download.
- ID: `NLP-SHALLOW-PARSE.2d` · Status: `deferred` (scope-bounded `2026-06-15` — exists: `normative_vocab.rs`) · Goal: **lemmatizer / morphology** — map verb inflections
  to one edge label; extend `normative_vocab.rs` + tiny `rust-stemmers` (Snowball) for unseen inflections.
- ID: `NLP-SHALLOW-PARSE.2e` · Status: `deferred` (scope-bounded `2026-06-15`) · Goal: **NP/VP chunker** over POS tags; hand-roll.
- ID: `NLP-SHALLOW-PARSE.2f` · Status: `done` (measured NO-GO as new code, `2026-06-15`) · Goal: **clause split +
  coordination** — distribute "X, which connects to Y, drives Z and W". **Measured resolution (`2026-06-15`, no
  production code):** the canonical example is ALREADY implemented and tested — `coordinated_active_drive_*` /
  `coordinated_active_read_extracts_all_sampled_objects` (`evidence.rs` test mod) prove "An interconnect *which
  connects to components …* can drive ARCHUNKEN **and** RCHUNKV" recovers BOTH objects with `interconnect` as the
  actor (relative-clause stripping in `extract_subject_phrase` + object-clause coordination via
  `active_object_contains_signal`, which scans the whole post-verb clause for each grounded signal token). Corpus
  coordinated sentences over the 78 evidence artifacts (`drive AERR and DERR`, `drive RVALID and BVALID LOW`,
  `drive LAMECID and LAHWATTR`) are all OBJECT coordination → already covered. The only unhandled sub-case —
  SUBJECT coordination ("Actor1 and Actor2 drive X", where `extract_subject_phrase` keeps only the nearest of the
  two) — has **≈0 grounded corpus prevalence** (the only "A and B drive" hits are noise: "current drive of the
  register", "deasserts valid and can drive data"). Building it would be speculative effort against zero measured
  demand. → do NOT build; KM card `nlp-coordination-already-handled`.
- ID: `NLP-SHALLOW-PARSE.2g` · Status: `deferred` (scope-bounded `2026-06-15` — exists: constraint negation) · Goal: **negation / modality** — `must not`/`shall`/`may`;
  extend the existing constraint-negation handling.
- ID: `NLP-SHALLOW-PARSE.2h` · Status: `done` (measured NO-GO as new code, `2026-06-15`) · Goal:
  **light dependency (passive + verb sense)** — fix the spike's direction errors ("Y is driven by X" ↔
  "X drives Y"). **Measured resolution (`2026-06-15`, no production code):** the premise (production has a
  drive/read direction bug) was OVERTURNED. `extract_actor_signal_relations` (`evidence.rs` ~L2822-3039)
  already matches four VOICE-SEPARATED patterns against `normative_vocab.rs` — passive/active × drives/reads —
  with disjoint inflection lists, so direction is already correct BY DESIGN; the spike's `manager Reads ARID`
  error was a property of the throwaway generic-SVO prototype, not this hand grammar. The only prose delta —
  the `to`/recipient transfer frame (`X is sent/returned to Y`) — is **negative-EV**: measured over the 78
  persisted evidence artifacts its subjects are overwhelmingly messages/transactions (`event`/`notification`/
  `response`/`Snoop`/`MSI`), which the signal-subject grounding gate rejects (≈0 grounded yield); `is driven
  to` is value-dominant (`driven to zero`); recipients are document-specific node names (RN/SN/PE/hart — ADR
  0006, not hardcodable); and wire docs contain the frame (AXI=10, AHB=1) so it is NOT additive-safe vs the
  wire-based-100% gold gate. It is also recall (not the precision fix `.2h` was scoped as) in the exact
  low-yield free-prose area the spike de-prioritized. → do NOT build; frontier advances to `.2f`. KM card
  `actor-signal-direction-passive-active-handled`.
- ID: `NLP-SHALLOW-PARSE.2i` · Status: `deferred` (scope-bounded `2026-06-15` — already strong: declared-signal catalog) · Goal: **grounding NER gate** — formalize the existing
  declared-signal/actor catalog as the explicit gate the parser proposes into (ADR 0006); largely exists.
- ID: `NLP-SHALLOW-PARSE.3` · Status: `deferred` (consolidation-only, no measured recall/precision gain —
  `2026-06-15`) · Goal: **SVO assembler + Extractor-tier integration** — wire the pieces into ONE `Extractor`
  (the `EXTRACTOR-ARCHITECTURE` framework) emitting `ActorSignalRelation` hypotheses through the grounding gate.
  **Reframed `2026-06-15`:** with `.2h` and `.2f` both measured NO-GO (their capabilities already live in the
  production hand grammar), `.3` is now a PURE CONSOLIDATION refactor (unify the scattered prose grammar into one
  Extractor) with **no recall/precision gain** — exactly what the spike rated "consolidation, not a recall
  multiplier." Against the wire-based-100% + additive-until-proven + deterministic-byte-identical gates, a
  behavior-preserving rewrite of the mature, well-tested grammar is high-risk / low-value. Build ONLY if a
  concrete maintainability need arises (e.g. a future prose lever genuinely needs the unified seam) or the owner
  directs the consolidation explicitly; never speculatively. `EXTRACTOR-ARCHITECTURE` already provides the
  one-place-registration seam if/when needed.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | `NLP-SHALLOW-PARSE.2h` | `done` (measured NO-GO, `2026-06-15`) | **Resolved without production code.** Production already handles passive+active drive/read DIRECTION correctly (voice-separated verb lexicon); the spike's direction error was prototype-only; the only prose delta (the `to`/recipient frame) is negative-EV (≈0 grounded yield, value-dominant `is driven to`, document-specific recipient names, present in wire docs → not additive-safe). KM card `actor-signal-direction-passive-active-handled`. |
| — | `NLP-SHALLOW-PARSE.2f` | `done` (measured NO-GO, `2026-06-15`) | **Resolved without production code.** Object coordination + relative-clause distribution (the canonical "…which connects to… drives Z and W" example) are ALREADY implemented and tested (`coordinated_active_drive_*` / `coordinated_active_read_*`); subject coordination has ≈0 grounded corpus prevalence → speculative. KM card `nlp-coordination-already-handled`. |
| — | `NLP-SHALLOW-PARSE.3` | `deferred` (consolidation-only) | With `.2h`+`.2f` NO-GO, the assembler is a pure behavior-preserving refactor with no recall/precision gain — high-risk/low-value vs the wire-100%/additive/deterministic gates. Build only on a concrete maintainability need or explicit owner direction. |
| — | (tree) | **BUILD frontier measured-EXHAUSTED** | Both spike-flagged gaps are already covered by the mature production hand grammar; the tree is `mostly done` / standing. Re-open a leaf only if a future measurement surfaces a real prose-recall/precision gap, or the owner wants the `.3` consolidation. PNT pivots to another lever (`CORPUS-PATTERN-REUSE.3b.3a` / `CANONICAL-PROMOTION-SWEEP.1`). |
| — | `.2a`/`.2b`/`.2c`/`.2d`/`.2e`/`.2g`/`.2i` | `deferred` | **Scope-bounded out (`2026-06-15`):** the spike proved POS quality is NOT the bottleneck (hand-roll ≈ nltk) and these mostly already exist in scattered form (tokenizer `is_hardware_signal_token`, lemmatizer `normative_vocab.rs`, grounding gate = the declared-signal catalog). Build a specific one ONLY if measurement during `.2h`/`.2f`/`.3` shows it is the concrete bottleneck — not speculatively. |

## Decisions

- `2026-06-09`: Owner directed building the NLP arm now (SpecForge as harness/orchestrator over NLP+VLM).
  Chosen path: deterministic in-Rust shallow parse (not the LLM as the core path) for reproducibility, feeding
  the existing grounding gates. Spike-first. See `project_nlp_shallow_parse_direction`.
- `2026-06-09`: **Spike `.1` measured — generic SVO recovers only 3–8% of the hand grammars' recall**, and a
  real statistical tagger (nltk) added ~nothing over a hand-rolled one. POS quality is not the bottleneck;
  most recall lives in tables + domain constructions. → the shallow-parse tier is a **consolidation +
  unseen-phrasing-robustness** play, not a recall multiplier; the bigger "digest any PDF" lever stays the
  table-kind classifier + VLM arm. Owner re-decision on build scope recorded as the frontier.
- `2026-06-09`: **Tooling = grounding-first, model-last.** The grounding gate neutralized the tagger-choke
  risk, so SpecForge needs almost no download: mostly hand-rolled deterministic Rust + at most two tiny
  pure-Rust crates (`rust-stemmers`, `unicode-segmentation`). Heavy ML (spaCy/BERT/nlprule) rejected for size
  + determinism. Measurement-only nltk venv (`.venv-nlp`, 54 MB) used then deleted.
- `2026-06-15`: **SCOPE DECIDED (owner-delegated `2026-06-15`: "build the full arm, a bounded consolidation
  slice, or keep parked — you decide" + "do all these"): GO, BOUNDED, gaps-first.** Build only the two
  spike-identified value-bearing pieces — `.2h` (passive-voice + verb-sense direction, the one piece the spike
  marked "none — the new capability" AND the fix for the spike's measured direction errors) then `.2f`
  (coordination distribution) — wired into ONE `Extractor` (`.3`) through the EXISTING grounding gate, reusing
  the already-present tokenization (`is_hardware_signal_token`) / lemmatizer (`normative_vocab.rs`) / grounding
  (declared-signal catalog). The full 9-component academic NLP ladder (`.2a`/`.2b`/`.2c`/`.2e`/`.2g`/`.2i` and
  the duplicate-of-existing `.2d`) is DEFERRED — the spike proved POS quality is NOT the bottleneck (hand-roll
  ≈ nltk) and generic SVO recovers only 3–8% of the hand grammars, so building the full stack speculatively
  would be effort against a lukewarm, measured payoff (quality-over-speed / measured-need discipline). Rationale
  for GO-not-park: the owner explicitly directed pursuing all three forward levers, and `.2h` delivers a real
  *precision* win (correct drive/read direction) independent of the recall verdict. **Hard gates (unchanged):**
  wire-based APB/AHB/AXI/SWD stay 100% on all scored surfaces; deterministic (double-run byte-identical);
  offline (no model download, runtime stays pure-Rust); retire a hand grammar ONLY when SVO provably ≥ it
  (additive until proven). A deferred ladder leaf is built ONLY if measurement during `.2h`/`.2f`/`.3` shows it
  is the concrete bottleneck — never speculatively.

- `2026-06-15`: **`.2h` MEASURED NO-GO as new production code (gaps-first discipline in action).** The first
  build slice opened by reading production + measuring the corpus (Acceptance Criterion #1: a measured decision
  precedes any production code). Finding: `extract_actor_signal_relations` ALREADY handles passive+active
  drive/read DIRECTION correctly via the voice-separated `normative_vocab` lexicon — the spike's `manager Reads
  ARID` error was a generic-SVO-prototype artifact, NOT a production bug, so the leaf's "fix direction errors"
  premise was already satisfied. The only prose delta (the `to`/recipient transfer frame) measured negative-EV
  over the 78 persisted evidence artifacts: ≈0 grounded yield (subjects are messages/transactions the
  signal-subject grounding gate rejects), precision-fraught (`is driven to` is value-dominant; recipients are
  document-specific node names — ADR 0006), and wire-doc-present (AXI=10/AHB=1 → not additive-safe vs the
  wire-based-100% gate) — and it is recall in the exact low-yield free-prose area the spike de-prioritized. →
  do NOT build `.2h`; advance frontier to `.2f`. This is gaps-first (don't rebuild what exists; build the real
  gap), with direct precedent (MEMORY-BOUNDED-INGEST.5 measured-DEFER, FULL-PAGE-INTENT-CAPTURE.1 NO-GO). KM
  card `actor-signal-direction-passive-active-handled`. `[[feedback_scoring_rigor]]`.

- `2026-06-15`: **`.2f` MEASURED NO-GO → the gaps-first BUILD frontier is measured-EXHAUSTED.** Opened `.2f`
  measurement-first: the canonical example ("X, *which connects to Y,* drives Z **and** W") is already
  implemented AND tested (`coordinated_active_drive_*` / `coordinated_active_read_extracts_all_sampled_objects`),
  object coordination is covered by `active_object_contains_signal` (it scans the whole post-verb clause for each
  grounded signal token), and the only residual — SUBJECT coordination — has ≈0 grounded corpus prevalence. So
  `.2f` would be speculative too. **Tree-level conclusion:** BOTH spike-flagged "gaps" (`.2h`, `.2f`) already
  live in the mature production hand grammar, so the gaps-first BUILD scope is empty; `.3` collapses to a pure
  consolidation refactor (no recall/precision gain) that the wire-100%/additive/deterministic gates make
  high-risk/low-value → `deferred`. The tree becomes STANDING (build-exhausted), re-opened only by a future
  measurement of a real prose gap or an explicit owner consolidation directive. This CONFIRMS the spike's
  "consolidation, not a recall multiplier" verdict at the production level: the bigger "digest any PDF" levers
  remain TABLES + the VLM arm. PNT pivots to another lever. KM card `nlp-coordination-already-handled`.
  `[[feedback_scoring_rigor]]` / `[[project_nlp_shallow_parse_direction]]`.

## Open Questions

- ~~Given the spike result (consolidation, not a recall leap), does the owner want to build the full tier now,
  build only the highest-value pieces (`.2h`/`.2f`), or redirect?~~ **RESOLVED `2026-06-15`** (owner delegated
  the call; see Decisions): GO, BOUNDED, gaps-first — `.2h` then `.2f` then the `.3` assembler; the full ladder
  deferred-unless-measured. **Then `.2h` itself resolved measured-NO-GO (`2026-06-15`) — production already
  covers it — so the frontier is now `.2f`.**

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-09` | `NLP-SHALLOW-PARSE.1` | spike: general SVO (hand-roll + nltk) vs hand grammars on real APB/AXI/SWD/SWP | done — 3–8% recall; tagger-choke neutralized by grounding; consolidation-not-revolution |
| `2026-06-15` | `NLP-SHALLOW-PARSE.2h` | read production `extract_actor_signal_relations` + `normative_vocab`; measured `by/from` vs `to` frame prevalence + subject/recipient grounding over the 78 persisted `evidence_ir.json`; checked wire-doc presence | NO-GO as new code — direction already correct (voice-separated lexicon); `to`/recipient frame negative-EV (≈0 grounded yield, value-dominant `is driven to`, wire docs AXI=10/AHB=1). No production code changed; frontier → `.2f` |
| `2026-06-15` | `NLP-SHALLOW-PARSE.2f` | read existing coordination tests + `active_object_contains_signal`/`extract_subject_phrase`; measured coordinated-drive prose over the 78 `evidence_ir.json` | NO-GO as new code — object coordination + relative-clause distribution already implemented & tested; subject coordination ≈0 grounded corpus prevalence. No production code changed; BUILD frontier measured-exhausted; `.3` → deferred (consolidation-only) |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `NLP-SHALLOW-PARSE.1` | (earlier slice) | docs-only spike + tree; no production code |
| `NLP-SHALLOW-PARSE.2h` | `29ef963b` `NLP-SHALLOW-PARSE.2h — measured NO-GO …` | docs-only measured resolution; no production code; frontier → `.2f` |
| `NLP-SHALLOW-PARSE.2f` | `NLP-SHALLOW-PARSE.2f — measured NO-GO …` (this slice) | docs-only measured resolution; no production code; BUILD frontier measured-exhausted |

## Changelog

- `2026-06-09`: Created task tree; owner-directed NLP arm; owned each of the ~9 pipeline components as leaves
  (`.2a`–`.3`); ran + recorded the `.1` spike (honest 3–8% finding); size-conscious tooling decision.
- `2026-06-15`: **SCOPE DECIDED (owner-delegated).** Resolved the build-scope Open Question: GO, BOUNDED,
  gaps-first — frontier set to `.2h` (passive/verb-sense direction, the spike's genuinely-new piece + a real
  precision fix) → `.2f` (coordination) → `.3` (assembler); the full academic NLP ladder
  (`.2a`/`.2b`/`.2c`/`.2d`/`.2e`/`.2g`/`.2i`) marked `deferred` (scope-bounded — POS quality is not the spike
  bottleneck; mostly already exists), built only if measurement proves a specific piece is the bottleneck.
  Hard gates unchanged (wire-based 100%, deterministic, offline, additive-until-proven). Docs-only ownership
  slice — no production code yet (the `.2h` build is the first code slice, for a fresh session). Handoff:
  ready for `.2h` implementation.
- `2026-06-15`: **`.2h` resolved MEASURED NO-GO (no production code).** Opened the `.2h` build measurement-first
  and found production already covers passive/active drive/read direction (voice-separated `normative_vocab`
  lexicon); the spike's direction error was prototype-only; the only delta (the `to`/recipient frame) is
  negative-EV (≈0 grounded yield, value-dominant `is driven to`, document-specific recipient names, wire-doc
  present → not additive-safe). Marked `.2h` `done` (NO-GO), advanced frontier to `.2f` (coordination), wrote
  KM card `actor-signal-direction-passive-active-handled`. Gaps-first discipline; measured-DEFER precedent
  (`.5`, FULL-PAGE-INTENT-CAPTURE.1).
- `2026-06-15`: **`.2f` resolved MEASURED NO-GO → BUILD frontier measured-EXHAUSTED.** Opened `.2f`
  measurement-first and found the canonical example ("…which connects to… drives Z and W") is ALREADY
  implemented and TESTED (`coordinated_active_drive_*` / `coordinated_active_read_extracts_all_sampled_objects`);
  object coordination is covered by `active_object_contains_signal`; the only residual (subject coordination)
  has ≈0 grounded corpus prevalence → speculative. Marked `.2f` `done` (NO-GO). **Tree-level implication:** both
  spike-flagged "gaps" (`.2h`, `.2f`) are already in the production hand grammar, so the gaps-first BUILD scope
  is measured-exhausted; `.3` is reduced to a pure consolidation refactor (no recall/precision gain, gate-risky)
  → `deferred`. Tree → standing (`active`, build-exhausted). KM card `nlp-coordination-already-handled`. **This
  is the spike's "consolidation, not a recall multiplier" verdict CONFIRMED at the production level** — the
  mature hand grammar already does what a shallow-parse tier would, so the bigger "digest any PDF" levers stay
  TABLES + the VLM arm (`PDF-VARIANT-DIGESTION` / `CORPUS-PATTERN-REUSE`), per the spike. `[[feedback_scoring_rigor]]`.
