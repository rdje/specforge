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
