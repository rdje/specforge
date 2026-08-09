# Derived-state containment adoption delta — `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.8a`

Owning leaf: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.8a` (PROBE/DOC; no production-code or canonical-artifact
change).

## Question and review boundary

The director requested a fresh review of FSMGen's forwardable live-document containment guide after SpecForge
ramp-up. SpecForge adopted the earlier package on `2026-08-08`; the donor doctrine inspected here was updated on
`2026-08-09`. The sibling checkout was an explicitly authorized, read-only external authority on the same SSD.
No donor path, threshold, measurement, registry row, task ID, or local conclusion is copied.

Captured identities:

| Artifact | Lines | Bytes | SHA-256 |
| --- | ---: | ---: | --- |
| FSMGen adoption guide (`docs/LIVE_DOCUMENT_SIZE_CONTAINMENT_ADOPTION_GUIDE.md` in the donor) | 419 | 20,515 | `6f10b9504b07306e97342861b32820ca4f89998856e90b6ddc5604709b7ee744` |
| FSMGen neutral doctrine plus its fenced local note | 426 | 26,058 | `5cd37a241bd21fd2d76da95a4742a028fc4f901149b15c841a0d9391ab627d88` |
| SpecForge locally owned doctrine before `.8` | 319 | 19,253 | `bb91866c970f8f2ba0b41cb53de01fb48fe5d1362b950a6a1c6beaad09203a9a` |

The donor/local byte difference is expected because the fenced adoption note is project-owned. The review below
classifies only portable neutral semantics.

## Portable delta classification

| Donor neutral change | SpecForge result | `.8` disposition |
| --- | --- | --- |
| A membership index may be a separately governed bounded catalog, not necessarily inside its collection. | Already implemented by `.5c.iii`; canonical collection catalogs have direct membership and fail-closed coverage. | No new implementation. Align neutral wording only if the doctrine is edited for the substantive delta. |
| Generated projections may be one bounded file or a directly indexed bounded projection set. | Already implemented by `.5d.ii`; the Knowledge Map is a bounded landing plus generated question shards with derive-and-diff. | No new implementation. |
| Compare a duplicate with its authority and repair concealed divergence before demotion. | Already performed by `.5a`/`.5h`: two book contradictions and four stale root mirrors were repaired before compatibility demotion, but SpecForge's neutral doctrine still has the older weaker sentence. | Adopt the portable wording in `.8b`. |
| Exact current-state fields are derived on read or retained as verified copies; immutable evidence and authored intent are distinct; field discovery is explicit and bounded. | **Real local gap.** Surface freshness exists, but no field-level registry or neutral field checker exists. | GO on bounded local adoption in `.8b`, then independent `.8c` audit. |
| Minimum deliverables and completion test include the field classification and executed verifier plane. | Not represented in the closed `.0`–`.7` acceptance surface because this donor revision postdates it. | Reopen only the containment tree as `.8`; do not disturb completed migrations. |

## Existing local enforcement census

The resulting SpecForge registry currently has 41 governed surface records. Eight surfaces declare enforced
currency contracts, 15 records execute a non-budget lifecycle verifier, and the generated Knowledge Map declares
canonical inputs and freshness. These are valid surface-level verified copies and should be referenced rather
than duplicated.

There are zero field-level contract records and no registry naming exact markers, derive-on-read accessors,
verified-copy authorities, or immutable-evidence/authored-intent dispositions. Three concrete seams demonstrate
why the missing layer is not merely editorial:

1. `MEMORY.md` correctly omits a hand-maintained `HEAD`, but its current-state paragraph says corpus/cache counts
   “remain” exact without an executable fresh-clone authority. Those counts belong to the task/evidence boundary,
   not the resume pointer.
2. `README.md` and `docs/book/src/getting-started.md` publish Rust `1.95.0`; CI installs `1.95.0`; the workspace
   declares `rust-version = "1.95"`. The values agree, but no unconditional verifier proves the copies against
   the workspace authority.
3. `docs/FSMGEN_FEEDBACK.md` and its protocol contract store gitlink
   `d327129b718ab29fc889db026c19257b0f7fcc49`. The feedback checker proves the two copies agree with each other,
   but it does not derive the canonical gitlink from the Git index. The current index does contain mode `160000`
   and that exact object, so the missing check is deterministic and clone-stable.

## Measured field-family disposition

| Field family | Classification | Authority / accessor | Current state | `.8b` action |
| --- | --- | --- | --- | --- |
| Current repository revision | `derive_on_read` | `git rev-parse HEAD` at resume time | Exact value is correctly absent from `MEMORY.md`; the book names the accessor. | Declare the reader marker and forbid a stored latest-commit shadow. |
| Active unit, concise state, selected next action, in-flight work, blockers | `authored_intent` | Owning engineer/director under task-tree doctrine | Correctly bounded in `MEMORY.md`; not mechanically derivable as a whole. | Declare the region so a neutral checker does not misclassify it. |
| Selected PNT priority | `authored_intent` | Director/engineer judgment | Lives in the bounded status gap; task catalog supplies candidates, not the choice. | Declare the marker; no equality oracle. |
| Task-tree catalog | `verified_copy` | `docs/tasks/*.md` metadata | Existing `check_task_tree_catalog.pl` derive-and-diff passes. | Register the existing verifier; do not add a second generator. |
| Knowledge Map landing/shards | `verified_copy` | fact cards + decisions | Existing Knowledge Map derive-and-diff passes. | Register the existing verifier. |
| Roadmap workstream table | `verified_copy` | roadmap contract + owning task trees | Existing `check_roadmap_projection_contract.pl` passes. | Register the existing verifier. |
| Reviewed validation projection | `verified_copy` | owner-reviewed contract and pinned producer regions | Existing read-only currentness verifier passes. | Register the existing verifier and capture-boundary semantics. |
| Source-PDF registry | `verified_copy` | Git-indexed PDFs + Rust key derivation | Existing currentness verifier passes. | Register the existing verifier. |
| Corpus-KB managed projections | `verified_copy` | tracked fixtures + reviewed validation boundary | Existing currentness verifier passes. | Register the existing verifier. |
| Book/root current product contracts | `verified_copy` | Rust seams + canonical mdBook routes | Existing book-current-truth verifier passes, but exact Rust prerequisite is not included. | Register the verifier; extend only the prerequisite authority check. |
| FSMGen open/closed correspondence | `verified_copy` | feedback contract + response/evidence routes | Existing feedback protocol passes. | Register existing verifier separately from the gitlink field. |
| Rust prerequisite copies | `verified_copy` | workspace `rust-version`, normalized to patch form | Copies agree but are not gated. | Add exact declared markers and one deterministic authority comparison. |
| Current FSMGen gitlink copies | `verified_copy` | Git-index mode/object for `subs/fsmgen` | Copies agree but are self-referential. | Compare both declared copies to the index-derived object and mode. |
| Corpus/cache counts in `MEMORY.md` | misplaced current copy | Task/evidence records, not a fresh-clone current authority | Ungated exact-current wording. | Remove the counts from the resume pointer; retain one task pointer. |
| Dated task/status/ledger measurements | `immutable_evidence` | Work-unit/date plus the Git commit that records the unit | Historical, not claims about an ambient current tree. | Declare the governed record regions/capture rule; do not reverify historical numbers as current. |

## Selected `.8b` contract

1. Add a small bounded `derived_state_contracts.jsonl` data plane. Each record names an exact path and marker,
   one closed classification, its authority/capture rule, and the accessor or executed verifier where required.
2. Keep the common checker neutral: validate schema/bounds/path locality/marker uniqueness/classification
   requirements, then execute a project-owned adapter for the two exact local comparisons. No field is inferred
   from dates, number shapes, or prose.
3. Extend the unconditional live-document doctrine gate and fail-closed fixture suite for missing/off-surface
   markers, stored derive-on-read values, absent/degraded verified-copy execution, invalid classifications, and
   registry displacement.
4. Normalize Rust semver for comparison; derive the FSMGen pin from the Git index, which works without checking
   out the submodule worktree. The `.8a` resume-pointer overwrite removes the irrelevant corpus/cache counts;
   `.8b` declares and enforces the field classes so such a current shadow does not recur.
5. Adopt the neutral derived-state and duplicate-authority wording into SpecForge's locally owned doctrine; align
   the mdBook, bootstrap/commit readers only where their contracts change. No ceiling increase or donor sync.
6. Reuse and update the `.8` durable fact rather than minting another card. Its new catalog row leaves
   `docs/knowledge/INDEX.md` at 32,664 bytes: 88.6% of the 36,864-byte health target and 514 bytes below the
   first failing 90% rollover byte. If `.8b` would reach 33,178 bytes, partition under a separately owned leaf
   before the append; never trim evidence or widen the ceiling to pass.
7. `.8c` independently enumerates all declared markers and consumer paths, exercises both positive and mutation
   cases, then runs full doctrines/CI and closes `.8`.

## `.8b` implementation result

The selected contract is now implemented without widening scope. The bounded registry contains 14 exact
contracts: one `derive_on_read`, two `authored_intent`, one `immutable_evidence`, and ten `verified_copy` records.
Eight verified-copy records execute the existing task catalog, Knowledge Map, roadmap, reviewed validation,
source-PDF, corpus-KB, book/root, and FSMGen-correspondence authorities. Two records delegate only the selected
local comparisons to `scripts/check_derived_state_authorities.pl`.

The Cargo adapter reads workspace `rust-version`, accepts the local major/minor or major/minor/patch forms, and
normalizes all values to a patch triplet before comparing README, mdBook, and CI. The FSMGen adapter reads the
stage-zero `subs/fsmgen` entry from the Git index, requires mode `160000`, and compares its object with both the
feedback Markdown boundary and JSON protocol copy. It does not require the submodule worktree to be checked out.

The neutral suite adds 28 cases spanning all four classifications, exact-marker uniqueness, off-surface and
historical rejection, stored derive-on-read shadows, missing accessors/capture boundaries, core/adapter execution,
schema and registry displacement, and an undeclared date/number/hash lookalike that must remain ordinary prose.
The local suite adds 16 Rust/gitlink cases, including every retained-copy drift, malformed/duplicate values,
wrong Git-index mode, missing index entry, and invalid adapter declaration. The resulting tree reports all 14
contracts and both authority groups green; `.8c` still owns the independent consumer and mutation audit.

## Reverification

From the SpecForge root:

```bash
jq -s '{surface_records:([.[]|select(.surface_id)]|length), enforced_currency:([.[]|select(.currency.status? == "enforced")]|length), executable_surface_verifiers:([.[]|select(.surface_id and .verifier != "builtin:budget")]|length)}' doctrine/live_document_size/surfaces.jsonl
git ls-files --stage subs/fsmgen
rg -n 'rust-version|toolchain:' Cargo.toml .github/workflows/ci.yml
rg -n 'Rust `1\.95\.0`|d327129b718ab29fc889db026c19257b0f7fcc49|Corpus remains 80 SourceIR' README.md docs/book/src/getting-started.md docs/FSMGEN_FEEDBACK.md doctrine/live_document_size/fsmgen_feedback.json MEMORY.md
```
