# CHANGES

## 2026-04-16 (AXI read-address control sideband stability KG fixture)

### Added: AXI read-address control sideband stability truthfulness fixture
- Added tracked KG fixture `axi_read_address_control_sideband_stability_gold` for AXI read-address control sideband hold behavior across `ARPROT`, `ARCACHE`, and `ARLOCK` with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `ARVALID`, `ARREADY`, `ARPROT`, `ARCACHE`, and `ARLOCK` through graph-backed actor relations, actor-relative ports, resolved `ARVALID` / `ARREADY` semantic roles, and three manager-grounded temporal stability rules.
- Each temporal rule requires `HandshakeComplete(ARVALID, ARREADY)` and actor-grounded stability for the sideband subject, pairing the prior write-address control-sideband coverage with the read-address protection, cache, and lock attributes.
- Refreshed corpus-KB KG projections so the tracked suite now reports `79` fixtures / `0` failures, AMBA-family coverage reports `30/30`, and temporal fixture coverage reports `36/36`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_read_address_control_sideband_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `79` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `79` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI write-address control sideband stability KG fixture)

### Added: AXI write-address control sideband stability truthfulness fixture
- Added tracked KG fixture `axi_write_address_control_sideband_stability_gold` for AXI write-address control sideband hold behavior across `AWPROT`, `AWCACHE`, and `AWLOCK` with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `AWVALID`, `AWREADY`, `AWPROT`, `AWCACHE`, and `AWLOCK` through graph-backed actor relations, actor-relative ports, resolved `AWVALID` / `AWREADY` semantic roles, and three manager-grounded temporal stability rules.
- Each temporal rule requires `HandshakeComplete(AWVALID, AWREADY)` and actor-grounded stability for the sideband subject, extending write-address coverage beyond burst length/size/type and transaction ID into protection, cache, and lock attributes.
- Refreshed corpus-KB KG projections so the tracked suite now reports `78` fixtures / `0` failures, AMBA-family coverage reports `29/29`, and temporal fixture coverage reports `35/35`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_write_address_control_sideband_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `78` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `78` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI read-address ID stability KG fixture)

### Added: AXI read-address ID stability truthfulness fixture
- Added tracked KG fixture `axi_read_address_id_stability_gold` for AXI read-address `ARID` hold behavior with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `ARVALID`, `ARREADY`, and `ARID` through graph-backed actor relations, actor-relative ports, resolved `ARVALID` / `ARREADY` semantic roles, and one manager-grounded temporal stability rule.
- The temporal rule requires `HandshakeComplete(ARVALID, ARREADY)` and actor-grounded stability for `ARID`, completing explicit transaction-ID stability coverage across AXI address, response, and read-data channels.
- Refreshed corpus-KB KG projections so the tracked suite now reports `77` fixtures / `0` failures, AMBA-family coverage reports `28/28`, and temporal fixture coverage reports `34/34`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_read_address_id_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `77` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `77` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI write-address ID stability KG fixture)

### Added: AXI write-address ID stability truthfulness fixture
- Added tracked KG fixture `axi_write_address_id_stability_gold` for AXI write-address `AWID` hold behavior with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `AWVALID`, `AWREADY`, and `AWID` through graph-backed actor relations, actor-relative ports, resolved `AWVALID` / `AWREADY` semantic roles, and one manager-grounded temporal stability rule.
- The temporal rule requires `HandshakeComplete(AWVALID, AWREADY)` and actor-grounded stability for `AWID`, extending write-address sideband coverage into transaction identity rather than only burst length/size/type fields.
- Refreshed corpus-KB KG projections so the tracked suite now reports `76` fixtures / `0` failures, AMBA-family coverage reports `27/27`, and temporal fixture coverage reports `33/33`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_write_address_id_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `76` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `76` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI read-data ID stability KG fixture)

### Added: AXI read-data ID stability truthfulness fixture
- Added tracked KG fixture `axi_read_data_id_stability_gold` for AXI read-data `RID` hold behavior with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `RVALID`, `RREADY`, and `RID` through graph-backed actor relations, actor-relative ports, resolved `RVALID` / `RREADY` semantic roles, and one subordinate-grounded temporal stability rule.
- The temporal rule requires `HandshakeComplete(RVALID, RREADY)` and actor-grounded stability for `RID`, pairing the prior write-response `BID` identity-sideband coverage with the read-data channel identity path.
- Refreshed corpus-KB KG projections so the tracked suite now reports `75` fixtures / `0` failures, AMBA-family coverage reports `26/26`, and temporal fixture coverage reports `32/32`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_read_data_id_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `75` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `75` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI write-response ID stability KG fixture)

### Added: AXI write-response ID stability truthfulness fixture
- Added tracked KG fixture `axi_write_response_id_stability_gold` for AXI write-response `BID` hold behavior with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `BVALID`, `BREADY`, and `BID` through graph-backed actor relations, actor-relative ports, resolved `BVALID` / `BREADY` semantic roles, and one subordinate-grounded temporal stability rule.
- The temporal rule requires `HandshakeComplete(BVALID, BREADY)` and actor-grounded stability for `BID`, extending write-response coverage from payload response semantics into transaction identity sideband stability.
- Refreshed corpus-KB KG projections so the tracked suite now reports `74` fixtures / `0` failures, AMBA-family coverage reports `25/25`, and temporal fixture coverage reports `31/31`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_write_response_id_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `74` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `74` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI read-data response stability KG fixture)

### Added: AXI read-data response stability truthfulness fixture
- Added tracked KG fixture `axi_read_data_response_stability_gold` for AXI read-data `RRESP` hold behavior with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `RVALID`, `RREADY`, and `RRESP` through graph-backed actor relations, actor-relative ports, resolved `RVALID` / `RREADY` semantic roles, and one subordinate-grounded temporal stability rule.
- The temporal rule requires `HandshakeComplete(RVALID, RREADY)` and actor-grounded stability for `RRESP`, extending read-data channel stability coverage beyond `RDATA` payload stability and `RLAST` last-beat stability.
- Refreshed corpus-KB KG projections so the tracked suite now reports `73` fixtures / `0` failures, AMBA-family coverage reports `24/24`, and temporal fixture coverage reports `30/30`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_read_data_response_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `73` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `73` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI read-address sideband stability KG fixture)

### Added: AXI read-address sideband stability truthfulness fixture
- Added tracked KG fixture `axi_read_address_sideband_stability_gold` for AXI read-address `ARSIZE` / `ARBURST` hold behavior with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `ARVALID`, `ARREADY`, `ARSIZE`, and `ARBURST` through graph-backed actor relations, actor-relative ports, resolved `ARVALID` / `ARREADY` semantic roles, and two manager-grounded temporal stability rules.
- The temporal rules require `HandshakeComplete(ARVALID, ARREADY)` and actor-grounded stability for `ARSIZE` and `ARBURST`, extending read-address sideband coverage beyond the earlier `ARLEN` path.
- Refreshed corpus-KB KG projections so the tracked suite now reports `72` fixtures / `0` failures, AMBA-family coverage reports `23/23`, and temporal fixture coverage reports `29/29`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_read_address_sideband_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `72` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `72` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI write-data last stability KG fixture)

### Added: AXI write-data last-beat stability truthfulness fixture
- Added tracked KG fixture `axi_write_data_last_stability_gold` for AXI write-data `WLAST` hold behavior with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `WVALID`, `WREADY`, and `WLAST` through graph-backed actor relations, actor-relative ports, resolved `WVALID` / `WREADY` semantic roles, and one manager-grounded temporal stability rule.
- The temporal rule requires `HandshakeComplete(WVALID, WREADY)` and actor-grounded stability for `WLAST`, proving a last-beat sideband flag remains an owned manager temporal obligation rather than inert table inventory.
- Refreshed corpus-KB KG projections so the tracked suite now reports `71` fixtures / `0` failures, AMBA-family coverage reports `22/22`, and temporal fixture coverage reports `28/28`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_write_data_last_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `71` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `71` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI read-data last stability KG fixture)

### Added: AXI read-data last-beat stability truthfulness fixture
- Added tracked KG fixture `axi_read_data_last_stability_gold` for AXI read-data `RLAST` hold behavior with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `RVALID`, `RREADY`, and `RLAST` through graph-backed actor relations, actor-relative ports, resolved `RVALID` / `RREADY` semantic roles, and one subordinate-grounded temporal stability rule.
- The temporal rule requires `HandshakeComplete(RVALID, RREADY)` and actor-grounded stability for `RLAST`, proving a last-beat sideband flag remains an owned subordinate temporal obligation rather than inert table inventory.
- Refreshed corpus-KB KG projections so the tracked suite now reports `70` fixtures / `0` failures, AMBA-family coverage reports `21/21`, and temporal fixture coverage reports `27/27`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_read_data_last_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `70` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `70` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI write-address sideband stability KG fixture)

### Added: AXI write-address sideband stability truthfulness fixture
- Added tracked KG fixture `axi_write_address_sideband_stability_gold` for AXI write-address sideband hold behavior with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `AWVALID`, `AWREADY`, `AWLEN`, `AWSIZE`, and `AWBURST` through graph-backed actor relations, actor-relative ports, resolved `AWVALID` / `AWREADY` semantic roles, and three manager-grounded temporal stability rules.
- The temporal rules require `HandshakeComplete(AWVALID, AWREADY)` and actor-grounded stability for `AWLEN`, `AWSIZE`, and `AWBURST`, proving non-handshake write-address sideband fields remain owned temporal obligations rather than inert table inventory.
- Refreshed corpus-KB KG projections so the tracked suite now reports `69` fixtures / `0` failures, AMBA-family coverage reports `20/20`, and temporal fixture coverage reports `26/26`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_write_address_sideband_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `69` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `69` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AHB write-data stability KG fixture)

### Added: AHB write-data stability truthfulness fixture
- Added tracked KG fixture `ahb_write_data_stability_gold` for AHB write-data hold behavior with `Manager signals` / `Subordinate signals` section context, `Name | Destination | Width | Description` table evidence, and an explicit structured signal constraint.
- The fixture locks `HSEL`, `HWRITE`, `HWDATA`, and `HREADY` through graph-backed actor relations, actor-relative ports, and one manager-grounded temporal stability rule.
- The temporal rule deliberately uses the write wait-state guard `HREADY LOW`, `HSEL HIGH`, and `HWRITE HIGH`, so the fixture requires `0` handshake-completion predicates while proving actor-grounded stability for the manager-owned write-data bus.
- Refreshed corpus-KB KG projections so the tracked suite now reports `68` fixtures / `0` failures, AMBA-family coverage reports `19/19`, and temporal fixture coverage reports `25/25`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality ahb_write_data_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `68` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `68` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AHB response stability KG fixture)

### Added: AHB response stability truthfulness fixture
- Added tracked KG fixture `ahb_response_stability_gold` for AHB wait-state read-data/response hold behavior with `Manager signals` / `Subordinate signals` section context, `Name | Destination | Width | Description` table evidence, and explicit structured signal constraints.
- The fixture locks `HSEL`, `HREADY`, `HRDATA`, and `HRESP` through graph-backed actor relations, actor-relative ports, and two subordinate-grounded temporal stability rules.
- The temporal rules deliberately use the stalled-transfer guard `HREADY LOW` and `HSEL HIGH`, so the fixture requires `0` handshake-completion predicates while still proving actor-grounded stability for subordinate-owned response outputs.
- Refreshed corpus-KB KG projections so the tracked suite now reports `67` fixtures / `0` failures, AMBA-family coverage reports `18/18`, and temporal fixture coverage reports `24/24`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality ahb_response_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `67` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `67` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AHB control stability KG fixture)

### Added: AHB control stability truthfulness fixture
- Added tracked KG fixture `ahb_control_stability_gold` for AHB wait-state address/control hold behavior with `Manager signals` / `Subordinate signals` section context, `Name | Destination | Width | Description` table evidence, and explicit structured signal constraints.
- The fixture locks `HADDR`, `HWRITE`, `HSIZE`, `HBURST`, `HPROT`, `HSEL`, and `HREADY` through graph-backed actor relations, actor-relative ports, and five manager-grounded temporal stability rules.
- The temporal rules deliberately use the stalled-transfer guard `HREADY LOW` and `HSEL HIGH`, so the fixture requires `0` handshake-completion predicates while still proving actor-grounded stability for manager-owned address/control outputs.
- Refreshed corpus-KB KG projections so the tracked suite now reports `66` fixtures / `0` failures, AMBA-family coverage reports `17/17`, and temporal fixture coverage reports `23/23`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality ahb_control_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `66` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `66` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (APB response stability KG fixture)

### Added: APB response stability truthfulness fixture
- Added tracked KG fixture `apb_response_stability_gold` for APB completion-side response hold behavior with `Signal | Source | Width | Description` table evidence plus explicit structured signal constraints.
- The fixture locks `PSEL`, `PENABLE`, `PREADY`, `PRDATA`, and `PSLVERR` through graph-backed actor relations, actor-relative ports, resolved `PSEL` / `PREADY` semantic roles, and two completer-grounded temporal stability rules.
- Unlike the APB wait-state write-control fixture, this fixture expects `HandshakeComplete(PSEL, PREADY)` because `PREADY` is `HIGH`; this makes the pair an executable contrast between wait-state stability and completed-transfer response stability.
- Refreshed corpus-KB KG projections so the tracked suite now reports `65` fixtures / `0` failures, AMBA-family coverage reports `16/16`, and temporal fixture coverage reports `22/22`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality apb_response_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `65` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `65` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (APB write-control stability KG fixture)

### Added: APB write-control stability truthfulness fixture
- Added tracked KG fixture `apb_write_control_stability_gold` for APB wait-state control/data hold behavior with `Signal | Source | Width | Description` table evidence plus explicit structured signal constraints.
- The fixture locks `PSEL`, `PENABLE`, `PREADY`, `PWRITE`, `PWDATA`, and `PSTRB` through graph-backed actor relations, actor-relative ports, resolved `PSEL` / `PREADY` semantic roles, and three actor-grounded temporal stability rules.
- The temporal rules deliberately require no `HandshakeComplete` predicate because the guard is the APB wait-state shape `PSEL HIGH`, `PENABLE HIGH`, and `PREADY LOW`; the value is in proving stable requester-owned control/data sidebands without pretending a transfer completed.
- Refreshed corpus-KB KG projections so the tracked suite now reports `64` fixtures / `0` failures, AMBA-family coverage reports `15/15`, and temporal fixture coverage reports `21/21`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality apb_write_control_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `64` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `64` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI sideband stability KG fixture)

### Added: AXI sideband stability truthfulness fixture
- Added tracked KG fixture `axi_sideband_stability_gold` for AXI read-address and write-data sideband hold behavior with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `ARVALID`, `ARREADY`, `ARLEN`, `WVALID`, `WREADY`, and `WSTRB` through graph-backed actor ports, resolved valid-like / ready-like semantic roles, single-source semantic grounding, and two actor-grounded handshake-stability temporal rules for `ARLEN` and `WSTRB`.
- Tightened corpus-KB fixture-family labeling so stability-hold fixtures count under temporal semantics, not only protocol-family coverage.
- Refreshed corpus-KB KG projections so the tracked suite now reports `63` fixtures / `0` failures, AMBA-family coverage reports `14/14`, and temporal fixture coverage reports `20/20`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_sideband_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `63` passed / `0` failed fixtures
- `cargo test --manifest-path Cargo.toml --lib corpus_kb::tests::kg_fixture_family_labels_are_deterministic_and_review_facing -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `63` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI read-address timing KG fixture)

### Added: AXI read-address timing truthfulness fixture
- Added tracked KG fixture `axi_read_address_timing_gold` for an AXI read-address channel with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `ARVALID`, `ARREADY`, `ARADDR`, and `ARLEN` through graph-backed actor ports, resolved valid-like / ready-like semantic roles, single-source semantic grounding, next-cycle read-address ready assertion, and `ARADDR` stability across an `ARVALID` / `ARREADY` handshake.
- Refreshed corpus-KB KG projections so the tracked suite now reports `62` fixtures / `0` failures, AMBA-family coverage reports `13/13`, and temporal fixture coverage reports `19/19`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_read_address_timing_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `62` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `62` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI write-data timing KG fixture)

### Added: AXI write-data timing truthfulness fixture
- Added tracked KG fixture `axi_write_data_timing_gold` for an AXI write-data channel with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `WVALID`, `WREADY`, `WDATA`, and `WSTRB` through graph-backed actor ports, resolved valid-like / ready-like semantic roles, single-source semantic grounding, next-cycle write-data ready assertion, and `WDATA` stability across a `WVALID` / `WREADY` handshake.
- Refreshed corpus-KB KG projections so the tracked suite now reports `61` fixtures / `0` failures, AMBA-family coverage reports `12/12`, and temporal fixture coverage reports `18/18`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_write_data_timing_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `61` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `61` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI read-data timing KG fixture)

### Added: AXI read-data timing truthfulness fixture
- Added tracked KG fixture `axi_read_data_timing_gold` for an AXI read-data channel with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `RVALID`, `RREADY`, `RDATA`, and `RRESP` through graph-backed actor ports, resolved valid-like / ready-like semantic roles, single-source semantic grounding, next-cycle read-data valid assertion, and `RDATA` stability across an `RVALID` / `RREADY` handshake.
- Refreshed corpus-KB KG projections so the tracked suite now reports `60` fixtures / `0` failures, AMBA-family coverage reports `11/11`, and temporal fixture coverage reports `17/17`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_read_data_timing_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `60` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `60` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-15 (AXI write-response timing KG fixture)

### Added: AXI write-response timing truthfulness fixture
- Added tracked KG fixture `axi_write_response_timing_gold` for an AXI write-response channel with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `BVALID`, `BREADY`, and `BRESP` through graph-backed actor ports, resolved valid-like / ready-like semantic roles, single-source semantic grounding, next-cycle response assertion, and `BRESP` stability across a `BVALID` / `BREADY` handshake.
- Refreshed corpus-KB KG projections so the tracked suite now reports `59` fixtures / `0` failures, AMBA-family coverage reports `10/10`, and temporal fixture coverage reports `16/16`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_write_response_timing_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `59` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `59` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-14 (GitHub Actions temporarily manual-only)

### Changed: hosted CI no longer auto-runs on push or pull request
- `.github/workflows/ci.yml` is temporarily restricted to `workflow_dispatch` to conserve the account's remaining GitHub Actions minutes.
- The workflow still exists and still delegates to `./scripts/run_ci.sh`, so it can be run manually from GitHub when hosted validation is explicitly desired.
- Local validation remains unchanged: `bash scripts/run_ci.sh` is still the canonical Rust + docs gate to run before commits or before any future push.

### Validation
- `git diff --check` -> passed
- `rg -n '^on:|workflow_dispatch|push:|pull_request:' .github/workflows/ci.yml` -> passed with only the workflow root and `workflow_dispatch` trigger key present
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-14 (KG bench asserts semantic grounding strength)

### Added: canonical semantic-grounding strength expectations in KG fixtures
- `kg-bench` fixtures can now assert canonical `InterfaceSignalRecord.semantic_grounding_strength` directly at the `SemanticIR` and `IntentIR` stages.
- The new `semantic_grounding_strengths_include` matcher checks a signal name plus expected strength such as `single_source`, `multi_source`, or `cross_modality`, instead of relying only on aggregate validation counters.
- Strengthened `cross_modality_semantic_grounding_gold`, `vlm_timing_semantic_grounding_gold`, `visual_semantic_prior_guided_caption_gold`, and `apb_requester_completer_handshake_gold` so multimodal, VLM timing-note, visual-prior, and APB handshake paths lock exact grounding-strength shape.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality cross_modality_semantic_grounding_gold vlm_timing_semantic_grounding_gold visual_semantic_prior_guided_caption_gold apb_requester_completer_handshake_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `58` passed / `0` failed fixtures
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed with the tracked fixture suite reporting `58` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-14 (KG bench asserts resolved semantic-role shape)

### Added: canonical resolved semantic-role expectations in KG fixtures
- `kg-bench` fixtures can now assert canonical `InterfaceSignalRecord.resolved_semantic_role` directly at the `SemanticIR` and `IntentIR` stages.
- The new `resolved_semantic_roles_include` matcher checks a signal name plus expected role such as `handshake_valid_like` or `handshake_ready_like`, instead of only proving that some semantic role was resolved.
- Strengthened `cross_modality_semantic_grounding_gold`, `vlm_timing_semantic_grounding_gold`, `visual_semantic_prior_guided_caption_gold`, and `apb_requester_completer_handshake_gold` so table, visual-caption, VLM timing-note, prior-guided visual, and APB handshake paths lock exact valid-like / ready-like canonical role shape.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality cross_modality_semantic_grounding_gold vlm_timing_semantic_grounding_gold visual_semantic_prior_guided_caption_gold apb_requester_completer_handshake_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `58` passed / `0` failed fixtures
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed with the tracked fixture suite reporting `58` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-14 (KG bench asserts resolved polarity shape)

### Added: canonical resolved signal-polarity expectations in KG fixtures
- `kg-bench` fixtures can now assert canonical `InterfaceSignalRecord.resolved_polarity` directly at the `SemanticIR` and `IntentIR` stages.
- The new `signal_polarities_include` matcher checks a signal name plus expected `active_high` / `active_low` polarity instead of relying only on `with_resolved_polarity` validation counts.
- Strengthened `non_reset_control_polarity_gold`, `multi_control_polarity_gold`, and `mixed_control_polarity_gold` so explicit asserted-when-level prose, collective active-low prose, and mixed clause-local active-low/active-high prose lock the exact resolved canonical polarity shape.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality non_reset_control_polarity_gold multi_control_polarity_gold mixed_control_polarity_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `58` passed / `0` failed fixtures
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed with the tracked fixture suite reporting `58` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-14 (KG bench asserts polarity conflict shape)

### Added: canonical signal-polarity conflict expectations in KG fixtures
- `kg-bench` fixtures can now assert canonical `signal_polarity_conflicts` directly at the `SemanticIR` and `IntentIR` stages.
- The new `signal_polarity_conflicts_include` matcher supports partial checks for conflict signal, optional conflict id, and included observations by polarity, source kind, supporting statement ids, and supporting table ids.
- Added tracked fixture `control_polarity_conflict_negative`, proving a `PRESETN` active-high prose observation and active-low signal-description-table observation remain an explicit carried polarity conflict instead of forcing a winner or hiding behind aggregate conflict counts.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality control_polarity_conflict_negative` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `58` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `58` fixtures / `0` failures
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed with the tracked fixture suite reporting `58` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-13 (KG bench asserts interface conflict shape)

### Added: canonical interface-signal conflict expectations in KG fixtures
- `kg-bench` fixtures can now assert canonical `interface_signal_conflicts` directly at the `SemanticIR` and `IntentIR` stages.
- The new `interface_signal_conflicts_include` matcher supports partial checks for conflict signal, optional conflict id, conflict kind, and included observation values plus supporting statement ids.
- Strengthened `negative_knowledge_prior_guided_interface_conflict_caution_gold` so it locks the `DATA` conflict shape directly: `direction_mismatch` preserves `input` / `output` observations, `width_mismatch` preserves `8` / `16` observations, and prior-memory caution still cannot mutate that local interface-shape conflict away.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality negative_knowledge_prior_guided_interface_conflict_caution_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `57` passed / `0` failed fixtures
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed with the tracked fixture suite reporting `57` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-13 (KG bench asserts connectivity conflict shape)

### Added: canonical signal-connectivity conflict expectations in KG fixtures
- `kg-bench` fixtures can now assert canonical `signal_connectivity_conflicts` directly at the `SemanticIR` and `IntentIR` stages.
- The new `signal_connectivity_conflicts_include` matcher supports partial checks for conflict signal, optional conflict id, conflict kind, included conflicting actor ids/names, and supporting statement ids.
- Strengthened `multi_producer_conflict_negative` and `negative_knowledge_prior_guided_connectivity_conflict_caution_gold` so they lock the `PREADY` multiple-producer conflict shape directly: `Completer` and `Monitor` both drive `PREADY`, and prior-memory caution still cannot mutate that local graph conflict away.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality multi_producer_conflict_negative` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality negative_knowledge_prior_guided_connectivity_conflict_caution_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `57` passed / `0` failed fixtures
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed with the tracked fixture suite reporting `57` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-13 (KG bench asserts semantic conflict shape)

### Added: canonical signal-semantic conflict expectations in KG fixtures
- `kg-bench` fixtures can now assert canonical `signal_semantic_conflicts` directly at the `SemanticIR` and `IntentIR` stages.
- The new `signal_semantic_conflicts_include` matcher supports partial checks for conflict signal, optional conflict id, and included conflict observations by semantic tag, source kind, source text, and supporting statement/table/visual evidence ids.
- Strengthened `visual_sources_semantic_conflict_negative` so it locks the multimodal disagreement shape directly: `XCTRL` has visual-caption evidence for `handshake_valid_like` and VLM timing-diagram annotation evidence for `handshake_ready_like`, with arbitration remaining non-decisive instead of forcing consensus.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality visual_sources_semantic_conflict_negative` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `57` passed / `0` failed fixtures
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed with the tracked fixture suite reporting `57` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-13 (KG bench asserts temporal conflict shape)

### Added: canonical temporal-conflict expectations in KG fixtures
- `kg-bench` fixtures can now assert canonical `temporal_conflicts` directly at the `SemanticIR` and `IntentIR` stages.
- The new `temporal_conflicts_include` matcher supports partial checks for signal name, phase, clock signal, edge, cycle window, antecedent predicates, conflicting values, supporting rule ids, and supporting statement ids.
- Strengthened `negative_knowledge_prior_guided_temporal_conflict_caution_gold` so it locks the current-document `PREADY` `HIGH` / `LOW` contradiction shape directly while still proving prior-memory caution only adds rescan/corroboration guidance instead of mutating the conflict away.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality negative_knowledge_prior_guided_temporal_conflict_caution_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `57` passed / `0` failed fixtures
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed with the tracked fixture suite reporting `57` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-13 (KG bench asserts temporal rule shape)

### Added: canonical temporal-rule expectations in KG fixtures
- `kg-bench` fixtures can now assert canonical `temporal_rules` directly at the `SemanticIR` and `IntentIR` stages.
- The new `temporal_rules_include` matcher supports partial checks for source text, clock signal, edge, cycle window, supporting statement ids, antecedent predicates, and consequent predicates.
- Strengthened the representative AXI next-cycle, APB setup/access, and AHB wait-state timing fixtures so they lock actor-grounded drive/stability predicates, compound guard predicates, handshake-completion predicates, and one-cycle windows as typed IR instead of only aggregate validation metrics.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_next_cycle_timing_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality apb_setup_access_timing_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality ahb_wait_state_timing_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `57` passed / `0` failed fixtures
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed with the tracked fixture suite reporting `57` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-13 (KG bench asserts clock/reset infrastructure topology)

### Added: canonical infrastructure expectations in KG fixtures
- `kg-bench` fixtures can now assert canonical `infrastructure_signals` and `infrastructure_topology` records directly at the `SemanticIR` and `IntentIR` stages.
- Added tracked fixture `clock_reset_topology_gold`, proving explicit current-document clock-gate, reset-synchronizer, and reset-tree topology survives canonically while generic clock-gate/synchronizer advice does not inflate topology counts.
- Refreshed the corpus-KB benchmark and infrastructure/polarity fixture pages from the now `57`-fixture KG suite.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality clock_reset_topology_gold` -> passed
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed with the tracked fixture suite reporting `57` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `57` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `57` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-13 (VLM timing filters motion-only annotations)

### Fixed: motion-only VLM timing annotations no longer become timing constraints
- `SemanticIR` now treats non-quantitative waveform-motion prose in VLM timing `annotations[]`, such as `XREQ rises, remains stable, then falls`, as visual markup rather than a typed timing constraint.
- The filter is deliberately conservative: setup/hold/delay/timing terms, explicit temporal relation words, and numeric/cycle-bearing annotations remain eligible for timing extraction instead of being blanket-suppressed.
- Added tracked KG fixture `vlm_timing_motion_annotation_negative`, proving motion-only annotations stay out of `TimingConstraintRecord`s while a concrete document-grounded `HIGH` sample still becomes typed temporal evidence.
- Refreshed the corpus-KB benchmark, timing, visual, and semantic/truthfulness pattern pages from the now `56`-fixture KG suite.

### Validation
- `cargo test --manifest-path Cargo.toml --lib vlm_timing_diagram_observation_rejects_waveform_motion_states -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml --lib vlm_timing_diagram_observation_accepts_fenced_json_with_trailing_prose -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_motion_annotation_negative` -> passed after failing before the parser fix with `timing_constraints = 2`
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_semantic_grounding_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `56` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `56` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `313` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g prior-candidate readiness manifest)

### Added: fixture-surface readiness for prior candidates
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes `corpus_kb/prior_candidates/kg-fixture-candidates.json` beside the Markdown prior-candidate page.
- The JSON manifest is schema-versioned and records candidate kind, target `CorpusMemory` schema, readiness, fixture counts, supporting/positive/guard fixture names, gate identifiers, and the explicit non-mutation promotion boundary.
- `corpus_kb/prior_candidates/kg-fixture-candidates.md` now includes a `Readiness Summary` table, with paired prior families marked as `fixture_paired_review_ready` and the caution-only negative-knowledge family marked as `caution_surface_review_ready`.
- The manifest and table remain review-only corpus-KB artifacts. They do not approve individual priors, write `generated/prior_memory/corpus_memory.json`, or mutate canonical IR.

### Validation
- `cargo fmt --all` -> passed
- `cargo test --manifest-path Cargo.toml --lib corpus_kb -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed the aggregate, fixture-family, Markdown prior-candidate, and JSON prior-candidate corpus-KB artifacts with `55` passed / `0` failed fixtures
- `git diff --check` -> passed
- `cargo fmt --all --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `313` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g typed prior-memory corpus KB page)

### Added: dedicated typed prior-memory fixture-family page
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes `corpus_kb/prior_memory/kg-fixtures.md` from KG fixtures tagged as `typed prior memory`.
- Added `corpus_kb/prior_memory/README.md` to define this page family as reviewable synthesis for prior-guided gold/negative pairs, caution-only negative knowledge, local-grounding boundaries, and future `CorpusMemory` benchmark gaps.
- The live refresh currently projects `21` passing typed-prior-memory fixtures with explicit fixture-path provenance, while preserving human synthesis outside the managed block.
- This page remains non-promoting corpus knowledge: it is not `generated/prior_memory/corpus_memory.json`, does not write `CorpusMemory`, and cannot mutate canonical IR.

### Validation
- `cargo fmt --all` -> passed
- `cargo test --manifest-path Cargo.toml --lib corpus_kb -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed the aggregate, typed prior-memory, fixture-family, and prior-candidate corpus-KB pages with `55` passed / `0` failed fixtures
- `git diff --check` -> passed
- `cargo fmt --all --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `313` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g state-machine corpus KB page)

### Added: dedicated state-machine fixture-family page
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes `corpus_kb/state_machines/kg-fixtures.md` from KG fixtures tagged as `VLM state machines`.
- Added `corpus_kb/state_machines/README.md` to define this page family as reviewable synthesis for VLM state labels, transition endpoint grounding, duplicate state merging, initial marker handling, and initial-cardinality validation behavior.
- The live refresh currently projects `5` passing state-machine fixtures with explicit fixture-path provenance, while preserving human synthesis outside the managed block.
- This page remains non-promoting corpus knowledge: it does not change KG-bench execution, validation scoring, canonical IR, typed prior memory, or adapter lowering.

### Validation
- `cargo fmt --all` -> passed
- `cargo test --manifest-path Cargo.toml --lib corpus_kb -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed the aggregate, state-machine, fixture-family, and prior-candidate corpus-KB pages with `55` passed / `0` failed fixtures
- `git diff --check` -> passed
- `cargo fmt --all --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `313` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g prior-candidate gate matrix)

### Added: review-gated prior-candidate surface
- `corpus_kb/prior_candidates/kg-fixture-candidates.md` now includes `review_scope: family_surface_not_individual_prior` so the managed projection cannot be mistaken for individual prior promotion.
- The managed prior-candidate block now emits a `Promotion Gate Review Matrix` covering `schema_gate`, `fixture_gate`, `harvest_gate`, `consumer_gate`, and the non-mutation `promotion_boundary` for every candidate family.
- The gate matrix reflects already visible implementation surfaces for the seven `CorpusMemory` families while keeping the corpus-KB page review-only: it does not write prior memory, approve a prior record, or mutate canonical IR.

### Validation
- `cargo fmt --all` -> passed
- `cargo test --manifest-path Cargo.toml --lib corpus_kb -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed the prior-candidate corpus-KB page with `55` passed / `0` failed fixtures
- `git diff --check` -> passed
- `cargo fmt --all --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `313` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g semantic/truthfulness corpus KB patterns)

### Added: semantic and truthfulness pattern page family
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes `corpus_kb/patterns/kg-fixtures.md` from KG fixtures tagged as actor/connectivity, semantic role arbitration, negative knowledge, truthfulness negatives/cautions, or residual/caveat patterns.
- Added `corpus_kb/patterns/README.md` to define this page family as reviewable synthesis for why candidate facts are accepted, contested, blocked, or left as residuals.
- The live refresh currently projects `42` passing semantic/truthfulness pattern fixtures with explicit fixture-path provenance, while preserving human synthesis outside the managed block.
- This page remains non-promoting corpus knowledge: it does not change KG-bench execution, validation scoring, canonical IR, or typed prior memory.

### Validation
- `cargo fmt --all` -> passed
- `cargo test --manifest-path Cargo.toml --lib corpus_kb -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed the aggregate, semantic/truthfulness pattern, fixture-family, and prior-candidate corpus-KB pages with `55` passed / `0` failed fixtures
- `git diff --check` -> passed
- `cargo fmt --all --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `313` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g prior-candidate corpus KB projection)

### Added: review-only prior-candidate bridge
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes `corpus_kb/prior_candidates/kg-fixture-candidates.md` from KG fixtures that already encode prior-guided gold/negative/caution behavior.
- The candidate projection groups fixture-backed candidates for `actor_taxonomy_prior`, `semantic_phrase_prior`, `semantic_modality_reliability_prior`, `temporal_phrase_prior`, `table_shape_prior`, `visual_motif_prior`, and `negative_knowledge_prior`.
- Each candidate row records the target `CorpusMemory` schema surface, supporting fixture count, positive fixtures, guard/caution fixtures, and required promotion gates.
- The page is explicitly non-promoting: `promotion_status` is `candidate_not_promoted_review_required`, `canonical_mutation_allowed` is `false`, and `corpus_memory_mutation_allowed` is `false`.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib corpus_kb -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed the aggregate, fixture-family, and prior-candidate corpus-KB pages with `55` passed / `0` failed fixtures
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `313` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g dedicated corpus KB fixture-family pages)

### Added: dedicated KG fixture-derived page families
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes the aggregate benchmark page plus dedicated managed corpus-KB pages for table, visual, timing, infrastructure, and AMBA-family fixture patterns.
- Added tracked page-family roots and READMEs under `corpus_kb/tables/`, `corpus_kb/visuals/`, `corpus_kb/timing/`, `corpus_kb/infra/`, and `corpus_kb/protocols/`.
- Refreshed the new managed pages from the tracked KG suite: table fixtures `8/8`, visual fixtures `18/18`, timing fixtures `14/14`, infrastructure/polarity fixtures `6/6`, and AMBA-family fixtures `9/9`, all with explicit fixture-path provenance.
- The new pages remain review-only corpus synthesis. They preserve human synthesis outside managed blocks and do not change KG-bench execution, canonical IR, validation scoring, or typed prior memory.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib corpus_kb -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed the aggregate plus dedicated KG fixture-family corpus-KB pages with `55` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `313` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g KG fixture-family corpus KB summary)

### Added: fixture-family benchmark synthesis
- `specforge corpus-kb --kg-fixtures-root ...` now projects a managed fixture-family summary table above the per-fixture KG benchmark results.
- The family summary is review-facing only: fixtures can appear in multiple orthogonal families, and the table does not mutate canonical IR, typed priors, or the executable `kg-bench` gate.
- Refreshed `corpus_kb/benchmarks/kg-fixtures.md` so the tracked suite now reports `55` total fixtures, `55` passed, `0` failed, plus family coverage for VLM timing/state-machine, actor connectivity, multimodal visual grounding, negative knowledge, polarity, AMBA-family protocols, semantic arbitration, table hygiene, temporal semantics, truthfulness negatives, and typed prior memory.
- Added focused corpus-KB coverage proving the managed KG projection preserves human synthesis while emitting family summaries.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib corpus_kb -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed `corpus_kb/benchmarks/kg-fixtures.md` with `55` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `313` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g quiet KG fixture validation path)

### Changed: benchmark validation can run quietly
- Added an internal quiet validation entrypoint so KG fixture execution can still persist validation sidecars/backannotations without printing full stage reports for every fixture.
- Switched `kg-bench` fixture evaluation to use that quiet path, which also keeps `specforge corpus-kb --kg-fixtures-root ...` concise when it projects KG fixture outcomes into the corpus knowledge base.
- Added a focused guard test proving the quiet-validation output flag restores its prior state after use.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib quiet_validation_output_guard_restores_previous_state -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml --lib kg_bench_reports_fixture_failure -- --nocapture` -> passed with concise fixture-failure output
- `cargo test --manifest-path Cargo.toml --lib corpus_kb_refreshes_kg_fixture_results_without_replacing_human_synthesis -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed with concise corpus-KB refresh output and `55` passed / `0` failed fixtures
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed, including the full `55`-fixture tracked KG suite
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `312` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g KG fixture-result corpus KB projection)

### Added: benchmark-result page family
- Added `corpus_kb/benchmarks/` with a page-family README and the managed `kg-fixtures.md` projection.
- The new page records the tracked KG fixture suite outcome as reviewable corpus synthesis: `55` total fixtures, `55` passed, and `0` failed, with fixture paths kept as provenance.
- The page preserves human-authored synthesis outside the managed block and remains guidance rather than canonical truth promotion.

### Changed: `specforge corpus-kb`
- `specforge corpus-kb` now accepts optional `--kg-fixtures-root <fixture-root>` and repeated `--kg-fixture <fixture>` selectors, so it can refresh benchmark-result pages independently of validation-report pages.
- `kg_bench` now exposes an internal fixture-outcome collection seam reused by the corpus-KB projection while preserving the standalone `specforge kg-bench` command behavior.
- Documented the then-open follow-up quality caveat that the benchmark projection reused validation paths that printed detailed stage reports during fixture execution.

### Validation
- `cargo fmt --all` -> passed
- `cargo test --manifest-path Cargo.toml --lib corpus_kb -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed, including the full `55`-fixture tracked KG suite
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed `corpus_kb/benchmarks/kg-fixtures.md`
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `311` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g corpus knowledge base bootstrap)

### Added: tracked corpus knowledge-base plane
- Added `corpus_kb/` with a schema/policy root and the first `failures/` page family for reviewable cross-document synthesis.
- Seeded `corpus_kb/failures/validation-findings.md` from the current four projected AMBA `IntentIR` validation reports, preserving human synthesis outside the managed block.

### Added: `specforge corpus-kb`
- Added a new CLI command that refreshes the managed validation-finding block from validation report sidecars.
- The command preserves existing human-authored notes and only replaces the managed block, so corpus KB pages remain guidance and synthesis rather than canonical truth promotion.
- Added focused coverage for CLI parsing and managed-block preservation.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib corpus_kb -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --repo-root . generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/validation_report.json generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/validation_report.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/validation_report.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/validation_report.json` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `307` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-12 (VLM timing filters signal bit-select annotation labels)

### Fixed: standalone signal index labels stay out of timing constraints
- `SemanticIR` now treats standalone VLM timing annotation labels such as `XREQ[0]`, `XREQ<1>`, and `XREQ[3:0]` as waveform/bit-select markup when they appear only in `annotations[]`.
- The filter remains scoped to standalone timing annotations: grounded `signals[].values[]` observations for `XREQ` still become typed signal constraints and temporal rules when they carry concrete sampled values such as `LOW` and `HIGH`.
- Strengthened the existing `vlm_timing_spurious_annotation_negative` fixture and direct semantic regression again, deepening the same 55-fixture KG-quality surface rather than adding a redundant fixture.

### Validation
- `cargo test --manifest-path Cargo.toml --lib vlm_timing_diagram_observation_rejects_label_only_noise -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_spurious_annotation_negative` -> passed
- `cargo fmt --all --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `305` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-12 (README bootstrap refresh)

### Changed: live bootstrap docs match the current Rust surface
- Executed the README -> `SESSION_BOOTSTRAP.md` handoff, re-read the referenced live docs, and compared their current claims against the active Rust codebase.
- Refreshed the README implementation-path map so it includes the active command modules for `doctor`, `converge`, `enrich`, `validate`, `rescan-plan`, `learn-priors`, `nlp-enrich`, plus `commands/mod.rs`, `test_support.rs`, and `ir/prior_memory.rs`.
- Refreshed `RUST_CODEBASE_ANALYSIS.md` and `MEMORY.md` so the current continuity baseline reflects commit `76274f6`, `29` Rust source files, `56,024` Rust source lines, `55` tracked KG-quality fixtures, and the latest `305`-test full-CI baseline.
- Normalized old checkout-specific repo-internal markdown links in the tracked changelog/memory surface to repo-relative paths.

### Validation
- repo-internal absolute checkout path scan across tracked markdown -> passed with no matches after the path-policy cleanup
- `git diff --check` -> passed
- `cargo fmt --all --check` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `305` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-12 (NLP alias learning rejects list markers)

### Fixed: Form 2 alias learning rejects broader markdown/list prefixes
- `extract_alias_phrase()` now rejects alias subjects beginning with common markdown bullets, block quotes, and ordered-list markers such as `*`, `+`, `>`, `1.`, and `2)` in addition to the existing `-`, `|`, and `#` guards.
- This prevents `specforge nlp-enrich` from learning garbage aliases from formatted list/table text while leaving ordinary prose aliases such as `address bus` unchanged.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib extract_alias_phrase_rejects_markdown_marker_prefixes -- --nocapture` -> passed
- `cargo clippy --manifest-path Cargo.toml --all-targets -- -D warnings` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `305` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-12 (VLM timing filters bracketed sample labels)

### Fixed: bracketed waveform sample labels stay out of timing constraints
- `SemanticIR` now treats bracketed VLM timing annotation labels such as `D[0]`, `A[1]`, `DATA[3]`, and `ADDR[7]` as low-value waveform/sample markup when they appear as standalone annotations.
- Strengthened `vlm_timing_spurious_annotation_negative` and the direct semantic regression again so bracketed bus/sample labels do not become `TimingConstraintRecord`s while grounded signal samples still produce typed temporal evidence.
- The tracked KG-quality fixture count remains `55`; this slice deepens the existing spurious-annotation negative fixture.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib vlm_timing_diagram_observation_rejects_label_only_noise -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_spurious_annotation_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `305` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-12 (VLM timing filters compact sample labels)

### Fixed: compact waveform sample labels stay out of timing constraints
- `SemanticIR` now treats compact VLM timing annotation labels such as `D0`, `A1`, `DATA0`, and `0xAA` as low-value waveform/sample markup when they appear as standalone annotations.
- Strengthened the existing `vlm_timing_spurious_annotation_negative` KG fixture and the direct semantic regression so those labels do not become `TimingConstraintRecord`s while real signal samples still produce typed temporal evidence.
- The tracked KG-quality fixture count remains `55`; this slice deepens an existing negative fixture rather than adding a duplicate case.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib vlm_timing_diagram_observation_rejects_label_only_noise -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_spurious_annotation_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `305` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-12 (Validation flags missing FSM initial states)

### Changed: missing initial states are covered like multiple initial states
- Added `vlm_state_machine_missing_initial_negative`, proving a VLM state-machine extraction with `IDLE` and `BUSY` but no initial marker keeps the state graph visible while validation flags the unsafe cardinality at both `SemanticIR` and `IntentIR`.
- Added a direct validation regression for explicit state declarations with zero initial states, sharing the same staged IR build helper as the multiple-initial regression.
- The tracked KG-quality fixture count is now `55`.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib validate_semantic_and_intent_ir_flag_missing_initial_state -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_state_machine_missing_initial_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `305` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-12 (Validation flags bad FSM initial cardinality)

### Changed: state machines with zero or multiple initial states are validation-visible
- `specforge validate` now emits `semantic_state_machine_initial_cardinality` and `intent_state_machine_initial_cardinality` warnings when a canonical state graph exists but does not have exactly one initial state.
- Added `vlm_state_machine_multiple_initial_negative`, proving a VLM state-machine extraction with both `IDLE` and `BUSY` marked initial keeps the graph visible while validation flags the unsafe initial-state cardinality at both `SemanticIR` and `IntentIR`.
- Added a direct validation regression for explicit state declarations with two initial states, so the warning is not only covered through the KG fixture path.

### Validation
- `cargo test --manifest-path Cargo.toml --lib validate_semantic_and_intent_ir_flag_multiple_initial_states -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_state_machine_multiple_initial_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 304 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-12 (Validation counts initial FSM states)

### Changed: validation makes initial-state cardinality visible
- `specforge validate` now reports `initial_regular_states` for both `SemanticIR` and `IntentIR`, next to the existing `regular_states` and `state_transitions` metrics.
- The `vlm_state_machine_duplicate_initial_gold` fixture now asserts that duplicate VLM state labels still leave exactly one canonical initial state after merge, not merely that `IDLE` appears in the initial-state name set.
- The mdBook KG-bench chapter documents this stronger validation surface for VLM state-machine truthfulness.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib validate_semantic_ir_artifact_reports_without_error -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_state_machine_duplicate_initial_gold` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 303 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-12 (VLM state machine merges duplicate initial markers)

### Fixed: duplicate VLM state labels preserve initial-state evidence
- `SemanticIR` now merges duplicate VLM state-machine state labels by state name before they enter canonical `RegularStateRecord`s, preserving an `is_initial` marker if any duplicate carries it.
- This mirrors the explicit state-declaration parser and prevents a VLM output like `IDLE` non-initial followed by duplicate `IDLE` initial from losing the true initial-state marker.
- `kg-bench` can now assert initial-state names directly, and `vlm_state_machine_duplicate_initial_gold` locks that `IDLE` stays the single initial state while `BUSY` stays non-initial.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib vlm_state_machine_observation_merges_duplicate_state_initial_markers -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_state_machine_duplicate_initial_gold` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 303 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (VLM state machine gates transition endpoints)

### Fixed: VLM transitions must target declared states
- `SemanticIR` now accepts VLM state-machine transitions only when both `from` and `to` endpoints refer to state labels accepted from the same `vlm_state_machine_extraction` observation.
- This keeps identifier-shaped but undeclared VLM endpoints such as `DONE` or `RESET` from becoming canonical transition graph facts just because they look like plausible FSM state names.
- Added `vlm_state_machine_undeclared_transition_negative`, proving `IDLE` / `BUSY` states and `IDLE->BUSY` survive while `BUSY->DONE` and `RESET->IDLE` are filtered.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib vlm_state_machine_observation_rejects_undeclared_transition_endpoints -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_state_machine_undeclared_transition_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 302 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (VLM state machine rejects prose labels)

### Fixed: state-machine VLM labels must be canonical identifiers
- `SemanticIR` now parses VLM state-machine `states[].name` and transition `from` / `to` endpoints through the same identifier boundary used by explicit state syntax, so prose labels such as `IDLE state` and `ACCESS phase` cannot become canonical FSM states or transition endpoints.
- `kg-bench` can now assert canonical state names and state-transition endpoints directly, which lets tracked fixtures lock VLM FSM truthfulness without inspecting generated artifacts by hand.
- Added `vlm_state_machine_label_noise_negative`, proving valid `IDLE` / `BUSY` state evidence and `IDLE->BUSY` transition evidence survive while prose-like VLM labels are filtered.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib vlm_state_machine_observation_rejects_non_identifier_state_labels -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_state_machine_label_noise_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 301 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (VLM timing rejects compact edge spellings)

### Fixed: compact edge labels stay out of signal values
- `SemanticIR` now rejects additional VLM timing waveform-motion spellings such as `POS_EDGE`, `NEG_EDGE`, `risingedge`, `LOW2HIGH`, and `HIGH2LOW` before the symbolic-value fallback can promote them into false signal-value facts.
- Strengthened the existing semantic regression and `vlm_timing_waveform_motion_negative` fixture so the only surviving VLM-authored signal constraint remains the concrete `HIGH` sample.

### Validation
- `cargo test --manifest-path Cargo.toml --lib vlm_timing_diagram_observation_rejects_waveform_motion_states -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_waveform_motion_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 300 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (VLM timing normalizes motion spellings)

### Fixed: separator variants of waveform transitions stay non-factual
- `SemanticIR` now normalizes VLM timing motion-state spellings across spaces, underscores, and hyphens before filtering them, so values like `RISING_EDGE`, `LOW_TO_HIGH`, and `HIGH_TO_LOW` cannot slip through as symbolic `MustBeValue` facts.
- Strengthened the existing semantic regression and `vlm_timing_waveform_motion_negative` fixture so they cover both plain motion labels and identifier-shaped transition labels while still preserving the concrete `HIGH` sample.

### Validation
- `cargo test --manifest-path Cargo.toml --lib vlm_timing_diagram_observation_rejects_waveform_motion_states -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_waveform_motion_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 300 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (VLM timing rejects waveform motion as values)

### Fixed: waveform motion labels no longer become fake signal values
- `SemanticIR` VLM timing lift now rejects motion-only `signals[].values[].state` labels such as `rising`, `falling`, `stable`, and `UNCHANGED` instead of promoting them as symbolic `MustBeValue` temporal facts.
- Concrete sampled values such as `HIGH`, `LOW`, `ASSERTED`, `DEASSERTED`, `0`, and `1` still survive through the existing bounded VLM signal-value path.
- Added semantic regression coverage proving a mixed waveform sequence keeps only the concrete `XREQ == HIGH @ T1` sample.
- Added tracked KG-quality fixture `vlm_timing_waveform_motion_negative`, which keeps the VLM timing extraction visible while requiring exactly one signal constraint / temporal rule and zero semantic-role hints from motion labels.

### Validation
- `cargo test --manifest-path Cargo.toml --lib vlm_timing_diagram_observation_rejects_waveform_motion_states -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_waveform_motion_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 300 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (SemanticIR keeps interface conflicts sticky)

### Changed: canonical interface shape conflicts cannot self-heal by repetition
- `SemanticIR` interface-signal accumulation now distinguishes unknown direction/width hints from conflict-collapsed hints, so a later duplicate declaration cannot resurrect a canonical direction or width after disagreement.
- Strengthened the existing interface-conflict regression with `Signal DATA is input width 8`, `Signal DATA is output width 16`, and then `Signal DATA is input width 8` again; `DATA.direction_hint` and `DATA.width_hint` must remain unresolved while the explicit direction and width conflict records stay visible.

### Validation
- `cargo test --manifest-path Cargo.toml --lib surfaces_interface_signal_conflicts_for_conflicting_explicit_declarations -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml --lib ir::semantic::tests -- --nocapture` -> passed with 65 semantic tests
- `cargo fmt --all --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 299 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (FSM adapter locks sticky actor-port width conflicts)

### Added: width regression for sticky adapter conflicts
- Added a standalone direct `.fsm` regression proving actor-port width disagreement stays unresolved after conflict collapse: a flat `DATA_OUT` width `8`, graph-backed `DATA_OUT` width `16`, and later duplicate graph-backed width `8` must still block lowering instead of self-healing.
- This completes regression coverage for the sticky adapter inventory conflict path across both role direction and numeric width hints.

### Validation
- `cargo test --manifest-path Cargo.toml --lib standalone_dt_keeps_conflicting_actor_port_width_unresolved -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml --lib ir::adapters::tests -- --nocapture` -> passed with 26 adapter tests
- `cargo fmt --all --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 299 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (FSM adapter keeps actor-port direction conflicts sticky)

### Changed: adapter inventory conflicts cannot self-heal by repetition
- `FsmSignalCandidate` inventory evidence now distinguishes unknown direction/width hints from already-conflicted hints, so a later duplicate actor-port or interface hint cannot resurrect a value after disagreement collapsed it to unresolved.
- Added a standalone direct `.fsm` regression where one unambiguous `controller` actor repeats `DATA_OUT` as `output`, then `input`, then `output`; the adapter must keep `DATA_OUT` unresolved and block lowering instead of treating the final duplicate as truth.

### Validation
- `cargo test --manifest-path Cargo.toml --lib standalone_dt_keeps_conflicting_actor_port_direction_unresolved -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml --lib ir::adapters::tests -- --nocapture` -> passed with 25 adapter tests
- `cargo fmt --all --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 298 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (FSM adapter locks graph-backed sequential system directions)

### Added: graph-backed sequential system-contract regression
- Added a focused `.fsm` adapter regression proving standalone sequential DT lowering remains renderable when flat top-level `direction_hint` values are cleared, as long as one unambiguous `IntentIR.actor_ports` context supplies graph-backed directions for `clk`, `rst_n`, `DATA_IN`, and `ACC`.
- The test locks the system-contract path specifically: graph-backed `clk` / `rst_n` inputs must satisfy `(+system ...)` renderability and avoid the `fsm_adapter_system_contract` residual.

### Validation
- `cargo test --manifest-path Cargo.toml standalone_sequential_dt_recovers_system_directions_from_actor_ports -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with 24 adapter tests
- `cargo fmt --all --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 297 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (KG bench locks detached mixed polarity rejection)

### Added: tracked negative fixture for detached mixed polarity
- Added [detached_mixed_control_polarity_negative](crates/specforge/test_data/kg_quality/detached_mixed_control_polarity_negative/fixture.json), which proves detached wording like `CS_N is active LOW and active HIGH` does not borrow an implicit subject or resolve polarity through `SemanticIR` / `IntentIR`.
- The fixture keeps `CS_N` declared and constrained, but requires `with_resolved_polarity: 0`, zero heuristic signal records, zero polarity conflicts, and zero temporal conflicts.

### Validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench detached_mixed_control_polarity_negative` -> passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` -> passed with 49 tracked KG fixtures
- `cargo fmt --all --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 296 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (EvidenceIR recovers mixed clause-local control polarity)

### Changed: safe clause-local polarity parsing
- Added a bounded clause-local fallback for mixed active-level prose, so a statement like `CS_N is active LOW and ENABLE is active HIGH` can recover active-low polarity for `CS_N` and active-high polarity for `ENABLE`.
- Kept the fallback strict: every recovered clause must bind one polarity phrase to exactly one known signal, every mentioned signal in the statement must be recovered, and detached wording such as `CS_N is active LOW and active HIGH` stays unresolved instead of inheriting an implicit subject.
- The whole-statement detector still returns no polarity for mixed low/high text; the new recovery path is separate and only promotes facts after clause-local validation succeeds.

### Added: tracked KG fixture for mixed control polarity
- Added [mixed_control_polarity_gold](crates/specforge/test_data/kg_quality/mixed_control_polarity_gold/fixture.json), which proves mixed clause-local polarity recovery produces two canonical declared signal records, two resolved polarities, zero heuristic duplicates, and zero polarity or temporal conflicts through `SemanticIR` and `IntentIR`.

### Validation
- `cargo test --manifest-path Cargo.toml mixed_polarity_prose_recovers_clause_local_control_polarities -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml detached_mixed_polarity_prose_does_not_guess_implicit_control_polarity -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml signal_polarity_detector_accepts_asserted_when_level_phrases -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench mixed_control_polarity_gold` -> passed
- `cargo test --manifest-path Cargo.toml polarity -- --nocapture` -> passed with 25 focused polarity tests
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` -> passed with 48 tracked KG fixtures
- `cargo fmt --all --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 296 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (EvidenceIR recovers collective control polarity)

### Changed: collective active-level prose can ground multiple controls
- Broadened `EvidenceIR` prose polarity recovery so a statement like `CS_N and WE_N are active LOW signals` produces active-low polarity observations for both declared controls.
- Kept the path conservative: mixed compound prose such as `CS_N is active LOW and ENABLE is active HIGH` stays unresolved until the extractor can parse each clause safely, and polarity still is not inferred from `_N` / `_B` suffixes alone.
- Tightened `SemanticIR` interface construction so polarity-only co-mentions of already declared signals enrich the authoritative signal records instead of minting duplicate low-confidence heuristic interface records.

### Added: tracked KG fixture for collective control polarity
- Added [multi_control_polarity_gold](crates/specforge/test_data/kg_quality/multi_control_polarity_gold/fixture.json), which proves collective active-low prose recovers two resolved polarities, refines asserted/deasserted constraints correctly, and preserves zero polarity or temporal conflicts through `SemanticIR` and `IntentIR`.

### Validation
- `cargo test --manifest-path Cargo.toml collective_active_low_prose_recovers_multiple_control_polarities -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml mixed_polarity_prose_does_not_guess_collective_control_polarity -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml collective_polarity_prose_does_not_duplicate_interface_records -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench multi_control_polarity_gold` -> passed
- `cargo test --manifest-path Cargo.toml polarity -- --nocapture` -> passed with 23 focused polarity tests
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` -> passed with 47 tracked KG fixtures
- `cargo fmt --all --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 295 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (EvidenceIR recovers asserted-when-level control polarity)

### Changed: explicit polarity prose covers more control-signal wording
- Broadened `EvidenceIR` signal-polarity detection to treat local phrases like `asserted when LOW`, `LOW when asserted`, `asserted by driving LOW`, and `driven LOW to assert` as active-low evidence.
- Added the symmetric active-high phrase forms for `HIGH`.
- This remains evidence-grounded polarity recovery: it does not infer polarity from a `_N` suffix alone and it applies only when the current document explicitly says how assertion maps to a logic level.
- Tightened `SemanticIR` interface construction so a one-signal local polarity/control sentence enriches an already declared signal instead of minting a duplicate low-confidence heuristic interface record.

### Tests
- Added a detector regression for asserted-when-level wording.
- Added a non-reset `CS_N` control-signal regression proving explicit `CS_N is asserted when LOW` recovers active-low polarity and refines `CS_N must be asserted` / `CS_N must be deasserted` into LOW / HIGH constraints.
- Added a `kg-bench` fixture locking the non-reset control polarity path with no polarity or temporal conflicts.

### Validation
- `cargo test --manifest-path Cargo.toml polarity -- --nocapture` -> passed with 21 focused polarity tests
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` -> passed with 46 tracked KG fixtures
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 292 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (KG bench can assert graph-backed direction coverage)

### Added: graph-native direction expectation surface
- Added `graph_direction_signal_names_include` / `graph_direction_signal_names_exclude` to `specforge kg-bench` canonical stage expectations.
- The new expectation checks `actor_ports` directly for non-`unknown` graph direction coverage by signal name, so fixtures can lock the graph-native surface without overloading flat `signal_directions_include`.
- Existing flat direction expectations are unchanged and still check `InterfaceSignalRecord.direction_hint` only.

### Tests
- Strengthened `actor_ports_gold` so both `SemanticIR` and `IntentIR` assert that `PREADY` has graph-backed direction coverage and `PSEL` does not.

### Validation
- `cargo fmt --all --check` -> passed
- `git diff --check` -> passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` -> passed with all 45 tracked KG fixtures
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 288 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (FSM adapter recovers width-only top port directions from links)

### Changed: top-boundary lowering can use explicit link topology
- `ExplicitTopPortRecord.direction_hint` is now optional, so `SemanticIR` / `IntentIR` can preserve width-only top boundary port records instead of dropping them before the adapter sees the composition.
- The `.fsm` top-composition adapter now recovers missing top boundary port directions from explicit top-link position: a top endpoint used as a link source is a top input, and a top endpoint used as a link target is a top output.
- Recovery remains deterministic and bounded: unresolved top ports still block, conflicting explicit direction versus link topology still blocks, and the adapter only uses this for explicit `?top:name` composition topology rather than inventing actor-relative roles.

### Tests
- Added a regression proving `Top datapath port result_data is width 8` survives into canonical top composition and renders as `result_data>8` only because the explicit link `consumer.result_data -> result_data` establishes it as a top output.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml top_composition -- --nocapture` -> passed with 5 focused tests
- `cargo test --manifest-path Cargo.toml extracts_explicit_modules_and_tops_from_markdown -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with 23 adapter tests
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 288 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-11 (FSM adapter consumes graph-backed direct directions)

### Changed: standalone direct lowering uses bounded actor-relative port evidence
- Updated the standalone direct `.fsm` adapter inventory path so it can recover missing local signal directions from `IntentIR.actor_ports` when the actor-port graph relevant to the direct local signal inventory has exactly one actor context.
- Kept the direct-root rule stricter than explicit module lowering: direct roots have no module name to identify the target actor, so mixed producer/consumer graph contexts remain blocked instead of guessing a perspective.
- The direct-root overlay only strengthens signals already present in the local direct inventory; it does not add graph-only signals for standalone lowering.
- Tightened actor-port provenance merging so a conflicting graph direction with multiple supporting ids cannot accidentally restore a resolved direction after the merge collapsed the conflict to unresolved.

### Tests
- Added a regression proving standalone direct `.fsm` lowering still renders when flat `direction_hint` values are cleared but one `controller` actor supplies `DATA_IN`, `DATA_OUT`, and `ZERO_FLAG` actor-port directions.
- Added a regression proving unrelated graph-only actor ports are ignored for the direct-root context gate and are not added to the standalone signal inventory.
- Added a regression proving mixed `producer` / `consumer` actor-port context is ambiguous for a standalone direct root and remains blocked with missing canonical direction hints.
- Strengthened the top-composition graph-conflict regression with duplicate supporting provenance ids to lock the conservative conflict merge behavior.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml standalone_dt -- --nocapture` -> passed with 4 focused tests
- `cargo test --manifest-path Cargo.toml top_composition_blocks_conflicting_actor_port_directions -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with 22 adapter tests
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 287 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-11 (FSM adapter consumes graph-backed module directions)

### Changed: explicit module lowering uses actor-relative port evidence
- Updated the `.fsm` adapter so explicit module candidates overlay matching `IntentIR.actor_ports` before renderability analysis.
- If an explicit module signal has width/provenance but its flat `direction_hint` is missing, the adapter can now recover that module-local input/output role from actor-relative graph evidence for the matching module actor.
- Conflicting or non-renderable graph directions still stay conservative: `in_out` and `unknown` do not become fake `.fsm` input/output hints, and normal hint merging still collapses contradictions to `None`.
- Added a regression that clears all flat child-module directions in an explicit top composition and proves the composition still lowers when `actor_ports` provide `producer_core.output_data` as output, `consumer_core.input_data` as input, and `consumer_core.result_data` as output.
- Added a companion regression proving a conflicting graph direction for `producer_core.output_data` blocks top lowering instead of silently overriding the flat module-local direction.

### Validation
- `cargo test --manifest-path Cargo.toml top_composition -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml top_composition_recovers_child_directions_from_actor_ports -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with 19 adapter tests
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 284 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build
- `git diff --check` -> passed

## 2026-04-11 (README bootstrap handoff hygiene)

### Fixed: compatibility guide root-doc list
- Re-ran the README -> `SESSION_BOOTSTRAP.md` handoff path and checked the high-signal live docs plus current Rust CLI/module/KG-fixture surface.
- Removed stray datapath/reset example bullets from `USER_GUIDE.md`'s root-document list so the compatibility pointer no longer mixes old extraction examples into the continuity-doc inventory.
- Refreshed continuity state so `MEMORY.md` records the latest committed baseline `35a8372 test(kg): lock active-low vlm reset release`.
- Updated the bootstrap analysis note with the current Rust source-file sanity count: 28 files and about 53,858 lines under `crates/specforge/src`.

### Validation
- `cargo run -p specforge -- --help` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-11 (Active-low VLM reset release polarity is fixture-locked)

### Added: reset-release timing-annotation KG fixture
- Added tracked KG-quality fixture `vlm_timing_active_low_deassertion_equivalence_gold`.
- The fixture proves that a VLM timing diagram reporting active-low `ARESETN` as both `deasserted` and `HIGH` produces typed temporal evidence without a false temporal or polarity conflict.
- This complements `vlm_timing_active_low_assertion_equivalence_gold`, so the benchmark surface now locks both active-low reset entry (`ASSERTED` == `LOW`) and reset release (`DEASSERTED` == `HIGH`) behavior.

### Validation
- `cargo run -p specforge -- kg-bench vlm_timing_active_low_deassertion_equivalence_gold` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 282 Rust tests under `RUSTFLAGS="-D warnings"` including the tracked 45-fixture KG benchmark test, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (Rustdoc warning gate joins CI)

### Changed: Rust API docs are warning-denied
- Fixed a broken rustdoc intra-doc-link interpretation in the `ActorSignalRelation` documentation by formatting the derived `output_of(A)` / `input_of(others)` wording as code/prose instead of bracket syntax.
- Updated `scripts/run_ci.sh` so local CI runs `RUSTDOCFLAGS="-D warnings" cargo doc --manifest-path Cargo.toml --no-deps` before the mdBook build.
- The new gate preserves caller-provided `RUSTDOCFLAGS`, matching the existing Rust warning flag composition for tests.

### Validation
- `RUSTDOCFLAGS="-D warnings" cargo doc --manifest-path Cargo.toml --no-deps` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 282 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (Clippy warning gate joins CI)

### Changed: Clippy is now part of the shared quality gate
- Fixed mechanical Clippy findings across enrichment, prior learning, rescan planning, validation, adapter, evidence, semantic, intent, and Docling helper code.
- Added localized `#[expect(...)]` attributes with reasons for intentional broad IR builder/evaluator signatures and coupled semantic/evidence return surfaces instead of globally allowing those lints.
- Updated `scripts/run_ci.sh` so local CI runs `cargo clippy --manifest-path Cargo.toml --all-targets -- -D warnings` before the Rust test suite.
- Updated GitHub Actions to install the `clippy` component, so hosted CI runs the same Clippy gate via the shared script.

### Validation
- `cargo clippy --manifest-path Cargo.toml --all-targets -- -D warnings` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 282 Rust tests under `RUSTFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (CI now denies Rust warnings)

### Changed: warning-clean baseline is enforced
- Updated `scripts/run_ci.sh` so the Rust test step runs with `RUSTFLAGS="-D warnings"`.
- Because GitHub Actions calls the same script, the warning gate now applies both locally and on hosted CI without duplicating workflow logic.
- Updated the README, mdBook getting-started page, Rust codebase analysis, memory, development notes, and live status to document the warning-deny CI contract.

### Validation
- `bash scripts/run_ci.sh` -> passed with 282 Rust tests under `RUSTFLAGS="-D warnings"` plus mdBook build

## 2026-04-11 (Rust dead-code warning baseline is clean)

### Fixed: stale warning-only code paths
- Removed the orphaned `.fsm` adapter renderability helper path that still operated on legacy `DecisionTreeFragmentRecord` actions after the active lowering path moved to `ControlBlockRecord` branches.
- Removed the unused `render_action` helper; active rendering now goes through `render_control_action`.
- Removed empty semantic-stage register/timing builder stubs that no longer had call sites because `SemanticIR` carries register and timing records directly from `EvidenceIR` plus VLM timing extraction.

### Validation
- `cargo test --manifest-path Cargo.toml --lib` -> passed with 282 tests and no dead-code warning output
- `bash scripts/run_ci.sh` -> passed with 282 tests, mdBook build, and no dead-code warning output

## 2026-04-11 (README bootstrap refresh updates Rust analysis)

### Updated: bootstrap and codebase analysis
- Executed the README handoff path by reading `SESSION_BOOTSTRAP.md` and the high-signal referenced live docs.
- Refreshed `RUST_CODEBASE_ANALYSIS.md` so it reflects the current CLI surface, including `project-validation`, `rescan-plan`, `kg-bench`, `learn-priors`, and `nlp-enrich`.
- Updated the analysis to capture schema-v2 rescan execution, the no-canonical-mutation promotion-review boundary, `CorpusMemory` schema v5 prior families, the 45-fixture KG benchmark surface, and the latest observed 282-test Rust baseline.
- Updated continuity memory so the latest committed baseline is `4cfb825 test(kg): lock active-low vlm timing polarity`.

### Validation
- `bash scripts/run_ci.sh` -> passed

## 2026-04-11 (Active-low VLM timing polarity is fixture-locked)

### Added: timing-annotation KG fixture
- Added tracked KG-quality fixture `vlm_timing_active_low_assertion_equivalence_gold`.
- The fixture proves that a VLM timing diagram that reports the same active-low reset as `asserted` and `LOW` produces typed timing-derived signal constraints and temporal rules without creating a false temporal conflict.
- The fixture also asserts that resolved reset polarity remains visible and no signal-polarity conflict is reported.

### Validation
- `cargo run -p specforge -- kg-bench vlm_timing_active_low_assertion_equivalence_gold` -> passed
- `cargo test --manifest-path Cargo.toml kg_bench_runs_tracked_fixtures -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-11 (Rescan approval artifact boundary is explicit)

### Documented: review metadata is not approval evidence
- Documented that `promotion_review` is a review-requirement descriptor, not an approval record and not permission to mutate canonical IR.
- Decided that future approval artifacts stay local/generated by default while no canonical IR mutation workflow exists.
- Tracked approval evidence should only be designed together with an explicit canonical mutation path, including current-document evidence links, validation-delta review, mutation scope approval, prior-memory non-authority, and replayable provenance.

### Validation
- Documentation-only change; no runtime behavior changed.

## 2026-04-11 (Negative-knowledge benchmark coverage now spans connectivity and interface conflicts)

### Added: prior-guided caution fixtures
- Added tracked KG-quality fixture `negative_knowledge_prior_guided_connectivity_conflict_caution_gold` for `signal_connectivity_conflict:multiple_producers`.
- Added tracked KG-quality fixture `negative_knowledge_prior_guided_interface_conflict_caution_gold` for `interface_signal_conflict:direction_mismatch` and `interface_signal_conflict:width_mismatch`.
- Both fixtures assert that the current local conflict remains present in `SemanticIR` and `IntentIR`; prior memory only adds `negative_knowledge_prior_matches`, rescan recommendations, corroboration requirements, and stage-specific rescan-guidance findings.

### Documentation
- Updated the README, roadmap, memory, development notes, and mdBook quality chapters so the public and continuity docs say the negative-knowledge benchmark suite now covers carried interface and connectivity conflict families too.

### Validation
- `cargo run -p specforge -- kg-bench negative_knowledge_prior_guided_connectivity_conflict_caution_gold negative_knowledge_prior_guided_interface_conflict_caution_gold` -> passed
- `cargo test --manifest-path Cargo.toml kg_bench_runs_tracked_fixtures -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-10 (Explicit clock/reset topology hints are typed)

### Added: bounded infrastructure topology records
- Updated [semantic.rs](crates/specforge/src/ir/semantic.rs) so `InfrastructureSignalRecord` now carries `infrastructure_topology` records for explicit current-document clock/reset topology hints.
- The first topology kinds are `clock_gated_branch`, `reset_synchronizer_stages`, and `reset_tree_targets`.
- The extractor records component names, reset synchronizer stage counts, target actor names, supporting statement IDs, and automation confidence when the source text is explicit enough.
- Vague wording such as a reset that "may use a synchronizer" remains ignored; the new surface preserves evidence but does not claim full physical clock-tree/reset-tree proof.

### Added: validation visibility
- Updated [validate.rs](crates/specforge/src/commands/validate.rs) so `SemanticIR` and `IntentIR` validation report `infrastructure_topology_records`, `infrastructure_clock_gated_branches`, `infrastructure_reset_synchronizer_stages`, and `infrastructure_reset_tree_targets`.
- Validation console output now shows per-signal topology counts and record summaries under `Infrastructure Signals`.
- Updated the mdBook clock/reset, SemanticIR, and IntentIR chapters plus live continuity docs to document the new boundary.

### Validation
- `cargo test --manifest-path Cargo.toml explicit_clock_reset_topology_recovers_only_current_document_evidence -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml validate_intent_ir_counts_explicit_infrastructure_topology -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-10 (Rescan promotion review path is explicit)

### Added: promotion-review policy record
- Updated [project_validation.rs](crates/specforge/src/commands/project_validation.rs) so `ProjectRescanExecutionSummary` now carries a structured `promotion_review` record beside `promotion_status` and `promotion_blockers`.
- The review record captures `review_status`, `approval_policy`, `required_decisions`, `approval_record_required`, and `canonical_mutation_allowed`.
- Changed rescan outcomes now require `human_review_required`, an approval record, current-document evidence support, validation-delta review, explicit canonical mutation scope approval, and a check that prior memory was not used as truth authority.
- No-change outcomes are marked `not_reviewable_no_change`, and every path keeps `canonical_mutation_allowed: false`.

### Preserved: no canonical mutation path yet
- Updated [rescan_plan.rs](crates/specforge/src/commands/rescan_plan.rs) so executed recommendations persist the new review record.
- Legacy execution summaries without `promotion_review` still deserialize and are normalized by `project-validation`.
- The validation snapshot and live-status projection now show the review status in addition to the not-promoted gate.

### Validation
- `cargo test --manifest-path Cargo.toml project_validation_normalizes_legacy_rescan_execution_summary_gate -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml project_validation_collects_negative_knowledge_rescan_guidance -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml rescan_plan_execute_marks_validated_no_change -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml rescan_plan_arbitration_verdict_tracks_validation_direction -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-10 (Visual-motif rescans now replay local enrichment)

### Added: explicit visual corroboration replay command
- Updated [project_validation.rs](crates/specforge/src/commands/project_validation.rs) so `evidence_visual_motif_corroboration_guidance` recommendations now emit `enrich_source_ir` before the downstream `rebuild_evidence_ir` and `validate_current_artifact` hints.
- Added `project-validation --rescan-vlm-provider auto-local|ollama|lmstudio|skip` and optional `--rescan-vlm-model <model>` so generated visual-motif enrichment hints can prefer ready local Ollama, fall back to ready local LM Studio, or be forced by policy instead of always emitting Ollama.
- Updated [doctor.rs](crates/specforge/src/commands/doctor.rs) with a reusable local default-model presence helper and bounded curl timeouts for readiness probes.
- Updated [rescan_plan.rs](crates/specforge/src/commands/rescan_plan.rs) so `rescan-plan --execute` can parse and run whitelisted local `enrich` hints in-process.
- The enrich replay parser accepts local Ollama, local LM Studio, or `skip`, supports an optional model and `--classify-only`, and intentionally rejects OpenAI replay hints so generated rescans stay local-first.
- Updated [cli.rs](crates/specforge/src/cli.rs) so VLM provider args can participate in typed rescan invocation equality tests.

### Documentation
- Updated the README, live notes, roadmap, memory, and mdBook validation/quality/multimodal/corpus-memory chapters so the public contract is clear: visual-motif prior memory may route a local enrichment rescan, but it still does not promote truth.

### Validation
- `cargo test --manifest-path Cargo.toml project_validation_collects_visual_motif_rescan_guidance -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml project_validation_rescan_vlm_policy_can_emit_lmstudio_hint -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml project_validation_auto_rescan_vlm_policy_prefers_ready_lmstudio_when_ollama_absent -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml project_validation_defaults_to_auto_local_rescan_vlm_provider -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml rescan_plan_parses_whitelisted_local_enrich_hint -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml rescan_plan_rejects_openai_enrich_hints -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml project_validation -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml doctor -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml rescan_plan -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Visual-motif priors now emit corroboration targets)

### Added: validation-visible visual corroboration targets
- Updated [validate.rs](crates/specforge/src/commands/validate.rs) so `EvidenceIR` validation now reports role-level visual evidence metrics: `visual_evidence_normative`, `visual_evidence_explanatory`, `visual_evidence_illustrative`, `visual_evidence_ambiguous`, and `visual_evidence_unknown`.
- Prior-classified normative visual evidence now also reports `visual_motif_corroboration_targets` and emits `evidence_visual_motif_corroboration_guidance` when it still lacks VLM timing/state extraction observations.
- The guidance is explicitly review/rescan routing only: it asks for targeted VLM/multimodal corroboration and still does not rewrite `SourceIR`, synthesize semantic facts, or promote canonical truth from prior memory.

### Added: gold/negative benchmark pair
- Strengthened [visual_motif_prior_guided_diagram_classification_gold](crates/specforge/test_data/kg_quality/visual_motif_prior_guided_diagram_classification_gold/fixture.json) so it now proves the prior-backed classification produces one normative visual evidence item, one corroboration target, and zero semantic hints.
- Added [visual_motif_prior_guided_diagram_classification_without_prior_negative](crates/specforge/test_data/kg_quality/visual_motif_prior_guided_diagram_classification_without_prior_negative/fixture.json), proving the same local `XREQ cycle trace` visual stays ambiguous and unclassified without staged visual-motif memory.
- Updated [project_validation.rs](crates/specforge/src/commands/project_validation.rs) so generic `rescan_guidance` findings with related ids can enter the schema-v2 rescan target list, including the new visual-motif corroboration finding.

### Validation
- `cargo test --manifest-path Cargo.toml visual_motif -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml project_validation_collects_visual_motif_rescan_guidance -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml validate_evidence_ir -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml kg_bench_runs_tracked_fixtures -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality visual_motif_prior_guided_diagram_classification_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality visual_motif_prior_guided_diagram_classification_without_prior_negative` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Rescan execution summaries now gate promotion explicitly)

### Added: machine-readable not-promoted gate
- Updated [project_validation.rs](crates/specforge/src/commands/project_validation.rs) so `ProjectRescanExecutionSummary` carries `promotion_status` and `promotion_blockers` in addition to automation status, arbitration verdict, and validation deltas.
- Updated [rescan_plan.rs](crates/specforge/src/commands/rescan_plan.rs) so executed recommendations now write `not_promoted_no_change` for unchanged validation snapshots and `not_promoted_review_required` for possible-improvement, regression, or neutral artifact-drift verdicts.
- The blockers make the policy explicit in generated schema-v2 plans: rescans do not mutate canonical IR, validation deltas are not truth promotion, and changed outcomes still require current-document evidence review.

### Preserved: backward-compatible local plans
- Older local execution summaries that do not yet contain promotion fields, or that contain stale nonmatching promotion text, deserialize safely and are normalized when `project-validation` preserves matching execution state.
- `VALIDATION_SNAPSHOT.md` and the managed live-status validation block now project the promotion gate beside verdict and delta details, so future review does not need to inspect raw JSON to see that a favorable delta remains unpromoted.

### Validation
- `cargo test --manifest-path Cargo.toml rescan_plan -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml project_validation -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Project validation projects rescan execution summaries)

### Added: review-facing rescan execution projection
- Updated [project_validation.rs](crates/specforge/src/commands/project_validation.rs) so `project-validation` preserves matching executed rescan summaries from the existing local schema-v2 plan before refreshing `generated/validation/rescan_plan.json`.
- `VALIDATION_SNAPSHOT.md` now projects rescan execution-summary counts and per-recommendation verdict/delta details when they exist.
- The managed live-status validation block now includes the same review-required counts and inline verdict/delta summary for queued recommendations.

### Preserved: refreshes do not erase review state
- Matching is keyed on document, stage, artifact path, finding id, extractor lane, and related ids.
- The refresh carries forward `automation_status` plus `execution_summary` only for matching recommendations, so stale unrelated execution state does not leak into fresh rescan targets.

### Validation
- `cargo test --manifest-path Cargo.toml project_validation -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Rescan execution persists arbitration summaries)

### Added: validation-backed execution summaries
- Updated [project_validation.rs](crates/specforge/src/commands/project_validation.rs) so schema-v2 rescan recommendations can carry an optional `execution_summary` after execution.
- Updated [rescan_plan.rs](crates/specforge/src/commands/rescan_plan.rs) so executed recommendations persist before/after validation snapshots, score deltas, finding-count deltas, added/removed finding ids, and an arbitration verdict.
- Verdicts are deliberately conservative: `validated_no_change`, `possible_improvement_review_required`, `regression_review_required`, or `neutral_change_review_required`.

### Preserved: review before promotion
- Updated [converge.rs](crates/specforge/src/commands/converge.rs) so convergence summaries count review-required verdicts and split them across possible-improvement, regression, and neutral artifact-change buckets.
- A possible improvement is still not a canonical truth promotion; it only means validation deltas moved in a favorable direction and need current-document evidence review.

### Validation
- `cargo test --manifest-path Cargo.toml rescan_plan -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml converge -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml project_validation_collects_negative_knowledge_rescan_guidance -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml project_validation_writes_snapshot_doc_and_updates_live_status -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Converge can consume rescan plans after stability)

### Added: opt-in rescan hook for the fixed-point loop
- Updated [converge.rs](crates/specforge/src/commands/converge.rs) so `specforge converge <source> --rescan-plan <plan>` consumes a schema-v2 validation rescan plan after the persisted pipeline snapshot stabilizes.
- Added `--execute-rescan-plan` and `--rescan-plan-limit` to keep execution explicit and bounded.
- Added a `--document-key` filter to standalone `rescan-plan`; the `converge` hook automatically filters multi-document plans to the current source document key.
- `--execute-rescan-plan` without `--rescan-plan <plan>` is rejected.

### Preserved: convergence is not auto-promotion
- Updated [rescan_plan.rs](crates/specforge/src/commands/rescan_plan.rs) to expose a structured run report reused by `converge`.
- The convergence summary now reports selected recommendations, changed/no-change validation outcomes, post-rescan snapshot drift, and an arbitration status.
- The convergence result remains the stable pre-rescan snapshot; `changed_requires_validation_review` is a review signal, not an improvement claim.

### Validation
- `cargo test --manifest-path Cargo.toml converge_defaults_to_ollama_for_vlm_and_nlp -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml converge -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml rescan_plan -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Rescan execution records validation deltas)

### Added: before/after validation accounting
- Updated [rescan_plan.rs](crates/specforge/src/commands/rescan_plan.rs) so `specforge rescan-plan --execute` validates the recommendation artifact before and after whitelisted command execution.
- The local generated plan now records neutral execution outcome statuses:
  - `executed_validated_no_change`
  - `executed_validated_changed`
- Added an execution-path regression that builds a real temporary `SourceIR`, executes a validate-only rescan recommendation, and verifies the plan is marked `executed_validated_no_change`.

### Preserved: changed does not mean improved
- The executor still does not classify a rebuild as improved.
- It only records whether the validation fingerprint, score, grade, or finding count changed.
- The deeper remaining step is promotion-grade arbitration over changed outcomes before treating rebuilt artifacts as better canonical truth.

### Validation
- `cargo test --manifest-path Cargo.toml rescan_plan -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Rescan plan now has a bounded executor)

### Added: dry-run-first rescan-plan command
- Added [rescan_plan.rs](crates/specforge/src/commands/rescan_plan.rs) with the new `specforge rescan-plan` command.
- The command reads schema-v2 `generated/validation/rescan_plan.json`, reports pending `planned_not_executed` targets by default, and supports `--limit` plus `--plan`.
- Added `--execute` to dispatch only whitelisted in-process `ingest`, `evidence`, `semantic`, `intent`, and `validate` hints from the structured command args.
- Successful `--execute` runs update local recommendation status in the generated plan.

### Preserved: execution is not truth promotion
- The executor refuses non-`cargo` executables, non-repository working directories, malformed cargo prefixes, and unsupported command intents.
- It does not shell out through command display strings.
- It does not let prior memory decide canonical facts; execution only rebuilds and validates stages, while changed outcomes still need promotion-grade evidence policy before they can count as improved.

### Documentation
- Updated the public mdBook command and validation pages with the `rescan-plan` behavior.
- Updated [README.md](README.md), [ROADMAP.md](ROADMAP.md), [DEVELOPMENT_NOTES.md](DEVELOPMENT_NOTES.md), and [MEMORY.md](MEMORY.md).

### Validation
- `cargo test --manifest-path Cargo.toml rescan_plan -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- rescan-plan` -> passed (`rescan_queue: empty`)
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Rescan plan now carries replayable command hints)

### Added: schema-v2 replay metadata for targeted rescans
- Updated [project_validation.rs](crates/specforge/src/commands/project_validation.rs) so `generated/validation/rescan_plan.json` now uses schema version 2.
- Each recommendation now carries typed `replay_inputs` such as `source_document`, `source_ir`, `evidence_ir`, or `semantic_ir`.
- Each recommendation now carries structured `recommended_commands` with executable, args, working directory, display string, and command intent.
- Recommendations now carry `automation_status: planned_not_executed` so future loops can distinguish planned targets from executed rescans.

### Preserved: replay hints are not auto-rescans
- `project-validation` still does not execute the command hints.
- It does not mutate IR beyond the existing validation backannotation behavior.
- It does not suppress findings, change scoring, decide arbitration, or promote facts from prior memory.
- The new metadata is what lets explicit rescan consumers execute safe args instead of scraping prose or shell strings.

### Documentation
- Updated the public mdBook validation and command pages to describe the replay-oriented plan contract.
- Updated [README.md](README.md), [DEVELOPMENT_NOTES.md](DEVELOPMENT_NOTES.md), and [MEMORY.md](MEMORY.md) with the schema-v2 behavior and remaining executor follow-up.

### Validation
- `cargo test --manifest-path Cargo.toml project_validation -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` -> passed (`rescan_recommendations: 0`, schema-v2 local plan refreshed)
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Project validation now consumes rescan guidance)

### Added: generated rescan/extractor-selection plan
- Updated [project_validation.rs](crates/specforge/src/commands/project_validation.rs) so `specforge project-validation` now consumes `*_negative_knowledge_rescan_guidance` findings from validation reports.
- The command now writes a local generated `generated/validation/rescan_plan.json` plan with document key, stage, artifact path, finding id, related current-surface ids, extractor lane, corroboration policy, and recommended action.
- `VALIDATION_SNAPSHOT.md` now renders a `Targeted Rescan Recommendations` section from the same plan.
- The managed `LIVE_ACHIEVEMENT_STATUS.md` validation projection now includes a concise `Targeted rescan queue`.

### Preserved: the plan routes work, not truth
- The consumer does not mutate IR artifacts.
- It does not suppress findings.
- It does not change scoring, arbitration, or canonical promotion.
- It keeps prior memory advisory by routing extraction effort toward dangerous current-document conflict/residual shapes.

### Validation
- `cargo test --manifest-path Cargo.toml project_validation -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` -> passed (`rescan_recommendations: 0`)
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Negative-knowledge now routes rescan/corroboration guidance)

### Added: machine-readable guidance from known failure shapes
- Updated [validate.rs](crates/specforge/src/commands/validate.rs) so exact negative-knowledge prior matches now emit `negative_knowledge_rescan_recommendations` and `negative_knowledge_corroboration_requirements`.
- Validation now also emits stage-specific `*_negative_knowledge_rescan_guidance` findings for `EvidenceIR`, `SemanticIR`, and `IntentIR`, with the matched current conflict/residual ids preserved as `related_ids`.
- This gives future rescan/extractor-selection loops a deterministic routing hook instead of only a human-readable caution.

### Preserved: guidance is not correction
- The new guidance still requires a current-document conflict or residual before any prior can match.
- It does not mutate artifacts.
- It does not suppress conflict/residual findings.
- It does not change scoring, arbitration, or canonical promotion.
- It does not create canonical facts from prior memory.

### Added: unit, fixture, and docs coverage
- Extended negative-knowledge validation regressions so rescan/corroboration metrics and findings are locked for evidence, semantic, intent, temporal-conflict, and residual-decision matches.
- Updated KG-quality fixtures so prior-guided cases require the new guidance while the no-prior semantic-conflict fixture excludes it and expects zero guidance metrics.
- Updated the public mdBook corpus-memory, validation, and pipeline chapters so this routing hook is documented as end-user-visible behavior.

### Validation
- `cargo test --manifest-path Cargo.toml negative_knowledge -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml kg_bench_runs_tracked_fixtures -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Explicit infrastructure distribution recovery stays bounded)

### Added: locally grounded clock/reset distribution evidence
- Updated [semantic.rs](crates/specforge/src/ir/semantic.rs) so explicit current-document phrases such as `ACLK is distributed to the Requester and Completer` can recover `distributed_to_*` targets directly into `InfrastructureSignalRecord`.
- Added active fanout parsing for bounded phrases such as `reset synchronizer feeds ARESETN to the Requester`, allowing the same local sentence to recover a reset infrastructure source and a distribution target.
- Updated [validate.rs](crates/specforge/src/commands/validate.rs) with a regression proving recovered distribution updates the existing infrastructure distribution metrics.

### Preserved: distribution is not ordinary protocol connectivity
- Distribution-only evidence does not create ordinary `ActorPortRecord`s.
- It does not fabricate source actors.
- It still rejects generic labels through the existing infrastructure component / meaningful actor filters.

### Validation
- `cargo test --manifest-path Cargo.toml explicit_clock_distribution_recovers_infrastructure_targets -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml explicit_reset_synchronizer_fanout_recovers_source_and_target -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml validate_intent_ir_counts_recovered_infrastructure_distribution -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml validate_intent_ir_counts_recovered_infrastructure_source -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-10 (Explicit infrastructure source recovery stays bounded)

### Added: locally grounded infrastructure source evidence
- Updated [semantic.rs](crates/specforge/src/ir/semantic.rs) so explicit current-document phrases such as `clock generator drives ACLK` can recover a source actor directly into `InfrastructureSignalRecord`.
- The parser accepts bounded infrastructure component terms such as `clock generator`, `reset controller`, `PLL`, `DLL`, `oscillator`, and synchronizer/gating-style component names without relaxing the ordinary protocol-actor filters.
- Existing graph-derived sources still contribute when the local KG already recovered a real source such as `PLL generates ACLK`.

### Preserved: infrastructure sources are not automatic consumers
- Tightened the system-contract fanout pass so implicit clock/reset read ports are added only to actors with non-infrastructure protocol relations.
- A recovered source actor for `ACLK` is no longer also marked as an `ACLK` consumer solely because a system contract exists.
- Generic labels such as `Clock`, `Reset`, `External`, `input`, and `output` remain blocked from becoming ordinary producer actors.

### Validation
- `cargo test --manifest-path Cargo.toml explicit_clock_generator_recovers_infrastructure_source_status -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml infrastructure_source_actor_is_not_marked_as_own_consumer -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml clock_and_reset_gain_input_actor_ports_for_relation_actors -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml validate_intent_ir_counts_recovered_infrastructure_source -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-10 (Infrastructure source/distribution status is first-class)

### Added: canonical infrastructure signal status
- Added `InfrastructureSignalRecord` to [semantic.rs](crates/specforge/src/ir/semantic.rs) so `SemanticIR` now records clock/reset infrastructure kind, unresolved vs recovered source status, recovered distribution status, supporting statements, and automation confidence.
- `SemanticIR` derives this surface from the local `SystemContractRecord` plus recovered `SignalConnectivityRecord`s, so `ACLK` / `ARESETN` can be marked as infrastructure while keeping unresolved source ownership explicit.
- Updated [intent.rs](crates/specforge/src/ir/intent.rs) so `IntentIR` carries the same `infrastructure_signals` surface forward as part of the canonical product artifact.

### Preserved: no fake clock/reset producer actors
- Unresolved infrastructure sourcing is now represented as `unresolved_source`.
- The model does not invent producer actors named `Clock`, `External`, or `input`.
- Recovered distribution status reports whether the signal reaches zero, one, or multiple recovered actors; it is not a physical clock-tree or reset-tree proof.

### Added: validation and docs
- Updated [validate.rs](crates/specforge/src/commands/validate.rs) with `infrastructure_signals`, unresolved-source, recovered-source, and recovered-distribution metrics for `SemanticIR` and `IntentIR`.
- Updated the public mdBook clock/reset and pipeline chapters so the new surface is documented as project-facing behavior, not just a continuity note.

### Validation
- `cargo test --manifest-path Cargo.toml system_contract_emits_infrastructure_records_without_actor_ports -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml clock_and_reset_gain_input_actor_ports_for_relation_actors -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml carries_infrastructure_signal_connectivity_class_into_intent_ir -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml validate_intent_ir_treats_clock_and_reset_as_infrastructure_connectivity -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-10 (Negative-knowledge cautions now reach carried semantic/intent surfaces)

### Added: deep-layer validation-only caution matching
- Updated [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) with shared normalized pattern builders for temporal value conflicts, interface-signal conflicts, signal-connectivity conflicts, and residual decision packets.
- Updated [learn_priors.rs](crates/specforge/src/commands/learn_priors.rs) so negative-knowledge harvesting and validation consumption now use the same shared pattern builders for every harvested negative-knowledge kind.
- Updated [validate.rs](crates/specforge/src/commands/validate.rs) so `SemanticIR` and `IntentIR` validation can recover the linked `EvidenceIR.prior_memory_path` and surface exact-match `negative_knowledge_prior_matches`.
- `SemanticIR` validation now emits `semantic_negative_knowledge_prior_matches` when a current carried conflict or residual packet class matches prior negative knowledge.
- `IntentIR` validation now emits `intent_negative_knowledge_prior_matches` for the same carried caution surface.

### Preserved: caution is not correction
- The new deep-layer consumer does not mutate `SemanticIR` or `IntentIR`.
- It does not suppress temporal, interface, connectivity, semantic-role, or residual findings.
- It does not change arbitration, scoring, or canonical promotion.
- It only makes repeated conflict/residual shapes visible as prior-memory caution.

### Added: unit and KG proof for deeper caution surfaces
- Added focused validation regressions for temporal-conflict and residual-decision negative-knowledge matches across `SemanticIR` and `IntentIR`.
- Strengthened the existing semantic-conflict caution fixtures so prior-guided and no-prior cases now also lock `SemanticIR` / `IntentIR` negative-knowledge validation behavior.
- Added [negative_knowledge_prior_guided_temporal_conflict_caution_gold](crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_temporal_conflict_caution_gold/fixture.json), proving a repeated high/low temporal contradiction is flagged as caution while the conflict remains present.
- Added [negative_knowledge_prior_guided_residual_caution_gold](crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_residual_caution_gold/fixture.json), proving a repeated residual packet class is flagged as caution without removing the residual.

### Validation
- `cargo test --manifest-path Cargo.toml negative_knowledge -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml kg_bench_runs_tracked_fixtures -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (EvidenceIR validation now consumes negative-knowledge priors)

### Added: bounded negative-knowledge caution surfacing
- Updated [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) with exact-pattern lookup support for `negative_knowledge_priors`.
- Moved the signal-semantic conflict pattern builder into the prior-memory module so harvesting and validation consumption use the same signature shape.
- Updated [learn_priors.rs](crates/specforge/src/commands/learn_priors.rs) to reuse that shared pattern builder for `SignalSemanticConflict` negative-knowledge harvesting.
- Updated [validate.rs](crates/specforge/src/commands/validate.rs) so `EvidenceIR` validation can load the persisted `prior_memory_path`, match current signal-semantic conflict patterns against prior negative knowledge, and report `negative_knowledge_prior_matches`.
- When a match exists, validation emits `evidence_negative_knowledge_prior_matches` as an info-level caution finding.

### Preserved: negative knowledge cannot suppress evidence
- The consumer is validation-only in this slice.
- It requires a current local `EvidenceIR.signal_semantic_conflicts` record before any prior can match.
- It does not mutate `EvidenceIR`.
- It does not change semantic arbitration.
- It does not delete conflicts, weaken findings, or synthesize canonical `SemanticIR` / `IntentIR` facts.

### Added: KG-quality proof for caution-only behavior
- Strengthened [visual_sources_semantic_conflict_negative](crates/specforge/test_data/kg_quality/visual_sources_semantic_conflict_negative/fixture.json) so the same local conflict reports `negative_knowledge_prior_matches = 0` when no prior memory is staged.
- Added [negative_knowledge_prior_guided_semantic_conflict_caution_gold](crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_semantic_conflict_caution_gold/fixture.json), which proves a staged negative-knowledge prior surfaces a validation caution while the current semantic conflict remains contested downstream.

### Validation
- `cargo test --manifest-path Cargo.toml negative_knowledge -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml validate_evidence_ir_surfaces_negative_knowledge_prior_matches -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml kg_bench_runs_tracked_fixtures -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (EvidenceIR now consumes visual-motif priors)

### Added: bounded visual-motif prior classification in `EvidenceIR`
- Updated [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) with a visual-caption lookup that resolves a unique learned `DiagramKind` only after normalizing a current caption against locally grounded signal and actor vocabulary.
- Updated [evidence.rs](crates/specforge/src/ir/evidence.rs) so `EvidenceIR` can use that lookup for a current `SourceIR.visual_assets` entry when:
  - the current visual asset has `diagram_kind = unknown`
  - the current visual asset has local caption text
  - the normalized caption matches exactly one learned visual-motif prior in the applicable protocol scope
- The result is an explicit `VisualObservationKind::Classification` observation created by `specforge_prior_memory` with medium confidence.
- The prior-guided diagram kind is allowed to influence the visual evidence role, so a recovered timing diagram can be treated as normative visual evidence.

### Preserved: local-grounding safety boundary
- This path does not mutate `SourceIR`.
- This path does not synthesize semantic-role, temporal, or canonical `IntentIR` facts.
- Explicit local `SourceIR.diagram_kind` values still win outright; prior memory only helps when the current source asset is still unknown.
- Ambiguous visual-motif memory stays silent rather than picking a diagram kind.

### Added: validation metric and KG-quality fixture
- Updated [validate.rs](crates/specforge/src/commands/validate.rs) so EvidenceIR validation now reports `visual_classification_observations`.
- Added [visual_motif_prior_guided_diagram_classification_gold](crates/specforge/test_data/kg_quality/visual_motif_prior_guided_diagram_classification_gold/fixture.json), which proves a locally unknown `XREQ cycle trace` visual asset gains a prior-backed timing-diagram classification only through staged prior memory.

### Validation
- `cargo test --manifest-path Cargo.toml visual_motif -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml kg_bench_runs_tracked_fixtures -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (CorpusMemory now has visual-motif and negative-knowledge prior families)

### Added: typed learning memory families for visual motifs and negative knowledge
- Updated [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) so `CorpusMemory` schema version `5` can carry `visual_motif_priors` and `negative_knowledge_priors`.
- Visual-motif priors remember reusable source-side visual patterns such as diagram kind, asset kind, normalized caption phrase, protocol family, support count, source documents, and strongest confidence.
- Negative-knowledge priors remember cautionary extraction archetypes such as semantic conflicts, temporal value conflicts, interface-signal conflicts, connectivity conflicts, and unresolved residual-decision classes.
- Added query helpers for retrieving visual-motif and negative-knowledge priors by protocol family and prior kind.
- The new families are typed, inspectable, and advisory-only; this slice does not let visual-motif or negative-knowledge priors directly author canonical `EvidenceIR`, `SemanticIR`, or `IntentIR` truth.

### Added: `learn-priors` harvesting for the new families
- Updated [learn_priors.rs](crates/specforge/src/commands/learn_priors.rs) so validated `IntentIR` artifacts can harvest visual motifs from their linked `SourceIR.visual_assets`.
- `learn-priors` now harvests negative-knowledge signatures from carried conflicts and residual decisions without declaring any individual conflicting phrase false.
- `learn-priors` now reports `visual_motif_priors` and `negative_knowledge_priors` counts in its CLI output.
- Updated the fixture-local prior-memory patch path in [kg_bench.rs](crates/specforge/src/commands/kg_bench.rs) so future KG-quality fixtures can seed those two prior families explicitly.

### Validation
- `cargo test --manifest-path Cargo.toml learn_priors_harvests_visual_motif_and_negative_knowledge_priors -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml learn_priors -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (VLM timing tuples now lift into temporal signal values)

### Added: typed signal-value lift from timing-diagram VLM observations
- Updated [semantic.rs](crates/specforge/src/ir/semantic.rs) so `TimingDiagramExtraction` observations now read `signals[].values[]` tuples, not only free-text `annotations`.
- Signal/value tuples such as `XREQ` at `T1` with state `HIGH` now become VLM-backed `SignalConstraintRecord` entries and feed the existing temporal-rule builder as `SignalValue` predicates.
- VLM timing signal names are gated against the document-grounded signal universe when possible, using interface signal records plus locally extracted statement signal tokens.
- Diagram cycle labels such as `T0` and `T1` are preserved as explicit cycle windows, so distinct timing-diagram states do not collapse into false same-cycle conflicts.
- Generic visual words such as `transfer` are still rejected as signal names, so timing-diagram lift does not turn diagram prose into fake hardware signals.
- VLM states such as `HIGH`, `LOW`, `ASSERTED`, `DEASSERTED`, `0`, and `1` are normalized into typed signal-constraint kinds, while unknown/don't-care values remain unpromoted.

### Added: regression coverage for timing tuple lift
- Strengthened `vlm_timing_diagram_observation_produces_timing_constraint_records` so it proves:
  - VLM timing annotations still become timing constraint records
  - VLM `signals[].values[]` tuples now become grounded signal constraints
  - those signal constraints feed temporal `SignalValue` predicates
  - generic timing-diagram words do not become VLM-authored signal constraints

### Validation
- `cargo test --manifest-path Cargo.toml vlm_timing -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml kg_bench_runs_tracked_fixtures -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (VLM state-machine guards no longer become raw fake signals)

### Fixed: bounded guard parsing for visual state-machine observations
- Updated [semantic.rs](crates/specforge/src/ir/semantic.rs) so `StateMachineExtraction` VLM transition guards are parsed conservatively before entering `SemanticIR`.
- `SemanticIR` now builds a document-grounded signal universe for VLM guard parsing from interface signal records plus extracted local statement signal tokens.
- VLM guard strings now prefer simple typed comparisons such as `PREADY = 1`, `PREADY == 1`, and `PREADY != 0` instead of turning the entire guard text into a `SignalIsHigh` record.
- Generic VLM words such as `transfer`, `transaction`, `request`, `response`, `beat`, `cycle`, and `phase` no longer become fake signal names unless they are explicitly grounded as document signal names.
- VLM guard values such as `0`, `1`, `HIGH`, `LOW`, `true`, `false`, `ASSERTED`, and `DEASSERTED` stay as literal guard values instead of being misread as signal references.

### Added: regression coverage for visual guard normalization
- Strengthened `vlm_state_machine_observation_accepts_fenced_json_with_trailing_prose` so it proves:
  - generic guard prose like `Transfer` is dropped when it is not document-grounded as a signal
  - compound guard prose like `PREADY = 1 and transfer` preserves the declared signal comparison `PREADY == 1`

### Validation
- `cargo test --manifest-path Cargo.toml vlm_state_machine -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-10 (book now explains multimodal evidence and visual grounding)

### Added: dedicated public chapter for visual evidence
- Added [multimodal-evidence.md](docs/book/src/pipeline/multimodal-evidence.md) under the mdBook Pipeline Model section.
- Updated [SUMMARY.md](docs/book/src/SUMMARY.md), [pipeline/overview.md](docs/book/src/pipeline/overview.md), [sourceir.md](docs/book/src/pipeline/sourceir.md), [evidenceir.md](docs/book/src/pipeline/evidenceir.md), [semanticir.md](docs/book/src/pipeline/semanticir.md), [architecture-rationale.md](docs/book/src/architecture-rationale.md), and [validation.md](docs/book/src/quality/validation.md) so the visual path is discoverable from the public docs.

### Clarified: VLM output is bounded evidence, not canonical truth
- The new chapter explains:
  - how `SourceIR.visual_assets` preserves figures, captions, placeholders, notes, and diagram classifications
  - how `EvidenceIR.visual_evidence` carries asset ids, roles, page/source grounding, captions, figure references, observations, and confidence
  - how `specforge enrich` can add bounded VLM notes such as `vlm_timing_diagram_extraction:` and `vlm_state_machine_extraction:`
  - how those notes become `TimingDiagramExtraction` and `StateMachineExtraction` observations
  - how `SemanticIR` can parse timing-diagram observations into timing constraints and state-machine observations into state / transition records
  - why captions can contribute semantic evidence only through the same observation, candidate, arbitration, and consensus machinery as prose and tables
  - why cross-modality agreement is stronger than repeated same-modality evidence, but still not automatic truth
  - why passive figure references should not create residuals by themselves
  - which visual and multimodal validation metrics users should inspect when debugging visual behavior

## 2026-04-10 (book now explains temporal semantics and timing rules)

### Added: dedicated public chapter for typed timing
- Added [temporal-semantics.md](docs/book/src/domain/temporal-semantics.md) under the mdBook Domain Model section.
- Updated [SUMMARY.md](docs/book/src/SUMMARY.md), [domain/overview.md](docs/book/src/domain/overview.md), [architecture-rationale.md](docs/book/src/architecture-rationale.md), [semanticir.md](docs/book/src/pipeline/semanticir.md), and [validation.md](docs/book/src/quality/validation.md) so the public docs now expose the temporal-rule model directly.

### Clarified: timing prose becomes typed obligations when grounded
- The new chapter explains:
  - `TemporalRuleRecord` structure at a public level
  - clock edges and tick phases
  - `SignalValue`, actor-grounded predicates, `SignalStable`, `SignalSampled`, and `HandshakeComplete`
  - cycle windows for same-cycle, next-cycle, and bounded timing language
  - temporal conflicts and the conflict evidence they preserve
  - polarity-aware comparison for `ASSERTED` / `DEASSERTED`
  - prior-guided timing phrase recovery, with the same local-grounding safety rule used elsewhere
  - validation metrics such as `temporal_rules_with_cycle_window`, `temporal_rules_with_actor_grounding`, and `temporal_rules_with_handshake_completion`

## 2026-04-09 (book now explains graph-first actor connectivity)

### Added: dedicated public chapter for actor connectivity
- Added [actor-connectivity.md](docs/book/src/domain/actor-connectivity.md) under the mdBook Domain Model section.
- Updated [SUMMARY.md](docs/book/src/SUMMARY.md), [domain/overview.md](docs/book/src/domain/overview.md), [introduction.md](docs/book/src/introduction.md), [architecture-rationale.md](docs/book/src/architecture-rationale.md), and [semanticir.md](docs/book/src/pipeline/semanticir.md) so the new graph-direction domain chapter is visible from the public docs path.

### Clarified: direction is a structural KG problem first
- The new chapter explains:
  - `Drives` / `Reads` as actor-signal graph relation types
  - how graph relations become actor-relative ports
  - how actor ports group into signal connectivity records
  - why graph-derived direction is stronger than flat compatibility direction hints
  - where graph facts come from today, including source/destination tables, drive/sample prose, prior-guided section headings, and local complementary actor recovery
  - why bogus labels such as `Clock`, `Reset`, `External`, `Tie-off`, `input`, and payload/event nouns must not become fake protocol actors
  - how multiple-producer ambiguity remains visible as connectivity conflict state
  - how graph facts support actor-grounded temporal predicates

## 2026-04-09 (book now explains handshake semantic-role arbitration)

### Added: dedicated public chapter for handshake and semantic roles
- Added [handshake-semantics.md](docs/book/src/domain/handshake-semantics.md) under the mdBook Domain Model section.
- Updated [SUMMARY.md](docs/book/src/SUMMARY.md), [domain/overview.md](docs/book/src/domain/overview.md), [introduction.md](docs/book/src/introduction.md), [architecture-rationale.md](docs/book/src/architecture-rationale.md), and [semanticir.md](docs/book/src/pipeline/semanticir.md) so the new domain chapter is discoverable from the public docs path.

### Clarified: semantic-role truthfulness is now documented as a domain model
- The new chapter explains the current role surface:
  - `handshake_valid_like`
  - `handshake_ready_like`
  - semantic observations before winners
  - candidates, grounding strength, arbitration, and consensus
  - alias-dependent and prior-guided meaning
  - `HandshakeComplete` temporal predicates
  - blocked name fallback when signal spelling is unsafe
- This makes the earlier semantic-arbitration doctrine public, instead of requiring readers to infer it from `SemanticIR` fields or validation findings.

## 2026-04-09 (book now has a domain-model section for clock/reset infrastructure)

### Added: dedicated public domain-model chapter
- Added [domain/overview.md](docs/book/src/domain/overview.md) as the first mdBook domain-model landing page.
- Added [domain/clock-reset.md](docs/book/src/domain/clock-reset.md) to explain why clocks and resets are infrastructure semantics, not ordinary protocol edges.

### Clarified: clock/reset truthfulness doctrine is now public
- The new chapter explains:
  - why clock and reset trees are sensitive digital-system infrastructure
  - why `ASSERTED` / `DEASSERTED` must remain polarity-relative
  - how `system_contract` records clock/reset signal, reset kind, reset polarity, assertion timing, release timing, and target kind
  - why `system_clock` / `system_reset` connectivity classes are distinct from `protocol`
  - why labels like `External` and `Tie-off` should not become ordinary protocol actors
  - what validation surfaces can show today
  - what the current model still does not attempt to cover physically, such as full clock-tree or reset-tree topology

### Changed: book navigation now exposes domain semantics explicitly
- Updated [SUMMARY.md](docs/book/src/SUMMARY.md), [introduction.md](docs/book/src/introduction.md), [architecture-rationale.md](docs/book/src/architecture-rationale.md), [semanticir.md](docs/book/src/pipeline/semanticir.md), and [README.md](README.md) so domain-model semantics are no longer buried inside the IR-stage discussion.

## 2026-04-09 (book reference section now explains generated artifacts and continuity boundaries)

### Added: dedicated mdBook reference landing page
- Added [overview.md](docs/book/src/reference/overview.md) so the book now has a stable reference entry point instead of making the generated-artifacts page double as the whole reference section.
- Updated [SUMMARY.md](docs/book/src/SUMMARY.md) so [Generated Artifacts](docs/book/src/reference/generated-artifacts.md) is now an explicit reference chapter.

### Expanded: generated artifact documentation
- Expanded [generated-artifacts.md](docs/book/src/reference/generated-artifacts.md) so it now explains:
  - why `generated/` is local execution state rather than source code
  - how the stage artifact roots map to `SourceIR`, `EvidenceIR`, `SemanticIR`, and `IntentIR`
  - what source-side sidecars, validation reports, adapter artifacts, and `CorpusMemory` are for
  - why arbitrary live generated artifacts stay untracked while curated fixtures can still be tracked
  - how to inspect artifacts by tracing problems backward through the staged pipeline

### Expanded: live-docs versus book contract
- Expanded [live-docs.md](docs/book/src/reference/live-docs.md) so the public book now spells out the distinction between:
  - the book as the world-facing documentation product
  - root markdown docs as the operational continuity plane for scores, roadmap state, handoff notes, and crash recovery
- Added practical guidance for when to update the book, the live docs, or both.

## 2026-04-09 (book IR stage chapters now explain their real boundaries)

### Expanded: the mdBook pipeline chapters are no longer only thin stage summaries
- Expanded [pipeline/overview.md](docs/book/src/pipeline/overview.md) so it now explains stage boundaries and the different truthfulness contracts each IR stage is supposed to uphold.
- Expanded [pipeline/sourceir.md](docs/book/src/pipeline/sourceir.md) so it now explains what `SourceIR` practically preserves, why early investment there mattered, and what structural failure modes still belong to Tier 1.
- Expanded [pipeline/evidenceir.md](docs/book/src/pipeline/evidenceir.md) so it now explains what `EvidenceIR` is allowed to extract, why provenance matters there, and what kinds of false promotion the stage is supposed to avoid.
- Expanded [pipeline/semanticir.md](docs/book/src/pipeline/semanticir.md) so it now explains semantic arbitration, graph-first direction, infrastructure handling, and why `SemanticIR` is the main semantic safety boundary before canonical intent.
- Expanded [pipeline/intentir.md](docs/book/src/pipeline/intentir.md) so it now explains what canonical means in this project, why adapters come later, and why honest incompleteness is still acceptable there.

### Changed: the public book now explains not just the stages, but the allowed decisions at each stage
- This moves the book closer to the intended public role: not only listing the pipeline, but explaining what each layer is for, what it should and should not decide, and why the staged separation exists.

## 2026-04-09 (book now has a first-class architecture rationale chapter)

### Added: dedicated mdBook chapter for why `specforge` is built this way
- Added [architecture-rationale.md](docs/book/src/architecture-rationale.md).
- It explains the core public-facing design logic:
  - why the tool is staged
  - why it is provenance-first
  - why it uses bounded AI instead of a black-box "read the whole PDF" approach
  - why residuals and conflicts are first-class
  - why the learning plane is symbolic and separate from canonical per-document truth

### Changed: the book entry path now exposes rationale earlier
- Updated [SUMMARY.md](docs/book/src/SUMMARY.md), [introduction.md](docs/book/src/introduction.md), [README.md](README.md), and [USER_GUIDE.md](USER_GUIDE.md) so readers encounter the architecture explanation before diving straight into usage details.

## 2026-04-09 (book now covers validation and learning as first-class topics)

### Added: dedicated mdBook chapters for validation and the learning plane
- Added [validation.md](docs/book/src/quality/validation.md) to explain how `specforge` judges artifact quality, why scores are secondary to findings, and how validation gates the learning plane.
- Added [kg-bench.md](docs/book/src/quality/kg-bench.md) to explain the fixture harness as a truthfulness regression system rather than just another CLI command.
- Added [corpus-memory.md](docs/book/src/quality/corpus-memory.md) to explain what actually grows over time, what the prior store learns, and why it is explicit symbolic memory rather than hidden model weights.

### Changed: command docs now point readers toward deeper rationale chapters
- Updated [quality-and-learning.md](docs/book/src/commands/quality-and-learning.md) so it stays the operational CLI page while linking to the deeper validation and learning chapters.
- Updated [introduction.md](docs/book/src/introduction.md), [SUMMARY.md](docs/book/src/SUMMARY.md), [README.md](README.md), and [USER_GUIDE.md](USER_GUIDE.md) so the new book coverage is visible from the entry path.

## 2026-04-09 (book and continuity docs now have an explicit split contract)

### Changed: the mdBook is now explicitly the public documentation product
- Updated [README.md](README.md), [ROADMAP.md](ROADMAP.md), and [DEVELOPMENT_NOTES.md](DEVELOPMENT_NOTES.md) so the repo now states this plainly:
  - the mdBook is what the outside world should read
  - it should openly explain what `specforge` does, how it works, and why it is designed that way
  - every meaningful user-facing aspect of the project should ultimately land in the book with its own section or chapter

### Added: a dedicated book page for documentation scope
- Added [documentation-scope.md](docs/book/src/reference/documentation-scope.md) and linked it from [SUMMARY.md](docs/book/src/SUMMARY.md).
- Updated [introduction.md](docs/book/src/introduction.md) and [live-docs.md](docs/book/src/reference/live-docs.md) so the book now explains the split directly instead of only implying it.

### Clarified: root markdown docs are a separate continuity plane
- The root docs are now described consistently as continuity / steering infrastructure for:
  - crash recovery
  - session handoff
  - roadmap and validation projection
  - engineering-state tracking
- They are no longer described as a second competing public documentation surface.

## 2026-04-09 (tie-off appendix rows no longer create fake AXI producers)

### Fixed: `Tie-off` is no longer treated as a real protocol actor
- Tightened [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) so shared actor-term hygiene now rejects `Tie-off` / `tie off` the same way it already rejects infrastructure placeholders like `External`.
- Tightened [evidence.rs](crates/specforge/src/ir/evidence.rs) so `Source = Tie-off` rows no longer author `ActorSignalRelation::Drives` edges through table relation recovery.

### Fixed: tie-off source rows now synthesize honest input declarations
- Updated [evidence.rs](crates/specforge/src/ir/evidence.rs) so `Tie-off` source labels still contribute local direction information, but as `input` declarations instead of fake `output` declarations.
- This keeps appendix control pins like `BROADCASTATOMIC`, `BROADCASTSHAREABLE`, `BROADCASTCACHEMAINT`, `BROADCASTCMOPOPA`, `BROADCASTPERSIST`, and `BROADCASTSTORAGE` in the declared signal surface without pretending there is a real driving actor named `Tie-off`.

### Added: regression coverage for tie-off input rows
- Added `tie_off_source_rows_become_input_declarations_without_fake_actor`, which proves `Tie-off` rows synthesize `Signal ... is input width 1.` declarations while keeping `Tie-off` out of the structural KG.
- Kept `external_source_rows_do_not_synthesize_infrastructure_outputs` green, so the broader infrastructure-label hygiene remains intact.

### Changed: AXI stays at `85/100 GOOD`, but the structural surface is more truthful
- Rebuilt AXI from `EvidenceIR -> SemanticIR -> IntentIR -> validate` and refreshed the tracked four-artifact projection.
- The fake `Tie-off` actor is gone.
- The old AXI warning about `6` consumer-less `BROADCAST*` connectivity records is gone.
- AXI now carries `7` findings instead of `8`, even though the overall score stays `85/100 GOOD`.
- The score stayed flat because graph-direction coverage became more honest after removing the fake producer path:
  - `with_graph_direction` dropped from `170` to `164`
  - graph-derived direction lag rose from `118` to `124`
- The remaining honest AXI outliers are now:
  - `ARCHUNKEN` producer ambiguity
  - graph-direction coverage lag
  - `15` typed temporal conflicts
  - the dedicated infrastructure sourcing note for `ACLK` / `ARESETN`

### Validation
- `cargo test --manifest-path Cargo.toml tie_off_source_rows_become_input_declarations_without_fake_actor -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml external_source_rows_do_not_synthesize_infrastructure_outputs -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/source_ir.json` → passed (`extracted_statement_count: 6974`)
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json` → passed (`actor_count: 15`, `residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json` → passed (`actor_count: 15`, `residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json` → passed (`85/100 GOOD`, `finding_count: 7`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-09 (external infrastructure rows no longer synthesize false AXI outputs)

### Fixed: `External` is no longer treated as a protocol actor in source-column relation recovery
- Tightened [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) so the shared actor-term hygiene now rejects generic environment labels like `External`.
- Tightened [evidence.rs](crates/specforge/src/ir/evidence.rs) so source-column signal-table relation extraction now uses the stricter relation-actor normalizer instead of the weaker raw table-label normalizer.

### Added: regression coverage for external infrastructure rows
- Added `external_source_rows_do_not_synthesize_infrastructure_outputs`, which proves `ACLK` / `ARESETN` rows with `Source = External` still recover clock/reset semantics locally but no longer synthesize fake protocol actors or false `output` declarations.
- Kept `source_table_relations_skip_infrastructure_labels` green, so the broader infrastructure-row rejection path remains intact.

### Changed: AXI still scores `85/100 GOOD`, but the interface-conflict surface is cleaner again
- Rebuilt AXI from `EvidenceIR -> SemanticIR -> IntentIR -> validate` and refreshed the four-artifact validation projection.
- AXI now carries:
  - `0` interface signal conflicts
  - `0` residual decisions
  - `0` semantic-role conflicts
  - `0` blocked handshake-name fallbacks
- The score stays `85/100 GOOD`, so the remaining drag is now clearly elsewhere:
  - `ARCHUNKEN` producer ambiguity
  - graph-direction coverage lag
  - `15` typed temporal conflicts
  - infrastructure sourcing still intentionally lives under the dedicated `[info:system_contract]` note for `ACLK` / `ARESETN`

### Validation
- `cargo test --manifest-path Cargo.toml external_source_rows_do_not_synthesize_infrastructure_outputs -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml source_table_relations_skip_infrastructure_labels -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/source_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json` → passed (`actor_count: 16`, `residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json` → passed (`actor_count: 16`, `residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json` → passed (`85/100 GOOD`, `interface_signal_conflicts: 0`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-09 (AXI semantic-hint hygiene removed false handshake conflict paths)

### Fixed: generic acknowledged-event prose no longer masquerades as ready-like semantics
- Tightened [evidence.rs](crates/specforge/src/ir/evidence.rs) so generic acknowledgment wording no longer becomes `handshake_ready_like` by default.
- Ready-like acknowledgment recovery now requires more specific request/transfer/receipt phrasing instead of treating any `acknowledged` sentence as handshake acceptance semantics.

### Fixed: table-of-contents dot-leader lines no longer produce semantic-role hints
- Added a structural-noise guard in [evidence.rs](crates/specforge/src/ir/evidence.rs) so dot-leader contents rows and similar non-semantic structural lines stop contributing prose semantic hints outside real signal-description tables.
- This closes the exact false-positive path that had been turning the AXI contents line for `A14.1.1 AWAKEUP rules and recommendations` into a bogus `valid_like` + `ready_like` conflict.

### Added: focused regressions for both false-positive paths
- Added `acknowledged_event_prose_does_not_create_ready_like_hint`.
- Added `dot_leader_contents_lines_do_not_create_semantic_hints`.

### Changed: AXI stays at `85/100 GOOD`, but the artifact is cleaner again
- Rebuilt AXI from `EvidenceIR -> SemanticIR -> IntentIR -> validate` and refreshed the four-artifact validation projection.
- AXI now carries:
  - `0` residual decisions
  - `0` semantic-role conflicts
  - `0` blocked handshake-name fallbacks
- The score stays `85/100 GOOD`, so the remaining drag is no longer semantic-role noise; the main live AXI gaps are now the `ARCHUNKEN` producer ambiguity, the `ACLK` / `ARESETN` interface direction disagreement, graph-direction coverage lag, and `15` temporal conflicts.

### Validation
- `cargo test --manifest-path Cargo.toml acknowledged_event_prose_does_not_create_ready_like_hint -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml dot_leader_contents_lines_do_not_create_semantic_hints -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/source_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json` → passed (`85/100 GOOD`, `signal_semantic_conflicts: 0`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-09 (mdBook is explicitly a live project book, not a static scaffold)

### Changed: the documentation contract now treats the book as a living project surface
- Updated [README.md](README.md) so the entry-point docs now say the `mdBook` should be treated as a live book that evolves with user-facing project changes.
- Updated [ROADMAP.md](ROADMAP.md) so the cross-cutting doctrine now says the `mdBook` under `docs/book/` must evolve alongside user-facing commands, runtime behavior, IR semantics, validation surfaces, and cross-document learning behavior.
- Updated [DEVELOPMENT_NOTES.md](DEVELOPMENT_NOTES.md) so the book is explicitly part of the continuity contract: it is the canonical user-facing documentation surface, it should be treated as a live book, and meaningful user-facing changes should refresh it in the same task instead of being left to drift.

## 2026-04-09 (mdBook is now the canonical user-facing docs surface)

### Added: a real `mdBook` for layered user-facing documentation
- Added the canonical book scaffold under [docs/book/book.toml](docs/book/book.toml) and [docs/book/src/SUMMARY.md](docs/book/src/SUMMARY.md).
- Seeded the first layered chapter set for:
  - introduction
  - getting started
  - runtime and `doctor`
  - command workflow
  - pipeline model (`SourceIR`, `EvidenceIR`, `SemanticIR`, `IntentIR`)
  - reference material for generated artifacts, live docs, and troubleshooting

### Changed: root docs now point to the book instead of trying to be the full user-doc surface themselves
- [README.md](README.md) now marks the `mdBook` as the canonical user-facing documentation path and explains how to build it locally.
- [USER_GUIDE.md](USER_GUIDE.md) is now a compatibility pointer to the book instead of a second large parallel user-doc surface.

### Changed: CI now treats docs as first-class project quality, not an optional side task
- Added [scripts/run_docs_ci.sh](scripts/run_docs_ci.sh) as the canonical local docs build entrypoint.
- [scripts/run_ci.sh](scripts/run_ci.sh) now runs the mdBook build after Rust formatting and tests.
- [.github/workflows/ci.yml](.github/workflows/ci.yml) now installs `mdbook v0.5.2` before running the shared CI script, so GitHub checks the same Rust + docs path that local CI runs.

### Validation
- `bash scripts/run_docs_ci.sh` → passed
- `bash scripts/run_ci.sh` → passed (`245/245` tests, then mdBook build)

## 2026-04-09 (abstract transport tables no longer leak into canonical AXI interfaces)

### Fixed: generic `Tx` / `Rx` transport-primitives no longer masquerade as top-level interface signals
- Tightened [evidence.rs](crates/specforge/src/ir/evidence.rs) so `signal_description` tables are rejected from the top-level signal surface when they are really abstract transport exemplars: bare transport primitive names such as `VALID`, `PENDING`, `CRDT`, `CRDTSH`, `SHAREDCRD`, and `RP` combined with only `Tx` / `Rx` actor terms.
- This keeps real prefixed interface tables like `AWVALID`, `ARCRDT`, or `AWSHAREDCRD` intact, while preventing appendix-level transport teaching tables from authoring canonical declarations, actor relations, and semantic hints.

### Added: regression coverage for abstract transport-table leakage
- Added a focused evidence regression proving that a `Credited channel signals` table containing only abstract `Tx` / `Rx` transport primitives does not synthesize top-level signal declarations, actor relations, or semantic hints.
- Kept the existing standalone `VALID` / `READY` semantic regression green, so the fix stays narrow instead of globally banning simple protocols that really do use those signal names.

### Changed: AXI still scores `85/100 GOOD`, but the artifact is much cleaner and more honest
- Rebuilt AXI from `EvidenceIR -> SemanticIR -> IntentIR -> validate` and refreshed the four-artifact validation projection.
- The fake bare-transport `VALID` surface is gone from canonical AXI. Actor count dropped from `19` to `17`, unresolved consumer-less connectivity collapsed from `178` signals to `6`, and structural producer ambiguity dropped from `2` signal-connectivity conflicts to `1`.
- The remaining AXI residual/finding surface is now narrower and more truthful:
  - blocked handshake fallback moved from contested bare `VALID` to contested `CRVALID`
  - semantic conflicts are now specific to `AWAKEUP` and `CRVALID`
  - the remaining structural producer ambiguity is `ARCHUNKEN`
  - interface conflicts remain the infrastructure direction disagreement on `ACLK` / `ARESETN`

### Validation
- `cargo test --manifest-path Cargo.toml abstract_transport_signal_tables_do_not_become_top_level_interfaces -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml builds_semantic_ir_from_handshake_evidence -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/source_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json` → passed (`actor_count: 17`, `residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json` → passed (`behavior_count: 1202`, `constraint_count: 1244`)
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json` → passed (`85/100 GOOD`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-09 (axi field-like message tables no longer leak pseudo-signals)

### Fixed: field-like `Name | Width | Description` tables no longer masquerade as interface signal tables
- Tightened [evidence.rs](crates/specforge/src/ir/evidence.rs) so top-level signal-table recovery now considers the nearest section title as well as the local caption and headers.
- Continued-page DVM message-field tables now stay classified as field-like context instead of leaking pseudo-signals such as `IS`, `PA`, and `COMPLETION` into `EvidenceIR`.

### Added: regression coverage for continued-page field-table leakage
- Added a focused evidence regression proving that a misclassified field-like `Name | Width | Description` continuation table does not synthesize fake signal declarations, polarity facts, or semantic hints.

### Changed: AXI live quality improved and its remaining gaps are more honest
- Rebuilt AXI from `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> validate` and refreshed the four-artifact validation projection.
- AXI improved from `84/100 GOOD` to `85/100 GOOD`.
- The fake `PA` / `COMPLETION` missing-producer warning is gone, the fake `IS` polarity conflict is gone, the canonical signal denominator dropped from `312` to `294`, graph-derived direction coverage improved from `57%` to `59%`, and the remaining AXI residual surface is now the single blocked handshake-name fallback on contested `VALID`.

### Validation
- `cargo test --manifest-path Cargo.toml misclassified_field_table_does_not_synthesize_fake_signal_semantics -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml field_like_width_table_does_not_leak_message_fields_as_signals -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/source_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json` → passed (`actor_count: 19`, `residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json` → passed (`behavior_count: 1202`, `constraint_count: 1272`)
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json` → passed (`85/100 GOOD`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (passive visual links no longer force semantic residuals)

### Fixed: ambiguous-visual residuals now require live semantic lift
- Tightened [semantic.rs](crates/specforge/src/ir/semantic.rs) so `semantic_ambiguous_visual_grounding` is emitted only when ambiguous or unknown visual evidence actually survives into carried semantic observations.
- Passive figure references that are merely linked from prose no longer keep a semantic-stage residual alive by themselves.

### Added: regression coverage for passive-vs-live visual grounding
- Added a focused semantic regression proving that a passive ambiguous figure link does not emit a residual packet.
- Added a paired regression proving that an actually lifted visual-backed semantic observation still keeps the residual visible when the visual role remains ambiguous.

### Changed: APB, AHB, and AXI now carry zero residual decisions
- Rebuilt `SemanticIR` / `IntentIR` / validation for the live APB, AHB, and AXI artifacts and refreshed the tracked four-artifact projection.
- APB, AHB, and AXI now all carry `0` residual decisions end to end; the old common `semantic_ambiguous_visual_grounding` residual is gone because those live artifacts were only carrying passive figure links, not active visual semantic lift.
- The refreshed live projection is now AXI `85/100 GOOD`, APB `94/100 EXCELLENT`, AHB `94/100 EXCELLENT`, and AXI-Stream `90/100 EXCELLENT`; a follow-on rebuild from current `SourceIR` / `EvidenceIR` restored APB and AHB to the excellent lane while leaving AXI as the main live quality outlier.

### Validation
- `cargo test --manifest-path Cargo.toml passive_ambiguous_visual_links_do_not_emit_residual_decision -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml emits_residual_decision_for_ambiguous_visual_semantic_grounding -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/evidence_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (authoritative signal surface now anchors interface grouping)

### Fixed: heuristic interface grouping now respects declared signal vocabularies
- Tightened [semantic.rs](crates/specforge/src/ir/semantic.rs) so statement-derived interface fragments are filtered against document-grounded explicit signal declarations whenever that authoritative signal surface exists.
- This means phase words, enum labels, width symbols, and similar metadata no longer survive into heuristic interface grouping just because they were co-mentioned next to real signals in prose.

### Added: regression coverage for authoritative grouping filters
- Added a focused semantic regression proving authoritative signal vocabularies suppress undeclared metadata like `SETUP` / `ACCESS` while retaining real declared signals.

### Changed: APB, AHB, and AXI all lost the carried interface-grouping residual
- Rebuilt `SemanticIR` / `IntentIR` / validation for the live APB, AHB, and AXI artifacts and refreshed the tracked four-artifact projection.
- `semantic_interface_grouping` is gone across all three; a later follow-up also removed the remaining passive visual residuals, so the current live baseline no longer carries any residual decisions on APB/AHB/AXI.
- AXI improved from the stale projected `79/100 GOOD` back to `84/100 GOOD`; APB remains `84/100 GOOD`, AHB later moved to `85/100 GOOD`, and AXI-Stream stays `90/100 EXCELLENT`.

### Validation
- `cargo test --manifest-path Cargo.toml retain_authoritative_interface_candidate_signals_prefers_declared_surface -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml overlapping_interface_signals_ignore_fragments_subsumed_by_explicit_interfaces -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/evidence_ir.json` → passed (`residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json` → passed (`residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json` → passed (`residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed
- `bash scripts/run_ci.sh` → passed
## 2026-04-08 (AXI-Stream interface grouping residual removed cleanly)

### Fixed: heuristic interface grouping now ignores width/table metadata noise
- Tightened [semantic.rs](crates/specforge/src/ir/semantic.rs) so heuristic interface grouping filters out metadata-only symbols like `*_WIDTH`, `_WIDTH`, `MIN`, and `MAX` before building statement-derived interface fragments.
- This complements the earlier signal-token gate that already rejected leading-digit hex-like values such as `0A`, `0B`, `0E`, and `0F`.

### Fixed: explicit interfaces now subsume smaller grouped fragments for overlap review
- `semantic_interface_grouping` residual generation now ignores heuristic fragments that are fully subsumed by an explicit interface, so carried overlap is only reported when there is still a real unresolved grouping question.
- In AXI-Stream, that resolves the last carried residual decision instead of preserving a bookkeeping artifact caused by one explicit interface plus many smaller statement fragments.

### Added: regression coverage for metadata filtering and explicit-subsumption overlap handling
- Added focused semantic regressions proving width/table metadata is filtered from heuristic interface candidates.
- Added focused semantic regressions proving explicit interfaces suppress already-subsumed overlap while genuinely unsubsumed heuristic overlap still remains visible.

### Changed: AXI-Stream now carries zero residual decisions without score inflation
- Rebuilt AXI-Stream `SemanticIR` and `IntentIR`, then refreshed the tracked four-artifact validation projection.
- AXI-Stream remains at `90/100 EXCELLENT`, but interface count drops from `46` to `29`, `SemanticIR` / `IntentIR` residual decisions both drop to `0`, and the only remaining projected finding is the infrastructure `system_contract` note for `ACLK` / `ARESETN`.

### Validation
- `cargo test --manifest-path Cargo.toml filtered_interface_candidate_signals_drop_width_and_table_metadata_noise -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml overlapping_interface_signals_ignore_fragments_subsumed_by_explicit_interfaces -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml overlapping_interface_signals_keep_unsubsumed_heuristic_overlap_visible -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json` → passed (`interface_count: 29`, `residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`90/100 EXCELLENT`, `residual_decisions: 0`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (doctor now checks LM Studio fallback readiness too)

### Added: doctor now verifies the LM Studio fallback path as well as the default Ollama path
- Extended [doctor.rs](crates/specforge/src/commands/doctor.rs) so `specforge doctor [--strict]` now checks and reports:
  - LM Studio `/v1/models`
  - default-model presence for `qwen2.5vl:7b`
  - LM Studio OpenAI-compatible `/v1/chat/completions`
- The strict gate still reflects the default local-first pipeline (`Docling` + `Ollama`), but the CLI now surfaces whether the `lmstudio` fallback is actually usable before a long rerun depends on it.

### Added: shared OpenAI-compatible parsing for local provider preflight
- `doctor.rs` now parses OpenAI-compatible `/v1/models` payloads and reuses the same chat-response parser for both Ollama and LM Studio, instead of keeping the loopback preflight logic Ollama-specific.
- Added focused unit coverage for `/v1/models` parsing and kept the OpenAI-compatible chat parsing under test.

### Changed: the live runtime picture is now more honest
- Verified outside the sandbox that `cargo run --manifest-path Cargo.toml -- doctor --strict` now reports:
  - Docling ready via `python3.11` + `docling 2.84.0`
  - Ollama loopback fully ready for `qwen2.5vl:7b`
  - LM Studio fallback not currently reachable at `http://localhost:1234`, even though LM Studio is installed locally
- That distinction matters: “installed” is not the same as “serving a model,” and `doctor` now makes that operational difference explicit.

### Validation
- `cargo test --manifest-path Cargo.toml parse_openai_models_response_detects_default_model -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml parse_openai_chat_response_accepts_openai_compatible_string_content -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml doctor_defaults_to_non_strict -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- doctor --strict` → passed (outside sandbox; Docling + Ollama ready, LM Studio fallback reported unavailable)

## 2026-04-08 (doctor now checks Ollama loopback readiness too)

### Added: doctor now verifies the default local Ollama runtime, not just Docling
- Extended [doctor.rs](crates/specforge/src/commands/doctor.rs) so `specforge doctor [--strict]` now checks:
  - Docling ingest readiness
  - Ollama `/api/tags`
  - default-model presence for `qwen2.5vl:7b`
  - Ollama OpenAI-compatible `/v1/chat/completions`
- This catches the exact failure mode discovered during the fresh AXI rerun: a long `converge` can otherwise get all the way through fresh ingest before discovering that the local chat-completions path is not actually usable in the current execution environment.

### Added: typed parsing and reporting for the default Ollama loopback path
- `doctor.rs` now parses visible Ollama models from `/api/tags`, validates OpenAI-compatible chat responses from `/v1/chat/completions`, and reports both readiness and resolution text explicitly.
- Added focused unit coverage for Ollama tags parsing and chat-response parsing.

### Changed: the local runtime preflight now covers the full default local-first pipeline
- Verified outside the sandbox that `cargo run --manifest-path Cargo.toml -- doctor --strict` now reports:
  - Docling ready via `python3.11` + `docling 2.84.0`
  - Ollama tags reachable
  - Ollama chat-completions reachable
  - default model `qwen2.5vl:7b` present
- That means the default `specforge converge` runtime can now be preflighted honestly before a large PDF run instead of discovering the Ollama-side failure deep into the loop.

### Validation
- `cargo test --manifest-path Cargo.toml doctor_defaults_to_non_strict -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml parse_ollama_tags_response_detects_default_model -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml parse_ollama_chat_response_accepts_openai_compatible_string_content -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml parse_ollama_chat_response_accepts_array_content -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- doctor --strict` → passed (outside sandbox; Docling + Ollama loopback both ready)

## 2026-04-08 (Docling runtime discovery, doctor command, and bootstrap path)

### Added: a first-class Docling runtime doctor command
- Added [doctor.rs](crates/specforge/src/commands/doctor.rs) and wired `specforge doctor [--strict]` into the CLI in [cli.rs](crates/specforge/src/cli.rs), [commands/mod.rs](crates/specforge/src/commands/mod.rs), and [lib.rs](crates/specforge/src/lib.rs).
- The new command reports Docling readiness, the selected Python candidate, version information, all probe results, the repo-local bootstrap script path, and the exact missing-runtime resolution when `--strict` is used.

### Changed: Docling runtime discovery is now operationally stronger
- Extended [docling_backend.rs](crates/specforge/src/ir/source/docling_backend.rs) so the backend no longer depends only on ambient `python3` / `python`.
- Runtime resolution now proceeds in this order:
  - `SPECFORGE_DOCLING_PYTHON`
  - repo-local `.venv-docling`
  - versioned Python probes such as `python3.11`, `python3.12`, and `python3.10`
  - generic `python3` / `python`
- The resolver now keeps a typed diagnosis surface instead of a one-bit import probe, which is shared by both `specforge doctor` and the actual ingest backend.

### Added: supported repo-local Docling bootstrap path
- Added [bootstrap_docling.sh](scripts/bootstrap_docling.sh) as the supported repository-local Docling runtime bootstrap entrypoint.
- The script creates `.venv-docling`, installs the known-good `docling==2.84.0` runtime family by default, and prints the resulting interpreter/version state.
- Added `/.venv-docling/` to [.gitignore](.gitignore) so that runtime stays local and untracked.

### Changed: the local runtime issue is now concretely verified, not just documented
- `cargo run --manifest-path Cargo.toml -- doctor --strict` now succeeds locally and selects `python3.11` with `docling 2.84.0`, while explicitly reporting that the ambient `python3` probe is still broken because it resolves to Python `3.14.3` without `docling`.
- A fresh original-PDF ingest rerun on the AHB spec now succeeds again:
  - `cargo run --manifest-path Cargo.toml -- ingest /Users/richarddje/Documents/livework/chipdoc/arm/amba/core/ahb/current/IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf`
  - result: `normalization_status: ready`, `page_artifact_count: 104`, `visual_asset_count: 70`

### Validation
- `cargo test --manifest-path Cargo.toml doctor_defaults_to_non_strict -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml inspect_docling_runtime_prefers_repo_local_venv -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml inspect_docling_runtime_prefers_python311_path_probe_over_generic_python3 -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- doctor --strict` → passed
- `cargo run --manifest-path Cargo.toml -- ingest /Users/richarddje/Documents/livework/chipdoc/arm/amba/core/ahb/current/IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf` → passed


## 2026-04-08 (system-contract infrastructure signals now populate canonical interfaces)

### Fixed: clock/reset signals from the system contract now reach the canonical interface surface
- Extended [semantic.rs](crates/specforge/src/ir/semantic.rs) so grounded `system_contract` clock/reset signals are synthesized into the top-level explicit interface when ordinary signal declarations do not already carry them.
- This lets infrastructure signals like `HCLK` and `HRESETN` contribute honest canonical interface direction/width coverage, and it allows reset polarity grounded only through system-contract text to surface as `resolved_polarity` in both `SemanticIR` and `IntentIR`.
- Added focused regressions in [semantic.rs](crates/specforge/src/ir/semantic.rs) and [validate.rs](crates/specforge/src/commands/validate.rs) covering the exact system-contract-only clock/reset case at semantic and intent validation time.

### Changed: AHB now reports resolved polarity in the live baseline
- Rebuilt AHB `SemanticIR` and `IntentIR` sequentially from the current `EvidenceIR`, re-validated the artifact, and refreshed the tracked four-document projection.
- AHB now reports `with_resolved_polarity: 1` and the live AMBA polarity line is now `1 / 1 / 1 / 1`; the overall AHB score stays `84/100 GOOD`, but the infrastructure reset polarity is now represented honestly in the canonical interface surface.
- A full original-PDF `converge` rerun was attempted first, but the local environment currently lacks an importable `docling` runtime for `python3`, so authoritative fresh-ingest reruns remain blocked until `docling` is installed or `SPECFORGE_DOCLING_PYTHON` points at a working interpreter.

### Validation
- `cargo test --manifest-path Cargo.toml system_contract_signals_become_explicit_interface_records -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml validate_semantic_ir_counts_system_contract_resolved_polarity -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml validate_intent_ir_counts_system_contract_resolved_polarity -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json` → passed (`84/100 GOOD`, `with_resolved_polarity: 1`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (resolved signal polarity now lives on canonical interface records)

### Added: canonical interface records now carry resolved polarity directly
- Extended [semantic.rs](crates/specforge/src/ir/semantic.rs) so each `InterfaceSignalRecord` can now carry `resolved_polarity` directly instead of forcing downstream consumers to reconstruct polarity only from the carried top-level side list.
- [intent.rs](crates/specforge/src/ir/intent.rs) now preserves that same per-signal polarity surface into `IntentIR`.
- [validate.rs](crates/specforge/src/commands/validate.rs) now reports `with_resolved_polarity` for both `SemanticIR` and `IntentIR`.

### Changed: the live corpus now shows the gap honestly
- Rebuilt the live AMBA `SemanticIR` / `IntentIR` artifacts, re-validated the four-document projection, and refreshed the tracked snapshot docs.
- The canonical polarity surface is now present in the live corpus too: AXI, APB, AHB, and AXI-Stream each currently report `with_resolved_polarity: 1`, so the remaining polarity work is broader non-reset control coverage rather than carry-through plumbing.
- That refresh also replaced a stale optimistic validation snapshot; after the later current-`SourceIR` / current-`EvidenceIR` rebuild and the follow-on AXI field-table truthfulness fix, the tracked live baseline now stands at AXI `85/100 GOOD`, APB `94/100 EXCELLENT`, AHB `94/100 EXCELLENT`, and AXI-Stream `90/100 EXCELLENT`.

### Validation
- `cargo test --manifest-path Cargo.toml carries_resolved_signal_polarity_into_interface_records -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml carries_resolved_signal_polarity_into_intent_ir -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml validate_semantic_ir_counts_resolved_signal_polarity -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml validate_intent_ir_counts_resolved_signal_polarity -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/semantic_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`90/100 EXCELLENT`, `with_resolved_polarity: 1`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (temporal conflict comparison is now polarity-aware)

### Fixed: asserted/deasserted temporal semantics now respect signal polarity
- Extended [semantic.rs](crates/specforge/src/ir/semantic.rs), [evidence.rs](crates/specforge/src/ir/evidence.rs), and [intent.rs](crates/specforge/src/ir/intent.rs) so resolved signal polarity now survives into canonical IR and can guide temporal-conflict comparison.
- `ASSERTED` and `DEASSERTED` are now treated as polarity-relative assertion semantics, not as fixed synonyms for `HIGH` and `LOW`.
- When the current document grounds polarity, conflict detection now maps assertion semantics through that local polarity:
  - active-high: `ASSERTED -> HIGH`, `DEASSERTED -> LOW`
  - active-low: `ASSERTED -> LOW`, `DEASSERTED -> HIGH`
- When polarity is still unknown, `ASSERTED` stays abstract instead of manufacturing or suppressing a level conflict.

### Changed: AXI-Stream timing semantics are now cleaner again without changing the score
- Rebuilt `SemanticIR` and `IntentIR` for AXI-Stream from the current `EvidenceIR`, re-validated the artifact, and refreshed the tracked four-document projection.
- The score stayed at `90/100 EXCELLENT`, but AXI-Stream now carries `0` typed temporal conflicts instead of `1`.
- The remaining dominant honest gaps are now:
  - the infrastructure-sourcing/system-contract note for `ACLK` and `ARESETN`
  - the carried `semantic_interface_grouping` residual decision

### Validation
- `cargo test --manifest-path Cargo.toml derives_typed_temporal_conflicts_from_conflicting_value_rules -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml asserted_and_high_do_not_form_temporal_conflicts_without_known_polarity -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml asserted_and_high_form_temporal_conflict_for_active_low_signal -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/semantic_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`90/100 EXCELLENT`, `temporal_conflicts: 0`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (same-cycle timing language now lands as bounded temporal semantics)

### Fixed: same-cycle timing phrases now recover explicit `0`-cycle windows
- Extended [semantic.rs](crates/specforge/src/ir/semantic.rs) so `extract_cycle_window_from_text()` now recognizes bounded same-cycle language such as `in the same ACLK cycle`, `in the same tick`, and `on the current rising edge`.
- Added focused semantic regressions that lock both layers of the behavior:
  - direct phrase recovery from same-cycle timing language
  - end-to-end temporal-rule derivation from a same-cycle signal constraint

### Changed: AXI-Stream timing semantics are now more explicit without changing the score
- Re-ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- Re-validated the rebuilt artifact and refreshed the tracked four-document projection.
- The score stayed at `90/100 EXCELLENT`, but six AXI-Stream temporal rules now carry explicit `0`-cycle windows for same-cycle handshake/timing language, so the old `intent_temporal_rules_missing_cycle_windows` warning is gone.
- The timing surface also got cleaner as a side effect: AXI-Stream now carries `1` typed temporal conflict instead of `2`.
- The remaining dominant honest gaps are now:
  - the dedicated infrastructure-sourcing note for `ACLK` / `ARESETN`
  - the single remaining typed temporal conflict
  - the carried `semantic_interface_grouping` residual decision

### Validation
- `cargo test --manifest-path Cargo.toml extracts_zero_cycle_window_from_same_cycle_phrases -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml derives_zero_cycle_window_from_same_cycle_constraint_text -- --nocapture` → passed
- full `specforge converge` on AXI-Stream with Ollama VLM + NLP Level 3 → converged in `2` iterations
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`90/100 EXCELLENT`, `temporal_rules_with_cycle_window: 6`, `temporal_conflicts: 1`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (clock/reset connectivity now validates as infrastructure)

### Changed: clock/reset connectivity is now classified as infrastructure in canonical IR
- Extended [semantic.rs](crates/specforge/src/ir/semantic.rs) so `SignalConnectivityRecord` now carries an explicit `connectivity_class`, with `SystemClock` and `SystemReset` derived from the local system contract instead of flattening those signals into ordinary protocol connectivity.
- Extended [intent.rs](crates/specforge/src/ir/intent.rs) so that infrastructure classification survives into `IntentIR` unchanged.
- Extended [validate.rs](crates/specforge/src/commands/validate.rs) so missing producers on infrastructure connectivity no longer emit the generic `[warning:signal_connectivity]` finding; they now surface as a dedicated `[info:system_contract]` note that keeps canonical sourcing on the system-contract side of the model.

### Changed: AXI-Stream still validates at `90/100 EXCELLENT`, but the remaining gap is now represented more honestly
- Re-ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- Re-validated the rebuilt artifact and refreshed the tracked four-document projection.
- The score stayed at `90/100 EXCELLENT`, with declared graph-direction and width coverage still at `22/22`, but `ACLK` and `ARESETN` now surface under `infrastructure_signal_connectivity: 2` with an `[info:system_contract]` finding instead of a generic missing-producer warning.
- The dominant remaining honest gaps are now:
  - typed temporal rules that still have no explicit cycle-window bounds
  - the two carried temporal conflicts
  - the remaining interface-grouping residual decision

### Validation
- `cargo test --manifest-path Cargo.toml clock_and_reset_gain_input_actor_ports_for_relation_actors -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml carries_infrastructure_signal_connectivity_class_into_intent_ir -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml validate_intent_ir_treats_clock_and_reset_as_infrastructure_connectivity -- --nocapture` → passed
- full `specforge converge` on AXI-Stream with Ollama VLM + NLP Level 3 → converged in `2` iterations
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`90/100 EXCELLENT`, `infrastructure_signal_connectivity: 2`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (corpus knowledge base plane added to roadmap)

### Added: explicit `R15g` workstream for a corpus knowledge base layer
- Logged a new roadmap slice in [ROADMAP.md](ROADMAP.md) for a persistent corpus knowledge base that sits beside the per-document IR pipeline and the typed `CorpusMemory` prior store.
- The design boundary is explicit:
  - per-document canonical truth stays in `SourceIR` / `EvidenceIR` / `SemanticIR` / `IntentIR`
  - typed machine-usable reuse stays in `CorpusMemory`
  - the new corpus knowledge base becomes the human+LLM synthesis layer for recurring motifs, failures, contradiction summaries, table/figure families, and protocol-family notes

### Changed: live architecture guidance now targets three cross-document planes, not one
- Updated [DEVELOPMENT_NOTES.md](DEVELOPMENT_NOTES.md), [README.md](README.md), and [LIVE_ACHIEVEMENT_STATUS.md](LIVE_ACHIEVEMENT_STATUS.md) so future work treats the long-term shape as:
  - document-local canonical IR
  - typed cross-document priors
  - corpus-level compiled knowledge base
- The docs also now make the safety boundary explicit: the corpus knowledge base may guide humans, LLM synthesis, benchmark design, and prior-candidate generation, but it must not directly author canonical IR truth.

## 2026-04-08 (AXI-Stream parity-check width semantics now survive end to end)

### Fixed: parity-check rows now recover bounded width hints from their local table semantics
- Extended [evidence.rs](crates/specforge/src/ir/evidence.rs) so `Check Signal / Signals Covered / Width / Granularity / Check Enable` rows can recover width hints from the `Signals Covered` cell when the literal `Width` cell is only a range placeholder like `1-8`.
- Added a bounded fallback from `Check Enable` / `Granularity` to the grounded base signal when `Signals Covered` only carries a width expression, so the structural and width semantics stay tied to local evidence instead of remaining partially orphaned.
- Tightened graph-derived declaration synthesis so width-only statements no longer block stronger relation-grounded `Signal X is output width ...` declarations for the same signal.

### Changed: AXI-Stream now validates at `90/100 EXCELLENT`
- Re-ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- The run still converged in `2` pipeline iterations, but the carried `*CHK` surface is now complete enough to count honestly: declared signal records rose from `21` to `22`, actor-signal relations rose from `38` to `40`, actor ports rose from `42` to `44`, signal connectivity rose from `21` to `22`, compatibility direction hints reached `22/22`, width coverage reached `22/22`, and the projected score improved from `88/100 GOOD` to `90/100 EXCELLENT`.
- The remaining dominant gaps are now:
  - unresolved producer attribution for infrastructure signals `ACLK` and `ARESETN`
  - typed temporal rules that still have no explicit cycle-window bounds
  - the two carried temporal conflicts

### Validation
- `cargo test --manifest-path Cargo.toml check_signal_tables_inherit_relations_from_covered_signals -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml source_table_relations_infer_unique_complementary_reads -- --nocapture` → passed
- full `specforge converge` on AXI-Stream with Ollama VLM + NLP Level 3 → converged in `2` iterations
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`90/100 EXCELLENT`, declared graph-direction and width coverage `22/22`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (AXI-Stream parity-check table now restores structural ownership)

### Fixed: parity-check tables now recover actor-signal relations from covered base signals
- Extended [evidence.rs](crates/specforge/src/ir/evidence.rs) with a bounded second-pass relation recovery path for `Check Signal / Signals Covered` tables.
- When a local parity-check row explicitly ties a check signal to a covered base signal that already has grounded actor relations, the check signal now inherits those local `drives` / `reads` edges instead of remaining structurally orphaned.
- Added the focused regression `check_signal_tables_inherit_relations_from_covered_signals`.

### Changed: AXI-Stream now validates at `88/100 GOOD`
- Re-ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- The run still converged in `2` pipeline iterations, but parity-check ownership now survives end to end: declared signal records rose from `16` to `21`, graph-direction coverage rose from `12/16` to `21/21`, actor ports rose from `24` to `42`, signal connectivity rose from `12` to `21`, and the projected score improved from `84/100 GOOD` to `88/100 GOOD`.
- The remaining dominant gaps are now:
  - missing widths on `TDESTCHK`, `TIDCHK`, `TSTRBCHK`, `TUSERCHK`, and `TWAKEUPCHK`
  - unresolved producer attribution for infrastructure signals `ACLK` and `ARESETN`

### Validation
- `cargo test --manifest-path Cargo.toml check_signal_tables_inherit_relations_from_covered_signals -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml source_table_relations_infer_unique_complementary_reads -- --nocapture` → passed
- full `specforge converge` on AXI-Stream with Ollama VLM + NLP Level 3 → converged in `2` iterations
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`88/100 GOOD`, graph-direction coverage `21/21`)

## 2026-04-08 (clock/reset semantics logged as infrastructure-first steering)

### Changed: design steering now treats clocks and resets as infrastructure semantics, not ordinary protocol edges
- Logged the implementation doctrine in [DEVELOPMENT_NOTES.md](DEVELOPMENT_NOTES.md): clocks and resets should remain first-class infrastructure semantics with conservative sourcing/distribution modeling, not flattened into ordinary protocol producer/consumer behavior.
- Updated [ROADMAP.md](ROADMAP.md) so `R15b` now explicitly carries that requirement forward into the clock-tick temporal-model workstream.
- This locks an important architectural boundary for future work on `ACLK`, `ARESETN`, and similar infrastructure signals: graph carry-through is allowed as a local aid, but long-term canonical truth should prefer dedicated infrastructure semantics over false graph completeness.

## 2026-04-08 (AXI-Stream graph-direction coverage rises after width-symbol cleanup)

### Fixed: width-only `_WIDTH` declarations no longer masquerade as interface signals
- Hardened [semantic.rs](crates/specforge/src/ir/semantic.rs) so synthesized declarations like `Signal TDATA_WIDTH is width LOW.` no longer become canonical interface-signal records when they carry width metadata but no real port direction.
- Added focused regressions for both sides of the boundary:
  - `width_only_width_parameter_declarations_do_not_become_interface_signal_records`
  - `width_only_signal_declarations_become_interface_signal_records`

### Fixed: relation-grounded actors now inherit clock/reset input ports from explicit system contracts
- Extended [semantic.rs](crates/specforge/src/ir/semantic.rs) so actors already grounded by structural KG evidence now receive `input` actor ports for the explicit clock and reset signals instead of leaving `ACLK` / `ARESETN` outside the graph-backed port surface.
- Added the regression `clock_and_reset_gain_input_actor_ports_for_relation_actors`, which locks that actor-relative clock/reset carry-through path.

### Changed: AXI-Stream now validates at `84/100 GOOD`
- Re-ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- The run still converged in `2` pipeline iterations, but the canonical denominator is now more honest: declared interface signals dropped from `20` to `16`, graph-direction coverage rose from `10/20` to `12/16`, and the projected score improved from `80/100 GOOD` to `84/100 GOOD`.
- The remaining dominant gaps are now narrower and clearer:
  - the four `*CHK` signals still lack graph-derived direction coverage
  - `ACLK` and `ARESETN` still lack resolved producer actors even though they now have graph-backed consumer ports

### Validation
- `cargo test --manifest-path Cargo.toml width_only_width_parameter_declarations_do_not_become_interface_signal_records -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml clock_and_reset_gain_input_actor_ports_for_relation_actors -- --nocapture` → passed
- full `specforge converge` on AXI-Stream with Ollama VLM + NLP Level 3 → converged in `2` iterations
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`84/100 GOOD`, graph-direction coverage `12/16`)

## 2026-04-08 (AXI-Stream consumer-side connectivity now survives from source tables)

### Fixed: source-column signal tables can now recover the opposite-side reader when it is uniquely grounded
- Extended [evidence.rs](crates/specforge/src/ir/evidence.rs) so a `Source` / `Driver` column no longer stops at `(actor, Drives, signal)` when the current document already exposes exactly one opposite actor role locally.
- The new helper path builds a small local actor-role inventory from signal-description tables and section headings, then adds the complementary `Reads` edge only when the opposite requester-like/completer-like actor is unique.
- Added focused regressions for both the positive case and the ambiguity guard:
  - `source_table_relations_infer_unique_complementary_reads`
  - `source_table_relations_skip_complementary_reads_when_opposite_actor_is_ambiguous`
- Updated the tracked KG fixtures whose expected graph shape now honestly includes these complementary consumer edges.

### Changed: AXI-Stream now keeps consumer-side structural connectivity without changing its score
- Re-ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- The run still converged in `2` pipeline iterations and still validates at `80/100 GOOD`, but `IntentIR` now carries the missing consumer-side structural KG edges:
  - `Receiver reads TVALID`
  - `Transmitter reads TREADY`
  - `Receiver reads TDATA/TSTRB/TKEEP/TLAST/TID/TDEST/TUSER/TWAKEUP`
- The AXI-Stream validation finding for missing consumer actors is now gone; the remaining dominant gap is graph-derived direction coverage, not missing connectivity.

### Validation
- `cargo test --manifest-path Cargo.toml source_table_relations_infer_unique_complementary_reads -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml source_table_relations_skip_complementary_reads_when_opposite_actor_is_ambiguous -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml actor_taxonomy_priors_guide_source_column_direction_inference -- --nocapture` → passed
- full `specforge converge` on AXI-Stream with Ollama VLM + NLP Level 3 → converged in `2` iterations
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`80/100 GOOD`, consumer-gap finding removed)

## 2026-04-08 (learning plane now rejects bogus actor vocabulary)

### Fixed: actor-taxonomy learning no longer harvests payload nouns as actors
- Hardened [learn_priors.rs](crates/specforge/src/commands/learn_priors.rs) so actor-taxonomy priors now skip non-actor payload/event terms like `control information`, even if an earlier document-local bug let that text survive into `IntentIR`.
- Added a shared actor-term hygiene guard in [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) so actor-taxonomy prior lookup also ignores those bogus terms if an older local `CorpusMemory` still contains stale entries.
- Reused the same guard in [evidence.rs](crates/specforge/src/ir/evidence.rs), so live relation extraction and cross-document learning now reject the same class of bogus actor terms instead of drifting apart.

### Changed: local `CorpusMemory` is now cleaned of the stale AXI-Stream actor prior
- Re-ran `specforge learn-priors` across AXI/APB/AHB/AXI-Stream `IntentIR` artifacts.
- The local prior store now yields `16` actor-taxonomy priors, `5` semantic phrase priors, `4` semantic modality-reliability priors, `266` temporal phrase priors, and `99` table-shape priors.
- The stale `control information -> requester_like` actor-taxonomy prior is now gone from local `generated/prior_memory/corpus_memory.json`.

### Validation
- `cargo test --manifest-path Cargo.toml learn_priors_skips_payload_like_actor_terms -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- learn-priors generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`16 / 5 / 4 / 266 / 99`)
- `bash scripts/run_ci.sh` → passed (`209/209` tests)

## 2026-04-08 (AXI-Stream bogus prose actor extraction fixed)

### Fixed: prose KG extraction no longer promotes payload nouns into actors
- Hardened [evidence.rs](crates/specforge/src/ir/evidence.rs) so prose actor extraction now normalizes candidate actor phrases through the same non-actor guard used by table extraction and rejects generic payload/event nouns like `control information`, `data`, and `transfer`.
- Tightened active-clause subject recovery so coordinated prose like `the Transmitter presents ... and asserts TVALID` keeps the real actor subject instead of capturing trailing payload phrases or clause verbs.
- Added the regression `coordinated_active_drive_extracts_real_actor_not_payload_phrase`, which locks the AXI-Stream-style sentence shape that previously leaked `control information` into the structural KG.

### Changed: AXI-Stream structural truthfulness improved without score inflation
- Re-ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- The artifact still converges in `2` pipeline iterations and still validates at `80/100 GOOD`, but the carried multi-producer conflict on `TVALID` is now gone: `control information` no longer appears as an actor in `EvidenceIR`, `SemanticIR`, or `IntentIR`, and `signal_connectivity_conflicts` for AXI-Stream dropped from `1` to `0`.
- The remaining honest AXI-Stream gap is now clearer: unresolved consumer actors and graph-direction coverage, not bogus producer attribution.

### Validation
- `cargo test --manifest-path Cargo.toml coordinated_active_drive_extracts_real_actor_not_payload_phrase -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed
- `bash scripts/run_ci.sh` → passed (`208/208` tests)

## 2026-04-07 (AXI-Stream unseen-protocol run populates semantic priors)

### Added: first unseen-protocol full converge + learning refresh
- Ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- The AXI-Stream artifact converged in `2` pipeline iterations and validates at `80/100 GOOD`; it is now included in the tracked [VALIDATION_SNAPSHOT.md](VALIDATION_SNAPSHOT.md) projection and the managed validation block in [LIVE_ACHIEVEMENT_STATUS.md](LIVE_ACHIEVEMENT_STATUS.md).
- Refreshing `specforge learn-priors` across AXI/APB/AHB/AXI-Stream now yields `16` actor-taxonomy priors, `5` semantic phrase priors, `4` semantic modality-reliability priors, `266` temporal phrase priors, and `99` table-shape priors in local `CorpusMemory`.

### Fixed: multi-signal table-row semantic-role leakage
- Extended [evidence.rs](crates/specforge/src/ir/evidence.rs) so signal-description table rows now sanitize against the full local known-signal set before semantic-role inference, preventing a secondary signal mention like `TREADY` from leaking a ready-like role onto a row subject like `TVALID`.
- Added the regression `signal_table_descriptions_ignore_other_handshake_signal_mentions`, which locks the AXI-Stream-style case where `TVALID` should stay valid-like even when its row also describes the handshake condition involving `TREADY`.

### Why this matters
- This is the first concrete proof that a new unseen protocol document can both expose an extraction flaw and then materially strengthen the learning plane once the artifact is repaired enough to be harvested.
- It also marks the first real-corpus point where semantic phrase priors and semantic modality-reliability priors become nonzero instead of remaining only architecturally possible.

## 2026-04-07 (learning-plane structure documented as first-class architecture)

### Added: explicit doctrine for how `R15f` should learn
- Logged in [DEVELOPMENT_NOTES.md](DEVELOPMENT_NOTES.md) that the cross-document learning plane should not try to "mimic humans completely," but should instead borrow the useful structural properties of human learning:
  - accumulate experience across many documents
  - abstract patterns from repeated successful cases
  - keep confidence graded
  - remember failures and false positives
  - use prior experience to guide attention
  - still require local evidence before promoting a fact
- Logged the non-negotiable architectural properties for the learning plane:
  - separation between document truth and learned priors
  - typed, inspectable memory
  - bounded influence
  - validation-gated feedback
  - negative learning
  - provenance on learned priors

### Why this matters
- This frames `R15f` as an epistemology layer, not just a bigger cache.
- It makes explicit that the design risk is not "too little learning," but poorly structured learning that can poison canonical truth.

## 2026-04-07 (table-shape timing-table benchmark added)

### Added: second table-shape gold/negative pair
- Added [table_shape_prior_guided_timing_table_gold](crates/specforge/test_data/kg_quality/table_shape_prior_guided_timing_table_gold/fixture.json) and [table_shape_prior_guided_timing_table_without_prior_negative](crates/specforge/test_data/kg_quality/table_shape_prior_guided_timing_table_without_prior_negative/fixture.json).
- The pair proves a locally `unknown` `Parameter | Min | Max | Unit` table stays inert without prior memory and yields `timing_constraints = 1` across `EvidenceIR`, `SemanticIR`, and `IntentIR` only when a matching table-shape prior is staged.

### Why this matters
- It broadens the first table-shape prior family beyond signal-description recovery and shows that the same bounded consumer already generalizes to timing-table interpretation.

## 2026-04-07 (semantic modality-reliability priors landed as the fifth bounded learning slice)

### Added: first semantic modality-reliability prior family in `CorpusMemory`
- Extended [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) with typed `semantic_modality_reliability_priors` plus advisory lookup helpers that score how reliable a given semantic source kind has been for a given role and protocol family.
- Extended [learn_priors.rs](crates/specforge/src/commands/learn_priors.rs) so `specforge learn-priors` now harvests those priors from decisive, non-alias-dependent semantic consensus records rather than from raw guesses.
- The latest local AMBA run over APB/AHB/AXI still yields `0` semantic modality-reliability priors, which is the honest current state: the family is landed, but the real canonical artifacts are not yet surfacing enough promoted semantic consensus to populate it automatically.

### Added: bounded semantic arbitration consumption and benchmark coverage
- Extended [semantic.rs](crates/specforge/src/ir/semantic.rs) so `SemanticIR` can advisory-adjust local semantic arbitration using modality-reliability priors, but only when the current PDF already contains multiple locally grounded semantic candidates; the original conflict remains visible either way.
- Extended [validate.rs](crates/specforge/src/commands/validate.rs) so semantic and intent validation now report prior-guided semantic arbitration and prior-guided semantic consensus explicitly instead of hiding that path inside the canonical result.
- Added the tracked gold/negative fixture pair [semantic_modality_reliability_prior_guided_conflict_gold](crates/specforge/test_data/kg_quality/semantic_modality_reliability_prior_guided_conflict_gold/fixture.json) and [semantic_modality_reliability_prior_guided_conflict_without_prior_negative](crates/specforge/test_data/kg_quality/semantic_modality_reliability_prior_guided_conflict_without_prior_negative/fixture.json), which prove locally conflicted role evidence stays contested without the staged prior and becomes decisively resolved only when the matching modality-reliability prior is present.

### Why this matters
- This is the first learning slice that improves semantic arbitration itself instead of only widening local phrase, actor-vocabulary, timing, or table-shape interpretation.
- It stays within the project doctrine: the learning plane can guide which locally grounded evidence should carry more weight, but it still cannot author canonical facts that the current document did not expose.

### Validation
- `cargo test --manifest-path Cargo.toml modality_reliability_priors_can_resolve_local_semantic_conflicts -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality semantic_modality_reliability_prior_guided_conflict_gold semantic_modality_reliability_prior_guided_conflict_without_prior_negative` → passed
- `bash scripts/run_ci.sh` → passed (`207/207` tests)

## 2026-04-07 (table-shape priors landed as the fourth bounded learning slice)

### Added: first table-shape prior family in `CorpusMemory`
- Extended [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) with typed `table_shape_priors`, normalized structured-table header signatures, and advisory lookup helpers that can resolve a local table kind only when the signature matches uniquely.
- Extended [learn_priors.rs](crates/specforge/src/commands/learn_priors.rs) so `specforge learn-priors` now harvests table-shape priors from validated document chains by walking `IntentIR -> SemanticIR -> EvidenceIR -> SourceIR`.
- The latest local AMBA run over APB/AHB/AXI now yields `94` table-shape priors in addition to the existing actor-taxonomy and temporal families.

### Added: bounded table-shape prior consumption and benchmark coverage
- Extended [evidence.rs](crates/specforge/src/ir/evidence.rs) so `EvidenceIR` can advisory-recover a local table kind from prior memory, but only when the current table is still `unknown`; explicit local `SourceIR.table_kind` values remain authoritative.
- Added the tracked gold/negative fixture pair [table_shape_prior_guided_signal_table_gold](crates/specforge/test_data/kg_quality/table_shape_prior_guided_signal_table_gold/fixture.json) and [table_shape_prior_guided_signal_table_without_prior_negative](crates/specforge/test_data/kg_quality/table_shape_prior_guided_signal_table_without_prior_negative/fixture.json), which prove a locally `unknown` `Name | Direction | Width` table stays inert without prior memory and gains signal-description recovery only when a matching learned prior is staged.

### Why this matters
- This is the first cross-document learning slice that improves table interpretation directly, not just actor vocabulary or phrase interpretation.
- It stays fully within the project doctrine: prior memory widens local interpretation, but it does not rewrite `SourceIR` or override explicit local classifications.

## 2026-04-07 (Learning-plane growth model clarified)

### Added: explicit note on what grows to materialize learning
- Logged in [DEVELOPMENT_NOTES.md](DEVELOPMENT_NOTES.md) that the learning capability is defined in code, while the thing that actually grows over time is the typed prior store, typically [corpus_memory.json](generated/prior_memory/corpus_memory.json).
- Added the matching short entry-point note in [README.md](README.md), so future sessions do not confuse `R15f` with neural-network-style hidden-weight learning.

### Why this matters
- This makes the learning model explicit: `specforge` uses symbolic, inspectable, typed memory rather than opaque learned weights.
- It also clarifies the safety boundary between:
  - code that defines how learning works
  - data that stores what has been learned
  - canonical per-document IR that remains provenance-pure

## 2026-04-07 (Actor-taxonomy priors now recover structural KG from section headings)

### Added: prior-guided structural KG recovery for width-only section-guided signal tables
- Extended [evidence.rs](crates/specforge/src/ir/evidence.rs) so actor-taxonomy priors can now lift width-only `Signal | Width` tables under headings like `Issuer signals` or `Acceptor signals` into structural `ActorSignalRelation::Drives` edges, not just flat compatibility directions.
- Strengthened the tracked gold fixture [actor_taxonomy_prior_guided_section_direction_gold](crates/specforge/test_data/kg_quality/actor_taxonomy_prior_guided_section_direction_gold/fixture.json) so it now requires graph-backed recovery too: `actor_signal_relations = 3`, `actor_ports = 3`, and `with_graph_direction = 3`.

### Why this matters
- This closes an important quality gap in the first learning-plane consumer: prior-guided section headings now improve the canonical structural KG, not just the compatibility hint surface.
- It directly supports the graph-first roadmap because learned actor vocabulary can now produce actor-relative port structure from locally grounded width-only tables.

### Validation
- `cargo test --manifest-path Cargo.toml actor_taxonomy_priors_guide_section_heading_direction_inference -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality actor_taxonomy_prior_guided_section_direction_gold actor_taxonomy_prior_guided_section_direction_without_prior_negative` → passed
- `bash scripts/run_ci.sh` → passed (`204/204` tests)

## 2026-04-07 (KG fixtures now lock prior-guided visual semantic recovery)

### Added: prior-guided visual semantic gold/negative pair
- Added [visual_semantic_prior_guided_caption_gold](crates/specforge/test_data/kg_quality/visual_semantic_prior_guided_caption_gold/fixture.json), which proves the unseen local visual-caption phrase `XACK can sink the transfer` gains ready-like semantic recovery only when a matching `visual_caption` semantic prior is staged into the fixture.
- Added [visual_semantic_prior_guided_caption_without_prior_negative](crates/specforge/test_data/kg_quality/visual_semantic_prior_guided_caption_without_prior_negative/fixture.json), which locks the honest fallback behavior that the same local caption stays semantically unresolved when prior memory is absent.

### Why this matters
- This broadens the first benchmark surface into a second modality without inventing a new unsafe prior family prematurely.
- It proves the bounded semantic-prior doctrine is not prose-only: prior memory can widen interpretation of local visual-caption phrasing, but it still cannot manufacture a role when the matching prior is absent.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality visual_semantic_prior_guided_caption_gold` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality visual_semantic_prior_guided_caption_without_prior_negative` → passed
- `bash scripts/run_ci.sh` → passed (`204/204` tests)

## 2026-04-07 (KG fixtures now lock prior-guided actor-taxonomy recovery)

### Added: prior-guided actor-taxonomy gold/negative pair
- Added [actor_taxonomy_prior_guided_section_direction_gold](crates/specforge/test_data/kg_quality/actor_taxonomy_prior_guided_section_direction_gold/fixture.json), which proves width-only `Issuer signals` / `Acceptor signals` sections gain canonical signal directions only when matching actor-taxonomy priors are staged into the fixture.
- Added [actor_taxonomy_prior_guided_section_direction_without_prior_negative](crates/specforge/test_data/kg_quality/actor_taxonomy_prior_guided_section_direction_without_prior_negative/fixture.json), which locks the honest fallback behavior that the same local section headings stay directionless when prior memory is absent.

### Why this matters
- This completes the first benchmark triangle for the three initial bounded prior families: actor-taxonomy, semantic-role phrases, and temporal-language phrases.
- It proves the local-grounding doctrine for actor vocabulary too: prior memory can widen how the extractor interprets explicit local actor terms, but it cannot manufacture direction when the matching prior is absent.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality actor_taxonomy_prior_guided_section_direction_gold` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality actor_taxonomy_prior_guided_section_direction_without_prior_negative` → passed
- `bash scripts/run_ci.sh` → passed (`204/204` tests)

## 2026-04-07 (KG fixtures now lock prior-guided semantic recovery)

### Added: prior-guided unseen-phrase semantic gold/negative pair
- Added [semantic_prior_guided_phrase_gold](crates/specforge/test_data/kg_quality/semantic_prior_guided_phrase_gold/fixture.json), which proves the unseen local phrase `XACK can receive the transfer` gains ready-like semantic recovery only when a matching semantic prior is staged into the fixture.
- Added [semantic_prior_guided_phrase_without_prior_negative](crates/specforge/test_data/kg_quality/semantic_prior_guided_phrase_without_prior_negative/fixture.json), which locks the honest fallback behavior that the same local phrase stays semantically unresolved when prior memory is absent.

### Why this matters
- This is the second tracked benchmark proof that the cross-document learning plane can strengthen analysis of an unseen local phrase without leaking canonical facts across documents.
- It shows the same “local text required, prior only widens interpretation” contract now holds for both temporal-language priors and semantic-role priors.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality semantic_prior_guided_phrase_gold` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality semantic_prior_guided_phrase_without_prior_negative` → passed
- `bash scripts/run_ci.sh` → passed (`204/204` tests)

## 2026-04-07 (KG fixtures now lock prior-guided temporal recovery)

### Added: fixture-owned prior-memory patching in `specforge kg-bench`
- Extended [kg_bench.rs](crates/specforge/src/commands/kg_bench.rs) so tracked fixtures can now stage a local `CorpusMemory` before `EvidenceIR` is built.
- The new patch surface is generic across actor-taxonomy, semantic, and temporal priors, so later `R15f` benchmark slices can exercise more prior families without depending on a shared mutable prior file.

### Added: prior-guided unseen-phrase temporal gold/negative pair
- Added [temporal_prior_guided_cycle_window_gold](crates/specforge/test_data/kg_quality/temporal_prior_guided_cycle_window_gold/fixture.json), which proves the unseen local phrase `PREADY must be asserted one beat later` gains a one-cycle `cycle_window` only when a matching temporal prior is staged into the fixture.
- Added [temporal_prior_guided_cycle_window_without_prior_negative](crates/specforge/test_data/kg_quality/temporal_prior_guided_cycle_window_without_prior_negative/fixture.json), which locks the honest fallback behavior that the same local phrase still yields a temporal rule but no bounded `cycle_window` without prior memory.

### Why this matters
- This is the first tracked benchmark evidence that the cross-document learning plane can improve analysis of an unseen local phrase without leaking canonical facts across documents.
- It upgrades `R15f` from “consumers exist” to “consumers are benchmarked against honest before/after behavior.”

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality temporal_prior_guided_cycle_window_gold` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality temporal_prior_guided_cycle_window_without_prior_negative` → passed
- `bash scripts/run_ci.sh` → passed (`204/204` tests)

## 2026-04-06 (bounded temporal prior consumption landed)

### Added: prior-guided cycle-window recovery in `SemanticIR`
- Extended [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) with a typed temporal phrase lookup that can resolve a unique learned `CycleWindowRecord` from locally grounded timing text.
- Extended [semantic.rs](crates/specforge/src/ir/semantic.rs) so `SemanticIR` now loads advisory prior memory from the persisted upstream `prior_memory_path` and uses temporal phrase priors only as a fallback when direct cycle-window parsing cannot recover the local timing window.
- Added a direct semantic regression proving `PREADY must be asserted one beat later` still yields no built-in cycle window on its own, but does recover a one-cycle temporal rule when a validated temporal prior is present.

### Why this matters
- The cross-document learning plane now has a third real bounded consumer, and it lives in the temporal model instead of only in evidence extraction.
- This lets the extractor become stronger on previously unseen local timing phrase shapes without weakening the rule that canonical timing still has to be justified by the current PDF.
- The temporal prior path is still honest: without the local timing sentence there is no rule, and without a unique learned prior there is no learned cycle-window fallback.

### Validation
- `cargo test --manifest-path Cargo.toml derives_cycle_window_from_temporal_phrase_prior_when_builtin_parser_cannot -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml extracts_single_cycle_window_from_idiomatic_clock_tick_phrases -- --nocapture` → passed
- `bash scripts/run_ci.sh` → passed (`204/204` tests)

## 2026-04-06 (bounded semantic prior consumption landed)

### Added: prior-guided semantic hint recovery in `EvidenceIR`
- Extended [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) with shared semantic-phrase normalization and lookup helpers, so the same phrase-shape logic now powers both `learn-priors` and runtime prior consumption.
- Extended [evidence.rs](crates/specforge/src/ir/evidence.rs) so `EvidenceIR` can now use semantic phrase priors to recover local signal-role hints from non-hardcoded grounded phrases.
- `EvidenceIR` now also persists the consulted `prior_memory_path`, so later `refresh_signal_semantic_hints()` calls during NLP loopback keep the same advisory prior guidance instead of silently dropping it.

### Why this matters
- The cross-document learning plane now has a second real bounded consumer, beyond actor-taxonomy direction guidance.
- This lets the extractor become stronger on phrases it has learned from prior validated documents without breaking the local-grounding rule.
- The semantic prior path is still honest: without the local phrase, there is no semantic promotion; with the local phrase, the prior only helps interpret it.

### Validation
- `cargo test --manifest-path Cargo.toml semantic_phrase_priors_guide_local_semantic_hint_recovery -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml actor_taxonomy_priors_guide_source_column_direction_inference -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml learn_priors_harvests_semantic_and_temporal_priors -- --nocapture` → passed
- `bash scripts/run_ci.sh` → passed (`203/203` tests)

## 2026-04-06 (first bounded prior-consumption path landed)

### Added: advisory prior-guided direction recovery in `EvidenceIR`
- Extended [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) with reusable actor-taxonomy lookup helpers, normalized actor-term matching, and protocol-family inference so the learning plane can be queried safely during extraction.
- Extended [evidence.rs](crates/specforge/src/ir/evidence.rs) with `build_with_prior_memory(...)` plus the first bounded prior consumer:
  - section-heading direction inference can now use actor-taxonomy priors
  - `Source` / `Destination` column direction inference can now use actor-taxonomy priors
  - prior guidance still requires explicit local actor terms already present in the current document
- Extended [cli.rs](crates/specforge/src/cli.rs), [evidence.rs](crates/specforge/src/commands/evidence.rs), and [converge.rs](crates/specforge/src/commands/converge.rs) so `specforge evidence` and `specforge converge` now consult `--prior-memory generated/prior_memory/corpus_memory.json` by default.

### Why this matters
- The cross-document learning plane is no longer just storing priors; it now has its first real bounded consumer in the staged pipeline.
- This replaces another brittle hardcoded actor-vocabulary heuristic with typed reusable memory while preserving the project’s truthfulness rule: priors may guide local interpretation, but they must not author canonical facts on their own.
- It gives the extractor a safe path to improve on PDF `N+1` from validated experience on PDFs `1..N` without letting document pipelines contaminate one another.

### Validation
- `cargo test --manifest-path Cargo.toml actor_taxonomy_priors_guide_source_column_direction_inference -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml actor_taxonomy_priors_guide_section_heading_direction_inference -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml converge_defaults_to_ollama_for_vlm_and_nlp -- --nocapture` → passed
- `bash scripts/run_ci.sh` → passed (`202/202` tests)

## 2026-04-06 (typed prior memory now learns actor taxonomy too)

### Added: actor-taxonomy priors for `R15f`
- Extended [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) with a typed `actor_taxonomy_priors` family plus query helpers by protocol family and taxonomy role.
- Extended [learn_priors.rs](crates/specforge/src/commands/learn_priors.rs) so `specforge learn-priors` now harvests actor-taxonomy priors from:
  - decisive, non-alias-dependent actor-grounded handshake-role evidence
  - conservative self-identifying actor vocabulary such as `requester`, `completer`, `manager`, and `subordinate`

### Why this matters
- The cross-document learning plane can now accumulate reusable protocol-role vocabulary, not just timing language.
- This is the first prior family that directly teaches the extractor how actor terminology varies across specs while still keeping canonical per-document truth local and validated.
- The latest live AMBA prior-memory run is now materially richer: `16` actor-taxonomy priors and `222` temporal phrase priors, while semantic phrase priors remain honestly at `0`.

### Validation
- `cargo test --manifest-path Cargo.toml learn_priors -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- learn-priors generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json --output generated/prior_memory/corpus_memory.json` → passed (`16` actor-taxonomy priors, `222` temporal phrase priors)
- `bash scripts/run_ci.sh` → passed (`199/199` tests)

## 2026-04-06 (local and hosted CI now share one entrypoint)

### Added: checked-in local CI runner
- Added [run_ci.sh](scripts/run_ci.sh), a repository-local CI entrypoint that runs the canonical Rust quality gate from the repo root:
  - `cargo fmt --all --check`
  - `cargo test --manifest-path Cargo.toml`

### Changed: GitHub Actions now reuses the local runner
- Updated [.github/workflows/ci.yml](.github/workflows/ci.yml) so GitHub Actions calls `./scripts/run_ci.sh` instead of duplicating the commands inline.

### Why this matters
- The full Rust CI path can now be run locally before push, which makes CI breakage easier to catch on the developer machine instead of waiting for GitHub.
- Using one checked-in entrypoint removes local-versus-hosted drift and makes future CI expansion safer.

### Validation
- `./scripts/run_ci.sh` → passed
- `cargo fmt --all --check` → passed
- `cargo test --manifest-path Cargo.toml` → passed

## 2026-04-06 (GitHub Actions CI baseline established)

### Added: repo-hosted Rust CI on `push` / `pull_request`
- Added [.github/workflows/ci.yml](.github/workflows/ci.yml), a GitHub Actions workflow that installs Rust `1.89.0`, caches Cargo artifacts, and runs the same baseline Rust gate used locally:
  - `cargo fmt --all --check`
  - `cargo test --manifest-path Cargo.toml`

### Why this matters
- The project now has a real hosted validation path instead of relying only on local discipline before commits and pushes.
- The CI contract stays intentionally narrow and trustworthy by mirroring the commands already used as the canonical local gate.
- This also closes a repository-bootstrap gap: the new GitHub repo now validates Rust changes automatically on every push and pull request.

### Validation
- `cargo fmt --all --check` → passed
- `cargo test --manifest-path Cargo.toml` → passed

## 2026-04-06 (first typed cross-document prior store landed)

### Added: `specforge learn-priors <intent_ir>...`
- Added a new CLI command that builds the first local typed `CorpusMemory` prior store under `generated/prior_memory/corpus_memory.json`.
- The command only learns from validated `IntentIR` artifacts and skips artifacts whose latest validation report carries error findings, so the new learning plane stays downstream of validation instead of becoming a shortcut around it.

### Added: first typed prior families for `R15f`
- Added `crates/specforge/src/ir/prior_memory.rs` with a typed `CorpusMemory` schema, explicit update-policy record, protocol-family scoping, and advisory query helpers.
- The first semantic prior family learns reusable semantic-role phrases only from decisive, non-alias-dependent canonical semantic consensus plus preserved observation text.
- The first temporal prior family learns reusable timing/constraint language from canonical `temporal_rules` plus validated canonical `signal_constraints` / `conditional_rules`, which makes the learning plane immediately useful even while real-document temporal-rule lift remains conservative.

### Why this matters
- This is the first real implementation of the separate cross-document learning plane captured in the roadmap and development notes.
- It keeps the document plane provenance-pure while finally giving the extractor a place to accumulate reusable knowledge about how chip specifications express meaning.
- The first live AMBA run is already informative: AXI/APB/AHB `IntentIR` artifacts currently yield `222` temporal phrase priors and `0` semantic phrase priors, which is exactly the kind of honest signal the project needs while the canonical semantic-consensus surface on real PDFs is still strengthening.

### Validation
- `cargo test --manifest-path Cargo.toml learn_priors -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- learn-priors generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json --output generated/prior_memory/corpus_memory.json` → passed (`222` temporal phrase priors)

## 2026-04-06 (KG fixtures now lock AHB wait-state timing recovery)

### Added: AHB-style wait-state timing gold fixture
- Added a tracked staged fixture proving that AHB-style `Manager signals` / `Subordinate signals` section-heading context, `Destination`-column signal tables, and explicit actor relations can recover wait-state timing semantics in addition to the earlier section-heading direction path.
- The fixture locks `HREADY must be asserted on the next cycle when HSEL is HIGH` together with waited-transfer hold rules on `HTRANS` and `HADDR`, and expects canonical actor-relative ports, one bounded `cycle_window`, multi-predicate temporal guards, actor-grounded temporal predicates, and no false handshake completion to survive through both `SemanticIR` and `IntentIR`.

### Why this matters
- The earlier AHB gold fixture proved that family-specific section headings can recover truthful per-signal direction. This follow-on slice proves the same AHB evidence path can also recover real wait-state timing without falling back to generic protocol heuristics.
- It closes an important family gap between “AHB direction works” and “AHB wait-state timing works,” which is necessary if the KG benchmark suite is going to be honest about protocol behavior rather than only port orientation.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality ahb_wait_state_timing_gold` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all --check` → passed
- `cargo test --manifest-path Cargo.toml` → passed

## 2026-04-06 (KG fixtures now lock APB setup/access timing recovery)

### Added: APB-style setup/access timing gold fixture
- Added a tracked staged fixture proving that APB-style `Signal | Source | Width | Description` tables plus guarded constraints can recover setup/access timing semantics in addition to the earlier requester/completer handshake path.
- The fixture locks `PENABLE must be asserted on the next cycle when PSEL is HIGH` together with wait-state and completion hold rules on `PADDR`, and expects canonical actor-relative ports, one bounded `cycle_window`, multi-predicate temporal guards, actor-grounded temporal predicates, and typed handshake completion to survive through both `SemanticIR` and `IntentIR`.

### Why this matters
- The earlier APB gold fixture proved that requester/completer roles plus one guarded constraint can recover handshake completion. This follow-on slice proves the same APB vocabulary can also recover setup-to-access timing and wait-state stability without losing actor grounding or collapsing guarded temporal structure.
- It closes an important protocol-family gap between “APB handshake meaning works” and “APB access timing works,” which is necessary if the KG benchmark suite is going to be honest about behavioral protocol semantics rather than just request/accept roles.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality apb_setup_access_timing_gold` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all --check` → passed
- `cargo test --manifest-path Cargo.toml` → passed

## 2026-04-06 (KG fixtures now lock AXI next-cycle timing recovery)

### Added: AXI-style next-cycle timing gold fixture
- Added a tracked staged fixture proving that AXI-style width-only channel tables plus prose `Manager` / `Subordinate` drive-sample relations can recover next-cycle timing semantics in addition to direction and signal inventory.
- The fixture locks `AWREADY must be asserted on the next cycle` together with `AWADDR must not change when AWVALID is HIGH and AWREADY is HIGH`, and expects canonical actor-relative ports, one bounded `cycle_window`, actor-grounded temporal predicates, and typed handshake completion to survive through both `SemanticIR` and `IntentIR`.

### Why this matters
- The first AXI gold fixture proved that width-only tables plus prose can recover truthful actor-relative direction. This follow-on slice proves the same family of evidence can also recover temporal meaning instead of stopping at static ports.
- It closes an important roadmap gap between “AXI direction works” and “AXI timing works,” which is necessary if the KG benchmark suite is going to be honest about protocol semantics rather than only signal inventory.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_next_cycle_timing_gold` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → passed

## 2026-04-06 (KG fixtures now lock AXI width-only prose-direction recovery)

### Added: AXI-style width-only plus prose-direction gold fixture
- Added a tracked staged fixture proving that AXI-style `Name | Width | Description` signal tables still recover truthful actor-relative ports when prose drive/sample relations provide the missing directionality.
- The fixture locks AXI write-address-channel recovery end-to-end: table-grounded widths for `AWVALID`, `AWREADY`, and `AWADDR`; prose-grounded `Manager` / `Subordinate` `Drives` and `Reads` relations; request/accept semantic grounding; and typed handshake completion from one guarded `AWADDR must not change when AWVALID is HIGH and AWREADY is HIGH` constraint.

### Fixed: width-only synthesized declarations now survive into canonical signal records
- Widened the semantic explicit-signal parser so synthesized statements like `Signal AWVALID is width 1.` are treated as real interface-signal declarations even without an immediate `input` / `output` token.
- This closes the AXI-family gap where width-only channel tables previously stopped at actor relations and connectivity instead of becoming canonical `SemanticIR` / `IntentIR` signal records.

### Why this matters
- AXI-family specs are a real stress case because the signal tables often omit direction columns entirely. If that mixed table-plus-prose recovery path regresses, the generic AMBA/APB/AHB fixture suite can still look healthy while AXI truth quietly drifts.
- This turns that family-specific extraction pattern into executable benchmark coverage instead of leaving it protected only by aggregate PDF scores.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_width_only_prose_direction_gold` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → passed

## 2026-04-06 (KG fixtures now lock APB Requester/Completer semantics)

### Added: APB-style `Requester` / `Completer` gold fixture
- Added a tracked staged fixture proving that APB-style `Source`-column signal tables recover `Requester` / `Completer` actor roles canonically instead of only being covered indirectly by broader AMBA fixtures.
- The fixture locks `(Requester, drives, PSEL)` and `(Completer, drives, PREADY)`, the corresponding actor-relative output ports, table-grounded request/accept semantics, and a typed handshake-completion temporal rule from one guarded `PADDR must not change when PSEL is HIGH and PREADY is HIGH` constraint.

### Why this matters
- APB-family specs use `Completer` as real protocol vocabulary. If that family-specific role word regresses, the broader AMBA fixture set can still look healthy while APB truth quietly degrades.
- This turns APB-specific table vocabulary into executable benchmark coverage rather than assuming it is already protected by the generic `Requester` / `Subordinate` path.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality apb_requester_completer_handshake_gold` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → passed

## 2026-04-06 (KG fixtures now lock AHB section-heading direction recovery)

### Added: AHB-style section-heading gold fixture
- Added a tracked staged fixture proving that `Manager signals` / `Subordinate signals` section context recovers per-signal direction and width correctly for AHB-style signal tables.
- The fixture locks canonical direction on `HADDR`, `HWRITE`, `HTRANS`, `HREADYOUT`, and `HRESP` through both `SemanticIR` and `IntentIR`.

### Improved: `kg-bench` can now patch `document_sections` and assert per-signal direction directly
- Added fixture support for patching `SourceIR.document_sections`, so section-heading-driven extraction paths are benchmarkable without needing heavyweight source documents.
- Added canonical per-signal direction expectations, so tracked fixtures can lock actual signal direction instead of inferring it through aggregate validation metrics.

### Why this matters
- AHB extraction quality genuinely depends on section-heading context in some real specs. If that path regresses, the pipeline can still look healthy at a coarse metric level while silently losing signal truth.
- This turns that protocol-family-specific path into executable benchmark coverage instead of leaving it as a fragile unit-test-only behavior.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality ahb_section_heading_direction_gold` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 192/192 passed

## 2026-04-06 (KG fixtures now lock AMBA destination-column receiver semantics)

### Added: AMBA-style `Destination`-column gold fixture
- Added a tracked staged fixture proving that AMBA-style `Destination` signal-description tables recover consumer-side actor-signal relations canonically as `Reads`.
- The fixture locks that `Subordinate` reads `XREQ` and `Requester` reads `XRESP`, and that those same relations survive downstream as actor-relative input ports in both `SemanticIR` and `IntentIR`.

### Improved: `kg-bench` can now assert canonical actor-signal relations directly
- Added canonical fixture expectations for actor-signal relations, so tracked truthfulness checks can lock `Drives` versus `Reads` semantics directly.
- This makes protocol-grade relation benchmarks stronger than relying only on actor-port projections or relation counts.

### Why this matters
- `Destination`-oriented AMBA tables carry receiver semantics, not producer semantics. If the KG flattens those into output-side relations, downstream truthfulness quietly drifts.
- Locking the relation itself, not just derived port shape, makes the benchmark harness more faithful to the graph-first architecture.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality amba_destination_column_reads_gold` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 192/192 passed

## 2026-04-06 (KG fixtures now lock spurious timing-annotation rejection)

### Added: spurious timing-annotation negative fixture
- Added a tracked staged fixture proving that low-value VLM timing-diagram labels like `T0`, `Addr 1`, and `Cycle 2` remain visible as timing-diagram extraction at the evidence stage but do not synthesize canonical timing constraints or temporal rules downstream.
- The fixture locks `timing_diagram_extractions = 1` together with `timing_constraints = 0` and `temporal_rules = 0`, so the pipeline keeps the observation without overclaiming semantics.

### Fixed: semantic timing lift now rejects label-only waveform noise
- Added a narrow semantic-stage filter so label-only VLM timing annotations are treated as waveform labels instead of timing semantics.
- This keeps the `EvidenceIR` timing observation honest while preventing `SemanticIR` / `IntentIR` from fabricating timing meaning from low-value annotation fragments alone.

### Why this matters
- Chip-spec timing diagrams often contain a mix of true behavioral annotations and low-value figure labels. The KG should learn from the former without hallucinating meaning from the latter.
- This closes a real false-positive path in the multimodal timing lift and makes that truthfulness boundary executable in the tracked benchmark suite.

### Validation
- `cargo test --manifest-path Cargo.toml vlm_timing_diagram_observation_rejects_label_only_noise -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_spurious_annotation_negative` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 192/192 passed

## 2026-04-06 (KG fixtures now lock field-table misclassification rejection)

### Added: field-table misclassification negative fixture
- Added a tracked staged fixture proving that a misclassified `Bits | Name | Description` table does not synthesize fake top-level signals or semantic roles from field names like `REQ` and `ACK`.
- The fixture locks that only the real declared top-level signal survives in the canonical semantic/intent surface, while evidence-stage semantic hints stay at zero.

### Fixed: table-driven top-level signal synthesis now rejects field-like layouts
- Added a shared table-level sanity gate so field-like `Bits | Name | Description` layouts and `... signal fields` captions are filtered before they can generate fake signal declarations, semantic hints, or related top-level table-derived facts.
- This guard now protects the table-driven name/semantic paths together instead of relying on one-off downstream cleanup.

### Why this matters
- This closes another KG false-positive path: register-field tables and bit-field tables often contain uppercase names that look like signals, but they are not top-level interface ports.
- It also makes table misclassification a benchmarked truthfulness property instead of an implicit hope in the upstream classifier.

### Validation
- `cargo test --manifest-path Cargo.toml misclassified_field_table_does_not_synthesize_fake_signal_semantics -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality table_misclassification_field_table_negative` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 191/191 passed

## 2026-04-06 (KG fixtures now lock bogus source-column actor rejection)

### Added: bogus actor-attribution negative fixture
- Added a tracked staged fixture proving that AMBA-style `Source`-column infrastructure rows like `Clock` and `Reset` do not become protocol actors in the KG.
- The fixture locks that only the true `Requester` / `Subordinate` rows survive as actor-signal relations and actor-relative ports while table-grounded semantic request/accept meaning still remains recoverable.

### Fixed: table relation extraction now distinguishes source vs destination semantics
- `Source` / `Driver` columns now yield `Drives` relations.
- `Destination` columns now yield `Reads` relations.
- Direction and infrastructure placeholders like `input`, `Clock`, and `Reset` are filtered instead of being promoted into fake actor names.

### Why this matters
- This closes a real KG false-positive path: chip-spec signal tables often mix protocol rows with clock/reset/infrastructure rows, and the graph should not silently invent actors from those metadata labels.
- It also hardens a subtle semantics boundary: destination-oriented tables are receiver facts, not disguised driver facts.

### Validation
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 190/190 passed

## 2026-04-06 (KG fixtures now lock multimodal conflict behavior too)

### Added: cross-modality semantic-conflict negative fixture
- Added a tracked staged fixture proving that table evidence and visual-caption evidence can disagree about the same signal role without collapsing into false cross-modality consensus.
- The fixture locks that `XCTRL`:
  - carries a semantic conflict
  - keeps multiple candidates and non-decisive arbitration
  - does not gain a resolved role or consensus

### Added: validation-metric expectations for contested multimodal grounding
- The new fixture also locks an important validator nuance:
  - `with_visual_semantic_grounding` stays non-zero because visual evidence is still present
  - `with_cross_modality_semantic_grounding` stays zero because the multimodal evidence never resolved into consensus

### Why this matters
- This protects against a subtle multimodal failure mode: “two modalities spoke” must not be mistaken for “two modalities agreed.”
- The benchmark harness now guards both the positive and negative sides of multimodal grounding quality.

### Validation
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 188/188 passed

## 2026-04-06 (KG fixtures now lock cross-modality grounding metrics)

### Added: validation-metric expectations in `specforge kg-bench`
- KG fixtures can now assert persisted validation metric values directly, not only finding ids.
- This lets the tracked harness lock quantitative truthfulness surfaces like:
  - `with_cross_modality_semantic_grounding`
  - `with_visual_semantic_grounding`
  - decisive semantic-arbitration counts

### Added: visual-asset patching in `SourceIR` fixtures
- `specforge kg-bench` fixtures can now patch `SourceIR.visual_assets` directly in addition to structured tables.
- That makes tracked multimodal regressions practical without needing a heavyweight external PDF for each case.

### Added: cross-modality semantic-grounding gold fixture
- Added a tracked staged fixture proving that `XREQ` can become valid-like through joint signal-description-table evidence plus visual-caption evidence.
- The fixture locks both canonical outcomes and validator metrics, proving the resulting role is:
  - resolved
  - decisive
  - cross-modally grounded
  - visually grounded

### Why this matters
- This extends `R15e` from canonical-shape assertions into quantitative grounding checks.
- The benchmark harness can now protect multimodal semantic-strength behavior directly instead of leaving it to crate-local unit tests or manual inspection.

### Validation
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 188/188 passed

## 2026-04-06 (cross-document learning plane captured in steering docs)

### Added: explicit roadmap target for cross-document extractor learning
- Logged the architecture for a separate cross-document learning plane that can improve extraction on PDF `N+1` using reusable priors learned from PDFs `1..N`.
- Made the safety boundary explicit:
  - per-document `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR` truth stays local and provenance-pure
  - cross-document memory learns extraction priors, not undocumented facts

### Added: detailed engineering note for prior-guided extraction
- Captured the full doctrine in `DEVELOPMENT_NOTES.md`, including:
  - the two-plane architecture
  - examples of good priors versus bad fact leakage
  - candidate memory shapes like `CorpusMemory`, `PriorGraph`, and `ExperienceIR`
  - the retrieval / grounding / validation / feedback loop
  - the rule that only validated/promoted outcomes should feed the learning plane

### Why this matters
- This is the clean path to making the extractor progressively more expert across many chip-spec PDFs without breaking the truthfulness contract of the canonical IR.
- It steers future implementation toward learning reusable extraction intelligence rather than contaminating document-local truth.

## 2026-04-06 (KG fixtures now lock semantic arbitration state directly)

### Added: canonical semantic-arbitration expectations in `specforge kg-bench`
- KG fixtures can now assert:
  - signals with any semantic candidates
  - signals with multiple semantic candidates
  - signals carrying semantic arbitration
  - signals with decisive semantic arbitration
  - signals with non-decisive semantic arbitration

### Added: direct arbitration checks to the staged handshake fixtures
- The contested handshake fallback fixture now proves that `XVALID` stays canonically contested while `XACK` stays decisively grounded.
- The alias-dependent handshake caveat fixture now proves that `XREQ` and `XACK` stay decisively grounded even though their accepted handshake meaning remains explicitly caveated as alias-dependent.

### Why this matters
- This upgrades `R15e` from benchmarking arbitration side effects to benchmarking arbitration state directly.
- The harness now checks the canonical truth model itself instead of inferring arbitration quality only from blocked heuristic fallback, residual packets, or validator findings.

### Validation
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 188/188 passed

## 2026-04-06 (KG fixtures now cover alias-dependent semantic caveats)

### Added: richer canonical expectations in `specforge kg-bench`
- KG fixtures can now assert:
  - alias-dependent semantic consensus by signal
  - alias-dependent semantic candidates by signal
  - alias-dependent handshake-completion counts
- `EvidenceIR` fixture patches can now seed alias maps and refresh semantic hints before downstream stages run.

### Added: alias-dependent handshake-completion caveat fixture
- Added a stage-patched tracked fixture proving that alias-grounded handshake recovery remains canonical only when its weaker grounding stays explicit through:
  - semantic residual decisions
  - intent assumptions
  - validator findings

### Why this matters
- This extends `R15e` from benchmarking only hard rejection cases to also benchmarking “useful but caveated” semantic recovery.
- The harness now protects both sides of the truthfulness contract:
  - unsafe heuristic promotion must stay blocked
  - weak but acceptable semantics must retain their caveat trail

### Validation
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 188/188 passed

## 2026-04-06 (KG fixtures can now patch staged inputs)

### Added: stage-patched KG fixtures
- `specforge kg-bench` fixtures can now patch `SourceIR` and `EvidenceIR` inputs directly before downstream stages run.
- This lets the tracked benchmark harness express richer structured/semantic conditions than plain markdown prose alone.

### Added: contested handshake-name fallback negative fixture
- Added a stage-patched tracked fixture proving that contested meaning for a handshake-shaped signal like `XVALID` blocks typed `HandshakeComplete` recovery.
- The fixture injects:
  - a structured signal-description table
  - a typed signal constraint guard
  - validation expectations showing the blocked fallback and preserved semantic-role conflict

### Why this matters
- This strengthens `R15e` from “benchmark simple markdown cases” into “benchmark real staged semantics.”
- The harness now protects a high-value truthfulness invariant:
  - contested semantic evidence must outrank handshake-name heuristics

### Validation
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 188/188 passed

## 2026-04-06 (KG benchmark now includes a first AMBA-style gold fixture)

### Added: representative AMBA-style handshake gold fixture
- Added `crates/specforge/test_data/kg_quality/amba_source_column_handshake_gold/`.
- The fixture patches an AMBA-style `Signal | Source | Width | Description` table with `Requester` / `Subordinate` source roles and one guarded `PAYLOAD must not change when XREQ is HIGH and XACK is HIGH` constraint.
- It proves:
  - `EvidenceIR` recovers `actor_signal_relations = 2`
  - `SemanticIR` / `IntentIR` recover driver-side actor ports for `Requester -> XREQ` and `Subordinate -> XACK`
  - table-grounded semantic role consensus survives for both handshake signals
  - the guarded constraint lifts into one typed temporal rule with `HandshakeComplete`

### Why this matters
- This is the first tracked benchmark step from seed synthetic truthfulness checks toward representative APB/AHB/AXI-style gold coverage.
- It locks an important real-doc pattern: AMBA-style `Source` columns can now be benchmarked end-to-end instead of only being covered by ad hoc unit tests.

## 2026-04-06 (KG benchmark now locks caption-vs-VLM visual semantic conflicts)

### Added: visual-source semantic-conflict negative fixture
- Added `crates/specforge/test_data/kg_quality/visual_sources_semantic_conflict_negative/`.
- The fixture patches one timing-diagram visual asset with:
  - a caption that implies valid-like meaning
  - a `vlm_timing_diagram_extraction` note that implies ready-like meaning
- It proves:
  - `EvidenceIR` reports one visual-caption semantic hint and one VLM timing-annotation semantic hint
  - downstream `SemanticIR` / `IntentIR` preserve a semantic conflict, multiple candidates, and non-decisive arbitration
  - `with_visual_semantic_grounding = 1` while `with_multi_source_semantic_grounding = 0`

### Why this matters
- The benchmark harness now locks an important same-asset arbitration nuance: two conflicting visual sub-sources must stay visibly grounded without being overpromoted into same-modality consensus.

## 2026-04-06 (KG benchmark now locks VLM-note semantic-noise rejection)

### Added: direct VLM timing-note semantic-noise negative fixture
- Added `crates/specforge/test_data/kg_quality/vlm_timing_name_only_semantic_noise_negative/`.
- The fixture patches a timing-diagram `VisualAsset.note` with `vlm_timing_diagram_extraction` content that only describes waveform motion around `XVALID`.
- It proves:
  - `EvidenceIR` reports `timing_diagram_extractions = 1`
  - `EvidenceIR` reports `signal_semantic_hints_from_vlm_timing_annotations = 0`
  - downstream `SemanticIR` / `IntentIR` keep semantic-role candidates, arbitration, and consensus at zero

### Why this matters
- The benchmark harness now locks both sides of direct VLM-note truthfulness:
  - real timing extraction should survive
  - semantic-role meaning must not leak from handshake-shaped signal spelling alone

## 2026-04-06 (KG benchmark now locks direct VLM-note semantics)

### Added: evidence-stage validation expectations in `specforge kg-bench`
- `specforge kg-bench` fixtures can now assert persisted validation metrics at the `EvidenceIR` stage, not only at `SemanticIR` and `IntentIR`.
- This lets tracked regressions prove exactly where a semantic hint came from when downstream visual-grounding metrics would be too coarse.

### Added: direct VLM timing-note semantic-grounding gold fixture
- Added `crates/specforge/test_data/kg_quality/vlm_timing_semantic_grounding_gold/`.
- The fixture patches a timing-diagram `VisualAsset.note` with `vlm_timing_diagram_extraction` content and proves:
  - `EvidenceIR` reports `signal_semantic_hints_from_vlm_timing_annotations = 1`
  - `EvidenceIR` reports `signal_semantic_hints_from_visual_captions = 0`
  - downstream `SemanticIR` / `IntentIR` still resolve the signal meaning with visual grounding

### Why this matters
- The benchmark harness now locks direct image-note-derived meaning explicitly instead of only checking downstream visual-grounding side effects.
- This closes an important quality gap in `R15e`: tracked fixture coverage now reaches caption grounding, caption-versus-table arbitration, and direct VLM timing-note grounding.

## 2026-04-06 (tracked KG-quality benchmark harness landed)

### Added: `specforge kg-bench` command
- Added a new `specforge kg-bench` CLI command that runs tracked KG-quality fixtures through the staged `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR` pipeline.
- The command can assert canonical IR expectations and persisted validation findings, and it fails the run when any gold or negative fixture drifts.

### Added: first tracked KG-quality fixture pack
- Added tracked fixtures under `crates/specforge/test_data/kg_quality/` for:
  - actor-relative port recovery
  - rejection of name-only semantic role noise
  - multi-producer structural conflict surfacing through validation
  - actor-boundary residual quality

### Why this matters
- This starts `R15e` as a real executable benchmark surface instead of leaving KG-quality evaluation as roadmap text or scalar scores alone.
- The benchmark harness now protects:
  - canonical graph truthfulness
  - false-positive control
  - conflict surfacing
  - residual-quality honesty

### Validation
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 188/188 passed

## 2026-04-06 (polarity conflicts now survive into canonical IR)

### Added: canonical carry-through for polarity disagreement
- `SemanticIR` now carries `signal_polarity_conflicts` forward from `EvidenceIR`.
- `IntentIR` now carries the same polarity-conflict surface forward from `SemanticIR`.

### Added: semantic and intent validation for polarity conflicts
- `specforge validate` now reports `signal_polarity_conflicts` for both `SemanticIR` and `IntentIR`, not only for `EvidenceIR`.
- Contradictory active-high/active-low evidence now stays visible all the way to the canonical artifacts instead of disappearing after the evidence stage.

### Added: regression coverage for polarity-conflict carry-through
- Added semantic-stage and intent-stage carry-through regressions for `signal_polarity_conflicts`.
- Added validator regressions proving polarity conflicts are flagged at both canonical stages.

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 186/186 passed

## 2026-04-05 (alias-dependent handshake completion is now canonical residual state)

### Added: canonical residual and assumption carry-through for alias-dependent handshake semantics
- `SemanticIR` now emits `semantic_alias_dependent_handshake_completion` when typed `HandshakeComplete` predicates still depend on alias-grounded semantic role consensus.
- `IntentIR` now carries that caution forward as `assumption_alias_dependent_handshake_completion`, so the weaker grounding remains inspectable even before validation runs.

### Why this matters
- Validator findings are useful, but SOTA-quality continuity needs the canonical artifacts themselves to preserve important semantic caveats.
- Alias-grounded transfer-progress structure remains usable, while still being marked as weaker than directly grounded or corroborated handshake semantics.

### Added: regression coverage for canonical alias-dependent handshake caveats
- Added semantic-stage coverage proving alias-grounded handshake completion emits a residual packet.
- Added intent-stage coverage proving that residual packet becomes an explicit canonical assumption.

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 182/182 passed

## 2026-04-05 (alias-dependent handshake completion is now explicit in validation)

### Added: validation visibility for alias-dependent temporal handshake semantics
- `specforge validate` now reports:
  - `temporal_rules_with_alias_dependent_handshake_completion`
- It also emits an explicit finding when typed `HandshakeComplete` predicates depend on alias-dependent semantic role consensus.

### Why this matters
- Alias-grounded handshake recovery remains useful, but it no longer looks as grounded as directly supported handshake semantics.
- The temporal layer now makes that weaker grounding visible instead of blending it into the generic handshake-completion count.

### Added: regression coverage for alias-dependent handshake validation
- Added a validation regression proving that alias-grounded semantic role consensus feeding a typed handshake predicate is reported explicitly.

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 181/181 passed

## 2026-04-05 (alias-dependent semantic role meaning is now explicit)

### Added: explicit alias-dependence on semantic candidates and consensus summaries
- `SemanticIR` and `IntentIR` now mark semantic role candidates and consensus summaries as `alias_dependent` when the current meaning still depends only on alias-grounded evidence.
- This keeps alias-grounded meaning usable while making that dependency explicit in the canonical IR instead of hiding it inside source-kind lists.

### Added: validation reporting for alias-dependent resolved roles
- `specforge validate` now reports:
  - `with_alias_dependent_semantic_consensus`
  - `alias_dependent_semantic_candidates`
- It also emits an explicit finding when resolved semantic roles still depend only on alias-grounded evidence.

### Added: regression coverage for alias-dependent canonical role visibility
- Added assertions proving:
  - alias-grounded semantic consensus is marked `alias_dependent`
  - direct visual/table grounded semantic consensus is not marked `alias_dependent`
  - intent validation reports alias-dependent resolved roles explicitly

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 180/180 passed

## 2026-04-05 (explicit signal names now outrank aliases for semantic grounding)

### Changed: alias-grounded semantic hints no longer double-count when direct signal names are present
- `EvidenceIR` now suppresses alias-grounded targeting for a signal when the same prose sentence or visual caption already contains an explicit mention of that signal.
- This means aliases stay a rescue path for implicit references, not an extra vote when the document is already explicit.

### Why this matters
- A sentence like `The request phase XREQ indicates that address and control information are valid for transfer.` now produces exactly one semantic hint for `XREQ`, grounded as direct prose rather than both direct and alias-grounded support.
- That keeps semantic-role arbitration honest and prevents artificial support inflation.

### Added: regression coverage for direct-name precedence over aliases
- Added an evidence-stage regression proving that explicit signal mentions outrank alias-grounding for the same statement.

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 179/179 passed

## 2026-04-05 (multi-signal prose and captions now ground per-signal semantic roles)

### Changed: semantic-role hint extraction now decomposes multi-signal text into per-signal context windows
- `EvidenceIR` no longer requires a whole prose statement or visual caption to resolve to exactly one signal before it can contribute a semantic role hint.
- When multiple known signals appear in the same sentence or caption, the extractor now carves out clause-local context windows around each signal mention and infers role meaning from that local description.

### Why this matters
- Text like `XVALID indicates request pending and XREADY indicates the subordinate can accept the transfer` can now produce:
  - a valid-like hint for `XVALID`
  - a ready-like hint for `XREADY`
- The previous weaker behavior either dropped that region entirely or would have required unsafe whole-text attribution.

### Added: regression coverage for multi-signal prose and caption grounding
- Added evidence-stage regressions proving that:
  - one prose sentence can contribute different semantic role hints to different signals
  - one visual caption can contribute different semantic role hints to different signals

### Cleanup
- removed the dead single-target semantic-role helper after the stronger per-signal path replaced it

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 178/178 passed

## 2026-04-05 (signal names no longer self-justify semantic role hints)

### Changed: semantic-role hint inference now strips signal identifiers from prose/visual text
- `EvidenceIR` now removes explicit signal tokens before semantic-role tag inference on prose descriptions, alias-grounded prose, visual captions, VLM timing annotations, and signal-description row text.
- This means identifiers like `AWVALID` / `AWREADY` no longer create valid-like or ready-like consensus by themselves.

### Added: regression coverage for declaration-only handshake-shaped names
- Added an evidence-stage regression proving that plain declarations such as `Signal AWVALID is input width 1.` do not create semantic handshake hints without descriptive language.

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 176/176 passed

## 2026-04-05 (provisional semantic roles no longer drive typed handshake recovery)

### Changed: handshake-role recovery now requires observation-backed consensus
- `SemanticIR` no longer lets fallback-only resolved semantic roles populate the canonical handshake-role context by themselves.
- Typed handshake-role recovery now trusts observation-backed `semantic_consensus` instead of any resolved role value that still lacks preserved grounding.

### Changed: provisional fallback-only role state now blocks handshake name fallback too
- Handshake-shaped signals with fallback-only provisional roles now block literal `VALID` / `READY` name fallback, not only signals with contested semantic arbitration.
- `specforge validate` now reports those blocked fallback cases under the existing handshake-fallback metric/finding surface.

### Added: regression coverage for provisional-role handshake blocking
- Added assertions proving:
  - provisional fallback-only semantic roles do not populate canonical handshake-role context
  - handshake-shaped provisional-role signals are reported as blocked name-fallback cases in validation

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 175/175 passed

## 2026-04-05 (fallback-only semantic roles are now explicit provisional state)

### Added: residual surfacing for resolved semantic roles without consensus
- `SemanticIR` now emits a `semantic_resolved_role_without_consensus` residual decision when a signal still carries a resolved semantic role but lacks preserved observation-backed consensus.

### Added: explicit IntentIR assumption for provisional semantic meaning
- `IntentIR` now turns that carried residual into `assumption_semantic_role_without_consensus` so provisional role meaning stays visible in the canonical artifact.

### Added: regression coverage for provisional semantic-role visibility
- Added assertions proving:
  - the semantic residual packet is emitted for fallback-only resolved roles
  - the intent stage carries that packet forward
  - `IntentIR` emits the matching provisional-role assumption

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 173/173 passed

## 2026-04-05 (blocked handshake fallback is now explicit residual state)

### Added: residual surfacing for intentionally blocked handshake promotion
- `SemanticIR` now emits a `semantic_handshake_name_fallback_blocked` residual decision when a signal looks handshake-shaped by name but preserved semantic arbitration is still contested.
- `IntentIR` carries that residual packet forward unchanged.

### Added: validation visibility for blocked handshake fallback
- `specforge validate` now reports:
  - `with_blocked_handshake_name_fallback`
- It also emits explicit semantic and intent findings when handshake-shaped signals intentionally block literal `VALID` / `READY` fallback.

### Added: regression coverage for blocked fallback visibility
- Added assertions proving:
  - the semantic residual packet is emitted
  - the intent stage carries it forward
  - semantic and intent validation both report the blocked-fallback state

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 171/171 passed


## 2026-04-05 (contested semantic evidence outranks handshake-name heuristics)

### Changed: temporal handshake derivation now respects contested semantic arbitration
- `SemanticIR` now builds a richer handshake-role context instead of relying only on a resolved-role map plus raw signal-name fallback.
- Signals with non-decisive `semantic_arbitration` now block literal `VALID` / `READY` name fallback during typed `HandshakeComplete` derivation.

### Why this matters
- A signal like `XVALID` can now stay honestly unresolved when preserved evidence disagrees about whether it is valid-like or ready-like.
- Literal spelling no longer overrides explicit contested semantic evidence in the temporal layer.

### Added: regression coverage for blocked handshake-name fallback
- Added a semantic regression proving that contested role evidence suppresses typed `HandshakeComplete` derivation even when the signal name looks handshake-shaped.

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 168/168 passed


## 2026-04-05 (canonical semantic arbitration summaries)

### Added: explicit semantic arbitration summaries on canonical interface signals
- `SemanticIR` now carries `semantic_arbitration` on `InterfaceSignalRecord`.
- `IntentIR` carries that same semantic-arbitration surface forward unchanged.

### Added: lead-vs-runner-up visibility without unsafe role forcing
- Each arbitration summary currently records:
  - candidate count
  - leading role
  - leading evidence weight
  - runner-up role and evidence weight when present
  - lead margin over the runner-up
  - decisive vs non-decisive status
- Multiple candidates still do not force a resolved semantic role; the arbitration surface is preserved for inspection while the canonical winner remains `None`.

### Added: validation reporting for decisive vs contested semantic roles
- `specforge validate` now reports:
  - `with_semantic_arbitration`
  - `with_decisive_semantic_arbitration`
  - `with_non_decisive_semantic_arbitration`
- It also emits an explicit finding when canonical semantic-role arbitration remains non-decisive.

### Added: regression coverage for semantic arbitration summaries
- Added assertions for:
  - decisive arbitration on single-candidate role meaning
  - non-decisive arbitration on conflicting role meaning
  - arbitration carry-through into `IntentIR`
  - validation metrics and findings for contested semantic arbitration

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 167/167 passed
## 2026-04-04 (canonical semantic candidate arbitration surface)

### Added: explicit semantic candidates on canonical interface signals
- `SemanticIR` now carries `semantic_candidates` on `InterfaceSignalRecord`.
- `IntentIR` carries that same candidate-arbitration surface forward unchanged.

### Added: typed candidate profiles for competing role meanings
- Each semantic candidate now records:
  - role
  - grounding strength
  - supporting source kinds
  - supporting observation count
  - strongest supporting automation confidence
  - deterministic evidence weight

### Changed: resolved semantic roles now build from canonical candidates
- Observation-backed resolved roles and consensus summaries are now built from the canonical candidate layer.
- When multiple role candidates exist, the signal keeps those candidates explicit instead of flattening the situation into only a conflict record.

### Added: validation metrics for canonical semantic arbitration
- `specforge validate` now reports:
  - `semantic_candidates`
  - `with_semantic_candidates`
  - `with_multiple_semantic_candidates`

### Added: regression coverage for canonical semantic candidates
- Added tests for:
  - candidate details on single-source, same-modality multi-source, and cross-modality role meanings
  - conflicting role meanings producing multiple canonical candidates without a resolved role
  - candidate carry-through into `IntentIR`
  - validation counts for multiple semantic candidates

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 167/167 passed

## 2026-04-04 (canonical semantic consensus summaries)

### Added: explicit semantic consensus summaries on canonical interface signals
- `SemanticIR` now carries `semantic_consensus` on `InterfaceSignalRecord` when a resolved role is backed by preserved observations.
- `IntentIR` now carries that same consensus summary forward unchanged.

### Added: semantic consensus profile details
- `semantic_consensus` currently records:
  - winning role
  - grounding strength
  - supporting source kinds
  - supporting observation count
  - strongest supporting automation confidence

### Changed: validation now surfaces fallback-only resolved roles
- `specforge validate` now reports:
  - `with_semantic_consensus`
  - `with_high_confidence_semantic_consensus`
  - `resolved_semantic_roles_without_consensus`
- It also emits an explicit finding when a resolved semantic role still lacks canonical consensus metadata.

### Added: regression coverage for canonical semantic consensus
- Added tests for:
  - consensus details on single-source, same-modality multi-source, and cross-modality semantic grounding
  - consensus carry-through into `IntentIR`
  - validation reporting and findings for resolved roles without consensus

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 166/166 passed

## 2026-04-04 (modality-aware semantic grounding strength)

### Changed: semantic grounding strength now distinguishes cross-modality reinforcement
- `SemanticIR` now derives `semantic_grounding_strength` as:
  - `single_source`
  - `multi_source`
  - `cross_modality`
- `IntentIR` carries that stronger distinction forward unchanged.

### Changed: repeated same-modality evidence no longer overclaims cross-modality support
- Repeated observations from one modality family now stay `multi_source`.
- Support spanning more than one modality family across table/prose/visual evidence now upgrades to `cross_modality`.

### Added: validation metric for cross-modality semantic grounding
- `specforge validate` now reports:
  - `with_cross_modality_semantic_grounding`

### Added: regression coverage for modality-aware role grounding
- Added tests for:
  - cross-modality semantic grounding on interface signals
  - same-modality multi-source semantic grounding on interface signals
  - cross-modality grounding carry-through into `IntentIR`
  - validation counts for both cross-modality and same-modality multi-source grounding

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 165/165 passed

## 2026-04-04 (canonical semantic-role consensus from preserved observations)

### Added: resolved semantic-role consensus on canonical interface signals
- `SemanticIR` now resolves `resolved_semantic_role` on `InterfaceSignalRecord` from canonical `semantic_observations` before falling back to merged `semantic_tags`.
- `IntentIR` carries that same canonical resolved-role surface forward unchanged.

### Added: grounding-strength visibility for semantic roles
- `InterfaceSignalRecord` now also carries `semantic_grounding_strength`.
- The canonical layers can now distinguish single-source grounding from multi-source grounding for resolved role meaning.

### Changed: validation now exposes semantic grounding quality directly
- `specforge validate` now reports:
  - `with_resolved_semantic_role`
  - `with_single_source_semantic_grounding`
  - `with_multi_source_semantic_grounding`
- This makes it visible when canonical role meaning is merely present versus reinforced by multiple preserved observations.

### Added: regression coverage for observation-backed role consensus
- Added tests for:
  - single-source resolved semantic roles on interface signals
  - multi-source semantic grounding on interface signals
  - validation counts for multi-source semantic grounding

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 162/162 passed

## 2026-04-04 (canonical semantic-role observation carry-through)

### Added: per-signal semantic observations in canonical IR
- `SemanticIR` now carries `semantic_observations` on `InterfaceSignalRecord`.
- `IntentIR` now carries the same role-observation surface forward.
- These observations preserve source kind, source text, and statement/table/visual provenance instead of flattening everything into merged `semantic_tags`.

### Changed: validation now exposes canonical role-grounding depth
- `specforge validate` now reports:
  - `semantic_observations`
  - `with_visual_semantic_grounding`
- This makes it visible when canonical signal meaning is actually grounded in preserved provenance rather than only implied by merged tags.

### Added: regression coverage for canonical observation preservation
- Added tests for:
  - carrying semantic observations into `SemanticIR` interface records
  - carrying semantic observations into `IntentIR`
  - exposing canonical semantic-observation counts in validation

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 160/160 passed

## 2026-04-04 (initial multimodal semantic-role grounding)

### Added: visual semantic-role hints in `EvidenceIR`
- `EvidenceIR` now mines `signal_semantic_hints` from grounded visual captions and VLM timing-diagram annotations.
- The visual path stays conservative: it only promotes hints when the text implies a role meaning and resolves to exactly one known signal.

### Added: explicit visual provenance for semantic-role hints
- `SignalSemanticHintRecord` now carries `supporting_visual_evidence_ids`.
- This keeps caption/VLM-derived role hints tied to concrete visual evidence instead of degrading into anonymous strings.

### Changed: validation now reports multimodal role-hint sources
- `specforge validate` now emits:
  - `signal_semantic_hints_from_visual_captions`
  - `signal_semantic_hints_from_vlm_timing_annotations`

### Added: end-to-end proof that visual grounding affects semantics
- Added tests for:
  - caption-grounded semantic-role hints
  - VLM timing-annotation-grounded semantic-role hints
  - semantic handshake completion derived from caption-grounded role hints

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 159/159 passed

## 2026-04-04 (semantic-role conflict carry-through into canonical IR)

### Added: carried semantic-role conflicts in `SemanticIR` and `IntentIR`
- `SemanticIR` now carries `signal_semantic_conflicts` forward from `EvidenceIR`.
- `IntentIR` now carries the same explicit role-conflict surface into the canonical endpoint.

### Changed: validation now reports semantic-role disagreement end-to-end
- `specforge validate` now prints and flags `signal_semantic_conflicts` for `SemanticIR` and `IntentIR`, not only for `EvidenceIR`.
- This keeps unresolved role disagreement visible to downstream consumers instead of letting it disappear after the evidence stage.

### Added: regression coverage for canonical conflict carry-through
- Added tests for:
  - carrying semantic-role conflicts into `SemanticIR`
  - carrying semantic-role conflicts into `IntentIR`
  - flagging those conflicts from semantic-stage and intent-stage validation

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 155/155 passed

## 2026-04-04 (explicit semantic-role conflict surfacing)

### Added: typed semantic-role conflicts in `EvidenceIR`
- `EvidenceIR` now persists `signal_semantic_conflicts` when meaning-based role evidence assigns incompatible roles to the same signal.
- This keeps role disagreement explicit instead of leaving it hidden inside a dual-tag ambiguity.

### Changed: validation now reports semantic-role disagreement clearly
- `specforge validate` now prints a dedicated semantic-role-conflict section for `EvidenceIR`.
- Validation now emits a `signal_semantic_conflicts` metric and a warning finding when incompatible role evidence is present.

### Added: regression coverage for semantic-role conflict surfacing
- Added tests for:
  - surfacing a role conflict when one source makes a signal look valid-like and another makes it look ready-like
  - flagging that conflict explicitly in `specforge validate`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 151/151 passed

## 2026-04-04 (SourceIR and ingest maturity guidance logged)

### Added: explicit steering on SourceIR maturity and remaining Tier 1 work
- Logged a durable implementation note clarifying that `specforge ingest` and `SourceIR` are different responsibilities:
  - ingest is the stage/command
  - `SourceIR` is the typed artifact/model
- Recorded the current maturity boundary:
  - architecturally strong and high leverage
  - not yet proven universal against arbitrary chip-spec PDFs

### Changed: roadmap now treats remaining Tier 1 work as robustness hardening
- `ROADMAP.md` now says the remaining `SourceIR` / ingest work should be:
  - robustness benchmarking
  - failure-mode detection
  - better fallback behavior
  - stronger source-level validation
- It also makes explicit that broad new Tier 1 expansion should stay secondary unless real PDFs expose a capture bottleneck.

### Validation
- docs-only change; Rust tests were not run

## 2026-04-04 (prose and alias-grounded semantic handshake roles)

### Added: prose and alias-grounded semantic role hints in `EvidenceIR`
- `EvidenceIR` now refreshes `signal_semantic_hints` from direct prose descriptions and alias-grounded prose descriptions, not only from `SignalDescription` tables.
- This means learned aliases can now contribute to typed semantic role grounding instead of only helping constraint reclassification.

### Changed: `nlp-enrich` now refreshes role hints before persistence
- `specforge nlp-enrich` now calls `refresh_signal_semantic_hints()` before writing updated `EvidenceIR`.
- Alias learning and backannotation can therefore feed the same loop-backed semantic-role surface immediately.

### Changed: validation now exposes where semantic role hints came from
- `specforge validate` now reports:
  - `signal_semantic_hints_from_tables`
  - `signal_semantic_hints_from_prose`
  - `signal_semantic_hints_from_alias_grounded_prose`

### Added: regression coverage for prose / alias-grounded role inference
- Added tests for:
  - alias-grounded prose descriptions producing semantic handshake hints in `EvidenceIR`
  - deriving a typed handshake predicate from alias-grounded semantic hints in `SemanticIR`
  - reporting alias-grounded semantic-hint counts in `specforge validate`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 149/149 passed

## 2026-04-04 (meaning-grounded handshake roles from signal descriptions)

### Added: typed semantic role hints in `EvidenceIR`
- `EvidenceIR` now persists `signal_semantic_hints` mined from `SignalDescription` table descriptions when the text establishes handshake-like request/valid or accept/ready meaning.
- The new records preserve source text, supporting table ids, and automation confidence instead of collapsing immediately into opaque downstream behavior.

### Changed: handshake detection now prefers grounded meaning before literal naming
- `SemanticIR` now carries per-signal `semantic_tags`, and `IntentIR` preserves the same surface at the canonical endpoint.
- Typed `HandshakeComplete` derivation now consults those meaning-grounded semantic tags before falling back to literal `VALID` / `READY` signal-name heuristics.

### Changed: validation now reports the new meaning-grounded role surface
- `specforge validate` now reports `signal_semantic_hints` for `EvidenceIR`.
- `specforge validate` now reports `with_semantic_tags` for `SemanticIR` and `IntentIR`.

### Added: regression coverage for meaning-grounded handshake-role carry-through
- Added tests for:
  - mining handshake-role semantic hints from signal-description tables in `EvidenceIR`
  - deriving a typed handshake predicate from semantic signal hints in `SemanticIR`
  - carrying signal semantic tags into `IntentIR`
  - reporting the new validation metrics in `specforge validate`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 146/146 passed

## 2026-04-04 (semantic-programming doctrine logged)

### Changed: the live roadmap now encodes how semantic intent should be programmed
- Logged a cross-cutting implementation doctrine in `ROADMAP.md`:
  - deterministic extraction where the source is crisp
  - typed protocol-world modeling as the semantic target
  - evidence aggregation and convergence instead of one-shot interpretation
  - bounded AI hypotheses rather than end-to-end black-box AI
  - explicit uncertainty, conflicts, and residual decisions as part of the truthfulness contract

### Added: detailed engineering guidance for programming semantics without full-pipeline AI
- Logged the full steering rationale in `DEVELOPMENT_NOTES.md` as a durable implementation note for future sessions.
- Refreshed `README.md` so the project objective explicitly states that `specforge` is building a typed protocol compiler, not an unrestricted English reader.

### Validation
- docs-only change; no Rust tests were run
## 2026-04-04 (typed ready/valid handshake completion)

### Added: protocol-native handshake predicates in the temporal layer
- `SemanticIR` now derives `TemporalPredicateRecord::HandshakeComplete` when a temporal rule contains grounded asserted `VALID` and `READY` signals in the same phase.
- This keeps ready/valid transfer completion visible as a first-class protocol event instead of only as two separate scalar guard clauses.

### Changed: validation now reports handshake-predicate coverage
- `specforge validate` now reports `temporal_rules_with_handshake_completion` for `SemanticIR` and `IntentIR`.
- This makes handshake-semantic coverage visible in the live validation surface instead of hiding it inside raw temporal-rule counts.

### Added: regression coverage for handshake temporal lift
- Added tests for:
  - deriving a typed handshake predicate from a valid/ready guard in `SemanticIR`
  - carrying that predicate into `IntentIR`
  - reporting handshake-predicate coverage in `specforge validate`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 141/141 passed

## 2026-04-04 (idiomatic one-cycle temporal language)

### Added: idiomatic one-cycle latency recovery in the temporal-rule layer
- `SemanticIR` temporal derivation now recognizes common protocol phrases such as `next cycle`, `next clock cycle`, `next tick`, and `next rising edge`.
- `following` and `subsequent` one-cycle variants now also map onto the canonical `CycleWindowRecord` surface instead of being left as unbounded prose.

### Changed: the explicit clock-tick model now covers both numeric and idiomatic latency language
- One-cycle prose no longer needs an explicit numeral like `within 1 cycle` to become a bounded temporal rule.
- This keeps the temporal model aligned with how real chip-design PDFs often describe synchronous behavior.

### Added: regression coverage for idiomatic one-cycle phrases
- Added tests for:
  - direct parser recovery of a single-cycle window from `next cycle`, `next tick`, and `next rising edge`
  - end-to-end temporal-rule derivation from a `next tick` signal constraint in `SemanticIR`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 138/138 passed

## 2026-04-04 (interface-signal conflict surfacing for conflicting declarations)

### Added: typed interface-signal conflicts in the canonical IR layers
- `SemanticIR` now persists `interface_signal_conflicts` when conflicting declarations disagree on a signal's direction or width.
- `IntentIR` now carries the same conflict surface forward so canonical interface-shape disagreement remains explicit downstream.

### Changed: validation now clearly reports interface-shape disagreement
- `specforge validate` now prints a dedicated interface-signal-conflict section for `SemanticIR` and `IntentIR`.
- Validation now emits a warning finding and metric when conflicting direction/width evidence is still unresolved in the canonical interface surface.

### Added: regression coverage for interface-signal conflict surfacing
- Added tests for:
  - deriving direction and width conflicts from contradictory explicit declarations in `SemanticIR`
  - carrying those conflicts into `IntentIR`
  - flagging them in `specforge validate`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 136/136 passed

## 2026-04-04 (structural KG conflict surfacing for multi-producer ambiguity)

### Added: typed structural connectivity conflicts in the canonical IR layers
- `SemanticIR` now persists `signal_connectivity_conflicts` when the structural KG resolves more than one producer for the same signal.
- `IntentIR` now carries the same conflict surface forward so unresolved producer ambiguity remains explicit at the canonical endpoint.

### Changed: validation now clearly reports structural producer ambiguity
- `specforge validate` now prints a dedicated signal-connectivity-conflict section for `SemanticIR` and `IntentIR`.
- Validation now emits a warning finding and metric when the structural KG still has unresolved multi-producer ambiguity.

### Added: regression coverage for structural KG conflict surfacing
- Added tests for:
  - deriving a signal-connectivity conflict from two producer claims in `SemanticIR`
  - carrying that conflict into `IntentIR`
  - flagging that carried conflict in `specforge validate`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 133/133 passed

## 2026-04-04 (explicit polarity-conflict surfacing in EvidenceIR validation)

### Added: typed polarity-conflict records in `EvidenceIR`
- `EvidenceIR` now persists `signal_polarity_conflicts` when prose and signal-description tables disagree on active-high/active-low semantics for the same anchored signal.
- This keeps contradictory polarity inspectable instead of only letting it disappear into a polarity-neutral derived constraint.

### Changed: validation now clearly reports polarity disagreement
- `specforge validate` now prints a dedicated polarity-conflict section for `EvidenceIR`.
- Validation now emits a warning finding and metric when signal polarity evidence disagrees across sources.

### Added: regression coverage for polarity-conflict reporting
- Added tests for:
  - persisting a polarity conflict while keeping the derived constraint neutral
  - flagging that persisted conflict in `specforge validate`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 130/130 passed

## 2026-04-04 (signal-table polarity refinement in convergent evidence)

### Added: signal-description tables now contribute polarity facts
- The convergent `EvidenceIR` loop now scans `SignalDescription` tables for active-high/active-low signal facts using known signals as anchors.
- This lets table rows refine asserted/deasserted constraints even when the polarity never appears in prose.

### Changed: polarity merging is now cross-modality and conservative
- Prose polarity and signal-table polarity are now merged before constraint refinement.
- Conflicting polarity across prose and tables cancels the refinement instead of forcing a wrong `MustBeHigh` / `MustBeLow` conversion.

### Added: regression coverage for table-driven polarity refinement
- Added end-to-end tests for:
  - table-driven active-low polarity refinement
  - preserving polarity neutrality when prose and table evidence disagree

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 129/129 passed

## 2026-04-04 (typed temporal conflict records)

### Added: explicit temporal conflict records in the canonical IR layers
- `SemanticIR` now derives `temporal_conflicts: Vec<TemporalConflictRecord>` from contradictory typed temporal value obligations.
- `IntentIR` now carries the same conflict surface forward so disagreement remains explicit downstream.

### Changed: validation now reports and flags typed temporal conflicts
- `specforge validate` now reports `temporal_conflicts` for `SemanticIR` and `IntentIR`.
- Validation now emits a dedicated warning when contradictory temporal value obligations are present in the typed rule set.

### Added: regression coverage for temporal contradiction surfacing
- Added end-to-end tests for:
  - deriving a typed temporal conflict from contradictory value obligations in `SemanticIR`
  - carrying the conflict into `IntentIR`
  - validating that the contradiction is surfaced as a temporal-conflict finding

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 127/127 passed

## 2026-04-04 (compound temporal antecedents in typed temporal rules)

### Added: conjunctive temporal guards now survive as multiple typed antecedents
- `parse_temporal_condition_predicates()` now preserves compound guards like `when HREADY is LOW and HSEL is HIGH` as multiple antecedent predicates when each clause grounds to a known signal.
- This means the typed temporal layer no longer drops half of a conjunctive protocol precondition during semantic lift.

### Changed: validation now reports multi-predicate temporal guard coverage
- `specforge validate` now reports `temporal_rules_with_multi_predicate_antecedents` for `SemanticIR` and `IntentIR`.
- This gives the live validation surface an explicit signal that conjunctive temporal guards are surviving into the canonical IR.

### Added: regression coverage for compound temporal guards
- Added end-to-end tests for:
  - deriving multi-predicate antecedents from a compound temporal guard in `SemanticIR`
  - carrying those antecedents into `IntentIR`
  - validation metrics for multi-predicate temporal antecedents

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 124/124 passed

## 2026-04-04 (actor-grounded stability semantics in temporal rules)

### Added: actor-relative stability predicates in the temporal layer
- `TemporalPredicateRecord` now includes `ActorMaintainsSignalStable`.
- Stable/hold-style temporal consequents now emit actor-grounded stability predicates when the structural KG resolves a unique producer for the signal.
- This means the temporal layer can now express not just that a signal remains stable, but which actor is responsible for maintaining that stability across the tick window.

### Changed: actor-grounding validation now counts actor-grounded stability too
- `specforge validate` now treats `ActorMaintainsSignalStable` as actor-grounded temporal evidence alongside `ActorDrivesSignal` and `ActorSamplesSignal`.

### Added: regression coverage for actor-grounded stability lift
- Added end-to-end tests for:
  - deriving `ActorMaintainsSignalStable` from a stable constraint with a unique producer
  - carrying actor-grounded stability rules into `IntentIR`
  - validation metrics for actor-grounded stability rules

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 121/121 passed

## 2026-04-04 (actor-grounded temporal drive events)

### Added: actor-relative drive predicates in typed temporal rules
- `TemporalPredicateRecord` now includes `ActorDrivesSignal`.
- Temporal-rule derivation now emits actor-relative drive predicates when the structural KG provides a unique producer for the constrained signal.
- This keeps the temporal layer aligned with the structural graph instead of representing every bounded/value rule as a signal-only event.

### Changed: validation now counts actor-grounded temporal rules
- `specforge validate` now reports `temporal_rules_with_actor_grounding` for `SemanticIR` and `IntentIR`.
- Validation now flags temporal-rule sets that exist alongside a non-empty actor-signal graph but still carry no actor-relative drive/sample grounding at all.

### Added: regression coverage for actor-grounded temporal lift
- Added end-to-end tests for:
  - deriving `ActorDrivesSignal` from a value constraint with a unique producer in the KG
  - carrying actor-grounded temporal rules into `IntentIR`
  - validation metrics for actor-grounded temporal rules

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 118/118 passed

## 2026-04-04 (cycle-window recovery in temporal rules)

### Added: bounded latency windows in the temporal-rule layer
- `SemanticIR` temporal-rule derivation now recovers `CycleWindowRecord` bounds from prose such as:
  - `within 2 cycles`
  - `for 2 cycles`
  - `at least 1 cycle`
  - `at most 3 cycles`
  - `between 1 and 3 cycles`
- Timing rows whose unit is already `cycles` now also project their numeric min/max/typ values into `cycle_window`.

### Changed: validation now counts bounded temporal rules
- `specforge validate` now reports `temporal_rules_with_cycle_window` for `SemanticIR` and `IntentIR`.
- Validation now flags when typed temporal rules exist but none of them currently carry explicit cycle-window bounds.

### Added: regression coverage for cycle-window carry-through
- Added tests for:
  - cycle-window derivation from a cycle-bounded signal constraint
  - cycle-window carry-through into `IntentIR`
  - validation metrics for bounded temporal rules

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 115/115 passed

## 2026-04-04 (typed temporal-rule surface in SemanticIR / IntentIR)

### Added: initial clock-tick temporal rules in the canonical IR layers
- `SemanticIR` now carries `temporal_rules: Vec<TemporalRuleRecord>` alongside legacy timing/constraint records.
- `IntentIR` now carries the same `temporal_rules` surface forward so downstream consumers can target a typed temporal layer instead of only free-form timing text.
- The first predicate set covers:
  - signal value predicates at explicit tick phases
  - signal stability across `pre_tick -> post_tick`
  - signal sampling on clock edges, with optional actor grounding

### Changed: temporal grounding no longer depends on a full reset contract
- Temporal-rule derivation now reuses an explicit clock declaration even when the spec has not yet surfaced a full `SystemContractRecord`.
- This lets timing/constraint semantics ground to a real clock as soon as `Clock <signal>.` is known, instead of waiting for both clock and reset declarations.

### Changed: validation now reports temporal-rule presence and grounding gaps
- `specforge validate` now reports `temporal_rules` and `temporal_rules_missing_clock_grounding` for `SemanticIR` and `IntentIR`.
- Validation findings now explicitly call out:
  - when typed temporal rules exist but still lack clock/edge grounding
  - when timing/constraint evidence exists but no typed temporal rules were derived

### Added: regression coverage for the new temporal layer
- Added end-to-end tests for:
  - temporal-rule derivation from a conditioned signal constraint plus explicit clock context
  - temporal-rule carry-through from `SemanticIR` into `IntentIR`
  - validation diagnostics for ungrounded temporal rules

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 114/114 passed

## 2026-04-04 (graph-first direction scoring in validation)

### Changed: `specforge validate` now scores direction coverage from the actor-relative graph first
- `validate_semantic_ir()` and `validate_intent_ir()` now treat actor-relative `actor_ports` coverage as the primary signal-direction surface and only fall back to flat `direction_hint` values as a compatibility layer.
- Validation metrics now split direction coverage into:
  - `with_resolved_direction`
  - `with_graph_direction`
  - `with_compat_direction_hint`
- Compatibility lag still surfaces as an informational finding, but flat `direction_hint` absence no longer lowers direction coverage when the actor-relative graph already resolves the signal.

### Added: regression coverage for graph-first validation behavior
- Added `validate_intent_ir_scores_direction_from_graph_before_compat_hints`.
- The regression locks the expected behavior: removing flat compatibility hints from an `IntentIR` fixture with intact actor-relative ports must not lower the direction score.

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 111/111 passed

## 2026-04-04 (roadmap retuned around semantic truthfulness)

### Changed: roadmap priorities now explicitly favor KG quality over adapter breadth
- Logged the roadmap reassessment in `DEVELOPMENT_NOTES.md`: the existing four-layer architecture is still right, but the near-term program must be semantic-truthfulness hardening rather than adapter expansion.
- Updated `ROADMAP.md` so the next named milestones are:
  - graph-first downstream semantics
  - explicit clock-tick temporal modeling
  - KG-guided multimodal rescans
  - cross-modality evidence arbitration
  - KG-quality evaluation with gold and negative fixtures
- Demoted SystemVerilog/Verilog/VHDL adapter expansion and adapter validation to horizon work until the semantic pipeline is materially harder to fool.

### Changed: continuity docs now steer future sessions toward the truth-model program
- Updated `LIVE_ACHIEVEMENT_STATUS.md`, `RUST_CODEBASE_ANALYSIS.md`, `README.md`, and `USER_GUIDE.md` so they no longer imply that adapter validation is the next priority.
- The repo now consistently states that adapters should consume truth, not compensate for missing truth in the KG and temporal model.

## 2026-04-04 (steering note: multimodal semantic recovery)

### Changed: implementation guidance now explicitly centers multimodal semantic recovery
- Logged the current extraction philosophy in `DEVELOPMENT_NOTES.md` as a steering principle for future work.
- The note makes the target explicit: recover enough grounded intent from tables, figures, and prose for downstream RTL/verification generation rather than treating PDF parsing as the end goal.
- It also records the preferred tactic for future hurdles: use the KG as a search index for repeated rescans, keep the pipeline provenance-first, and prefer reusable document-native lifting strategies over speculative adapter-side inference.

## 2026-04-04 (full-converge defaults + AXI convergence stabilization)

### Changed: `specforge converge` now defaults to the full Ollama-backed loop
- `ConvergeArgs` now default `--vlm-provider` and `--nlp-provider` to `ollama` instead of `skip`.
- The intended default pipeline path is now encoded in the CLI itself: figure enrichment and NLP Level 3 run automatically during `specforge converge` unless the caller explicitly opts out.

### Fixed: monotone knowledge accounting no longer treats fewer residuals as less knowledge
- `crates/specforge/src/commands/converge.rs` no longer counts downstream adapter residual work toward `knowledge_fact_count`.
- This fixes the false `pipeline knowledge shrank` failure mode seen on a full AXI rerun, where later passes correctly reduced residual decisions but the old accounting treated that as regression.
- `EvidenceIr` and `specforge nlp-enrich` now also normalize duplicate loopback NLP records before persistence so repeated identical extractions do not inflate later passes.

### Validation
- Added regression tests:
  - `dedup_loopback_records_removes_duplicate_constraints_and_rules`
  - `nlp_enrich_dedups_duplicate_extractions_before_persisting`
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 110/110 passed
- Full original-PDF AXI converge:
  - `cargo run -p specforge -- converge /Users/richarddje/Documents/livework/chipdoc/arm/amba/core/axi/current/IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf --target fsm --max-iterations 5 --vlm-provider ollama --vlm-model qwen2.5vl:7b --nlp-provider ollama --nlp-model qwen2.5vl:7b` → converged in 2 passes
- Refreshed projected AMBA validation snapshot:
  - APB `IHI0024_D`: 95/100 EXCELLENT
  - AHB `IHI0033_C`: 95/100 EXCELLENT
  - AXI `IHI0022_L`: 94/100 EXCELLENT
  - `cargo run -p specforge -- project-validation generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json` → passed

## 2026-04-04 (validation snapshot projection into tracked live docs)

### Added: deterministic live-doc projection for persisted validation reports
- Added `specforge project-validation <artifact>...`.
- The new command validates each passed IR artifact, reuses the persisted `validation_reports`, writes a tracked `VALIDATION_SNAPSHOT.md`, and refreshes the managed validation projection block in `LIVE_ACHIEVEMENT_STATUS.md`.
- Projection output is deterministic: artifact ordering uses `document_key`, findings sort by severity/category/id, and repo-internal artifact paths are rendered as relative paths.

### Changed: staged validation continuity no longer depends on manual markdown edits
- `generated/` remains untracked, but validation snapshots can now be pulled back into tracked docs on demand after local APB/AHB/AXI or other validation runs.
- The live roadmap/status/docs now treat staged IR validation projection as implemented; remaining validation work is adapter-focused.
- Refreshed the tracked validation snapshot against the current AMBA `IntentIR` artifacts:
  - APB `IHI0024_D`: 95/100 EXCELLENT
  - AHB `IHI0033_C`: 95/100 EXCELLENT
  - AXI `IHI0022_L`: 89/100 GOOD

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 107/107 passed
- `cargo run -p specforge -- project-validation generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json` → passed

## 2026-04-04 (validation back-annotation on IR artifacts)

### Added: persisted validation reports for the four IR stages
- `specforge validate <artifact>` now writes a deterministic `validation_report.json` sidecar next to `SourceIR`, `EvidenceIR`, `SemanticIR`, and `IntentIR` artifacts
- the same validation report is now backannotated into the artifact itself via a `validation_reports` field
- `SemanticIR` / `IntentIR` validation findings now include graph-aware checks for missing producers, missing consumers, and compatibility-surface lag relative to the actor-relative KG

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 106/106 passed

## 2026-04-04 (actor-relative KG carry-through into SemanticIR / IntentIR)

### Added: downstream preservation of the structural knowledge graph
- `SemanticIR` now preserves the extracted actor-signal graph via:
  - `actor_signal_relations`
  - `actor_ports`
  - `signal_connectivity`
- `IntentIR` now carries the same actor-relative KG surface forward as canonical output instead of forcing downstream consumers to rediscover relation evidence from `EvidenceIR`
- actor records now preserve grounded actor names when relation evidence makes them explicit

### Changed: validation now surfaces KG-native counts
- `specforge validate` now reports actor-signal relation, actor-port, and signal-connectivity counts for `SemanticIR` and `IntentIR`
- flat `direction_hint` fields remain as a compatibility surface, but they are no longer the only downstream representation of signal direction semantics

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 104/104 passed

## 2026-04-04 (continuity sync + local validation snapshot)

### Changed: live continuity docs now reflect the current post-converge state
- Updated the live documentation surface so crash recovery and handoff notes match the current repository status after the converge/VLM work.
- `generated/` is now treated as a local artifact root only: artifacts still materialize there, but the directory is git-ignored and no longer versioned.

### Validation
- `cargo test --manifest-path Cargo.toml` → 102/102 passed
- Current local validation snapshot:
  - APB `IHI0024_D`: 95/100 EXCELLENT after a full original-PDF `specforge converge` run with Ollama VLM + NLP Level 3; converged in 2 passes
  - AHB `IHI0033_C`: 95/100 EXCELLENT from the current local `IntentIR` snapshot
  - AXI `IHI0022_L`: 89/100 GOOD from the current local `IntentIR` snapshot
- Current AXI caveat:
  - the local AXI `SourceIR` has 20 timing diagrams classified, but the current local artifact still lacks persisted VLM timing enrichment, so timing remains the most obvious remaining score gap

## 2026-04-03 (whole-pipeline converge command + preserved loopback knowledge)

### Added: `specforge converge`
- Introduced a new top-level `converge` command that materializes the staged pipeline as a fixed-point loop instead of a one-shot chain.
- The command ingests a source once, optionally re-runs VLM figure enrichment and NLP Level 3 backannotation, rebuilds `EvidenceIR`, `SemanticIR`, `IntentIR`, and the selected adapter artifact, and stops when the persisted knowledge snapshot is unchanged between passes.
- The snapshot currently covers the staged artifacts that already persist facts today: `SourceIR`, `EvidenceIR`, `SemanticIR`, `IntentIR`, and adapter artifacts.

### Changed: `EvidenceIr::build()` now preserves prior loopback knowledge across rebuilds
- When rebuilding from the same persisted `SourceIR`, `EvidenceIr::build()` now carries forward:
  - learned signal aliases,
  - NLP-upgraded `ExtractedStatement` classes,
  - prior `SignalConstraintRecord`s,
  - prior `ConditionalRuleRecord`s.
- This closes the architectural gap where pass `N+1` could previously forget what `nlp-enrich` discovered in pass `N`.

### Changed: `specforge enrich` is now idempotent for already-enriched figures
- Timing/state-machine diagrams whose `VisualAsset.note` already contains a VLM extraction payload are skipped on later passes.
- This keeps multi-pass orchestration from re-querying the same diagram needlessly.

### Validation
- Added regression tests:
  - `converge_rebuilds_pipeline_until_snapshot_stabilizes`
  - a shared process-global test env lock now serializes `SPECFORGE_VLM_HELPER` mutations across convergence/NLP tests
- `cargo test --manifest-path Cargo.toml` → 100/100 passed

## 2026-04-03 (nlp-enrich alias marker filter + README staged-flow validation)

### Fixed: `extract_alias_phrase()` in `crates/specforge/src/commands/nlp_enrich.rs`
- Alias learning now rejects subject phrases that begin with markdown/table markers `-`, `|`, or `#`.
- This closes the remaining Form 2 cleanup gap where bullet rows, table cells, or heading-prefixed text could otherwise be learned as garbage aliases such as `- the address`.
- Ordinary prose alias learning remains unchanged for real noun phrases such as `address bus`.

### Validation
- Added regression test:
  - `extract_alias_phrase_rejects_markdown_marker_prefixes`
- `cargo test --manifest-path Cargo.toml` → 99/99 passed
- Re-executed the documented README entry flow on `README.md`:
  - `cargo run -p specforge -- --help`
  - `cargo run -p specforge -- inspect README.md`
  - `cargo run -p specforge -- ingest README.md --dry-run`
  - `cargo run -p specforge -- ingest README.md`
  - `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json --dry-run`
  - `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json`
  - `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json --dry-run`
  - `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json`
  - `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json --dry-run`
  - `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json`
  - `cargo run -p specforge -- adapt generated/intent_ir/readme/intent_ir.json --target fsm --dry-run`
- Materialized README artifacts now validate the staged flow end-to-end:
  - SourceIR: 0 page artifacts, 0 visual assets
  - EvidenceIR: 15 section anchors, 190 evidence spans, 190 extracted statements
  - SemanticIR: 2 actors, 6 phases, 5 invariants, 8 gates
  - IntentIR: 2 actors, 14 behaviors, 6 constraints, 1 assumption

## 2026-04-03 (convergent EvidenceIR enrichment + refreshed APB/AHB/AXI artifacts)

### Added: monotone convergent extraction loop in `evidence.rs`
- Replaced the one-shot post-table extraction tail in `EvidenceIr::build()` with `converge_evidence_extractions()`.
- Each pass now:
  - seeds from table-derived signal/enum facts,
  - rescans signal-anchored encoding tables,
  - collects newly discovered enum/value atoms,
  - extracts additional prose value constraints,
  - refines asserted/deasserted constraints with active-low/high polarity prose,
  - synthesizes KG-derived direction declarations,
  - repeats until no new synthesized statements appear.
- Rationale: the previous build order could synthesize useful `Enum ...` facts and then end the build before later prose extraction had a chance to reuse them in the same build.

### Added: anchored encoding-table recovery without hardcoded protocol value lists
- New helpers in `crates/specforge/src/ir/evidence.rs`:
  - `scan_encoding_tables_by_signal_anchor()`
  - `collect_discovered_enum_values()`
  - `extract_discovered_state_value_from_text()`
  - `extract_signal_polarity_from_prose()`
  - `apply_signal_polarity_to_constraints()`
  - `extract_dynamic_signal_constraints()`
  - `dedup_actor_signal_relations()`
- `synthesize_encoding_declarations()` now delegates to `synthesize_encoding_declarations_for_enum()` so the same encoding synthesis logic can be reused by the convergence loop.
- No new APB/AHB/AXI-specific enum/value list was added; discovered values come from extracted tables and synthesized `Enum ...` statements only.

### Changed: `classify_table_kind()` in `docling_backend.rs`
- Signature widened to `classify_table_kind(header_rows, body_rows=None, caption_text=None)`.
- Table classification now uses caption cues, header cues, and first-column body content together instead of headers alone.
- Added content-based encoding detection for weakly labeled tables by scanning body rows for binary/hex literals and bit-field references such as `HTRANS[1:0]`.
- The call site now passes `body_rows` into the helper so the classifier can use real cell content.

### Validation
- Added regression tests:
  - `anchored_encoding_scan_unlocks_dynamic_value_constraint_extraction`
  - `prose_polarity_refines_asserted_constraint_kind`
- `cargo test --manifest-path Cargo.toml` → 98/98 passed
- `cargo build --release --manifest-path Cargo.toml` → passed
- Refreshed generated APB/AHB/AXI artifacts and revalidated the current `IntentIR` outputs:
  - APB: 90/100 EXCELLENT
  - AHB: 95/100 EXCELLENT
  - AXI: 90/100 GOOD
- Net score change from the earlier baseline:
  - APB: 75 → 90
  - AHB: 95 → 95
  - AXI: 90 → 90

## 2026-04-03 (broader timing_diagram classification for figure captions)

### Fixed: classify_diagram_kind() in docling_backend.rs Python helper
- Added a second pass to the timing_diagram check: for any asset whose caption
  contains "figure", also classify as timing_diagram when caption uses protocol
  execution vocabulary: "transfer", "transaction", "handshake", "burst",
  "exit from reset", "sequence diagram".
- Rationale: bus protocol specs name clocked waveform figures after the operation
  they depict. Explicit "timing" / "waveform" words are often absent. The check
  is intentionally inclusive; VLM handles borderline cases gracefully.
- Simulated impact on AXI after re-ingest:
  - timing_diagram: 2 → 20 (+18)
  - New: VALID/READY handshake waveforms, write/read transaction dependencies,
    atomic transactions, wrapping transfers, PCMO, snoop, sequence diagrams.
  - Non-timing figures (architecture, data structure, topology) stay unknown.
- 96/96 tests pass.
## 2026-04-03 (caption-gated signal_description classification in Docling ingest helper)

### Fixed: classify_table_kind() in docling_backend.rs Python helper
- Added `caption_text=None` parameter; caption is now checked BEFORE header-based classification.
- Root cause: protocol payload/message field tables share the Name|Width|Description
  header structure with interface signal tables but describe message payload fields,
  not hardware interface pins. Without a caption check they were misclassified as
  signal_description.
  - AXI "Table A15.3: DVM message fields" is the confirmed instance: VA, PA, ASID,
    ASIDV, VMID, VMIDV, DVMType are DVM message payload fields, not interface signals.
- Fix: `caption_is_payload` flag blocks signal_description when caption contains:
  "message field(s)", "payload field(s)", "packet field(s)", "command field(s)",
  "frame field(s)".
- Call site updated: `classify_table_kind(header_rows, caption_text)`.
- Impact (requires re-ingest to take effect):
  - AXI DVM message field table: signal_description → unknown
  - DVM fields removed from declared signal set
  - 82 legitimate AXI signal_description tables unaffected
  - Width coverage: 98% → 100% after re-ingest + pipeline re-run
- 96/96 tests pass
## 2026-04-03 (row-scan clock/reset detection, immune to Docling column-ordering bugs)

### Fixed: synthesize_system_contract_from_table_descriptions() (evidence.rs)
- Replaced column-index-based detection with a full row-scan that inspects every
  cell in each body row, independent of column position.
- Root cause: Docling mis-assigns body cells to wrong column buckets for tables with
  visually distinctive (bold/boxed) cells whose PDF span-count arithmetic shifts.
  AHB Table 2-1 "Global signals" is a confirmed instance: HCLK/HRESETn are in
  col 0 of the PDF but Docling places them in col 3 of the parsed grid.
- New per-row algorithm:
  - Signal candidate: cell with ≤2 whitespace tokens, first token is a valid
    hardware signal name (not a role word like CLOCK/RESET/SOURCE).
    Excludes description cells (many words) and role cells ("Clock source" etc.).
  - Clock/reset desc: scan all cells for keyword patterns; keep the longest
    matching text so "The bus clock times all bus transfers…" beats "Clock source",
    giving accurate polarity/kind inference for resets.
- Result: AHB system contract (HCLK + HRESETn) correctly detected.
- AHB score: 90/100 → 95/100 EXCELLENT (contract_bonus 0/5 → 5/5).
- APB and AXI scores unchanged (signal tables were already correct).
- 96/96 tests pass (no new tests needed — existing contract detection tests pass).
## 2026-04-03 (header-clue + positional column detection; AMBA 5 direction mapping)

### Fixed: synthesize_signal_declarations() — direction column semantics (evidence.rs)
- Split the single dir_col into three distinct column types with correct semantics:
  - `explicit_dir_col`: header contains "direction" → literal input/output cell value
  - `source_col`: header contains "source" or "driver" → cell names the DRIVING actor
    - Requester/Initiator/Master → output; Completer/Subordinate/Slave/Target/Responder → input
    - Clock/Reset/System-bus/Global → input (infrastructure distributed into all blocks)
  - `dest_col`: header contains "destination" → cell names the RECEIVING actor (inverted)
    - Signal flows TO Subordinate/Completer → output; flows TO Manager/Requester → input
- Fixes APB: "Source" column with values "Requester"/"Completer"/"Clock"/"System bus reset"
  was previously unrecognised → all APB signals silently dropped; now correctly mapped
- Fixes AHB test: "Destination" column with "Subordinate"/"Manager" values now uses
  inverted semantics (flowing TO Subordinate = output, not input)

### Fixed: name column now header-detected with positional fallback (evidence.rs)
- name_col: search headers for signal/name/port/pin; fall back to col 0 (leftmost)
- Previously hardcoded to row.first(); now honours header position when available

### Fixed: emit width-only declaration when direction is unknown (evidence.rs)
- Added (None, Some(WidthHint::Numeric)) and (None, Some(WidthHint::Parametric)) arms
- Signals with known width but no determinable direction now emit "Signal X is width N."
  instead of being silently dropped

### Fixed: infer_signal_direction_from_section() (evidence.rs)
- Added "requester" → output (AMBA 5 APB terminology)
- Added "completer"/"target" → input
- Added "reset" to the infrastructure group → input

### Fixed: synthesize_system_contract_from_table_descriptions() (evidence.rs)
- Replaced misleading comment "No header analysis needed" with header-first detection
- name_col: headers with signal/name/port/pin → else col 0
- desc_col: headers with description/desc → else last column

### Fixed: duplicate OKAY pattern in collect_subject_signal_tokens() (evidence.rs)
- Removed second OKAY from the exclusion match arm (compiler unreachable_patterns warning)

### Test suite: 96/96 pass (no change in count)
## 2026-04-03 (WidthHint: parametric widths + table width map for KG synthesis)

### New type: WidthHint (source.rs)
- Replaces Option<u32> for signal width throughout the IR
- WidthHint::Numeric(u32) — fixed bit width (1, 2, 32, 64, ...)
- WidthHint::Parametric(String) — user-configurable RTL parameter (ADDR_WIDTH, DATA_WIDTH/8, ...)
- Backward-compatible serde: Numeric(32) -> JSON 32, Parametric("ADDR_WIDTH") -> JSON "ADDR_WIDTH"
- Both variants count as "known width" in coverage metrics and scoring

### Changed: synthesize_signal_declarations() in evidence.rs
- Width parsing now returns Option<WidthHint> instead of Option<u32>
- Numeric: parse::<u32>() -> WidthHint::Numeric
- Parametric: non-numeric, non-empty, contains alphabetic -> WidthHint::Parametric
- No artificial upper bound on numeric widths (removed the w <= 1024 filter)
- Synthesized text includes parametric widths: "Signal PADDR is output width ADDR_WIDTH."

### New: collect_signal_widths_from_tables() in evidence.rs
- Extracts width (numeric or parametric) from signal-description table Width columns
- Passed to synthesize_directions_from_relations() so KG-synthesized declarations carry width

### Changed: synthesize_directions_from_relations() in evidence.rs
- Now accepts width_map: HashMap<String, WidthHint>
- Produces "Signal PADDR is output width ADDR_WIDTH." instead of just "Signal PADDR is output."

### Changed throughout: semantic.rs, intent.rs, adapters.rs
- InterfaceSignalRecord.width_hint: Option<u32> -> Option<WidthHint>
- ExplicitTopPortRecord.width_hint: Option<u32> -> Option<WidthHint>
- InterfaceSignalAccumulator.width_hint: Option<u32> -> Option<WidthHint>
- ParsedInterfaceSignalDeclaration.width_hint: Option<u32> -> Option<WidthHint>
- parse_width_token() -> returns Option<WidthHint> (both numeric and parametric)
- merge_signal_hint<T: Copy+Eq> -> <T: Clone+Eq> (WidthHint is Clone but not Copy)
- register_interface_signal_record() signature updated
- WidthCast in expression parser kept as u32 (literal numeric, not parametric)
- Adapters convert Option<WidthHint> -> Option<u32> via .as_numeric() for FSM emission

### validate.rs: display numeric vs parametric width breakdown
- "with_width: N (X%) [N numeric, N parametric]"
- Both numeric and parametric count in width coverage score

### Results
| Spec | Before | After | Change |
|------|--------|-------|--------|
| AHB | 88/100 GOOD, 80% width | 90/100 EXCELLENT, 100% width [10 num, 11 para] | +2 pts |
| APB | 60/100 ADEQUATE, 0% width | 70/100 GOOD, 100% width [10 num, 8 para] | +10 pts |
| AXI | 75/100 GOOD, 0% width | 85/100 GOOD, 99% width [126 num, 51 para] | +10 pts |
## 2026-04-03 (R13: Tier 2 Knowledge Graph — actor-signal relation extraction)

### New types (source.rs)
- RelationKind enum (Drives | Reads)
- ActorSignalRelation struct { relation_id, actor_name, signal_name, relation, source_statement_ids, automation_confidence }

### New EvidenceIR field (evidence.rs)
- actor_signal_relations: Vec<ActorSignalRelation> (serde default = empty; extracted at build time)

### Tier 2 extraction: two complementary sources
1. **Prose verb-pattern extraction** (extract_actor_signal_relations()):
   - Passive drives: "SIGNAL is {driven|asserted|returned|...} by ACTOR" and "...from ACTOR"
   - Active drives: "ACTOR {drives|asserts|returns|...} SIGNAL" and "ACTOR must {drive|...} SIGNAL"
   - Passive reads: "SIGNAL is {read|sampled|monitored|...} by ACTOR"
   - Active reads: "ACTOR {reads|samples|...} SIGNAL"
   - Only searches for confirmed signal names (from tables + declarations)
2. **Signal table Source column extraction** (extract_relations_from_signal_tables()):
   - Reads Source/Driver/Direction column from signal_description tables
   - APB "PADDR | Requester | ..." → (Requester, Drives, PADDR)
   - APB "PREADY | Completer | ..." → (Completer, Drives, PREADY)
   - AHB tables already have direction column (covered by existing synthesis)
   - AXI tables have no Source column (no triples from tables, only from prose)

### Signal name collection: two sources
- collect_signal_names_from_tables(): ALL first-column signal names from signal_description tables (regardless of whether direction was extracted — covers APB/AXI where Source column is non-standard)
- collect_known_signal_names(): Signal names from existing "Signal X is input/output" prose declarations
- Union of both used as the search universe for prose verb patterns

### Direction synthesis: non-conflicting
- synthesize_directions_from_relations(): creates "Signal X is output." for Drives triples
- Skips signals already declared from tables (table declarations are authoritative)
- KG synthesis only adds direction for signals that had NO prior table-derived declaration

### Updated Layer D (semantic.rs) and Layer E (validate.rs)
- Declared signal set now includes Medium confidence (KG-derived) signals in addition to High confidence (table-derived)

### Results after R13
| Spec | Before R13 | After R13 | Change |
|------|-----------|-----------|--------|
| AHB  | 86/100 GOOD     | 85/100 GOOD | -1 (21 vs 17 declared, 100% dir, 47% width) |
| APB  | 35/100 NEEDS IMP | 60/100 ADEQUATE | +25 pts, 18 declared signals, 100% dir |
| AXI  | 85/100 (misleading, 1 sig) | 75/100 GOOD (honest, 182 signals) | Honest |

### Tests: 6 new (90 → 96, all passing)
- passive_drive_pattern_extracts_actor_and_signal
- active_drive_pattern_extracts_actor_and_signal
- passive_read_pattern_extracts_actor_and_signal
- must_drive_pattern_extracts_actor_from_requester_sentence
- synthesize_directions_produces_signal_is_output_declaration
- kg_extraction_produces_graph_declarations_in_evidence_ir
## 2026-04-02 (AHB + APB end-to-end pipeline validation run)

### AHB (IHI0033_C) results — full feedback loop on existing SourceIR
- Layer A suppressed 13 boilerplate NormativeStatements (85 → 72 residuals before nlp-enrich)
- nlp-enrich Pass 1: 72 candidates → 26 extracted (13 signal + 13 conditional), Form 1 backannotated 26, 1 alias learned (low-quality: "- the address" from markdown table row)
- nlp-enrich Pass 2: 46 candidates → 0 extracted → convergence at residual=46 (pass 3 stable check)
- Layer D gating: 17 declared signals (100% direction, 58% width), 248 heuristic noise excluded
- Final score: **86/100 — GOOD** (signal_dir=25/25, width=5.8/10, constraints=30/30, enums=15/15, registers=5/5, timing=5/5)
- Residual NormativeStatements: 46 (architectural/infrastructure sentences with no named signal)

### APB (IHI0024_E) results — full ingest from PDF + feedback loop
- Ingested: 48 pages, 35 visual assets
- EvidenceIR: 517 statements, 18 NormativeStatements, 37 signal_constraints (Level 2)
- nlp-enrich Pass 1: 18 candidates → 8 extracted, Form 1 backannotated 8, 0 aliases learned
- nlp-enrich Pass 2: 10 candidates → 0 extracted → convergence at residual=10
- Layer D gating: **0 declared signals** — APB signal description tables not detected as SignalDescription kind, so no High-confidence records; direction/width coverage = 0%
- Final score: **35/100 — NEEDS IMPROVEMENT** (constraints=30/30, registers=5/5, all signal coverage zero)
- Root cause: APB table classification is returning Unknown instead of SignalDescription for the signal description tables → no synthesized "Signal X is input/output" statements → Layer D has no declared set → direction/width = 0 → score tank

### Issues identified
1. **APB signal table classification**: APB tables not being classified as SignalDescription; need to inspect APB structured_tables
2. **Alias extraction quality**: "- the address" alias from markdown table row prefix is garbage — need to filter phrases starting with "-" or pure markdown tokens
## 2026-04-02 (remove --max-passes: residual-stable convergence criterion)
- **Removed --max-passes CLI option** from NlpEnrichArgs: was a safety net that is no longer needed.
- **New convergence criterion**: loop stops when residual(N) == residual(N-1).  Termination is guaranteed because the residual pool is finite and can only decrease or stay flat (monotone).  The criterion covers Form 2 alias reclassifications AND LLM extractions together, unlike the previous "pass_extracted == 0" check which only counted LLM extractions and could stop prematurely.
- **Loop structure**:  replaced by  with pass counter for display only.  dry-run breaks after one pass.
- **All 90 tests updated**: removed max_passes field from all NlpEnrichArgs struct literals; convergence test comment updated to describe residual-stable criterion.
## 2026-04-02 (Form 2: signal alias learning feedback loop)
- **EvidenceIr.signal_alias_map** (evidence.rs): new BTreeMap<String,String> field (serde default = empty). Persisted to JSON so aliases accumulate across nlp-enrich runs.
- **apply_alias_reclassification()** (EvidenceIr pub method): applies accumulated alias map to re-classify remaining NormativeStatements WITHOUT LLM calls. For each sentence containing a known alias phrase, substitutes the signal name (uppercase) and re-checks is_signal_value_constraint(). If true: reclassifies statement to SignalValueConstraint, synthesises a SignalConstraintRecord (AutomationConfidence::Low, alias-derived).
- **detect_constraint_kind_from_substituted()** (evidence.rs): helper detects must_not_change / must_be_stable / must_be_high / must_be_low / must_be_asserted / must_be_deasserted from substituted mixed-case text.
- **extract_alias_phrase()** (nlp_enrich.rs): after each successful LLM extraction, if the signal name does not appear literally in the source text, extracts a 2-4 word noun phrase (strips leading articles, rejects pronouns, limits to 4 words) and inserts it into evidence_ir.signal_alias_map.
- **Pass loop integration**: (1) START of each pass: apply_alias_reclassification() shrinks candidate pool for free; (2) AFTER each LLM extraction: learn alias if signal not in text; (3) write EvidenceIR even when only aliases were learned (no LLM extractions). Summary reports total_alias_reclassified and signal_alias_map_size.
- **Converging loop**: with --max-passes N, iteration 1 builds alias dict; iteration 2+ applies it, progressively reducing NormativeStatement residuals without LLM calls; converges when neither LLM extraction nor alias reclassification produces anything new.
- **7 new tests** (83 -> 90 total, all passing):
  - evidence.rs: apply_alias_reclassification_reclassifies_normative_statement_with_alias, apply_alias_reclassification_skips_already_covered_sentences
  - nlp_enrich.rs: extract_alias_phrase_returns_none_when_signal_appears_literally, extract_alias_phrase_extracts_noun_phrase_when_signal_absent, extract_alias_phrase_rejects_pronoun_only_subjects, extract_alias_phrase_limits_to_four_words, nlp_enrich_learns_alias_and_stores_in_evidence_ir
## 2026-04-02 (Form 1: backannotation feedback loop)
- **Form 1: backannotation** (nlp_enrich.rs): after each nlp-enrich pass, ExtractedStatement.class updated in-place: NormativeStatement -> SignalValueConstraint or ConditionalRule. Closes feedback loop from Level 3 back to EvidenceIR.
- **Fixed test parallelism bug**: added vlm_helper_lock() mutex (OnceLock<Mutex<()>>) to serialize 4 tests sharing SPECFORGE_VLM_HELPER env var.
- **Test suite: 82 -> 83 (+1, all passing)**
## 2026-04-02 (NLP pipeline Layers A/B/C/D/E: boilerplate suppression, grounded multi-pass NLP, declared-signal gating, spec-type-aware scoring)
- **Layer A — Section-aware boilerplate suppression** (`evidence.rs`)
  - New `is_boilerplate_section_title()` helper: matches Introduction, Revision History, Legal Notice, Normative/Informative References, Glossary, Acronyms, Bibliography, Scope, Terms and Definitions, About this Document, and related headings
  - `EvidenceIr::build()` block loop: looks up each sentence's section heading; if boilerplate, downgrades `NormativeStatement` → `SourceFact`
  - Effect: ~12 legal/compliance normative sentences removed from residual pool per real spec (e.g. AHB). Residuals: 59 → ~47
  - 2 tests: `is_boilerplate_section_title` unit test (12 positive + 5 negative assertions), integration test verifying intro section normative sentence becomes SourceFact while protocol section stays NormativeStatement
- **Layer D — Declared-signal gating** (`semantic.rs`)
  - `SemanticIr::build()`: after `build_interfaces()`, extracts declared signal set from `AutomationConfidence::High` interface records (those from formal `Signal X is input/output` synthesized declarations)
  - Filters `signal_constraints` and `conditional_rules` to only records where the subject/consequent signal is in the declared set; gating is disabled (all kept) if no signal declarations exist (prose-only specs)
  - Effect: heuristic noise signals (from NLP token extraction) suppressed from NLP records; only real declared signals survive. Eliminates the signal noise that diluted coverage metrics
  - 1 test: `signal_constraints_for_undeclared_signals_are_filtered_by_layer_d` — HREADY (declared) survives, NOTSIG (undeclared) removed
- **Layer E — Spec-type-aware quality scoring** (`validate.rs`)
  - Imports `AutomationConfidence` to filter signal records in `validate_intent_ir()`
  - Coverage metrics now count ONLY `AutomationConfidence::High` (declared) signals; heuristic signals reported separately as `heuristic_signal_records (excluded from coverage)`
  - New formula (100 pt max, additive, no FSM penalty for non-FSM specs):
    - Signal direction coverage (declared only): 0–25 pts
    - Signal width coverage (declared only): 0–10 pts
    - NLP constraint richness (signal + conditional, capped at 30): 0–30 pts
    - Encoding enum definitions: 0–15 pts
    - Register map records: 0–5 pts
    - Timing constraint records: 0–5 pts
    - State machine (bonus, not penalty): 0–5 pts
    - System contract (bonus, not penalty): 0–5 pts
  - Score breakdown printed per component for transparency
  - AHB projected score after all layers: ~80/100 (GOOD) vs. 27/100 before
- **Layers B+C — Grounded multi-pass NLP Level 3** (`cli.rs`, `nlp_enrich.rs`)
  - `NlpEnrichArgs`: added `--grounding-signals` (comma-separated declared signal names, or omit for auto-extraction) and `--max-passes` (default 1; multi-pass stops early on convergence)
  - `auto_extract_declared_signals()`: parses `Signal X is input/output` statements from EvidenceIR to auto-build grounding list
  - `build_nlp_prompt()` now accepts `grounding_signals: &[String]`; injects "Known hardware signals: HADDR, HTRANS, ..." section before the sentence when non-empty
  - Multi-pass loop: each pass re-derives candidates (skipping already-extracted sentences); stops when pass extracts 0 new records (convergence) OR max_passes reached; writes EvidenceIR after every productive pass
  - `count_candidate_statements()` extracted as helper for `skip` mode hint
  - 5 new tests: prompt includes grounding signals, no grounding section when empty, `parse_signal_declaration_name` extracts uppercase name, multi-pass convergence test (max_passes=3 stops after 1 productive pass)
  - Updated existing tests to include new `grounding_signals: None, max_passes: 1` fields
- **Test suite: 75 → 82 (+7 tests, all passing)**
## 2026-04-02 (NLP Level 1+2 pattern expansion: ~50%→70%+ coverage uplift)
- **Level 1 `classify_statement()` vocabulary expanded significantly**
  - `NormativeStatement`: added `cannot/can not`, `is not permitted/allowed`, `are not permitted/allowed`, `may not`, `is forbidden/illegal`, `will not`, `must/shall never`, `it is mandatory`, `is not valid/legal/supported`, `are required`
  - `ConditionalRule`: added `unless`, `provided that`, `as long as` (both leading and embedded); `while/during/after/before` now also work as leading conditionals; `cannot` added to consequent verb list
  - `SignalValueConstraint` (`is_signal_value_constraint()`): added `is tied high/low/to`, `is driven high/low`, `is held/kept high/low/stable/asserted`, `remains high/low/asserted/deasserted/stable`, `cannot change`, `cannot/will not/must not/shall not be changed`, `must/shall indicate`, `must/shall not be asserted/deasserted` (passive negation forms)
  - `TimingConstraint`: added `tco/tpd/toh/tih`, `rising/falling/clock/positive/negative edge`, `within one/two clock`, `cycles` plural
- **Level 2 `extract_signal_constraints()` multi-signal extraction**
  - Strip condition clause before scanning subject signals: `HREADY` in `"...when HREADY is LOW"` is no longer confused with the constrained signal
  - New helper `text_before_condition_marker()`: returns text before first `when/while/during/unless/provided/after/before` marker
  - New helper `collect_subject_signal_tokens()`: collects ALL valid uppercase signal tokens from a text fragment (excludes logic levels, protocol states, protocol family names, role names)
  - Multi-signal sentences like `"Both HTRANS and HADDR shall be stable"` now produce one `SignalConstraintRecord` per signal instead of one
  - Negation detection now includes `cannot` and `will not`
- **Level 2 `split_conditional_sentence()`**: added `unless`, `provided that`, `as long as` as leading conditional markers
- **Level 2 `extract_protocol_state_value()`**: added INCR4/INCR8/INCR16, WRAP4/WRAP8/WRAP16, EXCLUSIVE, RETRY, SPLIT, BYTE, HALFWORD, WORD
- **15 NLP regression tests added** (60 → 75 total; all passing)
  - Tests confirm: `cannot/is not permitted/may not` → NormativeStatement; `is tied high/is held stable/cannot change/remains stable` → SignalValueConstraint; `unless/provided that/before` → ConditionalRule; `rising edge period` → TimingConstraint; multi-signal subject extraction; condition-clause stripping; logic-level exclusion from subjects
  - Clarifying comments: tests document that more-specific `SignalValueConstraint` correctly wins over NormativeStatement when a sentence contains both a value-binding phrase and a prohibition keyword
## 2026-04-02 (qwen2.5vl:7b integration: VLM fix + NLP Level 3 nlp-enrich command)
- **Critical VLM truncation bug fixed in `enrich.rs`**
  - Removed `.min(120)` cap on VLM response storage that silently corrupted every real VLM response
  - Replaced fragile `"content":` string-search with proper `serde_json` parsing of `{choices[0].message.content}`; handles both string and array content parts with clear error on invalid JSON
  - `max_tokens` increased 1024 → 2048 for VLM diagram responses
- **Default Ollama model updated**: `llava:13b` → `qwen2.5vl:7b` (both VLM enrichment and NLP Level 3)
  - `qwen2.5vl:7b` outperforms GPT-4o-mini on document/diagram understanding benchmarks; available via `ollama pull qwen2.5vl:7b` (6GB)
  - `qwen2.5vl:7b` pulled and ready on local Ollama instance
- **`specforge nlp-enrich` command (NLP Level 3) implemented**
  - `specforge nlp-enrich <evidence-ir> --vlm-provider ollama [--vlm-model qwen2.5vl:7b] [--dry-run] [--max-sentences N]`
  - Reads `NormativeStatement` sentences from EvidenceIR not already covered by Level 2
  - Sends each sentence to LLM with a structured extraction prompt (text-only, no image)
  - Prompt yields a single JSON: `signal_constraint / conditional_rule / none`
  - Robust to markdown code-fence wrapping; validates uppercase signal names; graceful `none` handling
  - Writes new `SignalConstraintRecord` / `ConditionalRuleRecord` entries back to EvidenceIR JSON
  - `SPECFORGE_VLM_HELPER` env var override for unit testing
  - 5 tests: end-to-end pipeline, dry-run isolation, code-fence JSON parsing, invalid signal rejection, conditional rule extraction
- **Test suite: 55 → 60 (+5 NLP Level 3 tests)**
## 2026-04-02 (VLM wiring, validate command, 55-test suite, doc corrections)
- **VLM observations wired into EvidenceIR** (Steps 3.2/3.3 complete end-to-end)
  - Added `TimingDiagramExtraction` and `StateMachineExtraction` to `VisualObservationKind` in `evidence.rs`
  - New `inject_vlm_observations()`: reads `VisualAsset.note` prefix `"vlm_timing_diagram_extraction: {json}"` / `"vlm_state_machine_extraction: {json}"` and injects typed `VisualObservation` entries into the matching `VisualEvidenceItem`
  - Enriched figures automatically upgraded to `VisualEvidenceRole::Normative` (highest-priority evidence)
- **SemanticIR VLM observation parsing** (timing + state machine → typed records)
  - New `extract_records_from_vlm_observations()` in `semantic.rs` iterates EvidenceIR visual observations
  - `parse_timing_diagram_observation()`: each VLM annotation string → `TimingConstraintRecord { description: annotation, confidence: Medium }`; merged with table-synthesized timing constraints
  - `parse_state_machine_observation()`: each VLM state → `RegularStateRecord`; each VLM transition → `StateTransitionRecord`; merged with formal syntax records (non-duplicate append)
  - Result: timing constraints from both tables and VLM diagrams, state records from both formal syntax and VLM extraction
- **specforge validate command** (Step 4.1 complete)
  - New `crates/specforge/src/commands/validate.rs` — auto-detects IR stage from `stage` field in artifact JSON
  - `validate_source_ir`: document profile, table classification, diagram classification, VLM readiness, section classification, residual count
  - `validate_evidence_ir`: statement classification breakdown, NLP coverage %, structured extraction counts, VLM observation counts
  - `validate_semantic_ir`: signal coverage (with_direction %, with_width %, fully_typed %), semantic record counts, system contract, residual decisions
  - `validate_intent_ir`: signal coverage, intent record counts, quality score (0–100) with grade EXCELLENT/GOOD/ADEQUATE/NEEDS IMPROVEMENT/INCOMPLETE
  - Wired in `cli.rs` as `Commands::Validate(ValidateArgs)` and dispatched in `lib.rs`
- **Test suite expanded: 49 → 55 (+6)**
  - `ir::semantic::tests::vlm_timing_diagram_observation_produces_timing_constraint_records` — full chain: SourceIR note → EvidenceIR observation → SemanticIR timing constraints
  - `ir::semantic::tests::vlm_state_machine_observation_produces_state_and_transition_records` — full chain: SourceIR note → EvidenceIR observation → SemanticIR states/transitions
  - `commands::validate::tests::validate_source_ir_artifact_reports_without_error`
  - `commands::validate::tests::validate_evidence_ir_artifact_reports_without_error`
  - `commands::validate::tests::validate_semantic_ir_artifact_reports_without_error`
  - `commands::validate::tests::validate_intent_ir_artifact_reports_without_error`
  - All 55 tests pass, 0 failures
- **EXTRACTION_ARCHITECTURE.md** corrected with accurate status for all completed steps:
  - SourceIR: DiagramKind ✅, VLM enrichment ✅
  - EvidenceIR: SignalConstraintRecord ✅, ConditionalRuleRecord ✅, TimingDiagramExtraction/StateMachineExtraction ✅
  - SemanticIR: signal_constraints ✅, conditional_rules ✅, VLM state/transition merge ✅, VLM timing merge ✅
  - IntentIR: signal_constraints ✅, conditional_rules ✅, VLM state/transition records ✅
  - Tier 3 Steps 3.1/3.2/3.3: ✅ done; added Step 3.4 (NLP Level 3: LLM-based reclassification) as planned next step
  - Step 4.1 Validation: ✅ done
## 2026-04-02 (NLP Level 2 structured extraction + VLM enrichment pipeline)
- **EXTRACTION_ARCHITECTURE.md** updated as authoritative reference: NLP 4-level pyramid, classification-vs-extraction gap analysis, VLM provider architecture (Ollama/OpenAI/LM Studio), all implementation steps with precise ✅/❌ status
- **Level 2 NLP: SignalConstraintRecord extraction**
  - New types in `source.rs`: `SignalConstraintKind`, `SignalConstraintRecord`, `ConditionalRuleRecord`
  - `extract_signal_constraints()` in `evidence.rs`: for each `SignalValueConstraint` sentence, extracts `{subject_signal, constraint_kind, target_value, condition_text, negated}` via syntactic pattern matching; stop-worded for AMBA/ARM/company names
  - `extract_conditional_rules()` in `evidence.rs`: for each `ConditionalRule` sentence, extracts `{antecedent_text, consequent_signal, consequent_action}` by sentence splitting on when/if/while/during
  - `EvidenceIr.signal_constraints` + `EvidenceIr.conditional_rules` as first-class typed fields
  - Carried through `SemanticIr.signal_constraints` and `IntentIr.signal_constraints`
  - AHB result: 12 `SignalConstraintRecord` (HAUSER must_not_change, HEXOKAY must_be_deasserted, etc.), 36 `ConditionalRuleRecord`
- **DiagramKind classification in SourceIR (Step 3.1)**
  - New `DiagramKind` enum in `source.rs`: `TimingDiagram`, `StateMachineDiagram`, `BlockDiagram`, `RegisterBitfield`, `TruthTable`, `FlowChart`, `Unknown`
  - `VisualAsset.diagram_kind` field set from caption text in Docling Python helper
  - `classify_diagram_kind()` in Python helper: pattern-matches AMBA-specific caption vocabulary ("read transfer", "write transfer", "burst", "wait state" → `timing_diagram`; "Manager interface", "multiplexor interconnection" → `block_diagram`; etc.)
  - AHB result: **17 timing diagrams** correctly classified, 3 block diagrams
- **specforge enrich command (Step 3.2/3.3)**
  - New `specforge enrich <source-ir> --vlm-provider <provider>` command
  - Providers: `ollama` (localhost:11434, model `llava:13b`), `openai` (OPENAI_API_KEY, model `gpt-4o`), `lmstudio` (localhost:1234), `skip` (default)
  - `--classify-only` flag: shows timing/state-machine diagram counts without calling VLM
  - `--vlm-model` override for custom models
  - `--dry-run` shows which figures would be sent to VLM without making calls
  - `SPECFORGE_VLM_HELPER` env var override for unit testing (same pattern as `SPECFORGE_DOCLING_HELPER`)
  - Structured prompts: timing diagram → `{signals, cycles, annotations}` JSON; state machine → `{states, transitions}` JSON
  - All providers use OpenAI-compatible chat completions API (supports Docling's granite-docling model via Ollama or LM Studio)
  - VLM enrichment writes updated `VisualAsset.note` with typed extraction; downstream `specforge evidence` picks it up
- 48 tests, 0 failures
## 2026-04-02 (Tier 2: Register/Timing type system + NormativeStatement sub-classes)
- added `RegisterRecord` and `RegisterFieldRecord` types to `source.rs` (foundation layer, no circular deps)
- added `TimingConstraintRecord` type to `source.rs`
- re-exported these types from `semantic.rs` so `IntentIR` and adapters import from `semantic` as before
- added `synthesize_register_records()` in `evidence.rs`: reads `SourceIR.structured_tables` where `table_kind == RegisterMap`; extracts register name, offset address, bit field rows
- added `synthesize_timing_constraints()` in `evidence.rs`: reads `SourceIR.structured_tables` where `table_kind == TimingParameter`; extracts parameter name, min/typ/max values, unit
- added `EvidenceIr.register_records: Vec<RegisterRecord>` and `EvidenceIr.timing_constraints: Vec<TimingConstraintRecord>` as typed first-class fields
- carried `register_records` and `timing_constraints` through `SemanticIr` and `IntentIr` unchanged
- extended `StatementClass` with `TimingConstraint` (cycle counts, setup/hold references, latency bounds) and `ConditionalRule` (`when X then Y` / `if A then B` conditional behavioral structures)
- updated `classify_statement()` to detect `TimingConstraint` and `ConditionalRule` patterns before the generic `NormativeStatement` check
- updated `EXTRACTION_ARCHITECTURE.md` to reflect precise current done/pending status and sharpen modality descriptions with exact type names
- validated on AMBA AHB PDF: 21 register records, 8 timing constraints, statement classes: 91 normative_statement, 42 conditional_rule, 30 timing_constraint, 14 derived_rule, 5 explicit_abstraction (vs. 100% source_fact before)
- all `cargo fmt` and `cargo test` pass: 48 tests, 0 failures
## 2026-04-02 (SOTA SourceIR and EvidenceIR)
- created `EXTRACTION_ARCHITECTURE.md` — comprehensive reference document capturing the full SOTA extraction vision for chip spec PDFs: six information modalities, quality gap analysis per IR stage, target architecture, and priority-ordered implementation plan (Tier 1–4)
- updated `ROADMAP.md` with new workstreams R8 (SourceIR SOTA capture), R9 (EvidenceIR SOTA typed evidence), and R10 (EvidenceIR VLM visual content)
- extended Docling Python helper to extract in a single pass:
  - **structured table cell grids** (`StructuredTableRecord` with header/body row cells, row/col spans, `is_header` flags)
  - **table kind classification** (`classify_table_kind`): `signal_description`, `encoding`, `register_map`, `timing_parameter`, `feature_matrix`, `unknown`
  - **typed content elements** (`ContentElementRecord`): all text elements with Docling type labels (title, section_header, body_text, list_item, code, caption, footnote, formula), reading order, page provenance
  - **section hierarchy with semantic classification** (`ContentSectionRecord` with `SectionKind`): `Boilerplate`, `SignalDescription`, `Normative`, `Timing`, `RegisterDescription`, `Glossary`, `Appendix`, `TableOfContents`
  - **document profile** (`DocumentProfile`): title, page/table/figure/section counts
- added new Rust types to `source.rs`: `StructuredTableRecord`, `StructuredTableCellRecord`, `TableKind`, `ContentElementRecord`, `ContentElementKind`, `ContentSectionRecord`, `SectionKind`, `DocumentProfile`
- updated `SourceIr` struct with new `#[serde(default)]` fields: `structured_tables`, `content_elements`, `document_sections`, `document_profile`
- updated `DoclingBackendSummary` to deserialize all new fields; updated `SourceIr::materialize()` to populate them
- added `StatementClass::NormativeStatement` to `EvidenceIR` statement classification — sentences with `shall`/`must`/`shall not` in non-boilerplate sections are now correctly classified as behavioral requirements rather than generic `SourceFact`
- added `synthesize_declarations_from_tables()` in `EvidenceIR` that reads `source_ir.structured_tables` and synthesizes formal typed declarations:
  - `SignalDescription` tables → `Signal X is output/input [width N].` declarations (High confidence)
  - `Encoding` tables → `Enum <name> <member> = <value>.` declarations (High confidence)
  - Direction inferred from `ContentSectionRecord.section_kind` + section title keywords; width from numeric cell values; non-signal tokens filtered via `is_signal_synthesis_non_signal()`
- **removed `parse_signal_table_row` band-aid from `SemanticIR`** — signal declarations now flow cleanly from `EvidenceIR` structured table synthesis through `SemanticIR`'s existing `parse_explicit_signal_declaration` and `parse_explicit_symbol_definition` parsers
- validated on AMBA AHB Protocol Specification PDF (SOTA pipeline, re-ingested):
  - `source_ir.structured_tables`: 40 tables (11 signal_description, 2 encoding, 3 register_map, 2 timing_parameter, 22 unknown)
  - `source_ir.content_elements`: 1004 typed text elements
  - `source_ir.document_sections`: 172 sections (115 normative, 37 signal_description, 8 boilerplate, 5 timing, 4 appendix, 2 glossary, 1 table_of_contents)
  - `source_ir.document_profile`: page_count=104, table_count=40, figure_count=30, section_count=172
  - 17 AHB signals with explicit direction+width in adapter signal inventory (HSELX newly added from Decoder table)
  - `NormativeStatement` classification active for behavioral requirements
  - No SemanticIR band-aid; signal declarations flow architecturally
- all `cargo fmt` and `cargo test` checks pass: 48 tests, 0 failures
## 2026-04-02 (continued)
- improved `SemanticIR` extraction quality for real chip specification PDFs with three targeted fixes:
  - **expanded `signal_stop_words()`** with ~200 entries covering legal/contractual vocabulary, common English all-caps words (HIGH, LOW, etc.), AMBA/ARM protocol family names, company names, document structure words, and technology abbreviations that are never hardware signal names; this eliminates legal front-matter contamination from signal extraction
  - **added boilerplate section filtering** in `SemanticContext::from_evidence_ir` so statements from sections matching legal/admin patterns (licence, proprietary notice, change history, etc.) are excluded from actor/interface/invariant extraction entirely
  - **added interface noise filtering** in `build_interfaces` so heuristic interfaces with >8 signals require ≥2 supporting statements; this eliminates the large spurious interfaces created by co-mentions in legal paragraphs while keeping all small hardware signal groups
- added **markdown signal-table row parsing** in `build_interfaces`: when a table row's first cell looks like a hardware signal name and the section title matches a known direction context ("Manager signals" → output, "Subordinate signals" → input, "Global/Decoder signals" → input), the row is parsed directly into a typed `InterfaceSignalRecord` with explicit direction and width, extracted at Medium automation confidence
- validated improvements on the AMBA AHB Protocol Specification PDF (`IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf`):
  - 104 page artifacts and 70 visual assets materialized correctly by Docling
  - `interface_count` reduced from 172 → 94 (45% reduction, legal text gone)
  - `signal_candidate_count` in the adapter reduced from 250 → 57 (77% reduction, mostly real AHB signals)
  - 16 AHB signals now carry explicit direction and width from signal table parsing:
    - Manager outputs (direction=output): HADDR, HBURST, HEXCL, HMASTER, HMASTLOCK, HNONSEC, HPROT, HSIZE, HTRANS, HWDATA, HWRITE, HWSTRB
    - Subordinate outputs / Manager inputs (direction=input): HEXOKAY, HRDATA, HREADYOUT, HRESP
    - Key widths extracted: HTRANS=2, HSIZE=3, HWRITE=1, HMASTLOCK=1, HEXCL=1, HNONSEC=1, HREADYOUT=1, HRESP=1, HEXOKAY=1
  - adapter correctly blocked (honest: AHB spec prose does not carry formal control blocks or system contract declarations)
- added 1 new regression test: `extracts_signal_direction_and_width_from_markdown_signal_description_table`
  - verifies Manager-section rows are extracted as Output with correct numeric widths
  - verifies Subordinate-section rows are extracted as Input with correct numeric widths
  - total tests: 48 passing, 0 failing
- installed Docling 2.84.0 globally into Python 3.11 (`/opt/homebrew/lib/python3.11/site-packages/`) to enable PDF processing
- all `cargo fmt` and `cargo test` checks pass
## 2026-04-02
- widened `SemanticIR` so it now preserves canonical `.fsm`-relevant symbol-definition and structured-control surface rather than relying only on legacy decision-tree fragments:
  - canonical symbol definitions for `Constant`, `Define`, `Param`, and `Enum`
  - canonical control expressions, branch-local actions, and dedicated synchronous-reset/asynchronous-reset control-block roles
  - richer module-scoped carry-through for the same widened semantic surface
- widened the canonical reset contract so `SystemContractRecord` now preserves reset kind, reset polarity, assertion timing, release timing, and reset-target semantics explicitly instead of leaving real hardware reset behavior implicit
- tightened reset normalization so explicit reset declarations now:
  - preserve synchronous reset as synchronous assertion + synchronous release through the data-input path
  - preserve asynchronous reset as asynchronous assertion + synchronous release through the dedicated reset pin
  - infer active-low polarity from `_n` / `_b` reset naming and otherwise fall back to active-high with lower automation confidence when explicit polarity wording is omitted
- widened `IntentIR` so it now carries canonical symbol-definition sections and structured control blocks unchanged for downstream adapters
- refactored the `.fsm` adapter to lower from canonical `symbol_definitions` and `control_blocks` first, with legacy decision-tree fragments kept only as a fallback when the widened canonical surface is absent
- tightened `.fsm` system-contract renderability so reset polarity must stay recoverable honestly from `sreset` / `asreset` plus the reset signal name in the current target slice
- widened emitted `.fsm` text so the renderable slices now cover:
  - `+constants`, `+define`, `+params`, and `+enums` sections
  - structured standalone/DT and FSM-root lowering from canonical control blocks
  - canonical synchronous-reset and asynchronous-reset control-role blocks
  - explicit public-output targets and dual-output assignment forms carried through the widened control model when renderable
- widened the `.fsm` adapter so selector/test-node control and canonical compound-update actions now lower honestly into emitted `.fsm` text when their canonical selector/predicate/update shapes map directly to explicit `.fsm` test-selector tokens and update shorthand, while unsupported selector predicates remain blocked explicitly
- repaired accidental corruption in the `adapters.rs` regression module and tightened adapter residual logic so renderable compound-update artifacts no longer keep a stale `fsm_adapter_dt_action_graph` packet
- reviewed the current `fsmgen` direct-root contract and confirmed that `?mod:name` / `?module:name` are still compatibility-level accepted spellings on a shared single-module path rather than a settled backend-neutral semantic distinction for SpecForge
- tightened the SpecForge `.fsm` adapter root-kind model so it now only represents the current honest canonical roots (`dt`, `fsm`, `top`) and no longer carries speculative `mod` / `module` placeholder variants in adapter JSON or deferred-root decisions
- tightened explicit reset parsing so both of these phrasing styles now normalize into the widened backend-neutral reset contract:
  - `Reset rst_n is asynchronous active low.`
  - `Reset rst is synchronous active high.`
- added regression coverage for:
  - semantic extraction of synchronous active-high reset phrasing
  - intent carry-through of synchronous active-high reset phrasing
  - semantic and intent carry-through of inferred active-low reset polarity from `rst_n`
  - honest adapter blocking when reset polarity cannot be preserved through the reset signal name
  - renderable standalone DT lowering with canonical symbol-definition sections
  - renderable structured FSM lowering with canonical reset-role blocks
  - renderable selector-based standalone DT lowering
  - renderable computed-selector standalone DT lowering
  - honest blocking when a selector branch predicate does not map relative to the chosen selector
  - renderable compound-update standalone DT lowering
  - tightened deferred-root decisions so renderable/blocked adapter artifacts keep only the current honest root-kind set (`dt`, `fsm`, `top`)
- validated the widened `.fsm` semantic slice with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo fmt --all --manifest-path Cargo.toml --check`
  - `cargo test --manifest-path Cargo.toml adapters`
  - `cargo test --manifest-path Cargo.toml`
  - an execute-mode end-to-end CLI pipeclean on a temporary inferred-polarity reset sample through `ingest -> evidence -> semantic -> intent -> adapt`
  - an execute-mode end-to-end CLI pipeclean on a temporary synchronous-active-high reset sample through `ingest -> evidence -> semantic -> intent -> adapt`
  - an execute-mode end-to-end CLI pipeclean on a temporary selector/test-node sample through `ingest -> evidence -> semantic -> intent -> adapt`
  - an execute-mode end-to-end CLI pipeclean on a temporary compound-update sample through `ingest -> evidence -> semantic -> intent -> adapt`
- confirmed the representative inferred-polarity end-to-end adapter output is now safely renderable while preserving the widened reset contract in JSON:
  - `document_key: inferred_reset_cli`
  - `semantic/system_contract.reset_polarity: active_low`
  - `semantic/system_contract.assertion_timing: asynchronous_to_clock`
  - `semantic/system_contract.release_timing: synchronous_to_clock`
  - `semantic/system_contract.target_kind: dedicated_reset_pin`
  - `semantic/system_contract.automation_confidence: medium`
  - `intent/system_contract` matches the widened semantic reset contract exactly
  - `emitted_target_path: generated/adapters/fsm/inferred_reset_cli/inferred_reset_cli.fsm`
- confirmed the representative synchronous-active-high end-to-end adapter output is now safely renderable:
  - `document_key: sync_control`
  - `lowering_status: renderable`
  - `selected_root_kind: fsm`
  - `emitted_target_path: generated/adapters/fsm/sync_control/sync_control.fsm`
- confirmed the representative selector/test-node end-to-end adapter output is now safely renderable:
  - `document_key: selector_dt`
  - `lowering_status: renderable`
  - `selected_root_kind: dt`
  - `residual_decision_count: 2`
  - `emitted_target_path: generated/adapters/fsm/selector_dt/selector_dt.fsm`
  - emitted test-node block includes `(?MODE ...)` and the selector branch token `=mode_t.idle`
- confirmed the representative compound-update end-to-end adapter output is now safely renderable:
  - `document_key: compound_update_dt`
  - `lowering_status: renderable`
  - `selected_root_kind: dt`
  - `residual_decision_count: 4`
  - `emitted_target_path: generated/adapters/fsm/compound_update_dt/compound_update_dt.fsm`
  - emitted update block includes `(-bump` and `(+= ACC STEP)`
- refreshed the live documentation surface so the README, user guide, live status tracker, codebase analysis, development notes, roadmap, and continuity records now describe the widened canonical reset contract, the landed selector/test-node and compound-update slice, the current reset-naming convention, and the remaining direct-module alias gap plus explicit unsupported selector/predicate boundaries
- refreshed the same live documentation surface again so it now records the direct-module defer decision explicitly, removes speculative adapter root-kind language, and advances the next milestone to validation/back-annotation
## 2026-04-01
- enriched `SemanticIR` so it now preserves explicit backend-neutral module and top-composition records from `Module ...` and `Top ...` statements, including typed top ports, child-module references, and explicit wiring links
- tightened semantic extraction so module-scoped and top-scoped statements are handled through scoped parsing helpers and no longer leak into document-global direct-root inference
- enriched `IntentIR` so it now carries canonical explicit module and top-composition surface forward unchanged for downstream adapters
- widened the `.fsm` adapter beyond a single direct-root model so it now:
  - inventories explicit module candidates and explicit top candidates from canonical intent records
  - selects an honest `?top:name` root when exactly one explicit top composition is renderable
  - renders stable top-level support blocks such as `?ports:public_io` and `?toplink:wiring`
  - embeds referenced renderable child module roots after the selected `?top:name` root
  - keeps standalone direct `?mod:name` / `?module:name` alias roots deferred until there is a real backend-neutral direct-module distinction
- tightened adapter renderability checks for explicit top composition so emitted `.fsm` text now requires:
  - fully typed explicit top ports
  - existing referenced child modules
  - renderable child module roots
  - width-compatible and direction-compatible explicit link endpoints
  - explicit links for the current multi-child composition slice
- tightened adapter residual logic so standalone DT residuals are suppressed when an explicit `?top:name` source document is selected and composition-specific residuals remain honest when child modules or links are missing
- extended `specforge adapt` execute-mode summaries with `module_candidate_count` and `top_candidate_count`
- added regression coverage for:
  - explicit module/top extraction in `SemanticIR`
  - explicit module/top carry-through in `IntentIR`
  - renderable and blocked explicit top-composition adapter paths
- validated the new explicit composition slice with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/explicit_top.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/explicit_top/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/explicit_top/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/explicit_top/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/explicit_top/intent_ir.json --target fsm`
- confirmed the representative explicit top-composition end-to-end adapter output is now safely renderable:
  - `lowering_status: renderable`
  - `selected_root_kind: top`
  - `signal_candidate_count: 1`
  - `decision_tree_candidate_count: 0`
  - `state_candidate_count: 0`
  - `transition_candidate_count: 0`
  - `module_candidate_count: 2`
  - `top_candidate_count: 1`
  - `residual_decision_count: 3`
  - `emitted_target_path: generated/adapters/fsm/explicit_top/datapath.fsm`
- refreshed the live documentation surface so the roadmap, status trackers, user guide, architecture docs, and continuity files now describe the landed explicit `?top:name` slice and the still-deferred direct module-alias roots
- enriched `SemanticIR` so it now preserves backend-neutral regular-state and transition records from explicit `State ...` and `Transition ...` statements
- enriched `IntentIR` so it now carries canonical regular-state and transition surface forward for downstream adapters
- widened the `.fsm` adapter so it now selects honest `?fsm:name` roots from the explicit canonical state graph, groups state-matching control fragments into state bodies, preserves unmatched control fragments as standalone `-block` children, and renders sequential state-body assignments with `<=`
- tightened adapter-side residual logic so the unresolved state-graph packet only remains when structured FSM lowering is actually blocked
- extended `specforge adapt` execute-mode summaries with `transition_candidate_count` for explicit FSM-root pipecleans
- added regression coverage for:
  - explicit regular-state and transition extraction in `SemanticIR`
  - canonical regular-state and transition carry-through in `IntentIR`
  - renderable and blocked structured `?fsm:name` adapter paths
- validated the new structured FSM slice with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/explicit_fsm.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/explicit_fsm/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/explicit_fsm/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/explicit_fsm/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/explicit_fsm/intent_ir.json --target fsm`
- confirmed the representative explicit FSM end-to-end adapter output is now safely renderable:
  - `lowering_status: renderable`
  - `selected_root_kind: fsm`
  - `signal_candidate_count: 7`
  - `decision_tree_candidate_count: 1`
  - `state_candidate_count: 2`
  - `transition_candidate_count: 2`
  - `residual_decision_count: 2`
  - `emitted_target_path: generated/adapters/fsm/explicit_fsm/explicit_fsm.fsm`
- enriched `SemanticIR` so it now preserves backend-neutral system contract and init-assignment records from explicit `Clock ...`, `Reset ...`, and `Init ...` statements
- enriched `IntentIR` so it now carries canonical system contract and init-assignment surface forward for downstream adapters
- widened the `.fsm` adapter so it now renders explicit standalone sequential `?dt:name` text with `(+system ...)` and `(:= ...)` when the canonical system/init facts are complete
- added a dedicated adapter-side residual for unresolved system/init surface so sequential standalone DT cases stay blocked explicitly instead of inventing reset semantics
- added regression coverage for:
  - explicit system/init extraction in `SemanticIR`
  - canonical system/init carry-through in `IntentIR`
  - renderable and blocked standalone sequential `.fsm` adapter paths
- validated the new standalone sequential slice with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/seq_dt.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/seq_dt/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/seq_dt/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/seq_dt/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/seq_dt/intent_ir.json --target fsm`
- confirmed the representative explicit sequential end-to-end adapter output is now safely renderable:
  - `lowering_status: renderable`
  - `selected_root_kind: dt`
  - `signal_candidate_count: 4`
  - `decision_tree_candidate_count: 1`
  - `residual_decision_count: 4`
  - `emitted_target_path: generated/adapters/fsm/seq_dt/seq_dt.fsm`
- enriched `SemanticIR` so it now preserves typed signal records and backend-neutral guarded/action control fragments when the evidence is explicit enough
- enriched `IntentIR` so it now carries the canonical interface inventory and backend-neutral control fragments forward for downstream adapters
- widened the `.fsm` adapter so it now consumes the canonical interface/control surface instead of relying only on mined prose hints
- the `.fsm` adapter now emits a real standalone `?dt:name` file for explicit canonical cases and keeps broader sequential/system-contract/composition cases blocked instead of inventing semantics
- added regression coverage for:
  - explicit typed-signal/control extraction in `SemanticIR`
  - canonical interface/control carry-through in `IntentIR`
  - blocked and renderable `.fsm` adapter paths
- validated the new canonical/renderable slice with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/comb_dt.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/comb_dt/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/comb_dt/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/comb_dt/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/comb_dt/intent_ir.json --target fsm`
- confirmed the representative explicit end-to-end adapter output is now safely renderable:
  - `lowering_status: renderable`
  - `selected_root_kind: dt`
  - `signal_candidate_count: 3`
  - `decision_tree_candidate_count: 1`
  - `residual_decision_count: 2`
  - `emitted_target_path: generated/adapters/fsm/comb_dt/comb_dt.fsm`
- implemented the first real adapter slice on top of persisted `IntentIR` artifacts
- added `specforge adapt <intent-ir> --target fsm [--dry-run]` to preview or materialize `generated/adapters/fsm/<document_key>/adapter.json`
- replaced the old adapter planning-only scaffolding with a typed adapter artifact model in `crates/specforge/src/ir/adapters.rs`
- the first `.fsm` adapter slice now:
  - loads persisted `IntentIR` JSON from disk
  - selects a conservative DT-oriented root instead of inventing FSM or composition semantics
  - inventories low-confidence signal candidates and DT/state candidate structure from canonical intent records
  - preserves upstream residual decisions and emits adapter-side residuals for missing signal inventory, DT fragments, and broader root-kind expansion
  - blocks emitted `.fsm` text when target syntax would require semantic invention
- added adapter-stage unit tests for:
  - handshake-driven `.fsm` adapter artifact construction
  - wrong-stage input rejection before deserializing as `IntentIR`
- validated the new adapter slice with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/handshake.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/handshake/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/handshake/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/handshake/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/handshake/intent_ir.json --target fsm`
- confirmed the representative end-to-end adapter output is currently honest and blocked rather than fabricated:
  - `lowering_status: blocked`
  - `selected_root_kind: dt`
  - `signal_candidate_count: 2`
  - `decision_tree_candidate_count: 1`
  - `residual_decision_count: 3`
- pivoted the repository objective so `IntentIR` is now the canonical product boundary
- rewrote the core docs around the explicit staged pipeline:
  - `SourceIR`
  - `EvidenceIR`
  - `SemanticIR`
  - `IntentIR`
  - adapters
- added `INTENTIR_SPEC.md` as the canonical architecture/specification document for the new direction
- renamed the active Rust crate and CLI direction from `spec2fsm` to `specforge` with no compatibility aliasing
- renamed the workspace member path to `crates/specforge`
- refactored the Rust code layout around explicit staged IR modules:
  - `crates/specforge/src/ir/source.rs`
  - `crates/specforge/src/ir/evidence.rs`
  - `crates/specforge/src/ir/semantic.rs`
  - `crates/specforge/src/ir/intent.rs`
  - `crates/specforge/src/ir/adapters.rs`
- replaced the previous ingest-manifest framing with a real `SourceIR` artifact
- updated `specforge ingest` so:
  - dry-run prints computed `SourceIR` JSON
  - execute mode materializes `generated/source_ir/<document_key>/source_ir.json`
- formalized a stricter SOTA ingestion stance:
  - structured parser first
  - provenance-preserving page and visual asset capture second
  - selective multimodal enrichment for figures, charts, diagrams, and image-heavy regions third
- extended `SourceIR` scaffolding so it now reserves:
  - parser backend identity
  - page-artifact manifests
  - visual-asset manifests
  - placeholder bindings for normalized sources
- extended `EvidenceIR` scaffolding so it now reserves:
  - multimodal evidence spans
  - visual evidence items
  - text-to-figure links
  - picture-description / OCR-over-image / chart-extraction observations
- recorded adapter targets as downstream of `IntentIR`:
  - `.fsm`
  - SystemVerilog
  - Verilog
  - VHDL
- kept residual decision packets as a first-class mechanism for unresolved automation
- updated the live status tracker so the next highest-priority gap is:
  - close the remaining `SourceIR` structured-PDF-normalization gap and build the first real multimodal `EvidenceIR` extractor
- validated the renamed crate and staged IR refactor with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test`
  - `cargo run -p specforge -- --help`
  - `cargo run -p specforge -- ingest README.md --dry-run`
- confirmed that `specforge ingest README.md --dry-run` now exposes parser backend, page-artifact manifest, visual-asset manifest, and placeholder-binding fields in `SourceIR`
- confirmed the remaining `spec2fsm` mentions are historical continuity references rather than active product naming
- implemented the first real structured PDF normalization backend for `SourceIR`
- added `crates/specforge/src/ir/source/docling_backend.rs` to orchestrate a Docling-backed PDF conversion flow from Rust
- `specforge ingest <pdf>` now materializes:
  - promoted markdown
  - page images and per-page metadata sidecars
  - cropped picture and table assets
  - backend raw JSON and metadata JSON
  - `page_artifacts.json` and `visual_assets.json`
- `SourceIR` PDF execute mode now upgrades its normalization status from `planned_conversion` to `ready` after successful backend materialization
- added a `source_ref` field to visual-asset records so later stages can trace assets back into backend-native structured output
- added runtime dependency guidance:
  - discover `docling` from `python3` or `python`
  - optionally override with `SPECFORGE_DOCLING_PYTHON`
- added a backend-override seam for tests and advanced local integration with `SPECFORGE_DOCLING_HELPER`
- added a stubbed PDF materialization unit test so the real SourceIR backend path is exercised without requiring Docling inside `cargo test`
- validated the new PDF backend with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test`
  - `cargo run -p specforge -- ingest README.md`
  - `SPECFORGE_DOCLING_PYTHON=/tmp/specforge-docling-venv/bin/python cargo run -p specforge -- ingest /tmp/specforge-docling-sample.pdf`
- updated the live status tracker so the remaining top-priority gap is now the first real `EvidenceIR` extractor rather than the SourceIR PDF-normalization backend
- implemented the first real `EvidenceIR` extractor on top of persisted `SourceIR` artifacts
- added `specforge evidence <source-ir> [--dry-run]` to preview or materialize `generated/evidence_ir/<document_key>/evidence_ir.json`
- `EvidenceIR::build` now:
  - loads persisted `SourceIR` JSON from disk
  - requires `normalization_status: ready`
  - parses promoted markdown into section anchors and block-level evidence spans
  - projects `SourceIR` visual assets into typed visual evidence items
  - links caption spans with `describes` and figure/table references with `cites`
  - emits heuristic statement classes for source facts, derived rules, local design decisions, and explicit abstractions
- added stage-artifact loading support and deserialize coverage needed to rebuild `EvidenceIR` from saved `SourceIR` JSON
- added unit tests for:
  - markdown-only `EvidenceIR` construction
  - caption plus figure-reference grounding into visual evidence
- validated the new `EvidenceIR` stage with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run -p specforge -- ingest README.md`
  - `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json --dry-run`
  - `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json`
  - `SPECFORGE_DOCLING_PYTHON=/tmp/specforge-docling-venv/bin/python cargo run -p specforge -- ingest /tmp/specforge-docling-sample.pdf`
  - `cargo run -p specforge -- evidence generated/source_ir/specforge_docling_sample/source_ir.json`
- confirmed live execute-mode outputs for validation:
  - markdown-backed `EvidenceIR`: 14 section anchors, 167 evidence spans, 167 extracted statements
  - PDF-backed `EvidenceIR`: 18 section anchors, 225 evidence spans, 11 visual evidence items, 19 evidence links, 225 extracted statements
- updated the live status tracker so the remaining top-priority gap is now the first real `SemanticIR` constructor rather than the `EvidenceIR` extraction stage
- implemented the first real `SemanticIR` extractor on top of persisted `EvidenceIR` artifacts
- added `specforge semantic <evidence-ir> [--dry-run]` to preview or materialize `generated/semantic_ir/<document_key>/semantic_ir.json`
- `SemanticIR::build` now:
  - loads persisted `EvidenceIR` JSON from disk
  - derives typed semantic artifacts under `generated/semantic_ir/<document_key>/semantic_ir.json`
  - discovers actors, interfaces, phases, invariants, contracts, gates, abstractions, and decomposition candidates from deterministic heuristics
  - emits residual decisions for unresolved actor boundaries, overlapping interface groups, and ambiguous visual semantics
- added stage-artifact loading support needed to rebuild `SemanticIR` from saved `EvidenceIR` JSON
- added unit tests for:
  - handshake-driven actor/interface/invariant extraction
  - ambiguous visual grounding residual decisions
- validated the new `SemanticIR` stage with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run -p specforge -- ingest README.md`
  - `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json`
  - `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json --dry-run`
  - `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json`
  - `SPECFORGE_DOCLING_PYTHON=/tmp/specforge-docling-venv/bin/python cargo run -p specforge -- ingest /tmp/specforge-docling-sample.pdf`
  - `cargo run -p specforge -- evidence generated/source_ir/specforge_docling_sample/source_ir.json`
  - `cargo run -p specforge -- semantic generated/evidence_ir/specforge_docling_sample/evidence_ir.json`
- confirmed live execute-mode outputs for validation:
  - markdown-backed `SemanticIR`: 2 actors, 0 interfaces, 4 phases, 3 invariants, 3 gates, 1 abstraction, 12 decomposition candidates, 0 residual decisions
  - PDF-backed `SemanticIR`: 2 actors, 22 interfaces, 7 phases, 17 invariants, 2 contracts, 20 gates, 12 decomposition candidates, 2 residual decisions
- updated the live status tracker so the remaining top-priority gap is now the first real canonical `IntentIR` constructor rather than the `SemanticIR` stage
- implemented the first real `IntentIR` constructor on top of persisted `SemanticIR` artifacts
- added `specforge intent <semantic-ir> [--dry-run]` to preview or materialize `generated/intent_ir/<document_key>/intent_ir.json`
- `IntentIR::build` now:
  - loads persisted `SemanticIR` JSON from disk
  - derives canonical intent artifacts under `generated/intent_ir/<document_key>/intent_ir.json`
  - canonicalizes actor responsibilities, behaviors, constraints, and assumptions from deterministic heuristics over semantic records
  - preserves semantic residual decisions and emits additional canonicalization residuals only when the intent model would otherwise become speculative
- added unit tests for:
  - handshake-driven intent identity, behavior, constraint, and assumption construction
  - residual-decision preservation from `SemanticIR` into `IntentIR`
- validated the new `IntentIR` stage with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run -p specforge -- ingest README.md`
  - `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json`
  - `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json`
  - `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json --dry-run`
  - `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json`
  - `SPECFORGE_DOCLING_PYTHON=/tmp/specforge-docling-venv/bin/python cargo run -p specforge -- ingest /tmp/specforge-docling-sample.pdf`
  - `cargo run -p specforge -- evidence generated/source_ir/specforge_docling_sample/source_ir.json`
  - `cargo run -p specforge -- semantic generated/evidence_ir/specforge_docling_sample/evidence_ir.json`
  - `cargo run -p specforge -- intent generated/semantic_ir/specforge_docling_sample/semantic_ir.json`
- confirmed live execute-mode outputs for validation:
  - markdown-backed `IntentIR`: 2 actors, 7 behaviors, 3 constraints, 1 assumption, 0 residual decisions
  - PDF-backed `IntentIR`: 2 actors, 28 behaviors, 24 constraints, 1 assumption, 2 residual decisions
- updated the live status tracker so the remaining top-priority gap is now the first real adapter lowering pass rather than the `IntentIR` stage
- added `subs/fsmgen` as a pinned git submodule using the SSH remote `git@github.com:rdje/fsmgen.git`
- pinned the local `fsmgen` reference checkout at submodule revision `57f00e581b4fc9a2aa02318846d1eb8a726c8960`
- updated the live documentation surface so the repo map and continuity notes now treat `subs/fsmgen` as the local `.fsm` reference implementation for upcoming adapter work
- recorded the workflow rule that `subs/fsmgen` is contextual and read-only inside `specforge`
- established the local upstream bug-report ID format `FSMGEN-BUG-####` for any future `fsmgen` misbehavior found during adapter work
- no new `fsmgen` misbehavior was identified in this slice, so no local `FSMGEN-BUG-####` report was filed yet

## 2026-03-31
- initialized the `specforge` Git repository
- established the initial live documentation surface:
  - `README.md`
  - `SESSION_BOOTSTRAP.md`
  - `ROADMAP.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `USER_GUIDE.md`
  - `DEVELOPMENT_NOTES.md`
  - `CHANGES.md`
  - `MEMORY.md`
- defined `README.md` as the single project entry point
- recorded the working project/binary naming:
  - project: `specforge`
  - CLI: `spec2fsm`
- recorded the staged-tool architecture direction and the initial Rust architecture baseline
- updated `COMMIT.md` to reinforce live-document continuity requirements during long-running tasks
- added `.gitignore` rules so local workflow files and build artifacts remain untracked
- created the initial Rust workspace and bootstrap CLI
- established the initial continuity workflow and live-doc surface
- created the first repository baseline commit
