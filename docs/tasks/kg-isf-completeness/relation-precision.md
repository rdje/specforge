# KG-ISF-COMPLETENESS — relation precision

- Part ID: `relation-precision`
- State: `active`

## Post-migration record

- ID: `KG-ISF-COMPLETENESS.1d` · Status: `open` (`2026-09-23`; MEASUREMENT then CODE; opened by
  `.5.xiii.a.5.a`) · Goal: **a phrase is not an actor.** The actor-identity gate (`.1a`) admits prose
  fragments as relation subjects: Wishbone B4, once its names became matchable, publishes 9 relations
  whose actors are `interfaces so`, `similar manner`, `same operation`, `bus cycle`, `DAT_I signal`,
  `cases SLAVE`; ADIv6 publishes `details reads TDO` from *"For details about the Read TDO (RTDO) bit,
  see …"*. This is the class `.1c.ii` measured on eMMC (109 multi-word actors) and deferred as having no
  clean within-document structural gate — **its trigger is met**: it now reaches a rebuildable document.
  Measure corpus-wide which actor strings are neither a declared actor, a role noun the document uses as
  a subject elsewhere, nor a component the document names, and read a sample before choosing a gate.
  Verification: census over all 78 chains; replay over the 27 with a per-relation diff; WIRE-BASED-100
  with both binaries; the Wishbone 41 re-read.
  **MEASURED `2026-09-23` — the grounding test the leaf proposed is refuted, and the class has causes, not a
  shape.** The persisted artifacts cannot answer it: 51 of 78 chains were written by older binaries, and their
  actors include `of ordered` and `set of`, which `.1a` already refuses. So the current extractor was replayed
  over every chain's persisted statements (a throwaway `#[ignore]` probe, removed byte-exactly): **1,342
  relations, 794 document-actors, 409 of them two-word** (no current actor is longer). Grounding does not
  separate them: a declared actor matches **3** of 1,053 persisted actors (22 of 78 documents declare any), and
  mention counts point the wrong way — junk `bus cycle` occurs 75 times and in headings, real `watchdog timer`
  nowhere else. A seeded sample of 60 two-word actors read by hand: **about 10 real agents** (`Avalon-MM
  master`, `snooped Manager`, `MASTER interface`, `I/O device`), **about 9 real agents that swallowed their own
  verb** (`HBM responds`, `Manager sees`, `Completer guarantees`), **about 40 not agents at all**. Reading how
  each was minted gave the causes: (1) a relation verb used as a NOUN — the vocabulary holds `read`, `access`,
  `monitor`, … and *"the read pointer"* licenses a relation whose subject is whatever phrase precedes it:
  **124 relations, owned and landed as `.1d.i`**; (2) the subject cut takes two words after the nearest
  determiner even when the second is a finite verb — **about 20 actors, `.1d.ii`**; (3) clauses with no agent
  at all — a copula (*"MUST be the same operation"*), an imperative (*"Design … interfaces so that"*), a
  leading prepositional phrase fused onto the subject (*"In these cases SLAVE circuitry"*) — and a signal
  standing as actor (`DAT_I signal`), which stay here. After `.1d.i`: 377 two-word document-actors, 545
  relations.
  Commit: measurement recorded with `KG-ISF-COMPLETENESS.1d.i`.

