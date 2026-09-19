#!/usr/bin/env python3
"""The frozen per-row baseline the bounded-decision evaluation is scored against (read-only).

`BOUNDED-DECISION-PROVIDER.1` — a remote decision provider may only be admitted if it beats what
SpecForge already does, and today the two candidate defects are quantified as AGGREGATE
percentages (`18.3%` of declaration rows discarded, `739` captions admitted) rather than as a
per-row set a later change can be scored against. Without that set, "the model improved things"
is unfalsifiable, so this producer builds it, freezes it, and scores the current rules on it.

Two decisions, both of the shape a constrained decision model could answer -- a yes/no over a
candidate the deterministic pass already located:

  `declaration_row`    does this signal-table row declare an interface signal?
                       Owned by `SIGNAL-DECLARATION-ROW-DROP`.
  `caption_admission`  does this figure/table caption state a normative constraint?
                       Owned by `INVARIANT-SHAPE-ADMISSION`.

## The population, and why it is these four documents

`extraction_manifest.declaration_row_accounting` is the reader's own denominator, and only the
documents rebuilt since `SIGNAL-DECLARATION-ROW-DROP.1` shipped carry it: AXI `ihi0022_l`, APB
`ihi0024_e`, AHB `ihi0033_c`, ADIv6 `ihi0074_a` -- which are also the four wire-bearing
proof-carrying chains. Both decisions use the same four documents so the frozen set is one
population with one digest.

## Scoring the PRODUCT decision, not the reader's intermediate one

A row is scored `signal` only when a declaration from it SURVIVES to
`table_signal_declaration_provenance`. The distinction is load-bearing and was measured rather
than assumed: AXI `table_0011` (`Table A2.3: Credited channel signals`) has
`declarations_emitted == 6` in the accounting and ZERO provenance records, because
`withhold_base_name_template_declarations` (`WIRE-BASED-100.10b`) removes a base-name template
table's declarations after every table producer has run. Scoring the accounting alone would have
published six false positives the product does not make.

## Authoring, freezing, and re-derivation

The gold labels live in `D1_TABLE_VERDICTS` / `D2_CONSTRAINT_STATEMENTS` below, each with the
reason it was decided that way, so the adjudication is reviewable as code. `--emit` renders them
against the persisted artifacts into the tracked frozen set
`docs/research/bounded-decision-adjudication.jsonl`, which carries every row's cells verbatim --
so scoring re-derives from the TRACKED set alone, with no dependency on untracked `generated/`
state. `--verify-currency` is the separate question of whether the corpus still produces that
population.

Read-only and deterministic: no network, no clock, no randomness, and no write outside `--emit`.

Usage:
    python3 scripts/build_bounded_decision_baseline.py                 # score arm A, human report
    python3 scripts/build_bounded_decision_baseline.py --json
    python3 scripts/build_bounded_decision_baseline.py --check         # digest + pinned result
    python3 scripts/build_bounded_decision_baseline.py --self-test     # RED cases
    python3 scripts/build_bounded_decision_baseline.py --verify-currency
    python3 scripts/build_bounded_decision_baseline.py --emit          # regenerate the frozen set
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from measure_parametric_width_cell_shapes import (  # noqa: E402
    column_selection,
    repo_root,
    signal_names_in_name_cell,
)

ADJUDICATION_PATH = "docs/research/bounded-decision-adjudication.jsonl"

# The frozen set's identity. `--check` refuses any drift, because a baseline that can be edited
# without a failure is not a baseline (`CLAIM_VERIFICATION.md` anti-pattern: a current constant
# carried in prose with a comment telling future authors to refresh it).
ADJUDICATION_SHA256 = "d3c5898f657ee6c12c3ef062dbd3d33fe34931a48c08f256ada55a25cf403d05"

DOCUMENTS = (
    "ihi0022_l_2025_08_amba_axi_protocol_specification",
    "ihi0024_e_2023_02_amba_5_apb_protocol_specification",
    "ihi0033_c_2021_09_amba_5_ahb_protocol_specification",
    "ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification",
)

# A cell is kept verbatim up to this bound so the frozen set stays adjudicable from its own bytes
# without carrying whole description paragraphs. Truncation is recorded per row.
CELL_TEXT_BOUND = 96
CAPTION_TEXT_BOUND = 400

# `statement_is_a_caption` (crates/specforge/src/ir/semantic.rs), mirrored -- and already mirrored
# once in `scripts/measure_invariant_admission_shape.py`, which this agrees with by construction.
CAPTION = re.compile(r"^(Figure|Table)\s+[A-Za-z]?[0-9]")

# `is_invariant_like` route r1 (crates/specforge/src/ir/semantic.rs), mirrored verbatim. Route r1
# is tested BEFORE the caption refusal, so on this stratum the current rule is exactly "admit iff
# one of these phrases occurs".
MODAL_PHRASES = (
    "must",
    "shall",
    "always",
    "never",
    "required",
    "remains",
    "remain",
    "until",
    "only when",
    "cannot",
    "must not",
    "shall not",
)

# A deontic net deliberately WIDER than production's route `r1`, used only to screen the caption
# stratum: a caption carrying none of these, in a document that writes requirements with them
# everywhere else, is not a requirement. Reported rather than asserted, so "none of the other 607
# is a requirement" is a number a reader can re-derive instead of a claim about how hard I looked.
WIDE_DEONTIC = MODAL_PHRASES + (
    "should",
    "recommended",
    "mandatory",
    "forbidden",
    "illegal",
    "prohibited",
    "not permitted",
    "allowed",
    "reserved",
    "only",
    "has to",
    "have to",
    "need to",
    "obliged",
    "ensure",
    "guarantee",
    "is not",
    "are not",
    "no ",
)

# The verbs a cross-reference uses to report what a figure or table holds. A sentence whose main
# verb is one of these states nothing of its own, whatever its object contains.
REPORTING_VERB = re.compile(
    r"\b(shows?|lists?|summari[sz]es?|provides?|describes?|illustrates?|gives?|defines?|details?"
    r"|contains?|displays?|indicates?|presents?|specifies|specify|explains?|outlines?|applies"
    r"|apply|introduces?|repeats?|continues?|continued)\b",
    re.IGNORECASE,
)


def screen_captions(records: list[dict]) -> dict:
    """How the caption stratum decomposes -- the evidence behind "the rest state nothing"."""
    counts = {"deontic": 0, "pure_title": 0, "reporting_sentence": 0, "other_sentence": 0}
    other: list[str] = []
    for record in records:
        if record["decision"] != "caption_admission":
            continue
        text = record["text"]
        if any(phrase in text.lower() for phrase in WIDE_DEONTIC):
            counts["deontic"] += 1
        elif not (text.endswith(".") or ". " in text):
            counts["pure_title"] += 1
        elif REPORTING_VERB.search(text):
            counts["reporting_sentence"] += 1
        else:
            counts["other_sentence"] += 1
            other.append(f'{record["statement_id"]}: {text[:120]}')
    counts["other_sentence_rows"] = other
    return counts


# ── Decision 1: the gold, adjudicated per table ────────────────────────────────────────────────
#
# Every one of the 101 tables in this population is UNIFORMLY real or UNIFORMLY phantom -- not one
# is mixed. That reproduces `[[a-dropped-declaration-row-is-usually-not-a-signal]]`'s finding on a
# population 10x larger, and it is the single most consequential fact about this decision: a
# per-row ranking model is being asked a question whose answer is settled one level up.
#
# Listed here are only the tables whose rows declare NO interface signal; every other table in the
# accounting is `signal` for every row. Each entry carries the reason, which is a property of the
# table's own shape or of what the document states about it -- never of which protocol it is.
D1_TABLE_VERDICTS = {
    ("ihi0022_l_2025_08_amba_axi_protocol_specification", "table_0011"): "base_name_template",
    ("ihi0022_l_2025_08_amba_axi_protocol_specification", "table_0092"): "metavariable_grid",
    ("ihi0022_l_2025_08_amba_axi_protocol_specification", "table_0183"): "width_parameters",
    ("ihi0022_l_2025_08_amba_axi_protocol_specification", "table_0184"): "width_parameters",
    ("ihi0022_l_2025_08_amba_axi_protocol_specification", "table_0199"): "encoding_matrix",
    ("ihi0022_l_2025_08_amba_axi_protocol_specification", "table_0265"): "legend",
    ("ihi0033_c_2021_09_amba_5_ahb_protocol_specification", "table_0014"): "encoding_rows",
    ("ihi0033_c_2021_09_amba_5_ahb_protocol_specification", "table_0019"): "encoding_rows",
}

# Every basis id the records use, stated once in the frozen set's own header record rather than
# repeated on 1,257 rows. Each is a property of the table's own shape, or of what the document
# states about it -- never of which protocol it is.
BASES = {
    "signal_description_row": "a signal-description row whose name cell names an interface "
    "signal of the interface this document specifies",
    "base_name_template": "the document states these are BASE names, not wires: the table's own "
    "introducing sentence reads 'Signal names are the base name, when instantiated each includes "
    "a prefix to indicate which channel'. Production already agrees — "
    "`withhold_base_name_template_declarations` (WIRE-BASED-100.10b) removes them, which is why "
    "the accounting says 6 emitted and the provenance carries none.",
    "metavariable_grid": "a metavariable grid: every cell is an `Ax` form the document never "
    "declares as a wire (`AXLEN` is undeclared while `AWLEN`/`ARLEN` are). Reviewed in "
    "`[[a-dropped-declaration-row-is-usually-not-a-signal]]`.",
    "width_parameters": "width PARAMETERS, not wires — the header is "
    "`Name | Values | Default | Description` and the values are ranges (`0..8`). Reviewed.",
    "encoding_matrix": "an encoding matrix whose name column holds address-space VALUES "
    "(`Secure`, `Realm`, `NSP`), with the real signals in the header. Reviewed.",
    "legend": "a LEGEND — `Y | Mandatory | Mandatory`, `N | Not present | Not present`. Reviewed.",
    "encoding_rows": "an encoding table: the name column holds the values, or the meanings of the "
    "bit combinations, of the signal named in the header. Reviewed.",
    "caption_not_a_statement": "a label plus a noun phrase, or a sentence whose main verb reports "
    "what a figure or table shows",
    "finite_prohibition": "a finite main clause stating a prohibition about the specified system",
    "finite_requirement": "a finite main clause stating a requirement about the specified system",
}

# Rows whose label carries a caveat worth keeping beside it. The label is still the label; this
# records what a second annotator would most likely argue with.
D1_ROW_NOTES = {
    ("ihi0033_c_2021_09_amba_5_ahb_protocol_specification", "table_0033", 1): (
        "`HRESET` is the text layer's spelling of `HRESETn`. The row declares the global reset "
        "signal; the lost `n` is an ingest defect, not an admission one."
    ),
}

# ── Decision 2: the gold, adjudicated per statement ────────────────────────────────────────────
#
# The rubric, applied to every caption-shaped statement in the four documents:
#
#   `constraint`      the text contains a FINITE MAIN CLAUSE stating an obligation, prohibition,
#                     or permission restriction about the specified system.
#   `not_constraint`  a title (a label plus a noun phrase, no finite main clause), or a sentence
#                     whose main verb only reports what a figure or table shows.
#
# The two halves of that rubric are what separate `Table A8.2: Opcodes which must be cache line
# sized` (a title -- refused) from `Other combinations are not permitted.` (a prohibition --
# admitted), and neither turns on a vocabulary list.
D2_CONSTRAINT_STATEMENTS = {
    ("ihi0022_l_2025_08_amba_axi_protocol_specification", "statement_1426"): "finite_prohibition",
    ("ihi0022_l_2025_08_amba_axi_protocol_specification", "statement_3781"): "finite_prohibition",
    ("ihi0033_c_2021_09_amba_5_ahb_protocol_specification", "statement_0570"): "finite_prohibition",
    ("ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification", "statement_2092"): "finite_prohibition",
    ("ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification", "statement_2139"): "finite_prohibition",
    ("ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification", "statement_4660"): "finite_requirement",
}

# Where this adjudication would most plausibly be argued with, recorded rather than hidden. Every
# one of these is a deontic WORD inside a noun phrase or a subordinate clause, which the rubric
# refuses and a looser reading would admit.
D2_CONTESTED = {
    "statement_2299": "'Opcodes which must be cache line sized' -- a modal inside a TITLE.",
    "statement_2976": "'Required sequence of ...' -- a deontic adjective inside a TITLE.",
    "statement_6129": "'summarizes the required and recommended components' -- deontic adjectives "
    "under a reporting verb.",
    "statement_0670": "'the structure required to implement ...' -- a deontic adjective "
    "qualifying a noun under 'shows'.",
    "statement_2170": "'shows ... the sequence that a JTAG device must recognize' -- a modal in a "
    "relative clause under 'shows'. The closest call in the set.",
    "statement_0631": "'shows the recommended mapping' -- a deontic adjective under 'shows'.",
    "statement_0831": "'This only requires the addition of external logic' -- a finite clause, but "
    "about a design technique rather than an obligation on the interface.",
    "statement_1426": "admitted on its second sentence; its first carries a modal only inside a "
    "relative clause.",
}


def load_json(path: str) -> dict:
    with open(path, "r", encoding="utf-8") as handle:
        return json.load(handle)


def bound(text: str, limit: int) -> tuple[str, bool]:
    collapsed = " ".join(text.split())
    if len(collapsed) <= limit:
        return collapsed, False
    return collapsed[:limit], True


# ── Building the two populations from the persisted artifacts ──────────────────────────────────


def declaration_rows(root: str) -> list[dict]:
    """Every row the body-row declaration reader was handed, with the product's decision on it."""
    rows: list[dict] = []
    for document in DOCUMENTS:
        evidence = load_json(
            os.path.join(root, "generated/evidence_ir", document, "evidence_ir.json")
        )
        source = load_json(os.path.join(root, "generated/source_ir", document, "source_ir.json"))
        accounting = (evidence.get("extraction_manifest") or {}).get(
            "declaration_row_accounting"
        ) or []
        tables = {t.get("table_id"): t for t in source.get("structured_tables") or []}
        survived: dict[str, set[str]] = {}
        for record in evidence.get("table_signal_declaration_provenance") or []:
            survived.setdefault(record["table_id"], set()).add(record["signal_name"])

        for entry in accounting:
            table_id = entry["table_id"]
            table = tables.get(table_id)
            if table is None:
                raise SystemExit(f"error: {document} {table_id} has accounting but no table")
            name_col, _offset, _width_col = column_selection(table)
            headers = [cell["text"] for cell in (table.get("header_rows") or [[]])[0]]
            caption, _ = bound(table.get("caption_text") or "", CAPTION_TEXT_BOUND)
            pending = [drop["name_cell"] for drop in entry.get("dropped_rows") or []]
            reasons = [drop["reason"] for drop in entry.get("dropped_rows") or []]
            cursor = 0

            for row_index, row in enumerate(table.get("body_rows") or []):
                raw_name = row[name_col]["text"].strip() if name_col < len(row) else ""
                candidates = [name for name in signal_names_in_name_cell(raw_name) if name]
                if cursor < len(pending) and raw_name == pending[cursor]:
                    reader, reason = "dropped", reasons[cursor]
                    cursor += 1
                elif any(name in survived.get(table_id, set()) for name in candidates):
                    reader, reason = "emitted", None
                else:
                    # The reader built a declaration and a later producer withheld it. Measured,
                    # not assumed: this is AXI `table_0011` and nothing else in the population.
                    reader, reason = "withheld", None

                cells = []
                truncated = False
                for cell in row:
                    text, cut = bound(cell["text"], CELL_TEXT_BOUND)
                    truncated = truncated or cut
                    cells.append(text)
                basis = D1_TABLE_VERDICTS.get((document, table_id))
                record = {
                    "decision": "declaration_row",
                    "document": document,
                    "table_id": table_id,
                    "row_index": row_index,
                    "caption": caption,
                    "headers": headers,
                    "cells": cells,
                    "name_col": name_col,
                    "candidate_names": candidates,
                    "reader": reader,
                    "gold": "not_signal" if basis else "signal",
                    "basis_id": basis or "signal_description_row",
                }
                if truncated:
                    record["cells_truncated"] = True
                if reason:
                    record["reader_reason"] = reason
                note = D1_ROW_NOTES.get((document, table_id, row_index))
                if note:
                    record["note"] = note
                rows.append(record)
            if cursor != len(pending):
                raise SystemExit(
                    f"error: {document} {table_id} matched {cursor} of {len(pending)} dropped rows"
                )
    return rows


