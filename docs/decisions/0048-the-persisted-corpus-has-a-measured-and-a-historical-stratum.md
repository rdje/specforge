---
id: the-persisted-corpus-has-a-measured-and-a-historical-stratum
title: The persisted corpus has a measured and a historical stratum, and only one can carry a current claim
date: 2026-09-18
status: accepted
scope: corpus, evidence-ir, claim-verification, measurement, project-data-locality
evidence: scripts/check_proof_seal_currency.sh; scripts/check_chain_currency.sh; crates/specforge/src/ir/evidence.rs (EVIDENCE_IR_SCHEMA_VERSION, load_from_path, load_for_inspection); docs/knowledge/persisted-llm-constraint-corpus-predates-catalog-grounding.md; docs/tasks/extraction-quality-gauge/llm-path-family.md (.3j.2)
reverify: "python3 -c \"import json,glob,os;p=[k for k in glob.glob('generated/source_ir/*/source_ir.json') if any('proof' in x.lower() or 'ledger' in x.lower() for x in json.load(open(k)))];print(len(p),'proof-carrying of',len(glob.glob('generated/source_ir/*/source_ir.json')))\" — expect 27 of 78"
answers:
  - "what is ADR 0048"
  - "why does generated/ hold artifacts the canonical loader refuses"
  - "may I publish a current number measured from a persisted artifact"
  - "which documents may a new measurement population be drawn from"
  - "should the legacy corpus be deleted or rebuilt when the EvidenceIR schema bumps"
  - "how many persisted documents are proof-carrying and how many are legacy"
  - "why did a census over the persisted corpus have to be retracted"
---

# ADR 0048: The persisted corpus has a measured and a historical stratum, and only one can carry a current claim

## Context

`generated/` holds 78 persisted document chains, 5.7 GB, on the repository volume. They are not one
population. **27** carry a proof ledger and current schema-3 EvidenceIR; the current binary's canonical
loader accepts them, and `PROOF-SEAL-CURRENCY` (gate tier) and `CHAIN-CURRENCY` (CI tier) hold them
current. **51** do not. Their EvidenceIR is schema 2, `EvidenceIr::load_from_path` refuses them as
*legacy/proofless and inspection-only*, and `load_for_inspection` is the only way to read them.

The roadmap already publishes that split. What was missing is a rule about what may be *concluded* from
each, and its absence has already cost a published finding. `EXTRACTION-QUALITY-GAUGE.3j` measured five
positional subject gates against 149 persisted `llm_sigcon_*` records and published a precision of 3/7.
`.3j.2` then established that all seven of those documents are in the **historical** stratum and that
their records were minted **78 minutes before `declared_signal_catalog` existed**, by a grounding rule
that consulted no catalog at all. The measurement was of a producer that no longer exists; a caveat in
the same tree was retracted for the same reason. Nothing mechanical or doctrinal had stopped it.

The LLM-primary constraint path has, to date, **never been run on the measured stratum**.

## Decision

1. **The two strata are named and are not interchangeable.** The *measured* stratum is the set whose
   persisted `source_ir.json` carries a proof ledger — currently 27 documents. The *historical* stratum
   is the remainder — currently 51. Membership is derived, never asserted: the `reverify` command above
   recomputes it.
2. **Only the measured stratum may ground a current actionable claim** as `CLAIM_VERIFICATION.md` §1
   defines one. A count, rate, precision or coverage figure derived from the historical stratum is not
   publishable as current.
3. **A measurement over the historical stratum is dated evidence about its own era**, and must name the
   producer revision it measures — not merely the date it was run. `.3j.2` names a commit and a clock
   time 78 minutes apart; that is the standard.
4. **Neither stratum is deleted.** The historical stratum is the only surviving record of superseded
   producer behaviour, and it is load-bearing: `.3j.2`'s root cause could not have been established
   without it, and deleting it would convert a known answer back into archaeology. Its 5.7 GB is a
   deliberate cost, on the repository volume, under `PROJECT_DATA_LOCALITY.md`.
5. **Neither stratum is rebuilt merely because a schema bumped.** A schema bump makes the historical
   stratum inspection-only; that is the intended outcome, not a defect to repair. Re-ingesting a document
   is a deliberate, owned act with its own leaf, never a side effect of a gate going red.
6. **A new measurement population is drawn from the measured stratum by default.** Choosing the
   historical stratum instead requires stating why in the owning leaf — for example, that the question is
   *about* the superseded producer, which is the one case where it is the correct population.

## Consequences

- `EXTRACTION-QUALITY-GAUGE.3j.4` retargets the LLM-path measurement population onto the 27 measured
  documents, all of which load canonically today.
- `.3j.1.b`'s recorded blocker was wrong and is corrected: it is not blocked on a provider alone. Starting
  a model against the historical seven would produce another unpublishable number.
- `.3j`'s published 3/7 precision, and every count in `.3j.2`, `.3j.2.a` and `.3j.2.b`, remain valid as
  what they are — dated observations about the pre-catalog producer, each already labelled as such.
- The rules `.3j.2.a.i` and `.3j.2.b.i` wired are unaffected: both are properties of the resolver and its
  grammar, verified by in-tree controls, and neither rests on a corpus measurement.

## Links

- Bounds `CLAIM_VERIFICATION.md` §1 for corpus-derived figures.
- Builds on [ADR 0025](0025-persisted-chain-currency-is-measured-not-assumed.md).
- Fact card: `[[persisted-llm-constraint-corpus-predates-catalog-grounding]]`.
