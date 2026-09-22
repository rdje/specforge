# KG-ISF-COMPLETENESS — enum surface recent leaves

- Part ID: `enum-surface-recent-leaves`
- State: `active`

<!-- kg-isf-completeness-task-source-region:enum-surface-recent-leaves:start -->
- ID: `KG-ISF-COMPLETENESS.5.vi` · Status: `done` (`2026-09-21`; MEASUREMENT, no code — the
  adjudication is the deliverable and it refuted both rules the leaf was opened to choose between;
  it opens `.5.vi.a`) · Goal:
  **an enum whose members cannot come from one table is not one enum.** `.5.v` measured 231 of 550
  minted enums with no single source table, 228 of them with no column anywhere containing every
  member, carrying 90% of all minted members and 226 of the 512 symbols that reach `IntentIR`.
  Acceptance: a gate at the enum-IDENTITY seam, measured FP-free on the whole corpus before it ships,
  the way `.5.i`/`.5.ii`/`.5.iii` each earned theirs. The obvious rule — refuse an enum whose members
  span tables — must first be tested against the legitimate case the census names: a specification
  that splits one code table across a page break. Measure how many of the 228 are that, and how many
  are a name colliding across unrelated `Field | Description` tables, BEFORE choosing between refusing
  the enum and splitting it per table.
  Non-goal: re-opening the NAME gates; widening `scan_encoding_tables_by_signal_anchor`'s early return.
  Prerequisite: `.5.v`.
  Verification: per-item adjudication over the 228, plus a re-run of
  `scripts/measure_enum_member_value_fidelity.py` showing the promoted-symbol table move.

  **ADJUDICATED `2026-09-21`** — `scripts/adjudicate_conflated_enum_identity.py`, read-only over
  all 78 persisted chains; report `docs/research/enum-identity-adjudication.md`. Attribution
  replays the producer per table (`infer_encoding_column_indices` + the member/value synthesis)
  and segments the minted members into ordered runs, so the tables behind an enum are the ones the
  producer actually walked: **498 of 550 reconstruct in one monotone walk**, 45 need a convergence
  restart, 7 fall back to a set cover.

  | verdict | enums | members | >60 chars | promoted |
  | --- | ---: | ---: | ---: | ---: |
  | `single_table` | 316 | 2,130 | 308 | 283 |
  | **`stated_continuation`** | **2** | 10 | 2 | 2 |
  | `adjacent_same_shape` | 22 | 244 | 73 | 22 |
  | `distinct_tables` | 204 | 18,602 | 2,917 | 199 |
  | `unattributed` | 6 | 606 | 167 | 6 |

  **The legitimate case this leaf was told to protect is two enums.** `stated_continuation` is the
  document's own `Table B2.2 Continued from previous page` marker — read by `caption_table_ref` and
  already trusted in production by `PDF-VARIANT-DIGESTION.12a` — plus agreement on which table is
  continued. Exactly two minted enums qualify, both in the AXI gold `ihi0022_l`: `ARDOMAIN`
  (`Table A7.4`) and `SYSCOACK` (`Table A15.26`). **The near-zero is not vacuous, checked against
  BOTH dialects** — 209 captions in 7 documents write the marker after the label, 112 in 20
  documents write it at the end (`… (continued)`), one caption carries both, and reading all 320
  still gives 2. **27 fused enums sweep such fragments in**, which confirms the collision: CHI C2C
  `ihi0098_b`'s `TABLE` spans 114 tables including **29 fragments from 20 different parents**,
  where a continuation chain has one.

  The weaker layout test (`adjacent_same_shape`: identical first header row, compatible captions,
  adjacent pages, consecutive ids) finds 22, and all 22 were adjudicated by hand: the class
  **mixes**. AXI+ACE `AWSNOOP`/`ACSNOOP` genuinely resume one code sequence across the break while
  their document states nothing; RISC-V `ABSTRACT`/`CONFIGURATION`/`SYSTEM` each pair two
  `Field | Description | Access | Reset` tables for **different registers**, which is the name
  collision this leaf named, wearing the continuation's clothes. **So outside the two stated cases
  no available oracle separates them**, and both rules the leaf was opened to choose between are
  unusable: refusing a span fires on the genuine cases, and splitting per table turns one `TABLE`
  enum into **491 symbols**.

  **The defect is the NAME, not the span, and the discriminator is the caption LABEL.**
  `derive_encoding_enum_name` names a table after any declared signal in
  `caption + section_title + header`. Resolved per contributing table, `caption_label` — the
  publisher's `Table 2-1`/`Table A7.4` numbering furniture — carries **1,634 of the 3,325 tables**
  behind `distinct_tables` (49%) and **6 of 316** single-table enums, **0 of the 56** behind the
  continuations. Simulated corpus-wide, forbidding it refuses **6** single-table enums
  (`FIGURE` ×4, `TABLE` ×2 — furniture in every case), **0** continuations of either kind, 13
  conflations, and cuts 1,634 contributing tables. Across all 550 enums the rule reaches **54,
  carrying exactly three names: `TABLE` 41, `FIGURE` 11, `CHAPTER` 2**. There is no fourth, and
  `CHAPTER` is what makes it document grammar rather than a two-word denylist (ADR 0006): the label
  is `<word> <number>` whatever word the publisher chose.

  **CORRECTED `2026-09-21` by `.5.vii`: the two `stated_continuation` enums are legitimate in
  IDENTITY ONLY, and both are content junk.** `SYSCOACK` is
  `DISABLED=0 CONNECT=0 ENABLED=1 DISCONNECT=1` — every value duplicated, because `row_idx` is
  table-local and each fragment was numbered from zero, so a page-break continuation is the one case
  where the positional fallback CANNOT be right. `ARDOMAIN`'s `value_col` is the `ARDOMAIN 1` column
  holding `NSH, SH`, which never parses, so `name_col` fell to the Description column and its members
  are whole sentences with ordinal values. This leaf adjudicated WHICH TABLES an enum came from and
  protected the two the document vouches for; it never read what they contained. **Nothing above is
  withdrawn** — the identity verdict, the caption-label discriminator and the rule's reach all stand,
  and `.5.vii` re-derives this leaf's attribution exactly. What is withdrawn is the implication that
  a protected enum is a healthy one. KM `[[minted-enum-values-are-mostly-row-indices]]`.

  **The two alternatives are refuted by the same measurement.** Naming only from the table's own
  caption text or header refuses **233 of 316** healthy single-table enums (215 promoted); banning
  the section heading refuses **227** (210 promoted). The heading is the corpus's single most
  common naming source and is legitimate — it is *merging* unrelated tables on the resulting name
  that is wrong, and that lives in `build_symbol_definitions`, not at this seam.

  **THREE INSTRUMENT CORRECTIONS, recorded because they changed the headline.** (a) The
  caption-label pattern required the numbering token to begin with a digit, so every Arm
  volume-numbered caption (`Table A7.4`, `Table B13.12`) went unrecognised and its `Table` scored
  as caption prose: `caption_label` tables **1,217 → 1,634**, the rule's reach **43 → 54**. (b) A
  shared caption reference was accepted as a statement of continuation — three CHI C2C tables
  captioned `Chapter B15. …` share a chapter, and a chapter is not a table — while a punctuation
  mismatch (`Table A7.4:` against `Table A7.4`) hid the two real pairs. (c) Only ONE of the
  marker's two dialects was being read; the trailing `(continued)` form covers 20 documents to the
  prefix form's 7, so the instrument was blind on most of the corpus **while its vacuity check
  looked healthy**. Together: `stated_continuation` **0 → 2**, swept-in fragments **7 → 27**.
  **This leaf's first published conclusion — "the legitimate case has no population at all" — was
  wrong, and wrong in the direction that flattered the rule it was proposing.** All three were
  found by following one odd-looking item, never by re-reading the code: a suspicious
  `Chapter B15.` verdict, an AXI pair that obviously WAS a continuation, and a randomly sampled
  caption ending in `(continued)`. **A vacuity check that passes does not validate the numerator.**
  The rule survives all three. KM `[[caption-label-is-not-table-identity]]`.
  **Frontier → `.5.vi.a` (CODE).**

