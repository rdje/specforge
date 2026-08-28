#!/usr/bin/env python3
"""Adjudicate where a persisted SourceIR content element went when a re-ingest no longer emits it.

`SOURCE-IR-REPRODUCIBILITY.5`. The `.1` census reported that a current re-ingest drops 31 persisted
content elements across the live corpus, of which 28 are re-segmentation and **three** are text that
"the current toolchain emits nowhere". That verdict came from a whole-string containment test against
the joined replayed element stream, which by construction cannot see a paragraph the converter split
in two. It is an upper bound on loss, not a measurement of it.

This producer replaces that bound with a decision. For every persisted element whose text the census
calls absent, it aligns the text token by token against the converter's OWN document for the same run
and answers the only question with different remedies:

  retained            every persisted word is still there, in order — however differently the
                      converter split the paragraph, and however much it spliced in between
  specforge_dropped   the converter emitted it and SpecForge's element construction discarded it
  partially_absent    some persisted words align and some have no match at all
  docling_absent      no persisted word aligns; an upstream boundary to declare rather than assume

Because the same document is already loaded, it also censuses the standing conservation gap at that
boundary: every converter text item that reaches no `SourceIR` record and earns no residual. That gap
is present in the persisted artifacts too — it is not drift — and it is the population
`SOURCE-IR-REPRODUCIBILITY.7` must gate.

`SOURCE-IR-REPRODUCIBILITY.11` adds `--oracle`, which stops the largest of those buckets from
resting on a reading of upstream source. The drop model reimplements two docling-core predicates;
the oracle asks docling-core itself which items `iterate_items` yields, on the same serialized
document the artifact was built from, and reports every disagreement in both directions.

READ-ONLY with respect to the tracked tree and `generated/`: it re-ingests into a fresh
`.project-data/tmp` root and writes only there.
"""

from __future__ import annotations

import argparse
import difflib
import json
import os
from pathlib import Path
import shlex
import subprocess
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent))

# The `.1` census owns the frame, the staging protocol, the root normalization, and the drift shape.
# Importing it rather than restating it keeps this producer in lockstep: if the census changes what
# "absent" means, this adjudication changes with it instead of becoming a second opinion.
import measure_source_ir_reproducibility as census  # noqa: E402


ROOT = census.ROOT

# The documents `.1` reported as losing content the current toolchain emits nowhere. Declared here
# rather than rediscovered, so the sample is stated; `--document` overrides it. Every element this
# producer adjudicates is re-derived from the artifacts, never read back from that census.
DECLARED_DOCUMENTS = (
    "usb4_connection_manager_guide_v2_0_2025_11",
    "usb_3_2_revision_1_0_2017_09",
    "wbspec_b4_wishbone_b4_specification",
)

# `DoclingDocument.iterate_items` yields an item only when its content layer is in
# `DEFAULT_CONTENT_LAYERS`, which is `{ContentLayer.BODY}`.
BODY_LAYER = "body"

# Token width of the anchor used to find where a persisted text sits in the converter's stream.
# Long enough that a run of ordinary prose words is not a coincidence, short enough that a two-line
# paragraph still anchors.
ANCHOR_GRAM = 6

# How far around the anchor the alignment may look, as a multiple of the persisted text's length.
# An unbounded search would let scattered tokens anywhere in the document "recover" a paragraph
# that is genuinely gone, which is the one answer this producer must never give by accident.
ALIGNMENT_WINDOW = 2

# How many candidate anchor positions one alignment may try. Exceeding it is recorded rather than
# absorbed, so a search that ran out of budget never reads as proof that the text is gone.
ANCHOR_CANDIDATES = 64


def normalize(text: object) -> str:
    """Collapse whitespace exactly as a comparison across two layout runs must.

    The backend helper's own `normalize_text` only strips, so collapsing here is a superset that
    cannot manufacture a match the helper would have rejected. Nothing else is normalized: case
    folding or character substitution would start hiding real differences.
    """
    if text is None:
        return ""
    return " ".join(str(text).split())


def reference(value: object) -> str | None:
    """Resolve a Docling JSON pointer, which serializes as `{"$ref": "#/texts/0"}`."""
    if isinstance(value, dict):
        for key in ("$ref", "cref"):
            candidate = value.get(key)
            if isinstance(candidate, str):
                return candidate
        return None
    return value if isinstance(value, str) else None


def docling_label_to_kind(label: object) -> str:
    """Mirror `docling_label_to_kind` in the embedded backend helper, which owns this mapping."""
    value = str(label or "").lower().replace("-", "_")
    if "title" in value and "sub" not in value:
        return "title"
    if "section_header" in value or "heading" in value:
        return "section_header"
    if "list_item" in value:
        return "list_item"
    if "code" in value:
        return "code"
    if "caption" in value:
        return "caption"
    if "footnote" in value:
        return "footnote"
    if "formula" in value:
        return "formula"
    if "page_header" in value:
        return "page_header"
    if "page_footer" in value:
        return "page_footer"
    if "abstract" in value:
        return "abstract"
    return "body_text"


class Batch:
    """One converted document: the whole PDF, or one page range of a batched conversion."""

    def __init__(self, index: int, document: dict):
        self.index = index
        self.texts: list[dict] = list(document.get("texts") or [])
        self.pictures: list[dict] = list(document.get("pictures") or [])
        self.tables: list[dict] = list(document.get("tables") or [])
        self.groups: list[dict] = list(document.get("groups") or [])
        # Ancestry crosses collections — a list group inside a figure is a real shape — so the node
        # index must cover every collection a `parent` pointer can name, not `texts` alone.
        self.node_by_ref: dict[str, dict] = {}
        for collection in (self.texts, self.pictures, self.tables, self.groups):
            for item in collection:
                ref = reference(item.get("self_ref"))
                if ref is not None:
                    self.node_by_ref[ref] = item
        self.caption_refs: dict[str, set[str]] = {}
        for name, collection in (("pictures", self.pictures), ("tables", self.tables)):
            for position, item in enumerate(collection):
                self.caption_refs[f"#/{name}/{position}"] = {
                    ref
                    for ref in (reference(entry) for entry in (item.get("captions") or []))
                    if ref is not None
                }
        self.body_texts = [item for item in self.texts if item.get("content_layer") == BODY_LAYER]
        self.body_normalized = [normalize(item.get("text")) for item in self.body_texts]

    def blocking_picture(self, item: dict) -> tuple[str, str] | None:
        """Return the picture that hides this item from `iterate_items`, and the blocked child.

        `iterate_items(traverse_pictures=False)` skips every child of a `PictureItem` except the
        refs in that picture's own `captions` list. The skip is at the boundary, so everything below
        the blocked child is skipped with it — a list group inside a figure takes its list items
        down too. Walking only the direct parent would miss exactly those.
        """
        current = item
        visited: set[str] = set()
        while True:
            self_ref = reference(current.get("self_ref"))
            parent = reference(current.get("parent"))
            if parent is None or parent in visited:
                return None
            visited.add(parent)
            if parent.startswith("#/pictures/"):
                if self_ref is not None and self_ref in self.caption_refs.get(parent, set()):
                    return None  # reached through the captions exception
                return parent, str(self_ref)
            current = self.node_by_ref.get(parent)
            if current is None:
                return None

    def traversal_exclusion(self, item: dict) -> dict | None:
        """Name the predicate that stops `DoclingDocument.iterate_items` from yielding this item.

        This is the **single** definition of the library's traversal in this producer. The
        conservation census reaches it through `drop_reason`, which needs the named reason; the
        `--oracle` control reaches it through `traversal_yields`, which needs only the verdict.
        Keeping one definition is what makes the oracle a test of the model rather than a test of a
        second copy of it that could drift from the one the census actually uses.
        """
        self_ref = str(reference(item.get("self_ref")))
        layer = item.get("content_layer")
        if layer != BODY_LAYER:
            return {
                "reason": "content_layer_excluded",
                "deciding_field": f"texts[{self_ref}].content_layer = {layer!r}",
                "mechanism": (
                    "iterate_items yields an item only when its content layer is in "
                    "DEFAULT_CONTENT_LAYERS, which is {ContentLayer.BODY}"
                ),
            }
        blocked = self.blocking_picture(item)
        if blocked is not None:
            picture, child = blocked
            return {
                "reason": "picture_interior_not_traversed",
                "deciding_field": (
                    f"texts[{self_ref}] descends from {picture} through {child}, which is absent "
                    f"from {picture}.captions"
                ),
                "mechanism": (
                    "iterate_items is called with traverse_pictures=False, so a PictureItem's "
                    "children are skipped except the refs in that picture's own captions list, and "
                    "the skip takes every descendant of the blocked child with it"
                ),
            }
        return None

    def traversal_yields(self, item: dict) -> bool:
        """Predict whether `doc.iterate_items()` yields this text item, as production calls it."""
        return self.traversal_exclusion(item) is None

    def drop_reason(self, item: dict) -> dict:
        """Name the field that decides an item never becomes a `content_element`.

        Each branch mirrors one predicate on the ingest path — the first two in
        `DoclingDocument.iterate_items` (docling-core), the last two in the embedded backend helper
        in `crates/specforge/src/ir/source/docling_backend.rs`.
        """
        excluded = self.traversal_exclusion(item)
        if excluded is not None:
            return excluded
        self_ref = str(reference(item.get("self_ref")))
        kind = docling_label_to_kind(item.get("label"))
        if kind in ("page_header", "page_footer"):
            return {
                "reason": "skipped_by_kind",
                "deciding_field": f"texts[{self_ref}].label = {item.get('label')!r} -> {kind}",
                "mechanism": "the backend helper skips page_header and page_footer kinds",
            }
        if not normalize(item.get("text")):
            return {
                "reason": "empty_text",
                "deciding_field": f"texts[{self_ref}].text is empty after normalization",
                "mechanism": "the backend helper skips an element whose normalized text is empty",
            }
        return {
            "reason": "unexplained",
            "deciding_field": f"texts[{self_ref}]",
            "mechanism": (
                "no ingest predicate accounts for this item; the drop model is incomplete and this "
                "producer must not guess"
            ),
        }


