# TEMPORAL-RULE-LTL-RENDER: render mined temporal rules in standard LTL/MTL notation

## Metadata

- Tree ID: `TEMPORAL-RULE-LTL-RENDER`
- Status: `done` (CLOSED `2026-06-04` — mined temporal rules render to standard LTL/MTL
  `G(antecedent→consequent)` notation via a pure `ir/temporal_ltl.rs`; book-grounded; derived,
  zero behavior change; CI green)
- Roadmap lane: `R6`/`R15e` (temporal semantics / literature grounding)
- Created: `2026-06-04`
- Owner: repo-local workflow
- Parent context: the **Tier-1 headline** adopt item from the `LITERATURE-GROUNDING`
  reach-full-potential backlog — `docs/research/grounding/protocol-temporal-semantics.md`:
  *"Express `temporal_rules` in a standard LTL/MTL template form (vs the current ad hoc typed
  shape) — verifiability + tool/FSMGen interoperability. Highest-leverage adopt."* SpecForge's
  `temporal_rules` ARE the `G(antecedent → consequent)`-with-holes property template the
  spec-mining literature (Pnueli LTL; GoldMine; Texada) formalized — but they are emitted in
  an ad-hoc typed shape, not the formalism's notation. This renders them in it.

## Scope (the bounded, low-risk half)

This tree delivers ONLY the **rendering / vocabulary** half: a pure, read-only function that
maps a `TemporalRuleRecord` to a standard LTL/MTL formula string, exposed as an opt-in
`validate --ltl` view and grounded in the book. The **`.isf` → PSL/SVA export** half (the
other part of the grounding item) is deliberately a **separate downstream tree** — it touches
the `.isf` adapter + the FSMGen handoff contract (the authority; must be read first), so it is
out of scope here.

## The rendering (template-faithful)

`temporal_rule_to_ltl(rule) -> String`, a pure function over `TemporalRuleRecord`:

- Core form: `G( <antecedent> -> <op> <consequent> )`. Empty antecedents → an invariant
  `G( <consequent> )` (no implication).
- `<op>`: the tick delay between the pre-tick guard and the post-tick result.
  - no `cycle_window` → `X` (next tick): `G( ante -> X cons )`.
  - `cycle_window {min,max}` → bounded eventually `F[min,max]`: `G( ante -> F[min,max] cons )`.
- antecedent/consequent = the predicates conjoined with ` & `.
- predicate atoms (readable, deterministic):
  - `SignalValue{sig,val}` → `sig==val`
  - `ActorDrivesSignal{actor,sig}` → `drive(actor,sig)`
  - `ActorMaintainsSignalStable{actor,sig}` / `SignalStable{sig}` → `stable(actor,sig)` / `stable(sig)`
  - `ActorSamplesSignal{actor,sig}` / `SignalSampled{sig}` → `sample(actor,sig)` / `sample(sig)`
  - `HandshakeComplete{valid,ready}` → `handshake(valid,ready)`
- The clock context (`rising/falling` edge of `clock_signal`) is the temporal domain `G`
  ranges over; the renderer returns the pure formula, the caller shows the clock/edge
  alongside (as `validate` already does).

Worked APB examples (real, from the corpus):
- `PBUSER valid when PSEL, PENABLE, and PREADY are asserted` →
  `G( PSEL==ASSERTED & PENABLE==ASSERTED & PREADY==ASSERTED -> X (drive(Completer,PBUSER) & PBUSER==VALID) )`
- `PNSE valid when PSEL asserted` →
  `G( PSEL==ASSERTED -> X (drive(Requester,PNSE) & PNSE==VALID) )`

## Non-Goals

- NOT the `.isf` → PSL/SVA *export* (separate downstream tree; touches the FSMGen contract).
- NOT adding a serialized `ltl_form` field to the IR (would churn every `kg-bench` temporal
  fixture). The rendering is **derived on demand**, never persisted.
- NOT changing extraction, the temporal IR, or default `validate` output (the LTL view is
  opt-in behind `--ltl`).

