#!/usr/bin/env python3
"""Extract text from a chip-spec PDF for inspection — the reading strategy that WORKS
when the Claude Code Read tool refuses a file (permission-encrypted PDFs, and the
GitHub #38530 false-positive where it rejects even non-encrypted PDFs).

This is the lightweight raw-text path; docling ingestion is the structured path used
by the SpecForge pipeline. Per the multi-strategy principle, both are available and
the best one wins per chip-spec PDF — and both bypass the (unreliable) Read tool.

Handles permission-encrypted PDFs transparently (empty user password). Requires
pypdf (+ cryptography for AES) — installed in .venv-docling.

Usage:
    .venv-docling/bin/python scripts/pdf_text.py <file.pdf> [start_page] [end_page]
      (1-based, inclusive; default: all pages)
"""
import logging
import sys
from pathlib import Path

logging.getLogger("pypdf").setLevel(logging.ERROR)


def main() -> int:
    if not (2 <= len(sys.argv) <= 4):
        print("usage: pdf_text.py <file.pdf> [start_page] [end_page]", file=sys.stderr)
        return 2
    src = Path(sys.argv[1])
    if not src.is_file():
        print(f"error: not found: {src}", file=sys.stderr)
        return 2
    try:
        from pypdf import PdfReader
    except ImportError:
        print("error: pypdf not available — use .venv-docling/bin/python", file=sys.stderr)
        return 3

    reader = PdfReader(str(src))
    if reader.is_encrypted:
        reader.decrypt("")  # permission-encrypted PDFs open with the empty user password
    n = len(reader.pages)
    start = int(sys.argv[2]) if len(sys.argv) >= 3 else 1
    end = int(sys.argv[3]) if len(sys.argv) >= 4 else n
    start = max(1, start)
    end = min(n, end)
    print(f"# {src.name} — {n} pages; showing {start}..{end}")
    for i in range(start - 1, end):
        print(f"\n===== PAGE {i + 1} =====")
        print(reader.pages[i].extract_text() or "(no extractable text)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
