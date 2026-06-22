# Conditional-rule lowering triage — `DOC-INTENT-TAXONOMY.4e` (the `.2` Result 3 per-item triage)

- Tree leaf: `DOC-INTENT-TAXONOMY.4e`
- Date: `2026-06-23`
- Type: measurement triage (read-only, docs-only — no Rust code, no canonical-artifact mutation)
- Reinforces: `[[project_kg_isf_completeness]]`, `[[project_doc_intent_taxonomy]]`, `[[feedback_scoring_rigor]]`,
  `[[feedback_isf_no_hacks]]`

## The question

`.2` Result 3 flagged that the `constraints + temporal + conditional → (rule)` lowering ratio is low (cat 1
~41% / cat 2 ~45% / cat 3 ~11% / cat 4 ~22%), **dominated by `conditional_rules`**, and explicitly deferred
the verdict: *"part of this is honest (a conditional that names no signal obligation cannot become an ISF rule
without fabrication) and part is a lever (some conditionals could lower with a richer guard mapping) … needs
per-item triage in a dedicated `.4+` leaf before any fraction here is called a pure gap."* This leaf does that
triage, per item, so the conditional-rule shortfall is correctly classified as **honest residual vs a real
buildable lever** — never assumed.

## Method (read-only, reproducible)

Over 9 representative docs spanning all four buildable categories (cat-1 AHB/APB/AXI, cat-2 NVMe/RISC-V IOMMU,
cat-3 GIC-600/CoreSight SoC-600, cat-4 RISC-V Debug/AIA), I classified every `conditional_rules` entry against
the document's own **declared-signal inventory** (the union of `interfaces[].signals`,
`interfaces[].signal_records[].signal_name`, and `actor_ports[].signal_name`) and the consequent's quality:

- **A — no consequent signal** (`consequent_signal` empty): a prose condition with no signal obligation.
- **B — consequent names an undeclared signal**: references a signal the document does not declare.
- **C — declared consequent, placeholder action** (`consequent_action` empty or `(see source_text)`/`n/a`).
- **D — declared consequent + non-placeholder action**: the only candidate-lever bucket; further split by
  whether the action carries a **concrete** value/level/edge cue (`0b…`/`high`/`asserted`/`set to`/`== 1` …)
  versus a **bare deontic modal** (`shall`/`must`/`shall not`).

A conditional lowers to an ISF `(rule)` only when it names a declared signal **and** a concrete obligation
(value/level), so A/B/C are honest residual by construction; D is the only place a lever could hide.

## Measured evidence (603 conditional_rules, 9 docs)

| Bucket | Count | Share | Lowerable? |
|---|---|---|---|
| A — no consequent signal (prose condition) | 392 | 65.0% | No — honest residual |
| B — consequent names an undeclared signal | 14 | 2.3% | No — honest residual (would fabricate a signal) |
| C — declared consequent, placeholder action | 33 | 5.5% | No — honest residual (no obligation stated) |
| D — declared consequent + non-placeholder action | 164 | 27.2% | **Candidate lever — examined below** |

**The D bucket is not a clean ISF-lowering lever.** Of the 164, the `consequent_action` distribution is:
`shall` (45), `must` (35), `shall not` (14), `shall be cleared` (10), `must not` (7), `must be 4` (2),
`shall be 1` (2), … — i.e. **161 of 164 are bare deontic modal fragments** carrying no concrete signal
obligation, and **only 3 carry a concrete value/level cue** (`must be 4`, `shall be 1`, one more). A bare
`shall`/`must` against a declared signal is not a lowerable obligation: rendering it to an ISF `(rule)` would
require **synthesizing the value/level the document states only in prose** — fabrication, forbidden by
`[[feedback_isf_no_hacks]]`.

**Net:** `~439/603 (73%)` of conditional_rules are unambiguous honest residual; the remaining `~27%` are
deontic-modal fragments whose concrete obligation was not extracted, with only `~3/603 (0.5%)` carrying a
concrete cue. The ISF adapter already lowers the cleanly-grounded conditional obligations (516 corpus-wide per
`[[behavior-temporal-lowering-broader-corpus]]` / KG-ISF-COMPLETENESS.4); `signal_constraints` and
`temporal_rules` (the clean signal-obligation surfaces) lower well.

## Decision

**The conditional-rule lowering shortfall is NOT an ISF-completeness gap — it is honest residual.** Result 3
closes: there is **no buildable ISF-lowering lever and no FSMGen FR** for conditional rules. The shortfall is
(a) prose conditions that name no signal obligation (A, 65%), (b) undeclared-signal or placeholder consequents
(B+C, 8%), and (c) deontic-modal conditionals whose concrete obligation lives only in prose (D, 27%) — none of
which can become a faithful `(rule)` without fabrication. The adapter's silent skip of these is **correct**
(the same conclusion `.2`/`.2b` reached for the broader rule surface).

**The only upside is upstream EXTRACTION quality, not ISF lowering.** The D-bucket modal conditionals are
exactly where the grounded constraint extractor (`extract-constraints-llm`, the LLM-primary promotion path /
`EXTRACTION-QUALITY-GAUGE`) could, in principle, recover the concrete obligation from the conditional's
`source_text` — at which point it would lower automatically via the existing `(rule)` path, with **no new ISF
construct**. That is a constraint-extraction precision/recall question owned by the extraction-quality program,
distinct from the `.4` ISF-lowering program and **far lower-leverage** than the register/structure/topology
levers (Gap A done; Gap B; cat-3 `.4c.i`; cat-4 `.4d.i`). It is recorded here as a cross-reference, **not**
minted as a `.4` gap — calling a 0.5%-concrete residual a "gap" would violate `[[feedback_scoring_rigor]]`.

With `.4e` closed, the `.2` per-category ISF-completeness scorecard's **measurement phase is complete**: Gap A
lowered (`.4a.ii`), Gap B carrier identified (`.4b`, FSMGen-gated), cat-3 topology decided (`.4c` → `.4c.i`
measurement), cat-4 ISA decided (`.4d` → `.4d.i` extraction lever), conditional rules triaged (`.4e`, honest
residual). What remains is **CODE** (`.4d.i`, `.4c.i`, `.4b`), not measurement.

## Genericity (ADR 0006)

The triage keys off structural signal-declaration membership and consequent-action shape (concrete cue vs bare
modal) — no chip/vendor/protocol-instance name list. Reproducible from `generated/intent_ir/<key>/intent_ir.json`.

## Gates (this leaf)

- No Rust code, no canonical-artifact mutation → wire golds / `kg-bench` / emitted `.isf` byte-identical by
  construction (WIRE-BASED-100 orthogonal).
- `scripts/check_doctrines.sh` green (memory-arch + knowledge-map + task-acceptance); `mdbook build` green;
  knowledge-map derive-and-diff in sync after adding the `[[conditional-rule-lowering-triage]]` fact card.
- Objectively measured, per-item demonstrated (`[[feedback_scoring_rigor]]`): every count is read directly off
  the persisted corpus.
