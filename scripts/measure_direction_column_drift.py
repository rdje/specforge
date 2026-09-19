#!/usr/bin/env python3
"""Which signal tables put their direction in a DIFFERENT column on different rows? (read-only)

`SIGNAL-DECLARATION-ROW-DROP.2j.1`. Two leaves of this tree are blocked on the same missing number.
`.2j` closed its rule half by refusing the direction-abbreviation arm and handed on a second
deliverable — *size ADIv6 `table_0108`'s column garble and route it*. `.2h.2` is blocked outright on
*"a corpus census of per-row layout drift"*, because CoreSight TMC `table_0074` is **mixed, not
rotated** — six rows name-last, the seventh name-first — so `.2e`'s whole-table offset cannot serve
it, and one table is not a grammar.

This producer is that census, and it is deliberately built out of the ONE vocabulary in a signal
table that is closed and unambiguous: the direction values themselves.

## Why not a name heuristic, measured rather than assumed

The obvious census asks which column holds the signal NAME and whether that column is stable. Two
cheap name tests were tried against this corpus first and both fail, in OPPOSITE directions:

  * "the cell is a single identifier-shaped token" misses `PWDATA_S [31:0]`, `sample_req ,` and
    `PRDATA_S Output` — the name is there, with a bit range, a comma, or a fused direction beside it;
  * "the cell's FIRST token is identifier-shaped" then accepts `Test Clock`, `Port Connected` and
    `Return Clock` — ordinary description prose whose first word is a plain word.

So a name test either under- or over-selects, and the error is not small: the first cut of this
census reported 30 tables and 116 rows, the second 11 rows, and neither number survived reading its
own selection. **Direction values have no such problem.** `Input`/`Output`/`In`/`Out`/`Inout`/… is a
closed set, a whole-cell match needs no judgement, and a table whose direction cell moves between
rows is exactly the table whose columns drifted — whatever the name column is doing.

A table is reported when its body rows disagree about which column holds the direction. A table where
every row agrees is CONSISTENT, whether that column is the first, the last, or the one the header
names: a uniform offset is what `.2e` already serves.

Read-only and deterministic: no network, no clock, no randomness, no write.

Usage:
    python3 scripts/measure_direction_column_drift.py              # the census
    python3 scripts/measure_direction_column_drift.py --rows       # every drifted table, row by row
    python3 scripts/measure_direction_column_drift.py --json
    python3 scripts/measure_direction_column_drift.py --self-test  # the RED cases
"""

from __future__ import annotations

import argparse
import glob
import json
import os
import sys
from collections import Counter

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from build_bounded_decision_baseline import repo_root  # noqa: E402

DIRECTION_VALUES = frozenset({
    "in", "out", "io", "i", "o", "input", "output", "inout",
    "in/out", "input/output", "bidirectional", "bidir",
})

# SIGNAL-DECLARATION-ROW-DROP.2h.2 — the vocabulary the PRODUCTION reader actually has
# (`LITERAL_DIRECTION_CELL_VALUES`, crates/specforge/src/ir/evidence.rs). A census is allowed a
# wider net than a rule: `.2h.0` measured `i`/`o`/`io`/`in`/`out` at 0 true positives and 18 false
# ones (a presence matrix writing `O` for *Optional*), `.2j` re-adjudicated the refusal corpus-wide
# and kept it, so no rule may read an abbreviation. The two nets therefore see different
# populations, and the difference is a number rather than a caveat: `--reader-vocabulary` reports
# what a RULE can reach, and ADIv6 `table_0108` — whose direction cells are all `In`/`Out` — is
# exactly the table that is in one and not the other.
READER_DIRECTION_VALUES = frozenset({"input", "output", "inout"})


def normalise(text: str | None) -> str:
    return " ".join((text or "").split())


def direction_column(row: list[dict], vocabulary: frozenset[str] = DIRECTION_VALUES) -> int | None:
    """The single column of this row whose WHOLE cell is a direction value, if there is exactly one."""
    hits = [i for i, cell in enumerate(row) if normalise(cell.get("text")).lower() in vocabulary]
    return hits[0] if len(hits) == 1 else None


