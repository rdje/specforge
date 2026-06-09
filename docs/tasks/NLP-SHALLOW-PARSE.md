# NLP-SHALLOW-PARSE: a deterministic in-Rust shallow-parse tier (subject–verb–object understanding)

## Metadata

- Tree ID: `NLP-SHALLOW-PARSE`
- Status: `active`
- Roadmap lane: `R16`/`R15e` (extraction)
- Created: `2026-06-09`
- Last updated: `2026-06-09`
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

- ID: `NLP-SHALLOW-PARSE.2a` · Status: `proposed` · Goal: **domain-aware tokenizer** — keep `ARVALID`/`S1`/
  `WDATA[31:0]`/`1'b1`/`C6` as single tokens; hand-roll + tiny `unicode-segmentation` for boundaries.
- ID: `NLP-SHALLOW-PARSE.2b` · Status: `proposed` · Goal: **sentence segmenter** — spec-aware splitting that
  survives `e.g.`/`1.2.3`/bit-ranges; hand-roll.
- ID: `NLP-SHALLOW-PARSE.2c` · Status: `proposed` · Goal: **POS tagger** — deterministic lexicon+rule tagger
  seeded by the declared-signal/actor/verb catalogs (the spike showed a heavy statistical tagger adds ~nothing
  here); hand-roll, no model download.
- ID: `NLP-SHALLOW-PARSE.2d` · Status: `proposed` · Goal: **lemmatizer / morphology** — map verb inflections
  to one edge label; extend `normative_vocab.rs` + tiny `rust-stemmers` (Snowball) for unseen inflections.
- ID: `NLP-SHALLOW-PARSE.2e` · Status: `proposed` · Goal: **NP/VP chunker** over POS tags; hand-roll.
- ID: `NLP-SHALLOW-PARSE.2f` · Status: `proposed` · Goal: **clause split + coordination** — distribute
  "X, which connects to Y, drives Z and W"; hand-roll.
- ID: `NLP-SHALLOW-PARSE.2g` · Status: `proposed` · Goal: **negation / modality** — `must not`/`shall`/`may`;
  extend the existing constraint-negation handling.
- ID: `NLP-SHALLOW-PARSE.2h` · Status: `proposed` · Goal: **light dependency (passive + verb sense)** — the
  genuinely-new capability; fixes the spike's direction errors ("Y is driven by X" ↔ "X drives Y"); hand-roll.
- ID: `NLP-SHALLOW-PARSE.2i` · Status: `proposed` · Goal: **grounding NER gate** — formalize the existing
  declared-signal/actor catalog as the explicit gate the parser proposes into (ADR 0006); largely exists.
- ID: `NLP-SHALLOW-PARSE.3` · Status: `proposed` · Goal: **SVO assembler + Extractor-tier integration** —
  wire `.2a`–`.2i` into ONE `Extractor` (the `EXTRACTOR-ARCHITECTURE` framework) emitting `ActorSignalRelation`
  hypotheses through the grounding gate; prove ≥ the hand grammars on APB/AHB/AXI/SWD before retiring any.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `NLP-SHALLOW-PARSE.2a`–`.3` | `proposed` | Spike `.1` done (architecture sound but a consolidation play, not a recall revolution). Owner go/no-go on building the tier given the tempered result; then build incrementally, measured against the hand grammars on the 100% specs. |

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

## Open Questions

- Given the spike result (consolidation, not a recall leap), does the owner want to build the full tier now,
  build only the highest-value pieces (`.2h` passive/verb-sense + `.2f` coordination, which the spike showed
  are the real gaps), or redirect the energy to the higher-leverage table/VLM arm? (Owner go/no-go — frontier.)

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-09` | `NLP-SHALLOW-PARSE.1` | spike: general SVO (hand-roll + nltk) vs hand grammars on real APB/AXI/SWD/SWP | done — 3–8% recall; tagger-choke neutralized by grounding; consolidation-not-revolution |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `NLP-SHALLOW-PARSE.1` | pending (this slice) | docs-only spike + tree; no production code |

## Changelog

- `2026-06-09`: Created task tree; owner-directed NLP arm; owned each of the ~9 pipeline components as leaves
  (`.2a`–`.3`); ran + recorded the `.1` spike (honest 3–8% finding); size-conscious tooling decision.
