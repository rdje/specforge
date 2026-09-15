#!/usr/bin/env python3
"""Census the blast radius of extending `builtin_actor_taxonomy_role_in_text`
(read-only).

`SIGNAL-DECLARATION-ROW-DROP.2b` taught the declaration reader the flow-arrow
notation and measured that **49 of its 83 arrow cells fail closed on an actor
the taxonomy does not know** (`Distributor -> Remote chip`, `Source -> Sink`).
`.2d` owns the question of what to do about them, and its standing instruction
is that extending the taxonomy is NOT a local change: the same function decides
section-heading direction and the actor-signal relation graph, so a new term
moves populations this tree has not measured.

This census measures them. For every candidate term it enumerates, it reports
each site the term would newly reach, split by the **four consumption surfaces**
of `actor_taxonomy_role_in_text` in `crates/specforge/src/ir/evidence.rs`:

  S1 `direction-cell`  (evidence.rs ~9705, via `infer_signal_direction_from_actor_text`)
      the direction/source/destination cell of a signal-description row, both the
      literal actor-text reading and the `.2b` flow-arrow reading. Decides a
      declaration's `direction`.
  S2 `section-heading` (evidence.rs 4314, via `actor_name_and_role_from_section_heading`)
      a section title ending in ` signals`/` inputs`/... Decides the DEFAULT
      direction for every row of every table under it, and mints a `Drives`
      relation for every row of a table with no relation column.
  S3 `relation-actor`  (evidence.rs 4369)
      a source/destination cell's actor name, entered into the per-document
      by-role map.
  S4 `complementary-reader` (evidence.rs 4516, via `unique_complementary_reader_actor_name`)
      the map from S2+S3 mints an extra `Reads` relation only when the OPPOSITE
      role holds EXACTLY ONE name. This surface is **not monotone**: a term that
      takes an opposite-role set from one name to two DESTROYS every complementary
      `Reads` relation that document already had.

Four verdicts per changed site, because the change is not additive. A site whose
answer does not move is not reported at all:

  `gain`     the site has no direction/role today and would have one.
  `loss`     it has one today and would have none — the candidate's OPPOSITE role
             also matches the text, so `(requester_like, completer_like)` becomes
             `(true, true)` and the function returns `None`.
  `flip`     it keeps an answer, but a different one.
  `restage`  (S1 only) the same direction, decided by an EARLIER reading. Not
             cosmetic: a direction that moves from the flow-arrow reading to the
             literal actor-text reading has stopped consulting the arrow, and the
             next row of the same link will not agree with it.

Candidate terms are DISCOVERED, never listed: they are the normalized actor
sides of the arrow cells that currently fail closed, so this file carries no
document, vendor or protocol vocabulary of its own (ADR 0006). `--term
<text>=<requester|completer>` adds a hypothesis by hand for sizing.

It also tests the alternative `.2d` names — *read the flow relative to the
table's own subject, without a taxonomy at all* — in its one mechanical form:
the actor that appears on one side of EVERY arrow cell in the table. The
agreement of that rule with the 18 cells the taxonomy already admits is the
falsification oracle; disagreement means the rule is a competing authority, not
a fallback.

This census classifies the POPULATION. It is not a check on the Rust
implementation and must never be cited as one: it shares its understanding of the
actor-role taxonomy with the code it would be checking, so their agreement would
carry no information (`CLAIM_VERIFICATION.md` section 2). The independent leg is
the in-crate control suite, which runs the real reader over these cell forms.

Boundary and over-approximation, stated: persisted
`generated/source_ir/*/source_ir.json`, tables whose persisted `table_kind` is
`signal_description` -- the boundary `.0` and the notation census use. The Rust
producers gate further (`should_treat_table_as_top_level_signal_description`,
`normalize_relation_actor_name`), so every count here is an UPPER BOUND on the
sites that would actually move.

Read-only and deterministic: no network, no clock, no randomness, no write, no
rebuild.

Usage:
    python3 scripts/measure_actor_taxonomy_blast_radius.py
    python3 scripts/measure_actor_taxonomy_blast_radius.py --json
    python3 scripts/measure_actor_taxonomy_blast_radius.py --term 'sink=completer'
    python3 scripts/measure_actor_taxonomy_blast_radius.py \
        --vocabulary 'source=requester,sink=completer'
"""
import argparse
import collections
import glob
import json
import os
import sys

# Verbatim from FLOW_ARROW_FORMS / FLOW_ARROW_DISQUALIFIERS in evidence.rs.
RIGHT_ARROWS = ["⟶", "⇒", "→", "==>", "-->", "=>", "->"]
DISQUALIFYING_MARKERS = ["⟵", "⇐", "←", "↔", "⇔", "<-", "<="]

