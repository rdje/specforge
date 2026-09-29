# KG-ISF-COMPLETENESS — actor surface leaves

- Part ID: `actor-surface-leaves`
- State: `active`

New leaves of the actor surface (which phrase is an agent, and what a relation's actor is) continue here, after
`relation-precision` reached its size warning (`.1g`, `2026-09-29`).

## Post-migration records

- ID: `KG-ISF-COMPLETENESS.1g` · Status: `done` (`2026-09-29`; MEASUREMENT then CODE; checklist below; opened by
  `EXTRACTION-REPRESENTATION-GAPS.3.c.ii`) · Goal: **a register, a field or a bit is not an actor.** ADIv6's *This bit
  controls the CSYSPWRUPREQ signal* makes `bit` drive the wire once that wire is known. Across the 78 persisted chains
  105 relations name an actor whose head noun is a storage or value word: `register` 18, `field` 12, `CSW register` 8,
  `maximum number` 6, `bit` 6, `signal` 5, `register bits` 4, and more. The `.1a` gate rejects a phrase by its leading
  token, never by its head. Measure the class by the head noun structurally (a noun of the register model, a value
  noun), read a sample of each, and decide whether *X register controls S* states something the IR should keep (a
  software-visible control, not a drive) before gating. Verification: per-relation replay diff over the 31; the
  persisted-statement replay over all 78.
  **Decision.** No phrase in the class names an agent: all 105 are registers, fields, bits, values, numbers or
  signals. A few state a register-to-wire binding (*This field drives the nSRSTOUT signal*). That is a fact about the
  field, not an actor relation, and it is owned by `EXTRACTION-REPRESENTATION-GAPS.41`. **Done.**
  `normalize_relation_actor_name` refuses a phrase whose head noun (`is_storage_or_value_phrase`: `bit`, `field`,
  `register`, `number`, `value`, `signal` and their plurals) names one. The subject reader ends its search there
  (`Err`), because an earlier determiner would re-attribute the relation: the first replay published `meaning drives
  HPROT` and `instruction drives DBGTDI`. **Measured:** EvidenceIR dry-run of the 31 chains with the HEAD and new
  binaries: 27 identical, 13 relations removed and none added (TMC 4, AHB 2, ADIv6 6, Wishbone 1). ADIv6 also loses three
  output declarations that rested only on them. `DBGTDO` and `nSRSTOUT` were right by chance: their table rows are
  converter-shifted, `EXTRACTION-REPRESENTATION-GAPS.3.c.viii`. `PORTCONNECTED`'s was wrong, since the ports drive it.
  Rebuilt stage by stage: TMC's emitted `.isf` gains `FULL` and `SYNCREQS` as outputs, which its signal table states,
  and FSMGen `--strict --check` reports 0 diagnostics. Before `EXTRACTION-REPRESENTATION-GAPS.42`, the same gate had
  lowered TMC's condition-less *FULL is pulled HIGH* as `(FULL 1)`.
  Commit: `KG-ISF-COMPLETENESS.1g — a register, a field or a bit is not an actor`

- ID: `KG-ISF-COMPLETENESS.1h` · Status: `open` (`2026-09-29`; MEASUREMENT then CODE; opened by
  `EXTRACTION-REPRESENTATION-GAPS.3.c.ii`) · Goal: **a behaviour belongs to an actor where the actor acts, not where its
  name appears inside another name.** Once ADIv6 has a `debug interface` actor, 7 statements become its behaviours
  because they mention the *ARM Debug Interface*, the architecture's title (*This Architecture Specification is written
  for engineers implementing an ARM Debug Interface …*). `system power` had taken three *system power domain*
  statements the same way before `EXTRACTION-REPRESENTATION-GAPS.3.c.ii.a` removed it. Measure how many behaviours are
  attached by a name that occurs only inside a longer noun phrase or a title, read a sample, and decide the rule.
  Verification: IntentIR `behaviors` diff over the 31.

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.1g` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — 105 relations over the 78 persisted chains, 13 over the 31 current ones, name an actor
  headed by a storage or value noun (`IR register drives DBGTDI`, `number reads TDI`, `HPROT bit drives HPROT`,
  `signal drives SYNCREQS`); with `EXTRACTION-REPRESENTATION-GAPS.3.c.ii`'s reader, `bit` takes 32 of ADIv6's behaviours.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `normalize_relation_actor_name` judged a
  phrase by its leading token only (`.1a`), and `extract_subject_phrase` moved on to an earlier determiner when a phrase
  failed to normalize. RED, with the file restored byte-identically after each: dropping the head gate fails
  `a_phrase_headed_by_a_storage_or_value_noun_is_not_an_actor`; dropping the search end fails
  `a_refused_storage_subject_is_not_replaced_by_an_earlier_phrase`.
- [x] **ADDRESSED (verified)** — the four moved chains rebuilt stage by stage, each stage validated, the adapter last:
  no storage-headed actor; TMC's emitted `.isf` FSMGen-strict-clean.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains: 27 identical, only removals, every removed fact read
  downstream (junk actors, their behaviours, ports, conflicts and interface groupings). `check_chain_currency.sh` 31 of
  31 current at every stage. WIRE-BASED-100: all 11 seeds score identically (`seed_ahb` and `seed_swd` print 76 and
  13 stored relations instead of 78 and 19). `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline
  --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph`
  474 / 170 / 1,783 passed, 0 failed. The `.1b.i` test that used `register allows` as an agent now expects it refused.
  `flow_census.json` rolled forward under this leaf (+1 function, +2 decision sites, +35 helper edges).
- [x] **GENERICITY (ADR 0006)** — twelve closed nouns of the register and signal model, the phrase's head; no document,
  register or signal name.
- [x] **LOCKSTEP** — book `domain/actor-connectivity.md` (*What gets filtered*); KM
  `a-storage-or-value-noun-heads-no-agent` (new) and `partitioning-a-task-tree-has-a-fixed-registration-price`
  (adding a part); this part opened; `EXTRACTION-REPRESENTATION-GAPS.41` opened; `.3.c.viii` widened to Table B3-2.

