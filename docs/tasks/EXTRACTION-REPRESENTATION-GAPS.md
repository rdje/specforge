# EXTRACTION-REPRESENTATION-GAPS: one ledger of what a specification states, what SpecForge extracts, and what `.isf` can say — each gap owned on its own side

## Metadata

- Tree ID: `EXTRACTION-REPRESENTATION-GAPS`
- Status: `active`
- Roadmap lane: the program objective (ADR 0033, amended by ADR 0053) and `R0` downstream contract hygiene
- Created: `2026-09-25`
- Last updated: `2026-09-25`
- Owner: repo-local workflow
- Decision: [ADR 0053](../decisions/0053-extraction-and-isf-requests-advance-in-parallel.md)

## Goal

Every piece of design intent a chip specification states is **extracted** by SpecForge into canonical IR and
is **representable** in `.isf`. Where either fails, the gap is named, attributed to the side that owns it, and
owned by an open leaf: a SpecForge extraction leaf, an FSMGen request (through `FSMGEN-REQUESTS`), or both.
The owner's directive (`2026-09-25`): *SpecForge does its part and FSMGen does its part.*

The ledger is a book chapter, [`reference/representation-gaps.md`](../book/src/reference/representation-gaps.md),
so the owner can see every gap and who holds it without reading code.

## Non-Goals

- Filing a request without a reproduced gap. ADR 0053 keeps `PROCESS.md`'s evidence bar: cited
  specification text, and FSMGen's refusal reproduced on the pin and on upstream.
- Forwarding requests. The owner forwards; `FSMGEN-REQUESTS` records it.
- Fixing a SpecForge extraction gap inside this tree when an existing tree already owns that category. The
  ledger names the owner; the work stays where it belongs.
- Inspecting or changing `subs/fsmgen`. FSMGen is read only through its published book and CLI.

## Acceptance Criteria

- The ledger lists every gap the two censuses (`.2`, `.3`) find, with evidence, the owner of each half and a
  status, and a check refuses a row whose owner is not an open work unit (`.1`).
- Every FSMGen-owned row has a request bundle signed off under `PROCESS.md`, or a written reason it does not
  need one (already in FSMGen's published contract at the pin, or withdrawn).
- Every SpecForge-owned row has an open leaf in the tree that owns its category.
- The ledger is re-checked on every FSMGen pin refresh; the refresh leaf records the re-check.

## Task Tree

- ID: `EXTRACTION-REPRESENTATION-GAPS`
  Status: `active` (`2026-09-25`)
  Children: `.0`-`.16`

- ID: `EXTRACTION-REPRESENTATION-GAPS.0`
  Status: `done` (`2026-09-25`; DECISION + DOC)
  Goal: **open the lane.** Record the owner's directive as ADR 0053 (amending ADR 0033 item 4: an FSMGen
  request may precede the IntentIR carrier when an open SpecForge leaf owns the extraction); publish the
  ledger chapter seeded only with rows verified in `KG-ISF-COMPLETENESS.2a.vii.a`-`.c.iii` and against FSMGen's
  published book at the pin (13a *Interface*, 13k feature matrix rows *Actor report metadata and params*,
  *Interface ports*, *Reusable ISF libraries*); state its update rule; route it from the book summary, the
  FSMGen requests chapter and the ISF adapter chapter; add the lane to the roadmap's owner region.

- ID: `EXTRACTION-REPRESENTATION-GAPS.1`
  Status: `done` (`2026-09-25`; CHECK)
  Goal: **a ledger row cannot cite a closed or missing owner.** Declare the ledger's owner column as an
  `OWNERSHIP-CITATIONS` region. First extend `scripts/check_ownership_citations.pl`: it resolves a leaf's
  status only from the two-line `- ID:` / `Status:` form in a tree's root file, so a leaf of a partitioned tree
  (`KG-ISF-COMPLETENESS.2a.vii.c.iv`, declared as one line `- ID: … · Status: …` in a part under
  `docs/tasks/<tree>/`) reads as nonexistent. Resolve the one-line form in the root and in the tree's parts,
  with a self-test case for each form and a refused case for a closed leaf.
  **Landed.** `resolve_leaf_status` reads both grammars over the root and every part file under
  `docs/tasks/<tree in lower case>/` (`tree_part_texts`), and fails closed when a leaf's declarations disagree;
  the self-test calls it directly instead of re-stating its regex (the old leaf-shape case tested a copy), and
  gains five cases (one-line in a root, in a part, a closed one-line leaf, disagreeing declarations, a longer id
  sharing the prefix): 16/16. The ledger's open-gaps table is the `gap_owners` region, with its 10 owners
  classified `current_owner`. The limit mattered beyond this ledger: the one-line form is how every partitioned
  tree and many roots declare leaves, so any such citation in a declared region would have been refused as
  nonexistent.

