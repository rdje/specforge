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

The 97 weak occurrences form 38 unique document/token pairs across 28 documents. Fifteen pairs in 13
documents become relation-active (120 relations); 11 pairs in 10 documents are promoted to direction-only
output declarations. Examples include `signal interrupt if enabled`, `Signal level`, `Signal names`, and the
USB source clause `signal at its upstream port ...`.

**First unsupported promotion:** `collect_known_signal_names`. The source only contains a descriptor/verb
use, but the catalog grants declaration authority.

**Selected repair:** parse the declaration predicate already emitted everywhere else. A catalog entry is valid
only when the same sentence has `Signal <identifier> is input|output|internal|local|width ...`. This retains all
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

1. declaration catalogs require the formal `is <direction|width>` predicate;
2. parenthetical width-one capture removes `bus` from its wire-head grammar;
3. port/pin-only tables require compact inventory structure before the shared table consumers run.

Required focused regressions: the exact USB clause, acronym, and VBUS matrix; formal declarations after Unicode;
I2C/I2S/SWD/SWP prose declarations; genuine signal tables, rotated tables, SWJ pin routing, and field/status
negatives; convergence absence for all four USB names; explicit initiator tie behavior.

Required broader gates: focused EvidenceIR/ISF tests, `signal_table_inventory_authority_negative`,
`table_misclassification_field_table_negative`, all 156 KG fixtures, WIRE-BASED-100 APB/AHB/AXI/SWD/I2C
oracles, real FSMGen strict, full `scripts/run_ci.sh`, mdBook, doctrines, and project-data locality. The final
truth test is `.2.33d.iv`: rebuild the real USB EvidenceIR→adapter chain and prove the four false ports, phantom
initiator, and 29-rule model disappear without replacing them with fabricated hardware.
