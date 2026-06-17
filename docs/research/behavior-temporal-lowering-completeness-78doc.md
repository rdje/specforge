# Behavior/temporal lowering completeness — re-assessment on the broader 78-doc corpus

**Tree:** `KG-ISF-COMPLETENESS.4` (bar #5/#6 re-assessment) · **Date:** `2026-06-17` ·
**Mode:** read-only measurement, docs-only (no code change).

## Why this measurement

The owner north star (`[[project_kg_isf_completeness]]`) defines a 6-point "ISF-complete IntentIR"
bar. **Bar #5** says: *every behavior/temporal rule is carried or recorded as an explicit residual
(no silent drop)*. **Bar #6** (round-trip) says: *every IntentIR surface element appears in the
`.isf` or an explicit residual*.

`KG-ISF-COMPLETENESS.2` measured the ISF-lowering fidelity gauge **over 36 IntentIR docs**
(`[[isf-lowering-fidelity-gauge]]`). Since then `CORPUS-COVERAGE.0` built the corpus out **36 → 78
docs**, adding mostly register / coherency / command / profile docs (NVMe, AMD-IOMMU, VT-d, CCIX×4,
RISC-V-IOMMU, CHI-C2C, DTI, SMMU, ATP, …). Those are exactly the doc classes most likely to stress
the lowering (register-field obligations, message-field obligations, VLM-only signal references). The
resume-pointer standing candidate (b) was: *re-assess bar #2 / bar #5 on the now-broader corpus*. This
report is the bar-#5/#6 half.

## Method (faithful replication of the emitter filters)

Read-only Python over `generated/intent_ir/*/intent_ir.json` (78 docs), replicating the exact
`IsfIr::from_intent_ir` lowering filters (`crates/specforge/src/ir/isf_ir.rs`):

- `signal_names` = the union of `interfaces[].signal_records[].signal_name`, **minus clock/reset**,
  deduped (`isf_ir.rs:673-719`).
- `conditional_rules` (`:1055`) lower iff `consequent_signal` is `Some` **and** in `signal_names`;
  else `continue` with **no residual**.
- `signal_constraints` (`:1075`) lower iff `subject_signal` ∈ `signal_names`; else `continue`,
  **no residual**.
- `temporal_invariants` (`:1092`) lower iff `subject_signal` non-empty **and** ∈ `signal_names`; else
  `continue`, **no residual**.
- `temporal_rules` / `actor_contracts` lower to a `(contract)`, a `(rule temporal_…)`, **or** a
  `temporal_residual` — never a silent drop (`[[isf-temporal-lowering-no-silent-drop]]`).

(The clock/reset exclusion is a negligible approximation here — rule subjects are essentially never the
clock/reset signal, so the undeclared-subject classification is unaffected.)

## Result — corpus-wide `(rule)`-surface lowering breakdown (78 docs)

| surface | total | lowered | empty / no-consequent | **undeclared-named** |
|---|---|---|---|---|
| `conditional_rules`  | 2237  | 516 | 1670 (no-consequent) | **51** |
| `signal_constraints` | 369   | 287 | 0 | **82** |
| `temporal_invariants`| 29343 | 286 | 29030 (empty-subject) | **27** |
| `temporal_rules` (residualizing path) | 320 | — carried or `temporal_residual` | — | — |
| `actor_contracts` (residualizing path) | 211 | — carried or `temporal_residual` | — | — |

- **Empty-subject / no-consequent drops (30 700):** a `temporal_invariant` with an empty subject (a
  table-of-contents heading or section title misclassified as an invariant) or a `conditional_rule`
  with no consequent signal (vague "must"/"shall" boilerplate). These are **not rules** — there is no
  subject to assert. Leaving them silent is the **"absence is not an event"** honesty rule; mass-
  residualizing them is exactly the dishonest noise `.2`/`.2b` rejected. **Correctly silent.**
- **Undeclared-named-subject drops (160):** a rule with a real named subject that is **not** in the
  doc's declared `signal_records` inventory. This is the only subset that could be a genuine bar-#5
  silent loss — so it was characterised item-by-item.

## The 160 undeclared-named-subject drops — what they actually are

Cross-checked each undeclared subject against the doc's other IntentIR surfaces
(`register_records` fields, `message_field_records`, `actor_ports`, `actor_signal_relations`):

- **5** are already captured on the `register_records` surface (honest — homed elsewhere).
- **155** are on no other surface. Item inspection shows they are **field content + extraction noise,
  not wire-signal intent**:

| cluster | docs | examples | what it is |
|---|---|---|---|
| register/message FIELD mnemonics | NVMe (21), CCIX (16), RISC-V-IOMMU (8), CHI-C2C (4) | `MTFA`, `HMDLAL`, `GDHM`, `GENCTR`, `SAMH`, `ESMD`, `ESME`, `DC.tc.SXL`, `process_id[19:17]` | obligations about **register/struct fields** read out of `\| bits \| NAME description \|` tables. They belong to the register / message-field surfaces; the `.isf` adapter does not lower fields **by design** (bar #6 honest absence). |
| DTI message-field obligations | DTI `ihi0088` (34, 14 distinct) | `DTI_TBU_TRANS_REQ.MMUV`, `DTI_TBU_TRANS_REQ.SEC_SID`, `ATTR_OVR.MTCFG`, `{NSE,NS}`, `this bit`, `<unspecified field>`, `least significant byte of TKEEP` | **message-field** obligations that leaked into `signal_constraints` because DTI carries **no `message_field_records` at all** (its message-field tables were not recognized — see spin-out). |
| prose acronyms / literals (noise) | VT-d, SMMU, CHI-C2C, OpenCAPI, readme | `DMA`, `TLB`, `PCI`, `NUMA`, `IEC`, `IMPLEMENTATION`, `IBM`, `ONLY`, `Reserved`, `FFFF`, `FFFFFFFF_FFFFFFFF` | mis-captured prose subjects / hex VALUES / reserved markers — **not obligations**. Residualizing = dishonest noise. |
| garbled VLM fragments | ATP `ihi0082` (56, 5 distinct) | `ARVALID`, `RVALID`, `RREADY`, `RLAST`, `RACK` — sourced from `"VLM timing diagram observation: RREADY is RBR."` | real AXI **names** but fragmentary/garbled VLM observations on a profile doc that declares no signal table. The names are real; the "constraints" are VLM noise. |

**0 undeclared/silent drops on all four wire docs (APB/AHB/AXI/SWD)** — re-confirmed at 78-doc scale.

## Conclusion — bar #5/#6 holds at 2× corpus scale

1. `temporal_rules`/`actor_contracts` residualize → bar #5 is satisfied for the temporal-contract
   surface, corpus-wide (`[[isf-temporal-lowering-no-silent-drop]]`).
2. The 30 700 empty-subject / no-consequent drops are correctly silent ("absence is not an event").
3. The 160 undeclared-named-subject drops, on inspection, are **field content (homed on the
   register/message surfaces by design) + prose noise + garbled VLM fragments — not wire-intent loss.**
   The adapter's silent `continue`-skip of an undeclared-subject rule is the **correct** behavior:
   - a register/message-field obligation should be **routed upstream** to the field surfaces, not
     residualized at the adapter (residualizing would mask the upstream issue);
   - a prose/hex/VLM-fragment subject should be **filtered upstream** (subject precision), not
     residualized (residualizing = noise).
4. **No buildable lowering-residual lever.** This **confirms and extends `.2`/`.2b` to the doubled
   corpus** — the register-heavy expansion did **not** introduce a genuine silent-drop of grounded wire
   intent. The note in `[[isf-lowering-fidelity-gauge]]` (its `reverify` numbers were the 36-doc
   snapshot; the 78-doc snapshot is 29 343 / 2237 / 369 totals) is updated accordingly.

## Spun-out grounded observations (for the right trees — NOT this leaf)

These are genuine, measured **upstream** gaps surfaced by the measurement; each needs its own
measurement-first ownership before any code:

- **DTI message-field recognition gap** (`EXTRACTION-QUALITY-GAUGE.FIELD` / `PDF-VARIANT-DIGESTION`):
  DTI `ihi0088` is a message protocol (DTI_TBU_TRANS_REQ etc.) but carries **no `message_field_records`**
  — so 30 field obligations leaked into `signal_constraints` with dotted/undeclared subjects
  (`EXTRACTION-QUALITY-GAUGE.FIELD.4` was built to route exactly these, but only fires for
  *catalog-declared* message fields). **Root cause located (`2026-06-17` scoping over the DTI
  `source_ir.json`):** DTI defines its per-message field bit-layout in **prose `list_item`s of the form
  `"<FieldName>, bit [N]"` / `"<FieldName> bits [hi:lo]"`** (e.g. `MMUV, bit [69]`, `MPAM PARTID[8]`)
  under each message's section, plus a couple of `Field bits | Field name` tables (Table 3-7 `ATTR_OVR
  subfields`) that lack the container-anchoring caption. The `build_message_field_records` recognizer keys
  off field-titled **tables** whose caption anchors fields to a container noun (channel/packet/message/…),
  so it correctly sees **nothing** for DTI's prose-and-bare-table format. **The proper fix is a NEW
  prose-message-field extractor** (parse `"<Field>, bit [N]"` list-items scoped to a message section into
  `message_field_records`, then the existing FIELD.4 routing moves the obligations to
  `message_field_constraints`) — a substantial, ADR-0006-design-heavy slice needing its own
  measurement-first ownership (generality across the corpus must be measured: does the `"<Name>, bit [N]"`
  prose pattern appear in other docs? what precision gate keeps it from minting fields out of arbitrary
  prose?) + full `run_ci`/`kg-bench`/WIRE-BASED-100 re-verification. **This is the most actionable next
  gap, and a fresh-session-appropriate build (design-heavy, touches the extraction machinery).**
  **Generality MEASURED (`2026-06-17`, read-only over all 79 `source_ir.json`) → GENERAL, GO:** the
  `"<Name>, bit [N]"` / `"<Name> bits [hi:lo]"` prose field-definition form appears in **13 / 79 docs**
  (1499 matches), concentrated in the register/coherency architecture specs — SMMU `ihi0070` 421, ARM Debug
  v6 `ihi0074` 365, GIC `ihi0069` 328, **DTI `ihi0088` 173**, CoreSight `ihi0029` 170, ACC `ihi0076` 30,
  then a long noise tail (1–5 matches). So the prose-field extractor is a **corpus-wide lever (~6
  high-value docs)**, not DTI-only. **Precision is the crux:** the same scan also matches descriptive prose
  (`IMPLEMENTATION DEFINED, bits [31:0]`; `When reporting a virtual SEI, bits[24:0] …`), so the build MUST
  gate structurally — field name scoped to a register/message section, bit ranges that tile, never firing
  on free descriptive prose (the `.1a` agent-identity-gate philosophy, applied to field defs; ADR-0006, no
  name list). Recommended home: a new `PDF-VARIANT-DIGESTION` leaf (sibling of the `.10a`–`.10e`
  table-format field recognizers), measurement-first on the precision gate.
- **Signal-inventory prose noise on register-heavy docs** (signal-precision): DTI's declared
  `signal_records` include prose acronyms minted as signals (`AMBA`, `ARM`, `APCI`) — the same class of
  precision issue the agent-identity gate (`.1a`) addressed for actors, here on the signal inventory.
- **ATP `ihi0082` VLM-fragment quality:** its only "wire intent" is garbled VLM timing-diagram fragments
  — a VLM-extraction-quality datum for profile docs, not a lowering issue.

## Reproduce

```bash
# 78-doc (rule)-surface lowering breakdown + undeclared-named-subject decomposition
python3 - <<'PY'
import json, glob, os
docs=sorted(glob.glob("generated/intent_ir/*/intent_ir.json"))
C={}
def add(k): C[k]=C.get(k,0)+1
for p in docs:
    d=json.load(open(p)); sig=set()
    for i in d.get("interfaces") or []:
        for s in i.get("signal_records") or []: sig.add(s["signal_name"])
    for cr in d.get("conditional_rules") or []:
        cs=cr.get("consequent_signal")
        add("cr_total"); add("cr_lower" if cs in sig else ("cr_noc" if not cs else "cr_undecl"))
    for sc in d.get("signal_constraints") or []:
        s=sc.get("subject_signal") or ""
        add("sc_total"); add("sc_lower" if s in sig else ("sc_empty" if not s else "sc_undecl"))
    for inv in d.get("temporal_invariants") or []:
        s=inv.get("subject_signal") or ""
        add("ti_total"); add("ti_lower" if (s and s in sig) else ("ti_empty" if not s else "ti_undecl"))
print(C)
PY
```
