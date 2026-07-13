//! wiremock tests for `Ochami::hsm_component_*` — `/hsm/v2/State/Components`.
//!
//! One test per wrapper method (Task 3's 12 methods) plus two
//! retry-behavior tests exercising `retry_on_5xx` (`src/http.rs`).
//! This file establishes the Phase 6 wiremock test template reused by
//! Tasks 15-22 (design spec Section 4).

mod common;

use common::TEST_PEM;
use ochami_rs::hsm::component::types::{
  ComponentArrayPostArray, ComponentArrayPostByNidQuery,
  ComponentArrayPostQuery, ComponentCreate, ComponentPut, HmsState100,
  HmsType100, NidRange100, XNameRw100,
};
use ochami_rs::Ochami;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn component_create(xname: &str) -> ComponentCreate {
  ComponentCreate {
    arch: None,
    class: None,
    enabled: None,
    flag: None,
    id: XNameRw100(xname.to_string()),
    net_type: None,
    nid: None,
    role: None,
    software_status: None,
    state: HmsState100::Ready,
    sub_role: None,
    subtype: None,
  }
}

// ---------------------------------------------------------------------
// Method tests (one per `Ochami::hsm_component_*` wrapper method).
// ---------------------------------------------------------------------

#[tokio::test]
async fn hsm_component_get_all_returns_component_array() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/State/Components"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "Components": [{"ID": "x1c0s1b0n0", "Type": "Node", "State": "Ready"}]
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let comps = ochami
    .hsm_component_get_all("test-token", None)
    .await
    .expect("should succeed");
  assert_eq!(comps.components.len(), 1);
  assert_eq!(comps.components[0].id.as_ref().unwrap().0, "x1c0s1b0n0");
  assert!(matches!(comps.components[0].type_, Some(HmsType100::Node)));
}

#[tokio::test]
async fn hsm_component_get_all_nodes_returns_component_array() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/State/Components"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "Components": [{"ID": "x1c0s1b0n1", "Type": "Node", "State": "On"}]
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let comps = ochami
    .hsm_component_get_all_nodes("test-token", None)
    .await
    .expect("should succeed");
  assert_eq!(comps.components.len(), 1);
  assert_eq!(comps.components[0].id.as_ref().unwrap().0, "x1c0s1b0n1");
  assert!(matches!(comps.components[0].state, Some(HmsState100::On)));
}

#[tokio::test]
async fn hsm_component_get_returns_component_array() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/State/Components"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "Components": [{"ID": "x1c0s1b0n0", "Type": "Node", "State": "Ready", "NID": 100}]
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let comps = ochami
    .hsm_component_get(
      "test-token",
      None,
      Some("Node"),
      None,
      None,
      None,
      None,
      None,
      None,
      None,
      None,
      None,
      Some("100,101"),
      None,
      None,
      None,
      None,
      None,
      None,
      None,
      None,
    )
    .await
    .expect("should succeed");
  assert_eq!(comps.components.len(), 1);
  assert_eq!(comps.components[0].nid, Some(100));
}

#[tokio::test]
async fn hsm_component_get_query_returns_component_array() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/State/Components/Query/x1c0s1b0n0"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "Components": [{"ID": "x1c0s1b0n0", "Type": "Node", "State": "Ready"}]
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let comps = ochami
    .hsm_component_get_query(
      "test-token",
      "x1c0s1b0n0",
      None,
      None,
      None,
      None,
      None,
      None,
      None,
      None,
      None,
      None,
      None,
      None,
      None,
      None,
      None,
      None,
      None,
      None,
      None,
    )
    .await
    .expect("should succeed");
  assert_eq!(comps.components.len(), 1);
  assert_eq!(comps.components[0].id.as_ref().unwrap().0, "x1c0s1b0n0");
}

#[tokio::test]
async fn hsm_component_get_one_returns_component() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/State/Components/x1c0s1b0n0"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "ID": "x1c0s1b0n0", "Type": "Node", "State": "Ready",
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let comp = ochami
    .hsm_component_get_one("test-token", "x1c0s1b0n0")
    .await
    .expect("should succeed");
  assert_eq!(comp.id.as_ref().unwrap().0, "x1c0s1b0n0");
  assert!(matches!(comp.type_, Some(HmsType100::Node)));
}

#[tokio::test]
async fn hsm_component_get_by_nid_returns_component() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/State/Components/ByNID/100"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "ID": "x1c0s1b0n0", "Type": "Node", "State": "Ready", "NID": 100,
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let comp = ochami
    .hsm_component_get_by_nid("test-token", "100")
    .await
    .expect("should succeed");
  assert_eq!(comp.nid, Some(100));
}

#[tokio::test]
async fn hsm_component_post_succeeds() {
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path("/hsm/v2/State/Components"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(204))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let body = ComponentArrayPostArray {
    components: vec![component_create("x1c0s1b0n0")],
    force: None,
  };
  ochami
    .hsm_component_post("test-token", body)
    .await
    .expect("should succeed");
}

