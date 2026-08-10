#![deny(unsafe_code)]
#![warn(missing_docs)]

//! Rust bindings for the [OpenCHAMI](https://openchami.org/) backend
//! (HSM, BSS, PCS). See the crate [`README`](https://github.com/OpenCHAMI/ochami-rs)
//! and [`MIGRATION.md`](https://github.com/OpenCHAMI/ochami-rs/blob/main/MIGRATION.md)
//! for usage and version-migration notes.

/// Bearer-token helpers for authenticating against OCHAMI services.
pub mod authentication;
/// Top-level [`Ochami`] client and its backend-dispatcher trait impls.
pub mod backend_connector;
/// Boot Script Service (BSS) client and boot-parameter types.
pub mod bss;
/// Crate-wide error type returned by all fallible OCHAMI operations.
pub mod error;
mod error_convert;
/// Hardware State Manager (HSM) client — components, groups, inventory, state.
pub mod hsm;
/// Shared HTTPS client construction and response-handling helpers.
pub mod http;
/// Node-level helpers built on top of the HSM/BSS clients.
pub mod node;
/// Power Control Service (PCS) client — transitions, power cap, power status.
pub mod pcs;
mod wrapper_common;

pub use backend_connector::Ochami;