# Verbatim from builtin_actor_taxonomy_role_in_text.
BUILTIN_REQUESTER = ["manager", "initiator", "master", "requester"]
BUILTIN_COMPLETER = [
    "subordinate", "slave", "responder", "multiplexor", "completer", "target",
]

# Verbatim from actor_name_and_role_from_section_heading.
HEADING_SUFFIXES = [
    " signals", " signal", " inputs", " input", " outputs", " output",
]

# Header-keyword column detection, verbatim from synthesize_signal_declarations.
DIRECTION_HEADER_TERMS = ["direction"]
SOURCE_HEADER_TERMS = ["source", "driver"]
DEST_HEADER_TERMS = ["destination", "dest"]

# `is_meaningful_actor_term`, verbatim from prior_memory.rs.
NON_ACTOR_TERMS = {
    "input", "output", "inout", "bidirectional", "bidir", "external", "tie off",
    "tieoff", "reserved", "n a", "na", "none", "tbd", "see note", "information",
    "control information", "status information", "data", "payload",
    "data bytes", "control bytes", "byte lanes", "transfer", "transaction",
    "mixture", "mixture of",
}


def normalize_actor_term(text):
    """`normalize_actor_term` from crates/specforge/src/ir/prior_memory.rs."""
    folded = "".join(
        ch if (ch.isascii() and (ch.isalnum() or ch == "_")) else " "
        for ch in text.lower()
    )
    return " ".join(folded.split())


def normalized_text_contains_term(text, term):
    """Whole-token window match, as `normalized_text_contains_term` does."""
    text_tokens = text.split()
    term_tokens = term.split()
    if not text_tokens or not term_tokens or len(term_tokens) > len(text_tokens):
        return False
    return any(
        text_tokens[i:i + len(term_tokens)] == term_tokens
        for i in range(len(text_tokens) - len(term_tokens) + 1)
    )


def taxonomy_role(text, requester_terms, completer_terms):
    """`builtin_actor_taxonomy_role_in_text`, over a supplied term vocabulary."""
    normalized = normalize_actor_term(text)
    if not normalized:
        return None
    requester = any(
        normalized_text_contains_term(normalized, t) for t in requester_terms
    )
    completer = any(
        normalized_text_contains_term(normalized, t) for t in completer_terms
    )
    if requester and not completer:
        return "requester"
    if completer and not requester:
        return "completer"
    return None


def port_sense(actor_text, column_kind, requester_terms, completer_terms):
    """`infer_signal_direction_from_actor_text`, over a supplied vocabulary."""
    lowered = actor_text.lower()
    if normalize_actor_term(actor_text) in ("tie off", "tieoff"):
        return "input"
    if "output" in lowered:
        return "output"
    if "input" in lowered:
        return "input"
    role = taxonomy_role(actor_text, requester_terms, completer_terms)
    if role is None:
        return None
    return {
        ("source", "requester"): "output",
        ("source", "completer"): "input",
        ("destination", "requester"): "input",
        ("destination", "completer"): "output",
    }[(column_kind, role)]


def split_on_single_flow_arrow(text):
    """Return ((left, right), 1) for exactly one right-flow arrow, else (None, n)."""
    hits = []
    index = 0
    while index < len(text):
        for arrow in RIGHT_ARROWS:
            if text.startswith(arrow, index):
                hits.append((index, arrow))
                index += len(arrow)
                break
        else:
            index += 1
    if len(hits) != 1:
        return None, len(hits)
    at, arrow = hits[0]
    return (text[:at], text[at + len(arrow):]), 1


def arrow_sides(text):
    """The two operands of a well-formed single-arrow cell, else None."""
    if any(marker in text for marker in DISQUALIFYING_MARKERS):
        return None
    parts, count = split_on_single_flow_arrow(text)
    if count != 1 or parts is None:
        return None
    return parts


def arrow_verdict(text, requester_terms, completer_terms):
    """`infer_signal_direction_from_flow_arrow`; None when the cell has no arrow."""
    if any(marker in text for marker in DISQUALIFYING_MARKERS):
        return "closed:bidirectional_or_reverse_marker", None
    parts, count = split_on_single_flow_arrow(text)
    if count == 0:
        return None, None
    if parts is None:
        return "closed:multiple_arrows", None
    left, right = parts
    from_source = port_sense(left, "source", requester_terms, completer_terms)
    from_destination = port_sense(
        right, "destination", requester_terms, completer_terms
    )
    if from_source is None or from_destination is None:
        return "closed:unresolved_actor", None
    if from_source != from_destination:
        return "closed:contradictory", None
    return "admitted", from_source


