# ISF lowering-fidelity measurement (`KG-ISF-COMPLETENESS.2`)

Read-only measurement over the 36 persisted `generated/intent_ir/*/intent_ir.json` artifacts plus a
direct read of the ISF lowering code (`2026-06-17`). No code, no extraction change, no mutation. This is
the measurement-first leaf that gauges **which ISF-fidelity bar dimension carries the largest faithful-
lowering gap on the wire docs + broader corpus** (the `.2+` frontier), before any emitter change. Project
measurement-first doctrine; WIRE-BASED-100 a hard gate on anything that follows.

## Question

The owner's north star (`2026-06-16`): the IntentIR must be COMPLETE so it lowers **faithfully** to ISF —
"extracting for extracting is not the goal." Bar dimension **#6 (ISF round-trip)**: *every IntentIR
surface element appears in the `.isf` or an explicit residual* (no silent drop). Bar **#3 (signals)**:
*every interface signal carries direction + width where the document grounds it*. **Which dimension has the
largest addressable gap?** Specifically: which IntentIR surfaces reach the `.isf`, which are recorded as
honest residuals, and which are **silently dropped**?

## Method

1. **Prevalence** — count every design-intent-bearing IntentIR surface across all 36 docs (a surface that
   is empty corpus-wide is not a gap).
2. **Lowering classification** — read `IsfIr::from_intent_ir` (`ir/isf_ir.rs:613`) + `render()` +
   `build_isf_adapter_artifact` (`ir/adapters.rs`), newline-aware, and classify each surface as
   {lowered → `.isf` construct | recorded as residual | NOT read}. (Cross-checked by two independent
   read-only agents.)
3. **Silent-drop quantification** — replicate the lowering's own `signal_names` set and per-element
   filters in Python, count the typed-rule elements each filter `continue`-skips with no residual.
4. **Direction/width** — measure how the emitted `.isf` assigns direction + width vs what the IntentIR
   grounds.

## 1. Surface prevalence (36 docs)

| surface | #docs | total | | surface | #docs | total |
|---|---:|---:|---|---|---:|---:|
| behaviors | 36 | 22218 | | temporal_rules | 23 | 400 |
| constraints | 36 | 22682 | | signal_constraints | 23 | 444 |
| temporal_invariants | 31 | 16463 | | conditional_rules | 36 | 1634 |
| interfaces (signals) | 36 | 4285 | | transactions | 11 | 67 |
| register_records | 19 | 2561 | | symbol_definitions | 31 | 284 |
| actor_ports | 26 | 2523 | | actor_contracts | 18 | 196 |
| actor_signal_relations | 26 | 1928 | | signal_polarities | 18 | 81 |
| conditional_rules | 36 | 1634 | | infrastructure_signals | 13 | 26 |
| actor_drive_relations | 24 | 1112 | | assumptions | 36 | 484 |
| actor_sample_relations | 24 | 801 | | timing_constraints | 21 | 692 |

Empty **corpus-wide** (not gaps): `regular_states`, `state_transitions`, `control_blocks`,
`explicit_modules`, `explicit_tops`.

## 2. Lowering classification (what reaches `.isf`)

**Lowered** (read by `from_intent_ir`/`render`): `system_contract` + `infrastructure_signals` →
`(clock)`/`(reset)`; `interfaces[].signal_records` → signals; `symbol_definitions` →
`(constants)`/`(types)`/`(enums)`; `register_records` → `(storage (var … [reset]))`; `transactions` →
`(transaction)`; `temporal_rules`/`actor_contracts` → `(assert (monitor …))`/`(stage)`/`(rule)` **or an
explicit `temporal_residual`**; `conditional_rules` (`:1038`), `signal_constraints` (`:1058`),
`temporal_invariants` (`:1075`) → `(rule)`.

**Honest residual already:** temporal rules (`temporal_residuals`), register resets
(`storage_reset_residuals`) — the two paths the `ISF-TEMPORAL-LOWERING` / `ISF-REGISTER-RESET-EMIT` trees
built.

**NOT read** (never reach `.isf`, never a residual): `behaviors`, `constraints`, `assumptions`,
`timing_constraints`, the `actor_*` relation/port KG families, `signal_connectivity`, `signal_polarities`,
`temporal_conflicts`.

### `behaviors` (22 218) and `constraints` (22 682) are legacy free-text, not a lowering gap

