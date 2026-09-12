# ANCHORLESS-INVARIANT-DROP: the premise was wrong — a phantom was not an anchor, it was an admission gate

## Metadata

- Tree ID: `ANCHORLESS-INVARIANT-DROP`
- Status: `done` (`2026-09-12`; `.0` disproved the tree's own premise and handed the real defect to `[[INVARIANT-SHAPE-ADMISSION]]`)
- Roadmap lane: `R2` (extraction fidelity / residual accounting)
- Created: `2026-09-12`
- Last updated: `2026-09-12`
- Owner: repo-local workflow

## Goal

*(as opened)* A SemanticIR invariant carries `related_interface_ids`. When every one of those
references disappears, the invariant was believed to disappear with them, unrecorded. Make an
anchorless invariant accountable.

**`.0` disproved this.** `related_interface_ids` is not an admission condition:
**560 of ADIv6's 593 invariants carry no anchor at all** and are published normally. The 13 that
vanished were not losing an anchor; they were losing an **admission gate** — `is_invariant_like`'s
second route requires the statement to *mention a declared signal*, and the phantom `In` was
satisfying it for every sentence containing that word.

The tree closes with no code change. What it found on the way is a larger and correctly-premised
defect, tracked as `[[INVARIANT-SHAPE-ADMISSION]]`.

## How it was found

`ACTOR-NOUN-RELATION-DECLARATION.1` stopped an inferred declaration minting an ordinary word as a wire,
which removed ADIv6's phantom signal `In`. Rebuilding that document's chain:

| | before | after |
| --- | ---: | ---: |
| IntentIR interfaces | 21 | 14 — the 7 lost are all `…in…` groupings built on the phantom |
| IntentIR constraints | 614 | 601 |
| SemanticIR invariants | 606 | **593** |

Of the 57 constraints that moved, **44 keep their id** and simply shed the phantom-bearing entries from
`related_interface_ids`, retaining their real ones. That part is exactly right. The other **13 vanish**,
and their ids say why they were vulnerable:

```text
constraint_invariant_in_the_capture_dr_state_a_logic_0_is_loaded_into_this_register
constraint_invariant_in_the_update_dr_state_nothing_happens_the_shifted_in_data_is_ignored
constraint_invariant_in_the_shift_dr_state_this_data_is_shifted_out_least_significant_bit_first
…
```

Their sentences **begin with the word "In"** — which is precisely why the phantom was minted from them
in the first place. The reading at the time was that they had been anchored to nothing but that phantom,
so when it went, they went.

**That reading was wrong on both halves, and `.0` says so below.** They were not anchored to the
phantom — anchors do not gate publication — and they are not requirements: none of the 13 contains a
modal verb. What the phantom supplied was an admission gate, and what left with it was descriptive
prose that should never have been published as a normative constraint.

One thing from the opening reading survives intact. `SPEC-TO-INTENT-ALIGNMENT`'s signoff states "zero
fabrication and zero unexplained drops"; a single upstream correction moved 13 records out of the
artifact with nothing recording it. Whether those 13 deserved to be there is a separate question from
whether their departure should be visible, and `[[INVARIANT-SHAPE-ADMISSION]]`'s acceptance criteria
carry that requirement forward.

## Non-Goals *(as opened)*

- Do not keep an invariant by re-attaching it to a phantom. The upstream correction is right.
- Do not widen `related_interface_ids` to admit an anchor that is not a declared signal.
- Do not treat this as an ADIv6 defect; the mechanism is general and the census must say how general.

The third held and is what found the real defect. The first two were answered by the premise being
false: there is no anchor mechanism to re-attach to or widen.

## Acceptance Criteria *(as opened, and how they resolved)*

- ~~measure the fragile set — invariants anchored solely to one interface~~ → **not the fragile set.**
  560 of ADIv6's 593 invariants have no anchor at all and publish normally. The fragile set is the
  one gated on *mentioning a declared signal*: **304 of 5,856** current invariants, of which 14 rest
  on an inferred declaration and all 14 of those name real wires.
- ~~make the drop visible~~ → carried forward to `[[INVARIANT-SHAPE-ADMISSION]]`, which owns a
  population that genuinely should not be published.
- ~~no invariant with a real anchor changes~~ → vacuous; anchors do not gate publication.
- `scripts/check_doctrines.sh` green; no ceiling, milestone, or contract widened. **Met** — this tree
  ships a read-only census and no code change.

## Task Tree

- ID: `ANCHORLESS-INVARIANT-DROP` · Status: `done` (`2026-09-12`) · Children: `.0`

- ID: `ANCHORLESS-INVARIANT-DROP.0` · Status: `done` (`2026-09-12`) · Goal: **measure the fragile set,
  and find where the drop happens.** Both parts delivered, and the second disproved the tree.
  Producer: `python3 scripts/measure_invariant_admission_shape.py`.
  Commit: `ANCHORLESS-INVARIANT-DROP.0`

## Current Frontier

None. The tree is closed: its premise was false and the defect it was opened for does not exist. The
real one is `[[INVARIANT-SHAPE-ADMISSION]]`.

## `.0` — result (`2026-09-12`)

### The premise was false

`related_interface_ids` is not a condition of publication. ADIv6 publishes 593 invariants and **560 of
them carry an empty anchor list**. Nothing is dropped for want of an anchor.

### What actually admitted the 13

`is_invariant_like` (`crates/specforge/src/ir/semantic.rs`) has three routes, and the second is
`mentions a declared signal AND carries a weak phrase` from `handshake` / `asserted` / `deasserted` /
`transition` / **`state`** / `timing` / `observed`. All 13 contain `state`, and the declared signal
they mentioned was the phantom `In`.

**The general form, which is the durable part: a phantom declaration does not only add phantom
records — it widens an admission gate.** Any pass that asks "does this statement mention a declared
signal?" becomes true for every sentence containing that word, and a phantom spelled like an ordinary
word appears everywhere. Thirteen ADIv6 entries existed because `In` was a signal.

### The 13 adjudicated, which corrects the ledger of the slice that removed them

Every one was checked for a modal verb. **None has one.** They are descriptive, not normative:

```text
- In the Capture-DR state, a logic 0 is loaded into this register.
- In the Update-DR state, nothing happens. The shifted-in data is ignored.
- In the Shift-DR state, this data is shifted out, least significant bit first.
| PORTENABLED In | Port | Enabled | Can be deasserted by the JTAG subsystem, … |
```

An `InvariantRecord` is meant to be a normative constraint. These state what happens in a TAP state;
the last is a mangled table row whose `In` is a column fragment. **Their removal is a precision gain,
not a cost**, and `ACTOR-NOUN-RELATION-DECLARATION.1`'s commit message — which recorded them as "13
real JTAG requirements" lost — was too generous to the old behaviour. The substance of that slice
stands; this is the adjudication it did not yet have.

### The real defect, measured

Looking at admission routes exposed the shape of what they admit. Current stratum, at the product
boundary: **1,528 of 5,927 IntentIR constraints (25.8%) are not statements** — 769 serialized markdown
table rows and 759 figure captions, one of which reads `Figure 1.` in full. The `r3` visual-evidence
route is the worst: 910 non-statements against 245 prose. Handed to
`[[INVARIANT-SHAPE-ADMISSION]]` with the census.

## Decisions

- `2026-09-12` — **the upstream correction was shipped before this was fixed.** The alternative was to
  keep a phantom wire in a published IntentIR so that thirteen invariants would keep a false anchor.
  A downstream consumer asked to synthesise a port called `In` is a worse outcome than a recorded gap,
  and the gap is now recorded — here.
- `2026-09-12` — **opened as its own tree rather than a leaf of `SPEC-TO-INTENT-ALIGNMENT`.** That tree's
  frontier is its own product programme and its parts are sharded; this is one mechanism with one
  measurement, and folding it in would bury it.

## Open Questions *(resolved or routed)*

- ~~Which carrier should an anchorless invariant use?~~ **Moot** — an invariant with no anchor is
  published normally, so there is nothing to carry. The carrier question moves to
  `[[INVARIANT-SHAPE-ADMISSION]]`, where records genuinely will be removed.
- ~~How many of the fragile set are fragile because their only anchor is weak?~~ **Answered, once the
  question is restated against the real gate.** Of the 304 current invariants admitted by
  `mentions a declared signal + a weak phrase`, **14** qualify only through an *inferred* declaration
  — 9 in I2C, 5 in ADIv6 — and every one of those 14 names a real wire (`SCL`, `SDA`, `nSRST`, `TDO`,
  `CSYSPWRUPACK`). After `ACTOR-NOUN-RELATION-DECLARATION.1` no current invariant rests on a phantom.
- **Still open, and routed rather than answered**: is `r3` defensible? A statement admitted purely for
  sitting near a normative figure yields 245 prose invariants against 910 non-statements. Owned by
  `[[INVARIANT-SHAPE-ADMISSION]]`.

## Blockers

None.

## Verification Log

- `2026-09-12` — `.0`. Read-only; no artifact written, rebuilt or mutated.
  `python3 scripts/measure_invariant_admission_shape.py` over the persisted SemanticIR and IntentIR
  corpus. The phrase lists mirror `is_invariant_like` verbatim, including `contains_phrase`'s
  whole-word boundary semantics. Route attribution is by elimination from statement text — a statement
  with no modal and no weak phrase must have been admitted by visual evidence — which is stated in the
  script because it is the census's one inference rather than a reading.
  The 13 dropped ADIv6 invariants were recovered from
  `generated/preserved/ACTOR-NOUN-RELATION-DECLARATION.1/pre-rebuild/` and each checked individually
  for a modal verb; none has one.

## Commit Log

- Opened in the commit that shipped `ACTOR-NOUN-RELATION-DECLARATION.1` (`bea8437d`).
- `.0` — `ANCHORLESS-INVARIANT-DROP.0`. Tree CLOSED.

## Changelog

- `2026-09-12` — `.0` closed and the tree with it. The premise was false: 560 of 593 ADIv6 invariants
  carry no anchor and publish fine. The phantom was supplying an admission gate, not an anchor, and the
  13 it admitted are descriptive rather than normative — their removal is a precision gain. The census
  exposed the real defect: 1,528 of 5,927 published constraints are captions or table rows
  (`[[INVARIANT-SHAPE-ADMISSION]]`).
- `2026-09-12` — tree created. Removing one phantom signal from ADIv6 deleted 13 real JTAG invariants
  with no record of the deletion, while 44 sibling constraints correctly kept their id and shed only the
  phantom anchor.
