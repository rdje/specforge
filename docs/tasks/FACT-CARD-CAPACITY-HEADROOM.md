# FACT-CARD-CAPACITY-HEADROOM: restore headroom before the fact plane refuses new knowledge

## Metadata

- Tree ID: `FACT-CARD-CAPACITY-HEADROOM`
- Status: `done` (`.0`/`.2`/`.2a`/`.3`/`.3a` done; `.1` superseded)
- Roadmap lane: repository durability and portability (sibling of `FACT-CARD-CATALOG-CONTAINMENT`)
- Created: `2026-08-10`
- Last updated: `2026-09-16`
- Closed: `2026-08-11`
- Owner: repo-local workflow

## Goal

Keep writing a Knowledge Map fact card possible. Two independent authorities bound the fact plane, and the
tighter one is now **four facts** from refusing the next card — while the working discipline
(`AGENTS.md`, `COMMIT.md`) tells every slice to write a card whenever it establishes a durable fact. Restore
deliberate headroom before a routine slice hits a hard gate failure it did not cause and cannot locally fix.

> **Outcome (`2026-08-11`, closed).** The plane holds **336 cards / 44 decision records / 379 facts**, currently
> 193 / 30 / 199, and every fact-plane rollover warning is gone. The measurements and bounds recorded below
> describe the problem as it was found; ADR 0029 owns the current profile. The tree also answered a question it
> did not set out to ask: the advertised 198-card capacity had never been reachable, because the plane's own
> aggregate line ceiling refused it — and no compliant action could have cleared that state.

## Non-Goals

- Do not widen a ceiling merely to make the gate green; a limit is raised only with its own measured rationale,
  the way `ADR 0022`/`ADR 0023` tightened and re-derived these bounds.
- Do not delete or silently rewrite existing cards. Superseding is the documented lifecycle
  (`MEMORY_ARCHITECTURE.md` §10); compaction, if chosen, must preserve the audit trail.
- Do not change what a fact card *is*, or how the Knowledge Map derives its landing and shards.

## Measured pressure (`2026-08-10`, from `CORPUS-CHAIN-CURRENCY.2`)

Discovered while adding this session's retention card, not predicted from theory.

| Authority | Contract | Current | Maximum | Headroom |
| --- | --- | ---: | ---: | ---: |
| Fact count (cards + `answers:`-bearing decision records) | `doctrine/knowledge_map/shard_contract.json` `max_facts` | 196 | 200 | **4** |
| Card count | `doctrine/live_document_size/fact_card_catalog.json` `max_cards` | 193 | 198 | 5 |
| Landing lines | catalog `limits.landing.health_targets.lines` | 196 | 224 | 28 (87.5% — the checker already warns) |
| Title parts | catalog `limits.title_parts` | 4 parts / 193 cards | 4 parts / 224 slots | not binding |

`.2` has since removed the landing row from this table's logic: the landing is 10 lines at 193 cards and at most
10 at full capacity, so it is no longer a capacity dimension at all.

`.2a` then found that this table was **incomplete**, not merely superseded in one row. Two further hard
authorities bound the plane and neither appears above:

| Authority | Contract | Current | Ceiling | Headroom | Warned before `.2a`? |
| --- | --- | ---: | ---: | ---: | --- |
| Card aggregate lines | `knowledge_cards` `enforcement_ceilings.lines_total` | 9,773 | 10,000 | **~4.5 cards** | **no** |
| Card aggregate bytes | `knowledge_cards` `enforcement_ceilings.bytes_total` | 853,926 | 1,048,576 | ~44 cards | **no** |
| Decision-record files | `decision_records` `enforcement_ceilings.files` | 30 | 32 | **2 records** | yes (93.8%) |

The first is the tightest hard bound on the whole plane and was invisible: `check_live_document_size.pl`
suppressed aggregate milestones for `locator: "file"` surfaces while still failing their aggregate ceilings
closed, and `knowledge_cards` declared a `file` locator over a 195-file glob. Before `.2` shrank the landing it
stood at 9,959/10,000 — 41 lines against an average card of 50.1, i.e. under one card from a stop with no
warning. `.2a` fixed the blindness; `.3` re-derives all of it.

