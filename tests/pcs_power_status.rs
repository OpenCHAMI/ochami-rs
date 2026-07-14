//! Wiremock coverage for `Ochami::pcs_power_status_post`.

mod common;
use common::TEST_PEM;
use ochami_rs::Ochami;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn pcs_power_status_post_returns_status() {
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path("/power-control/v1/power-status"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
      "status": [
        {
          "xname": "x1c0s1b0n0",
          "powerState": "on",
          "managementState": "available",
          "supportedPowerTransitions": ["On", "Off"],
          "lastUpdated": "2026-07-14T00:00:00Z",
        }
      ]
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let status = ochami
    .pcs_power_status_post("test-token", Some(&["x1c0s1b0n0"]), None, None)
    .await
    .expect("should succeed");
  assert_eq!(status.status.len(), 1);
  assert_eq!(status.status[0].xname, "x1c0s1b0n0");
}

#[tokio::test]
async fn pcs_power_status_post_surfaces_ochami_error_on_5xx() {
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path("/power-control/v1/power-status"))
    .respond_with(
      ResponseTemplate::new(503).set_body_json(json!({"error": "boom"})),
    )
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let err = ochami
    .pcs_power_status_post("test-token", None, None, None)
    .await
    .expect_err("should fail with 503");
  match err {
    ochami_rs::error::Error::OchamiError { status, .. } => {
      assert_eq!(status, 503);
    }
    other => panic!("expected OchamiError, got {other:?}"),
  }
}

#[tokio::test]
async fn pcs_power_status_post_does_not_retry_on_5xx() {
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path("/power-control/v1/power-status"))
    .respond_with(
      ResponseTemplate::new(503).set_body_json(json!({"error": "boom"})),
    )
    .expect(1)
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let start = std::time::Instant::now();
  let _ = ochami
    .pcs_power_status_post("test-token", None, None, None)
    .await;
  let elapsed = start.elapsed();
  assert!(
    elapsed < std::time::Duration::from_millis(500),
    "elapsed = {elapsed:?} — POST unexpectedly retried"
  );
}
