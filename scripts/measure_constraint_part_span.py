#!/usr/bin/env python3
"""Census which SPAN of its statement each published signal constraint read its parts from.

`EXTRACTION-QUALITY-GAUGE.3k`.

A `SignalConstraintRecord` is assembled from several parts — subject, kind, bound value,
condition, negation — and each part is read from some span of the statement that produced the
record. When two parts are read from DIFFERENT spans, the published record is a composite of
clauses the document never joined. `.3i` proved that for the negation (`The DV operand must be 1
for IODIR` was published NEGATED because a later sentence said `must not`); this census measures
the same question for the KIND.

**The population of a kind-span change is the population of the CLASSIFIER, not of the record
table.** `classify_signal_constraint_kind` has exactly two callers, and only one of them reads the
whole statement:

  * `extract_signal_constraints`                    (`sigcon_*`)      — reads the WHOLE statement
  * `extract_signal_description_row_constraints`    (`row_sigcon_*`)  — reads one clause already

The other two producers of the same record type never reach the classifier at all:

  * `extract_dynamic_signal_constraints` (`dyn_sigcon_*`) types the record from its VALUE BINDER
    (`extract_discovered_state_value_from_text` / `logic_level_binding_kind_from_text`);
  * `crates/specforge/src/ir/constraint_extract_llm.rs` (`llm_sigcon_*`) takes the kind the model
    NAMED and parses it (`parse_kind`).

Comparing "classify over the clause" with "classify over the whole text" for those two strata
therefore measures a function that does not run on them. This census reports every stratum, but it
reports them SEPARATELY and labels the two that are out of the classifier's reach, so the number
can never again be read as the population a classifier change would move.

`--check` fails closed when that call-site topology changes: if `classify_signal_constraint_kind`
gains, loses, or moves a caller, the stratification above is stale and the census must be
re-derived rather than re-run.

This census classifies a POPULATION by mirroring the extractor's rule in Python. It is NOT a check
on the Rust implementation: mirror and original agreeing carries no information
(`CLAIM_VERIFICATION.md` section 2). The implementation's evidence is its own controls plus the
rebuilt-artifact diff.

Read-only and deterministic: no network, no clock, no randomness, no write, no rebuild.
Boundary: the persisted `generated/evidence_ir/*/evidence_ir.json` corpus plus the two call sites
in `crates/specforge/src/ir/evidence.rs`.

Usage:
    python3 scripts/measure_constraint_part_span.py
    python3 scripts/measure_constraint_part_span.py --json
    python3 scripts/measure_constraint_part_span.py --check
    python3 scripts/measure_constraint_part_span.py --self-test
"""

from __future__ import annotations

import argparse
import collections
import glob
import json
import os
import re
import sys

EVIDENCE_SOURCE = "crates/specforge/src/ir/evidence.rs"
CLASSIFIER = "classify_signal_constraint_kind"
# The gateway every PRODUCER now goes through (`EXTRACTION-QUALITY-GAUGE.3k.2a` for the statement
# path, `.3k.2e` for the table-row path). The bare classifier below is its implementation.
CLASSIFIER_ENTRY = "classify_signal_constraint_kind_typed"

# The call-site topology this census's stratification depends on: which enclosing function reaches
# the kind classifier, and whether that call is already narrowed to one obligation clause. A
# literal, declared independently of what the source scan finds, so a new caller fails the check
# instead of being silently folded into an existing stratum.
#
# Re-derived by `EXTRACTION-QUALITY-GAUGE.3k.2e`, and the re-derivation is the point rather than the
# edit. Read from each revision's own source with that revision's own scanner, this check has been
# RED since `.3k.2a` — which introduced the typed gateway, so the bare classifier's only production
# caller became the wrapper and the declared pair stopped matching. `.3k.2a`, `.3k.2b` and `.3k.2c`
# each shipped over it because nothing runs this script: it is named in `.3k`'s verification and is
# in no driver. A fail-closed check nothing executes is not a check
# (`EXTRACTION-QUALITY-GAUGE.3k.2j`).
#
# Two facts are pinned, because the stratification needs both: which producers reach the classifier
# and with what span, and that the untyped arm has exactly one way in.
EXPECTED_CLASSIFIER_CALLERS = {
    "extract_signal_description_row_constraints": "clause",
    "extract_signal_constraints": "whole",
}
EXPECTED_UNTYPED_CALLERS = {CLASSIFIER_ENTRY}