- ID: `KG-ISF-COMPLETENESS.5.vi.a` · Status: `done` (`2026-09-21`; CODE; opened by `.5.vi`'s
  adjudication; trial reverted once, then landed after its blocker cleared) · Goal: **a caption's label prefix may not name an encoding table.** Strip the
  `<word> <number>` caption label before the caption reaches `derive_encoding_enum_name`'s
  `identity_context` and its `caption_candidate` scan, so the publisher's numbering furniture can
  neither match a declared signal nor supply the fallback candidate. The header-sourced path
  (`.5.iv.a`) and the section-title path are untouched by construction.
  Acceptance: the measured population is exactly the 54 enums named `TABLE` (41), `FIGURE` (11) and
  `CHAPTER` (2) (`.5.vi`), so the landing must show that set and no other enum changing. The
  numbering token must admit a leading volume letter (`Table A7.4`) or the gate silently misses
  every Arm caption — the instrument defect `.5.vi` corrected in itself, and the same mistake is
  available to the production rule. This is byte-changing on
  scored documents, so it carries the full high-stakes protocol `.5.i` and `.5.iv.a` ran under:
  before/after WIRE-BASED-100 on rebuilt gold evidence (= 1.000 held), `kg-bench` 156/156, a
  corpus-wide old-versus-new replay naming every changed artifact, and FSMGen `--strict --check`
  on every changed `.isf`.
  Non-goal: the section-heading naming path — `.5.vi` measured it legitimate (227 of 316 healthy
  single-table enums depend on it); the merge-by-name in `build_symbol_definitions`; the NAME gates
  `.5.i`/`.5.iv.a` themselves.
  Prerequisite: `.5.vi`, and now **`.5.vii.a` as well** — see the trial below. (`.5.vii` closed
  `2026-09-21` as a MEASUREMENT and carries no cleaning effect; the leaf that must land first is its
  code slice `.5.vii.a`, and `.5.vii` also confirmed the trial's reading — the enums this rule adds on
  the three current chains are the positional class, and both of the continuations this tree protects
  are already corrupted by it.)
  Verification: re-run `scripts/adjudicate_conflated_enum_identity.py` and
  `scripts/measure_enum_member_value_fidelity.py` on rebuilt chains and show the
  `distinct_tables`/promoted-symbol rows moving by the predicted amount and nothing else.

  **TRIAL RUN `2026-09-21` (implemented, measured, REVERTED unlanded).** The patch is small and its
  shape is settled: a `caption_descriptive_text` helper strips a `<alphabetic word> <token-with-a-digit>`
  label (the token may open with a volume letter — `A7.4` — or the gate misses every Arm caption), and
  `derive_encoding_enum_name` reads that text in BOTH places it reads the caption, preserving the
  `None`-caption fallback to the section title exactly so the change only ever removes a naming source.
  Five unit tests pass; `cargo fmt` clean; `kg-bench` **156/156**.

  **`.5.vi`'s prediction was incomplete, and the trial is how that was found.** The simulation modelled
  the rule as SUBTRACTIVE — which tables lose their name. But the signal-match branch returns the
  **longest declared signal referenced in the identity context**, so deleting text also changes which
  signal WINS. The rule therefore ADDS enums as well, and `.5.vi` measured none of that.

  **Replay over all 27 rebuildable chains** (`evidence --dry-run`, read-only, against the persisted
  EvidenceIR): **24 identical on the entire statement surface; 3 changed, and every delta is a purely
  additive `Enum` statement with zero non-enum change.**

  | document | enum statements | added enums |
  | --- | --- | --- |
  | `ihi0022_l` (AXI wire gold) | 79 -> 133 | 10, plus `ARSNOOP`/`AWSNOOP` extended |
  | `ihi0074_a` (ADIv6) | 15 -> 50 | 5 |
  | `usb_3_2` | 0 -> 6 | 1 |

  **Most of what it unblocks is real.** On the AXI gold the label had been hiding genuine field
  encodings: `AxBURST` (`FIXED_BURST 0`, `INCREMENTING_BURST 1`, `WRAPPING_BURST 2`), `AxSIZE`
  (`1_BYTE_PER_TRANSFER 0` … ), `AxDOMAIN`, `AxMMUFLOW`, `AxMMUSECSID`, `Security`, `Storage_CMO`,
  `AXI_Transport`; on ADIv6, `Size` (`BYTE__8_BITS 0`, `HALFWORD__16_BITS 1`, …).

  **But it also widens the `.5.vii` defect, on the documents where it is measurable.** `Memory`
  (members `0B0000`…`0B1110___0B0110`, values `0..11`) and `Check_Type` (one member `Y`) on AXI, and
  `AP`/`Activation`/`Turnaround` on ADIv6, promote **encoding literals as member NAMES with positional
  values**; `Target` emits `OK` three times. That is exactly `.5.vii`'s class.

  **Which reverses the sequencing `.5.v` and `.5.vi` set.** All **54** enums `.5.vi` predicted this
  rule would clean live in **legacy schema-1 chains**, which cannot be rebuilt without re-ingest — so
  the BENEFIT is unrealised today, while the COST lands on three current chains, one of them a scored
  wire gold. **`.5.vii` must land first, or the two must land together**; shipping this alone makes the
  promoted symbol surface measurably worse exactly where it can be measured.

  **Verification state at revert**, recorded so the next attempt does not re-derive it: `kg-bench`
  156/156 green; APB (`seed_apb`) and AHB (`seed_ahb`) score `P=R=F1=1.000` unchanged, which is
  expected because both chains are byte-identical under the patch; **the AXI and SWD seeds REFUSE** —
  `EvidenceIR proof verification failed: registered derivation 'evidence.claim.schema_version.root'
  output or input topology is stale` — because a producer change stales the persisted proof seal
  ([[evidence-rule-field-content-stales-every-proof]]). The AXI gold therefore cannot be scored
  without rebuilding its chain, and `scripts/rebuild_stage_cascade.sh` **stops at any content delta by
  design and never absorbs one**, so a deliberate gold refresh is a governed manual protocol, not a
  scripted step. That protocol, not the patch, is the remaining work.

  **LANDED `2026-09-21`, and the trial's blocker is GONE because `.5.vii.a` removed it.** The trial
  refused to ship because the rule *"widens the `.5.vii` defect on the documents where it is
  measurable"* — `Memory`, `Check_Type`, `Target` on AXI and `AP`/`Activation`/`Turnaround` on ADIv6
  promoting encoding literals as member NAMES with POSITIONAL values. `.5.vii.a` deleted the
  positional fallback, so those members no longer exist to be promoted. **Re-measured, not assumed**
  — replay over all 27 rebuildable chains, old binary versus new:

  | document | statements | added | removed | non-`Enum` change |
  | --- | ---: | ---: | ---: | ---: |
  | `ihi0022_l` (AXI wire gold) | 6,422 -> 6,459 | **37** | **0** | **0** |
  | `ihi0074_a` (ADIv6 / SWD gold) | 6,647 -> 6,650 | **3** | **0** | **0** |
  | the other 25 | unchanged | 0 | 0 | 0 |

  `usb_3_2` changed in the trial and does **not** change now: all 6 of its enums were the positional
  class. Every other EvidenceIR surface — `register_records`, `timing_constraints`,
  `signal_constraints`, `conditional_rules`, `actor_signal_relations`, `signal_polarities`,
  `protocol_states` — is byte-identical on both documents.

  **WHY A NAME GATE ADDS MEMBERS, root-caused rather than observed.** `infer_encoding_column_indices`
  takes the enum NAME as an argument and matches it against the header, so the name decides
  `value_col` and therefore `name_col`. Under the label the AXI `AWSNOOP` encodings table
  (`AWSNOOP | AWDOMAIN 1 | Opcode | Enable | Description`) was named `TABLE`, which matched no header
  and let `code` inside **`Opcode`** claim the value column — words, which do not parse, so
  (since `.5.vii.a`) every member was dropped. Named `AWSNOOP` the value column is the `AWSNOOP`
  column and its `0b00000` codes parse. **The label was not merely adding a junk name; it was
  capturing the naming slot and mis-pointing the value column.**

  **What the 40 additions actually are, adjudicated per item.** **26 are exactly correct**: `AxSIZE`
  (`1_BYTE_PER_TRANSFER 0` … `128_BYTES_PER_TRANSFER 7`), `AxBURST` (`FIXED 0`/`INCR 1`/`WRAP 2`),
  `AxMMUFLOW` (`STALL 0`/`ATST 1`/`NOSTALL 2`/`PRI 3`), `AxMMUSECSID`, `Security`, `AxDOMAIN`, and
  ADIv6 `Size`. **11 are mixed** — `AWSNOOP` (7) and `ARSNOOP` (4) carry document-STATED values with
  member names taken from the `Description` column, because `infer_encoding_column_indices`'s
  name-word list has `description` but not `opcode`, so a prose column outranks the symbolic one.
  **3 more are the same table's repeated codes** (`AWSNOOP` genuinely maps `AWSNOOP` x `AWDOMAIN` to
  an opcode, so one code appears on several rows). That defect is `infer_encoding_column_indices`'s,
  not this gate's — it is visible on these tables only because this gate stopped hiding them — and it
  is opened as `.5.xiii` with its own measured population.
  Verification: **WIRE-BASED-100 before == after, PROVEN by rebuilding all four gold chains with BOTH
  binaries** and scoring all five seeds — `seed_axi`, `seed_axi_temporal`, `seed_ahb`, `seed_apb` and
  `seed_swd` — with `eval-extraction --provider skip`: the two reports are **byte-identical**, so no
  scored figure moves, including the unchanged pre-existing `drives` recall (0.333 AHB/APB, 0.000
  AXI/SWD). **This is the leg the trial could not run**, and the protocol that earns it is
  rebuild-then-validate, per stage, in order: validation back-annotates, so a stage built before its
  upstream is validated fails with `cumulative proof ledger does not retain the exact verified
  upstream prefix`. `kg-bench` **156/156**.