def is_meaningful_actor_term(text):
    normalized = normalize_actor_term(text)
    return bool(normalized) and normalized not in NON_ACTOR_TERMS


def header_texts(table):
    header_rows = table.get("header_rows") or []
    if not header_rows:
        return []
    return [cell["text"].lower() for cell in header_rows[0]]


def first_header_matching(headers, terms):
    for index, header in enumerate(headers):
        if any(term in header for term in terms):
            return index
    return None


def page_number(page_id):
    digits = "".join(ch for ch in (page_id or "") if ch.isdigit())
    return int(digits) if digits else -1


def heading_actor_text(title):
    """The text `actor_name_and_role_from_section_heading` hands the taxonomy."""
    lowered = title.lower()
    for suffix in HEADING_SUFFIXES:
        if lowered.endswith(suffix):
            trimmed = title[: len(title) - len(suffix)].strip()
            return trimmed or None
    return None


def load_documents(root):
    documents = []
    pattern = os.path.join(root, "generated/source_ir/*/source_ir.json")
    for path in sorted(glob.glob(pattern)):
        with open(path, encoding="utf-8") as handle:
            documents.append((os.path.basename(os.path.dirname(path)), json.load(handle)))
    return documents


def signal_tables(source_ir):
    return [
        table
        for table in source_ir.get("structured_tables", [])
        if table.get("table_kind") == "signal_description"
    ]


def direction_columns(headers):
    """(explicit_dir_col, source_col, dest_col), as synthesize_signal_declarations picks them."""
    return (
        first_header_matching(headers, DIRECTION_HEADER_TERMS),
        first_header_matching(headers, SOURCE_HEADER_TERMS),
        first_header_matching(headers, DEST_HEADER_TERMS),
    )


SECTION_KINDS_WITHOUT_DIRECTION = {"boilerplate", "glossary", "table_of_contents"}


def nearest_section(source_ir):
    """`page_to_section.range(..=table_page).next_back()`, page-keyed as in evidence.rs."""
    by_page = {}
    for section in source_ir.get("document_sections", []):
        page = page_number(section.get("page_id"))
        if page >= 0:
            by_page[page] = (section.get("section_kind"), section.get("title", ""))
    pages = sorted(by_page)

    def lookup(table_page):
        best = None
        for page in pages:
            if page <= table_page:
                best = by_page[page]
            else:
                break
        return best or (None, "")

    return lookup


def section_default_direction(section_kind, section_title, requester_terms, completer_terms):
    """`infer_signal_direction_from_section`."""
    if section_kind in SECTION_KINDS_WITHOUT_DIRECTION:
        return None
    actor_text = heading_actor_text(section_title)
    if actor_text is None:
        return None
    role = taxonomy_role(actor_text, requester_terms, completer_terms)
    if role is None:
        return None
    return {"requester": "output", "completer": "input"}[role]


def row_direction(row, dir_col, source_col, dest_col, section_default,
                  requester_terms, completer_terms):
    """`synthesize_signal_declarations`' direction chain, verbatim in its priority order.

    Returns `(direction, stage)`, so a change can be attributed to the stage that
    produced it. The stage matters more than the direction: the literal actor-text
    reading runs BEFORE the flow-arrow reading and is handed the WHOLE cell, so a
    term that matches the cell's text answers without the arrow ever being consulted.
    """
    if dir_col is not None and dir_col < len(row):
        text = row[dir_col]["text"].lower()
        if "output" in text:
            return "output", "explicit_literal"
        if "input" in text:
            return "input", "explicit_literal"
    for column, kind in ((source_col, "source"), (dest_col, "destination")):
        if column is None or column >= len(row):
            continue
        sense = port_sense(row[column]["text"], kind, requester_terms, completer_terms)
        if sense:
            return sense, f"actor_text:{kind}"
    seen_columns = []
    for column in (dir_col, source_col, dest_col):
        if column is None or column >= len(row) or column in seen_columns:
            continue
        seen_columns.append(column)
        verdict, sense = arrow_verdict(
            row[column]["text"].strip(), requester_terms, completer_terms
        )
        if verdict == "admitted":
            return sense, "flow_arrow"
    # The description-prose fallback reads no actor vocabulary; it cannot move.
    if section_default:
        return section_default, "section_default"
    return None, "none"


