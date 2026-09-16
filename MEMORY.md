# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`LIVE-DOCUMENT-PRESSURE-HEADROOM`** — **`.24a`** raised the fact plane in ONE transaction:
  `max_parts` **6→7** is the only value chosen, and `max_cards` **336→392**, the `knowledge_cards` anchor
  **338→394**, both aggregates as `files × per-file`, `fact_card_titles` **6→7**, `max_facts` **393→449** and
  `max_question_keys` **3,584→4,096** all derive, each asserted from both sides (self-test **60/60**).
  `knowledge_cards.files` **89.9%→77.2%**. **`.24b`** retired both authorities: the refusal fires per RECORD,
  naming both at once, so retiring one would still have been refused — first two-authority demonstration.
  **SEVEN PARTS IS TERMINAL**: an eighth declares 505 facts = **4,277** keys at the measured 8.47 ratio
  against the portable **4,096** cap (`check_knowledge_map_shard_contract.pl:107-118`) — an unreachable
  capacity, the ADR 0029 defect. The next capacity question belongs to the `knowledge-map/` bundle, not here.
  Earlier this session: **`.1`** split the 296-line genericity card (the map gates LOSSLESSNESS and CANNOT
  gate ASSIGNMENT — route each key by TF-IDF to the ORIGINAL paragraphs and compare cards); **`.23`** a
  doctrine JSON contract's encoder is per FILE, one `json.dumps` command reproduces **23 of 27** and **4 are
  hand-authored**; **`.24`** measured the plane and rendered it at full capacity.
- Next action: **`.25`** — two checkers
  (`check_live_document_size.pl:884`, `check_fact_card_catalog.pl:1185`) skip the `files` dimension when
  actual == target, so a FULL collection is the one state that reports nothing — the only undocumented
  exemption in a block where every other `next` cites its ADR.
- **Silent-failure lessons**: **[[chomp-is-a-no-op-under-a-callers-slurp]]** (a checker walked ZERO
  registries while every doctrine reported PASS — assert the POPULATION, not the exit code); and a sweep
  run with the wrong flag is a false green (`--check` on `check_rolling_ledger_protocol.pl` prints usage).
- **Continuation path PROVEN** (`CLAIM-VERIFICATION-ADOPTION.17`): append to the ONE active part OUTSIDE
  the markers, `- ID:` in the root registry, a `post_migration` route with NO `source_literal`, re-pin the
  part's `sha256`/metrics, `--write`, `--check`; index routes **2 open of 47**. That contract IS `JSON::PP`
  pretty+canonical, but DERIVE the encoder per file (`.23`) instead of assuming it.
- Also open: `SIGNAL-DECLARATION-ROW-DROP.2i` (doc claims an arrow arm it lacks; **0 of 663** rows, so fix
  the COMMENT), `.2f`, `.2h.2`; `COMMIT-GATE-SINGLE-RUN.0`; `TEXT-LAYER-IDENTIFIER-SPLIT.1`;
  `EXTRACTION-QUALITY-GAUGE.3j`/`.4c`; `CLAIM-VERIFICATION-ADOPTION.16`/`.17`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.15`-`.18`/`.20`/`.23`/`.4d.ii`.
- In-flight uncommitted: none. No background job outstanding.
- Blockers: none. Push cadence **400** (directive `2026-09-13`, FIXED), none due at 290; directive 16 gates
  it on full CI. **Corpus CURRENT — 27/27 at semantic and intent, retention exactly 24.**
  **Any new production Rust FUNCTION moves `flow_census.json`** — re-derive via `aggregate_change`, never
  edit the literal; boundary counts (144/9) must NOT move. **After editing a governed file OR a checker
  script run `python3 scripts/repin_claim_regions.py --check` then `--apply`**, refresh `shipped_behavior`,
  then the `claims.jsonl` digests LAST — that set IS the `Published-claims:` line. More hazards:
  **[[claim-verification-task-evidence-migrated]]**, **[[live-surface-edit-bookkeeping-chain]]**,
  **[[a-cheap-structural-rule-overfires-until-you-read-its-selection]]**, **[[actor-taxonomy-grows-in-pairs-not-terms]]**,
  **[[a-single-index-bit-cell-is-a-width-of-one]]**, **[[declaration-replay-reads-the-legacy-stratum]]**,
  **[[prior-phrase-utf8-byte-as-char]]**, **[[a-dropped-declaration-row-is-usually-not-a-signal]]**.
