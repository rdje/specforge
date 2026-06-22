# Per-category ISF-lowering completeness gauge (DOC-INTENT-TAXONOMY.2)

- Tree leaf: `DOC-INTENT-TAXONOMY.2`
- Date: `2026-06-22`
- Type: measurement, **read-only** (profiled the persisted `generated/{evidence_ir,intent_ir}/*` plus the
  `generated/adapters/isf/*/adapter.json` lowering artifacts; no `validate`, no `adapt` write → zero canonical
  mutation). For the two docs whose `adapter.json` was never materialized (`1_0_risc_v_debug_specification`,
  `den0068_…coresight_base_system_architecture`) the read-only `adapt … --dry-run` JSON was captured (it writes
  nothing — verified the adapter dirs stayed absent).
- Reproduce: `python3 scripts/measure_isf_completeness.py` (tracked; pass `--extra-adapter KEY=PATH` for the two
  dry-run docs). The category labels in that script are a one-off **measurement label** set reconstructing the
  `DOC-INTENT-TAXONOMY.1` ground truth (not shipped — the runtime recognizer `.3` must be structural, ADR 0006);
  the distribution checksums against `.1` exactly: **36 / 7 / 15 / 2 / 4 / 14**.
- Question: *for each purpose category (`DOC-INTENT-TAXONOMY.0`), what fraction of the document's typed intent
  actually reaches the emitted `.isf`, and — for what does not — is it an HONEST RESIDUAL (the document carries
  nothing there, or the surface is non-synthesizable config/metadata) or a TRUE GAP (the intent exists but no
  `Evidence→Intent` carrier and/or no ISF construct lowers it)?* This turns the `.0` maturity column
  (`MATURE / PARTIAL / THIN / non-target`) from honest-estimate into objectively-measured.

## Method — the surface-lowering ledger

`.isf` is the synthesis target for every category (the north star). The honest way to ask "how much intent
reaches `.isf`" is **per typed surface**, because the surfaces are heterogeneous and a single blended percentage
would be gameable (`[[feedback_scoring_rigor]]`). For each document we read the intent-bearing surfaces (the
denominator) and the realized `.isf` counts (the numerator), and classify each surface:

| Intent surface | Lives in | ISF construct | Lowering verdict basis |
|---|---|---|---|
| Signals | IntentIR `interfaces` / `actor_ports` (+ relation/constraint refs) | `(input/output …)` | mature path — see note ‡ |
| Registers | IntentIR `register_records` | `(storage (var … (width N)))` | **lowered 1:1** (width-only) |
| Register **bit-fields** | IntentIR `register_records[].fields` | *(none — registers lower as an opaque width-only var)* | **TRUE GAP** |
| Enums / symbol defs | IntentIR `symbol_definitions` (+ field enums) | `(enums …)` | mostly lowered |
| Constraints + temporal + conditional | IntentIR `signal_constraints`/`temporal_rules`/`conditional_rules` | `(rule …)` / asserts | partial (ratio meaningful) |
| Transactions | IntentIR `transactions` | `(transaction … steps)` | only `steps` lower (`.2b`) |
| **Message-field structures** | **EvidenceIR `message_field_records`** | *(none — no IntentIR carrier)* | **TRUE GAP** |
| Signal-presence matrices | EvidenceIR `signal_presence_records` | *(none)* | HONEST RESIDUAL (config/presence metadata) |

‡ **Signals are deliberately not scored as a present/lowered ratio.** The `.isf` signal set is built from a
*different basis* than the IntentIR `interfaces` array — it is the union of interface signals, actor-port graph
nodes, and signals referenced by relations/constraints — so `.isf` can carry **more** signals than the interface
inventory (Cortex-A76 TRM: 7 interface signals → 189 `.isf` signals) **or fewer** (AXI `ihi0022_h_c`: a 1026-row
interface inventory → 242 `.isf` signals, the rest unconnected/duplicated groupings). The signal *lowering path*
is mature (it is the heart of the WIRE-BASED-100 = 1.000 gold); the only signal-side concern the data surfaces is
**over-extraction noise** on non-target guides (see §5), which is a precision matter for the `.3` recognizer, not
an ISF-completeness gap.

