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

## Closed task trees — how each was implemented and verified

Per the `BOOK-METHOD-DOC` standing close-rule, each tree carries a
topically-placed implementation+verification summary; the
task-tree files remain the machine-tracked authority. The seven
trees below all closed at the ISF / FSMGen boundary; their `.isf`
adapter behaviour is captured here in one consolidated section.

### `R6-ISF-ADAPTER` — bring the `.isf` adapter under task-tree ownership

The `.isf` adapter landed before the task-tree workflow stabilized;
this tree retroactively put it under ownership: removed the
`IrStage` modeling smell (a leftover of the multi-target era),
raised the ISF self-test coverage to the IR-layer bar, and
documented the adapter's contract with FSMGen. No behavioural
regression — every leaf either preserves the pre-existing `.isf`
or records an explicit decision to change it. Verified by the
full IR self-test suite + FSMGen-strict acceptance + e2e adapter
fixtures. *Authoritative tracking:* `docs/tasks/R6-ISF-ADAPTER.md`.

### `ISF-ONLY-CONSOLIDATION` — `.isf` as the sole adapter target

Removed the multi-target HDL adapter surface (SystemVerilog /
Verilog / VHDL) and the entire `.fsm` adapter subsystem from
code, tests, fixtures, and documentation. After this tree
SpecForge lowers `IntentIR` to `.isf` only; FSMGen owns
everything downstream (scheduling, `.fsm`, HDL). Verified by
absence of HDL/`fsm` code paths (compile-checked), the e2e
suite still green against `.isf`-only fixtures, and the
pipeline overview chapter rewritten to match.
*Authoritative tracking:* `docs/tasks/ISF-ONLY-CONSOLIDATION.md`.

### `ISF-ONLY-IR-PRUNE` — remove IR surfaces that fed only the removed `.fsm`

After `.fsm` and HDL adapters were removed, several IR surfaces
no longer had a consumer (`init_assignments`,
`decision_tree_fragments`, `regular_states`, `state_transitions`).
This tree audited the consumer graph and removed the surfaces no
ISF/FSMGen path reads — a strict subset (`temporal_rules` was
audited and kept, given `validate.rs` R7/R15b references +
`learn_priors` + the back-compat fallback in `R16-CONTRACT-IR.3`).
Verified by full CI + adapter-fixtures regression.
*Authoritative tracking:* `docs/tasks/ISF-ONLY-IR-PRUNE.md`.

### `ISF-TEMPORAL-LOWERING` — `IntentIR.temporal_rules` reaches the adapter

The flagship-R15b deliverable (`IntentIR.temporal_rules`) was
gated by `assess_isf_renderability` but **never read** by
`IsfIr::from_intent_ir` — the typed temporal surface didn't
reach `.isf`. This tree wired the read path: classify every
temporal rule into a `TemporalRuleDisposition` (`Rule`,
`Contract`, or `Residual{reason}`); honest residual for shapes
without a representable `.isf` construct; FSMGen-strict
acceptance is a hard verification. Verified at every leaf via
the real FSMGen binary on a strict-baseline nvme fixture
(`0/250/92`-class metric) and `scripts/run_ci.sh`. Subsumed by
`R16-CONTRACT-IR.3` (which re-pointed the adapter at the typed
`actor_contracts` projection while preserving
`classify_temporal_rule` output exactly — parity by
construction). *Authoritative tracking:*
`docs/tasks/ISF-TEMPORAL-LOWERING.md`.

### `ISF-HANDSHAKE-STAGE-LOWERING` — `(stage p (ready r)(valid v))`