# Constraint-id prefix -> (stratum label, does this producer reach the kind classifier?).
PRODUCERS = {
    "sigcon": ("pattern statement path (extract_signal_constraints)", True),
    "row_sigcon": ("table-row path (extract_signal_description_row_constraints)", True),
    "dyn_sigcon": ("dynamic value-binding path (extract_dynamic_signal_constraints)", False),
    "llm_sigcon": ("LLM-primary path (constraint_extract_llm.rs)", False),
}

# --- mirrors of the extractor's own grammar (evidence.rs) ---------------------------------------

# `constraint_bearing_sentence` splits on exactly these, and returns the first part with a modal.
CLAUSE_SPLIT = re.compile(r"[.;•\n]")
MODALS = ("must", "shall")

# `classify_signal_constraint_kind`, arm by arm and in its own order. The arm label matters as much
# as the kind: two arms can both yield `must_be_stable`, and only one of them is a phrase the
# document actually wrote.
CLASSIFIER_ARMS = (
    ("must_not_change", "must_not_change", (
        "must not change", "shall not change", "must not be changed",
        "shall not be changed", "must remain stable", "shall remain stable",
    )),
    ("must_be_stable", "stable_phrase", (
        "must be stable", "shall be stable", "must hold", "shall hold",
    )),
    ("must_be_high", "must_be_high", (
        "must be high", "shall be high", "must remain high", "shall remain high",
        "must be driven high",
    )),
    ("must_be_low", "must_be_low", (
        "must be low", "shall be low", "must remain low", "shall remain low",
        "must be driven low",
    )),
    ("must_be_asserted", "must_be_asserted", (
        "must be asserted", "shall be asserted", "must remain asserted", "shall remain asserted",
    )),
    ("must_be_deasserted", "must_be_deasserted", (
        "must be deasserted", "shall be deasserted", "must remain deasserted",
        "shall remain deasserted", "must not be asserted", "shall not be asserted",
        "must not be active", "shall not be active",
    )),
)
VALID_PHRASES = ("must be valid", "shall be valid")
VALUE_BINDERS = ("must be ", "shall be ", "must remain ", "shall remain ")
VALUE_FILLERS = frozenset(
    ("a", "an", "the", "set", "driven", "to", "equal", "held", "kept", "in", "at", "its")
)
NEGATORS = ("must not", "shall not", "must never", "shall never", "cannot", "will not")
# `.3d` already refuses an inter-operand EQUALITY. A comparative MAGNITUDE whose right operand is a
# reference rather than a literal is the same shape one relation along, and has no typed slot
# either; these are the phrases that introduce it.
MAGNITUDE_PHRASES = ("greater than", "less than", "larger than", "smaller than", "more than")
# What makes the right operand a REFERENCE rather than a literal: the magnitude is stated against
# something the document names elsewhere, not against a number.
REFERENCE_OPERAND_PHRASES = (
    "than the size indicated by",
    "than the value of",
    "than the value indicated by",
    "than the number of",
    "than the size of",
    "than that indicated by",
)
# Both kind arms that publish `must_be_stable` without the document ever writing a stability
# phrase: the constraint is untyped and the kind is a default.
UNTYPED_DEFAULT_ARMS = ("untyped_default", "valid_no_value")

# Deleting a self-test case must make this script fail. The expected total is a literal declared
# here, independently of `SELF_TEST_CASES`, so dropping a case drops the executed count below it
# (`PRODUCTION-GRAPH-CENSUS-PIN.3`).
EXPECTED_SELF_TEST_CASES = 10


def repo_root() -> str:
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def constraint_bearing_sentence(text: str) -> str:
    for sentence in CLAUSE_SPLIT.split(text):
        lowered = sentence.lower()
        if any(modal in lowered for modal in MODALS):
            return sentence
    return text


def extract_protocol_state_value(lowered: str) -> str | None:
    for binder in VALUE_BINDERS:
        position = lowered.find(binder)
        if position < 0:
            continue
        rest = lowered[position + len(binder):]
        for word in re.split(r"[^a-z0-9_]", rest):
            if word and word not in VALUE_FILLERS:
                return word.upper()
    return None


def classify_kind(lowered: str) -> tuple[str, str]:
    """Return `(published kind, arm label)` exactly as `classify_signal_constraint_kind` would."""
    for kind, arm, phrases in CLASSIFIER_ARMS:
        if any(phrase in lowered for phrase in phrases):
            return (kind, arm)
    if any(phrase in lowered for phrase in VALID_PHRASES):
        value = extract_protocol_state_value(lowered)
        if value is not None:
            return (f"must_be_value:{value}", "valid_value")
        return ("must_be_stable", "valid_no_value")
    value = extract_protocol_state_value(lowered)
    if value is not None:
        return (f"must_be_value:{value}", "generic_value")
    return ("must_be_stable", "untyped_default")


