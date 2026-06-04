# FSMGEN-ASSERT-LOWERING: lower stable / antecedent→consequent / min>1 obligations into the ISF verification family

## Metadata

- Tree ID: `FSMGEN-ASSERT-LOWERING`
- Status: `done` (CLOSED `2026-06-04` — re-pin `92d7036b`; stable verified-negative; guarded
  windowed-eventual lowered faithfully; `.1`–`.3`)
- Roadmap lane: `R6` (FSMGen handoff / ISF adapter)
- Created: `2026-06-04`
- Owner: repo-local workflow
- Parent context: FSMGen shipped **both** flagged deltas — `(stable/changed/rose/fell SIG)`
  predicates (`ISF-PROPERTY-SAMPLED-VALUE`, `6700fbb4`) and the `min > 1` bounded window
  `(within B MIN MAX)` → `##[MIN:MAX]` (`ISF-PROPERTY-WINDOW-RANGE`, `92d7036b`, **with `1 <= MIN
  <= MAX` locked exactly per `FSMGEN-MIN-WINDOW-CONFIRM`**). With these, obligations SpecForge
  currently routes to **residuals** can lower to real ISF verification-family properties.

## What lowers now (the opportunity)

`ir/isf_ir.rs::classify_actor_contract` lowers a `temporal_rule` to exactly one disposition.
Today only two emit: `Obligation::Eventually{Level, Within{max}}` → the bounded-eventually
monitor `(assert (monitor (within s max)))` (`FSMGEN-ASSERT-MIGRATE`), and `Obligation::Drive`
→ an actor `(rule …)`. **Everything else residualizes** — including stability obligations
(`Obligation::Stable`) and general antecedent→consequent shapes. The new primitives unlock:

| Obligation | New ISF | Generated SVA | Status |
| --- | --- | --- | --- |
| `Stable{signal}` (+ optional guard) | `(assert (stable s))` / `(assert (=> g (stable s)))` | `$stable(s)` / `(g) \|-> $stable(s)` | **`.2`** |
| `Eventually` with `min > 1` | `(assert (=> g (within s MIN MAX)))` | `(g) \|-> ##[MIN:MAX] (s)` | `.3` |
| general antecedent→consequent value | `(assert (=> A B))` / `(=> A (next B))` | `(A) \|-> (B)` / `\|-> ##1 (B)` | `.3` |

All empirically strict-valid against `92d7036b` (`(assert (stable RVALID))` and
`(assert (=> RREADY (stable RVALID)))` both return `success=true`).

## Design

- New disposition `TemporalRuleDisposition::AssertProperty { rule_id, prop: String }` → renders
  `    (assert <prop>)`. (`prop` is a pre-built ISF boolean expression.)
- A shared predicate→ISF-boolean helper: `(value sig HIGH)` → `sig`, `(value sig LOW)` →
  `(! sig)`, `(value sig V)` → `(== sig V)`, `(stable sig)` → `(stable sig)`, `(handshake v r)`
  → `(& v r)`; a conjunction → `(& p1 p2 …)`. Only **declared** signals lower; anything with an
  undeclared signal or a non-representable predicate → residual (no fabrication — the existing
  fail-to-residual doctrine).
- **Parity is mandatory:** `classify_temporal_rule` (the test-only oracle) and
  `classify_actor_contract` (production) must agree pointwise — every new arm is added to BOTH,
  and the `classify_actor_contract_is_parity_equivalent_to_classify_temporal_rule` test must stay
  green.
- Empirical re-validation: every new emitted form is checked by the fsmgen-binary strict-check
  tests (`run_fsmgen_strict_check`) against the new pin (not on faith).

## Slices

- **`.1`** — own + design (this file) + **re-pin `subs/fsmgen` `43b29f5c → 92d7036b`** (existing
  emission re-validated green on the new pin; the new forms empirically strict-valid). Picks up
  both shipped primitives; no behavior change yet.
- **`.2`** — **stable**: `Obligation::Stable{signal}` → `(assert (stable s))`, with an optional
  declared guard → `(assert (=> g (stable s)))`. The `AssertProperty` disposition + the
  predicate helper (stable + value leaves) + both classify arms (parity) + render + the
  no-silent-drop metric + tests (incl. fsmgen-binary strict-check) + update the existing
  stable-residual tests to expect the new lowering. Book + KM.
