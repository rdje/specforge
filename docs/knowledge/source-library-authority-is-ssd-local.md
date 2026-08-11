---
id: source-library-authority-is-ssd-local
title: The caller-authorized chipdoc source library is on the same SSD as SpecForge
answers:
  - "where is the authoritative chipdoc source library now"
  - "is SpecForge still allowed to read chipdoc from the boot volume"
  - "where does .cache/local-references/chipdoc resolve"
  - "are the external corpus PDFs on the same filesystem volume as the repository"
  - "how many generated SourceIR records still name the old livework checkout"
  - "does changing a source path mean a corpus document was refreshed"
date: 2026-08-11
status: current
tags: [corpus, source-provenance, project-data-locality, host-library, storage, ssd]
evidence: docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.4b); .cache/local-references/chipdoc; generated/source_ir; doctrine/corpus_frontier/census.json
reverify: "realpath .cache/local-references/chipdoc && stat -c '%d %n' . /Volumes/SSD/Documents/livework/chipdoc && rg -n -uuu --hidden --glob '!.git/**' --glob '!target/**' '/Users/.*/Documents/livework' ."
---

The director-authorized external source library is `/Volumes/SSD/Documents/livework/chipdoc`, below the stated
SSD livework root `/Volumes/SSD/Documents/livework`. The repository-local ignored discovery link
`.cache/local-references/chipdoc` resolves there. The repository and library both report filesystem device
`16777240`, so corpus reads stay on the SSD volume. The library is caller-authorized read-only input, not
project-owned output; generated data, caches, tests, and temporary work remain repository-derived.

On `2026-08-11`, `.4b` found 44 stale absolute fields in 38 generated records. Thirty-seven mapped by exact
suffix to existing SSD files; the CXS B revision was correctly found under `cxs/legacy` rather than `current`.
Every resolved file was checked before replacement. After repair, all 57 host-library SourceIR documents name
the SSD library in `canonical_path`; 21 current normalized metadata records name it in `input_path`; and the
whole-workspace residue census finds no boot-volume livework reference.

The tracked `.4b` evaluation dataset does not persist this host path. External evaluation sources retain only a
portable filename, SHA-256 digest, and necessity statement. Its fixture builder may read the current SourceIR
path to authenticate the external bytes, but generated dataset output remains portable.

Path repair is not extraction history. The same slice changed `CORPUS-FRONTIER` from a retired-prefix heuristic
to an explicit refreshed/remaining partition, so all five unfinished documents may point at their valid SSD
sources without being misreported as current-binary refreshes. See `[[corpus-refresh-frontier-derivation]]`.