def obligation_is_negated(lowered: str) -> bool:
    return any(negator in lowered for negator in NEGATORS)


def is_reference_magnitude(lowered: str) -> bool:
    """A comparative magnitude stated against a named operand rather than a literal."""
    return any(phrase in lowered for phrase in MAGNITUDE_PHRASES) and any(
        phrase in lowered for phrase in REFERENCE_OPERAND_PHRASES
    )


# --- call-site topology --------------------------------------------------------------------------


def _test_module_lines(lines: list[str]) -> set[int]:
    """Line numbers (0-based) that belong to a `#[cfg(test)] mod … { … }` block.

    A test caller must not fail this check — a new control is added on almost every slice, and a
    gate that fires on its own tests is a gate people stop running, which is exactly what happened
    here. Production callers still fail closed. The block is bounded by the closing brace at the
    `mod` line's own indentation, which rustfmt guarantees for this source.
    """
    inside: set[int] = set()
    index = 0
    while index < len(lines):
        if lines[index].strip() == "#[cfg(test)]":
            item = index + 1
            while item < len(lines) and not lines[item].strip():
                item += 1
            # The attribute guards exactly the item that follows it — a `mod`, or a single helper
            # `fn`. Scanning ahead for the next `mod` instead would swallow every line between a
            # `#[cfg(test)] fn` and the next module, which is most of this file.
            head = lines[item].lstrip() if item < len(lines) else ""
            if head.startswith(("mod ", "fn ", "pub fn ", "pub(super) fn ", "pub(crate) fn ")):
                indent = len(lines[item]) - len(lines[item].lstrip())
                closing = " " * indent + "}"
                end = item + 1
                while end < len(lines) and lines[end].rstrip("\n") != closing:
                    end += 1
                inside.update(range(index, min(end + 1, len(lines))))
                index = end + 1
                continue
        index += 1
    return inside


def classifier_call_sites(root: str, symbol: str = CLASSIFIER_ENTRY) -> dict[str, str]:
    """Map each enclosing PRODUCTION function that calls `symbol` to the span it passes.

    The span is read from the call's own argument: a call whose argument is derived from
    `constraint_bearing_sentence` (or from an already-narrowed `clause`) is clause-scoped; anything
    else reads whatever the enclosing function calls the whole statement.
    """
    path = os.path.join(root, EVIDENCE_SOURCE)
    with open(path, "r", encoding="utf-8") as handle:
        lines = handle.readlines()
    test_lines = _test_module_lines(lines)
    definition = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?fn\s+([A-Za-z0-9_]+)")
    sites: dict[str, str] = {}
    enclosing = "<file scope>"
    pending_binding = ""
    for number, line in enumerate(lines):
        function = definition.match(line)
        if function:
            enclosing = function.group(1)
            pending_binding = ""
        stripped = line.strip()
        if stripped.startswith("let lowered"):
            pending_binding = stripped
        if f"{symbol}(" not in line or function or number in test_lines:
            continue
        argument = line.split(f"{symbol}(", 1)[1]
        source = argument + " " + pending_binding
        scope = "clause" if ("constraint_bearing_sentence" in source or "clause" in source) else "whole"
        sites[enclosing] = scope
    return sites


def untyped_classifier_callers(root: str) -> set[str]:
    """Production functions that reach the UNTYPED classifier directly, bypassing the gateway."""
    callers = set(classifier_call_sites(root, CLASSIFIER))
    # The bare name is a prefix of the typed one, so a `…_typed(` call matches `…kind(` only when
    # the paren follows immediately; `classifier_call_sites` splits on `f"{symbol}("`, which already
    # enforces that. The wrapper itself is the one legitimate caller.
    return callers


# --- census ---------------------------------------------------------------------------------------


