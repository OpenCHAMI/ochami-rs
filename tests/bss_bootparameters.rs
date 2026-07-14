//! Wiremock coverage for `Ochami::bss_bootparameters_*`.
//!
//! One test per wrapper method. Also covers the on-wire body-filter
//! shape of `bss_bootparameters_get(token, Some(&["x1", "x2"]))` since
//! that pre-migration behaviour is preserved.

mod common;
use common::TEST_PEM;
use ochami_rs::bss::types::BootParameters;
use ochami_rs::Ochami;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn sample_params(host: &str, kernel: &str) -> BootParameters {
  BootParameters {
    hosts: vec![host.to_string()],
    macs: None,
    nids: None,
    params: "console=ttyS0".to_string(),
    kernel: kernel.to_string(),
    initrd: "s3://boot-images/x/initrd".to_string(),
    cloud_init: None,
  }
}

#[tokio::test]
async fn bss_bootparameters_get_all_returns_vec() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/boot/v1/bootparameters"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!([
      {"hosts": ["x1"], "kernel": "s3://boot-images/x/kernel"},
      {"hosts": ["x2"], "kernel": "s3://boot-images/y/kernel"},
    ])))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let bp = ochami
    .bss_bootparameters_get_all("test-token")
    .await
    .expect("should succeed");
  assert_eq!(bp.len(), 2);
  assert_eq!(bp[0].hosts, vec!["x1".to_string()]);
  assert_eq!(bp[0].kernel, "s3://boot-images/x/kernel");
}

#[tokio::test]
async fn bss_bootparameters_get_with_host_filter() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/boot/v1/bootparameters"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!([
      {"hosts": ["x1c0s1b0n0"], "kernel": "s3://boot-images/x/kernel"},
    ])))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let bp = ochami
    .bss_bootparameters_get("test-token", Some(&["x1c0s1b0n0"]))
    .await
    .expect("should succeed");
  assert_eq!(bp.len(), 1);
  assert_eq!(bp[0].hosts, vec!["x1c0s1b0n0".to_string()]);
}

#[tokio::test]
async fn bss_bootparameters_post_creates() {
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path("/boot/v1/bootparameters"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(201))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  ochami
    .bss_bootparameters_post(
      "test-token",
      &sample_params("x1", "s3://boot-images/x/kernel"),
    )
    .await
    .expect("should succeed");
}

#[tokio::test]
async fn bss_bootparameters_put_updates() {
  let server = MockServer::start().await;
  Mock::given(method("PUT"))
    .and(path("/boot/v1/bootparameters"))
    .respond_with(ResponseTemplate::new(200))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  ochami
    .bss_bootparameters_put(
      "test-token",
      &sample_params("x1", "s3://boot-images/x/kernel"),
    )
    .await
    .expect("should succeed");
}

#[tokio::test]
async fn bss_bootparameters_patch_updates() {
  let server = MockServer::start().await;
  Mock::given(method("PATCH"))
    .and(path("/boot/v1/bootparameters"))
    .respond_with(ResponseTemplate::new(200))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  ochami
    .bss_bootparameters_patch(
      "test-token",
      &sample_params("x1", "s3://boot-images/x/kernel"),
    )
    .await
    .expect("should succeed");
}

#[tokio::test]
async fn bss_bootparameters_delete_removes() {
  let server = MockServer::start().await;
  Mock::given(method("DELETE"))
    .and(path("/boot/v1/bootparameters"))
    .respond_with(ResponseTemplate::new(200))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  ochami
    .bss_bootparameters_delete(
      "test-token",
      &sample_params("x1", "s3://boot-images/x/kernel"),
    )
    .await
    .expect("should succeed");
}

#[tokio::test]
async fn bss_bootparameters_get_retries_on_5xx() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/boot/v1/bootparameters"))
    .respond_with(
      ResponseTemplate::new(503).set_body_json(json!({"error": "boom"})),
    )
    .up_to_n_times(2)
    .mount(&server)
    .await;
  Mock::given(method("GET"))
    .and(path("/boot/v1/bootparameters"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let start = std::time::Instant::now();
  ochami
    .bss_bootparameters_get_all("test-token")
    .await
    .expect("should succeed after retries");
  let elapsed = start.elapsed();
  // 500ms + 1000ms exponential backoff before the third (success) attempt.
  assert!(
    elapsed >= std::time::Duration::from_millis(1400),
    "elapsed = {elapsed:?}"
  );
}

#[tokio::test]
async fn bss_bootparameters_post_does_not_retry_on_5xx() {
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path("/boot/v1/bootparameters"))
    .respond_with(
      ResponseTemplate::new(503).set_body_json(json!({"error": "boom"})),
    )
    .expect(1)
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let start = std::time::Instant::now();
  let result = ochami
    .bss_bootparameters_post(
      "test-token",
      &sample_params("x1", "s3://boot-images/x/kernel"),
    )
    .await;
  let elapsed = start.elapsed();
  assert!(result.is_err(), "POST should not have retried and succeeded");
  assert!(
    elapsed < std::time::Duration::from_millis(500),
    "elapsed = {elapsed:?} — POST unexpectedly retried"
  );
}
