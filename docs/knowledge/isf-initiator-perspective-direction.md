---
id: isf-initiator-perspective-direction
title: The emitted `.isf` interface now lowers grounded actor-relative signal DIRECTION from the protocol's INITIATOR actor's perspective (owner-authorized) — initiator = the net-producer actor (outputs > inputs, max (out,in)) recovered structurally from the actor-port graph (ADR 0006, no name list); a signal the initiator Drives → `(output)`, Reads → `(input)`, ungrounded → `(output)` honest residual; the module is also named after that initiator; FSMGen-strict-safe (0 new diagnostics — drives auto-suppressed for inputs)
answers:
  - "how does SpecForge decide signal direction (input/output) in the emitted .isf"
  - "what is the initiator-perspective direction emission / KG-ISF-COMPLETENESS.2a.ii"
  - "how is the protocol initiator actor identified structurally without a name list"
  - "why is the .isf module named after the initiator (manager / requester / debugger) instead of actors.first()"
  - "does emitting (input) signals break fsmgen --strict (no — 0 new diagnostics; drives are suppressed for inputs)"
  - "why did the .2a direction deferral get reopened (explicit owner steer 2026-06-17 — Build it, initiator perspective)"
  - "per-wire-doc direction flip results (APB input 2->12, AXI 4->52, SWD 0->1, AHB residual)"
  - "why does AHB not flip any signal to input under Manager perspective (sparse stale grounding — Manager's only graph inputs are HCLK/HRESETN, excluded as clock/reset)"
  - "what is select_initiator_actor / initiator_perspective_directions"
  - "is signal direction faithful to the document now or still defaulted to output"
date: 2026-06-18
tags: [kg-isf-completeness, isf, direction, initiator, actor-ports, emitter, owner-authorized, fsmgen-strict, adr-0006, honest-residual, north-star, measured]
evidence: crates/specforge/src/ir/isf_ir.rs (select_initiator_actor + initiator_perspective_directions + the signal-loop direction decision); crates/specforge/src/ir/adapters.rs (derive_isf_actor_name prefers the initiator); docs/tasks/KG-ISF-COMPLETENESS.md (.2a.ii node); docs/decisions / AskUserQuestion 2026-06-17 (owner chose "initiator perspective"); generated/intent_ir/<wire>/intent_ir.json (actor_ports)
reverify: "RAM-safe, no VLM. cargo build --release -p specforge. For each wire doc: target/release/specforge adapt generated/intent_ir/<key>/intent_ir.json --target isf --dry-run | awk 'f{print} /^adapter_json:/{f=1}' | jq -r '.isf.source_text' -> grep -c '(input ' / '(output '. Initiator cue: jq over .actor_ports group_by(actor_name), net producer (out>in) maximizing (out,in) -> AHB Manager(6,2)/APB Requester(20,12)/AXI Manager(116,52)/SWD debugger(2,1). Strict-safety (baseline-vs-after via git stash): perl subs/fsmgen/bin/fsmgen --strict --check --json on the emitted .isf -> 0 NEW diagnostics on all 4 wire docs (AHB HAUSER + AXI ASKSTOP pre-existing rule-write conflicts unchanged; APB/SWD PASS). Measured 2026-06-18: APB input 2->12, AXI 4->52, SWD 0->1, AHB 0->0 (honest residual)."
---

**Built `2026-06-18` (`KG-ISF-COMPLETENESS.2a.ii`, measurement-first, CODE — OWNER-AUTHORIZED).** The emitted
`.isf` interface now lowers the document-grounded, actor-relative signal DIRECTION instead of defaulting
non-`Input` signals to `(output)`. This is a north-star FAITHFULNESS fix: a signal the perspective actor READS
must be `(input)`, not a defaulted `(output)`.

