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
- repository-local `.cache/huggingface/` model availability
- provider readiness

After moving the repository, rebuild the venv with `bash scripts/bootstrap_docling.sh`; Python venv
launchers embed their creation path. Run `bash scripts/check_project_data_locality.sh` to detect a
stale launcher or escaping cache root before retrying ingest.

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

## `ingest` stops with "ingest aborted to protect the host"

This is **intentional and safe** — the opposite of the silent OOM kill above. While ingesting a
PDF, SpecForge watches the host's memory and stops itself before the machine reaches a danger
level, so a heavy document can never crash the host. When it acts, you will see a typed error like:

```
ingest aborted to protect the host: system memory was 86% used, at or above the 85% safety
ceiling, while running <docling helper>. The host was preserved and the previous normalized
bundle is intact. ...
```

Nothing was lost: the new bundle is staged and only swapped in on success, so your previous good
`normalized/` and `source_ir.json` are untouched. To proceed, do one of:

- **free memory** and rerun (close other applications; if a local model is loaded, `ollama stop`
  it first so ingest has the RAM to itself), or
- **raise the ceiling** if you know the host can take it: `SPECFORGE_INGEST_RAM_ABORT_PERCENT=90`, or
- **disable the guard** for a one-off run on a host you are watching yourself:
  `SPECFORGE_INGEST_RAM_ABORT_PERCENT=off`.

For very large PDFs, lowering `SPECFORGE_INGEST_BATCH_PAGES` (smaller batches use less peak memory)
is usually the better fix than raising the ceiling — it keeps the safeguard on while still letting
the document finish, just more slowly. SpecForge already does this for you automatically, sizing the
batch down on machines with less total RAM (see *Sizing the batch to the host* in the SourceIR
chapter); lowering the ceiling pushes it smaller still. If you ever need to pin the batch regardless
of host RAM, set `SPECFORGE_INGEST_ADAPTIVE_BATCH=off`.

## `ingest` stops with "ingest aborted before launching" (not enough disk)

Also intentional and safe. Before reading a PDF, SpecForge checks that the target filesystem has
enough free space for the run, scaled off the source PDF's size, and refuses up front rather than
filling the disk partway through. Because the check runs before any staging directory is created,
**nothing was written** — your previous normalized bundle is untouched. The message names the free
space and the estimated need:

```
ingest aborted before launching: only 90 MB free on the filesystem at <path>, below the 328 MB
this ingest is estimated to need. No work was started and any previous normalized bundle is
intact. ...
```

To proceed, do one of:

- **free disk space** (the `clean` command reclaims local generated artifacts — see "Generated
  artifacts are eating disk space" below) and rerun;
- **set an explicit floor** if you know the run will fit in less than the estimate:
  `SPECFORGE_INGEST_MIN_FREE_DISK_MB=120`;
- **disable the pre-flight** for a run on a host you are managing yourself:
  `SPECFORGE_INGEST_MIN_FREE_DISK_MB=off`.

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

For very large PDFs the heaviest part of the bundle — a full-resolution image of every page — is
already **not persisted** by default (only the figure/table region crops are), so a giant document's
`normalized/` stays bounded at ingest time. See
[Bounded disk footprint of very large PDFs](../pipeline/sourceir.md#bounded-disk-footprint-of-very-large-pdfs);
override with `SPECFORGE_INGEST_SAVE_PAGE_IMAGES=1` if you specifically want every page raster kept.

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