def caption_statements(root: str) -> list[dict]:
    """Every caption-shaped extracted statement, with the product's admission decision."""
    rows: list[dict] = []
    for document in DOCUMENTS:
        evidence = load_json(
            os.path.join(root, "generated/evidence_ir", document, "evidence_ir.json")
        )
        semantic = load_json(
            os.path.join(root, "generated/semantic_ir", document, "semantic_ir.json")
        )
        admitted = {record["statement"] for record in semantic.get("invariants") or []}
        for statement in evidence.get("extracted_statements") or []:
            collapsed = " ".join(statement["text"].split())
            if not CAPTION.match(collapsed):
                continue
            lowered = collapsed.lower()
            route = "r1_modal" if any(p in lowered for p in MODAL_PHRASES) else "caption_refusal"
            text, truncated = bound(collapsed, CAPTION_TEXT_BOUND)
            key = (document, statement["statement_id"])
            basis = D2_CONSTRAINT_STATEMENTS.get(key)
            record = {
                "decision": "caption_admission",
                "document": document,
                "statement_id": statement["statement_id"],
                "text": text,
                "reader": "admitted" if statement["text"] in admitted else "refused",
                "reader_route": route,
                "gold": "constraint" if basis else "not_constraint",
                "basis_id": basis or "caption_not_a_statement",
            }
            if truncated:
                record["text_truncated"] = True
            note = D2_CONTESTED.get(statement["statement_id"])
            if note:
                record["note"] = note
            rows.append(record)
    return rows


