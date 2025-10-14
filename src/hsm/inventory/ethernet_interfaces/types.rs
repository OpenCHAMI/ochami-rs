use serde::{Deserialize, Serialize};
//use crate::hsm::inventory::types::ComponentType;
//use manta_backend_dispatcher::types::hsm::inventory::{
use manta_backend_dispatcher::types::hsm::inventory::{
  ComponentEthernetInterface as FrontendComponentEthernetInterface,
  ComponentEthernetInterfaceArray as FrontendComponentEthernetInterfaceArray,
  IpAddressMapping as FrontendIpAddressMapping,
};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentEthernetInterface {
  #[serde(rename = "ID")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,
  #[serde(rename = "Description")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(rename = "MACAddress")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mac_address: Option<String>,
  #[serde(rename = "IPAddresses")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ip_addresses: Option<Vec<IpAddressMapping>>,
  #[serde(rename = "LastUpdate")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub last_update: Option<String>,
  #[serde(rename = "ComponentID")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub component_id: Option<String>,
  #[serde(rename = "Type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub parent_hms_type: Option<String>,
}
impl From<FrontendComponentEthernetInterface> for ComponentEthernetInterface {
  fn from(interface: FrontendComponentEthernetInterface) -> Self {
    ComponentEthernetInterface {
      id: interface.id,
      description: interface.description,
      mac_address: interface.mac_address,
      ip_addresses: interface.ip_addresses.map(|ips| ips.into_iter().map(IpAddressMapping::from).collect()),
      last_update: interface.last_update,
      component_id: interface.component_id,
      parent_hms_type: interface.parent_hms_type,
    }
  }
}
impl Into<FrontendComponentEthernetInterface> for ComponentEthernetInterface {
  fn into(self) -> FrontendComponentEthernetInterface {
    FrontendComponentEthernetInterface {
      id: self.id,
      description: self.description,
      mac_address: self.mac_address,
      ip_addresses: self.ip_addresses.map(|ips| ips.into_iter().map(|ip| ip.into()).collect()),
      last_update: self.last_update,
      component_id: self.component_id,
      parent_hms_type: self.parent_hms_type,
    }
  }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentEthernetInterfaceArray {
  #[serde(rename = "EthernetInterfaces")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ethernet_interfaces: Option<Vec<ComponentEthernetInterface>>,
}
impl From<FrontendComponentEthernetInterfaceArray> for ComponentEthernetInterfaceArray {
  fn from(array: FrontendComponentEthernetInterfaceArray) -> Self {
    ComponentEthernetInterfaceArray {
      ethernet_interfaces: array.ethernet_interfaces.map(|interfaces| {
        interfaces.into_iter().map(ComponentEthernetInterface::from).collect()
      }),
    }
  }
}
impl Into<FrontendComponentEthernetInterfaceArray> for ComponentEthernetInterfaceArray {
  fn into(self) -> FrontendComponentEthernetInterfaceArray {
    FrontendComponentEthernetInterfaceArray {
      ethernet_interfaces: self.ethernet_interfaces.map(|interfaces| {
        interfaces
          .into_iter()
          .map(|interface| interface.into())
          .collect()
      }),
    }
  }
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct IpAddressMapping {
  #[serde(rename = "IPAddress")]
  pub ip_address: String,
  #[serde(rename = "Network")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub network: Option<String>,
}
impl From<FrontendIpAddressMapping> for IpAddressMapping {
  fn from(address: FrontendIpAddressMapping) -> Self {
    IpAddressMapping {
      ip_address: address.ip_address,
      network: address.network,
    }
  }
}
impl Into<FrontendIpAddressMapping> for IpAddressMapping {
  fn into(self) -> FrontendIpAddressMapping {
    FrontendIpAddressMapping {
      ip_address: self.ip_address,
      network: self.network,
    }
  }
}
