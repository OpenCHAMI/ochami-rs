//! Slim re-exports from the progenitor-generated HSM types. This
//! module is an alias for `/Defaults/NodeMaps`; the same generated
//! `NodeMap100PostNodeMap` schema is re-exported through both here and
//! `crate::hsm::node_map::types`.
//!
//! Task 13 (Phase 5) collapsed the hand-written duplicate types.
//! Design spec Section 4 (Phase 5) — Types migration.

pub use crate::hsm::generated::types::{
  NodeMap100NodeMap as NodeMap, NodeMap100PostNodeMap as NodeMapPost,
  NodeMapArrayNodeMapArray as NodeMapArray,
};
