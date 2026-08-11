# CORPUS-COVERAGE — frontier census integrity

- Part ID: `frontier-census-integrity`
- State: `active`

## CORPUS-COVERAGE.4

- Status: `done` (`2026-08-11`) · Children: `.4.0` census correction (done), `.4.1` mechanical census gate (done)
- Goal: make the `.2` refresh frontier a **derived** count rather than a hand-carried decrement, so the
  program cannot silently lose a document from its own remaining-work queue.

`CORPUS-COVERAGE.3` closed the sibling defect — the frontier disagreeing with the artifact lifecycle — and its
acceptance said to "add a mechanical currentness check if the count is meant to stay live". The check that
landed covers **retention** (`doctrine/chain_currency/retained_bundles.json` versus the bundles on disk); it
does not cover the **remaining-work count**. `.4` closes that gap: `.4.0` corrects the census from measurement,
`.4.1` makes the corrected identity mechanically enforced.

## CORPUS-COVERAGE.4.0

- Status: `done` (`2026-08-11`, AUDIT/DOC; no child required)
- Goal: derive the `.2` cohort, refreshed, and remaining counts from persisted corpus evidence; correct every
  live surface that carries the hand-decremented count; and restore the one real chip-spec document the
  frontier had lost. Read-only with respect to `generated/` — no ingest, no artifact mutation.
- Children: none. The mechanical gate is a separate, independently reviewable code slice (`.4.1`).

### The measured census

Three properties are read from each document's own persisted `generated/source_ir/<key>/source_ir.json` plus
the retention declaration; nothing is inherited from a previous slice's table.

| Quantity | Rule | Count |
| --- | --- | ---: |
| Persisted documents | one `source_ir.json` per key under `generated/source_ir/` | 78 |
| Outside the `.2` cohort | `source.requested_path` begins `corpus/` — the tracked in-repo gold/eval corpus, never on the host-local library | 21 |
| `.2` cohort | persisted documents minus the in-repo corpus | **57** |
| Refreshed | cohort member with a retained normalized bundle **or** a `requested_path` no longer rooted at the retired boot volume `/Users/…` | **51** |
| Remaining | cohort member with neither witness | **6** |

The identity `57 = 51 + 6` holds exactly. Both witnesses are needed and each is load-bearing: 31 cohort members
carry a repository-relative `.cache/local-references/…` path and 19 carry a same-SSD `/Volumes/SSD/…` path, so
the path leg alone accounts for 50; `usb_3_2_revision_1_0_2017_09` was refreshed at `.2.33` before the volume
migration, so it still records a retired-volume path and is recovered only by the retained-bundle leg.

The six remaining documents, ranked by the established smallest-retained-source policy:

| Rank | Document key | Elements | Pages | Source bytes |
| --- | --- | ---: | ---: | ---: |
| 1 | `opencapi_25gbps_phy_mechanical_spec_v10` | 760 | 34 | 4,494,801 |
| 2 | `opencapi_3_0_transaction_layer_28jan2020` | 774 | 121 | 712,534 |
| 3 | `opencapi_3_1_transaction_layer_28jan2020` | 870 | 137 | 870,740 |
| 4 | `lpc_memory_agent_reference_design_guide_17jul2020` | 891 | 59 | 1,356,427 |
| 5 | `den0034_a_2013_09_13_debug_and_trace_configuration_and_usage_models` | 936 | 48 | 946,338 |
| 6 | `nvme_base_specification_2_0a_2021_07_26` | 4,577 | 454 | 5,154,704 |

Rank 6 is the restored document. `nvme_base_specification_2_0a_2021_07_26` is the 454-page NVM Express Base
Specification 2.0a; its source is present and reachable at
`.cache/local-references/chipdoc/nvm-express/nvme/current/NVMe-Base-Specification-2.0a-2021.07.26.pdf` at
5,154,704 bytes, and its persisted chain still records the retired boot volume with no normalized bundle. It is
an ordinary unrefreshed cohort member with no exception of any kind recorded against it.

### Root cause

The remaining count was **carried, not derived**. Every refresh from `.2.29` to `.2.51` decrements the previous
slice's number by exactly one — 27, 26, 25, … 8, 7, 6, 5 — so no later slice ever re-measured the denominator,
and any single error entered at the base would propagate untouched through twenty-two consecutive refreshes.