def collect_sites(documents):
    """Every site whose answer can move, tagged with its consumption surface.

    S1 is modelled as one site per ROW, evaluated through the whole direction
    chain, so a cell that two readings both touch is counted once and the stage
    that actually answers is recorded. S2 and S3 are one site per text.
    """
    sites = []
    for document, source_ir in documents:
        lookup = nearest_section(source_ir)
        for table in signal_tables(source_ir):
            headers = header_texts(table)
            dir_col, source_col, dest_col = direction_columns(headers)
            section_kind, section_title = lookup(page_number(table.get("page_id")))
            for row_index, row in enumerate(table.get("body_rows", [])):
                sites.append({
                    "document": document,
                    "surface": "S1:direction-cell",
                    "table_id": table.get("table_id"),
                    "row": row_index,
                    "section_kind": section_kind,
                    "section_title": section_title,
                    "columns": (dir_col, source_col, dest_col),
                    "cells": [
                        row[column]["text"].strip() if column is not None
                        and column < len(row) else None
                        for column in (dir_col, source_col, dest_col)
                    ],
                    "row_cells": row,
                })
            relation_cols = []
            for column in (source_col, dest_col):
                if column is not None and column not in relation_cols:
                    relation_cols.append(column)
            for row_index, row in enumerate(table.get("body_rows", [])):
                for column in relation_cols:
                    if column >= len(row):
                        continue
                    text = row[column]["text"].strip()
                    if not is_meaningful_actor_term(text):
                        continue
                    sites.append({
                        "document": document,
                        "surface": "S3:relation-actor",
                        "table_id": table.get("table_id"),
                        "row": row_index,
                        "text": text,
                    })
        for section in source_ir.get("document_sections", []):
            actor_text = heading_actor_text(section.get("title", ""))
            if actor_text is None:
                continue
            sites.append({
                "document": document,
                "surface": "S2:section-heading",
                "text": actor_text,
                "title": section.get("title", ""),
                "section_kind": section.get("section_kind"),
                "page": page_number(section.get("page_id")),
            })
    return sites


def discover_candidates(documents):
    """The normalized actor sides of arrow cells that fail closed today.

    Discovered from the corpus, never listed here: a censuses's candidate set
    must not be a vocabulary this file chose (ADR 0006).
    """
    candidates = collections.Counter()
    by_document = collections.defaultdict(set)
    examples = collections.defaultdict(collections.Counter)
    for document, source_ir in documents:
        for table in signal_tables(source_ir):
            headers = header_texts(table)
            dir_col, source_col, dest_col = direction_columns(headers)
            for row in table.get("body_rows", []):
                for column in (dir_col, source_col, dest_col):
                    if column is None or column >= len(row):
                        continue
                    text = row[column]["text"].strip()
                    verdict, _ = arrow_verdict(text, BUILTIN_REQUESTER, BUILTIN_COMPLETER)
                    if verdict is None:
                        continue
                    if verdict != "closed:unresolved_actor":
                        break
                    left, right = arrow_sides(text)
                    for side, kind in ((left, "source"), (right, "destination")):
                        if port_sense(side, kind, BUILTIN_REQUESTER, BUILTIN_COMPLETER):
                            continue
                        term = normalize_actor_term(side)
                        if not term:
                            continue
                        candidates[term] += 1
                        by_document[term].add(document)
                        examples[term][(document, text)] += 1
                    break
    return candidates, by_document, examples


def extended_vocabulary(vocabulary):
    """The builtin lists plus a hypothesis, as `builtin_actor_taxonomy_role_in_text` would hold them."""
    requester = list(BUILTIN_REQUESTER)
    completer = list(BUILTIN_COMPLETER)
    for term, role in vocabulary:
        (requester if role == "requester" else completer).append(term)
    return requester, completer


def classify(sites, vocabulary):
    """Per-site verdict for adopting `vocabulary`: gain / loss / flip / restage."""
    requester, completer = extended_vocabulary(vocabulary)

    verdicts = collections.Counter()
    changed = []
    for site in sites:
        if site["surface"] == "S1:direction-cell":
            dir_col, source_col, dest_col = site["columns"]
            row = site["row_cells"]
            before_default = section_default_direction(
                site["section_kind"], site["section_title"],
                BUILTIN_REQUESTER, BUILTIN_COMPLETER,
            )
            after_default = section_default_direction(
                site["section_kind"], site["section_title"], requester, completer
            )
            before, before_stage = row_direction(
                row, dir_col, source_col, dest_col, before_default,
                BUILTIN_REQUESTER, BUILTIN_COMPLETER,
            )
            after, after_stage = row_direction(
                row, dir_col, source_col, dest_col, after_default,
                requester, completer,
            )
            if (before, before_stage) == (after, after_stage):
                continue
            kind = (
                "gain" if before is None
                else "loss" if after is None
                else "flip" if before != after
                else "restage"
            )
            verdicts[(site["surface"], kind)] += 1
            changed.append({
                "document": site["document"],
                "surface": site["surface"],
                "table_id": site["table_id"],
                "row": site["row"],
                "text": " | ".join(cell or "" for cell in site["cells"]),
                "before": f"{before}@{before_stage}",
                "after": f"{after}@{after_stage}",
            })
            continue
        before = taxonomy_role(site["text"], BUILTIN_REQUESTER, BUILTIN_COMPLETER)
        after = taxonomy_role(site["text"], requester, completer)
        if before == after:
            continue
        kind = "gain" if before is None else "loss" if after is None else "flip"
        verdicts[(site["surface"], kind)] += 1
        changed.append({
            "document": site["document"],
            "surface": site["surface"],
            "text": site.get("title", site["text"]),
            "before": str(before),
            "after": str(after),
        })
    return verdicts, changed


