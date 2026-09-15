# CLAIM-VERIFICATION-ADOPTION — verification and chronology

- Part ID: `verification-and-chronology`
- State: `legacy`

<!-- claim-verification-task-source-region:verification-log:start -->
## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-30` | `.7.2.1a` | `git rev-list --count e6f5012d..HEAD`; blind-spot probe re-derived at `a88b91a3~1`; `git show 3c17ae5c~1:...` for the registry and the book chapter's tag count; four `git show <rev> -- <census> | grep -c '^-{'` runs; `check_rolling_ledger_protocol.pl --report`; `--produce` on both census producers; `scripts/check_doctrines.sh` | **asked a third time, and three of five findings do not hold.** The two that do re-derive exactly: the blind spot is **19 dropped, 16 compound / 2 ratio / 1 comma** at `a88b91a3~1`, and at `3c17ae5c~1` `governed_globs` was `["TOOLBOX.md"]` while the book chapter carried four claim tags no record named. **The eleventh instance is my own**: `.7.2.1` published "39 commits now separating `e6f5012d` from `HEAD`" — 43 four commits later — *in the sentence explaining that an open-ended window silently grows*. Withdrawn on three surfaces, and the count is replaced by the command rather than re-measured. "20 and then 24 rows re-pinned" were a run counter, not a per-commit measure; re-derived as 21/22/23/20. "Two thirds of each byte budget" is 66.9% for `changes` and **59.2%** for the status ledger — one number wearing two denominators. A first mechanism for the eleventh instance ("`commits` is missing from the closed noun list") was **discarded before publication**: that vocabulary is the *book* census's, and `--produce` shows the current-claim census takes one `CHANGES.md` candidate (its first non-blank line) while the assertion gate takes none (zero claim tags) |
| `2026-08-30` | `.7.2.1` | `perl scripts/check_published_assertions.pl --check`/`--self-test`; three revert-and-re-apply probes on real shipped prose; census/book/claim `--check`; `scripts/check_doctrines.sh` | **the population is closed and the registry is frozen.** 30 assertions — 1 derived, 2 gated, 1 authored, **26 dated** — over 6 governed regions in 2 governed and 2 exempt files, **0 unlisted**. Two sentences were repaired rather than recorded: an open-ended window ("the 27 transitions measured from `e6f5012d`", whose boundary had already moved its range grows with every commit) now names its closing revision `60a81db7`, and "The 27-case self-test" lost its numeral because that producer publishes the count only as prose on stderr, so neither `derived` nor `gated` fits it. A near-contradiction was **adjudicated, not corrected** — `TOOLBOX.md`'s 29 and the book's 28 are both right (28 consecutive revisions plus the older `50775894` anchor = 29 measurements), so neither was touched. **Three probes on REAL prose, each restored byte-exact**: an unmapped numeral gave `published value '41' at TOOLBOX.md:122 ... no assertion record lists`; editing a dated 39 -> 38 fired three legs at once (absent from its own region, stale digest, new unlisted value); and a claim-annotated file staged under `research_records` gave `belongs to surface 'research_records', which declares no disposition` — the fail-closed leg `.7.2.0` built, observed on the real tree |
| `2026-08-30` | `.7.2.0` | `perl scripts/check_published_assertions.pl --self-test`; `--check`/`--report` on the real tree; lifecycle and census-disposition candidate rules measured against the four claim-annotated files; `scripts/check_doctrines.sh` | **the scope is now derived and fails closed.** Membership is discovered by scanning every tracked Markdown file for a claim tag and resolved through `doctrine/live_document_size/surfaces.jsonl`, digest-bound by a `source` record; an undeclared surface, a file no surface owns, a reasonless exemption, an unknown surface id, and a stale surface-registry digest are each RED. Self-test **25/25**. **The exemption is proven load-bearing**, not incidental: flipping `change_history_archive_segments` from `exempt` to `governed` makes the very same value fatal, which separates "deliberately skipped" from "never discovered" — two accounts that predict the same green run. On the real tree: **2 governed and 2 exempt** claim-annotated files over **6** governed regions, unlisted **23 -> 27**, because the book chapter is now in scope where the glob list had silently excluded the one surface instance 5 actually drifted on. Two candidate rules were measured and rejected with reasons — lifecycle cannot separate `TOOLBOX.md` from the task tree, and the census's disposition would admit 85 dated evidence values |
| `2026-08-30` | `.7.1a` | independent widened-lookahead probe over the same governed paragraphs; `perl scripts/check_published_assertions.pl --self-test`; `--check`/`--produce` on the real tree; revert-and-re-apply of the grammar line alone; `scripts/check_doctrines.sh` | **the gate could not see two of the values it exists to watch.** An oracle built to disagree — the gate's own lookbehind kept, only the lookahead widened — reported **19** published values dropped across the four claim-annotated files: 16 compound-adjective forms (`27-case`, `56-unit`, `304-region`), two ratio halves (`304/304`, `15/15`), and one comma over-capture. A first probe without the lookbehind was **discarded**, not published: it reported `SHA-256` and `H1`, which the gate refuses correctly, and an oracle that calls correct behaviour a defect separates nothing. Two of the 19 sit on the current-facing surfaces and one is live — `TOOLBOX.md`'s "The 27-case self-test" is a current count of `check_current_claim_census.pl --self-test` (27/27 today) inside a claim-annotated paragraph the gate was written to watch. Real-tree unlisted **21 -> 23**. **Attribution by revert-and-re-apply, not by reading**: with the grammar line alone reverted, 5 of 19 self-test cases fail — both positives on `published value '40,' ... no assertion record lists`, and all three new cases; re-applied, **19/19**. The absorbed-punctuation case is the mirror the repair needs: a record whose `value` is `40,` now covers nothing, where before it was accepted |
| `2026-08-30` | `.7.1` | `perl scripts/check_published_assertions.pl --self-test`; `--check` on the real tree; revert-and-re-apply drift probe against `check_current_claim_census.pl --report`; `scripts/check_doctrines.sh` | **the leg no existing control supplied is now executable.** Self-test **16/16** with every `.7.0` fault plus five more driven RED on a disposable repository-local fixture. On the real tree the gate runs green at four seeded assertions over two governed regions in `inventory` phase, reporting unlisted values rather than failing on them until `.7.2` completes the population. **Drift observed RED on real shipped prose**, not a fixture: `TOOLBOX.md`'s `**5** views` bound to the census `views` field, edited to `**6**`, produced `is stale: 'views' re-derives to '5', published '6'`; both files restored byte-exact. A first probe against `docs/knowledge/INDEX.md` is recorded as an honest failure — its producer is derive-and-diff over that same file and exits nonzero before any field comparison, so it cannot demonstrate this leg. The coverage grammar was corrected mid-implementation from tag-line to **paragraph** scope, because a `[claim: <id>]` tag closes a paragraph and `TOOLBOX.md`'s two tags sit on lines carrying no quantity at all — keyed on the tag's own line the map would have been blind to precisely the sentences it exists to watch |
| `2026-08-30` | `.7.0` + `.11a` re-derivation | re-derived every figure `.11a` published: `git show --name-only` over the four attributed commits; `git show d23e8bae -- <book>` content-line count; `check_live_document_size.pl` warning population; literal match census over tracked non-archive Markdown at `fd09708d~1`; `§10` rows vs `DOCTRINES`; the three claim producers `--check`/`--report` | **`.11a` does not fully hold either.** Its warning-line total was **invalidated by its own commit** — published as 20, and that commit's `CHANGES.md` prepend crossed `change_history lines_each`, making it 21. And "exactly three surfaces" is a *classification*, not a match count: five tracked files carried the literal, two of them dated records §1 exempts; the conclusion holds and the predicate was unstated. Both are withdrawn rather than corrected. Everything else in `.11a` re-derives: the republication attribution, `d23e8bae`'s two changed book lines, the four-producer composition, the `done`-tree and self-satisfying classifier defects, and the dated-table withdrawal. **Five consecutive rounds of correction, each invalidated by its own transaction, is the evidence that closes the design question**: `.7` is split and `.7.0` freezes the contract |
| `2026-08-30` | `.11a` | `git show --name-only` over the four attributed commits; `git show d23e8bae~1:<book>`; `check_live_document_size.pl` vs `check_live_document_size.sh` warning populations; open-tree ownership screen recomputed from each tree's own `Status` line; all three claim producers `--check`/`--report`; 17 relocations re-derived HEAD vs HEAD~1; doctrine gate | **five of `.11`'s findings do not hold.** Only **one** of the three commits said to have republished `identity_gated 7` edited a publisher. `d23e8bae` withdrew `current_surfaces` from **two** surfaces, not three — the book chapter never carried it, and "all three" was quoted from that commit's own body. The **18 lines / 13 surfaces** census was one producer's, and only 17 of its lines named a surface; the gate-level producer emits **35** lines across **four**. The ownership screen counted `done` trees as owners, hiding `corpus_task_evidence_parts`, and the act of publishing the finding turned its own classifier green for the three surfaces it named. And the `Opening Pressure Boundary (92e59c97)` table is a **dated** snapshot §1 exempts, not a disagreeing census. Everything else re-derives unchanged: `identity_gated` 8 on three surfaces, 78 -> 89, 307/78/221/186 -> 321/89/224/189, 10/nine -> 11/10/1 at `1ccb7331`, the sibling frontier, the three withdrawals, 15 -> 16, 17 relocated + 1 added, 11 = 11, 5 book regions. `.11`'s own ledger prepends also moved the warned surface count 13 -> **15** (`achievement_status`, `change_history`; both owned) |
| `2026-08-30` | `.11` | enumerating command over tracked non-archive Markdown (anchored at `9fc76685`); per-revision re-derivation of `identity_gated` from `git show <rev>:doctrine/claim_verification/current_claim_census.jsonl` across 25 revisions; `check_current_claim_census.pl --check`/`--report`; `check_book_quantitative_claims.pl --check`/`--report` before and after the chapter edit; `check_fact_card_catalog.pl --write`/`--check`; knowledge-map derive-and-diff; `check_claim_verification.pl --check`; the three claim self-tests; doctrine gate | the population is **15** surfaces at `9fc76685` and **16** at this commit — writing the change record joins it — so the number is anchored and the command is the durable artifact; either way not the two `.6b` swept. **Six stale publications and three carried-but-correct ones**, withdrawn as one class: `.6`, `.6a`, and `.6b` each swept from memory and none ran an enumerating command. `identity_gated` **7 -> 8** on three surfaces, attributed to `d23e8bae`, which added exactly one `identity_gated` record for the `task_tree_catalog_parts` surface it registered — the same registration whose other effect (`current_surfaces` 39 -> 40) that commit *did* notice and withdraw, one sentence away; the book chapter's **78 -> 89** incomplete regions; the whole `.3b.3.3` vector (**307/78/221/186** -> 321/89/224/189) still carried in `mdbook-quantitative-census-freeze.md`, whose stale twin `.6b` repaired while quoting these numbers off it; and a registry set claim short by one member (**10/nine/ten** -> 11 registered / 10 gate / 1 CI, `PROOF-SEAL-CURRENCY` missing since `1ccb7331`); `MEMORY.md`'s five-surface live-document warning census against a producer that warns about **13**, three named by no tree; and this tree's sibling `LIVE-DOCUMENT-PRESSURE-HEADROOM` frontier still calling `.2a`/`.2b`/`.2c` pending. Three further constants are carried and still correct and go with them — `authority_outcomes.derived`, the mdBook claim assertion's eleven constants, and `control_audit`'s 7/7/6. All withdrawn as one class per §3 Leg 3 rather than replaced with today's values. The survivors are stated with what makes them true: `unresolved` **0** is gated by `validate_candidate_closure`, `views` **5** is authored by `.3a.0`. Book vector **321/8/89/224 identical** across the chapter edit; **17** regions relocated (16 census, one book) and one evidence row added for the new ledger head, derived by diffing both registries against HEAD rather than summing the passes — which first gave 18, `.9`'s ninth instance caught by deriving it; each relocation matched exactly one line and each file's shift equalled the cumulative insertions above it from the diff hunks; catalog valid for 249 cards; Knowledge Map 271 facts / 2152 keys; self-tests 27/27, 27/27, 19/19 |
| `2026-08-30` | `.10a` correction | replayed the narrow absence probe against `HEAD~1:CLAIM_VERIFICATION.md`; enumerated the census-governed surface set from `current_claim_census.jsonl`; `check_rolling_ledger_protocol.pl --report`; doctrine gate | the narrow probe ran **seventeen** terms, not fifteen — all seventeen zero. Fifteen is the *discriminating* subset and the count the **widened** probe ran; published on four surfaces as the narrow probe's size, which is `.7`'s ninth instance one commit after it was written. Also corrected an unenumerated set claim ("visible to no gate"), now counted: not a registry record, and carrying no numeral, so no candidate grammar reaches it — `MEMORY.md` is a governed census surface with four evidence rows and still produced none. Findings two and three re-derive **unchanged**; the ledger figures come from the producer, which now emits no warning for `development-notes` at all |
| `2026-08-30` | `.10` | section-by-section re-read of the upstream standard; seventeen-term absence probe over `CLAIM_VERIFICATION.md` alone, then a fifteen-term re-run over all governed claim surfaces; `check_book_quantitative_claims.pl --check` before and after the mdBook edit; knowledge-map derive-and-diff; `check_fact_card_catalog.pl --write`/`--check`; doctrine gate | source unchanged since `.1` (mtime `2026-08-26`), so the gap dates from the original adoption. The narrow probe returned zero hits for all seventeen terms in one file; **widening it to every governed claim surface changed the answer** — ADR 0042 already carried the general form as Context rationale. That is Leg 1's granularity rule catching its own adoption commit, and it is why the published set is scoped to "no home on any governed claim surface" rather than "absent from the standard". Remaining matches classified as unrelated word collisions before publishing. Book census **321/8/89/224 unchanged** across the edit; one region re-pointed 547 -> 580, line SHA-256 identical; Knowledge Map 271 facts / 2148 keys after one question-key collision was resolved in the fact card's favour; catalog valid for 249 cards |
| `2026-08-30` | `.6b` correction | re-derived the move count and staleness window from `git show <rev>:doctrine/claim_verification/book_quantitative_claims.jsonl` across its 31 revisions, with commit timestamps | `.6b` published "**seven** registry changes" and "stale for **twelve days**". Both wrong: **six** moves, and the chapter stayed *correct* until `4dac5642` (`2026-08-27 02:14`), repaired at `2b9e8899` (`2026-08-29 22:23`) — **two days and twenty hours** stale. The twelve-day span was writing-to-settling, substituted for staleness. Every other figure in the finding re-derives; corrected on all four live surfaces and logged as `.7`'s ninth instance |
| `2026-08-29` | `.6b` correcting `.6a` | outcome/row census of `current_claim_census.jsonl` at `fdda3c53` and `5fe81128` from Git blobs | both revisions hold **59** evidence rows and **5** `CHANGES.md` rows, differing by exactly **one removed / one added**. `.6a`'s published mechanism ("sealed 18 while adding 2") is **false**; the total held because one retirement cancelled one addition. Every number `.6a` published re-derives and the withdrawal stands. Recorded as `.7`'s seventh instance: a count-only gate passes a false mechanism |
| `2026-08-29` | `.6b` | outcome census over 31 registry revisions of `book_quantitative_claims.jsonl`; `check_book_quantitative_claims.pl --check`/`--report`/`--produce`; `check_current_claim_census.pl --check`; knowledge-map derive-and-diff; fact-card catalog; doctrine gate | the book's `regions=307/8/78/221` was **correct when written** at `be3b12e6` (`2026-08-16`) and then moved **six times** to 321/8/89/224 by `fdda3c53`, going stale at the first of those (`4dac5642`, `2026-08-27`) and repaired at `2b9e8899` — stale for two days and twenty hours; "56 exact evidence units" was correct at `50775894` and false from `e6f5012d`. Both withdrawn and routed to `--report`, the fact card retitled, and `TOOLBOX.md`'s last carried mdBook count withdrawn for consistency. Post-edit re-derive: **321/8/89/224 unchanged**, 39 book files / 21 candidate files. One frozen region re-pointed 540 -> 547. For `.9`: the chapter held **5** candidate lines in 557 and none was a drifted one — `units` and backticked `key=value` are both outside the grammar |
| `2026-08-29` | `.6a` | 28-consecutive-revision worktree trajectory (`e6f5012d` -> `60a81db7`) plus the `50775894` anchor, each with its own `check_current_claim_census.pl --report`; `check_claim_verification.pl --report`; `check_book_quantitative_claims.pl --report`; census `--self-test`; doctrine gate | `registered` **6 -> 5 -> 4** and closure **86/51/35 -> 72/50/22 -> 69/49/20**, stepping at `5fe81128` and `1507adbf`; so all four counts `.6` called confirmed were false **inside `.6`'s own commit**. `.6`'s unit trajectory is also corrected (**59**, not 60, at `5fe81128`) and its "one more excluded unit per rolling-ledger head" rule withdrawn — 15 rises, **2 falls**, 10 no-changes over 27 transitions. Stable across all 29: derived **11**, identity-gated **7**, no `incomplete`, unresolved **0**, surfaces **39**, views **5**; carried with producer fields named. `7/7/6/0/0` control audit and mdBook **89** incomplete re-derive; self-test **27/27** |
| `2026-08-29` | `.7` fifth stale-count instance | `test_live_document_size.pl`; `check_fact_card_catalog.pl --self-test`; `test_derived_state_contracts.pl`; `test_derived_state_authorities.pl`; `check_task_tree_archive.pl --self-test`; `check_active_task_evidence.pl --self-test` | producers report **84 / 60 / 47 / 25 / 15 / 44**. `DOCTRINE_ENFORCEMENT.md` published 84/**58**; the book published **81**/**58**. Two stale counts repaired; the other four re-derive. The book's stale `81` sits at a line the frozen census holds **no** region for; its stale `58` sits at a line the census holds an **incomplete** region for — one outside the denominator, one inside it and explicitly unverified. Neither a digest binding nor the frozen census can observe either, which is `.7`'s whole point |
--- |
| `2026-08-15` | `.0` | full source-standard read; claim/review/derived-state/doctrine/control/tracked-
  producer/capacity census; task catalog; derived-state report; canonical catalogs; live-size self-tests |
  `passed`; mapped 39 current surfaces, 14 derived-state contracts, 33/24 tracked/self-test checkers, zero
  governed ignored producers, and the 42/44 decision prerequisite without claiming adoption |
