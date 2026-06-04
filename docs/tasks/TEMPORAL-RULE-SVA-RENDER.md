# TEMPORAL-RULE-SVA-RENDER: render mined temporal rules as SystemVerilog Assertions (SVA)

## Metadata

- Tree ID: `TEMPORAL-RULE-SVA-RENDER`
- Status: `deferred` — **LOGGED `2026-06-04` per user direction; design captured, execution
  pending a decision.** The user asked to log the `.isf`→PSL/SVA export "in a tree for later …
  we will decide what to do with it." The decision is between **this SpecForge-side SVA/PSL
  export** and the alternative **FSMGen-native LTL/MTL-in-ISF support** (filed as a suggestion —
  see `FSMGEN-LTL-MTL-SUGGESTION`). Do NOT build `.2` until that decision is made.
- Roadmap lane: `R6`/`R15e` (temporal semantics / literature grounding)
- Created: `2026-06-04`
- Owner: repo-local workflow
- Parent context: the `LITERATURE-GROUNDING` spec→hardware item
  (`docs/research/grounding/spec-to-hardware.md`): *".isf → PSL/SVA assertion export so
  IntentIR contracts become directly checkable in FSMGen/sim — closes the loop with the IEEE
  formalisms."* `TEMPORAL-RULE-LTL-RENDER` (CLOSED) rendered the mined temporal rules in
  generic LTL/MTL; this renders them in **IEEE 1800 SystemVerilog Assertions** — the concrete,
  tool-consumable form a verification engineer pastes into a testbench or a formal tool checks.
  Per ADR-0005, SpecForge *mines*, it does not check — SVA is an **export**, not a checker.

## Scope (bounded, mirrors the LTL renderer)

A **pure, derived** `temporal_rule_to_sva(&TemporalRuleRecord) -> String` in a new
`crates/specforge/src/ir/temporal_sva.rs`, exposed via a book subsection + a KM card — exactly
the shape of `ir/temporal_ltl.rs`. **No IR persistence** (no fixture churn). The **file-export
command** (`*.sva` out) is the explicit deferred follow-up, just as the `validate --ltl` flag
was for the LTL renderer. Renders from `temporal_rules` (IntentIR/SemanticIR — the same source
as the LTL renderer), so it is independent of the `.isf` adapter and the FSMGen handoff
contract (an additive verification artifact for sim/formal tools, not a FSMGen input).

## The SVA mapping (the design substance)

Form (concurrent assertion):

```systemverilog
property <rule_id>;
  @(posedge PCLK) <antecedent> |=> <consequent>;
endproperty
assert property (<rule_id>);
```

- **clock/edge:** `@(posedge <clock>)` for `rising`, `@(negedge <clock>)` for `falling`
  (`clock` = `clock_signal`, else `clk`); `unknown` edge → `posedge` with a `// edge: unknown`
  note.
- **implication / window:** the pre-tick → post-tick delay is the **next cycle** `|=>`; a
  `cycle_window {min,max}` becomes `|-> ##[min:max]` (and `|-> ##n` when `min==max`). An empty
  antecedent is an invariant: `assert property (@(posedge clk) <consequent>);`.
- **predicate → SVA atom** (the value-mapping, using IEEE 1800 system functions where the
  abstract value has one):
  - `SignalValue{sig, HIGH}` → `sig`; `{sig, LOW}` → `!sig`; `{sig, ASSERTED}` → `sig`;
    `{sig, DEASSERTED}` → `!sig` (**active-high convention** — see Non-Goals re: polarity);
    `{sig, VALID}` → `!$isunknown(sig)`; any other value `V` → `sig == V` (best-effort).
  - `SignalStable{sig}` / `ActorMaintainsSignalStable{_,sig}` → `$stable(sig)`.
  - `HandshakeComplete{valid, ready}` → `(valid && ready)`.
  - `ActorDrivesSignal` / `ActorSamplesSignal` / `SignalSampled` → **dropped** — SVA expresses
    *signal values over time*, not *who drives* or *that a value is sampled* (sampling is
    implicit in the clocked property). These co-occur with a `SignalValue` carrying the
    checkable content (e.g. `drive(Completer,PBUSER) & PBUSER==VALID` → `!$isunknown(PBUSER)`),
    so dropping them loses no checkable assertion. If a side becomes empty after dropping, the
    rule renders a `// not SVA-expressible` note instead of a vacuous assertion (no fabrication).

Worked: `PBUSER valid when PSEL, PENABLE, PREADY asserted` →

```systemverilog
property t<id>;
  @(posedge PCLK) PSEL && PENABLE && PREADY |=> !$isunknown(PBUSER);
endproperty
assert property (t<id>);
```

## Non-Goals

- NOT the file-export **command** (deferred follow-up — gives the renderer a CLI consumer).
- NOT IR persistence (derived on demand; zero `kg-bench` fixture churn).
- NOT **polarity-aware** rendering — `ASSERTED/DEASSERTED` use an **active-high convention**;
  threading `signal_polarities` (active-low signals) into the rendering is a documented future
  refinement (the renderer stays pure over a single rule, like the LTL one).
