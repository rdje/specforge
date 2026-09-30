# EXTRACTION-REPRESENTATION-GAPS — unread intent leaves

- Part ID: `unread-intent-leaves`
- State: `active`

The `.3.d` family continues here, after `table-census-leaves` reached its mandatory rollover (`2026-09-30`): the
IntentIR assumptions and behaviors no typed record understands, and the readers that will.

## Post-migration records

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d`
  Status: `done` (`2026-09-30`; MEASUREMENT; the sealed record is in `program-and-census`; children `.3.d.i`-`.3.d.iii`)
  Goal: unchanged: decide what the unread IntentIR assumptions and behaviors are, and route each class.
  **Measured `2026-09-30`** (`python3 scripts/measure_unread_intent_records.py --samples 7`, new and read-only; it
  imports `measure_category_family_census.py`'s predicate). 443 records cite no statement a typed record cites: 166
  assumptions, 277 behaviors (167 and 288 on `2026-09-26`). No IR field says whether an agent is software (SemanticIR
  actors are role nouns), so the script declares a closed list of software-agent words; every other test is
  structural. Families, first match wins, seven seeded members of each read:
  - **pass assumption, 17:** IntentIR's own modeling choices, which cite no statement (`intent.rs` writes them).
    By design; not document intent.
  - **table row, 50:** Markdown table rows lifted as prose (7 of 7). Their tables' family owners (`.3.b`) own them.
  - **apparatus, 40:** document self-reference, figure examples, informative notes (5 of 7; 2 notes carry an
    obligation). Non-contract.
  - **software agent, 47:** software obligations (6 of 7; 34 of the 47 are in the USB4 connection-manager guide);
    one is I2C's *output drivers*, hardware. `.3.d.i`.
  - **assumed by word, 111:** SemanticIR lifts a sentence to an abstraction by the words *assume* or *treated as*;
    4 of 7 are hardware rules or defaults, 2 conventions, 1 a fragment (*60% Cu Assumer*). `.3.d.ii`.
  - **permission or description, 178:** 2 of 7 obligations no reader reads, 2 permissions, 3 descriptions; 17 carry
    the `timing_constraint` class though none states a time. `.3.d.iii`.
  **Decided:** assumptions are not a class of intent, and the behaviors are four kinds of content; each open kind
  has an owner. The census record is in [the category-family census](../../research/category-family-census.md).
  No code change, so no acceptance checklist applies.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.i`
  Status: `open` (`2026-09-30`; DESIGN + CODE; found by `.3.d`)
  Goal: **a software agent's obligation is recorded as software intent, not as an untyped behavior.** 47 unread
  records oblige or permit a software agent (*The Connection Manager may enable CLx 50ms after TMU has been
  activated*; *The firmware is responsible for creating the stage 2 translation tables for the untrusted OS*); 34 are
  in the USB4 connection-manager guide. The category
  contract calls such content non-applicable to executable IntentIR and requires an explicit disposition.
  **Reproduce:** `scripts/measure_unread_intent_records.py`, family `software_agent`. **Impact:** a document's
  software procedure is indistinguishable from unread hardware behavior. Acceptance: decide how an agent is known to
  be software from the document (no IR field says so today), give the obligation a typed non-executable disposition,
  and measure every chain.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.ii`
  Status: `done` (`2026-09-30`; MEASUREMENT, re-scoped from CODE; found by `.3.d`; children `.3.d.ii.a`, `.3.d.ii.b`)
  Goal: **an assumption is what a document states it assumes.** SemanticIR lifts any sentence saying *assume* or
  *treated as* to an abstraction, and IntentIR carries it as an assumption: 111 such records are hardware rules or
  defaults (*If a fault condition prevents locating a valid device context then the DTF value assumed for reporting
  such faults is 0*; *the missing signals are assumed to be LOW*), conventions (*the latencies shown assume an L1
  hit*) and fragments. **Reproduce:** family `assumed_by_word`. **Impact:** a default value the hardware must use is
  filed as an assumption no reader lowers. Acceptance: read a stated default (`X is assumed to be V`) into the typed
  record its subject takes, keep a convention as a non-contract assumption, and measure every chain.
  **Measured `2026-09-30`** (the family's 111 records, by sentence shape): 5 give an absent signal's value (AXI's
  *If the BRESP_WIDTH property is 0, it is not present and assumed to be 0b000 (OKAY)*, RRESP alike, *Any AxSNOOP bits
  not driven by an interface are assumed to be LOW*, and APB-E's and AHB's *the missing signals are assumed to be
  LOW*); 9 more state a default (AXI's domain defaults when `AxDOMAIN` is omitted, IOMMU's *the DTF value assumed …
  is 0*); 31 are *treated as* rules (*LPIs are always treated as Non-secure Group 1 interrupts*); 14 are conventions
  (*This guide assumes that you are familiar with …*); 52 mix the rest.
  **Why this is not one code slice:** the carrier an absent value belongs on, EvidenceIR's `signal_presence_records`
  (`BRESP_WIDTH > 0`), stops at EvidenceIR — no SemanticIR or IntentIR field carries presence. Carrying it is a new
  top-level field in two stages and re-seals every chain; FSMGen already accepted the matching `.isf` request
  (`sf-isf-optional-signal`, its `(absent-value V)`). So the measurement closes here and the code is two children.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.a`
  Status: `open` (`2026-09-30`; CODE; found by `.3.d.ii`)
  Goal: **a signal's stated absent value is carried with its presence to IntentIR.** Carry
  `signal_presence_records` through SemanticIR and IntentIR, and read a sentence that gives an absent or undriven
  signal a value (*not present and assumed to be 0b000*, *not driven … assumed to be LOW*, *the missing signals are
  assumed to be LOW*) into that signal's record as its absent value, citing the statement. **Reproduce:** the 5
  records of `.3.d.ii`'s first shape. **Impact:** a width-0 `BRESP` has no value in IntentIR, so no consumer can tie
  it off. Acceptance: the carrier and the reader, the re-seal handled as `.3.b.iii.g`'s was, and every chain measured.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b`
  Status: `open` (`2026-09-30`; MEASUREMENT then CODE; found by `.3.d.ii`)
  Goal: **a stated default or equivalence reaches its reader, and a convention is non-contract.** Of `.3.d.ii`'s
  family, 9 state a default of something other than an absent signal, 31 are *treated as* rules and 14 are
  conventions. **Reproduce:** `scripts/measure_unread_intent_records.py`, family `assumed_by_word`. **Impact:** a
  classification rule (*LPIs are always treated as Non-secure Group 1*) is filed as an assumption. Acceptance: route
  each shape to the reader it belongs to (an enum member's default, a classification) or record it non-contract, and
  measure every chain.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.iii`
  Status: `open` (`2026-09-30`; MEASUREMENT then CODE; found by `.3.d`)
  Goal: **a stated permission is typed intent, and an unread obligation reaches its reader.** 178 unread behaviors
  mix permissions (*A MASTER MAY use End-Of-Burst to indicate a single access*), obligations no reader reads (*The
  host shall schedule isochronous transactions such that …*) and descriptions; 17 carry EvidenceIR's
  `timing_constraint` class though none states a time (Wishbone's *MASTER and SLAVE interfaces MAY be designed …*).
  **Reproduce:** family `permission_or_description`. **Impact:** optional behavior has no carrier, and obligations
  stay untyped. Acceptance: split the family by modality, route each obligation class to the reader that should take
  it, decide a carrier for a permission, find why those 17 are classed as timing, and measure every chain.