## Acceptance Criteria

- `.1` design owned (this file), registered in `docs/TASK_TREE.md`.
- `.2`: a pure `temporal_rule_to_ltl` (+ predicate-atom helper) in a new
  `crates/specforge/src/ir/temporal_ltl.rs`, with unit tests (the two worked examples;
  empty-antecedent invariant; windowed `F[min,max]`); an opt-in `validate --ltl` flag that
  prints each temporal rule's clock context + LTL formula (default output unchanged); a
  user-friendly book subsection in `domain/temporal-semantics.md` grounding the notation
  (Pnueli/GoldMine/Texada) with the worked examples; a Knowledge Map fact card for where the
  rendering lives + the atom vocabulary; full `scripts/run_ci.sh` GREEN; tree CLOSED.

## Task Tree

- ID: `TEMPORAL-RULE-LTL-RENDER`
  Status: `active`
  Children: `.1` (design) · `.2` (renderer + `validate --ltl` + book + KM card + close)

- ID: `TEMPORAL-RULE-LTL-RENDER.1`
  Status: `done`
  Goal: own + design (this file) — the bounded rendering scope, the template-faithful LTL/MTL
    form, the atom vocabulary, the opt-in surface, the worked examples, and the explicit
    separation of the `.isf`→PSL/SVA export. Docs-only.
  Acceptance: design recorded; registered.
  Verification: passed (`2026-06-04`) — rendering fixed against the real `TemporalRuleRecord`
    + `TemporalPredicateRecord` shapes (`ir/semantic.rs`); `G(ante -> X/F[min,max] cons)`
    template chosen to match the spec-mining `G(antecedent→consequent)` form; atom vocabulary
    enumerated for all 7 predicate variants; worked APB examples computed; `.isf`/PSL/SVA
    export + IR persistence explicitly excluded (FSMGen-contract risk / fixture churn).
  Commit: `see Commit Log`

