//! Wiremock coverage for `Ochami::pcs_power_cap_*`.

mod common;
use common::TEST_PEM;
use ochami_rs::pcs::power_cap::types::PowerCapComponent;
use ochami_rs::Ochami;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn pcs_power_cap_get_returns_task_info() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/power-control/v1/power-cap"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
      "taskId": "task-1",
      "type": "snapshot",
      "taskStatus": "completed",
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let info = ochami
    .pcs_power_cap_get("test-token")
    .await
    .expect("should succeed");
  assert_eq!(info.task_id.as_deref(), Some("task-1"));
  assert_eq!(info.task_status.as_deref(), Some("completed"));
}

#[tokio::test]
async fn pcs_power_cap_task_get_returns_task_info() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/power-control/v1/power-cap/task-42"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
      "taskId": "task-42",
      "type": "patch",
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let info = ochami
    .pcs_power_cap_task_get("test-token", "task-42")
    .await
    .expect("should succeed");
  assert_eq!(info.task_id.as_deref(), Some("task-42"));
}

#[tokio::test]
async fn pcs_power_cap_snapshot_post_uses_post_not_put() {
  // Regression guard: the pre-migration hand-written `post_snapshot`
  // called `.put()`. Only accept POST.
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path("/power-control/v1/power-cap/snapshot"))
    .respond_with(
      ResponseTemplate::new(200).set_body_json(json!({"taskId": "s1"})),
    )
    .expect(1)
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let info = ochami
    .pcs_power_cap_snapshot_post("test-token", &["x1", "x2"])
    .await
    .expect("should succeed");
  assert_eq!(info.task_id.as_deref(), Some("s1"));
}

#[tokio::test]
async fn pcs_power_cap_patch_hits_correct_endpoint() {
  // Regression guard: the pre-migration hand-written `patch` called
  // `.put()` on `/power-cap/snapshot` (both wrong). Assert
  // PATCH on `/power-cap`.
  let server = MockServer::start().await;
  Mock::given(method("PATCH"))
    .and(path("/power-control/v1/power-cap"))
    .respond_with(
      ResponseTemplate::new(200).set_body_json(json!({"taskId": "p1"})),
    )
    .expect(1)
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let comps = vec![PowerCapComponent {
    xname: Some("x1c0s1b0n0".to_string()),
    error: None,
    limits: None,
    power_cap_limits: None,
  }];
  let info = ochami
    .pcs_power_cap_patch("test-token", &comps)
    .await
    .expect("should succeed");
  assert_eq!(info.task_id.as_deref(), Some("p1"));
}
