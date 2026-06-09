# EXTRACTOR-ARCHITECTURE: make the EvidenceIR extractor path a coherent whole

## Metadata

- Tree ID: `EXTRACTOR-ARCHITECTURE`
- Status: `active`
- Roadmap lane: `R0` (engineering coherence) / `R15`/`R16` (extraction breadth substrate)
- Created: `2026-06-09`
- Parent: owner directive (`2026-06-09`, devil's-advocate review) — "the extraction path seems fragile … it
  should be a coherent set of properly defined types, structures to ideally handle any chip-spec PDF … we
  need to build a coherent whole that doesn't grow erratically in every directory." Scope clarified by the
  owner to **the Extractor path** (the layer that POPULATES the typed IR), not the IR types themselves.

## Goal

Refactor the EvidenceIR **extractor layer** from a flat bank of ~60 free functions wired imperatively inside
one ~500-line `EvidenceIr::build()` into a **coherent, typed extractor framework**: a registry of named
`Extractor` units, each declaring the IR surface it produces, an explicit applicability gate, its grammar,
and its provenance tier; run by ONE driver that handles ordering, merge/dedup, conflict, and provenance
**uniformly** and emits an **inspectable run manifest**. New PDF families then grow by *registering one unit*
in one place — never by editing a god-function or sprinkling a new inline dedup loop. Behavior-preserving and
incremental: the grammars and the IR target are sound and stay; only the *structure that wires them* changes.

## Non-goals

- NOT a rewrite of the grammars/matching logic (they are sound; `.9.x`/`WIRE-BASED-100` proved them) — only
  the orchestration/wiring shape.
- NOT a change to the typed IR target (`EvidenceIR`/`SemanticIR`/`IntentIR` are already coherent — owner
  scoped this to the extractor path).
- NOT a regression: APB/AHB/AXI/SWD stay 100%; kg-bench 151/151; every migration slice proves byte-identical
  (or explicitly-justified) output before/after via the existing eval + kg-bench.
- NOT a behavior change in any single migration slice — structure first, capability later.

## Acceptance criteria

- A typed `Extractor` abstraction + `ExtractionContext` + per-surface merge/conflict policy + a single driver
  + an inspectable `ExtractionRun` manifest, all unit-tested.
- Each migrated cluster: same records out (proven by kg-bench + per-fact eval), fewer call-site lines, no
  inline dedup loop, explicit `applies_to`, uniform provenance.
- Adding a new extractor is a one-place registration (demonstrated by porting one `.9.x` grammar onto it).
- Full `scripts/run_ci.sh` green; KM card per durable structural fact.

## `.1` — Read-only architecture audit of the extractor path (THIS slice, `2026-06-09`)

**Method:** enumerated every extractor/synthesizer/helper in `crates/specforge/src/ir/evidence.rs`
(16,656 lines) via grep, then read the `EvidenceIr::build()` orchestration (≈ lines 500–1040) and the
representative cluster bodies. Findings below are the empirical basis for the `.2`+ design.

### Inventory (the producers, by surface)

| IR surface | strategy variants (free fns) | merge/dedup | applicability gate |
|---|---|---|---|
| declared signals | `synthesize_declarations_from_tables`, `synthesize_signal_declarations_from_prose`, `synthesize_signal_declarations` | `.extend` + downstream dedupe; prose runs only if `table_signal_count < 8` (magic threshold, inline) | sparse-catalog threshold (implicit) |
| actor↔signal relations | `extract_relations_from_signal_tables`, `extract_actor_signal_relations`, `synthesize_directions_from_relations` | inside the converge loop | prose/table shape |
| signal constraints | `extract_signal_constraints`, `extract_dynamic_signal_constraints`, `extract_conditional_rules` | inside the converge loop | prose shape |
| signal polarity | `extract_signal_polarity_from_prose`, `extract_signal_polarity_from_signal_tables`, `collect_signal_polarity_facts` | merge in converge loop | prose/table |
| semantic hints | `synthesize_signal_semantic_hints` → `_from_tables` / `_from_prose` / `_from_visual_evidence` | **a real dispatcher fn** + `signal_semantic_hint_key` dedup + `detect_signal_semantic_conflicts` | known-signals/actors |
| **FSM states** | `extract_protocol_states`, `extract_swd_line_states`, `extract_quoted_mode_states`, `extract_transition_bound_states` | **inline dedup-by-name loops at the call site (one per reader)** | keyword gate / serial-doc gate / self-gate — all different |
| serial frame | `extract_serial_frame_fields` | n/a (single) | serial-doc gate |
| operations | `extract_swd_operations` | n/a | serial-doc gate |
| registers | `synthesize_register_records`, `synthesize_register_field_tables` + post-hoc `retain`/`consolidate_register_field_fragments`/size-fill | `.extend` + imperative post-passes at call site | table kind |
| actors | `extract_protocol_actors` (+ `agent_definitions`) | dedup inside | prose shape |
| timing | `synthesize_timing_constraints` | n/a | table kind |
| encoding | `synthesize_encoding_declarations*` | inside converge loop | table kind |

### Findings (the structural facts)

1. **Coherent TARGET, incoherent PRODUCERS.** Every reader funnels into a stable typed record
   (`ProtocolStateRecord`, `InterfaceSignalRecord`, `SignalConstraint`, …) and downstream is clean — the IR
   is *not* a pile. The problem is purely the *producer layer*.
2. **The recurring shape already exists — implicitly, and reinvented per surface.** A "strategy" is
   `(context, prior_guidance) -> Vec<Record>`; a "merge" is a `*_key` + dedup; some surfaces add a conflict
   detector. `synthesize_signal_semantic_hints` is the **best** instance (an explicit dispatcher + key-dedup
   + conflict detector). The **FSM cluster is the worst**: four sibling free functions glued by *four
   separate inline dedup-by-name loops at the call site* (the 4th added by `.9.7` this same session — a live
   demonstration of the erratic-growth failure mode the owner flagged).
3. **`EvidenceIr::build()` is a ~500-line god-orchestrator** doing discovery + per-surface strategy
   invocation + merge + post-passes + provenance imperatively; each new surface/strategy edits it.
4. **Applicability gating is heterogeneous and implicit:** keyword gate (`extract_protocol_states`),
   content gate (`extract_swd_line_states` needs "swdio"/"serial wire"), magic threshold (signals `< 8`),
   structural self-gate (`.9.3a`/`.9.7`). There is no single "which extractors apply to this document?" view.
5. **Provenance is partial.** `fact_provenance` is tagged only for `SignalConstraint` + `ActorSignalRelation`
   (the recall-gauge surfaces); the other ~8 surfaces carry no uniform "which extractor produced this" tag.
6. **No introspection / manifest.** You cannot ask the system "which readers were eligible, which fired,
   what each contributed, where two disagreed" — you read code. This is the root of the "fragile / grows in
   every direction" feeling: the wiring is invisible and edited in many places.

**Verdict:** the extractor path scales *additively-safely* (additivity is why nothing regresses) but **not
elegantly** — growth is by accretion of free functions + call-site edits, which is exactly "erratic growth in
every direction." The fix is a refactor (not a rewrite) that promotes the implicit strategy/merge/gate shape
into a first-class, registered, inspectable framework. Verification: read-only; no code change. Commit: this slice.

## `.2` — The `Extractor` framework (owner-confirmed `2026-06-09`: "consolidate, unify, as much as possible")

**Owner decision (`2026-06-09`):** "the extractor path needs to be consolidated, unified, as much as
possible" → build the MAXIMAL-coherence shape (the trait + registry + driver below), not a light-touch
variant. Build un-gated.
**Status: `done` (`2026-06-09`).** Scaffolding landed in `crates/specforge/src/ir/extractor.rs`: the generic
`Extractor<R>` trait (`name`/`tier`/`applies_to`/`run`), the borrowed `ExtractionContext` (carries only
`statements` for now — grows one field per migrated cluster, no speculative fields), the single `run_surface`
driver (gate → run → first-wins dedup by a per-surface key → manifest), and the inspectable
`SurfaceRun`/`SurfaceManifest`/`ExtractionManifest` types. 4 hermetic driver tests (first-wins dedup,
gate-skips-run, registry-order precedence, aggregate manifest). **ZERO extractors migrated → no behavior
change**; `pub` lib API so no dead-code warning. fmt + clippy `-D warnings` clean; `run_ci.sh` green (lib
1448 → 1452). `.3` migrates the FSM cluster onto it (byte-identical, the first real consolidation).

```
ExtractionContext            // built ONCE: source_ir, statements, known signals/actors, alias map,
                             // visual evidence, prior_guidance, provider availability
trait Extractor {
    fn name(&self) -> &str;                  // stable id (e.g. "fsm.transition_bound")
    fn surface(&self) -> Surface;            // which typed IR surface it produces
    fn tier(&self) -> ExtractorTier;         // Pattern | Llm | Vlm  (uniform provenance for ALL surfaces)
    fn applies_to(&self, cx: &ExtractionContext) -> bool;   // the EXPLICIT, inspectable gate
    fn run(&self, cx: &ExtractionContext) -> SurfaceFacts;  // the grammar (existing fn body, moved)
}
SurfacePolicy { key(fact) -> Key; detect_conflicts(&[fact]) -> Vec<Conflict> }   // ONE per surface
Driver: build cx → for each registered extractor where applies_to → run → merge by SurfacePolicy →
        record ExtractionRun { eligible, fired, per-extractor counts, dedup_drops, conflicts } → uniform provenance
```

- The FSM cluster becomes 4 `Extractor` impls + one `ProtocolStateSurfacePolicy` (the dedup-by-name already
  written) — the 4 inline call-site loops collapse to a registration list.
- The LLM/VLM tiers are *the same frame*: extractors with `tier = Llm/Vlm` and an `applies_to` that checks
  provider availability; the "model proposes → declared signals/structure decide" rule becomes a shared
  driver post-filter (preserving the bounded-hypothesis-generator doctrine).
- `validate`/`audit-extraction` gain an inspectable manifest ("FSM: 4 eligible, 1 fired, 4 records, 0
  conflicts") — killing the introspection gap.

## Migration plan (incremental, behavior-preserving — one cluster per slice)

- `.2` scaffolding (types + registry + driver + manifest), migrate ZERO extractors (additive frame + tests).
- `.3` migrate the **FSM cluster** first (worst offender + freshly touched) — byte-identical output gated by
  kg-bench + the SWD/CAN/SWP re-measure. **DONE (`2026-06-09`).** The four FSM grammars are now
  `Extractor<ProtocolStateRecord>` units (`fsm.jtag_tap` / `fsm.swd_line` / `fsm.quoted_mode` /
  `fsm.transition_bound`) run by `protocol_state_surface` via the `.2` `run_surface` driver; the **four inline
  dedup loops at the `build()` call site collapsed to one call** (~24 lines → 1). Grammar bodies UNCHANGED
  (refactor of the wiring only). **Byte-identical proven on fresh-rebuilt evidence:** SWP 4
  (`DEACTIVATED`/`ACTIVATED`/`SUSPENDED`/`HALT`, same ids), CAN 3 (`mode_state_*`), SWD/ADI 13 (8
  `protocol_state_*` + 5 `swd_line_state_*`, same ids) — confirming the jtag↔swd_line uppercased-name dedup is
  a no-op (no collision). SWD FSM eval `serial_frame_field`/`swd_operation`/`protocol_state` all
  `P=R=F1=1.000`; kg-bench 151/151; full `run_ci.sh` green (lib 1452 → 1454). 2 new hermetic tests
  (cross-grammar union + dedup; non-FSM prose → empty). First real consolidation; the pattern is proven.
- `.4` migrate the **semantic-hints cluster** (already a near-dispatcher — the easy-win second proof).
  **DONE (`2026-06-09`).** The three meaning-inference strategies (signal-description tables, prose /
  alias-grounded prose, visual captions / VLM timing-diagram annotations) are now
  `Extractor<SignalSemanticHintRecord>` units (`semantic_hints.tables` / `.prose` / `.visual`) run by
  `signal_semantic_hint_surface` via `run_surface`; the hand-rolled "run tables → dedup-append prose →
  dedup-append visual" merge collapsed to one driver call (key = `signal_semantic_hint_key`, order =
  tables→prose→visual). Each strategy carries its cluster-specific inputs (derived `known_signals`/
  `known_actor_names`, alias map, visual evidence, source, prior guidance) on its extractor struct; the shared
  `ExtractionContext` supplies `statements` to the prose path. Conflict detection stays a post-merge step.
  **Byte-identical proven on fresh-rebuilt evidence:** RISC-V Debug (5 hints), SWD/ADI (3), I2C (1) — the
  `signal_semantic_hints` + `signal_semantic_conflicts` surfaces md5-identical; kg-bench 151/151 (its
  table-based semantic-hint fixtures confirm the now-uniform within-strategy dedup drops no real record); the
  13 existing semantic-hint unit tests pass through the migrated path. full `run_ci.sh` green (lib 1454).
  (Follow-up noted: a test-only `SourceIr` constructor would let surface helpers be unit-tested in isolation;
  today they're covered by the markdown-built integration tests + the corpus diff.)
- `.5` the **signals cluster**. **DONE (`2026-06-09`) — with a key re-categorization finding.** Investigating
  the signals cluster revealed it is NOT a `run_surface` merge-surface: it produces seed `ExtractedStatement`s
  (not typed surface records), mints synthetic statement ids via the **build-wide** `statement_counter` (shared
  with the main prose loop + the contract synthesizer), emits a provenance **side-output**, has a
  **cross-strategy fallback gate** (prose runs only if `table_signal_count < 8`), and concatenates with **no
  key-dedup**. That is the **statement-ASSEMBLY phase**, distinct from the typed **surface-extraction phase**
  the `Extractor`/`run_surface` framework serves — so forcing it through the dedup driver would be a hack
  (losing the side-output + gate, abusing the key). The honest consolidation: extract the seed orchestration
  out of the ~500-line `build()` into one cohesive `synthesize_signal_declaration_seed(...)` (table strategy +
  sparse-catalog prose fallback → seed statements + provenance), and DOCUMENT the **two-phase / two-category**
  model in `ir/extractor.rs` so the remaining migrations target the genuine merge-surfaces. Pure extraction
  (identical logic/order/counter threading). **Verification:** full `evidence_ir.json` BYTE-IDENTICAL on the 4
  DETERMINISTIC intact-bundle docs (RISC-V Debug table-signal path, I2C prose path, SWP, CAN); kg-bench
  151/151; lib 1454; full `run_ci.sh` green. **DISCOVERY:** SWD/ADI's evidence differed — root-caused to
  **pre-existing CONTENT-level non-determinism** (two runs of the *unchanged* `.4` code produce set-different
  `actor_signal_relations` + `extracted_statements`), NOT a `.5` regression. Spun a dedicated tree
  `EVIDENCE-DETERMINISM` for it (it undermines reproducibility, eval-score stability, and the byte-identical
  methodology). Remaining surface clusters to migrate (`.6`+): **registers, actors, signal-polarity**.
- `.6` the **registers cluster**. **DONE (`2026-06-09`) — and added a second driver mode.** Registers is a
  CONCAT surface (two strategies — register-map tables + `unknown` register-FIELD tables — producing disjoint
  `RegisterRecord`s merged by plain `.extend()`, NOT a key-dedup, then three post-passes: width fill-in,
  bit-layout-grid drop, fragment de-fragmentation). So the framework gained a second driver mode
  **`run_surface_concat`** (ordered, no-dedup, same `SurfaceRun` manifest) beside the key-dedup `run_surface`.
  The two strategies became `Extractor<RegisterRecord>` units (`registers.register_map` / `registers.field_table`,
  inputs on the struct), run by `run_surface_concat` inside a `register_record_surface` helper that then applies
  the three post-passes; the inline `build()` block collapsed to one call. **Byte-identical proven on
  fresh-rebuilt evidence:** RISC-V Debug (44 registers) + NVMe (42 registers) full `evidence_ir.json`
  md5-identical vs pre-`.6` AND deterministic double-run; kg-bench 151/151; +1 concat-driver test; full
  `run_ci.sh` green (lib 1455 → 1456). Now three categories are explicit: key-merge (`run_surface`: FSM,
  semantic-hints), concat (`run_surface_concat`: registers), stateful-assembly (own orchestrators: signal seed).
- `.7` migrate the **protocol-actors** surface. **DONE (`2026-06-09`).** Actors is a SINGLE-strategy surface
  (`extract_protocol_actors`, reads only `statements`); registered as a `ProtocolActorExtractor` unit
  (`actors.prose`) run through `run_surface_concat`. A single-strategy registration gains no merge benefit but
  gives a uniform run-manifest entry and readies the surface for future multi-strategy growth (more prose
  agent-definition forms plug in as units) — aligned with the "digest more PDF variants" aim. Byte-identical
  (I2C: full evidence md5 unchanged, 2 actors preserved, deterministic double-run); kg-bench 151/151; full
  `run_ci.sh` green (lib 1456). The two sibling single-strategy top-level surfaces (`serial_frame_fields`,
  `swd_operations`) follow the same trivial pattern — register opportunistically.
- `.8` **wire the `ExtractionManifest` into the output + `validate`** (owner pick `2026-06-09`: "(iii) for
  now"). **DONE (`2026-06-09`).** Added `EvidenceIr.extraction_manifest: ExtractionManifest` (additive,
  `#[serde(default)]`; serde derives on the manifest types — `name`/`surface` changed to owned `String` so
  they round-trip). The 4 migrated surface helpers (FSM / semantic-hints / registers / actors) now thread a
  `&mut ExtractionManifest` and `record()` their `SurfaceRun` (idempotent per surface name, so the
  semantic-hints refresh path doesn't duplicate). `validate <evidence>` surfaces it: metrics
  `extraction_manifest_surfaces` + `extraction_extractors_fired` plus an Info `evidence_extraction_manifest`
  finding (e.g. `register_records[registers.field_table] protocol_actors[actors.prose]
  signal_semantic_hints[semantic_hints.tables,semantic_hints.prose]`). Verified: RISC-V manifest populates
  correctly + deterministic double-run; non-manifest output unchanged (helpers return the same `run.records`,
  recording only borrows it); kg-bench 151/151; +2 tests (idempotent record + serde round-trip); full
  `run_ci.sh` green (lib 1456 → 1457); book subsection in `quality/validation.md`. **This UNBLOCKS
  `CORPUS-PATTERN-REUSE.2`** — the manifest IS the per-document behavioral fingerprint to cluster on.
- `.9`+ the remaining surfaces: the two sibling single-strategy ones (serial-frame, operations — thin), and
  the **converge-loop surfaces** (constraints / relations / polarity / conditional-rules) — these live inside
  `converge_evidence_extractions`'s fixed-point loop, a distinct sub-problem (the loop itself is not a simple
  `run_surface`). Then retire the `build()` god-orchestrator. **Pending — owner may steer scope.**
- `.9a` **register the serial-frame + operations surfaces** · Status: `done` (`2026-06-09`). The
  earlier "low-value" frontier note predates the profile plane: since `CORPUS-PATTERN-REUSE.3b.2`/`.3c`,
  every registered surface feeds the manifest → fingerprint → extraction-profile chain, so serial/debug docs
  (SWD/ADI, CAN, SWP) gain behavioral fingerprint tokens from exactly the surfaces that distinguish them.
  Faithfulness analysis done BEFORE coding: `serial_frame_fields` is a clean key-merge surface — strategy 1
  (`extract_serial_frame_fields`) upserts by name (unique names by construction) and strategy 2
  (`extract_composition_frame_fields`) dedups by name internally, so `run_surface(key = name)` with order
  [bit-range, composition] reproduces today's "composition defers to bit-range names" merge EXACTLY (the
  `.5` stateful-assembly trap does not apply: both strategies return finished lists). `swd_operations` is a
  single-strategy surface → `run_surface_concat`, the `.7` actors pattern.
  **DONE (`2026-06-09`).** `SerialFrameBitRangeExtractor` (`serial_frame.bit_range`) +
  `SerialFrameCompositionExtractor` (`serial_frame.composition`) → `serial_frame_field_surface` via
  `run_surface` keyed by field name; `SwdOperationExtractor` (`operations.prose`) → `swd_operation_surface`
  via `run_surface_concat`; both record into the manifest, `build()` call sites collapsed. **Byte-identical
  proof: ALL 12 intact-bundle docs rebuilt before/after — non-manifest JSON identical on every doc**; the
  manifest gains the two surfaces everywhere (by design, as in `.8`) and fires exactly where it should: CAN →
  `serial_frame.composition`, SWD/ADI → `serial_frame.bit_range` + `operations.prose`, all 10 others honestly
  empty. 4 hermetic tests (composition-defers-to-bit-range merge + per-strategy manifest counts +
  legacy-two-step equivalence + empty-surface manifest honesty). `run_ci.sh` green (lib **1495**); kg-bench
  151/151; book `quality/validation.md` example updated to the 6-surface manifest; KM
  `extractor-path-architecture` status-updated. Remaining in `.9`+: the converge-loop surfaces, then retire
  the god-orchestrator.

- `.9b` **converge-loop audit + migrate the signal-polarity surface** · Status: `done` (`2026-06-10`).
  **Pre-coding faithfulness audit of `converge_evidence_extractions` (the `.5` method) — the three
  converge-loop surface families categorize cleanly:**
  1. **Signal polarity = a CONCAT surface + arbitration post-passes** (the `.6` registers pattern).
     `collect_signal_polarity_facts` (single caller: the converge loop) runs two observation strategies —
     `extract_signal_polarity_from_prose(statements, known_signals)` then
     `extract_signal_polarity_from_signal_tables(source_ir, known_signals, prior_guidance)` — both returning
     `Vec<SignalPolarityObservationCandidate>`, concatenated in that order into a per-signal
     `BTreeMap` accumulator (`record_signal_polarity_observation` merges same `(polarity, source_kind)`
     observations; insert order = strategy order, preserved exactly by `run_surface_concat` order
     [prose, tables]), then arbitrated (single-polarity consensus → `SignalPolarityRecord`; disagreement →
     `SignalPolarityConflictRecord` with ids minted in BTreeMap = signal-name order). Migration: two
     `Extractor<SignalPolarityObservationCandidate>` units (`signal_polarity.prose` / `signal_polarity.tables`)
     via `run_surface_concat`, arbitration stays the post-pass — output byte-identical by construction.
  2. **Actor-signal relations = a CONCAT surface + ordered post-passes.** Prose strategy
     (`extract_actor_signal_relations`, per-pass `known_signals`) + table strategy (precomputed
     `extract_relations_from_signal_tables_with_prior_guidance`, cloned per pass), concatenated, THEN
     `augment_check_signal_relations_from_tables` (reads the full pre-dedup list), THEN
     `dedup_actor_signal_relations` (first-wins by `(actor, signal, is_drives)`). The dedup runs AFTER the
     augment, so it must stay a post-pass — using the driver's key-merge would dedup BEFORE augment and
     change what augment sees. → `.9c`.
  3. **Constraints + conditional rules = STATEFUL-ASSEMBLY** (the `.5` category — own orchestrator, NOT the
     driver). One per-pass `constraint_counter` mints sequential ids ACROSS three extractors
     (`extract_signal_constraints` → `extract_dynamic_signal_constraints` → `extract_conditional_rules`),
     plus a cross-surface post-pass (`apply_signal_polarity_to_constraints` consumes polarity's resolved
     map between the second and third). Forcing them through the driver would break cross-surface id
     continuity — they keep their cohesive in-loop orchestration, documented in the two-phase model.
  **Manifest semantics for fixed-point surfaces:** the converge loop re-runs each surface every pass;
  `ExtractionManifest::record` REPLACES per surface name, so per-pass recording leaves exactly the FINAL
  pass's run in the manifest — the converged truth, no special casing. `converge_evidence_extractions`
  gains a `&mut ExtractionManifest` parameter (the `.8` threading pattern).
  **This slice migrates surface 1 (polarity); verification = the `.9a` method:** 12-doc intact-bundle
  baseline rebuild (double-run fixpoint check) → migrate → rebuild → non-manifest JSON byte-identical on
  every doc; the manifest gains `signal_polarities` everywhere by design; hermetic unit tests; kg-bench
  151/151; full `run_ci.sh`.
  **DONE (`2026-06-10`).** `SignalPolarityProseExtractor` (`signal_polarity.prose`) +
  `SignalPolarityTableExtractor` (`signal_polarity.tables`) → `signal_polarity_surface` via
  `run_surface_concat` (concat, NOT key-dedup — a same-polarity second source must STRENGTHEN the record
  and a different-polarity observation must surface as a conflict, never be dropped) + the unchanged
  arbitration body extracted as `arbitrate_signal_polarity_observations`; `converge_evidence_extractions`
  threads `&mut ExtractionManifest` (the `.8` pattern) and records per pass — replace-per-surface-name
  keeps exactly the final converged pass. The legacy single-caller `collect_signal_polarity_facts` is gone.
  **Byte-identical proof: ALL 12 intact-bundle docs — baseline double-run fixpoint confirmed
  (byte-identical), then post-`.9b` rebuild non-manifest JSON (`jq del(.extraction_manifest)`)
  md5-identical on every doc.** The manifest gains `signal_polarities` everywhere and fires exactly where
  polarity evidence lives: AXI 39 prose + 45 table observations → 44 resolved / 0 conflicts; APB 4+2→3;
  AHB 2+1→1; AXI-Stream 1+0→1; SWD/ADI 4+1→3; SMBus 1+0→1; RISC-V/CAN/SWP/NVMe/I2C/I2S honestly 0 — new
  distinguishing fingerprint tokens for the CORPUS-PATTERN-REUSE profile plane (the AMBA family now shares
  fired `signal_polarity.*` tokens). 2 hermetic tests (legacy-two-step equivalence incl. conflict +
  strengthen-not-duplicate + per-strategy manifest counts; empty-surface manifest honesty). kg-bench
  151/151; full `run_ci.sh` green (lib 1495 → **1497**); book `quality/validation.md` updated to the
  7-surface example + converge-loop manifest semantics; KM `extractor-path-architecture` status-updated.

- `.9c` **migrate the actor-signal-relations converge-loop surface** · Status: `done` (`2026-06-10`).
  Design already fixed by the `.9b` audit: relations = a CONCAT surface (prose strategy
  `extract_actor_signal_relations` with per-pass `known_signals`; table strategy = the
  build-precomputed `extract_relations_from_signal_tables_with_prior_guidance` list re-emitted per pass)
  + two ORDERED post-passes — `augment_check_signal_relations_from_tables` (inherits check-signal
  relations from the FULL pre-dedup merged list; does its own existing-key skip) THEN
  `dedup_actor_signal_relations` (first-wins by `(actor, signal, is_drives)`). The dedup MUST stay a
  post-pass: the driver's key-merge would dedup BEFORE augmentation and change augment's input
  (`relations_by_signal` is built from the un-deduped list). New units `relations.prose` +
  `relations.tables` → `actor_signal_relation_surface` via `run_surface_concat`, recorded per pass
  (replace-semantics keep the final converged run — the `.9b` fixed-point manifest model). Verification =
  the `.9a`/`.9b` method: 12-doc baseline (current evidence is the post-`.9b` fixpoint) → migrate →
  rebuild → non-manifest byte-identical everywhere; hermetic legacy-equivalence + manifest-honesty tests;
  kg-bench 151/151; full `run_ci.sh`.
  **DONE (`2026-06-10`).** `ActorSignalRelationProseExtractor` (`relations.prose`) +
  `ActorSignalRelationTableExtractor` (`relations.tables`, re-emitting the build-precomputed table list per
  pass — exactly the legacy cloned extend) → `actor_signal_relation_surface` via `run_surface_concat`, then
  the two ordered legacy post-passes verbatim (augment over the full pre-dedup `run.records`, then
  `dedup_actor_signal_relations`); the in-loop block collapsed to one call. **Byte-identical proof: ALL 12
  intact-bundle docs — post-`.9c` rebuild non-manifest JSON md5-identical vs the post-`.9b` fixpoint
  baseline on every doc.** The manifest now tells the per-doc relation story honestly: AXI = table-dominant
  (356 table + 20 prose → 348 final: dedup drops the prose/table overlap), APB 44+7→69 / AHB 21+16→66 /
  AXI-Stream 22+10→54 (final exceeds produced where the augment post-pass adds check-signal `chk_asr_*`
  records AFTER the driver counts — e.g. the APB parity-check signals), SWD/ADI 26 + I2C 17 pure prose,
  RISC-V mixed 9+11→20, CAN/NVMe/SMBus/I2S honestly 0 — more fingerprint tokens for the profile plane. 2 hermetic
  tests (legacy-two-step equivalence proving prose-wins-dedup + a REAL augment inheritance `chk_asr_*`
  + per-strategy manifest counts; empty-surface manifest honesty). kg-bench 151/151; full `run_ci.sh`
  green (lib 1497 → **1499**); book example updated to the 8-surface form; KM status updated.

- `.9d` **refresh the `ir/extractor.rs` two-phase doc note** · Status: `done` (`2026-06-10`).
  The module doc's "Two phases, two categories" section was written at `.5` and is stale: it still names
  actors + signal-polarity as "remaining clusters to migrate" (both done) and predates the concat driver's
  converge-loop reach. Update it to the current truth: THREE producer categories with their live members
  (key-merge: FSM / semantic-hints / serial-frame · concat: registers / actors / SWD-operations /
  signal-polarity / actor-signal-relations · stateful-assembly: signal-declaration seed + the constraint
  family), the fixed-point manifest semantics (surfaces inside `converge_evidence_extractions` record per
  pass; `record()` replace-per-name keeps the final converged run), and the `.9b` audit's
  constraint-family categorization (cross-surface `constraint_counter` id-minting + mid-sequence polarity
  post-pass → deliberately OFF the drivers). Doc-comment-only change; rustdoc is the gate.
  **DONE (`2026-06-10`).** The section is now "Two phases, three categories": current members per driver
  mode (key-merge ×3 with their keys, concat ×5), the converge-loop fixed-point recording semantics, the
  relations post-pass ORDER as surface contract, and the constraint family as a deliberate design decision
  ("not migration debt"). Full `run_ci.sh` green (rustdoc warning-deny passed; lib 1499 unchanged —
  doc-comment only).

## Current frontier

`.1`–`.8` + `.9a` **done** — SIX surfaces registered and byte-identical-proven (FSM, semantic-hints,
registers, actors, serial-frame, SWD-operations); `.9a`'s value was re-rated UP by the profile plane
(`CORPUS-PATTERN-REUSE.3b.2`/`.3c`: manifests now feed learned extraction profiles). The framework
(key-merge `run_surface` + concat `run_surface_concat` + own orchestrators for assembly) **emits a
per-document run manifest** surfaced in `validate` — the CORPUS-PATTERN-REUSE fingerprint. **`.9b` + `.9c`
done (`2026-06-10`): the converge-loop audit + BOTH migratable fixed-point surfaces (polarity, relations)
— EIGHT surfaces registered, all byte-identical-proven over the 12 intact-bundle docs; the manifest now
distinguishes table-driven vs prose-driven relation recovery per document (AXI table-dominant; SWD/I2C
pure prose).** **`.9d` done (`2026-06-10`): the `ir/extractor.rs` module doc now encodes the full
three-category model (members per driver mode, fixed-point recording semantics, post-pass order as
contract, constraint family = design decision not debt).** Remaining: the `build()` god-orchestrator
retirement (assessment slice first). **Owner may steer scope.**