An error did enter at the base. The `.2.29` record reads "27 real chip-spec docs still normalized-missing
(plus the project README, not a chip spec)": the cohort was 57, one member was the project's own
`ingest README.md` artifact, and the denominator was therefore adjusted to 56 real chip-spec documents. That
adjustment was correct when written. It is not correct now — `generated/source_ir/` holds no `readme` key, so
all 57 present cohort members are real chip-spec documents — but because the count was only ever decremented,
the retired adjustment was never retired with it. The deficit of exactly one is what dropped the largest
remaining document out of view.

`.2.51` illustrates why re-measuring *candidates* was not enough to catch this. That slice deliberately
re-measured all six of its listed candidates from their own SourceIR profiles rather than inheriting `.2.50`'s
table — good practice, and it verified every row it had. But it re-derived the *rows*, not the *cardinality*:
a document already absent from the list cannot be validated by re-measuring the list. Only the cohort identity
`cohort = refreshed + remaining`, derived independently of any carried list, exposes an omission. That identity
is what `.4.1` will enforce.

### What this changes

The corrected product truth is **51 of 57 real chip-spec refreshes complete, with six remaining**. Stage
coverage, retention, emitted-ISF, and every other measured quantity are unaffected and unchanged: this is a
counting correction, not an artifact correction, and `CHAIN-CURRENCY` is green across it.

The next selection is also unaffected. NVMe is the largest of the six at 4,577 elements, so the
smallest-retained-source policy still selects `opencapi_25gbps_phy_mechanical_spec_v10` at 760 elements for
refresh `.2.52`. The correction changes the denominator and the length of the tail, not the next pick — but it
had to land before `.2.52`, because a refresh's ownership record states the frontier it selects from, and
recording a frontier already known to be wrong would have propagated the error one slice further.

Completed leaves are superseded, never rewritten. `.2.51`'s record keeps its own measurement and gains a
pointer to this correction; the `.2.29`–`.2.50` chronology and the `.2.48`-era decision record keep their dated
snapshots.

### Acceptance

- [x] **REPRODUCE / MEASURE** — the tracked frontier claims 51 of 56 with five remaining; the derived census
  over 78 persisted `source_ir.json` files and the retention declaration is 57 cohort / 51 refreshed / six
  remaining, with `57 = 51 + 6` exact.
- [x] **ROOT CAUSE (WHY + WHERE)** — the count is hand-decremented once per refresh across `.2.29`–`.2.51`
  with no re-derivation, and the `.2.29` record's one-document README adjustment outlived the artifact it
  described; the omitted document is `nvme_base_specification_2_0a_2021_07_26`, present and reachable, with no
  exception recorded against it.
- [x] **ADDRESSED (verified)** — the restored six-document frontier is re-derived from persisted evidence, and
  the root, index/manifest/contract, resume pointer, current status, change ledger, book, and fact surfaces
  state 51 of 57 with six remaining.
- [x] **NO REGRESSION** — no file under `generated/` is read for mutation or written; `check_chain_currency.sh`
  exits 0 at evidence 23/23, semantic 78/78, intent 78/78, isf-adapter 78/78 with retention exactly the 23
  declared bundles; the active-task-evidence contract, the complete doctrine driver, and the mdBook build pass.
- [x] **GENERICITY (ADR 0006)** — the census rule is structural: cohort membership is the persisted source
  root, and the refreshed witness is retention or persisted-path state. No document, vendor, or protocol name
  participates, and NVMe is admitted by the same rule that admits the other five.
- [x] **LOCKSTEP** — root, part, index, manifest, doctrine contract, `MEMORY.md`, `LIVE_ACHIEVEMENT_STATUS.md`,
  `CHANGES.md`, the book's live-docs chapter, and the containment fact card agree on the corrected census.

### Verification log