#[tokio::test]
async fn hsm_component_post_query_returns_component_array() {
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path("/hsm/v2/State/Components/Query"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "Components": [{"ID": "x1c0s1b0n0", "Type": "Node", "State": "Ready"}]
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let body = ComponentArrayPostQuery {
    arch: vec![],
    class: vec![],
    component_i_ds: vec![],
    enabled: vec![],
    flag: vec![],
    flagonly: None,
    group: None,
    nid: vec![],
    nid_end: vec![],
    nid_start: vec![],
    nidonly: None,
    partition: None,
    role: vec![],
    roleonly: None,
    softwarestatus: vec![],
    state: vec![],
    stateonly: None,
    subrole: vec![],
    subtype: vec![],
    type_: vec![],
  };
  let comps = ochami
    .hsm_component_post_query("test-token", body)
    .await
    .expect("should succeed");
  assert_eq!(comps.components.len(), 1);
  assert_eq!(comps.components[0].id.as_ref().unwrap().0, "x1c0s1b0n0");
}

#[tokio::test]
async fn hsm_component_post_bynid_query_returns_component_array() {
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path("/hsm/v2/State/Components/ByNID/Query"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "Components": [{"ID": "x1c0s1b0n0", "Type": "Node", "State": "Ready", "NID": 0}]
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let body = ComponentArrayPostByNidQuery {
    flagonly: None,
    nid_ranges: vec![NidRange100("0-1".to_string())],
    nidonly: None,
    partition: None,
    roleonly: None,
    stateonly: None,
  };
  let comps = ochami
    .hsm_component_post_bynid_query("test-token", body)
    .await
    .expect("should succeed");
  assert_eq!(comps.components.len(), 1);
  assert_eq!(comps.components[0].nid, Some(0));
}

#[tokio::test]
async fn hsm_component_put_succeeds() {
  let server = MockServer::start().await;
  Mock::given(method("PUT"))
    .and(path("/hsm/v2/State/Components/x1c0s1b0n0"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(204))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let body = ComponentPut {
    component: component_create("x1c0s1b0n0"),
    force: None,
  };
  ochami
    .hsm_component_put("test-token", "x1c0s1b0n0", body)
    .await
    .expect("should succeed");
}

#[tokio::test]
async fn hsm_component_delete_one_returns_action_response() {
  let server = MockServer::start().await;
  Mock::given(method("DELETE"))
    .and(path("/hsm/v2/State/Components/x1c0s1b0n0"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "code": "0", "message": "deleted 1 component"
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let resp = ochami
    .hsm_component_delete_one("test-token", "x1c0s1b0n0")
    .await
    .expect("should succeed");
  assert_eq!(resp.code, "0");
  assert_eq!(resp.message, "deleted 1 component");
}

#[tokio::test]
async fn hsm_component_delete_returns_action_response() {
  let server = MockServer::start().await;
  Mock::given(method("DELETE"))
    .and(path("/hsm/v2/State/Components"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "code": "0", "message": "deleted all components"
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let resp = ochami
    .hsm_component_delete("test-token")
    .await
    .expect("should succeed");
  assert_eq!(resp.code, "0");
  assert_eq!(resp.message, "deleted all components");
}

// ---------------------------------------------------------------------
// Retry-behavior tests (`retry_on_5xx` in `src/http.rs`).
// ---------------------------------------------------------------------

#[tokio::test]
async fn hsm_component_get_one_retries_on_5xx() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/State/Components/x1"))
    .respond_with(
      ResponseTemplate::new(503).set_body_json(json!({"error": "boom"})),
    )
    .up_to_n_times(2)
    .mount(&server)
    .await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/State/Components/x1"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ID": "x1"})))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let start = std::time::Instant::now();
  let comp = ochami
    .hsm_component_get_one("test-token", "x1")
    .await
    .expect("should eventually succeed");
  let elapsed = start.elapsed();
  assert_eq!(comp.id.as_ref().unwrap().0, "x1");
  assert!(
    elapsed >= std::time::Duration::from_millis(1400),
    "expected >= 1.4s of backoff, got {elapsed:?}"
  );
}

#[tokio::test]
async fn hsm_component_get_one_gives_up_after_3_5xx() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/State/Components/x1"))
    .respond_with(
      ResponseTemplate::new(503).set_body_json(json!({"error": "boom"})),
    )
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let start = std::time::Instant::now();
  let result = ochami.hsm_component_get_one("test-token", "x1").await;
  let elapsed = start.elapsed();

  match result {
    Err(ochami_rs::error::Error::OchamiError { status, .. }) => {
      assert_eq!(status, 503);
    }
    other => {
      panic!("expected Error::OchamiError{{status: 503, ..}}, got {other:?}")
    }
  }
  assert!(
    elapsed >= std::time::Duration::from_millis(1400),
    "expected >= 1.4s of backoff before giving up, got {elapsed:?}"
  );
}
