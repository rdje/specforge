---
id: canonical-promotion-output-path-artifact-layout
title: specforge stage commands write outputs to the canonical path from the artifact's recorded artifact_layout, NOT the input path — so promotion in place is correct, but running a command on a *.bak/copy clobbers canonical
answers:
  - "where does nli-verify / extract-constraints-llm / semantic / intent / adapt write their output"
  - "does specforge write to the input path I pass or to a canonical generated path"
  - "why did running nli-verify on a .prepromote.bak overwrite the real evidence_ir.json"
  - "is it safe to measure a backup copy of an evidence_ir with nli-verify"
  - "how does the no-re-ingest canonical promotion protocol stay in place"
date: 2026-06-15
tags: [canonical-promotion-sweep, artifact-layout, nli-verify, extract-constraints-llm, output-path, gotcha, llm-primary-promotion]
evidence: crates/specforge/src/commands/nli_verify.rs (module doc — "write_to_disk honors the recorded artifact_layout"); crates/specforge/src/ir/* persisted artifacts carry an artifact_layout; docs/tasks/CANONICAL-PROMOTION-SWEEP.md (.3 sweep protocol)
reverify: "Take any generated/evidence_ir/<key>/evidence_ir.json, cp it to /tmp/copy.json, run `specforge nli-verify /tmp/copy.json --vlm-provider skip`; observe the GAUGE/back-annotation lands on generated/evidence_ir/<key>/evidence_ir.json (the canonical path from artifact_layout), not on /tmp/copy.json."
---

**specforge stage commands persist by `artifact_layout`, not by the input path you pass.** Each persisted
IR artifact (`source_ir`/`evidence_ir`/`semantic_ir`/`intent_ir`/adapter) carries a recorded
`artifact_layout` that encodes its canonical `generated/<stage>/<document_key>/...` home. When a command
loads an artifact and writes back (e.g. `nli-verify` back-annotating its `extraction_quality_gauge`,
`extract-constraints-llm` replacing `signal_constraints`, `semantic`/`intent`/`adapt` materializing the next
stage), `write_to_disk` **honors that recorded layout** — it writes to the canonical path, ignoring whatever
path string you handed on the CLI.

**Why this matters two ways:**

1. **The no-re-ingest canonical-promotion protocol is correct *because* of this.** Passing the canonical
   `generated/evidence_ir/<key>/evidence_ir.json` to `extract-constraints-llm` rewrites that same canonical
   file in place — exactly the canonical mutation the `CANONICAL-PROMOTION-SWEEP` wants, with zero re-ingest.
   The downstream `semantic`/`intent`/`adapt` then write their own canonical paths by `document_key`.

2. **GOTCHA — never run a command on a `*.prepromote.bak` or any copy.** Doing so loads the *copy's* content
   but writes back to the *canonical* path, **clobbering** the real artifact with the backup's content. This
   bit the `.3` sweep once: `nli-verify <evidence_ir>.prepromote.bak` (an attempt to measure the pre-promote
   surface) silently reverted the canonical `evidence_ir.json` to the backup's Pattern surface. To measure a
   pre-promote/backup surface, copy it OUT to a path whose `artifact_layout` won't collide, or just measure
   BEFORE in-flow on the canonical file before the promote step overwrites it (what the sweep driver does).

**Consequence for the sweep driver:** always pass canonical paths; capture the BEFORE gauge from the
canonical file right after the BEFORE `nli-verify` and before `extract-constraints-llm` drops it; back up
with `cp` to `*.prepromote.bak` for revert, but revert by `cp`-ing the backup back over the canonical path
(a plain file copy), never by *running a command* on the backup.
