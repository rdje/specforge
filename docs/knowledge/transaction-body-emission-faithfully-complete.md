---
id: transaction-body-emission-faithfully-complete
title: The transaction ISF BODY is faithfully complete — a value-free `(sample S)` membership body is FSMGen-ACCEPTED on pin `030f8c273` but NOT faithful (it asserts an un-grounded entry-cycle capture + activation guard the document never states), so beyond the value-grounded enum-selector `(drive)` body (`.2b`) the body lever is exhausted; membership stays metadata (`.2c`/`.2i`/`.2m`), per FSMGen's `2026-06-16` steer
answers:
  - "can SpecForge lower a transaction's signal-set membership into the ISF transaction BODY"
  - "what did KG-ISF-TRANSACTIONS.2n measure / decide"
  - "is there a buildable transaction ordered multi-phase body lever beyond .2b"
  - "does FSMGen 030f8c273 accept a value-free (sample S as s) transaction body"
  - "does (sample S) work for an interface OUTPUT signal too (yes — FSMGen does not gate sample on direction)"
  - "why is a membership-derived (sample) body NOT faithful even though it is FSMGen-accepted"
  - "what does (on start (sample S as s)) assert in FSMGen semantics (an entry-cycle D-input capture, cycle N port && can_accept)"
  - "why does an in-body (drive NAME) need a top-level named-drive definition (drive 'X' not defined)"
  - "is the transaction body faithfully complete (yes — only the grounded enum-selector drive is body-lowerable)"
  - "what carries transaction membership faithfully instead of the body (IntentIR metadata: ports / phase_membership / channel_membership)"
  - "fresh empirical reconfirmation of the .2i body-emission parking on the current 030f8c273 binary"
date: 2026-06-17
tags: [kg-isf-transactions, transactions, isf, body-emission, sample, drive, fsmgen, 030f8c273, measured, no-go, adr-0006, honest-residual, north-star, probe-first]
evidence: docs/tasks/KG-ISF-TRANSACTIONS.md (.2n node + Frontier); crates/specforge/src/ir/intent.rs (mint_named_transaction — body steps only the Cue-B enum-selector drive); crates/specforge/src/ir/isf_ir.rs (the `!tx.steps.is_empty()` emit filter ~L822; partition_txn_steps ~L1321; render_txn_step Sample/Drive); subs/fsmgen/docs/book/src/13b-transactions.md (`(on port ...)` entry/idle state L133-163: "Cycle N: port && can_accept -> samples captured"; `(sample ...)` "No State, Piggybacks" L230-246); subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md (2026-06-16 phase-membership answer)
reverify: "RAM-SAFE, no VLM. fsmgen is subs/fsmgen/bin/fsmgen (Perl). Probe A (pure value-free sample body over a declared input) PASSES strict: printf '(actor probe\\n  (clock HCLK)\\n  (interface\\n    (input HREADY (width 1))\\n  )\\n  (transaction read_transfer\\n    (on start\\n      (sample HREADY as r)\\n    )\\n    (complete done)\\n  )\\n)\\n' > /tmp/pa.isf && perl subs/fsmgen/bin/fsmgen --strict --check --json /tmp/pa.isf  -> success:true diagnostic_count:0. Probe C (in-body (drive HTRANS 0) with NO top-level named drive) FAILS: drive 'HTRANS' not defined -> an in-body (drive NAME ...) is a CALL to a top-level named drive, which is why .2b's enum-selector drive works (the emitter also emits the top-level drive). Faithfulness check: 13b-transactions.md L158 — samples in (on ...) fire at the entry transition (cycle N), an un-grounded capture timing for a recognition-only transaction whose activation_port is None."
---

