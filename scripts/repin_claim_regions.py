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

## The second pin family: WHOLE-FILE digests (`CLAIM-VERIFICATION-ADOPTION.21`)

The registries pin two more digest classes, neither of which carries a line range, and until `.21`
neither had an instrument at all:

  * `durability.artifacts[].sha256` in `claims.jsonl`  - 107 pins over 69 paths
  * `{"record_type": "source", "path": ..., "sha256": ...}` in the other three - 6 pins over 3 paths

**This is the class ordinary work hits.** `MEMORY.md` carries three of them and `COMMIT.md` step 4 says
to overwrite it whenever the resume state changes, so a slice that touches nothing else still
invalidates three. `doctrine/live_document_size/surfaces.jsonl` carries four plus all three source
records, and registering a task-evidence part requires editing it. Two consecutive slices found this by
failing a gate and repaired it with a throwaway loop - the exact hand work `.12` exists to end.

**The hazard here is not ambiguity.** A whole-file digest either matches or it does not, so there is
nothing to resolve. Its hazard is that refreshing one silently asserts *"I have seen this change and
the claim still holds"*. So a refresh is REPORTED, with whether this working tree is the thing that
edited the path, rather than applied in silence.

**Convergence is two passes, and it is structural rather than lucky.** `claims.jsonl` watches the three
sibling registries and nothing watches `claims.jsonl`, so the dependency graph is an acyclic depth-2
DAG. Processing the registries in declaration order refreshes the sources before the artifacts that
watch them, and a verification pass then re-reads every whole-file pin: if any is still stale, the
graph has grown a cycle and the tool says so instead of looping.

  `refreshed`  the recorded digest is not the file's -> re-pin, and report it.
  `unchanged`  the recorded digest is the file's     -> leave it alone.
  `NO FILE`    the pinned path does not exist        -> REFUSED.

## `--accept-value`: the ABSENT region whose VALUE moved (`CLAIM-VERIFICATION-ADOPTION.22`)

A published count changes and its line is rewritten: `docs/knowledge/INDEX.md` line 3 goes from
*"343 fact cards"* to *"344 fact cards"*. The region is `ABSENT` and refusing is CORRECT, because only
a human can say the region is still the same region. What follows is three coupled edits — the
`value` in `published_assertions`, the digest in BOTH registries that pin that line, and the census
evidence identifier whose suffix must equal the digest, which `.17` had to gate after it was forgotten
twice. Measured: four consecutive slices in one session.

**The new value is DERIVED, never accepted.** A blanket `--accept` would bless whatever the line now
says — the ambiguity hazard above, one level up. The record already declares how to derive it
(`producer.argv` + `expected_exit` + `field`), so `--accept-value <path>` RUNS that producer, requires
the produced value to appear in the new line, and only then re-pins value, digest and identifier
together. Every other case stays a refusal:

  no `producer` declared            -> REFUSED (nothing to derive the value from)
  producer exits != `expected_exit` -> REFUSED
  produced value absent from the line -> REFUSED (the line is not carrying what the producer says)

This is strictly stronger than the hand repair it replaces, which never re-ran the producer at all.

**The SIBLING at the same line moves with it.** `docs/knowledge/INDEX.md` line 3 is pinned twice: by
the `published_assertions` record that declares the producer, and by a `current_claim_census` evidence
record that declares only a `verifier`. The census record can never derive a value of its own, so
accepting only the first would leave the repair half done — which is the half `.17` had to gate. A
region pinned to the SAME path, the SAME line range and the SAME old digest as one a producer just
verified is re-pinned with it, because it is evidence about the identical line and that line has just
been verified. It is recorded as `accept-value (sibling)` so the report says which is which.

Only records this tool actually moves are rewritten; every other line is preserved byte for
byte, so a diff shows the re-pins and nothing else.

Usage:
    python3 scripts/repin_claim_regions.py --check              # report, write nothing
    python3 scripts/repin_claim_regions.py --apply              # re-pin the unambiguous
    python3 scripts/repin_claim_regions.py --apply --path FILE  # limit to one governed file
    python3 scripts/repin_claim_regions.py --apply --accept-value docs/knowledge/INDEX.md
    python3 scripts/repin_claim_regions.py --self-test          # the RED matrix