| `2026-08-15` | `.1` | route/catalog self-tests and real checks; contract phrase probe; tracked path
  census; Knowledge Map; mdBook; live-size; doctrines | repository-owned standard/ADR/template published; exact
  claim/no-claim declaration active; 14 workflow standards and 36 README routes close; exact segment 0008 seals
  15 engineering records and restores the 61-record live root below warning; registry gate remains pending |
| `2026-08-15` | `.1a` | tracked Git-growth report + five controls; full-catalog arithmetic; catalog/KM/
  live-size/mdBook/doctrine gates | 14 current / peak 4 derive 21; current 66.7%, one peak 85.7%; explicit paths
  retained; exact one-use authority remained for `.1b` and was subsequently retired |
| `2026-08-15` | `.1b` | authority census; capacity reproducer; canonical catalog; task/KM/currentness;
  live-size; doctrine gates | registry returns to its one control record; 21-file profile and all evidence stay
  unchanged; `.2` is executable |
| `2026-08-15` | `.2` | checker syntax/report + 22-case RED matrix; 8 executed evidence commands; catalogs/KM/
  README/book/live-size/locality/mdBook/doctrines | 3 verified records current; prepared/HEAD publications resolve;
  tenth doctrine registered through the existing driver only |
| `2026-08-15` | `.3a.0` | live-registry lifecycle census; task catalog; memory; claim check/report; live-size;
  doctrines | 55 total - 15 archive - 1 frozen = 39 current surfaces; five category views and five closed
  outcomes frozen; no source assertion changed |
