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
3. **Signals** — collected from all interfaces; clock/reset excluded; inserted into `BTreeSet` for automatic dedup. **Width** comes from the signal's own width hint; when that is absent (the flat hint is `None`/symbolic for most signals, since the grounded width lives on the actor-relative graph), the adapter recovers a *single unambiguous concrete* width from the actor-port graph (`actor_ports[].width_hint`) — so a sideband like AXI `ARSIZE` emits `(width 3)` rather than the `(width 1)` default. A signal whose graph widths disagree, or that has no grounded width, keeps the honest `(width 1)` default — never a guess (`KG-ISF-COMPLETENESS.2a.i`). **Direction** is lowered from the protocol's *initiator* actor's perspective — see [Which way does each signal point?](#which-way-does-each-signal-point) below (`KG-ISF-COMPLETENESS.2a.ii`).
4. **Constants** — from declared symbolic constants
5. **Types/enums** — from type definitions and enum member-value maps
6. **Storage** — one `(storage (var …))` per register map record; when the register's documented per-field reset values compose to a clean integer it also carries a `(reset V)` (see [Register reset values](#register-reset-values) below)
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
  `(transaction txn_temporal_<id> (on start) (assert <prop>) (complete done))`
  carrying a FSMGen *verification-family* property. As of fsmgen pin `43b29f5c`
  FSMGen **removed** the standalone `(contract … (eventually …))` clause and
  generalized temporal properties into the `(assert/assume/cover …)` family
  (their decisions `0008`/`0009`). `<prop>` depends on whether the rule has a
  representable antecedent:
  - **unguarded** → the anchored monitor `(monitor (within s N))`
    (`FSMGEN-ASSERT-MIGRATE`);
  - **guarded** (a `SignalValue` antecedent) → the faithful implication
    `(=> (== g 1) (within s [MIN] MAX))` — the antecedent is **preserved**
    (the bare monitor dropped it) and a lower bound `> 1` emits the two-operand
    `(within s MIN MAX)` → `##[MIN:MAX]` (`FSMGEN-ASSERT-LOWERING.3`, using
    FSMGen's shipped `ISF-PROPERTY-WINDOW-RANGE`). A *guarded* 0 lower bound has
    no `|-> ##[0:N]` spelling, so it stays a residual.
  The mapping is empirically strict-valid — the fsmgen-binary strict-check tests
  run the new binary on both forms, and a parity oracle keeps the production and
  test classifiers byte-identical.
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

## Register reset values

When a chip-spec PDF documents a register map, SpecForge captures each register's
per-field **reset value** (the value the field powers up at). The `.isf` adapter lowers
that to FSMGen's optional storage reset clause, so the emitted register carries its
documented power-up value instead of silently defaulting to zero:

```
(storage
  (var dpidr_bit_assignments (width 32) (reset 469841015))   ;; powers up at 0x1c013477
  (var claimset_bit_assignments (width 2) (reset 3)))
```

**How the value is built.** A register's reset is composed from its fields by placing each
field's reset value at the field's bit offset (the same bit-tiling used to recover register
layouts) — `reset = field₀ << offset₀ | field₁ << offset₁ | …`. The clause is emitted **only**
when every field carries a value that parses as a clean non-negative integer (decimal, `0x…`,
`0b…`, or `…h`), the values do not overlap, and the result fits the register width.

**Honest by default — never a guessed reset.** If any field's reset is symbolic
(`-`, `X`, `IMPLEMENTATION DEFINED`, …), or only some fields carry a value, the register is
emitted **without** a `(reset V)` clause — FSMGen then defaults it to all-zeros, exactly as
before. A documented all-zeros reset is likewise left implicit (it already *is* the default).
Whenever a documented reset is dropped this way, the adapter records a single honest summary in
its `residual_decisions` (`isf_storage_reset_not_lowered`) saying how many were not lowered and
why — the values themselves always remain in the IntentIR register map. No power-up value is
ever fabricated.

This affects only register-bearing documents (register maps / CSRs); the protocol wire
specifications (APB/AHB/AXI/SWD) emit byte-identical `.isf` as before, because their signal
tables carry no composable register reset.

## Register bit-fields — an honest residual (pending an ISF abstraction)

A register is more than a width and a reset: it is a word partitioned into named **bit-fields**,
each with a bit range, an access type (read-only, write-1-to-clear, …), its own reset, sometimes
an enumeration. SpecForge captures all of that — the IntentIR register map carries every field —
and you can see it in the IntentIR artifact.

What it does **not** yet do is *synthesize* that field map into the `.isf`. The emitted storage
is the opaque `(var <register> (width N) [(reset V)])` you saw above — the register's bits are
there, but which bits mean what is not. The reason is deliberate and honest: today's ISF
`(storage …)` grammar can only declare opaque, width-only variables — it has no construct for
declaring named bit-fields inside a register. Rather than fake one (emitting a separate variable
per field would invent storage the chip does not have, and the runtime field *operators* ISF does
offer describe behaviour the spec never states), SpecForge keeps the field map as faithful
IntentIR metadata and raises the missing capability with the downstream consumer as a feature
request (`docs/FSMGEN_FEEDBACK.md`, 2026-06-22). Nothing is lost — the fields stay in the IntentIR —
and nothing is fabricated. When ISF gains a field-structured-storage construct, the adapter will
lower the field map into it; until then this is a known, documented residual (the corpus-wide
measurement of how much this affects is in `DOC-INTENT-TAXONOMY.2`).

## Which way does each signal point?

Every signal in a `.isf` interface is declared as an `(input …)` or an `(output …)`. But a
protocol signal has no single direction on its own — it depends on *who you are*. The same
`HREADY` wire is an **output** of the subordinate that drives it and an **input** of the manager
that waits on it. Direction is *relative to an actor*.

A `.isf` module, though, describes **one** actor. So SpecForge has to pick a viewpoint, and it
picks the one a reader cares about most: the protocol's **initiator** — the agent that starts
transactions by driving the request and then reads back the response (an AMBA *Manager* /
*Requester*, a debug *Host*). From the initiator's chair, the picture is intuitive: the request
signals it drives are `(output)`, and the response signals it waits on are `(input)`.

SpecForge finds the initiator **without any hard-coded protocol names** (it must work on the
101st protocol it has never seen). It reads the recovered actor↔signal graph and asks a purely
structural question: *which actor drives more than it reads?* The initiator is a **net producer**
— its driven signals outnumber its read signals — because it sources the request. A completer
(which mostly reads the request and drives only a response) and a stray prose fragment (which
looks balanced) are both excluded automatically. On the four AMBA wire specs this picks exactly
the right agent every time: AHB → *Manager*, APB → *Requester*, AXI → *Manager*, SWD → the
*debugger*. The module is then named after that same initiator, so its label and its
`(input)`/`(output)` columns tell one coherent story.

The chosen name is always made a **valid hardware identifier** before it is written out. A module
name (like every identifier in the `.isf`) must match `[A-Za-z_]\w*`, so SpecForge passes it through
one universal rule — keep letters, digits, and underscores; turn anything else into an underscore —
rather than a hand-written list of "bad" characters (a list can never be complete; it once missed a
stray arrow `→` that a messy actor label carried, which made the *entire* file unreadable to the
downstream tool). Clean names are untouched (`Manager` → `manager`), so the wire specs render exactly
as before; only an otherwise-malformed name is repaired. This is why a sprawling spec whose initiator
happens to be an awkward phrase still produces a `.isf` the downstream consumer accepts
(`KG-ISF-COMPLETENESS.2a.iii`).

For example, the APB module is now written from the **Requester's** point of view:

```text
(actor requester
  ...
  (interface
    (output PADDR  (width 1))     ;; the Requester drives the address...
    (output PWDATA (width 1))     ;; ...the write data...
    (output PSEL   (width 1))     ;; ...and the select/enable handshake out
    (input  PREADY (width 1))     ;; ...then waits on the completer's ready...
    (input  PRDATA (width 1))     ;; ...reads back the data...
    (input  PSLVERR (width 1))))  ;; ...and the error response
```

**It never guesses.** A signal the initiator neither clearly drives nor clearly reads — or one the
document simply does not connect to it — keeps the safe `(output)` default rather than inventing a
direction (an *honest residual*). And when a document has no identifiable initiator at all (a
register or command-set spec with no wire actors), nothing changes: the interface is emitted
exactly as before. This is purely a faithfulness improvement to what the `.isf` *says* about each
signal — FSMGen does not depend on signal direction for scheduling, so the change is safe by
construction (a signal flipped to `(input)` is automatically no longer driven), and the four wire
specs still pass FSMGen's strict checker with no new diagnostics.

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
task-tree files remain the machine-tracked authority. The trees
below all closed at the ISF / FSMGen boundary; their `.isf`
adapter behaviour is captured here in one consolidated section.

### `ISF-REGISTER-RESET-EMIT` — register reset values reach the `.isf`

#### Why it mattered

SpecForge already captured each register's per-field **reset value** (the value a field powers up
at) and the adapter already emitted a `(storage (var …))` per register — but the reset was thrown
away at the emit boundary, so every emitted register silently defaulted to all-zeros even when the
PDF documented a specific power-up value. That is exactly the kind of captured intent that should
reach the `.isf` (or be an explicit residual), not vanish.