- **`.3`** — **general `=>` + min>1**: antecedent→consequent values → `(assert (=> A B))` /
  `(=> A (next B))` / `(=> A (within B MIN MAX))`; windowed `min > 1` → `(within B MIN MAX)`.
  Extends the predicate helper to full conjunctions. Then close.

## Non-Goals / boundaries

- NOT changing the bounded-eventually monitor (`FSMGEN-ASSERT-MIGRATE`) or `(stage …)`.
- NOT lowering predicates that reference undeclared signals or have no ISF spelling
  (actor-drive/sample identity) → they stay residuals (honesty preserved).
- NOT rushing: each slice is empirically strict-validated + parity-checked before commit.

## Acceptance Criteria

- `.1`: design (this file) + re-pin `92d7036b`; existing emission CI-green on the new pin;
  registered.
- `.2`: stable lowering + parity + strict-check tests + metric + book + KM; full CI GREEN.
- `.3`: general `=>` + min>1 lowering + parity + strict-check tests; full CI GREEN; tree CLOSED.

## Task Tree

- ID: `FSMGEN-ASSERT-LOWERING`
  Status: `active`
  Children: `.1` (design + re-pin) · `.2` (stable) · `.3` (general `=>` + min>1 + close)

- ID: `FSMGEN-ASSERT-LOWERING.1`
  Status: `done`
  Goal: own + design (this file); re-pin `subs/fsmgen` to `92d7036b`; empirically confirm the new
    forms + the existing-emission re-validation.
  Acceptance: design recorded; re-pinned; CI green on the new pin; registered.
  Verification: passed (`2026-06-04`) — both deltas confirmed shipped (`6700fbb4` stable,
    `92d7036b` window-range with `1<=MIN<=MAX` locked per our answer); `(assert (stable RVALID))`
    and `(assert (=> RREADY (stable RVALID)))` empirically `success=true` on `92d7036b`; full
    `scripts/run_ci.sh` GREEN on the new pin (existing emission re-validated). Disposition +
    predicate-helper + parity design fixed.
  Commit: `see Commit Log`

- ID: `FSMGEN-ASSERT-LOWERING.2`
  Status: `done` (**verified-negative — stable lowering is NOT faithfully representable; no code change**)
  Goal: stable obligations → `(assert (stable s))` / `(assert (=> g (stable s)))`.
  Acceptance: investigate fidelity; lower only if faithful, else keep residual + record.
  Verification: passed (`2026-06-04`) — **investigation found the stable lowering would be
    UNFAITHFUL, so it is NOT implemented; the residual stays.** Every mined `SignalStable` /
    `ActorMaintainsSignalStable` carries `from_phase`+`to_phase` → `Obligation::Stable { during:
    Window::Between { tick_phase_events } }` (`contract.rs` ~L432): "stable **during**
    `[from_phase, to_phase]`". FSMGen's `(stable s)` is **unconditional per-tick** stability
    (`$stable(s)` every edge) — strictly stronger, so `(assert (stable s))` would over-assert
    (fidelity bug). The faithful `(assert (=> g (stable s)))` needs a boolean `g` for "inside the
    phase interval", but tick-phases are abstract markers, not `.isf` signals → no such guard
    exists, and the model has no level-guarded stability variant. So the existing residual
    ("bare stability across tick phases has no supported `.isf` construct") is **correct**.
    Recorded as KM card `stable-obligation-phase-scoped-residual`. No code change.

- ID: `FSMGEN-ASSERT-LOWERING.3`
  Status: `done`
  Goal: general antecedent→consequent `(assert (=> A B))` + min>1 `(within B MIN MAX)`; close.
  Acceptance: parity green; strict-check green; full CI GREEN; tree CLOSED.
  Verification: passed (`2026-06-04`) — **faithful fidelity fix shipped**: a *guarded*
    windowed-eventual now keeps its antecedent. Generalized `IsfContract` to carry a pre-built
    `prop` and the `Contract` disposition to `{ name, prop }` (the unguarded monitor output is
    byte-identical); added `windowed_eventual_prop` (unguarded → `(monitor (within s max))`;
    guarded → `(=> g (within s [min] max))`, `min` emitted only when `>= 2`; guarded `min=0` →
    `None` → residual) + `actor_guard_condition` (production guard, parity-matched to the oracle's
    `temporal_antecedent_condition`). Wired into **both** `classify_actor_contract` (production)
    and `classify_temporal_rule` (oracle) — the **parity test stays green**. 4 new tests
    (guarded→implication; min>1→two-operand window; guarded min=0→residual; end-to-end
    fsmgen-binary strict-check on `(=> RREADY (within RVALID 2 5))`). Book + KM updated. Full
    `scripts/run_ci.sh` GREEN (1239→1243). Tree CLOSED. (Unguarded `min>1` still uses the
    anchored monitor — no `##[min:max]` without an antecedent — noted, not regressed.)

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `FSMGEN-ASSERT-LOWERING.1` | `done` | design + re-pin `92d7036b` (validated) |
| 2 | `FSMGEN-ASSERT-LOWERING.2` | `done` | **verified-negative** — stable lowering would over-assert (phase-scoped ≠ unconditional `(stable s)`); residual is correct |
| 3 | `FSMGEN-ASSERT-LOWERING.3` | `done` | guarded windowed-eventual → `(=> g (within s [min] max))` (keeps antecedent + min); parity + strict-check green → **tree CLOSED** |