- ID: `EXTRACTION-REPRESENTATION-GAPS.2`
  Status: `done` (`2026-09-25`; MEASUREMENT)
  Goal: **the IntentIR-to-`.isf` census.** For every IntentIR family, over every live chain: how many records
  the IR carries, how many the adapter lowers, and, for the rest, why — the IR lacks an operand (SpecForge), or
  FSMGen's published contract cannot state it (FSMGen; cite the section), or both. The adapter's residual packets
  are the starting evidence. Each FSMGen-owned result becomes a request leaf here; each SpecForge-owned result a
  leaf in the tree that owns the family. Every row lands in the ledger with its evidence command.
  **Measured** (`python3 scripts/measure_intent_to_isf_reach.py`, read-only; result below under *`.2` — the
  IntentIR-to-`.isf` census*): the adapter reads 18 of IntentIR's families and never reads ~20; **every one of
  the 27 live adapters is `blocked`, all 27 for want of a clock/reset contract**, so no current chain hands FSMGen an
  `.isf` file; 150 residual packets fall into 8 attributable classes. New leaves `.7`-`.16`; `.3` re-anchored.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3`
  Status: `open` (`2026-09-25`; MEASUREMENT)
  Goal: **the PDF-to-IR census: extend the closed ontology.** `docs/research/intent-ontology-coverage.md`
  (`2026-05-31`) is the recall denominator, and it has no category for configuration parameters, presence
  properties or interface-class profiles, which is why the `.2a.vii.c` gaps were found by hand. Measure, per
  document category, which kinds of design intent the live corpus states that no IR carrier holds; extend the
  ontology with each, and give each an owning SpecForge leaf. The ledger gains a row per category.
  **Re-anchored by `.2`**: the per-category denominator already exists as
  `doctrine/spec_to_intent_category_contract.json` (six categories, each with semantic families mapped to IntentIR
  surfaces; `platform-system-ip`'s `component_topology_and_domains` family maps to none), so measure against it
  and extend it rather than the older ontology. It also owns the **7,477 untyped records** `.2` found
  (`constraints` 5,229, `behaviors` 2,091, `assumptions` 157: a free-text `statement` plus provenance): count how
  many state something no typed record holds, which is captured-but-not-understood intent.

- ID: `EXTRACTION-REPRESENTATION-GAPS.4`
  Status: `open` (`2026-09-25`; REQUEST)
  Goal: **FSMGen can state an optional signal.** AXI sets 20 width parameters to `0` by default (*the signal
  is absent unless enabled*) and gates signals on yes/no properties (`PROT_Present`). FSMGen's published
  contract refuses a zero-valued parameter as a width (13a, 13k *Interface ports*) and has no optional or
  conditional port. Draft, reproduce and sign off one `feature_request` covering a zero-width (absent) port, a
  port present only when a parameter enables it, and the value an absent input is tied to (AXI: *an attached
  Subordinate must have its AWSNOOP input tied LOW* when the Manager omits it; found by `.2`). SpecForge halves: `KG-ISF-COMPLETENESS.2a.vii.c.vi` (read the
  presence properties); the adapter's positive-width substitution is replaced once the feature ships.

- ID: `EXTRACTION-REPRESENTATION-GAPS.5`
  Status: `open` (`2026-09-25`; REQUEST)
  Goal: **FSMGen can carry a parameter's legal values and recommended range.** SpecForge now carries
  `legal_values` and `recommended_values` on 36 parameter symbols (`KG-ISF-COMPLETENESS.2a.vii.c.ii`/`.c.iii`);
  FSMGen's `(params (P default))` holds a default only (13k *Actor report metadata and params*), so the rest
  stops at IntentIR, and nothing checks that an override is legal. Draft, reproduce and sign off a
  `feature_request` for a declared value set / range on an actor parameter, checked on every default and
  override.

- ID: `EXTRACTION-REPRESENTATION-GAPS.6`
  Status: `open` (`2026-09-25`; REQUEST)
  Goal: **a generated block stays generic in its width parameters.** FSMGen gives ports *resolved positive
  integer widths* and lists *parameter-driven shape elaboration* as backlog (13k *Interface ports*, *Reusable ISF
  libraries*), so the HDL is fixed at the declared default and a different width means regenerating.
  Specifications describe generic blocks (`HRDATA` is `DATA_WIDTH` bits). Draft, reproduce and sign off a
  `feature_request` for HDL whose port widths remain module parameters, citing FSMGen's own backlog entry and
  checking FSMGen's feedback records for an earlier request first.

- ID: `EXTRACTION-REPRESENTATION-GAPS.7`
  Status: `open` (`2026-09-25`; MEASUREMENT then CODE; SpecForge; found by `.2`)
  Goal: **a real specification's clock and reset reach the system contract.** 0 of 27 live chains carry a
  `system_contract`, so `assess_isf_renderability` blocks all 27 and no current chain emits an `.isf` file; the
  carrier the adapter reads for clock/reset distribution (`infrastructure_signals`) is empty on all 27 too.
  `build_system_contract` (`ir/semantic.rs`) reads only canonical `Clock X.` / `Reset X …` statements and needs all
  seven fields (clock, reset, kind, polarity, assertion and release timing, target); no rule derives them from a
  signal table (`ACLK | Clock source | Global clock signal`, `ARESETn | … | active-LOW`). FSMGen can state it
  (13a *Clock and reset*: `(reset (rst_n async active_low))`). Measure what the 7 signal-bearing live documents
  state, decide which fields the adapter truly needs, and derive the contract from the document's own grammar.

- ID: `EXTRACTION-REPRESENTATION-GAPS.8`
  Status: `open` (`2026-09-25`; CODE; SpecForge defect, latent; found by `.2`)
  Goal: **"asserted" lowers by the signal's polarity.** `isf_literal_value` (`ir/isf_ir.rs`) maps `asserted` to `1`
  and `deasserted` to `0` for every signal, and the adapter never reads `signal_polarities` (51 records), so a rule
  on an active-low signal would lower to the opposite level: a fabricated fact. Reproduce with a unit fixture
  (no live chain reaches it today: 0 of the 27 have such a rule on one of their 5 active-low signals); resolve
  `asserted`/`deasserted` through the signal's polarity, and residualize when the polarity is unknown.

- ID: `EXTRACTION-REPRESENTATION-GAPS.9`
  Status: `open` (`2026-09-25`; CODE; SpecForge; found by `.2`)
  Goal: **a value word lowers to the value it names.** Five rule values are residualized as *not an ISF literal*
  although FSMGen accepts them: `ZERO` (2; the number 0) and the enum members `INCR` (2) and `SHAREABLE` (1)
  (13k *Enum member values*: enum members are legal rule and drive values). Map number words to numbers and a
  value naming a member of the target signal's own enum to that member, declaring the enum. `ARDOMAIN` *Must be
  Shareable (0b01 or 0b10)* names a set of two encodings, so it is a membership assertion (`.11`), not a drive.

- ID: `EXTRACTION-REPRESENTATION-GAPS.10`
  Status: `open` (`2026-09-25`; MEASUREMENT then CODE; SpecForge; found by `.2`)
  Goal: **a normative constraint keeps its condition, and is lowered as what it is.** 38 rules are dropped as
  conflicting drives under an empty guard (`AWSNOOP` 0 vs 1, `AWBURST` INCR vs WRAP) and 7 more overlap a
  transaction's drive: the source states each value under a condition that the rule lost. Read, not assumed:
  `AWSNOOP` 0 (`dyn_sigcon_0068`) comes from *"A Manager that only uses Opcodes where AWSNOOP is LOW can set
  AWSNOOP_WIDTH to 0 which omits the AWSNOOP output … An attached Subordinate must have its AWSNOOP input tied
  LOW"*: a configuration condition and a tie-off for an absent signal (`.4`), not a drive. Classify every dropped
  rule's lost condition (configuration, transaction, phase) before designing the fix. Recover the conditions; and decide, from the lowered actor's perspective, whether a constraint on a
  signal it drives is a drive, and one on a signal it reads an `assume`, rather than lowering every constraint as
  a drive.

- ID: `EXTRACTION-REPRESENTATION-GAPS.11`
  Status: `open` (`2026-09-25`; CODE; SpecForge; found by `.2`)
  Goal: **stability and exclusion obligations lower as assertions.** 7 rules are residualized as *bare stability
  across tick phases has no supported .isf construct* (`HSIZE must not be changed`) and 3 same-cycle exclusions as a
  0-cycle window (`HEXOKAY must not be asserted in the same cycle as HRESP`). The first claim is stale: FSMGen's
  matrix (13k *Immediate verification checks*) has `(assert …)` with `(stable SIG)`, `(=> A B)` and plain boolean
  exclusion. Lower them as assertions; an actor with no transaction to host one needs `.15`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.12`
  Status: `open` (`2026-09-25`; MEASUREMENT then CODE; SpecForge; found by `.2`)
  Goal: **a protocol state machine is typed.** 51 protocol records (45 states, 5 operations, 1 edge timing) are
  residualized because they carry no typed transitions, guards, initial state, encoding or actions, and the typed
  state-machine families (`regular_states`, `state_transitions`) are empty on all 27 live chains. FSMGen can state
  state machines and transactions; SpecForge must extract their parts. No open leaf owned this after
  `SWD-SERIAL-EXTRACTION.7d` introduced the residuals.

