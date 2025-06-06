use serde::{Deserialize, Serialize};

use crate::hsm::inventory::types::ComponentType;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IpAddressMapping {
  pub ip_address: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub network: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ComponentEthernetInterface {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  pub ip_addresses: Vec<IpAddressMapping>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub component_id: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EthernetInterface {
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  mac_address: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  ip_address: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  last_update: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  component_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  r#type: Option<ComponentType>,
}