Both are `{id, statement: String, …}` free-text surfaces built in `ir/intent.rs` from the SemanticIR
phases/contracts/gates (`behaviors`) and invariants/assertions/gates (`constraints`). Their *semantic*
content is the same obligations the **typed** twins carry and lower — `signal_constraints`/`temporal_rules`/
`conditional_rules`/`temporal_invariants`. Re-lowering the free-text twin would be a redundant second
rendering — exactly the `*_behavior` blob `KG-ISF-TRANSACTIONS.2b` correctly *removed*. Their non-lowering
is **not** a faithful-lowering gap.

## 3. Silent-drop of typed rules at the lowering boundary

The three lowered typed-rule surfaces are `continue`-skipped per-element when their subject signal is empty
or not a declared signal — **with no residual recorded**:

| surface | total | lower → `.isf` | DROP: ungroundable | DROP: undeclared signal |
|---|---:|---:|---:|---:|
| temporal_invariants | 16463 | 257 | 16179 (empty subject) | 27 |
| conditional_rules | 1634 | 498 | 1095 (no consequent signal) | 41 |
| signal_constraints | 444 | 361 | — | 83 |

Wire docs (0 undeclared-signal drops in all four):

| doc | temporal_invariants lower / drop | conditional_rules lower / drop | signal_constraints lower / drop |
|---|---|---|---|
| AXI | 45 / 990 | 19 / 81 | 45 / 0 |
| APB | 20 / 70 | 4 / 6 | 20 / 0 |
| AHB | 12 / 227 | 13 / 23 | 12 / 0 |
| SWD | 1 / 698 | 1 / 45 | 1 / 0 |

**The 17 425 raw "drops" do not represent lost grounded intent:**
- **`temporal_invariants` empty-subject (16 179)** are un-grounded free-text invariants. The AXI sample is a
  *table-of-contents heading* (`"| | | A2.3.2 Dependencies between channel handshake signals | … | 31 |"`)
  with no signal to drive. Mass-recording these as residuals would be **dishonest noise**, not fidelity
  (violates the "absence is not an event" honesty rule, `PDF-VARIANT-DIGESTION.11`).
- **`conditional_rules` no-consequent (1 095)** are actor-level prose obligations with no single signal=value
  drive: legal boilerplate (*"If any of the provisions … conflict …"*), vague *"must"/"shall"*, and real
  but non-driveable obligations (*"a Manager … must be able to provide all write data"*).
- **The "undeclared-signal" category (27 + 41 + 83 = 151, all 0 on wire docs)** — the seemingly "clean"
  bar-#6 violation — is, on inspection, dominated by **register/struct-field paths** from register docs:
  `process_id[19:17]`, `DC.tc.SXL`, `DID`, `Reserved` (RISC-V IOMMU, VT-d, NVMe). These are register-field
  obligations that correctly do **not** belong in the wire ISF rule surface (their typed home is the
  register / `message_field_constraints` surface), plus a couple of junk VLM observations
  (`ARVALID is "a"`). Forcing them into `.isf` rules would fabricate.

**Verdict for bar #6:** the silent-drop of *grounded, wire-relevant* intent is **small** (≈0 on the wire
docs) and the large raw drop counts are un-groundable free-text / non-wire register fields. Bar #6 is
essentially already honest for the intent that *should* lower. There is **no clean, sizable, wire-safe
faithful-lowering CODE gap** in the round-trip dimension.

## 4. Direction/width in the emitted `.isf` — the largest *true* infidelity (but design-gated)

The emitter reads only the legacy flat `direction_hint`/`width_hint` on `signal_records` (`:688`/`:692`),
defaulting an unknown direction to **output** and an unknown width to **1**. Measured:
- `direction_hint` is `None` for **85–98%** of declared signals (AXI 282/289, APB 29/34, AHB 38/41,
  SWD 8/13); `width_hint` is `None`/symbolic for **90–100%**.
- The persisted AXI `.isf` therefore emits **283 `(output …)` vs 4 `(input …)`, every one `(width 1)`**.