def census(root: str, vocabulary: frozenset[str] = DIRECTION_VALUES) -> dict:
    tables: list[dict] = []
    shapes: Counter = Counter()
    for path in sorted(glob.glob(os.path.join(root, "generated/source_ir/*/source_ir.json"))):
        document = os.path.basename(os.path.dirname(path))
        with open(path, "r", encoding="utf-8") as handle:
            source = json.load(handle)
        declared_by_table: dict[str, list[str]] = {}
        evidence = os.path.join(root, f"generated/evidence_ir/{document}/evidence_ir.json")
        if os.path.exists(evidence):
            with open(evidence, "r", encoding="utf-8") as handle:
                for record in json.load(handle).get("table_signal_declaration_provenance") or []:
                    declared_by_table.setdefault(record.get("table_id"), []).append(record.get("signal_name"))
        for table in source.get("structured_tables") or []:
            if table.get("table_kind") != "signal_description":
                continue
            body = table.get("body_rows") or []
            if not body:
                continue
            columns = [direction_column(row, vocabulary) for row in body]
            present = [c for c in columns if c is not None]
            if not present:
                shapes["no_direction_value"] += 1
                continue
            if len(set(present)) == 1:
                shapes["consistent"] += 1
                continue
            shapes["drifted"] += 1
            declared = sorted(declared_by_table.get(table["table_id"], []))
            tables.append({
                "document": document,
                "table_id": table["table_id"],
                "caption": normalise(table.get("caption_text")),
                "header": [normalise(c.get("text")) for c in (table.get("header_rows") or [[]])[0]],
                "body_rows": len(body),
                "declarations": len(declared),
                "declared_names": declared,
                "direction_columns": {str(k): v for k, v in sorted(Counter(columns).items(), key=lambda kv: (kv[0] is None, kv[0]))},
                "rows": [[normalise(c.get("text")) for c in row] for row in body],
            })
    tables.sort(key=lambda t: (-(t["body_rows"] - t["declarations"]), t["document"], t["table_id"]))
    return {"shapes": dict(shapes), "tables": tables}


def summarise(result: dict) -> dict:
    tables = result["tables"]
    return {
        "signal_description_tables": sum(result["shapes"].values()),
        "consistent": result["shapes"].get("consistent", 0),
        "no_direction_value": result["shapes"].get("no_direction_value", 0),
        "drifted_tables": len(tables),
        "drifted_documents": len({t["document"] for t in tables}),
        "drifted_body_rows": sum(t["body_rows"] for t in tables),
        "drifted_declarations": sum(t["declarations"] for t in tables),
        "rows_with_no_declaration": sum(t["body_rows"] - t["declarations"] for t in tables),
    }


def render(result: dict, show_rows: bool, reader_vocabulary: bool = False) -> None:
    summary = summarise(result)
    scope = "the READER's vocabulary — what a rule can reach" if reader_vocabulary \
        else "the census vocabulary — abbreviations included"
    print("=== SIGNAL-DECLARATION-ROW-DROP.2j.1 — per-row direction-column drift (read-only) ===")
    print(f"  vocabulary: {scope}")
    print(f"  signal_description tables {summary['signal_description_tables']}"
          f"   consistent {summary['consistent']}"
          f"   no direction value {summary['no_direction_value']}"
          f"   DRIFTED {summary['drifted_tables']}")
    print()
    print(f"== DRIFTED: {summary['drifted_tables']} tables across {summary['drifted_documents']} documents,"
          f" {summary['drifted_body_rows']} body rows,"
          f" {summary['drifted_declarations']} declarations,"
          f" {summary['rows_with_no_declaration']} rows with none")
    print()
    for table in result["tables"]:
        undeclared = table["body_rows"] - table["declarations"]
        print(f"  {table['document'][:40]:42s} {table['table_id']:11s}"
              f" rows={table['body_rows']:3d} declared={table['declarations']:3d}"
              f" undeclared={undeclared:3d} direction column={table['direction_columns']}")
    print()
    print("  `rows - declared` is an UPPER BOUND on the loss and not a loss count: a body row may be a")
    print("  note or a continuation, and a declaration may be a PHANTOM. CoreSight TMC table_0074 reads")
    print("  1 undeclared row and is in fact worse — it publishes DATA, which is no signal of that")
    print("  table, contradicts its own rows on three directions, and drops ATIDM[6:0] and AFREADYM.")
    print("  WHERE DATA came from is ESTABLISHED by SIGNAL-DECLARATION-ROW-DROP.2h.2 and pinned below:")
    print("  it is the first token of the DESCRIPTION cell of the one name-first row, read as a name")
    print("  because the whole-table name-column override puts the name column at index 2 for every")
    print("  row. `.2j.1a` was right that the ARTIFACT cannot decide it — the statement carries no")
    print("  evidence span — and right to refuse the two accounts it named; both are refuted here.")
    print("  `declared` is read from the PERSISTED evidence_ir. For a legacy proofless document that")
    print("  is not necessarily what the current binary produces, and for this table it is not: the")
    print("  artifact records `DATA` where the current reader emits `Data`.")
    if not show_rows:
        print("\n  --rows prints every drifted table row by row; the adjudication is in")
        print("  docs/research/direction-column-drift-census.md")
        return
    for table in result["tables"]:
        print(f"\n-- {table['document']} {table['table_id']} — {table['caption'] or '(no caption)'}")
        print(f"   header: {table['header']}")
        print(f"   declared: {table['declared_names']}")
        for index, row in enumerate(table["rows"]):
            print(f"   row {index:2d}: {[c[:40] for c in row]}")


