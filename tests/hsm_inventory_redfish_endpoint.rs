//! wiremock tests for `Ochami::hsm_inventory_redfish_endpoint_*` —
//! `/hsm/v2/Inventory/RedfishEndpoints`.
//!
//! One test per wrapper method (Task 10's 7 methods). Mirrors the
//! Phase 6 wiremock test template established in Task 14's
//! `tests/hsm_component.rs` (design spec Section 4, Phase 6).
//!
//! Return types here are the **slim generated re-exports** from
//! `src/hsm/inventory/redfish_endpoint/types.rs` — `RedfishEndpoint`
//! (aliasing `RedfishEndpoint100RedfishEndpoint`) and
//! `RedfishEndpointArray` (aliasing
//! `RedfishEndpointArrayRedfishEndpointArray`) — matching the
//! `hsm_component.rs` slim-generated-type pattern, not the
//! hand-rolled-type pattern used by `hsm_inventory_hardware.rs`.
//!
//! Per Task 10's wrapper doc comment
//! (`src/hsm/wrapper/inventory_redfish_endpoint.rs`):
//! - `hsm_inventory_redfish_endpoint_post` returns `Vec<ResourceUri100>`
//!   (201), not `HsmActionResponse`.
//! - `hsm_inventory_redfish_endpoint_put` / `_patch` return the full
//!   updated `RedfishEndpoint` (200), not `HsmActionResponse`.
//! - `hsm_inventory_redfish_endpoint_get_query` is a `GET` (not the
//!   design spec's originally-guessed `POST`) returning
//!   `RedfishEndpointArray`, fixing both the verb and a latent
//!   singular/plural URL bug in the pre-migration hand-written client.
//! - `hsm_inventory_redfish_endpoint_delete_one` returns
//!   `HsmActionResponse` (`Response_1.0.0`).

mod common;

use common::TEST_PEM;
use ochami_rs::hsm::inventory::redfish_endpoint::types::{
  RedfishEndpoint, XNameRfEndpoint100,
};
use ochami_rs::Ochami;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn sample_endpoint(xname: &str) -> RedfishEndpoint {
  RedfishEndpoint {
    discovery_info: None,
    domain: None,
    enabled: None,
    fqdn: None,
    hostname: None,
    id: XNameRfEndpoint100(xname.to_string()),
    ip_address: None,
    mac_addr: None,
    mac_required: None,
    name: None,
    password: None,
    rediscover_on_update: None,
    template_id: None,
    type_: None,
    use_ssdp: None,
    user: None,
    uuid: None,
  }
}

// ---------------------------------------------------------------------
// Method tests (one per `Ochami::hsm_inventory_redfish_endpoint_*`
// wrapper method).
// ---------------------------------------------------------------------

#[tokio::test]
async fn hsm_inventory_redfish_endpoint_get_returns_array() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/Inventory/RedfishEndpoints"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "RedfishEndpoints": [{"ID": "x1c0s1b0"}]
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let eps = ochami
    .hsm_inventory_redfish_endpoint_get(
      "test-token",
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
  assert_eq!(eps.redfish_endpoints.len(), 1);
  assert_eq!(eps.redfish_endpoints[0].id.0, "x1c0s1b0");
}

#[tokio::test]
async fn hsm_inventory_redfish_endpoint_get_one_returns_endpoint() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/Inventory/RedfishEndpoints/x1c0s1b0"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "ID": "x1c0s1b0",
        "FQDN": "x1c0s1b0.local"
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let ep = ochami
    .hsm_inventory_redfish_endpoint_get_one("test-token", "x1c0s1b0")
    .await
    .expect("should succeed");
  assert_eq!(ep.id.0, "x1c0s1b0");
  assert_eq!(ep.fqdn.as_deref(), Some("x1c0s1b0.local"));
}

#[tokio::test]
async fn hsm_inventory_redfish_endpoint_post_creates_endpoint() {
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path("/hsm/v2/Inventory/RedfishEndpoints"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(201).set_body_json(json!([
        {"ResourceURI": "/hsm/v2/Inventory/RedfishEndpoints/x1c0s1b0"}
    ])))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let ep = sample_endpoint("x1c0s1b0");
  let uris = ochami
    .hsm_inventory_redfish_endpoint_post("test-token", &ep)
    .await
    .expect("should succeed");
  assert_eq!(uris.len(), 1);
  assert_eq!(
    uris[0].resource_uri.as_deref(),
    Some("/hsm/v2/Inventory/RedfishEndpoints/x1c0s1b0")
  );
}

#[tokio::test]
async fn hsm_inventory_redfish_endpoint_get_query_returns_result() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/Inventory/RedfishEndpoints/Query/x1c0s1b0"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "RedfishEndpoints": [{"ID": "x1c0s1b0"}]
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let result = ochami
    .hsm_inventory_redfish_endpoint_get_query("test-token", "x1c0s1b0")
    .await
    .expect("should succeed");
  assert_eq!(result.redfish_endpoints.len(), 1);
  assert_eq!(result.redfish_endpoints[0].id.0, "x1c0s1b0");
}

#[tokio::test]
async fn hsm_inventory_redfish_endpoint_put_updates_endpoint() {
  let server = MockServer::start().await;
  Mock::given(method("PUT"))
    .and(path("/hsm/v2/Inventory/RedfishEndpoints/x1c0s1b0"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "ID": "x1c0s1b0",
        "Enabled": true
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let ep = sample_endpoint("x1c0s1b0");
  let updated = ochami
    .hsm_inventory_redfish_endpoint_put("test-token", "x1c0s1b0", &ep)
    .await
    .expect("should succeed");
  assert_eq!(updated.id.0, "x1c0s1b0");
  assert_eq!(updated.enabled, Some(true));
}

#[tokio::test]
async fn hsm_inventory_redfish_endpoint_patch_updates_field() {
  let server = MockServer::start().await;
  Mock::given(method("PATCH"))
    .and(path("/hsm/v2/Inventory/RedfishEndpoints/x1c0s1b0"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "ID": "x1c0s1b0",
        "Enabled": false
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let mut ep = sample_endpoint("x1c0s1b0");
  ep.enabled = Some(false);
  let updated = ochami
    .hsm_inventory_redfish_endpoint_patch("test-token", "x1c0s1b0", &ep)
    .await
    .expect("should succeed");
  assert_eq!(updated.id.0, "x1c0s1b0");
  assert_eq!(updated.enabled, Some(false));
}

#[tokio::test]
async fn hsm_inventory_redfish_endpoint_delete_one_removes_endpoint() {
  let server = MockServer::start().await;
  Mock::given(method("DELETE"))
    .and(path("/hsm/v2/Inventory/RedfishEndpoints/x1c0s1b0"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "code": "0", "message": "deleted"
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let resp = ochami
    .hsm_inventory_redfish_endpoint_delete_one("test-token", "x1c0s1b0")
    .await
    .expect("should succeed");
  assert_eq!(resp.code, "0");
  assert_eq!(resp.message, "deleted");
}
