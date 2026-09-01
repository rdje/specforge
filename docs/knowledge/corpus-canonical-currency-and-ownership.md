---
id: corpus-canonical-currency-and-ownership
title: Only 24 of 78 persisted documents (30.8%) pass the scorer's schema gate and just 2 carry a gold, and the two green coverage gates both describe something else — chain-currency reports 24/24 over the rebuildable stratum alone, while the refresh frontier's "52 refreshed / 5 remaining" counts a host-library sweep whose cohort rule excludes every corpus/-sourced document and 31 of whose refreshed members are still legacy; five of the seven eval-gold documents cannot be scored, and the APB/AHB/AXI three went legacy because the completed refresh CORPUS-PATTERN-REUSE.3c did on 2026-06-09 was invalidated by the 2026-08-12 schema bump with no gate reporting it
answers:
  - "how much of the specforge persisted corpus can actually be scored / is canonically current"
  - "why does check_chain_currency report 24/24 current when 54 documents are legacy"
  - "what does the corpus refresh frontier's 52 refreshed / 5 remaining actually mean"
  - "does refreshed in the corpus frontier census mean the document is at the current schema"
  - "why are the APB AHB AXI wire golds not in the corpus refresh frontier"
  - "which specforge documents carry an eval gold and which of them are measurable"
  - "who owns re-ingesting the legacy wire golds APB AHB AXI"
  - "why did the APB AHB AXI chains go legacy when CORPUS-PATTERN-REUSE.3c already re-ingested them"
  - "does any gate fail when a persisted chain falls below the current canonical schema"
  - "does 24 measurable mean 24 documents produce a score"
  - "why can eval-extraction score only SWD/ADI and I2C"
  - "what predicate decides whether eval-extraction will score a document"
  - "how many corpus/-sourced documents are there and how many are legacy"
