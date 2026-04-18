# Troubleshooting

## `doctor --strict` fails

Start there first.

If `doctor --strict` fails:

- Docling ingest may not really be runnable
- the chosen local provider may not actually be serving
- the default local model may be missing

Fix the runtime before blaming the IR pipeline.

## Fresh PDF ingest fails but existing artifacts work

That usually means the source runtime is the problem, not the downstream IR stages.

Check:

- Docling Python resolution
- repo-local `.venv-docling`
- provider readiness

## Score dropped after a refresh

Do not assume regression immediately.

Common causes:

- stale old validation snapshot replaced with a fresher honest one
- artifact freshness mismatch between protocols
- current validator now surfacing a conflict that used to be hidden

The right next step is usually:

1. rebuild the affected artifact cleanly
2. inspect the new `validation_report.json`
3. compare findings, not just the top-line score

## Generated artifacts are missing

Remember that `generated/` is local and untracked.

If you switch machines or clone fresh, you must rebuild local artifacts.

## Generated artifacts are eating disk space

Start with the explicit local cleanup surface:

```bash
cargo run --manifest-path Cargo.toml -- clean
```

That dry-runs the heavyweight `generated/source_ir/*/normalized` bundles and shows how much space is reclaimable.

If the plan looks right:

```bash
cargo run --manifest-path Cargo.toml -- clean --execute
```

Remember what that means:

- `normalized/` bundles are rebuildable caches, not tracked project assets
- deleting them preserves `source_ir.json` by default
- if you later need page images, visual crops, or backend dumps again, rerun `ingest` or `converge`

If one document is no longer relevant and you want a full local reset for it:

```bash
cargo run --manifest-path Cargo.toml -- clean --scope document --document-key <document_key> --execute
```

## Learning did not seem to improve anything

Check the actual prior store:

`generated/prior_memory/corpus_memory.json`

Also remember:

- code defines how learning works
- the prior store is what actually grows over time
- priors are advisory only

So it is normal for some new corpora to increase truthfulness only in narrow places rather than across the whole pipeline immediately.
