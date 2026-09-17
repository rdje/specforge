---
id: a-gate-doctrine-has-no-stable-wall-clock-on-this-host
title: A gate doctrine has no stable wall clock here — one unchanged tree gave 6x, so a single run is not a measurement
answers:
  - "how long does the LIVE-DOC-SIZE doctrine take to run"
  - "can I trust a single timing run of a SpecForge doctrine gate"
  - "why do doctrine gate timings vary so much between runs"
  - "does a stale tree make the live-document-size gate slower"
  - "is COMMIT-GATE-SINGLE-RUN.0's per-doctrine cost table reliable"
  - "why is FAST_EXCLUDE membership quoted but never a share"
  - "what does measure_doctrine_cost.sh's load threshold actually guarantee"
  - "how many runs does a doctrine timing need before it can be published"
date: 2026-09-17
status: current
tags: [doctrine-enforcement, measurement, gates, commit-workflow]
evidence: docs/tasks/COMMIT-GATE-SINGLE-RUN.md; scripts/measure_doctrine_cost.sh; scripts/check_doctrines.sh
reverify: bash -c 'for i in 1 2 3; do s=$(date +%s); bash scripts/check_doctrines.sh --only LIVE-DOC-SIZE >/dev/null 2>&1; echo "run $i: $(( $(date +%s) - s ))s"; done'
---

`bash scripts/check_doctrines.sh --only LIVE-DOC-SIZE` was timed nine times on `2026-09-17`. Six runs on
a green tree and three on a tree with one stale input gave **87s, 89s, 96s, 214s, 220s, 266s** and
**87s, 256s, 506s** — a span of **5.8x**. Four of the green runs were consecutive, on a byte-identical
tree, at load average **4.90–5.51**: they read **89s, 214s, 220s, 266s**, a **3.0x** spread inside a 12%
load band.

**Nothing about the tree or the load predicts the number.** The fastest run of all and the slowest are
both stale runs; the second-fastest is green; and the 87s green run was taken at the *highest* load in the
set. Two mechanisms were eliminated by measurement rather than argument: the nested freshness verifier
`knowledge-map/scripts/check_knowledge_map.sh` costs 10s stale against 13s green, far too little to carry
a 170s delta, and `scripts/check_live_document_size.sh` invoked directly on the stale tree finished in
96.4s, indistinguishable from green. The remaining candidate is host I/O contention, which a load average
does not measure and this repository has no instrument for.

**So a single wall-clock reading of a gate doctrine is not evidence.** `COMMIT-GATE-SINGLE-RUN.0` recorded
`LIVE-DOC-SIZE` at 1m25.9s — at the very bottom of the observed range, against a median near 217s for the
green same-load group, understating it by roughly 2.5x. That table's shares were already withdrawn for
contention at load 12.95; this is the wider withdrawal, because the instability is present at load 5 on an
unchanged tree. `scripts/measure_doctrine_cost.sh` refusing above load average 2.0 is necessary and **not
sufficient**: the threshold cannot make one reading per doctrine honest.

**What survives is membership, and it survives for a structural reason.** Noise of this size reorders
neighbouring doctrines freely, but it cannot lift a sub-second doctrine past one that costs 90–500s. That
is exactly the split `COMMIT-GATE-SINGLE-RUN.0` observed without being able to explain — the costliest
four were the same four in all three of its runs while the ordering *within* those four was not stable.
`FAST_EXCLUDE` in the driver rests only on the part that survives, and no share is published anywhere.

The practical rule: publish min/median/max over repeated runs, or publish nothing. A timing table with one
number per row describes one sample of a distribution three times its own width.
See [[a-doctrine-subset-must-assert-the-leg-that-pays-for-the-rest]].