# SIGNAL-DECLARATION-ROW-DROP.2j.1a — the adjudicated instance, pinned so its three surviving claims
# are gated rather than asserted, and so the claim that did NOT survive cannot come back.
#
# `.2j.1` published that TMC `table_0074` "mints DATA from the English word *data* in a description
# cell". An audit refused that MECHANISM: the declaration statement carries NO evidence span, and two
# accounts — the word `data` in one description and the token `ATDATA` in another — predict the same
# observation. `CLAIM_VERIFICATION.md` §3: evidence consistent with both hypotheses illustrates, it
# does not test. What IS established is pinned below; the mechanism is recorded as unestablished.
ADJUDICATED_DOCUMENT = "ddi0461_b_2010_12_10_coresight_trace_memory_controller_technical_reference_manual"
ADJUDICATED_TABLE = "table_0074"


def adjudicated_instance(root: str) -> dict | None:
    """What TMC `table_0074` publishes, measured against what its own rows say."""
    src = os.path.join(root, f"generated/source_ir/{ADJUDICATED_DOCUMENT}/source_ir.json")
    ev = os.path.join(root, f"generated/evidence_ir/{ADJUDICATED_DOCUMENT}/evidence_ir.json")
    if not (os.path.exists(src) and os.path.exists(ev)):
        return None
    with open(src, "r", encoding="utf-8") as handle:
        tables = json.load(handle).get("structured_tables") or []
    table = next((t for t in tables if t.get("table_id") == ADJUDICATED_TABLE), None)
    if table is None:
        return None
    with open(ev, "r", encoding="utf-8") as handle:
        evidence = json.load(handle)
    statements = {s["statement_id"]: s for s in evidence.get("extracted_statements") or []}
    row_direction: dict[str, str | None] = {}
    for row in table.get("body_rows") or []:
        cells = [normalise(c.get("text")) for c in row]
        found = [c for c in cells if c.lower() in DIRECTION_VALUES]
        name = next((c for c in cells if c.upper().startswith(("AT", "AF"))), None)
        if name:
            row_direction[name.split()[0].upper().rstrip(",")] = found[0] if found else None
    published, contradicting, spanless = {}, [], []
    for record in evidence.get("table_signal_declaration_provenance") or []:
        if record.get("table_id") != ADJUDICATED_TABLE:
            continue
        statement = statements.get(record.get("statement_id"), {})
        text = normalise(statement.get("text"))
        said = text.split(" is ")[-1].rstrip(".") if " is " in text else None
        name = (record.get("signal_name") or "").upper()
        published[name] = said
        table_says = row_direction.get(name)
        if table_says and said and not table_says.lower().startswith(said[:2]):
            contradicting.append(name)
        if name not in row_direction and not statement.get("evidence_span_ids"):
            spanless.append(name)
    document_directions: Counter = Counter()
    for record in evidence.get("table_signal_declaration_provenance") or []:
        text = normalise(statements.get(record.get("statement_id"), {}).get("text"))
        if " is " in text:
            document_directions[text.split(" is ")[-1].rstrip(".")] += 1
    return {
        "document": ADJUDICATED_DOCUMENT,
        "table_id": ADJUDICATED_TABLE,
        "table_signals": sorted(row_direction),
        "published": published,
        "contradicting_direction": sorted(contradicting),
        "not_a_signal_of_the_table": sorted(set(published) - set(row_direction)),
        "not_a_signal_and_spanless": sorted(spanless),
        "table_signals_with_no_declaration": sorted(set(row_direction) - set(published)),
        "document_direction_split": dict(document_directions),
    }


