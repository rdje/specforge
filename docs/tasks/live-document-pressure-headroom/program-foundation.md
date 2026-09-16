# LIVE-DOCUMENT-PRESSURE-HEADROOM — program foundation

- Part ID: `program-foundation`
- State: `legacy`

<!-- pressure-headroom-task-source-region:program-identity:start -->
# LIVE-DOCUMENT-PRESSURE-HEADROOM: keep current-facing canonical surfaces writable

## Metadata

- Tree ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM`
- Status: `active` (`.0`/`.1`/`.23`/`.24`/`.24a`/`.24b`/`.15`/`.18`/`.25`/`.26`/`.26a`/`.27`/`.29`/`.29a`/`.30`/`.3`/`.5`/`.7`/`.19`/`.21`/`.22`/`.22a`/`.22b`/`.22c`/`.22d`/`.22e`/`.22f`/`.22g`/`.2a`/`.2b`/`.2c`/`.4a`/`.4b`/`.4c`/`.4e`/`.4f`/`.14a` done; `.4`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.16`/`.17`/`.20`/`.27a`/`.28`/`.29b` pending)
- Roadmap lane: repository durability and portability
- Created: `2026-08-14`
- Last updated: `2026-09-16`
- Owner: repo-local workflow

## Goal

Restore actionable headroom in the non-rolling current-facing surfaces whose next ordinary update is near a
hard refusal, while distinguishing writable authorities from large immutable evidence and preserving every
canonical byte and route.

## Non-Goals

- Do not raise a bound, shrink evidence, or silence a warning merely to make the pressure report quiet.
- Do not rewrite accepted decisions, completed research, or historical task evidence in place.
- Do not combine independent lifecycle remedies into one migration transaction.
- Do not interrupt the active alignment task-evidence root-last transaction.

## Opening Pressure Boundary (`92e59c97`)

The composed live-size gate passes, but these non-rolling axes have little local room:

| Surface | Exact largest/current state | Registered target/ceiling | Remaining |
| --- | ---: | ---: | ---: |
| `knowledge_cards.lines_each` | `production-genericity-boundary.md` 299 | 300 | **1 line** |
| `task_evidence.files` | 144 root task Markdown files | 160 | **16 files** |
| `task_tree_index.lines_each` | `docs/TASK_TREE.md` 397 | 480 health / 512 ceiling | 83 / 115 lines |
| `shipped_behavior.bytes_each` | `pipeline/evidenceir.md` 118,004 | 131,072 | 13,068 bytes |
| `research_records.lines_each` | `production-genericity-pipeline-audit.md` 604 | 640 | 36 lines |
| `validation_snapshot.lines_each` | `VALIDATION_SNAPSHOT.md` 544 | 640 | 96 lines |
| `readme_entrypoint.line_bytes_each` | `README.md` 108 | 120 | 12 bytes |

The new ownership file deliberately consumes one task slot; after catalog regeneration the resulting task plane
is 145/160 files and the derived task index is 398 lines. That cost is explicit and buys one route for the
remedies instead of scattering unowned warnings across future product commits.

`decision_records` pressure is excluded because `DECISION-RECORD-CAPACITY-HEADROOM` already owns its independent
41/44 collection count and ADR 0038 member-shape axes. Rolling ledgers and the roadmap root have declared
repeatable rollover/remedy paths and remain under their existing owners.

## Acceptance Criteria

- Each pressure axis is classified by lifecycle, writer, expected growth, hard stop, and a remedy that is legal
  for that lifecycle before any authority moves.
- The 299-line current knowledge card is losslessly split or superseded through the fact-card protocol before a
  structural qualification fact needs to update it.
- Task-tree collection and derived-index capacity are redesigned together; adding an ownership task cannot move
  one axis while hiding the other.
- Maintained book content stays user-readable and current; immutable research/snapshot evidence is routed or
  partitioned only through an accepted lossless transaction.
- README policy and every canonical catalog remain exact; focused gates and full CI run in proportion to each
  leaf, and every slice commits through `COMMIT.md`.

## Task Tree

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM`
  Status: `active`
  Goal: keep non-rolling current-facing canonical surfaces writable without losing evidence
  Children: `.0`, `.1`, `.2`, `.3`, `.4`, `.5`, `.6`, `.7`, `.8`, `.9`, `.10`, `.11`, `.12`, `.13`, `.14`, `.15`, `.16`, `.17`, `.18`, `.23`, `.24`, `.25`, `.26`, `.27`, `.28`, `.29`, `.30`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.0`
  Status: `done`
  Goal: own and pin the exact pressure frontier before any surface or bound changes
  Acceptance: the clean boundary, exact axes, existing owner split, and ordered remediation leaves are durable;
  no governed content, bound, product, decision, research record, snapshot, or book page changes
  Verification: `opening 92e59c97: knowledge card 299/300 lines; task plane 144/160 files and index 397/480 health lines; EvidenceIR book page 118,004/131,072 bytes; largest research 604/640 lines; validation snapshot 544/640 lines; README max line 108/120 bytes; ownership adds one task file and one derived catalog row only; resulting 145 files / 398 index lines; catalogs, retrieval, live-size, and doctrines pass`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.0 — own the current live-surface pressure frontier`

<!-- pressure-headroom-task-source-region:program-identity:end -->