def rubric_header() -> dict:
    """The frozen set's own header: what each decision asks, and what each basis id means."""
    return {
        "record_type": "rubric",
        "schema_version": 1,
        "owner": "BOUNDED-DECISION-PROVIDER.1",
        "producer": "scripts/build_bounded_decision_baseline.py",
        "documents": list(DOCUMENTS),
        "decisions": {
            "declaration_row": "does this signal-table row declare an interface signal? The "
            "reader's answer is `emitted` when a declaration from the row SURVIVES to "
            "`table_signal_declaration_provenance`; `withheld` when it was built and a later "
            "producer removed it; `dropped` when the reader refused the row outright.",
            "caption_admission": "does this figure/table caption state a normative constraint? "
            "`constraint` requires a FINITE MAIN CLAUSE stating an obligation, prohibition, or "
            "permission restriction about the specified system. A title (a label plus a noun "
            "phrase) and a sentence whose main verb only reports what a figure shows are both "
            "`not_constraint`, however many deontic words they contain.",
        },
        "bases": BASES,
        "cell_text_bound": CELL_TEXT_BOUND,
        "caption_text_bound": CAPTION_TEXT_BOUND,
    }


def render_adjudication(root: str) -> str:
    records = [rubric_header()] + declaration_rows(root) + caption_statements(root)
    return "".join(json.dumps(r, sort_keys=True, ensure_ascii=False) + "\n" for r in records)