| `2026-08-15` | `.3a.1` | checker syntax; 14-case fault matrix; real check/report/producer; task/memory/claim/
  live-size/book/doctrine gates | 39 current = 32 inspection + 7 explicit exclusions; five views; 47 deterministic
  candidates; inventory remains unfrozen and no source assertion changed |
| `2026-08-15` | `.3a.2` | multi-view RED reproduction; 15-case fault matrix; frozen check/report/producer;
  15 joined verifiers; claim/task/memory/book/live-size/doctrine gates | 51 units = 11 derived + 4 identity-gated
  + 4 registered + 5 incomplete + 27 excluded; five exact `.3b` keys; no source assertion changed |
| `2026-08-15` | `.3b.0` | five-key authority-route audit; current census/check/report; README/catalog/book/
  task/memory/claim/live-size/doctrine gates | repair sequence frozen without changing any source assertion or
  one of the 51 existing authority outcomes |
| `2026-08-15` | `.3b.1` | README policy real/self-test; fact-card catalog real/self-test; canonical collection
  real/self-test; current census; task/memory/book/claim/live-size/doctrine gates | all three existing route
  authorities derive exact membership and fail closed; frozen census remains unchanged for `.3b.4` |
| `2026-08-15` | `.3b.2` | capacity derivation; five-case boundary controls; independent catalog-feasibility
  probe; canonical catalog; current census; claim/book/live-size/doctrine gates | authored workflow identity is
  separated from the existing registered capacity paragraph without changing source or census outcomes |