def phantom_source_cells(root: str, phantom: str) -> list[tuple[int, int, str]]:
    """Every cell of the adjudicated table whose FIRST TOKEN is the phantom's published name.

    `SIGNAL-DECLARATION-ROW-DROP.2h.2`. `.2j.1a` refused `.2j.1`'s mechanism because the
    declaration's statement carries no evidence span, and named two accounts the artifact could not
    separate: the word *data* in `Trace data, LSB aligned`, and the token `ATDATA` in `Number of
    valid bytes on ATDATA ,`. Both are refuted WITHOUT leaving the artifact, because the reader does
    not scan a cell for a name — it takes the cell's FIRST whitespace token
    (`signal_names_in_name_cell`). So the question has an answer here: which cells of this table
    could have produced this name at all? There is exactly one, and it is the DESCRIPTION cell of
    the single row whose name comes first, at the column index the whole-table override chose.
    """
    src = os.path.join(root, f"generated/source_ir/{ADJUDICATED_DOCUMENT}/source_ir.json")
    if not os.path.exists(src):
        return []
    with open(src, "r", encoding="utf-8") as handle:
        tables = json.load(handle).get("structured_tables") or []
    table = next((t for t in tables if t.get("table_id") == ADJUDICATED_TABLE), None)
    if table is None:
        return []
    hits = []
    for row_index, row in enumerate(table.get("body_rows") or []):
        for column, cell in enumerate(row):
            first = normalise(cell.get("text")).split(" ")[0] if normalise(cell.get("text")) else ""
            if first.strip(".,;:").upper() == phantom.upper():
                hits.append((row_index, column, normalise(cell.get("text"))))
    return hits


PINNED = {
    "drifted_tables": 9,
    "drifted_documents": 5,
    "drifted_body_rows": 91,
    "drifted_declarations": 40,
    "rows_with_no_declaration": 51,
}

# SIGNAL-DECLARATION-ROW-DROP.2h.2 — the same census under the READER's vocabulary. This is the
# population a RULE may act on, and it is one table smaller: ADIv6 `table_0108` writes `In`/`Out`,
# which `.2j` refused corpus-wide. It is pinned separately so neither number can be quoted for the
# other.
PINNED_READER_VOCABULARY = {
    "drifted_tables": 8,
    "drifted_documents": 4,
    "drifted_body_rows": 81,
}