# ── Scoring ────────────────────────────────────────────────────────────────────────────────────

POSITIVE = {"declaration_row": "signal", "caption_admission": "constraint"}
PREDICT_POSITIVE = {"declaration_row": {"emitted"}, "caption_admission": {"admitted"}}


def f1(true_positive: int, false_positive: int, false_negative: int) -> tuple[float, float, float]:
    precision = true_positive / (true_positive + false_positive) if true_positive + false_positive else 0.0
    recall = true_positive / (true_positive + false_negative) if true_positive + false_negative else 0.0
    harmonic = 2 * precision * recall / (precision + recall) if precision + recall else 0.0
    return precision, recall, harmonic


def score(records: list[dict]) -> dict:
    """Arm A -- the current deterministic rules -- per decision, over the frozen set."""
    result: dict = {}
    for decision in ("declaration_row", "caption_admission"):
        subset = [r for r in records if r["decision"] == decision]
        positive = POSITIVE[decision]
        predicted = PREDICT_POSITIVE[decision]
        matrix = {"tp": 0, "fp": 0, "fn": 0, "tn": 0}
        errors = []
        for record in subset:
            says_positive = record["reader"] in predicted
            is_positive = record["gold"] == positive
            if says_positive and is_positive:
                matrix["tp"] += 1
            elif says_positive and not is_positive:
                matrix["fp"] += 1
                errors.append({**record, "error": "false_positive"})
            elif not says_positive and is_positive:
                matrix["fn"] += 1
                errors.append({**record, "error": "false_negative"})
            else:
                matrix["tn"] += 1
        precision, recall, positive_f1 = f1(matrix["tp"], matrix["fp"], matrix["fn"])
        # The negative class scored symmetrically: a set this imbalanced makes positive-class F1
        # alone a misleading headline, and macro-F1 is the scale-free number the adoption bar uses.
        _, _, negative_f1 = f1(matrix["tn"], matrix["fn"], matrix["fp"])
        result[decision] = {
            "population": len(subset),
            "gold_positive": sum(1 for r in subset if r["gold"] == positive),
            "confusion": matrix,
            "precision": round(precision, 5),
            "recall": round(recall, 5),
            "f1_positive": round(positive_f1, 5),
            "f1_negative": round(negative_f1, 5),
            "macro_f1": round((positive_f1 + negative_f1) / 2, 5),
            "errors": matrix["fp"] + matrix["fn"],
            "error_rows": errors,
        }
    return result