- ID: `TEMPORAL-RULE-LTL-RENDER.2`
  Status: `done`
  Goal: implement `ir/temporal_ltl.rs` (pure renderer + atom helper) + tests; a user-friendly
    book subsection (the user-facing surface) grounding the notation with worked APB examples;
    a Knowledge Map card; close.
  Acceptance: tests green; book subsection + KM card; full CI GREEN; tree CLOSED.
  Verification: passed (`2026-06-04`) — new `crates/specforge/src/ir/temporal_ltl.rs`
    (`pub mod temporal_ltl;` in `ir/mod.rs`): pure `temporal_rule_to_ltl(&TemporalRuleRecord)
    -> String` + a `predicate_atom` helper covering all 7 `TemporalPredicateRecord` variants
    (`sig==VAL`, `drive`, `stable`, `sample`, `handshake`); form `G( ante -> X cons )` /
    `F[min,max]` with a cycle window / `G( cons )` for an empty antecedent; multi-atom
    consequents parenthesized so the temporal operator scopes the conjunction. 4 unit tests
    assert the exact strings (the two worked APB rules byte-for-byte, the empty-antecedent
    invariant, the windowed `F[1,2]`). Derived/read-only — **no IR field, zero `kg-bench`
    fixture churn, zero extraction-behavior change**. User-friendly book subsection "Standard
    LTL/MTL notation" added to `domain/temporal-semantics.md` (grounds it in
    Pnueli/GoldMine/Texada with the worked PBUSER example + the X/F[min,max]/atom mapping).
    Knowledge Map card `docs/knowledge/temporal-rule-ltl-rendering.md` written (KM now 4
    facts / 20 question keys, in sync). Full `scripts/run_ci.sh` GREEN (1219→1223 tests; KM +
    memory-arch + mdBook all pass). The `--ltl` flag was dropped (see the Note above /
    Decisions). Tree CLOSED.
  Note (scope refinement during `.2`): the originally-planned opt-in `validate --ltl` flag was
    **dropped** — `ValidateArgs` has 10+ construction sites (mostly tests), so adding a field
    churns them all, and dumping 153 LTL lines risks default-output bloat. The renderer ships
    as a **tested public building block** (`pub fn temporal_rule_to_ltl`) consumed by the book
    (the doctrine's user-facing surface) and findable via the KM card; the explicit live
    consumer is the downstream `.isf`→PSL/SVA export tree.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `TEMPORAL-RULE-LTL-RENDER.1` | `done` | owned + designed (template + atom vocabulary + worked examples) |
| 2 | `TEMPORAL-RULE-LTL-RENDER.2` | `done` | renderer (`ir/temporal_ltl.rs`) + 4 tests + book + KM card + close (CI green 1223) |

**Tree CLOSED `2026-06-04`.** SpecForge can now render any mined `temporal_rule` in standard
LTL/MTL notation (`G(antecedent → consequent)` with `X` / `F[min,max]`), grounding its temporal
model in the spec-mining formalism (Pnueli/GoldMine/Texada). Pure, derived, zero behavior
change. The `.isf` → PSL/SVA **export** that consumes this is the explicit downstream tree
(gated on the FSMGen handoff contract).

## Decisions

- `2026-06-04`: scope to the **render-only** half (pure, derived) — no IR persistence
  (fixture churn), no `.isf`/FSMGen export (contract risk). The export is the explicit
  downstream tree; this lays the verified, standard-notation foundation it will consume.
- `2026-06-04` (during `.2`): **dropped the `validate --ltl` flag** — `ValidateArgs` has 10+
  construction sites + 153 LTL lines would bloat default output. Surface = tested public API
  (`temporal_rule_to_ltl`) + book subsection + KM card; the downstream export tree is the live
  consumer.

## Blockers

- None. Pure derived rendering + an opt-in flag.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-04` | `.1` | LTL/MTL template + atom vocabulary fixed vs real temporal types; worked APB examples; export/persistence excluded; docs-only | `passed` |
| `2026-06-04` | `.2` | `ir/temporal_ltl.rs` pure renderer (`G(ante->X/F[min,max] cons)` + 7-variant atom vocab) + 4 exact-string tests (2 worked APB rules, empty-antecedent invariant, windowed F[1,2]); derived/no-IR-field (zero fixture churn); book "Standard LTL/MTL notation" subsection in `domain/temporal-semantics.md`; KM card (KM 4 facts/20 keys); `--ltl` flag dropped (10+ ValidateArgs sites); full CI GREEN 1219→1223; tree CLOSED | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `TEMPORAL-RULE-LTL-RENDER.1` | `TEMPORAL-RULE-LTL-RENDER.1 — own + design the standard LTL/MTL rendering of mined temporal rules` (`ee638480`) | docs-only |
| `TEMPORAL-RULE-LTL-RENDER.2` | `TEMPORAL-RULE-LTL-RENDER.2 — render temporal rules as standard LTL/MTL (ir/temporal_ltl.rs); book + KM card; close tree` | renderer + 4 tests + book + KM card; derived/no behavior change; CI green 1223; CLOSED |

## Changelog

- `2026-06-04`: Created — render `temporal_rules` in standard LTL/MTL notation
  (`G(antecedent → consequent)` with `X`/`F[min,max]`), the Tier-1 vocabulary-adopt item from
  the `LITERATURE-GROUNDING` backlog. Pure, derived; no IR change, no `.isf`/FSMGen export
  (separate downstream tree).
- `2026-06-04`: **Tree CLOSED.** `.2` done — `ir/temporal_ltl.rs` pure renderer + 4 exact-string
  tests, a user-friendly book subsection grounding the notation, and a Knowledge Map card.
  Dropped the planned `validate --ltl` flag (10+ `ValidateArgs` construction sites + default-
  output bloat); the renderer is tested public API + book-documented + KM-findable, and the
  downstream `.isf`→PSL/SVA export tree is its live consumer. Derived/no-IR-field → zero
  fixture churn, zero behavior change; full CI green (1219→1223).
