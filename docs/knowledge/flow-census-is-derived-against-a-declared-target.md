---
id: flow-census-is-derived-against-a-declared-target
title: The production-genericity flow census is derived against a DECLARED target (x86_64-unknown-linux-gnu), because with the host's cfg it differed by 3 sites between macOS and Linux
answers:
  - "why does the flow census differ on my machine"
  - "what is ANALYSIS_TARGET in the production graph"
  - "why does check_production_genericity_flow.sh disagree with flow_census.json"
  - "does the production graph honour cfg attributes"
  - "can I change the production graph analysis target"
  - "why did INFORMATION-FLOW fail only on CI"
  - "how do I re-pin decision_sites or helper_edges"
date: 2026-09-20
status: current
tags: [commit-gate-single-run, production-genericity, information-flow, portability, hosted-ci]
evidence: tools/production-genericity-graph/src/config.rs (ANALYSIS_TARGET); tools/production-genericity-graph/src/analyzer.rs:253; crates/specforge/src/ir/source/docling_backend.rs:1367-1381; doctrine/production_genericity/flow_census.json; docs/tasks/COMMIT-GATE-SINGLE-RUN.md (.16)
reverify: "bash scripts/check_production_genericity_flow.sh — expect agreement and 13348 decision sites / 15572 helper edges; rustc --print cfg | grep target_os versus rustc --print cfg --target x86_64-unknown-linux-gnu | grep target_os — expect them to differ on a Mac."
---

`tools/production-genericity-graph` evaluates `cfg` attributes (`analyzer.rs:253`,
`Configuration::attributes_are_active`), and SpecForge has genuinely platform-conditional code. So
the derived graph depends on which `cfg` set the tool is given, and **the tool declares one** rather
than asking the host:

```rust
pub(crate) const ANALYSIS_TARGET: &str = "x86_64-unknown-linux-gnu";
```

Before `2026-09-20` it ran `rustc --print cfg` with no `--target`, which answers for whoever runs
it. The memory-pressure guard in `ir/source/docling_backend.rs` compiles
`read_macos_used_memory_percent` plus `parse_macos_memory_pressure_used_percent` on macOS and
`read_linux_used_memory_percent` on Linux, so three sites moved between two counts:

| | macOS host | Linux runner |
| --- | ---: | ---: |
| `decision_sites` | 13345 | **13348** |
| `helper_edges` | 15575 | **15572** |

`flow_census.json` had been pinned from a Mac, so **hosted CI could never have agreed with it** —
invisible for as long as hosted CI never ran the gate (`COMMIT-GATE-SINGLE-RUN.11`-`.16`). Declaring
the target made a Mac reproduce the runner's two numbers exactly, which is both the fix and the
proof of the diagnosis.

Three things follow for anyone touching this:

- **The census measures the source, not the machine.** Any host re-derives the same counts. If yours
  disagree, you changed the source — or `ANALYSIS_TARGET`.
- **Changing `ANALYSIS_TARGET` re-derives every volume count.** It is a decision with an owning leaf,
  not a knob. Linux is chosen because CI runs there; the value matters far less than its being fixed.
- **Re-pin through the idiom, never by editing the number**: `volume.aggregate_change` takes
  `baseline` + `delta` + `authority_id` + `owner` + `rationale`, and `delta` is `i64`, so a negative
  one is declared rather than hidden. See [[hosted-ci-cannot-enforce-the-corpus-stratum]].

`rustc --print cfg --target <triple>` needs no installed std for that triple, so this costs a
developer machine of any platform nothing.