# The arm-A result this baseline pins. A change here is a real event -- either production moved or
# the frozen set did -- and `--check` is what makes it impossible to absorb silently.
PINNED = {
    "declaration_row": {
        "population": 644,
        "confusion": {"tp": 567, "fp": 0, "fn": 22, "tn": 55},
        "macro_f1": 0.90715,
    },
    "caption_admission": {
        "population": 613,
        "confusion": {"tp": 2, "fp": 5, "fn": 4, "tn": 602},
        "macro_f1": 0.65014,
    },
}


def read_frozen(root: str) -> tuple[dict, list[dict]]:
    """The frozen set's rubric header and its adjudicated rows, from the TRACKED file alone."""
    path = os.path.join(root, ADJUDICATION_PATH)
    with open(path, "r", encoding="utf-8") as handle:
        parsed = [json.loads(line) for line in handle if line.strip()]
    if not parsed or parsed[0].get("record_type") != "rubric":
        raise SystemExit(f"error: {ADJUDICATION_PATH} does not open with its rubric header")
    return parsed[0], parsed[1:]


def digest(root: str) -> str:
    path = os.path.join(root, ADJUDICATION_PATH)
    with open(path, "rb") as handle:
        return hashlib.sha256(handle.read()).hexdigest()


def render_screen(screen: dict) -> None:
    print()
    print("== caption stratum, screened for deontic language (a WIDER net than route r1)")
    print(f"   carries deontic vocabulary : {screen['deontic']}")
    print(f"   pure title (no main clause): {screen['pure_title']}")
    print(f"   reporting-verb sentence    : {screen['reporting_sentence']}")
    print(f"   other sentence             : {screen['other_sentence']}")
    for row in screen["other_sentence_rows"]:
        print(f"     {row}")


