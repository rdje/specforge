# LIVE-DOCUMENT-PRESSURE-HEADROOM — knowledge and fact plane

- Part ID: `knowledge-and-fact-plane`
- State: `legacy`

<!-- pressure-headroom-task-source-region:fact-card-split:start -->
- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`
  Status: `done`
  Goal: restore writable headroom for the production-genericity fact card
  Acceptance: current and immutable fact roles are separated losslessly through the existing catalog/map
  lifecycle; every answer route remains exact; no card bound moves; the next ordinary fact update succeeds
  **Measured `2026-09-16`, so the next session starts from structure rather than from the file.**
  `docs/knowledge/production-genericity-boundary.md` is **296 of the 300-line `knowledge_cards`
  `lines_each` ceiling — 4 lines**, and it is the plane's maximum by a wide margin (the next largest
  card is `transaction-capture-census.md` at 243). It is **73 lines of front matter carrying 64
  `answers:` keys** over **223 lines of body in 31 paragraphs**, and those two halves are what make the
  split shape obvious: the body is a chronological narrative of one remediation program, running
  SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapter -> implementation digests -> flow layer
  -> final qualification.
  **The role cut, stated as a hypothesis to verify rather than a conclusion.** Roughly, paragraphs 1,
  8-9 and 24-30 state the rule that still governs new code — what neutrality means, why a forbidden
  vocabulary list is insufficient, the opacity and proof-admission substrate, the closed flow registry
  and the composed doctrine — while paragraphs 2-7, 10-23 and 31 record how each stage was migrated and
  what each migration measured. Confirm that by mapping each of the 64 answer keys to the paragraph
  that actually answers it, then assigning by paragraph; do not assign answers by reading their
  wording, because several of them (for example the proof-ledger summary at key 22, and the prompt and
  corpus-KB rules at keys 11-13) read like history and state current rules, or the reverse.
  **The hazard specific to this split, which is why it was deferred rather than rushed.** The gates
  check the wrong thing for this decision. The Knowledge Map generator verifies that question keys are
  unique and that the landing and shards derive exactly, so a duplicated or dropped answer fails
  closed — losslessness is mechanically guaranteed. **Nothing checks that an answer landed on the
  card that answers it.** A mis-assigned key sends a future session to a card that does not contain
  its answer, and the projection will be perfectly green. So the assignment needs a paragraph-level
  mapping as evidence, not a reading.
  **Mechanical checks to run on the result**: the union of both cards' `answers:` equals today's 64
  with no duplicate; every body paragraph appears byte-identically in exactly one card; both cards are
  below the 300-line ceiling with band room; and the catalog chain is refreshed (map regeneration,
  `fact_card_catalog.json` `planned_outputs`, the `fact-card-catalog-count` assertion and its
  `docs/knowledge/INDEX.md` line-3 region, then the claim digests).
  **Executed `2026-09-16`, and the measurement's own hypothesis was corrected before it was applied.**
  The proposed ROLE cut (paragraphs 1, 8-9 and 24-30 current rule; the rest history) does not survive the
  key mapping. Paragraphs 2-7 answer present-tense neutrality questions (keys 6-13), paragraphs 11-23
  answer present-tense proof questions (`How is SemanticIR proof-carrying?`), and paragraph 30 is a
  measurement sitting inside the proposed rule run — so role is not the axis these 64 keys separate on.
  The axis they do separate on is SUBJECT, and it falls on one contiguous seam the narrative already
  marks: paragraph 8 ends `the following .e.iii slice installs the capability/kernel substrate` and
  paragraph 9 opens with that substrate.
  **The executed split.** Card A keeps `production-genericity-boundary` with **paragraphs 1-8 and 24-31**
  and **29 keys** — what neutrality means, why a denylist is not proof, the identity/spelling remediation
  and its qualification range, the crate boundary, then implementation digests, the syntax graph, the
  closed 141-row flow registry, the composed doctrine and current signoff. Card B is the new
  `proof-carrying-stage-ledger` with **paragraphs 9-23** and **35 keys** — the opacity/kernel substrate,
  the migration contract, and the five completed stage migrations. Each card gains one NEW one-sentence
  bridge paragraph naming the other by `[[id]]`; no moved paragraph is edited.
  **The hazard the leaf named was answered with an oracle, not with a reading.** Each of the 64 keys was
  routed by TF-IDF to one of the **31 original** paragraphs — blind to the card boundary — and the routed
  paragraph's card compared with where the key was placed: **top-1 agrees 53/64, top-3 agrees 64/64, and
  no key has a top-3 that contains no paragraph from its own card.** Every one of the 11 top-1
  disagreements was then resolved by naming the answering sentence, and every one is the collision the
  measurement predicted: keys 6-10 ask about schema **2** and the router sends them to the schema-**3**
  migration paragraphs; key 19's premise list (`typed capture/model/prior/upstream/axiom premises`) is
  verbatim in paragraph 9; keys 28 and 39 land on their own card at rank 2. Key 5 has a POINTER answer
  only — the body never names `PDF-AGNOSTIC-EXTRACTION`, and paragraph 1 routes to the audit that records
  its seeded-check closure at `production-genericity-pipeline-audit.md:451`.
  **Key 7 is the one genuinely two-paragraph key** (`What happens when schema-1 SourceIR is loaded?`):
  paragraph 2 answers it directly and says why, paragraph 12 adds the current inspection-API restriction.
  It goes with paragraph 2 on card A because paragraph 12 already owns key 25, and the bridge carries a
  reader to the other half.
  Verification: `losslessness proved mechanically against HEAD: 31 of 31 original paragraphs appear byte-identically in exactly one card, answers union equals the original 64 with no duplicate and no overlap, 2 new bridge paragraphs; assignment oracle 53/64 top-1 and 64/64 top-3 with all 11 disagreements resolved by named sentence; knowledge_cards lines_each 296/300 (98.7%, rollover) -> 243/300 (81.0%, warning) with the maximum relocating to transaction-capture-census.md; card A 148 lines / card B 164 lines; the 64-answer per-card cap is no longer at its bound (29 and 35); map regenerates to 325 facts / 2,748 keys UNCHANGED; catalog 301 -> 302 cards; all 15 executed gate-tier doctrines PASS`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.1 — split the oversized genericity card on the seam its own narrative marks`

