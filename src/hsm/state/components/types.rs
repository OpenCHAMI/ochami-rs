//! Slim re-exports from the progenitor-generated HSM types. This
//! module is an alias for `/State/Components`; the same generated
//! `Component100Component` schema is re-exported through both here and
//! `crate::hsm::component::types`.
//!
//! Task 13 (Phase 5) collapsed the hand-written duplicate types. Design
//! spec Section 4 (Phase 5) — Types migration.

pub use crate::hsm::generated::types::{
  Component100Component as Component,
  Component100ComponentCreate as ComponentCreate,
  ComponentArrayComponentArray as ComponentArray, ComponentArrayPostArray,
  ComponentArrayPostByNidQuery, ComponentArrayPostQuery,
};
