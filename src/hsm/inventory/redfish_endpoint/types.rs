//! Slim re-exports from the progenitor-generated HSM types, plus
//! `From`/`Into` conversions between the generated
//! `RedfishEndpoint100RedfishEndpoint` /
//! `RedfishEndpointArrayRedfishEndpointArray` types and the
//! `manta_backend_dispatcher` front-end `RedfishEndpoint` /
//! `RedfishEndpointArray` / `DiscoveryInfo` types.
//!
//! Task 13 (Phase 5) replaced the hand-written duplicate types with
//! direct aliases. Design spec Section 4 (Phase 5) — Types migration.

use std::str::FromStr;

pub use crate::hsm::generated::types::{
  HmsType100, RedfishEndpoint100RedfishEndpoint as RedfishEndpoint,
  RedfishEndpoint100RedfishEndpointDiscoveryInfo as DiscoveryInfo,
  RedfishEndpoint100RedfishEndpointDiscoveryInfoLastStatus as DiscoveryLastStatus,
  RedfishEndpoint100RedfishEndpointMacAddr as MacAddr,
  RedfishEndpointArrayRedfishEndpointArray as RedfishEndpointArray, Uuid100,
  XNameRfEndpoint100,
};

use manta_backend_dispatcher::types::hsm::inventory::{
  DiscoveryInfo as FrontEndDiscoveryInfo,
  RedfishEndpoint as FrontEndRedfishEndpoint,
  RedfishEndpointArray as FrontEndRedfishEndpointArray,
};

// ---------------------------------------------------------------------------
// DiscoveryInfo <-> FrontEndDiscoveryInfo
// ---------------------------------------------------------------------------

impl From<FrontEndDiscoveryInfo> for DiscoveryInfo {
  fn from(value: FrontEndDiscoveryInfo) -> Self {
    DiscoveryInfo {
      // Front-end uses a String timestamp; generated uses
      // `chrono::DateTime<Utc>`. Drop on inbound conversion (server
      // fills this in — no useful round-trip).
      last_attempt: None,
      last_status: value
        .last_status
        .and_then(|s| DiscoveryLastStatus::from_str(&s).ok()),
      redfish_version: value.redfish_version,
    }
  }
}

impl From<DiscoveryInfo> for FrontEndDiscoveryInfo {
  fn from(value: DiscoveryInfo) -> Self {
    FrontEndDiscoveryInfo {
      last_attempt: value.last_attempt.map(|dt| dt.to_rfc3339()),
      last_status: value.last_status.map(|s| s.to_string()),
      redfish_version: value.redfish_version,
    }
  }
}

// ---------------------------------------------------------------------------
// RedfishEndpoint <-> FrontEndRedfishEndpoint
// ---------------------------------------------------------------------------

impl From<FrontEndRedfishEndpoint> for RedfishEndpoint {
  fn from(value: FrontEndRedfishEndpoint) -> Self {
    RedfishEndpoint {
      id: XNameRfEndpoint100(value.id),
      type_: value.r#type.and_then(|s| HmsType100::from_str(&s).ok()),
      name: value.name,
      hostname: value.hostname,
      domain: value.domain,
      fqdn: value.fqdn,
      enabled: value.enabled,
      uuid: value.uuid.and_then(|s| Uuid100::from_str(&s).ok()),
      user: value.user,
      password: value.password,
      use_ssdp: value.use_ssdp,
      mac_required: value.mac_required,
      mac_addr: value.mac_addr.and_then(|s| MacAddr::from_str(&s).ok()),
      ip_address: value.ip_address,
      rediscover_on_update: value.rediscover_on_update,
      template_id: value.template_id,
      discovery_info: value.discovery_info.map(Into::into),
    }
  }
}

impl From<RedfishEndpoint> for FrontEndRedfishEndpoint {
  fn from(value: RedfishEndpoint) -> Self {
    FrontEndRedfishEndpoint {
      id: value.id.0,
      r#type: value.type_.map(|t| t.to_string()),
      name: value.name,
      hostname: value.hostname,
      domain: value.domain,
      fqdn: value.fqdn,
      enabled: value.enabled,
      uuid: value.uuid.map(|u| (*u).clone()),
      user: value.user,
      password: value.password,
      use_ssdp: value.use_ssdp,
      mac_required: value.mac_required,
      mac_addr: value.mac_addr.map(|m| (*m).clone()),
      ip_address: value.ip_address,
      rediscover_on_update: value.rediscover_on_update,
      template_id: value.template_id,
      discovery_info: value.discovery_info.map(Into::into),
    }
  }
}

// ---------------------------------------------------------------------------
// RedfishEndpointArray <-> FrontEndRedfishEndpointArray
// ---------------------------------------------------------------------------

impl From<FrontEndRedfishEndpointArray> for RedfishEndpointArray {
  fn from(value: FrontEndRedfishEndpointArray) -> Self {
    RedfishEndpointArray {
      redfish_endpoints: value
        .redfish_endpoints
        .map(|v| v.into_iter().map(RedfishEndpoint::from).collect())
        .unwrap_or_default(),
    }
  }
}

impl From<RedfishEndpointArray> for FrontEndRedfishEndpointArray {
  fn from(value: RedfishEndpointArray) -> Self {
    FrontEndRedfishEndpointArray {
      redfish_endpoints: Some(
        value
          .redfish_endpoints
          .into_iter()
          .map(Into::into)
          .collect(),
      ),
    }
  }
}
