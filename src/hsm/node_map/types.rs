//! Slim re-exports from the progenitor-generated HSM types.
//!
//! Task 13 (Phase 5) replaced the hand-written duplicate types with
//! direct aliases. `NodeMap` corresponds to `NodeMap100NodeMap`;
//! `NodeMapArray` to `NodeMapArrayNodeMapArray`; `NodeMapPost` to
//! `NodeMap100PostNodeMap`. Design spec Section 4 (Phase 5) — Types
//! migration.

pub use crate::hsm::generated::types::{
  NodeMap100NodeMap as NodeMap, NodeMap100PostNodeMap as NodeMapPost,
  NodeMapArrayNodeMapArray as NodeMapArray,
};
