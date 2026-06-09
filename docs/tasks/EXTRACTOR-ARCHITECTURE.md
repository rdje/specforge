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
  methodology). Remaining genuine merge-surfaces to migrate (`.6`+): **registers, actors, signal-polarity**.

## Current frontier

`.1` (audit) **done** this slice. `.2` (framework scaffolding) is **design-complete, build gated on one owner
confirmation** of the framework shape (trait-registry vs. lighter manifest-only vs. data-driven pipeline) and
how aggressive to be — recorded because committing to a load-bearing framework is the owner's architectural
call (doing the refactor itself erratically would betray the very principle). No code change until confirmed.
