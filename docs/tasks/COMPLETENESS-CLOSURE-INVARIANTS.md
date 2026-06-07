# COMPLETENESS-CLOSURE-INVARIANTS: first completeness miss-detectors (symbol closure + register tiling)

## Metadata

- Tree ID: `COMPLETENESS-CLOSURE-INVARIANTS`
- Status: `done`
- Roadmap lane: `R15d` (arbitration/closure) — first implementation from `INTENT-COMPLETENESS-RESEARCH`
- Created: `2026-05-31`
- Last updated: `2026-06-01`
- Owner: repo-local workflow

## Goal

The **first implementation** from the completeness research program (user chose
"closure invariants" as where coding begins). Build the cheapest, EXACT,
ground-truth-free miss detectors from the catalog
([`miss-detectors-catalog.md`](../research/miss-detectors-catalog.md) C1/C2) and
surface each detected miss as an explicit `validate` finding (the established
residual surface). This proves the whole **detect-a-miss → surface-a-residual →
validate** loop end-to-end on real specs, at minimal risk, and establishes the
plumbing future detectors plug into.

Two detectors:
- **Register bit-tiling (C2, EXACT):** within a `RegisterRecord`, the documented
  fields must not overlap, and must have no *interior* gap (an uncovered bit
  between the lowest and highest documented field). Overlap = a real defect; an
  interior gap = a likely **missed field**. (No register width is declared in the
  IR, so we deliberately do NOT flag bits above the highest field — that would
  require speculating width; conservative by construction.)
- **Symbol closure (C1, GATED):** a signal *referenced* (constraint subject,
  conditional/temporal rule, actor-signal relation) that is absent from the
  canonical signal inventory = a **dangling reference** (a missed declaration or a
  doc defect). Gated to avoid flagging legitimately-external signals.

## Non-Goals

- NOT building the full `CompletenessReport` yet (that is research `.5` + a later
  tree) — these detectors surface as `validate` findings + metrics, exactly like
  the existing polarity/semantic/connectivity conflict surfaces.
- NOT inventing facts — detectors only FLAG; they never mutate the IR.
- NOT assuming a register width (avoid width-speculation false positives).

## Acceptance Criteria

- A pure, unit-tested `ir/completeness.rs` module with the two detectors as pure
  functions over the existing IR types (no I/O).
- Register tiling: overlaps + interior gaps detected; conservative (no width
  assumption); surfaced in `validate` (EvidenceIR stage, where `register_records`
  live) as findings + metrics.
- Symbol closure: dangling references detected against the canonical inventory,
  gated to suppress known-external/false-positive cases; surfaced in `validate`.
- Behavior-neutral on extraction; full `scripts/run_ci.sh` green; book subsection
  in the topically-correct chapter (`quality/validation.md` or `pipeline/*`).

## Task Tree

- ID: `COMPLETENESS-CLOSURE-INVARIANTS`
  Status: `done`
  Goal: first completeness detectors (symbol closure + register tiling) surfaced in validate
  Children: `.1`, `.2`, `.3`, `.4`

- ID: `COMPLETENESS-CLOSURE-INVARIANTS.1`
  Status: `done`
  Goal: own + design (this file); record the first-slice decision in `INTENT-COMPLETENESS-RESEARCH.7`; register. Docs-only.
  Acceptance: tree created + registered; research `.7` records the chosen backlog + first slice.
  Verification: pending
  Commit: `see Commit Log`

- ID: `COMPLETENESS-CLOSURE-INVARIANTS.2`
  Status: `done`
  Goal: >
    Register bit-tiling detector: new `ir/completeness.rs` with a pure
    `register_tiling_residuals(&[RegisterRecord]) -> Vec<…>` (overlap + interior
    gap, conservative, no width assumption); unit tests over clean/overlap/gap/
    sparse cases; wire into `validate_evidence_ir` (findings + metrics). Full CI.
  Acceptance: detector + tests + validate surface; CI green.
  Verification: >
    passed (`2026-05-31`) — new `ir/completeness.rs` module with pure
    `register_tiling_residuals(&[RegisterRecord]) -> Vec<RegisterTilingResidual>`
    (`RegisterTilingKind::{Overlap, InteriorGap}`): per register, fields with both
    bit bounds are coverage-counted over `[min_low, max_high]`; overlaps (cover>1)
    and interior gaps (cover==0) collapse into compact bit ranges (`bits [hi:lo]`/
    `bit [n]`). Conservative — no width speculation (bits above the highest field
    never flagged), reversed bounds normalized, <2 bounded fields skipped, span
    capped at 4096 against garbage. 7 unit tests (clean/overlap/interior-gap/
    single-field/no-bounds/reversed/single-bit). Wired into `validate_evidence_ir`:
    a `Register Tiling` section + findings (Overlap→Warning
    `evidence_register_field_overlaps`; InteriorGap→Info
    `evidence_register_field_interior_gaps`) + 2 metrics; 1 validate wiring test.
    Extraction-neutral (flags only, never mutates). fmt/clippy clean; full
    `scripts/run_ci.sh` GREEN.
  Commit: `see Commit Log`