- ID: `EXTRACTION-REPRESENTATION-GAPS.13`
  Status: `open` (`2026-09-25`; MEASUREMENT; SpecForge, possibly FSMGen; found by `.2`)
  Goal: **every register field and reset value is accounted for.** 158 register fields are not lowered (no bit
  range, a shared name, or overlap) and 71 registers keep a documented reset that is not integer-lowerable
  (symbolic or implementation-defined, partial coverage, over-wide, overlapping). Break both down by cause; the
  capture causes are SpecForge's, and a symbolic or implementation-defined reset value that FSMGen's `(reset V)`
  cannot state becomes an FSMGen request here if the breakdown shows it.

- ID: `EXTRACTION-REPRESENTATION-GAPS.14`
  Status: `open` (`2026-09-25`; MEASUREMENT then CODE; SpecForge; found by `.2`)
  Goal: **platform topology lowers to composition.** `signal_connectivity` (235 records in 6 documents) is never
  read by the adapter, which emits one flat actor; FSMGen composes actors (13f). Measure which documents state a
  multi-component topology and lower it to FSMGen composition where the IR carries it.

- ID: `EXTRACTION-REPRESENTATION-GAPS.15`
  Status: `open` (`2026-09-25`; REQUEST; found by `.2`)
  Goal: **FSMGen can state an always-on assertion outside a transaction.** `(assert …)`, `(assume …)` and `(cover …)`
  are transaction-level clauses (13d, 13k *Immediate verification checks*), although each lowers to an always-on
  clocked property. A protocol invariant (`HSIZE must not change during a burst`) belongs to no transaction, and
  most SpecForge actors have none to host it. Draft, reproduce and sign off a `feature_request` for actor-level
  assert/assume/cover.

