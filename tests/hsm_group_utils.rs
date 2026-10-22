//! Wiremock coverage for `hsm::group::utils::update_hsm_group_members`.
//!
//! The function was a documented no-op in earlier releases; this
//! suite regression-guards the actual per-xname
//! post_member/delete_member implementation.

mod common;
use common::TEST_PEM;
use ochami_rs::hsm::group::utils::update_hsm_group_members;
use ochami_rs::Ochami;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn update_hsm_group_members_deletes_then_adds() {
  let server = MockServer::start().await;
  // Deletion of x1 comes first.
  Mock::given(method("DELETE"))
    .and(path("/hsm/v2/groups/compute/members/x1"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
      "code": "0", "message": "removed"
    })))
    .expect(1)
    .mount(&server)
    .await;
  // Then addition of x2.
  Mock::given(method("POST"))
    .and(path("/hsm/v2/groups/compute/members"))
    .respond_with(ResponseTemplate::new(201).set_body_json(json!([
      {"ResourceURI": "/hsm/v2/groups/compute/members/x2"}
    ])))
    .expect(1)
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  update_hsm_group_members(&ochami, "test-token", "compute", &["x1"], &["x2"])
    .await
    .expect("should succeed");
  // wiremock's `.expect(1)` above will fail the mount drop if the
  // request count didn't match.
}

#[tokio::test]
async fn update_hsm_group_members_returns_error_summary_on_partial_failure() {
  let server = MockServer::start().await;
  // First delete succeeds.
  Mock::given(method("DELETE"))
    .and(path("/hsm/v2/groups/compute/members/x1"))
    .respond_with(
      ResponseTemplate::new(200)
        .set_body_json(json!({"code":"0","message":""})),
    )
    .mount(&server)
    .await;
  // Add fails with 409 conflict — should be logged + collected but
  // not propagated as an early return.
  Mock::given(method("POST"))
    .and(path("/hsm/v2/groups/compute/members"))
    .respond_with(
      ResponseTemplate::new(409).set_body_json(json!({"detail": "exists"})),
    )
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let err = update_hsm_group_members(
    &ochami,
    "test-token",
    "compute",
    &["x1"],
    &["x2"],
  )
  .await
  .expect_err("should surface the failure");
  match err {
    ochami_rs::error::Error::Message(m) => {
      assert!(m.contains("1 of 2 operations failed"), "got: {m}");
      assert!(m.contains("post x2"), "got: {m}");
    }
    other => panic!("expected Error::Message, got {other:?}"),
  }
}

#[tokio::test]
async fn update_hsm_group_members_empty_lists_is_no_op_ok() {
  // No mocks — the function should not hit the wire at all.
  let server = MockServer::start().await;
  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  update_hsm_group_members(&ochami, "test-token", "compute", &[], &[])
    .await
    .expect("empty lists should succeed");
}
