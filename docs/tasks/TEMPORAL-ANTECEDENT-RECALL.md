# TEMPORAL-ANTECEDENT-RECALL: distribute a shared assertion-value across coordinated condition signals

## Metadata

- Tree ID: `TEMPORAL-ANTECEDENT-RECALL`
- Status: `done` (CLOSED `2026-06-02` — guarded shared-value distribution fixed the temporal
  antecedent under-capture; eval recall 0.667→1.000 on real APB; CI green)
- Roadmap lane: `R8`/`R15e` (extraction accuracy / completeness)
- Created: `2026-06-02`
- Owner: repo-local workflow
- Parent context: **the second catch of the measure→catch→fix loop** — surfaced by the new
  `TEMPORAL-RULE-EVAL` (CLOSED). On the real AMBA APB spec the temporal eval scored
  `temporal_rule P=0.400 R=0.667 F1=0.500 (tp=2 fp=3 fn=1)`, and investigation traced the
  one false negative to a genuine, live temporal-parser bug (the false positives trace
  upstream to the constraint tier — see Non-Goals).

## The bug (root-caused against the real data)

For the statement *"PBUSER must be valid when PSEL, PENABLE, **and** PREADY are asserted"*,
the EvidenceIR signal constraint `sigcon_0026` correctly carries the full
`condition_text = "PSEL , PENABLE , and PREADY are asserted."`. But the temporal parser
emits an antecedent of only `PREADY`, dropping PSEL and PENABLE — so the mined rule fires on
a **weaker precondition than the spec demands** (a real semantic recall gap).

Root cause in `parse_temporal_condition_predicates`
(`crates/specforge/src/ir/semantic.rs`): the condition is split on commas (and `&&`) into
clauses, then `split_temporal_condition_segment_on_and` splits the final segment on `" and "`.
For `"PSEL , PENABLE , and PREADY are asserted"` the clauses become
`["PSEL", "PENABLE", "and PREADY are asserted"]`. `parse_temporal_condition_clause` then
**drops** `"PSEL"` and `"PENABLE"` because, in isolation, those clauses carry **no value
keyword** (`asserted`/`HIGH`/`LOW`/…) — the shared trailing predicate "are asserted" sits
only on the last clause. So a coordinated signal list `"A, B, and C are asserted"` collapses
to just `C`.

## The fix

Distribute a **single shared trailing value** across the coordinated signal list: when a
condition yields ≥2 signal-bearing clauses, exactly one distinct value appears among them,
and ≥1 signal-bearing clause has **no** value of its own, fill the value-less clauses with
that shared value. So `"PSEL, PENABLE, and PREADY are asserted"` →
`SignalValue(PSEL=ASSERTED), SignalValue(PENABLE=ASSERTED), SignalValue(PREADY=ASSERTED)`.

Guard tightly to avoid over-application:
- only when there are **≥2 signals** in the condition and a value-less signal clause exists;
- only when there is **exactly one distinct value** among the clauses (so a genuinely mixed
  list like `"PSEL HIGH, PREADY LOW"` is untouched);
- bare signal mentions with no value and no shared value are still dropped (no fabrication).

This is a **recall** fix in the temporal condition parser; it does not change the upstream
constraints, the IRs, or any other stage.

## Verification plan

- Unit test on `parse_temporal_condition_predicates` (or `build_temporal_rules`): the
  coordinated-list condition yields one predicate **per signal**; a mixed-value list is
  unchanged; a bare value-less signal with no shared value is still dropped.
- **Re-run the temporal eval end-to-end** (`eval-extraction seed_apb_temporal.json`): expect
  the PBUSER rule to gain all three antecedents → the gold (3-condition) now matches →
  `tp 2→3`, the PBUSER under-captured FP resolves and the FN clears → **P 0.400→0.600,
  R 0.667→1.000, F1 0.500→0.750**. (The remaining 2 FPs are the upstream degenerate-header
  constraints — out of scope, see Non-Goals.) This improvement happens **at SemanticIR-build
  time**, so it needs no re-ingest.
