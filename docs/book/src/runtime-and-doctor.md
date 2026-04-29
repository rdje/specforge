# Runtime And Doctor

`specforge` is designed to fail early and clearly when the local runtime is not actually ready.

The command for that is:

```bash
cargo run --manifest-path Cargo.toml -- doctor --strict
```

## What `doctor` checks

`doctor` currently preflights three things:

1. Docling ingest readiness
2. default Ollama loopback readiness
3. LM Studio fallback readiness

In practice that means:

- selected Python candidate for Docling
- whether `import docling` works
- Ollama `/api/tags`
- presence of the default local model `qwen2.5vl:7b`
- Ollama `/v1/chat/completions`
- LM Studio `/v1/models`
- LM Studio `/v1/chat/completions`

The local chat-completions probes are strict, but they are cold-load tolerant.
If the model is present but not yet loaded, `doctor` gives the local provider a longer chat-probe window so a healthy Ollama or LM Studio runtime can load the model and answer instead of failing only because the first request was cold.

## Docling runtime resolution order

For PDF ingest, `specforge` resolves Python in this order:

1. `SPECFORGE_DOCLING_PYTHON`
2. repo-local `.venv-docling`
3. versioned candidates such as `python3.11`
4. generic `python3`
5. generic `python`

The supported repo-local bootstrap path is:

```bash
bash scripts/bootstrap_docling.sh
```

That gives the project a stable local runtime instead of depending on whichever `python3` happens to be first on `PATH`.

## Provider strategy

Today the practical provider policy is:

- keep Ollama as the default path
- use LM Studio as the supported local fallback
- keep stronger models as explicit fallback lanes, not silent defaults

`project-validation` follows that policy for generated visual-motif rescan hints too.
Its default `--rescan-vlm-provider auto-local` checks local default-model presence, prefers Ollama when ready, falls back to LM Studio when Ollama is not ready, and otherwise emits the Ollama hint so the missing default path remains obvious.

On the current local hardware, `qwen2.5vl:7b` is the practical everyday model. Stronger local VL models are possible in principle, but they should be treated as selective fallback lanes rather than the default operating path unless hardware margins clearly support them.

## When to run `doctor`

Run `doctor --strict`:

- before a fresh PDF ingest
- before a long `converge` run
- after changing Python runtimes
- after changing local model providers
- before deciding a pipeline failure is really an IR bug
