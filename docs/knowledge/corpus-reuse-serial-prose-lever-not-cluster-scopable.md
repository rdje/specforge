---
id: corpus-reuse-serial-prose-lever-not-cluster-scopable
title: The .9.10 serial-prose bus-line lever cannot be a CORPUS-PATTERN-REUSE first opt-in extractor — its safe/noisy split is lexical, orthogonal to the structural fingerprint (CORPUS-PATTERN-REUSE.3b.3a measured NO-GO)
answers:
  - "can the prose bus-line signal lever (PDF-VARIANT-DIGESTION.9.10) be cluster-scoped as a CORPUS-PATTERN-REUSE opt-in extractor"
  - "what is the first opt-in extractor for CORPUS-PATTERN-REUSE.3b.3 / activate-only consume"
  - "do SMBus / I2S / I2C cluster into a derived serial-bus family"
  - "why can't cluster-scoping replace the forbidden supply-rail (VDD/VSS) denylist"
  - "what makes a valid activate-only opt-in extractor candidate"
  - "is CORPUS-PATTERN-REUSE.3b.3a a go or no-go"
date: 2026-06-15
tags: [corpus-pattern-reuse, activate-only, opt-in-extractor, corpus-cluster, prose-bus-line, pdf-variant-digestion, measured-no-go, adr-0006, denylist, structural-fingerprint]
evidence: docs/tasks/CORPUS-PATTERN-REUSE.md (.3b.3a / Decisions 2026-06-15); docs/tasks/PDF-VARIANT-DIGESTION.md (.9.10 probe + owner steer); crates/specforge/src/ir/corpus_cluster.rs (document_fingerprint / cluster_documents); crates/specforge/src/ir/prior_memory.rs (extraction_profile_priors_for — multi-member clusters only)
reverify: "./target/release/specforge corpus-cluster  # SMBus / I2S / I2C / OpenCAPI-TL are singletons; eMMC pairs with GIC (register family). Then re-derive Form A: python3 over generated/evidence_ir/*/evidence_ir.json matching r'\\b[Tt]he\\s+([A-Z][A-Z0-9_]*#?)\\s+line' → fires on 5/78 docs (SMBus SMBCLK/SMBDAT/SMBSUS#, I2S WS, I2C SCL/SDA+VDD/VSS/DLEN, eMMC CMD/DAT+VDD, OpenCAPI-TL AFUC2 false-pos), 0 wire-based"
---

**`CORPUS-PATTERN-REUSE.3b.3a` (`2026-06-15`) measured the leading first-opt-in-extractor candidate — the
parked `PDF-VARIANT-DIGESTION.9.10` prose bus-line lever ("the/The `<ALLCAPS-id>[#]` line") — a NO-GO.** A
universally-safe extractor should just be default-on; the activate-only machinery only earns its keep for a
lever that is helpful on one derived cluster but too noisy to run corpus-wide. The hypothesis was that the
`.9.10` lever (parked because a universal version needs a forbidden supply-rail denylist) could be made the
first opt-in extractor by SCOPING it to the cluster where it is safe. Measurement refutes that.

**What the read-only measurement (no code/extraction-path change) showed:**

1. **Faithful Form-A re-derivation over the 78 persisted `evidence_ir` docs reproduces the probe: fires on
   5/78, 0 wire-based.** Genuine benefit on only **2** docs — SMBus (`SMBCLK`/`SMBDAT`/`SMBSUS#`) and I2S
   (`WS`). The other 3 firings are noise: I2C (`VDD`/`VSS`/`DLEN` on top of real `SCL`/`SDA`), eMMC (`VDD` on
   top of real `CMD`/`DAT`), and OpenCAPI-TL where Form A FALSE-POSITIVES `AFUC2` out of cache-line prose.

2. **No derived 2-wire-bus family exists.** At the SYSTEM threshold (0.6) all benefit/noise docs are structural
   SINGLETONS (SMBus, I2S, I2C, OpenCAPI-TL) except eMMC, whose only cluster partner is GIC — a register-heavy
   interrupt-controller spec, not a bus. A threshold sweep (0.40–0.60) finds no clean bus family at any cut:
   singletons at ≥0.55; at ≤0.50 they dissolve into 15–32-doc heterogeneous catch-alls sharing only the trivial
   ABSENCE token `shape:serial_frame:b0`. So there is no multi-member cluster signature to learn a profile for
   (and singletons are never persisted — a cluster of one carries no cross-document pattern).

3. **The split is structurally invisible.** SMBus (safe) and I2C (noisy) are both 2-wire buses with
   near-identical structural shape; the ONLY thing separating them is whether the doc's prose literally says
   "the `VDD` line" — a **lexical** property the structural fingerprint cannot see. Any cluster broad enough to
   carry SMBus's benefit also carries I2C's harm, and **I2C is a MEASURED doc (declared-signal gold precision
   0.600)**, so activating there REGRESSES a tracked score ([[feedback_scoring_rigor]]). The persisted
   `learn-priors` profile confirms it from the other side: I2C sits in a support-5 bus-SHAPE profile
   (TileLink/I2C/HBM2/GFB) while SMBus/I2S are absent from the harvest — the only learnable bus-ish cluster holds
   the regression doc and NOT the benefit docs.

**Why it generalizes (the reusable rule):** cluster-scoping is a STRUCTURAL gate; the `.9.10` discrimination is
LEXICAL (supply-rail names). A structural gate inherits the exact blindness that made the supply-rail DENYLIST
forbidden ([[feedback_avoid_denylists_prefer_structural]]), so it cannot resolve the lever's parking reason.
The serial-prose levers (`.9.10`/`.9.8b`) are lexically-discriminated by construction and genuinely need
participation-based signal identity (the `NLP-SHALLOW-PARSE` path, itself measured build-exhausted —
[[actor-signal-direction-passive-active-handled]]), NOT the reuse plane. **Criterion for a valid first opt-in
extractor:** it must be safe *because of* a structural property the fingerprint captures (so a derived
multi-member cluster cleanly separates "activate here" from "noisy there"). `.3b.3a2` continues the read-only
search under that criterion (e.g. the CCIX message-field family, whose distinctive `fired:` signature is a real
structural discriminator); `.3b.3b` (build the activate-only consume contract) stays correctly gated until such
a candidate is found. Honors the measured-DEFER/NO-GO precedents (NLP-SHALLOW-PARSE.2h/.2f,
MEMORY-BOUNDED-INGEST.5, FULL-PAGE-INTENT-CAPTURE.1) and ADR 0006 (no name lists; structural how, not names).
