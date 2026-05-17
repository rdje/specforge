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
9. **Rules** — from conditional rules, signal constraints, and temporal invariants with non-empty subject signals
10. **Priorities** — rule-over-transaction priority declarations when both rules and transactions exist

Temporal invariants with empty `subject_signal` (e.g. transition invariants like "idle → busy when GO") are skipped for rule generation — they represent state-transition assertions that cannot be lowered to ISF signal assignments.

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
ir/adapters.rs        AdapterArtifact, build_isf_source_text (thin wrapper), FSM adapter, tests
```

`adapters.rs` calls `IsfIr::from_intent_ir()` and `.render()` — it does not contain the ISF IR. The ISF IR lives in its own module, surfaced via `ir/mod.rs` as `pub mod isf_ir`.

## Generated artifact

ISF adapter output lives under:

- `generated/adapters/isf/<document_key>/adapter.json`

The adapter artifact carries the rendered `.isf` source text, renderability status, blocking reasons, and signal inventory metadata.

## Validation

ISF output is validated against FSMGen `--strict --check --json` in the test `isf_output_passes_fsmgen_strict_validation`. The test ensures that SPECFORGE-generated ISF passes FSMGen's strict acceptance surface — zero diagnostics, zero syntax errors.
