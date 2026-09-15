#!/usr/bin/env python3
"""Re-pin `line_range_sha256` regions BY CONTENT after a governed file changes.

`CLAIM-VERIFICATION-ADOPTION.12`. Three registries pin claim evidence to a one-based
line range plus a SHA-256 of exactly those lines:

  * `doctrine/claim_verification/current_claim_census.jsonl`
  * `doctrine/claim_verification/book_quantitative_claims.jsonl`
  * `doctrine/claim_verification/published_assertions.jsonl`   <- the one that gets forgotten
  * `doctrine/claim_verification/claims.jsonl`                 <- added by LIVE-DOCUMENT-PRESSURE-HEADROOM.22e

Three record SHAPES carry those pins, and the first version of this tool saw only one of them, so
editing a checker script moved seven regions while `--check` reported `unchanged`:

  * `{"path": ..., "region": {...}}`                     - a governed document line range
  * `{"control": {"red_case": {"path": ..., ...}}}`      - a RED case pinned INSIDE a checker script
  * `{"red_evidence": {"source_region": {...}}}`         - the same, with the file named by the
                                                           enclosing control's `producer`/`inputs[0]`

The last two pin line ranges inside the gate scripts themselves, so ANY edit to a checker shifts
them — a class the first two registries never exercised.

Any slice that prepends to a rolling ledger or inserts into a governed file shifts every
row below the edit, and the repair has been hand work or a throwaway script every time.
Measured over the tracked tree: **562 pinned regions across 63 files**.

**The hazard this instrument exists to refuse.** A re-pin that lands on the WRONG line is
invisible, because the digest it was moved to match is the digest it now has. The specific
case is not hypothetical: a region whose recorded content is a single blank line has digest
`01ba4719…546b`, which matches EVERY blank line in the file — and `docs/book/src/reference/
live-docs.md` alone holds **280** of them behind **163** pins. `CLAIM-VERIFICATION-ADOPTION.8`
recorded two rows that had already drifted onto blank lines exactly this way.

So this tool **reports ambiguity rather than resolving it, and refuses rather than guessing**:

  `moved`      exactly one location in the file has the recorded digest -> re-pin to it.
  `unchanged`  the recorded range already carries the recorded digest   -> leave it alone.
  `AMBIGUOUS`  more than one location matches -> REFUSED, every candidate line printed.
  `ABSENT`     no location matches; the content genuinely left the file -> REFUSED.
  `NO FILE`    the pinned path does not exist -> REFUSED.

A refusal is a human decision, not a failure to try harder. Re-pinning an ambiguous row is
how a claim silently comes to cite different text than it was verified against.

Only records this tool actually moves are rewritten; every other line is preserved byte for
byte, so a diff shows the re-pins and nothing else.

Usage:
    python3 scripts/repin_claim_regions.py --check              # report, write nothing
    python3 scripts/repin_claim_regions.py --apply              # re-pin the unambiguous
    python3 scripts/repin_claim_regions.py --apply --path FILE  # limit to one governed file
    python3 scripts/repin_claim_regions.py --self-test          # the RED matrix
"""
import argparse
import hashlib
import json
import os
import sys
import tempfile

REGISTRIES = [
    "doctrine/claim_verification/current_claim_census.jsonl",
    "doctrine/claim_verification/book_quantitative_claims.jsonl",
    "doctrine/claim_verification/published_assertions.jsonl",
    "doctrine/claim_verification/claims.jsonl",
]
BLANK_LINE_DIGEST = hashlib.sha256(b"\n").hexdigest()
# Declared beside the suite so the matrix cannot silently shrink; re-derive it, never guess.
EXPECTED_SELF_TEST_CASES = 19


def read_lines(path, cache):
    """The file's lines without the trailing empty element, so 1-based indexing is exact."""
    if path not in cache:
        text = open(path, encoding="utf-8").read().split("\n")
        if text and text[-1] == "":
            text.pop()
        cache[path] = text
    return cache[path]


def region_digest(lines, start, end):
    """SHA-256 of lines `start..end` inclusive, one-based, each with its newline."""
    return hashlib.sha256(
        "".join(line + "\n" for line in lines[start - 1 : end]).encode()
    ).hexdigest()


def locate(lines, span, wanted):
    """EVERY start line whose `span`-line window has the wanted digest. Never just the first."""
    return [
        start
        for start in range(1, len(lines) - span + 1)
        if region_digest(lines, start, start + span) == wanted
    ]


def walk(node, visit):
    if isinstance(node, dict):
        visit(node)
        for value in node.values():
            walk(value, visit)
    elif isinstance(node, list):
        for value in node:
            walk(value, visit)