## Result 1 — the two dominant, universal TRUE GAPS

Two surfaces are recovered in volume into the pipeline yet reach `.isf` **zero** times, across every buildable
category. They dwarf every other residual and are the prioritized program.

### Gap A — register **bit-fields** are dropped in lowering (registers lower as opaque width-only vars)

`3,449` registers DO reach `.isf` (lowered 1:1 to `(storage (var … (width N)))`), but their **`12,638`
constituent bit-fields reach `.isf` zero times** — the register *programming model* (which bits mean what, the
per-field access/reset, the field names) is entirely absent from the synthesized `.isf`.

| Cat | Register-fields present (IntentIR) | Lowered to `.isf` | Docs affected |
|---|---:|---:|---:|
| 1 wire-protocol | 2,121 | 0 | 13 |
| 2 register-IP | 1,030 | 0 | 5 |
| 3 platform-system-IP | **9,308** | 0 | 13 |
| 4 CPU-ISA | 179 | 0 | 1 |
| **Total** | **12,638** | **0** | **32** |

Live confirmation — the RISC-V IOMMU `.isf` storage block is `(var register_table_0033 (width 26))` … with no
field substructure, though IntentIR carried 147 fields across its 33 registers. This is the single largest
measurable intent-loss in the corpus, and it lands hardest on the platform/system-IP TRMs (CoreSight SoC-600
`100806_0701`: 833 registers / **2,978 fields** → storage 833, fields 0).

### Gap B — message-field **structures** never cross EvidenceIR → IntentIR

`1,220` message/structure fields (flit/packet/descriptor/queue/context layouts) are recovered into EvidenceIR by
the `.10b`–`.10g` strategies, but **IntentIR has no `message_field_records` key** (`has_msgfld_key=false` for
every doc) — so they are not carried, and not lowered.

| Cat | Message-fields present (EvidenceIR) | Carried to IntentIR | Lowered to `.isf` | Docs |
|---|---:|---:|---:|---:|
| 1 wire-protocol | 785 | 0 | 0 | 8 |
| 2 register-IP | 433 | 0 | 0 | 2 |
| 3 platform-system-IP | 2 | 0 | 0 | 1 |
| **Total** | **1,220** | **0** | **0** | **11** |

This is simultaneously the cat-2 "structure recall" frontier (NVMe 216, AMD-IOMMU 217) **and** the cat-1
message-heavy-protocol gap (CHI 106, DTI 159, CHI-C2C 210, CCIX ×4 ≈ 309) flagged by the `.1` register-heavy
trap — the flit/message fields are the *real* intent of those coherent protocols, and none of it synthesizes.

Both gaps point at the **same missing ISF abstraction** the owner anticipated (`2026-06-22`): structured
storage with named bit-fields, and packet/structure layouts — the family of "memory banks, single/dual-port
memory modules" FSMGen is adding. They are the headline `.4+` FSMGen ISF-abstraction feedback candidates
(filed only after empirical submodule verification — `[[feedback_verify_fsmgen_before_fr]]`,
`[[feedback_isf_no_hacks]]`).

## Result 2 — per-category scorecard (objectively measured)

For each surface: `units present → units lowered` (summed over the category), and the honest verdict. Signals
omitted from the ratio per note ‡; cat 5/6 are non-targets.

### Cat 1 — wire-level bus / interconnect protocol (36 docs) — **MATURE (confirmed)**

| Surface | present → lowered | verdict |
|---|---|---|
| registers → storage | 729 → 729 | **lowered 1:1** |
| enums | 269 → 260 | mostly lowered (8 docs partial) |
| constraints+temporal+conditional → rules | 1,736 → 716 (~41%) | partial |
| transactions → steps | 95 → 34 | thin (15 docs 0-lowered: recognition-only) |
| register-fields | 2,121 → 0 | TRUE GAP (Gap A) |
| message-field structures | 785 → 0 | TRUE GAP (Gap B) |
| signal-presence (config) | 315 → 0 | honest residual |

