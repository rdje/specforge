# EVIDENCE-DETERMINISM: make the EvidenceIR build reproducible (no content-level non-determinism)

## Metadata

- Tree ID: `EVIDENCE-DETERMINISM`
- Status: `done` (`.1` diagnosis + `.2` fix DONE `2026-06-09`; EvidenceIR build reproducible on the corpus)
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

## `.2` — Fix (DONE `2026-06-09`)

Two `HashSet`-iteration leaks found + fixed (both in `crates/specforge/src/ir/evidence.rs`):
1. **`extract_actor_signal_relations`** iterated `known_signals: &HashSet<String>` directly → the relation
   `asr_NNNN` ids + record order (+ the first-seen-wins dedup representative) depended on hash order. Fix:
   sort once into a `Vec<&String>` before the statement loop, iterate that. The relation SET
   (`(actor,signal,kind)` dedup keys) is invariant — verified the SWD key-set is unchanged (26 keys, == both
   pre-fix runs); only ids/order/attribution become stable.
2. **`derive_encoding_enum_name`** sorted candidates by `Reverse(len())` ONLY (a partial order) → same-length
   names (`TDO`/`TDI`) stayed tied and a stable sort kept the non-deterministic HashSet order, so the chosen
   enum name (`Enum <name> …` statements) was non-deterministic. Fix: a **total** order (length desc, then
   name).

**Verification:** ALL 6 intact-bundle docs (RISC-V Debug, I2C, SWP, CAN, NVMe, SWD/ADI) now **double-run
byte-identical** (was: SWD content-different run-to-run); SWD eval `serial_frame_field`/`swd_operation`/
`protocol_state` all `P=R=F1=1.000` (relation/frame/state surfaces unchanged); kg-bench 151/151; full
`run_ci.sh` green (lib 1454 → 1455 — +1 determinism regression test: build a `HashSet` twice → identical
relation output). Multiset-preserving (no fabrication; same facts, stable representative + order). KM card
`evidence-build-nondeterminism` updated to the fixed state + the "never let HashSet order reach output" pattern.

## Current frontier

`.1` diagnosis + `.2` fix DONE — the EvidenceIR build is now reproducible on the corpus. Tree effectively
closed at its scope (a deeper exhaustive `HashSet`/`HashMap` audit across ALL surfaces could be a future leaf
if a new non-determinism surfaces, but the corpus double-run is currently clean). Next program work returns to
`EXTRACTOR-ARCHITECTURE.6` (migrate registers/actors/polarity — and now every byte-identical proof is sound).
