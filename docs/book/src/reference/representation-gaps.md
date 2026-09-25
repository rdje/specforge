# Extraction And Representation Gaps

A chip specification says things about a design. SpecForge's job is to **extract** them into its IR;
FSMGen's `.isf` language has to be able to **state** them so they reach the generated design. This page is
the ledger of every known place where one of those two steps fails, and of who owns fixing it.

It exists because the two failures look the same from outside (something in the PDF does not reach the
design) but have different owners. SpecForge does its part, FSMGen does its part
([ADR 0053](../../../decisions/0053-extraction-and-isf-requests-advance-in-parallel.md)):

- **SpecForge-owned**: the specification states it, and SpecForge does not read it yet, or has nowhere in
  its IR to put it. A SpecForge task fixes it.
- **FSMGen-owned**: SpecForge carries it in IntentIR, and FSMGen's published `.isf` contract has no way to
  say it. SpecForge files a reproduced feature request ([Requests To FSMGen](fsmgen-requests.md)); FSMGen
  decides and ships; SpecForge then uses it.
- **Both**: each half has its own owner, and FSMGen is never asked to do SpecForge's half.

## Open gaps

**No current chain hands FSMGen an `.isf` file at all** (measured `2026-09-25`): all 27 documents SpecForge
can currently rebuild stop at the adapter for want of a clock/reset contract (the first row), so every other
row describes what *would* reach FSMGen once that is fixed.

Every row was re-checked on `2026-09-25`: SpecForge's side against the persisted IR of the live chains (the
parameter rows against AXI IHI0022 L, AHB IHI0033 C and APB 5 IHI0024 E; the rest by
`python3 scripts/measure_intent_to_isf_reach.py` over all 27), FSMGen's side against its published book at the
revision SpecForge pins (13a *Interface*, 13d *assert*; 13k feature-matrix rows *Actor report metadata and
params*, *Interface ports*, *Reusable ISF libraries*, *Enum member values* and *Immediate verification checks*).

<!-- gap_owners:start -->

