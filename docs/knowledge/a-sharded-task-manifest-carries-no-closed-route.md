---
id: a-sharded-task-manifest-carries-no-closed-route
title: A sharded task tree's manifest.json carries only routes that are not closed; closed routes live in the route catalog
answers:
  - "why does a task tree's manifest.json not list a closed leaf"
  - "where is a closed leaf's route (routes-000N.md)"
  - "what bounds a sharded task tree's manifest.json (65,536 bytes)"
  - "what is LIVE-DOCUMENT-PRESSURE-HEADROOM.17a"
  - "active task manifest exceeds 65536 bytes"
date: 2026-10-23
status: current
tags: [task-trees, live-document-size, task-evidence, manifest, route-catalog, measured]
evidence: scripts/check_active_task_evidence.pl (manifest_expected_subset); docs/tasks/live-document-pressure-headroom/pointer-index-and-ownership.md (.17a); docs/decisions/0057-a-sharded-trees-root-owner-registry-declares-the-leaves-not-yet-closed.md
reverify: "In docs/tasks/extraction-representation-gaps/manifest.json, every leaf_routes entry has lifecycle open; the contract's leaf_routes also lists the closed ones"
---

A task-evidence contract's `manifest.json` is a projection the checker derives (`manifest_expected_subset`, shared
by `--write` and `--check`). For a sharded contract it carries only the routes that are not closed, the same cut ADR
0057 made for the root's owner registry. The contract itself still lists every route, and the route catalog parts
(`routes-000N.md`) carry each with its lifecycle, so a closed leaf's part is found there. An inline contract has no
lifecycle and keeps every route under its `max_leaf_routes` count.

**Why (`LIVE-DOCUMENT-PRESSURE-HEADROOM.17a`, `2026-10-23`).** The manifest has a portable 65,536-byte ceiling with
no warning band. Carrying every route, the extraction representation gaps manifest reached 65,501 bytes at 228 routes
(190 closed), and opening two leaves refused the tree (`active task manifest exceeds 65536 bytes`). Dropping closed
routes took the six sharded manifests to about half: that one to 25,418 bytes at 38 open routes. A route costs 209 to
235 bytes in the pretty encoding, so the largest manifest binds near 190 more open leaves.

When a tree's manifest grows again, count its open leaves first: closures no longer cost it anything.
