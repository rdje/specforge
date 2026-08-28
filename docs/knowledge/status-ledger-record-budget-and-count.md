---
id: status-ledger-record-budget-and-count
title: The status ledger's record count is bounded but unreported, and its per-record budget is overhead-net
answers:
  - "how many records does LIVE_ACHIEVEMENT_STATUS.md hold"
  - "how do I count the records in the status ledger"
  - "what is the per-record byte budget for a status record"
  - "why can the 80-record status window never be reached"
  - "does any command report the live record count of a rolling ledger"
  - "why does check_live_document_size not report records"
  - "what does check_rolling_ledger_protocol --report actually measure"
  - "is planned_live the current live window"
  - "what is 64 in the status ledger"
  - "is 64 the status ledger record count"
  - "how big may one LIVE_ACHIEVEMENT_STATUS record be"
  - "why does the status ledger keep hitting rollover"
  - "what does current_snapshot_bullets_v1 treat as one record"
  - "how much fixed overhead does the status ledger live view carry"
date: 2026-08-28
status: current
tags: [live-document-size, rolling-ledger, status-ledger, measurement-integrity, doctrine]
evidence: doctrine/live_document_size/surfaces.jsonl; doctrine/live_document_size/rolling_ledgers.jsonl; scripts/check_rolling_ledger_protocol.pl; docs/tasks/STATUS-LEDGER-ROLLOVER.md
reverify: "LC_ALL=C awk '/^## Current snapshot$/{f=1;next} /^## Highest-priority remaining gap$/{f=0} f' LIVE_ACHIEVEMENT_STATUS.md | LC_ALL=C awk '{n++; b+=length($0)+1} END{printf \"%d records, %d record bytes, mean %.1f\\n\", n, b, b/n}'"
---

`LIVE_ACHIEVEMENT_STATUS.md` is a `rolling_ledger` with the `current_snapshot_bullets_v1` grammar:
**one `- ` bullet line between `## Current snapshot` and `## Highest-priority remaining gap` is one
record.** Everything outside that region — the title, the start marker, and the whole
`## Highest-priority remaining gap` + validation-projection trailer — is the live view's fixed
overhead.

Three things follow, and the first two are why a wrong count was published and survived review.

**1. No tracked producer reports the live record count.** `perl scripts/check_live_document_size.pl
--report` emits `bytes_each`, `lines_each`, `lines_total`, and `line_bytes_each` for each surface — no
record dimension. `perl scripts/check_rolling_ledger_protocol.pl --report` emits each ledger's
`planned_live`, which is the **frozen migration boundary** pinned in
`doctrine/live_document_size/rolling_ledgers.jsonl`, not the current window. The record dimension is
bounded (`live_limits.records`), is enforced on append, and is invisible to every report. The only way
to obtain it today is the `reverify` command above, or the checker's own parser probed indirectly
through rollover-plan boundary arithmetic (`opening_records: N+1` goes RED with
`has fewer records than its opening boundary`).

**2. `80 x 80% = 64` is this surface's record *warning threshold*, not a count.** `STATUS-LEDGER-ROLLOVER.4`
published 64 as an observed record count and then divided the root's byte size by it, producing a
1,605-byte record mean and a 71-record capacity bound. Both were artifacts of the division.
`STATUS-LEDGER-ROLLOVER.4a` re-derived it. A threshold and a measurement look identical once they are
prose; only the producer distinguishes them, and here there was none.

**3. The per-record budget is overhead-net.** The naive reading of the declared pair — an 80-record
window and a 115,000-byte health target — is `115,000 / 80` = 1,437.5 bytes per record. That budget
cannot be met, because the prologue and trailer are charged to the same 115,000 bytes. The honest
budget is `(health_bytes - live_view_overhead) / live_limits.records`. A gate that derives the budget
from the registry must subtract the overhead it measures, or it will report a compliant ledger as
compliant while the byte dimension still binds first.

The consequence is structural, not incidental: the two declared limits are mutually unsatisfiable at
current record sizes, so the byte dimension always binds before the record window fills and a rollover
only resets the clock. `STATUS-LEDGER-ROLLOVER.4` owns the gate that must publish the count and check
the budget; `.2` owns the pinned migration suffix that consumes most of the window.

Related: [[rolling-ledger-record-grammars]], [[live-document-coverage-authority]].
