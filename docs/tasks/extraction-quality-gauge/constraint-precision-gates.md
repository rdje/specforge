# EXTRACTION-QUALITY-GAUGE — constraint precision gates

- Part ID: `constraint-precision-gates`
- State: `legacy`

<!-- extraction-quality-gauge-task-source-region:constraint-precision-leaves:start -->
- ID: `EXTRACTION-QUALITY-GAUGE.3` · Status: `active` (split `2026-06-10`) · Goal: permission/relational
  disambiguation. Split after `.8` measured the residual FP ledger: ALL three labeled-statement FPs are
  the **condition-subject-read-as-obligation** class — APB `PSELx must_be_high` (if-clause), AHB
  `HRESP must_be_value ERROR` (unless-clause), AXI `ACTIVATEACK must_be_value LOW` (when-clause).
- ID: `EXTRACTION-QUALITY-GAUGE.3a` · Status: `done` (`2026-06-10`) · Goal: a deterministic
  **condition-only-subject gate** in the LLM-primary grounding path: a proposed constraint whose
  subject appears ONLY inside subordinate conditional clauses of the source sentence
  (when/if/unless/while/until/after/before/whenever/provided-that/as-long-as — universal grammar, no
  name lists) is the condition's subject, not an obligation's, and is dropped
  (`is_condition_only_subject` + `conditional_clause_spans` + `token_occurrences` in
  `ir/constraint_extract_llm.rs`, called from `ground_constraint` after signal typing). **The
  per-item audit caught two real defects in the first cut before they could ship** — (1) clause
  spans ran past sentence-final punctuation and swallowed the NEXT sentence's main clause; (2)
  "while **driving** HREADYOUT LOW" is action *coordination* (the obligation IS on HREADYOUT), not a
  condition — fixed by terminating spans at `.!?` too and by skipping marker+gerund clauses; the
  defective first cut wrongly dropped AHB's two `HREADYOUT` records, the corrected gate restores
  them while still killing all three target FPs (both behaviors fixture-locked in tests). +8 pure
  tests total (lib 1511). **Measured (the `.8` redirected-copy protocol; post-`.8` numbers = the
  baseline): APB P 0.857→1.000, AHB P 0.857→1.000, AXI P 0.800→1.000, recall HELD at 16/16 —
  every doc now scores P=R=F1=1.000 on the labeled constraint task, and the split-conformal
  tier-agreement threshold now calibrates on all three (empirical_error 0.000).** The three killed
  FPs, per item: APB `PSELx must_be_high` (if-clause), AHB `HRESP must_be_value ERROR`
  (unless-clause), AXI `ACTIVATEACK must_be_value LOW` (when-clause).
- ID: `EXTRACTION-QUALITY-GAUGE.3b` · Status: `done` (`2026-06-10`) · Goal: permission-vs-obligation
  gate. **Probed (the AHB sentences read against source):** `HPROT[0] must_be_high` ← "It is
  **recommended** that a Manager sets HPROT[0] HIGH…" (no must/shall) and `HSEL must_be_high` +
  `HTRANS must_be_value IDLE` ← "An alternative implementation **would be** for HSEL to be tied
  HIGH…" (hypothetical) are frame errors; but `HEXOKAY must_be_deasserted` ← "It is **permitted**
  for a Manager … the Exclusive Write transfer **must** fail and HEXOKAY **must** be deasserted"
  is a REAL conditional obligation behind a permissive lead-in. **Shipped gate:
  `is_permissive_only_subject_frame` — scoped to the SENTENCES CONTAINING THE SUBJECT** (same
  split as `is_normative_for_subject`): a permissive frame word
  (recommended/permitted/permissible/optional/may/can/could/would — word-boundary, universal
  normative vocabulary) in a subject-sentence with NO mandatory frame (must/shall) in any
  subject-sentence → drop; mandatory present → keep outright. **The per-item audit killed the
  first cut again**: a BLOCK-scoped check over-killed AHB 15→8 — the incidental "Although an OKAY
  response **can** be given in a single cycle" softened the ERROR-procedure sentences and wrongly
  dropped `HRESP`×2 + `HREADYOUT`×2; sentence-scoping restores them (15→12 = exactly the three
  frame errors; over-kill shape test-locked). **Measured (refined gate, the `.8`/`.3a` protocol):
  AHB 15→12 with precisely `HPROT[0]`/`HSEL`/`HTRANS-IDLE` removed and `HRESP`/`HREADYOUT`/
  `HEXOKAY`/`H*USER` all kept; APB 20 and AXI 54 unchanged (controls); all three docs stay
  P=R=F1=1.000 against the EXTENDED gold.** Honest scoring note: the eval-time WIRE-BASED-100
  filter (`is_normative_for_subject`) already masked this FP class from the labeled gauge — the
  `.3b` win is at the CANONICAL ARTIFACT level (the EvidenceIR downstream consumers read is now
  clean of frame errors), and the two new AHB gold NEGATIVE items (statements `0561`/`0678`,
  `agent_drafted`) are regression armor if the eval-time filter ever weakens. +6 pure tests (lib
  1517). Relational-vs-value ("set to the same value as") and descriptive-narration frames stay a
  later sub-slice (`.3c`, pending).