#### What this tree changed

- **`.0`/`.1` (measure first).** Before writing any emitter code, a read-only sweep of the whole
  persisted corpus established the surface: 1508 registers compose a clean integer reset, the
  documented values are mostly numeric (with a long symbolic tail like `IMPLEMENTATION DEFINED`),
  and — decisively — the protocol wire specs (APB/AHB/AXI/SWD) compose **zero** register resets, so
  the change could not touch them.
- **`.2` (emit).** The storage var gained an optional reset; a register's reset is composed from its
  fields by placing each field's value at its bit offset, and `(reset V)` is emitted only when every
  field carries a clean non-negative integer that fits, with no overlaps. Anything symbolic, partial,
  or unfit is left implicit and recorded as one honest summary residual.
- **`.3` (right-size the register).** The storage var width was the widest single *field*, which
  under-sized multi-field registers (a 32-bit register of two 16-bit fields was emitted at
  `(width 16)`). It now uses the true register width (`size_bits`, never below the highest field
  bit), which both fixes that latent mis-sizing and lets a wider documented reset fit.

#### What you see now

A register-map document's `.isf` now carries each register's documented power-up value at its true
width, e.g. `(var dpidr_bit_assignments (width 32) (reset 469841015))`. A register whose reset the
document leaves symbolic or unspecified is emitted reset-less (FSMGen defaults it to all-zeros), and
the adapter's `residual_decisions` carry a single honest summary of how many resets were not lowered
and why. No power-up value is ever guessed.

