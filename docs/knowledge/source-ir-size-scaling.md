---
id: source-ir-size-scaling
title: source_ir.json is O(pages) at ~9.3 KB/page; downstream stages load it with a full serde deserialize
answers:
  - "how big does source_ir.json get / how does it scale with page count"
  - "is source_ir.json bounded in size for very large PDFs"
  - "what is the size-immunity binding constraint for source_ir.json at extreme page counts"
  - "do evidence/semantic/intent stream source_ir.json or load it all into memory"
  - "why is MEMORY-BOUNDED-INGEST.5 summary streaming deferred"
  - "what dominates the source_ir.json size (content_elements? page_artifacts?)"
  - "could a huge PDF OOM the downstream stages even though ingest is bounded"
  - "how is SourceIr loaded from disk by downstream commands"
date: 2026-06-15
tags: [source-ir, memory-bounded, size-immunity, ingest, serde, scaling]
evidence: crates/specforge/src/ir/source.rs:573 (SourceIr::load_from_path = serde_json::from_str(&fs::read_to_string(path)?)); docs/tasks/MEMORY-BOUNDED-INGEST.md (.5 measured DEFER, .5a follow-up); measured over 78 persisted generated/source_ir/*/source_ir.json
reverify: python3 -c "import json,glob,os; r=[(json.load(open(f))['document_profile']['page_count'], os.path.getsize(f)) for f in glob.glob('generated/source_ir/*/source_ir.json')]; r=[x for x in r if x[0]]; n=len(r); sx=sum(p for p,_ in r); sy=sum(s for _,s in r); sxy=sum(p*s for p,s in r); sxx=sum(p*p for p,_ in r); b=(n*sxy-sx*sy)/(n*sxx-sx*sx); print(f'{b:.0f} bytes/page over n={n}, max {max(s for _,s in r)/1e6:.1f} MB')"; grep -n "fn load_from_path" crates/specforge/src/ir/source.rs
---

`source_ir.json` grows **linearly with page count at ~9.3 KB/page** (least-squares slope `9,312`
bytes/page measured over the 78 persisted artifacts; worst observed `17,755` B/page on a figure-dense
40-page doc). The size is dominated by `content_elements` (~13 typed elements/page) plus
`page_artifacts` (1/page); `structured_tables`/`document_sections`/`visual_assets` add smaller O(content)
terms. Concretely: the whole 82-doc chip-spec library tops out at **930 pages → 7.0 MB** (GIC-600 arch)
and **842 pages → 9.9 MB** (CoreSight SoC-600 TRM, the largest absolute). Extrapolated: 2,000 p → ~18 MB,
10,000 p → ~89 MB, 50,000 p → ~444 MB, 100,000 p → ~0.9 GB mean / ~1.65 GB worst.

**Why this matters for size-immunity ([[ingest-ram-guard]], [[ingest-disk-preflight]],
[[ingest-adaptive-batch-sizing]], [[page-image-disk-bounding]]):** the `MEMORY-BOUNDED-INGEST` program
bounded the *ingest* costs — peak conversion RAM (page-range batching `.1`/`.2` + the `.4a` clean-abort
guard, which satisfies the owner's non-negotiable "never crash the host") and per-page-PNG *disk* (`.3`).
At every realistic chip-spec size `source_ir.json` is trivially bounded (≤ 9.9 MB library-wide), so the
`.5` "stream the summary assembly" leaf was a **measured DEFER** — it has nothing to bound at realistic
sizes and would only risk the byte-identical guarantees.

**The genuinely unbounded-at-extreme cost is the downstream load, not ingest assembly.**
`SourceIr::load_from_path` (`ir/source.rs:573`) is `serde_json::from_str(&fs::read_to_string(path)?)` —
the *entire* artifact is read into a `String` then fully materialized into the `SourceIr` struct
(≈2–4× the JSON bytes in Rust), and `evidence`/`semantic`/`intent` run this with **no RAM guard**. So a
hypothetical 100,000-page doc would need ~2–6 GB just to LOAD downstream — but no chip-spec PDF reaches
that page count, so it stays a deferred follow-up (`MEMORY-BOUNDED-INGEST.5a`, YAGNI-deferred-until-
triggered, mirroring `.3b`). Re-open trigger: a real document whose `source_ir.json` would exceed a
RAM-safe bound (≈ > 20,000 pages / > ~200–500 MB).
