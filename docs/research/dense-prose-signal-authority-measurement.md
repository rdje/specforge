# Dense-prose signal-authority measurement — `CORPUS-COVERAGE.2.33d.i`

Owning leaf: `CORPUS-COVERAGE.2.33d.i` (PROBE/DOC; no production-code change).
Date: `2026-08-09`.

## Question and measured corpus

USB 3.2 produced a strict-clean but semantically false ISF adapter from four tokens: `AT`, `USB`,
`ENHANCED`, and `NO`. This probe asks where each token first receives signal authority, whether the same
grammar appears elsewhere, how the fixed-point consumes it, and what the smallest universal repair is.

The census reads all currently retained artifacts: 80 SourceIR, 80 EvidenceIR, and 79 IntentIR documents.
The corpus has mixed extraction vintages, so counts below describe retained artifact truth, not a claim that
every document was rebuilt by the same binary. The activated USB chain is current-binary. Every grammar named
below was also checked against the current Rust implementation, so an older artifact can illustrate reach but
does not define the repair.

USB before signature (retained real regression input):

- SourceIR: 548 pages, 507 visual assets, 283 tables, 5,830 content elements; table `table_0210` is the
  `Downstream USB Standard-A Port VBUS Requirements` matrix.
- EvidenceIR: 8,412 statements and 45 actor/signal relations. The four candidate relation counts are `AT`
  17 (12 drives / 5 reads), `USB` 15 (12/3), `ENHANCED` 5 (2/3), and `NO` 8 (6/2).
- Fixed-point outputs: statements `8416`–`8419` are `Signal USB|AT|ENHANCED|NO is output.`
- IntentIR: three tied two-output/zero-input net producers; adapter actor
  `setportfeature_port_over_current`; four ISF ports and 29 rules.
- FSMGen strict: success with zero diagnostics. This is a syntax result, not a fidelity result.

## 1. Sentence-start `signal <word>` is not a declaration grammar

`collect_known_signal_names` says it collects formal declarations, but the implementation accepts the first
token after any sentence-start `signal ` and uppercases it. It never checks the predicate after the token.

Across 80 EvidenceIR artifacts there are 3,201 sentence-start matches:

| shape | occurrences | interpretation |
| --- | ---: | --- |
| `Signal X is input ...` | 576 | formal declaration |
| `Signal X is output ...` | 1,159 | formal declaration |
| `Signal X is width ...` | 1,369 | formal width-only declaration |
| every other predicate | **97** | ordinary prose / headings / table text, not declarations |

The retained census contains no `is inout` declaration, but `Signal <name> is inout ...` is an existing
product grammar and is exercised by the protocol edge-timing contract. The `.d.ii` broad gate caught that
zero-corpus preservation case; the implementation therefore treats `inout` as a formal direction alongside
`input`, `output`, `internal`, and `local`. This does not alter the measured 3,104/97 corpus split.

The 97 weak occurrences form 38 unique document/token pairs across 28 documents. Fifteen pairs in 13
documents become relation-active (120 relations); 11 pairs in 10 documents are promoted to direction-only
output declarations. Examples include `signal interrupt if enabled`, `Signal level`, `Signal names`, and the
USB source clause `signal at its upstream port ...`.

**First unsupported promotion:** `collect_known_signal_names`. The source only contains a descriptor/verb
use, but the catalog grants declaration authority.

**Selected repair:** parse the declaration predicate already emitted everywhere else. A catalog entry is valid
only when the same sentence has `Signal <identifier> is input|output|inout|internal|local|width ...`. This retains all
3,104 formal matches and rejects all 97 weak matches without a token denylist. Original identifier case remains
permitted because tracked fixtures deliberately use declarations such as `Signal clk is input width 1.`; the
predicate, not capitalization, is the authority.

## 2. A bus acronym is not a one-bit wire

The sparse-catalog prose fallback synthesizes `Signal X is width 1.` for a parenthetical abbreviation whose
immediate head is one of `line/signal/clock/data/wire/bus/pin`. The `bus` member is category-wrong: a bus or
protocol acronym may contain many wires, but it is not itself a width-one hardware signal.

The retained artifacts carry 77 evidence-backed width-one prose declarations across 32 documents:

| synthesis grammar | count |
| --- | ---: |
| parenthetical abbreviation | 70 |
| `pin, NAME` appositive | 5 |
| definitional `NAME is a signal` / `NAME: signal` | 2 |

Exactly 17 parenthetical declarations across six documents have the immediate head `Bus`. They name `ATB`,
`AHB`, `APB`, `PPB`, `USB`, or `USB4` as width-one signals. Fourteen are relation-active, producing 135
relations and 12 direction promotions. None is a single wire. USB statement 1154 is the current-binary instance:
`Universal Serial Bus (USB)` becomes `Signal USB is width 1.`