#### How it is verified

Composition, parsing, width-sizing and the residual are covered by focused unit tests. On the live
corpus (release binary): the AMBA wire specs emit **byte-identical** `.isf`; CoreSight SoC-600 gains
183 `(reset V)` clauses; every emitted `.isf` passes FSMGen `--strict --check --json` with zero new
diagnostics; the wire-protocol extraction gold (WIRE-BASED-100) and `kg-bench` are unchanged
(register reset is orthogonal to the constraint/relation/temporal surfaces).

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

### `ISF-SYMBOL-SURFACE-EMIT` — emitting the recovered constant/type/enum surface

SpecForge recovers actor-local symbols (constants, enum-like types) from a
spec and **builds** them into the ISF model — but `render()` used to
**discard** them, even though the adapter artifact reported their
`constant_count`/`enum_count`. That was a *count-vs-emission honesty gap*:
the report claimed content the emitted `.isf` never carried.

Closing it needed one contract fact SpecForge could not infer safely, so it
**asked FSMGen** (the `2026-05-29` clarity request) rather than guess.
FSMGen answered (upstream `c0b7eaa7`, locked by `t/1378`): an
`(enums (NAME …))` is **not** a `(type NAME)` alias, so to make a recovered
enum usable as a width-bearing type you **co-declare both**
`(types (type NAME (bits k)))` and `(enums (NAME …))` — accepted, required,
not a conflict — with `k = ceil(log2(member_count))`. That *validated
SpecForge's existing build* (it already co-declares exactly that) and
corrected an earlier "enums-standalone" guess.

So this tree changed only `render()`: it now emits
`(types (type NAME (bits k)))`, `(enums (NAME (M V)…))`, and
`(constants (NAME VALUE))` near the top of the actor body (the book's
shape). One safety rule is load-bearing: a constant/enum value is emitted
only when it is a **whitespace-free scalar** (a literal `0`, a reference
`mode.BUSY`, a width-cast `(8'd5)`); operator expressions (`(| a b)`) render
with internal whitespace and are *not* valid in scalar position, so they
are **excluded rather than emitted as strict-invalid** `.isf`
(residual-honesty).