| Gap | What the specification says | SpecForge reads it | IntentIR holds it | `.isf` can say it | Owner | Status |
| --- | --- | --- | --- | --- | --- | --- |
| Clock and reset | AXI: `ACLK` is the global clock, `ARESETn` the global reset, active-LOW | **no**: 0 of 27 chains | **no** | yes: `(reset (rst_n async active_low))` | SpecForge `EXTRACTION-REPRESENTATION-GAPS.7` | open; **blocks every `.isf` file** |
| A signal's polarity | *nSRST is asserted* means it is driven low; AXI marks its resets *active-LOW* | partly: 51 records in 4 documents | yes | yes, as the right `1` or `0` | SpecForge `EXTRACTION-REPRESENTATION-GAPS.17` | open; `asserted` on a signal of unstated polarity now drives nothing (53 obligations held back on `2026-09-25`) |
| A value named by a word | AXI: `AWBURST must be INCR`; `ARADDR must be zero` | yes, as the word | yes | yes: enum members and numbers are legal rule values | SpecForge `EXTRACTION-REPRESENTATION-GAPS.9` | open (5 rules not lowered) |
| The condition of a value obligation | AXI: *an attached Subordinate must have its AWSNOOP input tied LOW*, when the Manager omits `AWSNOOP` (`AWSNOOP_WIDTH` 0) | **no**: the condition is lost | the value only, so it collides with the transaction that drives `AWSNOOP` | yes for a guard; a tie-off for an absent signal, see *Signal absent by default* | SpecForge `EXTRACTION-REPRESENTATION-GAPS.10` | open (23 rules dropped as conflicting or overlapping) |
| Stability and same-cycle exclusion | AHB: *the size of the transfer, as indicated by HSIZE, must not be changed*; *HEXOKAY must not be asserted in the same cycle as HRESP* | yes | yes | inside a transaction yes (`(assert (stable …))`); an always-on check outside one, **no** | SpecForge `EXTRACTION-REPRESENTATION-GAPS.11`; FSMGen `EXTRACTION-REPRESENTATION-GAPS.15` | open (10 rules not lowered) |
| A signal must hold a valid value | APB: *PSEL is asserted, which means that PADDR, PWRITE and PWDATA must be valid* | wrongly, as the value `VALID` | as a value | **no**: no known-value predicate | SpecForge `KG-ISF-COMPLETENESS.2a.vii.e`; FSMGen `EXTRACTION-REPRESENTATION-GAPS.16` | open (11 rules) |
| A protocol state machine | its states, transitions, guards, initial state and encodings | states only | untyped (45 states, 5 operations, 1 edge timing) | yes | SpecForge `EXTRACTION-REPRESENTATION-GAPS.12` | open |
| Register fields and reset values | a field's bit range; a register's reset value | partly | 158 fields without a usable range, 71 resets not a plain integer | yes for integers; symbolic resets to be measured | SpecForge `EXTRACTION-REPRESENTATION-GAPS.13` | open |
| Component topology | a platform's components and the links between them | partly | yes (235 links in 6 documents) | yes: FSMGen composes actors | SpecForge `EXTRACTION-REPRESENTATION-GAPS.14` | open; SpecForge emits one flat actor |
| Intent captured only as prose | 7,477 statements kept as text (constraints, behaviours, assumptions) | as text | untyped | not until typed | SpecForge `EXTRACTION-REPRESENTATION-GAPS.3` | open; how many say what no typed record says is to be measured |
| Derived width | AHB: `HWSTRB` is `DATA_WIDTH/8` bits; AXI: `ceil(ADDR_WIDTH/8)` (56 signals) | yes | yes | **no**: every published spelling is refused | FSMGen `FSMGEN-REQUESTS.1`; SpecForge uses it in `KG-ISF-COMPLETENESS.2a.vii.d` | FSMGen acknowledged `2026-09-24` |
| Signal absent by default | AXI: `MECID_WIDTH` values `0, 16`, default `0` (20 width parameters default to `0`) | yes | yes | **no**: a zero-valued parameter fails closed, and there is no optional port | FSMGen `EXTRACTION-REPRESENTATION-GAPS.4` | request being prepared; meanwhile SpecForge declares the smallest positive width and says so |
| Yes/no presence property | AXI: `PROT_Present`, values `True, False` (5 rows) | **no** | partly: nothing links the property to the signals it enables | **no**: no conditional port | SpecForge `KG-ISF-COMPLETENESS.2a.vii.c.vi`; FSMGen `EXTRACTION-REPRESENTATION-GAPS.4` | both open |
| Legal values and recommended range | AHB: `DATA_WIDTH can be 8, 16, … or 1024`; `ADDR_WIDTH is recommended to be between 10 and 64` | yes | yes (36 parameters) | **no**: a parameter declares a default only | FSMGen `EXTRACTION-REPRESENTATION-GAPS.5` | request being prepared; SpecForge uses the values to choose the default |
| A block that stays generic | AHB: `HRDATA` is `DATA_WIDTH` bits wide | yes | yes | partly: the `.isf` keeps the parameter's name, but generated ports get fixed widths and parameter-driven shapes are on FSMGen's backlog | FSMGen `EXTRACTION-REPRESENTATION-GAPS.6` | request being prepared |
| Negative or one-sided bound | AHB: *any value smaller than 32 or larger than 256 is not recommended*; APB: `PADDR can be up to 32 bits wide` | **no** | needs a range with one bound | not needed beyond the rows above | SpecForge `KG-ISF-COMPLETENESS.2a.vii.c.iv` | open; AHB `DATA_WIDTH` is declared `8`, legal but not recommended |
| Per-interface-class property table | AXI: `MPAM_WIDTH`, issue K, values `0, 11, 12, 14, 15`, one column per interface class (AXI5, ACE5-Lite, …) | **no** | values yes; a per-class value has no home | to be decided once SpecForge carries it | SpecForge `KG-ISF-COMPLETENESS.2a.vii.c.v` | open |
| Categories the recall measure does not count | configuration parameters, presence properties, interface-class profiles | measured by the census | — | — | SpecForge `EXTRACTION-REPRESENTATION-GAPS.3` | open |
| The score does not see parameters | WIRE-BASED-100 labels no parameter, so it scored identically before and after 36 parameters were added | — | — | — | SpecForge `KG-ISF-COMPLETENESS.2a.vii.f` | open |