- ID: `KG-ISF-COMPLETENESS.5.vii` · Status: `done` (`2026-09-21`; MEASUREMENT, no code — the
  adjudication is the deliverable; it decides the direction and opens `.5.vii.a` and `.5.viii`) · Goal:
  **a value the document never states should not look like one it did.** 160 minted enums carry values
  that are exactly `0..n-1` and appear in no column of their source table, so they came from `row_idx`;
  135 of them reach `IntentIR`. Acceptance: either the positional fallback stops minting a literal —
  emitting an honest residual instead, which is this programme's standing preference over fabricated
  certainty — or the census shows the fallback is right often enough to keep, with the number that says
  so. `.5.v` warns that a positional gate firing on a real code table would be worse than the residual,
  so the measurement decides the direction, not the intuition.
  Prerequisite: `.5.v`. **Sequencing REVERSED `2026-09-21` by `.5.vi.a`'s trial run**, which measured
  what `.5.v` could only reason about: the identity gate is additive as well as subtractive, and on
  the three current chains it changes it ADDS positional-value and literal-as-member enums (`Memory`,
  `Check_Type`, `AP`, `Activation`, `Turnaround`, `Compliance`) — this leaf's own class. Its cleaning
  effect, by contrast, falls entirely on legacy chains that cannot be rebuilt. **So `.5.vii` goes
  FIRST, or lands together with `.5.vi.a`**; the earlier ordering assumed a denominator change that
  the measurement does not support.
  Verification: the same census, re-run, with the positional count and the promoted-symbol count both
  re-derived.

  **MEASURED `2026-09-21`** — `scripts/adjudicate_positional_enum_values.py`, read-only over all 78
  persisted chains; report `docs/research/enum-value-provenance.md`. The instrument REPLAYS the
  producer per table carrying one extra bit per member — whether `parse_encoding_numeric_literal`
  returned a value or `row_idx` was substituted — and reuses `.5.vi`'s attribution to pin each minted
  member to the table the producer walked. **It partitions the population identically to `.5.vi`**
  (543 attributed per member = `.5.vi`'s 498 ordered + 45 multipass; 7 declined = its 7 set-cover,
  declined because a cover names tables but not which produced WHICH member), and re-derives `.5.v`'s
  512 promoted symbols.

  **`.5.v`'s 160 was a floor, and its instrument could not have found the rest.** The shape test
  (`0..n-1`, absent from every column) misses every enum that MIXES stated and fabricated values, and
  every enum whose ordinals have gaps — `row_idx` indexes `body_rows` while the `.5.ii`/`.5.iii` gates
  SKIP rows. Replaying the producer counts **10,189 of 19,023 attributed values — 53.6% — never
  stated by the document.**

  | per enum | enums | members | fabricated | promoted |
  | --- | ---: | ---: | ---: | ---: |
  | every value fabricated | **228** | 2,694 | 2,694 | 207 |
  | some values fabricated | **218** | 15,282 | 7,495 | 214 |
  | every value document-stated | **97** | 1,047 | 0 | 84 |

  **Only 84 of the 512 `symbol_definitions` reaching `IntentIR` (16%) carry no fabricated value.**

  **The fairness test `.5.v` demanded is answered against the fallback.** 9,631 of 10,189 (94.5%)
  fabricated numbers appear NOWHERE in their own source row; 66 (0.6%) are the one structurally
  defensible case (a bracketed index equal to the ordinal, `HPROT[3]` at row 3) and even there the
  number is right while the KIND is wrong. **5,688 slots in 143 enums are provably wrong** — the
  ordinal equals a value the same enum states elsewhere, and no encoding assigns one code twice.

  **The dominant cause is a header word, not a missing column.** 61.3% of fabrications come from a
  column a header word MATCHED; `infer_encoding_column_indices` accepts the substring `bit`, which
  matches `Bits` — the `31:16` column of a register field-LAYOUT table (41.3% of that class). Those
  cells never parse, so every field NAME in such a table is published as an enum member valued by its
  ordinal.

  **TWO CORRECTIONS, recorded because they change published conclusions.** (a) **`.5.vi`'s two
  protected `stated_continuation` enums are both corrupt.** `SYSCOACK` is
  `DISABLED=0 CONNECT=0 ENABLED=1 DISCONNECT=1` — every value duplicated, because `row_idx` is
  table-local and each fragment was numbered from zero; `ARDOMAIN`'s members are whole description
  sentences with ordinal values. A page-break continuation is the one case where this fallback CANNOT
  be right, and `.5.vi` selected those two as the population to protect without reading their content.
  **Identity being legitimate says nothing about content.** (b) **357 fabrications (3.5%, 3 OpenCAPI
  documents) are not residuals at all** — the document states `'1110'` and the parser trims only `[`
  and `]`, so the quotes defeat it; the repair is to READ the value. Split out as `.5.viii`.

  **Negative control, run because `.5.vi` was blind behind a passing vacuity check:** on the same gold
  documents the instrument reports ZERO fabricated values for `BRESP` (8), `ARSNOOP` (4), `AWCMO` (2),
  `BTAGMATCH` (9), `HTRANS` (4), `HSIZE` (6), `HRESP` (2) and APB `PPROT` (4) — including the enums
  `.5.ii`/`.5.iii` were written to repair. **DECISION: GO on removing the fallback.** KM
  `[[minted-enum-values-are-mostly-row-indices]]`. **Frontier → `.5.vii.a` (CODE).**