The IntentIR *does* carry the canonical direction — but in the **actor-relative graph** (`actor_ports`,
`actor_signal_relations`; the whole `R15-GRAPH-DIRECTION-MIGRATION`), NOT the flat hint. The emitter
ignores that graph. **Three real complications keep this from being a clean fix:**
1. **Direction is relationship-relative.** Every signal's `actor_ports` show **both** `input` and `output`
   (an output of its producer, an input of its consumer). The `.isf` is a **single flat module** named
   after `actors.first()` (`derive_isf_actor_name`, `adapters.rs:178`) but containing *all* signals — so
   per-signal direction needs a chosen **reference boundary**, a real design decision.
2. **The default is deliberate + documented.** `R6-ISF-ADAPTER.4` (`adapters.rs:202–217`): missing
   direction/width is *intentionally not* a blocker — "FSMGen performs the cycle scheduling for `.isf` …
   blocking on those would over-restrict otherwise-honest lowering without improving downstream
   correctness." This is in genuine tension with the `2026-06-16` north star and is an **owner decision**.
3. **The persisted corpus is pre-`.1a`.** The sampled `actor_port` belongs to actor `"For"` — a phantom
   prose-fragment the `.1a` gate now rejects (the `.1a`/`.1b` rebuilds went to a temp evidence-root per the
   WRITE-PATH GOTCHA), so an actor-port-driven direction recovery would need fresh canonical artifacts.

Width is less ambiguous than direction (a signal's width is boundary-independent) but is mostly **symbolic**
(`*_WIDTH` parameters) which `.parse::<u32>()` cannot turn into a concrete `(width N)` — needs an
FSMGen-contract check on symbolic widths.

## 5. Dimension ranking + recommendation

| bar dim | gap on wire docs | clean / addressable? |
|---|---|---|
| #6 round-trip — grounded silent-drop | ≈0 | already honest |
| #6 round-trip — undeclared typed rules | 0 (151 corpus, register-field) | NOT clean (wrong surface) |
| #6 round-trip — ungroundable free-text | huge | NOT a gap (honest absence; residualizing = noise) |
| #3 direction in `.isf` | ~98% defaulted output | **largest true infidelity**, but design-gated (relationship-relative + R6-ISF-ADAPTER.4 + owner steer) |
| #3 width in `.isf` | ~96% width-1 | mostly symbolic → FSMGen-contract-gated |
| #2 relation completeness | AXI 43% / APB 9% / AHB 5% signals with no relation | extraction dimension (partly pre-`.1a` phantom noise) |

**Conclusion.** The single largest *true* fidelity gap is **signal direction/width in the emitted `.isf`**,
but it is design-gated (deliberate default, relationship-relative, FSMGen-contract + owner-steer
prerequisites) — **not** a clean immediate code win. The bar-#6 silent-drop, by contrast, is already honest
for grounded wire-relevant intent; its large raw numbers are un-groundable free-text or non-wire register
fields that should NOT be force-residualized.

**Spun frontier:**
- **`.2a` — signal direction/width fidelity** (deferred-with-trigger): measurement-first, requires (i) an
  FSMGen-contract check (does `--strict --check` use signal direction at all? does FSMGen accept a symbolic
  `(width PARAM)`?), and (ii) a reference-boundary decision for the single flat module — and surfaces the
  **owner decision**: carry grounded per-signal direction/width (overriding R6-ISF-ADAPTER.4's deliberate
  default) vs keep FSMGen-schedules-it. Do NOT rush; flagged for owner steer.
- **`.2b` — ISF lowering-coverage visibility gauge** (the buildable candidate): make the lowering's
  per-surface coverage **honest and visible** (how many typed-rule elements lowered to `.isf` rules vs not,
  by category) as adapter metadata + a `validate <intent-ir>` surface — mirroring the existing
  `storage_reset_residual_packet` count-summary pattern. Purely additive (emitted `.isf` byte-identical →
  WIRE-BASED-100 trivially held), and it must be careful to count only *would-be-lowerable* intent, not the
  ungroundable-noise / register-field categories §3 showed are honest absences. Confirm scope
  measurement-first in its own slice.

## Reproduce

Read-only Python over `generated/intent_ir/*/intent_ir.json` replicating `signal_names` (clock/reset
excluded) + the `from_intent_ir` filters at `ir/isf_ir.rs:1038/1058/1075`; direction/width from
`signal_records[].{direction_hint,width_hint}` vs the persisted `generated/adapters/isf/*/agent.isf`
`(input|output … (width …))` lines. The lowering classification reproduces by grepping newline-aware
`\.<field>` consumption sites in `crates/specforge/src/ir/isf_ir.rs`.