Wire intent (signals, relations→ports, constraints, temporal, enums) lowers richly — this is the gold-locked
mature category (WIRE-BASED-100 = 1.000). Its residuals are (i) the message-field flit surface for the 8
register/message-heavy protocols (Gap B), (ii) transaction *bodies* — only the enum-grounded `steps` lower
(`.2b`), the recognised signal-set/channel/phase membership is structured-but-unlowered by design, and (iii)
the conditional-rule tail (see Result 3).

### Cat 2 — programmable register / memory-mapped IP (7 docs) — **PARTIAL (measured)**

| Surface | present → lowered | verdict |
|---|---|---|
| registers → storage | 204 → 204 | **lowered 1:1** |
| enums | 65 → 57 | mostly lowered |
| constraints+temporal+conditional → rules | 542 → 244 (~45%) | partial |
| register-fields | 1,030 → 0 | TRUE GAP (Gap A) |
| message-field structures | 433 → 0 | TRUE GAP (Gap B) |

The register *map* lowers (storage), but the programming model that IS a register IP's intent — the bit-fields
(1,030) and the in-memory structures (433, NVMe + AMD-IOMMU) — does not. This is the precise, measured form of
the `CORPUS-COVERAGE.2` #21 IOMMU "Lever-D" finding. (`jesd235` HBM is `blocked` — an under-extracted thin doc,
a recall matter, not a lowering one.)

### Cat 3 — platform / system-IP topology & integration (15 docs) — **PARTIAL (measured)**

| Surface | present → lowered | verdict |
|---|---|---|
| registers → storage | 2,472 → 2,472 | **lowered 1:1** |
| enums | 202 → 193 | mostly lowered |
| constraints+temporal+conditional → rules | 397 → 42 (~11%) | weak |
| register-fields | 9,308 → 0 | TRUE GAP (Gap A — largest absolute) |
| transactions → steps | 46 → 22 | partial |

Platform TRMs are register-storage-rich and lower their register *count* fully, but they carry the **largest
absolute field loss** (9,308) and their **topology / connectivity / clock-reset infrastructure stays
hint-level** — there is no topology→ISF construct, so the integration intent that defines category 3 does not
synthesize. Rule lowering is weakest here (~11%): platform prose is conditional-heavy and most does not reduce
to a clean signal obligation.

### Cat 4 — CPU ISA / privileged architecture (2 docs) — **THIN (confirmed)**

| Surface | present → lowered | verdict |
|---|---|---|
| registers → storage | 44 → 44 | lowered 1:1 (RISC-V Debug DTM/DMI registers) |
| enums | 25 → 23 | mostly lowered |
| constraints+temporal+conditional → rules | 101 → 22 (~22%) | weak |
| register-fields | 179 → 0 | TRUE GAP (Gap A) |

The two ISA docs lower only their register-shaped surface; the CSR-field semantics, the instruction set,
privilege modes, exceptions, and the memory-ordering model have **no ISF construct at all**. RISC-V Debug had
no materialized adapter until this measurement (it renders fine on dry-run: storage 44 / enum 18 / rule 0 / txn
2). This is the least-developed ISF story and the clearest candidate for a dedicated lowering decision in `.4+`.

### Cat 5 — physical / electrical / link layer (4 docs) — **HONEST NON-TARGET (confirmed)**

`.isf` correctly near-empty (signals 2–9, no rules/registers/transactions). PHY signaling/mechanical content is
not behavioral wire intent, so a thin `.isf` is the correct outcome, not a gap.

### Cat 6 — methodology / language / EDA standard / guide (14 docs) — **NON-TARGET (with a precision caveat)**

