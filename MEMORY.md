# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`EXTRACTION-QUALITY-GAUGE.3k.3` CLOSED `2026-09-13` (CODE)** — **one record per OBLIGATION, not one per statement.**
  The node said "narrow the kind's span"; re-derivation found that alone would REFUSE AHB `sigcon_0002`, whose first modal clause states
  no kind — the defect was the record's GRANULARITY. Three readings moved with it: a FRONTED condition leaves no subject part
  (`text_before_condition_marker` cuts at offset 0), the subject fallback scanned the whole statement, and
  `is_post_passive_binding_only_subject` judged the Nth record against the 1st record's clause. Corpus **187 → 211 replayed**; the only
  persisted records lost are NVMe `sigcon_0005`/`0006`, the leaf's own population. Rebuilt AHB 13 → 13 (condition CORRECTED),
  APB-E 23 → 27, AXI-L 40 → 53 — including `WTAGUPDATE must_be_value UPDATED` removed, closing `.3k.2b`'s named residual.
- **The method is the result: BUILD THE CHANGE AS A PROTOTYPE AND MEASURE IT BEFORE COMMITTING TO ITS DESIGN.** The first prototype
  fabricated (`ACADDR must_be_asserted` from a clause whose subject is the pronoun `it`); the second confirmed the container's ordering
  rationale by publishing `must_be_value UNIQUE`, which opened and landed `.3k.2k` first. Neither was predictable from reading the code.
- Next action: **`EXTRACTION-QUALITY-GAUGE.3k.4`** (the dynamic path's span discipline — its clause is BINDING-bearing, not modal, so
  `constraint_bearing_sentence` is the wrong narrowing and no helper computes the right one yet). Four leaves `.3k.3` opened are also
  ready and each carries its own measured sizing: `.3k.7` (AXI `WSTRB` — the obvious repair was measured and costs 3 reproduced records),
  `.3k.8` (statement/row duplication, 2 records before `.3k.3` and 6 after), `.3k.9` (`PARTITION\_ACCESS` escaped-underscore fragment),
  `.3k.10` (a statement-initial fronted condition has no leading space — and its first comma is a LIST separator, so the cheap repair
  drops a correct subject).
- In-flight uncommitted: none after this commit.
- Blockers: none. Push cadence is **400** commits per director directive `2026-09-13` and is FIXED there, so no push is due at 233;
  directive 16 still gates it on full CI. `scripts/check_doctrines.sh --all` did not finish in 50 minutes — `CHAIN-CURRENCY` re-executes
  the real pipeline for every persisted artifact across four stages. Budget hours, not minutes, and run it detached. Standing hazards:
  **an evidence-stage change that MOVES a current-schema artifact's content stales its proof and it then refuses to LOAD** — only
  AHB/AXI-L/APB-E are current-schema; a change that moves nothing leaves all 77 loadable (`.3k.2k` did). Rebuild is `evidence → validate →
  semantic → validate → intent → validate → adapt`, each validated exactly once, upstream-first; **the evidence stage needs the normalized
  bundle**, so restore from `generated/preserved/WIRE-BASED-100.10/{ahb,axi,apb}-normalized-bundle-held-out` into
  `generated/source_ir/<doc>/normalized`, rebuild, `diff -rq` against the held-out copy, remove, retention back to **24**.
  A tool reading a corpus root REFUSES an off-volume path (`/private/tmp`) — stage comparison artifacts under `generated/tmp/`.
  **Run `replay-constraints` before sizing any extractor change** (`[[persisted-census-measures-published-not-current]]`) and read
  `row_stratum_unjudged_documents` with it (`[[legacy-source-classifications-are-neutralized-on-load]]`). **A census counts the population
  of the FUNCTION being changed** (`[[constraint-record-producer-strata]]`) and **never from a mirror**
  (`[[one-modal-vocabulary-per-constraint-record]]`). **A control that asserts a record EXISTS pins whatever that record says, fabrication
  included** (`[[a-relational-predicate-is-not-a-value]]`). **A Rust change moves `flow_census.json` and the re-derivation must ATTRIBUTE
  the delta to the owning leaf**; a new module moves `module_inventory.tsv`; a new command `CLI_SURFACE_REGISTRY`; a new raw-evidence
  reader `information_flow_boundary.tsv` — all fail closed. **Editing a live surface sets off a fixed bookkeeping chain** —
  `[[live-surface-edit-bookkeeping-chain]]`: fact-card catalog `--print-plan` into `planned_outputs` then `--write`, the
  `fact-card-catalog-count` assertion plus its INDEX line-3 region, the `shipped_behavior` aggregate authority, the three
  `surface_registry` pins, then the claim digests — and the claim IDs that move ARE the `Published-claims:` declaration.
  **A book section about an extraction rule goes in its concern's chapter, not the stage chapter.** **The doctrine driver runs no cargo
  gate** (`[[doctrine-driver-runs-no-cargo-gate]]`). Never run the fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`).
