### WIRE-BASED-100.9c — AHB survives its re-ingest intact, and .9b's prediction that it would not is corrected

- THE ROUTE RAN UNCHANGED and restored the AHB chain to canonical authority: `DOCLING_DEVICE=cpu ingest` →
  `evidence` → `semantic` → `intent` → `adapt --target isf` on the current release binary, no model server —
  104 page artifacts / 70 visual / 0 residuals / `automation_confidence high`, schema now **SourceIR 3 /
  EvidenceIR 3 / SemanticIR 2 / IntentIR 2**. `scripts/measure_corpus_canonical_currency.py` moves from
  **25 measurable (32.1%) / 53 legacy** to **26 (33.3%) / 52**, and gold-carrying documents from **3 of 7** to
  **4 of 7**. The persisted chain was preserved with a SHA-256 manifest under
  `generated/preserved/WIRE-BASED-100.9c/pre-reingest/` BEFORE any command wrote over it.
- ALL FOUR CARRIED NUMBERS RE-DERIVE EXACTLY. `seed_ahb`: `signal_constraint P=R=F1=1.000` (tp=6 fp=0 fn=0)
  and `actor_signal_relation P=R=F1=1.000` (tp=6 fp=0 fn=0, source-tolerant + filtered), document-level recall
  6/6 constraints and 6/6 relations. `seed_ahb_temporal`: `temporal_rule P=R=F1=1.000` (tp=4 fp=0 fn=0).
  Per fact: constraints `HAUSER`/`HWUSER`/`HRUSER`/`HBUSER` `must_be_value VALID` plus `HAUSER`/`HWUSER`
  `must_not_change`; relations Subordinate→`HRESP`/`HREADYOUT`/`HRUSER`/`HBUSER` and
  Manager→`HAUSER`/`HWUSER`; temporal `HAUSER` and `HWUSER` unconditioned, `HRUSER` and `HBUSER` under
  `HREADY HIGH`.
- THE CORRECTION MATTERS MORE THAN THE PASS. `.9b` predicted `.9c` would lose its temporal score the same way
  APB did, because `.5` reuses `.4`'s index-family resolver for AHB `HSELx`. It did not, and the reason is
  exact rather than lucky: **the AHB temporal gold's only antecedent is `HREADY`, which the document declares
  with the spelling the prose uses, and no AHB gold item references the un-indexed `HSEL`.** So the defect
  `WIRE-BASED-100.4a` owns is confined to a gold whose antecedent names an INDEXED-FAMILY signal by its
  un-indexed prose spelling — which is narrower than "wire-wide" and wider than "APB-only": the mechanism
  still drops every such antecedent silently wherever no gold scores it, so `.4a` must fix the mechanism.
- THE REBUILD WAS STRUCTURALLY INERT for AHB, which is why the golds needed no re-anchoring
  (`content-anchored: re-resolved 0/17` and `0/4` — the ids were already current): the rebuilt EvidenceIR
  carries the SAME 1,322 statements / 172 anchors / 1,199 spans / 481 links / 70 visual records as the
  preserved legacy artifact. Only proof authority and identifier spelling moved. APB, by contrast, re-segmented
  by one statement (597 → 598).
- SAME TWO DISPOSITIONS AS .9b, BOTH ALREADY OWNED. The ISF adapter moved `renderable` → `blocked` on
  `no source-grounded system clock/reset contract` (`HCLK` demoted out of `(clock HCLK)` and into the
  interface), which is the documented policy for the whole measurable stratum; and the declared select moved
  `HSELX`/`HSELXCHK` → `HSELx`/`HSELxCHK` under identifier opacity, with no ISF signal lost. The normalized
  bundle is HELD OUT at `generated/preserved/WIRE-BASED-100.9c/ahb-normalized-bundle-held-out/` for the reason
  `RETAINED-BUNDLE-POPULATION-FROZEN` owns, so AHB's EvidenceIR replay reads UNMEASURABLE while its
  SemanticIR/IntentIR/adapter replays stay measurable and current.
- LOCKSTEP: the book's frontier statements move 25/53 → 26/52 across the SourceIR, EvidenceIR, SemanticIR,
  IntentIR, ISF-adapter, architecture-rationale, extraction-architecture and extraction-eval chapters;
  `.4a`'s scope note, `.9b`'s prediction and `.9d`'s expectations are corrected where they were published.

### WIRE-BASED-100.9b — the APB gold is scoreable again, and scoring it withdraws a published 1.000

- THE RE-INGEST RAN THE DEMONSTRATED ROUTE UNCHANGED and restored the APB chain to canonical authority:
  `DOCLING_DEVICE=cpu ingest` → `evidence` → `semantic` → `intent` → `adapt --target isf` on the current
  release binary, no model server, ~4 minutes — 48 page artifacts / 35 visual / 0 residuals /
  `automation_confidence high`, and schema now **SourceIR 3 / EvidenceIR 3 /
  SemanticIR 2 / IntentIR 2**. `scripts/measure_corpus_canonical_currency.py` moves from **24 measurable
  (30.8%) / 54 legacy** to **25 (32.1%) / 53**, and gold-carrying documents from **2 of 7** to **3 of 7**.
  The persisted chain was preserved with a SHA-256 manifest under
  `generated/preserved/WIRE-BASED-100.9b/pre-reingest/` BEFORE any command wrote over it, because the current
  binary refuses legacy input and cannot regenerate those bytes.
- TWO OF THE THREE APB SCORES RE-DERIVE EXACTLY. `seed_apb`: `signal_constraint P=R=F1=1.000` (tp=6 fp=0 fn=0)
  and `actor_signal_relation P=R=F1=1.000` (tp=5 fp=0 fn=0, source-tolerant + filtered), document-level recall
  6/6 constraints and 6/6 relations — identical, per fact, to what the tree carried. Content anchoring did its
  job across the re-segmentation the rebuild caused (extracted statements 597 → 598).
- THE THIRD DOES NOT, AND IS WITHDRAWN RATHER THAN CARRIED. `seed_apb_temporal` scores
  **`P=R=F1=0.333` (tp=1 fp=2 fn=2)** against the published `1.000`, so the `2026-06-06` "APB is 100% on ALL
  three extraction aspects" headline is retired. Only the `PSTRB` rule still matches; `PNSE` and `PBUSER` are
  each missed on their ANTECEDENT, and the scorer prints both missed keys.
- CAUSE, ATTRIBUTED FROM PRODUCER HISTORY RATHER THAN A DIFF. `git log -S resolve_indexed_signal_family --
  crates/specforge/src/ir/semantic.rs` returns exactly two commits: `WIRE-BASED-100.4` (`1c28516b`,
  `2026-06-06`) added it so an un-indexed prose reference (`PSEL`) resolved to the declared indexed family
  member (`PSELX`), and corrected the gold's antecedent to that canonical identity; then
  `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.ii` (`f88d463d`, `2026-08-12`, *make document identifiers opaque*) DELETED
  it and installed the opposite behaviour as a tested invariant —
  `temporal_condition_does_not_alias_an_undeclared_name_from_suffix_spelling` asserts that with `PSELX`
  declared, `"PSEL is asserted"` must yield NO predicate. The same commit removed identifier-spelling authority
  upstream, so the declared identity is now the document's own `PSELx` (visible as `sigcon_0011`'s antecedent
  moving `PSELX → PSELx` across the re-ingest). The gold's `PSELX` is therefore unreachable by two independent
  routes.
- THE RE-INGEST DID NOT CAUSE IT, AND THE PRESERVED BYTES PROVE IT. In the preserved pre-rebuild SemanticIR,
  `temporal_signal_constraint_sigcon_0009` (`PNSE`) already carries `antecedents: []` and `…_0014` (`PBUSER`)
  already carries only `PENABLE`+`PREADY`. Both defects were already persisted in the artifact the `1.000` was
  last associated with; the schema bump three days later made that artifact unscoreable, so the loss was
  INVISIBLE FOR FOUR WEEKS and the re-ingest is what made it visible. This is exactly the unowned defect `.9a`
  routed out — nothing fails when a persisted chain falls below the canonical schema — now demonstrated
  costing a real published number.
- OWNED, NOT LOGGED, AND THE GOLD WAS NOT TOUCHED. `WIRE-BASED-100.4a` now owns the choice between reinstating
  a SOURCE-GROUNDED (not spelling-inferred) binding of an un-indexed prose reference to its declared indexed
  identity, and ruling the two identities distinct and re-anchoring the gold. Editing the gold to recover the
  headline is the failure mode this tree's governing principle exists to forbid. `.5` reuses the same resolver
  for AHB `HSELx`, so `.9c` is expected to meet this again and will record it rather than absorb the fix.
- ONE MORE MOVEMENT, AND IT IS NOT A REGRESSION: the APB ISF adapter went `renderable` → `blocked`
  (`no source-grounded system clock/reset contract`; the emitted diagnostic reads
  `(clock __specforge_unresolved_clock)`), because `PCLK`/`PRESETn` were previously recognised as clock and
  reset from their spellings. That is the documented policy for the whole measurable stratum — every retained
  adapter manifest is honestly blocked with zero emitted `.isf` — so APB joins that set as the 25th.
- AND A SECOND FINDING, OWNED AS ITS OWN TREE: **the retained normalized-bundle population can neither grow
  nor shrink.** The rebuild restored APB's bundle, and `check_chain_currency.sh` fails closed on an undeclared
  bundle on disk — but declaring it turns `PRODUCTION-GENERICITY` and `RESIDUAL-ACTIONABILITY` red, because
  `scripts/validate_residual_actionability_contract.py` and `scripts/validate_canonical_recovery_contract.py`
  pin `len(retained_ids) != 24` as a literal and `scripts/check_behavioral_genericity_contract.py` joins the
  retained set by SET EQUALITY to a release-blocking behavioral qualification frozen at 24 rows, a 7/17
  calibration/holdout split and 51 declared held-out attempts. Recording a deliberate reclamation instead is
  refused by the same two validators (`reclamations != []`). ADR 0025 mandates BOTH operations — a refresh
  keeps its bundle, and reclamation stays deliberate and task-owned — so the first refresh to exercise it had
  no compliant move. `.9b` HELD the bundle at
  `generated/preserved/WIRE-BASED-100.9b/apb-normalized-bundle-held-out/` (repository volume, 25 MB,
  byte-identical, nothing deleted) and left the declaration at 24, so APB's EvidenceIR replay reads
  UNMEASURABLE while its SemanticIR/IntentIR/adapter replays stay measurable and current.
  `RETAINED-BUNDLE-POPULATION-FROZEN` owns the repair and restores the bundle in its `.3`.
- LOCKSTEP: the book's current 24/54 frontier statements move to 25/53 across the SourceIR, EvidenceIR,
  SemanticIR, IntentIR, ISF-adapter, architecture-rationale, extraction-architecture and extraction-eval
  chapters, with dated measurements left as dated rather than rewritten, and proof currency separated from
  bundle retention wherever the two were stated as one number.

### WIRE-BASED-100.9a — the corpus refresh frontier cannot own the wire re-ingest, and 5 of 7 gold documents cannot be scored

- `.8` closed by handing its remainder away in one sentence — *"Re-ingesting the legacy stratum stays with the
  corpus refresh frontier"* — and repeated the routing in its `Non-goal`. Re-derived rather than restated, that
  hand-off names an owner whose own contract excludes the work: the frontier's cohort rule is
  `excluded_source_prefixes: ["corpus/"]`, documented in `scripts/check_corpus_frontier_census.pl` as *"the
  tracked in-repo gold/eval corpus — copied into the repository, never part of the host-library refresh
  program"*. All 18 legacy `corpus/`-sourced documents, the three wire golds among them, sit outside its cohort
  by construction; its `5 remaining` would still read `5` after every wire gold had rotted.
- THE MEASURABLE SHARE IS 30.8%, AND NO GATE PUBLISHES IT. Censused read-only over the persisted corpus with
  the tracked reproducer `scripts/measure_corpus_canonical_currency.py`: **78 documents = 24 measurable + 54
  legacy**, stratified totally at every stage (SourceIR 54×1 / 24×3, EvidenceIR 54×2 / 24×3, SemanticIR
  54×1 / 24×2, IntentIR 54×1 / 24×2). "Measurable" is not a convention chosen here — it is the exact predicate
  `unmeasurable_disposition` (`commands/eval_extraction.rs`) applies before `eval-extraction` will score.
- TWO GREEN GATES PUBLISH COVERAGE AND NEITHER ANSWERS THE QUESTION. `check_chain_currency.sh` reads
  `24 replayed / 24 current / 0 stale` — true, and a statement about the REBUILDABLE stratum, which declares
  the other 54 UNMEASURABLE and does not count them, so it reads 100% while describing 31% of the corpus.
  `check_corpus_frontier.sh` reads `57 cohort = 52 refreshed + 5 remaining` — true, and a statement about the
  host-library re-ingest PROGRAM, so it reads 91% done.
- AND `refreshed` DOES NOT MEAN CURRENT: **31 of the 52 declared-refreshed documents are still legacy.** The
  census contract is internally honest (it defines `refreshed` as declared completed keys and binds membership
  to bundle retention, never to a schema), but the line printed on the terminal invites exactly the inference
  `.8` made. The sweep finished under a SourceIR schema that no longer carries canonical authority (schema 3
  landed `2026-08-12`, `bb5047c2`; a schema-3 SourceIR carries `proof_context`/`proof_ledger` keys a schema-1
  artifact does not have at all). Legacy ownership partitions **18 unowned + 31 refreshed + 5 remaining = 54**.
- THE CONSEQUENCE IS THE NUMBER THAT MATTERS. Deriving the scored set from the eval datasets' own `doc_key`
  fields: **7 documents carry an eval gold and 2 are measurable** — SWD/ADI and I2C. APB, AHB, AXI and RISC-V
  Debug have NO OPEN owning leaf; NVMe is owned only incidentally as a frontier `remaining` entry. Six of the
  eleven tracked eval datasets belong to the three wire golds, so every APB/AHB/AXI number this tree has
  published is currently un-re-derivable — the claim `CLAIM_VERIFICATION.md` refuses. Confirmed live: the
  binary prints the UNMEASURABLE disposition, withholds all 16 `seed_apb` gold items, and scores nothing.
  Stated as a bound rather than a demonstration: `24 measurable` is an ADMISSION count — only 2 of the 24
  carry a gold at all, and a schema-3 document failing for any OTHER reason aborts the run rather than being
  dispositioned, deliberately, so a real defect cannot hide inside a disposition.
- THE CAUSE IS AN INVALIDATED REFRESH, NOT NEGLECT — and finding it required auditing CLOSED leaves, not just
  the open frontier. `CORPUS-PATTERN-REUSE.3c` (`done`, `2026-06-09`) re-ingested APB/AHB/AXI/AXI-Stream with
  `DOCLING_DEVICE=cpu`, rebuilt their evidence and re-verified the wire scores at `1.000`. The persisted wire
  SourceIRs were last written `2026-08-09`; canonical schema 3 landed `2026-08-12` (`bb5047c2`) three days
  later and silently made that completed refresh legacy. **No gate reported it** — the frontier excludes these
  documents by cohort rule and chain-currency counts only the already-current stratum. The durable defect
  underneath is therefore that NOTHING FAILS when a persisted chain falls below the canonical schema; a check
  would have fired on `2026-08-12` rather than leaving this to a leaf that happened to need the scorer.
- OWNED, NOT LOGGED. `WIRE-BASED-100.9` now owns the wire re-ingest as `.9b` APB / `.9c` AHB / `.9d` AXI,
  smallest first, each with its own before/after evidence. The route is demonstrated rather than assumed: the
  three PDFs are git-tracked under `corpus/` by `.5d`, `specforge doctor` reports the repo-local Docling
  runtime ready, and the three `corpus/`-sourced documents that ARE measurable were produced through this same
  route after the schema bump. The golds are content-anchored (`.1`) so they survive re-segmentation, and
  scoring needs no model server (`--provider skip`).
- ROUTED OUT, DELIBERATELY NOT ABSORBED: RISC-V Debug is a `PDF-VARIANT-DIGESTION` register-class gold, not a
  wire spec; whether `refreshed` should keep meaning "sweep completed" is a corpus-program question about 31
  documents; and the missing canonical-currency gate is a `DOCTRINE-ENFORCEMENT`-class decision. All three are
  recorded in `.9`'s node so the next session finds them without this tree claiming them.
- CHALLENGED ON REVIEW BEFORE COMMIT, AND TWO CLAIMS DID NOT SURVIVE. The director asked whether the findings
  still held; re-derived rather than restated, (a) "no owning leaf at all" is FALSE as written — a closed leaf
  owned and performed exactly this work — and (b) "24 of 78 can be scored" is an admission bound, not a
  demonstration. Both corrected in place before publication. Every other conclusion re-derived unchanged: the
  cohort-rule exclusion, 24/54, 18/31/5, 31-of-52, 7-golds-2-measurable, and the live `seed_apb` refusal.
- TWO DOCTRINE LESSONS, both earned here. (1) **Read a gate's cohort rule before treating its ratio as coverage
  of anything** — the denominator a gate publishes is the population it was built for, not the one you are
  asking about; where a real population has no gate, derive it. (2) **The absence of an OPEN owner is not the
  absence of an owner** — search closed leaves before publishing a "nobody owns X" claim, because the corrected
  story (a completed refresh invalidated by a schema bump) points at a different and better fix than the wrong
  one (neglect).
- CAUTION RECORDED, NOT RESOLVED: a re-ingest destroys evidence the current binary cannot regenerate, and every
  wire number published before `2026-08-12` was measured on that evidence. `.9b` carries a
  preserve-before-rebuild acceptance item, and the publishing posture is `.8c`'s — a re-derived number replaces
  the published one, and a number that cannot be re-derived is withdrawn rather than carried.
- No Rust touched, so the wire/register golds and `kg-bench` are orthogonal by construction.

### WIRE-BASED-100.8f — re-challenge the audit: `.8d` survives an attempt to break it, and the trap is recorded

- `.8e` verified `.8d`'s figures against the production gate. It did not ask the harder question: does `.8d`'s
  CONCLUSION survive a BETTER detector? Tested adversarially, it does — and the test is worth keeping precisely
  because its raw number is a lie.
- THE ATTEMPT. Replace the production phase detector with a deliberately permissive proximity rule — a
  statement "states phase P" if `P` occurs within four tokens of `phase`/`phases` — and a nearest-preceding rule
  appears to get **5 of 11** right. Read as a rescue, that would say the detector was the problem and `.8d` gave
  up early.
- EVERY ONE OF THE FIVE IS A FALSE POSITIVE. `statement_1678` — *"A simple parity check is applied to all packet
  request and data transfer phases"* — is claimed for SEVEN fields; it names both phases and assigns neither.
  `statement_1798` is claimed for `Park` and is about a FAULT response. `statement_0813` is claimed for `DATAIN`
  from 702 statements away. Not one assigns a field to a phase.
- SO `.8d` IS CONFIRMED BY AN ATTEMPT TO BREAK IT, which is stronger than `.8e`'s confirmation: the conclusion
  no longer depends on the production detector being right, because a strictly more permissive detector recovers
  nothing real either. The control now ships in `scripts/measure_swd_frame_phase_scope.py` so the next attempt
  meets the false positives instead of the number.
- AND A PRECISION DEFECT IN `.8d`'s OWN WORDING, CORRECTED. It said the document "never states the phase in text
  at all" for `Start`/`Parity`/`Stop`/`Park`/`A`. The phase WORDS do occur near them; what never occurs is an
  ASSIGNMENT. That distinction is the entire point — a proximity rule sees the words and mints the wrong phase —
  so the loose wording would have taught the next reader exactly the wrong lesson. Corrected in the task node,
  the fact card and the book.
- OTHER AUDIT LEGS RE-CHECKED AND SOUND: all 11 gold statements fall strictly inside the section anchor
  attributed to them (`line_start <= line <= line_end`); every frame gold item carries exactly one fact, and the
  two statements carrying two items each are handled per item.
- RESIDUAL STATED HONESTLY: `56` remains a PORT-derived figure — the script ports `stated_phase_name` rather
  than calling it, and only the count-word list is machine-checked. It is not load-bearing: the
  production-derived fact is the manifest's `serial_frame.bit_range` and `serial_frame.composition` both at
  `produced: 0`. `11 of 11` is robust to the port — the loose probe and the faithful port agree.

### WIRE-BASED-100.8e — audit on challenge: every conclusion survives, three of `.8d`'s numbers do not

- THE DIRECTOR ASKED WHETHER THE FINDINGS HOLD. Re-deriving them rather than restating them: every CONCLUSION
  survives and three FIGURES do not, and the reason is the failure mode this lane spent four slices correcting
  in other people's work — a number read off a probe instead of derived, from a probe that was not a faithful
  port of the thing it measured.
- SURVIVES, RE-DERIVED. The restored oracle reproduces its scorecard exactly at HEAD (`serial_frame_field`
  0/11, `protocol_operation` 4/4, `protocol_state` 0/13, `interface_edge_timing` 1/1). `check_chain_currency.sh`
  is 24/24. The corpus-wide surface-selective loss holds. `89d8dee7`'s identity gate
  (`serial wire`/`packet request`/`shift-dr`/`swdio`/`swclk`) and the now-absent `SerialFramePhase` enum hold.
  Its blast radius holds EXACTLY: four SWD cards superseded, four book chapters updated, and `ROADMAP.md`, the
  extraction-eval chapter, both owning trees and `swd-derivation-scored-100` untouched. The schema census, the
  corpus-frontier population, and `picture_0038`/`picture_0039` carrying a caption as their only observation all
  hold.
- A POPULATION TRAP THAT DID NOT BITE, CHECKED BECAUSE THE POINTER SAYS TO. `.8c` cited `89d8dee7`'s "five
  operations and 40 structurally admitted states" as re-deriving today, measured over the 24 schema-3 documents,
  while the ledger sentence spoke of all 78. Re-measuring both frames: the 54 legacy artifacts carry 0 frame
  fields, 0 states and 0 operations, so the totals are identical and the citation is sound. Sound by luck of the
  migration rather than by construction — worth the check.
- THE THREE THAT FAILED. (a) "61 statements carry a stated phase name" is **56**: the probe dropped the gate's
  own `parse_count_word` rejection, so "two or three phases" counted as the phase name `three`, and it stripped
  non-alphabetic characters anywhere in a token instead of trimming only the ends as `trim_matches` does.
  (b) "wrong for at least 7 of 11" was **never computed** — it is wrong on **11 of 11, correct on 0**. An
  eyeballed floor published in the voice of a measurement; the evidence was always stronger than the claim.
  (c) "at most 4 of 11" section titles is not a valid ceiling: 4 land literally, **5** once `ACK responses` is
  counted.
- THE FIX IS A DERIVATION, NOT A REWORD. `scripts/measure_swd_frame_phase_scope.py` ports `stated_phase_name`
  and `parse_count_word` exactly, prints all three figures plus the per-field table, and CHECKS its ported
  count-word list against the Rust source so the two cannot drift silently. Demonstrated RED: adding a
  `"thirteen" => Some(13)` arm to a copy of `evidence.rs` makes it exit `parse_count_word drifted`.
- AND THE RULE GENERALISES TO ITS AUTHOR. `.8c` published that retiring a producer stales the surfaces carrying
  its NUMBER, not the ones describing its artifact. These three figures had spread to exactly six surfaces —
  this ledger, `MEMORY.md`, `LIVE_ACHIEVEMENT_STATUS.md`, the task tree, the fact card and the book — in under a
  day. The `.8d` record below is kept as written; this record withdraws its figures in place.
- `.8d`'s deferral is unchanged and better supported: scope binding does not work, the binding is in the figure,
  and `serial_frame_field` stays 0/11 deliberately.

### WIRE-BASED-100.8d — the proposed fix is disproven by its own measurement, and `.8` closes

- `.8d` proposed the obvious generic repair for SWD's retired frame fields: stop requiring the phase name and
  the bit range in the SAME statement, and bind a document-stated phase to the fields in its scope. Measured
  read-only against the 11 gold facts before writing any code, **that repair is wrong**, and the measurement is
  the deliverable.
- STATEMENT SCOPE FIRES EVERYWHERE AND IS RIGHT A THIRD OF THE TIME. The nearest preceding phase-stating
  statement resolves for all 11 fields, so the rule would always produce something — and for at least 7 it
  produces the WRONG phase. `APnDP`/`RnW` would inherit `transfer` against a gold of `request`;
  `Start`/`Parity`/`Stop` would inherit `transfer` from 28-44 statements back; `Park` would inherit `data`. `A`,
  `ACK` and `DATAIN` sit **278, 303 and 324** statements past the nearest one, which says `response`.
- SECTION SCOPE IS HONEST BUT THIN: at most 4 of 11. `Packet requests` gives `request` for `APnDP`/`RnW` and
  `Data transfers (WDATA and RDATA)` gives `data` for `WDATA`/`RDATA`, but `Start`/`Parity`/`Stop` sit under
  `B4.2 SWD protocol operation`, `Park` under `B4.2.5 Protocol error response`, `A` under `Attributes`, and
  `DATAIN` under `OK or FAULT response to a DPACC or APACC access`. No section title contains the word `phase`.
- BECAUSE THE DOCUMENT DRAWS IT RATHER THAN WRITING IT. For `Start`/`Parity`/`Stop`/`Park`/`A` the phase is
  never stated in text: the field-to-phase membership is in `Figure B4-1 SWD successful write operation` and
  `Figure B4-2`. Both are captured (`picture_0038`, `picture_0039`) with role `ambiguous` and a caption as their
  only observation, so the diagram content was never read. **That also explains the retired extractor's 11/11:
  it never read the frame either — it assigned the phase from the FIELD NAME (`wdata`/`rdata`/`datain`/`ack[`),
  which is SWD's field-to-phase table transcribed into production code.** A prose rule cannot replace a lookup
  that was never a reading.
- DEFERRED WITH ITS CONSEQUENCE STATED: SWD `serial_frame_field` stays 0/11 and nothing is minted, because a
  wrong phase is worse than a measured zero. Re-open trigger is figure-content extraction — the typed carrier
  (`VisualObservationKind::TimingDiagramExtraction`) and the assets already exist, so the gap is the pass, not
  the schema. Fact card `swd-frame-phase-binding-lives-in-the-figure`.
- `WIRE-BASED-100.8` CLOSES with every child resolved. Stated plainly, what it does not deliver: the APB, AHB
  and AXI wire golds it existed to protect are still unmeasurable because their EvidenceIR is legacy, so "the
  oracle is restored" means restored over the 24 rebuildable chains only.

### CHANGES-LEDGER-ROLLOVER.7 — roll the change ledger in the same transaction that filled it

- `WIRE-BASED-100.8b`'s own record took `CHANGES.md` to 1,633 lines = **90.7%** of its 1,800-line health target,
  the mandatory rollover signal, with bytes at 88.4%. Shortening the entry that crossed it would mean cutting a
  27-line record to under 14 — trimming evidence to dodge a declared milestone, which is exactly the Non-Goal
  the containment doctrine exists to enforce. So the rollover rides inside the same transaction, the established
  pattern for a blocking pair.
