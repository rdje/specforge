#!/usr/bin/env python3
"""Re-derive the caption-mediated-coverage counterfactual over the retained chains.

`SemanticIr::cited_provenance_ids` deliberately refuses statement-mediated coverage: `EvidenceIR`
relates a statement to a visual region when that statement *is* the region's caption, and a caption
reaching a canonical carrier says nothing about the region's own content. This measures what
admitting that mediation would have explained, under both readings of it:

  narrow = mediate only through the collections coverage already reads
  broad  = mediate through every SemanticIR collection carrying `supporting_statement_ids`,
           the shape a naive implementation takes

It also reports whether the reviewed CoreSight region `picture_0001` (`visual_0008`) falls in each
set, because that cell is the one the review requires to stay residual.

Run from the repository root; reads only persisted artifacts under `generated/`.
"""
import json
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
retained = json.load(open(ROOT/'doctrine/chain_currency/retained_bundles.json'))['retained']

NARROW = ['timing_constraints','signal_constraints','conditional_rules','regular_states',
          'state_transitions','temporal_rules','temporal_conflicts']
BROAD = NARROW + ['invariants','contracts','assertions','abstractions','gates','phases',
                  'decomposition_candidates']
FIGURE_KINDS = {'figure','diagram','chart','screenshot','formula_region'}

def prov(S, colls):
    ids = set()
    for c in colls:
        for r in S.get(c, []) or []:
            if isinstance(r, dict):
                ids.update(r.get('supporting_statement_ids', []) or [])
    for c in S.get('actor_contracts', []) or []:
        ids.update((c.get('provenance') or {}).get('supporting_statement_ids', []) or [])
    return ids

tot_fig = tot_res = 0
med = {'narrow': 0, 'broad': 0}
coresight = {}
for k in retained:
    ev, se = ROOT/'generated/evidence_ir'/k/'evidence_ir.json', ROOT/'generated/semantic_ir'/k/'semantic_ir.json'
    if not (ev.exists() and se.exists()): continue
    E, S = json.load(open(ev)), json.load(open(se))
    figs = [v for v in E.get('visual_evidence', []) or [] if v['asset_kind'] in FIGURE_KINDS]
    tot_fig += len(figs)
    stmt_to_vis = {}
    for st in E.get('extracted_statements', []) or []:
        for v in st.get('related_visual_evidence_ids', []) or []:
            stmt_to_vis.setdefault(st['statement_id'], set()).add(v)
    links = {}
    for l in E.get('evidence_links', []) or []:
        links.setdefault(l.get('from_evidence_span_id'), set()).add(l.get('to_visual_evidence_id'))

    narrow_p, broad_p = prov(S, NARROW), prov(S, BROAD)
    direct = {v['evidence_id'] for v in figs
              if v['evidence_id'] in narrow_p or f"figure:{v['asset_id']}" in narrow_p}
    residual = [v for v in figs if v['evidence_id'] not in direct]
    tot_res += len(residual)
    for label, p in (('narrow', narrow_p), ('broad', broad_p)):
        reach = set()
        for sid in p:
            reach |= stmt_to_vis.get(sid, set()); reach |= links.get(sid, set())
        med[label] += sum(1 for v in residual if v['evidence_id'] in reach)
    if k.startswith('den0068'):
        reach_n, reach_b = set(), set()
        for sid in narrow_p: reach_n |= stmt_to_vis.get(sid, set()) | links.get(sid, set())
        for sid in broad_p:  reach_b |= stmt_to_vis.get(sid, set()) | links.get(sid, set())
        coresight = {'visual_0008 in narrow-mediated': 'visual_0008' in reach_n,
                     'visual_0008 in broad-mediated': 'visual_0008' in reach_b}
print(json.dumps({'captured_figure_regions': tot_fig, 'residualized': tot_res,
                  'would_be_marked_explained': med, 'coresight_picture_0001': coresight}, indent=2))