"""
import argparse
import hashlib
import json
import os
import re
import subprocess
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
EXPECTED_SELF_TEST_CASES = 37


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


def whole_file_pins(node):
    """Every `(pin, path)` this node owns whose digest covers the WHOLE file, not a line range.

    The shape test is the absence of a span rather than a list of field names, because the two
    record shapes that carry these pins share nothing else: an artifact is
    `{path, role, sha256}` and a source record is `{path, record_type, schema_version, sha256,
    source_id}`. Requiring a named shape is what made the first version of this tool blind to two
    of the three line-range shapes, so the rule here is structural: a `path`, a `sha256`, and no
    `start_line`/`end_line` is a whole-file pin. Measured over the live registries on
    `2026-09-22`, that matches 113 pins and nothing else.
    """
    if (
        isinstance(node.get("path"), str)
        and isinstance(node.get("sha256"), str)
        and "start_line" not in node
        and "end_line" not in node
    ):
        return [(node, node["path"])]
    return []


def file_digest(path):
    """SHA-256 of the file's exact bytes. Read fresh every time.

    Deliberately uncached, unlike `read_lines`: `claims.jsonl` pins the three sibling registries
    that this same run rewrites, so a cached digest would be the pre-write one and the refresh
    would converge to the wrong value while reporting success.
    """
    with open(path, "rb") as handle:
        return hashlib.sha256(handle.read()).hexdigest()


def classify_whole_file(pin, path, root):
    """One whole-file pin's verdict. There is no ambiguity class here - see the module docstring."""
    absolute = os.path.join(root, path)
    if not os.path.isfile(absolute):
        return {"verdict": "NO FILE", "path": path, "region": pin, "whole_file": True}
    actual = file_digest(absolute)
    if actual == pin.get("sha256"):
        return {"verdict": "unchanged", "path": path, "region": pin, "whole_file": True}
    return {
        "verdict": "refreshed",
        "path": path,
        "region": pin,
        "whole_file": True,
        "actual": actual,
    }


def locally_modified(root):
    """The repository-relative paths this working tree has changed, for the refresh report.

    A refresh asserts that the author has seen the change, so the report distinguishes a path this
    slice edited from one that moved underneath it. Best effort: if Git is unavailable the report
    simply omits the annotation rather than failing the re-pin.
    """
    try:
        output = subprocess.run(
            ["git", "-C", root, "status", "--porcelain", "--"],
            capture_output=True, text=True, timeout=30, check=False,
        ).stdout
    except (OSError, subprocess.SubprocessError):
        return None
    changed = set()
    for line in output.splitlines():
        if len(line) > 3:
            changed.add(line[3:].strip().strip('"'))
    return changed


def derive_declared_value(record, root):
    """Run the record's DECLARED producer and return `(value, error)`.

    The record says how its own number is derived — `producer.argv`, `expected_exit` and the `field`
    to read out of the producer's JSON. Running it is what makes `--accept-value` a derivation rather
    than an assertion, and it is the whole safety property: a producer that will not run, or that
    disagrees with the line, is a refusal.
    """
    producer = record.get("producer")
    if not isinstance(producer, dict):
        return None, "record declares no producer"
    argv = producer.get("argv")
    field = producer.get("field")
    if not isinstance(argv, list) or not argv or not isinstance(field, str):
        return None, "producer declares no argv/field"
    try:
        completed = subprocess.run(
            argv, cwd=root, capture_output=True, text=True, timeout=600, check=False
        )
    except (OSError, subprocess.SubprocessError) as error:
        return None, f"producer could not run: {error}"
    expected_exit = producer.get("expected_exit", 0)
    if completed.returncode != expected_exit:
        return None, f"producer exited {completed.returncode}, expected {expected_exit}"
    try:
        payload = json.loads(completed.stdout)
    except ValueError:
        return None, "producer stdout is not JSON"
    if not isinstance(payload, dict) or field not in payload:
        return None, f"producer output has no field {field!r}"
    return str(payload[field]), None