def is_pin(region):
    """A pinned region carries a one-based span and the digest of exactly those lines.

    `kind` is absent on the `red_case` shape, so it is checked only when present rather
    than required — requiring it is what made the first version blind to two shapes.
    """
    return (
        isinstance(region, dict)
        and isinstance(region.get("start_line"), int)
        and isinstance(region.get("end_line"), int)
        and isinstance(region.get("sha256"), str)
        and region.get("kind", "line_range_sha256") == "line_range_sha256"
    )


def pinned_regions(node):
    """Every `(region, path)` this node owns directly, across all three record shapes.

    Each shape names its file differently, and the difference is the whole reason this is
    explicit rather than a generic search: a region whose file is guessed wrong resolves
    against the wrong text, which is the failure mode the tool exists to refuse.
    """
    found = []
    region = node.get("region")
    if is_pin(region) and isinstance(node.get("path"), str):
        found.append((region, node["path"]))
    red_case = node.get("red_case")
    if is_pin(red_case) and isinstance(red_case.get("path"), str):
        found.append((red_case, red_case["path"]))
    evidence = node.get("red_evidence")
    if isinstance(evidence, dict) and is_pin(evidence.get("source_region")):
        # The control that owns the evidence names the file it was measured in.
        source = node.get("producer")
        if not isinstance(source, str):
            inputs = node.get("inputs")
            source = inputs[0] if isinstance(inputs, list) and inputs else None
        if isinstance(source, str):
            found.append((evidence["source_region"], source))
    return found


def classify(region, path, cache, root):
    """One pinned region's verdict."""
    absolute = os.path.join(root, path)
    if not os.path.isfile(absolute):
        return {"verdict": "NO FILE", "path": path, "region": region}

    lines = read_lines(absolute, cache)
    start, end = region.get("start_line"), region.get("end_line")
    wanted = region.get("sha256")
    span = end - start
    if (
        1 <= start <= len(lines)
        and end <= len(lines)
        and region_digest(lines, start, end) == wanted
    ):
        return {"verdict": "unchanged", "path": path, "region": region}
    found = locate(lines, span, wanted)
    if not found:
        return {"verdict": "ABSENT", "path": path, "region": region, "found": []}
    if len(found) > 1:
        return {"verdict": "AMBIGUOUS", "path": path, "region": region, "found": found}
    return {"verdict": "moved", "path": path, "region": region, "found": found}


def process(root, only_path, apply_changes, out):
    cache, counts, refusals, moves = {}, {}, [], []
    for registry in REGISTRIES:
        registry_path = os.path.join(root, registry)
        if not os.path.isfile(registry_path):
            continue
        raw_lines = open(registry_path, encoding="utf-8").read().rstrip("\n").split("\n")
        rewritten, touched = [], 0
        for raw in raw_lines:
            if not raw.strip():
                rewritten.append(raw)
                continue
            record = json.loads(raw)
            pins = []
            walk(record, lambda node: pins.extend(pinned_regions(node)))
            changed = False
            for region, path in pins:
                verdict = classify(region, path, cache, root)
                if verdict is None:
                    continue
                if only_path and verdict["path"] != only_path:
                    continue
                counts[verdict["verdict"]] = counts.get(verdict["verdict"], 0) + 1
                if verdict["verdict"] in ("AMBIGUOUS", "ABSENT", "NO FILE"):
                    refusals.append({**verdict, "registry": registry})
                    continue
                if verdict["verdict"] == "moved":
                    start = verdict["found"][0]
                    span = region["end_line"] - region["start_line"]
                    moves.append(
                        {
                            **verdict,
                            "registry": registry,
                            "from": (region["start_line"], region["end_line"]),
                            "to": (start, start + span),
                        }
                    )
                    region["start_line"] = start
                    region["end_line"] = start + span
                    changed = True
            if changed:
                touched += 1
                rewritten.append(json.dumps(record, sort_keys=True, separators=(", ", ": ")))
            else:
                rewritten.append(raw)
        if apply_changes and touched and not refusals:
            handle, temporary = tempfile.mkstemp(
                dir=os.path.dirname(registry_path), suffix=".repin"
            )
            with os.fdopen(handle, "w", encoding="utf-8") as sink:
                sink.write("\n".join(rewritten) + "\n")
            os.replace(temporary, registry_path)
        print(f"{registry}: {touched} record(s) re-pinned", file=out)

    for move in moves:
        print(
            f"  move {move['path']} {move['from'][0]}-{move['from'][1]}"
            f" -> {move['to'][0]}-{move['to'][1]}  ({move['region']['sha256'][:12]})",
            file=out,
        )
    for refusal in refusals:
        detail = ""
        if refusal["verdict"] == "AMBIGUOUS":
            detail = f" candidates at lines {refusal['found']}"
            if refusal["region"]["sha256"] == BLANK_LINE_DIGEST:
                detail += " — this region is pinned to a BARE BLANK LINE"
        print(
            f"  REFUSED {refusal['verdict']}: {refusal['registry']} -> {refusal['path']}"
            f" {refusal['region']['start_line']}-{refusal['region']['end_line']}"
            f" ({refusal['region']['sha256'][:12]}){detail}",
            file=out,
        )

    summary = ", ".join(f"{key} {value}" for key, value in sorted(counts.items()))
    print(f"repin-claim-regions: {summary or 'no pinned regions in scope'}", file=out)
    if refusals:
        print(
            f"repin-claim-regions: REFUSED {len(refusals)} region(s) — a re-pin that lands on"
            " the wrong line is invisible, so these are yours to decide.",
            file=out,
        )
        if apply_changes:
            print(
                "repin-claim-regions: nothing was written; resolve the refusals first.",
                file=out,
            )
    return 1 if refusals else 0


