# EXTRACTION-QUALITY-GAUGE — kind span family

- Part ID: `kind-span-family`
- State: `active`

## Post-migration work

Leaves declared after the `2026-09-17` containment migration that continue the `.3k` kind-span
programme. The leaves that migration SEALED, and the records that close them, live in
[kind span successors](kind-span-successors.md) and [kind span scoping](kind-span-scoping.md); a sealed
record states its question and cannot receive its answer, so a sealed leaf that becomes current gets its
continuation here — the same route `ADR 0048` used for `.3j.1.b`.

- ID: `EXTRACTION-QUALITY-GAUGE.3k.9` · Status: `pending` — **DO NOT SHIP YET, on evidence**
  (continuation; the full analysis is the sealed record in
  [kind span successors](kind-span-successors.md), which cannot receive an answer) · Goal: **a Markdown
  escape fragments an identifier, and the fragment is then DECLARED as a signal.**
  Populations re-derived `2026-09-18` by `.3k.9.a`, unchanged: **TEXT 67 documents / CATALOG 18 names in
  2 documents / RECORDS 0**. Preconditions **(c)** — all 18 names adjudicated, all remove — and **(d)** —
  the circularity control, observed RED on the real corpus — are **discharged** by `.3k.9.a`.
  **What is left is (a) the corpus effect of unescaping at the shared tokenization seam, measured with
  `replay-constraints` **and** a full `scripts/check_doctrines.sh --all`, because this moves
  declarations and not only constraints; and (b) every current-schema document that moves rebuilt and
  diffed — AXI-L carries `AWSNOOP\_WIDTH` and `WSTRB\_Present`, so it will move.**

  **THE PARKING REASON IS FALSE, RE-DERIVED `2026-09-20`.** This leaf reads *"neither finishes inside
  a session, which is the reason this leaf is parked rather than blocked"*. Both were timed, in one
  session, alongside other work:
  `specforge replay-constraints --evidence-root generated/evidence_ir` is **60 s** (`real 60.18`), and
  `bash scripts/check_doctrines.sh --all` is **19 minutes** (`real 1148.86`) and reported **ALL 18
  executed doctrines PASS**. (b) is the same shape `EXTRACTION-GAP-FIX.5b` performed the same day: one
  document through `evidence -> semantic -> intent -> isf-adapter` with a validate per stage took about
  a minute, against a full pre-write snapshot, and the stratum re-verified at 27/27. **This leaf is a
  DECISION, not a scheduling problem**, and it should not be left parked on a timing claim that is not
  true.

  **THE REFUSAL REASON IS NOT FALSE, AND IT IS THE ONE THAT MATTERS.** Correcting the parking claim does
  not argue for shipping. The magnitude argument below is untouched by any timing: the only fix reaching
  the two contaminated documents moves the identity layer of **67** documents to correct **18 names in
  2**, with a published constraint effect of **0**. Whoever takes this leaf must decide it on that
  arithmetic — the runs are now merely affordable, not persuasive.
  **The magnitude argument that parked it still stands and must be re-read before shipping**: the only
  fix reaching the two contaminated documents moves the identity layer of **67** documents to correct
  **18 names in 2**, with a published constraint effect of **0**. `.3j.3` sharpens the case a little —
  catalog contamination bounds recall, and recall is this family's measured bottleneck — but it does not
  change the arithmetic, and this tree's own rule is that a rule nothing exercises does not ship.
  Prerequisite: `.3k.9.a` (done). Blocks: nothing.
  Verification: pending — (a) and (b)
  Commit: pending