def line_carries_value(lines, start, end, value):
    """Is the derived value present in the region's own text, on a token boundary?

    A substring test would accept `345` inside `1345`, which is exactly the kind of near-miss this
    tool exists to refuse.
    """
    text = "".join(line + "\n" for line in lines[start - 1 : end])
    return re.search(r"(?<![0-9A-Za-z_])" + re.escape(value) + r"(?![0-9A-Za-z_])", text) is not None


def accept_value(record, region, path, root, cache):
    """Re-pin an ABSENT region whose VALUE moved, deriving the new value from the producer.

    Returns `(accepted, detail)`. Mutates `record`/`region` only on acceptance, and re-pins all three
    coupled fields at once — the digest, the `value`, and any identifier whose suffix is the old
    digest prefix (`.17`'s gated convention, mechanized so it cannot be forgotten).
    """
    absolute = os.path.join(root, path)
    if not os.path.isfile(absolute):
        return False, "no file"
    lines = read_lines(absolute, cache)
    start, end = region.get("start_line"), region.get("end_line")
    if not (1 <= start <= len(lines) and start <= end <= len(lines)):
        return False, "recorded line range is outside the file"
    value, error = derive_declared_value(record, root)
    if error:
        return False, error
    if not line_carries_value(lines, start, end, value):
        return False, f"producer says {value!r} but the line does not carry it"
    old_digest = region.get("sha256") or ""
    new_digest = region_digest(lines, start, end)
    region["sha256"] = new_digest
    moved = [f"digest {old_digest[:12]} -> {new_digest[:12]}"]
    if "value" in record and str(record["value"]) != value:
        moved.append(f"value {record['value']} -> {value}")
        record["value"] = value
    for key in ("claim_key", "evidence_id", "assertion_id"):
        current = record.get(key)
        if isinstance(current, str) and old_digest and current.endswith(old_digest[:12]):
            record[key] = current[: -len(old_digest[:12])] + new_digest[:12]
            moved.append(f"{key} suffix -> {new_digest[:12]}")
    return True, "; ".join(moved)


def accept_sibling(record, region, new_digest):
    """Re-pin a region pinned to the SAME line as one a producer just verified.

    It carries no producer of its own — the `current_claim_census` evidence record declares a
    `verifier` instead — so it can never derive a value. What it CAN do is follow the line whose
    content a producer has just confirmed, which is exactly what the hand repair did and what `.17`
    had to gate when the human forgot it.
    """
    old_digest = region.get("sha256") or ""
    region["sha256"] = new_digest
    moved = [f"digest {old_digest[:12]} -> {new_digest[:12]} (sibling)"]
    for key in ("claim_key", "evidence_id", "assertion_id"):
        current = record.get(key)
        if isinstance(current, str) and old_digest and current.endswith(old_digest[:12]):
            record[key] = current[: -len(old_digest[:12])] + new_digest[:12]
            moved.append(f"{key} suffix -> {new_digest[:12]}")
    return "; ".join(moved)


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


