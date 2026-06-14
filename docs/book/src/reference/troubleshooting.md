# Troubleshooting

## `doctor --strict` fails

Start there first.

If `doctor --strict` fails:

- Docling ingest may not really be runnable
- the chosen local provider may not actually be serving
- the default local model may be missing
- if the model is present but the chat probe times out, inspect the reported provider detail; a timeout now means the provider could not answer within the cold-load-tolerant probe window, not just that the model had not been prewarmed

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

## `evidence` or `converge` is killed by the operating system (out of memory)

This is fixed in current builds; the note is here for anyone who hit it on an older one.

A large document that declares many multi-word actor names *and* contains non-ASCII
characters (for example a `•` bullet inside a signal-description cell) could drive the
cross-document prior-normalization step to grow a string without bound, until the OS killed
the process (a hard kill, not a Rust panic — so there was no error message). The cause was a
text-copy bug in prior-phrase normalization that mangled non-ASCII bytes and compounded the
mangling once per actor name.

If you see an evidence or converge run get killed with no diagnostic on a build from before
this fix, update to a current build — the normalization now copies text correctly and the
build completes with normal memory use. Nothing about the input PDF needs to change.

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

If the whole local generated tree is the problem and you want a true cold rebuild:

```bash
cargo run --manifest-path Cargo.toml -- clean --scope all-generated --execute
```

## Learning did not seem to improve anything

Check the actual prior store:

`generated/prior_memory/corpus_memory.json`

Also remember:

- code defines how learning works
- the prior store is what actually grows over time
- priors are advisory only

So it is normal for some new corpora to increase truthfulness only in narrow places rather than across the whole pipeline immediately.
