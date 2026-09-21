# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **none in flight**.
- Next action: **`KG-ISF-COMPLETENESS.5.vii.a`** (CODE — drop the `row_idx` value fallback in
  `synthesize_encoding_declarations_for_enum`); its node pins the predicted population and a per-enum
  expectation on every scored gold. `.5.viii` (read the quoted literal, 357 values) is independent —
  disjoint sets, either order. **`.5.vi.a` stays behind both** (additive; cleans only legacy chains).
- **`.5.vii` CLOSED `2026-09-21`**: **10,189 of 19,023 minted enum values (53.6%) are the row INDEX**,
  not a value any document stated; only **84 of 512** promoted symbols carry none. Fallback is
  uncorroborated by its own row 94.5% of the time and **provably wrong in 5,688 slots**.
  [[minted-enum-values-are-mostly-row-indices]]
- **Replay the producer; do not read its output's SHAPE** — `.5.v`'s `0..n-1` test saw 160 enums, the
  replay 10,189 members: a shape sees neither a MIXED enum nor a gapped one.
- **A protected population is not a healthy one** — `.5.vi` exempted 2 continuations without reading
  them; both are junk, and a continuation is where a table-local ordinal CANNOT be right.
- **`CLAIM-VERIFICATION-ADOPTION.20` tranche 1 landed (interleaved): 4 regions re-derived in
  `pipeline/isf-adapter.md`, 2 STALE — rate 2 in 4.** Population 167 -> 163; chapter 24 -> 20.
- **RECOVER THE ORIGINAL CLAIM'S DENOMINATOR BEFORE COMPARING ANYTHING TO IT.** Both first-pass
  verdicts were confidently wrong, in opposite directions: `82 -> 8` was over 33 REBUILDABLE docs (0
  today over 27; the 67 corpus-wide are frozen pre-gate legacy), and `6,570` counted fields reaching an
  emitted `.isf` (0 today — nothing is emitted) not the 12,602 in `IntentIR`. Next tranche: prefer a
  chapter whose claims are NOT corpus-derived totals.
- **Execute a claim's trigger; never guess a key** (a census returned 0 from `bit_fields`; it is
  `register_records[].fields`). **Run the negative control**: clearing `BRESP`/`HTRANS`/`PPROT` at zero
  is what makes 53.6% a finding and not a broken meter.
- **Every governed edit pays a claim-digest bill** — `repin_claim_regions.py --check`/`--apply` is
  TRANSACTIONAL and writes nothing until you hand-resolve each refusal. Rewriting a claim makes it
  ABSENT; rewording can also drop it out of the candidate predicate (`number` + <=2 words + a unit).
- **DERIVE a contract's JSON encoder every time; never look one up** — `CONTRACT-ENCODING` gates it.
- `cargo test -p specforge --lib <filter>` reports **0 passed, 474 filtered out** for an `ir/` change and
  exits 0. Use `--workspace --lib --exclude specforge-production-graph` (COMMIT.md step 8).
- `LIVE-DOCUMENT-PRESSURE-HEADROOM` backlog NOT fixed: 75 of 239 `###` book headings lead with a
  work-unit id; **`files` is the highest band in every part collection** (44-88%).
- `LEGACY-SOURCE-RECLASSIFICATION.0`+`.1`: **the bundle, not the label, is the binding constraint.**
- **After every push, watch the hosted run to a verdict** (directive `2026-09-20`); a stalled run is not a pass.
- **A parked leaf keeps its blocker until re-derived. Do not restate a handed-down number.**
- **Measure the PASS, not the function**; **coverage is PROVENANCE**; **read the ROOT CAUSE, not the symptom**.
- `.36d` owns the claim registry lifecycle at the 80% record band. **Never carry the residual as a number.**
- `BOUNDED-DECISION-PROVIDER`: **provider REJECTED** (ADR 0051); do not buy `TYPESAFE_API_KEY`.
- Corpus 27/27 current. A slow gate is **measured, not attributed**: `probe_exec_assessment_latency.sh`.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none.