| Date | Boundary | Result |
| --- | --- | --- |
| `2026-08-11` | derived census | 78 persisted / 21 in-repo / 57 cohort / 51 refreshed / six remaining; identity exact; both refreshed witnesses load-bearing (`usb_3_2_revision_1_0_2017_09` recovered by retention alone) |
| `2026-08-11` | omission identity | `nvme_base_specification_2_0a_2021_07_26` satisfies the unrefreshed test, its 5,154,704-byte source is present under the authorized `chipdoc` symlink, and no tracked surface records an exception for it |
| `2026-08-11` | propagation | the remaining count decrements by exactly one across every refresh `.2.29`–`.2.51`; no slice re-derives the denominator |
| `2026-08-11` | selection unaffected | NVMe is the largest of the six, so smallest-retained-source still selects `opencapi_25gbps_phy_mechanical_spec_v10` at 760 elements for `.2.52` |
| `2026-08-11` | no artifact movement | `check_chain_currency.sh` exits 0 — evidence 23/23, semantic 78/78, intent 78/78, isf-adapter 78/78, retention exactly the declared 23 |

### Decisions and incidents

- `2026-08-11`: the correction is taken before `.2.52` rather than folded into it. A refresh's ownership record
  is the durable statement of the frontier it selected from; writing one from a frontier already known to be
  short would have made the next slice's evidence wrong in the same way, and the fix would then have had to
  correct two records instead of one.
- `2026-08-11`: the mechanical gate is deliberately **not** in this leaf. This slice is an audit that touches
  only Markdown and one contract literal; the gate is a script with its own self-test and doctrine-registry
  wiring, and mixing them would put a code change and its own motivating measurement in one unreviewable
  commit. `.4.1` owns it, and until it lands the corrected count is still carried rather than enforced — that
  is the honest residual of this slice.
- `2026-08-11`: no per-document exception is introduced for NVMe, and none is needed. It was never excluded by
  a rule — it fell out of a hand-maintained list. Its refresh will be selected by the ordinary
  smallest-retained-source policy when the five smaller documents ahead of it are complete, which places it
  last in the tail at `.2.57`.

### Commit log

| Unit | Durable evidence |
| --- | --- |
| `CORPUS-COVERAGE.4.0` | `CORPUS-COVERAGE.4.0 — derive the corpus frontier census and restore the lost document` |

## CORPUS-COVERAGE.4.1

- Status: `done` (`2026-08-11`, CODE/DOC; no child required)
- Goal: a mechanical census gate that derives cohort / refreshed / remaining from persisted corpus evidence and
  fails when the tracked frontier disagrees, so the identity `cohort = refreshed + remaining` cannot drift
  again.

### What landed

`CORPUS-FRONTIER` is the eighth registered doctrine, gate-tier, wired into `scripts/check_doctrines.sh` through
the adapter `scripts/check_corpus_frontier.sh`. The core is `scripts/check_corpus_frontier_census.pl`
(derive-and-diff archetype) against the declaration `doctrine/corpus_frontier/census.json`.

Cohort and refreshed are derived; remaining is declared and then attacked four ways, each catching a failure the
others cannot see:

| # | Check | Catches |
| --- | --- | --- |
| 1 | **IDENTITY** — derived cohort and refreshed equal the declared `expected` | a count that drifted from the artifacts in either direction |
| 2 | **MEMBERSHIP** — every declared-remaining key exists, is in the cohort, and retains no bundle | a stale declaration naming a document that has since been refreshed |
| 3 | **OMISSION** — no cohort member outside the declared set still has a retired-root path and no bundle | the original defect: a document dropped from the list, invisible to any check over that list |
| 4 | **PROSE** — the root task file states exactly the declared counts | the read frontier and the derived census saying different things |

Check 3 is the one the defect needed, and it is why the rule scans the whole cohort rather than the list.

### Design decisions

- **Declared, not inferred, remaining set.** Mirrors `retained_bundles.json`: the derivation needs something
  exact to disagree with, and a set is strictly stronger than a count — check 2 caught a seeded swap whose
  cardinality was still correct.
- **The retired-root prefix lives in the declaration, not the executable.** It is workstation-shaped and will
  change again when the library moves; that makes it data an owner revises under a named leaf, not logic. No
  document, vendor, or protocol name appears in any check (ADR 0006) — NVMe is admitted by exactly the rule
  that admits the other five.
- **Gate-tier, not CI-tier like its `CHAIN-CURRENCY` sibling.** Only each SourceIR's `source` object is needed,
  so the check reads a bounded 8 KiB prefix and locates the object with an exact brace scan that tracks string
  and escape state — a naive scan would miscount a brace inside a path, and a regex would guess. The whole
  corpus costs 52 ms measured, so the drift is caught at commit time rather than after merge.