- ID: `EXTRACTION-REPRESENTATION-GAPS.16`
  Status: `open` (`2026-09-25`; REQUEST; found by `.2`)
  Goal: **FSMGen can say "X holds a known value when Y".** Specifications state validity obligations (`PADDR,
  PWRITE and PWDATA must be valid` when `PSEL` is asserted; 11 residualized records, and many more captured as
  constraints). FSMGen's property predicates are `stable`, `changed`, `rose`, `fell` and `past` (13k); there is no
  known-value (`$isunknown`-style) predicate. Draft, reproduce and sign off a `feature_request`. SpecForge's half,
  never emitting `VALID` as a value, is `KG-ISF-COMPLETENESS.2a.vii.e`.

## `.2` — the IntentIR-to-`.isf` census (`2026-09-25`)

Re-derive: `python3 scripts/measure_intent_to_isf_reach.py` (read-only, over the 27 live chains).

**Emission.** All 27 live adapters are `blocked`: 27 have no clock/reset contract, 20 also have no interface
signals, 4 also no behaviour. The 7 with signals (AXI, APB 5, AHB, ADIv6, I2C, I2S, Wishbone) are blocked by the
contract alone (`.7`). Every number below is what the adapter *would* lower; none of it reaches FSMGen today.

**Families the adapter never reads**, and why that is or is not a gap:

| Family | Records (docs) | Verdict |
| --- | --- | --- |
| `constraints`, `behaviors`, `assumptions` | 5,229 (26), 2,091 (24), 157 (22) | untyped prose (`statement` + ids): unlowerable by construction; how many say what no typed record does is `.3` |
| `timing_constraints` | 237 (5) | physical and electrical parameters (ns, V, dB, Ω, mA, UI) from PHY/electrical tables: not an `.isf` gap by decision (below) |
| `signal_polarities` | 51 (4) | read nowhere, which makes the `asserted` lowering polarity-blind: `.8` |
| `signal_connectivity` | 235 (6) | topology; FSMGen composes actors, SpecForge emits one flat actor: `.14` |
| `actor_signal_relations`, `actor_drive_relations`, `actor_sample_relations` | 546, 273, 273 (6) | inputs to `actor_ports` directions, which the adapter reads: not a gap |
| `*_conflicts`, `fidelity_findings`, `captured_region_residuals`, proofs, identities | — | accounting and provenance, not design intent: not a gap |
| `regular_states`, `state_transitions`, `explicit_modules`, `explicit_tops`, `actor_trigger_relations` | 0 | empty on every live chain: an extraction gap where the document states them (`.12`, `.14`) |

**Residual packets** (150 in 28 families over the 27 adapters), attributed:

| Class | Packets | Owner | Why |
| --- | --- | --- | --- |
| protocol state / operation / edge timing | 51 | SpecForge `.12` | the records lack typed transitions, guards, initial state, encoding, actions |
| conflicting drives under an empty guard, rule/transaction overlap | 45 | SpecForge `.10` | the source's condition was lost; constraints lowered as drives |
| value `VALID`/`INVALID` | 11 | SpecForge `KG-ISF-COMPLETENESS.2a.vii.e`; FSMGen `.16` | validity is not a value; no known-value predicate |
| bare stability, 0-cycle exclusion | 10 | SpecForge `.11`; FSMGen `.15` for placement | `(stable …)` exists; the adapter's claim that it does not is stale |
| value words `ZERO`, `INCR`, `SHAREABLE` | 5 | SpecForge `.9` | FSMGen accepts enum members and numbers |
| register fields / resets not lowered | 5 (229 items) | SpecForge `.13`, FSMGen maybe | capture causes; symbolic resets to be measured |
| signal-width parameters | 9 | FSMGen `FSMGEN-REQUESTS.1`, `.4`-`.6`; SpecForge `KG-ISF-COMPLETENESS.2a.vii.c.iv`-`.c.vi` | the ledger's parameter rows |
| upstream canonicalization, grouping, role, unreadable width, guard disagreement, missing interface signal | 14 | SpecForge (their producing stages) | IR-side, before the adapter |

**Decision — physical and electrical parameters are not requested of FSMGen.** `timing_constraints` holds
tables of nanoseconds, volts, decibels, ohms and unit intervals, from documents the category contract classes
`physical-link` (`executable_target: honest_non_target_by_default`). ADR 0033 §6, kept by ADR 0053, keeps such
knowledge typed in IntentIR and forbids forcing it into executable constructs. Requesting an FSMGen carrier for
them would reverse that; it is the owner's call and is raised to them, not assumed.

## Current Frontier

