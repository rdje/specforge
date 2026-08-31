---
id: live-document-width-remedy-coupling
title: A `line_bytes_each` warning is an extremal bound, not accumulation — narrow the widest lines without changing the file's line count
answers:
  - "how do I clear a live-document line_bytes_each warning (reflow the few widest lines; do not partition, roll over, or raise the ceiling — nothing is running out)"
  - "is a line_bytes_each warning the same kind of pressure as a lines_each or bytes_each warning (NO, but not because one is a maximum and the others are not — check_live_document_size.pl computes ALL THREE _each dimensions as per-surface maxima. What differs is the quantity being maximized: lines_each and bytes_each maximize a per-FILE quantity that accumulates as content is added, while line_bytes_each maximizes a per-LINE quantity that does not, so it has no growth driver and is freely reducible at any time)"
  - "does a live-document size warning always mean the surface is growing (no — an extremal dimension such as line_bytes_each can sit at rollover on a file that has not changed size at all)"
  - "what does 'at or above rollover' mean for a line_bytes_each dimension (only that the widest single line is at 90% of the width bound; the milestone vocabulary is written for a quantity that accumulates. Most surfaces have no rollover to perform anyway — rolling_ledgers.jsonl declares just four sources, and README is a bounded_snapshot that appears there only as a reader — and where one does exist it lowers a maximum only incidentally, as the CHANGES.md rollover did not: line_bytes 1629 either side)"
  - "can I just rewrap a whole live document to a narrower column (only after checking the surface's other bounds — every line break you insert costs one line and, at a two-space continuation indent, two bytes, so a naive rewrap can trade one warning for two)"
  - "what is the safe way to narrow an over-wide line in a governed Markdown surface (reflow the enclosing block at the narrowest column that preserves its line count exactly; that keeps the byte count identical too, because the number of separators is unchanged)"
  - "why is narrowing lines not free in SpecForge (the width bound and the line/byte bounds on the same surface are adversarial: the only remedy for the first spends the budget of the other two)"
  - "what breaks if I change the number of lines in README.md (three line-anchored regions in doctrine/claim_verification/current_claim_census.jsonl are pinned by start_line/end_line plus a sha256 of the pinned lines — the identity anchor, the derived Rust-prerequisite line, and the 17-line route block — so any edit that shifts line numbers stales them)"
  - "which README.md regions are pinned by line number (the census pins L1 as the document identity anchor, L30 as the rust_prerequisite_copies derived value, and L88-L104 as the entrypoint route block; the line numbers are current as of 2026-08-31 and must be re-derived, not assumed)"
  - "why did LIVE-DOCUMENT-PRESSURE-HEADROOM.4d reflow one bullet instead of rewrapping README.md"
  - "what is README.md's effective wrap column in SpecForge (about 94 bytes — that is the measured maximum of the population once the two drifted lines were reflowed; the enforced bound is 120 and the warning fires at 96)"
date: 2026-08-31
status: current
tags: [live-document-containment, readme, doctrine, claim-verification, retrieval]
evidence: LIVE_DOCUMENT_SIZE_CONTAINMENT.md (health target / milestone contract); doctrine/live_document_size/surfaces.jsonl (readme_entrypoint); scripts/check_live_document_size.pl (percent >= warning arithmetic); doctrine/claim_verification/current_claim_census.jsonl (README.md line-anchored regions); docs/tasks/LIVE-DOCUMENT-PRESSURE-HEADROOM.md (.4d.i)
reverify: "awk '{print length($0)}' README.md | sort -rn | head -3; wc -l -c README.md; python3 -c \"import json;[print(r['path'],r['region']['start_line'],r['region']['end_line']) for r in (json.loads(l) for l in open('doctrine/claim_verification/current_claim_census.jsonl') if l.strip()) if r.get('path')=='README.md']\""
---

# A width bound is a maximum, not an accumulation

`scripts/check_live_document_size.pl` measures every governed surface on the same dimension list and prints the
same sentence for each — `is at or above warning`, `is at or above rollover`. Two kinds of dimension hide behind
that uniform wording:

