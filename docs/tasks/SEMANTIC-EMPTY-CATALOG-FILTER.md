# SEMANTIC-EMPTY-CATALOG-FILTER: stop disabling the grounding filter on documents with no grounding

## Metadata

- Tree ID: `SEMANTIC-EMPTY-CATALOG-FILTER`
- Status: `done`
- Roadmap lane: `R15e`/`R16` extraction quality — semantic grounding
- Created: `2026-08-11`
- Last updated: `2026-08-11`
- Owner: repo-local workflow

## Goal

> **Landed `2026-08-11` — the guard quoted below no longer exists at HEAD.** This section is the problem
> statement as it was measured. For current behavior read
> `docs/book/src/pipeline/semanticir.md` or the fact card
> [`semantic-grounding-filter-is-catalog-independent`](../knowledge/semantic-grounding-filter-is-catalog-independent.md).

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
3. No populated-catalog document **loses a promoted record**, proved by `--dry-run` old-versus-new. Revised
   `2026-08-11` by `.1` from "byte-identical" to **"the only permitted change is added residuals"**: uniform
   demotion necessarily surfaces the 1,230 rules and 47 constraints the populated branch already dropped
   silently, so byte-identity was unreachable and would have forced two different answers to one question.
   **Met:** all 41 populated-catalog movers changed only `residual_decisions`.
4. `kg-bench` 156/156; WIRE-BASED-100 constraint+temporal/relation golds hold at 1.000; all emitted `.isf` pass
   FSMGen `--strict --check` with zero new diagnostics; `scripts/run_ci.sh` green.
5. The rule is structural grammar with no chip/vendor/protocol-name list (ADR 0006).

## Task Tree

| Leaf | Status | Scope |
| --- | --- | --- |
| `SEMANTIC-EMPTY-CATALOG-FILTER.0` | `done` | ownership, reproduction, and the corpus census above |
| `SEMANTIC-EMPTY-CATALOG-FILTER.1` | `done` | decide the rule for the empty-catalog case and pin it with paired unit tests |
| `SEMANTIC-EMPTY-CATALOG-FILTER.2` | `done` | corpus-wide old-versus-new replay + before/after evals; land behind the full gate |

## Acceptance Checklist (enforced) — `SEMANTIC-EMPTY-CATALOG-FILTER.2`

- [x] **REPRODUCE / MEASURE** — snapshotted the pre-rebuild corpus before touching it: SHA-256 of all 44 emitted
  `.isf`, all 78 `adapter.json`, all 78 `intent_ir.json`, and the exact 179-artifact set carrying
  `validation_reports`. `CORPUS-CHAIN-CURRENCY.3` recorded not snapshotting the emitted set as a past miss; this
  leaf does not repeat it.
- [x] **ROOT CAUSE (WHY + WHERE)** — `.1`'s change is confined to the semantic stage, so `EvidenceIR` is not an
  input to it; no evidence artifact was written and the nine provider-free evals (which read
  `generated/evidence_ir`) are orthogonal by construction, not by claim.
- [x] **ADDRESSED (verified)** — rebuilt `semantic` → `intent` → `adapt --target isf` for all 78 chains from
  their unchanged persisted `EvidenceIR` in **28s**, zero failures, no re-ingest; then re-validated the same 179
  artifacts, **0 failures**, population unchanged (179 before, 179 after, none lost, none gained).
  `check_chain_currency.sh` is green at **24 evidence / 78 semantic / 78 intent / 78 isf-adapter current**, with
  retention exactly the 24 declared bundles.
- [x] **NO REGRESSION** — all **44 emitted `.isf` byte-identical** (SHA-256 diff empty) and **44/44** pass
  FSMGen `--strict --check --json` with **0 diagnostics**; `kg-bench` **156/156**; the nine provider-free evals
  at baseline with every WIRE-BASED-100 filtered surface at **1.000** (APB/AHB/AXI constraint + document-level
  relation + temporal, SWD `swd_operation` and document-level `serial_frame_field`) and only the known SWD
  `CSYSPWRUPACK` residual missing; `scripts/run_ci.sh` exit **0** with all eight doctrines PASS.
  **Reconciliation, because the raw counts do not match at first glance:** 77 `intent_ir.json` and 72
  `adapter.json` hashes changed against 67 moved SemanticIRs. The excess is exactly the `validation_reports`
  back-annotation a rebuild clears — of the 11 documents with no demotion packet, 10 carried intent validation
  reports and 5 carried adapter ones, giving `67 + 10 = 77` and `67 + 5 = 72` exactly. Re-validation restored
  every one.