def render(result: dict, frozen_digest: str) -> None:
    print("=== BOUNDED-DECISION-PROVIDER.1 — arm A on the frozen adjudication set ===")
    print(f"  frozen set: {ADJUDICATION_PATH}")
    print(f"  sha256    : {frozen_digest}")
    for decision, entry in result.items():
        matrix = entry["confusion"]
        print()
        print(f"== {decision}: {entry['population']} rows, {entry['gold_positive']} gold-positive")
        print(
            f"   tp={matrix['tp']} fp={matrix['fp']} fn={matrix['fn']} tn={matrix['tn']}"
            f"   errors={entry['errors']}"
        )
        print(
            f"   precision={entry['precision']:.4f} recall={entry['recall']:.4f}"
            f" f1+={entry['f1_positive']:.4f} f1-={entry['f1_negative']:.4f}"
            f" macro-F1={entry['macro_f1']:.4f}"
        )
        for row in entry["error_rows"]:
            where = (
                f"{row['table_id']} row {row['row_index']}"
                if decision == "declaration_row"
                else row["statement_id"]
            )
            subject = (
                "/".join(row["candidate_names"]) or "<no name>"
                if decision == "declaration_row"
                else row["text"][:96]
            )
            print(f"     {row['error']:15s} {row['document'][:26]:28s} {where:22s} {subject}")


