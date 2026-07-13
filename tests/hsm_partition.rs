//! wiremock tests for `Ochami::hsm_partition_*` — `/hsm/v2/partitions`.
//!
//! One test per wrapper method (Task 7's 8 methods). Mirrors the Phase 6
//! wiremock test template established in Task 14's `tests/hsm_component.rs`
//! and Task 15's `tests/hsm_group.rs` (design spec Section 4, Phase 6).
//!
//! Status codes (verified against the generated `hsm_generated.rs`
//! `do_partition*` match arms): `do_partitions_get`,
//! `do_partition_get`, `do_partition_names_get`,
//! `do_partition_members_get`, `do_partition_delete`, and
//! `do_partition_member_delete` all match a single `200u16` success arm.
//! `do_partitions_post` and `do_partition_members_post` match a single
//! `201u16` success arm (mirroring `do_groups_post`/
//! `do_group_members_post` in `tests/hsm_group.rs`).

mod common;

use common::TEST_PEM;
use ochami_rs::hsm::partition::types::Partition;
use ochami_rs::Ochami;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn sample_partition() -> Partition {
  Partition {
    description: None,
    members: None,
    name: "p1".parse().expect("valid ResourceName"),
    tags: Vec::new(),
  }
}

// ---------------------------------------------------------------------
// Method tests (one per `Ochami::hsm_partition_*` wrapper method).
// ---------------------------------------------------------------------

#[tokio::test]
async fn hsm_partition_get_all_returns_array() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/partitions"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(
      ResponseTemplate::new(200).set_body_json(json!([{"name": "p1"}])),
    )
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let partitions = ochami
    .hsm_partition_get_all("test-token")
    .await
    .expect("should succeed");
  assert_eq!(partitions.len(), 1);
  assert_eq!(&*partitions[0].name, "p1");
}

#[tokio::test]
async fn hsm_partition_get_returns_partition() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/partitions/p1"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(
      ResponseTemplate::new(200).set_body_json(json!({"name": "p1"})),
    )
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let partition = ochami
    .hsm_partition_get("test-token", "p1")
    .await
    .expect("should succeed");
  assert_eq!(&*partition.name, "p1");
}

#[tokio::test]
async fn hsm_partition_get_names_returns_list() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/partitions/names"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!(["p1", "p2"])))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let names = ochami
    .hsm_partition_get_names("test-token")
    .await
    .expect("should succeed");
  assert_eq!(names, vec!["p1".to_string(), "p2".to_string()]);
}

#[tokio::test]
async fn hsm_partition_get_members_returns_ids() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/partitions/p1/members"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(
      ResponseTemplate::new(200).set_body_json(json!({"ids": ["x1", "x2"]})),
    )
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let members = ochami
    .hsm_partition_get_members("test-token", "p1")
    .await
    .expect("should succeed");
  assert_eq!(members.ids.len(), 2);
  assert_eq!(members.ids[0].0, "x1");
  assert_eq!(members.ids[1].0, "x2");
}

#[tokio::test]
async fn hsm_partition_post_creates_partition() {
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path("/hsm/v2/partitions"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(201).set_body_json(json!([
        {"ResourceURI": "/hsm/v2/partitions/p1"}
    ])))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let partition = sample_partition();
  let uris = ochami
    .hsm_partition_post("test-token", &partition)
    .await
    .expect("should succeed");
  assert_eq!(uris.len(), 1);
  assert_eq!(
    uris[0].resource_uri.as_deref(),
    Some("/hsm/v2/partitions/p1")
  );
}

#[tokio::test]
async fn hsm_partition_post_member_adds_xname() {
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path("/hsm/v2/partitions/p1/members"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(201).set_body_json(json!([
        {"ResourceURI": "/hsm/v2/partitions/p1/members/x1"}
    ])))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let uris = ochami
    .hsm_partition_post_member("test-token", "p1", "x1")
    .await
    .expect("should succeed");
  assert_eq!(uris.len(), 1);
  assert_eq!(
    uris[0].resource_uri.as_deref(),
    Some("/hsm/v2/partitions/p1/members/x1")
  );
}

#[tokio::test]
async fn hsm_partition_delete_one_removes_partition() {
  let server = MockServer::start().await;
  Mock::given(method("DELETE"))
    .and(path("/hsm/v2/partitions/p1"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "code": "0", "message": "deleted"
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let resp = ochami
    .hsm_partition_delete_one("test-token", "p1")
    .await
    .expect("should succeed");
  assert_eq!(resp.code, "0");
  assert_eq!(resp.message, "deleted");
}

#[tokio::test]
async fn hsm_partition_delete_member_removes_xname() {
  let server = MockServer::start().await;
  Mock::given(method("DELETE"))
    .and(path("/hsm/v2/partitions/p1/members/x1"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "code": "0", "message": "deleted"
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let resp = ochami
    .hsm_partition_delete_member("test-token", "p1", "x1")
    .await
    .expect("should succeed");
  assert_eq!(resp.code, "0");
  assert_eq!(resp.message, "deleted");
}
