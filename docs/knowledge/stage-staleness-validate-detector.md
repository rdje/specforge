---
id: stage-staleness-validate-detector
title: validate emits a stage_staleness Warning when a downstream IR carries 0 actor_signal_relations while its upstream carries some — false-positive-free (gating never empties a non-empty set), so it catches the silent tilelink-39->0 staleness class
answers:
  - "does validate detect a stale downstream artifact that silently dropped relations"
  - "what is the intent_stale_relations_dropped / semantic_stale_relations_dropped finding"
  - "how does validate know an intent_ir is stale relative to its semantic_ir"
  - "why is the stage-staleness check zero-versus-some and not a count comparison"
  - "does the stage-staleness detector false-fire on register/command docs with 0 relations (no — both empty)"
  - "how does validate reach the upstream artifact (carried semantic_ir_path / evidence_ir_path)"
  - "what does CORPUS-COVERAGE.1 add"
date: 2026-06-17
tags: [validate, stage-staleness, completeness, actor-signal-relations, corpus-coverage, false-positive-free, adr-0006, code]
evidence: crates/specforge/src/commands/validate.rs (stage_staleness_relation_finding helper + the let-chain guards in validate_intent_ir / validate_semantic_ir + 3 unit tests); docs/tasks/CORPUS-COVERAGE.md (.1); docs/book/src/quality/validation.md ("Catching a stale downstream stage")
reverify: "cargo test -p specforge --lib stage_staleness -> 3 pass. Live (temp-CWD to avoid the WRITE-PATH GOTCHA): synth an intent with actor_signal_relations=[] + document_key=demo + semantic_ir_path=<abs real semantic w/ N>0 rels>; run the release binary `validate /tmp/x.json` from a throwaway CWD -> prints `[warning:stage_staleness] this IntentIR carries 0 actor_signal_relations but its upstream SemanticIR carries N ...`. nvme (intent 0 / semantic 0) stays silent (honest absence); a healthy doc (N>0 rels) stays silent."
---

**Landed `2026-06-17` (`CORPUS-COVERAGE.1`, CODE).**

The pipeline stages (`evidence → semantic → intent`) do not auto-cascade — only `converge` rebuilds the whole
chain. Rebuilding one stage without cascading leaves the downstream **stale**: it holds the old result while
the upstream moved on. The sharp form is silent loss — an IntentIR carrying 0 `actor_signal_relations` while
its SemanticIR upstream carries dozens (the measured `tilelink` 39→0 class, `[[relation-completeness-staleness-vs-absence]]`).

`validate <intent-ir>` / `<semantic-ir>` now catches it: when the downstream carries no actor-signal
relations, it loads the upstream (via the carried `semantic_ir_path` / `evidence_ir_path` — `validate`
already loads upstream for graph-aware findings) and, if the upstream carries relations, raises a
`stage_staleness` **Warning** (`intent_stale_relations_dropped` / `semantic_stale_relations_dropped`) telling
the operator to re-run that stage (or `converge`).

**Why zero-versus-some, not a count comparison:** it is false-positive-free. The agent-identity gates that
clean the actor surface (consolidation `.1b.i`, coordinated split `.1b.iii`, interface fold `.1b.ii`, phantom
drop `.1b.iv`) only ever *re-attribute* or merge relations — they NEVER empty a non-empty set. So an
empty-downstream/non-empty-upstream split can only be staleness. A register/command/coherency protocol that
genuinely has 0 wire-signal relations has an empty UPSTREAM too (0-vs-0) → correctly silent (honest absence is
not a stale drop — `[[corpus-coverage-buildout]]`). The upstream I/O is paid only when the downstream is empty
(a `let`-chain short-circuit), and skipped when the upstream is off-disk (detached copy). Pure decision helper
`stage_staleness_relation_finding` (+3 unit tests); ADR-0006 (structural, no name list); WIRE-BASED-100
unaffected (validate-only additive finding; wire docs carry relations → silent).