def self_test(root: str) -> int:
    failures: list[str] = []
    passed = 0

    def check(case: str, condition: bool) -> None:
        nonlocal passed
        if condition:
            passed += 1
        else:
            failures.append(case)

    # RED 1 — the direction test is WHOLE-CELL. A description mentioning a direction word must not be
    # read as a direction cell, or every prose column becomes one.
    check("whole-cell-only", direction_column([
        {"text": "ATVALIDM"}, {"text": "Output"},
        {"text": "Valid signals in this cycle from the output of the master"}]) == 1)
    # RED 2 — a row with two direction cells is AMBIGUOUS and contributes no opinion, rather than the
    # first one it finds. CoreSight ihi0029 table_0037 has such rows, and guessing would invent drift.
    check("two-direction-cells-is-no-opinion", direction_column([
        {"text": "PWDATADBG[31:0]"}, {"text": "Output"}, {"text": "Input"}, {"text": "0"}]) is None)
    # RED 3 — a UNIFORM offset is not drift. It is exactly what `.2e`'s whole-table offset serves, and
    # reporting it would bury the nine real cases under the corpus.
    uniform = [[{"text": "Output"}, {"text": "desc"}, {"text": "ATVALIDM"}],
               [{"text": "Input"}, {"text": "desc"}, {"text": "ATREADYM"}]]
    check("uniform-offset-is-not-drift", len({direction_column(r) for r in uniform}) == 1)
    # RED 4 — drift is disagreement BETWEEN rows, whatever the header says. TMC table_0074 is headed
    # `Type`, not `Direction`, which is why a header-scoped census missed it entirely.
    drifted = [[{"text": "Output"}, {"text": "desc"}, {"text": "ATVALIDM"}],
               [{"text": "AFREADYM"}, {"text": "Output"}, {"text": "desc"}]]
    check("drift-is-row-disagreement", len({direction_column(r) for r in drifted}) == 2)

    # RED 5 — SIGNAL-DECLARATION-ROW-DROP.2j.1a. The adjudicated instance, gated rather than asserted.
    # Three claims survived the audit and are pinned; the one that did not is pinned as UNESTABLISHED.
    inst = adjudicated_instance(root)
    check("instance-publishes-a-name-that-is-not-a-signal-of-its-table",
          inst is not None and inst["not_a_signal_of_the_table"] == ["DATA"])
    check("instance-contradicts-its-own-table-on-three-directions",
          inst is not None and inst["contradicting_direction"] == ["ATBYTESM", "ATDATAM", "ATVALIDM"])
    check("instance-drops-two-real-wires",
          inst is not None and inst["table_signals_with_no_declaration"] == ["AFREADYM", "ATIDM[6:0]"])
    # `input` is not a blanket default, and the evidence is the document's OWN split. `.2j.1` cited
    # 121/101 here, which belong to a DIFFERENT document (CoreSight SDC-600); this one reads 15/8.
    check("instance-direction-is-read-not-defaulted",
          inst is not None and len(inst["document_direction_split"]) > 1
          and min(inst["document_direction_split"].values()) > 0)
    # The MECHANISM is unestablished and must stay that way until something separates the accounts:
    # the phantom's statement carries no evidence span, so nothing in the artifact says where it came
    # from. `.2j.1` published "minted from the English word data" and the audit refused it.
    check("phantom-mechanism-is-unestablished-because-the-statement-has-no-span",
          inst is not None and inst["not_a_signal_and_spanless"] == ["DATA"])

    # RED 6 — SIGNAL-DECLARATION-ROW-DROP.2h.2. The mechanism `.2j.1a` recorded as unestablished,
    # established from the artifact alone: exactly ONE cell of this table can produce the published
    # name under the reader's own name rule, and it is the description of the name-first row.
    sources = phantom_source_cells(root, "DATA")
    check("phantom-has-exactly-one-possible-source-cell", len(sources) == 1)
    check("phantom-source-is-the-description-of-the-name-first-row",
          len(sources) == 1 and sources[0][0] == 6 and sources[0][1] == 2
          and sources[0][2].startswith("Data flush complete"))
    # and the two accounts the audit named are refuted: neither cell STARTS with the name, which is
    # the only position the reader reads.
    check("phantom-is-not-the-word-data-inside-a-description",
          all(not (index == 4) for index, _, _ in sources))
    check("phantom-is-not-the-token-atdata-inside-a-description",
          all(not (index == 3) for index, _, _ in sources))

    # RED 7 — the live census still has the shape this leaf adjudicated.
    summary = summarise(census(root))
    for key, expected in PINNED.items():
        check(f"census-pin-{key}", summary[key] == expected)

    # RED 8 — and the READER's narrower vocabulary reaches one table fewer, in one document fewer.
    reader_summary = summarise(census(root, READER_DIRECTION_VALUES))
    for key, expected in PINNED_READER_VOCABULARY.items():
        check(f"reader-vocabulary-pin-{key}", reader_summary[key] == expected)
    check("the-abbreviation-table-is-in-the-census-and-not-in-the-rule",
          any(t["table_id"] == "table_0108" for t in census(root)["tables"])
          and all(t["table_id"] != "table_0108"
                  for t in census(root, READER_DIRECTION_VALUES)["tables"]))

    total = passed + len(failures)
    for case in failures:
        print(f"  RED FAIL: {case}", file=sys.stderr)
    print(f"direction column drift: {passed}/{total} RED cases pass")
    return 1 if failures else 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rows", action="store_true", help="print every drifted table row by row")
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    parser.add_argument("--self-test", action="store_true", help="run the RED cases")
    parser.add_argument("--reader-vocabulary", action="store_true",
                        help="census with the production reader's direction words only "
                             "(input/output/inout) — the population a RULE may act on")
    args = parser.parse_args()
    root = repo_root()
    if args.self_test:
        return self_test(root)
    vocabulary = READER_DIRECTION_VALUES if args.reader_vocabulary else DIRECTION_VALUES
    result = census(root, vocabulary)
    if args.json:
        json.dump({"summary": summarise(result), **result}, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return 0
    render(result, args.rows, args.reader_vocabulary)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
