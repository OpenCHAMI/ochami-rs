//! wiremock tests for `Ochami::hsm_memberships_*` — `/hsm/v2/memberships`.
//!
//! One test per wrapper method (Task 5's 2 methods). Mirrors the Phase 6
//! wiremock test template established in Task 14's `tests/hsm_component.rs`
//! and Task 15's `tests/hsm_group.rs` (design spec Section 4, Phase 6).

mod common;

use common::TEST_PEM;
use ochami_rs::hsm::memberships::types::Membership;
use ochami_rs::Ochami;
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn hsm_memberships_get_returns_array() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/memberships"))
    .and(header("authorization", "Bearer test-token"))
    .and(query_param("type", "Node"))
    .and(query_param("role", "Compute"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!([
        {
            "id": "x1c0s1b0n0",
            "groupLabels": ["compute"],
            "partitionName": "p1",
        }
    ])))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let memberships: Vec<Membership> = ochami
    .hsm_memberships_get(
      "test-token",
      None,
      Some("Node"),
      None,
      None,
      Some("Compute"),
      None,
      None,
      None,
      None,
      None,
    )
    .await
    .expect("should succeed");
  assert_eq!(memberships.len(), 1);
  assert_eq!(
    memberships[0].id.as_ref().map(|x| x.0.as_str()),
    Some("x1c0s1b0n0")
  );
}

#[tokio::test]
async fn hsm_memberships_get_xname_returns_one() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/memberships/x1c0s1b0n0"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "id": "x1c0s1b0n0",
        "groupLabels": ["compute"],
        "partitionName": "p1",
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let membership = ochami
    .hsm_memberships_get_xname("test-token", "x1c0s1b0n0")
    .await
    .expect("should succeed");
  assert_eq!(
    membership.id.as_ref().map(|x| x.0.as_str()),
    Some("x1c0s1b0n0")
  );
}
