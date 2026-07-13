//! wiremock tests for `Ochami::hsm_inventory_hardware_by_fru_*` —
//! `/hsm/v2/Inventory/HardwareByFRU`.
//!
//! One test per wrapper method (Task 9's 3 methods; `delete_all` is
//! intentionally excluded — see `src/hsm/wrapper/inventory_hardware_by_fru.rs`
//! doc comment). Mirrors the Phase 6 wiremock test template established
//! in Task 14's `tests/hsm_component.rs` (design spec Section 4, Phase
//! 6), following Task 19's `tests/hsm_inventory_hardware.rs` pattern.
//!
//! Return type here is the **hand-rolled** `HWInventoryByFRU` struct
//! from `src/hsm/inventory/types.rs` (2767 LOC, kept hand-rolled per
//! design spec § Section 3 — not slim-re-exported from progenitor's
//! typify emission). Unlike `HWInventoryByLocation` (an untagged enum,
//! per Task 19's finding), `HWInventoryByFRU` is a plain struct
//! (`src/hsm/inventory/types.rs:1182`) with fields `FRUID`, `Type`,
//! `FRUSubType` (all optional) and `HWInventoryByFRUType` (required).
//!
//! The `get_one` test exercises the URL fix Task 9 made: the existing
//! (buggy) `hardware_by_fru/http_client.rs::get_one` hit
//! `/Inventory/Hardware/{fruid}` (the HWInventoryByLocation endpoint,
//! a location xname lookup) instead of `/Inventory/HardwareByFRU/{fruid}`
//! (the correct FRU lookup). The mock here is only registered on the
//! corrected path, so a regression back to the old path would 404.
//!
//! Status codes verified against the vendored OpenAPI spec
//! (`src/hsm/ochami_api_docs.yaml`):
//! - `doHWInvByFRUGetAll` (`/Inventory/HardwareByFRU` GET): `200`,
//!   flat array.
//! - `doHWInvByFRUGet` (`/Inventory/HardwareByFRU/{fruid}` GET): `200`,
//!   single `HWInventoryByFRU`.
//! - `doHWInvByFRUDelete` (`/Inventory/HardwareByFRU/{fruid}` DELETE):
//!   `200` with `Response_1.0.0` (`HsmActionResponse`), not `204` —
//!   matching Task 8/9's convention for mutation responses.

mod common;

use common::TEST_PEM;
use ochami_rs::hsm::inventory::types::HWInventoryByFRU;
use ochami_rs::Ochami;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn hsm_inventory_hardware_by_fru_get_returns_array() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/Inventory/HardwareByFRU"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!([{
        "FRUID": "abc123",
        "Type": "Node",
        "FRUSubType": "",
        "HWInventoryByFRUType": "HWInvByFRUNode"
    }])))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let frus = ochami
    .hsm_inventory_hardware_by_fru_get(
      "test-token",
      None,
      Some(&["Node"]),
      None,
      None,
      None,
    )
    .await
    .expect("should succeed");
  assert_eq!(frus.len(), 1);
  let fru: &HWInventoryByFRU = &frus[0];
  assert_eq!(fru.fru_id, Some("abc123".to_string()));
  assert_eq!(fru.r#type, Some("Node".to_string()));
  assert_eq!(fru.hw_inventory_by_fru_type, "HWInvByFRUNode");
}

#[tokio::test]
async fn hsm_inventory_hardware_by_fru_get_one_returns_fru() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/Inventory/HardwareByFRU/abc123"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "FRUID": "abc123",
        "Type": "Node",
        "HWInventoryByFRUType": "HWInvByFRUNode"
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let fru = ochami
    .hsm_inventory_hardware_by_fru_get_one("test-token", "abc123")
    .await
    .expect("should succeed");
  assert_eq!(fru.fru_id, Some("abc123".to_string()));
  assert_eq!(fru.hw_inventory_by_fru_type, "HWInvByFRUNode");
}

#[tokio::test]
async fn hsm_inventory_hardware_by_fru_delete_one_removes_fru() {
  let server = MockServer::start().await;
  Mock::given(method("DELETE"))
    .and(path("/hsm/v2/Inventory/HardwareByFRU/abc123"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "code": "0", "message": "deleted 1 entry"
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let resp = ochami
    .hsm_inventory_hardware_by_fru_delete_one("test-token", "abc123")
    .await
    .expect("should succeed");
  assert_eq!(resp.code, "0");
  assert_eq!(resp.message, "deleted 1 entry");
}
