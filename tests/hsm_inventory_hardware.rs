//! wiremock tests for `Ochami::hsm_inventory_hardware_*` —
//! `/hsm/v2/Inventory/Hardware`.
//!
//! One test per wrapper method (Task 8's 4 methods; `delete_all` is
//! intentionally excluded — see `src/hsm/wrapper/inventory_hardware.rs`
//! doc comment). Mirrors the Phase 6 wiremock test template established
//! in Task 14's `tests/hsm_component.rs` (design spec Section 4, Phase
//! 6).
//!
//! Return types here are the **hand-rolled** `src/hsm/inventory/types.rs`
//! tree (2767 LOC, kept hand-rolled per design spec § Section 3 — not
//! slim-re-exported from progenitor's typify emission). Notably
//! `HWInventoryByLocation` is an `#[serde(untagged)]` enum wrapping
//! per-subtype structs (`HWInvByLocNode`, `HWInvByLocProcessor`, ...),
//! not a plain struct — assertions match on the enum variant.
//!
//! Status code verified against the vendored OpenAPI spec
//! (`src/hsm/ochami_api_docs.yaml`, `/Inventory/Hardware` `post:`):
//! `doHWInvByLocationPost` documents a single `200` success response
//! (`Response_1.0.0` / `HsmActionResponse`), unlike the `201` used by
//! `/hsm/v2/groups` and `/hsm/v2/partitions` POSTs.

mod common;

use common::TEST_PEM;
use ochami_rs::hsm::inventory::types::{
  HWInvByLocNode, HWInventory, HWInventoryByLocation, HWInventoryByLocationList,
};
use ochami_rs::Ochami;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn sample_node(xname: &str) -> HWInvByLocNode {
  HWInvByLocNode {
    id: xname.to_string(),
    r#type: Some("Node".to_string()),
    ordinal: None,
    status: None,
    hw_inventory_by_location_type: "HWInvByLocNode".to_string(),
    populated_fru: None,
    node_location_info: None,
    processors: None,
    node_accels: None,
    drives: None,
    memory: None,
    node_accel_risers: None,
    node_hsn_nics: None,
  }
}

// ---------------------------------------------------------------------
// Method tests (one per `Ochami::hsm_inventory_hardware_*` wrapper
// method).
// ---------------------------------------------------------------------

#[tokio::test]
async fn hsm_inventory_hardware_get_returns_array() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/Inventory/Hardware"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!([{
        "ID": "x1c0s1b0n0",
        "Type": "Node",
        "HWInventoryByLocationType": "HWInvByLocNode"
    }])))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let hardware = ochami
    .hsm_inventory_hardware_get(
      "test-token",
      Some(&["x1"]),
      Some(&["Node"]),
      None,
      None,
      None,
      None,
    )
    .await
    .expect("should succeed");
  assert_eq!(hardware.len(), 1);
  match &hardware[0] {
    HWInventoryByLocation::HWInvByLocNode(node) => {
      assert_eq!(node.id, "x1c0s1b0n0");
      assert_eq!(node.r#type, Some("Node".to_string()));
    }
    other => panic!("expected HWInvByLocNode variant, got {other:?}"),
  }
}

#[tokio::test]
async fn hsm_inventory_hardware_get_one_returns_hardware() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/Inventory/Hardware/x1c0s1b0n0"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "ID": "x1c0s1b0n0",
        "Type": "Node",
        "HWInventoryByLocationType": "HWInvByLocNode"
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let hardware = ochami
    .hsm_inventory_hardware_get_one("test-token", "x1c0s1b0n0")
    .await
    .expect("should succeed");
  match hardware {
    HWInventoryByLocation::HWInvByLocNode(node) => {
      assert_eq!(node.id, "x1c0s1b0n0");
    }
    other => panic!("expected HWInvByLocNode variant, got {other:?}"),
  }
}

#[tokio::test]
async fn hsm_inventory_hardware_get_query_returns_hierarchy() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/Inventory/Hardware/Query/x1c0"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "XName": "x1c0",
        "Nodes": [{
            "ID": "x1c0s1b0n0",
            "Type": "Node",
            "HWInventoryByLocationType": "HWInvByLocNode"
        }]
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let hierarchy: HWInventory = ochami
    .hsm_inventory_hardware_get_query(
      "test-token",
      "x1c0",
      None,
      Some(true),
      None,
      None,
      None,
    )
    .await
    .expect("should succeed");
  assert_eq!(hierarchy.xname, Some("x1c0".to_string()));
  let nodes = hierarchy.nodes.expect("expected Nodes array");
  assert_eq!(nodes.len(), 1);
  assert_eq!(nodes[0].id, "x1c0s1b0n0");
}

#[tokio::test]
async fn hsm_inventory_hardware_post_creates_hardware() {
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path("/hsm/v2/Inventory/Hardware"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "code": "0", "message": "1 entries created or updated"
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let hardware = HWInventoryByLocationList {
    hardware: Some(vec![HWInventoryByLocation::HWInvByLocNode(sample_node(
      "x1c0s1b0n0",
    ))]),
  };
  let resp = ochami
    .hsm_inventory_hardware_post("test-token", &hardware)
    .await
    .expect("should succeed");
  assert_eq!(resp.code, "0");
  assert_eq!(resp.message, "1 entries created or updated");
}