class ConverterDocument:
    """The converter's own output for one ingest, in either shape ingest can write it.

    A bounded-memory ingest converts page ranges and writes one document per range, so `self_ref`
    values are per-range and collide across ranges. Batches are therefore kept separate and an item
    is identified by `(batch, self_ref)`, never by `self_ref` alone.
    """

    def __init__(self, path: Path):
        self.path = path
        bundle = census.read_json(path)
        if not isinstance(bundle, dict):
            raise ValueError(f"converter bundle is not an object: {path}")
        if isinstance(bundle.get("documents"), list) and bundle.get("batched"):
            documents = bundle["documents"]
            self.batched = True
            self.page_batches = bundle.get("page_batches")
        elif isinstance(bundle.get("texts"), list):
            documents = [bundle]
            self.batched = False
            self.page_batches = None
        else:
            # Silence here would produce an empty document and a confidently wrong "nothing was
            # dropped". An unrecognized bundle must stop the measurement.
            raise ValueError(
                f"unrecognized converter bundle shape at {path}: expected a document with `texts` "
                f"or a batched bundle with `batched` and `documents`, found keys "
                f"{sorted(bundle)[:12]}"
            )
        self.batches = [Batch(index, document) for index, document in enumerate(documents)]
        self.items: list[tuple[Batch, dict]] = [
            (batch, item) for batch in self.batches for item in batch.texts
        ]
        # Body-layer text in conversion order: batches in page order, items in document order. That
        # is the only order `content_elements` can follow, so it is the order a split paragraph must
        # be searched in.
        self.body_items: list[tuple[Batch, dict]] = [
            (batch, item) for batch in self.batches for item in batch.body_texts
        ]
        self.index_body()

    def index_body(self) -> None:
        """Build the body-layer text, token stream, owner map, and anchor index.

        The token stream and its owner map are what an alignment works on: a persisted paragraph the
        converter now interrupts with a figure fragment is not contiguous in the joined text, so a
        string search cannot see it and a token alignment can.
        """
        self.body_normalized = [normalize(item.get("text")) for _batch, item in self.body_items]
        self.body_joined = " ".join(self.body_normalized)
        self.body_tokens: list[str] = []
        self.token_owner: list[int] = []
        for position, normalized in enumerate(self.body_normalized):
            for token in normalized.split():
                self.body_tokens.append(token)
                self.token_owner.append(position)
        self.gram_index: dict[tuple[str, ...], list[int]] = {}
        for start in range(0, max(0, len(self.body_tokens) - ANCHOR_GRAM + 1)):
            gram = tuple(self.body_tokens[start : start + ANCHOR_GRAM])
            self.gram_index.setdefault(gram, []).append(start)
        self.token_positions: dict[str, list[int]] = {}
        for position, token in enumerate(self.body_tokens):
            self.token_positions.setdefault(token, []).append(position)

    def address(self, batch: Batch, item: dict) -> str:
        ref = str(reference(item.get("self_ref")))
        return f"batch{batch.index}:{ref}" if self.batched else ref


class SourceIrIndex:
    """What one `SourceIR` artifact carries, indexed for the question "did this item reach it?".

    A batched ingest makes `source_ref` ambiguous — the Arm Debug guide's 6,784 content elements
    share 2,252 distinct refs — so reachability is keyed on the `(source_ref, text)` pair. That is
    exact for an unbatched document and the strongest available join for a batched one.
    """

    def __init__(self, artifact: dict):
        self.pairs: set[tuple[str, str]] = set()
        for record in artifact.get("content_elements") or []:
            ref = reference(record.get("source_ref"))
            if ref is not None:
                self.pairs.add((ref, normalize(record.get("text"))))
        for record in artifact.get("document_sections") or []:
            ref = reference(record.get("source_ref"))
            if ref is not None:
                self.pairs.add((ref, normalize(record.get("title"))))
        # A caption reaches SourceIR through the bound `caption_text` on its table or figure even
        # when its own text item carries no record, so counting it as dropped would overstate the
        # gap. Matching on text alone can only over-credit, which keeps the census conservative.
        self.caption_texts: set[str] = set()
        for collection in ("structured_tables", "visual_assets"):
            for record in artifact.get(collection) or []:
                text = normalize(record.get("caption_text"))
                if text:
                    self.caption_texts.add(text)
        self.residual_texts: set[str] = set()
        for record in artifact.get("residual_decisions") or []:
            if isinstance(record, dict):
                for value in record.values():
                    text = normalize(value)
                    if text:
                        self.residual_texts.add(text)

    def reached(self, item: dict) -> bool:
        ref = reference(item.get("self_ref"))
        text = normalize(item.get("text"))
        if ref is not None and (ref, text) in self.pairs:
            return True
        return bool(text) and (text in self.caption_texts or text in self.residual_texts)


def absent_elements(persisted: dict, replayed: dict) -> list[dict]:
    """Re-derive the census's absent set element by element rather than as a count.

    Deliberately the same opcode diff and the same whole-string containment test `.1` published, so
    the set adjudicated here is exactly the set that census called content loss. Agreement with the
    census count is asserted by the caller.
    """
    left_elements = persisted.get("content_elements") or []
    right_elements = replayed.get("content_elements") or []
    left = [str(element.get("text", "")) for element in left_elements]
    right = [str(element.get("text", "")) for element in right_elements]
    removed: list[int] = []
    matcher = difflib.SequenceMatcher(a=left, b=right, autojunk=False)
    for tag, i1, i2, _j1, _j2 in matcher.get_opcodes():
        if tag in ("delete", "replace"):
            removed.extend(range(i1, i2))
    joined = " \n".join(right)
    return [
        {
            "ordinal": index,
            "element_id": left_elements[index].get("element_id"),
            "kind": left_elements[index].get("kind"),
            "page_id": left_elements[index].get("page_id"),
            "persisted_source_ref": left_elements[index].get("source_ref"),
            "text": left[index],
        }
        for index in removed
        if left[index] not in joined
    ]