- Full `scripts/run_ci.sh` GREEN, including the `kg-bench` temporal fixtures (update any
  fixture whose expected temporal rule legitimately gains the now-distributed antecedents —
  the fixture must describe what the corrected code does).

## Non-Goals

- NOT fixing the temporal **false positives** the eval also surfaced — the degenerate
  self-referential header rule (`sigcon_0033`: subject `PSEL` from *"The following signals
  must be valid when PSEL is asserted:"*) and the `USER_RESP_WIDTH`-as-subject rule
  (`sigcon_0027`). Both trace to the **constraint tier** in `evidence.rs`, and the
  `*_WIDTH`-subject class is exactly what `CONSTRAINT-SUBJECT-PRECISION` already fixed; the
  on-disk APB EvidenceIR is **stale** (pre-`CONSTRAINT-SUBJECT-PRECISION`), so a clean
  re-ingest is expected to remove them. They are re-ingest-gated and tracked as a
  constraint-tier follow-up, not re-fixed here.
- NOT changing the temporal IR, the constraint extraction, or `.isf` lowering.

## Acceptance Criteria

- `.1` design owned (this file), registered in `docs/TASK_TREE.md`.
- `.2`: the shared-value distribution implemented in `parse_temporal_condition_predicates`
  with a tight guard; unit test(s); the temporal eval re-run shows the predicted recall
  improvement (P 0.400→0.600, R 0.667→1.000); book note refreshed if the eval numbers in
  `quality/extraction-eval.md` change; full CI GREEN; tree CLOSED.

## Task Tree

- ID: `TEMPORAL-ANTECEDENT-RECALL`
  Status: `active`
  Children: `.1` (design) · `.2` (fix + unit test + eval re-run + book + close)

- ID: `TEMPORAL-ANTECEDENT-RECALL.1`
  Status: `done`
  Goal: own + design (this file) — root-cause the antecedent under-capture against the real
    APB data, fix the scope to the temporal condition parser, separate the upstream
    constraint-tier FPs (Non-Goals), set the verification plan (incl. the expected eval
    delta). Docs-only.
  Acceptance: design recorded + registered.
  Verification: passed (`2026-06-02`) — root cause located in
    `parse_temporal_condition_predicates` (coordinated-list value not distributed),
    confirmed against `sigcon_0026` (`condition_text` is complete; the parser drops PSEL +
    PENABLE); the FP cohort separated as upstream/stale; expected eval delta computed.
  Commit: `see Commit Log`

- ID: `TEMPORAL-ANTECEDENT-RECALL.2`
  Status: `done`
  Goal: implement the shared-value distribution (guarded) + unit test(s); re-run the temporal
    eval to confirm the recall improvement; refresh the book eval numbers if they change;
    full CI; close.
  Acceptance: unit test green; eval P 0.400→0.600 / R 0.667→1.000; CI GREEN; tree CLOSED.
  Verification: passed (`2026-06-02`) — rewrote `parse_temporal_condition_predicates`
    (`ir/semantic.rs`) to parse each clause into `(signal, Option<value>)` and **distribute a
    single shared trailing value** across the coordinated list under a tight guard (≥2
    signals, exactly one distinct value present, fill only value-less signal clauses); the
    value extraction was factored into `temporal_clause_value` (replacing
    `parse_temporal_condition_clause`, its only caller). A bare signal with no own/shared
    value is still dropped (no fabrication). 2 unit tests added: the coordinated "PSEL,
    PENABLE, and PREADY are asserted" now yields one ASSERTED predicate **per signal**; a
    mixed-value list ("PSEL HIGH and PREADY LOW") is **not** cross-filled. **Eval re-run
    end-to-end on real APB confirms the predicted improvement:** `temporal_rule
    P=0.600 R=1.000 F1=0.750 (tp=2→3, fp=3→2, fn=1→0)` — the PBUSER under-capture FN→TP,
    recall now perfect; the remaining 2 FPs are the out-of-scope upstream degenerate-header
    constraints. Book eval numbers refreshed in `quality/extraction-eval.md` (the
    under-capture bullet reframed as caught-then-fixed). Full `scripts/run_ci.sh` GREEN
    (1216→1218; **no `kg-bench` fixture regression** despite the corpus-wide condition
    change). Tree CLOSED.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `TEMPORAL-ANTECEDENT-RECALL.1` | `done` | owned + designed (root-caused against real data) |
| 2 | `TEMPORAL-ANTECEDENT-RECALL.2` | `done` | guarded fix + 2 tests + eval re-run (R 0.667→1.000) + book + close (CI green 1218) |

**Tree CLOSED `2026-06-02`.** The temporal condition parser now distributes a shared trailing
value across a coordinated signal list, recovering dropped antecedent preconditions. The fix
was driven and confirmed by `TEMPORAL-RULE-EVAL` (recall 0.667→1.000 on the real APB seed) —
the measure→catch→fix loop's second completed cycle. Remaining temporal FPs (degenerate
header / `*_WIDTH` subject) are the upstream constraint-tier re-ingest-gated follow-up.

