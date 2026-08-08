# FSMGen Feedback Archive

This bounded index keeps the stable current channel and its exact sealed history directly retrievable.
The capsule is immutable; new open correspondence belongs only in the current channel.

## Routes

- Current channel: [`docs/FSMGEN_FEEDBACK.md`](../../FSMGEN_FEEDBACK.md)
- Exact source capsule: [`source-through-2026-08-08.md`](source-through-2026-08-08.md)
- Identity manifest: [`manifest.json`](manifest.json)
- FSMGen responses: [`subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md`](../../../subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md)
- Reproducible issue bundles: [`docs/catalogs/fsmgen-issue-packets.md`](../../catalogs/fsmgen-issue-packets.md)

## Sealed boundary

The capsule preserves the complete pre-containment source through `2026-08-08`: five ordinary directed
exchanges, one composite two-bug episode, the scope override, and the legacy primer. All six exchanges
were closed and the open-record count was zero when sealed. The current channel's register links each
closed exchange to both this exact history and independent response or resolution evidence.

Verify digest, dimensions, semantic regions, record identities, evidence, manifest, routes, current
markers, register, and bounds from the repository root:

```bash
perl scripts/check_fsmgen_feedback_protocol.pl --check
```
