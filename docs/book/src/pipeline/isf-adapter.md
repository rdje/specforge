# ISF Adapter — IntentIR to ISF

This page documents the lowering flow from `IntentIR` to `.isf` source text. The flow has been hardened against string-bashing bugs by introducing a typed intermediate representation (`IsfIr`) between the adapter and the text emitter.

## Flow overview

```
IntentIR
    │
    ▼
build_isf_source_text()          ← thin orchestration in ir/adapters.rs
    │
    ▼
IsfIr::from_intent_ir()          ← adapter: IntentIR → typed ISF IR  (ir/isf_ir.rs)
    │
    ▼
IsfIr                            ← typed tree (BTreeSet dedup, enforced reset, valid syntax)
    │
    ▼
IsfIr::render()                  ← emitter: recursive tree walk → S-expression text
    │
    ▼
.isf source text                 ← valid ISF consumed by FSMGen
```

## Why the ISF IR exists

Before the ISF IR, the adapter emitted ISF text by string concatenation ("string-bashing"). This caused bugs that were invisible to the compiler:

- **Duplicate clock/reset in interface** — because interface signals were built by iterating all signals without filtering infrastructure signals
- **Missing reset for strict mode** — because reset emission was conditional, but FSMGen `--strict` requires a reset clause
- **Invalid `(priority ... over ...)` syntax** — because priorities were rendered without validating the S-expression structure
- **Broken transaction nesting** — because parentheses were built by hand, not by tree walk

Each of these was a runtime error that only surfaced when FSMGen rejected the output. The ISF IR eliminates these bugs by construction:

| Bug class | ISF IR protection |
|-----------|-------------------|
| Duplicate signals | `BTreeSet<IsfSignal>` — automatic deduplication |
| Missing reset | `IsfReset` is non-optional — compiler enforces presence |
| Invalid syntax | Typed structs (`IsfRule`, `IsfPriority`) — no raw strings |
| Mismatched parens | Recursive tree walk — parentheses match by construction |

## ISF IR data model

The ISF IR is a typed tree in `ir/isf_ir.rs`. The root struct:

```rust
pub(crate) struct IsfIr {
    actor_name: String,
    clock: String,
    reset: IsfReset,                       // non-optional
    watchdog: u64,
    signals: BTreeSet<IsfSignal>,          // deduplicated
    constants: Vec<IsfConstant>,
    types: Vec<IsfTypeDef>,
    enums: Vec<IsfEnum>,
    storage: Vec<IsfStorageVar>,
    drives: Vec<IsfNamedDrive>,
    transactions: Vec<IsfTransaction>,
    rules: Vec<IsfRule>,
    priorities: Vec<IsfPriority>,
}
```

### Construction: `IsfIr::from_intent_ir(intent_ir, actor_name) -> IsfIr`

The adapter walks `IntentIr` and populates the typed tree:

1. **Clock** — from the system contract
2. **Reset** — always populated; kind/polarity from system contract with sensible defaults
3. **Signals** — collected from all interfaces; clock/reset excluded; inserted into `BTreeSet` for automatic dedup
4. **Constants** — from declared symbolic constants
5. **Types/enums** — from type definitions and enum member-value maps
6. **Storage** — from storage variable declarations
7. **Drives** — one `(drive (sig val) (sig val))` entry per output signal
8. **Transactions** — from `IntentIr` transaction intents plus control-block fallbacks; `TransactionStep` converted to typed `IsfTxnStep`
9. **Temporal rules** — every `IntentIr.temporal_rules` entry is classified by `classify_temporal_rule` into exactly one disposition (see below)
10. **Rules** — from conditional rules, signal constraints, temporal invariants with non-empty subject signals, plus the temporal value/guard→drive rules from step 9
11. **Priorities** — rule-over-transaction priority declarations when both rules and transactions exist

Temporal invariants with empty `subject_signal` (e.g. transition invariants like "idle → busy when GO") are skipped for rule generation — they represent state-transition assertions that cannot be lowered to ISF signal assignments.

#### Temporal-rule lowering (`classify_temporal_rule`)

`IntentIr.temporal_rules` is the R15b clock-tick deliverable; the ISF
adapter is the only target it can reach. A single classifier assigns each
rule exactly one disposition, so a rule is lowered one way or not at all:

- **Contract** — a bounded `cycle_window` (`max_cycles >= 1`) with a
  single-signal consequent naming a declared signal → a synthetic
  `(transaction txn_temporal_<id> (on start) (contract <id> (eventually
  <signal> (within <N>))) (complete done))`. This is FSMGen's shipped
  `bounded_eventually`; the **nested** `(within N)` is required (the flat
  `within N` printed in the FSMGen spec §11.8 is strict-rejected — see
  `docs/FSMGEN_FEEDBACK.md`).