- **Skips loudly on an absent corpus.** A fresh clone and a hosted runner have no `generated/`; silence would
  read as a pass, so the skip is printed and proven by a self-test case.

### Acceptance

- [x] **REPRODUCE / MEASURE** — before this leaf the corrected census was carried in prose with nothing binding
  it to the artifacts; `.4.0`'s own record names that as its honest residual.
- [x] **ROOT CAUSE (WHY + WHERE)** — no check related the tracked frontier to persisted evidence.
  `CORPUS-COVERAGE.3`'s currentness check covers retention only, and `CHAIN-CURRENCY` proves artifacts match the
  binary, not that the frontier matches the artifacts. `check_corpus_frontier_census.pl` closes exactly that gap.
- [x] **ADDRESSED (verified)** — the gate reports `57 cohort = 51 refreshed + 6 remaining` and agrees with the
  declaration, retention, and the root. Two **live negatives on the real corpus** fail closed: reproducing the
  historical defect (NVMe removed, denominator shortened to 56/five to match) raises five violations including
  the omission; and a swap that keeps the cardinality correct while listing a refreshed document
  (`usb_3_2_revision_1_0_2017_09`) still raises two.
- [x] **NO REGRESSION** — no Rust changed, so every extraction oracle is orthogonal by construction. The driver
  reports **all seven executed doctrines PASS (8 registered)** with `CHAIN-CURRENCY` deferred as registered, and
  the ten-case self-test passes, including the absent-corpus skip and the braced-path brace-scan case.
- [x] **GENERICITY (ADR 0006)** — the rule is structural (persisted source root, bundle retention) with every
  workstation-shaped prefix in the declaration; the executable contains no document, vendor, or protocol name.
- [x] **LOCKSTEP** — `DOCTRINE_ENFORCEMENT.md` §10, the book's doctrine-enforcement chapter, `TOOLBOX.md`,
  `CHANGES.md`, `LIVE_ACHIEVEMENT_STATUS.md`, the root, and the resume pointer record the new doctrine.

### Verification log

| Date | Boundary | Result |
| --- | --- | --- |
| `2026-08-11` | self-test | 10/10 — clean fixture, dropped remaining, inflated denominator, identity-breaking refreshed count, declared-remaining-with-bundle, non-cohort key, disagreeing root prose, duplicate key, absent-corpus skip, braced-path brace scan |
| `2026-08-11` | real corpus | `57 cohort = 51 refreshed + 6 remaining`; declaration, retention, and root frontier agree; 52 ms |
| `2026-08-11` | live negative 1 | the historical defect reproduced against the real corpus raises 5 violations, naming `nvme_base_specification_2_0a_2021_07_26` as absent from the declared set |
| `2026-08-11` | live negative 2 | a cardinality-preserving swap raises 2 violations, so a correct count cannot hide a wrong set |
| `2026-08-11` | driver | `ALL 7 executed doctrines PASS (8 registered, tier=gate)` with `CHAIN-CURRENCY` deferred |

### Commit log

| Unit | Durable evidence |
| --- | --- |
| `CORPUS-COVERAGE.4.1` | `CORPUS-COVERAGE.4.1 — gate the corpus frontier census as the eighth doctrine` |

### Current locality follow-up (`SPEC-TO-INTENT-ALIGNMENT.4b`, 2026-08-11)

The original omission check treated a `requested_path` outside the retired boot-volume prefix as a refresh
witness. That was sufficient for the historical migration sequence, but it coupled two independent facts:
where a source currently lives and whether the document completed a current-binary refresh. The director's
required cleanup of every stale boot-volume reference exposed the defect: all five unfinished documents could
name their correct SSD source while remaining unfinished, and the path-based omission scan would then lose the
ability to detect one being dropped from the declaration.

`SPEC-TO-INTENT-ALIGNMENT.4b` supersedes that current mechanism, not the historical measurements above. The
contract now declares exact, disjoint `refreshed` and `remaining` sets over the independently derived 57-member
cohort. The gate rejects an absent member, overlap, duplicate, out-of-cohort declaration, retained-but-not-
refreshed member, remaining member with retention, count drift, and prose drift. Its 13-case self-test uses SSD
paths for both refreshed and remaining fixtures, proving that path locality cannot change lifecycle state. The
real result remains 57 = 52 + 5.
