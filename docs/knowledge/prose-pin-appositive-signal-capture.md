---
id: prose-pin-appositive-signal-capture
title: Interface signals declared in prose ("a clock pin, SWCLK") are captured via the pin-appositive pattern
answers:
  - "how are SWCLK and SWDIO captured if they are not in a signal table"
  - "how does specforge declare a signal mentioned only in prose"
  - "what is synthesize_signal_declarations_from_prose / the pin-appositive pattern"
  - "how are serial/architecture spec interface signals added to the catalog"
date: 2026-06-07
tags: [swd, adi, serial, extraction, signal-declaration, adr-0006]
evidence: docs/tasks/SWD-SERIAL-EXTRACTION.md (.2); crates/specforge/src/ir/evidence.rs (synthesize_signal_declarations_from_prose)
reverify: python3 -c "import json,re; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); d={re.match(r'Signal (\w+)',s['text']).group(1) for s in e['extracted_statements'] if s['text'].startswith('Signal ')}; print('SWCLK',('SWCLK' in d),'SWDIO',('SWDIO' in d))"
---

Serial/architecture specs (the Arm Debug Interface, IHI0074) introduce their wire contract NOT in a
signal-description table but in **prose appositives**: "The SWD interface … requires a clock pin, **SWCLK**";
"a single bidirectional data pin, **SWDIO**". The table-only declaration path therefore misses them
(see `[[swd-adi-not-signal-table-spec]]`).

`synthesize_signal_declarations_from_prose` (wired into `EvidenceIr` build alongside the table path)
recovers them: it scans statements for the noun **"pin" immediately naming a signal across a comma** —
fused (`pin,` SIG) or separate (`pin` `,` SIG) — and emits a width-1 declaration (`Signal SWCLK is width
1.`) so the signal enters the declared catalog. Grammar, not names (ADR 0006). Additive: duplicates of
table-declared signals dedupe downstream, so APB/AHB/AXI are unaffected.

Guardrails: the candidate must pass `is_hardware_signal_token` (leading letter, see
`WIRE-BASED-100.5j`) and not be a non-signal word — cross-references like "… pin, **see** Figure B4-3"
are suppressed via `SEE` in `is_signal_synthesis_non_signal`. Result (SWD-SERIAL-EXTRACTION.2): `SWCLK`,
`SWDIO`, and `NSRST` (a real ADI system-reset) now declared; no parallel-bus regression. This is the
foundation the SWD relation/temporal leaves (`.3`–`.5`) build on.