<!-- pressure-headroom-task-source-region:fact-card-split:end -->

<!-- pressure-headroom-task-source-region:fact-plane-capacity-nodes:start -->
- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.23`
  Status: `done`
  Goal: record the per-file JSON encoder style of the doctrine contracts, because there is no repository-wide one
  Acceptance: an agent about to rewrite one field of a doctrine contract can learn its exact encoder from a fact
  card plus one cheap command, instead of discovering the style from a wholesale reformat that the diff then
  hides inside hundreds of unrelated lines
  **Measured `2026-09-16` while `.1` refreshed `fact_card_catalog.json` `planned_outputs`.** `MEMORY.md` carries
  the blanket rule "edit that contract with Perl `JSON::PP` pretty+canonical, NOT Python", and for this file that
  rule is **wrong**: `pretty(1)` also sets `space_before`, so it emits `"key" : value` while the committed file
  carries `"key": value`. The exact encoder is `canonical(1)->indent(1)->space_after(1)` with
  `indent_length(1)`. A round-trip census over the 27 parseable JSON contracts under `doctrine/` and
  `docs/catalogs/` shows **no repository-wide convention at all**: three task-evidence contracts round-trip
  under `JSON::PP` `pretty`+`canonical`, `fact_card_catalog.json` round-trips under the tight encoder above,
  and the remaining 23 reproduce under neither. So the generalization in the resume pointer is not merely
  imprecise for one file — it does not hold for the population, and the safe procedure is to DERIVE the
  encoder per file by round-tripping candidates before writing
  **Executed `2026-09-16`, and the census corrected the leaf's own framing twice.** First correction: the
  file is not best described as a Perl encoder at all. `fact_card_catalog.json` is exactly
  `json.dumps(indent=1, sort_keys=True, ensure_ascii=True)` plus a trailing newline, which is byte-identical
  to `JSON::PP` `canonical->indent(1)->space_after(1)` with `indent_length(1)`. Second and larger correction:
  **language is the wrong axis entirely.** `JSON::PP` `pretty` differs from `json.dumps` in exactly two
  dimensions — it sets `space_before` and defaults `indent_length` to 3 — so once separators are matched a
  single Python command reproduces **23 of the 27** contracts, including all three that `JSON::PP` wrote.
  What actually decides the bytes is per file: indent width, key order, `ensure_ascii`, `space_before`, and
  the trailing newline.
  **Three classes, not two.** 20 contracts are `json.dumps` with `(',', ': ')` and a trailing newline (15 at
  the plain `indent=2`, insertion-ordered, ASCII-escaped default, plus four variants); 3 are `JSON::PP`
  `canonical`+`pretty` and alone carry `"key" : value`; and **4 are hand-authored** —
  `fsmgen_feedback.json`, `roadmap_projection.json`, `spec_to_intent_vertical_eval_schema.json`,
  `trajectory_controller_input_schema.json` — keeping small objects inline on one line, which no encoder
  emits. Re-encoding one field of those expands them **9-27%** and destroys the layout, so they are text
  edits. That class is the reason the remedy is a derivation command and not a canonical formatter.
  **Routed to `[[live-surface-edit-bookkeeping-chain]]` rather than to a new card**, deliberately: the fact
  is a sub-fact of the chain that already answers "what must I update after adding a fact card", and the
  fact plane is at **304/338 files = 89.9%**, one card below its own 90% rollover milestone (`.24` owns
  that). Four answer keys and the derivation command were added to the existing card; it is 122 lines of a
  300 ceiling. No new file, no milestone tripped for a sub-fact.
  Verification: `census re-derived over all 27 parseable doctrine JSON contracts under two encoder families; the command the card publishes was EXECUTED over the whole population and reproduces 23 of 27 byte-for-byte while naming exactly the 4 hand-authored files; card 84 -> 122 lines, 10 -> 14 answer keys; all 15 executed gate-tier doctrines PASS`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.23 — derive a JSON contract's encoder instead of assuming the repository has one`
  Prerequisite: none; found by `.1` while executing the catalog-refresh chain

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.24`
  Status: `done`
  Children: `.24a`, `.24b`
  Goal: give the fact-card plane a lifecycle that relieves the axis under pressure, or say why it cannot
  Acceptance: `knowledge_cards.files` either gains a remedy compliant work can take, or the bound is
  re-derived against measured demand under an authority; the declared owner is current either way
  **Measured `2026-09-16` while `.1` and `.23` moved the plane.** `knowledge_cards.files` is **304 of 338 =
  89.9%**, with `health_targets.files` and `enforcement_ceilings.files` BOTH 338 — so, like `.2`'s task
  plane and `.4`'s research plane, there is no warning band, and **the very next card crosses the 90%
  rollover milestone** (305/338 = 90.2%). The catalog carries its own second enforcer at `max_cards: 336`
  in `scripts/check_fact_card_catalog.pl` `fixed_limits()`, so the bound has two homes again — the same
  two-enforcer shape `.2a` found for `$MAX_TASKS`, and both would have to move together.
  **The lifecycle that exists does not relieve the axis that is pressured.** A card can be `superseded`,
  and **11 of the 302 already are** — but a superseded card is still a file, so supersession relieves
  `answers:` routing and never `files`. There is no declared rollover, archive, or retirement for the card
  plane. That is the exact `LIVE-DOC-STOP-RISK` condition this tree keeps re-finding: a bound the surface
  can actually reach with no move compliant work can make.
  **The declared owner is closed and its closing statement is stale.** `FACT-CARD-CAPACITY-HEADROOM` closed
  on `2026-08-11` recording "capacity is 336 cards / 44 decision records / 379 facts with no fact-plane
  pressure". Cards have since gone to 302 of 336. Decide here whether that tree reopens or this leaf owns
  the remedy; do not re-derive the capacity without reading its `.2`/`.3` first
  **Measured `2026-09-16`, and the authority that binds is NOT the one that warns.** Seven authorities bound
  this plane; here is every one, with the enforcer that would refuse:

  | Authority | Enforcer | Current | Bound | % |
  | --- | --- | ---: | ---: | ---: |
  | card slots | `check_fact_card_catalog.pl` `fixed_limits()` `max_cards` | 302 | 336 | **89.9%** |
  | card files | `knowledge_cards.files` (health == ceiling) | 304 | 338 | **89.9%** |
  | facts | `shard_contract.max_facts`, derived `336 + (58 - 1)` | 325 | 393 | 82.7% |
  | question keys | `shard_contract.max_question_keys` | 2,752 | 3,584 | 76.8% |
  | shards | `shard_contract.max_shards` | 19 | 32 | 59.4% |
  | rendered title parts | `fixed_limits()` `max_parts` | 6 | 6 | **100%, exempt** |
  | portable caps | `check_knowledge_map_shard_contract.pl:107-118` | — | 512 facts / 4,096 keys / 64 shards | — |

  **ADR 0029's own sizing law is violated.** `FACT-CARD-CAPACITY-HEADROOM.3` set the profile so the measured
  population sits below the 80% warning and one measured peak day sits below the 90% rollover. The population
  alone is now **89.9%**, and the measured peak day exceeds the entire remaining allowance.
  **Headroom is 34 cards.** Creation measured from Git over the 21 active days since `2026-08-08` is **177
  cards, mean 8.43 per active day, peak 25** (`2026-08-09`; the recent 30-day peak is 14). So the plane has
  roughly **four active days**, or **1.4 peak days**, before `check_fact_card_catalog.pl` refuses the 337th card
  inside whatever unrelated slice happens to write it — the exact failure `FACT-CARD-CAPACITY-HEADROOM` was
  opened to prevent.
  **The projection reports NO pressure at any point up to that stop, and this was rendered rather than argued.**
  34 probe cards were written into the real tree, `--print-plan` (read-only) was run at exactly 336 cards, and
  the probes were removed with `git status` clean: landing **12/224 = 5.4%**, parts `lines_each` **63/80 =
  78.8%**, `lines_total` **378/480 = 78.8%**, `bytes_total` **73,928/147,456 = 50.1%**, `line_bytes_each`
  **288/384 = 75.0%** — every dimension under its 80% warning at full capacity. The seventh, `files` **6/6**,
  is **exempt by construction**: `collection_pressure_findings` skips `files` when actual equals target. So the
  only signal before refusal is the `knowledge_cards.files` surface warning, on a surface whose health equals
  its ceiling and which therefore has no warning band of its own either.
  **`max_parts` is re-confirmed as the only free parameter, by derivation and not by inheritance.** A rendered
  part is `7 + cards` lines against a `lines_each` health of 80, so a full part at `cards_per_part: 56` is
  **63/80 = 78.8%** and at 57 it is **64/80 = exactly 80.0%** — the warning. **56 is therefore the exact
  maximum this shape permits**, the same way `.0` found 198 to be exact, and `cards_per_part` is not a lever.
  **Decision.** Raise `max_parts` **6 -> 7** in one coupled transaction. Under ADR 0041's law the smallest
  sufficient capacity is `C >= 378` (population `302/C < 80%` gives `C > 377.5`; population plus one peak day
  `327/C < 90%` gives `C > 363.3`), and slots are `max_parts x 56`, so **7 parts = 392 slots**: `302/392 =
  77.0%` and `327/392 = 83.4%`. Six parts cannot satisfy it at 89.9%. Eight parts is NOT selected: it banks
  unmeasured authority, which ADR 0041 §3 already refused for the decision plane.
  **The join `.24a` must solve before it moves anything.** At 392 slots `max_facts` becomes `392 + (58 - 1) =
  449` of a portable 512 (87.7%), and at ADR 0041's assumed eight keys per fact the key budget is `449 x 8 =
  3,592`, which its 512-key rounding step lifts to **4,096 — exactly the portable cap**. A bound equal to its
  cap is health == enforcement, which `.22d` refused on this same tree. The assumed ratio is also stale: the
  measured ratio today is **2,752 / 325 = 8.47 keys per fact**, projecting 449 facts to about **3,803** keys.
  `.24a` re-derives the ratio from the current population and sizes the key budget BELOW the portable cap
  rather than onto it; if no rounding step does that, the rounding step is what changes, not the cap.
  **Compaction is still not a lever, re-measured rather than assumed.** **11 of 302** cards are `superseded`
  and each still occupies a file and a slot, so supersession relieves `answers:` routing and never `files`.
  Both this tree's Non-Goals and `FACT-CARD-CAPACITY-HEADROOM`'s forbid deleting a card for capacity.
  **`decision_records` moves with it**: 48/58 files = 82.8%, already warning, and `max_facts` derives from that
  ceiling, so `.24a` must state the decision-plane join explicitly instead of treating 58 as constant.
  Verification: `seven authorities enumerated with their enforcers; full-capacity projection RENDERED with the real checker at exactly 336 cards and every reported dimension below 80% while the files dimension is exempt at equality; creation re-derived from Git as 177 cards over 21 active days, mean 8.43, peak 25; cards_per_part proved exact at 56 because 57 lands on 80.0%; the portable 512/4,096/64 caps confirmed ENFORCED at check_knowledge_map_shard_contract.pl:107-118 rather than merely asserted by ADR 0041; probe cards removed with a clean tree`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.24 — measure the fact plane against its own sizing law before moving a bound`
  Prerequisite: none; measured by `.23` while choosing a home for its fact

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.24a`
  Status: `done`
  Goal: execute the coupled fact-plane profile at `max_parts: 7` in one transaction
  Acceptance: `max_parts` 6 -> 7, `max_cards` 336 -> 392, `knowledge_cards.files` 338 -> 394, `title_parts`
  files and both aggregate bands re-derived as `files x per-file`, `max_facts` re-derived from the card slots
  and the current decision ceiling, and `max_question_keys` re-derived from the MEASURED keys-per-fact ratio and
  sized strictly below the portable 4,096 cap; one exact `ceiling_increase_authorities` record carries the whole
  raise; a full-capacity render crosses no mandatory pressure and one card beyond it fails closed; no card and
  no decision record is edited; every join rolls back together if any fails
  Prerequisite: `.24`'s decision; read ADR 0029 and ADR 0041 first, and `FACT-CARD-CAPACITY-HEADROOM.3`'s
  self-test identities, which assert the derived profile from both sides
  **Executed `2026-09-16` as one transaction over six files and two authorities.** `max_parts` **6 -> 7** is
  the only value chosen; everything else derives and every derivation is asserted from both sides by the
  checker's own self-test: `max_cards` **336 -> 392** (`cards_per_part x max_parts`), `knowledge_cards.files`
  **338 -> 394** (the single anchor, `max_cards + 2`), both aggregate bands as `files x per-file`
  (**101,400 -> 118,200** lines, **12,460,032 -> 14,524,416** bytes), `fact_card_titles` **6 -> 7** files with
  health **480 -> 560** / **147,456 -> 172,032** and ceiling **576 -> 672** / **196,608 -> 229,376**,
  `projection_ceiling` **7 -> 8** files / **832 -> 928** lines / **229,376 -> 262,144** bytes,
  `max_facts` **393 -> 449** and `max_question_keys` **3,584 -> 4,096**. Two `increase` records carry it, one
  per raised surface; `.24b` retires both.
  **The raise did something the measurement did not predict: it gave the part dimension a warning band it
  never had.** At 6 of 6 parts the `files` dimension was exempt by the `actual == target` rule and reported
  nothing; at 6 of 7 it reports **85.7%**. Meanwhile `knowledge_cards.files` falls **89.9% -> 77.2%**. So the
  plane is quieter where it was loud and audible where it was mute — but note the exemption returns at full
  capacity, which is why `.25` is opened rather than noted.
  **Seven parts is the LAST raise this bundle permits, and that is the durable half of this leaf.** An eighth
  part would declare `448 + 57 = 505` facts; at the measured **8.47** keys per fact that is **4,277** keys
  against the portable **4,096** cap enforced at `check_knowledge_map_shard_contract.pl:107-118` — a declared
  capacity the projection could not reach, which is precisely the defect ADR 0029 exists to prevent. At seven
  parts the same arithmetic is comfortable: 449 facts project to **3,803** keys (92.8% of the cap), about
  **28 of 32** shards, and `fact_index` **29 of 33** files. The next capacity question is therefore not "add a
  part" but "move the portable bundle cap", which is a change to the `knowledge-map/` contract rather than to
  this repository's profile.
  **Both edited JSON contracts kept their own encoder**, derived per file by `.23`'s command rather than
  assumed: `fact_card_catalog.json` is `indent=1, sort_keys=True` and `shard_contract.json` is `indent=2`,
  insertion-ordered — so the raise is 13 changed lines across the two files instead of a wholesale reformat.
  Verification: `self-test 60/60 with five literals replaced by derivations (the 337-card overflow bound, its diagnostic, the capacity message, the capacity part count, and the surface-file fixture, which now states the 394 anchor once via fact_files()); catalog --check valid for 302 cards; shard contract feasible for 325 facts / 2,752 keys in 19 shards; knowledge-map derive-and-diff OK; knowledge_cards files 89.9% -> 77.2%; fact_card_titles files silent-at-6/6 -> reported 85.7%; all 15 executed gate-tier doctrines PASS`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.24a — raise the fact plane to seven title parts, the last raise this bundle permits`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.24b`
  Status: `done`
  Goal: retire the authority `.24a` consumes
  Acceptance: the banked-authority refusal is observed RED on the real tree before the record is removed, then
  green after, with no ceiling, target, or milestone moving — the protocol proven in both directions as `.4b`,
  `.22c` and `.22g` each did
  **Executed `2026-09-16`, and this is the first time the protocol has been exercised on TWO authorities at
  once.** `.4b`, `.22c` and `.22g` each retired exactly one record, so "the refusal fires per record rather
  than per commit" was an untested reading of the loop. Run against committed `49be08b3` before touching the
  registry, the gate named **both** by surface id — `'knowledge_cards' has unused or banked ceiling-increase
  authority` and `'fact_card_titles' has unused or banked ceiling-increase authority`, exit 1, two
  violations — so a commit that retired only one would still have been refused. Removing both leaves the
  registry meta record alone, **3 records -> 1**, and the gate is green at **993 Markdown files / 61 governed
  surfaces**. The two surfaces' four bands were captured before and after and are **byte-identical**, so the
  permission expired and the capacity did not.
  Verification: `RED observed first at 49be08b3 naming both surfaces, exit 1 / 2 violations; authorities 3 records -> 1; knowledge_cards and fact_card_titles health and ceiling objects byte-identical before and after; live-size green at 993 files / 61 surfaces; all 15 executed gate-tier doctrines PASS`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.24b — retire both consumed fact-plane authorities`
  Prerequisite: `.24a` committed

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.25`
  Status: `done`
  Goal: stop the `files` dimension going silent at exactly the moment it is full
  Acceptance: a collection whose file count is a consumable budget reports pressure at 100% like every other
  dimension, while a collection whose file count is constant by construction stays quiet for a stated reason
  rather than by an unnamed special case; each arm has a fail-closed test
  **Found `2026-09-16` by `.24`/`.24a`, in TWO independent checkers.**
  `scripts/check_live_document_size.pl:884` and `scripts/check_fact_card_catalog.pl:1185` both carry
  `next if $dimension eq 'files' && $actual == $target`, so a collection sitting at exactly its declared file
  count reports nothing at all. Every other `next` in that same generic loop names the ADR that decided it —
  the one-file aggregate rule cites ADR 0028, the frozen/archive skip and the transition-debt skip are
  explained, the headroom suffix cites ADR 0032 — and **this one is the only undocumented exemption in the
  block**.
  **Its observable effect, measured rather than argued.** Before `.24a`, `fact_card_titles` stood at **6 of 6
  parts** and the generic gate printed nothing for it; after `.24a` it stands at 6 of 7 and prints
  **85.7%**. Nothing about the surface's real fullness improved — it got emptier — yet it went from silent to
  warning. And `.24`'s full-capacity render showed the same property on the catalog side: at exactly 336 cards
  every reported dimension was under its 80% warning because the one at 100% was skipped.
  **The likely intent is legitimate and should be preserved.** A surface whose file count is fixed by
  construction — a four-file bundle that will always be four files — would otherwise warn forever, which is
  noise, and ADR 0028 already established that this class of decision must be made from MEASUREMENT rather
  than from a declared locator. The defect is that the current rule cannot tell a constant from a consumed
  budget, so it silences both. Decide the distinction explicitly (a declared field, or health < ceiling as the
  signal that the count is a budget) rather than inferring it from equality
  Prerequisite: none; found by `.24a` while raising the two bounds this exemption hid
  **Executed `2026-09-16`, and the first attempt was wrong in a way worth recording.** Narrowing the skip to
  derived projections alone — `canonical_inputs` + `freshness_verifier` — unsilenced **24** single-file
  surfaces at once (`README.md`, `MEMORY.md`, `ROADMAP.md`, every root pointer), each reporting
  `files is at or above rollover (100.0%) — 0 below its 1 ceiling` forever. That is the noise the blanket
  skip existed to suppress, and it proves the exemption had **two** legitimate cases, not one.
  **The measured discriminator.** A surface's file count is a BUDGET when ordinary work can consume it, and a
  CONSTANT when it cannot. Measured over the registry: **40 surfaces declare only glob-free targets and 21
  declare a glob**, and of the 40, exactly **five** bound themselves above the number they list
  (`workflow_standards` 14 of 21, and the four task-evidence part collections). So neither "has a glob" nor
  "is enumerated" is sufficient alone; the constant case is precisely **`files` bound == number of glob-free
  targets**, where the count cannot move without editing the registry that declares it. The final rule skips
  the `files` dimension at equality only for a derived projection or an exactly-enumerated surface, and
  reports every other 100% — including `workflow_standards` when it eventually reaches 21 of 21.
  **Each arm is proved by disabling it.** With the derived arm forced false, case 59 fails; with the
  enumerated arm forced false, case 60 fails; with the whole narrowing reverted to `next;`, case 58 — the
  canonical collection at 100% — fails. Three cases, three independent RED controls, and the suite's declared
  total moves **110 -> 113** so a silently dropped case still fails closed.
  **The catalog checker's copy is correct and is now documented rather than changed.** The only collection
  `check_fact_card_catalog.pl` measures is its own generated title-part projection, whose part count is a
  function of `max_cards`; it is the derived arm by construction.
  **One real warning surfaced, and `.26` owns it rather than the leaf noting it**:
  `rolling_ledger_archive_indexes` at **4 of 4** with health == ceiling == 4.
  Verification: `113/113 live-document cases (declared total 110 -> 113) with a RED control per arm; catalog self-test 60/60; on the real tree the narrowing adds exactly ONE warning, rolling_ledger_archive_indexes at 100.0%, and no other surface changes state; all 15 executed gate-tier doctrines PASS`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.25 — a full collection must not be the one state that reports nothing`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.26`
  Status: `done`
  Children: `.26a`
  Goal: decide what the rolling-ledger archive index count is a bound ON, now that it reports
  Acceptance: the surface either declares the cardinality relationship it actually has, or carries headroom
  derived from the ledger population; either way the next rolling ledger's archive index has a legal move
  **Measured `2026-09-16`, surfaced by `.25`.** `rolling_ledger_archive_indexes` globs
  `docs/archive/rolling-ledgers/*/INDEX.md` and stands at **4 of 4 files, health == ceiling == 4**, so there
  is no warning band and the next index is a hard refusal. It was silent until `.25` narrowed the equality
  skip.
  **The count is not arbitrary — it is one index per declared rolling ledger.** `rolling_ledgers.jsonl`
  declares exactly four (`changes`, `development-notes`, `live-achievement-status`,
  `rust-codebase-analysis`) and exactly four index files exist. So the bound is a real derived cardinality
  that the registry does not declare as derived, which is why it reads as saturated rather than as exact.
  **The trigger is near, not hypothetical.** A fifth rolling ledger is a legal future act this repository has
  performed repeatedly, and `VALIDATION_SNAPSHOT.md` — already warning at 85% of its line target and owned by
  `.4d.ii` — is the standing candidate. The commit that declares it would be refused by a surface it has
  nothing to do with.
  Decide between (a) declaring the derivation, so the bound follows `rolling_ledgers.jsonl` and 4 of 4 is
  exact rather than full, and (b) sizing the bound from the ledger population under an authority. Do NOT
  simply raise it to quiet the warning `.25` just made visible
  **Executed `2026-09-16`, and neither of the two options this leaf proposed survived.** Option (a),
  declaring the surface derived so `.25`'s new rule exempts it, was **rejected on inspection**:
  `canonical_inputs` and `freshness_verifier` are validated only inside the `generated_projection` branch of
  `check_live_document_size.pl`, so declaring them on a `partitioned_canonical` surface would be an
  **unchecked** declaration whose only effect is to buy silence — the label-decides-silence shape ADR 0028
  exists to prevent. Option (b), sizing from the ledger population, would have bounded the surface at its own
  population again. Enumerating the four literal paths was rejected too: it duplicates the `archive.index`
  path every ledger record already names.
  **What the measurement actually showed.** `rolling_ledgers.jsonl` names each ledger's `archive.index`
  explicitly and there are exactly four records for four index files, so the relationship is 1:1 and already
  declared. And the registry header already caps how many rolling ledgers may exist at all:
  **`max_records: 8`**. So `files: 4` was never a capacity — it was today's population, and a bound equal to
  its population has no headroom **by construction**, which is the defect ADR 0029 names rather than the
  state.
  **Remedy: bind the bound to the cap, and check the identity.** `rolling_ledger_archive_indexes` files
  **4 -> 8** in both bands with the aggregates re-derived as `files x per-file` (health **256 -> 512** lines /
  **32,768 -> 65,536** bytes; ceiling **384 -> 768** / **49,152 -> 98,304**), under one authority `.26a`
  retires. `check_rolling_ledger_protocol.pl` — the only file that reads both registries, and the surface's
  own declared verifier — now refuses any disagreement, and additionally refuses an aggregate below
  `files x per-file` and an `archive.index` outside the surface's targets. Run before the raise, it named the
  exact mismatch: `files 4 must equal the ledger registry max_records 8`.
  **This is a relocation, not a silence.** The capacity question "how many rolling ledgers may exist" keeps
  its home on `rolling_ledgers.jsonl` at **4 of 8 = 50%**, banded 80/90 by `.22b`, and the archive-index
  surface now reads 50% instead of 100% because it mirrors that cap rather than the population.
  **Found while adding cases: this checker's self-test had no declared total.** It printed
  `$checks parser/control self-tests pass` with nothing to compare against, so a deleted case would simply
  have reported one fewer — the `PRODUCTION-GRAPH-CENSUS-PIN.3` anti-pattern that
  `check_fact_card_catalog.pl` and `test_live_document_size.pl` both already guard. It now declares **45**
  independently, and perturbing the declaration to 44 exits non-zero. The die message is deliberately pure
  ASCII: this script prints without a UTF-8 layer, and the first draft's em dash rendered as mojibake.
  Verification: `identity observed RED on the real tree naming files 4 vs max_records 8 before any bound moved; four new self-test cases, one per direction of the identity, the aggregate rule and the out-of-surface index; self-test 45/45 with a newly declared total that fails closed at 44; the generic gate's rolling_ledger_archive_indexes 100% warning is replaced by a 50% reading; all 15 executed gate-tier doctrines PASS`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.26 — bind the archive-index bound to the ledger cap instead of to its own population`
  Prerequisite: none; surfaced by `.25`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.26a`
  Status: `done`
  Goal: retire the authority `.26` consumes
  Acceptance: the banked-authority refusal is observed RED on the real tree before the record is removed, then
  green after, with no ceiling, target or milestone moving
  Prerequisite: `.26` committed
  **Executed `2026-09-16`.** Run against committed `3f2f6079` before touching the registry, the gate reported
  `'rolling_ledger_archive_indexes' has unused or banked ceiling-increase authority`, 1 violation. Removing the
  record leaves the meta record alone, **2 -> 1**, and the gate is green at **993 Markdown files / 61 governed
  surfaces** with both bands byte-identical across the removal. Fourth demonstration of the protocol on this
  tree and the first where what the authority licensed was a **derived identity** rather than a chosen number:
  the permission expired, and the equation `check_rolling_ledger_protocol.pl` now enforces did not.
  Verification: `RED observed first at 3f2f6079, 1 violation; authorities 2 records -> 1; health and ceiling byte-identical before and after; live-size green at 993 files / 61 surfaces; all 15 executed gate-tier doctrines PASS`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.26a — retire the consumed archive-index authority`

<!-- pressure-headroom-task-source-region:fact-plane-capacity-nodes:end -->