The originally-recorded binding authority is `max_facts`, and it counts five `docs/decisions/` records alongside
the 193 cards. Failure is not graceful — `KNOWLEDGE-MAP` is a `gate`-tier doctrine, so the pre-commit hook refuses
the commit that writes the 201st fact, in whatever unrelated slice happens to write it.

### `.0` census (`2026-08-10`)

**Every bound is pinned in code as well as data**, so no single edit can widen one. `max_cards` is *derived*
(`scripts/check_fact_card_catalog.pl:380`) as the `knowledge_cards` surface file ceiling minus README and INDEX,
and both the file ceiling (`:374`) and the shard contract's `max_facts` (`:402`) are pinned to the literal 200.
`fixed_limits()` (`:97`) additionally pins `cards_per_part` 56, `max_parts` 4, and the landing's 224-line health
target / 256-line ceiling. Its self-test already covers fixed-limit inflation and question-capacity drift, so a
raise must move the pinned literals, their fixtures, and the data together.

**Landing growth is exactly linear**: 196 lines at 193 cards, i.e. `lines = cards + 3`, because the landing
carries one bare ID line per card. The 4 × 56 title-part slots permit 224 cards, and the 2,048-key question
budget (1,439 keys today, 7.37 per card) would not bind until roughly 278 — so the landing is the binding
dimension.

**How binding, exactly** (corrected by ADR 0027 after `.0` first compared against the health target instead):
the checker treats **≥ 90% of a health target as a mandatory-rollover error**, not a warning
(`pressure_findings`, `:1061`). At 198 cards the landing is 201/224 lines = 89.7% — inside by 0.3 points — and
at 199 it is 90.2% and fails. An existing regression asserts exactly that (`:1364`, `:1373`), and setting
`max_cards` to 199 reproduces it. **198 is therefore not a chosen limit but the exact maximum this projection
shape permits**, and no bound can be raised until the shape changes.

**Consumption is fast, and was measured rather than assumed** — cards created per active day, from Git:
23 (`2026-08-08`), 25 (`2026-08-09`), 20 (`2026-08-10`). Four facts is a few hours of ordinary work.

**Compaction is not a lever**: of 193 cards only 57 declare a `status:` at all, and exactly two are
`superseded`. There is no backlog of retired knowledge to reclaim.

[ADR 0026](../decisions/0026-fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards.md)
records the census and two standing decisions — stop the landing scaling with cards, and never delete a card
for capacity. Its third (raise the bounds to a 221-card join) is superseded by
[ADR 0027](../decisions/0027-fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound.md),
which shows there is no join to raise to until the projection is reshaped.

## Acceptance Criteria

- The binding authority is re-derived from measured evidence rather than raised by preference, in an ADR, and
  every raised literal stays pinned from both sides by an exact-boundary regression.
- Whatever mechanism is chosen (a re-derived maximum, a fifth title part, supersession compaction, or a
  combination) keeps every existing card and decision record reachable and byte-honest.
- `scripts/check_fact_card_catalog.pl`, `knowledge-map/scripts/check_knowledge_map.sh`, and
  `scripts/check_live_document_size.sh` pass with no pressure warning at the new state.
  **Met for every capacity dimension.** One non-capacity warning survives and is recorded rather than hidden:
  `knowledge_cards` `lines_each` at 81.0%, because `transaction-capture-census.md` is 243 lines against a
  300-line per-card bound. That is a per-card quality signal with a local remedy (split or supersede that card)
  and `.3` is forbidden from editing card content.
- The book's live-docs / doctrine-enforcement chapters and `FACT-CARD-CATALOG-CONTAINMENT`'s recorded bounds
  agree with the new authority before the tree closes.

## Task Tree

- ID: `FACT-CARD-CAPACITY-HEADROOM`
  Status: `done`
  Goal: restore deliberate headroom in the fact plane before it blocks an unrelated slice
  Children: `FACT-CARD-CAPACITY-HEADROOM.0`, `.1` (superseded), `.2` (done), `.2a` (done), `.3` (done), `.3a` (done)

