# CLAIM-VERIFICATION-ADOPTION — registry capacity and re-pin

- Part ID: `registry-capacity-and-repin`
- State: `legacy`

<!-- claim-verification-task-source-region:census-capacity-node:start -->
- ID: `CLAIM-VERIFICATION-ADOPTION.13`
  Status: `done` (`2026-09-15`)
  Goal: decide what watches a published value on a surface that carries no claim tag
  Acceptance: the eleventh instance landed on `CHANGES.md:56` and **no governed population reaches it**,
  which bounds what `.7` closed. Derived from the producers rather than from the code: the current-claim
  census yields exactly one `CHANGES.md` candidate, its first non-blank line, on basis `surface_review`; the
  published-assertion gate yields none, because that surface carries zero `[claim: <id>]` tags and the
  discovered population is *files carrying a tag*; and the book census governs `docs/book/src/**` only. So
  the published-assertion gate's population is gated on an author remembering to write a tag — the check and
  the thing it checks share a parent, which is the general form `CLAIM_VERIFICATION.md` §2 names, applied to
  this repository's newest gate. This is **not** `.9`, which is the *book* census's closed noun vocabulary,
  a different producer and a different mechanism; state the difference in any decision rather than folding
  the two together. Candidate directions, none yet chosen: extend the discovered population from
  tag-bearing files to whole governed surfaces; or accept the bound explicitly and say so wherever `.7` is
  described as closed, which `.7.2.1a` has done as an interim. Do not close this by widening a glob
  **Twelfth instance (`2026-09-12`, from `EXTRACTION-QUALITY-GAUGE.3k`, a second surface CLASS):**
  `docs/tasks/*.md` carries no `[claim: <id>]` tag anywhere, and a task leaf is where this project states
  the population a rule was measured over — the number that decides whether the rule is worth shipping.
  `EXTRACTION-QUALITY-GAUGE.3i` published *"differs on 18 of 349"* as the population its successor
  inherited; `.3k`'s scoping re-derived it to 19 over that table and to **4** over the population the
  change could actually move, because three of the four producers of that record type never reach the
  function being edited. Nothing watched it, and the wrong number then steered the successor leaf's design
  for a day. This is the same mechanism as the `CHANGES.md:56` instance on a surface whose published values
  are *more* actionable, which argues for the "whole governed surfaces" direction over the "author
  remembers a tag" one — but the decision stays open here rather than being taken in a leaf of another tree
  Prerequisite: `CLAIM-VERIFICATION-ADOPTION.7.2.1a`
  **DECIDED `2026-09-15`: do NOT widen the discovered population; state the bound.** Both candidate
  directions were argued, never costed. Costed with the book census's own `is_candidate` grammar:

  | population | lines | files | growth |
  | --- | ---: | ---: | --- |
  | `docs/book/src/**` (existing frozen census) | **464** | 25 | frozen |
  | `docs/tasks/**` — "whole governed surfaces" | **4,987** | 145 | **+5.1 / commit** (4,825 -> 4,987 / 32) |
  | `docs/tasks/**` — decision-bearing `N of M` | **285** | 43 | **+1.2 / commit** (245 -> 284 / 32) |

  Refused on cost: the frozen model needs one adjudicated region per candidate, so widening is **4,987
  records on day one** — 10x the book registry beside it (`max_records: 512`) — then ~5/commit forever.
  Raising a bound cannot rescue that. The narrow `N of M` shape IS what the twelfth instance argues for
  (`.3i`'s *"18 of 349"* steered a successor's design for a day) and is buildable at 285, but +1.2/commit
  is ~+480 per push cycle, so it needs its own registry AND lifecycle — opened as `.16`, sized.
  Meanwhile the harm is bounded by something already shipped, which fired twice on `2026-09-15`: the rule
  that a published population is re-derived, not inherited, caught `.2h.0` withdrawing `.2d`'s 124 for 106
  and `.14` refusing a "4 of 5 stale" cascade. Neither was caught by a gate.
  Verification: counts from the book census's `is_candidate` applied per file; growth per revision from
  Git over 32 commits; `N of M` via `git grep -hoE '[0-9][0-9,]* of [0-9][0-9,]*'`.
  Commit: see log