| `2026-08-15` | `.3b.3.0` | independent prose-only quantitative scan; design/schema/bound review; current census;
  task/memory/book/claim/live-size/doctrine gates | bounded candidate and exact-region authority design frozen;
  no book assertion or outer census outcome changed |
| `2026-08-15` | `.3b.3.1` | checker syntax; 18-case fault matrix; real check/report/producer; parent-revision
  replay; task/memory/book/claim/live-size/doctrine gates | 39 book files, 304 candidates across 21 files; the
  provisional 301 estimate is corrected; inventory remains region-free and the outer census is unchanged |
| `2026-08-15` | `.3b.3.2` | frozen exact-coverage replay; 19-case matrix; independent outcome diff; excluded-
  current-language and incomplete-past-tense audits; task/memory/book/claim/live-size/doctrine gates | 304 exact
  regions: 8 registered + 75 incomplete + 221 excluded; record/array bound defect repaired; outer census unchanged |
| `2026-08-15` | `.3b.3.3` | clean result replay; 19-case controls; claim rederive/registry/stale joins; fact-card catalog
  and Knowledge Map derive-and-diff; task/memory/book/live-size/doctrine gates | verified census-mapping authority
  freezes 304 regions and preserves 75 incomplete assertions; no underlying assertion is promoted |
| `2026-08-15` | `.3b.4` | parent/current semantic-outcome diff plus exact identity audit; 15-case census controls; all five route/claim joins;
  current and mdBook census reports; fact/Knowledge Map; task/memory/book/live-size/locality/doctrine gates |
  56 exact units: 11 derived + 7 identity-gated + 6 registered + 0 incomplete + 32 excluded; `.3b` closed |
| `2026-08-15` | `.3c` | clean `50775894` replay; 27-case all-family/coverage matrix; candidate-closure report;
  independent 304-region book replay; claim/catalog/Knowledge Map/task/memory/book/live-size/locality/doctrines |
  79 candidates = 51 exact + 28 registered + 0 unresolved; 56-unit vector unchanged; `.3` closed |
| `2026-08-15` | `.4` | 27-case claim-gate matrix; all seven control commands and exact RED regions; controlled
  workflow sub-ceiling; tracked/ignored/untracked producer census; current/book/fact/Knowledge Map/task/memory/
  mdBook/live-size/locality/doctrine gates | 7 controls = 7 exact RED regions across 6 producers; 0 ignored and
  0 untracked candidates; current closure 84 = 51 + 33 + 0; `.4` closed |
| `2026-08-16` | `.5` | clean-boundary current/manual replay; author/auditor review; production-graph exact-
  oracle repair and focused four-test replay; fact/task/public-status/architecture alignment; all-tier doctrines;
  selected full CI; mdBook/live-size/locality/residue gates | 10 doctrines and 11 genericity qualifications;
  1,996 Rust tests / 8 ignored / 0 failed plus 5 compile-fail doctests; adoption closed at 79 modules / 141 rows,
  zero silent current candidates and 304/304 manual candidates adjudicated; exact cleanup removes 12,097
  rebuildable incremental files / about 9.6 GiB plus two zero-byte temp logs; `.7` aligned |

<!-- claim-verification-task-source-region:verification-log:end -->

