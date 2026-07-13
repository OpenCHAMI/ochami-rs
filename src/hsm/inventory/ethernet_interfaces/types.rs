//! Slim re-exports from the progenitor-generated HSM types, plus
//! `From`/`Into` conversions between the generated `CompEthInterface100`
//! family of types and the `manta_backend_dispatcher` front-end
//! `ComponentEthernetInterface` / `IpAddressMapping` types.
//!
//! Task 13 (Phase 5) replaced the hand-written duplicate types with
//! direct aliases. Design spec Section 4 (Phase 5) — Types migration.

use std::str::FromStr;

pub use crate::hsm::generated::types::{
  CompEthInterface100 as ComponentEthernetInterface,
  CompEthInterface100IpAddressMapping as IpAddressMapping,
  CompEthInterface100IpAddressMappingPatch as IpAddressMappingPatch,
  CompEthInterface100Patch as ComponentEthernetInterfacePatch, HmsType100,
  XNameRw100,
};

use manta_backend_dispatcher::types::hsm::inventory::{
  ComponentEthernetInterface as FrontendComponentEthernetInterface,
  IpAddressMapping as FrontendIpAddressMapping,
};

// ---------------------------------------------------------------------------
// IpAddressMapping <-> FrontendIpAddressMapping
// ---------------------------------------------------------------------------

impl From<FrontendIpAddressMapping> for IpAddressMapping {
  fn from(value: FrontendIpAddressMapping) -> Self {
    IpAddressMapping {
      ip_address: value.ip_address,
      network: value.network,
    }
  }
}

impl From<IpAddressMapping> for FrontendIpAddressMapping {
  fn from(value: IpAddressMapping) -> Self {
    FrontendIpAddressMapping {
      ip_address: value.ip_address,
      network: value.network,
    }
  }
}

// ---------------------------------------------------------------------------
// ComponentEthernetInterface <-> FrontendComponentEthernetInterface
// ---------------------------------------------------------------------------

impl From<FrontendComponentEthernetInterface> for ComponentEthernetInterface {
  fn from(value: FrontendComponentEthernetInterface) -> Self {
    ComponentEthernetInterface {
      id: value.id,
      description: value.description,
      // MACAddress is required on the generated type; missing input maps
      // to empty string (upstream will reject on POST — deferred to
      // caller validation).
      mac_address: value.mac_address.unwrap_or_default(),
      ip_addresses: value
        .ip_addresses
        .map(|v| v.into_iter().map(IpAddressMapping::from).collect())
        .unwrap_or_default(),
      // Front-end uses a String timestamp; generated uses
      // `chrono::DateTime<Utc>`. Drop on conversion (LastUpdate is
      // server-populated and not round-tripped through the front-end).
      last_update: None,
      component_id: value.component_id.map(XNameRw100),
      type_: value.parent_hms_type.and_then(|s| HmsType100::from_str(&s).ok()),
    }
  }
}

impl From<ComponentEthernetInterface> for FrontendComponentEthernetInterface {
  fn from(value: ComponentEthernetInterface) -> Self {
    FrontendComponentEthernetInterface {
      id: value.id,
      description: value.description,
      mac_address: Some(value.mac_address),
      ip_addresses: Some(
        value.ip_addresses.into_iter().map(Into::into).collect(),
      ),
      last_update: value.last_update.map(|dt| dt.to_rfc3339()),
      component_id: value.component_id.map(|x| x.0),
      parent_hms_type: value.type_.map(|t| t.to_string()),
    }
  }
}

// ---------------------------------------------------------------------------
// ComponentEthernetInterface -> ComponentEthernetInterfacePatch
// ---------------------------------------------------------------------------

impl From<ComponentEthernetInterface> for ComponentEthernetInterfacePatch {
  fn from(value: ComponentEthernetInterface) -> Self {
    ComponentEthernetInterfacePatch {
      component_id: value.component_id,
      description: value.description,
      ip_addresses: value.ip_addresses,
    }
  }
}

impl From<FrontendComponentEthernetInterface> for ComponentEthernetInterfacePatch {
  fn from(value: FrontendComponentEthernetInterface) -> Self {
    ComponentEthernetInterfacePatch {
      component_id: value.component_id.map(XNameRw100),
      description: value.description,
      ip_addresses: value
        .ip_addresses
        .map(|v| v.into_iter().map(IpAddressMapping::from).collect())
        .unwrap_or_default(),
    }
  }
}