- [x] **GENERICITY (ADR 0006)** — no code changed in this leaf; the rebuild applies `.1`'s universal rule
  uniformly across the corpus.
- [x] **LOCKSTEP** — `docs/book/src/pipeline/semanticir.md` gains the grounding principle, a worked
  `semantic_ungrounded_records_not_promoted` example under Residual decisions, and the `BOOK-METHOD-DOC` closing
  subsection; the new fact card `[[semantic-grounding-filter-is-catalog-independent]]` states current behavior
  and `[[semantic-empty-catalog-disables-grounding-filter]]` is marked `superseded` with a reverify that
  confirms the supersession instead of a census that can no longer reproduce; `CHANGES.md`,
  `LIVE_ACHIEVEMENT_STATUS.md`, and `MEMORY.md` updated; the book aggregate authority and fact-card catalog
  projections re-derived.

## Acceptance Checklist (enforced) — `SEMANTIC-EMPTY-CATALOG-FILTER.1`

- [x] **REPRODUCE / MEASURE** — `.0`'s census over all 78 persisted `SemanticIR` artifacts: 33 empty-catalog
  documents, 29 of them carrying **1,423 conditional rules and 100 signal constraints** promoted with no
  grounding check; and `.1`'s pre-design measurement of the populated branch: **41 of 45** populated-catalog
  documents already drop **1,230 conditional rules and 47 signal constraints** silently.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/semantic.rs:283` (`signal_constraints`) and `:293`
  (`conditional_rules`) both read `if declared_signal_names.is_empty() { evidence_ir.<records>.clone() }`, so the
  grounding filter was **disabled on exactly the documents with no signal authority**. Replaying the semantic
  stage from the stale pre-refresh EvidenceIR with the same binary reproduces the stale artifact exactly
  (0 rules / 2 interfaces / 7 actors), isolating the movement to that guard rather than to a code delta.
- [x] **ADDRESSED (verified)** — one predicate now governs every document, and a rejected record is demoted to a
  `semantic_ungrounded_records_not_promoted` residual packet rather than dropped. Read-only `semantic --dry-run`
  replay of all 78 documents against their persisted artifacts: **11 identical · 41 residual-packet-only · 26
  content-moved**, and every one of the 26 has an empty declared catalog. Empty-catalog promotion falls
  `1,423 → 780` conditional rules (the 780 that name no signal are genuine system-level rules and are kept) and
  `100 → 0` signal constraints. Four paired unit tests pin both branches, the system-level exemption, the
  proportionate name bound, and the no-packet-when-clean case.
- [x] **NO REGRESSION** — `cargo test -p specforge --lib` **1810 passed / 0 failed**; `kg-bench` **156/156**;
  `scripts/run_ci.sh` green. Two prose-only fixtures (`builds_semantic_ir_from_handshake_evidence`,
  `builds_intent_ir_from_handshake_semantics`) asserted `residual_decisions.is_empty()`; they declare no signals,
  so they now carry exactly the one demotion packet and their assertions were tightened to state that, naming the
  packet and the demoted subjects. The corpus-wide oracles (`.isf` byte-identity, FSMGen strict, the nine
  provider-free evals, `CHAIN-CURRENCY`) are `.2`'s deliverable.
- [x] **GENERICITY (ADR 0006)** — the predicate is "is this token in the document's own declared-signal
  catalog", derived per document at runtime. No chip, vendor, or protocol name appears in the rule or in the
  packet text.
- [x] **LOCKSTEP** — the tree records the rule, the measurement, and the answered open question; the book
  chapter, fact card, and live docs land with `.2`, which closes the tree.

## Current Frontier

**Empty — the tree is closed (`2026-08-11`).** All three leaves are `done` and every acceptance criterion is
met. The authorised rule, kept for readers of any later delta: **the director authorised (c) composed with (a)
on `2026-08-11`** — symmetric filter, rejected records demoted rather than dropped.

The one thing this tree deliberately did **not** claim is now the open question below: the documents whose
signal catalogs were never captured at all. That belongs to the extraction-breadth lane, and the demotion packet
is what makes it findable per document.

- **(a) Symmetric filter.** Delete the `declared_signal_names.is_empty()` special case at `semantic.rs:283`
  and `:293`. One predicate governs both branches: keep a record when its `consequent_signal`/`subject_signal`
  is a declared signal, or when it has none (a genuine system-level rule). With an empty catalog the predicate
  is simply never satisfied by a named subject, which is the intended outcome rather than a special case.
- **(c) Demote, don't drop.** A record the predicate rejects becomes a `residual_decision` rather than
  vanishing, so the evidence stays visible without claiming canonical authority — the project's
  residual-over-fabrication doctrine.

**A measurement taken `2026-08-11`, after the rule was chosen, constrains how (c) is applied — read this before
writing code.** The populated branch is *not* a no-op today: of the 45 populated-catalog documents, **41 already
drop records silently**, totalling **1,230 conditional rules and 47 signal constraints** (AMD IOMMU 154 → 29,
AXI `ihi0022_h_c` 175 → 147, CCIX rev 2.0 76 → 7). So applying (c) uniformly does *not* leave populated
documents untouched — it gives 41 of them new `residual_decisions` — which contradicts Acceptance Criterion 3 as
originally written. Two ways forward:

- **Uniform demotion (recommended).** Apply (a) and (c) to both branches and revise Criterion 3 from
  "byte-identical" to "no populated-catalog document loses a promoted record; the only permitted change is added
  residuals". An asymmetric rule — demote on the empty branch, drop silently on the populated one — would
  reintroduce exactly the discontinuity this tree exists to remove, and the corpus needs a full downstream
  rebuild either way because any semantic-stage change invalidates every persisted `SemanticIR` under
  `CHAIN-CURRENCY`.
- **Scoped demotion.** Apply (c) only where the catalog is empty, preserving byte-identity on all 45 populated
  documents. Smaller blast radius, but it keeps two different answers to the same question.

**Resolved `2026-08-11` by `.1`: uniform demotion.** The replay showed no populated-document regression — all 41
populated-catalog movers changed **only** `residual_decisions` — so the scoped variant bought nothing and would
have kept two answers to the same question. Acceptance Criterion 3 is revised accordingly (see below).

Reproducer for the constraint above:

```bash
python3 - <<'PY'
import json,glob,os
for sp in sorted(glob.glob("generated/semantic_ir/*/semantic_ir.json")):
    key=sp.split(os.sep)[2]; ep=f"generated/evidence_ir/{key}/evidence_ir.json"
    if not os.path.exists(ep): continue
    s=json.load(open(sp)); e=json.load(open(ep))
    declared={r.get("signal_name") for i in s.get("interfaces",[])
              for r in i.get("signal_records",[]) if r.get("automation_confidence")!="low"}
    if not declared: continue
    dcr=len(e.get("conditional_rules",[]))-len(s.get("conditional_rules",[]))
    dsc=len(e.get("signal_constraints",[]))-len(s.get("signal_constraints",[]))
    if dcr or dsc: print(key, dcr, dsc)
