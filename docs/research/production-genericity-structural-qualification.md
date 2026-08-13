# Production-genericity structural qualification

Status: **qualified structurally; population-level behavioral qualification remains**
Owner: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.vii`
Qualification date: `2026-08-14`
Clean implementation range: `f7da4ab8..07b1f874`

## Result

The complete `.6d.ii.e` program is structurally qualified. Its 15 committed implementation slices changed 146
tracked files with 26,093 insertions and 3,756 deletions. The resulting boundary contains 77 classified
production modules, 41 claim families, 168 field-root rules, 117 producer/mutator entrypoints, 53 canonical
seams, four conformance bypasses, and one compiler-derived 140-row information-flow boundary. The final graph
contains 2,169 production functions, 11,295 helper edges, 10,419 semantic decisions, 19 protected constructions,
28 protected calls, and 1,371 sensitive macro invocations.

All 24 chains backed by retained, verifiable capture bundles are current at every proof-bearing stage. The other
54 corpus chains remain explicitly legacy and proof-unmeasurable at every stage; no proof was synthesized for
them. This is the exact compatibility boundary, not an assertion that 78 chains are current.

The final 24 adapter ledgers all use ruleset
`7683ecc5977f2e4498105d01357b36c7754002deb5946b14a2c023a9e2defc8f`, contain all 168 registered rule ids,
and contain 148,708 cumulative claims. Their stage partition is exact:

| Claim stage | Current schema / legacy schema | Current / legacy artifacts | Families / fields | Claims in final cumulative ledgers | Current residual objects | Migration delta outside proof/validation |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| SourceIR | 3 / 1 | 24 / 54 | 5 / 19 | 31,382 | 0 | 0 |
| EvidenceIR | 3 / 2 | 24 / 54 | 11 / 39 | 89,758 | N/A: no residual surface | 0 |
| SemanticIR | 2 / 1 | 24 / 54 | 12 / 49 | 12,399 | 3 | 0 |
| IntentIR | 2 / 1 | 24 / 54 | 9 / 49 | 14,288 | 8 | 0 |
| ISF adapter | 2 / 1 | 24 / 54 | 4 / 12 | 881 | 62 | 0 |
| **Total** | — | **120 / 270 artifacts** | **41 / 168** | **148,708** | **73 stage-surface objects** | **0 / 120** |

Residual counts are stage-surface objects, not deduplicated semantic issues. They are reported because
`residual_decisions` remained inside the exact public comparison. The `.e.iv.vii` proof-only reconciliation
excluded only `proof_context`, `proof_ledger`, and `validation_reports`; all 120 remaining artifact comparisons
were exact. Therefore proof migration added current executable authority while changing zero pre-existing
residual or other public value. The comparison snapshot was exactly 120 files / 964,137,330 bytes and was removed
after verification. Later `.e.v` structural analysis and `.e.vi` test-only alpha qualification did not change the
production-semantic ruleset or require another artifact migration.

All 24 current adapter states are honestly blocked and reconcile to zero current emitted files. Existing legacy
`.isf` files are not counted as current proof-bearing output.

## Per-chain ledger and residual attribution

Counts below are cumulative at each persisted stage. A residual suffix is shown only on stages that expose a
`residual_decisions` field.

| Document key | Source claims/residuals | Evidence claims | Semantic claims/residuals | Intent claims/residuals | Adapter claims/residuals |
| --- | ---: | ---: | ---: | ---: | ---: |
| `102196_0100_01_2022_05_05_aarch64_external_debug_guide` | 368/0 | 953 | 1,056/0 | 1,141/1 | 1,162/1 |
| `102520_0101_01_2025_09_15_introducing_coresight_debug_and_trace` | 381/0 | 958 | 1,064/0 | 1,167/0 | 1,188/0 |
| `109242_0100_01_2023_09_04_arm_smmu_software_guide` | 799/0 | 2,108 | 2,331/0 | 2,554/0 | 2,574/0 |
| `198123_0302_03_2025_04_22_generic_interrupt_controller_overview_guide` | 618/0 | 1,683 | 1,865/0 | 2,049/0 | 2,069/0 |
| `den0068_2018_07_23_coresight_base_system_architecture` | 547/0 | 1,612 | 1,829/0 | 2,102/0 | 2,122/0 |
| `ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification` | 9,129/0 | 24,706 | 27,201/2 | 29,524/2 | 29,806/17 |
| `opencapi_25gbps_phy_mechanical_spec_v10` | 926/0 | 2,969 | 3,151/0 | 3,347/0 | 3,367/0 |
| `opencapi_25gbps_phy_signaling_spec_1_0` | 386/0 | 1,467 | 1,653/0 | 1,821/0 | 1,841/0 |
| `opencapi_3_0_certified_definition_v1_1` | 238/0 | 662 | 742/0 | 816/0 | 836/0 |
| `opencapi_3_0_certified_test_resources_engineering_note_v1_0` | 244/0 | 656 | 726/0 | 782/0 | 803/0 |
| `opencapi_3_0_ready_definition_v1_1` | 251/0 | 681 | 800/0 | 922/0 | 942/0 |
| `opencapi_3_0_ready_test_resources_engineering_note_v1_0` | 177/0 | 480 | 553/0 | 615/0 | 636/0 |
| `opencapi_4_0_32g_phy_signal_spec_1_0_16nov2020` | 528/0 | 2,347 | 2,619/0 | 2,907/0 | 2,927/0 |
| `opencapi_4_0_32gbps_phy_mech_spec_v10_17mar2021` | 335/0 | 1,051 | 1,192/0 | 1,342/0 | 1,362/0 |
| `opencapi_afu_address_space_usage` | 194/0 | 528 | 607/0 | 689/2 | 712/2 |
| `opencapi_data_link_layer_v20_09jul2020` | 840/0 | 4,413 | 4,661/0 | 4,926/0 | 4,946/0 |
| `opencapi_discovery_configuration_v201` | 393/0 | 2,693 | 2,865/0 | 3,049/0 | 3,073/1 |
| `pjdoc_466751330_7215_10_0_cortex_a76_software_optimization_guide` | 530/0 | 2,631 | 2,748/0 | 2,833/2 | 2,855/2 |
| `um10204_rev7_0_2021_i2c_bus_specification` | 1,052/0 | 3,405 | 3,921/1 | 4,484/1 | 4,592/6 |
| `um11732_v3_2022_02_17_i2s_bus_specification` | 215/0 | 639 | 753/0 | 870/0 | 895/0 |
| `usb4_connection_manager_guide_v2_0_2025_11` | 1,649/0 | 5,359 | 5,903/0 | 6,501/0 | 6,523/2 |
| `usb4_inter_domain_service_specification_v2_0_2025_11` | 923/0 | 3,958 | 4,465/0 | 5,141/0 | 5,161/0 |
| `usb_3_2_revision_1_0_2017_09` | 8,114/0 | 48,792 | 54,028/0 | 61,067/0 | 61,117/30 |
| `wbspec_b4_wishbone_b4_specification` | 2,545/0 | 6,389 | 6,806/0 | 7,178/0 | 7,199/1 |

## Structural signoff composition

The signoff is conjunctive:

1. Cargo compilation enforces one-way `specforge-core <- specforge-conformance`, while the application composes
   both.
2. Sealed capabilities, exact registered replay, and the promotion kernel make canonical authority
   proof-carrying and fail closed for legacy, stale, proofless, forged, unauthorized, and future artifacts.
3. Compiler-derived production-semantic digests bind the five stage registries without comment/test churn.
4. The unconditional `PRODUCTION-GENERICITY` doctrine joins dependency, complete inventory, rule/seam/bypass,
   compiled graph, information-flow, and proof-only promotion checks.
5. CI qualification rejects 27 controlled dependency/schema/rule/flow faults and executes the structural alpha
   obligation of every one of the 168 live rules. Legal display, provenance, and excluded test-only spelling uses
   remain admitted.
6. `CHAIN-CURRENCY` replays the 24 measurable chains as current, reports zero stale chains and 54 explicit
   unmeasurable chains, and reconciles 24 blocked adapters to zero current emitted files.

Full CI passes all nine doctrines and genericity components, formatting, warning-denied Clippy and Rustdoc,
1,953 Rust tests with six ignores and zero failures, five compile-fail doctests, mdBook test/build, containment,
and final locality. Cleanup removed 73 generated book files plus 144 incremental files (223,680 KiB total);
both disposable roots and requested `.bin`/`.log` residue are absent.

## Honest boundary and next owner

This result proves the structural core boundary and the exact proof migration. It does **not** prove behavioral
invariance on the population. `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f` remains the sole owner of whole-document symbol
alpha-renaming, identity perturbation, structure-preserving paraphrases, negative controls, held-out families,
complete-population replay, and the final parent genericity decision.

## Reverification

```bash
git rev-list --count f7da4ab8..07b1f874
git diff --shortstat f7da4ab8..07b1f874
bash scripts/check_production_genericity.sh --self-test
bash scripts/check_chain_currency.sh --check
bash scripts/run_ci.sh
```