def by_role_map(documents, requester_terms, completer_terms):
    """`collect_local_actor_names_by_taxonomy_role`, per document."""
    maps = {}
    for document, source_ir in documents:
        roles = {"requester": set(), "completer": set()}
        for table in signal_tables(source_ir):
            headers = header_texts(table)
            relation_cols = [
                index
                for index, header in enumerate(headers)
                if any(
                    term in header
                    for term in SOURCE_HEADER_TERMS + DEST_HEADER_TERMS
                )
            ]
            for row in table.get("body_rows", []):
                for column in relation_cols:
                    if column >= len(row):
                        continue
                    text = row[column]["text"].strip()
                    if not is_meaningful_actor_term(text):
                        continue
                    role = taxonomy_role(text, requester_terms, completer_terms)
                    if role:
                        roles[role].add(normalize_actor_term(text))
        for section in source_ir.get("document_sections", []):
            actor_text = heading_actor_text(section.get("title", ""))
            if actor_text is None or not is_meaningful_actor_term(actor_text):
                continue
            role = taxonomy_role(actor_text, requester_terms, completer_terms)
            if role:
                roles[role].add(normalize_actor_term(actor_text))
        maps[document] = roles
    return maps


OPPOSITE = {"requester": "completer", "completer": "requester"}


def complementary_reader_effect(documents, vocabulary):
    """S4: how the exactly-one-opposite-name condition moves, per document.

    `unique_complementary_reader_actor_name` mints a `Reads` relation only when
    the opposite role holds exactly one name. Adding a term can take that set
    from one to two and DESTROY every complementary relation the document had.
    """
    requester, completer = extended_vocabulary(vocabulary)
    before = by_role_map(documents, BUILTIN_REQUESTER, BUILTIN_COMPLETER)
    after = by_role_map(documents, requester, completer)
    effects = []
    for document in before:
        for side in ("requester", "completer"):
            was, now = before[document][side], after[document][side]
            if was == now:
                continue
            # A `Drives` relation whose actor is in OPPOSITE[side] reads this set.
            was_unique, now_unique = len(was) == 1, len(now) == 1
            if was_unique and not now_unique:
                effect = "complementary_reads_destroyed"
            elif not was_unique and now_unique:
                effect = "complementary_reads_created"
            else:
                effect = "set_membership_only"
            effects.append({
                "document": document,
                "role_set": side,
                "drives_side": OPPOSITE[side],
                "before": sorted(was),
                "after": sorted(now),
                "effect": effect,
            })
    return effects


def flow_sense_collapses(rows):
    """Opposite flows between one actor pair that the chain gives the SAME direction.

    Vocabulary-free, and therefore the only non-circular oracle available here:
    whatever `A -> B` means, `B -> A` must mean the opposite. A reading that
    returns one direction for both has contradicted the document without any
    need to know which of the two is correct. This is what a PARTIAL vocabulary
    buys -- one term of a pair makes the literal actor-text reading match the
    WHOLE cell, so it answers before the arrow and answers the same both ways.
    """
    by_pair = collections.defaultdict(list)
    for row in rows:
        if row["left"] is None or row["after"] is None:
            continue
        by_pair[(row["document"], row["table_id"],
                 frozenset((row["left"], row["right"])))].append(row)
    collapses = []
    for (document, table_id, pair), members in sorted(
        by_pair.items(), key=lambda kv: (kv[0][0], kv[0][1], sorted(kv[0][2]))
    ):
        senses = {(row["left"], row["right"]): row["after"] for row in members}
        for (left, right), sense in sorted(senses.items()):
            mirror = senses.get((right, left))
            if mirror is not None and mirror == sense and left < right:
                collapses.append({
                    "document": document,
                    "table_id": table_id,
                    "pair": sorted(pair),
                    "direction": sense,
                    "rows": sum(
                        1 for row in members
                        if {row["left"], row["right"]} == set(pair)
                    ),
                    "stage": members[0]["stage"],
                })
    return collapses


