# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`EXTRACTION-QUALITY-GAUGE.3k.6` CLOSED `2026-09-12` (CODE)** — `specforge replay-constraints <evidence-ir>` /
  `--evidence-root <root>`: re-runs the REAL deterministic producer over a persisted artifact's own `extracted_statements` and says, per
  published record, whether today's code still mints it and which gate stands in the way. It reaches the frozen 54 because the constraint
  surface is a function of the STATEMENTS, not the PDF, and it reads the legacy stratum via `load_for_inspection`.
  **Corpus answer: 144 of 179 published deterministic records reproduce; 35 do not** (17 no gate — kind/condition/negation moved; 12
  `CORPUS-COVERAGE.2.50a`; 4 `.3k.1`+`.2.50a`; 1 `.3h`; 1 `.3g`), 67 subjects needed a granted declaration, 1 artifact a NAMED skip (I2C,
  current-schema with a stale proof). Calibrated both ways: APB 15/15 and AHB 13/13 (rebuilt by `.3i`), CXS 0/2 (the exact shape `.3i`
  retyped). Open: `.3k.2`-`.3k.5`; **`.3j`**; `INVARIANT-SHAPE-ADMISSION.4`; `PROSE-NAME-CELL-DECLARATION.3`; `SIGNAL-DECLARATION-ROW-DROP`
  `.2c`/`.2d`; `RETAINED-BUNDLE-POPULATION-FROZEN` `.1`-`.3`; `KG-ISF-COMPLETENESS` beyond `.5`; `LIVE-DOCUMENT-PRESSURE-HEADROOM`
  `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`; `SOURCE-IR-REPRODUCIBILITY` `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`;
  `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`; `WIRE-BASED-100`
  `.10d`/`.10f`/`.2`/`.3`/`.5` (SWD `13/29`).
- Current state: three commits this session, each one correcting a number the previous leaf published. `.3k` — stratify a census by the
  PRODUCER the rule edits (`[[constraint-record-producer-strata]]`). `.3k.1` — a persisted census measures what was PUBLISHED, not what the
  code does (`[[persisted-census-measures-published-not-current]]`). `.3k.6` — the instrument for that, and **its own first number was wrong
  too**: the catalog widening seeded the `HashSet` the producer takes, which only the inference-antecedent sibling reads, so 120/179 became
  144/179 once the widening used declaration STATEMENTS. A control written before publishing caught it. 281 fact cards (6 title parts, at the
  declared `max_parts`); 15 registered doctrines, 13 at gate tier; core lib 1,445.
- Next action: **`EXTRACTION-QUALITY-GAUGE.3k.2`** — re-size it with `replay-constraints` FIRST; its published population (26 untyped
  `MustBeStable` defaults + 11 ungated `generic_value` records, all `sigcon_*`) is a published count and the actionable one will be smaller.
  It decides what a clause that types nothing may publish, and it must land before `.3k.3` (the kind's span, 4 published) or narrowing the
  span moves NVMe onto a fabricated `must_be_value UNIQUE`. Then `.3k.5` (the refusal gates are statement-scoped while the records they
  suppress are clause-scoped — 4 of 181 equality statements differ), then `.3k.4` (the dynamic path's clause; it needs a BINDING-bearing
  clause since its records need not contain a modal).
- In-flight uncommitted: none after this commit.
- Blockers: none. Standing hazards: **run `replay-constraints` before sizing any extractor change** — 54 of 78 documents cannot be rebuilt,
  so a third of the published constraint surface is not what this code would produce. **A census counts the population of the FUNCTION being
  changed** (`[[constraint-record-producer-strata]]`). **Both deterministic paths derive their own catalog from the statements** — the
  `declared_signals` parameter of `extract_normative_signal_constraints` reaches only the inference-antecedent sibling. **A new Rust module
  must be added to `doctrine/production_genericity/module_inventory.tsv`, a new CLI command to the `CLI_SURFACE_REGISTRY` in
  `commands/converge.rs`, and any new raw-evidence reader to `information_flow_boundary.tsv`** — all three fail closed, and the flow census
  baseline+delta must then be re-derived and attributed. **A new Markdown file must be `git add`ed before `check_live_document_size.pl` sees
  it.** **An obligation binds to the token immediately before its modal.** **`specforge evidence` says only `path does not exist:
  …/normalized/<key>.md`** (`[[evidence-rule-field-content-stales-every-proof]]`). **Grep `KNOWLEDGE_MAP.md` for the wall** —
  `[[live-surface-edit-bookkeeping-chain]]`. **The doctrine driver runs no cargo gate** (`[[doctrine-driver-runs-no-cargo-gate]]`). Never run
  the fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`). This file's cap is 50 lines.
