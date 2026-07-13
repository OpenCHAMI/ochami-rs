//! wiremock tests for `Ochami::hsm_group_*` — `/hsm/v2/groups`.
//!
//! One test per wrapper method (Task 4's 9 methods) plus a
//! POST-no-retry test verifying `hsm_group_post` does not go through
//! `retry_on_5xx` (`src/http.rs`). Mirrors the Phase 6 wiremock test
//! template established in Task 14's `tests/hsm_component.rs` (design
//! spec Section 4, Phase 6).

mod common;

use common::TEST_PEM;
use ochami_rs::hsm::group::types::{Group, GroupExt};
use ochami_rs::Ochami;
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn sample_group() -> Group {
  Group::new("test", None)
}

// ---------------------------------------------------------------------
// Method tests (one per `Ochami::hsm_group_*` wrapper method).
// ---------------------------------------------------------------------

#[tokio::test]
async fn hsm_group_get_all_returns_group_array() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/groups"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(
      ResponseTemplate::new(200)
        .set_body_json(json!([{"label": "compute"}])),
    )
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let groups = ochami
    .hsm_group_get_all("test-token")
    .await
    .expect("should succeed");
  assert_eq!(groups.len(), 1);
  assert_eq!(&*groups[0].label, "compute");
}

#[tokio::test]
async fn hsm_group_get_returns_filtered() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/groups"))
    .and(header("authorization", "Bearer test-token"))
    .and(query_param("group", "compute"))
    .and(query_param("group", "worker"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!([
        {"label": "compute"},
        {"label": "worker"},
    ])))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let groups = ochami
    .hsm_group_get("test-token", Some(&["compute", "worker"]), None)
    .await
    .expect("should succeed");
  assert_eq!(groups.len(), 2);
}

#[tokio::test]
async fn hsm_group_get_one_returns_group() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/groups/compute"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(
      ResponseTemplate::new(200).set_body_json(json!({"label": "compute"})),
    )
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let group = ochami
    .hsm_group_get_one("test-token", "compute")
    .await
    .expect("should succeed");
  assert_eq!(&*group.label, "compute");
}

#[tokio::test]
async fn hsm_group_get_labels_returns_names() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/groups/labels"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(
      ResponseTemplate::new(200)
        .set_body_json(json!(["compute", "worker"])),
    )
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let labels = ochami
    .hsm_group_get_labels("test-token")
    .await
    .expect("should succeed");
  assert_eq!(labels, vec!["compute".to_string(), "worker".to_string()]);
}

#[tokio::test]
async fn hsm_group_get_members_returns_ids() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/groups/compute/members"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(
      ResponseTemplate::new(200)
        .set_body_json(json!({"ids": ["x1", "x2"]})),
    )
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let members = ochami
    .hsm_group_get_members("test-token", "compute")
    .await
    .expect("should succeed");
  assert_eq!(members.ids.len(), 2);
  assert_eq!(members.ids[0].0, "x1");
  assert_eq!(members.ids[1].0, "x2");
}

#[tokio::test]
async fn hsm_group_post_creates_group() {
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path("/hsm/v2/groups"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(201).set_body_json(json!([
        {"ResourceURI": "/hsm/v2/groups/compute"}
    ])))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let group = sample_group();
  let uris = ochami
    .hsm_group_post("test-token", &group)
    .await
    .expect("should succeed");
  assert_eq!(uris.len(), 1);
  assert_eq!(
    uris[0].resource_uri.as_deref(),
    Some("/hsm/v2/groups/compute")
  );
}

#[tokio::test]
async fn hsm_group_post_member_adds_xname() {
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path("/hsm/v2/groups/compute/members"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(201).set_body_json(json!([
        {"ResourceURI": "/hsm/v2/groups/compute/members/x1"}
    ])))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let uris = ochami
    .hsm_group_post_member("test-token", "compute", "x1")
    .await
    .expect("should succeed");
  assert_eq!(uris.len(), 1);
  assert_eq!(
    uris[0].resource_uri.as_deref(),
    Some("/hsm/v2/groups/compute/members/x1")
  );
}

#[tokio::test]
async fn hsm_group_delete_one_removes_group() {
  let server = MockServer::start().await;
  Mock::given(method("DELETE"))
    .and(path("/hsm/v2/groups/compute"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "code": "0", "message": "deleted"
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let resp = ochami
    .hsm_group_delete_one("test-token", "compute")
    .await
    .expect("should succeed");
  assert_eq!(resp.code, "0");
  assert_eq!(resp.message, "deleted");
}

#[tokio::test]
async fn hsm_group_delete_member_removes_xname() {
  let server = MockServer::start().await;
  Mock::given(method("DELETE"))
    .and(path("/hsm/v2/groups/compute/members/x1"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "code": "0", "message": "deleted"
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let resp = ochami
    .hsm_group_delete_member("test-token", "compute", "x1")
    .await
    .expect("should succeed");
  assert_eq!(resp.code, "0");
  assert_eq!(resp.message, "deleted");
}

// ---------------------------------------------------------------------
// POST-no-retry test: `hsm_group_post` routes through `run()`, not
// `retry_on_5xx` — a 503 should surface as an `Err` immediately, with
// exactly one request reaching the server.
// ---------------------------------------------------------------------

#[tokio::test]
async fn hsm_group_post_does_not_retry_on_5xx() {
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path("/hsm/v2/groups"))
    .respond_with(
      ResponseTemplate::new(503).set_body_json(json!({"error": "boom"})),
    )
    .expect(1)
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let group = sample_group();
  let start = std::time::Instant::now();
  let result = ochami.hsm_group_post("test-token", &group).await;
  let elapsed = start.elapsed();

  assert!(result.is_err());
  assert!(
    elapsed < std::time::Duration::from_millis(500),
    "expected no retry backoff, got {elapsed:?}"
  );
  // `.expect(1)` on the Mock above is verified when `server` drops.
}
