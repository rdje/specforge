# EVIDENCE-DETERMINISM: make the EvidenceIR build reproducible (no content-level non-determinism)

## Metadata

- Tree ID: `EVIDENCE-DETERMINISM`
- Status: `active`
- Roadmap lane: `R0` (correctness/reproducibility) / `R15e` (eval integrity)
- Created: `2026-06-09`
- Parent: surfaced by `EXTRACTOR-ARCHITECTURE.5` verification — a pure refactor's byte-identical check failed
  on SWD/ADI, and the investigation proved the EvidenceIR build is **non-deterministic at the content level**
  (independent of the refactor).

## Goal

The same `SourceIR` must always produce the same `EvidenceIR` (byte-identical). Non-determinism silently
undermines: (1) **reproducibility** / crash-safe handoff, (2) **eval-score stability** — the scorer reads
PERSISTED evidence ([[feedback_scoring_rigor]]: every score objectively measured), so a non-deterministic
build means a score can drift run-to-run, (3) the **byte-identical verification methodology** the
`EXTRACTOR-ARCHITECTURE` migrations rely on to prove "behavior-preserving" (a sound proof needs a
deterministic baseline). Fix so a double-run is byte-identical, with no fabrication / no behavior loss.

## Non-goals

- NOT changing WHAT is extracted (the relation set must stay the same multiset) — only making the chosen
  representative + ordering deterministic.
- NOT a broad rewrite — targeted: replace order-leaking `HashSet`/`HashMap` iteration with deterministic order.

## `.1` — Diagnosis (DONE `2026-06-09`)

**Confirmed non-deterministic:** two runs of the IDENTICAL `EXTRACTOR-ARCHITECTURE.4` code (HEAD `ebc4e48d`)
on the SWD/ADI `source_ir` produced **set-different** `actor_signal_relations` and `extracted_statements`
(order-insensitive comparison `same-CONTENT(set)=False`), with `fact_provenance` same-content but
different-order. The 4 other intact-bundle docs (RISC-V Debug, I2C, SWP, CAN) were byte-identical run-to-run —
so the non-determinism is specific to the **relation-heavy prose** path SWD exercises.

**Root cause localized:** `extract_actor_signal_relations` (`crates/specforge/src/ir/evidence.rs`) iterates
`known_signals: &HashSet<String>` (`for signal in known_signals`) to drive prose relation extraction, and a
first-seen-wins `seen: HashSet<(String,String,u8)>` dedup then keeps **different representatives** depending
on the non-deterministic `HashSet` iteration order → both the order AND the surviving content of
`actor_signal_relations` vary run-to-run. That cascades into `extracted_statements` (relation-derived
synthesized statements minted in that order via the build-wide counter) and `fact_provenance` (order).

**Method note (durable):** the `EXTRACTOR-ARCHITECTURE` byte-identical proofs must from now on either (a) use
deterministic docs, or (b) compare order-insensitively, until `.2` lands — recorded so future migration
verification is not mis-read as a regression.

## `.2`+ — Fix (pending)

Replace the order-leaking iteration with a deterministic order (iterate a sorted view of `known_signals`, or
make the relevant set a `BTreeSet`), audit the relation/statement-assembly path for other `HashSet`/`HashMap`
iterations that leak into output (e.g. `relations_by_signal` `HashMap` @ ~ evidence.rs:2563), and VERIFY by
double-run byte-identicality on SWD/ADI + re-checking that the relation multiset (and any eval score that
reads it) is unchanged in CONTENT (only the chosen representative/order is now stable). Behavior-preserving in
the multiset sense; a careful slice because it touches relation output that downstream surfaces consume.

## Current frontier

`.1` diagnosis owned. `.2` (the deterministic-iteration fix) is the recommended next correctness step — it
makes every future `EXTRACTOR-ARCHITECTURE` migration proof sound and stabilizes eval. Owner may sequence it
against continuing the extractor migration (`EXTRACTOR-ARCHITECTURE.6`: registers/actors/polarity).