<!-- claim-verification-task-source-region:commit-log:start -->
## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.0` | `CLAIM-VERIFICATION-ADOPTION.0 — own and map three-leg claim verification` | standard/local seam audit; implementation remains pending |
| `.7.2.1a` | `CLAIM-VERIFICATION-ADOPTION.7.2.1a — re-derive .7.2.1's findings and withdraw the three that do not hold` | two findings re-derive exactly; an open-ended `HEAD` window, a run-counter figure, and a two-denominator share withdrawn; `.13` owns the population bound |
| `.7.2.1` | `CLAIM-VERIFICATION-ADOPTION.7.2.1 — close the population at zero and freeze` | two sentences repaired, 26 dated records, unlisted closed at zero, phase frozen, three real-prose probes RED and restored |
| `.7.2.0` | `CLAIM-VERIFICATION-ADOPTION.7.2.0 — derive the governed scope, and fail closed on an undeclared surface` | stored glob list replaced by discovered membership plus authored per-surface disposition; six new RED cases at 25/25; the book chapter enters scope |
| `.7.1a` | `CLAIM-VERIFICATION-ADOPTION.7.1a — measure the coverage grammar's blind spot, then close it` | 19 published values were invisible to the population scanner; grammar repaired on measurement, 19/19 self-test with three grammar-dependent RED cases, real-tree unlisted 21 -> 23 |
| `.7.1` | `CLAIM-VERIFICATION-ADOPTION.7.1 — execute the producer and compare the field` | twelfth registered doctrine; 16/16 RED matrix; drift observed on real shipped prose; coverage corrected to paragraph scope; §10 and the mdBook chapter updated in the same commit |
| `.7.0` | `CLAIM-VERIFICATION-ADOPTION.7.0 — freeze the published-assertion gate design` | eight-element contract traced element-by-element to the instance that defeated the alternative; `.7` split into freeze, gate, and population; `.11a`'s two invalidated figures withdrawn in the same commit |
| `.11a` | `CLAIM-VERIFICATION-ADOPTION.11a — re-derive .11's findings and correct the five that do not hold` | two mechanism claims and two set claims withdrawn; the ownership classifier replaced because publishing the finding satisfied it; producer scope corrected from one checker to the gate |
| `.11` | `CLAIM-VERIFICATION-ADOPTION.11 — enumerate the surfaces, then withdraw the class` | population derived from a command instead of recalled (15 at the parent, 16 here); six stale publications and three carried-but-correct constants withdrawn as one class; every survivor labelled gated or authored; `.7` tenth instance, `.9` third demonstration |
| `.10` | `CLAIM-VERIFICATION-ADOPTION.10 — re-adopt the upstream claim standard` | absent upstream rules made normative; non-adoption recorded with reasons; `.7` scope settled to counts; mandatory `DEVELOPMENT_NOTES.md` rollover to segment 0009 |
| `.10a` | `CLAIM-VERIFICATION-ADOPTION.10a — correct .10's own probe count and set claim` | seventeen-term narrow probe restored on four surfaces; the "no gate sees this" assertion enumerated; findings two and three verified unchanged |
| `.1` | `CLAIM-VERIFICATION-ADOPTION.1 — publish the claim-verification contract` | normative scope, ADR 0042, discovery, authoring/review contract, and mdBook alignment |
| `.1a` | `CLAIM-VERIFICATION-ADOPTION.1a — re-derive workflow-standard capacity` | tracked measurement, ADR 0043, exact authority, stable explicit topology |
| `.1b` | `CLAIM-VERIFICATION-ADOPTION.1b — retire the consumed workflow-capacity authority` | one-use authority removed after the 21-file profile became baseline |
| `.2` | `CLAIM-VERIFICATION-ADOPTION.2 — gate published claim provenance` | bounded registry, executable evidence join, stale digest gate, publication resolution, tenth doctrine |
| `.3a.0` | `CLAIM-VERIFICATION-ADOPTION.3a.0 — freeze the current-claim census design` | 39-surface denominator, five category views, closed authority outcomes, staged implementation/results |
| `.3a.1` | `CLAIM-VERIFICATION-ADOPTION.3a.1 — implement the current-claim census` | bounded inventory contract, deterministic producer, exact region/authority joins, fail-closed controls |
| `.3a.2` | `CLAIM-VERIFICATION-ADOPTION.3a.2 — freeze current-claim census findings` | 51 exact outcomes, complete surface/view coverage, five-key repair frontier |
| `.3b.0` | `CLAIM-VERIFICATION-ADOPTION.3b.0 — freeze the current-claim repair map` | authority-specific route, workflow-baseline, mdBook-quantitative, and closure leaves |
| `.3b.1` | `CLAIM-VERIFICATION-ADOPTION.3b.1 — bind maintained-reference authorities` | existing README, fact-card, and canonical catalog authorities proven sufficient for later atomic binding |
| `.3b.2` | `CLAIM-VERIFICATION-ADOPTION.3b.2 — bind the workflow baseline authority` | authored title identity separated from the existing registered capacity paragraph |
| `.3b.3.0` | `CLAIM-VERIFICATION-ADOPTION.3b.3.0 — freeze the mdBook quantitative census design` | prose candidate grammar, exact coverage/authority schema, portable bounds, staged result freeze |
| `.3b.3.1` | `CLAIM-VERIFICATION-ADOPTION.3b.3.1 — implement the mdBook quantitative census` | bounded inventory contract, deterministic producer, exact membership/fence/region validation, 18-case RED matrix |
| `.3b.3.2` | `CLAIM-VERIFICATION-ADOPTION.3b.3.2 — adjudicate mdBook quantitative assertions` | 304 exact semantic regions, honest 75-line incomplete set, verified workflow join, record/array bound repair |
| `.3b.3.3` | `CLAIM-VERIFICATION-ADOPTION.3b.3.3 — freeze mdBook quantitative authorities` | verified mapping claim, digest-complete stale replay, retrievable fact card, explicit incomplete boundary |
| `.3b.4` | `CLAIM-VERIFICATION-ADOPTION.3b.4 — close current-claim repairs` | five exact narrow replacements, zero outer incompletes, 46 non-frontier semantic outcomes preserved |
| `.3c` | `CLAIM-VERIFICATION-ADOPTION.3c — close the current-claim sweep` | clean-boundary replay, all-family RED matrix, mechanically closed candidate denominator |
| `.4` | `CLAIM-VERIFICATION-ADOPTION.4 — prove tracked producers and falsifying controls` | exact known-bad regions, repaired workflow probe, derived six-producer/scratch census |
| `.5` | `CLAIM-VERIFICATION-ADOPTION.5 — close three-leg claim verification adoption` | complete author/reviewer workflow, five-architecture retrieval, public alignment, independent full signoff |
| `.6` | `CLAIM-VERIFICATION-ADOPTION.6 — re-derive the drifted claim-annotated prose counts` | 11 counts re-derived, two per-commit counters withdrawn, mdBook incomplete 75 -> 89; four "confirmed" counts later proved false in this same commit |
| `.6a` | `CLAIM-VERIFICATION-ADOPTION.6a — own four counters .6 confirmed that have drifted again` | leaf opened; withdrawal deliberately deferred until the trajectory was measured |
| `.6a` | `CLAIM-VERIFICATION-ADOPTION.6a — measure the drift trajectory before withdrawing the counters` | 28-consecutive-revision worktree measurement plus the `50775894` anchor; four counters withdrawn with producer fields named, six carried, `.6`'s unit trajectory corrected, `.6b` opened |
| `.6b` | `CLAIM-VERIFICATION-ADOPTION.6b / CHANGES-LEDGER-ROLLOVER.3 — sweep the two surfaces .6 and .6a left` | book chapter and fact card routed to `--report`, card retitled, drift attributed across 31 registry revisions, `.9` second demonstration |

<!-- claim-verification-task-source-region:commit-log:end -->

<!-- claim-verification-task-source-region:changelog:start -->
## Changelog

- `2026-08-30`: closed `.7.2.1a`. **Asked a third time whether the findings held, and three of five did not.**
  The sharpest is the eleventh instance of this tree's own class, and it is mine: `.7.2.1` published a count
  of commits between a fixed revision and `HEAD` *inside the sentence explaining that a window with no end
  silently grows*. Four commits later it was wrong. That is the fourth consecutive round in which a slice
  invalidated its own publication, and the pattern is now specific enough to name: **the defect a slice is
  describing is the defect it is most likely to commit while describing it.** The repair replaces the count
  with the command rather than re-measuring it, because a re-measured count is stale on the next commit. Two
  findings re-derived exactly, which is worth saying too — the blind-spot decomposition to the value, and the
  scope finding at its own revision. `.13` now owns what this exposed: the eleventh instance landed where no
  governed population reaches, so `.7` closed the class over two files and the sentences saying so now say
  which two.