- ID: `FACT-CARD-CAPACITY-HEADROOM.0`
  Status: `done` (`2026-08-10`, PROBE/DOC)
  Goal: pin the exact current pressure across both authorities, enumerate every reader and writer of each
  bound, classify the existing cards by lifecycle, and accept an ADR that decides the mechanism before any
  limit or file changes.
  Acceptance: `an ADR is accepted; the census names every bound, its pinned readers, the landing growth law, the measured burn rate, and the compaction surface; no contract value or generated surface is edited in this leaf`
  Verification: the census above; ADR 0026 accepted; nothing under `doctrine/`, `docs/knowledge/`, or
  `docs/knowledge-catalog/` changed in this leaf.
  Commit: `FACT-CARD-CAPACITY-HEADROOM.0 — measure the fact plane and decide its capacity law`

- ID: `FACT-CARD-CAPACITY-HEADROOM.1`
  Status: `superseded` by `FACT-CARD-CAPACITY-HEADROOM.3`
  Goal: (withdrawn) raise the four pinned authorities to ADR 0026 decision 1's 221-card join.
  Reason: the premise is false. ADR 0027 shows 198 is already the exact maximum this projection shape permits —
  199 cards puts the landing at 90.2% of its health target, which the checker treats as a mandatory-rollover
  **error**, and an existing regression asserts the 199-card projection fails closed. Nothing can be raised
  before the shape changes, so this leaf's work moves behind `.2` and is re-scoped as `.3`.

- ID: `FACT-CARD-CAPACITY-HEADROOM.2`
  Status: `done` (`2026-08-10`, DOCTRINE/PROJECTION)
  Goal: stop the landing scaling with cards (ADR 0026 decision 2, promoted to the critical path by ADR 0027) —
  the landing becomes a bounded router over the title parts with each part's explicit ID range, so capacity is
  no longer bounded by a per-card landing line.
  Acceptance: `the landing is fixed-size and range-complete; every card is reachable in one extra deterministic hop; derive-and-diff, membership, residue, and exact-boundary rules still hold; the book and FACT-CARD-CATALOG-CONTAINMENT bounds agree`
  Verification: the landing is 10 lines / 878 bytes at 193 cards (was 196 / 15,417) and its size follows the part
  count — 7 lines at one part, 10 at the four-part maximum — proven by a three-point self-test asserting
  `lines == parts + 6`. Range rows are re-parsed and re-derived against the canonical card list; three focused
  regressions reject a miscounted part, a renamed boundary id, and a dropped row. The landing is rejected if it
  links a card directly, and every card must resolve exactly once across the parts. `knowledge_cards` now
  declares `routed_membership` through `fact_card_titles`, a new generic index kind with a fixed one-hop
  contract. 49/49 catalog cases and 64/64 live-document cases pass; the four title parts are byte-identical.
  Commit: `FACT-CARD-CAPACITY-HEADROOM.2 — make the fact-card landing a fixed-size router`

- ID: `FACT-CARD-CAPACITY-HEADROOM.2a`
  Status: `done` (`2026-08-10`, DOCTRINE)
  Goal: make aggregate pressure visible before any capacity is re-derived. `.3`'s measurement found a fifth
  authority the `.0` census missed and that no warning could ever have surfaced: `knowledge_cards.lines_total`
  is 9,773 of a hard 10,000-line ceiling (97.7%, ~4.5 average cards), because
  `check_live_document_size.pl:490` suppresses aggregate milestone warnings for `locator: "file"` surfaces while
  `:475` still fails their aggregate ceilings closed. Four surfaces declare a `file` locator over a multi-file
  glob — `knowledge_cards` (195), `task_evidence` (147), `decision_records` (29), `fsmgen_issue_packets` (7) —
  so the whole class runs silent up to a hard error.
  Acceptance: `the aggregate exemption applies only to a genuinely single-file surface; a file locator is proven to match exactly one path; every multi-file surface is reclassified and its real aggregate pressure is reported; each rule is covered by a fail-closed case; no ceiling moves and the gate stays green`
  Verification: the exemption now keys off the measured `files` count, not the declared locator; a `file`
  locator matching more than one path is rejected outright; `knowledge_cards`, `task_evidence`,
  `decision_records`, and `fsmgen_issue_packets` are reclassified as collections with no ceiling, target,
  milestone, lifecycle, index, or verifier change. `knowledge_cards` now reports `lines_total` 9,773/10,000
  (97.7%, rollover) and `bytes_total` 853,926/1,048,576 (81.4%, warning) — pressure it always had and never
  showed. 67/67 live-document cases pass, including a rejected multi-path file locator, a warning collection,
  and a still-exempt single-file surface. ADR 0028 accepted, deliberately without an `answers:` block.
  Commit: `FACT-CARD-CAPACITY-HEADROOM.2a — make aggregate pressure visible before re-deriving capacity`

