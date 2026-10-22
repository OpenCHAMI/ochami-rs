use manta_backend_dispatcher::types::pcs::transitions::types::{
  Location as FrontEndLocation, Operation as FrontEndOperation,
  Task as FrontEndTask, TaskCounts as FrontEndTaskCounts,
  Transition as FrontEndTransition,
  TransitionResponse as FrontEndTransitionResponse,
};

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::error::Error;

/// Target component of a PCS power transition request.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Location {
  /// Component xname (Cray/OpenCHAMI hardware identifier).
  pub xname: String,
  /// Optional deputy key authorizing the transition on this component.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "deputyKey")]
  pub deputy_key: Option<String>,
}

impl From<FrontEndLocation> for Location {
  fn from(value: FrontEndLocation) -> Self {
    Location {
      xname: value.xname,
      deputy_key: value.deputy_key,
    }
  }
}
impl From<Location> for FrontEndLocation {
  fn from(val: Location) -> Self {
    FrontEndLocation {
      xname: val.xname,
      deputy_key: val.deputy_key,
    }
  }
}

// this matches the query, not the response
/// Power transition operation requested from PCS.
#[derive(Display, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Operation {
  /// Power on.
  #[serde(rename = "On")]
  On,
  /// Power off (graceful, then hard if needed).
  #[serde(rename = "Off")]
  Off,
  /// Graceful shutdown request.
  #[serde(rename = "Soft-Off")]
  SoftOff,
  /// Graceful restart request.
  #[serde(rename = "Soft-Restart")]
  SoftRestart,
  /// Immediate restart (hard reset).
  #[serde(rename = "Hard-Restart")]
  HardRestart,
  /// Initialization / reinitialize sequence.
  #[serde(rename = "Init")]
  Init,
  /// Immediate power cut (no graceful shutdown).
  #[serde(rename = "Force-Off")]
  ForceOff,
}

/// Parses a lowercase operation name (e.g. `"on"`, `"soft-off"`) into an [`Operation`].
impl FromStr for Operation {
  type Err = Error;

  fn from_str(operation: &str) -> Result<Operation, Error> {
    match operation {
      "on" => Ok(Operation::On),
      "off" => Ok(Operation::Off),
      "soft-off" => Ok(Operation::SoftOff),
      "soft-restart" => Ok(Operation::SoftRestart),
      "hard-restart" => Ok(Operation::HardRestart),
      "init" => Ok(Operation::Init),
      "force-off" => Ok(Operation::ForceOff),
      _ => Err(Error::Message("Operation not valid".to_string())),
    }
  }
}

impl From<FrontEndOperation> for Operation {
  fn from(value: FrontEndOperation) -> Self {
    match value {
      FrontEndOperation::On => Operation::On,
      FrontEndOperation::Off => Operation::Off,
      FrontEndOperation::SoftOff => Operation::SoftOff,
      FrontEndOperation::SoftRestart => Operation::SoftRestart,
      FrontEndOperation::HardRestart => Operation::HardRestart,
      FrontEndOperation::Init => Operation::Init,
      FrontEndOperation::ForceOff => Operation::ForceOff,
    }
  }
}
impl From<Operation> for FrontEndOperation {
  fn from(val: Operation) -> Self {
    match val {
      Operation::On => FrontEndOperation::On,
      Operation::Off => FrontEndOperation::Off,
      Operation::SoftOff => FrontEndOperation::SoftOff,
      Operation::SoftRestart => FrontEndOperation::SoftRestart,
      Operation::HardRestart => FrontEndOperation::HardRestart,
      Operation::Init => FrontEndOperation::Init,
      Operation::ForceOff => FrontEndOperation::ForceOff,
    }
  }
}

/// Body of a PCS transition request: one operation applied to a set of locations.
#[derive(Debug, Serialize, Deserialize)]
pub struct Transition {
  /// Operation to perform on every listed location.
  pub operation: Operation,
  /// Optional per-task deadline in minutes before PCS gives up.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "taskDeadlineMinutes")]
  pub task_deadline_minutes: Option<usize>,
  /// Targets of this transition.
  pub location: Vec<Location>,
}

impl From<FrontEndTransition> for Transition {
  fn from(value: FrontEndTransition) -> Self {
    Transition {
      operation: Operation::from(value.operation),
      task_deadline_minutes: value.task_deadline_minutes,
      location: value.location.into_iter().map(Location::from).collect(),
    }
  }
}

impl From<Transition> for FrontEndTransition {
  fn from(val: Transition) -> Self {
    FrontEndTransition {
      operation: val.operation.into(),
      task_deadline_minutes: val.task_deadline_minutes,
      location: val.location.into_iter().map(|v| v.into()).collect(),
    }
  }
}