def vocabulary_effect(documents, vocabulary):
    """Adopt a whole term SET at once, and ask what the arrow cells actually get.

    A single term is not how a taxonomy grows: closing `Distributor -> Remote chip`
    needs BOTH names. Evaluated together, the interesting number is not how many
    rows gain a direction but how many gain the direction the FLOW states — so
    this compares the chain's answer against the flow-arrow reader's own answer
    under the same vocabulary. They differ whenever an earlier stage answered
    first, which is the whole point: `infer_signal_direction_from_actor_text`
    is handed the WHOLE cell, and a cell naming both endpoints of a flow matches
    a term on either side, so both senses of the same bus collapse to one
    direction without the arrow ever being consulted.
    """
    requester, completer = extended_vocabulary(vocabulary)

    rows = []
    for document, source_ir in documents:
        for table in signal_tables(source_ir):
            headers = header_texts(table)
            dir_col, source_col, dest_col = direction_columns(headers)
            for row_index, row in enumerate(table.get("body_rows", [])):
                cell = None
                for column in (dir_col, source_col, dest_col):
                    if column is None or column >= len(row):
                        continue
                    text = row[column]["text"].strip()
                    if any(marker in text for marker in RIGHT_ARROWS):
                        cell = text
                        break
                if cell is None:
                    continue
                before, before_stage = row_direction(
                    row, dir_col, source_col, dest_col, None,
                    BUILTIN_REQUESTER, BUILTIN_COMPLETER,
                )
                after, after_stage = row_direction(
                    row, dir_col, source_col, dest_col, None, requester, completer
                )
                arrow, arrow_sense = arrow_verdict(cell, requester, completer)
                sides = arrow_sides(cell)
                rows.append({
                    "document": document,
                    "table_id": table.get("table_id"),
                    "row": row_index,
                    "cell": cell,
                    "left": normalize_actor_term(sides[0]) if sides else None,
                    "right": normalize_actor_term(sides[1]) if sides else None,
                    "before": before,
                    "after": after,
                    "stage": after_stage,
                    "arrow_verdict": arrow,
                    "arrow_says": arrow_sense,
                    "agrees_with_flow": arrow == "admitted" and after == arrow_sense,
                })
    return rows


def table_subject_rule(documents):
    """`.2d`'s taxonomy-free alternative, in its one mechanical form.

    The table's subject is the actor that appears on one side of EVERY arrow
    cell in the table. With a subject, `subject -> other` is an output and
    `other -> subject` an input, and no vocabulary is needed. Reported against
    the cells the taxonomy already admits, which is the falsification oracle:
    the rule is a fallback only if it AGREES there.
    """
    results = []
    for document, source_ir in documents:
        for table in signal_tables(source_ir):
            headers = header_texts(table)
            dir_col, source_col, dest_col = direction_columns(headers)
            cells = []
            for row in table.get("body_rows", []):
                for column in (dir_col, source_col, dest_col):
                    if column is None or column >= len(row):
                        continue
                    text = row[column]["text"].strip()
                    sides = arrow_sides(text)
                    if sides is None:
                        if any(a in text for a in RIGHT_ARROWS):
                            break
                        continue
                    cells.append((text, normalize_actor_term(sides[0]),
                                  normalize_actor_term(sides[1])))
                    break
            if not cells:
                continue
            common = None
            for _, left, right in cells:
                names = {left, right}
                common = names if common is None else (common & names)
            common = common or set()
            subject = next(iter(common)) if len(common) == 1 else None
            agreements = disagreements = unreadable = 0
            for text, left, right in cells:
                taxonomy, sense = arrow_verdict(text, BUILTIN_REQUESTER, BUILTIN_COMPLETER)
                if taxonomy != "admitted":
                    continue
                if subject is None:
                    unreadable += 1
                    continue
                subject_sense = "output" if subject == left else "input"
                if subject_sense == sense:
                    agreements += 1
                else:
                    disagreements += 1
            results.append({
                "document": document,
                "table_id": table.get("table_id"),
                "arrow_cells": len(cells),
                "common_actors": sorted(common),
                "subject": subject,
                "taxonomy_admitted": sum(
                    1 for text, _, _ in cells
                    if arrow_verdict(text, BUILTIN_REQUESTER, BUILTIN_COMPLETER)[0]
                    == "admitted"
                ),
                "agrees": agreements,
                "disagrees": disagreements,
                "no_subject": unreadable,
            })
    return results