- ID: `FACT-CARD-CAPACITY-HEADROOM.3`
  Status: `done` (`2026-08-11`, DOCTRINE/CAPACITY)
  Goal: re-derive the whole projection profile once the landing is fixed-size (ADR 0027 decision 3) — per-part
  lines, part totals, part count, the surface file ceiling that `max_cards` derives from, and the independent
  `max_facts` / `max_question_keys` authorities — each against its own 90%-rollover rule, in one transaction.
  Acceptance: `every raised literal is derived and re-pinned from both sides (a full-capacity projection crosses no mandatory pressure and one card beyond it fails closed); no card content changes; catalog, knowledge-map, and live-document gates pass`
  Verification: measurement found a plainer defect than a tight limit — **the declared capacity was not
  reachable**: 198 cards at the measured 50.5-line mean need 10,026 aggregate lines against a 10,000-line
  ceiling, and that ceiling had no legal exit (cards are canonical and never deleted or rolled over).
  ADR 0029 fixes the class: pressure belongs on a dimension with a remedy, so every aggregate becomes the file
  bound times the per-file bound, and capacity is set so the measured population is below the 80% warning and
  one measured peak day is below the 90% rollover. `max_parts` is the only free parameter (6, bound by the
  part-file dimension: four rendered parts must stay under 80%); `max_cards` 336, `knowledge_cards.files` 338,
  `decision_records.files` 44, `max_facts` 379, `max_question_keys` 3,072, projection 12,384 lines /
  1,581,056 bytes. `max_facts` stopped being a literal — the checker derives it from `max_cards` plus the
  decision-record file ceiling. 58/58 catalog cases (was 49) including the derived-profile identities, the
  336-card no-pressure render, the 337-card fail-closed, and a case per fact writer drifting alone; 67/67
  live-document cases; the real gate is green at 733 files / 51 surfaces with every fact-plane rollover
  warning gone. No card and no decision record was edited.
  Commit: `FACT-CARD-CAPACITY-HEADROOM.3 — re-derive the fact plane as one capacity profile`

- ID: `FACT-CARD-CAPACITY-HEADROOM.3a`
  Status: `done` (`2026-08-11`, DOCTRINE)
  Goal: retire the four consumed `ceiling_increase_authorities` records. The containment protocol rejects a
  banked authority, and once `.3` is committed its increases are history, so the records must not outlive them.
  Acceptance: `the four increase records are removed, no ceiling or target moves, and the live-document gate passes against the committed .3 boundary`
  Verification: against committed `.3` the gate reported exactly the predicted four
  `unused or banked ceiling-increase authority` violations, which is the protocol working — the increase is in
  history, so the authority that licensed it is spent. Removing the four `increase` records leaves the registry
  meta record alone; no ceiling, health target, milestone, or surface field moved. The gate is green again at
  734 files / 51 surfaces with no fact-plane pressure.
  Commit: `FACT-CARD-CAPACITY-HEADROOM.3a — retire the consumed ceiling-increase authorities`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | — | — | The tree is closed. `.0`, `.2`, `.2a`, `.3`, and `.3a` are done and `.1` is superseded; capacity is 336 cards / 44 decision records / 379 facts with no fact-plane pressure. Work returns to `CORPUS-COVERAGE.2.51` |
| — | — | — | **Superseded as a current reading (`2026-09-16`).** That profile has since been consumed: the plane is **302 of 336 cards = 89.9%**, about four active days of headroom, and ADR 0041 has already moved the decision join to 58 records / 393 facts. The closing line above describes `2026-08-11`, not today. `LIVE-DOCUMENT-PRESSURE-HEADROOM.24` re-measured it and owns the remedy (`.24a`/`.24b`); this tree stays closed |

## Decisions