- `2026-08-30`: closed `.7.2.1`, and with it `.7`. **The class is now under a control that fails — on the two
  governed surfaces.** (`.7.2.1a` bounded this sentence, which first read "the class that ran for ten recorded
  instances ... is now under a control that fails" with no scope at all. The governed population is two files;
  the eleventh instance landed in `CHANGES.md`, which no population reaches.) Every published value in a
  governed region resolves to a record with a closed outcome, the registry is `frozen`, and an unmapped
  numeral, a drifted value, or a claim-annotated file on an undeclared surface each turn the gate RED — all
  three observed on real shipped prose, not fixtures, and restored byte-exact. Two sentences were repaired
  instead of recorded because no honest outcome fitted them; a third apparent contradiction was adjudicated
  against the project's own history and left alone. Twenty-six of the thirty records are `dated`, which is
  worth saying plainly: that surface publishes how the counters moved, deliberately, and the gate now proves
  the history stays attached to the revisions it describes.
- `2026-08-30`: closed `.7.2.0`. **The gate was about to freeze on a stored list of one.** `governed_globs`
  named `TOOLBOX.md` and nothing else, so flipping to `frozen` would have declared the map complete while the
  book chapter — the surface instance 5 actually drifted on — sat outside it. Membership is now discovered by
  scanning for claim tags and resolved through the live-document surface registry, and an undeclared surface
  is an error rather than a silent omission, which is the property `.7.0` element 3 wanted and a glob list
  cannot supply. Two exemptions, each with its reason, and the exemption is proven load-bearing by flipping a
  surface to governed and watching the same value turn fatal. Two candidate rules were measured and rejected
  rather than left unmentioned: lifecycle cannot separate `TOOLBOX.md` from the task tree, and the census's
  own disposition would admit 85 dated verification rows. `.7.2.1` now owns the population, plus two TOOLBOX
  sentences that no closed outcome fits as written.
- `2026-08-30`: closed `.7.1a`. **The gate had a blind spot in the one thing it derives itself.** `.7.1`
  shipped a numeral grammar that excluded every `-` and `/` after a numeral — written to keep dates and
  identifiers out, and it does — and that also dropped every count closing a compound adjective or a ratio.
  Measured against an independent tokenizer rather than by reading the regex: **19** published values across
  the four claim-annotated files were invisible, including `TOOLBOX.md`'s live "The 27-case self-test", a
  current count of `check_current_claim_census.pl --self-test` sitting inside a paragraph this gate exists to
  watch. A second defect absorbed sentence commas into values (`40,`), which would have forced records to bind
  punctuation and reported the honest record as unlisted. Both are repaired to the form the identifiers
  actually take: a comma joins a numeral only before exactly three digits, and only `-` followed by a digit is
  excluded. Three new RED cases, all proven grammar-dependent by reverting the single line (14/19 reverted,
  19/19 re-applied). This is why `.7.2` could not have gone first: closing an unlisted report at zero certifies
  nothing when the population is wrong.
- `2026-08-30`: closed `.7.1`. The gate exists and executes. `scripts/check_published_assertions.pl` runs a
  record's named producer and compares its report field to the literal published in the governed region — the leg
  that ten recorded instances went stale for want of, because a digest proves a region has not changed and never
  that its numbers still re-derive. Registered as the twelfth doctrine through the existing driver, so hook and
  CI wiring are unchanged. The four outcomes are closed by construction: a fifth is refused, so there is no
  expressible way to carry a value on a trajectory. Cross-surface disagreement fails before any producer runs;
  a set is compared as an enumeration rather than a size; and `excludes_self` is required the moment an
  enumerator returns the record's own publishing surface. Self-test 16/16 on a disposable repository-local
  fixture, and the drift leg additionally observed RED on **real shipped prose** by revert-and-re-apply rather
  than on a fixture alone. Two implementation findings are recorded rather than smoothed over: the coverage
  grammar first keyed on the `[claim: <id>]` tag's own line, which would have been blind to every value the tag
  actually governs, since authors close a paragraph with it — `TOOLBOX.md`'s two tags sit on lines with no
  quantity at all; and the first real-content drift probe chose a producer that is derive-and-diff over the file
  it was meant to watch, so it exited nonzero before comparing anything. The registry ships in `inventory` phase
  where an unlisted value is reported, not fatal; `.7.2` completes the population and flips it to `frozen`.