- ID: `COMPLETENESS-CLOSURE-INVARIANTS.3`
  Status: `descoped`
  Goal: >
    Symbol-closure / dangling-reference detector: pure fn computing referenced
    signals (constraint subjects, conditional/temporal rule targets, actor-signal
    relations) minus the canonical inventory (declared ports/signals), GATED to
    suppress legitimately-external/uppercase-noise cases; unit tests; wire into
    `validate`. Full CI.
  Acceptance: detector + tests + validate surface (gated); CI green.
  Verification: >
    DESCOPED to a future tree (`2026-06-01`). Real-corpus runs showed the
    discriminating problem is genuinely hard: the I2C run had an EMPTY
    actor-signal graph (inventory can be empty → naive symbol closure flags
    *everything*), and a legitimately-external signal referenced-but-undeclared is
    indistinguishable from a missed declaration without careful gating + corpus
    validation. Done crudely it is **noise, not signal** — which would violate the
    signoff bar. Deferred (not abandoned) so it gets the careful, corpus-validated
    design it needs rather than a rushed heuristic; recorded as a follow-on in the
    miss-detector catalog (`docs/research/miss-detectors-catalog.md` C1, GATED).
  Commit: `see Commit Log`

- ID: `COMPLETENESS-CLOSURE-INVARIANTS.4`
  Status: `done`
  Goal: book subsection (how the closure detectors work + why they matter, user-friendly) + close; refresh research `.7`/program status.
  Acceptance: book updated; full CI green; tree CLOSED.
  Verification: >
    passed (`2026-06-01`) — added the user-friendly `COMPLETENESS-CLOSURE-INVARIANTS`
    register bit-tiling subsection to `pipeline/evidenceir.md` (what it gives you,
    why it's an exact closure invariant, the conservative no-width-speculation
    design, and its double duty as the extraction-precision signal that exposed
    the register-classifier bug — closing the doc-drift gap, since the detector
    shipped live in `.2` without dedicated book coverage). With `.3` symbol closure
    descoped to a future careful tree, the tree CLOSES on the delivered + now-
    documented register-tiling detector. mdBook builds (docs CI green).
  Commit: `see Commit Log`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `COMPLETENESS-CLOSURE-INVARIANTS.1` | `done` | tree + design + research `.7` decision recorded |
| 2 | `COMPLETENESS-CLOSURE-INVARIANTS.2` | `done` | register tiling landed (`ir/completeness.rs` + validate); 8 tests; CI green |
| 3 | `COMPLETENESS-CLOSURE-INVARIANTS.3` | `descoped` | symbol closure → future tree (genuinely hard; would be noise if rushed) |
| 4 | `COMPLETENESS-CLOSURE-INVARIANTS.4` | `done` | register-tiling book subsection added (closed doc-drift gap); tree closed |

**Tree CLOSED `2026-06-01`** — register bit-tiling delivered (`.2`), real-corpus-
validated (precise: 1 TP + 0 FP/84), and now documented in the book (`.4`).
Symbol closure (`.3`) is descoped to a future careful, corpus-validated tree
(rushing it would produce noise, not signal — recorded in the catalog).

## Decisions

- `2026-05-31`: surface detectors as `validate` findings + metrics (not a new
  persisted report yet) — consistent with the existing conflict surfaces; the
  unifying `CompletenessReport` is deferred to research `.5` + a later tree.
- `2026-05-31`: register tiling checks only `[min_field, max_field]` (overlaps +
  interior gaps); no width speculation → exact, no width-induced false positives.
- `2026-05-31`: register tiling first (EXACT, lowest-risk), symbol closure second
  (GATED, needs careful inventory/reference definition + external-signal gating).

## Real-corpus validation (`2026-05-31`)

Ran the detector on a real downloaded spec (NXP I2C UM10204, via
`<owner local chip-doc corpus>`, ingested with Docling 2.84.0 →
`evidence` → `validate`). Result: the register-tiling detector **fired on real
data** — `register_field_overlaps: 1`, finding
`evidence_register_field_overlaps` (Warning). Inspection of the offending
"register" (`register_name = "0000 000"`, fields `0000 000` / `1111 1XX` …)
showed it is the I2C **reserved slave-address table** (UM10204 Table 4),
**mis-classified as a register map** by the upstream register-map table
classifier, with address bit-patterns parsed as field names + spurious bit
columns (two bogus fields both at bit [1] ⇒ the flagged overlap).

