---
id: prior-phrase-utf8-byte-as-char
title: prior-phrase normalization UTF-8 byte-as-char mangling — exponential OOM found & FIXED (PDF-VARIANT-DIGESTION.13b.1)
answers:
  - "why did the ACE evidence build run out of memory / get SIGKILLed (exit 137)"
  - "why does an evidence/converge build OOM on a doc with many multi-word actor names"
  - "what does bytes[index] as char do to non-ASCII UTF-8 text in prior_memory.rs"
  - "why did normalize_prior_phrase grow a string exponentially"
  - "is replace_term_with_placeholder safe on non-ASCII signal/actor names"
date: 2026-06-14
tags: [utf-8, prior-memory, normalize-prior-phrase, oom, mojibake, evidence-ir, pdf-variant-digestion]
evidence: docs/tasks/PDF-VARIANT-DIGESTION.md (.13b.1); crates/specforge/src/ir/prior_memory.rs (replace_term_with_placeholder — the `else` arm; normalize_prior_phrase — the per-multi-word-term chained passes)
reverify: "build a fresh binary, then `target/debug/specforge evidence generated/source_ir/ihi0022_h_c_2021_01_amba_axi_and_ace_protocol_specification/source_ir.json --dry-run` — it now completes in ~21s at ~56 MB max RSS (was 419 s / 17.2 GB RSS / SIGKILL exit 137 pre-fix)"
---

**FIXED by `PDF-VARIANT-DIGESTION.13b.1` (`2026-06-14`).** This card records a now-resolved
defect plus the anti-pattern to never re-introduce.

`replace_term_with_placeholder` (`crates/specforge/src/ir/prior_memory.rs`) copied each
non-matching position with `result.push(bytes[index] as char); index += 1;`. Casting a single
`u8` to `char` is **not** a UTF-8 decode: a multi-byte character (e.g. `•` = `E2 80 A2`)
becomes three separate Latin-1 scalars (`U+00E2 U+0080 U+00A2`), each of which re-encodes to
**two** UTF-8 bytes — so every non-ASCII char roughly **doubles** in byte-length on each pass.

`normalize_prior_phrase` calls that function **once per multi-word replacement term** (chained,
each pass over the previous pass's output). With N multi-word terms over a string containing a
non-ASCII char, the mojibake re-doubles N times → **`2^N` growth**. ACE's semantic-hints surface
derives **173** multi-word actor names (AXI: 3) and its `table_0201` `Signal | Width | Description`
cells carry a `•`; one `infer_signal_semantic_tags_from_description` call grew the string until
the kernel jetsam-killed the process (**17.2 GB max RSS, 419 s, exit 137** on a 24 GB host;
sampled stacks 100% in `normalize_prior_phrase`). Every other persisted doc carried too few
multi-word terms to detonate, which is why the defect stayed invisible behind a green test suite.

**Fix:** the `else` arm now copies one whole UTF-8 char —
`text[index..].chars().next()` → `result.push(ch)` → `index += ch.len_utf8()`. `index` always
sits on a char boundary (it advances by an ASCII match's `term_bytes.len()` or by one whole char),
so the slice is valid; on pure-ASCII input it is byte-for-byte identical to the old copy. Proven
by a pre-fix-vs-post-fix fresh `evidence --dry-run` over the 15 non-ACE intact bundles = all
**BYTE-IDENTICAL**, and ACE now completes at **21 s / 56 MB**.

**Anti-pattern (avoid):** never `push(byte as char)` when copying text — that is a Latin-1 cast,
not a UTF-8 decode. Copy whole `char`s (`str::chars`) or whole validated `&str` slices. And be
wary of **chained string-rewrite passes** (one pass per term): a per-char size bug that is linear
in one pass becomes exponential across many passes. Cf. [[evidence-build-nondeterminism]] for the
other "looks fine on the test corpus, breaks on a real doc" prior-path defect.