**Verified** two ways: a render-lock unit test (the surface is emitted, and
an expression-valued constant is skipped), and — the load-bearing one — a
**fsmgen-`--strict` end-to-end test** that builds an actor carrying the
co-declared `(type)`/`(enums)` + a literal `(constants)`, renders it, and
runs the *real* `subs/fsmgen` binary, asserting it accepts the output.
*Authoritative tracking:* `docs/tasks/ISF-SYMBOL-SURFACE-EMIT.md`.

### `ISF-SYMBOL-COUNT-EMITTED` — the symbol counts now match the emitted `.isf`

A small honesty follow-on. The adapter artifact reports
`constant_count`/`enum_count`; SpecForge already derives
`transaction_count`/`rule_count` from the rendered model ("metric ==
emitted content"), but the symbol counts were still counting *recovered*
`symbol_definitions`. Since the symbol emitter excludes expression-valued
entries, the recovered count could over-report what the `.isf` carries. So
`IsfIr` now exposes `emitted_constant_count()`/`emitted_enum_count()` —
DRY-shared with `render()` via private `emitted_constants()`/
`emitted_enums()`, so the count *is* the emitted set by construction — and
the artifact derives both from them. **Verified** by a unit test that mixes
safe and operator-expression-valued symbols and asserts the counts equal
only the safe-emitted subset that `render()` produces.
*Authoritative tracking:* `docs/tasks/ISF-SYMBOL-COUNT-EMITTED.md`.

### `ISF-RULE-CONFLICT-RESIDUAL` — a dropped conflicting rule is now surfaced, not silently lost

FSMGen's strict mode rejects two rules that drive the **same** signal to
**different** values under the **same** guard. The emitter has always had to
drop one of a conflicting pair to keep the `.isf` valid — and it kept the
first and **silently discarded** the rest. That is the one place the emitter
violated the project's own rule: *never hide ambiguity.*

#### Why it mattered

Everywhere else, when the `.isf` emitter can't represent something it records
a `ResidualDecisionPacket` (preserved on `IsfIr.temporal_residuals`, surfaced
by the adapter as `residual_decisions`) — so a human can see exactly what
`.isf` could not carry. The conflict-dedup was the lone exception: a real,
specification-level contradiction (two rules disagreeing on a signal's value)
just vanished from the output with no trace.

#### What this tree changed

The dedup is now a pure, testable
`dedup_conflicting_rules(rules) -> (kept, residuals)`. It still keeps the first
rule and drops the conflicting ones — **the emitted `.isf` is byte-for-byte
unchanged**, so strict validity is untouched — but each dropped conflict now
produces an explicit `ResidualDecisionPacket`:

- it names the signal, the guard, the value the dropped rule wanted, and the
  value the kept rule emits;
- it offers two candidate interpretations (keep the earlier rule vs. keep the
  dropped one) so the decision is visible, not pre-made and hidden;
- it lands in the adapter artifact's `residual_decisions` alongside every
  other honest gap.

#### What you see now

If a spec contains a genuine drive conflict, `specforge adapt`'s artifact
gains an `isf_rule_conflict_<rule>` residual explaining it — instead of the
conflict disappearing. The `.isf` you hand to FSMGen is identical; you just
also get told what was dropped and why.

#### How it is verified

Unit tests over the extracted helper: a conflicting pair → the first rule
kept, the second dropped *and* recorded as a residual naming the signal; a
non-conflicting set (same signal, different guards) → all kept, no residual.
The fsmgen-strict end-to-end tests still pass unchanged (emission is
identical).

*Authoritative tracking:* `docs/tasks/ISF-RULE-CONFLICT-RESIDUAL.md`.

### `FSMGEN-ASSERT-MIGRATE` — following FSMGen's verification-family generalization

This one is a small but important story about staying honest with a *downstream*
contract. SpecForge had asked FSMGen (through the tracked feedback channel) whether
ISF could express the full temporal template directly. FSMGen's answer was the best
kind: *"yes — and it already shipped."* They had **removed** the narrow
`(contract … (eventually s (within N)))` clause and replaced it with a general
*verification family* — `(assert …)` / `(assume …)` / `(cover …)` — that expresses any
`G(antecedent → next/within consequent)` property.

That is good news, but it also means the old spelling SpecForge emitted is now
*invalid*: feed it to the new FSMGen and you get *"unsupported '(contract ...)'
clause"*. So this tree re-pinned the FSMGen submodule to the new version and migrated
the one affected emission — a bounded-eventually now lowers to
`(assert (monitor (within <signal> <N>)))`, the verification-family monitor form.
Nothing else moved: `(stage …)` and the `(rule …)` lowering were unchanged, and the
verification is not on faith — the fsmgen-binary strict-check tests run the *new*
binary on a real SpecForge `.isf` and confirm acceptance.

Two obligations SpecForge mines — *stable* (a value holding across a tick) and windows
with a lower bound greater than one — do not yet have ISF primitives, so they continue
to surface as honest residual decisions until FSMGen adds them. And because the
spec→checkable-property loop now closes *inside* the existing `IntentIR → .isf → FSMGen`
handoff, a separate SpecForge-side SVA exporter became unnecessary and was retired.
*Authoritative tracking:* `docs/tasks/FSMGEN-ASSERT-MIGRATE.md`.

### `ISF-VALUE-WIDTH-EMIT` — value literals that match the signal's width

> **Guarantee:** when SpecForge writes a numeric value into a rule — `(ATID 0x7D)` —
> the number it emits is *exactly as wide as the signal it drives*, and if a value
> genuinely cannot fit the signal, the rule is set aside as an honest residual rather
> than quietly chopped down to size.

Here is the small but real problem this tree fixed. A specification might say "drive
`ATID` to `0x7D`" (that is the decimal number 125, a real 7-bit trace-ID). FSMGen — the
downstream tool that turns the `.isf` into hardware — is strict about bit widths: it reads
`0x7D` as an **8-bit** literal (two hex digits, four bits each) and refuses to assign an
8-bit value onto a signal it believes is 1 bit wide, because silently truncating a number
is exactly the kind of bug a strict checker exists to catch. SpecForge used to hand the
value over verbatim, so the `.isf` was rejected before any hardware could be generated.

There were *two* root causes, and fixing only one would not have been honest:

1. **The signal was emitted too narrow.** A signal can be declared in several places in a
   document, and SpecForge kept the *first* description it saw. When that first description
   didn't state a width (so it defaulted to 1 bit) but a *later* one said "width 7", the
   real width was being thrown away. The fix gathers the width from **all** of a signal's
   descriptions (and from the actor-port graph), and uses the one concrete width they agree
   on — so `ATID` is now correctly emitted at `(width 7)`. If the descriptions *disagree*,
   SpecForge stays at the safe 1-bit default rather than guessing.

2. **The value was never width-matched.** Even with the right width, `0x7D` still needs to
   be written in a form FSMGen accepts. So a final pass re-renders the literal as an explicit
   width-cast — `7'd125` — *only when the value actually fits the signal* (here 125 fits in
   7 bits). When a value genuinely does **not** fit, SpecForge does the honest thing: it
   **drops that one rule and records a residual decision** explaining the mismatch, instead
   of truncating a real number into a wrong one. Plain decimals (which FSMGen treats as
   unsized) and enum symbols are left untouched.

The result, checked against the real FSMGen `--strict --check`: the two affected documents
(the AMBA DTI and the AMBA Trace Bus specs) now pass cleanly, where before they failed with
an "incompatible width" error. The four wire-protocol gold documents pick up **no new**
diagnostics, and because this is a pure *emitter* change — it never touches what was
extracted, only how an already-extracted fact is spelled — the extraction-quality gold
(WIRE-BASED-100) is unaffected by construction. The whole change is arithmetic over the
number and the width: there is no list of signal names anywhere (ADR-0006), so it works on
any chip-specification PDF, not just the ones it was measured on.

Two related issues are deliberately *out* of this tree and tracked separately: an AXI
rule-assignment grammar gap (`(port expr)`) and a DTI case where the value was attached to
the wrong signal upstream of the emitter — fixing those here would have meant guessing.
*Authoritative tracking:* `docs/tasks/ISF-VALUE-WIDTH-EMIT.md`.