<!-- gap_owners:end -->

The rows from *Clock and reset* to *Intent captured only as prose* come from the IntentIR-to-`.isf` census
(`EXTRACTION-REPRESENTATION-GAPS.2`); the PDF-to-IR census (`EXTRACTION-REPRESENTATION-GAPS.3`) will add what
the specifications state that no IR family holds.

## Deliberately not requested

**Physical and electrical parameters.** PHY and electrical specifications state setup and hold times, voltages,
impedances and unit intervals, and SpecForge keeps them typed in IntentIR (237 records on `2026-09-25`). They are not state-machine
behaviour, so SpecForge does not ask FSMGen to carry them: the project's category contract treats such
documents as an honest non-target by default, and ADR 0033 keeps non-executable knowledge in the IR rather than
forcing it into executable constructs. Asking FSMGen for a carrier (for example to feed timing constraints to
implementation tools) would reverse that decision; it is recorded here so the choice stays visible.

## What already flows end to end

These started as gaps and are closed on both sides:

- **A width written as a parameter** (`HRDATA | DATA_WIDTH`) lowers to `(width DATA_WIDTH)` with a
  `(params …)` declaration instead of `(width 1)` (`KG-ISF-COMPLETENESS.2a.vii.b`).
- **A default stated in a parameter table** (AXI `ADDR_WIDTH | 1..64 | 32`) becomes `(ADDR_WIDTH 32)`
  (`.2a.vii.c.i`).
- **A parameter with no usable default** takes the smallest positive value the document allows or
  recommends (AXI `DATA_WIDTH 8`, AHB `ADDR_WIDTH 10`), and the adapter's residual says it was a choice
  (`.2a.vii.c.ii`, `.2a.vii.c.iii`).
- **A rule drives the level the document states** (`EXTRACTION-REPRESENTATION-GAPS.8`): `must be LOW` drives `0`,
  `asserted` goes through the signal's stated polarity, and an obligation that states no level drives nothing
  and is listed instead. Until `2026-09-25` such obligations were given `1`, some of them the opposite of the
  specification; across the six affected documents the rules went from 250 to 95, 44 AXI values were corrected,
  and 166 obligations are now listed rather than invented. See
  [What value does a rule drive?](../pipeline/isf-adapter.md#what-value-does-a-rule-drive)

See [How wide is each signal?](../pipeline/isf-adapter.md#how-wide-is-each-signal) for how these work.

## How this page is kept current

- **When SpecForge pins a new FSMGen revision**, the refresh re-reads FSMGen's published book for every
  FSMGen-owned row and updates its *`.isf` can say it* and *Status* cells; the refresh task records that it did.
- **When a task that owns a row closes**, the same commit updates the row.
- **A row leaves the table only when both halves are done** and the result is verified on a real chain. It then
  moves to *What already flows end to end*.
- **A new gap** gets a row the moment it is found, with an owner on each side it concerns; a finding with no
  owner is not allowed to stand.
- **The owner column is checked on every commit.** Every task named in the open-gaps table must be declared
  and still open; naming a finished task, or one that does not exist, is refused (the `OWNERSHIP-CITATIONS`
  doctrine, which reads the table between its `gap_owners` markers).

Authoritative tracking: `EXTRACTION-REPRESENTATION-GAPS`; decision ADR 0053.