Originally a separate tree; **superseded by and DELIVERED via
`R16-CONTRACT-IR.4`** (`6869c128`, 2026-05-19). The
`HandshakeComplete` temporal_rules now lower as
`HandshakeBarrier` obligations and emit `(stage …)` only when
the actor's input direction permits (FSMGen rejects a `(stage)`
whose `ready` is not an actor input — gated by
`input_signal_names`, else honest residual; never strict-invalid).
Real-binary verified at FSMGen pin `9bfb9a20`. Verified-but-
**dormant** on the current corpus (no `handshake_complete`
temporal_rule yet ⇒ zero `.isf` change); activates the moment
extraction grounds handshakes. *Authoritative tracking:*
`docs/tasks/ISF-HANDSHAKE-STAGE-LOWERING.md` (superseded
marker) and `docs/tasks/R16-CONTRACT-IR.md` (`.4`).

### `FSMGEN-ISSUE-REPORTING` — downstream-issue bundles

Two genuine FSMGen findings surfaced during
`ISF-TEMPORAL-LOWERING.2.1` (F1: flat-eventual nested syntax;
F2: ready/valid stage acceptance) were filed via the official
protocol (`subs/fsmgen/docs/DOWNSTREAM_ISSUE_REPORTING.md`;
`bin/fsmgen-issue-bundle`), with bundles stored under SPECFORGE
so the pinned `subs/fsmgen/` working tree is never modified.
**Honest count**: two findings, not three (the third candidate
was a downstream-spec interpretation issue, not a FSMGen bug).
Verified by `fsmgen-issue-bundle` integration tests + the
serialised parallel-CWD lock (`FSMGEN_TEST_LOCK`).
*Authoritative tracking:* `docs/tasks/FSMGEN-ISSUE-REPORTING.md`.

### `FSMGEN-SUBMODULE-BUMP` — pin `effe591d` → `9bfb9a20`

Updated the pinned `subs/fsmgen` submodule to upstream HEAD
`9bfb9a20`, which carries explicit fixes for the two filed
findings (`610cb26e` + `d4d6dfab`) plus structured-JSON failure
emission (`9bfb9a20`). Both fixes were audited from the
SPECFORGE side: nested `(eventually s (within N))` now accepts;
`(stage p (ready r)(valid v))` now accepts (with the
ready-is-actor-input contract, gated in `R16-CONTRACT-IR.4`).
Verified by re-running both `fsmgen-issue-bundle` reproducers
against the new pin and confirming both produce success; the
existing `scripts/run_ci.sh` flow regressed cleanly to the new
pin. *Authoritative tracking:*
`docs/tasks/FSMGEN-SUBMODULE-BUMP.md`.

### `ISF-TXN-GRAMMAR-FIX` — emitter grammar made FSMGen-contract-exact

After the `FSMGEN-REFRESH-INTEGRATE` refresh, a review of the
transaction-step emitter (`render_txn_step`) against the FSMGen ISF
book/contract — *not* the binary (parser-acceptance ≠ support) — found
six sites emitting forms the grammar does not use:

- `shift-left` / `shift-right` (hyphen) → `shift_left` / `shift_right`
  (book *Data Manipulation*: "the form is exact: `(shift_left reg bit)`").
- `await-all` / `await-any` (hyphen) → `await_all` / `await_any`
  (book *Composition*: `(await_all done)` / `(await_any done)`).
- `(spawn child instance)` → `(spawn child as instance)` — the `as`
  keyword is mandatory (book *Composition*: "the base form is exact:
  `(spawn child as name)`").

These steps are dormant on today's corpus (no current spec drives them
through `IntentIR`), so the fix changes no emitted artifact today — but it
removes a latent trap: the moment a richer transaction *did* produce them,
SpecForge would have emitted FSMGen-strict-invalid `.isf`. `(do child)`
was deliberately left unchanged — its syntax is already contract-exact;
the separate "child must be a declared transaction" concern is semantic,
not a grammar typo. **Verified** by a render-lock unit test asserting the
contract-exact forms are emitted and the old hyphen / missing-`as` forms
are gone, plus full `scripts/run_ci.sh`. *Authoritative tracking:*
`docs/tasks/ISF-TXN-GRAMMAR-FIX.md`.