def census(root: str) -> dict:
    paths = sorted(glob.glob(os.path.join(root, "generated", "evidence_ir", "*", "evidence_ir.json")))
    strata: dict[str, dict] = {
        prefix: {
            "label": label,
            "reaches_classifier": reaches,
            "records": 0,
            "kind_span_differs": [],
            "negation_on_untyped_default": [],
            "reference_magnitude": [],
        }
        for prefix, (label, reaches) in PRODUCERS.items()
    }
    unknown: collections.Counter = collections.Counter()
    documents = 0

    for path in paths:
        documents += 1
        document = os.path.basename(os.path.dirname(path))
        with open(path, "r", encoding="utf-8") as handle:
            evidence = json.load(handle)
        for record in evidence.get("signal_constraints", []):
            constraint_id = record["constraint_id"]
            prefix = re.sub(r"_\d+$", "", constraint_id)
            if prefix not in strata:
                unknown[prefix] += 1
                continue
            stratum = strata[prefix]
            stratum["records"] += 1

            text = record["source_text"]
            clause = constraint_bearing_sentence(text)
            whole_kind, whole_arm = classify_kind(text.lower())
            clause_kind, clause_arm = classify_kind(clause.lower())
            entry = {
                "document": document,
                "constraint_id": constraint_id,
                "subject_signal": record.get("subject_signal"),
                "published_kind": record.get("constraint_kind"),
                "whole_kind": whole_kind,
                "whole_arm": whole_arm,
                "clause_kind": clause_kind,
                "clause_arm": clause_arm,
                "clause": clause.strip(),
            }
            if whole_kind != clause_kind:
                stratum["kind_span_differs"].append(entry)
            if obligation_is_negated(clause.lower()) and clause_arm in UNTYPED_DEFAULT_ARMS:
                stratum["negation_on_untyped_default"].append(entry)
            if is_reference_magnitude(clause.lower()):
                stratum["reference_magnitude"].append(entry)

    return {
        "documents": documents,
        "records": sum(s["records"] for s in strata.values()),
        "strata": strata,
        "unknown_prefixes": dict(unknown),
    }


def report(result: dict, sites: dict[str, str]) -> None:
    print(f"constraint-part-span census over {result['documents']} persisted evidence artifacts")
    print(f"  {result['records']} signal-constraint records")
    print()
    print("kind-classifier call sites (the population a kind-span change can move):")
    for name, scope in sorted(sites.items()):
        print(f"  {name:<48} reads the {scope}")
    print()
    for prefix, stratum in sorted(result["strata"].items()):
        reaches = stratum["reaches_classifier"]
        reach = "reaches the kind classifier" if reaches else (
            "DOES NOT reach the kind classifier — every classifier-derived row below is a MIRROR "
            "ARTIFACT, not a population"
        )
        # A row this producer's kind never passes through cannot be a population for a change to
        # that classifier. Marking it is the whole point of stratifying: `.3k` opened on a count
        # that had been read off strata the classifier never runs on.
        mark = "" if reaches else "[n/a] "
        print(f"[{prefix}_*] {stratum['label']}")
        print(f"    {stratum['records']} records — {reach}")
        print(f"    {mark}kind differs clause vs whole : {len(stratum['kind_span_differs'])}")
        print(f"    {mark}negation on untyped default  : {len(stratum['negation_on_untyped_default'])}")
        print(f"    reference-operand magnitude  : {len(stratum['reference_magnitude'])}")
        for key in ("kind_span_differs", "negation_on_untyped_default", "reference_magnitude"):
            prefix_mark = "" if reaches or key == "reference_magnitude" else "[n/a] "
            for entry in stratum[key]:
                print(f"      · {prefix_mark}{key} {entry['document']} {entry['constraint_id']} "
                      f"subject={entry['subject_signal']} "
                      f"whole={entry['whole_kind']}({entry['whole_arm']}) "
                      f"clause={entry['clause_kind']}({entry['clause_arm']})")
                print(f"        clause: {entry['clause'][:200]}")
        print()
    if result["unknown_prefixes"]:
        print(f"unknown constraint-id prefixes: {result['unknown_prefixes']}")


# --- self-test -------------------------------------------------------------------------------------

