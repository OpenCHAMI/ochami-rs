//! wiremock tests for `Ochami::hsm_inventory_ethernet_interface_*` —
//! `/hsm/v2/Inventory/EthernetInterfaces`.
//!
//! One test per wrapper method (Task 11's 9 methods; `delete_all` is
//! intentionally excluded — see
//! `src/hsm/wrapper/inventory_ethernet_interfaces.rs` doc comment).
//! Mirrors the Phase 6 wiremock test template established in Task 14's
//! `tests/hsm_component.rs` (design spec Section 4, Phase 6).
//!
//! Return types here are the **slim generated re-exports** from
//! `src/hsm/inventory/ethernet_interfaces/types.rs` —
//! `ComponentEthernetInterface` (aliasing `CompEthInterface100`),
//! `ComponentEthernetInterfacePatch` (aliasing `CompEthInterface100Patch`),
//! `IpAddressMapping` (aliasing `CompEthInterface100IpAddressMapping`), and
//! `IpAddressMappingPatch` (aliasing
//! `CompEthInterface100IpAddressMappingPatch`) — matching the
//! `hsm_inventory_redfish_endpoint.rs` slim-generated-type pattern, not
//! the hand-rolled-type pattern used by `hsm_inventory_hardware.rs`.
//!
//! Per Task 11's wrapper doc comment
//! (`src/hsm/wrapper/inventory_ethernet_interfaces.rs`):
//! - `hsm_inventory_ethernet_interface_post` and
//!   `_post_ip_address` return a singular `ResourceUri100` (201), not a
//!   `Vec<ResourceUri100>` and not `HsmActionResponse`.
//! - `hsm_inventory_ethernet_interface_patch` and
//!   `_patch_ip_address` return `()` (200 with no body schema), not
//!   `HsmActionResponse` and not the full updated resource.
//! - `hsm_inventory_ethernet_interface_delete_one` and
//!   `_delete_ip_address` return `HsmActionResponse` (`Response100`) on a
//!   `200` — **not** `204` as the brief's sketch guessed; verified against
//!   the generated `do_comp_eth_interface_delete_v2` /
//!   `do_comp_eth_interface_ip_address_delete_v2`, both of which match
//!   only on `200u16` and deserialize a `Response100` body.
//! - `hsm_inventory_ethernet_interface_delete_ip_address` takes only
//!   `(token, id, ip_address)` — no `_group_label` argument (dropped as a
//!   dead parameter per spec § Latent Bugs).

mod common;

use common::TEST_PEM;
use ochami_rs::hsm::inventory::ethernet_interfaces::types::{
  ComponentEthernetInterface, ComponentEthernetInterfacePatch,
  IpAddressMapping, IpAddressMappingPatch,
};
use ochami_rs::Ochami;
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn sample_interface(id: &str, mac: &str) -> ComponentEthernetInterface {
  ComponentEthernetInterface {
    component_id: None,
    description: None,
    id: Some(id.to_string()),
    ip_addresses: vec![],
    last_update: None,
    mac_address: mac.to_string(),
    type_: None,
  }
}

fn sample_ip(ip: &str, network: Option<&str>) -> IpAddressMapping {
  IpAddressMapping {
    ip_address: ip.to_string(),
    network: network.map(str::to_string),
  }
}

// ---------------------------------------------------------------------
// Method tests (one per `Ochami::hsm_inventory_ethernet_interface_*`
// wrapper method).
// ---------------------------------------------------------------------

#[tokio::test]
async fn hsm_inventory_ethernet_interface_get_returns_array() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/Inventory/EthernetInterfaces"))
    .and(query_param("MACAddress", "aa:bb:cc:dd:ee:ff"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!([
        {"ID": "abc123", "MACAddress": "aa:bb:cc:dd:ee:ff"}
    ])))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let eis = ochami
    .hsm_inventory_ethernet_interface_get(
      "test-token",
      Some("aa:bb:cc:dd:ee:ff"),
      None,
      None,
      None,
      None,
      None,
      None,
    )
    .await
    .expect("should succeed");
  assert_eq!(eis.len(), 1);
  assert_eq!(eis[0].id.as_deref(), Some("abc123"));
  assert_eq!(eis[0].mac_address, "aa:bb:cc:dd:ee:ff");
}

#[tokio::test]
async fn hsm_inventory_ethernet_interface_get_one_returns_ei() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path("/hsm/v2/Inventory/EthernetInterfaces/abc123"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "ID": "abc123", "MACAddress": "aa:bb:cc:dd:ee:ff"
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let ei = ochami
    .hsm_inventory_ethernet_interface_get_one("test-token", "abc123")
    .await
    .expect("should succeed");
  assert_eq!(ei.id.as_deref(), Some("abc123"));
  assert_eq!(ei.mac_address, "aa:bb:cc:dd:ee:ff");
}