- ID: `KG-ISF-COMPLETENESS.5.vii.a` · Status: `done` (`2026-09-21`; CODE; LANDED with the four
  affected chains rebuilt and the full high-stakes protocol green) · Goal:
  **a member whose value the document does not state is not minted.** Replace
  `.unwrap_or(row_idx as u32)` in `synthesize_encoding_declarations_for_enum`
  (`crates/specforge/src/ir/evidence.rs`) with a `continue`, so a row whose value cell does not parse
  emits no statement — the honest residual this programme prefers over fabricated certainty, and the
  same shape the `.5.ii` spine gate and `.5.iii` `_WIDTH` gate already use at this seam.
  Acceptance: **CORRECTED BEFORE IMPLEMENTING — `.5.vii`'s 10,189 is what is ON DISK, not what a
  landing can show.** 10,137 of those values (99.5%) are in the **51 legacy chains that cannot be
  rebuilt without re-ingest**; only **52 across 4 documents** are in the 27-document rebuildable
  stratum, so a replay can only ever observe those. An acceptance bar of 10,189 would have been
  unverifiable by construction — the same stratum trap that reversed `.5.vi.a`'s sequencing, caught
  here before the code was written rather than after. The bar is therefore the measurable set: the 4
  documents, and no other document or enum changing.
  **The rule is purely subtractive at the member seam** — it can only skip a row, so it cannot rename
  an enum, change which declared signal wins, or mint one. **That argument is why it is safe to
  attempt and is NOT evidence**: `.5.vi.a` was also reasoned subtractive, was measured additive, and
  was reverted. Prove it by replay.
  This is byte-changing on scored documents, so it carries the full high-stakes protocol `.5.i`,
  `.5.iv.a` and `.5.vi.a` ran under: before/after WIRE-BASED-100 on rebuilt gold evidence, `kg-bench`
  156/156, a corpus-wide old-versus-new replay naming every changed artifact, and FSMGen
  `--strict --check` on every changed `.isf`. **`.5.vi.a`'s blocked verification leg applies here
  unchanged**: a producer change stales the persisted proof seal, so the AXI and SWD seeds refuse
  `eval-extraction` and the gold refresh is a governed manual protocol, not a scripted step.
  Non-goal: re-pointing `value_col` away from a `bit`-headed layout column — real, and the largest
  single cause, but it moves `name_col` and therefore the members, so it is not subtractive and must
  be measured on its own; removing the fallback subsumes its symptom meanwhile. Also out: the NAME
  gates, the conflation seam (`.5.vi.a`), and the quoted-literal parser gap (`.5.viii`).
  Prerequisite: `.5.vii`.
  Verification: re-run `scripts/adjudicate_positional_enum_values.py` and
  `scripts/measure_enum_member_value_fidelity.py` on rebuilt chains and show the fabricated-value and
  promoted-symbol rows moving by the predicted amount and nothing else.

  **LANDED `2026-09-21`.** `synthesize_encoding_declarations_for_enum` no longer substitutes
  `row_idx`; a row whose value cell does not parse is skipped, and the `.enumerate()` that fed the
  fallback is gone with it. +3 tests.

  **MEASURED 54, PREDICTED 52 — and the 2-statement gap is an instrument correction, not a rounding.**
  Old-versus-new statement sets over all 4 documents: **54 removed, 0 added, and every one of the 54
  is an `Enum` statement.** AXI `ihi0022_l` 6,451 -> 6,422 (29), AHB `ihi0033_c` 1,320 -> 1,309 (11),
  ADIv6 `ihi0074_a` 6,657 -> 6,647 (10), Wishbone `wbspec_b4` 1,530 -> 1,526 (4). The extra 2 are AHB
  `Enum HREADYOUT 0 = 0.` and `Enum HREADYOUT 1 = 1.` — **an enum whose MEMBERS are bare digits**, so
  its `(member, value)` pairs are producible by **seven** different tables in that document and the
  census's ordered walk attributed them to `table_0022`, where those cells parse, instead of
  `table_0024`, where they do not. Measured corpus-wide, **601 member slots (2.8%) in 159 enums** have
  at least two candidate tables disagreeing on parsed-versus-ordinal. **The error direction is an
  UNDER-count, so `.5.vii`'s 10,189 is a floor**, and the reproducer now reports this residual itself
  rather than leaving the next reader to find it.

  **The rule is measured SUBTRACTIVE, not argued subtractive** — `0` statements added anywhere, and
  `0` non-`Enum` statements removed. `.5.vi.a` was reverted for exactly the failure of reasoning that
  a member-seam rule "can only remove"; this leaf did not rely on that argument.

  **NO REGRESSION — the full protocol, including the leg `.5.vi.a` could not run.** `check_chain_currency.sh`
  **27 replayed / 27 current / 0 stale at evidence, semantic, intent AND isf-adapter** (51 legacy
  UNMEASURABLE), after rebuilding the 4 affected chains stage-by-stage with `validate` between stages —
  the ordering matters, because validation back-annotates and a stage built before its upstream is
  validated fails with `cumulative proof ledger does not retain the exact verified upstream prefix`.
  **WIRE-BASED-100 before == after, PROVEN by rebuilding all 4 chains with BOTH binaries** and scoring
  `eval-extraction --provider skip`: `seed_axi` constraint 4/4 + relation 6/6 nested, `seed_axi_temporal`
  3/3, `seed_ahb` 6/6 + 6/6 nested, `seed_apb` 6/6 + 5/5 nested — every figure identical old-versus-new,
  including the unchanged pre-existing `drives` recall of 0.333 on AHB/APB and 0.000 on AXI. **That
  eval REFUSED during `.5.vi.a`'s trial** (`EvidenceIR proof verification failed`); a proper
  rebuild-plus-validate re-earns the seal, which is the concrete protocol that leaf was missing.
  `kg-bench` **156/156**. `cargo fmt` clean, `cargo clippy --offline --all-targets -D warnings` clean,
  `cargo test --workspace --lib --exclude specforge-production-graph` green (specforge 473, conformance
  168, core 1,594). FSMGen `--strict` is **vacuous by construction, not passed**: the adapter emits
  **0 `.isf` files** across all 27 documents, every one blocked on `no source-grounded system
  clock/reset contract`, so there is nothing to check — stated rather than ticked.

  **PRODUCT EFFECT, re-derived on the rebuilt chains.** The census moves 550 -> 545 minted enums across
  52 -> 51 documents, attributed members 19,023 -> 18,969 (**-54, exactly the statement count**),
  fabricated values 10,189 -> 10,139, every-value-fabricated enums 228 -> 224, and
  every-value-document-stated enums **97 -> 100**. Five enums disappear entirely — AXI `AWBURST`,
  `AWDOMAIN`, `ARDOMAIN`, Wishbone `Nomenclature`, AHB `HREADYOUT` — of which `AWDOMAIN` and `ARDOMAIN`
  were promoted `symbol_definitions`, so **2 promoted symbols are withdrawn and 4 more now carry only
  document-stated values** (promoted-from-fully-stated 84 -> 88). Every withdrawn symbol was
  adjudicated junk per-item in `.5.vii`: `AWBURST` was members named after ADDRESSES with ordinal
  values and `0X00` appearing three times, and `ARDOMAIN`'s members were whole description sentences.

