use serde::{Deserialize, Serialize};

/// Wrapper for the list of power-cap tasks returned by PCS.
#[derive(Debug, Serialize, Deserialize)]
pub struct PowerCapTaskList {
  /// Power-cap tasks returned by the endpoint.
  pub tasks: Vec<PowerCapTaskInfo>,
}

/// Roll-up of per-task states reported by PCS for a power-cap task.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskCounts {
  /// Total number of subtasks.
  pub total: usize,
  /// Subtasks in the `new` state (accepted, not yet started).
  pub new: usize,
  /// Subtasks currently in progress.
  pub in_progress: usize,
  /// Subtasks that failed.
  pub failed: usize,
  /// Subtasks that completed successfully.
  pub succeeded: usize,
  /// Subtasks whose operation is not supported by the target hardware.
  pub un_supported: usize,
}

/// Hardware-reported power-cap limit envelope for a component.
#[derive(Debug, Serialize, Deserialize)]
pub struct Limit {
  /// Maximum power (watts) the host will accept as a cap.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "hostsLimitMax")]
  pub hosts_limit_max: Option<usize>,
  /// Minimum power (watts) the host will accept as a cap.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "hostsLimitMin")]
  pub hosts_limit_min: Option<usize>,
  /// Power (watts) required to power the component on.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "powerupPower")]
  pub powerup_power: Option<usize>,
}

/// A single named power-cap control and its current/allowed values.
#[derive(Debug, Serialize, Deserialize)]
pub struct PowerCapLimit {
  /// Control name as reported by the hardware.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  /// Currently configured cap (watts).
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "currentValue")]
  pub current_value: Option<usize>,
  /// Maximum cap value (watts) the control accepts.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "maximumValue")]
  pub maximum_value: Option<usize>,
  /// Minimum cap value (watts) the control accepts.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "minimumValue")]
  pub minimum_value: Option<usize>,
}

/// Per-component result inside a power-cap task.
#[derive(Debug, Serialize, Deserialize)]
pub struct PowerCapComponent {
  /// Component xname.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub xname: Option<String>,
  /// Error message if the component failed the operation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub error: Option<String>,
  /// Hardware-reported limits for this component.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub limits: Option<Limit>,
  /// Current power-cap control settings for this component.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "power_cap_limits")]
  pub power_cap_limits: Option<PowerCapLimit>,
}

/// Metadata and per-component results for a single PCS power-cap task.
#[derive(Debug, Serialize, Deserialize)]
pub struct PowerCapTaskInfo {
  /// Unique identifier assigned by PCS to this task.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "taskId")]
  pub task_id: Option<String>,
  /// Task kind reported by PCS (e.g. `snapshot` or `patch`).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub r#type: Option<String>, // TODO: convert to enum. Valid values are `snapshot` and `patch`
  /// Timestamp (RFC 3339) when the task was created.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "taskCreateTime")]
  pub task_create_time: Option<String>,
  /// Timestamp (RFC 3339) after which the task record is auto-expired.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "automaticExpirationTime")]
  pub automatic_expiration_time: Option<String>,
  /// Overall status string reported by PCS for the task.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "taskStatus")]
  pub task_status: Option<String>,
  /// Roll-up counts of subtasks by state.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "taskCounts")]
  pub task_counts: Option<TaskCounts>,
  /// Per-component results for this task.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub components: Option<Vec<PowerCapComponent>>,
}