#[tokio::test]
async fn hsm_inventory_ethernet_interface_post_creates_ei() {
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path("/hsm/v2/Inventory/EthernetInterfaces"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(201).set_body_json(json!({
        "ResourceURI": "/hsm/v2/Inventory/EthernetInterfaces/abc123"
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let ei = sample_interface("abc123", "aa:bb:cc:dd:ee:ff");
  let uri = ochami
    .hsm_inventory_ethernet_interface_post("test-token", &ei)
    .await
    .expect("should succeed");
  assert_eq!(
    uri.resource_uri.as_deref(),
    Some("/hsm/v2/Inventory/EthernetInterfaces/abc123")
  );
}

#[tokio::test]
async fn hsm_inventory_ethernet_interface_patch_updates_ei() {
  let server = MockServer::start().await;
  Mock::given(method("PATCH"))
    .and(path("/hsm/v2/Inventory/EthernetInterfaces/abc123"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let patch = ComponentEthernetInterfacePatch {
    component_id: None,
    description: Some("updated".to_string()),
    ip_addresses: vec![],
  };
  ochami
    .hsm_inventory_ethernet_interface_patch("test-token", "abc123", &patch)
    .await
    .expect("should succeed");
}

#[tokio::test]
async fn hsm_inventory_ethernet_interface_delete_one_removes_ei() {
  let server = MockServer::start().await;
  Mock::given(method("DELETE"))
    .and(path("/hsm/v2/Inventory/EthernetInterfaces/abc123"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "code": "0", "message": "deleted"
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let resp = ochami
    .hsm_inventory_ethernet_interface_delete_one("test-token", "abc123")
    .await
    .expect("should succeed");
  assert_eq!(resp.code, "0");
  assert_eq!(resp.message, "deleted");
}

#[tokio::test]
async fn hsm_inventory_ethernet_interface_get_ip_addresses_returns_array() {
  let server = MockServer::start().await;
  Mock::given(method("GET"))
    .and(path(
      "/hsm/v2/Inventory/EthernetInterfaces/abc123/IPAddresses",
    ))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!([
        {"IPAddress": "10.0.0.1", "Network": "NMN"}
    ])))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let ips = ochami
    .hsm_inventory_ethernet_interface_get_ip_addresses("test-token", "abc123")
    .await
    .expect("should succeed");
  assert_eq!(ips.len(), 1);
  assert_eq!(ips[0].ip_address, "10.0.0.1");
  assert_eq!(ips[0].network.as_deref(), Some("NMN"));
}

#[tokio::test]
async fn hsm_inventory_ethernet_interface_post_ip_address_adds_ip() {
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path("/hsm/v2/Inventory/EthernetInterfaces/abc123/IPAddresses"))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(201).set_body_json(json!({
        "ResourceURI": "/hsm/v2/Inventory/EthernetInterfaces/abc123/IPAddresses/10.0.0.1"
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let ip = sample_ip("10.0.0.1", Some("NMN"));
  let uri = ochami
    .hsm_inventory_ethernet_interface_post_ip_address(
      "test-token",
      "abc123",
      &ip,
    )
    .await
    .expect("should succeed");
  assert_eq!(
    uri.resource_uri.as_deref(),
    Some("/hsm/v2/Inventory/EthernetInterfaces/abc123/IPAddresses/10.0.0.1")
  );
}

#[tokio::test]
async fn hsm_inventory_ethernet_interface_patch_ip_address_updates_ip() {
  let server = MockServer::start().await;
  Mock::given(method("PATCH"))
    .and(path(
      "/hsm/v2/Inventory/EthernetInterfaces/abc123/IPAddresses/10.0.0.1",
    ))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let patch = IpAddressMappingPatch {
    network: Some("NMN".to_string()),
  };
  ochami
    .hsm_inventory_ethernet_interface_patch_ip_address(
      "test-token",
      "abc123",
      "10.0.0.1",
      &patch,
    )
    .await
    .expect("should succeed");
}

#[tokio::test]
async fn hsm_inventory_ethernet_interface_delete_ip_address_removes_ip() {
  let server = MockServer::start().await;
  Mock::given(method("DELETE"))
    .and(path(
      "/hsm/v2/Inventory/EthernetInterfaces/abc123/IPAddresses/10.0.0.1",
    ))
    .and(header("authorization", "Bearer test-token"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "code": "0", "message": "deleted"
    })))
    .mount(&server)
    .await;

  let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
  let resp = ochami
    .hsm_inventory_ethernet_interface_delete_ip_address(
      "test-token",
      "abc123",
      "10.0.0.1",
    )
    .await
    .expect("should succeed");
  assert_eq!(resp.code, "0");
  assert_eq!(resp.message, "deleted");
}
