//! Slim re-exports from the progenitor-generated HSM types.
//!
//! Task 13 (Phase 5) replaced the hand-written duplicate types with
//! direct aliases. `Partition` = `Partition100`; `Members` =
//! `Members100`; `MemberId` (POST body) is the shared single-member
//! shape used by both `/groups/{label}/members` and
//! `/partitions/{name}/members`. Design spec Section 4 (Phase 5) —
//! Types migration.

pub use crate::hsm::generated::types::{
  MemberId, Members100 as Members, Partition100 as Partition,
  Partition100Patch as PartitionPatch,
};
