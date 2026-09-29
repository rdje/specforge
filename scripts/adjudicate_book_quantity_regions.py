#!/usr/bin/env python3
"""Adjudicate the book's quantitative candidates one paragraph at a time (LIVE-DOCUMENT-PRESSURE-HEADROOM.44).

`scripts/check_book_quantitative_claims.pl` accepts a region that spans several lines and adjudicates every
candidate inside it with the region's one outcome. A region per candidate LINE made the registry grow with every
numeral the book prints. A region per run of candidates that share one Markdown block and one outcome grows with
the book's paragraphs instead, and the checker refuses two regions that could be one.

  --compact              merge every pair of regions the checker would refuse as mergeable
  --add PATH LINE...     cover new candidate lines of PATH with `incomplete` regions, then compact PATH

A block is the checker's `block_ids`: a run of non-blank lines that no heading, list item, table row, block
quote or fence line begins. Only `excluded` and `incomplete` regions merge, and only with the same scope reason
or the same missing legs; a `registered` region names one claim and is never merged. A merged region keeps the
first member's `region_id`, spans both members, and pins the SHA-256 of the lines it spans.
"""
import argparse
import hashlib
import json
import re
import sys
from pathlib import Path

REGISTRY = Path('doctrine/claim_verification/book_quantitative_claims.jsonl')
BLOCK_START = re.compile(r'^\s{0,3}(?:#|\||>|```|~~~|[-*+]\s|[0-9]+[.)]\s)')
MERGEABLE = {'excluded', 'incomplete'}


def encode(record):
    return json.dumps(record, sort_keys=True, ensure_ascii=False, separators=(', ', ': '))


def book_lines(path, cache):
    if path not in cache:
        cache[path] = Path(path).read_text(encoding='utf-8').splitlines(keepends=True)
    return cache[path]


def block_ids(lines):
    """One id per line; None for a blank line. Mirrors `block_ids` in check_book_quantitative_claims.pl."""
    ids, current, previous_blank, in_fence = [], 0, True, False
    for line in lines:
        text = line.rstrip('\r\n')
        if not text.strip():
            ids.append(None)
            previous_blank = True
            continue
        if in_fence or BLOCK_START.match(text) or previous_blank:
            current += 1
        if re.match(r'^\s{0,3}(?:```|~~~)', text):
            in_fence = not in_fence
        ids.append(current)
        previous_blank = False
    return ids


def signature(record):
    if record.get('outcome') not in MERGEABLE:
        return None
    return (record['path'], record['outcome'], record.get('scope_reason'), tuple(record.get('missing_legs') or ()))


def region_sha(lines, start, end):
    return hashlib.sha256(''.join(lines[start - 1:end]).encode('utf-8')).hexdigest()


def compact(records, cache, only_path=None):
    """Merge adjacent same-signature regions of one block, file by file; returns the merge count."""
    merged = 0
    regions = [r for r in records if r.get('record_type') == 'region'
               and (only_path is None or r['path'] == only_path)]
    by_path = {}
    for region in regions:
        by_path.setdefault(region['path'], []).append(region)
    dropped = set()
    for path, members in by_path.items():
        lines = book_lines(path, cache)
        blocks = block_ids(lines)
        members.sort(key=lambda r: r['region']['start_line'])
        keep = members[0]
        for region in members[1:]:
            same_block = (blocks[keep['region']['end_line'] - 1] is not None
                          and blocks[keep['region']['end_line'] - 1] == blocks[region['region']['start_line'] - 1])
            if signature(keep) is not None and signature(keep) == signature(region) and same_block:
                keep['region']['end_line'] = region['region']['end_line']
                keep['region']['sha256'] = region_sha(lines, keep['region']['start_line'], keep['region']['end_line'])
                dropped.add(id(region))
                merged += 1
            else:
                keep = region
    records[:] = [r for r in records if id(r) not in dropped]
    return merged


def add(records, head, path, wanted, cache):
    lines = book_lines(path, cache)
    for n in wanted:
        sha = hashlib.sha256(lines[n - 1].encode('utf-8')).hexdigest()
        rid = 'book-quantity-' + hashlib.sha256(f'{path}\0{n}\0{sha}'.encode('utf-8')).hexdigest()[:16]
        region = {'missing_legs': ['rederive', 'falsification', 'durability'], 'outcome': 'incomplete',
                  'path': path, 'record_type': 'region',
                  'region': {'end_line': n, 'kind': 'line_range_sha256', 'sha256': sha, 'start_line': n},
                  'region_id': rid, 'schema_version': 1}
        same = [i for i, r in enumerate(records) if r.get('record_type') == 'region' and r['path'] == path]
        after = [i for i in same if records[i]['region']['start_line'] > n]
        position = after[0] if after else (same[-1] + 1 if same else len(records))
        records.insert(position, region)
    head['expected_candidate_lines'] += len(wanted)
    return compact(records, cache, only_path=path)


def main():
    parser = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    parser.add_argument('--compact', action='store_true')
    parser.add_argument('--add', nargs='+', metavar=('PATH', 'LINE'))
    args = parser.parse_args()
    if not args.compact and not args.add:
        parser.error('choose --compact or --add PATH LINE...')
    raw = REGISTRY.read_text(encoding='utf-8')
    rows = [line for line in raw.split('\n') if line]
    if any(encode(json.loads(row)) != row for row in rows):
        sys.exit(f'{REGISTRY}: a record is not in the registry encoding; refusing to rewrite it')
    head, records = json.loads(rows[0]), [json.loads(row) for row in rows[1:]]
    cache = {}
    before = sum(1 for r in records if r.get('record_type') == 'region')
    if args.add:
        path, wanted = args.add[0], sorted(int(n) for n in args.add[1:])
        merged = add(records, head, path, wanted, cache)
        print(f'added {len(wanted)} candidate line(s) to {path}; {merged} merge(s); '
              f'expected_candidate_lines {head["expected_candidate_lines"]}')
    if args.compact:
        merged = compact(records, cache)
        print(f'compacted {merged} region(s) into their block neighbours')
    after = sum(1 for r in records if r.get('record_type') == 'region')
    REGISTRY.write_text('\n'.join([encode(head)] + [encode(r) for r in records]) + '\n', encoding='utf-8')
    print(f'regions {before} -> {after}')


if __name__ == '__main__':
    main()