1. `.8` — a latent fabrication in the adapter; small, so fixed first (§15).
2. `.4`, `.5`, `.6`, `.15`, `.16` — the five FSMGen requests.
3. `.7` — the clock/reset contract that blocks every emission.
4. `.11`, `.9`, `.10`, `.12`, `.13`, `.14` — the SpecForge lowering and extraction gaps `.2` attributed.
5. `.3` — the PDF-to-IR census against the category contract.

## Decisions

- `2026-09-25` — ADR 0053: extraction and FSMGen requests advance in parallel; the evidence bar is unchanged;
  one ledger in the book.
- `2026-09-25` — the ledger is seeded only with rows re-verified today. The legacy-document reach was left
  out: chain currency reports 51 documents unmeasurable, while `CORPUS-COVERAGE`'s census counts refreshed
  documents differently; `.2` re-derives the reach before it is published.

## Open Questions

None.

## Blockers

None.

## Verification Log

- `2026-09-25` — `.0`: every seeded row re-derived in this session. FSMGen facts read at the pin from its
  published book (`subs/fsmgen/docs/book/src/13a-actor-interface.md` *Interface*;
  `13k-isf-feature-support-matrix.md` rows *Actor report metadata and params*, *Interface ports*, *Reusable ISF
  libraries*); SpecForge facts from the persisted IntentIR of the three live AMBA chains (36 `Param` symbols: AXI
  28, AHB 7, APB 5 1) and the `.2a.vii.c.iii` record.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.1` (CHECK) — DONE `2026-09-25`

- [x] **REPRODUCE / MEASURE** — with the ledger declared as a region, `perl scripts/check_ownership_citations.pl --check` at `869a2b62` would read `KG-ISF-COMPLETENESS.2a.vii.c.iv` (declared one-line in `docs/tasks/kg-isf-completeness/isf-parameter-widths.md`) and `FSMGEN-REQUESTS.1` (declared one-line in its root) as nonexistent: `unit_status` matched only `- ID:` + newline + `Status:` in the root file.
- [x] **ROOT CAUSE (WHY + WHERE)** — `unit_status` (`scripts/check_ownership_citations.pl`) knew one of the two leaf grammars and never opened a partitioned tree's parts.
- [x] **ADDRESSED (verified)** — `resolve_leaf_status` + `tree_part_texts`; live check: 38 cited units across 3 regions resolve. End to end: pointing one ledger row at the closed `KG-ISF-COMPLETENESS.2a.vii.c.iii` is refused by name (`its own Status is 'done'`), then restored.
- [x] **NO REGRESSION** — self-test 16/16 (11 prior cases unchanged in intent); `ROADMAP.md` and the pressure-headroom region still resolve (28 units before the ledger region, 38 with it); RED: deleting the one-line grammar fails *a one-line leaf in a root did not resolve*, and resolving disagreeing declarations to the first fails *disagreeing declarations were resolved to one status*.
- [x] **GENERICITY** — the rule reads the task-tree grammars and the repository's `docs/tasks/<tree>/` layout; no tree is named.
- [x] **LOCKSTEP** — `docs/DOCTRINE_INSTANCE.md` `OWNERSHIP-CITATIONS` row; the ledger chapter's update rule states the check; this tree's frontier.

## Commit Log

- `.0` — `EXTRACTION-REPRESENTATION-GAPS.0 — one ledger of extraction and representation gaps, owned per side`.
- `.1` — `EXTRACTION-REPRESENTATION-GAPS.1 — the gap ledger's owners are checked, in either leaf grammar`.
- `.2` — `EXTRACTION-REPRESENTATION-GAPS.2 — census: every live adapter is blocked, and 150 residuals attributed`.

## Changelog

- `2026-09-25` — tree opened on the owner's directive; `.0` done (ADR 0053, ledger chapter); `.1`-`.6` open.
- `2026-09-25` — `.1` done: the ledger's owner column is an enforced region, and the checker reads one-line and
  partitioned-tree leaves.
- `2026-09-25` — `.2` done: all 27 live adapters blocked for want of a clock/reset contract; 150 residuals and
  ~20 unread families attributed; `.7`-`.16` opened (8 SpecForge, 2 FSMGen); `.3` re-anchored on the category
  contract; physical/electrical parameters not requested, by decision, and raised to the owner.