**Tree CLOSED `2026-06-04`.** Re-pinned `92d7036b`; stability obligations correctly stay residual
(`.2` fidelity finding); guarded windowed-eventuals now lower faithfully to
`(assert (=> g (within s [min] max)))` (`.3`), preserving the antecedent the monitor dropped and
honoring `min > 1`. Parity oracle green; both forms fsmgen-binary strict-validated; CI green 1243.

## Decisions

- `2026-06-04`: add an `AssertProperty` disposition + a shared predicate→ISF-boolean helper;
  parity oracle mandatory; empirically strict-validate every new form; declared-only, else
  residual. Slice stable first (FSMGen's explicit ask, simplest), then general `=>`/min>1.

## Blockers

- None — both primitives shipped; re-pin validated.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-04` | `.1` | both deltas shipped (`6700fbb4`/`92d7036b`, MIN-lock per our answer); stable forms strict-valid on `92d7036b`; existing emission CI-green on new pin; design fixed | `passed` |
| `2026-06-04` | `.2` | **verified-negative**: stability obligations are phase-scoped (`Between{tick_phases}`), not faithfully representable by FSMGen's unconditional `(stable s)` (would over-assert); residual is correct; KM card `stable-obligation-phase-scoped-residual`; no code change | `passed` |
| `2026-06-04` | `.3` | guarded windowed-eventual → `(=> g (within s [min] max))` (preserves antecedent + min); `IsfContract`/`Contract` carry a pre-built `prop`; `windowed_eventual_prop` + `actor_guard_condition`; wired into BOTH classify paths (parity test green); 4 new tests incl. fsmgen-binary strict-check; book + KM; full CI GREEN 1243 | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `FSMGEN-ASSERT-LOWERING.1` | `FSMGEN-ASSERT-LOWERING.1 — design + re-pin subs/fsmgen 92d7036b (both deltas shipped); validate` | re-pin + docs |
| `FSMGEN-ASSERT-LOWERING.2` | `FSMGEN-ASSERT-LOWERING.2 — verified-negative: stability obligations stay residual (phase-scoped != unconditional (stable s)); KM card` | docs-only; no code change |
| `FSMGEN-ASSERT-LOWERING.3` | `FSMGEN-ASSERT-LOWERING.3 — lower guarded windowed-eventuals to (=> g (within s [min] max)) (keep antecedent + min); parity + strict-check; close tree` | +4 tests; CI green 1243 |

## Changelog

- `2026-06-04`: Created — FSMGen shipped `(stable …)` + `(within B MIN MAX)`. Design the lowering
  of stability / antecedent→consequent / min>1 obligations (currently residuals) into the ISF
  verification family; re-pin `92d7036b`; slice stable first.
- `2026-06-04`: `.2` **verified-negative** — investigating the stable slice found the
  `(stable s)` primitive does NOT faithfully represent SpecForge's phase-scoped stability
  obligations (over-assert risk); residual is correct, no code change, finding carded. The
  genuinely-faithful enabled lowering is `.3` (antecedent→consequent + min>1, where the
  antecedent is a representable boolean).
- `2026-06-04`: **Tree CLOSED.** `.3` shipped the faithful guarded windowed-eventual lowering —
  `(assert (=> g (within s [min] max)))` preserving the antecedent the monitor dropped + honoring
  `min > 1` — via a pre-built `prop` on `IsfContract`/`Contract`, the `windowed_eventual_prop` +
  `actor_guard_condition` helpers, wired into both classify paths (parity green), 4 new tests
  (incl. fsmgen-binary strict-check), book + KM. CI green 1243.
