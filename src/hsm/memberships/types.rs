//! Slim re-exports from the progenitor-generated HSM types.
//!
//! Task 13 (Phase 5) replaced the hand-written duplicate types with a
//! direct alias to `Membership100`. Note the generated shape differs
//! from the pre-migration hand-rolled `Membership`: `id` is
//! `Option<XName100>`, `partition_name` is `Option<String>`, and
//! `group_labels` is `Vec<String>` (not Option). Design spec
//! Section 4 (Phase 5) — Types migration.

pub use crate::hsm::generated::types::Membership100 as Membership;