**First unsupported promotion:** `synthesize_signal_declarations_from_prose` treats the semantic category
`bus` as a single-wire noun.

**Selected repair:** remove only `bus` from the single-wire-head grammar. Keep `line`, `signal`, `clock`,
`data`, `wire`, and `pin`; keep the independent pin-appositive and definitional forms. This preserves the
tracked serial oracles: I2C `SDA/SCL/USCL/USDA/SDAH/SCLH`, I2S `SCK/SD`, SWD `SWCLK/SWDIO/NSRST`, and SWP
`S1/S2`. It also removes protocol/bus acronyms without naming any protocol.

## 3. Ordinary port vocabulary does not make a signal inventory

`classify_table_kind` treats a caption containing `signal`, `port`, or `pin` as positive evidence for
`signal_description`. `should_treat_table_as_top_level_signal_description` then treats any header containing
those substrings as explicit signal structure. In USB `table_0210`, ordinary phrases such as `Hub Upstream Port
Connection Status` therefore pass both gates. `collect_signal_names_from_tables` uppercases the first word of
each row, admitting `ENHANCED`, `USB`, and `NO` from state/requirement labels.

Corpus shape census:

- 624 tables in 44 SourceIR documents are currently classified `signal_description`.
- 32 tables across 11 documents are classified from port/pin vocabulary while having neither an explicit
  `signal(s)` caption nor a compact signal/name/symbol/pin identity header.
- 11 of those tables across five documents pass the current top-level gate. Their first column yields 32 raw
  hardware-shaped row candidates (23 unique document/token pairs). The family mixes real connector pins with
  false state, presence, instruction, and port-status rows, so dropping every port/pin table would lose facts.
- USB contributes three accepted weak tables: a state-machine legend (`table_0208`), the VBUS requirements
  matrix (`table_0210`), and port-status type codes (`table_0245`). The activated table-only names `ENHANCED`
  and `NO` produce 13 relations and two direction promotions; `USB` overlaps the prose-bus seed.

**First unsupported promotion:** the shared `should_treat_table_as_top_level_signal_description` authority
gate accepts ordinary `port` substrings as inventory structure; every table-based catalog/width/relation consumer
then trusts the result.

**Selected repair:** for the port/pin-only family, require actual inventory structure: a compact identity header
(`signal`, `name`, `symbol`, or a pin-routing header ending in `pin/pins`), or the headerless connector/pin-diagram
shape. An explicit `signal(s)` caption continues to authorize normal and headerless signal inventories. This
rejects USB's verbose port-state/status matrices while preserving the SWJ pin-routing table, headerless MIPI
connector diagrams, `Signal | Description` tables, and rotated `Name | Destination | Width | Description`
tables. The existing content-based rotation logic remains downstream of this authority check and is unchanged.

## 4. Relation-derived directions amplify authority; they do not originate names

The retained corpus contains 766 direction-only `Signal X is output.` statements. This mechanism is important:
many real signal inventories state an actor relationship but omit a conventional direction column.

Runtime production is nevertheless closed over the known-signal set:

1. prose relations scan only names supplied by table catalogs or formal/synthesized declarations;
2. table relations use the same `should_treat_table_as_top_level_signal_description` gate;
3. `synthesize_directions_from_relations` copies the relation's existing signal name and adds only direction.

It therefore cannot create a fifth independent signal name. In USB it makes the three upstream mistakes look
formal, but after those catalog boundaries reject `AT`, `USB`, `ENHANCED`, and `NO`, no relation on those names
can be produced and there is nothing for direction synthesis to promote.

**Decision for `.2.33d.iii`: measured unnecessary as a separate product heuristic.** The implementation slice
must add a convergence regression proving rejected candidates cannot re-enter through relations. If that direct
test and the real USB rebuild pass, `.d.iii` closes as unnecessary. Adding a second downstream deny/filter would
duplicate catalog authority, risk deleting legitimate relation-derived directions, and conceal the actual source
of trust.

### `.2.33d.iii` closure verification

The conditional leaf is closed as measured unnecessary. `git diff --exit-code 5c95a041..HEAD` proves
`evidence.rs` and `isf_ir.rs` have not drifted since the `.d.ii` implementation. The direct combined fixture
`weak_dense_prose_names_cannot_reenter_through_relations` and its four supporting declaration/table/
parenthetical/initiator tests pass independently. Code inspection reconfirms that prose relations scan the
known-signal catalog, table relations share the structural authority gate, and direction synthesis copies only
an existing relation name. No fourth name-authority seam exists.

