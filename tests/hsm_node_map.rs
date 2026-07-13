//! wiremock tests for `Ochami::hsm_node_map_*` — `/hsm/v2/Defaults/NodeMaps`.
//!
//! One test per wrapper method (Task 6's 6 methods — the generated
//! client collapses the design spec's separate single-item POST and
//! bulk POST into a single array-shaped `do_node_map_post`, so there
//! is no 7th "bulk" method to test; see `src/hsm/wrapper/node_map.rs`
//! module docs). Mirrors the Phase 6 wiremock test template
//! established in Task 14's `tests/hsm_component.rs` and Task 15's
//! `tests/hsm_group.rs` (design spec Section 4, Phase 6).
//!
//! Status codes: the generated `hsm_generated.rs` `do_node_map*`
//! methods each match on a single `200u16` success arm (no 201/204
//! variants), so every mock below responds with `200`.

mod common;

use common::TEST_PEM;
// `XNameForQuery100` is re-exported via the `component` types module
// (it is a shared generated newtype, not node-map-specific); `NodeMap`,
// `NodeMapArray`, `NodeMapPost` are node-map-specific re-exports.
use ochami_rs::hsm::component::types::XNameForQuery100;
use ochami_rs::hsm::node_map::types::{NodeMap, NodeMapArray, NodeMapPost};
use ochami_rs::Ochami;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ---------------------------------------------------------------------
// Method tests (one per `Ochami::hsm_node_map_*` wrapper method).
// ---------------------------------------------------------------------

#[tokio::test]
async fn hsm_node_map_get_all_returns_array() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/Defaults/NodeMaps"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "NodeMaps": [{"ID": "x1c0s1b0n0", "NID": 1}]
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let node_maps: NodeMapArray = ochami
    .hsm_node_map_get_all("test-token")
    .await
    .expect("should succeed");
  assert_eq!(node_maps.node_maps.len(), 1);
  assert_eq!(&*node_maps.node_maps[0].id, "x1c0s1b0n0");
  assert_eq!(node_maps.node_maps[0].nid, 1);
}

#[tokio::test]
async fn hsm_node_map_get_one_returns_map() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/Defaults/NodeMaps/x1c0s1b0n0"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "ID": "x1c0s1b0n0", "NID": 1
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let node_map: NodeMap = ochami
    .hsm_node_map_get_one("test-token", "x1c0s1b0n0")
    .await
    .expect("should succeed");
  assert_eq!(
    node_map.id.as_ref().map(|x| x.0.as_str()),
    Some("x1c0s1b0n0")
  );
  assert_eq!(node_map.nid, 1);
}

#[tokio::test]
async fn hsm_node_map_post_creates_maps() {
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path("/hsm/v2/Defaults/NodeMaps"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "code": "0", "message": "created"
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let node_map_array = NodeMapArray {
    node_maps: vec![NodeMapPost {
      id: XNameForQuery100("x1c0s1b0n0".to_string()),
      nid: 1,
      role: None,
      sub_role: None,
    }],
  };
  let resp = ochami
    .hsm_node_map_post("test-token", &node_map_array)
    .await
    .expect("should succeed");
  assert_eq!(resp.code, "0");
  assert_eq!(resp.message, "created");
}

#[tokio::test]
async fn hsm_node_map_put_updates_map() {
  let server = MockServer::start().await;
  Mock::given(method("PUT"))
    .and(path("/hsm/v2/Defaults/NodeMaps/x1c0s1b0n0"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "ID": "x1c0s1b0n0", "NID": 2
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let node_map = NodeMap {
    id: None,
    nid: 2,
    role: None,
    sub_role: None,
  };
  // Task 6 finding: `hsm_node_map_put` returns a typed `NodeMapPost`
  // (aliased from `NodeMap100PostNodeMap`), not `HsmActionResponse`.
  let updated: NodeMapPost = ochami
    .hsm_node_map_put("test-token", "x1c0s1b0n0", &node_map)
    .await
    .expect("should succeed");
  assert_eq!(&*updated.id, "x1c0s1b0n0");
  assert_eq!(updated.nid, 2);
}

#[tokio::test]
async fn hsm_node_map_delete_all_removes_all() {
  let server = MockServer::start().await;
  Mock::given(method("DELETE"))
    .and(path("/hsm/v2/Defaults/NodeMaps"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "code": "0", "message": "deleted all"
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let resp = ochami
    .hsm_node_map_delete_all("test-token")
    .await
    .expect("should succeed");
  assert_eq!(resp.code, "0");
  assert_eq!(resp.message, "deleted all");
}

#[tokio::test]
async fn hsm_node_map_delete_one_removes_map() {
  let server = MockServer::start().await;
  Mock::given(method("DELETE"))
    .and(path("/hsm/v2/Defaults/NodeMaps/x1c0s1b0n0"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "code": "0", "message": "deleted"
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let resp = ochami
    .hsm_node_map_delete_one("test-token", "x1c0s1b0n0")
    .await
    .expect("should succeed");
  assert_eq!(resp.code, "0");
  assert_eq!(resp.message, "deleted");
}
