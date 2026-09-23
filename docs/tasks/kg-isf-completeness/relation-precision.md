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

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.1e` (CODE) — DONE `2026-09-23`

- [x] **REPRODUCE / MEASURE** — Wishbone's 41 relations read by hand (`.5.xiii.a.5.a`): 7 wrong directions; 6 trace to one object span (the verb's object ran to its clause end).
- [x] **ROOT CAUSE (WHY + WHERE)** — `active_object_contains_signal` (`crates/specforge/src/ir/evidence.rs`) cut the object only at `.`/`;` and condition markers, so a coordinated second verb's object and a causal adjunct's trigger were read as the first verb's object.
- [x] **ADDRESSED (verified)** — six seam tests in `kg_isf_completeness_1e`, each RED-verified by neutralising the rule it pins: coordination cut (2 fail), `in response to` (2 fail), shared subject (1 fails), finite agreement (Avalon test fails), governing verb before `and` (TileLink test fails); the AHB test is a labelled regression guard.
- [x] **NO REGRESSION** — `evidence --dry-run` over all 78 chains with both binaries: 27 replayed, 26 byte-identical, Wishbone −6 wrong relations only; persisted-statement replay over all 78: 34 wrong facts removed, 0 correct lost. WIRE-BASED-100 unmoved by construction (no scorable gold is Wishbone, and every other replayable chain is byte-identical); `kg-bench` 158/158; `cargo fmt` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` green (473 / 168 / 1,643, +6). Wishbone chain rebuilt through the adapter (still no `.isf`, as before).
- [x] **GENERICITY (ADR 0006)** — English clause structure plus the existing relation-verb vocabulary; no name, document or protocol identity. Flow census +3 functions / +9 decision sites / +4 helper edges / +1 macro under `aggregate_change`; boundary unchanged.
- [x] **LOCKSTEP** — book `pipeline/evidenceir.md` (the Wishbone relation-precision paragraph); tree root frontier; `MEMORY.md`. No rule deleted.