def align(text: str, document: ConverterDocument) -> dict:
    """Locate one persisted text in the converter's own document, word by word.

    A string search answers only "is this exact run of characters still there", and both ways a
    converter can keep a paragraph while moving it defeat that question: it can split the paragraph
    across items, and it can splice a newly detected figure fragment into the middle of a sentence.
    Neither loses a word. So the reading here is an ordered token alignment inside a bounded window:
    every persisted token accounted for, in order, is preservation however much was inserted around
    it; a persisted token with no match is the only thing that is loss.

    The window is what keeps that honest. Without it, a long-enough document would "recover" any
    paragraph from tokens scattered across unrelated pages.
    """
    target = normalize(text)
    if not target:
        return {"located_as": "empty", "covering": [], "missing_tokens": [], "inserted_tokens": 0}
    exact = [
        (batch, item) for batch, item in document.items if normalize(item.get("text")) == target
    ]
    if exact:
        return {
            "located_as": "whole_item",
            "covering": exact,
            "missing_tokens": [],
            "inserted_tokens": 0,
        }
    within = [
        (batch, item) for batch, item in document.items if target in normalize(item.get("text"))
    ]
    if within:
        return {
            "located_as": "within_item",
            "covering": within,
            "missing_tokens": [],
            "inserted_tokens": 0,
        }

    tokens = target.split()
    starts, truncated = anchor_positions(tokens, document)
    if not starts:
        return {
            "located_as": "docling_absent",
            "covering": [],
            "missing_tokens": tokens,
            "inserted_tokens": 0,
        }
    best: dict | None = None
    for start in starts:
        candidate = align_at(tokens, document, start)
        if best is None or len(candidate["missing_tokens"]) < len(best["missing_tokens"]):
            best = candidate
        if not best["missing_tokens"]:
            break
    assert best is not None
    if not best["missing_tokens"]:
        best["located_as"] = "aligned_with_insertions" if best["inserted_tokens"] else "split_across_items"
    elif truncated:
        # The search could not try every candidate anchor, so "not found" here is a limit of the
        # search rather than a statement about the document. Say that instead of claiming absence.
        best["located_as"] = "unresolved_anchor_truncated"
    elif len(best["missing_tokens"]) == len(tokens):
        best["located_as"] = "docling_absent"
    else:
        best["located_as"] = "partially_absent"
    return best


def anchor_positions(tokens: list[str], document: ConverterDocument) -> tuple[list[int], bool]:
    """Where in the converter's token stream this text most plausibly begins.

    Two anchors, widest first. A shared n-gram is the strong one: each votes for the stream position
    the text would start at, and the best-supported positions are tried first. But every n-gram of a
    short paragraph can be broken by one spliced-in fragment, and a truncated text may be shorter
    than the gram itself, so the fallback votes from the text's **rarest** token instead — rare
    enough to be a real anchor, and defined for any non-empty text.

    Returns the candidates and whether the candidate list had to be truncated, because an alignment
    that could not try every candidate must not be reported as proof of absence.
    """
    if len(tokens) >= ANCHOR_GRAM:
        votes: dict[int, int] = {}
        for offset in range(0, len(tokens) - ANCHOR_GRAM + 1):
            gram = tuple(tokens[offset : offset + ANCHOR_GRAM])
            for position in document.gram_index.get(gram, ()):
                implied = position - offset
                votes[implied] = votes.get(implied, 0) + 1
        if votes:
            ranked = sorted(votes.items(), key=lambda pair: (-pair[1], pair[0]))
            return [start for start, _count in ranked[:ANCHOR_CANDIDATES]], False
    rarest_offset: int | None = None
    rarest: list[int] = []
    for offset, token in enumerate(tokens):
        positions = document.token_positions.get(token)
        if not positions:
            continue
        if rarest_offset is None or len(positions) < len(rarest):
            rarest_offset, rarest = offset, positions
    if rarest_offset is None:
        return [], False
    truncated = len(rarest) > ANCHOR_CANDIDATES
    return [position - rarest_offset for position in rarest[:ANCHOR_CANDIDATES]], truncated


def align_at(tokens: list[str], document: ConverterDocument, start: int) -> dict:
    """Align the persisted tokens against one bounded window of the converter's stream."""
    span = len(tokens)
    low = max(0, start - span)
    high = min(len(document.body_tokens), start + (ALIGNMENT_WINDOW + 1) * span)
    window = document.body_tokens[low:high]
    matcher = difflib.SequenceMatcher(a=tokens, b=window, autojunk=False)
    opcodes = matcher.get_opcodes()
    matched = [index for index, opcode in enumerate(opcodes) if opcode[0] == "equal"]
    # Insertions are counted only BETWEEN the first and last matched token. The window deliberately
    # extends past the text on both sides so an alignment can start anywhere in it; counting that
    # surrounding context as interleaved would inflate every result.
    first, last = (matched[0], matched[-1]) if matched else (len(opcodes), -1)
    missing: list[str] = []
    inserted = 0
    owners: list[int] = []
    for index, (tag, i1, i2, j1, j2) in enumerate(opcodes):
        if tag in ("delete", "replace"):
            missing.extend(tokens[i1:i2])
        if tag in ("insert", "replace") and first < index < last:
            inserted += j2 - j1
        if tag == "equal":
            owners.extend(document.token_owner[low + j1 : low + j2])
    covering = [document.body_items[position] for position in sorted(set(owners))]
    return {
        "covering": covering,
        "missing_tokens": missing,
        "inserted_tokens": inserted,
    }


def adjudicate(element: dict, document: ConverterDocument, index: SourceIrIndex) -> dict:
    """Decide where one persisted content element's text went, and name the deciding field."""
    location = align(element["text"], document)
    record = {
        **element,
        "located_as": location["located_as"],
        "inserted_tokens": location["inserted_tokens"],
        "missing_tokens": location["missing_tokens"][:24],
        "missing_token_count": len(location["missing_tokens"]),
        "covering_items": [],
    }
    if location["located_as"] in (
        "docling_absent",
        "partially_absent",
        "unresolved_anchor_truncated",
    ):
        record["verdict"] = location["located_as"]
        record["deciding_field"] = (
            f"{len(location['missing_tokens'])} of {len(normalize(element['text']).split())} "
            "persisted tokens have no match in the converter's own body-layer stream within the "
            "bounded alignment window"
        )
        return record

    dropped: list[dict] = []
    for batch, item in location["covering"]:
        reached = index.reached(item)
        provenance = (item.get("prov") or [{}])[0]
        summary = {
            "address": document.address(batch, item),
            "label": item.get("label"),
            "content_layer": item.get("content_layer"),
            "parent": reference(item.get("parent")),
            "page_no": provenance.get("page_no") if isinstance(provenance, dict) else None,
            "text": normalize(item.get("text"))[:160],
            "reached_source_ir": reached,
        }
        if not reached:
            summary["drop"] = batch.drop_reason(item)
            dropped.append(summary)
        record["covering_items"].append(summary)

    if dropped:
        record["verdict"] = "specforge_dropped"
        record["deciding_field"] = "; ".join(entry["drop"]["deciding_field"] for entry in dropped)
        record["drop_reasons"] = sorted({entry["drop"]["reason"] for entry in dropped})
        return record
    record["verdict"] = "retained"
    record["deciding_field"] = (
        f"every persisted token is present in order ({location['inserted_tokens']} tokens inserted "
        "around them) and every covering converter item carries a SourceIR record: "
        + ", ".join(entry["address"] for entry in record["covering_items"])
    )
    return record


def persisted_result(records: list[dict]) -> dict:
    """Summarise a persisted-mode run: no adjudication, only what reached the artifact."""
    measured = [record for record in records if "conservation" in record]
    by_reason: dict[str, int] = {}
    for record in measured:
        for reason, bucket in record["conservation"]["by_reason"].items():
            by_reason[reason] = by_reason.get(reason, 0) + bucket["count"]
    return {
        "documents_measured": len(measured),
        "converter_text_items": sum(r["conservation"]["converter_text_items"] for r in measured),
        "converter_items_with_no_source_ir_record": sum(
            r["conservation"]["reached_no_source_ir_record"] for r in measured
        ),
        "by_reason": dict(sorted(by_reason.items())),
    }


def conservation_census(document: ConverterDocument, index: SourceIrIndex) -> dict:
    """Count every converter text item this ingest carried over, and every one it did not.

    This is the measurement the pipeline has nowhere else: stage conservation starts at `SourceIR`
    because there is no upstream artifact to conserve against, so nothing observes what ingest
    leaves behind at the PDF boundary.
    """
    kept = 0
    buckets: dict[str, dict] = {}
    for batch, item in document.items:
        if index.reached(item):
            kept += 1
            continue
        drop = batch.drop_reason(item)
        bucket = buckets.setdefault(
            drop["reason"],
            {"count": 0, "labels": {}, "mechanism": drop["mechanism"], "text_samples": []},
        )
        bucket["count"] += 1
        label = str(item.get("label"))
        bucket["labels"][label] = bucket["labels"].get(label, 0) + 1
        text = normalize(item.get("text"))
        if text and len(bucket["text_samples"]) < 4:
            bucket["text_samples"].append(text[:80])
    for bucket in buckets.values():
        bucket["labels"] = dict(sorted(bucket["labels"].items()))
    return {
        "batched": document.batched,
        "converter_batches": len(document.batches),
        "converter_text_items": len(document.items),
        "reached_source_ir": kept,
        "reached_no_source_ir_record": len(document.items) - kept,
        "by_reason": dict(sorted(buckets.items())),
    }