Two conclusions, both valuable:
1. The detector works on real specs and **doubles as an extraction-precision
   signal** — an overlap on garbage fields is a strong "this register extraction
   is wrong" indicator, exactly the kind of miss/issue the completeness program
   exists to surface (and it did so honestly, no fabrication).
2. It exposed a real upstream issue: the **register-map table classifier is
   over-eager** (an address-assignment table became a register). That is a
   candidate **future owned tree** (`register-map classifier precision`), now
   anchored in a concrete real example — not a guess.

Also confirmed live on the same spec: the `R15C-CONVERGENCE-REPORT` detector
(`converged in 1 pass`, honest) and the existing residual surfaces (59 partially-
structured normative statements; empty actor-signal graph despite 27 behavioral
rules; 126 visual assets with 0 timing-diagram extractions) — real, located miss
anchors for the next detectors (region accounting, symbol closure, cross-modal).

**Register-rich confirmation — eMMC JESD84-B50 (`2026-05-31`):** ingested (Docling)
→ evidence (6561 statements, **48 register_records**) → validate. Register tiling:
**0 overlaps, 0 interior gaps across 48 real registers** — i.e. **zero false
positives** on register-rich real data. Together with the I2C true-positive
(mis-classified table), this establishes the detector is **precise**: it fires on
the real issue and stays silent on clean register maps. The convergence report
also proved its value here: **2 passes, 194 genuinely-new facts** recovered then
converged (vs I2C's honest 0) — the metric distinguishes a spec where anchored
rescanning helps from one where it doesn't. (Real anchors: 259 nlp residuals; 3
semantic-role conflicts; 344 un-enriched visuals.)

## Open Questions

- Which stage owns symbol closure — EvidenceIR (relations/constraints present) vs
  SemanticIR/IntentIR (canonical `actor_ports` inventory present)? Likely the
  stage where BOTH the inventory and the references coexist; resolve in `.3`.
- Interior-gap severity: Info vs Warning? (Lean Info — a gap is a *candidate*
  missed field, not a proven defect; overlap = Warning, a real contradiction.)

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-31` | `.1` | tree created + registered; research `.7` decision (closure-invariants first) recorded; docs-only (CI invariant) | `passed` |
| `2026-05-31` | `.2` | `ir/completeness.rs` register-tiling detector (overlap + interior gap, conservative/no-width-speculation); 7 unit + 1 validate-wiring tests; `validate_evidence_ir` findings (Warning/Info) + 2 metrics; extraction-neutral; fmt/clippy clean; full `scripts/run_ci.sh` GREEN | `passed` |
| `2026-06-01` | `.3` | symbol closure DESCOPED to a future tree — genuinely hard (empty inventory → noise; missed-decl vs external indistinguishable without careful corpus validation); not rushed (signoff) | `descoped` |
| `2026-06-01` | `.4` | register-tiling book subsection added to `pipeline/evidenceir.md` (closed the doc-drift gap; `.2` shipped live without dedicated coverage); mdBook green; tree CLOSED | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `COMPLETENESS-CLOSURE-INVARIANTS.1` | `COMPLETENESS-CLOSURE-INVARIANTS.1 — own the first completeness detectors + record the first-slice decision` | docs-only |
| `COMPLETENESS-CLOSURE-INVARIANTS.2` | `COMPLETENESS-CLOSURE-INVARIANTS.2 — register bit-tiling detector (ir/completeness.rs) + validate surface` | code; first completeness detector landed |
| `COMPLETENESS-CLOSURE-INVARIANTS.{3,4}` | `COMPLETENESS-CLOSURE-INVARIANTS.4 — register-tiling book note; descope symbol closure; close` | book; `.3` descoped to future tree |

## Changelog

- `2026-06-01`: `.4`+`.3` — added the register-tiling book subsection (closed the
  doc-drift gap); descoped symbol closure (`.3`) to a future careful tree.
  **Tree CLOSED** on the delivered + documented register-tiling detector.
- `2026-05-31`: `.2` — landed the register bit-tiling detector in a new
  `ir/completeness.rs` (overlap + interior gap, conservative), surfaced in
  `validate` (findings + metrics); 8 tests; CI green. **The first completeness
  miss-detector is live.** Frontier → `.3` (symbol closure, gated).
- `2026-05-31`: Created — first implementation from `INTENT-COMPLETENESS-RESEARCH`
  (user chose closure invariants). Register tiling (EXACT) then symbol closure
  (GATED), surfaced as `validate` findings. Frontier → `.2` (register tiling).