**The difference is not maximum-versus-total.** `check_live_document_size.pl` (the `$metrics{..._each}` block)
computes `bytes_each`, `lines_each` **and** `line_bytes_each` alike, as per-surface *maxima* over members. What
separates them is the quantity each one maximizes:

- **Maxima over an accumulating quantity** — `lines_each` and `bytes_each` maximize a per-*file* line or byte
  count, and a file's line count grows as content is added. (`lines_total`, `bytes_total` and `files` accumulate
  outright.) These have an identifiable growth driver and writer set, and the doctrine's remedies apply:
  partition, rollover, archive, or a separately authorized ceiling increase.
- **A maximum over a non-accumulating quantity** — `line_bytes_each` maximizes a per-*line* width, and one
  line's width does not grow as the document does. It has no growth driver, and it is freely reducible at any
  time. A file that has not changed size in months can sit at rollover here because one line was written wide.

The milestone words are borrowed in that second case. **A rollover is not even available to most surfaces.**
The repository's only rollover transaction is the rolling-ledger protocol, and
`doctrine/live_document_size/rolling_ledgers.jsonl` declares exactly four sources — `CHANGES.md`,
`DEVELOPMENT_NOTES.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `RUST_CODEBASE_ANALYSIS.md` — which are the four
`rolling_ledger` surfaces. `readme_entrypoint` is a `bounded_snapshot` with overwrite semantics and appears in
that registry only as a *reader*, so no rollover exists for it. And where a rollover does exist it lowers a
maximum only incidentally, if the extreme member happens to fall in the sealed part: the last `CHANGES.md`
rollover left the ledger's `line_bytes` at 1629 on both sides of the cut.

Classify what the dimension maximizes before choosing a remedy. Read as capacity pressure, a `line_bytes_each`
warning argues for partitioning or ceiling-raising a surface that needs neither.

## The remedy is a reflow — and it is not free

The only way to reduce a maximum line width is to insert line breaks. On a Markdown continuation line indented by
two spaces, each break you insert converts one `" "` separator into `"\n  "`: **+1 line and +2 bytes**. So on a
surface that also bounds `lines_each` and `bytes_each`, the remedy for the width dimension is *adversarial* to the
other two, and a whole-file rewrap can clear one warning while tripping two.

Measured on `readme_entrypoint` at `2026-08-31`, before `LIVE-DOCUMENT-PRESSURE-HEADROOM.4d.i`:

| Dimension | Measured | Health target = ceiling | Warning fires at | Headroom |
| --- | ---: | ---: | ---: | ---: |
| `lines_each` | 118 | 150 | 120 | 2 lines |
| `bytes_each` | 4,637 | 5,800 | 4,640 | 3 bytes |
| `line_bytes_each` | 108 | 120 | 96 | at rollover |

Three bytes and two lines. A rewrap of the whole file to a narrower column would have inserted dozens of breaks
and pushed both of the other dimensions over their warnings to buy width headroom that two lines needed.

## Reflow at the narrowest line-count-preserving column

The safe operation is: take the smallest block that contains the over-wide lines, and re-wrap it at **the
narrowest column that still fits in the same number of lines**. Same number of separators means the same byte
count, so the file is byte-identical in size and the surface's other two dimensions do not move at all.

On README's five-line architecture bullet the boundary was exact: width 88 wrapped to 5 lines / 423 bytes, width
87 spilled to 6 lines / 424 bytes. Reflowing at 88 took the file from `108/120` (90.0%, rollover) to `94/120`
(78.3%, clear) at **118 lines and 4,637 bytes unchanged**, with the same words in the same order.

## Why line-neutrality is a correctness property here, not tidiness

`doctrine/claim_verification/current_claim_census.jsonl` pins README regions by **line number plus a SHA-256 of
the pinned lines**, not by content search. As of `2026-08-31` those are L1 (document identity anchor), L30 (the
`rust_prerequisite_copies` derived value) and L88–L104 (the entrypoint route block). Any README edit that changes
the line count above a pin moves that region and stales its digest, dragging a census re-anchor into a slice that
had no business touching the claim plane. A line-count-preserving reflow leaves all three digests valid — which is
verifiable directly, and was: all three re-derived unchanged across the `.4d.i` reflow.

Re-derive the pinned line numbers before relying on them; they move whenever README's shape legitimately changes.