# --------------------------------------------------------------------------------------------
# Traversal oracle
# --------------------------------------------------------------------------------------------
#
# The conservation census's largest bucket rests on this producer's own reimplementation of two
# docling-core predicates, read out of that library's source. A reading is not a measurement. The
# oracle replaces it with one: it asks the library itself which items `iterate_items` yields, on the
# same serialized document ingest built its elements from, and compares that against the model.


# Resolved the way production resolves it (`DOCLING_PYTHON_ENV` / `DEFAULT_DOCLING_VENV_DIR` in
# `crates/specforge/src/ir/source/docling_backend.rs`), so the oracle observes the same interpreter
# and the same installed docling-core that built the artifacts.
DOCLING_PYTHON_ENV = "SPECFORGE_DOCLING_PYTHON"
DEFAULT_DOCLING_PYTHON = Path(".venv-docling/bin/python")

# Runs inside that interpreter, because `DoclingDocument` is a pydantic model only there. It calls
# `iterate_items()` with no arguments — exactly as the embedded backend helper does — so the
# defaults under test (`traverse_pictures=False`, body-only content layers) are production's, not a
# restatement of them. It also re-exports the validated document and compares it to the input,
# which is what licenses reading a serialized bundle as the document ingest actually traversed.
TRAVERSAL_PROBE = r"""
import json
import sys
from importlib.metadata import version

from docling_core.types.doc.document import DoclingDocument

bundle = json.load(open(sys.argv[1], encoding="utf-8"))
documents = bundle["documents"] if bundle.get("batched") else [bundle]

batches = []
for index, raw in enumerate(documents):
    document = DoclingDocument.model_validate(raw)
    yielded = []
    by_collection = {}
    for item, _level in document.iterate_items():
        ref = getattr(item, "self_ref", None)
        if not isinstance(ref, str):
            continue
        collection = ref.split("/")[1] if ref.startswith("#/") and "/" in ref[2:] else "?"
        by_collection[collection] = by_collection.get(collection, 0) + 1
        if collection == "texts":
            yielded.append(ref)
    batches.append(
        {
            "batch": index,
            "yielded_text_refs": yielded,
            "yielded_by_collection": dict(sorted(by_collection.items())),
            "round_trips": document.export_to_dict() == raw,
        }
    )

json.dump(
    {"docling_core": version("docling-core"), "batches": batches}, sys.stdout, ensure_ascii=False
)
"""


def resolve_docling_python() -> Path:
    """Resolve the interpreter production would use, and refuse rather than fall back to this one.

    Silently running the probe under the producer's own interpreter would import whatever
    docling-core happens to be on the system path, or none, and an oracle observing a different
    library than the one that built the artifacts is worse than no oracle.
    """
    override = os.environ.get(DOCLING_PYTHON_ENV)
    candidate = Path(override) if override else ROOT / DEFAULT_DOCLING_PYTHON
    if not candidate.is_file():
        raise ValueError(
            f"no docling interpreter at {candidate}; set {DOCLING_PYTHON_ENV} to the interpreter "
            f"that has docling-core installed"
        )
    return candidate