def accept_value_pass(root, accept_value_path, apply_changes, out):
    """Accept the ABSENT regions of ONE governed file, producer first and siblings after.

    This is a PRE-pass rather than a branch inside the main loop, because the order matters and the
    registry order does not supply it: `current_claim_census` is scanned before `published_assertions`
    yet it is the census record that has no producer. Deriving first and sweeping siblings second is
    the only order in which the sibling rule can see a verified location.
    """
    cache, accepted, refusals = {}, [], []
    loaded = []
    for registry in REGISTRIES:
        registry_path = os.path.join(root, registry)
        if not os.path.isfile(registry_path):
            continue
        raw_lines = open(registry_path, encoding="utf-8").read().rstrip("\n").split("\n")
        records = []
        for raw in raw_lines:
            records.append(json.loads(raw) if raw.strip() else None)
        loaded.append([registry, registry_path, raw_lines, records, False])

    def absent_regions():
        for entry in loaded:
            registry, _, _, records, _ = entry
            for index, record in enumerate(records):
                if record is None:
                    continue
                pins = []
                walk(record, lambda node: pins.extend(pinned_regions(node)))
                for region, path in pins:
                    if path != accept_value_path:
                        continue
                    if classify(region, path, cache, root)["verdict"] == "ABSENT":
                        yield entry, record, region, path, index

    verified = {}
    pending = []
    for entry, record, region, path, index in list(absent_regions()):
        key = (path, region.get("start_line"), region.get("end_line"), region.get("sha256"))
        ok, detail = accept_value(record, region, path, root, cache)
        if ok:
            verified[key] = region["sha256"]
            entry[4] = True
            accepted.append({"registry": entry[0], "path": path, "detail": detail})
            cache.pop(os.path.join(root, path), None)
        else:
            pending.append((entry, record, region, path, key, detail))

    for entry, record, region, path, key, detail in pending:
        if key in verified:
            entry[4] = True
            accepted.append({"registry": entry[0], "path": path, "sibling": True,
                             "detail": accept_sibling(record, region, verified[key])})
        else:
            refusals.append({"registry": entry[0], "path": path, "region": region,
                             "verdict": "ABSENT", "detail": detail})

    if apply_changes and accepted and not refusals:
        for registry, registry_path, raw_lines, records, changed in loaded:
            if not changed:
                continue
            rewritten = [
                json.dumps(record, sort_keys=True, separators=(", ", ": ")) if record is not None
                else raw
                for raw, record in zip(raw_lines, records)
            ]
            handle, temporary = tempfile.mkstemp(
                dir=os.path.dirname(registry_path), suffix=".repin"
            )
            with os.fdopen(handle, "w", encoding="utf-8") as sink:
                sink.write("\n".join(rewritten) + "\n")
            os.replace(temporary, registry_path)
    return accepted, refusals