- `2026-08-10`: track this as its own tree rather than reopening the closed `FACT-CARD-CATALOG-CONTAINMENT`.
  That tree solved the *browse plane* (a monolithic landing) and closed on its own evidence; this is a
  different problem — the *capacity* of the fact plane itself — and it binds on a contract that tree does not
  own (`max_facts`, in the knowledge-map shard contract).
- `2026-08-10` (ADR 0026): re-derive the bounds to the measured structural join, make the landing O(parts)
  rather than O(cards), and never delete or merge a card to buy capacity.
- `2026-08-10` (ADR 0027, correcting ADR 0026 decision 1): there is no join to re-derive to. The checker treats
  ≥ 90% of a health target as a mandatory-rollover **error**, so full capacity at 198 cards (201/224 lines,
  89.7%) is already the exact edge, 199 fails closed by an existing regression, and a raise requires reshaping
  the projection first. Caught before any contract, card, or generated surface was edited.
- `2026-08-11` (ADR 0029): pressure belongs on a dimension that has a remedy. For a collection that may never
  be deleted or rolled over, every aggregate line/byte bound is the file bound times the per-file bound, so a
  corpus of individually-legal files can never be refused; capacity is then set from the measured population
  against the doctrine's own 80/90 milestones, and the whole profile derives from a single free parameter
  (`max_parts`) whose identities the self-test asserts.

## Open Questions

- Answered by `.0`: compaction is not a lever (two superseded cards out of 193), and the binding join is the
  landing's linear growth, not the file ceiling everyone would reach for first.
- Answered by `.2`: the landing publishes literal first/last ids, not an alphabetic partition rule. The landing
  is regenerated on every card addition regardless (its count line and at least one boundary id move), so the
  rule's only advantage — stability across rewrites — buys nothing, while the literal range is exactly
  re-derivable from the card list and is now checked that way.
- Still open: `max_facts` counts `answers:`-bearing decision records alongside cards. ADR 0026 reserves eight
  slots for them (three exist). If decision records ever grow quickly, that reservation needs its own measure.

## Blockers