def self_test(root: str) -> int:
    """RED cases: each asserts that one binding in this producer actually catches its own fault."""
    rubric, records = read_frozen(root)
    failures: list[str] = []
    passed = 0

    def check(case: str, condition: bool) -> None:
        nonlocal passed
        if condition:
            passed += 1
        else:
            failures.append(case)

    baseline = score(records)
    check("live-result-matches-pin", all(
        baseline[d]["confusion"] == PINNED[d]["confusion"]
        and baseline[d]["population"] == PINNED[d]["population"]
        and baseline[d]["macro_f1"] == PINNED[d]["macro_f1"]
        for d in PINNED
    ))

    # RED 1 — a gold label flipped must move the score. A scorer that ignored the gold, or that
    # scored the reader against itself, would pass every other case here and fail this one.
    flipped = copy.deepcopy(records)
    for record in flipped:
        if record["decision"] == "declaration_row" and record["gold"] == "not_signal":
            record["gold"] = "signal"
            break
    check("flipped-gold-moves-the-score", score(flipped)["declaration_row"]["confusion"]
          != baseline["declaration_row"]["confusion"])

    # RED 2 — a reader decision flipped must move the score, in the other direction.
    moved = copy.deepcopy(records)
    for record in moved:
        if record["decision"] == "declaration_row" and record["reader"] == "dropped":
            record["reader"] = "emitted"
            break
    check("flipped-reader-moves-the-score", score(moved)["declaration_row"]["confusion"]
          != baseline["declaration_row"]["confusion"])

    # RED 3 — `withheld` must NOT score as a declaration. Treating it as one publishes six false
    # positives the product does not make, which is exactly the mistake the accounting invites.
    as_emitted = copy.deepcopy(records)
    withheld = 0
    for record in as_emitted:
        if record.get("reader") == "withheld":
            record["reader"] = "emitted"
            withheld += 1
    check("withheld-is-not-emitted", withheld == 6
          and baseline["declaration_row"]["confusion"]["fp"] == 0
          and score(as_emitted)["declaration_row"]["confusion"]["fp"] == 6)

    # RED 4 — the caption route must be the real r1 modal list, recomputed here from each record's
    # own text rather than trusted from the file. A truncated list would silently turn false
    # positives into true negatives and flatter the current rule, and reading `reader_route` back
    # would never notice, because `--emit` wrote it with the same truncated list.
    captions = [r for r in records if r["decision"] == "caption_admission"]
    check("modal-route-reproduces-the-reader", bool(captions) and all(
        (record["reader"] == "admitted")
        == any(phrase in record["text"].lower() for phrase in MODAL_PHRASES)
        == (record["reader_route"] == "r1_modal")
        for record in captions
    ))

    # RED 5 — every label resolves to a stated basis in the frozen set's own rubric, so no label
    # is an unexplained assertion and no basis can be deleted without a failure.
    check("every-label-resolves-a-basis", bool(rubric.get("bases")) and all(
        record.get("basis_id") in rubric["bases"] for record in records
    ))

    # RED 6 — the two populations are exactly the frozen sizes, so a silently shortened file fails.
    check("population-sizes-are-exact", all(
        baseline[d]["population"] == PINNED[d]["population"] for d in PINNED
    ))

    # RED 7 — macro-F1 must not collapse to positive-class F1; the imbalance is the whole reason
    # the adoption bar is stated on the macro figure.
    check("macro-f1-is-not-positive-f1", all(
        baseline[d]["macro_f1"] != baseline[d]["f1_positive"] for d in PINNED
    ))

    # RED 9 — the screen must account for every caption exactly once. A net that silently dropped
    # rows would make "the rest state nothing" a claim about a smaller set than the one measured.
    screen = screen_captions(records)
    check("caption-screen-is-exhaustive", sum(
        screen[key] for key in ("deontic", "pure_title", "reporting_sentence", "other_sentence")
    ) == PINNED["caption_admission"]["population"])

    # RED 8 — the two decisions partition the file: a stray record would be scored by neither and
    # would shrink a population without failing any count above.
    check("decisions-partition-the-file", sum(
        1 for r in records if r["decision"] in PINNED
    ) == len(records))

    total = passed + len(failures)
    for case in failures:
        print(f"  RED FAIL: {case}", file=sys.stderr)
    print(f"bounded-decision baseline: {passed}/{total} RED cases pass")
    return 1 if failures else 0


