#!/usr/bin/env python3
"""GriTS cross-tool orchestrator (GRITS-CROSS-TOOL.2).

Emits the witness JSON consumed by `specforge grits-consensus`. Witnesses are extractors whose
errors are UNCORRELATED with docling's (the system under test, never a witness):
  - pdfplumber  — geometric / PDF content-stream (no ML); the orthogonal witness.
  - qwen2.5vl   — vision LLM (with --vlm); coverage on borderless tables + a tiebreaker.
docling's grids come from the SourceIR (the *prediction*). Tables are matched by physical page
(docling `page_0NNN` == pdfplumber page N), then by best cell-overlap on that page.

Run with the eval venv:  .venv-eval/bin/python scripts/grits_cross_tool.py <pdf> <source_ir.json> [--vlm] [--max N]
Output (stdout): {"tables":[{"table_id","page","witnesses":[grid,...],"prediction":grid}]}.
"""
import base64
import io
import json
import re
import sys

import pdfplumber

OLLAMA = "http://localhost:11434/api/generate"


def docling_tables(source_ir_path):
    s = json.load(open(source_ir_path))
    out = []
    for t in s.get("structured_tables", []):
        rows = []
        for row in (t.get("header_rows") or []) + (t.get("body_rows") or []):
            rows.append([(c.get("text") or "").strip() for c in row])
        m = re.search(r"(\d+)", str(t.get("page_id")))
        out.append({
            "table_id": t.get("table_id"),
            "page": int(m.group(1)) if m else None,
            "grid": rows,
        })
    return out


def cells(grid):
    return {(r, c, (t or "").strip().lower())
            for r, row in enumerate(grid) for c, t in enumerate(row) if (t or "").strip()}


def overlap(g1, g2):
    a, b = cells(g1), cells(g2)
    return len(a & b) / max(1, len(a | b))


def vlm_table_grid(pil_image):
    """Ask qwen2.5vl for the dominant table in an image as a JSON grid (rows of cells)."""
    import urllib.request
    buf = io.BytesIO()
    pil_image.save(buf, format="PNG")
    b64 = base64.b64encode(buf.getvalue()).decode()
    prompt = ("Extract the single most prominent TABLE in this image as JSON: a list of rows, each "
              "row a list of cell strings (left-to-right). Output ONLY the JSON array, no prose.")
    req = {"model": "qwen2.5vl:7b", "prompt": prompt, "images": [b64], "stream": False,
           "options": {"temperature": 0}}
    try:
        r = urllib.request.urlopen(urllib.request.Request(
            OLLAMA, data=json.dumps(req).encode(), headers={"Content-Type": "application/json"}),
            timeout=120)
        resp = json.loads(r.read()).get("response", "")
        m = re.search(r"\[.*\]", resp, re.S)
        if not m:
            return None
        grid = json.loads(m.group(0))
        return [[str(c) for c in row] for row in grid if isinstance(row, list)]
    except Exception:
        return None


def main():
    pdf_path, source_ir = sys.argv[1], sys.argv[2]
    use_vlm = "--vlm" in sys.argv
    cap = int(sys.argv[sys.argv.index("--max") + 1]) if "--max" in sys.argv else 8

    docling = docling_tables(source_ir)
    out_tables = []
    with pdfplumber.open(pdf_path) as pdf:
        # pdfplumber tables by physical page (1-based)
        pp_by_page = {}
        for pno, page in enumerate(pdf.pages, start=1):
            grids = [[[(c or "").strip() for c in row] for row in t]
                     for t in (page.extract_tables() or [])]
            grids = [[r for r in g if any(r)] for g in grids if g]
            if grids:
                pp_by_page[pno] = grids
        for dt in docling:
            page = dt["page"]
            witnesses = []
            # pdfplumber witness: the table on this page best overlapping docling's grid (if any).
            if page in pp_by_page:
                witnesses.append(max(pp_by_page[page], key=lambda g: overlap(g, dt["grid"])))
            # qwen2.5vl witness: runs on docling's page directly (vision shares the semantic
            # notion of a table — the apt witness where pdfplumber finds only geometric grids).
            if use_vlm and page and 1 <= page <= len(pdf.pages):
                vg = vlm_table_grid(pdf.pages[page - 1].to_image(resolution=150).original)
                if vg:
                    witnesses.append(vg)
            if witnesses:
                out_tables.append({
                    "table_id": dt["table_id"], "page": page,
                    "witnesses": witnesses, "prediction": dt["grid"],
                })
            if len(out_tables) >= cap:
                break
    json.dump({"tables": out_tables}, sys.stdout)


if __name__ == "__main__":
    raise SystemExit(main())