- NOT a checker — SpecForge mines; SVA is an export (ADR-0005). No `.isf`/FSMGen change.

## Acceptance Criteria

- `.1` design owned (this file), registered in `docs/TASK_TREE.md`.
- `.2`: pure `ir/temporal_sva.rs` (`temporal_rule_to_sva` + atom helper) with unit tests (the
  worked PBUSER `|=>` rule; a windowed `|-> ##[min:max]`; `$stable`/`$isunknown`/handshake
  atoms; an empty-antecedent invariant; a drive-only side → `// not SVA-expressible`); a
  user-friendly book subsection (`domain/temporal-semantics.md`, alongside the LTL one) +
  a KM card; full `scripts/run_ci.sh` GREEN; tree CLOSED.

## Task Tree

- ID: `TEMPORAL-RULE-SVA-RENDER`
  Status: `active`
  Children: `.1` (design) · `.2` (renderer + tests + book + KM card + close)

- ID: `TEMPORAL-RULE-SVA-RENDER.1`
  Status: `done`
  Goal: own + design (this file) — the SVA mapping (clock/edge, `|=>` / `|-> ##[min:max]`,
    per-predicate atoms incl. `$stable`/`$isunknown`, dropping non-SVA-expressible predicates,
    the active-high convention), the bounded renderer-only scope (command + polarity deferred),
    independence from the FSMGen contract. Docs-only.
  Acceptance: design recorded; registered.
  Verification: passed (`2026-06-04`) — SVA mapping fixed against the real `TemporalRuleRecord`
    + `TemporalPredicateRecord` shapes; IEEE 1800 system functions (`$stable`, `$isunknown`)
    chosen for `stable`/`valid`; `|=>` (next cycle) / `|-> ##[min:max]` (window) chosen;
    actor-drive/sample predicates dropped (not SVA-expressible) with the no-vacuous-assertion
    guard; active-high convention + polarity-awareness deferral documented; renderer-only scope
    (command deferred) mirrors `TEMPORAL-RULE-LTL-RENDER`; renders from `temporal_rules`
    (independent of `.isf`/FSMGen). ADR-0005 (mine, don't check) reaffirmed.
  Commit: `see Commit Log`

- ID: `TEMPORAL-RULE-SVA-RENDER.2`
  Status: `deferred` (do NOT build until the user decides: SpecForge-side SVA export vs
    FSMGen-native LTL/MTL-in-ISF — `FSMGEN-LTL-MTL-SUGGESTION`)
  Goal: implement `ir/temporal_sva.rs` (pure renderer + atom helper) + tests; book subsection;
    KM card; close.
  Acceptance: tests green; book + KM card; full CI GREEN; tree CLOSED.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `TEMPORAL-RULE-SVA-RENDER.1` | `done` | owned + designed (SVA mapping + bounded scope) — the **log** |
| 2 | `TEMPORAL-RULE-SVA-RENDER.2` | `deferred` | renderer + tests + book + KM card + close — **pending user decision** |

**DEFERRED `2026-06-04` (logged, not built).** Per user direction, the `.isf`→PSL/SVA export is
logged here for a later decision. The design (`.1`) is captured so the decision is informed; the
implementation (`.2`) waits until the user chooses between this SpecForge-side SVA export and
FSMGen adding native LTL/MTL support to ISF (filed as `FSMGEN-LTL-MTL-SUGGESTION` — if ISF gains
LTL/MTL, SpecForge could lower `temporal_rules` straight into ISF, no separate SVA export needed).

## Decisions

- `2026-06-04`: render-only (pure, derived), mirroring the LTL renderer — no IR persistence,
  no export command yet, no polarity threading (active-high convention). Drop non-SVA-expressible
  predicates (actor-drive/sample) rather than fabricate; never emit a vacuous assertion. SVA is
  an export, not a checker (ADR-0005); independent of the `.isf`/FSMGen handoff.

## Blockers

- None. Pure derived rendering over the existing temporal IR.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-04` | `.1` | SVA mapping fixed vs real temporal types; `$stable`/`$isunknown`, `|=>`/`|-> ##[min:max]`, drop-non-expressible + no-vacuous guard, active-high convention; renderer-only scope; independent of FSMGen; docs-only | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `TEMPORAL-RULE-SVA-RENDER.1` | `TEMPORAL-RULE-SVA-RENDER.1 — own + design SVA rendering of mined temporal rules` | docs-only |

## Changelog

- `2026-06-04`: Created — render mined `temporal_rules` as IEEE 1800 SystemVerilog Assertions
  (the spec→hardware `LITERATURE-GROUNDING` item), the concrete tool-consumable successor to the
  LTL renderer. Pure/derived/renderer-only (command + polarity deferred); SVA is an export, not
  a checker (ADR-0005).