- SEAM-CLEAN, NOT MINIMAL. The committed root at `4926fa84` held 19 post-migration records; the plan keeps
  **two** — the `WIRE-BASED-100.8` pair this transaction continues — and seals 17. That is the deepest cut at
  which every multi-record story below the boundary seals whole: both `KG-ISF-COMPLETENESS.5.iv.a` records, both
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.4f` records, and the older `.4`/`.7` block. Root 94 → 77 committed records /
  1,606 → 1,123 lines / 223,007 → 177,679 bytes; with `.8b`'s record riding over the cut the live window is
  78 records / 1,150 lines (63.9%) / 180,186 bytes (70.7%), about nine records of headroom.
- AND A CONSTRAINT NOBODY HAD WRITTEN DOWN. The applied run failed once and rolled back cleanly to exact
  preflight bytes: `staged output identity drift for … manifest.jsonl`, preceded by `Wide character in print at
  scripts/check_rolling_ledger_protocol.pl line 942`. The manifest writer emits without a UTF-8 layer, so a
  **non-ASCII byte in a plan `reason`** — an em dash here — makes the staged manifest fail its own identity
  check. Every earlier plan's reason happens to be ASCII, so the constraint had never been observed. The
  fail-closed restore behaved exactly as designed; the undocumented constraint is the defect, now recorded in
  `CHANGES-LEDGER-ROLLOVER.7` and in the plan file. A plan `reason` must be ASCII.

### WIRE-BASED-100.8b — a document the oracle cannot read gets a disposition, not a zero (and the legacy stratum is not schema 1)

- BEFORE: `eval-extraction` on any dataset naming a legacy document printed its header and died —
  `build_predictions` propagated the load error through `?`, so the whole run produced no output even for its
  measurable documents. There was exactly one way to represent a refusal: abort.
- AFTER: the extractor returns `TaskOutcome::{Records, Unmeasurable}`. `unmeasurable_disposition` probes the
  artifact's OWN `schema_version` through the new `EvidenceIr::persisted_schema_version`; a below-current
  artifact becomes an UNMEASURABLE disposition carrying that version and its re-ingest route, its remaining
  tasks are not retried, and its gold items are withheld from every scorer. `seed_apb.json` now reports
  `persisted EvidenceIR is schema 2, below the current canonical schema 3 … 16 gold item(s) withheld from
  scoring` and exits 0. **Any other failure still aborts** — a hermetic control pins that, because a disposition
  must never become a place to absorb real defects.
- WHY WITHHOLDING MATTERS, PINNED AS A CONTROL: the same test asserts that WITHOUT the filter those labels score
  as false negatives. Rendering "cannot be measured" as `R=0.000` is the fabricated number this tree forbids.
- AND A CORRECTION THIS SLICE FOUND IN ITS OWN INHERITED WORDING. `.8` called the 54 legacy documents "schema
  1". Censusing every persisted artifact instead of assuming: they are SourceIR schema 1 / EvidenceIR schema 2 /
  SemanticIR schema 1 / IntentIR schema 1, against 3/3/2/2 for the 24 current ones — ZERO schema-1 EvidenceIRs
  exist. "Legacy schema 1" is true of their SourceIR and false of the EvidenceIR `eval-extraction` refuses. The
  shorthand had spread into MEMORY.md, LIVE_ACHIEVEMENT_STATUS.md, the book, the `.8a` fact card, two SWD cards,
  and a `.5j` correction written hours earlier in this same session; all corrected. A stratum whose legacy
  version differs per stage cannot be named by one number — which is why the disposition prints the artifact's
  own version rather than a constant.
- Rebuildable golds unchanged value for value (I2C 1.000 6/6; SWD constraint 1.000, relation 1.000; SWD
  derivation 4/4 and 1/1 with the retired 0/11 and 0/13). `kg-bench` 156/156; workspace tests green (specforge
  lib 472, +2 controls). Flow census moved only its three size counters, every authority and protection count
  unchanged.

### WIRE-BASED-100.8c — the SWD 29/29 is retired, and the miss was a PARTIAL retirement, not an unpropagated one

- RE-DERIVED, `--provider skip`, on the oracle `.8a` restored: `seed_swd_derivation.json` scores
  `protocol_operation` 1.000 (4/4) and `interface_edge_timing` 1.000 (1/1), but `serial_frame_field` 0.000 (0/11)
  and `protocol_state` 0.000 (0/13) — document-level **5/29** against a published `29/29 at 1.000`. Content
  anchoring is not the explanation: 28 of 29 gold statement ids already resolve and the gold sentences match
  current statements at ratio 1.00.
- NOT A REGRESSION, ESTABLISHED THREE WAYS RATHER THAN BY READING A DIFF. (1) `check_chain_currency.sh --check`
  reports 24 replayed / 24 current / 0 stale, so the zeros are the current producer's real output, not a stale
  artifact. (2) The loss partitions along a published boundary CORPUS-WIDE: a read-only census of all 24 schema-3
  EvidenceIRs finds 0 serial_frame_fields in every document and `machine_name` on 0 of 40 protocol_states, while
  protocol_operations total 5 — and `89d8dee7`'s own entry below says the comparison *"retires 22 fixed-phase
  frame and four named-operation records; the generic producer retains five operations and 40 structurally
  admitted states."* Five and forty re-derive exactly. (3) Per-revision producer evidence: at `89d8dee7^`
  `extract_serial_frame_fields` gated the whole document on `serial wire`/`packet request`/`shift-dr`/`swdio`/
  `swclk` and sorted fields with a fixed `SerialFramePhase {Request, Acknowledge, Data}` enum keyed on
  `wdata`/`rdata`/`datain`/`ack[`. That enum is gone from today's source.
- SO THE CAUSE IS STRONGER THAN "A GENERICITY TRADE": ADR 0006 forbids that class of production decision
  outright, so the 29/29 never measured generic capability — it measured a protocol recogniser. Retiring it was
  mandatory. The gold is untouched and stays faithful; only the claim that the path is finished is withdrawn.
- THE MISS WAS PARTIAL, AND ITS SHAPE IS THE REUSABLE LESSON. `89d8dee7` DID supersede four SWD fact cards and
  update four book chapters. It missed `swd-derivation-scored-100` — the one card whose title asserts the score —
  plus two more cards, `ROADMAP.md`, `docs/book/src/quality/extraction-eval.md` (the chapter that publishes the
  number), and both owning task trees. **It retired the artifact-authority surfaces and missed the
  score-assertion surfaces.** When a change retires a producer, hunt the surfaces publishing its NUMBER; they are
  rarely the ones describing its ARTIFACT.
- CORRECTED EVERYWHERE, WITH THE RETIRED NUMBER KEPT AS DATED HISTORY: roadmap, book (a new *"The 29/29 signoff
  is retired"* section carrying the current scorecard, plus the stale `swd_operation` task name fixed to
  `protocol_operation`), `WIRE-BASED-100.5j`, the `SWD-SERIAL-EXTRACTION` tree head, a new fact card
  `swd-serial-frame-score-retired-by-genericity`, `swd-derivation-scored-100` superseded, and two cards corrected.
- THE RESIDUAL IS LOCALISED AND OWNED (`.8d`): `extract_serial_frame_fields` admits a field only when ONE
  statement both states a phase name and parses a bit range. SWD states phases in 61 statements and writes
  `A[3:2]`/`WDATA[31:0]` in others, so the conjunction never holds. The admissible repair is scope binding — a
  phase named in a section binds the fields in its scope — which is document grammar, not protocol identity.
  Stated honestly: "0 frame fields corpus-wide" is NOT proof the grammar is dead, because the one document that
  would exercise its composition-list path (the CAN specification) is a legacy chain and unmeasurable.

### WIRE-BASED-100.8a — give a proof-carrying artifact a supported way to move, and restore the scoring oracle

- THE ORACLE WAS DOWN, AND IT WAS THE ARTIFACT'S ADDRESS, NOT ITS CONTENT. `eval-extraction` refused every
  document in the corpus; on the rebuildable stratum the diagnostic was `registered derivation
  'evidence.claim.schema_version.root' output or input topology is stale`. Isolated read-only: two copies of the
  I2C EvidenceIR, one keeping the `<base>/<document_key>` convention and one flat, each with ONLY
  `artifact_layout` rewritten and every other byte identical, are both refused by `specforge entity-type`, while
  the artifact in place verifies and runs. So the flat layout was never the issue — relocation as such was.
- MECHANISM. `proof_kernel` registers one `evidence.current-replay` derivation over
  `serde_json::to_vec(public_field_values())` and makes it the sole input of every `evidence.claim.<surface>.<key>`
  derivation; `public_field_values` inserts `artifact_layout`. Each claim premise's `inputs_sha256` therefore binds
  the artifact's storage path, and `validate_premise`'s `RegisteredDerivation` arm raises that exact diagnostic when
  the recomputed topology differs. `extract_on_copy` must relocate, precisely so the corpus is never mutated, so
  every eval task failed before scoring.
- THE FIX IS A SUPPORTED OPERATION, NOT A LOOSENED SEAL. `EvidenceIr::load_relocated_to_artifact_base_root`
  verifies the artifact where it is, moves it to `<base>/<document_key>/evidence_ir.json`, and re-derives its proof
  for the new location from the same verified SourceIR prefix and the same sealed proof context, mutation chain
  intact. It cannot launder authority: the artifact must already verify, and `write_to_disk` re-verifies the result
  against an independent rebuild. An unsealed `artifact_layout` rewrite stays refused, and a hermetic control pins
  both halves. The deeper fix — taking the replay over everything except the location — was rejected here because
  it changes the frozen 38-family/170-field producer graph and invalidates all 24 sealed chains at once.
- MEASURED, `--provider skip`, first re-derivation since `2026-08-09`. I2C `declared_signal` source-tolerant
  1.000 (tp=6 fp=0 fn=0) with complete-gold precision 6/6; SWD `signal_constraint` 1.000 (1/1) and
  `actor_signal_relation` source-tolerant 1.000 (1/1); SWD derivation gold `protocol_operation` 1.000 (4/4) and
  `interface_edge_timing` 1.000 (1/1). The corpus is provably unmutated (both exercised artifacts keep their
  pre-change mtimes) and `check_chain_currency.sh --check` stays 24 replayed / 24 current / 0 stale.
- AND THE FIRST THING THE WORKING ORACLE FOUND: the SWD derivation gold also scores `serial_frame_field` 0/11 and
  `protocol_state` 0/13 — document-level 5/29, against a published `29/29 at 1.000`. That is not a regression and
  not caused by this slice: it is the published effect of `SPEC-TO-INTENT-ALIGNMENT.6d.ii.c` (`89d8dee7`,
  `2026-08-12`), whose own entry below records that exact comparison *"retires 22 fixed-phase frame and four
  named-operation records"*. The trade was honest; leaving the retired score standing as current for 20 days was
  not, and nothing caught it because the oracle that would have was itself down. Owned as `WIRE-BASED-100.8c`.

### LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4a — lossless rolling-ledger protocol locked

- Measured the four oversized root ledgers by their actual whole-record grammars and pinned their
  exact source identities, live-window boundaries, archive routes, and load-bearing consumers in a
  bounded data-only registry.
- Added an unconditional checker that reconstructs every source byte-for-byte, validates the
  planned survivor independently on record/line/byte/width axes, and protects the status writer's
  managed validation markers. It also defines the migrated source-capsule manifest/retrieval path.
- Root-caused two containment-program records appended below the 2026-03-31 legacy tail. The protocol
  promotes those exact records into the future current window while retaining their original bytes
  and source order in the immutable capsule; this slice performs no move or silent reorder.

### LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5a — current mdBook truth locked to code

- Reverified the two ramp-up drifts against `isf_ir.rs`, the CLI, and command dispatch. The `.isf`
  adapter does lower initiator-perspective graph directions, and `extract-contracts` is a delivered
  prose-to-contract producer.
- Repaired the current actor-connectivity and constrained-extraction prose, time-labelled the older
  audit narrative, and linked each statement to its detailed canonical book chapter.
- Added `scripts/check_book_current_truth.sh` as an executed maintained-reference currency verifier;
  it rejects both stale formulations and requires the load-bearing code seams and canonical pointers.

### LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.3c — lifecycle and control-plane proofs closed

- Added a 48-case fixture suite for all seven governed Markdown lifecycles, membership/query,
  freshness/currency execution, routes, exact coverage, locality, every independent size dimension,
  frozen/archive/reference/debt controls, and Git-history ceiling/baseline authority.
- Closed the JSONL schema below the raw-record layer: schema versions, allowed fields, identifier and
  locator domains, array cardinality, and scalar byte limits now fail closed. README route data also
  has independent record-count, total-byte, and record-byte bounds.
- Fixtures live only under repository-local `generated/`, remove themselves, and run unconditionally
  in quiet mode through `LIVE-DOC-SIZE`. The direct suite passes 48/48.

### LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.3b — complete enforcement activated

- Adopted the SpecForge-owned neutral live-document containment doctrine and ADR 0007 without an
  external runtime dependency or copied donor thresholds.
- Added a self-bounded JSONL registry covering all 553 resulting-tree Markdown files exactly once
  across 24 lifecycle-owned surfaces, plus a bounded ceiling-increase authority registry.
- Added the non-mutating resulting-tree checker and shell adapter, composed the README route guard,
  and registered `LIVE-DOC-SIZE` as doctrine five. Generated freshness is executed; unproved
  currentness remains named debt; maintained mdBook scope carries an exact task-owned delta.
- The first complete pass found and repaired nested research coverage and the missing genericity ADR
  entry in the decision index. No historical document was deleted or renamed.

### LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.3a — complete surface contract

- Decomposed `.3` before implementation so the normative doctrine, registry, checker, ADR, and
  driver entry activate atomically rather than landing prose without enforcement.
- Classified all 550 parent-tracked Markdown files at entry into 13 exclusive path families and
  recorded the 132,211-line/12,253,499-byte census, largest files, and widest-line pressure. The
  required coverage-authority fact card makes the committed set 551 without adding a new family.
- Locked the local JSONL schema, seven lifecycle classes, independent size dimensions, exact debt
  baseline rule, maintained-reference change authority, exactly-once coverage rule, and `.3c`
  fail-closed matrix. The FSMGen submodule remains outside SpecForge's parent Git authority.

### LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.2 — bounded README and route closure

- Replaced the 602-line/170,891-byte duplicate-heavy README with a verified
  127-line/4,834-byte landing page while retaining purpose, scope, prerequisites, quick start,
  architecture, navigation, support/contribution, and license notice.
- Adopted the SpecForge-owned `README_POLICY.md`; derived independent ceilings of 150 lines and
  5,800 bytes from the retained survivor rather than copying FSMGen's values.
- Added the reader/author route registry and unconditional `README-POLICY` doctrine check. The
  route proof found and corrected stale root references to the knowledge-map architecture; the
  canonical location is now recorded in a Knowledge Map fact card.
- Verified independent fail-closed limit probes, all 21 unique README routes, 4/4 doctrines,
  mdBook, CLI help, strict runtime preflight, the `inspect README.md` first-use path, and diff hygiene.

### PDF-VARIANT-DIGESTION.10i — CODE: block-qualified recovery of the genuinely-different register class (the `.10h` residual)
`.10h` recovered the *collapsible* half of the `.10g` reused-mnemonic residual (identical/nested views of ONE register → one record) but left the GENUINELY-DIFFERENT half — disjoint field sets under one mnemonic (MEM-AP `CSW` 11 fields vs JTAG-AP `CSW` 7 fields) — fully dropped, because the flattened heading hierarchy carries no ancestor-block heading to qualify with. `.10i` recovers it. The decisive measurement (the `.10h` `#[ignore]` block probe, read-only over persisted `source_ir`) refined the `.10h` pessimism: the block name is still printed — as the parent SECTION TITLE, reached via the dotted-parent number — in the universal form `<dotted-num> <BLOCK> register descriptions` (`C2.6 MEM-AP register descriptions` → `MEM-AP`, `C3.5 JTAG-AP register descriptions` → `JTAG-AP`, `C1.4 AP Register Descriptions` → `AP`), while a parent with no block token (`D4.5 Register descriptions`) honestly yields none. `extract_section_header_registers` (`ir/evidence.rs`) now threads each container's dotted number (`SectionHeaderFieldContainer.dotted`, additive — `.10f` ignores it) and, when `.10h` containment returns `None`, BLOCK-QUALIFIES each occurrence via the new pure helpers `section_dotted_number` + `dotted_parent` + `derive_register_block_name` + a `by_number` (dotted-number → title) map: each block-named occurrence emits as `<NAME>@<BLOCK>`, a no-block occurrence stays residual, and ≥2 still-disjoint occurrences sharing one block re-run `.10h` containment (else residual — never conflated). Universal section grammar over the document's own block headings, no chip-name list (ADR 0006).
- **Recall (measured per-item):** the genuinely-different class lives in EXACTLY 1 doc — ARM-Debug `ihi0074` gains `CSW@MEM-AP`{11}, `CSW@JTAG-AP`{7}, `CLAIMSET@AP`/`@MEM-AP`/`@JTAG-AP`{2}; `section_header_register_corpus_sweep` ARM-Debug 15→**20** registers / 69→**93** fields; full `evidence` register_records ARM-Debug 40→**45**. The no-block `D4.5` CLAIMSET stays residual; CoreSight `ihi0029` 6/29, GIC 73, SMMU 88, ACC 2 byte-identical (CoreSight's lone reused `AUTHSTATUS` is `.10h`-collapsible → never reaches `.10i`).
- **`.isf`-safe by sanitization:** the 5 block-qualified registers reach the ARM-Debug `debugger.isf` as 5 distinct sanitized storage vars (`csw_mem_ap`/`csw_jtag_ap`/`claimset_ap`/`claimset_mem_ap`/`claimset_jtag_ap`), and the re-emit passes FSMGen `--strict --check --json` with **0 diagnostics / 0 errors** (45 storage vars, 0 blocking_reasons).
- **NO REGRESSION:** a `git stash` baseline-vs-change full-`evidence` diff over the 8 `.10h` golds (CCIX r1.0, NVMe; AXI, AHB; GIC, SMMU, ACC; DTI) + CoreSight `ihi0029` is **byte-identical**; only `ihi0074` changes — ADDing 5 records with ZERO removals (all 40 baseline records byte-identically preserved) → pure recall gain, no conflation. `kg-bench` **156/156**; full `run_ci.sh` GREEN (+3 hermetic tests — block-qualify disjoint / skip no-block occurrence / block-name grammar — fmt + clippy `-D warnings` + rustdoc + mdBook). Book `pipeline/evidenceir.md` `.10i`; KM `section-header-register-block-qualification` (the `.10h` card carries a superseding cross-link).

### PDF-VARIANT-DIGESTION.10h — CODE: block-ambiguous register-mnemonic recovery by field-set containment (the `.10g` residual)
`.10g` reads register fields written as section headings (`<NAME>, bits [hi:lo]`) into `register_records`, but DROPPED every register mnemonic reused across ≥2 register-routed containers (ARM-Debug `AUTHSTATUS`/`CSW`/`IDR`/`CLAIMSET`/`DEVARCH`, CoreSight `AUTHSTATUS`) — the dotted heading carries only the short name and the occurrences mix identical cross-references, nested views, and genuinely-different registers. `.10h` recovers the SAFE half. A probe-first measurement established the residual lives in EXACTLY 2 docs (ARM-Debug `ihi0074`, CoreSight `ihi0029`), that the PDF backend flattens every heading to L1 (no ancestor-block heading to qualify with — only the dotted-number hierarchy + the field lists survive), and that the field set cleanly separates the three sub-classes. The new pure helper `collapse_section_header_register_identity` (`ir/evidence.rs`) replaces the `.10g` drop-all gate: a reused mnemonic collapses to ONE record IFF every occurrence's field set (by uppercased field name) is a subset of one MAXIMAL occurrence (identical + nested views of a single register), keeping that fullest occurrence's REAL layout; disjoint / partially-overlapping sets (≥2 genuinely-different registers — MEM-AP `CSW` vs JTAG-AP `CSW`) have no common superset → honest residual (never conflated into a fabricated mega-register). Universal field-set containment, no chip-name list (ADR 0006).
- **Recall (measured per-item):** `section_header_register_corpus_sweep` ARM-Debug 12→**15** registers / 57→**69** fields (+`AUTHSTATUS`/`DEVARCH`/`IDR`), CoreSight 5→**6** / 24→**29** (+`AUTHSTATUS`); GIC 73 / SMMU 88 / ACC 2 byte-identical (no reused names). Full `evidence --dry-run` register_records: ARM-Debug 37→**40**, CoreSight 26→**27**; each collapsed record carries the maximal field set (`AUTHSTATUS` = HID,NSID,NSNID,SID,SNID) and `CSW`/`CLAIMSET` stay residual.
- **NO REGRESSION:** a `git stash` baseline-vs-change full-`evidence` diff over 8 docs (CCIX r1.0, NVMe register golds; AXI, AHB wire golds; GIC, SMMU, ACC section-header no-dup; DTI `.10f` message-field) is **byte-identical**; the only 2 changed docs ADD records with ZERO removals and every baseline record byte-identically preserved → pure recall gain, no conflation. `kg-bench` **156/156**; full `run_ci.sh` GREEN (lib 1718→**1721**: +3 hermetic tests — nested-view collapse / identical-cross-ref collapse / disjoint residual — + 1 `#[ignore]` corpus probe; fmt + clippy `-D warnings` + rustdoc + mdBook). Book `pipeline/evidenceir.md` `.10h`; KM `section-header-register-identity-collapse`.

### KG-ISF-COMPLETENESS.5.iii — CODE: per-member `_WIDTH` parameter-leak enum gate (AXI gold `.isf` enum surface now faithful)
The enum surface now drops width-PARAMETER members that leaked into encoding tables. `is_width_parameter_leak_member` (`ir/evidence.rs`) + a `continue`-skip in `synthesize_encoding_declarations_for_enum`'s member loop after the `.5.ii` spine gate: a member named `<X>_WIDTH` where `X` is the enum's own name OR a declared signal (the `known_signals` set, now threaded from the signal-match caller; the `None` caller covers the enum-self case) is a width parameter → skipped, so a polluted enum keeps its codes and a pure-parameter enum empties (no statements → no `SymbolDefinition` → honest residual). Document-grounded (ADR 0006, like `.5.i`), NOT a structure-word name list.
- **Effect (AXI gold `ihi0022_l`):** evidence rebuild drops EXACTLY the 7 leaks (`BRESP_WIDTH`/`RRESP_WIDTH`/`RCHUNKNUM_WIDTH`/`RCHUNKSTRB_WIDTH`/`AWSNOOP_WIDTH`/`ARSNOOP_WIDTH`/`AWCMO_WIDTH`; statements 6414→6407; non-Enum statement set byte-identical). The `manager.isf` now emits `(BRESP (OKAY 0) (EXOKAY 1) (SLVERR 2) (DECERR 3) (DEFER 4) (TRANSFAULT 5) (RESERVED 6) (UNSUPPORTED 7))` (8 clean codes, no value-`0` dup) + `(AWCMO (CLEAN_AND_INVALIDATE 0) (CLEAN_ONLY 1))`; the false `(RRESP (RRESP_WIDTH 0))` / `RCHUNKNUM` / `RCHUNKSTRB` / `AXSNOOP` `_WIDTH`-only enums are GONE (emptied → honest residual). FSMGen `--strict --check` **success / 0 diagnostics**.
- **FP-free (genericity):** corpus false-positive set EMPTY — no legit `FULL_WIDTH`/`HALF_WIDTH` value exists, and the declared-signal arm never catches one (`FULL`/`HALF` are not signals); a unit test pins `FULL_WIDTH` KEPT. The only members caught are the 7 AXI-gold leaks.
- **NO REGRESSION:** WIRE-BASED-100 = **1.000 before==after** (AXI before/after `eval-extraction --provider skip` byte-identical: seed_axi 4/4·6/6, seed_axi_temporal 3/3; APB/AHB/SWD/i2c evidence rebuilt with both binaries **byte-identical** → gate inert). Full wire eval holds 1.000 (SWD constraint 0/1 documented promotion-only). `kg-bench` 156/156. `run_ci.sh` GREEN (lib **1718**, +2 tests).
- The `SECSID_WIDTH`/`SID_WIDTH`/`SSID_WIDTH` self-named pseudo-enums (enum NAME ends `_WIDTH`, prefix not a declared signal) stay an honest residual. **`.5` enum-surface fidelity now has all three member gates** (`.5.i` name + `.5.ii` spine + `.5.iii` `_WIDTH`). Report `docs/research/generic-enum-conflation-measurement.md` §`.5.iii LANDED`; KM `[[generic-enum-conflation]]`.

### KG-ISF-COMPLETENESS.5.iii — MEASUREMENT: the `_WIDTH` parameter-leak is the one buildable deeper enum residual (GO)
Read-only census (docs-only, no code) of the five deeper enum member-quality classes `.5.ii` deferred — glossary `SEE…`, front-matter/ToC, section-caption `B2_3_1_…`, `_WIDTH` parameter leaks, value-restart-of-all-clean — over all 78 persisted IntentIR docs. Tracked deterministic reproducer `scripts/measure_enum_width_leak.py` (RAM-safe; no VLM/Docling/rebuild).
- **Most deeper classes are SUBSUMED by `.5.i`** — 47/54 `_WIDTH` members and the bulk of the 319 section-caption survivors live in generic-named enums (`TABLE`/`TRANSLATION`/`DEBUG`) that `.5.i` already drops whole → no `.isf` reach.
- **The `_WIDTH` leak DOES reach the AXI WIRE-GOLD `.isf` and is materially damaging.** 7 members in real-signal-named enums in `ihi0022_l` emit `(BRESP (BRESP_WIDTH 0) (OKAY 0) …)` (junk member duplicates value `0`), `(RRESP (RRESP_WIDTH 0))` (the real RRESP codes are entirely replaced), `(AXSNOOP (AWSNOOP_WIDTH 0) (ARSNOOP_WIDTH 1))` (pure junk), `(AWCMO (AWCMO_WIDTH 0) …)` (duplicates `0`). Root cause: a config/parameter row (`Enum BRESP BRESP_WIDTH = 0.`) leaked into the value enum — a width PARAMETER mis-read as an encoding VALUE. A false bar-#6 `.isf` fact, unscored by WIRE-BASED-100 (the enum surface is emitter-orthogonal — why it held 1.000 while the `.isf` carried junk).
- **Decision GO** on a per-member `_WIDTH` parameter-leak gate (drop `<X>_WIDTH` iff `X` is a declared signal OR the enum's own name) — document-grounded (ADR 0006, like `.5.i`), corpus false-positive set EMPTY (no legit `FULL_WIDTH`/`HALF_WIDTH` value exists, and the declared-signal arm never catches one since `FULL`/`HALF` are not signals). Per-member, not per-enum (keeps BRESP's codes; empties RRESP/AXSNOOP → honest residual). **NO-GO** on section-caption (leading `[A-Z]?digit` collides with real codes `D1`/`L2`), restart-of-clean (no fidelity defect), glossary/front-matter (tiny + name-ish) → honest residuals.
- No code → WIRE-BASED-100 / register+wire golds / `kg-bench` orthogonal by construction; `check_doctrines.sh` GREEN; `KNOWLEDGE_MAP.md` regen (126 facts / 932 keys). Report `docs/research/generic-enum-conflation-measurement.md` §`.5.iii measurement`; KM `[[generic-enum-conflation]]`. Frontier → the `.5.iii` CODE slice (the `_WIDTH` gate; before/after WIRE-BASED-100 eval required, byte-changing on the AXI gold).

### KG-ISF-COMPLETENESS.5.ii — CODE: per-member sentence-spine enum-fragment gate
The enum surface now carries a per-MEMBER member-quality gate the `.5.i` name-gate cannot reach. `is_prose_fragment_member_name` + `PROSE_SENTENCE_SPINE_WORDS` (`ir/evidence.rs`) gate the member loop in `synthesize_encoding_declarations_for_enum`: a synthesized member whose `_`-token set carries an English sentence-spine word (copula/auxiliary/modal/article/demonstrative/relativizer/subordinator; collisions `a`/`i`/`its`/`can`/`may`/`am` EXCLUDED per `.1a`) is a captured prose sentence → skipped, so a conflated enum keeps its genuine codes and a pure-prose table empties (no statements → no `SymbolDefinition` → honest residual). One seam covers both call paths. ADR 0006 (universal grammar, no name list).
- **Per-member, not per-enum** — a whole-enum drop would destroy real codes (AXI `BRESP` = the codes FUSED with prose); **not value-restart** — AHB `HPROT` restarts but all 15 members are clean identifiers.
- **Effect:** the AXI `manager.isf` now emits `(BRESP (BRESP_WIDTH 0) (OKAY 0) (EXOKAY 1) (SLVERR 2) (DECERR 3) (DEFER 4) (TRANSFAULT 5) (RESERVED 6) (UNSUPPORTED 7))` (codes recovered from the 16-member prose-fused enum); `PPROT`/`RRESP`/`ARCACHE`/`ARTAGOP`/`AWTAGOP`/`RLAST`/`WLAST` empty out; CCIX `TABLE`'s 27 clean-identifier members stay intact.
- **NO REGRESSION:** WIRE-BASED-100 = **1.000 before==after** (PROVEN — before/after `eval-extraction --provider skip` over all 10 seeds on gold evidence rebuilt with the preserved baseline vs gated binary; scored surface byte-identical: APB 6/6·5/5, AHB 6/6·6/6, AXI 3/3·6/6, temporal 3/3·4/4·3/3, SWD 1/1+0/1, SWD-derivation 11/11·4/4·13/13, i2c 6/6). Rebuilt AXI + APB `.isf` pass FSMGen `--strict --check --json` (`success`). `kg-bench` 156/156. `run_ci.sh` GREEN (lib **1716**, +4 tests).
- **Honest residuals deferred:** glossary `SEE…` refs, front-matter/ToC members, section-caption (`B2_3_1_…`) members, `_WIDTH` parameter leaks, value-restart conflations with all-clean members. **`.5` enum-surface fidelity is now built** (`.5.i` name-gate + `.5.ii` member-gate). Report §`.5.ii LANDED`; KM `[[generic-enum-conflation]]`.

### KG-ISF-COMPLETENESS.5.ii — MEASUREMENT/CALIBRATION: the member-quality gate is PER-MEMBER (design corrected, GO)
Read-only member-quality census over all 78 persisted IntentIR docs (561 enums / 12 509 members) to calibrate the residual junk-enum gate `.5.i`'s name-gate cannot reach. It **overturns the recorded `.5` plan** and locks a clean design:
- **Per-member, not per-enum.** The surviving junk enums are CONFLATIONS of a junk table and a clean table — AXI-gold `BRESP` fuses 7 prose-fragment members + `BRESP_WIDTH` WITH the 8 genuine codes `OKAY/EXOKAY/SLVERR/DECERR/DEFER/TRANSFAULT/RESERVED/UNSUPPORTED`. A whole-enum drop would destroy the codes; the fix drops the prose MEMBERS and keeps the codes (`BRESP` 16→9).
- **Value-restart is NOT a junk signal.** AHB-gold `HPROT` restarts (3 fused sub-encodings) yet every one of its 15 members is a clean identifier — a restart-gate is a false positive. Restart is dropped from the rule (sub-enum splitting of clean conflations deferred).
- **The load-bearing signal is per-member NAME shape — an English sentence-SPINE token.** The synthesis sanitizes a name-cell to `[A-Z0-9_]`, so a prose sentence becomes one `_`-joined member name; a real hardware symbol never contains a copula/auxiliary/modal (`IS`/`ARE`/`BE`/`HAS`/`MUST`/`SHALL`), article/demonstrative (`THE`/`THIS`/`THAT`), or relativizer/subordinator (`WHICH`/`WHEN`/`IF`/`BECAUSE`). Collisions EXCLUDED per the `.1a` discipline: `A`/`I`/`ITS`/`CAN`/`MAY`/`AM`. Universal grammar, ADR 0006 — no name list.
- **Precision/recall (per-item, `[[feedback_scoring_rigor]]`):** clean anchor **0/115 flagged → precision 1.000**; junk anchor **269/269 caught → recall 1.000**; corpus-wide 3 781/12 509 (30.2 %) members drop. Wire-gold blast radius: `BRESP` 16→9 (recovers codes), `PPROT`/`ARCACHE`/`AWCACHE`/`RLAST`/`WLAST`→0 (pure prose), `HSIZE`/`HTRANS`/`HRESP` untouched.
- **Honest residuals deferred:** glossary `SEE…` refs, front-matter/ToC members, section-caption (`B2_3_1_…`) members, `_WIDTH` leaks, value-restart-with-clean-members.
- **Decision: GO** — land a per-member sentence-spine fragment drop at `synthesize_encoding_declarations_for_enum` (`ir/evidence.rs`); byte-changing on wire golds (strict improvement) → the code slice needs a before/after WIRE-BASED-100 eval. No code → all oracles orthogonal; `check_doctrines.sh` GREEN. Report `docs/research/generic-enum-conflation-measurement.md` §`.5.ii measurement`; KM `[[generic-enum-conflation]]`.

### KG-ISF-COMPLETENESS.5.i — CODE: extraction-side enum fallback name-gate + emitter orphan-`(type)` fix
The `.isf` no longer emits generic junk-named or cross-table-conflated enums. Two edits, both ADR-0006-structural (no name list):
- **Extraction gate (`ir/evidence.rs`):** `derive_encoding_enum_name`'s fallback used to name an unmatched encoding table after the FIRST signal-shaped caption token, so `Table 7 — …` → a junk `(type TABLE …)`, and `build_symbol_definitions` (`semantic.rs`) merged every same-named table into ONE mega-enum (HBM2 `TABLE` = 7 tables / 57 members / 30 sentence-fragment names). The fallback now keeps the candidate **only when independently evidenced** — a declared signal OR a column-header reference token of the table — else returns `None` (existing `continue` contract → no enum minted → honest residual).
- **Emitter fix (`ir/isf_ir.rs`):** the `(types …)` block is gated by `emitted_enums()`, so a member-dropped enum (`isf_enum_is_emittable` / `.2a.iv`) leaves no orphan `(type …)` line.
- **Two corrections to the `.5` measurement (both cleaner):** the genuinely-named enums come from the signal-match loop ABOVE the fallback → **byte-identical**, and the structural gate is *strictly stronger* than a name-only gate (it also drops fallback-origin "real-named-but-junk" `COMMAND`/`AMBA`/`READ`/`CACHE`/`RELEASE`). It also cleans off-gold junk value-constraints derived from dropped-enum `discovered_values` (AXI `ACTIVATEACK A` → grounded `ACTIVATEACK 1`), score-orthogonal (distinct facts identical).
- **Measured:** corpus census (33 rebuildable docs) generic-named enums **82→8** / total enum records **422→105**; the 8 survivors are document-evidenced column-header tokens (CCIX "Table of Contents", gic_600 `DATA`) → `.5.ii` residual. Per-doc before→after on HBM2 + 4 wire golds + AXI-Stream + SWP + 6 worst offenders (amd 34→5, gic_600 23→6, coresight 19→5, cortex_a76 16→7, CHI 14→4, CHI-C2C 7→1): real signal enums byte-identical, every `.isf` FSMGen `--strict` 0 diagnostics.
- **NO REGRESSION:** WIRE-BASED-100 = **1.000 before==after** (proven via before/after `eval-extraction` on rebuilt gold evidence); nvme-registers + i2c golds identical; `kg-bench` 156/156; `run_ci.sh` GREEN (lib 1712, +4 tests). Genericity proven: `DATA` kept where a real gic_600 signal, dropped where a bare HBM2 caption word.
- Report `docs/research/generic-enum-conflation-measurement.md` §`.5.i LANDED`; KM `[[generic-enum-conflation]]` → LANDED. Frontier → `.5.ii` (member-quality gate, calibration-gated).

### KG-ISF-COMPLETENESS.5 — generic-`TABLE` mega-enum conflation MEASURED (read-only, docs-only)
PNT pivot off the now-low-value re-ingest tail to a surfaced upstream extraction-precision lever (`feedback_not_complete_attack_substantive_gaps`). The `.isf` emits a generic junk-named enum (HBM2 `(type TABLE (bits 6))`) fusing ~7 unrelated value-tables — a bar-#6 ISF-fidelity defect deferred by `.2a.iv` (Lever F).
- **Localized (a read-only agent + own verification): EXTRACTION-born, not the emitter.** `derive_encoding_enum_name`'s fallback (`evidence.rs:4457-4461`) names an unmatched encoding table after the first caption token passing `is_hardware_signal_token` (`evidence.rs:7106`, which accepts `Table`→`TABLE`); `build_symbol_definitions` (`semantic.rs:2782-2789`) merges every same-named table into ONE enum by name; the emitter (`isf_ir.rs:889-912`) lowers it faithfully.
- **Measured corpus-wide:** 56/78 docs carry a generic-named enum (96 generic `TABLE`/`FIGURE`/`DATA`/… vs 493 real); ~95 reach `.isf`. HBM2 `TABLE` = 57 members from ~7 conflated tables (29 dup values, 30 sentence-fragment member names). **Decisive insight:** a name-only gate misses **271 real-named-but-junk** enums (`COMMAND`/`DWORD_MISR`/`AMBA`) — the load-bearing signal is member quality, the generic name is the symptom; only 222 of 493 real enums are clean.
- **Decision: GO, extraction-side, decomposed** — `.5.i` (safe fallback name-gate: `derive_encoding_enum_name` returns `None` unless the token is a declared signal → kills the 96 generic + the conflation; + emitter orphan-`(type)` fix at `isf_ir.rs:403-409`), `.5.ii` (per-table member-quality gate, calibration-gated). Universal structural rule, ADR-0006.
- **WIRE-BASED-100:** scores ORTHOGONAL (enums unscored by `eval-extraction`); but the `.isf` BYTES change on all 4 wire golds (each emits a junk `TABLE`; AHB's fuses HTRANS+HSIZE which already have correct enums) — a strict improvement needing a deliberate snapshot refresh, so `.5.i` is deferred to a FRESH/focused slice for signoff quality (not rushed at the tail of a long measurement).
- No code → all oracles orthogonal by construction; `check_doctrines.sh` GREEN; knowledge map 125→126. Report `docs/research/generic-enum-conflation-measurement.md`; KM `[[generic-enum-conflation]]`.

### CORPUS-COVERAGE.2 re-ingest #32 — JEDEC HBM-gen1 (`jesd235`): bundle-restoration + corpus-provenance finding
PNT slice (re-ingest sweep, RAM-light deterministic cascade on the current binary; the strict frontier after `KG-ISF-COMPLETENESS` has no buildable leaf, so PNT advances to `CORPUS-COVERAGE.2`). RAM 83% free, no ollama model loaded; stale `source_ir.json` backed up before re-ingest (staged-swap).
- **Corpus-provenance finding:** the library `JESD235_2013-10_HBM_DRAM.pdf` is a **6-page legal-exhibit cover** (markdown reads `DOCKET`/`ALARM`/`JEDEC STANDARD … HBM DRAM … OCTOBER 2013`/`Netlist Inc.`/`Netlist Ex 2021` + 12 cover images, **0 tables**), NOT the full HBM gen1 standard — so its thin yield (0 registers/relations/constraints) is HONEST source-driven absence, not an extraction gap (contrast #28 HBM2 `jesd235a`, the real 172pp standard with registers/transactions).
- **Result:** all deterministic surfaces byte-near-identical to the retained-`source_ir` stale evidence (register/message-field/relations/constraints/conditional all 0 held; `extracted_statements` 69→68; intent `actors` 2→2 / `interfaces` 1→7 minor current-binary regroup / 0 txns). `adapt --target isf` recovers 16 signals on actor `channel` but **BLOCKS honestly** — `is_renderable: false`, sole reason "no behavioral content (temporal/conditional rules, signal constraints, or control blocks)", 1 residual; **no `.isf` fabricated** (the correct `CORPUS-COVERAGE.0` "intent-no-isf" class). normalized bundle RESTORED (6pp / 12 visual / 0 residuals; Docling CPU).
- After #32: **24 real chip-spec docs still normalized-missing.** The remaining tail is overwhelmingly thin/degraded (OpenCAPI×13 PHY/mech/TL + USB3.2/USB4 guides + CoreSight/debug guides) → the high-value substantive work now shifts to the surfaced upstream extraction-precision levers, not more bundle-restorations.
- No code change → WIRE-BASED-100 / `kg-bench` orthogonal by construction; `generated/` git-ignored (durable trace = the `.2` ledger row); `check_doctrines.sh` GREEN.

### BOOK-COMMAND-COVERAGE.3 — complete the Commands-overview "Current surface" list (21→28; mdBook drift reconciliation)
PNT slice (fresh-session mdBook drift survey, then RE-DERIVED by hand per the "re-derive a census with the build's own rules before trusting it" doctrine). The book is the owner's only window into the tool; the operational map at `docs/book/src/commands/overview.md` is meant to enumerate the full command surface, but its "Current surface:" list named only **21 of the 28** `cli.rs` subcommands.
- **Drift:** missing `nli-verify`, `entity-type`, `extract-conditions`, `extract-constraints-llm`, `eval-extraction`, `audit-extraction`, `grits-consensus` (the EXTRACTION-QUALITY-GAUGE / GRITS-cross-tool harness members). Distinct from `BOOK-COMMAND-COVERAGE.1`/`.2` (which guaranteed each command has a dedicated *section* — all 7 already had one in `commands/quality-and-learning.md` / `quality/extraction-eval.md`); only the overview *list* was stale.
- **Fix:** added the 7 to the list after `signal-resolve` (README order), with flag signatures cross-checked directly against the `cli.rs` clap arg structs (lines 268–365 / 539–554), not just `README.md`; added a navigation pointer to the detail chapters.
- **Self-check rigor:** an initial pointer sentence claiming "every one of them defaults to a CI-safe no-op" was **corrected** after re-deriving the actual provider defaults — `signal-resolve`/`nli-verify`/`entity-type`/`extract-conditions`/`extract-constraints-llm` default to live `ollama` (no-op only on `skip`), while `eval-extraction`/`audit-extraction` default to `skip` and `grits-consensus` is offline. The book now states the accurate behavior.
- **Verified:** `mdbook build docs/book` exit 0; the "Current surface" block now counts **28** `- ``-led bullets (= the 28 `cli.rs` commands); all 7 grep `PRESENT`. Both book command surfaces (overview list + per-command sections) are now locked to the CLI.
- Docs-only (no Rust change) → lib unchanged, oracles orthogonal; `check_doctrines.sh` GREEN.

### AUDIT-DOC-RECONCILE.3 — refresh `RUST_CODEBASE_ANALYSIS.md` size/command/module/test counts
PNT slice (the second discovered live-doc drift; owned by re-opening the doc-reconciliation tree `AUDIT-DOC-RECONCILE` with `.3`). The doc's `2026-06-08` "ramp-up currency correction" had drifted four counts as the extraction/quality/transaction/completeness/memory-bounded work landed.
- **Refreshed (each backed by a reproducible command, verified at HEAD):** whole `crates/specforge/src` **≈103,200 → 128,742 LoC / 65 `.rs` files**; command surface **25 → 28 subcommands**; IR namespace **25 → 28 `ir/*.rs` modules**; `cargo test -p specforge --lib` **1435 → 1709 passing** (0 failed, 4 ignored; ran `2026-06-24`, 6.86s). Added a largest-modules table (`evidence.rs` 26,195 · `semantic.rs` 22,604 · `validate.rs` 16,105 · …).
- **Pattern:** added a new dated `## Session update (2026-06-24 …)` section (the doc's established self-maintenance pattern); older dated sections left verbatim as historical record; no architecture claim reversed (still four IR stages + `.isf`-only adapter + R16 typed layer + Ollama/Qwen2.5VL production-default).
- **Self-check rigor:** an initial sub-claim naming "the 3 commands added since 2026-06-08" was **withdrawn** after noticing the old entry's own enumeration undercounted (it described `recover-register-bits` in prose but omitted it) — only the verified live total is stated, not an unverifiable delta.
- Docs-only (no Rust change) → lib unchanged (1709), oracles orthogonal; `check_doctrines.sh` GREEN.

### BOOK-COMMAND-COVERAGE.2 — dedicated `nli-verify` section in the Commands chapter (mdBook drift reconciliation)
PNT slice (lane switch off the re-ingest tail to a discovered mdBook gap — the user's #1 no-drift concern). The session-start mdBook survey found `nli-verify` was the **only** quality command without a dedicated section in `commands/quality-and-learning.md` (14 of 15 had one); its substantive treatment lived only in `architecture-rationale.md`, so a user browsing the Commands reference for it found nothing.
- **Added** a `## nli-verify` section (purpose, `nli-verify <evidence-ir> [--vlm-provider …] [--model …]` invocation, behavior, flags, and the honest boundary): the NLI entailment gate that judges whether each constraint's source sentence *entails* the constraint-as-a-claim (catching a condition read as an obligation), only-ever-strengthens + abstains when the model is unavailable (CI-safe), uses a *text* model, persists the `extraction_quality_gauge` (a vacuous all-abstained pass is never persisted), and how `intent --nli-verify` makes the gate *active* (demoting non-entailed contracts, surfaced by `validate` as `nli_demoted_contracts`).
- **Facts cross-checked** against `commands/nli_verify.rs` + `cli.rs`; `mdbook build docs/book` exit 0; the chapter's quality-command section count rose 14 → 15. The richer narrative in `architecture-rationale.md` is unchanged — the new entry is the Commands-reference pointer to it.
- Owned by re-opening `BOOK-COMMAND-COVERAGE` with a `.2` leaf (the bar was raised from "≥1 substantive treatment anywhere" to "every command has a dedicated Commands-chapter section"). Docs-only (no Rust change) → lib unchanged, oracles orthogonal; `check_doctrines.sh` GREEN.

### CORPUS-COVERAGE.2 — re-ingest #31: AMBA CHI C2C 2026 variant (`ihi0098_a_b`) — marquee message-field refresh (0→143)
PNT slice (binary current). `DOCLING_DEVICE=cpu ingest` (122pp / 96 visual / 0 residuals; normalized bundle RESTORED) → deterministic cascade.
- **Result:** **`message_field_records` 0 → 143 across 12 containers** — sibling of #29's 0→149, confirming the CHI-C2C family is a genuine marquee (the `.10` families fire on CHI packet/flit field tables absent in the stale pre-`.10` evidence). transactions 2→3 (recognition-only); the lone stale `actor_signal_relations` 1→0 (a fragment relation dropped by the current agent-identity gates — CHI is a coherency/message protocol, honest 0 per `KG-ISF-COMPLETENESS.3`); conditional_rules 25 held; 0 registers.
- **`.isf`:** `agent.isf` renderable (69 ports / 3 enums / 0 rules); **FSMGen `--strict --check --json` success / 0 diagnostics**. Same surfaced residuals as #29 (generic-`TABLE` mega-enum + signal-acronym noise — not fixed in-slice).
- After #31: 31 re-ingested, **25 real chip-spec docs still normalized-missing**. No code change → oracles orthogonal; `generated/` git-ignored; `check_doctrines.sh` GREEN.

### CORPUS-COVERAGE.2 — re-ingest #30: RISC-V AIA (`1_0_2025_03_12`) — bundle-restoration + confirmation
PNT slice (continuing the `.2` sweep; binary already current from #29, no rebuild). `DOCLING_DEVICE=cpu ingest` (89pp / 105 visual / 0 residuals; normalized bundle RESTORED) → deterministic cascade.
- **Result:** all deterministic surfaces byte-near-identical to the retained-`source_ir` evidence (statements 1193, `register_records` 0, `message_field_records` 0, `actor_signal_relations` 0, `conditional_rules` 39 — all held; intent 8 actors / 0 rel / 0 txns) — the #21/#22 "already current-binary-equivalent" class. Re-ingest value = **normalized-bundle restoration + current-binary confirmation**.
- **Honest absence + Lever-D:** 0 registers/relations (memory-mapped interrupt ISA — its APLIC/IMSIC register layouts sit in 12 `unknown`-classified structured tables that don't match the `.10b`/`.10c`/`.10g` families → re-confirms the #21 RISC-V-IOMMU structure-table recall opportunity, not a regression).
- **`.isf`:** `agent.isf` renderable (91 ports / 5 enums / 22 rules); **FSMGen `--strict --check --json` success / 0 diagnostics**.
- After #30: 30 re-ingested, **26 real chip-spec docs still normalized-missing**. No code change → WIRE-BASED-100 / `kg-bench` orthogonal; `generated/` git-ignored (durable trace = the `.2` log); `check_doctrines.sh` GREEN.

### CORPUS-COVERAGE.2 — re-ingest #29: AMBA CHI C2C (`ihi0098_a`) — marquee message-field refresh (0→149)
PNT slice (first eligible leaf of the next active tree after `KG-ISF-COMPLETENESS` ran out of buildable leaves). RAM-guarded current-binary re-ingest of a normalized-missing doc.
- **Pre-flight:** rebuilt the release binary to current (the persisted one predated `.2a.v`/`.2a.vi`), confirmed via the `isf_rule_value` symbol; RAM ample (~22% used).
- **Re-ingest:** `DOCLING_DEVICE=cpu specforge ingest` (108pp / 88 visual / 0 residuals; normalized bundle RESTORED — was missing) → deterministic `evidence → semantic → intent → adapt` cascade (no LLM).
- **Result (vs the STALE pre-`.10` evidence):** **`message_field_records` 0 → 149 across 13 containers** — the `.10` message-field families fire on CHI's packet/flit field tables that the stale evidence never carried (a real KG-completeness gain, not bundle-restoration). 0 register_records / 0 actor_signal_relations (CHI is a coherency/message protocol — its intent lives in message fields; honest absence per `KG-ISF-COMPLETENESS.3`); conditional_rules 17 + transactions 2 (recognition-only) held; 2239 statements.
- **`.isf`:** renderable (`agent.isf`, 69 ports / 3 enums / 1 rule / 0 txn bodies); **FSMGen `--strict --check --json` success / 0 diagnostics**.
- **Surfaced (extraction-precision, NOT emitter — honest residuals, deliberately not fixed in a re-ingest slice):** a generic `(type TABLE (bits 8))` mega-enum conflating many distinct field-value tables (duplicate values + sentence-fragment names — the #28 HBM2 generic-`TABLE` class), and a few interface ports that are prose-acronym noise (`AES`/`AMBA`/`ARM` — the `KG-ISF-COMPLETENESS.4` signal-inventory prose-noise class).
- After #29: 29 docs re-ingested; **27 real chip-spec docs still normalized-missing**. `generated/` is git-ignored, so the `.2` log table in `docs/tasks/CORPUS-COVERAGE.md` is the durable trace. WIRE-BASED-100 orthogonal (gold docs not re-ingested); `kg-bench` unaffected (no code change); `check_doctrines.sh` GREEN.

### KG-ISF-COMPLETENESS.3 — CLOSED: bar #2 relation-completeness resolved (verification-only, no code change)
Fresh-session PNT pick (the first eligible leaf of the first active tree, per the PNT selection rules). `.3` measured (`2026-06-17`) that whole docs carrying actors+constraints but ZERO `actor_signal_relations` split into two causes: (A) STALE IntentIR (recoverable by a deterministic cascade) and (B) HONEST ABSENCE (register/command/coherency protocols declaring ~0 wire signals). Its two frontier sub-steps are now both satisfied.
- **(i) corpus refresh — DONE.** A full re-census over all **78** persisted `intent_ir.json` vs their `evidence_ir.json` (`actor_signal_relations` array length, both stages) finds **0 stale docs** (zero with `evidence>0 & intent==0`). The `CORPUS-COVERAGE.2` re-ingest sweep rebuilt the once-stale docs through `converge` (which cascades the whole `SourceIR→…→IntentIR` chain), so the recovered relations have landed canonically: `tilelink_1_7_1` 33/33, `tilelink_1_8_0` 34/34, `um10204` I2C **17/17**, `gic_600` 101/101, `mmu_700` 25/25, ATS `ihi0082` 9/9, DTI 1/1, opencapi transaction-layer 15/15, USB4 13/13 (each `intent`==`evidence`); `wbspec` is now 0/0 (re-ingest reclassified it to honest-absence). 33 register/PHY/command docs at 0/0 are the correct (B) honest-absence class.
- **(ii) stage-staleness detector — DONE (already shipped as `CORPUS-COVERAGE.1`).** `commands/validate.rs::stage_staleness_relation_finding` emits `semantic_stale_relations_dropped` / `intent_stale_relations_dropped` (category `stage_staleness`) when a downstream artifact carries 0 relations while its upstream carries some, with 3 unit tests (fires on emptied-vs-nonempty-upstream; silent when downstream carries relations or upstream also empty).
- **Outcome:** bar #2 relation-completeness resolved for the recoverable class (stale set empty corpus-wide + a detector preventing silent recurrence); correctly N/A for register/message protocols (intent on register/message-field/transaction surfaces); wire protocols held at **WIRE-BASED-100 = 1.000**. No fabrication. Closure appended to `docs/research/relation-completeness-measurement.md`; KM cards `[[relation-completeness-staleness-vs-absence]]` + `[[stage-staleness-validate-detector]]`. No code change → all oracles orthogonal; `check_doctrines.sh` GREEN.

### KG-ISF-COMPLETENESS.2a.vi — CODE: ISF rule-drive-value validity gate → renderable corpus is now 70/70 FSMGen-strict-clean
Fresh-session PNT continuation (closing the last open ISF-emit strict-FAIL after Lever C).
- **Reproduce:** AXI+ACE `ihi0022_h_c` `manager.isf` fails `fsmgen --strict --check` with `Error: rule 'constraint_48' assignment actions require '(port expr)'`.
- **Root cause:** the rule's drive VALUE is free PROSE — `(RLOOP the value that was presented on the ARLOOP signal)` / `(BLOOP … AWLOOP …)`, a loopback obligation the extractor captured as a sentence. FSMGen requires a rule assignment-action RHS to be a renderable value expression (`(port expr)`); the emitter rendered the value verbatim (`ir/isf_ir.rs`) with no value-validity gate.
- **Fix:** new pass `drop_unrenderable_rule_values` (before the width/dedup passes) drops a rule whose any drive value fails `is_safe_isf_scalar_value` (non-empty, whitespace-free) and records an `isf_rule_value_<name>` residual. The prose value is unrecoverable (a temporal loopback, not the current `(port ARLOOP)`), so it is never fabricated into a `(port expr)`. ADR-0006 structural value-shape test, no name list.
- **Verified:** read-only scan of all 70 current-emit `.isf` → exactly 4 prose-valued drives, ALL in `ihi0022_h_c`; 0 elsewhere → byte-identical on every other doc by construction. AXI+ACE re-emit now FSMGen success / 0 diagnostics (4 `isf_rule_value_*` residuals). **Full FSMGen sweep over all 70 current emits → 70/70 strict-clean** (was 69/70) — the ISF-emit strict-FAIL frontier (Levers A/B/C/F + this gate) is fully CLOSED. `kg-bench` 156/156; `run_ci.sh` GREEN (lib **1709**, +1 test); WIRE-BASED-100 orthogonal by construction. KM `[[isf-unrenderable-rule-value-residual]]`; book `pipeline/isf-adapter.md`.

### KG-ISF-COMPLETENESS.2a.v — CODE: ISF unconditional-rule-overlap conflict residual (Lever C) → 6 docs FSMGen-strict FAIL→PASS (incl. all 3 wire golds)
Fresh-session PNT slice (the last open ISF strict-FAIL lever). Closed the `isf_conflicting_rule_writes` cross-guard conflict the same-guard dedup missed.
- **Reproduce:** the real `subs/fsmgen/bin/fsmgen --strict --check --json` on a fresh re-emit of the AMBA Low Power Interface (`ihi0068_d`) `controller.isf` returns `success:false`, `isf_conflicting_rule_writes` on `PREQ` (`rule_5` ←1 vs guarded `temporal_..._dyn_sigcon_0012` ←0; a twin on `PACCEPT` is masked behind it). A full corpus sweep showed the SAME conflict class also failing the AXI/AHB/AXI-Stream wire golds + LTI + NVMe — the resume-pointer "27/28 clean, 1 FAIL (LPI)" tally was STALE (the cached wire-gold `.isf` were byte-identical to fresh AND already failing).
- **Root cause:** `dedup_conflicting_rules` (`ir/isf_ir.rs`) keys on `(signal, condition)` — same-guard only — so an unconditional rule (`IsfRule.condition == ""`) and a guarded rule hash to different keys and the overlap is missed; FSMGen flags it because its `_condition_terms_prove_disjoint` (`subs/fsmgen/perl/FSM/Scheduler/ISF/LoweringIR.pm:10458`) can never prove an absent condition disjoint, so an unconditional rule's firing set ⊇ every guard.
- **Fix:** new post-pass `drop_unconditional_overlap_conflicts` (after the same-guard dedup, before priority emit): per signal with an unconditional driver value `V`, drop every other rule driving it to `≠ V` and record an `isf_unconditional_overlap_<name>` residual (the unconditional value wins, never a fabricated precedence). The `(priority …)` escape-hatch was empirically tested and rejected (the guarded minority rule conflicts with EVERY same-value unconditional rule → an ungrounded winner over each = fabrication). ADR-0006: structural ISF semantics, no name list.
- **Verified:** LPI `controller.isf` + `channel.isf` now FSMGen `--strict --check` success / 0 diagnostics; exactly `..._0011`/`..._0012` dropped + residualized (adapter `residual_decision_count` 4→6). Re-emit diff (binary WITH vs WITHOUT the change): exactly **7 `.isf` differ** (all current primary emits), every other emit byte-identical; FSMGen on the 7 → **6 FAIL→PASS** (AXI `ihi0022_l`, AHB `ihi0033_c`, AXI-Stream `ihi0051_b`, LPI, LTI `ihi0089_d`, NVMe), **0 PASS→FAIL**, AXI+ACE `ihi0022_h_c` stays FAIL on the orthogonal pre-existing `(port expr)` grammar. **Honest current-emit tally (after cleaning 37 stale cache-cruft `.isf` — `adapt` emits one primary actor per doc): 69 of 70 PASS** (lone FAIL = `ihi0022_h_c` `(port expr)`) → the ISF-emit strict-FAIL frontier is effectively closed (Levers A/B/C/F resolved). `kg-bench` 156/156; `run_ci.sh` GREEN (lib **1708**, +2 tests); WIRE-BASED-100 orthogonal by construction (emitter-only). KM `[[isf-unconditional-rule-overlap-conflict]]`; book `pipeline/isf-adapter.md`.

### KG-ISF-COMPLETENESS.2a.iv — CODE: ISF enum value-literal emit-gate (Lever F) → JEDEC HBM2 strict-clean; + Lever A drift corrected (DTI already clean)
Fresh-session PNT slice. A probe re-verified the `CORPUS-COVERAGE.2` "3 strict-FAIL" tally against the current binary + the real FSMGen, then landed the one open emitter lever it confirmed.
- **Measurement-first correction (a wrong hypothesis caught):** the initial root cause hypothesis was "count-derived enum width overflow" (`ir/isf_ir.rs:878` sets the backing `(type … (bits B))` width from member COUNT, so a value `>= 2^B` overflows). A coded width-fits gate was **DISPROVEN** by a before/after `.isf` diff (it wrongly dropped legit AXI `AWATOP`(49)/`AWSNOOP`/`ARSNOOP` + AHB `TABLE`(64)) and a value sweep against the real FSMGen (GIC-600 emits `69152` strict-clean). The TRUE rule: FSMGen's package-symbol parser rejects a **bare token of only `0`/`1` digits with length >= 4** (an un-qualified binary literal — `1000`/`1010`/`1111`/`10000` fail; `0`/`1`/`111`, any value with a 2-9 digit `999`/`1020`/`69152`, and qualified `4'b1000`/`16'd1000` all pass at any magnitude).
- **Root cause:** `emitted_enums()` (`ir/isf_ir.rs`) gated only on `is_safe_isf_scalar_value` (whitespace-free), so HBM2's `TABLE.REPAIR_LANE` binary codes mis-read as bare decimals (`1000`/`1111`) reached FSMGen and broke the whole `.isf`. The `TABLE` enum is itself a mis-extraction (a generic mega-enum conflating ~10 distinct doc tables, with duplicate values + sentence-fragment member names).
- **Code:** new `isf_enum_value_is_emittable_literal(value)` (FALSE only for the bare `[01]`-only len>=4 token) + free fn `isf_enum_is_emittable(e)` (non-empty + all members safe-scalar AND emittable-literal); `emitted_enums()` filters on it; new `enum_residuals()` accessor records an `isf_enum_value_literal_<name>` packet for an enum dropped *specifically* by this gate; `adapters.rs` extends `residual_decisions`. +2 unit tests (binary-token value excluded + residual; 999-boundary + `16'd1000` radix token still emit).
- **Verified (per item):** HBM2 `hbm.isf` now FSMGen `--strict --check --json` **success / 0 diagnostics** — only the malformed `TABLE` dropped (residual `isf_enum_value_literal_table`); `EXTEST_RX`(212)/`DWORD_MISR`(19)/`COMMAND`/`UPDATEWR` enums KEPT; HBM2 has 0 `TABLE.<member>` references so dropping strands nothing.
- **No regression:** a fresh re-emit + `diff` of ALL emitted `.isf` → the ONLY changed file is HBM2 `hbm.isf`; the 4 WIRE-BASED-100 golds + Avalon + CoreSight SoC-600 + GIC-600 (`69152`) + ARM-Debug (`3360`) emit byte-identical (the binary-token criterion never flags a legit decimal). `run_ci.sh` GREEN (lib **1706**, +2; clippy/fmt/rustdoc warning-deny + mdBook); `kg-bench` 156/156; WIRE-BASED-100 orthogonal (emitter-only). Doctrines GREEN.
- **Lever A drift CORRECTED:** the same probe found DTI (Lever A) is ALREADY strict-clean — `ATST` emits `1'd1`, FSMGen 0 diagnostics; the `CORPUS-COVERAGE.2` #8 strict-FAIL (`2'b1`) was a STALE log entry (the width-alignment landed in `ISF-VALUE-WIDTH-EMIT.2` and was never re-verified). `CORPUS-COVERAGE.2` tally corrected: **27 of 28 renderable docs strict-clean, 1 FAIL (LPI Lever C — cross-surface rule-conflict on `PREQ`)**.
- **Genericity (ADR 0006):** universal token grammar, no chip-name list; verified against the real FSMGen, not guessed. **LOCKSTEP:** README current-state bullet; book `pipeline/isf-adapter.md`; KM card `isf-enum-value-literal-emit-gate`; task leaf `.2a.iv` + enforced acceptance checklist; CORPUS-COVERAGE.2 drift correction. **The upstream mega-enum conflation stays an honest residual → a future extraction-precision lever.**

### KG-ISF-COMPLETENESS.1c.ii — MEASURED → deferred as a bounded residual (read-only): the bulk dense-prose phantom class has no clean within-doc structural gate; the fix is upstream
Read-only measurement (no code change). The bulk of the dense-prose explosion (eMMC: 109 multi-word actors) is single-`REL-INFERRED`, leading-noun relation subjects (`basic bus`, `actual sector`, `B write`).
- **Disproof of a structural gate:** the most promising candidate — a `.1b.ii`-style connectivity fold (rewrite a multi-word subject onto its last content token when that token is an independently-connected single-word agent) — mishandles the real cases on AXI+ACE `ihi0022_h_c`: `caching Manager`/`initiating Manager`→`Manager` is correct, but `Manager component`→`component` is WRONG (the agent is the modifier `Manager`, not the noun-phrase head; a fold-on-first rule instead breaks `caching Manager`→`caching`). The agent token's POSITION varies (modifier vs head), so no fixed structural position is safe.
- **Deeper reason:** within one document a real descriptive reference (`caching Manager`) and a phantom (`basic bus`) are structurally indistinguishable — same single-relation participation, same `SECTION-PHASE`/`PROSE-PARA` provenance markers, same noun-phrase shape; and eMMC carries no first-class `ProtocolActorRecord` agent-definition surface (its EvidenceIR actor surface is only `actor_signal_relations`) to ground them. A drop is forbidden by the genericity guardrail; a participation threshold by the completeness north star.
- **Outcome — bounded honest residual:** the genuine fix is **upstream relation-subject extraction precision on descriptive prose** (the owner-directed in-Rust shallow-parse direction, `NLP-SHALLOW-PARSE`), NOT a downstream actor-surface rule. The multi-word prose phantoms stay a residual; they never reach the emitted `.isf` (the adapter lowers the renderable initiator's signals/behaviours, not raw `actors[]`), so the bounded cost is IntentIR `actors[]` precision (bar #1) on dense-prose specs. **Lever E fully scoped: `.1c.i` (clean structural win) shipped; `.1c.ii` upstream-NLP-gated.** Report `docs/research/agent-identity-prose-class-measurement.md` §8. No code change → all oracles orthogonal; `check_doctrines.sh` GREEN.

### KG-ISF-COMPLETENESS.1c.i — CODE: dense-prose trailing preposition/auxiliary strip (the measurement-clean first gate of Lever E)
Same-session continuation of the `.1c` probe (fresh session, measurement in context). Extended the `.1b.i` trailing-fragment consolidation to a closed class of trailing prepositions + auxiliaries/modals, so dense descriptive-prose specs consolidate `host has`/`host to`/`cache in`-style relation subjects onto their leading agent instead of minting phantoms.
- **Code:** new const `NON_ACTOR_TRAILING_FUNCTION_WORDS` (prepositions + auxiliaries/modals, a deliberate SUBSET of `NON_ACTOR_LEADING_FUNCTION_WORDS`, EXCLUDING conjunctions — which stay `.1b.iii`'s coordinated-subject job) added as a third strip class in `consolidate_trailing_fragment` (`crates/specforge/src/ir/evidence.rs`), inside `normalize_relation_actor_name` BEFORE the `.1a` reject (same seam/ordering as `.1b.i`). `before`/`after`/`until` already live in the discourse set, not duplicated. +2 tests (a `.1c.i` consolidation test + the `trailing_function_words_are_known_leading_non_conjunctions` drift guard).
- **Verified (per item):** new-binary `evidence→semantic→intent` cascade on JEDEC eMMC → **actors 153→138** (the 4 aux/prep `host` variants `host has`/`host is`/`host to`/`host with` MERGE onto `host`; host variants 11→7 — the remaining 4 are trailing-*verb* residuals, honestly `.1c.ii`), relations 349→341, 29 phantom names removed; `host.isf` renders (62 signals) + FSMGen `--strict --check --json` **success / 0 diagnostics** (emitter-safe).
- **No regression:** **WIRE-BASED-100 = 1.000** on fresh-Pattern new-binary evidence (4 gold docs rebuilt into a temp evidence-root) — constraints APB/AHB/AXI 6/6·6/6·3/3, actor-relations APB/AHB/AXI/SWD 5/5·6/6·6/6·1/1, temporal 3/3·4/4·3/3 (SWD lone constraint 0/1 = the documented promotion-only, unchanged). `kg-bench` 156/156. `scripts/run_ci.sh` GREEN (lib **1704** passed, +2; clippy/fmt/rustdoc warning-deny + mdBook). The wire golds carry no `X <aux/prep>` actor → their relation surface is byte-identical.
- **Genericity (ADR 0006):** closed-class universal grammar (subset of the leading lexicon), conjunctions excluded; corpus-wide safety MEASURED — 0 ≥8-port actors are `X <aux/prep>` across all 78 docs, so no real high-participation agent is renamed.
- **LOCKSTEP:** README current-state bullet (`.1c.i`); book `pipeline/evidenceir.md` caveat updated (strip LANDED); KM card `agent-trailing-function-word-consolidation`; task leaf `.1c.i` with enforced acceptance checklist. Frontier → `.1c.ii` (single-relation noun-phrase precision, deferred).

### KG-ISF-COMPLETENESS.1c — agent-identity precision for the DENSE-PROSE doc class (Lever E): OWNED + measurement-first PROBE done (read-only)
Fresh-session PNT pivot off `CORPUS-COVERAGE.2`: two consecutive re-ingests (#27 eMMC, #28 HBM2) each surfaced a NEW substantive lever, so per the owner's "attack substantive gaps, not easy incremental" directive the high-value move is ACTING on a surfaced lever, not grinding more re-ingests. Read-only measurement over the 78 persisted IntentIR docs (no code change). Owns Lever E (spun out of #27 eMMC's actor explosion to 153/349 vs #28 HBM2's consolidation 52→38).
- **It is a relation-subject extraction-precision problem, NOT an actors[] prose-mint problem:** of eMMC's 153 actors, **148 are CONNECTED** (each minted from a single `REL-INFERRED` relation subject at the `.1a`/`.1b` seam — leading token is a noun so `.1a` passes, carries a relation so `.1b.iv` can't touch); only **5 are pure-unconnected** (the grounded-keep generic role terms `.1b.iv` correctly preserves — 0 droppable, so `.1b.iv` already does the right thing here).
- **A name-SHAPE-only drop is DISPROVEN unsafe:** AMBA's REAL agents (`agent`/`controller`/`decoder`/`device`, `address decoder`, `Exclusive Access Monitor`) occupy the SAME structural shape classes as eMMC's phantoms (`adapter`, `basic bus`, `actual sector`) → a shape drop would destroy real AMBA agents and fail WIRE-BASED-100. The fix must be grammatical NORMALIZATION (rewrite, not drop) or participation/grounding, never shape (`feedback_avoid_denylists_prefer_structural`).
- **The doc class is DENSE-PROSE, not "non-AMBA":** the dense AXI+ACE `ihi0022_h_c` (189 actors) and CHI `ihi0050_g` (87) explode too; the terse WIRE-BASED-100 AXI gold `ihi0022_l` (21) is clean.
- **Scoped sub-leaves:** `.1c.i` — extend the proven `.1b.i` trailing-strip to a closed class of trailing **prepositions + auxiliaries** (`advantage of`→`advantage`, `host has`→`host`, `cache in`→`cache`), the clean landable first gate; **corpus-wide safety already MEASURED CLEAN** — across all 78 docs ZERO ≥8-port actors are `X <aux/prep>` shaped, so the strip never renames a real high-participation agent (the `.1a`/`.1b.i` bar) and the 4 wire golds carry no such actor; reach 138 names / ~17 docs. `.1c.ii` (deferred-with-trigger) — single-relation noun-phrase phantom precision (the bulk ≈120 of eMMC's 153), needs a participation+grounding discriminator and its OWN measurement first.
- **Artifacts:** report `docs/research/agent-identity-prose-class-measurement.md`; KM card `agent-identity-prose-class-measurement`; task leaf `KG-ISF-COMPLETENESS.1c`; book honesty caveat added to `pipeline/evidenceir.md` (the "widen without minting noise" overclaim, corrected for the dense-prose class).
- **Gates:** probe = no code change → WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal by construction; binary current; `scripts/check_doctrines.sh` GREEN; `mdbook build` GREEN. Frontier → `.1c.i`.

### CORPUS-COVERAGE.2 — re-ingest #28 JEDEC HBM2 DRAM (MIXED refresh: confirms phantom explosion is prose-specific (actors 52→38), transactions 0→3, BUT strict-FAILS on enum-value literal → new lever F; 28/57)
Fresh-session PNT slice, register/TRM phase, **deliberate diagnostic pick (DRAM register/timing spec — structured contrast to #27's prose)**. Re-ingested JEDEC HBM2 DRAM (`jesd235a_2015_11_hbm2_dram`, 172pp / 224 visual): Docling CPU (0 residuals; RAM 81% free; ~2 min) → deterministic cascade.
- **CONFIRMS the #27 phantom explosion is PROSE-SPECIFIC:** HBM2 actors **52→38 CONSOLIDATED DOWN** (the `.1a`/`.1b` gates fold fragments like the structured AMBA/CoreSight class — OPPOSITE of eMMC's 20→153), relations 89→55, interfaces 72→31. Lever E is scoped to descriptive-prose docs. **GENUINE win:** transactions 0→3 [read/write/trr_mode_operation], register_records 17 held, 0 message-fields (honest). `validate` no stage-staleness, 55/100 ADEQUATE.
- **BUT `.isf` strict-FAILS → NEW spun-out lever F (ISF-emitter, kin to Lever A):** the emitter built a generic `(type TABLE (bits 6))` mega-enum conflating ~10 distinct doc tables (REPAIR_LANE codes + microbump pitches + test ops + IDD currents + mode-reg refs) with restarting/duplicate values, and emits REPAIR_LANE BINARY codes (`0,1,10,…,1000,1111`) as bare decimal-looking tokens → FSMGen rejects `TABLE.REPAIR_LANE_8`=`1000` (package symbol values must be width/radix-qualified scalar literals like `4'b1000`); the bare token is also semantically wrong (binary 1000 ≠ decimal 1000). Lever F = ISF enum-member value literal format + binary-radix preservation + a generic-`TABLE` mega-enum extraction-precision concern.
- **Phase finding:** two consecutive re-ingests (#27/#28) each surfaced a NEW substantive lever (E precision, F enum-literal) rather than a clean refresh — the re-ingest tail is now reliably a LEVER-SURFACING exercise; the next high-value work is ACTING on the levers, not grinding more re-ingests.
- **Gates:** no code change → WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal by construction; binary current; `scripts/check_doctrines.sh` GREEN; no book change. 25/28 re-ingested docs strict-clean (3 FAIL: DTI/A, LPI/C, HBM2/F). Coverage: 28 of 57.

### CORPUS-COVERAGE.2 — re-ingest #27 JEDEC eMMC 5.0 (MIXED refresh: transactions 0→6 BUT phantom-actor explosion 20→153 on descriptive prose → new lever E; strict-clean .isf, 27/57)
Fresh-session PNT slice, register/TRM phase, **first descriptive-prose register/protocol spec re-ingested**. Re-ingested JEDEC eMMC 5.0 (`jesd84_b50_2013_09_emmc_5_0`, 296pp / 344 visual): Docling CPU (0 residuals; RAM 80–82% free, `.4a` guard armed, Ollama idle, ~5 min) → deterministic cascade.
- **GENUINE wins:** **transactions 0→6** [`boot`/`alternative_boot`/`device_lock_unlock`/`dual_data_rate_mode`/`background`/`h_w_reset`_operation — 5 with grounded signal set (9 members), all real eMMC ops], register_records 17 held, interfaces 9→38, signal_constraints 0→2, 0 message-fields (honest), normalized bundle restored. `.isf` renderable (`host.isf`, 62 signals / 17 storage / 28 enums / 16 rules); real `fsmgen --strict --check --json` **success / 0 diagnostics** (strict-clean). `validate` no stage-staleness, 51/100 ADEQUATE.
- **REGRESSION surfaced (honest):** **actors 20→153 / relations 23→349** — fresh Docling (7017 statements vs 6544) mints ~90+ phantom sentence-fragment actors (`host has`/`host to`/`cache in`/`B write`/`device behaves`/`following`) that the AMBA-tuned `.1a`/`.1b` agent-identity gates don't catch on descriptive prose. Phantoms never reach `.isf` (emitter lowers only the renderable actor) → a **KG-fidelity gap (bar #1), not a strict-FAIL**. → **NEW spun-out lever E: agent-identity precision for descriptive-prose / non-AMBA specs** (`KG-ISF-COMPLETENESS` family; probe-first, structural gates not denylists), HIGH-VALUE for the prose tail (USB/HBM/guides).
- **Phase finding:** descriptive-prose docs behave OPPOSITE to structured AMBA/CoreSight docs — fresh re-ingest *worsens* the actor surface rather than refining it; genuine refresh value (transactions/registers/bundle) is mixed with phantom-actor noise.
- **Gates:** no code change → WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal by construction; binary current; `scripts/check_doctrines.sh` GREEN; `mdbook build` GREEN (no book number changed). 25/27 re-ingested docs strict-clean. Coverage: 27 of 57.

### CORPUS-COVERAGE.2 — re-ingest #26 Intel VT-d 5.0 (genuine refresh — corrects the #22 "already current" survey; +15 message-fields +2 transactions; strict-clean .isf, 26/57)
Fresh-session PNT slice, register/TRM phase, **first distinct-vendor (Intel) re-ingest of the tail**. Re-ingested Intel VT-d 5.0 (`5_0_2024_08_intel_virtualization_technology_for_directed_io_specification`, 354pp / 810 visual) with the current binary: Docling CPU (0 residuals; confidence high; RAM steady 80–83% free, `.4a` guard armed, Ollama idle, ~4 min) → deterministic cascade.
- **GENUINE refresh — CORRECTS the #22 "already current-binary-equivalent" survey expectation:** the one-pass register-count survey flagged VT-d's 103 registers as current, but its stale evidence predated BOTH the message-field family AND the section-heading transaction recognizer → register_records 103 held (exact), **`message_field_records` (key absent)→15 / 4 containers** (NEW — the `.10c` structure-field family fires on VT-d's `Root-Entry Format`/context-table structures), **transactions 0→2** [`device_tlb_operation`, `set_root_table_pointer_operation`, both recognition-only — honest], actors 14→13 (consolidation), signal_constraints 1→4, temporal_rules 1→4; 0 relations (memory-mapped register spec — honest absence, `KG-ISF-COMPLETENESS.3`).
- **`.isf` renderable** (`agent.isf`, 510 signals / 103 storage(reset) / 4 enums / 44 rules / 0 txns); real `fsmgen --strict --check --json` **success / 0 diagnostics** (strict-clean). `validate` no stage-staleness (0-vs-0 honest absence), quality 8/100 INCOMPLETE (honest for a memory-mapped register/structure spec — the #21 RISC-V-IOMMU class). Honest residuals: register bit-fields + field-resets UNLOCATED (6 adapter `residual_decisions`); 7/17 unexplained intent-bearing tables (Lever-D structure-table recall opportunity).
- **Book sync:** `document-categories.md` in-memory-structure total measured 1,220/11→**1,235/12 docs** — VT-d's 15 fields actualize the book's existing `.10c` "VT-d gains 15 typed structure fields" claim (reconciles a latent inconsistency vs `evidenceir.md`).
- **Gates:** no code change → WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal by construction; binary current; `scripts/check_doctrines.sh` GREEN. 24/26 re-ingested docs strict-clean. **Phase correction: the one-pass register survey under-counts refresh value** — a register-current doc can still gain message-fields + transactions. Coverage: 26 of 57.

### CORPUS-COVERAGE.2 — re-ingest #25 CoreSight SoC-600 0100 TRM (completes SoC-600 cluster; consolidation+transaction refresh; strict-clean .isf, 25/57)
Fresh-session PNT slice, register/TRM phase. Re-ingested the CoreSight SoC-600 0100 TRM (`100806_0100`, 702pp / 1157 visual) with the current binary: Docling CPU (0 residuals; RAM steady 67–77% free, `.4a` guard armed, Ollama idle, ~5 min) → deterministic cascade. **Completes the CoreSight SoC-600 corpus cluster** (0701=#23, 0200=#24, 0100=#25).
- **GENUINE refresh (same #23/#24/#17 class):** register_records 597 held (exact), **actor_signal_relations 41→31 / actors 44→34 / interfaces 12→5** (consolidation), **transactions 0→1** (recognizer fires); conditional_rules 46 held; 0 message-fields (honest).
- **`.isf` renderable** (`dp.isf`, 597 storage(reset) / 12 enums / 3 rules / 4 signals); real `fsmgen --strict --check --json` **success / 0 diagnostics** (strict-clean). `validate` no stage-staleness (relations 31-vs-31), quality 52/100 ADEQUATE. Honest residuals: register bit-fields + field-resets not lowered (UNLOCATED); 1 `isf_temporal_unrepresentable` (honest, not emitted).
- **Gates:** no code change → WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal by construction; binary current; `scripts/check_doctrines.sh` GREEN. 23/25 re-ingested docs strict-clean. Coverage: 25 of 57.

### CORPUS-COVERAGE.2 — re-ingest #24 CoreSight SoC-600 0200 TRM (genuine consolidation+transaction refresh; strict-clean .isf, 24/57)
Fresh-session PNT slice, register/TRM phase. Re-ingested the CoreSight SoC-600 0200 TRM (`100806_0200`, 761pp) with the current binary: Docling CPU (761 page artifacts / 1302 visual / 0 residuals; RAM steady 74–79% free, `.4a` guard armed, Ollama idle, ~5 min) → deterministic cascade.
- **GENUINE refresh (same #23/#17 class):** stale evidence predated the `.1a`/`.1b` consolidation gates + transaction recognizer → register_records 631 held (exact), **actor_signal_relations 47→35 / actors 46→37 / interfaces 13→6** (consolidation), **transactions 0→1** (recognizer fires); conditional_rules 51 held; 0 message-fields (honest).
- **`.isf` renderable** (`dp.isf`, 631 storage(reset) / 9 enums / 3 rules / 4 signals); real `fsmgen --strict --check --json` **success / 0 diagnostics** (strict-clean). `validate` no stage-staleness (relations 35-vs-35), quality 52/100 ADEQUATE. Honest residuals: register bit-fields + field-resets not lowered (UNLOCATED); 1 `isf_temporal_unrepresentable` (honest, not emitted).
- Completes the CoreSight SoC-600 corpus cluster's 0200 version (0701 = #23; 0100 pending).
- **Gates:** no code change → WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal by construction; binary current; `scripts/check_doctrines.sh` GREEN. 22/24 re-ingested docs strict-clean. Coverage: 24 of 57.

### CORPUS-COVERAGE.2 — re-ingest #23 CoreSight SoC-600 0701 TRM (genuine consolidation+transaction refresh; strict-clean .isf, 23/57)
Fresh-session PNT slice, register/TRM phase. Re-ingested the CoreSight SoC-600 0701 TRM (`100806_0701`, 842pp) with the current binary: Docling CPU (842 page artifacts / 1935 visual / 0 residuals; RAM steady 71–80% free, `.4a` guard armed, Ollama idle, ~9 min) → deterministic cascade `evidence`→`semantic`→`intent`→`adapt --target isf` (evidence 17445 statements).
- **GENUINE refresh (NOT byte-identical like #22):** the stale evidence predated the `.1a`/`.1b`/`.1b.iv` agent-identity consolidation gates AND the section-heading transaction recognizer → register_records 833→828 (fresh-Docling table-boundary variance, minor), **actor_signal_relations 64→41 / actors 60→47 / interfaces 20→8** (consolidation folds the fragment/phantom actors the older evidence carried — cleaner KG, the #17 Avalon class), **transactions 0→2** (NEW typed surface), conditional_rules 74 held, 0 message-fields (no packet/structure tables — honest).
- **`.isf` renderable** (`dp.isf`, 828 storage(reset) / 19 enums / 4 rules / 5 signals); real `fsmgen --strict --check --json` **success / 0 diagnostics** (strict-clean — the 3 `isf_rule_conflict` on `ATB`, the ISF-RULE-CONFLICT-RESIDUAL family, correctly RESIDUALIZE and are NOT emitted, so unlike LPI #7 the file stays strict-valid). `validate` no stage-staleness (semantic+intent relations both 41 → non-zero, silent), quality 53/100 ADEQUATE.
- **Honest residuals:** 791 bit-fields + 164 field-resets not lowered (UNLOCATED, located-fields-only rule); the 3 `ATB` rule-conflicts.
- **Phase refinement:** "already current-binary-equivalent" is not uniform — #22 Cortex-A76 was fully current (byte-identical), but #23 got a genuine consolidation+transaction refresh because its stale evidence predated those gates; so the remaining tail is a MIX of pure confirmation (#21/#22) and consolidation/recognition refreshes (#17/#23) + bundle restoration.
- **Gates:** no code change → WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal by construction; binary current (no rebuild); `scripts/check_doctrines.sh` GREEN. 21/23 re-ingested docs strict-clean. Coverage: 23 of 57. Durable trace: `docs/tasks/CORPUS-COVERAGE.md` `.2` log.

### CORPUS-COVERAGE.2 — re-ingest #22 Cortex-A76 TRM (already current-binary-equivalent; bundle restored, strict-clean .isf)
Fresh-session PNT slice, register/CPU-core-TRM phase. Re-ingested the Cortex-A76 TRM (`100798_0401`, 620pp) from the git-ignored `.cache/local-references/chipdoc` symlink with the current binary: Docling CPU (620 page artifacts / 476 visual / automation_confidence high / 0 residuals; RAM steady 73–83% free, `.4a` guard armed, Ollama idle) → deterministic cascade `evidence`→`semantic`→`intent`→`adapt --target isf` (evidence 9534 statements / 185 links).
- **Honest finding — already current-binary-equivalent (NOT pre-`.10` stale, the #21 class):** register_records 42 held / conditional_rules 26 held / 0 message-fields / 0 relations / 0 interfaces / 0 transactions; IntentIR unchanged (9 actors / 7 interfaces / 707 behaviors / 393 constraints / 0 relations). The `.10c`/`.10g` register families predate this doc's stale evidence (rebuilt from the retained `source_ir.json` before the `normalized/` bundle was reclaimed), so re-ingest's value is **normalized-bundle restoration + current-binary confirmation**, not a marquee jump.
- **`.isf` renderable** (`agent.isf`, 189 signals / 42 storage(reset) / 13 enums); real `fsmgen --strict --check --json` **success / 0 diagnostics** (strict-clean). `validate` shows no stage-staleness (semantic+intent relations both 0 → 0-vs-0 honest absence) and IntentIR quality 24/100 INCOMPLETE (honest for a CPU-core TRM lacking wire direction/width/clock/reset grounding — graph direction 0%, semantic role 0%).
- **Honest absences:** 0 relations/interfaces (register/behavioral intent, not wire relations); 0 message-fields (no packet/structure tables); the 42 registers' bit-fields are largely UNLOCATED → honest `.isf` residuals (`isf_register_fields_not_lowered` 469 bit-fields + `isf_storage_reset_not_lowered` 17 field-resets — no fabrication, the located-fields-only rule).
- **Survey correction:** a one-pass scan of the 35 remaining normalized-missing docs shows the named register-heavy candidates (CoreSight SoC-600 ×3 = 597/631/833 regs, AMD-IOMMU = 217 msg-fields, VT-d = 103 regs) are ALSO already current-binary-equivalent → the marquee-jump phase is effectively over; the remaining slices are the #21/#22 confirmation+restoration class (still genuine coverage: restoring `normalized/` makes a doc rebuildable rather than dependent on a single retained `source_ir.json`).
- **Gates:** no code change (re-ran existing deterministic extractors) → WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal by construction; binary current (no rebuild). 20/22 re-ingested docs strict-clean. Coverage: 22 of 57 normalized-missing docs re-ingested. Durable trace: `docs/tasks/CORPUS-COVERAGE.md` `.2` log.

### DOC-INTENT-TAXONOMY.4d.i — cat-4 RISC-V CSR bit-position recovery MEASURED non-viable → honest residual (no code, no FR)
Resolved `.4d.i` by testing the planned deterministic CSR bit-position-recovery CODE against the actual RISC-V Debug source data + the human-reviewed bit gold **before writing it** — overturning the pre-investigation's optimistic *"deterministically tractable — the table carries the positions"* verdict with per-item, gold-checked evidence (`[[feedback_scoring_rigor]]`). Measurement + decision packet, read-only, **no Rust code**; tracked reproducer `scripts/measure_cat4_csr_bit_recovery.py`.
- **The bits live in the IMAGE, not in text.** 53 of 56 RISC-V Debug register-with-address headings carry the bit layout as an `![Image]` diagram (incl. the cleanest gold register `dmcontrol`, captured as `![Image]` **only** — no table to parse); only 7 diagrams were flattened to a text table. This is exactly the non-text modality `EXTRACTION-GAP-FIX.4` already built a **VLM** reader for.
- **The 7 flattened tables are garbled or symbolic.** `dmstatus`'s explicit high-bit row is **off by ~8** (places `ndmresetpending` at bit 16, gold 24), drops the 7-field middle band (`allresumeack`(17)…`allrunning`(11)), and mixes doubled cells + two stacked half-rows; `tdata1`'s positions are **symbolic XLEN-relative** (`XLEN-1`, `XLEN-5`). A deterministic parse recovers ~0 correct fields and/or **fabricates** wrong bits.
- **The tiling gates are order-blind.** `ir/register_bits.rs` gates on width-sum + name-multiset only, not field order, so a row-jumbled flattened table could pass both gates with WRONG bits — strictly more dangerous than the VLM front-end, on the shared register path `.4a.ii` (24 docs / 6,570 fields) relies on.
- **Verdict.** Deterministic-table lever non-viable → **honest residual, no Rust code, no FSMGen FR** (ISF already expresses register fields via `.4a.ii`; a speculative parser would fabricate — `[[feedback_verify_fsmgen_before_fr]]` / `[[feedback_isf_no_hacks]]`). The genuine lever is a **sharper VLM read** for the existing gated path (`recover-register-bits`), bound purely by VLM accuracy, owned OUTSIDE the `.4` ISF-lowering program; the AIA sub-lever is blocked on RAM/Docling-gated re-ingest (`CORPUS-COVERAGE`) + prose-bound. Mirrors `.4c.i` (cat-3 topology). With `.4d.i` resolved, the `DOC-INTENT-TAXONOMY.4` actionable frontier is exhausted except the FSMGen-gated `.4b`.
- **Gates.** Report `docs/research/cat4-csr-bit-position-recovery-measurement.md`; KM `[[cat4-csr-bit-position-recovery-not-deterministic]]` (map 119→120). No code / no canonical mutation → wire golds / `kg-bench` / emitted `.isf` byte-identical by construction (WIRE-BASED-100 orthogonal). `scripts/check_doctrines.sh` green (memory-arch + knowledge-map + task-acceptance); `mdbook build` green.

### DOC-INTENT-TAXONOMY.4d.i — pre-investigation feasibility probe (read-only, no code; leaf stays pending)
Read-only feasibility probe for the next CODE lever (cat-4 RISC-V CSR bit-position recovery), recorded for continuity — no decision, the `.4d.i` leaf stays `pending`.
- **Found.** RISC-V Debug's `normalized/` bundle is present (re-ingest not needed); 0/179 fields located. The bit positions **do exist in the source** — as a Docling-flattened register-**diagram** table sitting *adjacent to but separate from* the `Field | Description | Access | Reset` table the current `field_table` strategy reads (e.g. `dmstatus`, normalized `.md` ~L1042). The diagram table is intricate/noisy: wide fields doubled across cells, separate width + high-bit rows, the 32-bit register drawn as two stacked half-rows, interspersed reserved `0` fields; the diagram is also an image (a VLM path exists but is **not** required — the table carries the positions).
- **Verdict.** `.4d.i` is **deterministically tractable (no VLM strictly needed)** but a **substantial, regression-sensitive** build: it must add a robust register-diagram-table grammar keyed off the diagram *shape* (ADR 0006, no RISC-V name list) and join it to the field-description table by name, and it modifies the shared register path that `.4a.ii` (24 docs / 6,570 emitted fields) depends on. Warrants a fresh, full-focus session for signoff sharpness; the design starts from this probe. Full finding in the `.4d.i` task node.
- **Gates.** No code / no canonical mutation → wire golds / `kg-bench` / emitted `.isf` byte-identical by construction (WIRE-BASED-100 orthogonal). `scripts/check_doctrines.sh` green; knowledge-map unchanged (no new fact card). Continuity note only.

### DOC-INTENT-TAXONOMY.4c.i — cat-3 topology-capture recall: capture-recall-gated, not abstraction-gated → no FR (honest residual)
Measurement (read-only, docs-only — no Rust code; tracked reproducer `scripts/measure_cat3_topology_recall.py`). Spun out of `.4c`, which deferred the cat-3 ISF-construct decision (verified FSMGen FR vs honest non-target) to a capture-recall measurement.
- **Measured.** Full 15-doc cat-3 (platform/system-IP) set vs the 4 cat-1 wire gold docs as a reference baseline. **Cat-3 topology capture is unfaithful: SPARSE** (380 actors → 135 `signal_connectivity` edges = **0.355 edges/actor**, vs the SAME surface at **4.108 edges/actor** on wire docs — ~12× denser there because the protocol signal graph IS the topology), **three-quarters HALF-CONNECTED** (only **24%** of cat-3 edges carry both a producer AND a consumer, vs **85%** on wire; GIC-600's 66 edges collapse to 4 usable, GIC-400's 8 to 0), and **ROOTLESS** (**0/10** `infrastructure_signals` carry a resolved clock/reset source; only 6 carry a fan-out list). Name quality is the *smaller* problem (endpoints **95%** clean; 12 escaped corpus-wide) — refining `.4c`'s "sparse + noisy" to **sparse + half-connected + rootless-infra with minor name noise**. All three findings robust to dropping the 4 borderline docs (cat-3 core 0.395 / 20% / 0).
- **Verdict.** Cat-3 topology capture is **capture-recall-gated, NOT abstraction-gated** → **no FSMGen FR** (an FR on a 12×-too-sparse / three-quarters-broken capture would be unfalsifiable — `feedback_verify_fsmgen_before_fr`); topology stays an **honest residual**. The same surface is dense+clean+rooted on wire docs, so it is *capable* — the cat-3 shortfall is how much of a multi-component TRM's prose interconnect the extractor recovers. The buildable lever, if ever pursued, is **upstream extraction-recall owned OUTSIDE the `.4` ISF-lowering program** (denser/fully-connected connectivity + clock/reset source resolution), recorded as a cross-reference NOT a `.4` gap (mirrors `.4d.i` cat-4 + the cat-2 structure-recall frontier; `feedback_scoring_rigor`). Even with faithful capture, lowering would also need a multi-actor ISF emit (today single-initiator, KG-ISF-COMPLETENESS.2a.ii) + a construct FSMGen's behavioral ATL frontier does not subsume — resolved WITH FSMGen only after capture clears the measured bar. Resolves `.4c`'s deferred construct question.
- **Gates.** No code / no canonical mutation → wire golds / `kg-bench` / emitted `.isf` byte-identical by construction (WIRE-BASED-100 orthogonal). `scripts/check_doctrines.sh` green (memory-arch + knowledge-map + task-acceptance); `mdbook build` green; knowledge-map derive-and-diff in sync (118 → 119 facts). Report `docs/research/cat3-topology-capture-recall-measurement.md`; KM `docs/knowledge/cat3-topology-capture-recall.md`.

### DOC-INTENT-TAXONOMY.4e — conditional-rule lowering triage (.2 Result 3): honest residual, no ISF lever, no FR (.2 measurement phase complete)
Measurement triage (read-only, docs-only — no Rust code). Resolves `.2` Result 3 (the `conditional_rules` ISF-lowering shortfall), separating honest residual from a real lever per item, before any fraction is called a gap.
- **Measured.** 603 `conditional_rules` across 9 representative docs (all 4 buildable categories), classified against each doc's declared-signal inventory (`interfaces[].signals` + `signal_records[].signal_name` + `actor_ports[].signal_name`) + consequent quality: **A** no-consequent prose 392 (65%) / **B** undeclared signal 14 (2%) / **C** declared+placeholder 33 (6%) / **D** declared+non-placeholder action 164 (27%). The D "candidate-lever" bucket is **161/164 bare deontic modals** (`shall` 45 / `must` 35 / `shall not` 14 / `shall be cleared` 10 / …) with no concrete obligation — only **3/603** carry a concrete value/level cue.
- **Verdict.** The conditional-rule shortfall is **honest residual, NOT an ISF-completeness gap**: no buildable ISF lever, no FSMGen FR. A conditional lowers to an ISF `(rule)` only when it names a declared signal AND a concrete obligation; lowering a bare `shall`/`must` would fabricate the obligation (forbidden — `feedback_isf_no_hacks`). The adapter already lowers the cleanly-grounded conditional obligations (516 corpus-wide, KG-ISF-COMPLETENESS.4); `signal_constraints` + `temporal_rules` lower well. The only improvement path is **upstream EXTRACTION quality** (recover the concrete obligation from the modal conditionals' `source_text`), owned by the extraction-quality program (`extract-constraints-llm` / `EXTRACTION-QUALITY-GAUGE`), distinct from the `.4` ISF-lowering program and lower-leverage than register/structure/topology — recorded as a cross-reference, deliberately NOT minted as a `.4` gap (`feedback_scoring_rigor`). With `.4e` closed, the `.2` per-category scorecard's **measurement phase is COMPLETE**; remaining `.4` work is CODE (`.4d.i`, `.4c.i`, `.4b`).
- **Gates.** No code / no canonical mutation → wire golds / `kg-bench` / emitted `.isf` byte-identical by construction (WIRE-BASED-100 orthogonal). `scripts/check_doctrines.sh` green (memory-arch + knowledge-map + task-acceptance); `mdbook build` green; knowledge-map derive-and-diff in sync (117 → 118 facts). Report `docs/research/conditional-rule-lowering-triage.md`; KM `docs/knowledge/conditional-rule-lowering-triage.md`.

### DOC-INTENT-TAXONOMY.4c — cat-3 topology lowering decision: topology captured-but-sparse, no static-topology ISF construct, no FR yet (spun out .4c.i)
Measurement + decision packet (read-only, docs-only — no Rust code). Resolves whether category-3 (platform / system-IP topology & integration) intent maps onto an existing ISF construct or needs a new one, from measured evidence over 3 representative cat-3 docs (of 15) + the current FSMGen pin `d327129b7`.
- **Measured.** Cat-3 docs **do** carry a typed topology surface (refining the `.2` "hint-level" label to *captured-but-sparse-and-unlowered*): CoreSight SoC-600 = 60 actors / **6** `signal_connectivity` edges / 833 registers; GIC-600 = 55 actors / **66** `signal_connectivity` / **2** `infrastructure_signals` (carrying `infrastructure_topology` / `distributed_to_actor_ids`); CoreSight Base System Arch = 5 actors / prose. `signal_connectivity` is a producer→consumer graph; `infrastructure_signals` is the clock/reset distribution.
- **Decision (two halves).** (1) **Cat-3's register half already lowers** — register maps + bit-fields via `.4a.ii` (SoC-600 ~3,250 fields), infrastructure signals, actor ports — the same constructs as cat-2; not the gap. (2) **The distinctive topology half is captured but unlowerable today** — ISF has **no declarative static-topology construct** (composition is transaction-level only, `13f-composition.md`; FSMGen's ATL multi-actor frontier is BEHAVIORAL generated-child transaction wiring, not a declarative IP-interconnect netlist, `14-feature-backlog.md`; the SpecForge emit is single-initiator-actor, so cross-component topology is structurally absent by design). **No FSMGen FR is filed yet** — premature because the topology capture is sparse/noisy (6 edges across SoC-600's 60 actors; `None`/escaped actor names) AND ISF is a per-actor format where static topology may deliberately be the integrator's concern above per-module synthesis (a scoping question for FSMGen). Topology stays an honest residual; the buildable next step is the measurement `.4c.i` (topology-capture recall across all 15 cat-3 docs), after which the construct decision (verified FR vs honest-non-target) becomes real.
- **Gates.** No code / no canonical mutation → wire golds / `kg-bench` / emitted `.isf` byte-identical by construction (WIRE-BASED-100 orthogonal). `scripts/check_doctrines.sh` green (memory-arch + knowledge-map + task-acceptance); `mdbook build` green; knowledge-map derive-and-diff in sync (116 → 117 facts). Report `docs/research/cat3-topology-isf-lowering-decision.md`; KM `docs/knowledge/cat3-topology-isf-lowering-decision.md`; book `docs/book/src/document-categories.md` cat-3 maturity refined.

### DOC-INTENT-TAXONOMY.4d — cat-4 ISA/CSR lowering decision: CSRs reuse register/storage; gap is extraction recall (spun out .4d.i)
Measurement + decision packet (read-only, docs-only — no Rust code). Resolves the standing category-4 (CPU ISA / privileged architecture) ISF-lowering Open Question from measured evidence over the corpus's 2 cat-4 docs (RISC-V Debug, RISC-V Advanced Interrupt Architecture) and the current FSMGen pin `d327129b7`.
- **Measured.** RISC-V Debug = **44 `register_records` / 179 fields / 0 located** (every field carries `field_name`+`access_type`+`reset_value`+`description` but none carries `bits_high`/`bits_low`/`bit_width`); RISC-V AIA = **0 `register_records`** (its IMSIC/APLIC CSR intent sits in 39 prose `conditional_rules` + 493 behaviors; its 211 `interfaces` are empty prose shells — 0 constraints / 0 relations). FSMGen pin `d327129b7` titles `(storage (var … (fields …)))` the "register map / CSR" construct (`subs/fsmgen/docs/book/src/13a-actor-interface.md:419`/`:468`) and declares **no** instruction/privilege/exception construct.
- **Decision (cat-4 splits into three).** (1) **CSR / register intent REUSES the existing register/storage abstraction** — no new ISF construct, no FSMGen FR (CSRs are structurally registers; the construct `.4a.ii` lowers cat-2/cat-3 fields into is the register-map/CSR construct). (2) **The cat-4 register gap is EXTRACTION RECALL, not abstraction** — fields reach `.isf` 0 times because they are *unlocated* (Debug) or *uncaptured* (AIA), not because ISF can't express them → spun out **`.4d.i`** (recover RISC-V CSR field bit positions + a RISC-V-shaped register recogniser; once located, fields auto-lower via `.4a.ii`, no emitter change). (3) **Instruction / privilege-mode / exception / memory-ordering → honest NON-TARGET** (software-visible ISA semantics, not synthesizable hardware intent; no ISF construct + FSMGen lists none → no FR per the verify-before-FR / no-hacks doctrine; a conditional-future only if FSMGen's SV/UVM verification path explicitly scopes ISA-model verification).
- **Gates.** No code / no canonical mutation → wire golds / `kg-bench` / emitted `.isf` byte-identical by construction (WIRE-BASED-100 orthogonal). `scripts/check_doctrines.sh` green (memory-arch + knowledge-map + task-acceptance); `mdbook build` green; knowledge-map derive-and-diff in sync (115 → 116 facts). Report `docs/research/cat4-isa-csr-lowering-decision.md`; KM `docs/knowledge/cat4-isa-csr-lowering-decision.md`; book `docs/book/src/document-categories.md` cat-4 maturity refined.

### DOC-INTENT-TAXONOMY.4a.ii — emit register bit-field map into ISF field-structured storage (6,570 fields / 24 docs)
Gap A of the per-category ISF-completeness scorecard: register **bit-fields** reached the emitted `.isf` zero times (12,638 captured fields) because the emitter discarded them at `isf_ir.rs` and emitted opaque width-only `(var …)`. FSMGen shipped the declarative field-structured-storage construct (pin `d327129b7`), so the emitter now lowers the field map into it.
- **Emitter (`crates/specforge/src/ir/isf_ir.rs`).** New `IsfStorageField` + `IsfStorageVar.fields`; the storage render emits `(var NAME (width N) [(reset V)] (fields (field FNAME (bits HI LO) [(access …)] [(reset V)] [(enum (M V)…)]) …))` when fields are present, and the opaque single-line form (byte-identical to before) when not. Field derivation is the pure `register_storage_fields` + `normalize_field_access`; the new `isf_register_fields_not_lowered` adapter residual records the unlowered remainder (`crates/specforge/src/ir/adapters.rs`). A test-only `run_fsmgen_schedule_json` helper (`ir/mod.rs`) exposes FSMGen's `inferred_storage[].fields[]` report.
- **Admission gate (ADR 0006, structural, fail-closed).** Located fields only (unlocated bits → honest gap; the block allows gaps); a sanitized field name shared by ≥2 located fields drops the whole colliding group (uniformly handles repeated `Reserved`/`res0` gaps and flattened mis-extractions — FSMGen fails closed on duplicate field names, and renaming/dropping-one would fabricate/guess); any residual bit overlap fails the register's field block closed; `access_type` normalized to FSMGen's `ro|rw|wo|w1c|w0c|rc|rs|warl|wpri|reserved` set plus unambiguous synonyms (omitted when unmapped); a field `(reset)` only when the parent reset is composed, emitted as that value's own bit slice (matches the parent slice by construction); `(enum)` keeps members whose numeric value fits the field width (`meaning`→sanitized member, deduped). No bit position, access, reset, or enum is ever fabricated.
- **Measured live with the real emitter (`adapt --target isf` over the corpus):** 6,570 register bit-fields now reach `.isf` across 2,531 registers in 24 docs (was 0) — CoreSight SoC-600 1,166/1,118/974, GIC arch `ihi0069` 424, CCIX 380, SMMU `ihi0070` 332, CHI-C2C 276, …
- **No regression.** The 4 WIRE-BASED-100 golds (AXI/APB/AHB/AXI-Stream) emit 0 fields → emitted `.isf` byte-identical (proved by old-vs-new `adapt` diff); metadata-only / schedule-safe (FSMGen's scheduled `.fsm` is byte-identical with vs without `(fields …)`); RISC-V IOMMU (122) / GIC (424) / CoreSight SoC-600 (974) keep `fsmgen --strict` `success / 0 diagnostics` (0 new); `inferred_storage[].fields[]` round-trip asserted; `kg-bench 156/156`; full `cargo test 1702/0` (warning-deny, +6 tests); `cargo fmt`/`clippy -D warnings` clean; `run_ci.sh` green.
- **Lockstep:** mdBook (`pipeline/isf-adapter.md` "Register bit-fields" flipped from residual to lowered + closed-task subsection; `document-categories.md` cat-2 maturity); KM card `register-bit-field-isf-lowering-gap` + regenerated `KNOWLEDGE_MAP.md`; README ISF-emitter bullet; task tree `docs/tasks/DOC-INTENT-TAXONOMY.md` (`.4a.ii` done + acceptance checklist) + `docs/TASK_TREE.md`; `DEVELOPMENT_NOTES.md`; `RUST_CODEBASE_ANALYSIS.md`; `LIVE_ACHIEVEMENT_STATUS.md`; `MEMORY.md`. `.4a.i` superseded; Gap B (`.4b`) stays FSMGen-deferred.

### FSMGEN-REFRESH-INTEGRATE-5.1 — bump subs/fsmgen 5ce0335c5→d327129b7 (SHIPPED storage fields) + un-gate DOC-INTENT-TAXONOMY.4a.ii
Fifth FSMGen refresh cycle (owner-directed: "FSMGEN just pushed. Update FSMGEN submodule then read [the declarative-storage-fields spec/book sections]"). FSMGen **shipped** the field-structured-storage construct that SpecForge's `DOC-INTENT-TAXONOMY.4a` FR requested.
- **Pin bump** `5ce0335c5 → d327129b7` (+2 commits: `ISF-FIELD-STRUCTURED-STORAGE-FRONTIER.1` contract-select + `.2` ship-scalar-storage-fields). No SpecForge Rust code touched.
- **Verified empirically on the new binary:** the 6 `*_passes_fsmgen_strict_validation` canaries PASS (emitted `.isf` still strict-valid), `run_ci.sh` green (mdBook built), `kg-bench 156/156`. The feature is additive + metadata-only (FSMGen: scheduled `.fsm` byte-identical with vs without `(fields …)`) → emitted `.isf` unchanged, WIRE-BASED-100 orthogonal, no contract break.
- **Shipped construct** (refs on `d327129b7`: `ISF_DOWNSTREAM_INTEGRATION_SPEC.md:637`/`:3450`, `book/src/13a-actor-interface.md:468`, matrix `13k-…:42`): `(storage (var NAME (width N) [(reset V)] (fields (field FNAME (bits HI LO) [(access ro|rw|wo|w1c|w0c|rc|rs|warl|wpri|reserved)] [(reset V)] [(enum (M VAL)…)]) …)))` — metadata-only/schedule-safe, published in the report as `inferred_storage[].fields[]: name, msb, lsb, width, access, reset, enum`. Fail-closed: overlap/out-of-width fields, unsupported access tokens, a field reset not matching the parent reset slice, out-of-width enum values. Deferred: `(enums)` refs, banks, **packet/flit layouts** (Gap B), access enforcement.
- **Consequence:** `DOC-INTENT-TAXONOMY.4a.ii` (the Gap-A register bit-field emit — 12,638 fields / 32 docs that reach `.isf` zero today) is **UN-GATED and is now the highest-leverage buildable lever**; it **supersedes** `.4a.i` (the honest residual, now a fallback). The exact `RegisterFieldRecord`→ISF mapping + the reuse of the `classify_register_reset` tiling gate + the `inferred_storage[].fields[]` verification oracle are captured in the `.4a.ii` node for the next session. Gap B (`.4b`) stays gated (packet/flit deferred).
- **Lockstep:** FR marked SHIPPED in `docs/FSMGEN_FEEDBACK.md`; `README.md` pin updated (`5ce0335c5 → d327129b7`); `docs/tasks/DOC-INTENT-TAXONOMY.md` `.4a.ii` un-gated + grammar/mapping/oracle; new tree `docs/tasks/FSMGEN-REFRESH-INTEGRATE-5.md` + `docs/TASK_TREE.md` row; KM card updated (map 828 keys, in sync). Tree CLOSED.

### FSMGEN-REFRESH-INTEGRATE-4.1 — bump subs/fsmgen 030f8c273→5ce0335c5 + integrate the accepted field-structured-storage FR answer
Fourth FSMGen refresh cycle (owner-directed: "FSMGEN provided an answer to your request, please update FSMGEN's submodule then read docs/SPECFORGE_FEEDBACK_RESPONSE.md"). Triggered by FSMGen's response to the `DOC-INTENT-TAXONOMY.4a` field-structured-storage FR.
- **Pin bump** `030f8c273 → 5ce0335c5` (+106 upstream commits: `ISF-FIELD-STRUCTURED-STORAGE-RESPONSE.1/.2` + FSMGen-internal IAL2 frontier work + a doctrine-adoption). No SpecForge Rust code touched.
- **Empirically verified on the new binary** (`[[feedback_fsmgen_contract]]` / `[[feedback_verify_fsmgen_before_fr]]` — never trust commit subjects): the 6 `*_passes_fsmgen_strict_validation` canaries PASS (emitted `.isf` still strict-valid), `run_ci.sh` green (1696 tests / 0 failed, rustdoc, mdBook), `kg-bench 156/156`. Contract delta = none for SpecForge (FSMGen states the response changes no parser/scheduler/lowerer/syntax). Emitted `.isf` unchanged → WIRE-BASED-100 orthogonal.
- **FR answer integrated.** FSMGen **ACCEPTED** the field-structured-storage FR as a real ISF representational gap + valid future direction and accepted the exact proposed shape (a storage var with an optional declarative field partition — per field: name, bit range, optional access `ro`/`rw`/`w1c`/`warl`, reset, enum, provenance; first version = checked metadata, fail-closed validation), but it is **NOT shipped** — gated on FSMGen's own `ISF-FIELD-STRUCTURED-STORAGE-FRONTIER.1` readiness/contract audit. FSMGen confirmed the **no-hack** stance (no `set-field`/`extract`/fake-drive/comment substitute — "would fabricate behavior"). The sanctioned near-term posture (FSMGen's words): keep recovered register/CSR + packet/flit field maps as IntentIR metadata/residuals, keep emitting opaque storage, fabricate nothing.
- **Consequence for the Gap-A program:** `DOC-INTENT-TAXONOMY.4a.ii` (field-structured emit) STAYS gated on FSMGen shipping the construct; `.4a.i` (the adapter honest residual + keeping field maps as IntentIR metadata) is now the FSMGen-**endorsed** near-term move.
- **Lockstep:** FR marked RESOLVED in `docs/FSMGEN_FEEDBACK.md`; `README.md` pin reference updated (`030f8c273 → 5ce0335c5`); `docs/tasks/DOC-INTENT-TAXONOMY.md` `.4a.ii` gating + `.4a.i` endorsement; new tree `docs/tasks/FSMGEN-REFRESH-INTEGRATE-4.md` + `docs/TASK_TREE.md` row; KM card `register-bit-field-isf-lowering-gap` updated (map 826 keys, in sync). Tree CLOSED.

### DOC-INTENT-TAXONOMY.4a — Gap A register bit-field ISF lowering: verified FSMGen field-structured-storage FR (measurement/design)
Measurement/design slice (docs-only, **no Rust code**) owning the `.2` highest-leverage gap: register **bit-fields** reach the emitted `.isf` **zero** times (12,638 fields / 32 docs) while registers lower 1:1 to opaque `(storage (var … (width N)))`. Two read-only probes localized the loss and decided the doctrine-correct fix.
- **Code-path map** — the bit-field intent is FULLY captured and carried; it is dropped ONLY at the ISF-emit boundary. `RegisterFieldRecord` (name/bits/width/access/reset/description/enums) at `crates/specforge/src/ir/source.rs:414` → carried UNCHANGED into IntentIR (`IntentIr.register_records = semantic_ir.register_records.clone()`, `crates/specforge/src/ir/intent.rs:193`) → discarded at `IsfStorageVar { name, width, reset }` (`crates/specforge/src/ir/isf_ir.rs:852`; `r.fields` read only to compose a register-wide reset), rendered opaque `(var NAME (width N) [(reset V)])` at `:391`. **No SpecForge carry gap.**
- **Empirical FSMGen probe (pin `030f8c273`)** — the ISF `(storage …)` grammar declares opaque width-only scalars only (`subs/fsmgen/docs/ISF_DOWNSTREAM_INTEGRATION_SPEC.md` §8); there is **no named-bit-field / packed-record construct**. The shipped `set-field`/`when-field`/`extract`/`assemble` are runtime read-modify-write ops on an opaque register (`13k` matrix), NOT a static field-map declaration; field-structured storage is not on the FSMGen backlog (`14-…`).
- **Decision** — Gap A is a genuine missing ISF abstraction → filed a **verified FSMGen feature request** for declarative field-structured storage (`docs/FSMGEN_FEEDBACK.md`, `## Feature request (2026-06-22) — declarative field-structured storage`), per `[[feedback_isf_no_hacks]]` / `[[feedback_verify_fsmgen_before_fr]]`. The three emitter-only alternatives (per-field vars / runtime `extract` / comments) are documented and rejected as fabrication or intent-loss. Gap B (1,220 message-field structures) shares the same missing abstraction and additionally lacks an `Evidence→Intent` carrier (zero `message_field` in `intent.rs`).
- **Artifacts** — report `docs/research/register-bit-field-isf-lowering-design.md`; KM card `docs/knowledge/register-bit-field-isf-lowering-gap.md` (map regenerated 114 → 115 facts / 824 keys, in sync); user-facing honest-residual note added to `docs/book/src/pipeline/isf-adapter.md`. Decomposed `.4` → `.4a` (done) / `.4a.i` adapter honest-residual (code) / `.4a.ii` field-structured emit (gated on FSMGen) / `.4b` Gap-B carrier / `.4c` cat-3 topology / `.4d` cat-4 ISA / `.4e` conditional-rule triage.
- **Gates:** `scripts/check_doctrines.sh` green (memory-arch + knowledge-map + task-acceptance); `mdbook build` green; knowledge-map derive-and-diff in sync. **No code / no canonical-artifact mutation → WIRE-BASED-100 + `kg-bench` + emitted `.isf` orthogonal by construction.** Frontier → `.4a.i` (RAM-light, no FSMGen dependency) then `.4b`.

### DOC-INTENT-TAXONOMY.3c — recognizer fixtures + user-facing mdBook chapter + KM card (.3 recognizer complete)
Test + docs slice closing the `.3` recognizer sub-tree (the recognizer was implemented in `.3b`).
- **`crates/specforge/src/commands/validate.rs`** — new end-to-end integration test
  `validate_evidence_ir_reports_document_intent_category`: builds an EvidenceIR through the real pipeline and asserts the
  `document_intent_category` / `document_intent_category_confidence` metrics + the `evidence_document_intent_category`
  finding reach the persisted report (near-empty doc → honest `unresolved` residual). The per-category gold/negative cases
  stay locked by the 13 in-file unit tests added in `.3b`.
- **`docs/book/src/quality/validation.md`** — new "What is the document *about*? — the purpose category" section beside
  the Document Class section: the 6 categories, the high-confidence-only-where-unambiguous policy, the register-never-vetoes
  -wire and flit-only-without-a-register-map discriminators, and the honest residual.
- **`docs/book/src/document-categories.md`** — the "how SpecForge determines a category" section flipped from "target" to
  "now reported directly by the CLI" (the recognizer is live), with the confidence/residual semantics.
- **`docs/knowledge/document-intent-category-recognizer.md`** — KM fact card (map regenerated 113 → 114 facts / 815 keys),
  with a `reverify` that re-runs the recognizer + the 78-doc tally; in sync.
- **ISA/PHY vocab calibration:** precision-verified against real corpus front-matter — the ISA vocabulary matches 0 docs
  (the 2 corpus ISA docs honestly fall through, as `.1` predicted), PHY recovers all 4 OpenCAPI PHY docs; no further
  widening without risking false positives.
- **Gates:** `kg-bench 156/156`; full `cargo test` green (warning-deny, +1 integration test); `cargo fmt`/`clippy -D
  warnings` clean; `mdbook build` green; knowledge-map derive-and-diff in sync; `run_ci.sh` green. WIRE-BASED-100
  orthogonal (test + docs only). **`DOC-INTENT-TAXONOMY.3` (fast category recognizer) COMPLETE**; frontier → `.4+`
  per-category levers (Gap-A register bit-field lowering) + FSMGen ISF-abstraction FRs.

### DOC-INTENT-TAXONOMY.3b — implement the 6-category document PURPOSE recognizer (validate-reported)
Code slice (the FIRST Rust change gated by the new `TASK-ACCEPTANCE` doctrine). Operationalizes the `.3a` design: a
deterministic, name-list-free recognizer that reports what a chip-spec PDF is *about* (its purpose category), beside the
existing structural `document_class`.
- **`crates/specforge/src/ir/completeness.rs`** — new `DocumentIntentCategory` (6 purpose variants + `Unresolved`),
  `IntentCategoryConfidence`, `DocumentIntentClassification`, and the pure `classify_document_intent_category(census)`
  (sibling to `classify_document`, in-file tests). `DocumentClassCensus` extended with `message_field_records` /
  `signal_presence_records` / `front_matter_isa` / `front_matter_phy` (recognizer-only; `classify_document` unchanged).
  New generic front-matter helpers `front_matter_declares_isa` / `front_matter_declares_phy` (ADR 0006) + a shared
  `front_matter_has_word` (existing `front_matter_doc_type_hint` refactored onto it, behavior identical).
- **`crates/specforge/src/commands/validate.rs`** — census now bound once and fed to BOTH classifiers; new printed
  "Document Intent Category" block, `evidence_document_intent_category` Info finding (carries confidence + the honest
  residual), and `document_intent_category` / `document_intent_category_confidence` metrics.
- **Measured refinement of `.3a`:** per-document measurement falsified the literal `.3a` flit cue (it would misclassify
  NVMe / AMD-IOMMU / GIC-600 as wire). The shipped discriminators: flit fields are a cat-1 cue ONLY without a register
  map (CHI/DTI reg=0 vs NVMe/AMD/CCIX reg>0), plus a wire-weight vs register/structure-weight dominance test (register
  count never vetoes a clean wire shape — AXI: wire 401 ≥ struct 229). Only clean wire + self-declared guide are HIGH
  confidence; everything else is LOW + explicit residual.
- **Verified:** live `validate` over all 78 persisted docs → 21 wire-protocol / 8 methodology-guide (both high) /
  28 register-or-platform / 16 unresolved / 5 physical-link (low), **0 high-confidence false positives**.
- **Gates:** `kg-bench 156/156`; completeness lib `66/66` (13 new recognizer tests); full `cargo test` `1695 passed; 0
  failed` (warning-deny); `cargo fmt`/`clippy -D warnings` clean. WIRE-BASED-100 provably orthogonal (pure new fn +
  additive reporting; no extraction/emitter path touched → wire golds byte-identical by construction). User-facing
  mdBook chapter + KM card + gold/negative fixtures + ISA/PHY vocab calibration land in `.3c`.

### DOCTRINE-ENFORCEMENT-ADOPT.2 — user-facing mdBook chapter + KM card (book-method-doc close); tree CLOSED
Closing leaf (docs-only). Documents the adopted doctrine-enforcement system on the user-facing surface and writes the
durable fact card, completing the `DOCTRINE-ENFORCEMENT-ADOPT` tree.
- **`docs/book/src/reference/doctrine-enforcement.md`** (in `SUMMARY.md` under Reference): user-friendly,
  why-before-what — why "trust me" compliance + silent drift both fail; doctrine = rule + a deterministic check that
  exits nonzero on breach; the 3 check kinds (structural / oracle / evidence); the 3 registered doctrines; the
  acceptance checklist for code changes + "earned, not ticked"; the E1→E4 layering with stated honest limits (local
  hooks bypassable, hosted CI manual-only); how to run + extend `check_doctrines.sh`; how the adoption was verified.
  `mdbook build` green.
- **`docs/knowledge/doctrine-enforcement-adoption.md`** — KM fact card (map 112→113 facts / 802 question keys), with a
  `reverify` that re-runs the driver and demonstrates the TASK-ACCEPTANCE block; in sync (driver KM check PASS).
- **Gates:** docs-only, no Rust → WIRE-BASED-100 + register/wire golds + `kg-bench` orthogonal; `mdbook build` green;
  memory-arch + knowledge-map + task-acceptance (exempt) 3/3 PASS. **Whole tree complete** (`.0` `810b510b` + `.1`
  `a9c8d415` + `.2`). Next active work → `DOC-INTENT-TAXONOMY.3b`.

### DOCTRINE-ENFORCEMENT-ADOPT.1 — SpecForge-native task-acceptance evidence check + SpecForge TOOLBOX.md
Mechanizes SpecForge's flagship doctrine (decision 0003, owner-restated repeatedly): no code change without an owning
task-tree leaf first, and the change must be PROVABLY taken through diagnose→address→no-regression, not "trust me".
Owner clarification (`2026-06-22`): `TOOLBOX.md` must catalog SpecForge's OWN debug/diagnostic tools. Scripts+docs only,
no Rust.
- **`scripts/check_task_acceptance.sh`** (evidence archetype, bash-3.2-safe — no `mapfile`): a staged Rust code change
  (`crates/**/*.rs`, `crates/**/test_data/**`) must have a staged owning `docs/tasks/*.md` leaf whose acceptance
  checklist carries ROOT CAUSE / ADDRESSED / NO REGRESSION, ticked AND backed by SpecForge tool signatures (`validate`/
  `adapt` findings + `blocking_reason`, `kg-bench 156/156`, `WIRE-BASED-100 1.000`, byte-identical, `run_ci`/`cargo`).
  Docs/scripts/mdBook/.githooks changes are EXEMPT (own gates) → no false-blocking of continuity work. Knobs
  `SPECFORGE_TASK_ACCEPTANCE_RANGE` / `_WAIVER` (loud, never silent). Registered `TASK-ACCEPTANCE` in
  `scripts/check_doctrines.sh` → driver 3/3 PASS.
- **`TOOLBOX.md`** — SpecForge's diagnostic-tool catalog (WHAT/WHEN/HOW/OUTPUT per tool): `doctor`, `inspect`,
  `validate` (metrics/findings/`document_class`), `adapt --target isf` (`blocking_reasons`), FSMGen `--strict --check`
  (the `.isf` contract canary), `kg-bench` (156/156), the WIRE-BASED-100 golds, `--dry-run` byte-identical orthogonality
  proof, `nli-verify`, `eval-extraction`, `audit-extraction`, `grits-consensus`, `measure_isf_completeness.py`,
  `corpus-cluster`, `run_ci.sh` / `check_doctrines.sh`, `project-validation`/`rescan-plan`, `clean`, RAM-bounded build
  — plus the acceptance-checklist template and 3 diagnostic protocols (extraction miss / `.isf` lowering block /
  kg-bench-or-gold regression).
- **Gates:** all 5 gate behaviors tested (exempt / block-no-leaf / pass / block-unticked / block-unbacked) with throwaway
  staged files fully reverted; `bash -n` clean; driver 3/3 PASS; no Rust → WIRE-BASED-100 + register/wire golds +
  `kg-bench` orthogonal. Frontier → `.2` mdBook chapter.

### DOCTRINE-ENFORCEMENT-ADOPT.0 — adopt the portable Doctrine-Enforcement architecture (framework + register existing checks + wire gates)
Owner directive (`2026-06-22`): "adopt this doctrine enforcement system" → `DOCTRINE_ENFORCEMENT.md`, the portable,
project-agnostic standard (the 4th architecture, sibling of `MEMORY_ARCHITECTURE.md`) that turns every written
doctrine into `rule + a deterministic check that exits nonzero on any breach`, run from one registry/driver and
gated by the same E1→E4 defense-in-depth. SpecForge already had E1–E4 for two doctrines (memory-architecture,
knowledge-map) as a hand-rolled pre-commit/CI stack; this slice unifies them under one driver and lands the
standard. No extraction/emitter Rust touched.
- **`scripts/check_doctrines.sh`** — the registry+driver: runs every registered `check_*.sh`, prints a per-doctrine
  PASS/FAIL report, exits nonzero on any breach, and **meta-checks** that each registered enforcer exists + is
  executable (a registry entry can never be a dangling promise). Registers `MEMORY-ARCH`
  (`scripts/check_memory_architecture.sh`) + `KNOWLEDGE-MAP` (`knowledge-map/scripts/check_knowledge_map.sh`).
- **`.githooks/pre-commit`** (E3) and **`scripts/run_ci.sh`** (E4) now route through the driver, preserving the
  knowledge-map regen+stage step that must run before the driver validates the derived map.
- **`DOCTRINE_ENFORCEMENT.md`** (standard; §0–§9/§11 portable, §10 = SpecForge's live registry), decision record
  **`0006`**, and discovery pointers (`README.md` doc map + `AGENTS.md` + `CLAUDE.md`).
- **Gates:** driver green (2/2 doctrines PASS); knowledge-map derive-and-diff in sync; no Rust → WIRE-BASED-100 +
  register/wire golds + `kg-bench` orthogonal by construction. Owning tree `docs/tasks/DOCTRINE-ENFORCEMENT-ADOPT.md`;
  frontier → `.1` native task-acceptance check + SpecForge `TOOLBOX.md`.

### DOC-INTENT-TAXONOMY.2 — per-category ISF-lowering completeness gauge (read-only): the maturity column, measured
Read-only measurement: a per-surface lowering ledger over all 78 docs (76 persisted `adapter.json` + 2 read-only
`adapt --dry-run` for the unmaterialized adapters; no `validate`/`adapt` write → zero canonical mutation),
reproducible via tracked `scripts/measure_isf_completeness.py`. Turns the `DOC-INTENT-TAXONOMY.0` maturity column
from honest-estimate into objective measurement, surface by surface (a single blended % would be gameable —
`feedback_scoring_rigor`).
- **Two dominant TRUE GAPS** (intent captured in volume, lowered to `.isf` zero times, across every buildable
  category): **(A) register bit-fields** — 3,449 registers lower 1:1 to opaque width-only `(storage (var (width N)))`,
  but their **12,638 constituent bit-fields reach `.isf` ZERO times** across 32 docs (cat 3 largest at 9,308; the
  register *programming model* does not synthesize). **(B) message-field structures** — 1,220 flit/packet/descriptor
  fields are recovered into EvidenceIR but **IntentIR has no `message_field_records` carrier** (`has_msgfld_key=false`),
  so 0 are carried and 0 lowered across 11 docs (NVMe 216, AMD-IOMMU 217, CHI 106, DTI 159, CHI-C2C 210, CCIX ×4 ≈ 309).
- **Both gaps converge on the SAME missing FSMGen ISF abstraction** — field-structured storage (register-with-fields)
  and packet/structure layouts (the memory-bank / single-dual-port-memory family the owner anticipated) → the headline
  `.4+` FSMGen FR candidates, filed only after empirical submodule verification (`feedback_verify_fsmgen_before_fr`,
  `feedback_isf_no_hacks`).
- **Scorecard, measured:** cat 1 wire-protocol MATURE (signals/relations/constraints/temporal/enums lower;
  WIRE-BASED-100 = 1.000) · cat 2 register-IP + cat 3 platform-IP PARTIAL (registers→storage 1:1, but fields +
  structures unlowered; cat 3 topology stays hint-level, rules ~11%) · cat 4 CPU-ISA THIN (only the register-shaped
  surface lowers; no instruction/CSR/privilege/exception construct) · cat 5 PHY + cat 6 guide honest non-targets
  (thin `.isf` is correct). **Precision note:** 5/14 cat-6 guides over-extract spurious `.isf` (cortex-a76 sw-opt 537
  signals, readme 152, smmu software guide 144, gic overview 89, aarch64 debug guide 73) — a `.3`-recognizer precision
  motive, not an ISF-completeness gap. Signals deliberately NOT scored as a present/lowered ratio (the `.isf` signal set
  has a different basis than IntentIR `interfaces`); the rule-lowering shortfall is mixed (honest residual + lever),
  flagged for per-item triage.
- Report `docs/research/document-intent-isf-completeness.md`; KM card `document-intent-isf-completeness` (map → 112 facts).
- **Gates:** read-only, no code, no canonical mutation → WIRE-BASED-100 + register/wire golds + `kg-bench` orthogonal;
  memory-arch + knowledge-map gates green; mdBook builds. Frontier → `.3` fast category recognizer; `.4+` Gap-A
  register-field lowering as the highest-leverage first lever.

### DOC-INTENT-TAXONOMY.1 — corpus census by category (read-only): 36/7/15/2/4/14, and what surface counts can't classify
Read-only measurement (profiled all 78 persisted `evidence_ir`/`intent_ir` docs by typed surface; no `validate` →
zero artifact mutation). Establishes the per-category denominator before the `.2` ISF-completeness gauge.
- **Distribution (78 docs):** 36 wire-protocol (cat 1) · 7 register-IP (cat 2) · 15 platform/system-IP (cat 3) ·
  2 CPU-ISA (cat 4) · 4 PHY/electrical (cat 5) · 14 methodology/guide (cat 6). Wire-protocol-dominant (46%) — why
  cat 1 is the mature one; cat 2+3 (28%) are the largest buildable expansion; cat 5+6 (23%) are honest non-targets.
- **Three measured structural blind spots** (why the 4-way `document_class` is too coarse for purpose): (a) cat 2↔3
  NOT separable by surface counts (both register/structure-dominant — the IP-vs-platform difference is semantic);
  (b) cat 4 (ISA) has NO distinct signature — the 2 ISA docs split across the `prose-only` and `reg/struct` buckets
  (confirms `document_class` has no ISA slot); (c) cat 5↔6 indistinguishable (both near-empty).
- **The register-heavy-protocol trap (deepest finding):** 8 cat-1 *protocols* are register/message-dominant — all
  4 CCIX (reg 131–143), AXI (reg 71 but rel 348), CHI (msg 106), DTI (msg 159), CHI-C2C (reg 81/msg 210). So "has
  registers ⇒ register-IP" is wrong; the dominant surface is not the purpose. The `.3` recognizer must weigh
  wire-relation shape + front-matter/self-declared type + topology cue, not counts (ADR-0006, no name lists).
- Report `docs/research/document-intent-category-census.md`; KM card `document-intent-category-census`.
- **Gates:** read-only, no code, no canonical mutation → WIRE-BASED-100 + register/wire golds + `kg-bench` orthogonal;
  memory-arch + knowledge-map gates green. Frontier → `.2` per-category ISF-completeness gauge.

### DOC-INTENT-TAXONOMY.0 — define the 6-category chip-spec intent taxonomy + capture it (mdBook + task-tree)
New owner-directed tree (`2026-06-22`, multi-message): every chip-spec PDF is *about* something — a small set of intent
categories — and SpecForge must understand all variations, build IntentIR, and lower EVERYTHING to ISF (ISF is the way to
synthesize the PDF intent); in fine all categories must be FULLY handled. `.0` is docs-only (no code).
- **The 6-category purpose taxonomy** (the guiding lens — what a PDF is *about*, with the honest per-category ISF-synthesis
  maturity): (1) wire-level bus/interconnect protocol — MATURE (WIRE-BASED-100 1.000); (2) programmable register/memory-mapped
  IP — PARTIAL (registers→ISF storage/reset; structure/message-field table recall is the frontier, e.g. the IOMMU #21 Lever-D);
  (3) platform/system-IP topology & integration — PARTIAL; (4) CPU ISA/privileged-arch — THIN (least-developed ISF story);
  (5) physical/electrical/link layer — honest-thin non-target; (6) methodology/language/EDA-standard/guide — non-target.
- **Captured identically** in the mdBook (new page `docs/book/src/document-categories.md`, added to `SUMMARY.md` after
  Architecture Rationale) and the task-tree `docs/tasks/DOC-INTENT-TAXONOMY.md`; tree registered in `docs/TASK_TREE.md`.
- **Relationship to `document_class`:** the existing structural 4-way class (`protocol`/`register`/`interface`/`guide`) is a
  coarse proxy this tree builds on — it folds 1+5→protocol, 2+3→register/interface, has no ISA slot, sends 6→guide.
- **FSMGen feedback (owner context):** FSMGen is now adding a verification-oriented SV/UVM + VHDL lowering path alongside the
  default synthesizable HDL; new ISF abstractions (memory banks, single/dual-port memory modules, …) are anticipated for both →
  where the current ISF can't capture a category naturally/elegantly, the gap is fed back to FSMGen as a verified feature
  request, never hacked into the emitter.
- **Frontier:** `.1` corpus census by category (read-only) → `.2` per-category ISF-completeness gauge → `.3` fast category
  recognizer (CODE) → `.4+` per-category levers + FSMGen feedback.
- **Gates:** docs-only — no extraction/emitter code → WIRE-BASED-100 + register/wire golds + `kg-bench` orthogonal; mdBook
  builds; memory-arch + knowledge-map gates green.

### CORPUS-COVERAGE.2 — re-ingest #21: RISC-V IOMMU Architecture Spec (108pp) — register/arch refresh + honest "already-current" finding
Fresh-session PNT slice (21 of 57 normalized-missing docs done), register/TRM/ISA phase.
- **Re-ingest:** Docling CPU, 108 pages / 196 visual / 0 residuals / confidence high; RAM steady 76–78% free, `.4a`
  guard armed, Ollama idle. Cascade `evidence`→`semantic`→`intent`→`adapt --target isf` on the current binary.
- **Honest finding — evidence was ALREADY current-binary-equivalent (NOT pre-`.10` stale):** register_records 33 held /
  147 fields, signal_constraints 6→8, conditional_rules 47; source_ir near-identical to the retained Jun-8 capture
  (13-byte diff), IntentIR unchanged (12 actors / 628 constraints / 608 behaviors). The Jun-15 stale evidence had been
  rebuilt from the retained `source_ir.json` before the `normalized/` page-image bundle was disk-reclaimed, so it already
  carried the `.10` register families → re-ingest's value here is **normalized-bundle restoration + current-binary
  confirmation**, not a marquee `.10c`/`.10g` jump (distinct from the genuinely pre-`.10` SMMU/GIC/MMU docs).
- **`.isf`:** renderable (`agent.isf`, 175 signals / 33 storage(reset) / 5 enums / 45 rules); real `fsmgen --strict
  --check --json` **success / 0 diagnostics** (strict-clean). `validate`: no stage-staleness (0-vs-0 honest absence);
  IntentIR quality 3/100 INCOMPLETE — honest for a memory-mapped register/structure spec with no wire-signal grounding.
- **Honest absences:** 0 relations / 0 interfaces (intent lives in registers); 0 `message_field_records` — of 83
  structured tables the IOMMU's device-context / command-queue **STRUCTURE** tables don't match the
  `.10b`/`.10d`/`.10e` two-column families → a surfaced **RISC-V structure-table recall opportunity** (Lever D), not a
  regression; 0 transactions (command vocabulary not in the section-heading recognizer — honest, like TileLink).
- **Gates:** no extraction code change → WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal; memory-arch
  + knowledge-map gates green.

### CORPUS-COVERAGE.2 — re-ingest #20: GIC-400 TRM (`ddi0471`, 57pp) — healthy register/TRM-phase refresh
Fresh-session PNT slice (20 of 57 normalized-missing docs done); first of the register/TRM phase pivot after the two
thin non-AMBA protocol findings.
- **Re-ingest:** Docling CPU, 57 pages / 38 visual / 0 residuals; RAM steady 74–77% free. Cascade: `evidence` (873
  statements) → `semantic` → `intent` → `adapt`.
- **Result (healthy, NOT thin):** 5 of 25 Docling tables classify as `signal_description` → a 16-signal
  AXI-slave-interface `.isf` (`a_4_axi_slave_interface.isf`) with **5 transactions + 4 storage (reset) + 6 relations**;
  real `fsmgen --strict --check` success / 0 diagnostics; `validate` no stage-staleness, score 56/100.
- **Registers modest:** register_records 3→4 only — GIC-400's registers largely sit in 14 `unknown`-classified tables
  (an older/different table style than GIC-600's `.10c`-shaped 15→33), a minor Lever-D recall opportunity rather than the
  marquee `.10c` gain. Confirms the register/TRM phase yields healthy multi-signal `.isf` even when the big `.10c` jump
  doesn't apply to a given doc.
- **Gates:** no extraction code change → WIRE-BASED-100 + register/wire golds + `kg-bench` orthogonal; memory-arch +
  knowledge-map gates green.

### CORPUS-COVERAGE.2 — re-ingest #19: OpenCAPI 4.0 TL Arch (240pp) — honest table-recognition-gap finding
Fresh-session PNT slice (19 of 57 normalized-missing docs done). OpenCAPI 4.0 Transaction-Layer Architecture re-ingested
with the current binary; like Wishbone, the result is an honest extraction-gap measurement, not a surface gain.
- **Re-ingest:** Docling CPU, 240 pages / 376 visual / automation_confidence high / 0 residuals; RAM steady 73% free.
  Cascade: `evidence` (3170 spans / 3528 statements) → `semantic` → `intent` (797 free-text constraints) → `adapt`.
- **Finding:** of **246** fresh Docling tables, **0 classify as `signal_description`** (219 `unknown` / 20 encoding /
  7 feature_matrix) → relations 17→0, signal_constraints 8→0, 0 message_field_records; `.isf` THIN (`channel.isf`, 1
  signal), real `fsmgen --strict --check` success / 0 diagnostics; `validate` no stage-staleness (0-vs-0), score 38/100.
  IntentIR still captures 797 free-text constraints + 1 transaction + 12 actors.
- **Regression ruled out (rigor):** verified the current binary still produces DTI 159 message-fields / MMU-700 63
  registers / AHB 66 relations — so the thin OpenCAPI/Wishbone results are doc-style specific, not a code regression.
  OpenCAPI TL is a packet/command-layer spec whose tables don't match the AMBA `Signal|Direction|Width|Description`
  shape; the 219 `unknown` tables are the recall opportunity → Lever D family (non-AMBA table recognition), spun-out,
  not fixed in-slice.
- **Phase pivot:** the high-value AMBA-style protocol specs are now exhausted (#17–#19 are thin non-AMBA); the next
  re-ingest phase targets the register/TRM/ISA docs where the `.10` register families demonstrably fire.
- **Gates:** no extraction code change → WIRE-BASED-100 + register/wire golds + `kg-bench` orthogonal; memory-arch +
  knowledge-map gates green.

### CORPUS-COVERAGE.2 — re-ingest #18: Wishbone B4 (`wbspec_b4`, 128pp) — honest signal-recall-gap finding
Fresh-session PNT slice (18 of 57 normalized-missing docs done). Wishbone B4 re-ingested with the current binary and
cascaded deterministically; the result is a valuable honest extraction-gap measurement rather than a surface gain.
- **Re-ingest:** Docling CPU, 128 pages / 356 visual assets / automation_confidence high / 0 residuals; RAM steady
  72–78% free (`.4a` guard armed, Ollama idle). Binary already current (no rebuild). Cascade: `evidence` (2005 spans /
  2041 statements) → `semantic` → `intent` → `adapt --target isf`.
- **Finding:** the fresh evidence carries **0 interfaces / 0 signal_records → 0 actor_signal_relations** (the lone
  stale relation was a fragment the `.1a` gate drops — not a regression; the stale build also had ~1). IntentIR still
  captures **215 free-text constraints + 2 transactions + 8 actors**, so the intent is present as obligations/behaviors,
  just not as the typed wire surface. `.isf` renders but THIN (`arbiter.isf`, 1 signal); real `fsmgen --strict --check`
  → **success / 0 diagnostics**; `validate` → no stage-staleness (0-vs-0 honest absence), score 42/100.
- **Root cause (read-only probe):** Wishbone documents its signals in the `SIGNAL_O()`/`SIGNAL_I()` suffix-notation +
  prose signal-list style, which the current signal-table/prose extractors don't recognize. This SURFACES an upstream
  signal-recall lever (Lever D) for the `PDF-VARIANT-DIGESTION` family (kin to the parked `.9.10` prose-bus-line lever) —
  NOT an emitter lever, and per the tree rule NOT fixed inside a re-ingest slice (it needs its own owned leaf with the
  wire-based-100 gate). The re-ingest sweep is doing exactly its job: surfacing which doc styles extract well vs poorly.
- **Gates:** no extraction code change → WIRE-BASED-100 + register/wire golds + `kg-bench` orthogonal by construction;
  memory-arch + knowledge-map gates green.

### CORPUS-COVERAGE.2 — re-ingest #17: Avalon Interface Spec (`683091`, 63pp) — current-binary refresh
Fresh-session PNT slice continuing the corpus re-ingest batch (17 of 57 normalized-missing docs done). The Intel
Avalon Interface Specification reached IntentIR back in `.0` but its EvidenceIR was stale (built Jun 7, before the
`.10`/`.12`/`.2` extractor families and the `.1a`/`.1b` agent-identity gates landed). Re-ingested with the current
binary and cascaded deterministically.
- **Setup:** rebuilt the release binary first (it was stale — `isf_ir.rs` newer than the prior build; `CARGO_BUILD_JOBS=2`,
  1m30s) so the cascade ran HEAD code. Docling CPU re-ingest: 63 pages / 192 visual assets / automation_confidence
  high / 0 residuals; RAM steady 77–78% free throughout (built-in `.4a` guard armed at 85% used, Ollama idle).
- **Cascade:** `evidence` (712 spans / 823 statements) → `semantic` → `intent` → `adapt --target isf`.
- **Refresh result (after vs stale before):** relations **126→111**, actors **64→50** — the current `.1a`/`.1b`/`.1b.iv`
  agent-identity consolidation gates fold the fragment/phantom actors the stale evidence still carried (a cleaner KG,
  not a loss); register_records 8 held; transactions 5 held. **Honest absence of message-field/presence surfaces** —
  Avalon is a prose/diagram interface spec with no register-field/message-field/presence tables, so the `.10b`/`.10f`/`.12`
  families correctly produce nothing (no fabrication).
- **`.isf`:** renderable (`source.isf`, 26 signals / 8 storage / 7 enums / 1 transaction body); real
  `subs/fsmgen/bin/fsmgen --strict --check --json` → **success / 0 diagnostics**. `validate` reports **no stage-staleness
  warning** (fresh cascade; intent carries 111 relations).
- **Gates:** no extraction code change (re-run of existing deterministic extractors; `generated/` is git-ignored, so the
  durable trace is the `.2` log table row) → WIRE-BASED-100 + register/wire golds + `kg-bench` orthogonal by construction
  (the 4 gold docs are not re-ingested). 15 of 17 re-ingested docs are strict-clean; the 2 strict-FAIL (DTI width-align
  Lever A, LPI rule-conflict Lever C) are pre-existing spun-out emitter levers, unrelated to Avalon.

### ISF-VALUE-WIDTH-EMIT.2 — emit value-width-aligned ISF literals + complete width recovery (CODE; TREE CLOSED)
The emit slice landed once the host had RAM headroom (81% free by `memory_pressure`), closing the
`ISF-VALUE-WIDTH-EMIT` tree. Two faithful, ADR-0006 numeric-only fixes in `ir/isf_ir.rs`, no name list:
- **Width recovery completeness.** A new `interface_widths` aggregate collects the single unambiguous concrete
  (`WidthHint::Numeric > 1`) width across **all** of a signal's `interfaces[].signal_records` (a conflict keeps
  the honest width-1 default, never a guess) and is chained ahead of the `.2a.i` `port_widths` in the
  signal-width fallback (`interface_widths → port_widths → 1`). This recovers a concrete width that lives in a
  NON-FIRST interface record, which the first-seen dedup otherwise dropped — e.g. trace-bus `ATID` (width 7 in
  its 3rd record, no actor-port). Live: `ATID` now emits `(output ATID (width 7))` (was width 1).
- **Value-literal width-alignment or honest residual.** A new `align_rule_drive_widths` post-pass (before
  `dedup_conflicting_rules`) with helpers `parse_sized_literal` / `align_value_to_width` / `ValueAlign` /
  `value_width_residual_packet`: a based value literal (`0b…`/`0x…`) whose notation width ≠ the signal's emitted
  width is re-rendered as a width-cast `W'd<v>` when `value < 2^W`, else the whole rule is DROPPED with an
  explicit `isf_value_width_*` residual (the `ISF-RULE-CONFLICT-RESIDUAL` pattern) — never truncated. Bare
  decimals (FSMGen-unsized), enum symbols, references, and already-matching literals are left byte-identical.
- **Verification (real `subs/fsmgen/bin/fsmgen --strict --check --json`):** DTI `(ATST 1'd1)` (was `2'b1`) and
  trace-bus `(ATID 7'd125)` (was `8'h7D` on width 1) both now report `has_diagnostics: false` — the
  OperandContract value-width error is CLEARED on both target documents. The 4 wire golds carry **0 NEW**
  diagnostics: APB 0/0 and SWD 0/0 unchanged; AHB 1/1 and AXI 1/1 are the pre-existing **orthogonal**
  `isf_conflicting_rule_writes` (HAUSER/ASKSTOP) and `(port expr)` (constraint_33) issues, count unchanged.
- **Gates:** 47/47 `isf_ir` unit tests (incl. 3 new + 5 `*_passes_fsmgen_strict_validation` canaries); full
  suite **1682 passed / 0 failed**; `scripts/run_ci.sh` GREEN (memory-arch + knowledge-map + fmt + clippy
  `-D warnings` + rustdoc + mdBook); `kg-bench` **156/156**. WIRE-BASED-100 orthogonal by construction
  (emitter-only — it measures extraction F1, not `.isf` bytes). The AXI `(port expr)` grammar lever and the DTI
  ATST upstream mis-attribution stay spun out (tree Non-Goals). Book close-rule subsection added to
  `docs/book/src/pipeline/isf-adapter.md`.

### ISF-VALUE-WIDTH-EMIT.0/.1 — own + measure ISF value-literal width-alignment (measurement-first, docs-only, GO)
A new `ISF-*-EMIT` emitter-fidelity tree, spun out of the `CORPUS-COVERAGE.2` re-ingest sweep as "Lever A",
and picked as the RAM-light PNT slice because the host is a 6.3 GB machine at ~16% free RAM where a Docling
re-ingest (the other open thread) is unsafe. The ISF emitter copies a `(rule … (SIGNAL value))` value literal
verbatim (`render_isf_control_expression`, `ir/isf_ir.rs:1493-1495`) and never reconciles its width against the
signal's declared `(width N)`; FSMGen strict `--check` (`OperandContractValidationSupport.pm`) rejects a literal
whose **notation width** (digit count — `0x7D`=8, `0b00`=2, **not** value) ≠ the LHS width, requiring an
"explicit width-aligned source expression" (no implicit truncation).
- **Measurement (read-only over the 86 persisted `.isf` + intent_ir + real `subs/fsmgen/bin/fsmgen --strict
  --check` probes, Perl — RAM-safe, no cargo build):** 4 docs / 13 clauses — DTI ATST ×3, AXI+ACE
  ARTAGOP/BTAGMATCH ×6, AXI-gold AWCMO ×1, trace-bus ATID ×3. **DTI + TRACE confirmed FAIL** on OperandContract
  (DTI: *"assignment to 'ATST' uses RHS '2'b1' with incompatible width 2 for LHS width 1"*). The AXI docs fail
  FIRST on the orthogonal `(port expr)` grammar error, masking their width cases.
- **Key finding — the naive "truncate the value" fix is DISHONEST.** All four signals are emitted at `(width 1)`
  **even when the IntentIR grounds a wider width** (ATID 7 / ARTAGOP 2): the first-seen signal dedup
  (`isf_ir.rs:696-700`) takes the first `signal_records` entry (`w=None`→1) and **skips** the later
  concrete-width record, and `.2a.i` recovery only checks `actor_ports` (ATID has none). The over-width literal
  is a symptom; ATID `0x7D`=125 is a genuine 7-bit trace-ID value.
- **GO — probe-validated fix design (`.2`):** (1) recover the grounded width across **all** interface
  `signal_records` + `actor_ports`; (2) re-render the value literal width-aligned as `W'<radix><digits>` when
  `value < 2^W`, else **residualize** (the `ISF-RULE-CONFLICT-RESIDUAL` pattern) — never truncate. Bare decimals
  are unsized and untouched. ADR-0006 numeric-only, no name list. After the fix: ATID → `(width 7)` +
  `(ATID 7'd125)` (PASS), ARTAGOP → `(width 2)` + `(ARTAGOP 2'b00)` (PASS).
- **Spun out:** DTI ATST **upstream mis-attribution** (source text's `0b01` belongs to `ATTR_OVR.SHCFG`; ATST is
  a value of FLOW, not the constrained signal) + the AXI `(port expr)` grammar lever.
- **Wire-gold blast radius:** only AXI `ihi0022_l` carries one (AWCMO, already failing on `(port expr)`);
  WIRE-BASED-100 measures extraction F1, not `.isf` bytes → orthogonal by construction.
- **Gates:** docs-only — `scripts/check_memory_architecture.sh` + knowledge-map derive-and-diff GREEN
  (110 facts / 772 keys); no Rust/CI/`.isf` change; WIRE-BASED-100 + `kg-bench` untouched by construction. `.2`
  is compile-gated → HELD until the host has RAM headroom. Report
  `docs/research/isf-value-width-alignment-measurement.md`; KM card `isf-value-width-operand-contract`.

### KG-ISF-COMPLETENESS.2a.iii — ISF module-name HDL-sanitization (CODE, owner-chosen) — a fragment-actor name no longer breaks the whole `.isf`
The emitted `.isf` module name (and every internal `.isf` identifier) is now HDL-sanitized so a prose-fragment
initiator actor name carrying punctuation no longer malforms the entire file. Surfaced by `CORPUS-COVERAGE.2`
re-ingest #14: GIC-600's net-producer initiator was the phrase `redistributor → distributor distributor →
redistributor`, and the emitted `(actor …)` header kept the unicode arrow `→`, so FSMGen `--strict --check`
rejected the whole `.isf` with `Malformed top-level FSM source … expects '?fsm:name' with an HDL-identifier-
compatible module name ([A-Za-z_]\w*)`.
- **Allowlist, not denylist:** `sanitize_isf_name` (`ir/isf_ir.rs`) was a char denylist of ASCII punctuation —
  which can never enumerate every offender and in fact missed `→`. It is now an allowlist: lowercase, keep
  `[A-Za-z0-9_]`, map every other char to `_` (then the existing collapse/trim/empty/leading-digit guards) —
  exactly FSMGen's `[A-Za-z_]\w*` contract, universal, no name list (ADR 0006; aligns with the owner's
  prefer-structural-over-denylists steer). `derive_isf_actor_name` (`ir/adapters.rs`) now routes the module
  label through the same shared sanitizer (one rule, no drift).
- **Safe by construction:** the allowlist is byte-identical to the old denylist on every ASCII-punctuation input
  it already covered, so the 4 wire-gold `.isf` are byte-identical (`Manager`→`manager`, `Requester`→`requester`,
  `debugger`); and sanitizing the module LABEL cannot affect initiator port matching (`from_intent_ir` re-derives
  the initiator raw internally; `actor_name` is only the label).
- **Verified:** GIC-600 `.isf` header → `(actor redistributor_distributor_distributor_redistributor`, FSMGen
  `success=true` / **0 diagnostics** (whole file now lowers strict-valid); wire golds show **0 NEW diagnostics**
  (APB/SWD pass; AHB/AXI only their pre-existing `HAUSER`/`ASKSTOP` rule-conflicts). `run_ci.sh` GREEN (lib 1679);
  `kg-bench` 156/156; WIRE-BASED-100 orthogonal. KM `isf-module-name-hdl-sanitization`; book `pipeline/isf-adapter.md`.

### CORPUS-COVERAGE.2 — re-ingest frontier unblocked: host-local library via git-ignored symlink (ownership/provisioning)
Owner re-provisioned the host-local spec library (`chipdoc`, 88 PDFs, permanent) to attack substantive gap #2
(corpus coverage). Chosen provisioning mechanism: a **git-ignored symlink** `.cache/local-references/chipdoc`
rather than copying ~150 MB of PDFs into tracked `corpus/` (which would permanently bloat git history). `/.cache/`
was added to `.gitignore`, so the owner's absolute library path is never recorded in any tracked file
(`feedback_source_pdfs_in_repo`); tracked docs cite only the repo-relative `.cache/local-references/chipdoc/...`
path. The 22 gold/measured docs stay copied in `corpus/` for the reproducible WIRE-BASED-100/eval path.
- **The substantive win:** all 57 normalized-missing docs already reach IntentIR (via `CORPUS-COVERAGE.0`), but
  their EvidenceIR is STALE — built before the `.10a`–`.10g` register/message-field families, the `.12a`/`.12b`
  presence records, and the `.2a`–`.2m` transaction recognition landed. RAM-guarded per-doc re-ingest with the
  current binary surfaces all that new typed intent → more complete KG/IntentIR → more faithful `.isf`.
- **Method:** PNT, one doc per slice (protocol specs first — CXS/GFB/ACC/ATP/TileLink/LPI/DTI/CHI-C2C — then
  register/TRM/ISA docs), `DOCLING_DEVICE=cpu`, the built-in `.4a` RAM guard active (clean abort at ≥85% used),
  Ollama idle, RAM+swap monitored, commit per `COMMIT.md`. No extraction code change (re-runs existing
  deterministic extractors); WIRE-BASED-100 + register/wire golds + `kg-bench` 156/156 orthogonal (the 4 gold
  docs are not re-ingested). Ownership/provisioning slice — frontier now active under `CORPUS-COVERAGE.2`.

### KG-ISF-COMPLETENESS.2a.ii — initiator-perspective signal DIRECTION in the emitted `.isf` (CODE, owner-authorized)
The emitted `.isf` interface now lowers the document-grounded, actor-relative signal DIRECTION from the protocol's
INITIATOR actor's perspective, instead of defaulting non-`Input` signals to `(output)`. A signal the initiator
drives → `(output)`, reads → `(input)`; an ungrounded signal stays `(output)` (honest residual). This is a
north-star faithfulness fix (a signal the perspective actor reads must be `(input)`); the owner authorized it
(AskUserQuestion, `2026-06-17`: "Build it, initiator perspective"), satisfying the explicit re-open trigger the
`KG-ISF-COMPLETENESS.2a` deferral recorded.
- **Initiator identification (structural, ADR 0006 — no name list):** `select_initiator_actor` (`ir/isf_ir.rs`)
  picks the **net-producer** actor — output (`Drives`) ports strictly exceed input (`Reads`) ports — maximizing
  `(outputs, inputs)` lexicographically. `out > in` excludes balanced prose-fragment actors (AHB `address decoder`)
  and input-dominant completers (`Subordinate`/`Completer`); the `(out, in)` tiebreak prefers a real initiator
  (reads responses) over an output-only register fragment. No net producer → no initiator → the prior
  default-`output` behavior (byte-identical, honest residual). Validated per-item: AHB `Manager` (6/2), APB
  `Requester` (20/12), AXI `Manager` (116/52), SWD/debug `debugger` (2/1).
- **Direction + module name:** `initiator_perspective_directions` builds the initiator's per-signal map
  (`Drives`→`(output)`, `Reads`→`(input)`; a both-driven-and-read / `InOut` / `Unknown` signal omitted → residual);
  the signal loop prefers it, else the flat hint, else `(output)`. `derive_isf_actor_name` (`ir/adapters.rs`) names
  the module after the same initiator so the label and its interface are coherent (AHB `address_decoder`→`manager`,
  APB `apb_protocol`→`requester`, AXI `agent`→`manager`, SWD `agent`→`debugger`).
- **Strict-safe by construction:** the per-output named-drive block is already filtered to `IsfDirection::Output`,
  so a signal flipped to `(input)` is automatically NOT driven (FSMGen rejects driving an input). Verified:
  **0 NEW `--strict --check` diagnostics on all 4 wire docs** (baseline-vs-after via `git stash`; AHB `HAUSER` + AXI
  `ASKSTOP` pre-existing rule-write conflicts unchanged; APB/SWD still PASS).
- **Measured flip (`2026-06-18`):** APB input **2→12**, AXI input **4→52**, SWD input **0→1** (faithful — e.g. APB
  Requester now reads PRDATA/PREADY/PSLVERR and drives PADDR/PWDATA/PWRITE/PSEL/PENABLE); AHB input **0→0** (honest
  residual — the stale persisted intent grounds Manager only to sideband outputs + clock/reset inputs, so the rich
  HADDR/HREADY/HRDATA signals carry no Manager relation and stay `(output)`; a fresh post-`.1a` rebuild flips more).
- **Gates:** `run_ci.sh` GREEN (lib 1677→1679, +2 tests); `kg-bench` 156/156; WIRE-BASED-100 orthogonal
  (emitter-only, downstream of all extraction); ADR-0006; honest residual over fabrication. KM card
  `docs/knowledge/isf-initiator-perspective-direction.md`; book `pipeline/isf-adapter.md`.

### KG-ISF-TRANSACTIONS.2n — the transaction ISF BODY is faithfully complete (measurement-first, read-only, docs-only — NO-GO)
A fresh probe-first cycle on the current FSMGen pin `030f8c273`, run because the `2026-06-17` resume-pointer triage
named the transaction ordered multi-phase body "the substantive buildable critical-path transaction lever." The
honest outcome is **NO-GO**: the transaction body is already faithfully complete, and the one unbuilt candidate (a
value-free `(sample …)` membership body) is FSMGen-accepted but not faithful.
- **Empirical (`subs/fsmgen/bin/fsmgen --strict --check --json`, pin `030f8c273` — verified on the binary, not the
  book):** (A) a pure `(transaction read_transfer (on start (sample HREADY as r)) (complete done))` over a declared
  `(input HREADY …)` → `success:true`, 0 diagnostics; (D) three samples in one `(on start …)` state → `success:true`
  (order-free among themselves); (B) `(sample HTRANS as t)` over an interface `(output …)` → `success:true`
  (`(sample …)` is NOT direction-gated); (C) an in-body `(drive HTRANS 0)` with no top-level named drive →
  `success:false`, `drive 'HTRANS' not defined` (an in-body `(drive NAME …)` is a CALL to a top-level named drive,
  which is exactly why `.2b`'s enum-selector drive renders — the emitter also emits the top-level drive block).
- **Why FSMGen-accepted ≠ faithful (the NO-GO reason):** the FSMGen book (`13b-transactions.md`, `(on port ...)`
  Entry/Idle State, L133–163) shows a sample inside `(on …)` fires AT THE ENTRY TRANSITION — "Cycle N:
  `port && can_accept` → samples captured" — and `(on …)` needs an activation guard `port`. For a recognition-only
  transaction SpecForge has grounded only WHICH signals participate, NOT that they are captured once at the entry
  cycle, and `mint_named_transaction` sets `activation_port: None`. So a membership-derived
  `(on start (sample HREADY))` would assert an un-grounded entry-cycle capture + a `start` guard the document never
  states, and could misrepresent a wait-for-ready read as a one-shot entry sample — the fabrication the
  honest-residual doctrine and FSMGen's `2026-06-16` phase-membership answer forbid ("emit body steps only for facts
  whose value AND ordering are grounded; keep membership as metadata").
- **Decision:** the body lever is exhausted. The only body-lowerable grounded fact (the enum-selector `(drive)`)
  already ships (`.2b`); the membership a sample body would carry already lives faithfully as IntentIR metadata
  (`.2c` ports / `.2i` phase / `.2m` channel), the home FSMGen explicitly chose. The cross-`.isf` carriage of that
  membership awaits FSMGen's future checked transaction phase-group metadata surface (FSMGen-owned, not shipped).
  This reconfirms the `.2i` parking with FRESH evidence on `030f8c273` (the prior probe was at `8c39827f`).
- **Gates:** read-only/docs-only — no code; WIRE-BASED-100 untouched; ADR-0006 (universal grammar, no name list);
  `scripts/check_memory_architecture.sh` + the knowledge-map derive-and-diff green. KM card
  `docs/knowledge/transaction-body-emission-faithfully-complete.md`. The next SUBSTANTIVE north-star fidelity lever
  is OUTSIDE this tree — interface signal DIRECTION emission (`KG-ISF-COMPLETENESS.2`, the ~98%-defaulted-`output`
  gap), which is owner-design-gated (the single-flat-module actor-perspective decision).

### PDF-VARIANT-DIGESTION.10g — section-HEADING register-field recognizer (the register-routed twin of `.10f`)
The same `<NAME>, bits [hi:lo]` section-heading field layout that DTI uses for MESSAGE fields is how ARM
ARCHITECTURE specs (not TRMs) lay out REGISTER fields — GIC (`ihi0069`), SMMU (`ihi0070`), CoreSight
(`ihi0029`), ACC (`ihi0076`), ARM-Debug-v6 (`ihi0074`) give each register a dotted-numbered container heading
(`B2.2.1 ABORT, Abort register`, `6.3.1 SMMU_IDR0`) with a `Field descriptions` anchor and one heading per
field. `.10f` routed those register containers away (deferred to `.10g`); `.10g` reads them into `register_records`.
- **No-drift design:** factored `.10f`'s container-walk into ONE shared classifier
  (`scan_section_header_field_containers` → `{name, is_register, has_anchor, fields}`) + a shared field gate
  (`distinct_section_header_fields`), so the register-vs-message routing is decided in exactly one place. `.10f`
  keeps non-register containers (byte-identical, proven by its 7 tests + the parity sweep), `.10g` keeps the
  register-routed ones. New strategy `registers.section_header_field` runs LAST in `register_record_surface`
  (`run_surface_concat`); access/reset/offset/description honestly absent (a heading states only name + bit range).
- **Per-document name-uniqueness residual gate (decisive precision lever):** a short register mnemonic is reused
  across access-port blocks (ARM-Debug `AUTHSTATUS`/`CSW`/`IDR`/`CLAIMSET`/`DEVARCH`; CoreSight `AUTHSTATUS`),
  the dotted heading carries only the short name, and the occurrences are a MIX of identical cross-references,
  subset views, and GENUINELY DIFFERENT registers (`CSW` MEM-AP vs JTAG-AP have disjoint fields). So a name reused
  across ≥2 register containers is structurally ambiguous → held as an honest residual, never over-counted nor
  conflated by the existing all-distinct fragment merge into a fabricated mega-register. Universal grammar, no
  name list (ADR 0006). A unique name matching an existing 0-field record (e.g. ARM-Debug `DPIDR`) MERGES via
  `consolidate_register_field_fragments` (no double-count; existing identity kept since `.10g` runs last).
- **Measured:** `register_records` rise on exactly 5 architecture specs — GIC 73/468, SMMU 88/381, CoreSight 5/24,
  ACC 2/4, ARM-Debug 12/57 = **180 registers / 934 fields**; a full ARM-Debug `evidence` rebuild goes 29 → 37
  register records (+8 brand-new; `DPIDR` enriched to 4 fields; ZERO duplicate names). ONLY these 5 fire; DTI emits
  0 registers and keeps its 159 `.10f` message fields.
- **Gates ALL GREEN:** old-vs-new `evidence --dry-run` over the wire gold + register golds (NVMe 42 reg/216 msg,
  AMD 217 msg, CCIX 131 reg/92 msg, RISC-V Debug 44 reg, APB/AHB/AXI/AXI-Stream) byte-identical except the run
  manifest's new `registers.section_header_field` entry (produced 0) → WIRE-BASED-100 + register golds provably
  orthogonal; `.10f` message fields byte-identical; `kg-bench` 156/156; full `scripts/run_ci.sh` GREEN (lib
  1672 → **1677**, +5 hermetic tests + 1 `#[ignore]` corpus-sweep measurement). Book `pipeline/evidenceir.md`
  `.10g`; KM `section-header-register-field-extraction`. The block-ambiguous reused-mnemonic registers are the
  honest residual a future block-qualified lever can recover.

### PDF-VARIANT-DIGESTION.10f — section-HEADING prose message-field recognizer (DTI-class message protocols)
Spun from `KG-ISF-COMPLETENESS.4`'s §spun-out gap. AMBA DTI (`ihi0088`) is a message protocol but carried
**0 `message_field_records`**, so field obligations (`the MMUV field must be 0`) leaked into `signal_constraints`
with dotted/undeclared subjects. The `.10a`–`.10e` readers all read TABLES; DTI's field layout is not in tables.
- **Scoping correction (measurement-first):** the spun-out note guessed prose `list_item`s `"<Field>, bit [N]"`;
  re-measuring the persisted `source_ir.json` showed DTI defines each field as its own **section HEADING**
  (`STAGES, bits [27:26]` / `SPD, bit [25]` / `M_MSG_TYPE, bits [3:0]`) under a dotted-numbered message container
  (`3.1.1 DTI_TBU_CONDIS_REQ`) with a `Field descriptions` anchor — a cleaner anchor than free prose (`list_item`
  matches ~0; `body_text` matches are descriptive prose/cross-refs, excluded by the section-heading restriction).
- **Build:** additive `extract_section_header_message_fields` (`ir/evidence.rs`) registered as a 4th strategy
  `message_fields.section_header_field` in `message_field_surface`, reading `source_ir.document_sections`. The SAME
  heading shape describes register fields in TRMs (GIC 612 / SMMU 434 / CoreSight 42 / ACC 5 / ARM-Debug-v6 138),
  so **container-decides routing** (the `.10b`/`.10c`/`.10e` rule — register iff caption-`register` /
  `Attributes` / `Accessing`, else message/structure) keeps register fields out of the message surface (deferred
  to a sibling `.10g`). Two ADR-0006 name cleanups from the live eyeball: a Docling spacing artifact
  `<ident> [slice]` normalizes to `<ident>[slice]` (one record); the bare unit word (`Bits, bit [5:4]`) is an
  unnamed reserved range → dropped (honest residual). Overlapping Manager/Subordinate views of one position
  (`M_MSG_TYPE[3:0]`/`S_MSG_TYPE[3:0]`) are both kept. ADR-0006: universal section grammar, no chip-name list.
- **Measured:** DTI `message_field_records` **0 → 159 / 17 containers**; **only DTI fires (1/79)**;
  GIC/SMMU/CoreSight/ACC/ARM-Debug-v6 section-heading fields are register-routed → 0 message fields.
- **FIELD.4 (honest):** closing the recognition gap lets the LLM-primary constraint reader type a DTI field
  obligation into `message_field_constraints` (`LLM-PRIMARY-PROMOTION.5`); the deterministic Pattern
  `signal_constraints` surface does not consult the catalog, so the persisted leak is corrected on the next
  live-NLP DTI rebuild (its normalized bundle is host-local-blocked).
- **Verification:** lib tests 1664→1672 (7 new: DTI-shape positive, register-container exclusion, anchor+≥2 gate,
  M/S overlap kept, spacing-normalize+unit-word drop, two parse-form unit tests, surface field-id+manifest;
  +1 `#[ignore]` corpus-sweep). **Old-vs-new `evidence --dry-run` parity over 7 intact docs (4 wire gold + NVMe 216
  + AMD 217 + ARM-Debug-v6): every surface byte-identical, ONLY delta = the manifest gaining
  `message_fields.section_header_field` (produced 0)** → WIRE-BASED-100 provably orthogonal. `kg-bench` 156/156;
  full `scripts/run_ci.sh` GREEN (fmt + clippy `-D warnings` + lib 1672 + rustdoc + mdBook). Book
  `pipeline/evidenceir.md` `.10f`; KM `section-header-message-field-extraction`.

### KG-ISF-COMPLETENESS.4 — bar #5/#6 behavior/temporal lowering-completeness re-assessed on the broader 78-doc corpus (measurement-first, read-only, docs-only)
`.2` measured ISF-lowering fidelity over 36 IntentIR docs; `CORPUS-COVERAGE.0` then doubled the corpus to 78
(register/coherency/command/profile-heavy). This slice re-checks bar #5 (every behavior/temporal rule carried or
recorded as an explicit residual — no silent drop) + bar #6 (round-trip) at the new scale, the resume-pointer
standing candidate (b). No code change.
- **Method:** read-only Python faithfully replicating the three `IsfIr::from_intent_ir` `(rule)` filters
  (`crates/specforge/src/ir/isf_ir.rs`) over all 78 `intent_ir.json` — `signal_names` = `interfaces[].signal_records`
  names minus clock/reset; `conditional_rules`/`signal_constraints`/`temporal_invariants` each `continue`-skip with
  NO residual when the subject is empty/undeclared; `temporal_rules`/`actor_contracts` residualize.
- **78-doc breakdown:** `conditional_rules` 2237 total / 516 lower / 1670 no-consequent / **51 undeclared-named**;
  `signal_constraints` 369 / 287 / 0 / **82**; `temporal_invariants` 29343 / 286 / 29030 empty-subject / **27**;
  residualizing path `temporal_rules` 320 + `actor_contracts` 211. The 30 700 empty-subject/no-consequent drops are
  correctly silent ("absence is not an event").
- **The 160 undeclared-NAMED-subject drops** (the only genuine bar-#5 silent-loss candidate) were characterised
  item-by-item: 5 already on the register surface; the other 155 are **field content + noise, NOT wire intent** —
  register/message FIELD mnemonics (NVMe `MTFA`/`HMDLAL`, CCIX `SAMH`/`ESMD`, RISC-V `DC.tc.SXL`; homed on the field
  surfaces by design — the `.isf` does not lower fields), DTI message-field obligations leaked into
  `signal_constraints` (DTI carries **no `message_field_records`**), prose/hex noise (`DMA`/`TLB`/`PCI`/`IBM`/`FFFF`/
  `Reserved`/`this bit`), and the ATP `ihi0082` cluster of real AXI names from **garbled** VLM fragments
  (`"RREADY is RBR"`). **0 undeclared/silent drops on all four wire docs** (re-confirmed at 2× scale).
- **Conclusion: bar #5/#6 HOLDS on the broader corpus — no buildable lowering-residual lever.** The adapter's silent
  skip of an undeclared-subject rule is CORRECT (field obligations route upstream to the field surface; prose/VLM
  noise filters upstream — never residualize at the adapter, which would reintroduce exactly the noise `.2`/`.2b`
  rejected). CONFIRMS + extends `.2` to the doubled corpus.
- **Spun-out grounded observations** (each needs its own measurement-first ownership): (1) **DTI `ihi0088`
  message-field recognition gap** → `EXTRACTION-QUALITY-GAUGE.FIELD`/`PDF-VARIANT-DIGESTION` (most actionable next);
  (2) signal-inventory prose noise (`AMBA`/`ARM`/`APCI` minted as DTI signals); (3) ATP VLM-fragment quality.
- **Deliverables:** `docs/research/behavior-temporal-lowering-completeness-78doc.md`; KM card
  `behavior-temporal-lowering-broader-corpus` (KNOWLEDGE_MAP 104 facts / 719 keys). ADR-0006 (universal counts,
  structural classification, no name list). **Gates:** docs-only — WIRE-BASED-100 untouched (no code);
  `scripts/check_memory_architecture.sh` + knowledge-map derive-and-diff green.

### KG-ISF-TRANSACTIONS.2m — deterministic AXI-family channel-membership lever (CODE, measurement-first)
Builds the `.2l` Q1 finding into code: a recognized transaction's `.2c`/`.2k` signal-set membership is now grouped
by the **channel** the document declares each signal belongs to, recovered DETERMINISTICALLY (no VLM) from the
universal `<role> channel signals` table-caption cue. Channel membership is a DISTINCT typed dimension (the
document's own channel role kept verbatim — `write request`, `read data`, … — deliberately NOT re-interpreted into
abstract address/data/response phases, which would be AXI-family semantic knowledge, non-universal, and could
fabricate a phase the document never named for a signal); it fills the `.2i` AXI-empty `phase_membership`
deterministically.
- **EvidenceIR** (`ir/evidence.rs`, the only stage with BOTH the provenance and the SourceIR captions): new
  `SignalChannelMembershipRecord {signal_name, channel_role, table_ids}` + `EvidenceIr.signal_channel_memberships`
  (serde-skip-if-empty), built by `build_signal_channel_memberships` — `derive_channel_role` parses
  `[Table ]<number> <role> channel signal[s]` with a table-number grammar (`[A-Za-z]*\d+(?:[.\-]\d+)*`) that
  handles both the 2025 `B1.1:` colon and the 2021 `A2-2` dash forms (so a dash digit never leaks into the role; a
  role holding a digit/non-letter is rejected; bare `... channel` without `signals` is not a cue); continuation
  fragments (`B1.1 Continued from previous page`) are chained to their head's role by the caption's own table
  NUMBER; and an **ambiguity gate** (a signal earns a channel iff EVERY channel-captioned table that declares it
  agrees on one role — else dropped as an honest residual) enforces boundary precision (bar #3).
- **SemanticIR** (`ir/semantic.rs`): carries `signal_channel_memberships` forward (mirrors `transaction_anchors`).
- **IntentIR** (`ir/intent.rs`): new `TransactionChannelMembership {channel_role, ports}` +
  `TransactionIntent.channel_membership` (serde-skip-if-empty); `mint_named_transaction` groups the transaction's
  ports by channel role. Metadata only — NOT lowered to `.isf` (the emitter lowers `steps`; mirrors `.2i`).
- **validate** (`commands/validate.rs`, intent path): `transactions_with_channel_membership` +
  `transaction_channel_groups` metrics, an `intent_transaction_channel_membership` Info finding (non-empty-only),
  and a `with_channel_membership:` human-summary line.

Measured: 2025 AXI clean (154 signals → 8 channel roles, 0 ambiguous); live `validate` reports
`with_channel_membership: 5 (10 channel groups)` with `atomic_transaction [read data×4/write data×3/write
request×12/write response×3]` (multi-channel), `prefetch`/`writedeferrable` multi-channel — AXI's `phase_membership`
is 0 (its prose names no phase signal), so the channel grouping fills that gap. The 2021 AXI+ACE doc is
boundary-gated (31/105 ambiguous dropped → honest `unmapped`); AXI-Stream/APB/AHB/SWD have no channel captions →
empty surface (no fabrication). **Gates:** ADR-0006 ✓; **WIRE-BASED-100 PROVABLY ORTHOGONAL** — a `git stash`
HEAD-before-`.2m` vs HEAD-with-`.2m` AXI EvidenceIR diff shows the ONLY changed field is `signal_channel_memberships`
(154), all wire-gold surfaces byte-identical, and the emitted `.isf` is byte-identical (emitter `isf_ir.rs` lowers
`tx.steps` only — confirmed by adapting with vs without the metadata) ✓; `kg-bench` 156/156 ✓; `run_ci.sh` GREEN
(fmt + warning-deny clippy/tests/rustdoc + mdBook; lib **1664**, +2: caption-grammar keep/reject + the
continuation-chaining/ambiguity-gate builder test; one `collapsible_if` let-chain fixed) ✓. Book
`pipeline/intentir.md` "Grouping a transaction's signals by channel"; KM `transaction-channel-membership`.

### KG-ISF-TRANSACTIONS.2l — AXI/SWD per-signal phase membership: VLM-tier candidate RESOLVED measurement-first
Measurement-first, read-only + a bounded VLM probe, docs-only (owner-chosen fresh session for this design-heavy +
RAM-heavy slice). Closes the last `.2j`-recorded transaction candidate — "AXI/SWD timing-diagram phase columns
(VLM-tier)" — by MEASURING before any code and weighing the cheaper deterministic cue against the VLM, per the
scoring-rigor + structured-first/best-wins doctrines. **Q1 (AXI) → GO, but DETERMINISTIC (VLM superseded):** AXI's
transaction phases (address/data/response) ARE its channels, and the document's OWN caption-named tables declare
exactly the signals in each channel. Measured live over the persisted AXI EvidenceIR —
`table_signal_declaration_provenance` (411 entries) maps each signal to the channel table that declared it:
`B1.1 Write request channel signals`→26 `AW*` (write address phase), `B1.2 Write data`→15 `W*` (data),
`B1.3 Write response`→14 `B*` (response), `B1.4 Read request`→26 `AR*`, `B1.5 Read data`→18 `R*`,
`B1.6/B1.7`→snoop. So AXI per-signal phase membership is recoverable WITHOUT a VLM, via the universal caption cue
`<role> channel signals` (role→phase: request→address, data→data, response→response) — the document's own
vocabulary, no name list (ADR 0006). **Q2 (SWD) → honest DEGENERATE absence:** SWD is a 2-wire serial protocol
(`SWCLK`+`SWDIO`); its packet phases (request/ACK/data/turnaround) are TIME segments of bit-fields on the single
shared `SWDIO` wire (confirmed visually by Figure B4-1: `Start│APnDP│RnW│A│Parity│Stop│Park │Trn│ ACK │Trn│
WDATA[0:31] │Parity`, "Wire driven by: Host│Target│Host"). The phase names label bit-fields, not declared signals;
SWD's recognised phases all carry `signal_set=[]` and its transactions `ports=[]` → nothing to recover by any
lever; forcing it would fabricate. **Q3 (VLM) → evidence-based NO-GO:** the AXI/SWD `timing_diagram` crops carry no
per-declared-signal phase-column cue (generic handshake waveforms / credit-timing / dependency graphs / single-wire
packets). A bounded `qwen2.5vl:7b` probe (2 crops, temp 0) was internally contradictory, merely re-stated the
deterministic channel mapping while missing the figure's real content (the ordering arrows), and hallucinated
signal semantics (called both `CRDT` and `VALID` "write data valid"). It is RAM-expensive too — the 7B VLM loaded
as **13 GB** and pushed the host to **87% used**, across the 85% autonomous-kill threshold (`ollama stop` issued
immediately, recovered to 73% free). The only VLM-unique signal in the diagrams is phase ORDER (`picture_0011`
dependency arrows `AW*`/`W*`→`B*`) = the `.2h` residual, already FSMGen-blessed as "don't fabricate order".
**Decision:** the VLM-tier candidate is RESOLVED — replaced by a deterministic AXI-family channel-membership lever
(`.2m`, RAM-safe, structured-first); SWD per-signal membership + the VLM-membership lever are NO-GO honest-absence;
the VLM-tier transaction frontier is exhausted. **Gates:** read-only/docs-only — WIRE-BASED-100 untouched,
ADR-0006 ✓ (the channel-caption cue is the document's own universal vocabulary, no name list); no Rust code
changed; book unchanged (no user-facing behavior change yet — the book lands with the `.2m` implementation). KM
card `transaction-phase-membership-vlm-vs-channel` (KNOWLEDGE_MAP.md regenerated, derive-and-diff green).

### KG-ISF-TRANSACTIONS.2k — descendant-subsection scope for a transaction's signal-set membership
CODE (measurement-first, GO). Owner substantive gap #3 (transactions), the first of the two `.2j`-recorded future
candidates: the APB `.2c`-membership-breadth lever (deterministic, RAM-safe). A transaction is NAMED from a section
heading and its `.2c` signal-set membership is the declared signals that section's statements reference — but
section anchors are line-range and non-overlapping, so a transaction named from a PARENT heading (APB
`3.1 Write transfers`) saw only that section's own intro statements. APB `write_transfer` was a lone `{PCLK}` while
the signal-rich prose (`PADDR`/`PWDATA`/`PWRITE`/`PENABLE`/…) sits one level down under `3.1.1`/`3.1.2`. The
signals were already captured document-wide (the `.2g` `transaction_phases` surface recovers the rich
`setup`/`access` sets) — only the per-transaction SCOPE was too narrow. `build_transaction_anchors`
(`ir/semantic.rs`) now broadens a transaction's statement scope to every section whose dotted number is a strict
DESCENDANT of its own (new helpers `leading_section_number` + `is_descendant_section_number`), then derives the
`signal_set` + `supporting_statement_ids` from that subtree. Universal document-structure grammar keyed off the
section number, **no name list (ADR 0006)**; byte-identical when the section has no subsections. **Boundary-precise
(bar #3):** nested numbers do not overlap, so write (`3.1.x`) and read (`3.3.x`) subtrees are disjoint and a read
never picks up write-only signals. (Naively unioning the document-global `.2g` phase sets into both transactions
was REJECTED for exactly that reason.) **Measured (old-vs-new deterministic rebuilds; `generated/` gitignored):**
APB `write_transfer` 1→10 / `read_transfer` 1→7 (read correctly excludes `PWDATA`/`PSTRB`/`PWUSER`); AXI off its
previously-empty state — `atomic_transaction` 0→22, `prefetch_transaction` 0→8, `writezero_transaction` 0→6,
`writedeferrable_transaction` 0→10 (distinct, each subtree-scoped, max pairwise Jaccard 0.56). Corpus-wide
boundary scan (78 docs): 101/260 transactions carry membership, **0 over-broad** (none ≥80 % of the declared
inventory; max 77.8 % = a 9-signal flash-bus doc). **Gates:** ADR-0006 ✓; **`.isf` BYTE-IDENTICAL** on all 4 wire
docs (old-vs-new diff) ✓; **WIRE-BASED-100 provably orthogonal** — `actor_signal_relations`/`signal_constraints`/
`temporal_rules`/`conditional_rules` byte-identical old-vs-new, only `transactions` changed (membership is
`ports`/`phase_membership` metadata; the emitter lowers `steps`) ✓; `kg-bench` 156/156 ✓; `run_ci.sh` GREEN (lib
1662, +2 tests) ✓. Book `pipeline/intentir.md`; KM `transaction-membership-subsection-scope`. Remaining `.2j`
candidate (AXI/SWD timing-diagram phase columns, VLM-tier) stays a future lever.

### CORPUS-COVERAGE.1 — stage-staleness detector in `validate` (surface a downstream artifact that silently dropped relations)
CODE. The durable north-star win from gaps #1/#2: a downstream IR can go STALE relative to its upstream (the
stages don't auto-cascade — only `converge` rebuilds the whole chain), silently dropping its relations (the
measured `tilelink` evidence-39/intent-0 class). `validate <intent-ir>`/`<semantic-ir>` now loads the upstream
(via the carried `semantic_ir_path`/`evidence_ir_path` — `validate` already loads upstream for graph-aware
findings) and emits a `stage_staleness` **Warning** (`intent_stale_relations_dropped` /
`semantic_stale_relations_dropped`) when the downstream carries **0 `actor_signal_relations` while the upstream
carries some**. Deliberately a **zero-versus-some** test, not a count comparison → **false-positive-free**: the
agent-identity gates (consolidation/split/interface-fold/phantom-drop) only re-attribute or merge relations,
NEVER empty a non-empty set, so an empty-downstream/non-empty-upstream split can only be staleness; a
register/command/coherency protocol with 0 wire-signal relations has an empty UPSTREAM too (0-vs-0) and is
correctly left silent (honest absence ≠ stale drop); the upstream I/O is paid only when the downstream is
empty (a `let`-chain short-circuit), skipped when the upstream is off-disk. Pure decision helper
`stage_staleness_relation_finding` + 3 unit tests (0-vs-39 fires; 39/17-vs-N + both-empty silent). **Verified
live** (release binary, run from a temp CWD to dodge the WRITE-PATH GOTCHA): POSITIVE fires on a synthetic
stale tilelink (0 vs real semantic 39), NEGATIVE silent on `nvme` (0-vs-0 honest absence — the critical
no-false-positive case) and healthy tilelink (39); canonical untouched. ADR-0006 (universal/structural, no name
list); WIRE-BASED-100 unaffected by construction (validate-only additive finding; wire docs carry relations →
silent; extraction/IR content untouched). `run_ci.sh` GREEN (lib 1660 passed, +3); `kg-bench` 156/156. Book
`quality/validation.md` "Catching a stale downstream stage"; KM `stage-staleness-validate-detector`; tree
`CORPUS-COVERAGE.1`.

### CORPUS-COVERAGE.0 — build every ingested doc through to IntentIR/.isf (coverage 36→78 intent / 36→75 isf, 0 failures)
Owner-directed substantive north-star slice #2 (corpus coverage), following directly from `KG-ISF-COMPLETENESS.3`'s
staleness finding. The local corpus had only **36 of 79** ingested docs carried through to IntentIR — the other 42
sat at evidence-only (operational: a sweep rebuilt EvidenceIR without cascading downstream; the per-stage commands
do not auto-cascade, only `converge` rebuilds the whole chain). **Key fact:** building `semantic`→`intent`→`adapt`
needs only the already-persisted `evidence_ir.json`, NOT the heavyweight `normalized/` bundle (that is only needed
to rebuild EVIDENCE from source) — so the 42 evidence-only docs are cheap-buildable with no re-ingest. Rebuilt the
42 evidence-only + 3 stale docs (`tilelink_1_8_0`/`i2c`/`wbspec`) `semantic`→`intent`→`adapt --target isf`,
excluding the 4 gated WIRE-BASED-100 docs: **42/42 OK, 0 failures, RAM steady 77%** (deterministic, no LLM/Docling).
Coverage **36→78 intent / 36→75 isf**; **0 stale-intent remaining** (tilelink_1_8_0/i2c/wbspec recovered 40/17/1
relations). The 3 intent-without-isf docs (`risc_v_debug`/coresight `den0068` BSA/GIC `ihi0069_g`) **block honestly**
(`adapt`: "no behavioral content to lower" — register/architecture docs), the designed behavior, not a failure. The
staged pipeline is now validated end-to-end on the entire corpus. The 57 docs lacking a `normalized/` bundle need a
re-ingest (Docling + source PDF, RAM-gated) for an EVIDENCE rebuild — standing frontier. Docs-only deliverable (the
generated tree is git-ignored local cache); durable trace = the census + tree + KM `corpus-coverage-buildout`.
Frontier → `.1` generic stage-staleness `validate` detector. New tree `CORPUS-COVERAGE`.

### KG-ISF-COMPLETENESS.3 — relation-completeness measurement: 0-relation docs are stale-intent (recoverable) + honest register-protocol absence, NOT an extraction gap
Owner-directed substantive north-star push (after the owner pushed back on the "buildable frontier exhausted"
framing). Read-only census over all 78 `evidence_ir.json` + 36 `intent_ir.json` + content sampling, to answer
bar #2: why do whole docs (`nvme`/`tilelink`/`wbspec`/`i2c`/`ccix`/VT-d/IOMMU) carry actors+constraints but
ZERO `actor_signal_relations`? **Answer: it is NOT a relation-extraction gap.** Two cleanly-separated causes.
**(A) STALE IntentIR (recoverable):** the canonical `intent_ir.json` is stale relative to its
`evidence_ir.json` — `tilelink_1_7_1` carries 39 relations in evidence but 0 in its (05-16-dated) intent;
`tilelink_1_8_0` 40→0, `um10204` I2C 17→0, `wbspec` 1→0, plus a broader "intent older than evidence" set
(gic_600/mmu_700/ATS/dti/opencapi×3/usb4). A deterministic `semantic`→`intent` rebuild (no LLM/Docling)
recovers them — **PROVEN live on `tilelink_1_7_1`: relations 0→39, actor_ports 0→69, all 40 actors connected**
(RAM steady 77%). Operational cause (evidence rebuilt under a sweep without cascading downstream; only
`converge` rebuilds the whole chain), not a code bug. **(B) HONEST ABSENCE (not a gap):** `nvme`/`iommu`/
VT-d/`ccix` declare ~0 wire signals (content-sampled — their drive/read "cues" are ToC entries, register
legends `RO`/`RW`, and agent-MESSAGE prose, never agent-SIGNAL); these register/command/coherency protocols
express intent through `register_records`/`message_field_records`/`transactions`, so 0 actor-signal relations
is CORRECT — forcing them would FABRICATE. **Genericity insight:** the relation surface is intrinsically
wire-protocol-shaped; relation-completeness is the wrong bar dimension for register/message protocols.
Docs-only (no Rust change). Frontier → corpus refresh (owner gap #2) + a generic stage-staleness `validate`
detector. Report `docs/research/relation-completeness-measurement.md`; KM
`relation-completeness-staleness-vs-absence`; tree `KG-ISF-COMPLETENESS.3`.

### LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.1 — stabilize pointer and commit semantics

`COMMIT.md` now routes documentation by changed truth instead of mandatory co-staging: the owning task
leaf changes per slice, while README, MEMORY, status, roadmap, architecture, ledgers, and mdBook change
only when their own contract changes. `MEMORY.md` no longer shadows HEAD/ahead state; resume queries Git.
The memory gate now enforces the locally reviewed survivor on three axes (50 lines, 4,096 bytes, 160
bytes per content line), requires active-unit/next-action/in-flight/blocker fields, and rejects a
`latest_commit` shadow. Current pointer passes; a forced byte-cap probe fails closed.
### LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.0 — own and measure the containment/locality program

Opened the project-owned adoption program after reading FSMGEN's README policy, adoption guide, and
neutral live-document doctrine in full. The read-only SpecForge census proves routed pressure:
`README.md` is 602 lines/170,891 bytes, `CHANGES.md` 2.62 MB, `DEVELOPMENT_NOTES.md` 2.17 MB,
`LIVE_ACHIEVEMENT_STATUS.md` 572 KB, and the generated `KNOWLEDGE_MAP.md` 1.02 MB. The current commit
contract also creates ceremonial live-doc/MEMORY co-staging. A separate locality census found
production and test `tempfile::tempdir()` calls defaulting to the boot-volume temp directory. New task
tree `docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md` owns independent leaves for pointer
stabilization, README policy, common registry/checker, lossless migrations, mdBook truth/partitioning,
same-volume storage, and final closure. No content was deleted, migrated, or reclassified in this
ownership slice; donor thresholds and FSMGEN-local decisions were not copied.