def observe_traversal(bundle: Path) -> dict:
    """Ask docling-core itself which text items `iterate_items` yields for this bundle."""
    python = resolve_docling_python()
    completed = subprocess.run(
        [str(python), "-c", TRAVERSAL_PROBE, str(bundle)],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    if completed.returncode != 0:
        raise ValueError(
            f"traversal probe failed for {bundle.name} with return code {completed.returncode}: "
            + " ".join((completed.stderr or "").split())[-400:]
        )
    observation = json.loads(completed.stdout)
    observation["interpreter"] = (
        python.relative_to(ROOT).as_posix() if python.is_relative_to(ROOT) else python.as_posix()
    )
    return observation


def traversal_oracle(document: ConverterDocument, observation: dict, artifact: dict) -> dict:
    """Compare the library's own traversal against the drop model's prediction, batch by batch.

    Both directions are reported separately and never netted. A single agreement count would let an
    item the model wrongly excludes cancel an item it wrongly includes, and that pair is exactly the
    error a hand-written reimplementation of someone else's traversal makes.

    The comparison is per batch because a bounded-memory ingest writes one document per page range
    and `self_ref` restarts at zero in each, so pooling the refs would let batch 1 vouch for an item
    batch 0 never yielded.
    """
    observed_batches = observation.get("batches") or []
    if len(observed_batches) != len(document.batches):
        raise ValueError(
            f"traversal probe reported {len(observed_batches)} batches but the bundle has "
            f"{len(document.batches)}; a partial comparison would read as agreement"
        )

    per_batch: list[dict] = []
    predicted_not_yielded: list[str] = []
    yielded_not_predicted: list[str] = []
    unknown_refs: list[str] = []
    duplicate_refs: list[str] = []
    predicted_total = 0
    yielded_total = 0
    filtered_by_kind: dict[str, int] = {}
    filtered_empty_text: dict[str, int] = {}
    reachable = 0
    round_trips = True
    by_collection: dict[str, int] = {}

    for batch, observed in zip(document.batches, observed_batches):
        refs = list(observed.get("yielded_text_refs") or [])
        observed_set = set(refs)
        round_trips = round_trips and bool(observed.get("round_trips"))
        for collection, count in (observed.get("yielded_by_collection") or {}).items():
            by_collection[collection] = by_collection.get(collection, 0) + count
        address = f"batch{batch.index}:" if document.batched else ""
        if len(refs) != len(observed_set):
            seen: set[str] = set()
            for ref in refs:
                if ref in seen:
                    duplicate_refs.append(address + ref)
                seen.add(ref)

        known = {reference(item.get("self_ref")) for item in batch.texts}
        unknown_refs.extend(address + ref for ref in sorted(observed_set - known))

        predicted_set = set()
        for item in batch.texts:
            ref = reference(item.get("self_ref"))
            if ref is not None and batch.traversal_yields(item):
                predicted_set.add(ref)

        missing = sorted(predicted_set - observed_set)
        extra = sorted(observed_set - predicted_set)
        predicted_not_yielded.extend(address + ref for ref in missing)
        yielded_not_predicted.extend(address + ref for ref in extra)
        predicted_total += len(predicted_set)
        yielded_total += len(observed_set)

        # The residue is measured against what the library actually yielded, not against what the
        # model predicted, so a model error cannot also distort the element accounting.
        for item in batch.texts:
            if reference(item.get("self_ref")) not in observed_set:
                continue
            label = str(item.get("label"))
            kind = docling_label_to_kind(label)
            if kind in ("page_header", "page_footer"):
                filtered_by_kind[label] = filtered_by_kind.get(label, 0) + 1
                continue
            if not normalize(item.get("text")):
                filtered_empty_text[label] = filtered_empty_text.get(label, 0) + 1
                continue
            reachable += 1

        per_batch.append(
            {
                "batch": batch.index,
                "converter_text_items": len(batch.texts),
                "library_yielded": len(observed_set),
                "model_predicted_yield": len(predicted_set),
                "predicted_not_yielded": len(missing),
                "yielded_not_predicted": len(extra),
            }
        )

    content_elements = len(artifact.get("content_elements") or [])
    disagreements = {
        "predicted_not_yielded": {
            "count": len(predicted_not_yielded),
            "meaning": "the model predicts iterate_items yields this item; the library did not",
            "samples": predicted_not_yielded[:20],
        },
        "yielded_not_predicted": {
            "count": len(yielded_not_predicted),
            "meaning": "the library yielded this item; the model predicts it is excluded",
            "samples": yielded_not_predicted[:20],
        },
        "yielded_ref_absent_from_bundle": {
            "count": len(unknown_refs),
            "meaning": (
                "the library yielded a text ref this bundle's texts collection does not hold"
            ),
            "samples": unknown_refs[:20],
        },
        "duplicate_yields": {
            "count": len(duplicate_refs),
            "meaning": "the library yielded the same ref twice within one batch",
            "samples": duplicate_refs[:20],
        },
    }
    disagreement_count = sum(entry["count"] for entry in disagreements.values())
    return {
        "docling_core": observation.get("docling_core"),
        "interpreter": observation.get("interpreter"),
        "serialized_document_round_trips": round_trips,
        "batched": document.batched,
        "converter_batches": len(document.batches),
        "converter_text_items": len(document.items),
        "library_yielded_text_items": yielded_total,
        "model_predicted_yield": predicted_total,
        "library_yielded_by_collection": dict(sorted(by_collection.items())),
        "disagreements": disagreements,
        "disagreement_count": disagreement_count,
        "agrees": disagreement_count == 0,
        "element_residue": {
            "library_yielded_text_items": yielded_total,
            "filtered_skipped_by_kind": dict(sorted(filtered_by_kind.items())),
            "filtered_empty_text": dict(sorted(filtered_empty_text.items())),
            "expected_content_elements": reachable,
            "artifact_content_elements": content_elements,
            "unexplained": content_elements - reachable,
        },
        "per_batch": per_batch,
    }


def oracle_result(records: list[dict]) -> dict:
    """Summarise an oracle run: the model is confirmed only when nothing disagrees anywhere.

    A document the probe could not read is a hole in the frame, not a neutral omission — the run
    exits on this verdict, so confirming while a document was skipped would make the gate green for
    the wrong reason.
    """
    measured = [record for record in records if "traversal_oracle" in record]
    unmeasurable = [record for record in records if "traversal_oracle" not in record]
    oracles = [record["traversal_oracle"] for record in measured]
    return {
        "documents_measured": len(measured),
        "documents_unmeasurable": len(unmeasurable),
        "unmeasurable": [
            {"document_key": record["document_key"], "reason": record.get("reason")}
            for record in unmeasurable
        ],
        "converter_text_items": sum(o["converter_text_items"] for o in oracles),
        "library_yielded_text_items": sum(o["library_yielded_text_items"] for o in oracles),
        "model_predicted_yield": sum(o["model_predicted_yield"] for o in oracles),
        "disagreements": sum(o["disagreement_count"] for o in oracles),
        "serialized_documents_round_trip": all(
            o["serialized_document_round_trips"] for o in oracles
        ),
        "unexplained_content_elements": sum(
            o["element_residue"]["unexplained"] for o in oracles
        ),
        "model_confirmed": bool(oracles)
        and not unmeasurable
        and all(o["agrees"] and o["serialized_document_round_trips"] for o in oracles),
    }


# --------------------------------------------------------------------------------------------
# Controls
# --------------------------------------------------------------------------------------------


def _text(ref: str, text: str, **overrides) -> dict:
    record = {
        "self_ref": ref,
        "text": text,
        "label": "text",
        "content_layer": BODY_LAYER,
        "parent": {"$ref": "#/body"},
        "prov": [{"page_no": 1}],
    }
    record.update(overrides)
    return record


def _element(text: str, ordinal: int = 0) -> dict:
    return {
        "ordinal": ordinal,
        "element_id": f"elem_{ordinal:05d}",
        "kind": "body_text",
        "page_id": "page_0001",
        "persisted_source_ref": "#/texts/0",
        "text": text,
    }


class _FixtureDocument(ConverterDocument):
    """A `ConverterDocument` built from a literal, so each control names one exact predicate."""

    def __init__(self, *documents: dict, batched: bool = False):  # noqa: D107
        self.path = Path("<fixture>")
        self.batched = batched
        self.page_batches = None
        self.batches = [Batch(index, document) for index, document in enumerate(documents)]
        self.items = [(batch, item) for batch in self.batches for item in batch.texts]
        self.body_items = [(batch, item) for batch in self.batches for item in batch.body_texts]
        self.index_body()


def run_self_test() -> int:
    """Each control fixes one reading the adjudication must not default to."""
    checks: list[tuple[bool, str]] = []

    kept_doc = _FixtureDocument({"texts": [_text("#/texts/0", "alpha beta")]})
    kept = adjudicate(
        _element("alpha beta"),
        kept_doc,
        SourceIrIndex({"content_elements": [{"source_ref": "#/texts/0", "text": "alpha beta"}]}),
    )
    checks.append(
        (
            kept["verdict"] == "retained" and kept["located_as"] == "whole_item",
            "a text the converter emitted and SpecForge recorded must not read as lost",
        )
    )

    # The failure mode that produced `.1`'s three-element upper bound: one paragraph, two items.
    split_doc = _FixtureDocument(
        {"texts": [_text("#/texts/0", "alpha beta"), _text("#/texts/1", "gamma delta")]}
    )
    split = adjudicate(
        _element("alpha beta gamma delta"),
        split_doc,
        SourceIrIndex(
            {
                "content_elements": [
                    {"source_ref": "#/texts/0", "text": "alpha beta"},
                    {"source_ref": "#/texts/1", "text": "gamma delta"},
                ]
            }
        ),
    )
    checks.append(
        (
            split["verdict"] == "retained"
            and split["located_as"] == "split_across_items"
            and [entry["address"] for entry in split["covering_items"]]
            == ["#/texts/0", "#/texts/1"],
            "a paragraph the converter split across two items must never read as absent",
        )
    )

    half_dropped = adjudicate(
        _element("alpha beta gamma delta"),
        split_doc,
        SourceIrIndex({"content_elements": [{"source_ref": "#/texts/0", "text": "alpha beta"}]}),
    )
    checks.append(
        (
            half_dropped["verdict"] == "specforge_dropped"
            and half_dropped["drop_reasons"] == ["unexplained"],
            "a covering item with no SourceIR record must be a drop, and an unmodelled drop must "
            "say so rather than be absorbed",
        )
    )

    # Figure interior, direct child and one group deeper — the shape a direct-parent test misses.
    interior_doc = _FixtureDocument(
        {
            "texts": [
                _text("#/texts/0", "interior label", parent={"$ref": "#/pictures/0"}),
                _text("#/texts/1", "bound caption", label="caption", parent={"$ref": "#/pictures/0"}),
                _text("#/texts/2", "nested item", label="list_item", parent={"$ref": "#/groups/0"}),
            ],
            "groups": [
                {"self_ref": "#/groups/0", "label": "list", "parent": {"$ref": "#/pictures/0"}}
            ],
            "pictures": [{"self_ref": "#/pictures/0", "captions": [{"$ref": "#/texts/1"}]}],
        }
    )
    empty_index = SourceIrIndex({})
    interior = adjudicate(_element("interior label"), interior_doc, empty_index)
    checks.append(
        (
            interior["verdict"] == "specforge_dropped"
            and interior["drop_reasons"] == ["picture_interior_not_traversed"]
            and "#/pictures/0.captions" in interior["deciding_field"],
            "a picture child outside the picture's captions must name the picture and its captions "
            "as the deciding fields",
        )
    )
    nested = adjudicate(_element("nested item"), interior_doc, empty_index)
    checks.append(
        (
            nested["verdict"] == "specforge_dropped"
            and nested["drop_reasons"] == ["picture_interior_not_traversed"]
            and "#/groups/0" in nested["deciding_field"],
            "figure-interior text one group deeper must still resolve to the picture that hides it, "
            "not to an unexplained drop",
        )
    )
    bound = adjudicate(
        _element("bound caption"),
        interior_doc,
        SourceIrIndex(
            {"visual_assets": [{"source_ref": "#/pictures/0", "caption_text": "bound caption"}]}
        ),
    )
    checks.append(
        (
            bound["verdict"] == "retained",
            "a caption carried by a bound visual asset must not be counted as dropped merely "
            "because its own text item has no record",
        )
    )

    gone = adjudicate(
        _element("nowhere at all"),
        _FixtureDocument({"texts": [_text("#/texts/0", "alpha beta")]}),
        SourceIrIndex({"content_elements": [{"source_ref": "#/texts/0", "text": "alpha beta"}]}),
    )
    checks.append(
        (
            gone["verdict"] == "docling_absent",
            "text the converter never emitted must read as absent from the converter, not as a drop",
        )
    )

    # The measured failure mode: a figure fragment spliced into the middle of a sentence. The
    # sentence keeps every word, so this must never read as loss.
    sentence = "alpha beta gamma delta epsilon zeta eta theta"
    spliced_doc = _FixtureDocument(
        {
            "texts": [
                _text("#/texts/0", "alpha beta gamma delta"),
                _text("#/texts/1", "Sampled 4.0g"),
                _text("#/texts/2", "epsilon zeta eta theta"),
            ]
        }
    )
    spliced_index = SourceIrIndex(
        {
            "content_elements": [
                {"source_ref": "#/texts/0", "text": "alpha beta gamma delta"},
                {"source_ref": "#/texts/1", "text": "Sampled 4.0g"},
                {"source_ref": "#/texts/2", "text": "epsilon zeta eta theta"},
            ]
        }
    )
    spliced = adjudicate(_element(sentence), spliced_doc, spliced_index)
    checks.append(
        (
            spliced["verdict"] == "retained"
            and spliced["located_as"] == "aligned_with_insertions"
            and spliced["inserted_tokens"] == 2
            and spliced["missing_token_count"] == 0,
            "a sentence the converter interrupted with an inserted fragment must read as retained "
            "with the insertion counted, never as absent",
        )
    )

    # The same splice with document around it: the count must be the interleaved fragment, not the
    # window the search happened to look at.
    padded_doc = _FixtureDocument(
        {
            "texts": [_text(f"#/texts/{index}", f"lead{index} padding words here now") for index in range(4)]
            + [
                _text("#/texts/4", "alpha beta gamma delta"),
                _text("#/texts/5", "Sampled 4.0g"),
                _text("#/texts/6", "epsilon zeta eta theta"),
            ]
            + [_text(f"#/texts/{index}", f"tail{index} padding words here now") for index in range(7, 11)]
        }
    )
    padded = align(sentence, padded_doc)
    checks.append(
        (
            padded["inserted_tokens"] == 2 and padded["missing_tokens"] == [],
            "the interleaved-token count must cover only what sits between the first and last "
            "matched word, never the surrounding window",
        )
    )

    # Real partial loss must stay visible, and must name what is missing.
    truncated_doc = _FixtureDocument({"texts": [_text("#/texts/0", "alpha beta gamma delta epsilon")]})
    truncated = adjudicate(
        _element(sentence),
        truncated_doc,
        SourceIrIndex(
            {"content_elements": [{"source_ref": "#/texts/0", "text": "alpha beta gamma delta epsilon"}]}
        ),
    )
    checks.append(
        (
            truncated["verdict"] == "partially_absent"
            and truncated["missing_tokens"] == ["zeta", "eta", "theta"],
            "text the converter truncated must read as partially absent and name the missing words",
        )
    )

    # The window is the only thing stopping a long document from "recovering" anything from noise.
    scattered_doc = _FixtureDocument(
        {
            "texts": [_text("#/texts/0", "alpha beta gamma delta epsilon zeta eta theta")]
            + [_text(f"#/texts/{index}", "filler " * 30) for index in range(1, 12)]
            + [_text("#/texts/12", "alpha beta gamma delta")]
            + [_text(f"#/texts/{index}", "filler " * 30) for index in range(13, 24)]
            + [_text("#/texts/24", "epsilon zeta eta theta")]
        }
    )
    far_apart = align(sentence, scattered_doc)
    checks.append(
        (
            far_apart["located_as"] in ("whole_item", "within_item"),
            "an exact copy elsewhere in the document must be preferred over a scattered alignment",
        )
    )
    without_copy = _FixtureDocument(
        {
            "texts": [_text("#/texts/0", "alpha beta gamma delta")]
            + [_text(f"#/texts/{index}", "filler " * 30) for index in range(1, 12)]
            + [_text("#/texts/12", "epsilon zeta eta theta")]
        }
    )
    checks.append(
        (
            align(sentence, without_copy)["missing_tokens"] != [],
            "halves separated by more than the alignment window must not be joined into a recovery",
        )
    )

    # Two batches reusing `#/texts/0` for different text: the ref alone must not credit either one.
    batched_doc = _FixtureDocument(
        {"texts": [_text("#/texts/0", "first range")]},
        {"texts": [_text("#/texts/0", "second range")]},
        batched=True,
    )
    batched_index = SourceIrIndex(
        {"content_elements": [{"source_ref": "#/texts/0", "text": "first range"}]}
    )
    batched_conservation = conservation_census(batched_doc, batched_index)
    checks.append(
        (
            batched_conservation["reached_no_source_ir_record"] == 1
            and batched_conservation["converter_text_items"] == 2,
            "a batched bundle must be keyed on (batch, ref) plus text, so a colliding self_ref in "
            "another page range is not credited with a record it does not have",
        )
    )
    checks.append(
        (
            batched_doc.address(batched_doc.batches[1], batched_doc.batches[1].texts[0])
            == "batch1:#/texts/0",
            "a batched item's address must carry its batch, because self_ref alone is ambiguous",
        )
    )

    conservation = conservation_census(
        _FixtureDocument(
            {
                "texts": [
                    _text("#/texts/0", "kept"),
                    _text("#/texts/1", "interior", parent={"$ref": "#/pictures/0"}),
                    _text("#/texts/2", "running head", label="page_header", content_layer="furniture"),
                ],
                "pictures": [{"self_ref": "#/pictures/0", "captions": []}],
            }
        ),
        SourceIrIndex({"content_elements": [{"source_ref": "#/texts/0", "text": "kept"}]}),
    )
    checks.append(
        (
            conservation["reached_no_source_ir_record"] == 2
            and conservation["by_reason"]["picture_interior_not_traversed"]["count"] == 1
            and conservation["by_reason"]["content_layer_excluded"]["count"] == 1,
            "the conservation census must separate a figure-interior drop from a furniture-layer "
            "exclusion rather than pooling them",
        )
    )

    persisted = {"content_elements": [{"element_id": "elem_00001", "text": "alpha beta"}]}
    replayed = {"content_elements": [{"element_id": "elem_00001", "text": "alpha beta extra"}]}
    checks.append(
        (
            absent_elements(persisted, replayed) == [],
            "a persisted text still contained in a replayed element must not enter the absent set",
        )
    )
    replayed_lost = {"content_elements": [{"element_id": "elem_00001", "text": "unrelated"}]}
    checks.append(
        (
            len(absent_elements(persisted, replayed_lost))
            == census.content_drift_shape(persisted, replayed_lost)[
                "removed_text_absent_from_replay"
            ],
            "the adjudicated set size must equal the census's own absent count",
        )
    )

    # An unrecognized bundle must stop the measurement rather than read as an empty document.
    unknown = Path(".project-data/tmp/.ingest-content-loss-selftest-bundle.json")
    unknown.parent.mkdir(parents=True, exist_ok=True)
    unknown.write_text(json.dumps({"unexpected": True}), encoding="utf-8")
    try:
        ConverterDocument(unknown)
        refused = False
    except ValueError:
        refused = True
    finally:
        unknown.unlink(missing_ok=True)
    checks.append(
        (refused, "an unrecognized converter bundle must be refused, never read as zero items")
    )

    # ── Traversal oracle ──────────────────────────────────────────────────────────────────────
    # The oracle's own failure mode is agreeing by construction. Each control below drives the
    # comparator with an observation the library did NOT produce, so a comparator that cannot fail
    # is caught here rather than in a run whose green result would mean nothing.

    def _oracle_fixture() -> tuple[_FixtureDocument, dict]:
        """One body text, one figure-interior text, one empty formula the library still yields."""
        document = _FixtureDocument(
            {
                "texts": [
                    _text("#/texts/0", "kept"),
                    _text("#/texts/1", "interior", parent={"$ref": "#/pictures/0"}),
                    _text("#/texts/2", "", label="formula"),
                ],
                "pictures": [{"self_ref": "#/pictures/0", "captions": []}],
            }
        )
        artifact = {"content_elements": [{"source_ref": "#/texts/0", "text": "kept"}]}
        return document, artifact

    def _observation(*batches: list[str], round_trips: bool = True) -> dict:
        return {
            "docling_core": "fixture",
            "batches": [
                {
                    "batch": index,
                    "yielded_text_refs": refs,
                    "yielded_by_collection": {"texts": len(refs)},
                    "round_trips": round_trips,
                }
                for index, refs in enumerate(batches)
            ],
        }

    oracle_doc, oracle_artifact = _oracle_fixture()
    agreeing = traversal_oracle(
        oracle_doc, _observation(["#/texts/0", "#/texts/2"]), oracle_artifact
    )
    checks.append(
        (
            agreeing["agrees"]
            and agreeing["disagreement_count"] == 0
            and agreeing["model_predicted_yield"] == 2,
            "the oracle must confirm the model when the library yields exactly the predicted set",
        )
    )
    checks.append(
        (
            agreeing["element_residue"]["filtered_empty_text"] == {"formula": 1}
            and agreeing["element_residue"]["expected_content_elements"] == 1
            and agreeing["element_residue"]["unexplained"] == 0,
            "an empty formula the library yields must be attributed to the backend helper's own "
            "filter, not left in the unexplained residue",
        )
    )

    # RED: the library yields an item the model predicts is hidden inside a figure.
    extra = traversal_oracle(
        oracle_doc,
        _observation(["#/texts/0", "#/texts/1", "#/texts/2"]),
        oracle_artifact,
    )
    checks.append(
        (
            not extra["agrees"]
            and extra["disagreements"]["yielded_not_predicted"]["count"] == 1
            and extra["disagreements"]["yielded_not_predicted"]["samples"] == ["#/texts/1"],
            "a figure-interior item the library actually yields must be reported against the "
            "model, because that is the direction that would overstate the published gap",
        )
    )

    # RED: the library does not yield an item the model predicts it does.
    missing = traversal_oracle(oracle_doc, _observation(["#/texts/2"]), oracle_artifact)
    checks.append(
        (
            not missing["agrees"]
            and missing["disagreements"]["predicted_not_yielded"]["count"] == 1
            and missing["disagreements"]["predicted_not_yielded"]["samples"] == ["#/texts/0"],
            "an item the model predicts but the library never yields must be reported, because "
            "that is the direction that would understate the published gap",
        )
    )

    # RED: one error of each kind, which a single agreement count would net to zero.
    offsetting = traversal_oracle(
        oracle_doc, _observation(["#/texts/1", "#/texts/2"]), oracle_artifact
    )
    checks.append(
        (
            not offsetting["agrees"]
            and offsetting["library_yielded_text_items"] == offsetting["model_predicted_yield"]
            and offsetting["disagreements"]["predicted_not_yielded"]["count"] == 1
            and offsetting["disagreements"]["yielded_not_predicted"]["count"] == 1,
            "two offsetting errors must both be reported; an equal yield count is not agreement",
        )
    )

    # RED: a batched bundle must be compared per batch, not on a pooled ref set.
    batched_oracle_doc = _FixtureDocument(
        {"texts": [_text("#/texts/0", "first range")]},
        {"texts": [_text("#/texts/0", "second range", parent={"$ref": "#/pictures/0"})],
         "pictures": [{"self_ref": "#/pictures/0", "captions": []}]},
        batched=True,
    )
    pooled = traversal_oracle(
        batched_oracle_doc,
        _observation(["#/texts/0"], ["#/texts/0"]),
        {"content_elements": [{"source_ref": "#/texts/0", "text": "first range"}]},
    )
    checks.append(
        (
            not pooled["agrees"]
            and pooled["disagreements"]["yielded_not_predicted"]["samples"] == ["batch1:#/texts/0"],
            "a colliding self_ref in another page range must not let one batch vouch for an item "
            "the model excludes in the other",
        )
    )

    # RED: a probe that reported fewer batches than the bundle holds must stop the comparison.
    try:
        traversal_oracle(batched_oracle_doc, _observation(["#/texts/0"]), {})
        truncated_refused = False
    except ValueError:
        truncated_refused = True
    checks.append(
        (
            truncated_refused,
            "a probe covering fewer batches than the bundle must be refused, never compared as a "
            "prefix that reads as agreement",
        )
    )

    # RED: a document that does not survive its own serialization is not evidence about ingest.
    lossy = traversal_oracle(
        oracle_doc,
        _observation(["#/texts/0", "#/texts/2"], round_trips=False),
        oracle_artifact,
    )
    checks.append(
        (
            lossy["agrees"] and not oracle_result([{"traversal_oracle": lossy}])["model_confirmed"],
            "a serialized document that does not round-trip must withhold confirmation even when "
            "every yielded ref agrees",
        )
    )

    checks.append(
        (
            not oracle_result(
                [
                    {"traversal_oracle": agreeing},
                    {"document_key": "unreadable", "reason": "probe failed"},
                ]
            )["model_confirmed"],
            "a document the probe could not read must withhold confirmation, because the run's "
            "exit code is the gate and a skipped document is a hole in the frame",
        )
    )

    failures = [message for ok, message in checks if not ok]
    for ok, message in checks:
        print(f"{'PASS' if ok else 'FAIL'}  {message}", file=sys.stderr)
    print(f"{len(checks) - len(failures)}/{len(checks)} controls pass", file=sys.stderr)
    return 1 if failures else 0


# --------------------------------------------------------------------------------------------
# Entry point
# --------------------------------------------------------------------------------------------


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-root", type=Path)
    parser.add_argument("--census-id", type=str)
    parser.add_argument("--owner", type=str)
    parser.add_argument("--document", action="append", dest="documents")
    parser.add_argument("--external-source-map", type=Path)
    parser.add_argument("--plan-only", action="store_true")
    parser.add_argument("--compare-only", action="store_true")
    parser.add_argument("--persisted", action="store_true")
    parser.add_argument("--oracle", action="store_true")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()

    if args.oracle and args.persisted:
        raise ValueError("--oracle and --persisted are separate measurements; run one at a time")

    if args.self_test:
        if args.output_root or args.census_id or args.owner or args.documents:
            raise ValueError("--self-test takes no other arguments")
        return run_self_test()
    if args.output_root is None or args.census_id is None or args.owner is None:
        raise ValueError("--output-root, --census-id, and --owner are required")

    census_id = census.census_identifier(args.census_id, "census id")
    owner = census.census_identifier(args.owner, "owner")
    output_absolute = census.safe_repository_path(
        args.output_root, "output root", below_project_tmp=True
    )

    external_sources: dict[str, Path] = {}
    external_map_path = None
    if args.external_source_map is not None:
        map_absolute = census.safe_repository_path(
            args.external_source_map, "external source map", below_project_tmp=True
        )
        if not map_absolute.is_file():
            raise ValueError(f"external source map is missing: {args.external_source_map}")
        external_sources = census.load_external_map(map_absolute)
        external_map_path = args.external_source_map.as_posix()

    available = dict(census.persisted_documents())
    if args.documents:
        wanted = tuple(args.documents)
    elif args.oracle:
        # The oracle's frame is not the three documents `.1` flagged: the predicate under test is
        # document-neutral code, so the strongest frame is every persisted artifact whose converter
        # document was retained, with no sampling inside it.
        wanted = tuple(
            key
            for key in available
            if (
                ROOT / census.PERSISTED_SOURCE_ROOT / key / "normalized" / f"{key}.backend.json"
            ).is_file()
        )
    else:
        wanted = DECLARED_DOCUMENTS
    unknown = [key for key in wanted if key not in available]
    if unknown:
        raise ValueError(f"no persisted SourceIR artifact for: {', '.join(unknown)}")

    records = [
        census.classify_document(key, available[key], external_sources, args.output_root)
        for key in wanted
    ]

    if not args.compare_only and (
        subprocess.run(["git", "diff", "--quiet", "--", "crates"], cwd=ROOT).returncode != 0
    ):
        raise ValueError("production Rust sources must be unmodified while this producer ingests")
    revision = subprocess.run(
        ["git", "rev-parse", "HEAD"], cwd=ROOT, check=True, capture_output=True, text=True
    ).stdout.strip()

    report = {
        "schema_version": 1,
        "census_id": census_id,
        "owner": owner,
        "mode": "compare_only" if args.compare_only else "re_ingest",
        "production_revision": revision,
        "output_root": args.output_root.as_posix(),
        "external_source_map": external_map_path,
        "selection_rule": (
            "every persisted artifact whose converter document was retained, with no sampling "
            "inside that frame, because the traversal predicate under test is document-neutral"
            if args.oracle and not args.documents
            else "the documents SOURCE-IR-REPRODUCIBILITY.1 reported as losing content the current "
            "toolchain emits nowhere, or the explicit --document list. Within a document there is "
            "no selection: every persisted element the census's own absent test flags is "
            "adjudicated, and the conservation census covers every converter text item."
        ),
        "oracle_rule": (
            "the drop model's two traversal predicates are compared against docling-core's own "
            "iterate_items, called with production's arguments on the same serialized document the "
            "artifact was built from; disagreements are reported in both directions and never "
            "netted, per batch because self_ref restarts in each page range"
        ),
        "adjudication_rule": {
            "whole_item": "some converter text item equals the persisted text after whitespace collapse",
            "within_item": "the persisted text is a substring of one converter text item",
            "split_across_items": "an exact ordered token alignment across more than one item",
            "aligned_with_insertions": (
                "every persisted token aligns in order, with tokens the converter inserted between "
                "them — a figure fragment spliced into a sentence keeps the sentence"
            ),
            "partially_absent": "some persisted token has no match inside the bounded window",
            "docling_absent": "no persisted token aligns; the converter's document does not carry it",
            "alignment_window": (
                f"anchored on shared {ANCHOR_GRAM}-token grams and bounded to "
                f"{ALIGNMENT_WINDOW + 1}x the persisted length, so scattered tokens cannot recover "
                "a paragraph that is genuinely gone"
            ),
            "verdict": (
                "retained when every persisted token aligns and every covering item carries a "
                "SourceIR record, specforge_dropped when a covering item does not, "
                "partially_absent or docling_absent when tokens have no match"
            ),
            "reachability": (
                "keyed on the (source_ref, text) pair, because a bounded-memory ingest writes one "
                "converter document per page range and self_ref collides across ranges"
            ),
        },
        "documents": records,
    }

    if args.plan_only:
        plan = dict(report)
        plan["documents"] = [
            {key: value for key, value in record.items() if key != "stage_source"}
            for record in records
        ]
        json.dump(plan, sys.stdout, indent=2)
        sys.stdout.write("\n")
        return 0

    if args.compare_only:
        if not output_absolute.is_dir():
            raise ValueError(f"output root does not exist: {args.output_root}")
    elif output_absolute.exists():
        raise ValueError(f"output root already exists: {args.output_root}")
    else:
        output_absolute.mkdir(parents=True)

    measurable = [record for record in records if record["stratum"] == "live_measured"]

    if args.oracle:
        # The oracle needs no ingest: it re-reads the exact serialized document each persisted
        # artifact was built from and asks docling-core which items its own traversal yields.
        for record in records:
            record.pop("stage_source", None)
            key = record["document_key"]
            bundle = (
                ROOT / census.PERSISTED_SOURCE_ROOT / key / "normalized" / f"{key}.backend.json"
            )
            if not bundle.is_file():
                record["stratum"] = "live_unmeasurable"
                record["reason"] = (
                    "no retained normalized bundle, so the converter document this artifact was "
                    f"built from is not available: {bundle.relative_to(ROOT).as_posix()}"
                )
                continue
            print(f"observing traversal of {key}", file=sys.stderr, flush=True)
            record["converter_bundle"] = bundle.relative_to(ROOT).as_posix()
            # The probe runs to completion before this process loads the same bundle. Both hold the
            # whole converter document in memory and the largest is 300 MiB on disk, so overlapping
            # them would double a peak the observation itself does not need: what comes back is a
            # list of refs.
            try:
                observation = observe_traversal(bundle)
            except ValueError as error:
                record["stratum"] = "live_unmeasurable"
                record["reason"] = str(error)
                continue
            persisted = census.read_json(ROOT / Path(record["persisted_artifact"]))
            record["traversal_oracle"] = traversal_oracle(
                ConverterDocument(bundle), observation, persisted
            )
        report["mode"] = "oracle"
        report["result"] = oracle_result(records)
        report_path = output_absolute / "ingest_traversal_oracle.json"
        report_path.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
        json.dump(report["result"], sys.stdout, indent=2)
        sys.stdout.write(f"\nreport: {(args.output_root / report_path.name).as_posix()}\n")
        return 0 if report["result"]["model_confirmed"] else 1

    if args.persisted:
        # The conservation census is defined on a (converter document, SourceIR) pair, and the
        # persisted bundle is exactly such a pair. Running it there needs no ingest and answers a
        # different question from the rest of this producer: whether the gap is drift, or whether
        # the artifacts the corpus already stands on carry it too.
        for record in records:
            record.pop("stage_source", None)
            key = record["document_key"]
            bundle = (
                ROOT / census.PERSISTED_SOURCE_ROOT / key / "normalized" / f"{key}.backend.json"
            )
            if not bundle.is_file():
                record["stratum"] = "live_unmeasurable"
                record["reason"] = (
                    "no retained normalized bundle, so the converter document this artifact was "
                    f"built from is not available: {bundle.relative_to(ROOT).as_posix()}"
                )
                continue
            persisted = census.read_json(ROOT / Path(record["persisted_artifact"]))
            record["converter_bundle"] = bundle.relative_to(ROOT).as_posix()
            record["conservation"] = conservation_census(
                ConverterDocument(bundle), SourceIrIndex(persisted)
            )
        report["mode"] = "persisted"
        report["result"] = persisted_result(records)
        report_path = output_absolute / "ingest_content_loss_persisted.json"
        report_path.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
        json.dump(report["result"], sys.stdout, indent=2)
        sys.stdout.write(f"\nreport: {(args.output_root / report_path.name).as_posix()}\n")
        return 0

    for position, record in enumerate(measurable, 1):
        key = record["document_key"]
        source_path = Path(record["source_path"])
        staged = census.stage_source(record, copy=not args.compare_only)
        record.pop("stage_source", None)
        record["source_sha256"] = census.sha256(staged)
        record["staged_source_bytes"] = staged.stat().st_size
        if record["staged_source_bytes"] != record["recorded_source_bytes"]:
            record["stratum"] = "live_unmeasurable"
            record["reason"] = (
                f"resolved source is {record['staged_source_bytes']} bytes but the persisted "
                f"artifact records {record['recorded_source_bytes']}; this is a different input"
            )
            continue

        replay_root = args.output_root / "replays" / key
        replay_base_root = (replay_root / "source_ir").as_posix()
        replayed_artifact = Path(replay_base_root) / key / "source_ir.json"
        if args.compare_only:
            if not (ROOT / replayed_artifact).is_file():
                record["stratum"] = "live_unmeasurable"
                record["reason"] = f"--compare-only found no replay artifact at {replayed_artifact}"
                continue
            print(f"[{position}/{len(measurable)}] adjudicating {key}", file=sys.stderr, flush=True)
        else:
            command = census.replay_command(source_path, replay_root)
            print(f"[{position}/{len(measurable)}] re-ingesting {key}", file=sys.stderr, flush=True)
            try:
                replay_report = census.run_json(command)
            except subprocess.CalledProcessError as error:
                record["stratum"] = "live_unmeasurable"
                record["reason"] = (
                    f"re-ingest failed with return code {error.returncode}: "
                    + " ".join((error.stderr or "").split())[-400:]
                )
                continue
            if replay_report.get("document_key") != key:
                raise ValueError(f"replay report identity mismatch for {key}")
            record["replay_command"] = shlex.join(command)

        replayed = census.normalized_replay_artifact(ROOT / replayed_artifact, replay_base_root)
        persisted = census.read_json(ROOT / Path(record["persisted_artifact"]))
        bundle = ROOT / Path(replay_base_root) / key / "normalized" / f"{key}.backend.json"
        document = ConverterDocument(bundle)
        index = SourceIrIndex(replayed)

        flagged = absent_elements(persisted, replayed)
        expected = census.content_drift_shape(persisted, replayed)["removed_text_absent_from_replay"]
        if len(flagged) != expected:
            raise ValueError(
                f"{key}: adjudicated {len(flagged)} absent elements but the census reports {expected}"
            )
        record["replayed_artifact"] = replayed_artifact.as_posix()
        record["converter_bundle"] = bundle.relative_to(ROOT).as_posix()
        record["census_absent_count"] = expected
        record["adjudications"] = [adjudicate(element, document, index) for element in flagged]
        record["conservation"] = conservation_census(document, index)

    verdicts: dict[str, int] = {}
    for record in records:
        for entry in record.get("adjudications", []):
            verdicts[entry["verdict"]] = verdicts.get(entry["verdict"], 0) + 1
    measured = [record for record in records if "conservation" in record]
    report["result"] = {
        "documents_measured": len(measured),
        "elements_adjudicated": sum(len(r.get("adjudications", [])) for r in records),
        "verdicts": dict(sorted(verdicts.items())),
        "converter_text_items": sum(r["conservation"]["converter_text_items"] for r in measured),
        "converter_items_with_no_source_ir_record": sum(
            r["conservation"]["reached_no_source_ir_record"] for r in measured
        ),
    }

    report_path = output_absolute / "ingest_content_loss.json"
    report_path.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    json.dump(report["result"], sys.stdout, indent=2)
    sys.stdout.write(f"\nreport: {(args.output_root / report_path.name).as_posix()}\n")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, census.CommandFailure, RuntimeError) as error:
        print(f"error: {error}", file=sys.stderr)
        sys.exit(1)
