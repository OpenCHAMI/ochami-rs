use manta_backend_dispatcher::types::pcs::power_status::types::{
  ManagementState as FrontEndManagementState, PowerState as FrontEndPowerState,
  PowerStatus as FrontEndPowerStatus, PowerStatusAll as FrontEndPowerStatusAll,
};

use crate::pcs::transitions::types::Operation;

use serde::{Deserialize, Serialize};

/// Current power state of a component as reported by PCS.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PowerState {
  /// Component is powered on.
  #[serde(rename = "on")]
  On,
  /// Component is powered off.
  #[serde(rename = "off")]
  Off,
  /// Power state could not be determined.
  #[serde(rename = "undefined")]
  Undefined,
}
impl From<FrontEndPowerState> for PowerState {
  fn from(value: FrontEndPowerState) -> Self {
    match value {
      FrontEndPowerState::On => PowerState::On,
      FrontEndPowerState::Off => PowerState::Off,
      FrontEndPowerState::Undefined => PowerState::Undefined,
    }
  }
}
impl From<PowerState> for FrontEndPowerState {
  fn from(val: PowerState) -> Self {
    match val {
      PowerState::On => FrontEndPowerState::On,
      PowerState::Off => FrontEndPowerState::Off,
      PowerState::Undefined => FrontEndPowerState::Undefined,
    }
  }
}

/// Whether PCS can currently manage the component.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ManagementState {
  /// Component cannot be reached or controlled.
  #[serde(rename = "unavailable")]
  Unavailable,
  /// Component is reachable and controllable.
  #[serde(rename = "available")]
  Available,
}
impl From<FrontEndManagementState> for ManagementState {
  fn from(value: FrontEndManagementState) -> Self {
    match value {
      FrontEndManagementState::Unavailable => ManagementState::Unavailable,
      FrontEndManagementState::Available => ManagementState::Available,
    }
  }
}
impl From<ManagementState> for FrontEndManagementState {
  fn from(val: ManagementState) -> Self {
    match val {
      ManagementState::Unavailable => FrontEndManagementState::Unavailable,
      ManagementState::Available => FrontEndManagementState::Available,
    }
  }
}

/// Power-status record for a single component returned by PCS.
#[derive(Debug, Serialize, Deserialize)]
pub struct PowerStatus {
  /// Component xname.
  pub xname: String,
  /// Current power state, if known.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "powerState")]
  pub power_state: Option<PowerState>,
  /// Current management-plane availability, if known.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "managementState")]
  pub management_state: Option<ManagementState>,
  /// Error message if the component's status could not be read.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "error")]
  pub error: Option<String>,
  /// Power transition operations the component currently supports.
  #[serde(rename = "supportedPowerTransitions")]
  pub supported_power_transitions: Vec<Operation>,
  /// Timestamp (RFC 3339) of the last successful status update.
  #[serde(rename = "lastUpdated")]
  pub last_updated: String,
}

impl From<FrontEndPowerStatus> for PowerStatus {
  fn from(value: FrontEndPowerStatus) -> Self {
    PowerStatus {
      xname: value.xname,
      //power_state_filter: value.power_state_filter.map( |v| PowerState::from(v)),
      power_state: value.power_state.map(PowerState::from),
      management_state: value
        .management_state
        .map(ManagementState::from),
      //management_state_filter: value.management_state_filter.map(|v| ManagementState::from(v)),
      error: value.error,
      supported_power_transitions: value
        .supported_power_transitions
        .into_iter()
        .map(Operation::from)
        .collect(),
      last_updated: value.last_updated,
    }
  }
}

impl From<PowerStatus> for FrontEndPowerStatus {
  fn from(val: PowerStatus) -> Self {
    FrontEndPowerStatus {
      xname: val.xname,
      //power_state_filter: self.power_state_filter.map(|v| v.into()),
      power_state: val.power_state.map(|v| v.into()),
      management_state: val.management_state.map(|v| v.into()),
      //management_state_filter: self.management_state_filter.map( |v| v.into()),
      error: val.error,
      supported_power_transitions: val
        .supported_power_transitions
        .into_iter()
        .map(|v| v.into())
        .collect(),
      last_updated: val.last_updated,
    }
  }
}

/// Wrapper for the full list of component power statuses returned by PCS.
#[derive(Debug, Serialize, Deserialize)]
pub struct PowerStatusAll {
  /// Power-status entries for every queried component.
  pub status: Vec<PowerStatus>,
}

impl From<FrontEndPowerStatusAll> for PowerStatusAll {
  fn from(value: FrontEndPowerStatusAll) -> Self {
    PowerStatusAll {
      status: value.status.into_iter().map(PowerStatus::from).collect(),
    }
  }
}

impl From<PowerStatusAll> for FrontEndPowerStatusAll {
  fn from(val: PowerStatusAll) -> Self {
    FrontEndPowerStatusAll {
      status: val.status.into_iter().map(Into::into).collect(),
    }
  }
}
