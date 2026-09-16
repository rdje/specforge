# LIVE-DOCUMENT-PRESSURE-HEADROOM — registry bands and gates

- Part ID: `registry-bands-and-gates`
- State: `legacy`

<!-- pressure-headroom-task-source-region:registry-band-nodes:start -->
- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.21`
  Status: `done` (`2026-09-15`; opened the same day by `CLAIM-VERIFICATION-ADOPTION.15`)
  Goal: partition `docs/tasks/CLAIM-VERIFICATION-ADOPTION.md`, which is 14 bytes from refusing its own
  next leaf
  Acceptance: the file is **278,514 of 278,528 bytes** (`task_evidence.bytes_each`, both bands). This is
  the BYTE axis, not the `task_evidence.files` count this tree already removed in `.2a`/`.2c` — a
  different bound on the same surface, and it is the one that now binds.
  **Measured `2026-09-15`, and the growth is not historical drift but ordinary current work**: five
  leaves closed that day (`.8`, `.12`, `.13`, `.14`, `.15`) took the file from **263,917 to 278,514**,
  **+14.6 KB in one session**. `.13` had to be compressed TWICE to land, and `.15` could not add a note
  to its own frontier table — the file can no longer document its own state, which is the operational
  definition of "not writable" this tree exists to prevent.
  **The pressure is structural, not stylistic.** `CLAIM-VERIFICATION-ADOPTION` is an ACTIVE tree with
  `.16` already open and a standing pattern of closing leaves by measurement — each of which carries its
  re-derivation commands and counts, which is exactly the evidence the doctrine requires and refuses to
  let anyone trim. Compressing prose a third time would trade signoff evidence for bytes.
  **Remedy to decide, not assumed** — `ACTIVE-TASK-EVIDENCE-CONTAINMENT` is `done` and states it must not
  be reopened, so its mechanism is precedent, not owner. `.5` routed a constant preamble out of a task
  file and `.19`/`.20` split a book chapter twice; the candidate shapes are (a) partition the closed
  leaves into a bounded history file with an index, the way this tree's own history is held, or (b) route
  per-leaf evidence blocks to a sibling under `docs/tasks/claim-verification-adoption/`, the way
  `spec-to-intent-alignment/` and `corpus-coverage/` already are. Pick with a measured before/after and
  a re-derivable index, not by taste.
  **Do not widen `bytes_each`.** This tree's own doctrine is that no ceiling is widened to hide pressure
  (`.2b` refused a banked increase on the very next commit), and the byte band here is health AND
  enforcement at the same value, so a widening would remove the warning as well as the stop.
  Blocks: any further `CLAIM-VERIFICATION-ADOPTION` leaf, including the already-open `.16`.
  Prerequisite: none; found by `CLAIM-VERIFICATION-ADOPTION.15` while being refused the room to record
  its own decision
  **Shape (b), and the reason is that only one candidate has a writer.** The accepted active-task-evidence
  contract (ADR 0039 topology, ADR 0046 sharded route catalog) already ships an atomic, rollback-safe
  materializer, a derive-and-diff index/route-catalog/manifest, and a gate that refuses a byte the
  contract does not derive. Shape (a) — a bounded history file with a hand-written index — has none of
  those, so adopting it would have meant building a second containment mechanism to avoid reusing the
  first. `ACTIVE-TASK-EVIDENCE-CONTAINMENT` stays closed: the contract is applied here, not reopened.
  **Eleven parts cut by reader concern, not by size.** `program-foundation` (`.0`-`.5`),
  `census-and-withdrawal` (`.6`-`.7`), `provenance-gate` (`.7.0`-`.7.2.1a`), `standard-readoption`
  (`.8`, `.10`-`.11a`), `registry-capacity-and-repin` (`.12`-`.15`), `count-currency-and-grammar`
  (`.7.3`, `.9`), `decisions`, `closure-records`, `acceptance-checklists`,
  `verification-and-chronology`, and the one **active** part `current-and-open-work`, which carries the
  only open leaf `.16`, the frontier table, and the open questions. Seventeen contiguous regions cover
  lines 1-2801 with no gap and no overlap; a part owns several regions where its concern is not
  contiguous in the accreted source, which is why no byte had to move to make the cut land.
  Verification: **the partition is lossless, proved independently of the checker that wrote it.**
  Re-harvesting only the marker-delimited payloads out of the eleven part FILES and concatenating them in
  declared source order reproduces the committed pre-migration blob **byte-for-byte**, SHA-256
  `938f909f88e71f3b335549232be4c5388a9d91a41846de5d1d15553b76c7674b` for all **278,514** bytes; the
  archived capsule is that same blob; all **46** `- ID:` node declarations survive in the parts and all
  46 are re-declared in the bounded root's owner registry. **The stop is gone, and the number says so**:
  the root is **8,157 bytes — 2.9% of the 278,528-byte ceiling**, from 99.995%, and
  `task_evidence.bytes_each` now reports `EXTRACTION-QUALITY-GAUGE.md` at 258,302 as the surface maximum.
  No ceiling, health target, or milestone was widened anywhere. `perl scripts/check_active_task_evidence.pl
  --contract doctrine/live_document_size/claim_verification_task_evidence.json --report` is
  migrated/complete with **0 uncorroborated routes** — every one of the 46 declared lifecycles is
  cross-checked against its own node's `Status:` line — and the route catalog's open set is exactly
  `{.16}`. The bounded index is 40 lines of a 128-line health target. `scripts/check_doctrines.sh`: all
  15 executed gate-tier doctrines PASS.
  **Registration, stated because it is the part a reader cannot see from the diff.** Four surfaces join
  `doctrine/live_document_size/surfaces.jsonl` (index, parts, route parts, archive), the contract joins
  `scripts/check_task_evidence_contracts.sh`, the current-claim census denominator moves **41 -> 44**
  with three included dispositions, and the one census evidence region the partition moved is re-pointed
  by CONTENT — `docs/tasks/CLAIM-VERIFICATION-ADOPTION.md:2347` to
  `docs/tasks/claim-verification-adoption/acceptance-checklists.md:209`, digest unchanged and exactly one
  candidate location, so the re-point could not land on the wrong line. Two `surface_disposition` records
  restate the existing `task_evidence` dated-evidence exemption for bytes that did not change.
  **The continuation protocol this leaf installed was exercised on `2026-09-16`, and a topology that
  cannot take its next leaf is not contained, only compressed.** `CLAIM-VERIFICATION-ADOPTION.17` was
  declared through it: the node appended to the ONE active part **outside every marked region** (the
  legacy payloads stay byte-immutable), one `- ID:` line added to the bounded root's owner registry, a
  `post_migration` leaf route with no `source_literal` added to the contract, the active part's pinned
  `sha256`/metrics re-derived, and `--write` regenerating the index, route catalog and manifest. The
  bounded landing now routes **2 open of 47** declared leaves — which is the property ADR 0046 bought:
  the index measures work in flight, so it grew by one row while the tree grew by one leaf and the
  other 45 stayed in the catalog. `--check` is migrated/complete with the new route's `open` lifecycle
  cross-checked against its own node's `Status:` line.
  **One correction from that exercise, recorded rather than quietly fixed.** Editing the contract with
  a JSON writer that was not the one `--migrate` uses reformatted all 1,281 lines — 3-space indent but
  `"key": value` where the materializer emits `"key" : value`. Nothing failed, because the contract is
  validated by content and not by bytes, but it left an authority in a form its own writer would never
  produce. It is normalized back, and the file now round-trips: re-encoding it through
  `JSON::PP->new->canonical(1)->pretty(1)` reproduces it byte-for-byte. The distinction worth keeping
  is that the contract is an authored **input** that `--migrate` happens to write once, not a derived
  projection like the index, route catalog and manifest, which `--check` does compare byte-for-byte
  against their generator. So this is tidiness, not the ADR 0046 drift defect — but a hand edit to a
  JSON authority should use the emitter that authority was written with.
  **One finding, owned rather than reported**: the surface registry is now **61 of its declared
  `max_records: 64`**, and that bound has no warning band at all — `check_live_document_size.pl` errors
  only when it is exceeded. The next partitioned tree needs four records and would fail the build with no
  prior notice. `.22` owns it. **This commit's own message published `62 of 64` and that count does not
  re-derive**: `max_records` bounds the *surface* records, and `read_jsonl_registry` shifts the registry
  meta record off before counting, so 62 file lines are 61 bounded records. Corrected here rather than
  left standing; `.22` re-derived it from the loader's own arithmetic.
  Commit: see log.

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.22`
  Status: `done` (`2026-09-15`, CODE; opened the same day by `.21`)
  Goal: give the surface registry's own record capacity a warning band or a declared remedy
  Acceptance: `doctrine/live_document_size/surfaces.jsonl` is **61 of its declared `max_records: 64`**
  after `.21` registered four surfaces for one partitioned tree — 62 file lines, because the loader shifts
  the registry meta record off before it counts. The portable hard cap in
  `scripts/check_live_document_size.pl` is **128**, so the declared bound is self-imposed and could move —
  but the defect is not the number, it is the **shape**: `max_records` has **no milestone block and no
  warning band**. The checker emits one unconditional error, `has more records than its declared
  max_records`, and nothing warns below it. Every other bound this registry enforces on the surfaces it
  describes carries `warning_pct`/`rollover_pct`; the registry does not apply that discipline to itself.
  **The stop is reachable by ordinary compliant work and it is two records away.** One partitioned task
  tree costs exactly four records — index, parts, route parts, archive — as `.21` just demonstrated, so
  the next tree that needs containment fails the build with no prior notice, in the commit that is trying
  to remove a different stop. That is the `LIVE-DOC-STOP-RISK` shape, and it is the shape this tree
  exists to refuse.
  **Two sibling registries measured at the same time, so the decision is taken over the class rather than
  one file.** `doctrine/claim_verification/current_claim_census.jsonl` is **121 of `max_records: 128`**
  and has the same milestone-free shape, but not the same risk: `CLAIM-VERIFICATION-ADOPTION.8` measured
  its rollover lifecycle retiring records 2.6x faster than they accrete (127 -> 115 net over 120
  revisions), so it has a remedy the surface registry lacks.
  `doctrine/claim_verification/published_assertions.jsonl` is 41 of 128 and is not near anything.
  Decide from measurement whether a registry's record count is a resource worth bounding at all — the
  question `.2a` asked of the task plane and answered with ADR 0045 — or whether it needs the milestone
  block every surface it governs already has. Do not simply raise 64; a raise with no band relocates the
  same silent stop to 128, and the single-use ceiling-increase authority protocol (`.2b`, `.4b`) applies
  to any raise that does land.
  Prerequisite: none; found by `.21` while registering the four surfaces a partition requires
  **Answered by measuring the whole class, not this one file.** Ten bounded JSONL registries declare
  `max_records`; **zero** declare a milestone block, and three are already above 90% of it —
  `surfaces.jsonl` 61/64, `current_claim_census.jsonl` 121/128, `book_quantitative_claims.jsonl`
  469/512. So the defect is the shape of a registry header, not the size of one number.
  **Removal was considered and refused on the relocation test `.2a` established.** `max_records` and
  `max_bytes` measure the same resource — the cost of reading one file whole — and which binds first
  depends only on the mean record size: for `surfaces.jsonl` records bind at 64 against bytes at
  roughly 83, while for `claims.jsonl` bytes bind at about 13 against records at 64. Deleting the
  record bound would therefore hand the stop to `max_bytes`, which has the **same** milestone-free
  shape, and fix nothing. The band fixes both dimensions at once.
  **The band, not the bound.** `read_jsonl_registry` accepts and now REQUIRES `milestones` in the
  registry header, with the same `warning_pct`/`rollover_pct` vocabulary and the same validation the
  surfaces use, and reports `max_records`/`max_bytes` pressure through the existing warning channel.
  One difference is stated in the code rather than left for a reader to infer: a surface measures
  pressure against a health target BELOW its ceiling, while a registry declares one number per
  dimension and that number IS the stop, so the percentages measure distance to the refusal itself.
  **No bound moved anywhere in this leaf.** 64 stays 64.
  **What the band immediately revealed, which is the point of adding it.** `surfaces.jsonl` reports
  **rollover 95.3% — 3 below its 64 max_records**, and re-deriving the history shows it was **57 of 64
  = 89.1% before `.21` ran**: already past the 80% band for a month, with nothing able to say so. The
  growth is episodic rather than linear — 25 -> 56 records over six days of the containment adoption,
  then +1, +1 over the next 32 days, then +4 in one commit — so a rate is the wrong instrument and the
  event size is the right one: one partitioned tree costs 4, and 3 remain. `.22a` owns that.
  **A second enforcer, exactly as `.2a` warned.** `scripts/check_derived_state_contracts.pl` reads the
  same `surfaces.jsonl` header with its own `reject_unknown_fields` list, so the first gate run after
  the field was added reported `derived-state: surface registry registry record has unknown field
  'milestones'` — the change looked complete against its own checker while a second reader refused it.
  `.2a` found the same shape in `$MAX_TASKS`, and the lesson generalises: a registry header has more
  than one validator, and an allowed field must be allowed in every one. The other readers of this file
  (`check_claim_verification.pl`, `check_current_claim_census.pl`, `check_book_quantitative_claims.pl`,
  `check_canonical_collection_catalogs.pl`) take records rather than validating the header, so two
  loaders was the true count — measured by running the gate, not by reading the greps.
  Verification: `scripts/test_live_document_size.pl` **99/99** (declared count re-derived 93 -> 99
  beside the suite, per `CLAIM-VERIFICATION-ADOPTION.7.3`), six of them new. **RED observed in two
  different ways rather than asserted.** First, adoption was fail-closed: with the requirement added
  and neither registry yet declaring a block, the real tree reported `surface registry registry record
  must declare milestones` and `ceiling-authority registry registry record must declare milestones`,
  2 violations. Second, suppressing ONLY the `registry_pressure` call — leaving the field accepted, so
  every fixture stays valid — turns exactly **5 of the 6** new cases red. The sixth, `registry below
  its band reports no pressure`, stays green, **and that is the honest result**: a suppressed band also
  reports nothing, so the silence case cannot discriminate alone, which is precisely why the three
  positive cases exist beside it. `perl scripts/check_live_document_size.pl` 991 Markdown files / 61
  governed surfaces, and `ceiling_increase_authorities.jsonl` at 0 of 32 stays silent, so the band does
  not fire on a registry that is not under pressure.
  Commit: see log.

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.22a`
  Status: `done` (`2026-09-15`, CODE; opened the same day by `.22`)
  Goal: restore surface-registry headroom, so the next partition is not refused by a bound the band can
  now only watch
  Acceptance: `.22` made the pressure visible and deliberately moved nothing; this leaf decides the
  number. `surfaces.jsonl` holds **61 of `max_records: 64`** against a portable hard cap of **128**, and
  one partitioned task tree costs exactly **4** records, so the present headroom of 3 is already smaller
  than one compliant event. Size the raise from the measured event, not from the percentage: derive how
  many further containment migrations the plane can be expected to need — the trees still on `inline`
  route catalogs and any tree approaching `task_evidence.bytes_each` — and set the bound so at least two
  such events fit above the warning band rather than below the stop. **A raise consumes a single-use
  ceiling-increase authority** (`doctrine/live_document_size/ceiling_increase_authorities.jsonl`, the
  `.2b`/`.4b` protocol): the authority is declared in the same commit as the raise and a following leaf
  retires it, or the very next commit refuses it as banked. Also check `max_bytes` in the same
  measurement — at the current mean record size it binds at roughly 83 records, so a record bound raised
  past that relocates the stop onto the byte bound instead of removing it, and this leaf must state
  which one it leaves binding.
  **The premise was wrong, and finding that out was the leaf's first result.** This leaf was opened
  assuming the single-use ceiling-increase authority protocol already covered the raise. It did not:
  `validate_ceiling_history` compares a **surface's** `enforcement_ceilings` across Git and the
  authority record is keyed by `surface_id`, so a registry **header** bound was outside the protocol
  entirely. Demonstrated rather than reasoned: raising `max_records` 64 -> 96 with no authority
  declared left the gate **green and silent** — and worse than silent, because the raise removes the
  `.22` warning by deleting the reason for it. A band that can be switched off by moving the bound it
  measures is not a control. So the protocol extension had to land before the raise, not after.
  **The header joins the protocol it already enforced on everything else.** An authority now names
  exactly one of `surface_id` or `registry_id`; a `registry_id` authority must name the surface
  registry itself, carries `old`/`new` over `max_records`/`max_bytes`, and is compared against
  `git show HEAD:` exactly as a surface ceiling is — same exact-match rule, same banked refusal.
  Only those two bounds are authorised: `max_record_bytes`, `max_array_items` and `max_scalar_bytes`
  bound authoring shape rather than population, so raising one is not a capacity event.
  **Then, and only then, the raise — sized from the event.** `max_records` **64 -> 96** and
  `max_bytes` **65,536 -> 98,304**, consuming exactly one authority. 96 is not a round number chosen
  for looks: one partitioned tree costs **4** records and **3** remained, while measured near-term
  demand is about **10** — `EXTRACTION-QUALITY-GAUGE.md` at 258,302 of 278,528 (**92.7%**) will need
  partitioning, `WIRE-BASED-100.md` at 74.9% is heading for the same band, and the two `inline` route
  catalogs each cost one more when they shard. At 96 the registry sits at **61/96 = 63.5%**, admits
  **three** partition events below the 80% band and **eight** below the stop.
  **`max_bytes` moves with it, and the relocation is stated rather than absorbed.** The two bounds
  measure the same resource, so raising only the record count would have handed the stop to the byte
  bound: at the measured **796-byte** mean record, 65,536 bytes binds at about **82** records, below
  the new 96. At 98,304 the byte bound binds at about **123**, so the record bound stays the binding
  one — which is the comprehensible one, since growth arrives in whole records. Both stay under their
  portable hard caps of 128 and 131,072.
  **This commit consumes the authority, so the very next commit refuses it as banked** unless `.22c`
  retires it. That is the protocol working, not a defect.
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.22`
  Verification: `scripts/test_live_document_size.pl` **105/105** (declared count re-derived 99 -> 105),
  six of them new and covering the protocol in both directions: a header raise with no authority is
  refused; one exact fresh authority is accepted; a **near-miss** authority naming a different
  destination than the one written is refused; an unused registry authority is refused as banked; an
  authority naming both a surface and a registry is refused; and a `registry_id` naming a different
  file is refused. RED observed on the real tree before the protocol existed — the 64 -> 96 probe
  passed with `991 Markdown files satisfy 61 governed surfaces` and no diagnostic — and again after,
  where the same probe reports `surface registry increased header bounds without exact authority:
  max_records`. GREEN with the authority: 991 files / 61 surfaces, and the `.22` rollover warning is
  gone because the headroom is real, not because the bound moved out from under it.
  Commit: see log.

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.22c`
  Status: `done` (`2026-09-15`; opened the same day by `.22a`)
  Goal: retire the single-use registry-header authority `.22a` consumed
  Acceptance: the authority record naming
  `registry_id: doctrine/live_document_size/surfaces.jsonl` authorised exactly one raise, which has
  landed. From the next commit onward `validate_registry_bound_history` finds no increase against
  `HEAD`, so the record is unused and `'<id>' has unused or banked ceiling-increase authority` fails
  the build — the same refusal `.2b` and `.4b` retired for surface authorities, now proven to reach
  the header too. Observe that refusal RED on the real tree first, then remove the record and confirm
  green; a retirement that never saw the refusal has not tested the protocol it closes
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.22a`
  Verification: **the refusal was observed on the real tree before the record was touched.** At
  `39425655`, with the raise now in `HEAD` and therefore no longer an increase against it,
  `perl scripts/check_live_document_size.pl` reported
  `'doctrine/live_document_size/surfaces.jsonl' has unused or banked ceiling-increase authority`,
  1 violation — the first time that refusal has fired for a **header** authority rather than a surface
  one, which is the half of `.22a`'s protocol a synthetic fixture alone could not prove. Removing the
  one record returns the registry to 1 record and the gate to green: 991 Markdown files / 61 governed
  surfaces. **The raise it authorised is untouched and must be**: the header still reads
  `max_records: 96` / `max_bytes: 98304`, so what expired is the permission, not the capacity. The
  single-use property is therefore demonstrated end to end for the new authority kind — granted,
  consumed, refused when stale, retired — in three commits, exactly as `.2a`/`.2b` and `.4a`/`.4b`
  demonstrated it for surface ceilings.
  Commit: see log.

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.22b`
  Status: `done` (`2026-09-15`, CODE; opened the same day by `.22`)
  Goal: give the other nine bounded registries the band `.22` gave the surface registry
  Acceptance: `.22` measured ten bounded JSONL registries and fixed the two that
  `check_live_document_size.pl` loads. The other eight are read by their own checkers with their own
  loaders — `check_current_claim_census.pl`, `check_book_quantitative_claims.pl`,
  `check_published_assertions.pl`, `check_claim_verification.pl`, and the task-node removal registry —
  and each repeats the same milestone-free shape. Two are already in the band nothing reports:
  `current_claim_census.jsonl` at **121 of 128** and `book_quantitative_claims.jsonl` at **469 of 512**.
  The census one has a remedy the surface registry lacks — `CLAIM-VERIFICATION-ADOPTION.8` measured its
  rollover lifecycle retiring records 2.6x faster than they accrete — so the right output is a band, not
  a raise, and the band is what tells a future session which of the two situations it is in. Prefer one
  shared loader over four copies of the same twenty lines if the readers can agree on a header shape;
  decide that from the four headers rather than assuming it. `.22` also measured that a header has more
  than one validator — `check_derived_state_contracts.pl` refused the new field until it was allowed
  there too — so this leaf must enumerate the readers of each registry before changing any header
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.22`
  **Neither option in the acceptance was taken, and the third one is better than both.** A shared
  loader was measured and refused: there is **no shared Perl library in this repository** — `FindBin`
  appears in ten gate scripts and resolves the repository root, never a module — so every gate script
  is standalone by design, and introducing `scripts/lib/*.pm` would change that property for all of
  them to fix a reporting gap. Copying the band into seven more loaders was the other option and is
  seven places to drift. The band is instead computed **once, centrally**, in
  `check_live_document_size.pl`, over the registries **discovered** in the tracked `doctrine/` tree.
  Each registry still declares its own `milestones`; only the arithmetic is shared.
  **Discovered, not declared, and that distinction is the fail-closed property.** A declared list of
  registries can be left short of a new one silently — the same defect `.7.2.0` fixed for governed
  surfaces in the claim census. The observer enumerates tracked `doctrine/**/*.jsonl`, treats a file
  as a registry only when its first record says so, and refuses any it finds without a band. Adoption
  proved it: all **eight** remaining registries were discovered and refused in one run before any of
  them declared one. Tracked-ness is also the boundary that keeps an untracked scratch file out.
  **The other loaders change by one line each, not twenty.** Six Perl header validators and one
  Python one had to ALLOW the new field — `check_book_quantitative_claims.pl`,
  `check_claim_verification.pl`, `check_current_claim_census.pl`, `check_published_assertions.pl`,
  `check_canonical_collection_catalogs.pl` (which uses `require_exact_keys`, so the field is required
  there rather than merely permitted), and `check_task_node_retention.py`;
  `check_derived_state_contracts.pl` needed it too. **A sweep that ran each checker with the wrong
  arguments reported a false green**: `check_rolling_ledger_protocol.pl --check` prints a usage error
  and exits without validating anything, so it counted zero rejections while genuinely refusing the
  field — the composed gate caught it. Run a checker the way its gate runs it, or the sweep measures
  argument parsing rather than behaviour.
  Verification: `scripts/test_live_document_size.pl` **108/108** (declared count re-derived 105 ->
  108), three of them new: a discovered registry with no band fails closed, a discovered registry
  reports its own pressure, and a discovered `.jsonl` whose first record is not a registry is left
  alone. Adoption RED first — eight `registry record must declare milestones` violations naming every
  remaining registry. All ten bounded registries now declare a band.
  **Editing seven checkers staled seven region pins the tracked re-pinner cannot reach, and that gap
  is a finding of its own.** `scripts/repin_claim_regions.py` re-pins the top-level `region` of the
  three claim registries; it does not reach `claims.jsonl` at all, nor the nested
  `falsification.controls[].red_evidence.source_region` and `control.red_case` regions that pin line
  ranges **inside the checker scripts themselves** — and those stale on exactly the same insert. Seven
  of them moved here. They were repaired by the tool's own rule rather than by a first-match
  throwaway: enumerate every window in the file whose digest matches and refuse unless exactly one
  does. All seven resolved to exactly one candidate, so nothing was guessed. `.22e` owns extending the
  instrument so the next session does not repeat the reasoning.
  **What the band revealed on its first run, both previously invisible, both owned rather than
  reported.** `book_quantitative_claims.jsonl` is at **rollover 91.4%** — 468 of 512 records, 44
  below the stop; `current_claim_census.jsonl` is at **rollover 93.8%** on records (120 of 128, 8
  below) and **warning 81.4%** on bytes. The census one has a remedy the other lacks:
  `CLAIM-VERIFICATION-ADOPTION.8` measured its rollover lifecycle retiring records 2.6x faster than
  they accrete, so its band is a status light rather than a countdown. The book registry has no such
  lifecycle; `.22d` owns it.
  Commit: see log.

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.22d`
  Status: `done` (`2026-09-16`; opened `2026-09-15` by `.22b`)
  Goal: give the mdBook quantitative registry a declared remedy before its record bound refuses a
  book edit
  Acceptance: `doctrine/claim_verification/book_quantitative_claims.jsonl` holds **468 of
  `max_records: 512`** — the first measurement of it that anything reported, because `.22b` is what
  made it visible. Growth is re-derived from its own history rather than estimated: **5 -> 469
  records across 217 revisions** since `2026-08-15`.
  **`.22b` first published a rate here that does not re-derive, and it is withdrawn.** It read
  `309 -> 469 = +160 over 216 revisions, about 0.74 per revision`, and attributed the large step to a
  chapter split. Both are wrong. The history has **two** one-off steps, not one: `5 -> 309` seeded the
  registry (`.3b.3.2`), and `351 -> 463` at `f7c77609` was `CLAIM-VERIFICATION-ADOPTION.9` **widening
  the candidate grammar** — it re-classified prose that already existed. No chapter split is involved
  at all. Excluding both, the steady rate is **+42 over 194 revisions = 0.216/revision** before the
  widening and **+6 over 25 = 0.240** after it, i.e. **0.219 combined** — a third of what was
  published. At that rate the remaining **43** records are about **196 revisions**, not 58.
  **The corrected model changes what the remedy must defend against.** The rate is comfortable; the
  EVENT is not. One grammar widening consumed **112 records, 22% of the whole bound, in a single
  commit**, and the population is a function of the grammar rather than of authoring volume: widen
  what counts as a prose candidate and every already-written line that now matches needs a record.
  So size against a widening, not against a rate, and state which bound is left binding — at the
  measured 369-byte mean record, `max_bytes: 262144` binds at about 710 records, below any
  `max_records` raised past that.
  **The durable owner is `CLAIM-VERIFICATION-ADOPTION`, and unlike `.18` the reason for carrying a
  claim-verification finding here has expired.** `.18` was parked in this tree because that tree was
  at 91.2% of its byte ceiling and a new leaf there would have spent the axis its own commit had to
  protect; `.21` removed that constraint. So the first thing this leaf should decide is whether to
  move itself and `.18` — and moving one exercises the post-migration continuation path (bounded root
  plus the one active part plus the contract's route registry plus `--write`), which nothing has
  exercised yet.
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.22f`, which is what makes the raise governable at all
  **A lifecycle was looked for first and does not exist at a useful rate.** A region record retires
  only when its candidate line leaves the manual, and the manual's prose is split and moved far more
  often than deleted — `.19` moved two whole blocks between chapters and retired nothing, because a
  split re-points records rather than removing them. So unlike the current-claim census, which
  `CLAIM-VERIFICATION-ADOPTION.8` measured retiring 2.6x faster than it accretes, this population only
  grows. The remedy is capacity, and capacity has to be authorised.
  **Raised to 896 records / 393,216 bytes, and the number is argued from the event.** 512 admitted
  today's 469 plus **43**, while one measured grammar widening costs **112**. 896 holds 469 plus one
  such widening plus 400 revisions of steady growth at **74.6%**, below the warning band.
  **It deliberately stops short of the portable cap.** `check_book_quantitative_claims.pl` fixes the
  hard caps at 1,024 records and 524,288 bytes. Raising to 1,024 would have bought the most headroom
  and left the declared bound **equal to the cap** — health and enforcement at the same value with no
  band between them, which is the exact defect `.21` was opened to fix on a different axis. 896 leaves
  **128 records and 131,072 bytes** of reserve, so a future capacity event still has a legal raise.
  **`max_bytes` moves with it and the relocation is stated.** At the measured **368-byte** mean record,
  262,144 bytes binds at about **712** records, below the new 896; at 393,216 it binds at about 1,068,
  so the record bound stays the binding one. Both bands light up near the stop rather than one
  silently overtaking the other.
  Verification: the raise is refused without its authority — `.22f`'s probe on this exact registry
  reported `registry '…/book_quantitative_claims.jsonl' increased header bounds without exact
  authority: max_records` — and accepted with the single-use record this commit declares and consumes.
  `perl scripts/check_book_quantitative_claims.pl --check` is green at 42 book files / 464 candidate
  lines / 464 adjudicated regions; the rollover warning that `.22b` surfaced is gone because the
  headroom is real, not because the band was removed. The consumed authority is refused as banked on
  the very next commit unless `.22g` retires it.
  Commit: see log.

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.22g`
  Status: `done` (`2026-09-16`; opened the same day by `.22d`)
  Goal: retire the single-use authority `.22d` consumed
  Acceptance: the authority naming
  `registry_id: doctrine/claim_verification/book_quantitative_claims.jsonl` authorised exactly one
  raise, which has landed. From the next commit `validate_registry_bound_history` finds no increase
  against `HEAD`, so the record is unused and the build fails with
  `has unused or banked ceiling-increase authority`. Observe that refusal RED on the real tree first —
  it will be the first time it fires for a registry OTHER than the surface one, which is the half of
  `.22f` a fixture alone cannot prove — then remove the record and confirm green.
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.22d`
  Verification: **the refusal fired on the real tree, for a registry that is not the surface one, before
  the record was touched.** At `34add927`, `perl scripts/check_live_document_size.pl` reported
  `'doctrine/claim_verification/book_quantitative_claims.jsonl' has unused or banked ceiling-increase
  authority`, 1 violation. That is the half of `.22f` a fixture could not prove: the generalized
  protocol expires a permission for **any** registry it governs, not only for the one `.22a` was
  standing on. Removing the record returns the authority registry to 1 record and the gate to green at
  992 Markdown files / 61 governed surfaces, and the header still reads `max_records: 896` /
  `max_bytes: 393216`, so the permission expired and the capacity it granted stands.
  **Two authorities have now been granted, consumed, refused when stale and retired in one session —
  one on the surface registry, one on a claim registry** — so the single-use property is demonstrated
  end to end for both halves of the generalized protocol rather than for the path that happened to be
  built first.
  Commit: see log.

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.22f`
  Status: `done` (`2026-09-16`, CODE; opened the same day by `.22d`'s first measurement)
  Goal: govern every registry header, not the one `.22a` happened to be standing on
  Acceptance: `.22a` put the surface registry's header inside the single-use authority protocol and
  `validate_authority_schema` pinned a `registry_id` authority to that one path. `.22b` then gave all
  **ten** bounded registries a warning band. The two together left a hole with the exact shape `.22a`
  existed to close: nine registries now *report* pressure and none of them needs an authority to raise
  the bound that produces the report. **Demonstrated on the real tree, not argued**: raising
  `book_quantitative_claims.jsonl` from `max_records: 512` to `1024` with no authority declared left
  the gate **green and silent**, and erased its own `91.4%` rollover warning in the same stroke.
  An authority may now name **any registry this checker discovers** — the same tracked
  `doctrine/**/*.jsonl` population `.22b` enumerates — and the header history is compared per registry
  against `git show HEAD:` under the same exact-match rule and the same banked refusal. No bound moved.
  **A silent-disable bug was found inside the change and is the more valuable half of it.**
  Moving the observer inside `validate_ceiling_history` put it under that function's `local $/`, and
  `chomp` removes `$/` — so with `$/` undef it is a **no-op**, `git_top()` returned the repository path
  with a trailing newline, the `$git_top ne $root` guard fired, and discovery returned **zero
  registries**. The band and the new authority check both went quiet while every gate still reported
  PASS. It was caught only by re-running the probe that had just failed and finding it passing again.
  The fix is at the source rather than at the call site: `git_top()` sets its own separator and strips
  the terminator with an explicit substitution, so no caller's slurp can disable it.
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.22a`, `.22b`
  Verification: the ungoverned-raise probe on `book_quantitative_claims.jsonl` passed green before the
  change and reports `registry '…/book_quantitative_claims.jsonl' increased header bounds without exact
  authority: max_records` after it. `scripts/test_live_document_size.pl` **110/110** (declared count
  re-derived 108 -> 110), two new cases covering a discovered registry's raise refused without an
  authority and accepted with an exact one, plus two existing patterns updated because the diagnostic
  now names the registry instead of saying "surface registry". The band still reports all three real
  pressures after the fix, which is the regression guard for the `chomp` defect.
  Commit: see log.

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.22e`
  Status: `done` (`2026-09-16`, CODE; opened `2026-09-15` by `.22b`)
  Goal: extend the tracked re-pinner to the regions it cannot currently reach
  Acceptance: `CLAIM-VERIFICATION-ADOPTION.12` built `scripts/repin_claim_regions.py` so that
  re-pinning is a tracked instrument that refuses ambiguity rather than hand work that guesses. It
  covers the top-level `region` of three registries — **562 regions across 63 files** when it was
  written. It does **not** cover `doctrine/claim_verification/claims.jsonl` at all, and within the
  registries it does read it does not descend into
  `falsification.controls[].red_evidence.source_region` or `control.red_case`. Those pin line ranges
  **inside the checker scripts**, so any edit to a gate script shifts them, and `.22b` moved seven at
  once while `--check` reported `unchanged 565` — the tool was right about its own population and
  blind to the rest. Extend it to those fields, deriving the file from the record's own `path`,
  `producer`, or `inputs[0]` as each shape requires, and add the refusal cases to its RED matrix.
  Re-derive the published region total afterwards: the `562` in `TOOLBOX.md` §7.7 and in the `.12`
  checklist is the covered population, and widening coverage moves it, so it must be read from
  `--check` rather than edited.
  Prerequisite: none; found by `.22b` when its own script edits staled pins the instrument missed
  **Three record shapes, and the first version required all of them to look like the one it knew.**
  `classify()` demanded a `region` sub-object with `kind: line_range_sha256` and a `path` sibling on
  the same node. `control.red_case` carries its span and digest **inline with its own `path` and no
  `kind` at all`; `red_evidence.source_region` carries `kind` but **no path**, because the file is
  named by the enclosing control's `producer` or `inputs[0]`. Each shape is matched explicitly rather
  than by a generic search for anything digest-shaped: a region resolved against a file nobody named
  is the wrong-landing failure this instrument exists to refuse, wearing a different disguise. A
  control that names no file at all is therefore **skipped**, not guessed at, and a case holds that.
  Verification: **the gap was demonstrated as a real-tree control before and after, not argued.** One
  line inserted at line 31 of `scripts/check_claim_verification.pl`, then both versions run at the
  same path against the same tree: the version at `HEAD` reported **`unchanged 565`, zero moved, exit
  0** — health, while three regions were genuinely displaced — and this version reports **`moved 3,
  unchanged 571`**, naming each one. Restored, the tree reports `unchanged 574`.
  **Coverage 565 -> 574 regions**: seven `red_evidence.source_region` pins in `claims.jsonl`, which
  the tool never opened, and two `control.red_case` pins in a registry it did read.
  `--self-test` is **19/19**, six of them new, covering both shapes, the `inputs[0]` fallback, a
  region with no `kind`, and the no-named-file skip. The matrix now declares its own expected total
  beside the suite (`PRODUCTION-GRAPH-CENSUS-PIN.3`), and that declaration was itself proven
  fail-closed: setting it to 18 against a 19-case run reports the disagreement and exits 1.
  **The published totals are repaired the way `.7.2.1a` decided, not re-measured.** `TOOLBOX.md` §7.7
  carried `562 pinned regions across 63 files` and a `13-case RED matrix`; both are per-commit
  counters, and a re-measured counter is stale on the next commit. The counts are removed and the
  command named instead, the three shapes are documented there, and the entry now says to run the
  tool rather than quote it. The `562` inside the `.12` acceptance checklist is left exactly as it
  is: it sits in an immutable legacy payload and is a dated observation correctly scoped to the day
  it was measured, which is what this repository's Non-Goals protect.
  Commit: see log.