## Why it needed an owner decision
Direction in a protocol is actor-relative (a signal one actor drives, another reads), but the emitted `.isf` is a
SINGLE flat module — so it must be lowered from ONE actor's perspective. `R6-ISF-ADAPTER.4` deliberately defaulted
all non-`Input` signals to `(output)`, and `KG-ISF-COMPLETENESS.2a` measured that direction is FSMGen-strict-NEUTRAL
(flipping a driven `(output)`→`(input)` still passes `--strict`), so it deferred direction as "low-value, revisit
only on explicit owner steer." The owner gave that steer (AskUserQuestion, `2026-06-17`): **"Build it, initiator
perspective"** — reframing it as a faithfulness gap (FSMGen ignoring direction does not make a defaulted-output
faithful).

## How the initiator is identified (structural, ADR 0006 — no name list)
`select_initiator_actor` (`ir/isf_ir.rs`): the initiator drives the request and reads the response, so it is the
actor whose OUTPUT (`Drives`) ports strictly exceed its INPUT (`Reads`) ports — a **net producer** — with the
largest footprint, picked as the net producer maximizing `(outputs, inputs)` lexicographically. `out > in` excludes
a balanced prose-fragment actor (AHB `address decoder`, out=in) and an input-dominant completer
(`Subordinate`/`Completer`); the `(out, in)` tiebreak prefers a real initiator (also reads responses) over an
output-only register fragment. No net producer → `None` → the prior default-`output` behavior (honest residual,
byte-identical). Measured initiators: AHB `Manager` (6/2), APB `Requester` (20/12), AXI `Manager` (116/52), SWD/debug
`debugger` (2/1) — all correct.

## How direction is emitted + why it is strict-safe
`initiator_perspective_directions` builds the initiator's per-signal direction map (`Drives`→`Output`, `Reads`→
`Input`; a signal it both drives and reads, or touches only as `InOut`/`Unknown`, is OMITTED → honest residual).
The signal loop prefers that map, else falls back to the flat hint, else `(output)`. The module is also NAMED after
the same initiator (`derive_isf_actor_name`, `ir/adapters.rs`) so the label and its interface stay coherent (AHB
`address_decoder`→`manager`, APB `apb_protocol`→`requester`, AXI `agent`→`manager`, SWD `agent`→`debugger`).
**Strict-safe:** the per-output named-drive block is already filtered to `IsfDirection::Output`, so a signal flipped
to `(input)` is automatically NOT driven (FSMGen rejects driving an input) — verified: 0 NEW `--strict` diagnostics
on all 4 wire docs (AHB `HAUSER` + AXI `ASKSTOP` pre-existing rule-write conflicts unchanged; APB/SWD still PASS).

## Measured flip (2026-06-18, baseline-vs-after via `git stash`)
- **APB** → Requester: input **2 → 12**, output 30 → 20 (faithful: drives PADDR/PWDATA/PWRITE/PSEL/PENABLE…, reads PRDATA/PREADY/PSLVERR/PBUSER…).
- **AXI** → Manager: input **4 → 52**, output 283 → 235.
- **SWD** → debugger: input **0 → 1**, output 13 → 12.
- **AHB** → Manager: input **0 → 0** (HONEST RESIDUAL — the stale persisted intent grounds Manager only to 6 sideband outputs + HCLK/HRESETN inputs, which are excluded as clock/reset; the rich HADDR/HREADY/HRDATA signals carry no Manager relation in that artifact, so they default to `(output)`. A fresh post-`.1a` rebuild would flip more — the emitter faithfully reflects whatever the graph grounds).

Gates: `run_ci.sh` GREEN (lib 1677→1679, +2 tests); `kg-bench` 156/156; WIRE-BASED-100 orthogonal (emitter-only,
downstream of all extraction); ADR-0006; honest residual over fabrication. `[[fsmgen-ignores-signal-direction]]` ·
`[[isf-lowering-fidelity-gauge]]` · `[[project_kg_isf_completeness]]` · `[[feedback_no_hardcoded_chip_spec_names]]` ·
`[[feedback_isf_no_hacks]]`.
