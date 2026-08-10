pub(crate) mod generated;
/// PCS power-cap resource: per-component power-cap query and update.
pub mod power_cap;
/// PCS power-status resource: per-component power/management state.
pub mod power_status;
/// PCS transitions resource: queued on/off/reset operations.
pub mod transitions;
mod wrapper;