**Measured + decided `2026-06-17` (`KG-ISF-TRANSACTIONS.2n`, measurement-first, read-only — a FRESH probe-first
cycle on the current FSMGen pin `030f8c273`, the lever the `2026-06-17` resume-pointer triage named "the
substantive buildable critical-path transaction lever").** The honest outcome: **NO-GO** — the transaction ISF
body is already faithfully complete, and the one unbuilt candidate (a value-free `(sample …)` membership body) is
FSMGen-accepted but NOT faithful.

## What the body carries today
`mint_named_transaction` (`ir/intent.rs`) composes body `steps` ONLY for the **Cue-B enum-selector** subset — a
transaction named after an enumerated value of a declared signal becomes `(drive SIG MEMBER)` (`idle_transfer` ⟺
`(drive HTRANS IDLE)`, `.2b`). That is the only body fact the document **grounds with both a value AND an order**
(a single drive is trivially ordered). Everything else — the `.2c`/`.2k` signal-set membership, the `.2i` phase
grouping, the `.2m` channel grouping — is checked IntentIR **metadata**, never a body step. A recognition-only
transaction (empty `steps`) is held out of the `.isf` by the `!tx.steps.is_empty()` emit filter (`isf_ir.rs`).

## The fresh empirical probe (pin `030f8c273`, `--strict --check --json`)
- **Probe A — pure value-free `(on start (sample HREADY as r)) (complete done)` over a declared input → `success:true`, 0 diagnostics.** A value-free sample body IS strict-valid.
- **Probe D — three samples in one `(on start …)` state → `success:true`, 0 diagnostics.** Multiple samples in the entry state carry no order *among themselves* (they piggyback, no extra cycle).
- **Probe B — `(sample HTRANS as t)` where `HTRANS` is an interface `(output …)` → `success:true`.** FSMGen does NOT gate `(sample …)` on signal direction.
- **Probe C — in-body `(drive HTRANS 0)` with NO top-level named drive → `success:false`: `drive 'HTRANS' not defined`.** An in-body `(drive NAME …)` is a CALL to a top-level named drive that must be defined elsewhere; `.2b` works because the emitter also emits the top-level drive block.

## Why a membership-derived sample body is NOT faithful (the decisive NO-GO reason)
FSMGen's book (`13b-transactions.md`, `(on port ...)` Entry/Idle State, L133–163) is explicit: a sample inside
`(on …)` fires **at the entry transition — "Cycle N: `port && can_accept` → samples captured"** (a D-input
capture in the idle→active cycle), and `(on …)` needs an activation guard `port`. For a recognition-only
transaction SpecForge has grounded only **which** signals participate — NOT that they are captured **once at the
entry cycle**, and NOT an activation port (`mint_named_transaction` sets `activation_port: None`). So
`(on start (sample HREADY as r))` would assert an entry-cycle capture timing + a `start` guard the document never
states — and worse, it could MISrepresent a *wait-for-ready* read as a one-shot entry sample. That is exactly the
fabrication the honest-residual doctrine (`[[feedback_isf_no_hacks]]`) and FSMGen's `2026-06-16` answer forbid:
**"emit body `drive`/`sample` steps ONLY for facts whose values AND ordering are grounded; keep membership as
metadata."** Membership ordering/placement is not grounded → it stays metadata.

## Decision
The transaction body lever is **exhausted / faithfully complete**: the only body-lowerable grounded fact (the
enum-selector drive) already ships (`.2b`), and the membership a sample body would carry is already faithfully
present as IntentIR metadata (`.2c` ports / `.2i` phase / `.2m` channel) — the home FSMGen explicitly chose. The
cross-`.isf` carriage of that membership awaits FSMGen's future **checked transaction phase-group metadata**
surface (FSMGen-owned, not shipped). This reconfirms the `.2i` parking decision with FRESH empirical evidence on
the current `030f8c273` binary (the prior probe was at `8c39827f`). Gates: read-only/docs-only — WIRE-BASED-100
untouched, ADR-0006 (no name list; the probe used universal grammar only). `[[transaction-capture-census]]` ·
`[[transaction-phase-membership-vlm-vs-channel]]` · `[[project_kg_isf_transactions]]` ·
`[[project_kg_isf_completeness]]` · `[[feedback_scoring_rigor]]`.