- **Rule** — no window, a `SignalValue` consequent naming a declared
  signal with an ISF-literal value → an actor `(rule temporal_<id>
  [<guard>] (<signal> <value>))`. The guard is `(== <declared-signal>
  <literal>)` derived from a `SignalValue` antecedent, else the rule is
  conditionless (both forms are fsmgen-strict-verified). The exact
  declared signal name is used (FSMGen is case-sensitive).
- **Residual** — anything with no representable supported ISF construct
  (`HandshakeComplete`, `(within 0)`, undeclared signal, non-literal
  value, a consequent that names a signal but carries no concrete value)
  → an explicit `isf_temporal_unrepresentable_<id>` residual decision on
  the adapter artifact. Syntax is **never** fabricated; the dropped
  obligation is visible, not silently lost.

Temporal value/guard→drive rules join the rule set before dedup, so a
temporal rule that conflicts with an existing rule on the same
signal+guard is dropped rather than emitted as invalid `.isf`.

### Rendering: `IsfIr::render() -> String`

The emitter performs a recursive tree walk:

1. Emit `(actor <name>`
2. Emit `(clock <name>)`
3. Emit `(reset (<signal> <sync/async> <active_high/active_low>))`
4. Emit `(watchdog <value>)`
5. Emit `(interface ...)` — all signals with directions and widths
6. Emit `(drive ...)` — one per output
7. Emit `(transaction <name> ...)` — recursively walk `IsfTxnStep` tree, emitting S-expressions at each level. Nesting (`when`, `switch`, `while`, `until`, `repeat`) produces properly balanced parentheses regardless of depth.
8. Emit `(rule <name> <condition> (<signal> <value>)...)` — one per rule
9. Emit `(priority <higher> over <lower>)` — one per priority pair
10. Close with `)`

## Module boundaries

```
ir/isf_ir.rs          ISF IR types, adapter (from_intent_ir), emitter (render), helpers
ir/adapters.rs        AdapterArtifact + shared scaffolding, ISF adapter (build_isf_adapter_artifact, renderability policy), tests
```

`adapters.rs` calls `IsfIr::from_intent_ir()` and `.render()` — it does not contain the ISF IR. The ISF IR lives in its own module, surfaced via `ir/mod.rs` as `pub mod isf_ir`.

## Renderability policy

ISF lowering blocks on exactly two conditions:

1. **No signals** — no signal records in any interface, and
2. **No behavioral content** — no temporal rules, conditional rules, signal constraints, or control blocks.

If either holds, the adapter artifact is `Blocked` with explicit `blocking_reasons` and no `.isf` text is emitted.

Crucially, **missing per-signal direction or width is _not_ a blocker**. `IsfIr::from_intent_ir` defaults an unknown direction to `output` and an unknown width to `1`, and emission proceeds. The rationale: FSMGen performs the cycle scheduling for `.isf` and accepts a default direction/width, so blocking on those would over-restrict otherwise-honest lowering without improving downstream correctness — the `.isf` adapter can safely default and let FSMGen schedule. (`.isf` is SpecForge's only adapter target; `.fsm`/HDL are owned by FSMGen downstream.) The policy lives next to `assess_isf_renderability` in `ir/adapters.rs`.

## Generated artifact

ISF adapter output lives under:

- `generated/adapters/isf/<document_key>/adapter.json`

The adapter artifact carries the rendered `.isf` source text, renderability status, blocking reasons, and signal inventory metadata. It is tagged with the `isf_adapter` `IrStage`, which stage-keyed tooling (e.g. `specforge validate`, `project-validation`) dispatches on.

Every behavioral count in the artifact reflects **what the emitter
actually rendered**, never a blind `IntentIr`-derived guess:
`transaction_count` and `rule_count` are taken from the single emitted
ISF model (`emitted_transaction_count()` / `emitted_rule_count()`), so
they include temporal-synthesized `(contract …)` transactions and
temporal `(rule …)` and exclude anything the emitter dropped. Temporal
rules with no representable construct are counted as
`residual_decisions` (`isf_temporal_unrepresentable_*`). The regression
`isf_adapter_counts_equal_emitted_content` locks metric == emitted
content; `isf_temporal_rules_reach_isf_end_to_end` proves the full
markdown → `IntentIr` → `.isf` temporal path end-to-end.

## Validation

Two complementary checks cover ISF output:

- **FSMGen strict acceptance** — ISF text is validated against FSMGen `--strict --check --json` in the test `isf_output_passes_fsmgen_strict_validation`, ensuring SPECFORGE-generated ISF passes FSMGen's strict surface with zero diagnostics and zero syntax errors.
- **`specforge validate <isf adapter.json>`** — auto-detects the `isf_adapter` stage and runs `validate_isf_adapter`, reporting structural and coverage findings (missing ISF payload, unexpected schema version, not renderable, empty signal inventory, no behavioral surface, residual decisions).