9 of 14 are correctly near-empty. **5 over-extract** — they synthesize spurious wire intent despite being
guides: `cortex_a76_software_optimization_guide` (`.isf` 537 signals!), `readme` (152 sig / 8 txn),
`arm_smmu_software_guide` (144), `gic_overview_guide` (89), `aarch64_external_debug_guide` (73). This is **not
an ISF-completeness gap** — it is an over-extraction / recognizer-precision finding: a category-6 document
should classify as a non-target and not produce a chip contract. It is direct evidence for the `.3` recognizer
(the owner's "quickly determine which category") and a genericity concern (`[[feedback_genericity_guardrail]]`).

## Result 3 — the rule-lowering shortfall is mixed, not a single gap

`constraints + temporal + conditional → rules` lowers at ~41% (cat 1), ~45% (cat 2), ~11% (cat 3), ~22% (cat 4).
The shortfall is dominated by `conditional_rules`: `signal_constraints` and `temporal_rules` are clean signal
obligations that lower well, but many `conditional_rules` are prose-derived conditions that are not (yet) a
clean ISF rule. Part of this is **honest** (a conditional that names no signal obligation cannot become an ISF
rule without fabrication) and part is a **lever** (some conditionals could lower with a richer guard mapping).
This surface needs per-item triage in a dedicated `.4+` leaf before any fraction here is called a pure gap —
it is flagged honestly, not assumed.

## Prioritized buildable program (the scorecard → the roadmap)

1. **Gap A — register bit-field lowering** (32 docs, 12,638 fields, 0 lowered). The highest-leverage single
   lever; touches cat 1/2/3/4. Needs a SpecForge emitter path **and** an FSMGen field-structured-storage ISF
   abstraction. File the FSMGen FR after empirical submodule verification.
2. **Gap B — message-field structure carry + lowering** (11 docs, 1,220 fields, not even carried to IntentIR).
   Add the `Evidence→Intent` `message_field_records` carrier, then an ISF packet/structure construct (FSMGen
   FR). Closes the cat-2 structure frontier and the cat-1 message-heavy-protocol gap together.
3. **Cat 3 topology lowering** — promote clock/reset infrastructure + component connectivity from hint-level to
   a synthesizable ISF surface (likely another FSMGen abstraction).
4. **Cat 4 ISA lowering decision** — does CSR-field / instruction / privilege intent map onto existing
   register/storage abstractions or need a new ISF construct? Resolve with a measured decision packet.
5. **Conditional-rule lowering triage** (Result 3) — per-item, separate honest residual from a real lever.
6. **(Precision, not completeness) Cat-6 over-extraction** — feeds the `.3` recognizer; out of this gauge's
   ISF-completeness scope but recorded for the recognizer.

Cat 1 stays held at 1.000 by the wire golds throughout; cat 5/6 remain honest non-targets (their thin `.isf`
is correct).

## Honest caveats

- The category labels are one-off measurement labels (reconstructing the un-persisted `.1` ground truth); the
  five borderline calls flagged in `.1` (ARM Debug v6 → cat 3, eMMC → cat 1, OpenCAPI DLL → cat 1, plus
  `opencapi_discovery_configuration` → cat 2, `lpc_memory_agent_reference_design` → cat 3 here) were chosen to
  reproduce the `.1` distribution exactly and are documented in `scripts/measure_isf_completeness.py`. The
  gauge's conclusions are **label-robust**: Gap A and Gap B are objective per-document facts driven by measured
  surfaces, not by the exact label of any borderline doc.
- "Lowered" counts the realized `.isf` units, not their downstream FSMGen validity; the emitted `.isf` is
  separately gold-checked against FSMGen `--strict --check` and held at WIRE-BASED-100 = 1.000 / `kg-bench`
  156/156 by construction (this measurement changed no code, no canonical artifact).
- `jesd235` HBM lowers `blocked` (under-extracted thin doc, 2 actors) — a recall matter tracked elsewhere, not
  an ISF-lowering gap; it is the lone non-`renderable` doc in the buildable set.
