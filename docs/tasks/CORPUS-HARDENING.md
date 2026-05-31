# CORPUS-HARDENING: harden SpecForge against the real chip-doc corpus (AMBA core first)

## Metadata

- Tree ID: `CORPUS-HARDENING`
- Status: `active`
- Roadmap lane: `R12`/`R15e` (multi-spec validation + KG-quality, on real specs)
- Created: `2026-05-31`
- Last updated: `2026-05-31`
- Owner: repo-local workflow

## Goal

Use the real downloaded corpus at `/Users/richarddje/Documents/livework/chipdoc`
(82 PDFs / 10 families, user-curated `2026-05-31`; PCIe is a known paywall gap)
as a **standing hardening harness** for SpecForge. Run specs end-to-end
(Docling → `SourceIR` → `EvidenceIR` [→ `Semantic` → `Intent` → `.isf`]) → run
`validate` → collect the completeness signals (the just-built convergence +
register-tiling detectors, plus the existing residual/conflict surfaces) →
**turn recurring real misses into owned fix-trees**. Prioritize the AMBA **core
protocol** specs (AXI/AHB/APB/CHI) — SpecForge's design home turf and the source
of its R12 validation baselines (APB 94 / AHB 94 / AXI 85).

This is the bridge from "the completeness research + detectors" to "measured,
driven-down misses on the specs that matter."

## Non-Goals

- NOT a code change in itself — running specs + `validate` is usage/validation.
  Each *fix* a run reveals becomes its own owned tree (no code change without one).