<!-- claim-verification-task-source-region:census-capacity-node:end -->

<!-- claim-verification-task-source-region:repin-and-staleness-nodes:start -->
- ID: `CLAIM-VERIFICATION-ADOPTION.12`
  Status: `done` (`2026-09-15`)
  Goal: make the per-slice registry re-pin a tracked instrument instead of an ad-hoc rewrite each time
  Acceptance: three registries pin regions by one-based line range plus SHA-256 —
  `current_claim_census.jsonl`, `book_quantitative_claims.jsonl`, and `published_assertions.jsonl` — so **every
  slice that prepends to a rolling ledger or edits a governed file invalidates every row below the edit**, and
  the repair is currently performed by hand or by a throwaway script. Re-derived per commit rather than from
  a running counter (`git show <rev> -- doctrine/claim_verification/current_claim_census.jsonl | grep -c
  '^-{'`): `a88b91a3` 21, `3c17ae5c` 22, `3ff9e363` 23, `4529d535` 20 — every slice rewrites about twenty
  evidence rows in that one registry, plus a new ledger-head row and a fan of `sha256` refreshes across
  `claims.jsonl`. (**`.7.2.1a` withdrew "20 and then 24".** Those were the repair script's own counters from
  repeated runs *within* a slice, so they answered a different question than the sentence asked — the
  granularity rule in `CLAIM_VERIFICATION.md` §3 Leg 1. The per-commit figures above re-derive from Git.) That is mechanical work with a correctness hazard — a re-pin that silently lands
  on the *wrong* line is invisible, because the digest it was moved to match is the digest it now has.
  **The failure mode is specific, not hypothetical.** A row whose recorded content is a blank line has digest
  `01ba4719…546b`, which matches every blank line in the file, so a content search re-pins it to an arbitrary
  one; `.8` already recorded two dead rows that had drifted onto blank lines exactly this way. Any tracked
  instrument must therefore report ambiguity rather than resolve it, and must refuse rather than guess.
  Deliver a tracked script with its own RED matrix — ambiguous match, no match, a row whose content genuinely
  left the file — and route `COMMIT.md` to it. Do not widen a bound or relax a pin to avoid the work
  Prerequisite: `CLAIM-VERIFICATION-ADOPTION.7.2.1`

  **DELIVERED `2026-09-15`: `scripts/repin_claim_regions.py`** (`--check` / `--apply` /
  `--self-test`, `--path` to scope one governed file). It resolves every region BY CONTENT across all
  three registries, and its whole design is the refusal the acceptance demanded: `locate()` returns
  **every** start line whose window carries the recorded digest, never the first, and a region with
  more than one candidate is **REFUSED with all candidates printed** — with a named note when the
  digest is `01ba4719…546b`, the bare blank line. A region whose content left the file is refused as
  `ABSENT`; a pinned path that no longer exists is refused as `NO FILE`. **On any refusal the tool
  writes nothing at all**, so a run is all-or-nothing per registry and a diff shows the re-pins and
  nothing else.

  **Exposure, measured on the tracked tree `2026-09-15`: 562 pinned regions across 63 governed
  files**, of which **0 are ambiguous, 0 absent and 0 pinned to a bare blank line today**. The hazard
  is latent, and the blast radius is why it is worth an instrument rather than care:
  `docs/book/src/reference/live-docs.md` holds **280 blank lines behind 163 pins**, so one drifted
  row would have 280 candidate destinations and a hand repair would pick one silently. `.8` recorded
  two rows that had already drifted this way.
  **Demonstrated, not asserted**: a synthetic blank-line pin injected into a copy of the tree's own
  census is refused with all 280 candidate lines listed, exit 1, registry byte-unchanged.
  Verification: see the `.12` checklist below.
  Commit: see log.

