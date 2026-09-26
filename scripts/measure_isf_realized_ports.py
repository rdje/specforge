#!/usr/bin/env python3
"""Does every port an emitted `.isf` declares reach the generated module? (FSMGEN-REQUESTS.6). Read-only.

A strict check passes on an `.isf` whose declared ports FSMGen may still leave out of the generated HDL: on the pin
`54ac1c22` an input that no transaction reads is dropped, and an undriven output is dropped on every revision up to
FSMGen's `.33.9` repair. So this script generates SystemVerilog for each current chain's emitted `.isf` inside a
COMPLETE read-only FSMGen export (FSMGen writes generated files only inside its own tree, and SPECFORGE never writes
into `subs/fsmgen`), then compares the module's port list with the `.isf` interface declarations.

Export a revision first (docs/upstream/fsmgen/PROCESS.md §2), and strict-check a known-good input on it:

    git -C <FSMGEN_CHECKOUT> archive <rev> | tar -x -C generated/tmp/fsmgen-<rev>

Usage: python3 scripts/measure_isf_realized_ports.py generated/tmp/fsmgen-<rev>
Writes only under <export>/scratch-specforge/. Exit 0 when every declared port is realized, 1 otherwise.
"""

from __future__ import annotations

import json
import re
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PORT = {"input": re.compile(r"\(input\s+([A-Za-z_]\w*)"), "output": re.compile(r"\(output\s+([A-Za-z_]\w*)")}
MODULE_PORT = {
    kind: re.compile(rf"\b{kind}\s+(?:wire|logic|reg)?\s*(?:\[[^\]]*\]\s*)?([A-Za-z_]\w*)") for kind in PORT
}


def module_header(sv: str) -> str:
    start = sv.find("module ")
    return sv[start : sv.find(");", start) + 2] if start >= 0 else ""


def main() -> int:
    if len(sys.argv) != 2:
        print(__doc__, file=sys.stderr)
        return 2
    export = Path(sys.argv[1]).resolve()
    if not (export / "bin" / "fsmgen").is_file():
        print(f"not an FSMGen export: {export}", file=sys.stderr)
        return 2
    retained = json.loads((ROOT / "doctrine/chain_currency/retained_bundles.json").read_text())["retained"]
    scratch = export / "scratch-specforge"
    totals = {kind: [0, 0] for kind in PORT}
    for key in sorted(retained):
        for isf in sorted((ROOT / "generated/adapters/isf" / key).glob("*.isf")):
            work = scratch / key / isf.stem
            if work.exists():
                shutil.rmtree(work)
            work.mkdir(parents=True)
            source = work / "in.isf"
            shutil.copy(isf, source)
            run = subprocess.run(
                ["./bin/fsmgen", "--strict", "--outdir", str(work / "lowered"), "-l", "sv", "-o", str(work / "out.sv"),
                 str(source)],
                cwd=export, capture_output=True, text=True,
            )
            text = isf.read_text()
            header = module_header((work / "out.sv").read_text()) if run.returncode == 0 else ""
            row = [f"{key} {isf.name}: generate exit {run.returncode}"]
            for kind, pattern in PORT.items():
                declared = pattern.findall(text)
                realized = set(MODULE_PORT[kind].findall(header))
                absent = [name for name in declared if name not in realized]
                totals[kind][0] += len(declared)
                totals[kind][1] += len(absent)
                row.append(f"{kind}s {len(declared)} declared, {len(absent)} absent {absent[:6]}")
            print("; ".join(row))
    print("; ".join(f"{kind}s: {d} declared, {a} absent from the generated module" for kind, (d, a) in totals.items()))
    return 0 if all(absent == 0 for _, absent in totals.values()) else 1


if __name__ == "__main__":
    sys.exit(main())