/// Roll-up of per-task states reported by PCS for a transition.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskCounts {
  /// Total number of tasks in the transition.
  pub total: usize,
  /// Tasks in the `new` state (accepted, not yet started).
  pub new: usize,
  /// Tasks currently in progress.
  pub in_progress: usize,
  /// Tasks that failed.
  pub failed: usize,
  /// Tasks that completed successfully.
  pub succeeded: usize,
  /// Tasks whose operation is not supported by the target hardware.
  pub un_supported: usize,
}

impl From<FrontEndTaskCounts> for TaskCounts {
  fn from(value: FrontEndTaskCounts) -> Self {
    TaskCounts {
      total: value.total,
      new: value.new,
      in_progress: value.in_progress,
      failed: value.failed,
      succeeded: value.succeeded,
      un_supported: value.un_supported,
    }
  }
}

impl From<TaskCounts> for FrontEndTaskCounts {
  fn from(val: TaskCounts) -> Self {
    FrontEndTaskCounts {
      total: val.total,
      new: val.new,
      in_progress: val.in_progress,
      failed: val.failed,
      succeeded: val.succeeded,
      un_supported: val.un_supported,
    }
  }
}

/// Per-component task status inside a PCS transition response.
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
  /// Component xname this task targets.
  pub xname: String,
  /// Current task status string as reported by PCS.
  #[serde(rename = "taskStatus")]
  pub task_status: String,
  /// Human-readable description of the task status.
  #[serde(rename = "taskStatusDescription")]
  pub task_status_description: String,
  /// Error message if the task failed.
  pub error: Option<String>,
}

impl From<FrontEndTask> for Task {
  fn from(value: FrontEndTask) -> Self {
    Task {
      xname: value.xname,
      task_status: value.task_status,
      task_status_description: value.task_status_description,
      error: value.error,
    }
  }
}

impl From<Task>
  for manta_backend_dispatcher::types::pcs::transitions::types::Task
{
  fn from(val: Task) -> Self {
    manta_backend_dispatcher::types::pcs::transitions::types::Task {
      xname: val.xname,
      task_status: val.task_status,
      task_status_description: val.task_status_description,
      error: val.error,
    }
  }
}

/// Response body describing a PCS transition and its per-component tasks.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransitionResponse {
  /// Unique identifier assigned by PCS to this transition.
  #[serde(rename = "transitionID")]
  pub transition_id: String,
  /// Timestamp (RFC 3339) when PCS accepted the transition.
  #[serde(rename = "createTime")]
  pub create_time: String,
  /// Timestamp (RFC 3339) after which the transition record is auto-expired.
  #[serde(rename = "automaticExpirationTime")]
  pub automatic_expiration_time: String,
  /// Overall status string reported by PCS for the transition.
  #[serde(rename = "transitionStatus")]
  pub transition_status: String,
  /// Operation this transition is performing.
  pub operation: Operation,
  /// Roll-up counts of tasks by state.
  #[serde(rename = "taskCounts")]
  pub task_counts: TaskCounts,
  /// Per-component task entries.
  pub tasks: Vec<Task>,
}

impl From<FrontEndTransitionResponse> for TransitionResponse {
  fn from(value: FrontEndTransitionResponse) -> Self {
    TransitionResponse {
      transition_id: value.transition_id,
      create_time: value.create_time,
      automatic_expiration_time: value.automatic_expiration_time,
      transition_status: value.transition_status,
      operation: Operation::from(value.operation),
      task_counts: TaskCounts::from(value.task_counts),
      tasks: value.tasks.into_iter().map(Task::from).collect(),
    }
  }
}

impl From<TransitionResponse> for FrontEndTransitionResponse {
  fn from(val: TransitionResponse) -> Self {
    FrontEndTransitionResponse {
      transition_id: val.transition_id,
      create_time: val.create_time,
      automatic_expiration_time: val.automatic_expiration_time,
      transition_status: val.transition_status,
      operation: val.operation.into(),
      task_counts: val.task_counts.into(),
      tasks: val.tasks.into_iter().map(|v| v.into()).collect(),
    }
  }
}

/// Wrapper for a list of transition responses returned by PCS.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransitionResponseList {
  /// Transitions returned by the endpoint.
  pub transitions: Vec<TransitionResponse>,
}

impl From<Vec<FrontEndTransitionResponse>> for TransitionResponseList {
  fn from(value: Vec<FrontEndTransitionResponse>) -> Self {
    TransitionResponseList {
      transitions: value.into_iter().map(TransitionResponse::from).collect(),
    }
  }
}

impl From<TransitionResponseList> for Vec<FrontEndTransitionResponse> {
  fn from(val: TransitionResponseList) -> Self {
    val.transitions.into_iter().map(|v| v.into()).collect()
  }
}
