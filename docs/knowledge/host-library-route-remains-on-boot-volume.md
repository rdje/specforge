---
id: host-library-route-remains-on-boot-volume
title: The corpus host-library symlink still resolves to a boot-volume Git checkout after the project move
answers:
  - "where does the specforge chipdoc host library symlink currently resolve"
  - "why is corpus coverage refresh 34 blocked before ingest"
  - "is the chipdoc source library on the same SSD volume as specforge"
  - "does the external SSD project directory contain the USB4 Inter-Domain Service PDF"
  - "what must happen before the USB4 inter-domain corpus refresh can run"
  - "was any USB4 pipeline artifact changed by the source locality probe"
date: 2026-08-09
tags: [corpus, source-provenance, project-data-locality, host-library, usb4, storage]
evidence: docs/tasks/CORPUS-COVERAGE.md (.2.34a); .cache/local-references/chipdoc (host-local symlink); PROJECT_DATA_LOCALITY.md
reverify: "Resolve .cache/local-references/chipdoc and compare its filesystem identity with the repository root; census the external SSD project directory for a chipdoc checkout and the selected USB4 PDF. Do not ingest until the routes share the intended SSD volume or the director explicitly authorizes a documented one-time read-only copy into repository-local project data."
---

**Observed `2026-08-09` (`CORPUS-COVERAGE.2.34a`).** The repository-local
`.cache/local-references/chipdoc` symlink still resolves to the pre-move checkout on the boot filesystem.
Filesystem identities prove it is not on the repository's external SSD. A bounded census found no `chipdoc`
checkout and no `USB4_Inter-Domain_Service_Specification_v2.0_2025-11.pdf` in the external SSD project
directory. The remaining boot checkout is 935 MiB; the selected PDF hash is
`ab337460641c1f012a78c63dcf8cf182bf90deb29a7a841726973be79dcd7396`.

This conflicts with the director's session statement that all Git projects moved to the 4T SSD and only shared
Rust stores remain on the boot volume. The corpus workflow therefore fails closed before `ingest`; an old but
readable cross-volume symlink is not treated as proof that the boot checkout is still the intended authority.

No source copy, symlink rewrite, pipeline command, rollback, or generated-artifact mutation occurred. The retained
USB4 chain and its one-signal/two-rule adapter remain byte-identical. `CORPUS-COVERAGE.2.34b` may proceed after
either (1) the intended external-SSD checkout route is supplied, or (2) the director explicitly authorizes one
read-only source copy into repository-local project data, with hash verification and documented cleanup.
