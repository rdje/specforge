# SEMANTIC-EMPTY-CATALOG-FILTER: stop disabling the grounding filter on documents with no grounding

## Metadata

- Tree ID: `SEMANTIC-EMPTY-CATALOG-FILTER`
- Status: `active`
- Roadmap lane: `R15e`/`R16` extraction quality — semantic grounding
- Created: `2026-08-11`
- Last updated: `2026-08-11`
- Owner: repo-local workflow

## Goal

`SemanticIr::build` filters EvidenceIR's `signal_constraints` and `conditional_rules` down to records whose
subject/consequent is a **declared signal** — the grounding rule that keeps prose-derived records from becoming
canonical hardware authority. Both filters are wrapped in the same guard:

```rust
// crates/specforge/src/ir/semantic.rs:283  (signal_constraints)
// crates/specforge/src/ir/semantic.rs:293  (conditional_rules)
let conditional_rules = if declared_signal_names.is_empty() {
    evidence_ir.conditional_rules.clone()          // <- no filter at all
} else {
    /* keep only rules whose consequent_signal is declared, or absent */
};
```

So the filter is **strongest on documents that have signal authority and absent on documents that have none**.
A document declaring one signal filters every record against that catalog; a document declaring zero filters
nothing and promotes its entire prose-derived record set into `SemanticIR` — and from there into `IntentIR`.

That is a discontinuity, not a gradient: the guard's own premise ("no catalog to filter against") is exactly the
condition under which an unfiltered record is *least* likely to be grounded. Make the empty-catalog case at least
as strict as the populated one.

## Non-Goals

- Do not delete EvidenceIR records. Evidence is the honest capture layer; this tree governs **promotion into
  `SemanticIR`/`IntentIR`**, not what the evidence stage may observe.
- Do not special-case any document, vendor, or protocol name (ADR 0006). The fix is a grounding rule.
- Do not silently drop system-level rules that legitimately carry no `consequent_signal`. The populated branch
  already keeps those (`.unwrap_or(true)`); any repair must preserve that.
- Do not fold this repair into a corpus refresh slice. It is a shared-extractor change and needs its own
  corpus-wide old-versus-new replay and before/after evals.

## Reproduction and measurement (`2026-08-11`)

Found by `CORPUS-COVERAGE.2.52`, which refreshed `opencapi_25gbps_phy_mechanical_spec_v10`. The refresh retired
that document's two false acronym signals (`IS`, `OD`), which emptied its declared-signal set — and its
`SemanticIR` conditional rules went **0 → 2**, an increase caused solely by removing false signals.

The delta is input-driven, not a code delta. Replaying the semantic stage with the current binary from the
**stale** EvidenceIR reproduces the stale result exactly, which isolates the cause to the guard:

```
$ ./target/release/specforge semantic <stale evidence_ir.json> --dry-run
conditional_rules: 0      interfaces: 2      actors: 7
```

The two rules the refresh promoted are not grounded:

| Rule | `consequent_signal` | `consequent_action` | Source phrase |
| --- | --- | --- | --- |
| `condrule_0002` | `PWR` | `(see source_text)` | truncated prefix of `PWR_GOOD`; antecedent spans two sentences |
| `condrule_0003` | `OPEN` | `must be taken` | prefix of `OPEN_CAPI`; captured from the idiom "Care must be taken that…" |

Neither `PWR` nor `OPEN` is a signal in that document; both are prefixes cut at the `_` of a real identifier or a
technology name.

**Corpus census (read-only, all 78 persisted `SemanticIR` artifacts).** 33 of 78 documents have an empty declared
signal set; 29 of those carry records that therefore rode the unfiltered branch:

- **1,423 conditional rules** and **100 signal constraints** promoted with no grounding check.
- Largest contributors: `usb_3_2_revision_1_0_2017_09` (542 rules), `nvme_base_specification_2_0a_2021_07_26`
  (250), `ihi0088_g_2024_06_amba_dti_protocol_specification` (116), `5_0_2024_08_intel_virtualization_technology…`
  (97), `ccix_base_specification_r1_0a_v1_0_for_evaluation` (89).
- Observed `consequent_signal` values include `NOTICE`, `PDF`, `IMPLEMENTATION`, `UNPREDICTABLE`, `DATASHEET`,
  `MUST`, `RISC`, `PCI`, `IBM`, `FFFF`, `YYY`, `QRDDL`, `HMBKH`, `NWXNF` — document metadata, boilerplate, English
  modals, technology names, and table noise, none of them wires.

Reproducer:

```bash
python3 - <<'PY'
import json,glob
for sp in sorted(glob.glob("generated/semantic_ir/*/semantic_ir.json")):
    s=json.load(open(sp))
    declared={r.get("signal_name") for i in s.get("interfaces",[])
              for r in i.get("signal_records",[]) if r.get("automation_confidence")!="low"}
    cr,sc=s.get("conditional_rules",[]),s.get("signal_constraints",[])
    if not declared and (cr or sc):
        print(sp.split("/")[2], len(cr), len(sc),
              [r.get("consequent_signal") for r in cr if r.get("consequent_signal")][:6])
PY
```