- ID: `EXTRACTION-QUALITY-GAUGE.3c` · Status: `done` (`2026-06-14`, CODE + full gold battery;
  unblocked by `PDF-VARIANT-DIGESTION.13c`) · Goal: the **descriptive-narration frame** gate (the
  larger of the two `.3c` classes; relational-value deferred to `.3d`).
  **SHIPPED:** pure `is_descriptive_narration_binding(text)` in `ir/evidence.rs`, called in
  `extract_dynamic_signal_constraints` in the logic-level-binding branch (Pattern path = the root
  mint site, so it cleans BOTH surfaces — the LLM-primary recall universe is the Pattern sentence
  set). Gate fires ONLY when: an ACTION bind verb (`set/sets/setting/drive/drives/driving/driven`,
  never static `tied/held/pulled/forced`) is present, NO mandatory modal (`must`/`shall`/`required
  to` — the bare adjective "required" as in "the required value" is NOT mandatory; this fixed a
  first-cut over-keep caught by the timing-walkthrough test), AND a descriptive marker (`this
  signal` / timing anchor `T<n>` / `figure … shows`). +6 pure tests; lib 1614 → **1620**; full
  `scripts/run_ci.sh` GREEN (incl. a `manual_contains` clippy fix). **Verified (fresh release bin):**
  CHI rebuilt 13 → **5** signal constraints — removing EXACTLY the 8 `*FLITV`/`*LCRDV` description
  cells, keeping the 5 `REQ must_be_value 0/I` field-obligation rows (a DIFFERENT class — the
  `.FIELD.4`/promotion concern, correctly untouched). **Gold-safe (per-item, on gated-Pattern
  rebuilds with backup/restore of the promoted wire artifacts):** APB/AHB/AXI constraints
  P=R=F1=1.000 + WIRE-BASED-100 relations 1.000 + temporal 3/3+4/4+3/3; SWD constraints/relations
  1.000 + SWD-derivation frame/operation/state 1.000; kg-bench 156/156. **HONEST gauge note
  (`feedback_scoring_rigor`):** the CHI NLI not-entailed RATE *rose* 69.2% → 100% (5/5) when the 8
  descriptive constraints dropped — because the NLI judge textually entails "sets HIGH" ⇒ "is HIGH"
  and so had ENTAILED 4 of the 8; but a *valid* strobe (`REQFLITV`) is semantically NOT an always-high
  invariant, so per-item the cleaned surface is strictly MORE correct. The gauge is a textual
  heuristic; the per-item semantic audit is ground truth (same precedent as `.3b`). The 5 remaining
  100%-not-entailed REQ rows are the field-mis-attribution class a converge's now-default promotion
  cleans (the `.4` sweep measured CHI 69.2%→16.7% promoted). Book: `pipeline/evidenceir.md` `.3c`
  subsection (+ FIELD.4 example reconciled). The original `.3c` design follows. **Probe-first (read-only, no 14B, over all
  78 persisted evidence docs, `source_text` of 583 signal_constraints):** the descriptive-narration
  class = **38 records** the deterministic Pattern path mints by reading an *action* of an actor as a
  global invariant — `logic_level_binding_kind_from_text` matches `sets|drives|driven <signal>
  HIGH/LOW` and `extract_dynamic_signal_constraints` mints `MustBeHigh/Low`. Two sub-forms, both
  pure narration: (a) **signal-description cells** — "The transmitter/receiver **sets this signal**
  HIGH to indicate/return …" (CHI's 8 FLITV/LCRDV — the exact records the `.13c` gauge flagged
  not-entailed); (b) **timing-diagram walkthrough** — "At T2, the power controller **sets PREQ
  HIGH**. The interface state is now P\_REQUEST." / "- T3 The Manager **sets FABORT HIGH** to
  request that the WRITE is aborted." / "Figure 3-5 **shows the case where** the controller sets
  PREQ HIGH" (ihi0068 low-power 13, ihi0083 GPIO 13, HBM2 1). **Per-item audit: all 38 are genuine
  errors** — a signal that GOES high as part of its function (cell) or AT a waveform step (timing)
  is not a global `must_be_high` invariant; real timing facts belong in the temporal layer, not a
  flat constraint. **Gold-safety proven by the probe: ZERO APB/AHB/AXI/SWD constraints match either
  sub-form** (gold obligations are phrased "X must be …" / "must drive X LOW" / static "tied HIGH" —
  none is actor-action narration), so the gate cannot regress the wire gold gates. **Placement
  decision: gate the PATTERN path (`extract_dynamic_signal_constraints`), not the LLM-primary path
  where `.3a`/`.3b` live** — because the LLM-primary extractor's recall universe IS the Pattern
  surface's sentences, so refusing to mint at the root cleans BOTH surfaces with one deterministic,
  14B-free-verifiable change (and most of the corpus carries the Pattern surface canonically). Gate
  design (conservative, over-kill-guarded per the `.3a`/`.3b` lesson): drop a logic-level-binding
  constraint ONLY when the bind verb is an ACTION verb (`set/sets/setting/drive/drives/driving/
  driven` — NEVER the static-invariant verbs `tied/held/pulled/forced`), there is NO mandatory modal
  (`must/shall/required`) in the subject clause, AND a descriptive marker is present (the phrase
  `this signal`, a timing anchor `T\d+`, or a figure-narration phrase `figure … shows`). Verify:
  re-run the corpus probe for the exact removed-constraint diff (audit every drop), gold eval ×3 +
  SWD `--provider skip` stay 1.000, kg-bench green, full `run_ci.sh`, CHI gauge re-measured on the
  cleaned surface. Universal grammar only (ADR 0006 — no name lists).