- ID: `EXTRACTION-QUALITY-GAUGE.3k.9.a`
  Status: `done` (`2026-09-18`, ADJUDICATION + CODE)
  Goal: **discharge the two preconditions `.3k.9` can meet without a detached corpus rebuild — (c) the
  18 names adjudicated individually, and (d) a control for the circularity — and leave (a) and (b)
  cleanly scoped.** `.3k.9`'s disposition is DO NOT SHIP YET and this leaf does not disturb it: nothing
  here changes the identifier tokenization or any artifact.
  **(d) The circularity control now runs the classification, and it was observed RED.** The census
  already had a unit case pinning `prose_only`, which is the part; what it lacked was a control over the
  **decision**, which is what a future reader actually runs. `spurious_heads` now carries the whole
  classification in one function with an `exclude_synthesized` switch that exists only for its RED case.
  Two new cases: with the exclusion, an opaque `XQPART` fixture classifies as
  `{"XQPART": ["XQPART_ACCESS"]}`; without it, `{}`. **A/B on the real corpus**, with `prose_only`
  reduced to the identity: the census reports **`catalog_population: 0 declared name(s) across 0
  document(s)`** — the precise wrong answer this leaf's first re-derivation reached — and the self-test
  fails at *"classification finds the fragment"* and at the older unit case together. Restored: 8/8 and
  67 / 18 / 0. The circularity can no longer be re-derived as emptiness in silence.
  **(c) All 18 adjudicate the same way — remove — and the risk the leaf flagged is the strongest
  evidence for it, not against.** `.3k.9` warned that several are real English words (`POWER`, `USER`,
  `CLASS`, `NUMBER`) whose removal from a catalog might withdraw records correct for unrelated reasons.
  Read against the documents, the opposite holds: each has **zero** standalone UPPERCASE occurrences —
  that is what put it in the population — while carrying a large lowercase English population that is
  prose and never an identifier. Measured case-insensitively over prose: `POWER` 361, `OPERATION` 336,
  `CLASS` 170, `PARTITION` 161, `NUMBER` 114, `USER` 101, `PARTITIONS` 97, `LARGE` 79, `PRE` 60, `REQUEST`
  48 (GIC-600), `TAG` 46, `NATIVE` 34, `PARTITIONING` 27, `PRODUCTION` 26, `EXCEPTION` 25, `PROGRAM` 11,
  `PERIODIC` 1, `CONTEXT` 122. So `POWER` is declared a signal of eMMC on the strength of
  `POWER\_CLASS` and `POWER\_OFF\_LONG`, while the document writes the word *power* 361 times and the
  identifier `POWER` never. **The real identity in every case is the compound** — `PARTITION_ACCESS`,
  `USER_WP`, `TAG_UNIT_SIZE`, `REQUEST_COMPLETE` are genuine EXT_CSD fields and GIC-600 states; the head
  is the artefact. No adjudication splits, no name is retained, and the record population stays 0, so
  nothing is withdrawn today.
  **Two observations worth carrying rather than rediscovering.** `PRE_SOLDERING_` keeps a trailing
  underscore and `PRODUCTION_STATA_AWARENESS` sits beside `PRODUCTION_STATE_AWARENESS` — the source's own
  truncation and typo, not this stage's, and a reminder that the compound is copied rather than parsed.
  And the defect class is not unique to this mechanism: `ACTOR-NOUN-RELATION-DECLARATION` owns *an
  inferred declaration mints an ordinary word as a wire*, which is the same outcome by a different route.
  A fix at the tokenization seam addresses only this route.
  **What remains for `.3k.9`, unstarted — and no longer "detached", see the timings on `.3k.9`:**
  (a) the corpus effect of unescaping at the
  tokenization seam, measured with `replay-constraints` (60 s) and a full `scripts/check_doctrines.sh
  --all` (19 min); (b) every current-schema document that moves rebuilt and diffed — AXI-L carries
  `AWSNOOP\_WIDTH` and `WSTRB\_Present`, so it will move. Both need a run that does not finish inside a
  session; neither is blocked on a decision.
  Prerequisite: none. Blocks: nothing — `.3k.9` is nearer, not unblocked.
  Verification: the re-derived census, the RED A/B on the real corpus, and the per-name adjudication
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.9.a — control the circularity, and adjudicate all eighteen names`

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.9.a`

- [x] **REPRODUCE / MEASURE** — `python3 scripts/measure_escaped_identifier_fragments.py` re-derives
  **67 / 18 / 0** unchanged across the refactor, and `--self-test` reports **8/8** (was 6/6).
- [x] **ROOT CAUSE (WHY + WHERE)** — the census's circularity defence lived in `prose_only` and was
  pinned only at that function. `census` inlined the classification, so nothing exercised the decision a
  future reader runs; a regression in the exclusion would have surfaced as a *quiet zero*, which is the
  one failure mode this leaf exists to prevent.
- [x] **ADDRESSED (verified)** — **RED observed on the real corpus**, not only on a fixture: with
  `prose_only` reduced to the identity the census prints `catalog_population: 0 declared name(s) across
  0 document(s)` and the self-test fails at two cases; restored, 8/8 and 67 / 18 / 0. The classification
  is now one named function with its own controls.
- [x] **NO REGRESSION** — the script is read-only over persisted artifacts and the refactor is
  behaviour-preserving on the whole corpus (67 / 18 / 0 before and after). No Rust changed, so
  `PRODUCTION-GENERICITY` is unmoved and every artifact in `generated/` is byte-identical.
- [x] **GENERICITY (ADR 0006)** — the new control uses an opaque `XQPART` fixture, so the classification
  demonstrably reads the escape's structure and not a real name. The census reads a literal Markdown
  escape and a declaration form; no vendor, protocol or document vocabulary appears in it.
- [x] **LOCKSTEP** — no user-visible behaviour changed and no production rule was deleted; `.3k.9` ships
  nothing, so the book has nothing to state yet and will gain the rule if and when (a) and (b) land.
  Fact card: `[[escaped-identifier-fragment-adjudication]]`.
  **One structural note the gate taught rather than the plan**: a new semantic part is invisible to
  `LIVE-DOC-SIZE` until it is **tracked**, so the classification check passed for as long as the file was
  untracked and failed the moment it was staged. The part is now registered in
  `doctrine/live_document_size/surfaces.jsonl` under `extraction_quality_gauge_task_evidence_parts`. A
  green run before `git add` is not evidence for a new file.
