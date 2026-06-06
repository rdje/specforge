#!/usr/bin/env python3
"""GriTS cross-tool witness: extract table grids from a PDF with pdfplumber.

pdfplumber is a GEOMETRIC / content-stream table extractor (no ML) — an INDEPENDENT
witness for evaluating docling's (ML) table extraction. We never grade a tool against
itself; the value here is that pdfplumber's failure modes are uncorrelated with
docling's, so where they AGREE is high-confidence gold (TABLE-GRITS-CONFORMAL; cross-tool
agreement = silver gold). Run with the eval venv: `.venv-eval/bin/python`.

Output (stdout): {"tables": [{"page": <1-based>, "rows": [[cell, ...], ...]}, ...]}.
"""
import json
import sys

import pdfplumber


def main() -> int:
    if len(sys.argv) < 2:
        print("usage: pdfplumber_tables.py <pdf>", file=sys.stderr)
        return 2
    out = {"tables": []}
    with pdfplumber.open(sys.argv[1]) as pdf:
        for page_no, page in enumerate(pdf.pages, start=1):
            for table in page.extract_tables() or []:
                rows = [[(cell or "").strip() for cell in row] for row in table]
                rows = [r for r in rows if any(c for c in r)]  # drop fully-empty rows
                if rows:
                    out["tables"].append({"page": page_no, "rows": rows})
    json.dump(out, sys.stdout)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