- ID: `EXTRACTION-QUALITY-GAUGE.3d` · Status: `done` (`2026-06-14`, CODE + gold battery; spun from
  the `.3c` probe) · Goal: the **relational-value frame** — the second `.3c` class the corpus probe
  surfaced. An *inter-signal/field equality* ("ALLOW_UW **must be equal to the value of** ALLOW_PW",
  or the bounded "a value **less than or equal to the value of** the NVM Set Identifier Maximum
  field") has no slot in the constraint-kind vocabulary, so BOTH value-binding paths mis-mint a
  garbage `must_be_value` (a truncated value lifted off a condition token — DTI's `<token>
  must_be_value "E"` with wrong subjects `BYPASS`/`STRW`/`EL3`). **SHIPPED:** pure
  `is_relational_equality_constraint(text)` (keys on "… the value of …" / "the same value as …" —
  NOT bare "equal to", so a literal "equal to 0" is untouched), wired as an early refuse in BOTH
  `extract_dynamic_signal_constraints` (`dyn_sigcon_*`, the DTI 15) AND `extract_signal_constraints`
  (`sigcon_*`, the NVMe 5 — a SECOND extractor the per-item audit caught: the NVMe relational record
  was minted there, not by the dynamic path). Decision: refuse → honest residual (a typed
  `MustEqual{other}` kind was the alternative, deferred — no consumer needs inter-signal equality
  yet). +2 pure tests; lib 1620 → **1622**; full `run_ci.sh` GREEN; kg-bench 156/156. **Verified:**
  NVMe rebuilt 26 → **21** (all 5 relational records gone, 0 remaining) — DTI's 15 `dyn_sigcon_*`
  would clear identically but its `normalized/` bundle was artifact-reclaimed (canonical cleanup
  rides the next DTI re-ingest; gate proven by the NVMe rebuild + unit tests). **Gold-safe** per-item
  on gated-Pattern rebuilds of ALL four wire docs (backup/restore, non-destructive): APB/AHB/AXI
  constraints 1.000 + WIRE-BASED-100 relations 1.000 + temporal 3/3+4/4+3/3; SWD constraints/
  relations 1.000 + SWD-derivation operation/state/frame 1.000 — probe-confirmed gold-safe (no
  wire-doc obligation is a relational equality). NVMe `must_be_stable` field-description cells
  (`This field indicates …`) are a SEPARATE descriptive-field-cell shape (a future gate), not
  relational. Book: `pipeline/evidenceir.md` `.3d` paragraph. **The `.3` constraint-precision program
  (`.3a` condition-subject / `.3b` permissive-frame / `.3c` descriptive-narration / `.3d`
  relational-value) is now complete.**
- ID: `EXTRACTION-QUALITY-GAUGE.3e` · Status: `done` (`2026-06-15`, CODE + full gold battery; PNT pick
  — the candidate future leaf `.3d` recorded: the NVMe `must_be_stable` "This field indicates …"
  descriptive-field-cell class, a decidable, probe-first, no-14B sibling of `.3c`/`.3d`) · Goal: the
  **descriptive-field-cell spurious-subject** gate.
  **SHIPPED:** pure `is_descriptive_field_cell_spurious_subject(text, subject)` in `ir/evidence.rs`,
  wired as a `subject_signals.retain(…)` in BOTH value-binding extractors (`extract_signal_constraints`
  `sigcon_*` — where all 9 errors mint — AND `extract_dynamic_signal_constraints` `dyn_sigcon_*`, so a
  future doc on the dynamic path is covered; the dynamic NVMe keeps stay kept). The gate fires ONLY
  when: the subject is a plain identifier; the source carries a `"this field <descriptive-verb>"`
  marker (`indicates`/`specifies`/`describes`/`contains`/`defines`/`represents`/`reports`/`identifies`/
  `provides` — never the obligation lead "this field shall/must/should"); AND the subject does NOT
  occur (identifier-boundary, case-insensitive) BEFORE the marker (= it is not the field's own name).
  +3 pure tests (drop / keep-own-mnemonic / never-touch-non-field); lib **1622 → 1625**; `cargo fmt`
  clean; warning-deny clippy clean. **Verified (fresh release bin, canonical rebuilds):** NVMe 21 → 20
  (drops exactly `sigcon_0004 FFFF must_be_stable`; the 4 dynamic-path keeps `SANICAP`/`HMDLLA`/
  `HMDLAL`/`ELEN` + `CBA`×2 + the real `ANA…` `must_not_change` sentence all kept; NVMe semantic+intent
  rebuilt to stay coherent), CCIX r1.0a 23 → 15 (drops exactly the 8 descriptive-cell subjects
  `CCIX`/`PCI`/`SRAM`/`DDR`/`NVDIMM`/`HBM`/`SAMA`/`CCIX`; the non-"This field" `HAC`/`HAM`/`DDR`/
  `NVDIMM`/… table rows correctly UNTOUCHED — scope discipline). **Corpus residual = 0** (no
  descriptive-field-cell spurious-subject record remains in the 78-doc corpus). **Gold-safe (live
  battery, gated-Pattern rebuilds of all four wire docs with backup/restore — non-destructive):**
  APB/AHB/AXI constraints **1.000** + relations **1.000** + temporal **3/3+4/4+3/3**; SWD constraints/
  relations **1.000** + SWD-derivation frame **11/11**/operation **4/4**/state **13/13**; kg-bench
  **156/156**. Refuse → honest residual (the field's real obligation, if any, rides its own mnemonic;
  recovering the correct subject of a `"the field must indicate 0h"`-style obligation is a separate
  recall concern, not this precision gate). The out-of-scope `DDR`/`NVDIMM` `must_be_stable` table rows
  that lack a "This field" marker are an honest residual (a different structural shape; no denylist —
  `feedback_avoid_denylists_prefer_structural`). Book: `pipeline/evidenceir.md` `.3e` paragraph. A register/structure field-definition cell narrates what the field IS — the
  field's own name PRECEDES a `"This field <descriptive-verb>"` marker, while value-meaning tokens
  appear only AFTER it (memory-type names `SRAM`/`DDR`/`NVDIMM`/`HBM`, protocol acronyms `CCIX`/`PCI`,
  a hex literal `FFFF`, a truncated `SAMA`). The deterministic constraint path lifts one of those body
  tokens as the subject and mints a garbage `must_be_*` whose subject is not a wire/field at all
  (`.gauge` root cause #1, the "Spurious subject" class — largest). **Probe-first (read-only, no 14B,
  over all 78 persisted evidence docs, `source_text` of every signal_constraint):** the class =
  **15 records / 2 register-structure docs** whose `source_text` carries a `"This field <verb>"`
  descriptive marker — CCIX r1.0a ×8 (`MultiPortDevCap`/`MemPoolSpcificMemTypeCap`/`MemPoolAddrCap`
  cells → subjects `CCIX`/`PCI`/`SRAM`/`DDR`/`NVDIMM`/`HBM`/`SAMA`) + NVMe ×7. **Per-item audit:**
  the **9** whose subject appears ONLY AFTER the marker are genuine spurious-subject errors (8 CCIX +
  NVMe `sigcon_0004` `FFFF`); the **6** whose subject is the cell's own leading mnemonic
  (`(CBA):`/`(SANICAP):`/`(HMDLLA):`/`(HMDLAL):`/`(ELEN):` — appears BEFORE the marker, with a real
  `shall` obligation) are legitimate and MUST be kept. **Discriminator (structural, ADR 0006 — no name
  lists):** in a `"This field <descriptive-verb>"` cell, the subject must appear (whole word,
  case-insensitive) BEFORE the marker (= it is the field's name) → keep; a subject appearing only
  after it was lifted from the descriptive body → spurious → drop. All 9 errors are minted in
  `extract_signal_constraints` (`sigcon_*`); gate wired into BOTH value-binding extractors (mirrors
  `.3d`) so a future doc on the dynamic path is covered too — the 4 dynamic-path NVMe keeps stay kept
  by the before-marker rule. **Gold-safe by the probe: ZERO APB/AHB/AXI/SWD constraints match** (wire
  docs declare no `"This field"` cells; obligations are phrased "X must be …"). Verify: NVMe rebuild
  21→20 (FFFF gone), CCIX r1.0a rebuild 23→15 (8 gone), wire gold ×4 stay 1.000 on gated-Pattern
  rebuilds, kg-bench green, full `run_ci.sh`. Refuse → honest residual (the field's real obligation,
  if any, rides its own mnemonic — recall of the correct subject is a separate concern).
- ID: `EXTRACTION-QUALITY-GAUGE.3f` · Status: `done` (`2026-06-15`, CODE + full gold battery; PNT pick —
  owner-chosen "EQG constraint precision" direction; the per-item NVMe audit under `.3e` left this
  residual) · Goal: the **alphabetic-value word-boundary** gate on the deterministic value binder.
  **SHIPPED + VERIFIED:** pure `lead_binds_value(text, lead, value_lower)` in `ir/evidence.rs` replaces
  the substring `contains_any` inside `extract_discovered_state_value_from_text` (the ONE shared value
  matcher used by `extract_dynamic_signal_constraints` `dyn_sigcon_*`) — it requires a trailing
  identifier boundary after the matched value ONLY when the value ends in a letter; numeric values stay
  lenient (preserve "0h"). +3 tests, lib 1625 → **1628**, `cargo fmt` clean, warning-deny clippy clean
  (`let`-chain collapse), full `run_ci.sh` GREEN, kg-bench **156/156**. **Verified (fresh release bin):**
  NVMe `evidence --dry-run` **20 → 19** — removes EXACTLY `dyn_sigcon_0013 SANICAP must_be_value NO`
  (the "shall be `no`n-zero" fragment), nothing added, `message_field_records`/`timing_constraints`
  byte-identical. **Wire-build invariance proven:** a fresh-Pattern rebuild (new bin) of all four wire
  docs has **0** alphabetic `must_be_value` records that `.3f` would alter → OLD/NEW bins produce
  identical wire Pattern builds. **Gold-safe (non-destructive temp-evidence-root eval, new bin):**
  APB/AHB/AXI/SWD constraints **1.000** + relations **1.000** (content-anchored) + APB/AHB/AXI temporal
  **3/3+4/4+3/3**; SWD-derivation frame **11/11** / operation **4/4** / state **13/13**;
  `seed_nvme_registers` field-name recall **28/29** unchanged. Refuse → honest residual.
  **Probe-first (read-only, no 14B, over all 78 persisted evidence docs):** the dynamic value binder
  `extract_discovered_state_value_from_text` (`ir/evidence.rs`) matches a discovered enum value behind a
  normative lead phrase (`must be`/`shall be`/`must remain`/`shall remain` `<value>`) with a PLAIN
  substring `contains_any`, so an ALPHABETIC value is lifted out of a longer word: NVMe `SANICAP` mints
  `dyn_sigcon_0013 must_be_value NO` off "this field **shall be `no`n-zero**" — the true obligations are
  "shall be non-zero" / "shall be cleared to 0h", so `NO` is a fabricated fragment
  (`feedback_scoring_rigor` — honest residual over a fabricated fact). **Measured fix:** require a
  trailing identifier boundary after the matched value ONLY when the value ends in a LETTER (an
  alphabetic enum value `NO`/`YES`/`VALID` must match a WHOLE word); a numeric value keeps lenient
  matching so a radixed literal ("shall be `0`h" → ELEN/RECFMT, genuine) still binds. **Corpus impact
  (measured pre-build over the 78-doc persisted surface):** removes EXACTLY 1 record (NVMe `SANICAP NO`)
  and flips ZERO wire-doc (APB/AHB/AXI/SWD) constraints; the genuine numeric `must_be_value 0` /
  "shall be 0h" family is untouched — the blanket after-boundary rule was REJECTED by measurement
  (it would wrongly drop ELEN/RECFMT "shall be 0h", where `0` legitimately prefixes `0h`). Universal
  grammar, no name lists (ADR 0006); root-cause fix in the ONE shared matcher (DRY); the `is <value>
  when` form is inherently whole-word-bounded and unchanged. Acceptance: +unit tests (the non-zero bug
  → None, whole-word `NO` → bound, `0h` numeric preserved, `valid` preserved); NVMe `evidence
  --dry-run` 20→19 (SANICAP NO gone, other 19 identical); gated-Pattern rebuild + `eval-extraction
  --provider skip` of all four wire docs (constraints+relations+temporal) stays 1.000 with
  `seed_nvme_registers` unaffected; kg-bench 156/156; full `run_ci.sh` GREEN; book
  `pipeline/evidenceir.md` `.3f` note + README bullet + KM card.
- ID: `EXTRACTION-QUALITY-GAUGE.3g` · Status: `done` (`2026-06-15`, CODE + full gold battery; PNT pick —
  owner-chosen "EQG constraint precision" direction; the per-item NVMe audit residual orthogonal to
  `.3e`) · Goal: the **dotted-cross-reference spurious-subject** gate.
  **SHIPPED + VERIFIED:** pure `is_dotted_cross_reference_subject(text, subject)` in `ir/evidence.rs`
  (drop iff the subject is a plain identifier AND every whole-word occurrence is immediately preceded by
  `<ident>.`), wired as a `subject_signals.retain(…)` in BOTH value-binding extractors right after the
  `.3e` retain. +2 test fns (dotted-ref dropped; standalone/own-mnemonic/mixed/absent kept), lib 1628 →
  **1630**, `cargo fmt` clean, warning-deny clippy clean, full `run_ci.sh` GREEN, kg-bench **156/156**.
  **Verified (fresh release bin, `.3f`+`.3g`):** NVMe `evidence --dry-run` **20 → 18** — removes exactly
  `SANICAP NO` (`.3f`) + `MPS must_be_value 0` (`.3g`, from `CC.MPS`); `BADD`'s genuine alignment
  obligation is KEPT; nothing added. Wire Pattern builds byte-identical (the gate alters 0 wire records);
  gold-safe on the non-destructive temp-evidence-root eval: APB/AHB/AXI/SWD constraints + relations +
  temporal + SWD-derivation all **1.000**; `seed_nvme_registers` 28/29 unchanged. Refuse → honest
  residual. **Probe-first (read-only, all 78 persisted evidence
  docs):** a constraint subject lifted from a `Reg.Field` dotted cross-reference inside the cell body —
  the NVMe `BADD` cell "Buffer Address (BADD): Indicates the host memory address … aligned to the memory
  page size (**CC.MPS**). The least significant bits … shall be 0" yields the genuine `BADD must_be_value
  0` (alignment) AND a spurious co-subject `MPS` (value 0) lifted from `CC.MPS` — a cross-reference to the
  CC register's MPS field, NOT this cell's subject. `.3e` does not reach it (the cell has no "this field
  <verb>" marker — it reads "(BADD): Indicates …"). **Measured impact:** the class = subjects whose EVERY
  whole-word occurrence in the source is immediately preceded by `<ident>.` — **1 record corpus-wide
  (NVMe `MPS`), 0 wire-doc**; the pure-hex-literal alternative was REJECTED by measurement (it would
  wrongly flag the real fields `CBA`/`BADD`, all-hex-letter names). **Fix:** pure
  `is_dotted_cross_reference_subject(text, subject)` (drop iff every whole-word occurrence is dotted-ref-
  prefixed) wired as a `subject_signals.retain(…)` in BOTH value-binding extractors, mirroring `.3e`;
  universal grammar (`Reg.Field` cross-reference is a cross-vendor register-spec idiom — structural, no
  name lists, ADR 0006). The cell's genuine subject (`BADD`, written "(BADD):") is kept. Acceptance:
  +unit tests (dotted-ref subject dropped; standalone/own-mnemonic subject kept; mixed standalone+dotted
  kept); NVMe `evidence --dry-run` drops `MPS` (18 with `.3f`+`.3g`); wire Pattern builds byte-identical
  (0 records altered, proven by fresh-bin scan) → wire gold ×4 1.000; kg-bench 156/156; full `run_ci.sh`
  GREEN; book `pipeline/evidenceir.md` `.3g` note + KM card.
- ID: `EXTRACTION-QUALITY-GAUGE.3h` · Status: `done` (`2026-08-10`, CODE + full gold battery) · Goal: the
  **value-position spurious-subject** gate — the last member of the `.3d`–`.3g` family, carried over as a named
  residual from `CORPUS-COVERAGE.2.50a` rather than left as a note.
  **Probe-first (read-only, all 80 persisted evidence docs):** a value-binding preposition phrase puts the bound
  VALUE after it, never the constrained thing, so a candidate reachable only there is a literal wearing a
  subject's clothes. NVMe: "… all entries … **shall have the Controller ID field set to FFFFh**" mints
  `FFFF must_be_value NO` — the obligation is on the Controller ID field, `FFFF` is the hex literal it is set to,
  and the record's own `target_value` (`NO`, from a later "shall be no more than one") comes from a *different*
  phrase, so the pattern path's positional value exclusion never reaches it. **Measured impact:** the class =
  subjects whose EVERY uppercase-run occurrence is immediately preceded by a value binder — **1 record
  corpus-wide (NVMe `FFFF`), 0 wire-doc, 0 in any rebuildable document.**
  **SHIPPED + VERIFIED:** pure `is_value_position_subject(text, subject)` in `ir/evidence.rs` over
  `uppercase_run_tokens` (the same tokenization `collect_subject_signal_tokens` uses, so `FFFFh` is seen as the
  candidate `FFFF` the extractor actually lifted), wired as a `subject_signals.retain(…)` in BOTH deterministic
  extractors right after the `.2.50a` retain. Standalone wins, exactly as in `.3g`: a candidate that occurs even
  once outside a value position is never touched. +3 test fns (the exact NVMe sentence yields no `FFFF`;
  standalone-beats-value-position; `PSEL must be set to HIGH` still yields `PSEL`), lib 1,804 → **1,807**.
  The all-hex-literal shortcut stays rejected by the `.3g` measurement (it would flag real `CBA`/`BADD`).
  Universal grammar, no name/radix/literal list (ADR 0006).
  **Honest limit:** NVMe has no normalized bundle, so its persisted artifact keeps the `FFFF` record until that
  document's own refresh re-ingests it; the replay proves the code is correct and ADR 0025's currency check
  reports the unmeasurable population rather than implying coverage.

- ID: `EXTRACTION-QUALITY-GAUGE.3i` · Status: `done` (`2026-09-12`) · Goal: **a flag that modifies an
  obligation must be read from that obligation.** Opened by `INVARIANT-SHAPE-ADMISSION.3` over one
  imprecise record and closed over a larger and sharper defect: `negated` was computed across the WHOLE
  statement while the subject and the condition already came from `constraint_bearing_sentence`, so a
  `must not` in one sentence flipped a constraint minted from another. RISC-V IOMMU published
  `The DV operand must be 1 for IODIR` as a **negated** constraint for exactly that reason.
  **Measured, read-only over the persisted corpus:** 20 negated records; **4** carry a negation that is
  not in their own obligation clause; **all 20** sit on a kind the classifier never matched, because
  every phrase in the table is affirmative (`must be X`) and a negated obligation never contains one.
  Shipped: the narrowing in BOTH deterministic paths (the dynamic path also stopped carrying a second
  copy of the negator list), plus the two spellings whose absence caused the defaulting —
  `must not be changed` → `MustNotChange`, and `must not be asserted` / `must not be active` →
  `MustBeDeasserted`, reached only after the affirmative `must be asserted` arm, which those strings do
  not contain.
  Rebuilt APB + AHB: **3 records retyped, 0 added, 0 removed** —
  `PSTRB` `must_be_stable`+negated → **`must_be_low`**, `HSIZE` → `must_not_change`,
  `HEXOKAY` → `must_be_deasserted`. `PSTRB` is the corroboration: it is refined to `LOW` by the polarity
  layer and now **agrees with `dyn_sigcon_0015`**, the same document's prose
  *"For read transfers, the Requester must drive all bits of PSTRB LOW"*. Two independent extraction
  paths, two different places in the document, one typed obligation — where before `3i` they
  contradicted each other, which is why APB's `actor_contracts` fall 15 → 14.
  Producer: the census in this leaf's Verification Log; controls in `mod extraction_quality_gauge_3i`.
  Prerequisite: none. Verification: all 20 adjudicated; observed RED; APB + AHB rebuilt.
  Commit: `EXTRACTION-QUALITY-GAUGE.3i`

<!-- extraction-quality-gauge-task-source-region:constraint-precision-leaves:end -->
