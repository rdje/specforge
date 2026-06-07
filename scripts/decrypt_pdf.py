#!/usr/bin/env python3
"""Strip permission-encryption from a PDF, producing an unencrypted copy for any
downstream tool that needs unencrypted input. PDF-VARIANT-DIGESTION tooling.

NOTE (verified `2026-06-07`): this does NOT unlock the Claude Code Read tool — the
Read tool rejects even a freshly re-saved, `Encrypted: no` PDF (GitHub #38530). For
INSPECTION, use the working paths instead: `scripts/pdf_text.py` (raw text via pypdf)
or docling `content_elements` (structured) — both bypass the Read tool and handle all
82 PDFs. This decrypt helper remains useful for tools that choke on the encryption
layer itself.

The 12 encrypted PDFs in the corpus are *permission*-encrypted (owner password
restricting copy/print) but open with an EMPTY user password, so no password is
needed. This re-saves a decrypted copy.

Usage:
    .venv-docling/bin/python scripts/decrypt_pdf.py <input.pdf> <output.pdf>

Requires pypdf (+ cryptography for AES) — installed in .venv-docling.
Exit 0 on success; nonzero with a message otherwise. Never overwrites a source
PDF in place: <output.pdf> must differ from <input.pdf>.
"""
import logging
import sys
from pathlib import Path

logging.getLogger("pypdf").setLevel(logging.ERROR)  # silence noisy annotation/object warnings


def main() -> int:
    if len(sys.argv) != 3:
        print("usage: decrypt_pdf.py <input.pdf> <output.pdf>", file=sys.stderr)
        return 2
    src, dst = Path(sys.argv[1]), Path(sys.argv[2])
    if not src.is_file():
        print(f"error: input not found: {src}", file=sys.stderr)
        return 2
    if src.resolve() == dst.resolve():
        print("error: output must differ from input (no in-place overwrite)", file=sys.stderr)
        return 2
    try:
        from pypdf import PdfReader, PdfWriter
    except ImportError:
        print("error: pypdf not available — use .venv-docling/bin/python", file=sys.stderr)
        return 3

    reader = PdfReader(str(src))
    if reader.is_encrypted:
        # permission-encrypted PDFs open with the empty user password.
        if reader.decrypt("") == 0:
            print(f"error: {src.name} needs a real open-password (not permission-only)", file=sys.stderr)
            return 4
    writer = PdfWriter()
    writer.append(reader)  # decrypted; no encryption applied to the writer
    dst.parent.mkdir(parents=True, exist_ok=True)
    with open(dst, "wb") as fh:
        writer.write(fh)
    enc = "encrypted" if reader.is_encrypted else "clear"
    print(f"ok: re-saved {enc} {src.name} -> {dst} ({len(reader.pages)} pages, no encryption)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
