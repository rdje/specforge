# Rolling-Ledger Archive

This bounded index routes from the stable current roots to immutable, exact source capsules. Capsules
are query-first historical terminals, not author-overflow destinations. Verify all identities and
current/history links from the repository root:

```text
perl scripts/check_rolling_ledger_protocol.pl --report
```

## `changes`

- Current view: [`CHANGES.md`](../../../CHANGES.md)
- Immutable source capsule: [`source-through-2026-08-08.md`](changes/source-through-2026-08-08.md)
- Repository-relative capsule locator:
  `docs/archive/rolling-ledgers/changes/source-through-2026-08-08.md`
- Frozen boundary: 1,798 records; 32,682 lines; 2,629,033 bytes; SHA-256
  `d89809322857aab3c506dde1cc6caaf57e22d0349655b37ddaab1f7bf22ba994`
- First post-migration sealed segment: [`segment-0001-2026-08-08.md`](changes/segment-0001-2026-08-08.md)
  preserves 29 whole records / 379 lines / 29,714 bytes at SHA-256
  `d38987e4e8ddba52c3182b3181a12831b8e62175980596249f733c86a0e7f4e3`; only the former successor separator is
  normalized to the protocol's canonical terminal newline. The segment sits between the newer live root and the
  older source capsule.
- Current-window rule: newest 85 records, then the exact detached `.1` and `.0` compatibility records.
- Overlap: the immutable capsule includes the retained live window so complete-source identity remains
  independently reproducible; later rollover archives only newly aged-out records.

## `development-notes`

- Current view: [`DEVELOPMENT_NOTES.md`](../../../DEVELOPMENT_NOTES.md)
- Immutable source capsule: [`source-through-2026-08-08.md`](development-notes/source-through-2026-08-08.md)
- Repository-relative capsule locator:
  `docs/archive/rolling-ledgers/development-notes/source-through-2026-08-08.md`
- Frozen boundary: 1,601 records; 20,921 lines; 2,170,230 bytes; SHA-256
  `76b51a3f450cdb1e764922dc366cf6ff55529cb95f7a6f410bfba1f1f378fedc`
- Current-window rule: H1 prologue followed by the newest 60 whole H2 records.
- Overlap: the immutable capsule includes the retained live window; later rollover archives only newly
  aged-out engineering-rationale records.

## `live-achievement-status`

- Current view: [`LIVE_ACHIEVEMENT_STATUS.md`](../../../LIVE_ACHIEVEMENT_STATUS.md)
- Immutable source capsule: [`source-through-2026-08-08.md`](live-achievement-status/source-through-2026-08-08.md)
- Repository-relative capsule locator:
  `docs/archive/rolling-ledgers/live-achievement-status/source-through-2026-08-08.md`
- Frozen boundary: 1,920 records; 1,960 lines; 581,239 bytes; SHA-256
  `b00ff5f5c4a29554a20eea9d901a95848d54a644749eba74ad9618798dd9bd6a`
- First post-migration sealed segment: [`segment-0001-2026-08-08.md`](live-achievement-status/segment-0001-2026-08-08.md)
  preserves 12 whole records / 12 lines / 12,671 bytes at SHA-256
  `ee4c7aed44efd79ebc9cbf7b80c4f4f7f599bbcaa8db2acfcc8a00f845ff8067`; it sits between the newer live
  root and the older source capsule.
- Current-window rule: H1/current heading, the newest post-capsule bullets plus the exact 40-record migration
  suffix, then the complete gap and writer-managed validation trailer. The first rollover leaves 60 live records.
- Overlap: the immutable capsule includes the retained live window; later rollover archives only newly
  aged-out status records and never edits the validation projection in place.

## `rust-codebase-analysis`

- Current view: [`RUST_CODEBASE_ANALYSIS.md`](../../../RUST_CODEBASE_ANALYSIS.md)
- Immutable source capsule: [`source-through-2026-08-08.md`](rust-codebase-analysis/source-through-2026-08-08.md)
- Repository-relative capsule locator:
  `docs/archive/rolling-ledgers/rust-codebase-analysis/source-through-2026-08-08.md`
- Frozen boundary: 1,350 records; 9,039 lines; 1,046,679 bytes; SHA-256
  `95e1665628b615498e94f67d6dc6e0083d4d104a6ca8a815b4c23cf2f97cc7ce`
- First post-migration sealed segment: [`segment-0001-2026-08-08.md`](rust-codebase-analysis/segment-0001-2026-08-08.md)
  preserves four whole records / 101 lines / 8,955 bytes at SHA-256
  `765cb04793666ddf63ded2ed93ce72e3833d6684febad01eeed739897a5b6fde`; only the former successor separator is
  normalized to the protocol's canonical terminal newline. The segment sits between the newer live root and the
  older source capsule.
- Current-window rule: H1 plus complete Purpose prologue, followed by the newest 60 whole H2 records.
- Overlap: the immutable capsule includes the retained live window; later rollover archives only newly
  aged-out architecture records.
