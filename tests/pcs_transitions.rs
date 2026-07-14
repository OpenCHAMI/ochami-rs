//! Wiremock coverage for `Ochami::pcs_transitions_*` +
//! `pcs_transition_*`.

mod common;
use common::TEST_PEM;
use ochami_rs::Ochami;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn transition_response_json(id: &str, status: &str) -> serde_json::Value {
  json!({
    "transitionID": id,
    "createTime": "2026-07-14T00:00:00Z",
    "automaticExpirationTime": "2026-07-14T01:00:00Z",
    "transitionStatus": status,
    "operation": "On",
    "taskCounts": {
      "total": 1, "new": 0, "in_progress": 0, "failed": 0,
      "succeeded": 1, "un_supported": 0,
    },
    "tasks": [],
  })
}

#[tokio::test]
async fn pcs_transitions_get_returns_vec() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/power-control/v1/transitions"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
      "transitions": [
        transition_response_json("t1", "completed"),
        transition_response_json("t2", "in-progress"),
      ]
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let list = ochami
    .pcs_transitions_get("test-token")
    .await
    .expect("should succeed");
  assert_eq!(list.len(), 2);
  assert_eq!(list[0].transition_id, "t1");
}

#[tokio::test]
async fn pcs_transition_get_returns_one() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/power-control/v1/transitions/t1"))
    .respond_with(
      ResponseTemplate::new(200)
        .set_body_json(transition_response_json("t1", "completed")),
    )
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let one = ochami
    .pcs_transition_get("test-token", "t1")
    .await
    .expect("should succeed");
  assert_eq!(one.transition_id, "t1");
  assert_eq!(one.transition_status, "completed");
}

#[tokio::test]
async fn pcs_transition_delete_returns_body() {
  let server = MockServer::start().await;
  Mock::given(method("DELETE"))
    .and(path("/power-control/v1/transitions/t1"))
    .respond_with(
      ResponseTemplate::new(200).set_body_json(json!({"abortSignaled": true})),
    )
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let body = ochami
    .pcs_transition_delete("test-token", "t1")
    .await
    .expect("should succeed");
  assert_eq!(body["abortSignaled"], json!(true));
}

#[tokio::test]
async fn pcs_transitions_post_creates_transition() {
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path("/power-control/v1/transitions"))
    .respond_with(
      ResponseTemplate::new(200)
        .set_body_json(transition_response_json("new-1", "new")),
    )
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let started = ochami
    .pcs_transitions_post("test-token", "on", &["x1".to_string()])
    .await
    .expect("should succeed");
  assert_eq!(started.transition_id, "new-1");
}

#[tokio::test]
async fn pcs_transitions_post_rejects_invalid_operation() {
  let server = MockServer::start().await;
  // No mock — the invalid operation should error before hitting the
  // wire.
  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let err = ochami
    .pcs_transitions_post("test-token", "not-a-valid-op", &["x1".to_string()])
    .await
    .expect_err("should reject invalid operation");
  match err {
    ochami_rs::error::Error::Message(m) => {
      assert!(m.contains("Operation not valid"), "got: {m}");
    }
    other => panic!("expected Error::Message, got {other:?}"),
  }
}
