# Rolling-Ledger Archive

This fixed landing routes to the four independently bounded ledger archives. Each ledger index is the complete,
newest-to-oldest member route; immutable capsules and sealed segments are not mandatory current reads.

Verify every identity, chronology edge, route, and live/history boundary from the repository root:

```text
perl scripts/check_rolling_ledger_protocol.pl --report
```

## `changes`

- Current view: [`CHANGES.md`](../../../CHANGES.md)
- Complete ledger index: [`changes/INDEX.md`](changes/INDEX.md)
- Authority manifest: [`changes/manifest.jsonl`](changes/manifest.jsonl)

## `development-notes`

- Current view: [`DEVELOPMENT_NOTES.md`](../../../DEVELOPMENT_NOTES.md)
- Complete ledger index: [`development-notes/INDEX.md`](development-notes/INDEX.md)
- Authority manifest: [`development-notes/manifest.jsonl`](development-notes/manifest.jsonl)

## `live-achievement-status`

- Current view: [`LIVE_ACHIEVEMENT_STATUS.md`](../../../LIVE_ACHIEVEMENT_STATUS.md)
- Complete ledger index: [`live-achievement-status/INDEX.md`](live-achievement-status/INDEX.md)
- Authority manifest: [`live-achievement-status/manifest.jsonl`](live-achievement-status/manifest.jsonl)

## `rust-codebase-analysis`

- Current view: [`RUST_CODEBASE_ANALYSIS.md`](../../../RUST_CODEBASE_ANALYSIS.md)
- Complete ledger index: [`rust-codebase-analysis/INDEX.md`](rust-codebase-analysis/INDEX.md)
- Authority manifest: [`rust-codebase-analysis/manifest.jsonl`](rust-codebase-analysis/manifest.jsonl)