def verify_currency(root: str) -> int:
    """Does the persisted corpus still produce the frozen population, byte for byte?"""
    rendered = render_adjudication(root)
    with open(os.path.join(root, ADJUDICATION_PATH), "r", encoding="utf-8") as handle:
        frozen = handle.read()
    if rendered == frozen:
        print("bounded-decision baseline: the persisted corpus reproduces the frozen set exactly")
        return 0
    rendered_lines = rendered.splitlines()
    frozen_lines = frozen.splitlines()
    print(
        f"bounded-decision baseline: DRIFT — corpus renders {len(rendered_lines)} records against "
        f"{len(frozen_lines)} frozen",
        file=sys.stderr,
    )
    for index, (left, right) in enumerate(zip(rendered_lines, frozen_lines)):
        if left != right:
            print(f"  first difference at record {index}:", file=sys.stderr)
            print(f"    corpus: {left[:300]}", file=sys.stderr)
            print(f"    frozen: {right[:300]}", file=sys.stderr)
            break
    return 1


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="emit the scored result as JSON")
    parser.add_argument("--check", action="store_true", help="verify the digest and pinned result")
    parser.add_argument("--self-test", action="store_true", help="run the RED cases")
    parser.add_argument(
        "--verify-currency",
        action="store_true",
        help="re-derive the population from generated/ and diff it against the frozen set",
    )
    parser.add_argument(
        "--emit", action="store_true", help="regenerate the frozen set from generated/"
    )
    args = parser.parse_args()
    root = repo_root()

    if args.emit:
        path = os.path.join(root, ADJUDICATION_PATH)
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(render_adjudication(root))
        print(f"wrote {ADJUDICATION_PATH} sha256={digest(root)}")
        return 0
    if args.verify_currency:
        return verify_currency(root)
    if args.self_test:
        return self_test(root)

    frozen_digest = digest(root)
    _rubric, records = read_frozen(root)
    result = score(records)
    if args.check:
        problems = []
        if frozen_digest != ADJUDICATION_SHA256:
            problems.append(
                f"frozen set digest {frozen_digest} != pinned {ADJUDICATION_SHA256}"
            )
        for decision, pinned in PINNED.items():
            live = result[decision]
            if live["confusion"] != pinned["confusion"]:
                problems.append(
                    f"{decision} confusion {live['confusion']} != pinned {pinned['confusion']}"
                )
            if live["population"] != pinned["population"]:
                problems.append(
                    f"{decision} population {live['population']} != pinned {pinned['population']}"
                )
            if live["macro_f1"] != pinned["macro_f1"]:
                problems.append(
                    f"{decision} macro-F1 {live['macro_f1']} != pinned {pinned['macro_f1']}"
                )
        for problem in problems:
            print(f"  BASELINE DRIFT: {problem}", file=sys.stderr)
        if problems:
            return 1
        print("bounded-decision baseline: frozen set and arm-A result agree with their pins")
        return 0
    screen = screen_captions(records)
    if args.json:
        json.dump({"arm_a": result, "caption_screen": screen}, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return 0
    render(result, frozen_digest)
    render_screen(screen)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
