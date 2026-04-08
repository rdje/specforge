# Reference

This chapter is the quick map of what `specforge` writes and where.

## Generated artifact roots

- `generated/source_ir/<document_key>/source_ir.json`
- `generated/evidence_ir/<document_key>/evidence_ir.json`
- `generated/semantic_ir/<document_key>/semantic_ir.json`
- `generated/intent_ir/<document_key>/intent_ir.json`
- `generated/adapters/fsm/<document_key>/adapter.json`
- `generated/prior_memory/corpus_memory.json`

## Important tracked docs

Tracked docs are not generated artifacts.
They are continuity and steering surfaces.

The most important user-visible ones are:

- `README.md`
- `VALIDATION_SNAPSHOT.md`
- `LIVE_ACHIEVEMENT_STATUS.md`

The more development-centric ones are described in the next chapter.

## Local versus tracked

`generated/` stays local and untracked.

That rule matters because:

- generated artifacts can be large
- they change often
- continuity should live in stable tracked docs, not in versioned generated blobs

