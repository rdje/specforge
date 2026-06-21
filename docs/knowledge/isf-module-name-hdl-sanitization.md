---
id: isf-module-name-hdl-sanitization
title: The emitted `.isf` module name (and every internal `.isf` identifier) is HDL-sanitized by `sanitize_isf_name`, an ALLOWLIST (keep `[A-Za-z0-9_]`, map every other char to `_`) — so a prose-fragment initiator actor name carrying punctuation (e.g. the unicode arrow `→`) no longer malforms the whole `.isf`; the prior denylist missed `→` (KG-ISF-COMPLETENESS.2a.iii, surfaced by CORPUS-COVERAGE.2 on GIC-600)
answers:
  - "why does the emitted .isf module name get sanitized / how is the (actor <name>) label derived"
  - "what is KG-ISF-COMPLETENESS.2a.iii (ISF module-name HDL-sanitization)"
  - "why did GIC-600's .isf fail fsmgen strict with 'Malformed top-level FSM source ?fsm:redistributor→…'"
  - "how does derive_isf_actor_name produce a valid HDL identifier ([A-Za-z_]\\w*)"
  - "what does sanitize_isf_name do (allowlist [A-Za-z0-9_] -> everything else becomes _)"
  - "why an allowlist instead of a denylist for .isf identifier sanitization (a denylist can't enumerate every bad char — it missed the arrow →)"
  - "does sanitizing the module name break initiator port matching (no — from_intent_ir re-derives the initiator raw; actor_name is only the label)"
  - "are the wire-gold .isf affected by the module-name sanitization (no — byte-identical; their names are pure alphanumeric, so the allowlist is a no-op)"
date: 2026-06-21
tags: [kg-isf-completeness, isf, emitter, module-name, hdl-identifier, fsmgen-strict, adr-0006, allowlist, corpus-coverage, sanitization, fix]
evidence: crates/specforge/src/ir/isf_ir.rs (sanitize_isf_name — allowlist; pub(crate)); crates/specforge/src/ir/adapters.rs (derive_isf_actor_name routes the module label through sanitize_isf_name); docs/tasks/KG-ISF-COMPLETENESS.md (.2a.iii node); docs/tasks/CORPUS-COVERAGE.md (.2 Lever B, GIC-600 re-ingest #14)
reverify: "RAM-safe, no VLM. cargo build --release -p specforge. GIC-600 (re-ingested): target/release/specforge adapt generated/intent_ir/100336_0106_00_2019_02_08_gic_600_technical_reference_manual/intent_ir.json --target isf -> emitted .isf head is '(actor redistributor_distributor_distributor_redistributor' (valid id); perl subs/fsmgen/bin/fsmgen --strict --check --json <isf> -> success=true, 0 diagnostics (module-name error cleared). Wire golds byte-identical: APB '(actor requester'/SWD '(actor debugger' (0 diag), AHB/AXI '(actor manager' (only the pre-existing HAUSER/ASKSTOP rule-conflicts — 0 NEW). Unit: sanitize_isf_name('redistributor→_distributor') == 'redistributor_distributor'. run_ci.sh GREEN (lib 1679); kg-bench 156/156."
---

**Built `2026-06-21` (`KG-ISF-COMPLETENESS.2a.iii`, CODE — owner-chosen "Fix ISF emitter bug B").** Surfaced by
`CORPUS-COVERAGE.2` re-ingest #14 (GIC-600): FSMGen `--strict --check` rejected the WHOLE `.isf` with
`Malformed top-level FSM source '?fsm:redistributor→_distributor…'. expects '?fsm:name' with an
HDL-identifier-compatible module name ([A-Za-z_]\w*)`.

## Root cause
The `.2a.ii` initiator-named module (`derive_isf_actor_name`, `ir/adapters.rs`) did its OWN minimal
`.replace([' ', '-', '.'], "_")`, which left the unicode arrow `→` (and any other non-`[A-Za-z0-9_]` char) in the
name. GIC-600's net-producer initiator was a prose-fragment actor `redistributor → distributor distributor →
redistributor`, so the emitted `(actor …)` header — and FSMGen's derived `?fsm:…` module name — was a malformed
identifier, which fails the strict frontend before anything else can be checked.

## Fix (allowlist, not denylist)
`sanitize_isf_name` (`ir/isf_ir.rs`) was a char DENYLIST enumerating ASCII punctuation — which can never cover
every offender and in fact missed `→`. It is now a char ALLOWLIST: lowercase, then keep `[A-Za-z0-9_]` and map
every other char to `_` (then the existing collapse-`__` / trim / empty→`unnamed` / leading-digit→`reg_` guards).
This is exactly FSMGen's `[A-Za-z_]\w*` contract, universal and name-list-free (ADR 0006), and aligns with the
owner's "prefer structural gates over denylists" steer (`[[feedback_avoid_denylists_prefer_structural]]`).
`derive_isf_actor_name` now routes the module label through this shared sanitizer (one rule, no drift).

## Why it is safe (no wire-gold regression)
- The allowlist is **byte-identical** to the prior denylist on every ASCII-punctuation input the denylist already
  covered; it only ever changes a name that was **already broken**. Clean names are unchanged: `Manager`→`manager`,
  `Requester`→`requester`, `debugger`. So the 4 wire-gold `.isf` are byte-identical.
- Sanitizing the module LABEL cannot affect initiator selection: `from_intent_ir` re-derives the initiator RAW
  internally for direction matching (`select_initiator_actor` + `initiator_perspective_directions`); the passed
  `actor_name` is used only for the `(actor …)` label.

## Verified
GIC-600 module header → `(actor redistributor_distributor_distributor_redistributor`, FSMGen `success=true`,
**0 diagnostics** (module-name error cleared — the whole `.isf` now lowers strict-valid). Wire golds:
APB/SWD 0 diag, AHB/AXI only the pre-existing `HAUSER`/`ASKSTOP` rule-conflicts → **0 NEW diagnostics**.
`run_ci.sh` GREEN (lib 1679); `kg-bench` 156/156; WIRE-BASED-100 orthogonal (emitter-only).
`[[isf-initiator-perspective-direction]]` · `[[project_kg_isf_completeness]]` ·
`[[feedback_avoid_denylists_prefer_structural]]` · `[[feedback_no_hardcoded_chip_spec_names]]`.
