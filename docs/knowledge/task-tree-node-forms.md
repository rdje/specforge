---
id: task-tree-node-forms
title: Task-tree leaves use two node forms; an audit matching only `- ID:` reports false unowned leaves
answers:
  - "how is a task-tree leaf written in docs/tasks (two forms: the absolute `- ID: `TREE.x` · Status: ...` line used by most trees, and a nested relative `  - `.x` · Status: ...` line used for children written inline under their parent, e.g. DOC-INTENT-TAXONOMY .3b/.3c under .3)"
  - "why does my task-tree audit report leaves that are actually owned (it probably matches only the absolute `- ID:` node form; the nested relative `  - `.3b` · Status:` form is equally legitimate and owns its leaf, so an audit that misses it produces false positives)"
  - "how do I check whether a named frontier has an owning task-tree leaf"
  - "does every Frontier -> .x mention in a task tree resolve to a real leaf"
  - "was there ever a task-tree lane whose named next step had no owning leaf (yes, exactly one: KG-ISF-COMPLETENESS.5.iv.a, named as the frontier by .5.iv on 2026-08-11 but never given a node; found and owned 2026-08-31 by LIVE-DOCUMENT-PRESSURE-HEADROOM.4e while auditing that report's writer set)"
  - "how many task trees have an unowned named frontier"
date: 2026-08-31
status: current
tags: [task-tree, doctrine, governance, audit]
evidence: docs/tasks/KG-ISF-COMPLETENESS.md (.5.iv.a, absolute form); docs/tasks/DOC-INTENT-TAXONOMY.md (.3b/.3c, nested relative form); docs/TASK_TREE.md; docs/TASK_TREE_README.md; docs/decisions/0003-task-tree-and-commit-doctrine.md
reverify: "grep -nE '^ *[-*] ID: `KG-ISF-COMPLETENESS[.]5[.]iv[.]a`' docs/tasks/KG-ISF-COMPLETENESS.md # absolute node form, and the leaf this card says is now owned; grep -nE '^ *[-*] `[.]3b` . Status' docs/tasks/DOC-INTENT-TAXONOMY.md # nested relative node form. Run the detector in the body over docs/tasks/*.md; it must print 0 trees."
---

# Task-tree leaves are written in two node forms

`docs/tasks/*.md` uses two equally legitimate shapes for a leaf, and both own the leaf they name:

- **Absolute** — `- ID: `KG-ISF-COMPLETENESS.5.iv` · Status: `done` · Goal: …`. Most trees use this
  throughout, one top-level list item per node at any depth.
- **Nested relative** — `  - `.3b` · Status: `done` (…) · **Implemented** …`, indented under its parent's
  node. `DOC-INTENT-TAXONOMY.3` writes `.3a`/`.3b`/`.3c` this way; the id is relative to the enclosing tree.

The consequence is practical: **any audit that scans the task plane must recognise both forms.** An audit
matching only `- ID:` reports the nested children as unowned, which is a false positive — and a false positive
in a governance audit is worse than no audit, because it invites either a duplicate leaf or a dismissal of the
whole result.

## The detector

Recognise both forms, then check every `Frontier → ` mention resolves:

```python
import re, glob, os
tot = 0
for p in sorted(glob.glob('docs/tasks/*.md')):
    txt = open(p, encoding='utf-8').read()
    m = re.search(r'^- Tree ID: `([A-Z0-9\-]+)`', txt, re.M)
    tree = m.group(1) if m else os.path.basename(p)[:-3]
    abs_ids = set(re.findall(r'^\s*[-*] ID: `([A-Z0-9\-]+(?:\.[0-9a-zA-Z]+)*)`', txt, re.M))
    rel = {i[len(tree):] for i in abs_ids if i.startswith(tree + '.')}
    rel |= set(re.findall(r'^\s*[-*] `(\.[0-9a-zA-Z.]+)`\s*(?:·|:)\s*\*{0,2}Status', txt, re.M))
    fr = set(re.findall(r'[Ff]rontier\s*(?:→|->)\s*\**`(\.[0-9a-zA-Z.]+|[A-Z0-9\-]+\.[0-9a-zA-Z.]+)`', txt))
    missing = sorted(f for f in fr if f not in rel and f not in abs_ids)
    if missing:
        tot += 1
        print(os.path.basename(p), '->', missing)
print('trees with an unowned named frontier:', tot)
```

## What the corrected audit found

Run over every tree on `2026-08-31`, the corrected detector (the `reverify` script above) reports **exactly one**
tree whose prose named a frontier with no owning node: `KG-ISF-COMPLETENESS`, whose `.5.iv` measurement closed
with `**Frontier → `.5.iv.a` (CODE)**` on `2026-08-11` and never gave `.5.iv.a` a leaf. Under
`docs/decisions/0003-task-tree-and-commit-doctrine.md` that lane's named next step could not legally be started
without first creating the node, and nothing detected the gap. `LIVE-DOCUMENT-PRESSURE-HEADROOM.4e` found it
while counting that lane's remaining writers, and `.5.iv.a` now has its node.

The uncorrected first pass reported three trees. The two extra —`DOC-INTENT-TAXONOMY.3b`/`.3c` and
`BOOK-USER-FRIENDLY-BACKFILL.2.b`–`.2.f` — were nested-relative nodes, already owned. That correction is the
reason this card exists: the detector, not the finding, was the thing worth writing down.

Links: [[project_kg_isf_completeness]], [[generic-enum-conflation]], [[research-record-size-profile]].