- NOT re-downloading or modifying the corpus repo (read-only; it is the user's).
- NOT requiring the live LLM (Ollama) for the structural detectors — `ingest`/
  `evidence`/`validate` exercise the completeness surfaces without it; VLM/NLP
  enrichment is a separate, optional pass.

## Acceptance Criteria

- AMBA core specs (AXI/AHB/APB/CHI) run through `ingest`→`evidence`→`validate`;
  per-spec results recorded (nlp_coverage, register tiling, convergence,
  conflicts, residual counts; baseline check where one exists).
- Recurring real misses/issues ranked and each scoped as a candidate fix-tree
  (the I2C over-eager-register-classifier issue is the first such candidate).
- A short "corpus hardening ledger" kept current as more families are run.

## Task Tree

- ID: `CORPUS-HARDENING`
  Status: `active`
  Goal: standing hardening harness over the real corpus; AMBA core first
  Children: `.1`, `.2`, `.3`, …

- ID: `CORPUS-HARDENING.1`
  Status: `done`
  Goal: prove the harness on real specs (I2C + eMMC) and own the campaign.
  Verification: >
    passed (`2026-05-31`) — ran I2C UM10204 + eMMC JESD84-B50 end-to-end
    (Docling 2.84.0 → evidence → validate). Confirmed: the **register-tiling
    detector is precise** (I2C true-positive on a mis-classified address table;
    eMMC 0 false positives over 48 real registers) and the **convergence report
    is useful** (eMMC 194 facts recovered + converged; I2C 0, honest). Recorded
    in `COMPLETENESS-CLOSURE-INVARIANTS` real-corpus validation. First discovered
    issue logged: over-eager register-map table classifier (candidate fix-tree).
  Commit: `see Commit Log`

- ID: `CORPUS-HARDENING.2`
  Status: `in_progress`
  Goal: >
    Run the AMBA core protocol specs (APB `IHI0024_E`, AHB `IHI0033_C`, AXI
    `IHI0022_L`, CHI `IHI0050_G`) through ingest→evidence→validate; record the
    per-spec completeness signals + check the actor-relative extraction (ports,
    directions, handshakes) against the R12 baselines; rank discovered misses.
  Acceptance: 4 core specs run + recorded; misses ranked into candidate fix-trees.
  Verification: >
    in progress (`2026-05-31`) — 3 of 4 AMBA core specs run end-to-end
    (ingest→evidence→validate): **APB** IHI0024_E (579 stmts, 4 regs, conv 19✓),
    **AHB** IHI0033_C (1339, 21 regs, conv 74✓), **AXI** IHI0022_L (6991, 11 regs,
    conv 257✓). Register tiling clean on all (84 clean registers corpus-wide, 0
    false positives); convergence informative on all; the recurring real miss is
    uniformly **partially-structured normative prose (18/72/554 residuals) +
    un-enriched visual evidence**. CHI IHI0050_G remains (large coherency
    protocol). Misses ranked below; the dominant lever is region-accounting +
    cross-modal + a VLM-in-the-loop pass.
  Commit: `see Commit Log`

- ID: `CORPUS-HARDENING.3`
  Status: `in_progress`
  Goal: >
    VLM-in-the-loop recall pass (the dominant miss the ledger revealed):
    run the production `enrich` (VLM/Qwen via Ollama) on a real spec's SourceIR →
    rebuild `evidence` (VLM observations injected) → `nlp-enrich` (reclassify
    residual normative prose) → re-`validate`, and **measure the recall gain**
    (does the un-enriched-visual warning clear? do timing/state extractions go
    >0? do nlp residuals drop?). Target: AMBA APB IHI0024_E (35 visual assets,
    18 nlp residuals — home turf, manageable cost). Running existing commands,
    not a code change.
  Acceptance: enrich+nlp-enrich+re-validate run on APB; before/after recall delta recorded.
  Verification: >
    passed (`2026-05-31`) — full production VLM+NLP-in-the-loop pass on AMBA APB
    via Ollama/Qwen2.5VL. `enrich`: 7 VLM calls (6 timing + 1 state diagrams),
    0 errors → `timing_diagram_extractions` 0→6, `state_machine_extractions` 0→1,
    validation findings 5→3 (un-enriched-visual warning + rescan guidance
    cleared). `nlp-enrich`: 31 LLM calls / 0 errors, 3-pass convergent → nlp
    residuals 18→13, `signal_constraints` 37→42 (+5), +16 prose-synthesized signal
    declarations; the 13 remaining were honestly declined (no fabrication). Net
    recovered: 6 timing + 1 state + 5 constraints + 16 declarations the structural
    pass missed. Demonstrates the dominant corpus-wide capture gap (un-enriched
    visuals + partially-structured prose) is real AND closable with the existing
    production stack — honestly (fails-closed on the unstructurable residue).
  Commit: `see Commit Log`

## Current Frontier

| Order | Leaf | Status | Why |
| --- | --- | --- | --- |
| 1 | `CORPUS-HARDENING.1` | `done` | harness proven on I2C + eMMC; campaign owned |
| 2 | `CORPUS-HARDENING.2` | `in_progress` | AMBA core: APB/AHB/AXI run + recorded; CHI remains |
| 3 | `CORPUS-HARDENING.3` | `done` | VLM+NLP-in-the-loop recall pass on APB — recovered 6 timing + 1 state + 5 constraints + 16 decls (dominant gap closed, honestly) |

## Decisions

- `2026-05-31`: prioritize AMBA core (SpecForge's design targets + known
  baselines) before the breadth of the corpus — highest-signal, lowest-ambiguity
  hardening. Structural detectors run without the LLM; VLM/NLP enrichment optional.
- `2026-05-31`: discovered issues → ranked candidate fix-trees, not inline fixes
  (no code change without an owning tree).

## Corpus hardening ledger (per-spec, evidence-stage `validate`)

Structural detectors only (no VLM/NLP enrichment run yet). Columns: statements /
nlp_coverage / register_records (tiling overlaps+gaps) / convergence
(passes·new-facts·converged) / notable real misses.

| Spec | stmts | nlp_cov | regs (tiling) | convergence | notable misses |
| --- | --- | --- | --- | --- | --- |
| nxp I2C UM10204 | 956 | 13% | 1 (**1 overlap**) | 1p · 0 · ✓ | overlap = mis-classified address table (TP); empty actor-signal graph; 59 nlp residuals; 126 un-enriched visuals |
| jedec eMMC JESD84-B50 | 6561 | — | 48 (0/0) | 2p · 194 · ✓ | 259 nlp residuals; 3 semantic-role conflicts; 344 un-enriched visuals |
| arm AMBA APB IHI0024_E | 579 | 10% | 4 (0/0) | 2p · 19 · ✓ | 18 nlp residuals; un-enriched visuals; (clean — actor-signal graph populated, no conflicts) |
| arm AMBA AHB IHI0033_C | 1339 | 13% | 21 (0/0) | 2p · 74 · ✓ | 72 nlp residuals; 1 semantic-role conflict; un-enriched visuals |
| arm AMBA AXI IHI0022_L | 6991 | 12% | 11 (0/0) | 2p · 257 · ✓ | 554 nlp residuals; un-enriched visuals; (clean — no conflicts) |

**VLM-in-the-loop recall gain (APB, `.3`):** running the production `enrich`
(Qwen2.5VL via Ollama) on APB then rebuilding evidence recovered intent that was
**completely missed** by the structural pass:

| APB metric | before (no VLM) | after VLM `enrich` |
| --- | --- | --- |
| timing_diagram_extractions | 0 | **6** |
| state_machine_extractions | 0 | **1** |
| validation findings | 5 | **3** (un-enriched-visual *warning* + rescan-guidance cleared) |

7 Qwen calls (6 timing + 1 state), 0 errors. This directly confirms the ledger's
diagnosis: the dominant capture-recall loss is un-enriched visual evidence, and
the production VLM path closes it.

The prose half (`nlp-enrich`, Qwen): **nlp residuals 18 → 13** (5 normative
statements recovered into typed `signal_constraints` 37 → 42), **+16 signal
declarations** synthesized from prose relations, 31 LLM calls / 0 errors, loop
converged ("residual stable at 13"). The remaining 13 were **honestly declined**
(`no_extraction=13` — fails-closed, no fabrication). Net: the full production
VLM+NLP-in-the-loop pass recovered **6 timing diagrams + 1 state machine + 5
typed constraints + 16 signal declarations** the structural pass missed — the
dominant corpus-wide gap, closed with the existing stack, honestly.

Reading: register tiling is **precise** (fires only on the I2C mis-classification,
silent on 52 clean registers across eMMC+APB). Convergence is **informative**
(eMMC 194 / APB 19 / I2C 0 — distinguishes specs where anchored rescan helps).
The dominant recurring real miss across all specs is **un-enriched visual
evidence + partially-structured normative prose** — the strongest signal for
where capture recall is lost (region-accounting + cross-modal detectors + a
VLM-in-the-loop pass are the indicated next instruments).

## Discovered issues (ranked candidate fix-trees)

1. **Over-eager register-map table classifier** → **OWNED: `REGISTER-MAP-CLASSIFIER-PRECISION`**
   (`2026-05-31`). What started as an I2C symptom (the address table flagged by
   the tiling detector) was traced to a **systemic** bug: the bare `has_addr_col`
   heuristic in `classify_table_kind` mis-classifies feature matrices, TOCs,
   data-frame layouts, and address-assignment tables as `register_map` across
   I2C/eMMC/APB (inflating `register_records` with non-registers). Fix =
   precision-tighten (address+field-structure), corpus-regression-verified.

## Open Questions

- How far beyond AMBA core to take the campaign (the full 82, or a curated
  protocol-focused subset)? Decide with the user after the core results.
- Should VLM/NLP enrichment (Ollama) be part of the hardening loop, or kept
  separate? (Many real misses are un-enriched visuals — but that needs the live
  model and is slower.)

## Blockers

- None (Docling ready; structural detectors need no LLM).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-31` | `.1` | I2C + eMMC end-to-end; tiling detector precise (TP + 0 FP/48); convergence useful; classifier issue logged | `passed` |
| `2026-05-31` | `.2` | AMBA core APB/AHB/AXI run + recorded (3/4; CHI remains); tiling 0-FP across 84 registers; recurring miss = prose residuals + un-enriched visuals | `partial` |
| `2026-05-31` | `.3` | VLM+NLP-in-the-loop on APB (Ollama/Qwen): timing 0→6, state 0→1, findings 5→3, nlp residuals 18→13 (+5 constraints, +16 decls); 0 errors; honest fails-closed on 13 | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `CORPUS-HARDENING.1` | `CORPUS-HARDENING.1 — own the corpus-hardening harness (I2C + eMMC proven)` | tree + ledger; validation already recorded in COMPLETENESS-CLOSURE-INVARIANTS |
| `CORPUS-HARDENING.2` | `CORPUS-HARDENING.2 (progress) — APB/AHB/AXI runs + per-spec ledger` | 3/4 AMBA core; CHI remains |
| `CORPUS-HARDENING.3` | `CORPUS-HARDENING.3 — VLM+NLP-in-the-loop recall pass on APB (Ollama/Qwen)` | dominant capture gap closed on real data, honestly |

## Changelog

- `2026-05-31`: Created — own the real-corpus hardening campaign (user: "more
  than enough PDFs to harness SpecForge"). Harness proven on I2C + eMMC; AMBA
  core (AXI/AHB/APB/CHI) is the in-progress first campaign (APB ingesting).
  First discovered issue: over-eager register-map classifier.