PY
```

### Implementation order for the next session

1. Mark `.1` `in_progress` and paste the `TOOLBOX.md` acceptance checklist into this tree — a Rust change
   cannot commit without it (`scripts/check_task_acceptance.sh`).
2. Change `crates/specforge/src/ir/semantic.rs:276-308`; add paired unit tests (an empty catalog rejects a named
   subject and emits a residual; a populated catalog keeps its declared subjects).
3. Corpus-wide old-versus-new `--dry-run` replay; classify every moved document.
4. Rebuild all 78 downstream chains from their unchanged EvidenceIR — no re-ingest is needed, only the evidence
   stage reads a normalized bundle (`CORPUS-CHAIN-CURRENCY.3` did exactly this in 2m32s) — then re-validate.
5. `kg-bench` 156/156, the nine provider-free evals, 44/44 FSMGen strict, `scripts/run_ci.sh`.
6. Update this tree, the fact card `[[semantic-empty-catalog-disables-grounding-filter]]`, the book, and the
   live docs; commit per `COMMIT.md`.

## Decisions

- `2026-08-11` (`.1`): **the demotion is one proportionate summary packet per document**, not one packet per
  rejected record. `semantic_ungrounded_records_not_promoted` states both counts, the declared-catalog size, and
  a sorted, capped sample of the undeclared names (`UNGROUNDED_PROMOTION_SAMPLE_LIMIT = 12`, the rest elided as
  "and N more"). One document rejects 216 rules; a per-record dump would ride into every `SemanticIR`, `IntentIR`,
  and `adapter.json` that carries residual decisions. The bound is the same shape `isf_storage_reset_not_lowered`
  and `isf_register_fields_not_lowered` already use. Sorting keeps the packet text deterministic.
- `2026-08-11` (`.1`): **the packet is emitted only when a record is actually rejected**, so a fully grounded
  document gains nothing and the demotion surface never becomes ambient noise. 11 of 78 documents carry no packet.
- `2026-08-11`: **the director authorised (c) composed with (a)** — symmetric filter, with rejected records
  demoted to `residual_decisions` rather than dropped. The choice is settled; `.1` implements it. Recorded here
  because a decision that lives only in a conversation is not saved.
- `2026-08-11`: measuring the populated branch *after* the decision changed what "orthogonal" can mean. 41 of 45
  populated-catalog documents already drop 1,230 conditional rules and 47 signal constraints silently, so
  uniform demotion necessarily adds residuals there. Acceptance Criterion 3 is therefore the open sub-question,
  not the rule itself. The recommendation is uniform demotion plus a revised criterion; the reasoning is in
  Current Frontier.
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
- ~~Is `AutomationConfidence::Low` exclusion from `declared_signal_names` (`semantic.rs:276-281`) itself pushing
  documents into the empty-catalog branch that have low-confidence but real declarations?~~ **Answered
  `2026-08-11` (`.1`): no — not on this corpus.** All **33** empty-catalog documents carry **zero** interface
  signal records of any confidence; **none** has a signal catalog that is merely low-confidence. The Low
  exclusion is therefore not what empties a catalog here; nothing was captured at all. (The shape does exist in
  synthetic prose: the `builds_semantic_ir_from_handshake_evidence` fixture holds Low-confidence VALID/READY
  records and now demotes their constraints.) Reproducer: for each `generated/semantic_ir/*/semantic_ir.json`,
  partition documents with an empty non-`low` declared set by whether `interfaces[].signal_records` is empty.
- **New (`2026-08-11`, `.1`): what should own the missing signal catalogs the demotion now makes visible?** The
  rejected subjects on the largest movers are dominated by noise (`DATASHEET`, `MUST`, `PCI`, `IMPLEMENTATION`,
  `PDF`, `AMBA`, `FPGA`) — but they also contain real protocol tokens truncated at an underscore or a suffix:
  Wishbone `CLK`/`CYC`/`STB`/`RST`/`STALL` (the document spells them `CLK_I`, `CYC_O`, `STB_O`, `RST_I`), AMBA
  DTI `TDATA`/`TKEEP`/`TLAST`, USB `ACK`/`ERDY`/`NRDY`. Those documents have **no captured signal catalog at
  all**, so this is a capture gap in the signal-declaration extractors, not a grounding-rule defect. This tree
  deliberately does not claim it (see the `PWR`/`OPEN` truncation decision below); it belongs to the extraction
  breadth lane. The demotion packet is what makes it findable per document.

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
| `2026-08-11` | `.1` | measured the populated branch's silent drop volume before designing the demotion | 41 of 45 populated-catalog documents already drop records — 1,230 conditional rules and 47 signal constraints — so uniform demotion cannot leave them byte-identical; read-only, no artifact mutated |
| `2026-08-11` | `.1` | `cargo test -p specforge --lib` | 1810 passed / 0 failed / 5 ignored, including the four new paired tests |
| `2026-08-11` | `.1` | `specforge kg-bench` | `fixtures_passed: 156`, `fixtures_failed: 0` |
| `2026-08-11` | `.1` | read-only `semantic --dry-run` replay of all 78 documents against their persisted artifacts | 11 identical · 41 changed **only** in `residual_decisions` · 26 content-moved, all 26 with an empty declared catalog; no populated-catalog document lost a promoted record |
| `2026-08-11` | `.1` | partitioned the 33 empty-catalog documents by whether any interface signal record exists | 33 have **none at all**, 0 are low-confidence-only — answers the Low-exclusion open question for this corpus |
| `2026-08-11` | `.2` | rebuilt `semantic` → `intent` → `adapt --target isf` for all 78 chains from unchanged persisted EvidenceIR | 28s, zero failures, no re-ingest |
| `2026-08-11` | `.2` | re-validated the exact 179-artifact population that carried `validation_reports` | 0 failures; 179 before, 179 after — none lost, none gained |
| `2026-08-11` | `.2` | SHA-256 diff of all 44 emitted `.isf` against the pre-rebuild snapshot | **byte-identical**; the 77 intent / 72 adapter hash changes reconcile exactly as 67 moved + 10 / 5 cleared `validation_reports` |
| `2026-08-11` | `.2` | `subs/fsmgen/bin/fsmgen --strict --check --json` over all 44 emitted `.isf` | **44 pass / 0 fail, 0 diagnostics** |
| `2026-08-11` | `.2` | nine provider-free `eval-extraction` datasets | at baseline; every WIRE-BASED-100 filtered surface 1.000, only the known SWD `CSYSPWRUPACK` residual missing |
| `2026-08-11` | `.2` | `bash scripts/check_chain_currency.sh` | green: 24 evidence / 78 semantic / 78 intent / 78 isf-adapter current, retention exactly the 24 declared bundles |
| `2026-08-11` | `.2` | `bash scripts/run_ci.sh` | exit 0; all eight doctrines PASS; lib 1810 passed / 0 failed |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `SEMANTIC-EMPTY-CATALOG-FILTER.0` | `CORPUS-COVERAGE.2.52 — refresh the OpenCAPI 25 Gbps PHY mechanical spec and retire its acronym signals` | ownership, reproduction, and census landed with the refresh that found it |
| `SEMANTIC-EMPTY-CATALOG-FILTER.1` | `SEMANTIC-EMPTY-CATALOG-FILTER.1 — one grounding predicate for every document, rejected records demoted` | the symmetric filter, the demotion packet, and the paired tests |
| `SEMANTIC-EMPTY-CATALOG-FILTER.2` | `SEMANTIC-EMPTY-CATALOG-FILTER.2 — rebuild the corpus to the new rule and close the tree` | corpus replay/rebuild evidence, the oracles, the book method-doc, and the fact cards |

## Changelog

- `2026-08-11`: created from a measured finding in `CORPUS-COVERAGE.2.52`; ownership and census only, no code
  change.
- `2026-08-11`: `.1` landed the authorised rule — the `is_empty()` special case is gone, one predicate governs
  every document, and rejected records are demoted to a proportionate residual packet. Acceptance Criterion 3
  revised to "added residuals only" and met; the Low-confidence open question answered and struck; a new open
  question recorded for the missing signal catalogs the demotion makes visible. Frontier advanced to `.2`.
- `2026-08-11`: `.2` rebuilt all 78 downstream chains and closed the tree. The product boundary did not move —
  44/44 emitted `.isf` byte-identical and FSMGen-strict clean at zero diagnostics — and the validated-artifact
  population is exactly restored at 179. Book, fact cards, `CHANGES.md`, and `LIVE_ACHIEVEMENT_STATUS.md` are in
  lockstep. **Tree status `done`.**