<!-- pressure-headroom-task-source-region:registry-band-nodes:end -->

<!-- pressure-headroom-task-source-region:staleness-gate-node:start -->
- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.18`
  Status: `done`
  Goal: execute the staleness gate every claim declares, or stop calling it a gate
  Acceptance: `scripts/check_claim_verification.pl` executes `rederive.commands` and
  `falsification.controls` under `--execute` but **never** `durability.stale_check`; it only schema-validates it
  and checks that its `inputs` cover the watched artifacts. So every claim's staleness gate is decorative, and
  the proof is that `current-claim-census-frozen`'s pinned `stdout_contains` read `current-claim-census: 39
  current surfaces` while the producer printed 40 before this slice and 41 after — a marker that would have
  refused for at least one prior commit had anything run it. `.14a` repaired that one pin to the stable phrase
  the claim's own assertion requires (the denominator is read from `--report`, never carried); this leaf
  decides whether the stale gate is executed, folded into the rederive commands, or removed. The durable owner
  is `CLAIM-VERIFICATION-ADOPTION`; it is carried here because that tree is at **91.2%** of its
  `task_evidence.bytes_each` ceiling and a new leaf there spends the axis its own commit must protect
  Prerequisite: none; found by `.14a` while refreshing the claim pins its inputs moved
  **Decided `2026-09-16`: the gate is executed, and finding out WHY it never was is most of the leaf.**
  The premise was confirmed first, as an A/B on the real registry: replacing a `stale_check`'s
  `stdout_contains` with text its producer can never print left the HEAD checker at **exit 0**, and the same
  perturbation against this version exits 1 naming the marker. `durability.stale_check` was schema-validated
  and input-covered but never passed to `execute_declared_command`.
  **It could not simply be executed. Three of the five declared staleness gates name THIS checker as their
  producer** — because the thing that would detect a claim about the claim-verification contract going stale
  is this run — so a naive execution re-enters the process and each nested run re-enters again. The first
  attempt did not terminate and had to be killed. A gate is therefore executed only when a DIFFERENT producer
  discharges it; a self-referential one is discharged by the run in progress, recognised from the ARGV that
  would actually re-enter rather than from the declared producer string.
  **No extra guard was needed against claiming that exemption falsely, and the dead code was removed rather
  than kept.** A first version refused a record whose `producer` named this checker while its argv invoked
  something else; the self-test showed the existing `argv does not invoke its declared producer` rule fires
  first, so the pairing is already enforced and the guard was unreachable.
  **Tier, from a corrected measurement.** The first cost figure was wrong and is withdrawn: HEAD appeared to
  run in **1.0 s** only because it exited early on unrelated stale digests, and `validate_registry` skips
  command execution when errors already exist. Measured cleanly on one working tree, the gate tier is
  **30.4 s** and the full staleness tier **55.2 s**, and the 24.8 s difference is dominated by
  `check_current_claim_census.pl`, which this same driver pass ALREADY runs twice — once as a doctrine and
  once as this claim's `rederive` command. So the staleness gates execute under
  `--execute-stale-gates` / `CLAIM_VERIFICATION_EXECUTE_STALE_GATES=1`, which
  `scripts/check_doctrines.sh` exports for `--all`; the summary always reports **executed / deferred /
  self-referential** separately, because the defect was never the cost — it was output that implied
  verification which had not happened.
  Verification: `A/B on the real registry: a false staleness marker passes at HEAD (exit 0) and is refused here by name; self-test 27 -> 32 cases under an independently declared total, including the marker-unmet case that was missing for the gate's whole life, its met counterpart, and three direct assertions of the self-reference discriminator; gate tier reports 0 executed / 2 deferred / 3 discharged at 30.4 s and the CI tier 2 / 0 / 3 at 55.2 s; the pinned 27/27 self-test marker moved to 32/32 and the census marker was restored after a global substitution over-applied it; all 16 executed gate-tier doctrines PASS`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.18 — execute the staleness gate, and say which ones did not run`

<!-- pressure-headroom-task-source-region:staleness-gate-node:end -->