- ID: `KG-ISF-COMPLETENESS.5.vii.b` · Status: `done` (`2026-09-21`; MEASUREMENT + correction, no
  Rust; opened and closed by the same slice, because the defect is in what `.5.vii` PUBLISHED) ·
  Goal: **`.5.vii`'s headline is a fact about the corpus on disk, and it was written as a fact about
  the producer.** `CORPUS-CHAIN-CURRENCY.11` had already set the rule — *a persisted legacy
  `evidence_ir.json` may be cited as evidence about ITSELF, never about the reader* — and
  deliberately did NOT gate it, because "a census reading a legacy artifact" is a property of intent
  rather than of a file. `.5.vii` is exactly the case it left ungated.

  **Measured `2026-09-21`, and the split is not close.**

  | stratum | documents | minted enums | members |
  | --- | ---: | ---: | ---: |
  | replayable — the CURRENT producer | 27 | **15** | **79** |
  | legacy — artifact-only evidence | 51 | **530** | **21,459** |

  **99.6% of the member surface this programme has been measuring was minted by binaries that no
  longer exist.** `.5.v`'s 550 enums / 21,592 members, `.5.vi`'s 550-enum adjudication and `.5.vii`'s
  10,189 fabricated values are all legitimate statements about the ARTIFACTS, and none of them is a
  statement about what SpecForge does today. `.5.vii` was written in the producer voice
  ("SpecForge picks a value column … falls back to the row's position"), which is the error.

  **What is NOT withdrawn, stated precisely so the correction is not read as a retraction.** The
  producer defect was real and is proven independently of the legacy stratum: on the replayable
  documents alone the fallback minted **52 fabricated values in 20 enums across 4 documents**, and
  `.5.vii.a` removed **54 statements with 0 added and 0 non-`Enum` removed**, verified by
  old-versus-new replay with both binaries. The rule change stands on that evidence. What is
  corrected is the SCALE the headline implied.

  **A post-landing control the fix itself created.** After `.5.vii.a` the producer cannot mint a
  fabricated value at all, so the census must read **0** on the replayable stratum. It reads **2** —
  ADIv6 `In` members `DPIDR1` and `BASEPTR0`, each with exactly 2 candidate tables. Both are the
  bare-ambiguity class `.5.vii.a` identified, so the instrument's false-positive rate on that stratum
  is **2 of 79 members (2.5%)**, which agrees with the independently derived 3.0% ambiguity share.
  **Two differently built estimates of the same instrument defect agree**, which is what makes the
  residual a known bound rather than an unknown.
  Non-goal: re-caveating every `.5` record (the same caveat applies to `.5.v` and `.5.vi`, and their
  cards now say so, but auditing the whole programme's published numbers is its own slice);
  re-ingesting the 51 (`CORPUS-COVERAGE` owns that, and `CORPUS-CHAIN-CURRENCY.11`'s non-goals
  forbid it here); gating the rule, which `.11` already decided against for a reason that still holds.
  Prerequisite: `.5.vii.a`.
  Verification: `scripts/adjudicate_positional_enum_values.py` now prints the stratum split above
  every total it reports, and names `CORPUS-CHAIN-CURRENCY.11` while doing it.

- ID: `KG-ISF-COMPLETENESS.5.viii` · Status: `done` (`2026-09-21`; CODE; opened by `.5.vii`) · Goal:
  **a value the document DOES state must not be read as unstated.** `parse_encoding_numeric_literal`
  (`crates/specforge/src/ir/evidence.rs`) trims `[` and `]` and nothing else, so a quoted binary
  literal — `'1110'`, the convention in the three OpenCAPI transaction-layer specifications — fails
  to parse and the member is dropped (it was published as the row's ordinal until `.5.vii.a`).
  Re-derived: **357 of the 10,139 fabricated values, 3.5%, across 3 documents** (129 / 123 / 105).

  **THE PINNED PREMISE WAS FALSE, AND IT INVERTS THE FIX.** `.5.vii` wrote *"a bare `'1110'` already
  parses as binary under the all-`0`/`1` arm, so stripping the wrapper is sufficient and introduces
  no new radix rule."* It does not. The reader tries `parse::<u32>()` **first**, so the all-`0`/`1`
  arm is reached only when the decimal parse OVERFLOWS `u32` — **never below eleven digits**. Proven
  against the exact function body: `'0010'` → 10 (not 2), `'1110'` → 1110 (not 14), `'1111'` → 1111
  (not 15). **Scored over all 357 slots, the prescribed repair is wrong on 314 (88.0%), and on 57 it
  is strictly WORSE than the fabricated ordinal it replaces** — the ordinal is accidentally right for
  100 slots and stripping the quote breaks 57 of them.

  **THE STATED ACCEPTANCE BAR COULD NOT HAVE CAUGHT THAT.** `.5.vii` wrote it as *"the quote-wrapped
  count must go to 0 and the fabricated total fall by exactly 357"* — a COUNT. The correct repair and
  the 88%-wrong one satisfy it identically. **When a slice changes what a value IS, the oracle must
  compare values.** Bar corrected before implementing, as `.5.vii.a` corrected its stratum.

  **THE RADIX IS THE DOCUMENT'S OWN.** All three documents state the convention verbatim: *"Binary
  values in sentences are shown in single quotation marks. For example '1010'."* and *"Hexadecimal
  values are preceded by an x and enclosed in single quotation marks. For example x'0A00'."* The
  wrapper CARRIES the radix, which is why stripping it discards information. Landed
  `parse_encoding_value_literal`: `'<bits>'` → binary, `x'<hex>'` → hex, digit grouping removed
  (`'0000 0000'`, `x'FF 00'`), and every quote-wrapped cell that is not one literal declined — ranges
  `'0011' - '1110'`, multi-value `'0010 0000' '0010 0100'`, don't-care `'0000 000x'`, quoted words
  `'IVRS'` — which stay `.5.vii.a`'s honest residual. Both halves of one legend, so both landed
  together (`x'...'`: 353 cells / 5 documents). ADR 0006: notation grammar, no document key.

  **HAZARD HONOURED, AND IT WAS WIDER THAN PINNED.** The pin named `looks_like_encoding_literal`
  only. `parse_encoding_numeric_literal` has a **second** caller that MINTS:
  `derive_header_sourced_enum_name` clause 5 asks whether any value cell parses, so a table that
  starts parsing starts NAMING an enum — enum-IDENTITY-additive, the exact trap `.5.vi.a` was
  reverted for. The reading is therefore confined to the member-VALUE seam and the shared reader is
  byte-identical; a unit test pins both seams against drift. Clause 5's reach measured at **3 tables
  in 1 document** and handed to `.5.x`.
  Verification: **all 27 replayable chains replay byte-identically** (`evidence --dry-run` over every
  persisted `source_ir.json`; 27 replayed / 51 refused as schema-1; 27/27 `cmp` identical), which is
  the correct result — the 357 are all LEGACY and the 306 quoted cells the replayable stratum carries
  sit in `register_map` **Offset** columns and unclassified tables, never an encoding value column.
  **Negative control: 0 quote-wrapped cells parse today**, so no published value can change.
  Cross-checked the tracked reproducer's Python mirror against the Rust extracted verbatim from
  `evidence.rs` over **7,151 distinct corpus cells** — 0 mismatches in the quoted branch.
  Commit: `KG-ISF-COMPLETENESS.5.viii — the quote states the radix, and stripping it is the wrong fix`

- ID: `KG-ISF-COMPLETENESS.5.x` · Status: `open` (`2026-09-21`; CODE; opened by `.5.viii`) · Goal:
  **decide whether `derive_header_sourced_enum_name` clause 5 should read the quoted notation too.**
  `.5.viii` deliberately left it on the narrow reader, because clause 5 does not recover a value — it
  decides whether a table NAMES an enum at all, so widening it MINTS. Measured reach: **3 tables in 1
  document** (`opencapi_4_0_transactionlayer_arch_16jun2020`, all header field `cmd_flag`, tables
  `table_0097`/`table_0098`/`table_0117`), all legacy. The question is the `.5.vi`/`.5.vi.a` identity
  question, not the `.5.vii` value question: a header-sourced name merges BY NAME in
  `build_symbol_definitions`, so three tables sharing `cmd_flag` would fuse. Adjudicate whether they
  are one field before widening anything.
  Non-goal: changing any value `.5.viii` already reads; re-ingesting.
  Prerequisite: `.5.viii`. Verification: the 3 tables read by hand against the document, then the
  `.5.vi` conflation test applied to the merged candidate.

- ID: `KG-ISF-COMPLETENESS.5.xi` · Status: `open` (`2026-09-21`; MEASUREMENT then CODE; opened by
  `.5.viii`) · Goal: **the decimal-before-binary ordering is a defect for UNQUOTED cells too, and it
  reaches the current producer.** `parse_encoding_numeric_literal` tries `parse::<u32>()` before its
  all-`0`/`1` arm, so a bare `0010` reads as **10** and `10` reads as **ten**; the binary arm fires
  only above ten digits, making the radix a function of the string's LENGTH with a discontinuity at
  11 characters. Measured corpus-wide: **230 cells whose leading zero makes decimal impossible** (2
  replayable) and **943 without one** (156 replayable), the second class genuinely ambiguous in
  isolation. **This is worse than a fabrication**: a fabricated ordinal is at least flagged by
  `.5.vii`'s census, whereas this publishes a number that LOOKS document-stated. The leading-zero
  class is unambiguous and landable; the bare class needs a column-level discriminator (are the
  column's cells fixed-width? does a sibling carry a leading zero?) and its own measurement first.
  Non-goal: the quoted notation (`.5.viii`, landed); reordering the arms globally without measuring
  the decimal-intending columns that ordering currently serves.
  Prerequisite: `.5.viii`. Verification: `evidence --dry-run` before/after over the 27 replayable
  chains, per-value diff, plus WIRE-BASED-100 — the wire docs use `0b`/`2'b` notation, so a move
  there would be the alarm.

- ID: `KG-ISF-COMPLETENESS.5.xii` · Status: `open` (`2026-09-21`; CODE; opened by `.5.viii`) · Goal:
  **`u32::from_str_radix` rejects digit grouping, so the producer refuses literals it recognises.**
  `0x01_0101` and `0b0101_0000` reach the `0x`/`0b` arms and then fail to convert, so the member is
  dropped although the notation is the one the reader claims to support. Measured **77 distinct
  cells, 74 legacy / 3 replayable**. `.5.viii` already removes grouping inside the quoted forms; this
  is the same rule for the prefixed ones. **It also carries an instrument correction**: the Python
  mirror in `scripts/adjudicate_conflated_enum_identity.py` accepts these (`int()` tolerates `_`),
  so the census UNDER-reports fabrication for exactly these cells — found by cross-checking the
  mirror against the extracted Rust over 7,151 distinct corpus cells, and the mirror must be fixed in
  the same slice or the census will disagree with the producer in the other direction.
  Non-goal: any other radix rule.
  Prerequisite: none (disjoint from `.5.viii`/`.5.xi`). Verification: the 7,151-cell mirror/Rust
  cross-check returns 0 mismatches, and `evidence --dry-run` over the 27 chains moves exactly the 3
  replayable cells.

- ID: `KG-ISF-COMPLETENESS.5.xiii` · Status: `open` (`2026-09-21`; MEASUREMENT then CODE; opened by
  `.5.vi.a`) · Goal: **a prose `Description` column outranks the symbolic column that actually names
  the member.** `infer_encoding_column_indices` picks `name_col` as the FIRST header matching
  `name|meaning|description|state|transfer|response|type`, so AXI `Table A7.3`
  (`AWSNOOP | AWDOMAIN 1 | Opcode | Enable | Description`) names its members from `Description` and
  emits `AWSNOOP ACT_WRITE_ACCESS__INDICATED_BY_AWACTV_ASSERTED = 0` where the document's own member
  name is the `Opcode` cell (`WriteACT`). The values are right; the names are the wrong column.
  Measured **upper bound: 262 enum-minting tables across 34 documents** whose `name_col` is a
  description/meaning column while another column holds short, single-token, mostly-DISTINCT cells —
  the shape a member name has (`error mnemonic | encoding | … | description` is the clean case). It
  is an upper bound because the shape test alone cannot tell a member-name column from an incidental
  one, which is why adjudication is this leaf's first step and not its conclusion.
  **The fix must be content SHAPE, not a wider word list** (ADR 0006, and the `.1c` probe's rule that
  a name-shape-only decision must be measured before it is trusted): adding `opcode`/`mnemonic` to the
  list is a name list and would not generalise.
  **Also in scope, from the same table:** a genuine multi-key table (`AWSNOOP` x `AWDOMAIN` -> opcode)
  cannot be a flat enum at all — it emits one code on several members — so the adjudication must
  decide whether such a table mints a residual instead.
  Non-goal: the caption gate (`.5.vi.a`, landed); `value_col` inference.
  Prerequisite: `.5.vi.a`. Verification: adjudicate a sample of the 262 by hand against the documents,
  then `evidence --dry-run` before/after over the 27 replayable chains with a per-member diff, and
  WIRE-BASED-100 on rebuilt gold chains with both binaries.

- ID: `KG-ISF-COMPLETENESS.5.ix` · Status: `done` (`2026-09-21`; DECISION — **taken, not routed**;
  the premise it was opened on was wrong and re-deriving it reversed the answer) · Goal: decide
  whether the `.5` programme should continue, given that its verification surface is 15 enums.

  **THE PREMISE WAS FALSE, AND IT IS THE WHOLE DECISION.** This leaf was opened claiming the retained
  normalized-bundle set is *"deliberately frozen at 27 documents
  (`RETAINED-BUNDLE-POPULATION-FROZEN`, closed `2026-09-19`)"*. **That tree exists because the freeze
  was a DEFECT, and it removed it.** Its stated goal is *"make `retained_bundles.json` movable in the
  two directions ADR 0025 already mandates"*; `.1` retired the `len(retained_ids) != 24` literal and
  the `reclamations != []` clause from both contract validators on `2026-09-19`. **The set has since
  grown 24 -> 27**, which is the live count in `doctrine/chain_currency/retained_bundles.json` today.
  The title was read as the finding when it is the problem statement. **Read the Goal, not the
  title, before citing a tree as a constraint.**

  **What follows from the corrected premise.** Replayability is not a fixed ceiling — it grows with
  each refresh, and `CORPUS-COVERAGE` runs that refresh programme continuously (52 of a 57-document
  cohort done, 5 remaining). So a `.5` gate is not stranded by the small surface. **A refresh bakes
  in whatever gates exist at the moment it runs**: every gate landed now is inherited free by all 51
  future refreshes, and every gate NOT landed is a defect re-imported at full cost.

  **The measurement that makes this concrete rather than a principle.** `CORPUS-COVERAGE`'s own
  frontier names its next refresh target: **`opencapi_3_0_transaction_layer_28jan2020`**. That
  document carries **318 fabricated enum values, 129 of them quote-recoverable** — it is one of the
  three legacy documents holding the whole of `.5.viii`'s population (357 across the three).
  Re-derived `2026-09-21`:

  | document | fabricated | quote-recoverable (`.5.viii`) |
  | --- | ---: | ---: |
  | `opencapi_3_0_transaction_layer_28jan2020` *(next refresh)* | 318 | **129** |
  | `opencapi_3_1_transaction_layer_28jan2020` | 342 | 123 |
  | `opencapi_4_0_transactionlayer_arch_16jun2020` | 281 | 105 |

  **DECISION: CONTINUE, and sequence `.5.viii` AHEAD of that refresh.** If `.5.viii` lands first the
  refresh imports the document's real codes; if it does not, the refresh bakes 129 fabricated values
  into a freshly-current chain, and only another full refresh removes them. The cost of landing first
<!-- kg-isf-completeness-task-source-region:enum-surface-recent-leaves:end -->

## Post-migration record

- ID: `KG-ISF-COMPLETENESS.5.vi.a.1` · Status: `done` (`2026-09-21`; CORRECTION, docs-only; opened and
  closed by an audit of `.5.vi.a`'s own lockstep) · Goal: **re-pin the published figures `.5.vi.a`
  moved.** The leaf verified its own oracles — the 27-chain replay diff, `kg-bench` 156/156,
  WIRE-BASED-100 byte-identical on rebuilt golds — and every one of them passed. None of them looks at
  the surfaces that PUBLISH the census, and `.5.vi.a` moved the census: minted enums **545 -> 552**,
  attributed members **18,969 -> 19,009**, fully-stated enums **100 -> 107**. Three surfaces were left
  stating the old numbers, found by re-deriving rather than by any gate:
  1. **the book** (`pipeline/isf-adapter.md`) said in the PRESENT tense that the current pipeline mints
     *"15 enums and 79 members"* and the corpus holds *"545 enums / 21,538 members"*. Re-derived:
     **22 enums / 119 members** replayable, **552 / 21,578** total. It understated by 47% the very
     improvement this leaf shipped, in the owner's only window.
  2. **`[[minted-enum-values-are-mostly-row-indices]]`'s `reverify`** declared 4 of its 8 expected
     values as they stood before this leaf, so the next session running it would have read a
     regression that had not happened. That is precisely the stale-detection leg
     `CLAIM_VERIFICATION.md` requires, defeated by its own owner.
  3. **`docs/research/enum-value-provenance.md`**'s notation table listed binary as 450 cells when the
     landed reader handles **455** — it omitted the 5 grouped-literal cells (`'0000 0000'`), which the
     implementation reads and a unit test pins, so the three rows summed to 824 of 829 quoted cells.
  **THE RULE THIS EARNS, stated for the next producer change: a change that moves a CENSUS must
  re-pin every surface that publishes that census, not only the surfaces the change edited.** A slice's
  own acceptance oracles answer *"did I break the product?"*; they never answer *"did I falsify a
  number someone else published?"* `.5.viii` had the same exposure and escaped it only because its
  replay was byte-identical, which is luck, not method.
  Non-goal: re-opening `.5.vi.a`'s verdict — every measured claim in it re-derives unchanged
  (+40 statements, 2 documents, 0 removed, 26 of 40 exactly correct).
  Verification: `scripts/adjudicate_positional_enum_values.py` re-run — 552 / 545 / 7 / 19,009 /
  10,139 / 224 / 214 / 107, matching the card's re-pinned `reverify` literally; the 829 quoted cells
  tile as 450 + 5 + 353 + 21; `.5.xi` (1,173 / 158), `.5.xii` (77, 0 in the quoted branch),
  `.5.xiii` (262 / 34) and the 357-of-10,139 headline all re-derive UNCHANGED.

- ID: `KG-ISF-COMPLETENESS.5.xi` · Status: `done` (`2026-09-22`; MEASUREMENT — the adjudication is the
  deliverable and it REVERSED the premise the leaf was opened on; it opens `.5.xi.a`) · Goal: decide
  the radix of an UNQUOTED all-`0`/`1` encoding cell on a measurement rather than on the arm ordering.

  **THE OPENING PREMISE WAS WRONG IN ITS CONCLUSION, NOT ITS COUNTS.** `.5.xi` was opened on a
  corpus-wide CELL census — 230 leading-zero (2 replayable) / 943 bare (156 replayable) — and inferred
  that it *"reaches the current producer in quantity"* and is the only one of the four open defects
  that does. Both counts re-derive exactly. **A cell is published only if it lands in the `value_col`
  of a table that MINTS**, and nearly every all-`0`/`1` cell in this corpus is a bit position, bit
  range or slot number. Replaying the producer and attributing each minted member to its row gives
  **336 minted member values, 0 of them replayable** — and the attribution-free upper bound is 0 too,
  so it is not an artefact of the walk. The replayable stratum states its codes as `0b`/`0x` literals
  (94 + 4 of 126 matches) or single digits (28), which are radix-invariant. **Count the population the
  SEAM reads, not the population the corpus holds.**

  **The producer trims `[`/`]` and the opening census did not** — 67 further cells, and they are bit
  POSITIONS, i.e. the decimal-intending class the repair must leave alone. Corrected population:
  **1,240 cells** (230 / 1,010), still 158 replayable.

  **THE DISCRIMINATOR IS A NOTATIONAL WITNESS, AND IT MAY BE IN THE ROW.** Content shape only, no
  header vocabulary (ADR 0006): a zero-padded all-`0`/`1` sibling in the same column **at the same
  width** or in the same **row** carries a bare cell to binary; a same-width digit ≥ 2 refutes it
  (zero-padded decimal) and outranks it; absent a witness, any digit ≥ 2 in the column makes it decimal
  (`0 1 2 … 11` is a decimal RUN whose `10`/`11` are all-`0`/`1` by coincidence). Verdicts over the
  336: **198 binary / 133 decimal / 0 zero-padded-decimal / 5 undecidable**. Width stratification is
  load-bearing — `opencapi_4_0` `table_0092` fuses a width-1 bit-index table with a width-4 encode
  table in ONE column — and the row witness is **17 of the 22** slots a column-only rule cannot decide
  (Cortex-A76 `op0|op1|CRn|CRm|op2` = `11|011|0100|0100|000`). All 18 contestable calls and all 5
  residual slots were read by hand against the source documents; the 5 are genuinely decimal, so the
  residual costs nothing.

  **UNLIKE `.5.viii`, THIS REPAIR CANNOT REACH THE IDENTITY SEAMS, AND THAT IS PROVED NOT PLACED.**
  For an all-`0`/`1` string the decimal arm succeeds only at ten digits or fewer, where the binary
  reading is at most 1023; above ten digits the decimal arm already fails and the binary arm already
  runs. The set of cells that PARSE is identical under either ordering — only the value changes — so
  `derive_header_sourced_enum_name` clause 5, `infer_encoding_column_indices` and
  `looks_like_encoding_literal` see exactly what they see today. Checked over all **71,001** distinct
  trimmed corpus cells: **0** whose parse success would change.
  Non-goal: the quoted notation (`.5.viii`, landed); landing the reader change (`.5.xi.a`).
  Verification: `python3 scripts/adjudicate_decimal_before_binary_encoding_values.py --residual`
  re-derives 230 / 943 / 1,173 / 158 (the opening census), 230 / 1,010 / 1,240 trimmed, 336 / 0,
  198 / 133 / 0 / 5, and 0 of 71,001. WIRE-BASED-100 exposure is **0 slots** — the wire golds use
  `0b`/`2'b`. Report: `docs/research/decimal-before-binary-encoding-values.md`.
  Commit: `KG-ISF-COMPLETENESS.5.xi — 158 replayable cells, and not one of them reaches a value column`

- ID: `KG-ISF-COMPLETENESS.5.xi.a` · Status: `open` (`2026-09-22`; CODE; opened by `.5.xi`) · Goal:
  **read an unquoted all-`0`/`1` value cell in the radix its own table states.** Land `.5.xi`'s
  witness rule at the member-VALUE seam: a cell with its own leading zero, or with a same-width
  column sibling or a row sibling that has one, is binary unless the same-width stratum carries a
  digit ≥ 2; everything else keeps today's decimal reading. Expected effect: **198 of 336 legacy
  member values corrected, 0 replayable values moved** — so the oracle that matters is a per-VALUE
  diff, never a count (`.5.viii`'s standing rule), and a byte-identical replay is the PREDICTED
  result rather than a no-op.
  **The reader may be widened in place rather than duplicated**, which `.5.viii` could not do: the
  reordering is parse-success-invariant, so it is not enum-identity-additive. The unit test must pin
  that invariance, not merely the values.
  Non-goal: the quoted notation; `value_col`/`name_col` inference (`.5.xiii`); the 5 residual slots.
  Prerequisite: `.5.xi`. Verification: `evidence --dry-run` over all 27 replayable chains with a
  per-value diff (27 replayed / 51 refused as schema-1 is the correct shape), WIRE-BASED-100 = 1.000
  on rebuilt golds with both binaries, kg-bench 156/156, and the adjudicator re-run showing the
  198 slots move to their stated codes.

- ID: `KG-ISF-COMPLETENESS.5.vi.a.2` · Status: `done` (`2026-09-22`; CORRECTION, docs-only; opened and
  closed by `.5.xi`'s re-derivation of the same census) · Goal: **finish the re-pin `.5.vi.a.1` began.**
  `.5.vi.a.1` corrected the book, a fact card's `reverify` and a research table after `.5.vi.a` moved the
  census, and stated the rule: *a change that moves a census must re-pin every surface that publishes
  it.* Re-deriving that census while measuring `.5.xi` found **four more surfaces still stating the
  pre-`.5.vi.a` figure**, all in the PRESENT tense:
  1. this tree's own root — *"The current producer mints 15 enums / 79 members"*, and 99.6%;
  2. `[[minted-enum-values-are-mostly-row-indices]]` — same sentence, and internally inconsistent, since
     530 legacy + 15 replayable is 545 against the 552 its own re-pinned `reverify` expects;
  3. `[[minted-enums-are-mostly-cross-table-conflations]]` — same sentence;
  4. `docs/research/enum-value-provenance.md` — a stratum row LABELLED *"the CURRENT producer"*, whose
     row is now dated instead, since the record's measurements are correct as taken.
  **The rule needs a search, not an intention.** `.5.vi.a.1` re-pinned the surfaces it could think of;
  the three it missed are the ones no oracle points at. Re-derived `2026-09-22`: **22 enums / 119
  members** replayable, **530 / 21,459** legacy, **552 / 21,578** corpus-wide, legacy share **99.45%**.
  Non-goal: dated records inside the sealed source region, which are correct as of their dates.
  Verification: `minted_enums` counted directly over all 78 `generated/evidence_ir/*/evidence_ir.json`
  by stratum; agrees with `.5.vi.a.1`'s `adjudicate_positional_enum_values.py` totals (552 / 19,009) and
  with `.5.xi`'s independent replay (22 enums / 119 members / 126 parsing matches).
  Commit: `KG-ISF-COMPLETENESS.5.xi — 158 replayable cells, and not one of them reaches a value column`
