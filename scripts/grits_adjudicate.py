#!/usr/bin/env python3
"""Render the disputed-cell regions for the adjudication queue (GRITS-CROSS-TOOL.3).

Completes the cross-tool loop: `grits-consensus --adjudicate-out` lists the cells where docling
disagrees with the consensus gold; this renders each disputed table's page to a PNG so an
EVIDENCE-GROUNDED AGENT (or a human) can rule each split against the SOURCE — never by a correlated
vote. Updates the queue in place with an `image` path per cell; the agent then reads each image and
records the verdict.

Run with the eval venv:  .venv-eval/bin/python scripts/grits_adjudicate.py <queue.json> <pdf> <outdir>
"""
import json
import os
import sys

import pdfplumber


def main():
    queue_path, pdf_path, outdir = sys.argv[1], sys.argv[2], sys.argv[3]
    os.makedirs(outdir, exist_ok=True)
    queue = json.load(open(queue_path))
    pages = sorted({c["page"] for c in queue["cells"] if c.get("page")})
    rendered = {}
    with pdfplumber.open(pdf_path) as pdf:
        for p in pages:
            if 1 <= p <= len(pdf.pages):
                path = os.path.join(outdir, f"page_{p:04d}.png")
                pdf.pages[p - 1].to_image(resolution=220).save(path)
                rendered[p] = path
    for c in queue["cells"]:
        c["image"] = rendered.get(c.get("page"))
    json.dump(queue, open(queue_path, "w"), indent=2)
    print(f"rendered {len(rendered)} page(s) for {len(queue['cells'])} disputed cells -> {outdir}")


if __name__ == "__main__":
    raise SystemExit(main())