def parse_vocabulary(spec, flag, parser):
    """`TEXT=ROLE[,TEXT=ROLE...]` into the `(term, role)` pairs the sizers take."""
    vocabulary = []
    for member in spec.split(","):
        term, _, role = member.partition("=")
        term = normalize_actor_term(term)
        if role not in ("requester", "completer") or not term:
            parser.error(f"{flag} expects TEXT=requester|completer, got {member!r}")
        vocabulary.append((term, role))
    return vocabulary


def build_report(documents, sites, hypotheses, vocabulary):
    """The whole census as one structure; `--json` dumps it, the printer walks it."""
    candidates, by_document, examples = discover_candidates(documents)
    report = {
        "documents": len(documents),
        "sites_by_surface": dict(
            collections.Counter(site["surface"] for site in sites)
        ),
        "candidates": [
            {
                "term": term,
                "fail_closed_arrow_sides": count,
                "documents": sorted(by_document[term]),
                "cells": [
                    {"document": doc, "cell": cell, "count": n}
                    for (doc, cell), n in sorted(
                        examples[term].items(), key=lambda kv: (-kv[1], kv[0])
                    )
                ],
            }
            for term, count in sorted(
                candidates.items(), key=lambda kv: (-kv[1], kv[0])
            )
        ],
        "blast_radius": [],
        "table_subject_rule": table_subject_rule(documents),
        "vocabulary": None,
    }

    if vocabulary:
        rows = vocabulary_effect(documents, vocabulary)
        verdicts, changed = classify(sites, vocabulary)
        report["vocabulary"] = {
            "sites_changed": sum(verdicts.values()),
            "by_surface": [
                {"surface": surface, "verdict": verdict, "sites": n}
                for (surface, verdict), n in sorted(verdicts.items())
            ],
            "complementary_reader": complementary_reader_effect(documents, vocabulary),
            "changed_sites": changed,
            "terms": [{"term": t, "role": r} for t, r in vocabulary],
            "arrow_rows": len(rows),
            "answered_before_the_arrow": sum(
                1 for row in rows if not row["stage"].startswith("flow_arrow")
                and row["after"] is not None
            ),
            "agrees_with_flow": sum(1 for row in rows if row["agrees_with_flow"]),
            "contradicts_flow": sum(
                1 for row in rows
                if row["arrow_verdict"] == "admitted"
                and row["after"] is not None
                and row["after"] != row["arrow_says"]
            ),
            "still_undirected": sum(1 for row in rows if row["after"] is None),
            "flow_sense_collapses": flow_sense_collapses(rows),
            "rows": rows,
        }

    seen = set()
    for term, role, discovered in hypotheses:
        if (term, role) in seen:
            continue
        seen.add((term, role))
        verdicts, changed = classify(sites, [(term, role)])
        report["blast_radius"].append({
            "term": term,
            "role": role,
            "fail_closed_arrow_sides": discovered,
            "sites_changed": sum(verdicts.values()),
            "by_surface": [
                {"surface": surface, "verdict": verdict, "sites": n}
                for (surface, verdict), n in sorted(verdicts.items())
            ],
            "complementary_reader": complementary_reader_effect(
                documents, [(term, role)]
            ),
            "changed_sites": changed,
        })
    return report


def print_candidates(report):
    print()
    print(f"--- candidate terms discovered from fail-closed arrow sides: "
          f"{len(report['candidates'])}")
    for candidate in report["candidates"]:
        print(f"  {candidate['fail_closed_arrow_sides']:4d}  {candidate['term']!r} "
              f"({len(candidate['documents'])} document(s))")
        for cell in candidate["cells"]:
            print(f"        {cell['count']:3d}  {cell['document'][:34]:36s} "
                  f"{cell['cell']!r}")


def print_blast_radius(report):
    print()
    print("--- blast radius per (term, role)")
    for entry in report["blast_radius"]:
        print(f"  == {entry['term']!r} as {entry['role']}: "
              f"{entry['sites_changed']} site(s) change")
        for row in entry["by_surface"]:
            print(f"       {row['sites']:5d}  {row['verdict']:6s} {row['surface']}")
        for effect in entry["complementary_reader"]:
            print(f"       S4 {effect['effect']:32s} {effect['document'][:34]:36s} "
                  f"{effect['role_set']} {effect['before']} -> {effect['after']}")
        forms = collections.Counter(
            (site["surface"], site["text"], site["before"], site["after"])
            for site in entry["changed_sites"]
        )
        for (surface, text, before, after), count in sorted(
            forms.items(), key=lambda kv: (-kv[1], kv[0])
        ):
            print(f"       {count:5d}  {surface:20s} {before} -> {after}  {text!r}")