- ID: `KG-ISF-COMPLETENESS.1d.i` · Status: `done` (`2026-09-23`; CODE; opened by `.1d`'s measurement) ·
  Goal: **a verb after an article is a noun.** A relation is licensed by any occurrence of a vocabulary verb
  whose object holds the signal, and the vocabulary carries base forms (`read`, `access`, `monitor`, …) that
  English uses as nouns and modifiers: *"the read pointer"*, *"a read transaction"*, *"the access requests"*.
  Acceptance: an occurrence directly preceded by an article or determiner no longer licenses a relation;
  capital `A` stays a possible label (*"Master A asserts"*); every removed relation is read; per-relation replay
  over the 27 and the persisted-statement replay over all 78 with both binaries; WIRE-BASED-100 with both.
  **LANDED.** `is_nominal_verb_occurrence` refuses an occurrence whose preceding word is `the`, `an`, lowercase
  `a`, `this`, `these`, `those`, `each`, `every`, `any`, `no` or `another`, at all four active licensing
  sites. `that` stays out (the relative pronoun of *"the Manager that drives HTRANS"*), as do `its` (the GIC
  `ITS` agent, `.1a`: *"the ITS forwards any operations"*) and `all`/`both`/`some`, which can precede a verb.
  Capital `A` stays a label because the measurement found one: *"1. Master A asserts address"* keeps `Master
  drives ADDRESS`, and a case-blind `a` fails that guard.
  **Reach, persisted statements of all 78 chains:** 1,342 -> 1,218 relations, **124 removed across 16
  documents, 0 added**, and every removal read by hand (`read` 84, `access` 27, `writes` 11, `accesses` 1, `activates` 1):
  the relation verb was a noun in each. **Rebuild of the 27 replayable chains with both binaries:** **25
  byte-identical**; AHB 75 -> 68 and ADIv6 109 -> 99, only `actor_signal_relations` and the surfaces derived
  from it moving. The 17: 15 wrong — among them ADIv6 `details reads TDO` (named in `.1d`), eight `… reads In`
  from *"a read or write packet request"*, AHB `Subordinate reads Manager` from *"the read data bus moves
  data from a Subordinate to the Manager"* — and **2 true but never evidenced**: AHB `Subordinate reads
  HREADY` was read out of *"drives the read data bus"*, and `Subordinate reads HREADYCHK` inherits from it (a
  check signal takes its protected signal's relation). HREADY is a Subordinate input in AHB5, so that is a
  recall residual with a wrong source, owned as `.1d.i.a`, not a fact to keep on a misread sentence.
  **WIRE-BASED-100 unmoved:** no removed relation's statement carries any gold item, and all seven seeds
  (`ahb`, `apb`, `axi`, `swd`, three temporal) score identically with each binary on its own build — only
  temp paths and the AHB/ADIv6 relation counts differ in the full output.
  **The two moved chains are rebuilt in this commit**, because the proof binds the derivation topology
  (`[[producer-change-and-corpus-rebuild-are-one-transaction]]`): the first commit attempt was refused by
  `PROOF-SEAL-CURRENCY` for exactly AHB and ADIv6. Rebuilt evidence -> semantic -> validate -> intent -> validate
  -> adapt, then rebuilt again with the HEAD binary for comparison: `adapter.json` differs only in
  `proof_ledger` hashes — the lowered `isf`, `lowering_status` and `residual_decisions` are identical.
  Verification: acceptance checklist below.
  Commit: `KG-ISF-COMPLETENESS.1d.i — a verb after an article is a noun`

- ID: `KG-ISF-COMPLETENESS.1d.i.a` · Status: `open` (`2026-09-23`; MEASUREMENT then CODE; opened by `.1d.i`) ·
  Goal: **recover AHB `Subordinate reads HREADY` from a sentence or table that states it.** `.1d.i` removed it
  because its only support was *"the Subordinate drives the read data bus"*; HREADY is a Subordinate input in
  AHB5, and `Subordinate reads HREADYCHK` follows from it. Measure where the specification states the
  direction (a port table, a signal description) and why no relation path reads it today.

- ID: `KG-ISF-COMPLETENESS.1d.ii` · Status: `open` (`2026-09-23`; MEASUREMENT then CODE; opened by `.1d`) ·
  Goal: **a subject phrase ends before its own verb.** `extract_subject_phrase` takes up to two words after the
  nearest determiner, so *"the HBM responds following …"* yields `HBM responds`. Of 40 two-word actors whose
  second word is a lowercase `-s` word, about 20 are this (`Manager sees`, `host tries`, `specification
  permits`) and the rest plural nouns (`Distributor registers`, `snoop transactions`); measure a cue that
  separates the two before choosing a rule.
  **MEASURED `2026-09-23` at `306878ab` — trimming is refuted; the defect is governance, not the phrase.**
  The current extractor replayed over all 78 chains (1,218 relations) gives **40** two-word actors whose second
  word is a lowercase `-s` word, every one read in its sentence: **21 swallowed a finite verb**, 19 end in a
  plural noun (first published as 41 / 22 / 20 from a count of a printed list; re-derived from the records on
  `2026-09-23`, per `CLAIM_VERIFICATION.md` §6.1 a named count is exact). Of the 21, trimming to the head would be right in only **2** — a coordinated relation verb
  (*"The Manager sees that LMOPENACK is asserted and … asserts LRCREDIT"*) and a catenative with a
  to-infinitive (*"the component continues to receive more DVM … operations"*). In the other **19** the relation
  verb sits inside the swallowed verb's COMPLEMENT and is not governed by the subject at all: trimming would
  turn `Arm recommends drives SPI`, `IOMMU offers reads DEVICE`, `chapter dictates reads DATA` into
  plausible-looking `Arm drives SPI`, `IOMMU reads DEVICE` — laundering junk into facts. **Nor does the next
  word separate verb from plural noun**: `that` follows both (*"recommends that sources"* / *"encodings that
  are"*). So the rule to measure is a GOVERNANCE test: when the subject phrase ends in a finite verb V, the
  relation is licensed only if its verb is coordinated with V (`.1e`'s shared subject) or is V's to-infinitive
  complement; otherwise it is refused, not re-attributed. Measure that test's reach both ways before coding.
  **Caveat found on re-verification:** *"the host wants to access a partition"* and *"… tries to access …"* also
  have the catenative + to-infinitive shape, so that exception alone would keep `host reads PARTITION`, where
  `PARTITION` is catalog noise rather than a signal — the governance test must be measured together with the
  signal side, not instead of it.


- ID: `KG-ISF-COMPLETENESS.1e` · Status: `done` (`2026-09-23`; CODE; opened by `.5.xiii.a.5.a`) · Goal: **a
  relation's direction comes from the verb that governs the signal, not from co-occurrence.**
  **SIX OF WISHBONE'S SEVEN WRONG DIRECTIONS WERE ONE DEFECT: AN ACTIVE VERB'S OBJECT RAN TO THE END OF ITS
  CLAUSE.** `active_object_contains_signal` now also ends the object at a **coordinated third-person relation
  verb** (`coordinated_relation_verb_start`: *"latches [DAT_I()] and negates its [STB_O]"*, *"monitors the
  MASTER's [STB_O] signal, and asserts [ERR_I]"*) and at the causal adjunct **`in response to`** (*"asserts
  [ACK_I] … in response to every assertion of the [STB_O]"*). Only an `-s` verb opens a verb phrase:
  coordinated NOUN objects stay the object (`4cb4b535`), and a base form after `and` is routinely a noun or a
  signal name.
  **THE CUT NEEDED ITS PARTNER, MEASURED.** Alone it lost 3 correct facts (Avalon `source interface drives
  VALID`, LTI `Subordinate drives LACREDIT`/`LCCREDIT`): the coordinated verb's subject is ellipted.
  `active_verb_subject` gives a coordinated FINITE verb the subject of the verb it is coordinated with
  (`last_third_person_relation_verb`), at all five active subject sites. Two further corrections, each from
  a measured record: the governing verb may end right before `and` (without it, both TileLink documents
  published `agents drives DATA`), and a base form never shares a subject (without it, Avalon's *"…, and read
  after the rising edge of clk"* published `Master reads CLK`).
  **Reach.** 27 replayable chains: **26 byte-identical**; Wishbone 41 -> 35, exactly the six wrong
  relations (`fact_provenance` -6, manifest and proof ledger with them). Persisted statements of all 78
  chains, replayed both ways: **34 facts removed, all wrong** (the six, LTI `Subordinate reads LACREDIT`,
  TileLink `receiver drives DATA`, HBM `HBM device reads DBI`, and junk actors or signals: `possible
  mismatch`, `NOT`, `ANY`, `AN`); 7 re-attributed, all on non-signal catalog names (`AN`, `NOT`, `BUS`,
  `RCA`, `CLASS`); AHB `Manager drives HTRANS`, LTI's two credits and Avalon `VALID` verified present.
  The seventh wrong direction (`MASTER reads STB_I`, a pronoun subject) is a different class: `.1f`.
  Verification: acceptance checklist below.
  Commit: `KG-ISF-COMPLETENESS.1e — a relation's object ends where its verb phrase ends`

- ID: `KG-ISF-COMPLETENESS.1f` · Status: `open` (`2026-09-23`; MEASUREMENT then CODE; opened by `.1e`) ·
  Goal: **a pronoun subject resolves to its antecedent, not to the nearest role noun.** Wishbone publishes
  `MASTER reads STB_I` from *"the MASTER will assert the select signals [SEL_O()] during every bus cycle, but
  a particular slave is only accessed when it monitors that its [STB_I] input is asserted"*: the subject of
  `monitors` is `it`, whose antecedent is *a particular slave*. Measure how many active relations take their
  actor across a pronoun, then read a sample before choosing a rule. Verification: per-relation replay diff
  over the 27; the persisted-statement replay over all 78.

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.1d.i` (CODE) — DONE `2026-09-23`

- [x] **REPRODUCE / MEASURE** — the current extractor replayed over all 78 chains' persisted statements (throwaway `#[ignore]` probe, removed byte-exactly): 1,342 relations; attribution through the extractor's own `active_object_contains_signal` + `active_verb_subject` found 125 licensed only by a determiner-preceded verb occurrence, and a 40-relation sample read by hand was wrong in every case.
- [x] **ROOT CAUSE (WHY + WHERE)** — `extract_actor_signal_relations` (`crates/specforge/src/ir/evidence.rs`) licensed a relation at any spelling of a vocabulary verb, and `ACTIVE_READS_VERBS`/`ACTIVE_DRIVES_VERBS` hold forms English uses as nouns (`read`, `access`, `writes`), so *"the read pointer"* minted `value reads APB`.
- [x] **ADDRESSED (verified)** — `is_nominal_verb_occurrence` at the four active licensing sites; five seam tests in `kg_isf_completeness_1d_i`: neutralising the helper fails exactly the three RED tests, and a case-blind `a` fails exactly the `Master A asserts` label guard.
- [x] **NO REGRESSION** — persisted-statement replay over all 78: 124 removed, 0 added, every removal read; `evidence --dry-run` over the 27 with both binaries: 25 byte-identical, AHB 75 -> 68 and ADIv6 109 -> 99 (15 wrong, 2 true-but-unevidenced, owned as `.1d.i.a`); both chains rebuilt through the adapter, `adapter.json` differing from a HEAD-binary rebuild only in `proof_ledger` hashes; WIRE-BASED-100 seeds `ahb`/`apb`/`axi`/`swd` and three temporal score identically with each binary on its own build (source-tolerant relations 1.000, document-level recall 1.000); `kg-bench` 158/158; `cargo fmt` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` green (473 / 168 / 1,648, +5).
- [x] **GENERICITY (ADR 0006)** — English determiner grammar; no name, document or protocol identity. Flow census +1 function / +3 decision sites / +1 helper edge / +0 macros under `aggregate_change`; boundary unchanged.
- [x] **LOCKSTEP** — book `pipeline/evidenceir.md` (the relation-precision section); tree root frontier; `MEMORY.md`. No rule deleted.

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.1e` (CODE) — DONE `2026-09-23`

- [x] **REPRODUCE / MEASURE** — Wishbone's 41 relations read by hand (`.5.xiii.a.5.a`): 7 wrong directions; 6 trace to one object span (the verb's object ran to its clause end).
- [x] **ROOT CAUSE (WHY + WHERE)** — `active_object_contains_signal` (`crates/specforge/src/ir/evidence.rs`) cut the object only at `.`/`;` and condition markers, so a coordinated second verb's object and a causal adjunct's trigger were read as the first verb's object.
- [x] **ADDRESSED (verified)** — six seam tests in `kg_isf_completeness_1e`, each RED-verified by neutralising the rule it pins: coordination cut (2 fail), `in response to` (2 fail), shared subject (1 fails), finite agreement (Avalon test fails), governing verb before `and` (TileLink test fails); the AHB test is a labelled regression guard.
- [x] **NO REGRESSION** — `evidence --dry-run` over all 78 chains with both binaries: 27 replayed, 26 byte-identical, Wishbone −6 wrong relations only; persisted-statement replay over all 78: 34 wrong facts removed, 0 correct lost. WIRE-BASED-100 unmoved by construction (no scorable gold is Wishbone, and every other replayable chain is byte-identical); `kg-bench` 158/158; `cargo fmt` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` green (473 / 168 / 1,643, +6). Wishbone chain rebuilt through the adapter (still no `.isf`, as before).
- [x] **GENERICITY (ADR 0006)** — English clause structure plus the existing relation-verb vocabulary; no name, document or protocol identity. Flow census +3 functions / +9 decision sites / +4 helper edges / +1 macro under `aggregate_change`; boundary unchanged.
- [x] **LOCKSTEP** — book `pipeline/evidenceir.md` (the Wishbone relation-precision paragraph); tree root frontier; `MEMORY.md`. No rule deleted.