**Blast-radius bound (measured, `2026-08-11`).** None of this reaches an emitted `.isf` today: all 44 emitted
targets come from documents with a populated catalog, and an empty catalog blocks the adapter on
`no signals declared in interface` before any rule is rendered. The defect pollutes canonical `SemanticIR`/
`IntentIR` and anything reading them (validation surfaces, priors, corpus KB, `learn-priors`), not the product
boundary — today. It is one grounded signal away from doing so, because a document that declares a single real
signal *and* carries this prose noise takes the populated branch and filters correctly, while a near-miss
document promotes everything.

## Acceptance Criteria

1. The empty-declared-set branch applies a grounding rule at least as strict as the populated branch.
2. The census above re-runs with a materially smaller unfiltered population, and every surviving record is
   explained (a real system-level rule with no consequent signal, or a genuinely grounded subject).
3. No document with a populated catalog changes at all — proved byte-identical by `--dry-run` old-versus-new.
4. `kg-bench` 156/156; WIRE-BASED-100 constraint+temporal/relation golds hold at 1.000; all emitted `.isf` pass
   FSMGen `--strict --check` with zero new diagnostics; `scripts/run_ci.sh` green.
5. The rule is structural grammar with no chip/vendor/protocol-name list (ADR 0006).

## Task Tree

| Leaf | Status | Scope |
| --- | --- | --- |
| `SEMANTIC-EMPTY-CATALOG-FILTER.0` | `done` | ownership, reproduction, and the corpus census above |
| `SEMANTIC-EMPTY-CATALOG-FILTER.1` | `todo` | decide the rule for the empty-catalog case and pin it with paired unit tests |
| `SEMANTIC-EMPTY-CATALOG-FILTER.2` | `todo` | corpus-wide old-versus-new replay + before/after evals; land behind the full gate |

## Current Frontier

`SEMANTIC-EMPTY-CATALOG-FILTER.1` — choose the rule. The candidates, in the order they should be measured:

- **(a) Symmetric filter.** Drop the special case: an empty catalog filters everything with a
  `consequent_signal`/`subject_signal` and keeps only the no-consequent system-level rules. Simplest and most
  honest; measure how much genuine content it costs on the 29 documents.
- **(b) Grounded-token test.** Keep a record whose subject appears in the document as an identifier-shaped token
  with independent evidence (a table cell, a declaration, an alias-map entry) rather than as a prose prefix. More
  recall, more machinery.
- **(c) Demote, don't drop.** Route unfiltered records to `residual_decisions` so the evidence survives visibly
  without claiming canonical authority — consistent with the project's residual-over-fabrication doctrine.

(c) composes with either (a) or (b) and is likely the honest default; (a) is the measurement baseline that says
what (b) would have to buy.

## Decisions

- `2026-08-11`: found by `CORPUS-COVERAGE.2.52` and deliberately **not** repaired inside that refresh. A data
  refresh may not carry a shared-extractor change: the repair moves 29 documents, so it needs its own leaf, its
  own corpus-wide replay, and its own before/after evals. `.2.51` set the same precedent for the `SEC_SID`
  header-sourcing lever.
- `2026-08-11`: `.2.52` still records its own promoted rules as the honest current-binary result. Suppressing
  them by hand would have hidden the defect inside the very artifact that exposed it.
- `2026-08-11`: the truncation that produces `PWR` from `PWR_GOOD` and `OPEN` from `OPEN_CAPI` is a *separate*
  question from this guard and is not claimed here. This tree governs whether an ungrounded record is promoted,
  not why the token was cut at the underscore.

## Open Questions

- Does the same empty-catalog inversion exist on other promotion paths (temporal rules, polarities, semantic
  hints, register records)? The census only measured the two guards at `semantic.rs:283` and `:293`.
- Should the filter consult `signal_alias_map` before rejecting, so a real signal named only through an alias is
  not lost?
- Is `AutomationConfidence::Low` exclusion from `declared_signal_names` (`semantic.rs:276-281`) itself pushing
  documents into the empty-catalog branch that have low-confidence but real declarations?

## Blockers

- None. This tree blocks no corpus refresh: the defect is pre-existing on 29 documents and reaches no emitted
  target.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-11` | `.0` | replayed the semantic stage from the stale EvidenceIR with the current binary | 0 conditional rules / 2 interfaces / 7 actors — reproduces the stale artifact, isolating the delta to the input, not a code change |
| `2026-08-11` | `.0` | read `crates/specforge/src/ir/semantic.rs:276-308` | both `signal_constraints` and `conditional_rules` bypass their grounding filter when `declared_signal_names.is_empty()` |
| `2026-08-11` | `.0` | census over all 78 persisted `SemanticIR` artifacts | 33 empty-catalog documents; 29 carry 1,423 unfiltered conditional rules and 100 unfiltered signal constraints |
| `2026-08-11` | `.0` | checked every emitted target against the census | 44/44 emitted `.isf` come from populated-catalog documents; no unfiltered record reaches the product boundary today |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `SEMANTIC-EMPTY-CATALOG-FILTER.0` | `CORPUS-COVERAGE.2.52 — refresh the OpenCAPI 25 Gbps PHY mechanical spec and retire its acronym signals` | ownership, reproduction, and census landed with the refresh that found it |

## Changelog

- `2026-08-11`: created from a measured finding in `CORPUS-COVERAGE.2.52`; ownership and census only, no code
  change.