def print_vocabulary(vocabulary):
    print()
    terms = ", ".join(
        f"{entry['term']}={entry['role']}" for entry in vocabulary["terms"]
    )
    print(f"--- whole-vocabulary adoption: {terms}")
    print(f"    sites changed (S1/S2/S3)    : {vocabulary['sites_changed']}")
    for row in vocabulary["by_surface"]:
        print(f"       {row['sites']:5d}  {row['verdict']:8s} {row['surface']}")
    for effect in vocabulary["complementary_reader"]:
        print(f"       S4 {effect['effect']:32s} {effect['document'][:34]:36s} "
              f"{effect['role_set']} {effect['before']} -> {effect['after']}")
    print(f"    arrow-bearing rows          : {vocabulary['arrow_rows']}")
    print(f"    answered BEFORE the arrow   : "
          f"{vocabulary['answered_before_the_arrow']}")
    print(f"    agree with the stated flow  : {vocabulary['agrees_with_flow']}")
    print(f"    CONTRADICT the stated flow  : {vocabulary['contradicts_flow']}")
    print(f"    still undirected            : {vocabulary['still_undirected']}")
    collapses = vocabulary["flow_sense_collapses"]
    print(f"    FLOW-SENSE COLLAPSES        : {len(collapses)} actor pair(s), "
          f"{sum(entry['rows'] for entry in collapses)} row(s)")
    for entry in collapses:
        print(f"      {entry['rows']:4d}  {entry['document'][:34]:36s} "
              f"{entry['table_id']:12s} {entry['pair']} both {entry['direction']} "
              f"@{entry['stage']}")
    forms = collections.Counter(
        (row["document"][:34], row["cell"], str(row["after"]), row["stage"],
         str(row["arrow_verdict"]), str(row["arrow_says"]))
        for row in vocabulary["rows"]
    )
    for key, count in sorted(forms.items(), key=lambda kv: (-kv[1], kv[0])):
        document, cell, after, stage, verdict, says = key
        print(f"      {count:4d}  {document:36s} {cell!r}")
        print(f"            chain -> {after}@{stage}   arrow -> {verdict}/{says}")


def print_table_subject_rule(report):
    print()
    print("--- taxonomy-free alternative: the table's own subject")
    print("    (the actor on one side of EVERY arrow cell in the table)")
    for row in report["table_subject_rule"]:
        print(f"  {row['document'][:34]:36s} {row['table_id']:12s} "
              f"cells={row['arrow_cells']:3d} admitted={row['taxonomy_admitted']:3d} "
              f"subject={row['subject']!r} common={row['common_actors']}")
        if row["taxonomy_admitted"]:
            print(f"      vs taxonomy: agrees={row['agrees']} "
                  f"disagrees={row['disagrees']} no_subject={row['no_subject']}")


def print_report(report):
    print("=== actor-taxonomy blast-radius census (read-only, persisted SourceIR) ===")
    print(f"  documents                 : {report['documents']}")
    for surface, count in sorted(report["sites_by_surface"].items()):
        print(f"  sites {surface:24s}: {count}")
    print_candidates(report)
    print_blast_radius(report)
    if report["vocabulary"]:
        print_vocabulary(report["vocabulary"])
    print_table_subject_rule(report)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="machine-readable report")
    parser.add_argument(
        "--vocabulary",
        metavar="TEXT=ROLE[,TEXT=ROLE...]",
        help="adopt a whole term set at once and report what the arrow cells get",
    )
    parser.add_argument(
        "--term",
        action="append",
        default=[],
        metavar="TEXT=ROLE",
        help="size an extra hypothesis, e.g. --term 'sink=completer'",
    )
    args = parser.parse_args()

    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    documents = load_documents(root)
    sites = collect_sites(documents)
    candidates, _, _ = discover_candidates(documents)

    hypotheses = [
        (term, role, count)
        for term, count in sorted(candidates.items(), key=lambda kv: (-kv[1], kv[0]))
        for role in ("requester", "completer")
    ]
    for spec in args.term:
        for term, role in parse_vocabulary(spec, "--term", parser):
            hypotheses.append((term, role, candidates.get(term, 0)))

    vocabulary = (
        parse_vocabulary(args.vocabulary, "--vocabulary", parser)
        if args.vocabulary else None
    )
    report = build_report(documents, sites, hypotheses, vocabulary)

    if args.json:
        json.dump(report, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return 0

    print_report(report)
    return 0


if __name__ == "__main__":
    sys.exit(main())