## Decisions

- `2026-06-02`: scope to the **live** temporal-parser recall bug only; the FP cohort is
  upstream constraint-tier (`CONSTRAINT-SUBJECT-PRECISION` class) + re-ingest-gated, tracked
  separately. Keep the fix tightly guarded (one distinct shared value; ≥2 signals; fill only
  value-less clauses) so mixed-value conditions and bare unanchored mentions are untouched.

## Open Questions

- Does any `kg-bench` temporal fixture encode the old under-captured antecedent? If so it is
  updated to the corrected (fuller) antecedent — the fixture must describe correct behavior.

## Blockers

- None. The fix is verifiable end-to-end now (temporal parse runs at SemanticIR build).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-02` | `.1` | root cause in `parse_temporal_condition_predicates` confirmed vs real APB `sigcon_0026`; FP cohort separated as upstream/stale; fix + verification plan (incl. expected eval delta) fixed; docs-only | `passed` |
| `2026-06-02` | `.2` | guarded shared-value distribution implemented (`temporal_clause_value` + parts-based pipeline, ≥2 signals / one distinct value / fill value-less; no fabrication); 2 unit tests (distribution + mixed-value guard); **eval re-run live on real APB `P=0.600 R=1.000 F1=0.750` (tp=3 fp=2 fn=0)** — recall 0.667→1.000; book eval refreshed; full CI GREEN 1216→1218, no kg-bench regression; tree CLOSED | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `TEMPORAL-ANTECEDENT-RECALL.1` | `TEMPORAL-ANTECEDENT-RECALL.1 — own + design the coordinated-condition antecedent recall fix` (`<see git>`) | docs-only |
| `TEMPORAL-ANTECEDENT-RECALL.2` | `TEMPORAL-ANTECEDENT-RECALL.2 — distribute shared assertion-value across coordinated condition signals; close tree` | code + 2 tests; eval R 0.667→1.000; CI green 1218; tree CLOSED |

## Changelog

- `2026-06-02`: Created — the second catch of the measure→catch→fix loop (after
  `CONSTRAINT-SUBJECT-PRECISION`), surfaced by the new `TEMPORAL-RULE-EVAL`. Own + design a
  guarded fix that distributes a single shared trailing assertion-value across a coordinated
  signal list in the temporal condition parser, recovering dropped antecedent conditions.
- `2026-06-02`: **Tree CLOSED.** `.2` done — implemented the guarded distribution in
  `parse_temporal_condition_predicates` (factored value extraction into `temporal_clause_value`;
  parts-based pipeline filling value-less clauses when ≥2 signals + one distinct value; no
  fabrication), +2 unit tests. **Eval re-run on real APB confirmed the fix:** `temporal_rule
  P=0.600 R=1.000 F1=0.750` (recall 0.667→1.000; the PBUSER FN→TP). Book eval numbers
  refreshed. Full CI green (1216→1218), no kg-bench regression. The remaining temporal FPs
  (degenerate header / `*_WIDTH` subject) are the upstream constraint-tier re-ingest-gated
  follow-up.