# Each case: (id, callable, expected). The mirrors are the only thing worth testing here — the
# census itself is a fold over them.
SELF_TEST_CASES = (
    (
        "clause-is-the-first-modal-sentence",
        lambda: constraint_bearing_sentence(
            "PREADY is asserted. PADDR must be stable. HSELx must be asserted."
        ).strip(),
        "PADDR must be stable",
    ),
    (
        "clause-falls-back-to-the-whole-text-without-a-modal",
        lambda: constraint_bearing_sentence("PSEL is tied HIGH"),
        "PSEL is tied HIGH",
    ),
    (
        "kind-phrase-in-a-later-sentence-is-outside-the-clause",
        lambda: (
            classify_kind("when the subordinate is selected, it must also monitor hreadyx")[1],
            classify_kind(
                "when the subordinate is selected, it must also monitor hreadyx. "
                "zetaselx must be asserted in the same cycle"
            )[1],
        ),
        ("untyped_default", "must_be_asserted"),
    ),
    (
        "negated-spelling-types-the-obligation",
        lambda: classify_kind("zetastrb must not be asserted during a read")[0],
        "must_be_deasserted",
    ),
    (
        "stability-phrase-is-not-the-untyped-default",
        lambda: classify_kind("zetaaddr must be stable")[1],
        "stable_phrase",
    ),
    (
        "value-binder-lifts-the-bound-word-not-a-filler",
        lambda: extract_protocol_state_value("the zeta field shall be set to the reserved value"),
        "RESERVED",
    ),
    (
        "negation-is-read-from-whatever-span-it-is-given",
        lambda: (
            obligation_is_negated("the zeta operand must be 1 for zetadir"),
            obligation_is_negated("a command must not be issued while the queue is full"),
        ),
        (False, True),
    ),
    (
        "reference-operand-magnitude-is-recognized",
        lambda: is_reference_magnitude(
            "the range given by this field must not be greater than the size indicated by the oas field"
        ),
        True,
    ),
    (
        "literal-operand-magnitude-is-not-a-reference-magnitude",
        lambda: is_reference_magnitude("this field must be greater than 0"),
        False,
    ),
    # `EXTRACTION-QUALITY-GAUGE.3k.2e` — the attribute guards the ITEM that follows it. Scanning
    # ahead for the next `mod` swallowed every line between a `#[cfg(test)] fn` and the next test
    # module, which silently emptied the topology instead of failing closed.
    (
        "cfg-test-marks-the-item-it-guards-not-everything-until-the-next-mod",
        lambda: sorted(
            _test_module_lines(
                [
                    "#[cfg(test)]\n",
                    "pub(super) fn helper() -> u8 {\n",
                    "    0\n",
                    "}\n",
                    "fn production() -> u8 {\n",
                    "    1\n",
                    "}\n",
                    "#[cfg(test)]\n",
                    "mod tests {\n",
                    "    fn control() {}\n",
                    "}\n",
                ]
            )
        ),
        [0, 1, 2, 3, 7, 8, 9, 10],
    ),
)


def self_test() -> int:
    passed = 0
    for case_id, run, expected in SELF_TEST_CASES:
        actual = run()
        if actual != expected:
            print(f"self-test FAIL {case_id}: expected {expected!r}, got {actual!r}")
            return 1
        passed += 1
    if passed != EXPECTED_SELF_TEST_CASES:
        print(
            f"self-test FAIL: ran {passed} cases, expected {EXPECTED_SELF_TEST_CASES} — "
            "a case was removed; restore it or move the declared total deliberately."
        )
        return 1
    print(f"constraint-part-span: {passed}/{EXPECTED_SELF_TEST_CASES} mirror cases pass.")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    parser.add_argument(
        "--check",
        action="store_true",
        help="fail when the kind-classifier call-site topology no longer matches this census",
    )
    parser.add_argument("--self-test", action="store_true", help="run the mirror cases only")
    args = parser.parse_args()

    if args.self_test:
        return self_test()

    root = repo_root()
    sites = classifier_call_sites(root)

    if args.check:
        untyped = untyped_classifier_callers(root)
        if sites != EXPECTED_CLASSIFIER_CALLERS:
            print(
                "constraint-part-span: the kind classifier's call sites moved.\n"
                f"  expected {EXPECTED_CLASSIFIER_CALLERS}\n"
                f"  found    {sites}\n"
                "This census stratifies by which producer reaches the classifier; re-derive it."
            )
            return 1
        if untyped != EXPECTED_UNTYPED_CALLERS:
            print(
                "constraint-part-span: a producer reaches the UNTYPED classifier directly.\n"
                f"  expected {EXPECTED_UNTYPED_CALLERS}\n"
                f"  found    {untyped}\n"
                "The terminal arm publishes a kind no document stated; every producer must go "
                "through the typed gateway (EXTRACTION-QUALITY-GAUGE.3k.2a/.3k.2e)."
            )
            return 1
        print(
            "constraint-part-span: kind-classifier call sites unchanged "
            f"({len(sites)} producers, {sum(1 for s in sites.values() if s == 'whole')} reading the "
            f"whole statement; the untyped arm is reached only by {sorted(untyped)[0]})."
        )
        return 0

    result = census(root)
    if args.json:
        json.dump({"call_sites": sites, **result}, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return 0
    report(result, sites)
    return 0


if __name__ == "__main__":
    sys.exit(main())
