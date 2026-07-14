# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project is pre-1.0 (see [`MIGRATION.md`](MIGRATION.md)'s version
policy); breaking changes may land between `beta.N` releases.

## [Unreleased]

### Breaking Changes

- BSS client migrated to progenitor codegen. Removed
  `bss::http_client::{get, get_all, post, put, patch, delete}` free
  functions; replaced by `Ochami::bss_bootparameters_{get, get_all,
  post, put, patch, delete}` methods.
- `bss_bootparameters_put` returns `Result<(), Error>` (was
  `Result<BootParameters, Error>` — the BSS spec has an empty 200
  response on PUT). Re-fetch via `bss_bootparameters_get` if the
  round-tripped value is needed.
- `bss_bootparameters_delete` returns `Result<(), Error>` (was
  `Result<String, Error>` returning raw response text).
- PCS fully migrated to wrapper methods. Removed all three hand-written
  http_client modules (`pcs::power_status`, `pcs::power_cap`,
  `pcs::transitions`); replaced by `Ochami::pcs_power_status_{get,
  post}`, `pcs_power_cap_{get, task_get, snapshot_post, patch}`,
  `pcs_transitions_{get, post}`, `pcs_transition_{get, delete,
  post_block, wait_to_complete}`. `power_status_post` uses a raw-reqwest
  carve-out (progenitor codegen limitation on the `xname` array query
  parameter — see `scripts/patch-pcs-spec.jq`). The other 10 methods
  use raw reqwest against the hand-rolled trait-boundary types (no
  benefit to routing through the generated client's type shapes).
- Silent bug fixes: `pcs_power_cap_snapshot_post` uses POST (was
  PUT); `pcs_power_cap_patch` uses PATCH on `/power-cap` (was PUT on
  `/power-cap/snapshot`). Callers relying on the wrong verbs will
  see server-side behaviour change.

### Runtime Behavior Changes

- `bss_bootparameters_get*` retry up to 3 times on 5xx with
  exponential backoff (500ms → 1000ms), matching HSM. Mutating
  methods (POST/PUT/PATCH/DELETE) do NOT retry.
- BSS errors surface as `Error::OchamiError { status, body }`,
  matching the HSM improvement from beta.16. Downstream trait
  consumers see `manta::Error::CsmError`.

### Added

- `src/wrapper_common.rs` — shared progenitor `map_err` helper used by
  `hsm::wrapper`, `bss::wrapper`, and `pcs::wrapper`. Service name is
  passed as a parameter so log lines identify which service surfaced
  the error.
- `Makefile` gains `convert-spec-bss` and `convert-spec-pcs` targets;
  the aggregate `convert-spec` target regenerates all three service
  specs.
- `scripts/patch-bss-spec.jq` — jq patches for BSS.
- `scripts/patch-pcs-spec.jq` — jq patches for PCS (synthesises 13
  missing operationIds, downgrades the `non_empty_string_list` schema
  to `type: string` to work around a progenitor codegen limitation,
  and strips non-2xx responses).
- `src/bss/generated.rs` and `src/pcs/generated.rs` include the
  build-time progenitor-generated clients.
- `src/bss/ochami_api_docs.openapi3.json` and
  `src/pcs/ochami_api_docs.openapi3.json` — vendored, committed
  OpenAPI 3.0 spec conversions.
- `uuid` crate now has the `"serde"` feature enabled (required by the
  generated PCS types).

### Fixed

- BSS errors now carry HTTP status codes end-to-end instead of being
  stringified into `Error::Message` at the trait boundary.

## [1.0.0-beta.16] - 2026-07-13

The HSM (Hardware State Manager) client is rewritten: the hand-written
HTTP layer under `src/hsm/` is replaced by a client generated at build
time from HSM's vendored OpenAPI spec via `progenitor`. See
[`MIGRATION.md`](MIGRATION.md) for the full guide, before/after
recipes, and cold-build cost note.

### Breaking Changes

- Removed all `hsm::*::http_client::*` free functions. Replaced by
  `impl Ochami { hsm_* }` async methods.
- `Error::OchamiError(Value)` is now `Error::OchamiError { status: u16,
  body: Value }` and the `Error` enum is `#[non_exhaustive]`.
- `Group::new` / `add_xnames` / `get_members` / `get_members_opt` moved
  to an extension trait, `ochami_rs::hsm::group::types::GroupExt`
  (must be imported to call them — orphan-rule workaround for the
  generated `Group` type).
- `Group.label` and `Partition.name`: `String` → `ResourceName`
  (private inner newtype; read via `Deref<Target = String>`, construct
  via `ResourceName::from_str()`).
- `Group.tags`: `Option<Vec<String>>` → `Vec<ResourceName>`.
- `Component` fields (`id`, `type_`, `state`, `flag`, `arch`, `class`,
  `net_type`, `role`, `sub_role`) are now newtype/enum wrapped
  (`XName100`, `HmsType100`, `HmsState100`, etc.) instead of raw
  `String`. `Component.nid`: `Option<usize>` → `Option<i64>`.
  `ComponentArray.components`: `Option<Vec<Component>>` →
  `Vec<Component>` (`#[serde(default)]`).
- `hsm_group_post`, `hsm_group_post_member`, `hsm_partition_post`,
  `hsm_partition_post_member`: return type changed from
  `HsmActionResponse` to `Vec<ResourceUri100>`.
- `hsm_inventory_ethernet_interface_patch` / `patch_ip_address`: return
  type changed to `()`.
- `hsm_inventory_ethernet_interface_post` / `post_ip_address`: return
  type changed to a singular `ResourceUri100` (was `HsmActionResponse`).
- `hsm_node_map_put`: return type changed from `()` to the typed
  `NodeMapPost` body.
- `hsm_component_delete_one` / `hsm_component_delete`:
  `serde_json::Value` → `HsmActionResponse`.
- `hsm_inventory_hardware_by_fru_delete_one`: `serde_json::Value` →
  `HsmActionResponse`.
- Removed `hsm_inventory_hardware_delete_all`,
  `hsm_inventory_hardware_by_fru_delete_all`,
  `hsm_inventory_ethernet_interface_delete_all` (unconditional
  delete-everything endpoints with no in-crate callers; not ported).
- `hsm_inventory_ethernet_interface_delete_ip_address`: dropped the
  unused `_group_label` parameter.
- `hsm_group_get`: `tags` parameter narrowed from `Vec<String>` to
  `Option<&str>` (the underlying endpoint only ever supported a
  single-value `tag` filter).
- `hsm_partition_get` (single-partition lookup): dropped the `tags`
  parameter (endpoint has no such filter).
- `hsm_memberships_get`: dropped 6 filter parameters (`enabled`,
  `softwarestatus`, `subtype`, `nid`, `nid_start`, `nid_end`) that the
  real endpoint does not accept.

### Runtime Behavior Changes

- Added HTTP timeouts: 45-minute connect timeout, 15-minute request
  timeout (previously unbounded).
- Added retry-on-5xx for read (`GET`) methods: up to 3 attempts,
  500ms exponential backoff. Mutating methods are not retried.
- `hsm_component_get_query` gained comma-split `nid` semantics
  (matching `hsm_component_get`'s existing behavior).
- Silent URL bug fixes (same wrapper method name, corrected path):
  - `hsm_component_delete`: `/State/Componnets` (typo) → `/State/Components`.
  - `hsm_component_post_query`: added missing `/Query` suffix.
  - `hsm_inventory_hardware_by_fru_get_one`: `/Inventory/Hardware/{fruid}` → `/Inventory/HardwareByFRU/{fruid}`.
  - `hsm_inventory_redfish_endpoint_get_query`: `/Inventory/RedfishEndpoint/Query` (singular) → `/Inventory/RedfishEndpoints/Query` (plural); verb corrected from `POST` to `GET`.
- `hsm_component_delete_one` now sends the correct (empty) request
  body; the old hand-written code sent an incorrect body.

### Fixed

19 latent bugs fixed as side effects of the migration (typo'd URLs,
dropped query filters, empty-body deserialize failures, rename-only
serde bugs, and more) — full itemized list in `MIGRATION.md` § 4.

### Added

- `Cargo.toml`: `keywords` and `categories` for crates.io
  discoverability.
- `MIGRATION.md`, `CONTRIBUTING.md` (new).
- Module-level rustdoc for `src/hsm/mod.rs` and every file under
  `src/hsm/wrapper/`.
- Provenance header on the vendored `src/hsm/ochami_api_docs.yaml`.

[1.0.0-beta.16]: https://github.com/OpenCHAMI/ochami-rs/releases/tag/v1.0.0-beta.16