## Acceptance Checklist — `.12` (enforced)
- [x] **REPRODUCE / MEASURE** — `python3 scripts/repin_claim_regions.py --check` over the tracked
  tree: **`unchanged 562`**, exit 0 — 562 pinned `line_range_sha256` regions across 63 governed files
  in the three registries. Blank-line ambiguity multiplier, per file:
  `live-docs.md` 280 blank / 163 pins, `trajectory.md` 70 / 43, `TOOLBOX.md` 88 / 28,
  `evidenceir.md` 226 / 23, `sourceir.md` 162 / 26.
- [x] **ROOT CAUSE (WHY + WHERE)** — the three registries pin evidence to a one-based line range plus
  a digest of exactly those lines, so any insert above a row invalidates it. The repair was a
  throwaway each time, and the throwaway's natural shape is *find the first line whose digest
  matches* — which is wrong precisely when it matters: `sha256("\n")` = `01ba4719…546b` matches every
  blank line in the file, so the first match is an arbitrary one. The re-pinned row then looks
  perfectly valid, because the digest it was moved to match is the digest it now has.
- [x] **ADDRESSED (verified)** — `locate()` enumerates all candidates; `classify()` returns
  `moved` / `unchanged` / `AMBIGUOUS` / `ABSENT` / `NO FILE`; any refusal suppresses the write for the
  whole run. **Demonstrated against the tree's own data**: a blank-line pin injected into a copy of
  `current_claim_census.jsonl` against the real `live-docs.md` is refused, prints all **280** candidate
  lines with the bare-blank-line note, exits 1, and leaves the registry byte-identical.
- [x] **NO REGRESSION** — `python3 scripts/repin_claim_regions.py --self-test` **13/13** locate,
  refusal and write-suppression cases pass, including: a blank line finds every candidate (not the
  first), a repeated MULTI-LINE window is ambiguous too, a clean shift re-pins and exits zero, and each
  of the three refusal classes writes nothing. `--check` over the live tree reports 562 unchanged and
  exits 0, so the instrument agrees with the state four doctrine gates already accept. No Rust,
  fixture or artifact is touched; `bash scripts/check_doctrines.sh` all executed doctrines PASS.
- [x] **GENERICITY (ADR 0006)** — the tool knows three registry paths and one digest convention. It
  reads no document, vendor or protocol vocabulary, and it derives every candidate from the governed
  file's own bytes.
- [x] **LOCKSTEP** — `COMMIT.md` routes the re-pin step to it instead of leaving it as hand work;
  `TOOLBOX.md` §7.7 registers it; the fact card
  `[[live-surface-edit-bookkeeping-chain]]` replaces its "re-anchor by content, never by offset"
  instruction with the command that does it and cannot guess. **Producer sub-clause: no production
  rule was deleted or replaced** — this is a repair instrument for derived state, not a producer.

- ID: `CLAIM-VERIFICATION-ADOPTION.14`
  Status: `done` (`2026-09-15`)
  Goal: a `stale_check` that carries a per-commit counter is stale by construction, and nothing executes it
  Acceptance: `mdbook-quantitative-census-frozen`'s `durability.stale_check.stdout_contains` read
  **`309 adjudicated region(s)`**. The checker reports **464**. The marker was wrong by 155 and had been
  wrong for a long time: at `HEAD~6` the registry already held **462** region records.
  **Two things are wrong, and the second is why the first survived.** (1) It **contradicts its own
  claim**: that record's `assertion` says its totals *"are per-commit counters, not constants ... derived
  on read"*, and its `rederive` marker is the SHAPE `"phase":"frozen","regions":` — the `stale_check`
  carried the counter the assertion forbids. (2) **No gate executes it**: `check_claim_verification.pl`
  runs `rederive.commands` and `falsification.controls` and never `durability.stale_check`, so the one
  wrong marker was the one nothing re-ran.
  **Audited rather than spot-fixed**: all five claims' `stale_check` commands were executed. **One of five is
  stale**; the other four already use shape markers (`current surfaces (`, `claim-verification:`) and pass.
  So this is a single defect, not a class, and the repair is to make the fifth match the four.
  Repair: the marker becomes `in 'frozen' phase with` — asserts the phase, carries no count. Non-goal:
  executing `stale_check` from the gate (`.15`); widening it to fail would have failed every prior commit.
  Prerequisite: none
  Verification: see the `.14` checklist below
  Commit: see log

