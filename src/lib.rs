#![deny(unsafe_code)]
#![warn(missing_docs)]

//! Rust bindings for the [OpenCHAMI](https://openchami.org/) backend
//! (HSM, BSS, PCS). See the crate [`README`](https://github.com/OpenCHAMI/ochami-rs)
//! and [`MIGRATION.md`](https://github.com/OpenCHAMI/ochami-rs/blob/main/MIGRATION.md)
//! for usage and version-migration notes.

pub mod authentication;
pub mod backend_connector;
pub mod bss;
pub mod error;
mod error_convert;
pub mod hsm;
pub mod http;
pub mod node;
pub mod pcs;

pub use backend_connector::Ochami;