date: 2026-09-01
tags: [wire-based-100, corpus-coverage, corpus-frontier, chain-currency, schema-version, eval-extraction, measured, ownership, claim-verification, legacy-stratum]
evidence: docs/research/corpus-canonical-currency-census.md; docs/tasks/WIRE-BASED-100.md (.9/.9a); reproducer scripts/measure_corpus_canonical_currency.py (read-only over generated/*_ir headers, the four Rust schema constants, doctrine/corpus_frontier/census.json, and crates/specforge/test_data/llm_eval/*.json). Gating predicate = unmeasurable_disposition in crates/specforge/src/commands/eval_extraction.rs. Cohort exclusion = cohort_rule.excluded_source_prefixes in doctrine/corpus_frontier/census.json, rationale in scripts/check_corpus_frontier_census.pl.
reverify: "python3 scripts/measure_corpus_canonical_currency.py — expect 78 persisted, 24 MEASURABLE (30.8%), 54 LEGACY; stage histogram 54x1/24x3 source, 54x2/24x3 evidence, 54x1/24x2 semantic, 54x1/24x2 intent; legacy owners 18 outside-frontier + 31 declared-refreshed + 5 declared-remaining; 7 gold documents, 2 measurable, 5 unmeasurable (APB, AHB, AXI, RISC-V Debug with no OPEN owner; NVMe frontier-remaining). Cross-check the refusal live: ./target/release/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_apb.json --provider skip prints the UNMEASURABLE disposition and scores nothing. Cross-check the ownership claim against CLOSED leaves too, not just the open frontier: grep -n 'ihi0024_e|ihi0033_c|ihi0022_l' docs/tasks/*.md finds CORPUS-PATTERN-REUSE.3c (done 2026-06-09), the leaf that last performed this re-ingest. Numbers move when a legacy document is re-ingested (that is the point) or a schema constant is bumped; the reproducer reads the constants, so a bump shows up as a changed canonical row rather than a silent pass."
---

**Measured `2026-09-01` (`WIRE-BASED-100.9a`, read-only, no code change).** `WIRE-BASED-100.8` restored
the scoring oracle, then closed by routing the remaining work away in one sentence: *"Re-ingesting the
legacy stratum stays with the corpus refresh frontier."* This card records why that routing could never
have worked, and what the corpus health numbers actually are. Companion to
`[[evidence-proof-binds-artifact-location]]` (the `.8a` kernel finding) and
`[[eval-scores-persisted-evidence]]`.

**The measurable share is 30.8%, and it is not what either green gate reports.** A document is
measurable exactly when its persisted EvidenceIR is at the current canonical schema — the predicate
`eval-extraction` applies in `unmeasurable_disposition` before it will score anything. Censused over the
persisted corpus: **24 of 78 measurable, 54 legacy.** Read this as an ADMISSION count, not a
demonstration: only **2 of the 24 carry an eval gold**, so only two are scoreable today and the other 22
would merely be admitted if a gold existed. A schema-3 document that fails canonical verification for
any *other* reason is not dispositioned — it propagates and **aborts the run**, deliberately, so a real
defect cannot hide inside a disposition. The split is total at every stage (SourceIR
54×1 / 24×3, EvidenceIR 54×2 / 24×3, SemanticIR 54×1 / 24×2, IntentIR 54×1 / 24×2), so no document is
half-migrated, and a schema-3 SourceIR is structurally distinguishable — it carries `proof_context` and
`proof_ledger` keys a schema-1 artifact does not have. Schema 3 landed in `bb5047c2` (`2026-08-12`).

**Both existing coverage gates are green and both are about something else.**
`scripts/check_chain_currency.sh` reports `24 replayed / 24 current / 0 stale` — a currency statement
about the *rebuildable* stratum, which declares the other 54 UNMEASURABLE and does not count them, so it
reads 100% while describing 31% of the corpus. `scripts/check_corpus_frontier.sh` reports
`57 cohort = 52 refreshed + 5 remaining` — a completion statement about the host-library re-ingest
program (`CORPUS-COVERAGE.2`), which reads 91% done.

**`refreshed` does not mean current: 31 of the 52 declared-refreshed documents are still legacy.** The
census contract is honest internally — it defines `refreshed` as declared completed keys, and its
membership check constrains that declaration in one direction only (a *retained* normalization bundle
forces a key into `refreshed`; a declared-*remaining* key must have no retained bundle). Nothing ties
`refreshed` to a schema. But the line printed on the terminal invites the inference `.8` made. The
sweep finished under a SourceIR schema that no longer carries canonical authority.

**The frontier excludes the wire golds by contract, so it could not have owned them.** Its cohort rule
is `excluded_source_prefixes: ["corpus/"]`, documented as *"`corpus/` is the tracked in-repo gold/eval
corpus — copied into the repository, never part of the host-library refresh program."* Of the 78
persisted documents **21 are `corpus/`-sourced and outside the cohort; 18 of those are legacy.** The
frontier's `5 remaining` would still read `5` after every wire gold had rotted. Legacy ownership
partitions as **18 unowned + 31 declared-refreshed + 5 declared-remaining = 54**.

**The consequence is the important number: five of seven gold documents cannot be scored.** Deriving the
scored set from the eval datasets' own `doc_key` fields, only `ihi0074_a` (SWD/ADI) and `um10204` (I2C)
are measurable. APB, AHB, AXI, and RISC-V Debug are `corpus/`-sourced and have **no OPEN owning leaf**;
NVMe is owned only incidentally, as a frontier `remaining` entry. SpecForge's whole re-derivable
evaluation surface is two documents.

**The cause is an invalidated refresh, not neglect — and a closed leaf is where it is recorded.**
`CORPUS-PATTERN-REUSE.3c` (`done`, `2026-06-09`) re-ingested APB, AHB, AXI and AXI-Stream with
`DOCLING_DEVICE=cpu`, rebuilt their evidence, and re-verified the wire scores at `1.000`. The persisted
wire SourceIRs were last written `2026-08-09`; canonical schema 3 landed `2026-08-12` (`bb5047c2`),
three days later, and silently made that completed refresh legacy. **No gate reported it**: the frontier
excludes these documents by cohort rule, and chain-currency counts only the stratum already current. So
the durable defect underneath is that **nothing fails when a persisted chain falls below the canonical
schema** — that is a `DOCTRINE-ENFORCEMENT`-class gate nobody owns, and this census is the derivation it
would use.

**Why this is a doctrine lesson and not just a count.** Two rules, both earned here.
(1) **Read a gate's cohort rule before treating its ratio as coverage of anything** — `[[feedback_scoring_rigor]]`
applied to coverage rather than to scores. `.8` read `52 refreshed + 5 remaining`, reasonably concluded
the legacy stratum had an owner, and handed off to a program whose own contract excluded the documents.
The denominator a gate publishes is the population it was built for, not the one you are asking about;
where a real population has no gate, derive it.
(2) **The absence of an OPEN owner is not the absence of an owner.** This card's first draft said the
wire golds "had no owning leaf at all"; searching only the open frontier missed `CORPUS-PATTERN-REUSE.3c`,
which owned and performed exactly this work and closed. The closed leaf is where the causal story lives —
search it before publishing a "nobody owns X" claim, because the corrected story (a completed refresh
invalidated by a schema bump) points at a different and better fix than the wrong one (neglect).

**Owned, not merely logged.** `WIRE-BASED-100.9` owns the wire re-ingest (`.9b` APB, `.9c` AHB, `.9d`
AXI, smallest first, each with its own before/after evidence). The route is available and demonstrated
twice over: the three PDFs are git-tracked under `corpus/` by `WIRE-BASED-100.5d`, the repo-local
Docling runtime is ready, the three currently-measurable `corpus/`-sourced documents were produced
through this route after the schema bump, and `CORPUS-PATTERN-REUSE.3c` already ran this exact sequence
on these exact documents. The three shortfalls this tree does NOT own — RISC-V Debug
(`PDF-VARIANT-DIGESTION` register class), the 31 refreshed-but-legacy cohort documents (a corpus
program question about what `refreshed` should mean), and the missing canonical-currency gate — are
recorded in `.9`'s node as findings routed out, deliberately not absorbed.

**The caution a re-ingest carries.** The current binary cannot regenerate a legacy artifact, so a
re-ingest destroys evidence no rebuild can restore, and every wire number published before `2026-08-12`
was measured on that evidence. The posture is the one `[[generic-enum-conflation]]` and
`WIRE-BASED-100.8c` established: a re-derived number replaces the published one, and a number that
cannot be re-derived is withdrawn rather than carried.