def process(root, only_path, apply_changes, out, accept_value_path=None):
    cache, counts, refusals, moves = {}, {}, [], []
    refreshes, accepted = [], []
    if accept_value_path:
        accepted, accept_refusals = accept_value_pass(
            root, accept_value_path, apply_changes, out
        )
        refusals.extend(accept_refusals)
        counts["accepted"] = len(accepted)
    modified = locally_modified(root)
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
            pins, whole = [], []
            walk(record, lambda node: pins.extend(pinned_regions(node)))
            walk(record, lambda node: whole.extend(whole_file_pins(node)))
            changed = False
            for pin, path in whole:
                verdict = classify_whole_file(pin, path, root)
                if only_path and verdict["path"] != only_path:
                    continue
                counts[verdict["verdict"]] = counts.get(verdict["verdict"], 0) + 1
                if verdict["verdict"] == "NO FILE":
                    refusals.append({**verdict, "registry": registry})
                    continue
                if verdict["verdict"] == "refreshed":
                    refreshes.append({**verdict, "registry": registry,
                                      "was": pin.get("sha256"), "claim": record.get("claim_id")})
                    pin["sha256"] = verdict["actual"]
                    changed = True
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

    for record in accepted:
        label = "accept-value (sibling)" if record.get("sibling") else "accept-value"
        print(f"  {label} {record['path']} ({record['registry'].split('/')[-1]})"
              f"  {record['detail']}", file=out)

    for refresh in refreshes:
        # A registry digest that moved during this very run must not read as "changed elsewhere":
        # the working-tree snapshot is taken before any write, so only this annotation can tell the
        # author that the tool itself is what moved it.
        if refresh["path"] in REGISTRIES:
            note = " [rewritten by this run]"
        elif modified is None:
            note = ""
        else:
            note = " [edited here]" if refresh["path"] in modified else " [changed elsewhere]"
        owner = f" {refresh['claim']}" if refresh.get("claim") else ""
        print(
            f"  refresh {refresh['path']}{owner}"
            f"  {(refresh['was'] or '')[:12]} -> {refresh['actual'][:12]}{note}",
            file=out,
        )

    # The convergence assertion the docstring promises. `claims.jsonl` pins the three sibling
    # registries this run may have just rewritten; processing them in declaration order refreshes
    # the sources first, so one pass is enough. If it ever is not, the dependency graph has grown a
    # cycle and that must be loud rather than papered over with another pass.
    if apply_changes and refreshes and not refusals:
        residual = []
        for registry in REGISTRIES:
            registry_path = os.path.join(root, registry)
            if not os.path.isfile(registry_path):
                continue
            for raw in open(registry_path, encoding="utf-8").read().splitlines():
                if not raw.strip():
                    continue
                pins = []
                walk(json.loads(raw), lambda node: pins.extend(whole_file_pins(node)))
                for pin, path in pins:
                    if only_path and path != only_path:
                        continue
                    verdict = classify_whole_file(pin, path, root)
                    if verdict["verdict"] == "refreshed":
                        residual.append((registry, path))
        if residual:
            print(
                "repin-claim-regions: CONVERGENCE FAILED — "
                f"{len(residual)} whole-file pin(s) are still stale after one pass, so the"
                " registries now depend on each other cyclically: "
                + ", ".join(f"{reg} -> {path}" for reg, path in residual[:6]),
                file=out,
            )
            return 1

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
        # A whole-file pin carries no span, so the line range is printed only when there is one.
        span = ""
        if not refusal.get("whole_file"):
            span = (f" {refusal['region']['start_line']}-{refusal['region']['end_line']}")
        if refusal.get("detail"):
            detail += f" — accept-value refused: {refusal['detail']}"
        print(
            f"  REFUSED {refusal['verdict']}: {refusal['registry']} -> {refusal['path']}{span}"
            f" ({refusal['region']['sha256'][:12]}){detail}",
            file=out,
        )

    summary = ", ".join(f"{key} {value}" for key, value in sorted(counts.items()))
    print(f"repin-claim-regions: {summary or 'no pinned regions in scope'}", file=out)
    if refreshes and not apply_changes:
        print(
            f"repin-claim-regions: {len(refreshes)} whole-file pin(s) are stale — these carry no"
            " line range, so there is nothing to resolve; --apply refreshes them and reports each.",
            file=out,
        )
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
    """The RED matrix: ambiguity, absence, a blank-line pin, a clean shift, and whole-file digests."""
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

        # --- CLAIM-VERIFICATION-ADOPTION.21: the whole-file digest family -------------------
        # Detection is structural, so both record shapes must be seen by the same rule.
        artifact = {"path": "governed.md", "role": "canonical_input", "sha256": "0" * 64}
        source = {"path": "governed.md", "record_type": "source", "schema_version": 1,
                  "sha256": "0" * 64, "source_id": "governed"}
        check("an artifact pin is detected", whole_file_pins(artifact) == [(artifact, "governed.md")])
        check("a source-record pin is detected", whole_file_pins(source) == [(source, "governed.md")])
        # A line-range region must NOT be mistaken for a whole-file pin, or a span would be lost.
        check(
            "a line-range region is not a whole-file pin",
            whole_file_pins({"path": "governed.md", "sha256": "0" * 64,
                             "start_line": 1, "end_line": 1}) == [],
        )

        open(governed, "w", encoding="utf-8").write("alpha\nbeta\n")
        actual = hashlib.sha256(b"alpha\nbeta\n").hexdigest()
        claims = os.path.join(registry_dir, "claims.jsonl")
        open(registry, "w", encoding="utf-8").write("")
        stale = {"claim_id": "fixture-claim",
                 "durability": {"artifacts": [dict(artifact)]}}
        open(claims, "w", encoding="utf-8").write(json.dumps(stale) + "\n")
        exit_code = process(root, None, True, sink)
        refreshed = json.loads(open(claims, encoding="utf-8").read().strip())
        check("a stale whole-file pin refreshes and exits zero", exit_code == 0)
        check(
            "a stale whole-file pin takes the file's digest",
            refreshed["durability"]["artifacts"][0]["sha256"] == actual,
        )

        # Already current: the record must be left byte-identical, not rewritten.
        before = open(claims, encoding="utf-8").read()
        exit_code = process(root, None, True, sink)
        check("an unchanged whole-file pin writes nothing",
              open(claims, encoding="utf-8").read() == before and exit_code == 0)

        # A pin whose file is gone is a REFUSAL, exactly as the line-range classes are.
        os.remove(governed)
        before = open(claims, encoding="utf-8").read()
        exit_code = process(root, None, True, sink)
        check("a whole-file pin with no file refuses", exit_code == 1)
        check("a refused whole-file pin writes nothing",
              open(claims, encoding="utf-8").read() == before)

        # --- CLAIM-VERIFICATION-ADOPTION.22: --accept-value DERIVES, never accepts ------------
        open(claims, "w", encoding="utf-8").write("")
        producer = os.path.join(root, "producer.py")

        def install(count, exit_code_wanted=0, payload=None):
            body = payload if payload is not None else (
                '{"card_count": %d}' % count
            )
            open(producer, "w", encoding="utf-8").write(
                "import sys\nsys.stdout.write(%r)\nsys.exit(%d)\n" % (body, exit_code_wanted)
            )

        def assertion(value, digest, with_producer=True):
            record = {
                "assertion_id": "fixture-count-" + digest[:12],
                "outcome": "derived",
                "path": "governed.md",
                "record_type": "assertion",
                "region": {"kind": "line_range_sha256", "start_line": 1, "end_line": 1,
                           "sha256": digest},
                "schema_version": 1,
                "value": value,
            }
            if with_producer:
                record["producer"] = {"argv": [sys.executable, "producer.py"],
                                      "expected_exit": 0, "field": "card_count"}
            return record

        stale = hashlib.sha256(b"> 343 cards\n").hexdigest()
        open(governed, "w", encoding="utf-8").write("> 344 cards\n")
        install(344)

        # 1. The producer AGREES with the new line: value, digest and identifier move together.
        open(registry, "w", encoding="utf-8").write(
            json.dumps(assertion("343", stale)) + "\n")
        exit_code = process(root, None, True, sink, "governed.md")
        moved = json.loads(open(registry, encoding="utf-8").read().strip())
        fresh = hashlib.sha256(b"> 344 cards\n").hexdigest()
        check("accept-value exits zero when the producer agrees", exit_code == 0)
        check("accept-value re-pins the digest", moved["region"]["sha256"] == fresh)
        check("accept-value re-derives the value", moved["value"] == "344")
        check("accept-value renames the identifier suffix",
              moved["assertion_id"].endswith(fresh[:12]))

        # 2. The producer DISAGREES with the line -> refuse, write nothing.
        install(999)
        before = json.dumps(assertion("343", stale)) + "\n"
        open(registry, "w", encoding="utf-8").write(before)
        exit_code = process(root, None, True, sink, "governed.md")
        check("a producer that disagrees refuses", exit_code == 1)
        check("a disagreeing producer writes nothing",
              open(registry, encoding="utf-8").read() == before)

        # 3. No producer declared -> refuse. There is nothing to derive the value FROM.
        before = json.dumps(assertion("343", stale, with_producer=False)) + "\n"
        open(registry, "w", encoding="utf-8").write(before)
        exit_code = process(root, None, True, sink, "governed.md")
        check("a record with no producer refuses", exit_code == 1)
        check("a producerless record writes nothing",
              open(registry, encoding="utf-8").read() == before)

        # 4. The producer fails -> refuse, even though the line looks right.
        install(344, exit_code_wanted=3)
        before = json.dumps(assertion("343", stale)) + "\n"
        open(registry, "w", encoding="utf-8").write(before)
        exit_code = process(root, None, True, sink, "governed.md")
        check("a producer exiting non-zero refuses", exit_code == 1)
        check("a failed producer writes nothing",
              open(registry, encoding="utf-8").read() == before)
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
    parser.add_argument(
        "--accept-value",
        metavar="FILE",
        help="for ABSENT regions in FILE, DERIVE the new value from the record's declared"
             " producer and re-pin value, digest and identifier together; refuses without a"
             " producer, on a producer error, or when the line does not carry the derived value",
    )
    args = parser.parse_args()

    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    if args.self_test:
        return self_test(sys.stdout)
    return process(root, args.path, args.apply, sys.stdout, args.accept_value)


if __name__ == "__main__":
    sys.exit(main())
