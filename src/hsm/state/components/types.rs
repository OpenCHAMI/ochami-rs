use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Component {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename(serialize = "ID"))]
  pub id: Option<String>,
  #[serde(rename(serialize = "Type"))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub r#type: Option<String>,
  #[serde(rename(serialize = "State"))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub state: Option<String>,
  #[serde(rename(serialize = "Flag"))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub flag: Option<String>,
  #[serde(rename(serialize = "Enabled"))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub enabled: Option<bool>,
  #[serde(rename(serialize = "SoftwareStatus"))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub software_status: Option<String>,
  #[serde(rename(serialize = "Role"))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub role: Option<String>,
  #[serde(rename(serialize = "SubRole"))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sub_role: Option<String>,
  #[serde(rename(serialize = "Nid"))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nid: Option<u32>,
  #[serde(rename(serialize = "SubType"))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subtype: Option<String>,
  #[serde(rename(serialize = "NetType"))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub net_type: Option<String>,
  #[serde(rename(serialize = "Arch"))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub arch: Option<String>,
  #[serde(rename(serialize = "Class"))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub class: Option<String>,
  #[serde(rename(serialize = "Reservation"))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reservation_disabled: Option<bool>,
  #[serde(rename(serialize = "Locked"))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub locked: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentArray {
  #[serde(rename(serialize = "Components"))]
  pub components: Vec<Component>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename(serialize = "Force"))]
  pub force: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentPostQuery {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename(serialize = "ComponentIDs"))]
  pub component_ids: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub partition: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub group: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename(serialize = "stateonly"))]
  pub state_only: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename(serialize = "flagonly"))]
  pub flag_only: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename(serialize = "roleonly"))]
  pub role_only: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename(serialize = "nidonly"))]
  pub nid_only: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub r#type: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub state: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub flag: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub enabled: Option<bool>,
  #[serde(rename(serialize = "softwarestatus"))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub software_status: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub role: Option<String>,
  #[serde(rename(serialize = "subrole"))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sub_role: Option<String>,
  #[serde(rename(serialize = "subtype"))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subtype: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub arch: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub class: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nid: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nid_start: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nid_end: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentPostByNidQuery {
  #[serde(rename(serialize = "NIDRanges"))]
  pub nid_ranger: Vec<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub partition: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stateonly: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub flagonly: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub roleonly: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nidonly: Option<bool>,
}
