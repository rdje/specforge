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

Every row was re-checked on `2026-09-25`: SpecForge's side against the persisted IR of the AXI (IHI0022 L),
AHB (IHI0033 C) and APB 5 (IHI0024 E) chains, FSMGen's side against its published book at the revision
SpecForge pins (13a *Interface*; 13k feature-matrix rows *Actor report metadata and params*, *Interface
ports* and *Reusable ISF libraries*).

| Gap | What the specification says | SpecForge reads it | IntentIR holds it | `.isf` can say it | Owner | Status |
| --- | --- | --- | --- | --- | --- | --- |
| Derived width | AHB: `HWSTRB` is `DATA_WIDTH/8` bits; AXI: `ceil(ADDR_WIDTH/8)` (56 signals) | yes | yes | **no**: every published spelling is refused | FSMGen `FSMGEN-REQUESTS.1`; SpecForge uses it in `KG-ISF-COMPLETENESS.2a.vii.d` | FSMGen acknowledged `2026-09-24` |
| Signal absent by default | AXI: `MECID_WIDTH` values `0, 16`, default `0` (20 width parameters default to `0`) | yes | yes | **no**: a zero-valued parameter fails closed, and there is no optional port | FSMGen `EXTRACTION-REPRESENTATION-GAPS.4` | request being prepared; meanwhile SpecForge declares the smallest positive width and says so |
| Yes/no presence property | AXI: `PROT_Present`, values `True, False` (5 rows) | **no** | partly: nothing links the property to the signals it enables | **no**: no conditional port | SpecForge `KG-ISF-COMPLETENESS.2a.vii.c.vi`; FSMGen `EXTRACTION-REPRESENTATION-GAPS.4` | both open |
| Legal values and recommended range | AHB: `DATA_WIDTH can be 8, 16, … or 1024`; `ADDR_WIDTH is recommended to be between 10 and 64` | yes | yes (36 parameters) | **no**: a parameter declares a default only | FSMGen `EXTRACTION-REPRESENTATION-GAPS.5` | request being prepared; SpecForge uses the values to choose the default |
| A block that stays generic | AHB: `HRDATA` is `DATA_WIDTH` bits wide | yes | yes | partly: the `.isf` keeps the parameter's name, but generated ports get fixed widths and parameter-driven shapes are on FSMGen's backlog | FSMGen `EXTRACTION-REPRESENTATION-GAPS.6` | request being prepared |
| Negative or one-sided bound | AHB: *any value smaller than 32 or larger than 256 is not recommended*; APB: `PADDR can be up to 32 bits wide` | **no** | needs a range with one bound | not needed beyond the rows above | SpecForge `KG-ISF-COMPLETENESS.2a.vii.c.iv` | open; AHB `DATA_WIDTH` is declared `8`, legal but not recommended |
| Per-interface-class property table | AXI: `MPAM_WIDTH`, issue K, values `0, 11, 12, 14, 15`, one column per interface class (AXI5, ACE5-Lite, …) | **no** | values yes; a per-class value has no home | to be decided once SpecForge carries it | SpecForge `KG-ISF-COMPLETENESS.2a.vii.c.v` | open |
| Categories the recall measure does not count | configuration parameters, presence properties, interface-class profiles | measured by the census | — | — | SpecForge `EXTRACTION-REPRESENTATION-GAPS.3` | open |
| The score does not see parameters | WIRE-BASED-100 labels no parameter, so it scored identically before and after 36 parameters were added | — | — | — | SpecForge `KG-ISF-COMPLETENESS.2a.vii.f` | open |

Two censuses will add rows: `EXTRACTION-REPRESENTATION-GAPS.2` measures, family by family, what IntentIR
holds that the adapter does not lower and why; `EXTRACTION-REPRESENTATION-GAPS.3` measures what the corpus
states that no IR family holds.

## What already flows end to end

These started as gaps and are closed on both sides:

- **A width written as a parameter** (`HRDATA | DATA_WIDTH`) lowers to `(width DATA_WIDTH)` with a
  `(params …)` declaration instead of `(width 1)` (`KG-ISF-COMPLETENESS.2a.vii.b`).
- **A default stated in a parameter table** (AXI `ADDR_WIDTH | 1..64 | 32`) becomes `(ADDR_WIDTH 32)`
  (`.2a.vii.c.i`).
- **A parameter with no usable default** takes the smallest positive value the document allows or
  recommends (AXI `DATA_WIDTH 8`, AHB `ADDR_WIDTH 10`), and the adapter's residual says it was a choice
  (`.2a.vii.c.ii`, `.2a.vii.c.iii`).

See [How wide is each signal?](../pipeline/isf-adapter.md#how-wide-is-each-signal) for how these work.

## How this page is kept current

- **When SpecForge pins a new FSMGen revision**, the refresh re-reads FSMGen's published book for every
  FSMGen-owned row and updates its *`.isf` can say it* and *Status* cells; the refresh task records that it did.
- **When a task that owns a row closes**, the same commit updates the row.
- **A row leaves the table only when both halves are done** and the result is verified on a real chain. It then
  moves to *What already flows end to end*.
- **A new gap** gets a row the moment it is found, with an owner on each side it concerns; a finding with no
  owner is not allowed to stand.

Authoritative tracking: `EXTRACTION-REPRESENTATION-GAPS`; decision ADR 0053.