## Acceptance Checklist — `.14` (enforced)
- [x] **REPRODUCE / MEASURE** — every claim's `durability.stale_check` executed and its
  `stdout_contains` tested against real stdout: **1 of 5 stale**.
  `mdbook-quantitative-census-frozen` wants `309 adjudicated region(s)`; the checker prints *"… in
  'frozen' phase with **464** adjudicated region(s)"*. `git show HEAD~6:…book_quantitative_claims.jsonl`
  counts **462** region records, so it was wrong before this session touched anything.
- [x] **ROOT CAUSE (WHY + WHERE)** — `claims.jsonl`,
  `mdbook-quantitative-census-frozen.durability.stale_check.stdout_contains`: a per-commit counter its
  own `assertion` forbids and its `rederive` marker avoids, surviving because
  `check_claim_verification.pl` executes only `rederive.commands` and `falsification.controls`.
- [x] **ADDRESSED (verified)** — marker is now `in 'frozen' phase with`; re-executed, all five pass,
  **0 of 5 stale**.
- [x] **NO REGRESSION** — one string in one registry record; no Rust, fixture or artifact.
  `check_claim_verification.pl --check` green; `scripts/check_doctrines.sh` all executed doctrines PASS.
- [x] **GENERICITY (ADR 0006)** — a marker string; no document, vendor or protocol vocabulary.
- [x] **LOCKSTEP** — the tree records the finding and the deliberately-unfixed gap (`.15`).
  **Producer sub-clause: no production rule was deleted or replaced.**

- ID: `CLAIM-VERIFICATION-ADOPTION.15`
  Status: `done` (`2026-09-15`) — decided, not built
  Goal: nothing executes `durability.stale_check`, so a staleness marker has no oracle behind it
  Acceptance: `.14` found the one wrong marker was the one nothing re-ran, which reads as a missing
  execution. **DECIDED: do not execute it — the field is not an unrun check, its producer already runs.**
  1. **3 of the 5 would be SELF-INVOCATION**: `claim-verification-contract-published`,
     `workflow-standard-capacity-profile` and `claim-provenance-gate-active` each declare
     `perl scripts/check_claim_verification.pl --check` — the very binary that would execute it.
  2. **The other 2 already run their sibling mode**: `mdbook-quantitative-census-frozen` and
     `current-claim-census-frozen` declare `--check`, while their `rederive.commands` declare `--report`
     on the SAME producer, and `rederive` runs every commit. Only a duplicate invocation is missing.
  3. **A marker-shape validator was refused too, on measurement.** The cheap gate — reject a
     `stdout_contains` carrying a digit — was tested against the registry: **9 of 17 carry one**, nearly
     all legitimately (`20/20`, `27/27`, `8/8` are self-test CASE counts, constant until a case is
     added). A per-commit counter and a constant are syntactically identical; only the producer
     separates them, so no syntactic rule can.
  Residual risk is `.14`'s defect, already forbidden by each record's own `assertion`; all five markers
  are now shape-only. Reopen only for a `stale_check` whose argv is NOT covered by its `rederive`.
  Prerequisite: `CLAIM-VERIFICATION-ADOPTION.14`
  Verification: the five `stale_check` argv and all 17 markers enumerated from `claims.jsonl`.
  Commit: see log

<!-- claim-verification-task-source-region:repin-and-staleness-nodes:end -->
