---
id: pdf-encryption-and-read-access
title: 12/82 corpus PDFs are permission-encrypted (open w/ empty password); docling+pypdf read all 82; the Read tool is unreliable
answers:
  - "which corpus PDFs are password/permission protected"
  - "do any chip-spec PDFs need a real password (no)"
  - "why does the Claude Read tool refuse some PDFs / report password-protected"
  - "how to read a chip-spec PDF when the Read tool refuses it"
  - "what are scripts/pdf_text.py and scripts/decrypt_pdf.py"
date: 2026-06-07
tags: [pdf, ingestion, encryption, read-tool, tooling, pdf-variant-digestion]
evidence: scripts/pdf_text.py; scripts/decrypt_pdf.py; pypdf+cryptography in .venv-docling; pdfinfo
reverify: ".venv-docling/bin/python -c \"from pypdf import PdfReader; r=PdfReader('<pdf>'); print('enc',r.is_encrypted); r.decrypt('') if r.is_encrypted else 0; print(len(r.pages))\""
---

Of the 82 corpus PDFs, **12 are encrypted** but **NONE need a real (open/user) password** — they are
*permission*-encrypted (owner password restricting copy/print) and open with the **empty user password**.
docling AND pypdf read all 82. So **encryption is NOT a blocker** for SpecForge ingestion.

The 12 (verified `2026-06-07`): **RC4-128** — CoreSight SDC-600 TRM, CoreSight SoC-600 TRM (2017-08 &
2017-12), Cortex-A76 TRM, GIC-600 TRM. **AES-128** — OpenCAPI Transaction-Layer 3.0/3.1, OpenCAPI 4.0 32G
PHY-Signal & PHY-Mech, 4.0 Transaction-Layer, Data-Link-Layer, Discovery_Configuration.

**The Claude Code Read tool is unreliable for these** and is NOT fixable by a plugin/setting (refusal is
hardcoded in Anthropic's VPC; confirmed by the claude-code-guide agent + GitHub #30819/#38530): it refuses
permission-encrypted PDFs AND false-positives some non-encrypted ones (the ADI is `Encrypted: no` yet the
Read tool rejected it — GitHub #38530). Decrypting/re-saving does NOT help — the Read tool rejects even a
freshly `Encrypted: no` copy.

**Working reading strategies (both bypass the Read tool; multi-strategy, best-wins-per-PDF —
[[feedback_multi_strategy_best_wins]]):**
- `docling` ingestion → structured `content_elements` (the SpecForge pipeline path; used to read all of SWD
  Chapter B4).
- `.venv-docling/bin/python scripts/pdf_text.py <pdf> [start] [end]` → raw text via pypdf (handles
  permission-encrypted transparently). Quick inspection path.
- `scripts/decrypt_pdf.py <in> <out>` strips the encryption layer for any tool that needs unencrypted input
  (it does NOT unlock the Read tool, per above).

Tooling installed: **pypdf 6.13 + cryptography** in `.venv-docling`. Do NOT log the owner library path
([[feedback_source_pdfs_in_repo]]).
