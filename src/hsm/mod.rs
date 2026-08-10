//! Hardware State Manager (HSM) client.
//!
//! HSM is OpenCHAMI's single source of truth for the state of the
//! system: it inventories hardware, tracks component state/role/NID,
//! stores Redfish endpoint connection info, and lets administrators
//! define groups and partitions of components. Everything under this
//! module talks to HSM's REST API (`{base_url}/hsm/v2/...`) on behalf
//! of the top-level [`crate::Ochami`] client.
//!
//! # Codegen pipeline
//!
//! Almost all of this module's wire-level plumbing is generated at
//! build time rather than hand-written:
//!
//! 1. `src/hsm/ochami_api_docs.yaml` — the vendored upstream Swagger
//!    2.0 spec (see its header comment for provenance).
//! 2. `make convert-spec` runs `npx swagger2openapi` to upgrade it to
//!    OpenAPI 3.0, then pipes it through `scripts/patch-hsm-spec.jq`
//!    (a small set of documented jq patches that fix spec defects
//!    progenitor/typify can't tolerate — e.g. missing `operationId`s,
//!    an overly-narrow `ResourceName` regex). Output:
//!    `src/hsm/ochami_api_docs.openapi3.json` (committed).
//! 3. `build.rs` reads that JSON at every build and feeds it to
//!    [`progenitor`](https://docs.rs/progenitor), which emits a full
//!    async HTTP client + typify-generated request/response types into
//!    `$OUT_DIR/hsm_generated.rs`.
//! 4. The (private) `generated` submodule `include!`s that file
//!    verbatim and is `pub(crate)` — nothing outside this crate
//!    touches generated symbols directly.
//! 5. The (private) `wrapper` submodule bridges the generated client to
//!    `impl Ochami { hsm_* }` methods: it injects bearer auth, applies
//!    retry-on-5xx, and maps `progenitor_client::Error` onto this
//!    crate's [`crate::error::Error`]. A handful of methods bypass
//!    progenitor and use raw `reqwest` instead, where the generated
//!    client can't express a needed query shape (see each
//!    `wrapper/*.rs` file's module docs for which, and why).
//!
//! # Submodules
//!
//! - [`types`] / `component::types`, `group::types`, etc. — thin
//!   re-exports of generated types (some slim aliases, some with
//!   hand-written `From`/`Into` conversions to/from
//!   `manta_backend_dispatcher` front-end types).
//! - [`component`], [`group`], [`partition`], [`memberships`],
//!   [`node_map`], [`inventory`], [`state`], [`defaults`] — per-resource
//!   type modules matching HSM's REST resource families.
//!
//! See `MIGRATION.md` at the repository root for the beta.15 → beta.16
//! breaking-change guide covering this module.

pub(crate) mod generated;
pub mod types;

/// HSM component (node) resource: query, create, delete.
pub mod component;
/// HSM defaults resource (node-map defaults applied to new components).
pub mod defaults;
/// HSM group resource: labels, members, tags, per-member add/delete.
pub mod group;
/// HSM inventory resource: hardware, Redfish endpoints, Ethernet interfaces.
pub mod inventory;
/// HSM component memberships (a component's group/partition affiliations).
pub mod memberships;
/// HSM node-map resource (xname → NID/role/subrole mapping).
pub mod node_map;
/// HSM partition resource (mutually-exclusive component groupings).
pub mod partition;
/// HSM component-state changes (transitions between Ready/Off/etc.).
pub mod state;

mod wrapper;