- None. The tree is closed; the pressure it tracked never became a failure.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-10` | ownership | `perl scripts/check_fact_card_catalog.pl --report`; `doctrine/knowledge_map/shard_contract.json` | 193 cards / 196 facts; `max_cards` 198, `max_facts` 200; landing at 87.5% of health with an active warning |
| `2026-08-10` | `.0` census | catalog report, shard contract, `check_fact_card_catalog.pl` pinned literals, `git log --diff-filter=A` over `docs/knowledge/`, `status:` distribution | four authorities enumerated with their pinned readers; landing law `lines = cards + 3`; landing-derived join first read as 221 cards, corrected to the exact 198 by the next row; burn 23/25/20 cards per active day; two superseded cards |
| `2026-08-10` | `.0` no mutation | `git status --short` | only the tree, ADR 0026, its index row, and the resume pointer changed; no contract, card, or generated projection touched |
| `2026-08-10` | `.0` correction (ADR 0027) | set `max_cards => 199` in `fixed_limits()`, run `perl scripts/check_fact_card_catalog.pl --self-test`, restore | `199-card ceiling did not fail closed` — the raise is rejected by an existing exact-boundary regression; restored file is byte-identical, and 41/41 cases pass again |
| `2026-08-10` | `.2` shape | `perl scripts/check_fact_card_catalog.pl --print-plan`, `--write`, `--check` | landing 196 lines / 15,417 bytes → 10 lines / 878 bytes / 190 max-line bytes at 193 cards; all four title-part digests unchanged; contract `planned_outputs` re-pinned |
| `2026-08-10` | `.2` fixed-size law | `perl scripts/check_fact_card_catalog.pl --self-test` | 49/49 cases; `lines == parts + 6` holds at 1, 57, and 198 cards; the 198-card capacity render crosses no mandatory pressure and 199 still fails closed |
| `2026-08-10` | `.2` route proof | `perl scripts/test_live_document_size.pl`; `bash scripts/check_live_document_size.sh` | 64/64 cases including eight `routed_membership` cases; the real gate passes at 731 files / 51 surfaces, and the catalog's own 87.5%-of-health landing warning is gone (17 → 16 warnings) |
| `2026-08-10` | `.2` no content change | `git status --short`; `git diff --stat docs/knowledge-catalog/` | only the landing, contract, registry, checkers, tests, and prose changed; no card front matter and no title part touched |
| `2026-08-10` | `.2a` blind-authority census | `perl scripts/check_live_document_size.pl --no-history --report` against the registry ceilings | four surfaces declare a `file` locator over a multi-file glob; `knowledge_cards` `lines_total` 9,773/10,000 (97.7%) and `bytes_total` 853,926/1,048,576 (81.4%) were enforced but never reported; `task_evidence` 74.5%/67.7%, `decision_records` 55.9%/53.7%, `fsmgen_issue_packets` 5.7%/6.0% |
| `2026-08-10` | `.2a` first-pass correction | compared a shell-glob approximation with the checker's `glob_regex` | the approximation let `*` cross `/` and mis-scored `task_evidence` at 147 files / 88.1%; the checker's 130 files / 74.5% is authoritative and is what the ADR records |
| `2026-08-10` | `.2a` ledger rollover | `perl scripts/check_rolling_ledger_protocol.pl --rollover-plan docs/research/fact-card-capacity-headroom-2a-changes-rollover-plan.jsonl` then `--apply-rollover` | this leaf's required `CHANGES.md` entry crossed the 90% record threshold (116/128); the minimal exact cut is 15 records — 14 clears records but leaves lines at 80.3% — sealing `changes-0004` (15 records / 184 lines / 14,899 bytes) and returning the live root to 101 records / 1,433 lines / 203,366 bytes, every dimension below 80% |
| `2026-08-10` | `.2a` fix | `perl scripts/test_live_document_size.pl`; `bash scripts/check_live_document_size.sh` | 67/67 cases; a `file` locator over two paths is rejected, a collection at 91% of its aggregate line target warns, a genuinely single-file surface stays exempt; the real gate reports the two previously-silent `knowledge_cards` aggregates and still exits 0 |
| `2026-08-11` | `.3` reachability | measured card corpus (195 files: 193 cards + README + INDEX) against every declared bound | 9,747 card lines / 851,983 bytes; mean 50.50 lines / 4,414 bytes; max card 243 lines / 29,329 bytes. **198 cards need 10,026 lines against a 10,000 ceiling** — the advertised capacity was unreachable, and with cards never deleted or rolled over the breach had no legal exit. `decision_records` (4,000 vs 32 × 512 legal lines) and `fact_index` (267,938 bytes vs a 262,144 health target — already over) share the shape |
| `2026-08-11` | `.3` burn | `git log --diff-filter=A` over `docs/knowledge/*.md` and `docs/decisions/0*.md` | cards: 196 created over 20 active days — median 8, p90 20, **peak 25**; records: 29 over 7 active days, **peak 9**. Both peaks feed the milestone rule directly |
| `2026-08-11` | `.3` derivation | ADR 0029 profile against each dimension's own milestone | binding dimension is the part-file count, not the card count: four rendered parts must stay under 80% of permitted parts → `max_parts` 6 → `max_cards` 336 (the card rule alone would have allowed 5 parts / 280). `knowledge_cards.files` 338 = 336 + 2; aggregates 338 × 300 lines / 338 × 36,864 bytes; `decision_records.files` 44 = ⌈39/0.90⌉; `max_facts` 379 = 336 + 43; `max_question_keys` 3,072 = 379 × the bundle's own declared 8-keys-per-fact ratio, rounded to its 512 step |
| `2026-08-11` | `.3` both-side pins | `perl scripts/check_fact_card_catalog.pl --self-test`, `--check` | 58/58 (was 49). Eight new inline assertions pin the identities themselves — `max_cards = cards_per_part × max_parts`, each aggregate band = files × that band's per-file bound, projection ceiling = landing ceiling + part aggregate. The 336-card capacity render crosses no mandatory pressure; 337 fails closed. `max_facts` is derived from the `decision_records` ceiling, with a fail-closed case for each writer drifting alone |
| `2026-08-11` | `.3` gates | `bash scripts/check_live_document_size.sh`; `bash knowledge-map/scripts/check_knowledge_map.sh`; `perl scripts/check_knowledge_map_shard_contract.pl --check` | green at 733 files / 51 surfaces; 67/67 live-document cases; every fact-plane rollover warning gone (files 195/338 = 57.7%, records 30/44 = 68.2%, projection lines 2,976/12,384 = 24.0%). Contract feasible for 199 facts / 1,459 keys in 10 shards. The surviving `knowledge_cards` `lines_each` warning (81.0%) is one 243-line card, a per-card signal this leaf may not touch |
| `2026-08-11` | `.3` no content change | `git status --short`; `git diff --stat docs/knowledge/ docs/decisions/` | only contracts, the registry, the checker, ADR 0029 + its index row, the regenerated question projection, and prose changed; no card front matter, no title part, and no existing decision record touched |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `FACT-CARD-CAPACITY-HEADROOM` ownership | `FACT-CARD-CAPACITY-HEADROOM — track measured fact-plane capacity pressure` | surfaced by `CORPUS-CHAIN-CURRENCY.2` |
| `FACT-CARD-CAPACITY-HEADROOM.0` | `FACT-CARD-CAPACITY-HEADROOM.0 — measure the fact plane and decide its capacity law` | measurement + ADR 0026 only |
| `FACT-CARD-CAPACITY-HEADROOM.1` | `superseded` | premise falsified by ADR 0027 before any implementation |
| `FACT-CARD-CAPACITY-HEADROOM.2` | `FACT-CARD-CAPACITY-HEADROOM.2 — make the fact-card landing a fixed-size router` | shape only; no limit moved and no card content changed |
| `FACT-CARD-CAPACITY-HEADROOM.2a` | `FACT-CARD-CAPACITY-HEADROOM.2a — make aggregate pressure visible before re-deriving capacity` | ADR 0028; visibility only, no ceiling moved; carries the `changes-0004` ledger rollover its own entry required |
| `FACT-CARD-CAPACITY-HEADROOM.3` | `FACT-CARD-CAPACITY-HEADROOM.3 — re-derive the fact plane as one capacity profile` | ADR 0029; four consumed `ceiling_increase_authorities` records travel with it |
| `FACT-CARD-CAPACITY-HEADROOM.3a` | `FACT-CARD-CAPACITY-HEADROOM.3a — retire the consumed ceiling-increase authorities` | the increase is history once `.3` commits, so its authority is spent |

## Changelog

- `2026-08-10`: Created task tree from pressure measured while `CORPUS-CHAIN-CURRENCY.2` added a fact card.
- `2026-08-10`: `.0` closed with the four-authority census; ADR 0026 accepted; `.1` and `.2` added.
- `2026-08-10`: ADR 0027 corrected ADR 0026 decision 1 against the checker's mandatory-rollover rule; `.1`
  superseded, `.2` promoted to the frontier, `.3` added for the re-derived raise.
- `2026-08-10`: `.2` closed. The landing is a fixed-size router over the title parts; the containment doctrine
  gained a `routed_membership` index kind with a fixed one-hop completeness proof. Capacity is deliberately
  unchanged — `.3` owns the re-derivation.
- `2026-08-10`: `.3`'s opening measurement found a fifth authority the `.0` census missed and that no warning
  could have surfaced — `knowledge_cards.lines_total` at 97.7% of a hard ceiling. `.2a` inserted before `.3`,
  ADR 0028 accepted, and the blindness fixed before any bound is re-derived.
- `2026-08-11`: `.3` closed. Re-measuring against every now-visible authority found that the advertised
  198-card capacity was never reachable — its own aggregate line ceiling refused it — and that the breach had
  no legal exit. ADR 0029 accepted: pressure belongs on a dimension with a remedy, aggregates are the file
  bound times the per-file bound, and the whole profile derives from `max_parts`. Capacity is now 336 cards /
  44 decision records / 379 facts with every fact-plane rollover warning gone. `.3a` added to retire the
  consumed ceiling-increase authorities before the tree closes.
- `2026-08-11`: `.3a` closed and the tree with it. The four ceiling-increase authorities `.3` consumed are
  retired; a banked authority fails the containment gate by design, and the gate is green again with no
  fact-plane pressure. Frontier returns to `CORPUS-COVERAGE.2.51`.
