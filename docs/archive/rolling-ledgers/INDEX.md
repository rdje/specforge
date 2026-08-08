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
- Current-window rule: H1/current heading, newest 50 whole snapshot bullets, then the complete gap and
  writer-managed validation trailer.
- Overlap: the immutable capsule includes the retained live window; later rollover archives only newly
  aged-out status records and never edits the validation projection in place.