Adding a convergence or adapter deny-filter would therefore be policy duplication, not defense in depth: it
could suppress a real signal whose grounded direction exists only as a driver relation. No production or
generated artifact changes in `.d.iii`. `.d.iv` remains the independent real USB cascade proof and may reopen
this decision only if current-binary evidence produces a weak name through a newly observed authority path.

## 5. Initiator ties are deterministic, but the comment is wrong

Across 79 retained IntentIR artifacts, nine documents have a maximum `(outputs, inputs)` net-producer tie,
covering 51 tied actors. USB has three tied actors at `(2, 0)`. `select_initiator_actor` iterates a name-ascending
`BTreeMap` and calls `max_by_key`; equal maxima replace the earlier item, so the lexicographically **last** name
wins. The live USB winner proves it. The nearby comment claims the lexicographically smallest name wins, which
is incorrect.

Tie ordering is not the USB root cause: all three candidates are false. The code slice should preserve the
deterministic behavior, correct the comment, and add an explicit equal-key test so later readers do not infer a
different contract.

## Repair and verification handoff

`CORPUS-COVERAGE.2.33d.ii` owns three universal changes and no others:

1. declaration catalogs require the formal `is <direction|width>` predicate, including `inout`;
2. parenthetical width-one capture removes `bus` from its wire-head grammar;
3. port/pin-only tables require compact inventory structure before the shared table consumers run.

Required focused regressions: the exact USB clause, acronym, and VBUS matrix; formal declarations after Unicode
plus an explicit `inout` preservation case;
I2C/I2S/SWD/SWP prose declarations; genuine signal tables, rotated tables, SWJ pin routing, and field/status
negatives; convergence absence for all four USB names; explicit initiator tie behavior.

Required broader gates: focused EvidenceIR/ISF tests, `signal_table_inventory_authority_negative`,
`table_misclassification_field_table_negative`, all 156 KG fixtures, WIRE-BASED-100 APB/AHB/AXI/SWD/I2C
oracles, real FSMGen strict, full `scripts/run_ci.sh`, mdBook, doctrines, and project-data locality. The final
truth test is `.2.33d.iv`: rebuild the real USB EvidenceIR→adapter chain and prove the four false ports, phantom
initiator, and 29-rule model disappear without replacing them with fabricated hardware.

## Real-cascade checkpoint (`CORPUS-COVERAGE.2.33d.iv.a`)

The preserved-source rebuild confirms the original three authorities are closed but does **not** yet pass the
last clause above. SourceIR stays byte-identical at SHA-256 `e87f5003…a13`. Fresh EvidenceIR has 8,267 statements,
zero actor-signal relations, ten signal constraints, 542 conditional rules, and 102 timing constraints (before:
8,412 / 45 / 2 / 542 / 102). Fresh IntentIR has 18 actors, zero actor ports, and zero relations (before: 51 / 42 /
45). `AT`, `USB`, `ENHANCED`, `NO`, and `setportfeature_port_over_current` are absent from the current adapter.

Two independent findings prevent signoff:

1. The first repaired `adapt` wrote `channel.isf` but retained the obsolete false-actor `.isf`. The generic
   writer reconciliation in `.iv.a` now removes obsolete regular/symlink `.isf` siblings after a successful
   write and preserves unrelated files/directories. The real document directory now contains only
   `adapter.json` and `channel.isf`.
2. SemanticIR still contains 918 low-confidence heuristic interfaces / 2,940 signal records. With the formal
   authority set empty, `retain_authoritative_interface_candidate_signals` admits every statement token group;
   encoding/data table rows then deduplicate to 556 one-bit `.isf` outputs despite an empty actor graph. This is
   a second authority loop, not residue from the original four names. `.iv.b` owns its corpus measurement and
   generic repair; `.iv.c` owns the final real-cascade proof and rollback deletion.

## Authority-empty SemanticIR repair (`CORPUS-COVERAGE.2.33d.iv.b`)

The retained-corpus before census found 21 documents whose interface signal records were all low confidence:
5,527 interfaces, 18,397 records, and 4,060 per-document unique names. Their current adapters consumed 4,060
signals and 544 rules; 19 of 21 were marked renderable. This is the complete affected set:

| document key | interfaces | records | unique | adapter status | signals | rules |
| --- | ---: | ---: | ---: | --- | ---: | ---: |
| `5_0_2024_08_intel_virtualization_technology_for_directed_io_specification` | 929 | 3,154 | 510 | renderable | 510 | 44 |
| `usb_3_2_revision_1_0_2017_09` | 918 | 2,940 | 556 | renderable | 556 | 170 |
| `ihi0088_g_2024_06_amba_dti_protocol_specification` | 530 | 2,325 | 294 | renderable | 294 | 81 |
| `nvme_base_specification_2_0a_2021_07_26` | 737 | 2,176 | 550 | renderable | 550 | 116 |
| `ccix_base_specification_r1_0a_v1_0_for_evaluation` | 480 | 1,694 | 279 | renderable | 279 | 36 |
| `1_0_1_2026_02_22_risc_v_iommu_architecture_specification` | 426 | 1,384 | 175 | renderable | 175 | 45 |
| `pjdoc_466751330_7215_10_0_cortex_a76_software_optimization_guide` | 228 | 825 | 537 | renderable | 537 | 0 |
| `1_0_2025_03_12_risc_v_advanced_interrupt_architecture` | 211 | 619 | 91 | renderable | 91 | 22 |
| `bosch_can_specification_2_0_1991` | 131 | 531 | 97 | renderable | 97 | 6 |
| `ihi0098_b_2026_03_23_amba_chi_chip_to_chip_c2c_architecture_specification` | 195 | 512 | 109 | renderable | 109 | 8 |
| `109242_0100_01_2023_09_04_arm_smmu_software_guide` | 135 | 470 | 144 | renderable | 144 | 3 |
| `readme` | 78 | 345 | 152 | renderable | 152 | 3 |
| `ihi0076_a_2018_05_02_advanced_communications_channel_architecture_specification` | 121 | 293 | 133 | renderable | 133 | 4 |
| `198123_0302_03_2025_04_22_generic_interrupt_controller_overview_guide` | 83 | 276 | 89 | renderable | 89 | 3 |
| `den0068_2018_07_23_coresight_base_system_architecture` | 88 | 230 | 100 | renderable | 100 | 2 |
| `ihi0098_a_b_2026_02_03_amba_chi_chip_to_chip_c2c_architecture_specification` | 82 | 208 | 69 | renderable | 69 | 0 |
| `ihi0098_a_2024_02_07_amba_chi_chip_to_chip_c2c_architecture_specification` | 74 | 193 | 69 | renderable | 69 | 1 |
| `102196_0100_01_2022_05_05_aarch64_external_debug_guide` | 64 | 182 | 73 | renderable | 73 | 0 |
| `jesd235_2013_10_hbm_dram` | 7 | 18 | 16 | blocked | 16 | 0 |
| `opencapi_afu_address_space_usage` | 5 | 12 | 8 | blocked | 8 | 0 |
| `opencapi_4_0_32gbps_phy_mech_spec_v10_17mar2021` | 5 | 10 | 9 | renderable | 9 | 0 |

Twenty documents have no actor ports or actor/signal relations. DTI alone has one of each:
`TBU reads DOWNSTREAM` at medium confidence from statement 1740. That is not a grounded heuristic-only wire;
`DOWNSTREAM` is ordinary direction prose, while the same document's 294 current adapter signals are visibly
dominated by statement-token groups. Preserving it would make an actor relation a second declaration authority
and would not justify the other 2,324 records. No exception is selected.

The first unsupported promotion is the empty-set branch in
`retain_authoritative_interface_candidate_signals`: it returned all candidates when no formal/system-contract
authority existed. The repair replaces that allow-all with a narrow positive grammar. Formal declarations still
take the explicit interface path; the system contract inserts clock/reset names before grouping. Without either,
a multi-signal statement may ground its own group only when it begins with one candidate identifier and then
makes a deontic signal action (`must`/`shall` plus assert/deassert/stability). This preserves the existing
declaration-free `VALID must remain asserted until READY is observed` handshake contract while rejecting raw
encoding rows, ordinary prose, and DTI's `DOWNSTREAM` read. Focused tests lock both empty-authority outcomes,
full handshake construction, declared-surface filtering, and system-contract preservation.

A release-binary dry run over the exact 21-document cohort produces **0 interfaces / 0 records** in every member.
Preserved-binary comparison proves APB, AHB, AXI, and SWD interface surfaces byte-identical. WIRE-BASED-100 stays
1.000 on APB/AHB/AXI constraints, relations, and temporal rules, I2C declarations, SWD relation and complete
11/4/13/1 derivation; the documented SWD promotion-only constraint remains 0/1. `kg-bench` is 156/156.

The materialized USB rebuild is the concrete downstream proof. SemanticIR and IntentIR contain zero interfaces
and interface records; actor ports/relations remain zero. `adapt` reports `blocked`, `no signals declared in
interface`, zero signals/rules, one retained storage record, and 15 honest residuals. The successful blocked
write removes `channel.isf`, leaving exactly `adapter.json`. Other semantic candidates can remain visible in
typed rules or residual source text, but none becomes an emitted hardware port without signal authority.
`.iv.c` retains the complete final cascade/gate repetition and verified-rollback deletion.
