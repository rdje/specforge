# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`EXTRACTION-QUALITY-GAUGE.3k.11` CLOSED `2026-09-13` (CODE)** — **a logic level belongs to a SIGNAL, not to the verb
  that sets it.** The retired `logic_level_binding_kind_from_text` took the LAST level within six words of a binding verb and attached it
  to EVERY declared signal the statement named; `| P_ACCEPT | … Controller must set PREQ LOWand PREQCHK HIGH |` published
  `PREQ must_be_high`. Corpus replayed is **304 before and 304 after**, and the composition is the result: **11 fabrications removed, 15
  correct records added, 1 correct record lost and named** (`.3k.12`). All 22 reproduced logic-level records plus every added/removed one
  adjudicated individually in the node. AHB 13 → 12 and AXI-L 53 → 55 rebuilt.
- **Three "obvious" rules were each refuted by the corpus, and one by an existing control.** Nearest-PRECEDING is wrong (a signal can
  follow its level). Recognising an identifier by its SHAPE is wrong — `WIRE-BASED-100.5i`'s alpha-invariance control went RED and forced
  identity to be read through the document's own catalog, which is the only ADR-0006-safe answer. Clean word boundaries are wrong: the
  normalizer loses the space in `LOW and`, so a level is read on a token's leading uppercase run too.
- Next action: pick from the open `.3k` leaves, each carrying its own already-measured sizing — **`.3k.5`** (clause-scope the two
  vocabulary refusals; 4 statements measured), `.3k.7` (AXI `WSTRB`; the obvious repair costs 3 reproduced records), `.3k.8`
  (statement/row duplication, 2 records before `.3k.3` and 6 after), `.3k.9` (`PARTITION\_ACCESS` escaped-underscore fragment),
  `.3k.10` (statement-initial fronted condition), `.3k.12` (a predicate between a signal and its level), `.3k.13` (the dynamic path has
  no MODALITY gate — AHB publishes `HPROT must_be_high` from *"It is recommended that…"*). `.3j` (the LLM path's gates) is open and unsized.
- In-flight uncommitted: none after this commit.
- Blockers: none. Push cadence is **400** commits per director directive `2026-09-13` and is FIXED there, so no push is due at 235;
  directive 16 still gates it on full CI. `scripts/check_doctrines.sh --all` did not finish in 50 minutes — `CHAIN-CURRENCY` re-executes
  the real pipeline for every persisted artifact across four stages. Budget hours, not minutes, and run it detached. **The method that is
  paying for itself: BUILD THE CHANGE AS A PROTOTYPE AND MEASURE IT WITH `replay-constraints` BEFORE COMMITTING TO ITS DESIGN.** Four
  leaves in a row had their shipped shape decided by a measurement that contradicted the node, and none was predictable from reading the
  code. Standing hazards: **an evidence-stage change that MOVES a current-schema artifact's content stales its proof and it then refuses
  to LOAD** — only AHB/AXI-L/APB-E are current-schema. Rebuild is `evidence → validate → semantic → validate → intent → validate →
  adapt`, each validated exactly once, upstream-first; **the evidence stage needs the normalized bundle**, so restore from
  `generated/preserved/WIRE-BASED-100.10/{ahb,axi,apb}-normalized-bundle-held-out` into `generated/source_ir/<doc>/normalized`, rebuild,
  `diff -rq`, remove, retention back to **24**. A tool reading a corpus root REFUSES an off-volume path (`/private/tmp`) — stage
  comparison artifacts under `generated/tmp/`. **A census counts the population of the FUNCTION being changed**
  (`[[constraint-record-producer-strata]]`) and **never from a mirror** (`[[one-modal-vocabulary-per-constraint-record]]`). **A control
  that asserts a record EXISTS pins whatever that record says, fabrication included**
  (`[[a-relational-predicate-is-not-a-value]]`). **A Rust change moves `flow_census.json` and the re-derivation must ATTRIBUTE the delta
  to the owning leaf**; a new module moves `module_inventory.tsv`; a new command `CLI_SURFACE_REGISTRY`; a new raw-evidence reader
  `information_flow_boundary.tsv` — all fail closed. **Editing a live surface sets off a fixed bookkeeping chain** —
  `[[live-surface-edit-bookkeeping-chain]]`: fact-card catalog `--print-plan` into `planned_outputs` then `--write`, the
  `fact-card-catalog-count` assertion plus its INDEX line-3 region, the `shipped_behavior` aggregate authority, the three
  `surface_registry` pins, the book's line-pinned regions re-anchored BY CONTENT, then the claim digests — and the claim IDs that move
  ARE the `Published-claims:` declaration. **A book section about an extraction rule goes in its concern's chapter, not the stage
  chapter, and a changed rule leaves standing book text.** **The doctrine driver runs no cargo gate**
  (`[[doctrine-driver-runs-no-cargo-gate]]`). Never run the fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`).