def self_test(out):
    """The RED matrix the leaf requires: ambiguity, absence, a blank-line pin, a clean shift."""
    cases, failures = 0, 0

    def check(name, condition):
        nonlocal cases, failures
        cases += 1
        if not condition:
            failures += 1
            print(f"  FAIL {name}", file=out)

    lines = ["alpha", "beta", "", "gamma", ""]
    blank_span = hashlib.sha256(b"\n").hexdigest()

    # A blank line is ambiguous by construction: two candidates, and the tool must see both.
    check("blank line finds every candidate", locate(lines, 0, blank_span) == [3, 5])
    # A unique line resolves to exactly one.
    check("unique content resolves once", locate(lines, 0, region_digest(lines, 1, 1)) == [1])
    # Content that left the file resolves to none.
    check("absent content resolves to none", locate(lines, 0, hashlib.sha256(b"zeta\n").hexdigest()) == [])
    # A multi-line region keeps its span.
    two = region_digest(lines, 1, 2)
    check("multi-line region resolves once", locate(lines, 1, two) == [1])
    # A repeated multi-line window is ambiguous too, not just single blank lines.
    repeated = ["alpha", "beta", "gamma", "alpha", "beta"]
    check(
        "repeated multi-line window is ambiguous",
        locate(repeated, 1, region_digest(repeated, 1, 2)) == [1, 4],
    )
    # The digest convention is the one the registries use.
    check("blank-line digest matches the recorded constant", blank_span == BLANK_LINE_DIGEST)

    with tempfile.TemporaryDirectory() as root:
        registry_dir = os.path.join(root, "doctrine", "claim_verification")
        os.makedirs(registry_dir)
        governed = os.path.join(root, "governed.md")
        # A clean shift: two lines inserted above the pinned one.
        open(governed, "w", encoding="utf-8").write("new\nnew\nalpha\nbeta\n")
        pinned = hashlib.sha256(b"alpha\n").hexdigest()
        record = {
            "path": "governed.md",
            "region": {
                "kind": "line_range_sha256",
                "start_line": 1,
                "end_line": 1,
                "sha256": pinned,
            },
        }
        registry = os.path.join(registry_dir, "current_claim_census.jsonl")
        open(registry, "w", encoding="utf-8").write(json.dumps(record) + "\n")
        sink = open(os.devnull, "w")
        exit_code = process(root, None, True, sink)
        moved = json.loads(open(registry, encoding="utf-8").read().strip())
        check("a clean shift re-pins and exits zero", exit_code == 0)
        check("a clean shift lands on the new line", moved["region"]["start_line"] == 3)

        # An ambiguous pin: the same content twice, and the tool must write NOTHING.
        open(governed, "w", encoding="utf-8").write("alpha\nbeta\nalpha\n")
        record["region"]["start_line"] = record["region"]["end_line"] = 2
        before = json.dumps(record) + "\n"
        open(registry, "w", encoding="utf-8").write(before)
        exit_code = process(root, None, True, sink)
        check("an ambiguous pin refuses", exit_code == 1)
        check(
            "an ambiguous pin writes nothing",
            open(registry, encoding="utf-8").read() == before,
        )

        # Content that left the file: also a refusal, also no write.
        open(governed, "w", encoding="utf-8").write("beta\ngamma\n")
        open(registry, "w", encoding="utf-8").write(before)
        exit_code = process(root, None, True, sink)
        check("absent content refuses", exit_code == 1)
        check(
            "absent content writes nothing",
            open(registry, encoding="utf-8").read() == before,
        )

        # A pinned path that does not exist at all.
        os.remove(governed)
        exit_code = process(root, None, True, sink)
        check("a missing governed file refuses", exit_code == 1)
        # A refusal anywhere suppresses every write in the run, which is the all-or-nothing
        # property; clear the census fixture so the shape cases below measure themselves.
        os.remove(registry)

        # LIVE-DOCUMENT-PRESSURE-HEADROOM.22e — the two shapes the first version could not see.
        # Both pin line ranges INSIDE a checker script, so any edit to a gate shifts them, and
        # both were reported as `unchanged` while genuinely displaced.
        checker = os.path.join(root, "scripts")
        os.makedirs(checker)
        script = os.path.join(checker, "check_fixture.pl")
        open(script, "w", encoding="utf-8").write("head\nRED case line\n")
        red = hashlib.sha256(b"RED case line\n").hexdigest()

        # `control.red_case` carries its own path and no `kind` at all.
        assertions = os.path.join(registry_dir, "published_assertions.jsonl")
        record = {
            "assertion_id": "fixture",
            "control": {
                "red_case": {
                    "path": "scripts/check_fixture.pl",
                    "start_line": 1,
                    "end_line": 1,
                    "sha256": red,
                }
            },
        }
        open(assertions, "w", encoding="utf-8").write(json.dumps(record) + "\n")
        exit_code = process(root, None, True, sink)
        moved = json.loads(open(assertions, encoding="utf-8").read().strip())
        check("a red_case region without a kind is still a pin", exit_code == 0)
        check(
            "a red_case region re-pins to its new line",
            moved["control"]["red_case"]["start_line"] == 2,
        )
        os.remove(assertions)

        # `red_evidence.source_region` names no path; the enclosing control's producer does.
        claims = os.path.join(registry_dir, "claims.jsonl")
        record = {
            "claim_id": "fixture",
            "falsification": {
                "controls": [
                    {
                        "id": "fixture-control",
                        "producer": "scripts/check_fixture.pl",
                        "red_evidence": {
                            "source_region": {
                                "kind": "line_range_sha256",
                                "start_line": 1,
                                "end_line": 1,
                                "sha256": red,
                            }
                        },
                    }
                ]
            },
        }
        open(claims, "w", encoding="utf-8").write(json.dumps(record) + "\n")
        exit_code = process(root, None, True, sink)
        moved = json.loads(open(claims, encoding="utf-8").read().strip())
        region = moved["falsification"]["controls"][0]["red_evidence"]["source_region"]
        check("a source_region resolves through its control's producer", exit_code == 0)
        check("a source_region re-pins to its new line", region["start_line"] == 2)

        # With no producer, `inputs[0]` names the file instead.
        control = record["falsification"]["controls"][0]
        del control["producer"]
        control["inputs"] = ["scripts/check_fixture.pl"]
        control["red_evidence"]["source_region"]["start_line"] = 1
        control["red_evidence"]["source_region"]["end_line"] = 1
        open(claims, "w", encoding="utf-8").write(json.dumps(record) + "\n")
        exit_code = process(root, None, True, sink)
        moved = json.loads(open(claims, encoding="utf-8").read().strip())
        region = moved["falsification"]["controls"][0]["red_evidence"]["source_region"]
        check("a source_region falls back to inputs[0]", region["start_line"] == 2)

        # A control that names NO file is skipped rather than guessed at: resolving a region
        # against a file nobody named is the wrong-landing failure in a different disguise.
        del control["inputs"]
        control["red_evidence"]["source_region"]["start_line"] = 1
        control["red_evidence"]["source_region"]["end_line"] = 1
        before = json.dumps(record) + "\n"
        open(claims, "w", encoding="utf-8").write(before)
        exit_code = process(root, None, True, sink)
        check(
            "a source_region with no named file is left alone",
            open(claims, encoding="utf-8").read() == before,
        )
        sink.close()

    # PRODUCTION-GRAPH-CENSUS-PIN.3 — a running counter has nothing to compare itself against:
    # delete a case and the line simply reports one fewer. The expected total is declared here,
    # so a case removed, or one added and not declared, fails instead of shrinking the matrix.
    if cases != EXPECTED_SELF_TEST_CASES:
        print(
            f"repin-claim-regions: ran {cases} cases, declaration expects"
            f" {EXPECTED_SELF_TEST_CASES} — re-derive the declaration beside the suite",
            file=out,
        )
        failures += 1
    print(
        f"repin-claim-regions: self-test {cases - failures}/{cases}"
        " locate, refusal, and write-suppression cases pass.",
        file=out,
    )
    return 1 if failures else 0


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--check", action="store_true", help="report drift, write nothing")
    mode.add_argument("--apply", action="store_true", help="re-pin every unambiguous region")
    mode.add_argument("--self-test", action="store_true", help="run the RED matrix")
    parser.add_argument("--path", help="limit to one governed file (repository-relative)")
    args = parser.parse_args()

    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    if args.self_test:
        return self_test(sys.stdout)
    return process(root, args.path, args.apply, sys.stdout)


if __name__ == "__main__":
    sys.exit(main())
