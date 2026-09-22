# One code on two members — the class `.5.vii.a` could not reach, and the oracles that cannot see it (`KG-ISF-COMPLETENESS.5.xiv`)

`.5.v` set the standing rule: a fabricated value equal to one the SAME enum states elsewhere is
*"provably wrong — no encoding assigns one code to two members, so this is a defect that needs no
interpretation."* `.5.vii.a` then deleted the positional fallback, which removed every collision made
by a row index. **266 enums still collide, and 9 of them are replayable.** Reproduce with:

```bash
python3 scripts/adjudicate_enum_member_name_column.py --collisions
```

## They survive because the cell DOES parse

`.5.vii.a` drops a member whose value cell the reader cannot read. These cells read perfectly — the
value is document-stated. What fails is the CHOICE of column: `infer_encoding_column_indices` takes the
first header matching `value|encoding|code|binary|hex|bit`, and that column does not determine the
member.

## Reach, followed to the product boundary rather than asserted

| document | enum | members | colliding | IntentIR `symbol_definitions` |
| --- | --- | ---: | ---: | --- |
| AHB `ihi0033_c` | `HPROT` | 8 | **6** | yes |
| AXI `ihi0022_l` | `AWSNOOP` | 8 | 5 | yes |
| AXI `ihi0022_l` | `BTAGMATCH` | 9 | 5 | yes |
| AHB `ihi0033_c` | `HSIZE` | 6 | 4 | yes |
| AXI `ihi0022_l` | `ARSNOOP` | 8 | 3 | yes |
| AXI `ihi0022_l` | `ARCACHE` | 10 | 2 | yes |
| AXI `ihi0022_l` | `SYSCOACK` | 4 | 2 | yes |
| APB `ihi0024_e` | `PPROT` | 4 | 2 | **no** |
| AXI `ihi0022_l` | `AWATOP` | 11 | 1 | yes |

**`README.md` declares `IntentIR` the product boundary, and 8 of the 9 cross it.** APB `PPROT` does
not, because that document promotes no symbols at all — its `symbol_definitions` is empty. **None
reaches `.isf`**: AXI, AHB and APB are all `lowering_status: blocked`, for temporal, state and
width residuals unrelated to enums. So the defect is live at the boundary the project publishes and
stops one stage short of the file FSMGen would consume.

## The leaf was opened on one cause. There are three.

`.5.xiv` was opened claiming `value_col` *"picks ONE BIT of a split field"*. Classifying all nine
refutes a single cause:

- **split bit field** — AHB `Table 3-2` is `HSIZE[2] | HSIZE[1] | HSIZE[0] | Size (bits) |
  Description`, so `value_col` matched the TOP BIT and `BYTE`, `HALFWORD`, `WORD`, `DOUBLEWORD` all
  publish as 0. One enum: `HSIZE`.
- **one bit of a wider field, the rest not in the table** — APB `Table 2-2` is
  `PNSE | PPROT[1] | Physical address space`: `value_col` is `PPROT[1]` alone and `name_col` is `PNSE`,
  so the published members are named `0` and `1` and each carries two different codes.
- **genuine multi-key** — AXI `Table A7.3` is `(AWSNOOP, AWDOMAIN) -> opcode`; 13 rows, 8 distinct
  codes, `0b00000` on five of them. Also `ARSNOOP`, `ARCACHE`, `SYSCOACK`, `BTAGMATCH`.
- **cross-table** — `AWATOP` collides between `table_0096` and `table_0097`, each of which determines
  its own members. That is `.5.vi` conflation, not a column defect.

**The unifying statement is weaker and truer than the one the leaf was opened on: the value column
does not functionally determine the member.** A repair must not assume which way it fails.

## The finding that changes how the repair must be verified

`.5.xiv`'s own contract named *"WIRE-BASED-100 scored on rebuilt golds with both binaries"* as its
verification. **That bar cannot fail on this defect class.**

| oracle | scope | can it see an enum member value? |
| --- | --- | --- |
| `kg-bench` | 156 fixtures | **no** — 0 of 156 mention `enum` or `symbol_definition` at all |
| WIRE-BASED-100 / `eval-extraction` | 93 labelled items | **no** — its 9 task kinds are `signal_constraint`, `actor_signal_relation`, `temporal_rule`, `declared_signal`, `register_field`, `serial_frame_field`, `protocol_operation`, `protocol_state`, `interface_edge_timing` |

This is the same failure mode `.5.viii` recorded as *"a count cannot verify a value"*, one level up:
**an oracle that cannot fail is not verification.** It also explains the `.5` programme's own history —
mega-conflation, positional values, the quoted radix, the unquoted radix, the naming column and now
these collisions were all invisible to the project's two headline gates, which is why each was found by
a census rather than by a test. `qualified-role-header-proves-no-role` recorded the same lesson for
signal tables: *a green wire-protocol score is not evidence that extraction is intact.*

## Decision

1. **The oracle lands with the gate, in `.5.xiv.a`, and not after it.** A kg-quality fixture that
   asserts an enum member value is the missing red case; without it the gate ships against a bar that
   passes whatever it does.
2. **The repair is SUBTRACTIVE and needs no new inference.** An enum whose members carry contradictory
   codes is not faithfully lowerable, so it is held out and recorded as a named residual — the
   mechanism `.2a.iv` already established at the `.isf` boundary for an un-renderable value literal,
   applied at the seam where the collision becomes visible.
3. **Recovery is a separate question and a later leaf.** Composing `HSIZE[2:0]` into one code, or
   lowering a multi-key table as a relation, are additive repairs that each need their own
   measurement. Publishing a contradiction while they are designed is the worse option.

## Authoritative tracking

`docs/tasks/KG-ISF-COMPLETENESS.md` — `.5.xiv` (this measurement), `.5.xiv.a` (the gate and its
oracle). Instrument: `scripts/adjudicate_enum_member_name_column.py --collisions`.
Knowledge Map: `[[enum-member-values-have-no-oracle]]`,
`[[enum-member-name-column-needs-adjudication-not-a-score]]`,
`[[minted-enum-values-are-mostly-row-indices]]`.
