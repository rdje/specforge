---
id: containment-numbers-are-type-checked-at-decode
title: A containment registry's numbers must be JSON numbers, and the type is checked once at decode because Perl cannot recover it after first use
answers:
  - "can a Perl gate tell the JSON string \"1643\" from the number 1643"
  - "why does check_live_document_size.pl decode every registry a second time"
  - "does the live-document checker reject string-encoded numbers"
  - "why did my quoted bytes value fail LIVE-DOC-SIZE"
  - "field holds the string numeric evidence must be a JSON number"
  - "a numeric field must be a number everywhere"
  - "how does JSON::PP mark a decoded number versus a string"
  - "why does a digit regex not catch a quoted number in a contract"
  - "which registries does the numeric-type check cover"
date: 2026-09-23
status: current
tags: [live-documents, doctrine, containment, json, perl, tooling]
evidence: scripts/check_live_document_size.pl (containment_registry_paths, validate_numeric_evidence_encoding, collect_scalar_types); scripts/test_live_document_size.pl (the four .40 cases); LIVE_DOCUMENT_SIZE_CONTAINMENT.md (Registry and mechanical enforcement); docs/tasks/live-document-pressure-headroom/contract-encoding.md (.40)
reverify: "perl scripts/test_live_document_size.pl --quiet — expect the declared total to pass, including 'a quoted numeral in the surface registry is refused'. Then quote one part's bytes value in doctrine/live_document_size/claim_verification_task_evidence.json, run bash scripts/check_live_document_size.sh — expect \"field 'bytes' holds the string\" — and restore it with git checkout."
---

Every registry beside `doctrine/live_document_size/surfaces.jsonl` (19 on `2026-09-23`: the surface
registry, the task-evidence contracts, the catalogs, the ledger and archive contracts) must store a
bound, measurement, baseline, or delta as a **JSON number**. `check_live_document_size.pl` refuses:

1. a string that is a decimal numeral of at most 15 integer digits, under **any** key. Every count is
   below 10^15, and the shortest identity these registries hold is a 16-hex-digit fingerprint, so an
   identity that happens to be all decimal digits is never mistaken for a number;
2. a string under a key that is a JSON number anywhere else in the population, which also refuses
   `"1,643"` and `"1643 bytes"`.

**Why a separate pass, and why at decode.** Before `LIVE-DOCUMENT-PRESSURE-HEADROOM.40` every loader in
this family accepted `"42768"` exactly as it accepted `42768`: they read values with a digit regex or a
numeric comparison, and both treat the quoted form identically. The type cannot be recovered later.
JSON::PP (4.06, Perl 5.34 here) gives a decoded number the `IOK`/`NOK` flag and a decoded string the
`POK` flag, and **the first numeric or string use of either sets the other flag** — measured: after one
`+0` and one `.""`, both the number and the string read `IOK=1 POK=1`. So the check re-decodes each
registry and inspects each scalar through the `@_` alias, before anything uses it.

The population is the directory of `--registry`, not a hard-coded path, so the test fixture's
`control/` runs the same code as the live tree. Residual: an integer too large for a native integer
decodes as a string and is refused as one; no containment bound comes near that size.

Out of scope on purpose: the claim registries, where `published_assertions.jsonl` legitimately quotes a
value copied from prose as a string.
