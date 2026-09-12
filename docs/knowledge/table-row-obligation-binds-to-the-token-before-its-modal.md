---
id: table-row-obligation-binds-to-the-token-before-its-modal
title: A table cell's obligation belongs to whatever immediately precedes its modal, and the whole-statement subject scan ignores that — which is how HBURST_WIDTH must be 0 or 3 became HBURST must be 0
answers:
  - "why does AHB say HBURST must_be_value 0 when the document says HBURST_WIDTH must be 0 or 3"
  - "why is HPROT constrained to 0 in the persisted AHB evidence"
  - "how does a width parameter's obligation become a signal constraint"
  - "why does is_post_passive_binding_only_subject exempt a table row"
  - "which subject does a signal-description row's description cell constrain"
  - "why does RRESP get no signal constraint from its own table row"
  - "what is obligation_subject / ObligationSubject"
  - "how does specforge decide whether a description cell constrains the row's signal"
  - "why does a subjectless obligation clause produce no constraint from the statement path"
  - "which documents carry obligation-bearing signal-description rows"
date: 2026-09-12
status: current
tags: [evidence-ir, signal-constraints, precision, table-semantics, adr-0006, invariant-shape-admission]
evidence: crates/specforge/src/ir/evidence.rs (obligation_subject; extract_signal_description_row_constraints; is_post_passive_binding_only_subject gate 2; collect_subject_signal_tokens; extract_dynamic_signal_constraints); scripts/measure_signal_row_obligation_subject.py; docs/tasks/INVARIANT-SHAPE-ADMISSION.md (.3)
reverify: "python3 scripts/measure_signal_row_obligation_subject.py — expect 20 obligation clauses over 567 declared signal-description rows, split absent 1 / self 11 / pronoun 5 / other 3; then `python3 -c \"import json;d=json.load(open('generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json'));print([(c['constraint_id'],c['subject_signal'],c['constraint_kind']) for c in d['signal_constraints'] if c['subject_signal'] in ('HBURST','HPROT')])\"` — expect dyn_sigcon_0013/0014 with must_be_value 0"
---

**Established `2026-09-12` (`INVARIANT-SHAPE-ADMISSION.3`).** English binds an obligation to the
nominal that **immediately precedes** its modal. A signal-description row's description cell is prose
*about* a signal, not a sentence *whose subject is* that signal, so an obligation inside it may be
about something else entirely.

Two deterministic paths mint a `SignalConstraintRecord` from a serialized table row by scanning the
**whole statement** for declared-signal tokens. That scan cannot tell these apart:

| the cell says | the obligation is about | the scan attributes it to |
| --- | --- | --- |
| `… HBURST_WIDTH must be 0 or 3.` | the width **parameter** | `HBURST` — `dyn_sigcon_0013`, `must_be_value 0` |
| `… HPROT_WIDTH must be 0, 4, or 7 …` | the width **parameter** | `HPROT` — `dyn_sigcon_0014`, `must_be_value 0` |
| `Indicates which tags must be written to memory …` | the **tags** | `WTAGUPDATE` |
| `PSTRB must not be active during a read transfer.` | `PSTRB` | `PSTRB` — correct, by accident of the cell repeating its own name |

The two AHB records are **fabricated twice over**: wrong subject, and a value (`0`) that is one
alternative of a set the document writes as `0 or 3`.

`is_post_passive_binding_only_subject` is the predicate that would refuse exactly this — its own
doc-comment says a passive obligation binds to a subject that PRECEDES the modal — but its **gate 2
exempts a table row**: *"a table row supplies subject context from its other cells → out of scope."*
That exemption is right for the case it was written for and wrong here. It assumes the row's subject
context is always the obligation's subject; when the description cell names a different one, the
exemption hands the row's name-cell signal to an obligation that was never about it.

## The symmetric loss

The same blindness drops a real constraint. `| RRESP | RRESP_WIDTH | 0b000 (OKAY) | Response for
transactions on the read channels. Must be valid when RVALID is asserted. |` is classified
`signal_value_constraint` and still yields nothing, because the clause has **no subject at all**:
after narrowing to the obligation and cutting the condition, the subject scan is left with
`Must be valid`, whose three words all pass the permissive `is_hardware_signal_token` identifier test,
so the full-text fallback never runs — and then all three are dropped as undeclared.

## The rule

`obligation_subject` reads the clause's last content token before its modal and returns
`NotAnObligation` / `Absent` / `Head(token)`. `extract_signal_description_row_constraints` admits a
clause only when it is `Absent` (only the row's header can supply a subject) or `Head` equal to the
row's own declared signal. A pronoun head is reported as a head and refused: `| HWRITE | … it must
remain constant … |` means the signal but `| HSELx | … When the Subordinate is initially selected, it
must also monitor … |` means the Subordinate, and telling them apart is anaphora, not a rule.

Universal grammar, no name lists (ADR 0006). Measured over the proof-carrying corpus: 20 obligation
clauses across 567 declared signal-description rows — `absent` 1, `self` 11, `pronoun` 5, `other` 3.

**The refusal half is not yet applied to the two paths that already mis-attribute.** `.3` adds the
row reader; the `dyn_sigcon_0013`/`0014` records stand until a leaf owns the gate-2 exemption
(`INVARIANT-SHAPE-ADMISSION.5`).

The whole population lives in APB, AXI and AHB, whose normalized bundles are held out, so observing
either half in a rebuilt artifact means restoring them first from
`generated/preserved/WIRE-BASED-100.10/{apb,ahb,axi}-normalized-bundle-held-out/` and removing them
after — the procedure in `[[evidence-rule-field-content-stales-every-proof]]`. Reaching for that card
BEFORE concluding the work is blocked is the point: `.3` spent a measurable detour deciding the chain
was unrebuildable because `specforge evidence` reports only `path does not exist: …/normalized/<key>.md`,
which names the missing input and not the place it is kept.