- `2026-08-30`: closed `.7.0` and stopped correcting. Asked a third time whether the findings held, `.11a` was
  re-derived and **it does not fully hold either**: its warning-line total was invalidated by its own commit
  (20 published, 21 after that commit's own prepend crossed a band), and its "exactly three surfaces" was a
  classification published without its predicate — five files carried the literal, two of them dated records §1
  exempts. Both withdrawn rather than corrected, because correcting is what has failed. **Five consecutive rounds
  — `.6`/`.6a`/`.6b`, `.10`/`.10a`, `.11`/`.11a` — each invalidated by its own transaction, settle the design
  question that `.7` had left open.** A sixth correction would behave identically, so `.7` is split and `.7.0`
  freezes the contract that ends the class, every element traced to the instance that defeated the alternative:
  execute the producer and compare the field, because a digest proves only that a region has not changed; four
  outcomes with no slot for "a trajectory shows it has held"; a population derived at check time and never
  stored, because three commits in a row moved their own population by describing it; a declared
  `excludes_self`, because `.11`'s classifier was satisfied by the act of writing the finding; cross-surface
  disagreement, which needs no producer at all; membership compared as an enumeration rather than a size; and
  mechanism claims explicitly out of scope with an `adjudicated_against` field instead, since no checker can
  decide whether two accounts predict the same observation.
- `2026-08-30`: closed `.11a`. Asked a second time whether `.11`'s findings held — the question that produced
  `.6b` and `.10a`, and which has now found a real defect all three times — every figure was re-derived instead
  of re-read, and **five did not hold**. Two were mechanism claims taken from co-occurrence or from a commit
  body: only one of three commits actually republished `identity_gated 7`, and `d23e8bae` withdrew
  `current_surfaces` from two publishers rather than three, because the book chapter never carried it. Two were
  set claims with the wrong denominator: the warning census was one checker's output presented as the doctrine's,
  where the gate-level producer emits 35 lines across four producers rather than 18 over 13 surfaces; and the
  ownership screen both counted `done` trees as owners — hiding `corpus_task_evidence_parts` — and turned green
  for the three surfaces it named, because publishing the finding is what made `docs/tasks/` mention them. The
  fifth was a category error: the `Opening Pressure Boundary (92e59c97)` table is a dated snapshot §1 exempts,
  not a competing census. Everything else re-derives unchanged. The corrections are each **stronger** than what
  they replace: `.10` edited `TOOLBOX.md` while adopting the rule against carrying the value; the real unowned
  set was four, not three; and a classifier a finding can satisfy by being written is §2's shared-parent defect,
  which is why `LIVE-DOCUMENT-PRESSURE-HEADROOM.7` now requires an explicit reviewed assignment over the
  gate-level population instead of a grep. Self-catch rate for this adoption: **four**, three of them found only
  because the director asked again, none by any rule, gate, or review step this repository has.
- `2026-08-30`: closed `.11`. Ran the command that enumerates the population three earlier leaves swept from
  memory — every tracked non-archive Markdown surface citing one of the three claim producers — and it returns
  **15**, where `.6b` had written "two other current-facing surfaces". Six stale publications, none of which any
  gate could see. `authority_outcomes.identity_gated` **7 -> 8**, on `TOOLBOX.md`, the mdBook enforcement chapter,
  and the census fact card, attributed by per-revision re-derivation to `d23e8bae` — the commit that registered the
  task-catalog parts surface, noticed that the registration moved `current_surfaces` 39 -> 40, withdrew *that* value
  on all three publishers, and left `identity_gated` carried on the same sentence. The book chapter's `78` incomplete
  assertion-level regions (now 89). The entire `.3b.3.3` book vector still carried in
  `mdbook-quantitative-census-freeze.md`, including its title — the exact numbers `.6b` corrected on the chapter
  while quoting them off it, in a card two directory entries away that nothing looked for. And a set claim whose own
  enumeration was short by one: the doctrine-adoption card's registered-doctrine list, missing `PROOF-SEAL-CURRENCY`
  since `1ccb7331`. Two more of the same class on surfaces the leaf had to touch anyway: `MEMORY.md`'s
  five-surface live-document warning census against a producer that warns about 13, three of them named by no
  tree at all; and `LIVE-DOCUMENT-PRESSURE-HEADROOM`'s own frontier still calling `.2a`/`.2b`/`.2c` pending.
  Three further constants are carried and still *correct* and go with them — `authority_outcomes.derived`,
  the `mdbook-quantitative-census-frozen` assertion's eleven constants, and `control_audit`'s 7/7/6.
  **The remedy is one class, not nine numbers.** `.10` adopted "a repository-derived constant is
  derived or gated, never carried" and retired the trajectory licence these values were carried under, so
  `authority_outcomes.derived` — which had not moved — is withdrawn with `identity_gated`, which had, because
  unmoved is not the same as immovable. Writing `8` for `7` was available and refused: §3 Leg 3 says a right
  unwatched number replacing a wrong one is not a fix. What survives is stated with its reason — `unresolved` **0**
  because `validate_candidate_closure` raises an error the moment it leaves zero, `views` **5** because `.3a.0`
  froze them — so a later reader can tell a gated value from an unmoved one without re-running the trajectory that
  failed here. Recorded and not repaired: nothing checks `DOCTRINE_ENFORCEMENT.md` §10's row set against the
  driver's `DOCTRINES` array, although the driver's own header calls §10 its mirror "kept in lockstep"; enumerated
  rather than asserted, the two agree today at 11 each, so it is an unwatched coupling for `.7` rather than a
  defect. Both stale book counts again sit outside the mdBook census denominator, which is `.9`'s third
  demonstration and its second on the census's own chapter.
- `2026-08-30`: closed `.10a`. The director asked a second time whether `.10`'s findings were trusted — the
  question that produced `.6b` — so they were re-derived rather than re-read, and one number was wrong. The narrow
  absence probe ran **seventeen** terms, not fifteen; fifteen is the discriminating subset and the count the
  widened probe ran. A real quantity substituted for the one the sentence names, which is `.7`'s ninth instance
  committed one commit after that instance was described. A second defect in the same paragraph: "visible to no
  gate this repository has" is a set claim published without its enumeration, the rule `.10` had just made
  normative. Both corrected on every publishing surface; the corrected probe account is strictly better because it
  explains why the two probes differ, which the single number hid. Findings two and three re-derive unchanged, with
  every rollover figure taken from the producer rather than from arithmetic. The self-catch rate is therefore
  **three** in this adoption, two of them set claims, and the third was caught only because the director asked
  again — not by any rule, gate, or review step this repository has.
- `2026-08-30`: closed `.10`. Re-read the upstream standard section by section, which directive 17 requires and
  which nothing had done since `.1`. The source had not moved, so the gap was original to the adoption — and it is
  wider than `.10` predicted: the leaf named the three rules that had already produced a recorded defect here,
  while the reading found a larger set with no home on any governed claim surface, now enumerated in
  `CLAIM_VERIFICATION.md` §11 with each rule's local home. Both statements hold under different denominators, so
  the leaf's own text stands. Adopted: the taxonomy of what each check class still permits and its general form, the
  illustration rule, the set-enumeration rule in both directions, the project-history oracle, deriving classifiers
  from the producer, revert-and-re-apply attribution, granularity matching, one derived source over N copies,
  derived-or-gated-never-carried, the unwatched-number rule, and the domain-free statement rule. Recorded four
  deliberate refusals with reasons. One rule was already here but demoted to ADR 0042 rationale while the standard
  carried only its instances; promoted, with the ADR keeping the sentence as reasoning. The first absence probe was
  scoped to one file and would have published a claim about the repository on evidence about that file — the
  granularity rule being adopted in the same commit — so it was widened before anything was published, which is
  what found the ADR hit. `.7`'s open scope question is answered and it **narrows**: a claimed mechanism is a
  review obligation under the now-normative illustration rule, because no checker can decide whether two accounts
  predict the same observation; the gate stays on counts, keeping the cross-surface disagreement signal and an
  optional prior-adjudication field as its mechanizable residues.
- `2026-08-30`: `.10` performed the mandatory `DEVELOPMENT_NOTES.md` rollover in its own slice. The rationale
  record crossed the 90% line milestone, and this tree's `.1` had already adjudicated the same shape by rolling
  the same ledger inside its own slice; `2b9e8899` did the equivalent for `CHANGES.md`. Segment
  `development-notes-0009` seals 22 whole records; the live root returns to 62 records / 1,384 lines, under the
  80% warning on every dimension. The dry-run was proven exact before applying and reported `future_prepends: 1`,
  which is how the in-flight record was shown to survive the cut rather than assumed to.
- `2026-08-29`: opened `.6a`. Re-derived `TOOLBOX.md`'s census sentences against their producers at
  `c1609558` and found four of the eight counts `.6` recorded as "confirmed unchanged" no longer hold
  (`6 registered` -> 5; closure `86/51/35` -> `72/50/22`); the other four and the provenance sentence are
  current. `1507adbf`, which touched only the resume pointer and two claim registries, then moved all four
  again to 4 and `69/49/20`. The leaf is scoped to measure the trajectory before withdrawing anything, so
  "per-commit counter" is proved the way `.6` and `LIVE-DOCUMENT-PRESSURE-HEADROOM.5` proved theirs rather
  than inferred from two endpoints.
- `2026-08-29`: closed `.6a`. The trajectory was measured over the 28 consecutive revisions since `.6`, plus
  `.6`'s own older `50775894` anchor, in a detached worktree, each with its own checker, and it answers the question the leaf opened with: `6 registered` became 5 **inside
  `5fe81128`**, alongside the closure triple, so nothing in `.6`'s "confirmed unchanged" list was true when
  `.6` wrote it. All four are withdrawn from `TOOLBOX.md` in favour of `--report` with their producer fields
  named; the six values that did hold at every measurement are carried. `.6`'s own unit trajectory and its
  stated growth rule are corrected as a fifth number in the same section. Opened `.6b` for the mdBook doctrine
  chapter and the census fact card, which publish the same counts and were outside `.6`/`.6a`'s stated sweep
  boundary. Added `[[worktree-doctrine-measurement-gitlink]]`, because the rig fails closed and silently until
  the `subs/fsmgen` gitlink is populated in the worktree.
- `2026-08-29`: closed `.6b`, which finishes the sweep `.6` started and `.6a` bounded. The mdBook doctrine
  chapter and the `current-claim-census-freeze` fact card both stop publishing census totals and route to
  `--report`; the card is retitled, because its old title published `56 exact authority units` as current.
  Attribution came free: the mdBook census's frozen contract is a tracked registry, so counting outcomes in
  each of its 31 revisions gives the exact trajectory with no worktree at all — a cheaper instrument than
  `.6a`'s, and worth reaching for first when the thing being measured is itself a tracked artifact. Both stale
  sentences were correct on the day they landed and wrong afterwards. `.9` gains its second and
  sharper demonstration: the chapter documenting the census held five candidate lines in 557, and none of them
  was a line the census got wrong.
- `2026-08-28`: `.7` gained its fourth instance and its repair, and `.8` was opened. `STATUS-LEDGER-ROLLOVER.4a`
  found `current-claim-census-frozen`'s own assertion stale and attributed it exactly: `.6` measured 86/51/35
  before applying its own `CHANGES.md` rollover, and that rollover moved 14 claim-annotated regions into an
  archive segment that is not a current surface. The volatile counters are withdrawn from the assertion. The
  same review measured that the census registry grows one record per ledger-prepending slice against a
  declared 128-record bound, which `.8` now owns.
- `2026-08-15`: Created from the owner's explicit adoption directive after confirming that SpecForge has no
  local claim-verification standard, claim registry/checker, or dedicated task tree.
- `2026-08-15`: `.0` maps the source checklist onto existing exact-currentness and mutation infrastructure,
  freezes scope, and identifies decision-record capacity as the required clean-boundary prerequisite to `.1`.
- `2026-08-15`: `.1` publishes the local standard and ADR 0042, requires one exact claim/no-claim declaration in
  commit and PR review, closes bootstrap/README/workflow-catalog discovery, and leaves mechanical provenance
  honestly pending for `.2`; `.1a` owns the newly visible 14/16 workflow-standard capacity warning first.
- `2026-08-15`: `.1`'s required engineering rationale crosses the development-note line rollover; the exact
  boundary-authenticated plan seals 15 whole records as segment 0008 and retains the current record in a warning-
  safe live root without editing any older archive member.
- `2026-08-15`: `.1a` re-derives 21 workflow slots from 14 current members and a four-member peak, preserves the
  stable explicit topology, and leaves exact authority retirement to `.1b` after commit.
- `2026-08-15`: `.1b` removes the consumed 16→21 authority while leaving the committed workflow profile and all
  of its current evidence unchanged; `.2` is executable.
- `2026-08-15`: `.2` activates ADR 0044's bounded executable evidence join, migrates three verified current
  claims, and registers the tenth doctrine without adding another hook or CI path; `.3` owns the constant sweep.
- `2026-08-15`: `.3a.0` derives the exact 39-surface current denominator and freezes the bounded five-view
  evidence schema before `.3a.1` implements any producer or `.3b` changes any source assertion.
- `2026-08-15`: `.3a.1` implements the self-bounded inventory contract and deterministic producer, proves its
  missing/unknown/duplicate/untracked/stale/bound controls, and leaves every finding unfrozen for `.3a.2`.
- `2026-08-15`: `.3a.2` repairs a first-view-only candidate-key defect before freezing 51 exact evidence units;
  five incomplete keys become the only legal `.3b` repair frontier and all source assertions remain unchanged.
- `2026-08-15`: `.3b.0` decomposes repair by authority: three maintained-reference routes, the authored-policy /
  registered-capacity split, exact mdBook quantitative regions, then one atomic result re-freeze.
- `2026-08-15`: `.3b.1` proves existing README, fact-card, and canonical-catalog route authorities plus their RED
  controls; no new checker or premature frozen-result rewrite is needed.
- `2026-08-15`: `.3b.2` reuses `workflow-standard-capacity-profile` for the exact actionable paragraph and keeps
  authored workflow identity outside measurement scope; no hand-carried copy or duplicate claim is added.
- `2026-08-15`: `.3b.3.0` freezes a prose-only quantitative candidate grammar and exact-region authority schema,
  separating inventory, adjudication, and result freeze before the outer census can consume it.
- `2026-08-15`: `.3b.3.1` implements the self-bounded inventory and exact-region validator, corrects the
  unreproducible design estimate from 301 to 304 stable candidates, and leaves semantic adjudication to
  `.3b.3.2` without changing the outer result.
- `2026-08-15`: `.3b.3.2` adjudicates every exact candidate, binds eight workflow-capacity lines to their verified
  claim, exposes 75 actionable lines with all three evidence legs missing, and excludes 221 exact authored,
  example/identity, or dated observations; frozen replay also repairs the record/array bound dimension defect.
- `2026-08-15`: `.3b.3.3` registers and routes the exact census-mapping authority with executable replay,
  controlled falsification, digest-complete staleness, and a Knowledge Map fact card; the verified claim preserves
  rather than certifies the 75 incomplete underlying assertions, and `.3b.3` closes.
- `2026-08-15`: `.3b.4` atomically replaces the five broad incomplete title anchors with narrow identity plus
  route/registered authorities, re-freezes 56 units with zero outer incompletes, proves all other 46 semantic
  outcomes unchanged, and closes `.3b` without promoting the 75 inner mdBook gaps.
- `2026-08-15`: `.3c` adds the missing reverse candidate join, expands controls from 15 derived-heavy cases to a
  27-case matrix spanning every outcome family and coverage boundary, proves the final lockstep boundary closes
  79 current candidates as 51 exact + 28 registered + zero unresolved, and independently closes `.3` from clean
  commit `50775894`.
- `2026-08-15`: `.4` binds all seven cited controls to exact known-bad producer regions, repairs the independent
  workflow feasibility probe with a controlled sub-ceiling RED path, derives six governed producers with zero
  ignored/untracked candidates, and expands claim-gate controls from 22 to 27 before handing signoff to `.5`.
- `2026-08-16`: `.5` publishes the runnable classification/author/auditor workflow and five-architecture join,
  repairs the stale public priority and exact production-graph test oracle, independently closes both claim
  censuses, runs selected full CI, and closes the adoption tree with `SPEC-TO-INTENT-ALIGNMENT.7` next.
<!-- claim-verification-task-source-region:changelog:end -->
