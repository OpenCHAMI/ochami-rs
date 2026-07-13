# Progenitor output reference — `src/hsm/ochami_api_docs.openapi3.json`

Generated against `progenitor` 0.8.0 via `progenitor::Generator::default()` in
`build.rs` (this crate's own build; no separate smoke crate needed — Task 0's
end-to-end `cargo build` verified codegen against the real crate).

The spec fed to progenitor is the committed
`src/hsm/ochami_api_docs.openapi3.json` (OpenAPI 3.0.0, machine-converted
from the upstream Swagger 2.0 `src/hsm/ochami_api_docs.yaml` via
`swagger2openapi@7.0.8`). Four small jq patches are applied to the converted
JSON before progenitor accepts it; see Section F. All four patches are
committed in `scripts/patch-hsm-spec.jq` and applied by
`make convert-spec`.

All generated types live in `mod types`. The client struct is `Client` at
the module root (`crate::hsm::generated::Client`).

## A. Generated type names

Total: 231 structs + 133 enums = 364 types in `mod types`. Plus one
`ConversionError` newtype in `mod types::error`. Grep
`pub (struct|enum) ` on the generated file
(`target/debug/build/ochami-rs-*/out/hsm_generated.rs`) to enumerate them.

### A.1. Design-spec-listed schemas (Section 3 § "Type mapping")

| YAML schema | Rust path (`types::…`) |
|---|---|
| `Component.1.0.0_Component` | `types::Component100Component` |
| `Component.1.0.0_ComponentCreate` | `types::Component100ComponentCreate` |
| `Component.1.0.0_Put` | `types::Component100Put` |
| `ComponentArray_ComponentArray` | `types::ComponentArrayComponentArray` |
| `ComponentArray_PostArray` | `types::ComponentArrayPostArray` |
| `ComponentArray_PostQuery` | `types::ComponentArrayPostQuery` |
| `ComponentArray_PostByNIDQuery` | `types::ComponentArrayPostByNidQuery` |
| `Group.1.0.0` | `types::Group100` |
| `Group.1.0.0_Patch` | `types::Group100Patch` |
| `Members.1.0.0` | `types::Members100` |
| `Membership.1.0.0` | `types::Membership100` |
| `Partition.1.0.0` | `types::Partition100` |
| `NodeMap.1.0.0` | `types::NodeMap100NodeMap` |
| `NodeMap.1.0.0_PostNodeMap` | `types::NodeMap100PostNodeMap` |
| `NodeMapArray_NodeMapArray` | `types::NodeMapArrayNodeMapArray` |
| `RedfishEndpoint.1.0.0_RedfishEndpoint` | `types::RedfishEndpoint100RedfishEndpoint` |
| `RedfishEndpointArray_RedfishEndpointArray` | `types::RedfishEndpointArrayRedfishEndpointArray` |
| `CompEthInterface.1.0.0` | `types::CompEthInterface100` |
| `CompEthInterface.1.0.0_Patch` | `types::CompEthInterface100Patch` |
| `CompEthInterface.1.0.0_IPAddressMapping` | `types::CompEthInterface100IpAddressMapping` |
| `HMSRole.1.0.0` | `types::HmsRole100` (open newtype `pub struct HmsRole100(pub String);`) |
| `HMSSubRole.1.0.0` | `types::HmsSubRole100` (open newtype `pub struct HmsSubRole100(pub String);`) |
| `HMSState.1.0.0` | `types::HmsState100` (closed enum) |
| `HMSType.1.0.0` | `types::HmsType100` (closed enum) |
| `HMSFlag.1.0.0` | `types::HmsFlag100` (closed enum) |
| `HMSArch.1.0.0` | `types::HmsArch100` (closed enum) |
| `HMSClass.1.0.0` | `types::HmsClass100` (closed enum) |
| `NetType.1.0.0` | `types::NetType100` (closed enum) |
| `XName.1.0.0` | `types::XName100` (newtype `pub struct XName100(pub String);`) |
| `XNameRW.1.0.0` | `types::XNameRw100` (newtype `pub struct XNameRw100(pub String);`) |
| `XNameForQuery.1.0.0` | `types::XNameForQuery100` |
| `XNamePartition.1.0.0` | `types::XNamePartition100` |
| `XNameRfEndpoint.1.0.0` | `types::XNameRfEndpoint100` |
| `XNameCompOrPartition.1.0.0` | `types::XNameCompOrPartition100` |
| `ResourceName` | `types::ResourceName` (newtype `pub struct ResourceName(String);` — inner is **PRIVATE**) |
| `ResourceURI.1.0.0` | `types::ResourceUri100` |
| `Response.1.0.0` | `types::Response100` (used everywhere the design spec calls `HsmActionResponse`) |
| `MemberId` | `types::MemberId` (single-member body for `POST /groups/{label}/members` and `/partitions/{name}/members`) |

Notes on mangling (matches csm-rs HSM Task 0 findings exactly):
- Typify lowercases acronyms: `HMS`→`Hms`, `HW`→`Hw`, `URI`→`Uri`, `IP`→`Ip`,
  `FRU`→`Fru`, `NID`→`Nid`, `RW`→`Rw`.
- `.1.0.0` version suffix collapses to `100` (dots stripped).
- Composite names keep inner parts, each mangled
  (`ComponentArray_PostByNIDQuery` → `ComponentArrayPostByNidQuery`).
- Field names containing acronyms mangle badly:
  `ComponentArrayPostQuery.ComponentIDs` becomes `component_i_ds` (typify
  reads "IDs" as three letters). Wrapper code that reads/writes this field
  must use `component_i_ds`; the wire name stays `ComponentIDs` via
  `#[serde(rename = "ComponentIDs")]`.

### A.2. Schemas the design spec named but that are absent

- **`HsmActionResponse`** — no schema by this name exists in the spec. The
  endpoints the design spec's Section 3 calls "HsmActionResponse" actually
  return `Response.1.0.0` (`types::Response100`) or a per-endpoint concrete
  type. Section 3's `hsm/types.rs` alias line should be:
  `pub use crate::hsm::generated::types::Response100 as HsmActionResponse;`.
  csm-rs HSM Task 0 landed on the same aliasing (§ "Schemas referenced by
  the plan that are NOT generated").

## B. Generated method names

Total: **133 `pub async fn`s on `impl Client`**. 13 of them are the
patched-in `/locks/*` methods (Section F.2); the remaining 120 come from
the upstream spec's operationIds unchanged.

### B.1. Method-naming rule

The spec's operationIds are camelCase (`doComponentsGet`,
`doGroupMembersPost`). Progenitor snake_cases them
(`do_components_get`, `do_group_members_post`). No `_builder` suffix
appears anywhere in the file — see B.3.

### B.2. Design-spec-relevant method names

| Design spec sketch | Actual generated method | HTTP verb + path |
|---|---|---|
| `do_state_components_get` / `hsm_state_components_get` | `do_components_get` | GET `/State/Components` |
| `do_state_components_post` | `do_components_post` | POST `/State/Components` |
| `do_state_components_delete_all` | `do_components_delete_all` | DELETE `/State/Components` |
| `do_state_components_get_one` | `do_component_get` | GET `/State/Components/{xname}` |
| `do_state_components_put` | `do_component_put` | PUT `/State/Components/{xname}` |
| `do_state_components_delete_one` | `do_component_delete` | DELETE `/State/Components/{xname}` |
| `do_state_components_by_nid_get` | `do_component_by_nid_get` | GET `/State/Components/ByNID/{nid}` |
| `do_state_components_post_query` | `do_components_query_post` | POST `/State/Components/Query` |
| `do_state_components_post_bynid_query` | `do_components_by_nid_query_post` | POST `/State/Components/ByNID/Query` |
| `do_groups_get` | `do_groups_get` | GET `/groups` |
| `do_groups_post` | `do_groups_post` | POST `/groups` |
| `do_group_get` | `do_group_get` | GET `/groups/{group_label}` |
| `do_group_delete` | `do_group_delete` | DELETE `/groups/{group_label}` |
| `do_group_patch` | `do_group_patch` | PATCH `/groups/{group_label}` |
| `do_group_labels_get` | `do_group_labels_get` | GET `/groups/labels` |
| `do_group_members_get` | `do_group_members_get` | GET `/groups/{group_label}/members` |
| `do_group_members_put` | `do_group_members_put` | PUT `/groups/{group_label}/members` |
| `do_group_members_post` | `do_group_members_post` | POST `/groups/{group_label}/members` |
| `do_group_member_delete` | `do_group_member_delete` | DELETE `/groups/{group_label}/members/{xname_id}` |
| Partition family | `do_partitions_get`, `do_partitions_post`, `do_partition_get`, `do_partition_delete`, `do_partition_patch`, `do_partition_names_get`, `do_partition_members_get`, `do_partition_members_post`, `do_partition_member_delete` | `/partitions*` |
| `hsm_memberships_get` | `do_memberships_get` | GET `/memberships` |
| `hsm_membership_get` | `do_membership_get` | GET `/memberships/{xname}` |
| Ethernet interface family (10 methods) | `do_comp_eth_interfaces_get_v2`, `do_comp_eth_interface_post_v2`, `do_comp_eth_interface_delete_all_v2`, `do_comp_eth_interface_get_v2`, `do_comp_eth_interface_delete_v2`, `do_comp_eth_interface_patch_v2`, `do_comp_eth_interface_ip_addresses_get_v2`, `do_comp_eth_interface_ip_addresses_post_v2`, `do_comp_eth_interface_ip_address_delete_v2`, `do_comp_eth_interface_ip_address_patch_v2` | `/Inventory/EthernetInterfaces*` |
| RedfishEndpoint family | `do_redfish_endpoints_get`, `do_redfish_endpoints_post`, `do_redfish_endpoints_delete_all`, `do_redfish_endpoint_get`, `do_redfish_endpoint_put`, `do_redfish_endpoint_patch`, `do_redfish_endpoint_delete`, `do_redfish_endpoints_query_get` | `/Inventory/RedfishEndpoints*` |

### B.3. Progenitor 0.8 default codegen emits DIRECT methods, not builders

**This overrides the design spec's Section 2 wrapper sketches**, which
show `.builder().field(x).send().await`. The actual signatures take
positional `Option<T>` args and return
`Result<ResponseValue<T>, Error<E>>` directly.

Example (`do_components_get` — 20 query params):

```rust
pub async fn do_components_get<'a>(
    &'a self,
    arch: Option<types::DoComponentsGetArch>,
    class: Option<types::DoComponentsGetClass>,
    enabled: Option<&'a str>,
    flag: Option<types::DoComponentsGetFlag>,
    flagonly: Option<bool>,
    group: Option<&'a str>,
    id: Option<&'a str>,
    nid: Option<&'a str>,
    // …total 20 Option<T> params
    type_: Option<types::DoComponentsGetType>,
) -> Result<ResponseValue<types::ComponentArrayComponentArray>, Error<()>> { … }
```

Wrapper callsites must pass `None` for every unset param (positional
convention), or wrap in helper functions that fill defaults. The design
spec's sketch:

```rust
c.do_state_components_get_one_builder().xname(xname).send().await
```

should be re-written as:

```rust
c.do_component_get(xname).await
```

Phase 3 wrapper code should be adjusted accordingly. Phase 2's
`wrapper/mod.rs` sketch (§ Section 2) still applies to the `run()`
combinator — that combinator wraps *any* `Future<Output = Result<
ResponseValue<T>, Error<E>>>`, and generated direct methods return
exactly that.

### B.4. Inline enums synthesised from query params

Wherever the spec's query params reference a schema like `HMSType.1.0.0`,
progenitor extracts an operation-specific inline enum with the mangled
name `Do<OpId><ParamName>`. Examples from `do_components_get`:

- `types::DoComponentsGetArch` — corresponds to `HMSArch.1.0.0`.
- `types::DoComponentsGetClass` — corresponds to `HMSClass.1.0.0`.
- `types::DoComponentsGetFlag`
- `types::DoComponentsGetState`
- `types::DoComponentsGetType`

Each such enum has identical variants to its `HMSX.1.0.0` counterpart
but is a distinct type. Wrapper code coercing from `&str` at the
edge uses `.parse()` (each enum implements `FromStr`). The design spec's
"7 enum-typed filter fields on `hsm_memberships_get`" is now 6 (there's
no filter for HMSSubRole because it's an open string; the spec query
has `subrole: Option<&str>`).

### B.5. Duplicate-method spec-conformance findings

The design spec's Section 1 "Duplicate-module handling" hypothesised
duplicate operations in the SPEC (short vs long path prefix). Reality:
the spec declares each endpoint ONCE. The ochami-rs module duplication
(`hsm::component` vs `hsm::state::components`; `hsm::node_map` vs
`hsm::defaults::node_map`) is a Rust-side artefact only — both members
of each pair route to the SAME generated method (`do_components_get`,
`do_node_maps_get`). No spec-level de-duplication needed.

## C. The `Error` enum

Defined in `progenitor-client 0.8.0` and re-exported at the top of
generated file (line 2):

```rust
pub use progenitor_client::{ByteStream, Error, ResponseValue};
```

Because Patch 4 strips all non-2xx and `default` responses (see F.4),
**every generated method returns `Result<ResponseValue<T>, Error<()>>`
— the error-payload type parameter is always the unit type `()`**.
Undocumented status codes and 4xx/5xx responses all take the
`Error::UnexpectedResponse(reqwest::Response)` branch. Wrappers that
need typed Problem7807 details from an error body decode the response
manually in `map_err`:

```rust
UnexpectedResponse(resp) => {
    let status = resp.status().as_u16();
    let bytes = resp.bytes().await.unwrap_or_default();
    match serde_json::from_slice::<serde_json::Value>(&bytes) {
        Ok(body) => Error::OchamiError { status, body },
        Err(_) => Error::Message(…),
    }
}
```

(Matches design spec Section 2 `map_err` sketch.)

Progenitor 0.8's default `Generator` does **NOT** emit `pre_hook`
plumbing. `PreHookError` still exists on the `Error` enum (it's in the
progenitor-client crate) but no generated method can produce it. The
design spec's `map_err` `PreHookError(s) => …` arm is dead code today;
keep it to make the match exhaustive against future re-generation.

## D. Constructor signatures

```rust
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    pub fn new(baseurl: &str) -> Self { … 15-sec default timeouts … }
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self { … }
    pub fn baseurl(&self) -> &String { &self.baseurl }
    pub fn client(&self) -> &reqwest::Client { &self.client }
    pub fn api_version(&self) -> &'static str { "1.0.0" }
}
```

**Implication for the wrapper:** identical to csm-rs. `gen_client()`
MUST use `Client::new_with_client` so the crate's shared
`reqwest::Client` controls the timeout, rustls config, and insecure-TLS
toggle — the built-in 15-second timeout on `Client::new` is too short
for HSM's known-slow inventory endpoints.

`api_version()` returns `"1.0.0"` (unchanged across the Swagger →
OpenAPI conversion).

## E. basePath / servers behaviour

Upstream YAML declared `basePath: /apis/smd/hsm/v2`. swagger2openapi
converted this into:

```json
"servers": [{ "url": "https://sms/apis/smd/hsm/v2" }]
```

Progenitor **does NOT auto-prepend `servers[0].url`**. Every generated
method computes URL as `format!("{}/<path>", self.baseurl)`, where
`<path>` is what's literally under `paths:` in the spec (`/State/Components`,
`/groups`, `/Inventory/EthernetInterfaces`, …). None of them carry the
`/apis/smd/hsm/v2` prefix.

**Baseurl verification for ochami-rs**: the wrapper's `gen_client()`
should construct baseurl as `format!("{}/hsm/v2", client.base_url())`.
Evidence for this URL prefix comes from in-crate `http_client.rs` files
— every module currently exercised by `backend_connector.rs`
constructs URLs as `format!("{}/hsm/v2/…", base_url)` (grep-verified:
`hsm/group/http_client.rs` and every other file uses `/hsm/v2/…`).

**Verification against a real deployment: NOT yet performed.** No
OpenCHAMI SMD deployment docs were consulted for this Task 0, and no
live instance was available to curl. Add to the release checklist:

> **Release checklist item (before publishing beta.16):** curl
> `https://<real-openchami-host>/hsm/v2/State/Components` without a
> token and verify a 401 Unauthorized comes back (confirms path
> resolves). If the real prefix is `/apis/smd/hsm/v2/…` (spec's
> basePath) or otherwise different, update `gen_client()`'s baseurl
> format string in `src/hsm/wrapper/mod.rs`.

## F. Spec patches applied

Four patches are applied to `src/hsm/ochami_api_docs.openapi3.json` by
`scripts/patch-hsm-spec.jq` after swagger2openapi conversion. All four
match — with adjustments for jq syntax — the Python-based patches
csm-rs's HSM Task 0 used
(`csm-rs/docs/superpowers/plans/2026-06-13-progenitor-output-reference.md § F`).
Patches are ordered so each stage's output is a valid OpenAPI 3.0
document.

### F.1. Widen `ResourceName.pattern`

The spec's `ResourceName.pattern = "^[0-9a-f_\-.]{1,}$"` is hex-only —
rejects real HSM labels like `"compute"` or `"worker"` (letters beyond
`f`). Typify enforces the pattern at Deserialize via `regress::Regex`
inside `FromStr`:

```rust
impl FromStr for ResourceName {
    type Err = ConversionError;
    fn from_str(value: &str) -> Result<Self, ConversionError> {
        if regress::Regex::new("^[0-9a-zA-Z._-]{1,}$").unwrap().find(value).is_none() {
            return Err("doesn't match pattern …".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl<'de> Deserialize<'de> for ResourceName {
    // routes through FromStr, so regex is enforced at wire-decode time
}
```

Consequence: **this patch is release-blocking**. Without it,
`Group100.label` (typed `ResourceName`) fails to deserialize whenever
a real group label contains letters beyond `f`. Patched pattern
`"^[0-9a-zA-Z._-]{1,}$"` matches the design spec's Section 3
"ResourceName spec patch" sketch.

### F.2. Synthesise operationIds on `/locks/*` paths (13 ops)

Progenitor 0.8 refuses to codegen operations without an `operationId`
(`UnexpectedFormat("path <path> is missing operation ID")`). The
upstream spec omits operationId on all 13 `/locks/*` endpoints. Follows
csm-rs BSS Task 0 precedent
(`2026-06-13-progenitor-bss-output-reference.md § F.1`). Added
operationIds use camelCase `<verb>Locks<Suffix>`:

| Path | Verb | Added operationId | Generated method |
|---|---|---|---|
| `/locks/reservations/remove` | POST | `postLocksReservationsRemove` | `post_locks_reservations_remove` |
| `/locks/reservations/release` | POST | `postLocksReservationsRelease` | `post_locks_reservations_release` |
| `/locks/reservations` | POST | `postLocksReservations` | `post_locks_reservations` |
| `/locks/service/reservations/release` | POST | `postLocksServiceReservationsRelease` | `post_locks_service_reservations_release` |
| `/locks/service/reservations` | POST | `postLocksServiceReservations` | `post_locks_service_reservations` |
| `/locks/service/reservations/renew` | POST | `postLocksServiceReservationsRenew` | `post_locks_service_reservations_renew` |
| `/locks/service/reservations/check` | POST | `postLocksServiceReservationsCheck` | `post_locks_service_reservations_check` |
| `/locks/status` | POST | `postLocksStatus` | `post_locks_status` |
| `/locks/status` | GET | `getLocksStatus` | `get_locks_status` |
| `/locks/lock` | POST | `postLocksLock` | `post_locks_lock` |
| `/locks/unlock` | POST | `postLocksUnlock` | `post_locks_unlock` |
| `/locks/repair` | POST | `postLocksRepair` | `post_locks_repair` |
| `/locks/disable` | POST | `postLocksDisable` | `post_locks_disable` |

These endpoints have no in-crate consumer today. The operationIds keep
them present as `pub async fn`s on `Client` for future wrapper work
rather than dropped.

### F.3. Drop duplicate `application/problem+json` response content entries

Every response in the spec declared both `application/json` and
`application/problem+json` under `content:` with the exact same
schema. Progenitor's `extract_responses` step treats the two media
types as distinct branches even when their schemas match and trips a
downstream assertion (§ F.4). Dropping `application/problem+json`
leaves `application/json` as the sole content type per response,
which progenitor handles cleanly. csm-rs HSM applied the equivalent
patch (446 entries touched in csm-rs; comparable count here).

Real HSM servers may still respond with `Content-Type:
application/problem+json`. Because Patch 4 strips error responses
anyway, this content-type asymmetry is moot: those responses take
`Error::UnexpectedResponse(resp)` and wrappers decode the body
manually.

### F.4. Strip non-2xx and `default` responses (348 entries)

Progenitor 0.8's success-extractor groups 2xx and `default` responses
and asserts (`progenitor-impl-0.8.0/src/method.rs:1264`,
`response_types.len() <= 1`) that the resulting set of typed responses
has exactly one element. The spec uses `Problem7807` for
`default`/4xx/5xx AND the operation-specific resource schema for 2xx,
so the set has 2 elements on nearly every operation. Codegen panics
before this patch. csm-rs HSM applied the same patch
(2026-06-13-progenitor-output-reference.md § F.3, 348 total entries
stripped).

We keep only 2xx entries. After the patch the spec's `paths[*].[verb].responses`
maps only contain `"200"`, `"201"`, `"202"`, and `"204"` keys (verified
via `jq`).

Effect on generated code: `Error<E>` is `Error<()>` for every method.
Non-2xx responses reach the client as
`Error::UnexpectedResponse(reqwest::Response)`. The wrapper's
`map_err` (Section 2 sketch) handles both documented and undocumented
non-2xx uniformly by inspecting `resp.status()` and decoding the body
JSON manually. Typed 4xx handling can be reintroduced later by
selective per-operation edits — not required for Phase 1 or Phase 3.

## G. Runtime crate dependencies referenced by generated code

| Crate | In Cargo.toml? | Notes |
|---|---|---|
| `progenitor-client` (0.8) | yes | Re-exported at file top: `pub use progenitor_client::{ByteStream, Error, ResponseValue};` |
| `reqwest` (0.12) | yes | Already present |
| `serde` | yes | Already present |
| `serde_json` | yes | Already present |
| `chrono` (0.4, with `serde`) | yes | 9 usages of `chrono::DateTime<chrono::offset::Utc>` (lines: 1541, 5091, 5097, 5318, 23770, 28207, 29762, 32619, 32668). Fields: `last_update`, `created_time`, `expiration_time` ×3, `last_update_time`, `timestamp`, `created`, `last_attempt` |
| `regress` (0.10) | yes | Referenced only by `ResourceName::from_str`'s regex validator |

**Date-time crate decision (Verify list Q13): `chrono`** — confirmed
by grep. No `time::OffsetDateTime` references. Kept the design spec's
default `chrono = { version = "0.4.45", default-features = false,
features = ["clock", "serde"] }` line; no swap to the `time` crate
needed.

**No** references to `uuid::` or `time::` in the generated file. `uuid`
stays as a runtime dep only for the existing non-HSM code paths that
use it.

## H. Typify emission observations

### Newtype inner-field visibility

Mixed across newtypes:

- **`pub String` inner (public)**: `XName100`, `XNameRw100`,
  `XNameForQuery100`, `XNamePartition100`, `XNameCompOrPartition100`,
  `XNameRfEndpoint100`, `HmsRole100`, `HmsSubRole100`.
- **Private `String` inner**: `ResourceName`,
  `RedfishEndpoint100RedfishEndpointMacAddr`.

The distinction correlates with whether the schema has a `pattern:`
constraint. Constrained newtypes (like `ResourceName` with
`^[0-9a-zA-Z._-]{1,}$`) get PRIVATE inner and force construction via
`FromStr` (which enforces the regex). Unconstrained newtypes get PUB
inner. Wrapper/consumer code that needs the string value uses `Deref`
(all newtypes implement `Deref<Target = String>`), or `.parse()` /
`String::from(rn)` to construct.

**Design spec Section 3 impact**: the sketch `Group.label: String →
ResourceName # .label.0 for string access` is WRONG for
`ResourceName`. Correct patterns:

- Read: `&*group.label` or `group.label.deref()` (via Deref).
- Construct: `ResourceName::from_str("compute")?` (or
  `ResourceName::try_from("compute")?`), NOT `ResourceName("compute".into())`.

**XName newtypes are fine as designed** — inner is `pub String`, so
`.id.as_ref().map(|x| &x.0)` reads still work.

### `Default` emissions

**Zero `impl Default for` blocks in the file.** Typify's default
config does NOT emit `Default` for structs. Wrapper code that wants to
construct a "zero" Component needs `..Default::default()` on
`Component100Component` etc. to fail. The design spec's `Group::new`
constructor sketch already uses explicit field initialisation, which
still works.

### `Deref<Target = String>` on newtypes

**Yes for every String-backed newtype:** `XName100`, `XNameRw100`,
`ResourceName`, `HmsRole100`, `HmsSubRole100`,
`RedfishEndpoint100RedfishEndpointMacAddr`, and all other XName
variants. Verified by grep for `impl ::std::ops::Deref for` at each
newtype site.

### Regex validation at Deserialize

**Yes for `ResourceName`** (Deserialize routes through FromStr, which
enforces the regex via `regress::Regex`). See F.1 above.

**No for `XName100`** and other unconstrained newtypes (`FromStr`
returns `Ok(Self(value.to_string()))` with `Err = Infallible`).

### Enum emission

Closed enums (`HmsState100`, `HmsType100`, `HmsFlag100`,
`HmsArch100`, `HmsClass100`, `NetType100`) all get:
- `#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]`.
- `impl Display` (via `write!(f, "…")` per variant).
- `impl FromStr` with `Err = ConversionError` (unknown variants error).
- `impl TryFrom<&str>`, `TryFrom<&String>`, `TryFrom<String>` via
  `.parse()`.
- **No** `IntoEnumIterator` or `strum` support. Wrapper code that
  needs to iterate variants gets it manually.

`HmsState100` variants: `Unknown, Empty, Populated, Off, On, Standby,
Halt, Ready`. Extends the design spec's Section 3 sentinel candidates.

Open newtypes (`HmsRole100`, `HmsSubRole100`) — `FromStr` returns
`Infallible`, so any string is accepted.

## I. Key schema field-shape observations

### `Component100Component` (spec.get: `/State/Components/{xname}`)

```rust
pub struct Component100Component {
    #[serde(rename = "Arch", default, skip_serializing_if = "Option::is_none")]
    pub arch: Option<HmsArch100>,
    #[serde(rename = "Class",       …)] pub class:                Option<HmsClass100>,
    #[serde(rename = "Enabled",     …)] pub enabled:              Option<bool>,
    #[serde(rename = "Flag",        …)] pub flag:                 Option<HmsFlag100>,
    #[serde(rename = "ID",          …)] pub id:                   Option<XName100>,
    #[serde(rename = "Locked",      …)] pub locked:               Option<bool>,
    #[serde(rename = "NetType",     …)] pub net_type:             Option<NetType100>,
    #[serde(rename = "NID",         …)] pub nid:                  Option<i64>,
    #[serde(rename = "ReservationDisabled", …)] pub reservation_disabled: Option<bool>,
    #[serde(rename = "Role",        …)] pub role:                 Option<HmsRole100>,
    #[serde(rename = "SoftwareStatus",     …)] pub software_status: Option<String>,
    #[serde(rename = "State",       …)] pub state:                Option<HmsState100>,
    #[serde(rename = "SubRole",     …)] pub sub_role:             Option<HmsSubRole100>,
    #[serde(rename = "Subtype",     …)] pub subtype:              Option<String>,
    #[serde(rename = "Type",        …)] pub type_:                Option<HmsType100>,
}
```

Notes vs design spec Section 3 "Field-shape changes" table:

- `id` and `state` are declared `required` in the schema but typify
  emits them as `Option<_>` because 2xx response bodies may omit them.
  Design spec assumed `id: Option<XName100>` — correct. `state` is
  ALSO Option (design spec didn't state one way or the other).
- `type_` (spec's `Type`) is `Option<HmsType100>` — matches design spec.
- `nid` is `Option<i64>` — matches design spec (signed → signed shift
  from current `Option<usize>`).
- Full enum coverage: `arch/class/flag/net_type/state/type_` are all
  `Option<HmsX100>` — closed enums, `role/sub_role` are open
  newtypes.

### `Group100` (POST/PUT/GET `/groups`)

```rust
pub struct Group100 {
    pub label: ResourceName,                           // REQUIRED
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "exclusiveGroup", default, skip_serializing_if = "Option::is_none")]
    pub exclusive_group: Option<ResourceName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<Members100>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<ResourceName>,                       // Vec, NOT Option<Vec>
}
```

`tags` is `Vec<ResourceName>` with `#[serde(default,
skip_serializing_if = "Vec::is_empty")]` — an empty Vec elides on
serialize and a missing field defaults to empty on deserialize. Design
spec expected `Option<Vec<ResourceName>>`; actual is `Vec<ResourceName>`.
Consumer impact: drop the `.unwrap_or_default()` when reading tags.

### `Members100` (nested inside `Group100.members`)

```rust
pub struct Members100 {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ids: Vec<XNameRw100>,   // Vec, NOT Option<Vec>
}
```

Confirms the design spec's Verify item "Members100.ids shape
(`Vec<XNameRw100>` vs `Option<Vec<XNameRw100>>`)" — **it is
`Vec<XNameRw100>`**, no Option.

### `Membership100` (GET `/memberships/{xname}`)

```rust
pub struct Membership100 {
    #[serde(rename = "groupLabels", default, skip_serializing_if = "Vec::is_empty")]
    pub group_labels: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<XName100>,
    #[serde(rename = "partitionName", default, skip_serializing_if = "Option::is_none")]
    pub partition_name: Option<String>,
}
```

There is **no `ids` field on `Membership100`** — it has a single
`id: Option<XName100>`. The design spec's Verify item
"Membership100.ids shape" was based on a misreading of the schema;
`Members100.ids` was the intended field (already resolved above).
`group_labels` is `Vec<String>` (not `Vec<ResourceName>` — the schema
doesn't reference `ResourceName` here; this is another candidate for
a follow-up upstream spec fix).

### `MemberId` — POST /members body shape

```rust
pub struct MemberId {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<XNameRw100>,
}
```

Used as the body of both `do_group_members_post` and
`do_partition_members_post`. **SINGLE-member semantics** — resolves the
design spec's Verify item "POST /members body-shape empirically (single
vs bulk)" as **SINGLE**. The current in-crate `hsm/group/http_client.rs`
POST-member helper sends a bulk payload, which the design spec's
Section 2 flagged as buggy (dropped method candidate). Task 3's group
wrapper implements `do_group_members_post` per single-member
semantics; bulk-add helpers layer on top (loop N times).

### `ComponentArrayPostQuery` (POST /State/Components/Query body)

No `falg_only` typo in the current spec — the field is spelled
`flagonly` (single-word, correctly spelled). The design spec's
"`falg_only` → `flag_only`" typo-fix entry (Section 3, Field-shape
changes) DOES NOT APPLY. The spec must have been corrected upstream
between the design draft and Task 0.

`ComponentIDs` field mangles to `component_i_ds` (see A.1 note).

## J. Generated artefact stats

- **Generated file**: `hsm_generated.rs` — 37,664 lines,
  1,547,992 bytes (~1.48 MB).
- **Type count in `mod types`**: 364 (231 structs + 133 enums) + 1
  `ConversionError` newtype in `mod types::error`.
- **Method count on `impl Client`**: 133 `pub async fn`s
  (120 from upstream operationIds + 13 patched in on `/locks/*`).
- **One `pub struct Client` at module root**, plus `new`,
  `new_with_client`, `baseurl`, `client`, `api_version` inherents.
- **Compile-time cost (cold)**: ~+15-25 sec on first `cargo build`.
  Warm builds unaffected. Matches design-spec estimate.

## K. Answers to Task 0 § "Verifies during Phase 1"

| # | Verify item | Finding |
|---|---|---|
| 1 | Baseurl `/hsm/v2` correct? | Unverified against real deployment (see § E). In-crate evidence: all `backend_connector.rs`-exercised modules use `/hsm/v2/…`. **Added to release checklist**. |
| 2 | Upstream spec repo URL | Not researched this task (offline). Likely `github.com/OpenCHAMI/hms-smd` or similar; verify at Section 5 follow-up. |
| 3 | OpenCHAMI spec license | `info.license` field is null in the converted JSON (upstream YAML has no explicit `license:` block). Assume upstream repo LICENSE governs; check when the repo URL is known. |
| 4 | Typify newtype visibility | Mixed (§ H). Constrained newtypes (`ResourceName`) private; unconstrained (XName family, HmsRole, HmsSubRole) public. |
| 5 | Typify `Default` emission | **Zero `impl Default`** (§ H). |
| 6 | Typify `Deref<Target = String>` on newtypes | Yes for every String-backed newtype (§ H). |
| 7 | Regex validation at Deserialize | Yes for `ResourceName` (§ F.1, § H). **ResourceName patch is release-blocking.** No for `XName100` etc. |
| 8 | Exact mangled names | See § A.1, § B.2. `XName100`, `Component100Component`, `HmsType100`, etc. all present. |
| 9 | Duplicate-method spec-conformance | Spec declares each endpoint once; module dedup is Rust-side only (§ B.5). |
| 10 | POST /members body-shape | **SINGLE** (`MemberId { id: Option<XNameRw100> }`) — § I. |
| 11 | `Members100.ids` / `Membership100.ids` shape | `Members100.ids: Vec<XNameRw100>` (no Option); `Membership100` has no `ids` field — has `id: Option<XName100>` and `group_labels: Vec<String>` (§ I). |
| 12 | Wiremock `up_to_n_times` API existence | Not verified this task (wiremock not yet a dep — added in Phase 6). Phase 6 verifies. |
| 13 | Progenitor 0.8 `pre_hook` API | Default `Generator` emits no pre_hook plumbing (§ C). `PreHookError` enum variant exists but is unreachable in current codegen. |
| 14 | Typify date-time crate | `chrono::DateTime<chrono::offset::Utc>` (§ G). Kept `chrono` in Cargo.toml. |
| 15 | swagger2openapi warnings | swagger2openapi 7.0.8 emitted no warnings on stderr for the ochami YAML — clean conversion. |

## L. Design-spec reconciliation notes

Findings that materially affect Section 2 or Section 3 of the design
spec:

1. **Section 2 "Default: route through progenitor" wrapper sketch** —
   the `.builder().field(x).send()` pattern doesn't exist. Wrappers call
   direct methods with positional Option args (§ B.3). The `run()`
   combinator design remains valid; only the callsite shape inside `op`
   closures changes:
   - Before: `c.do_state_components_get_one_builder().xname(xname).send().await`
   - After:  `c.do_component_get(xname, None).await`

2. **Section 2 duplicate-method count** — the spec-level "duplicate
   endpoint" concern doesn't exist (§ B.5). Rust module aliasing
   (component ≡ state/components, node_map ≡ defaults/node_map) is
   fine — both aliases route to the SAME generated method. Zero method
   drops needed at spec level.

3. **Section 3 `HsmActionResponse` alias target** — no schema by that
   name. Alias `Response100` instead (§ A.2).

4. **Section 3 `ResourceName` access pattern** — inner is private;
   `.label.0` DOESN'T COMPILE. Read via `Deref` (`&*group.label`);
   construct via `ResourceName::from_str()` / `try_from()` (§ H).

5. **Section 3 `Group.tags` shape** — `Vec<ResourceName>`, not
   `Option<Vec<ResourceName>>`. Consumers drop `.unwrap_or_default()`.

6. **Section 3 `Membership100.ids` question** — moot (Membership100
   has no `ids` field; it's `Members100.ids: Vec<XNameRw100>`).

7. **Section 3 `falg_only` typo-fix** — doesn't apply; already spelled
   `flagonly` in the spec (§ I).

8. **Section 2 `hsm_memberships_get`'s "7 in-body enum coercion
   fields"** — actually 6 (arch, class, flag, state, type — plus
   `role` is a &str not enum since HmsRole is open). Adjust wrapper
   sketch accordingly.

Non-material findings:
- Method-name mangling followed csm-rs precedent (`doComponentsGet` →
  `do_components_get`, etc.), so any wrapper written in Phase 3 can
  follow the same convention.
- `chrono` was the correct default; no crate swap.

Structural rework (routing decisions changing per resource — e.g.,
ethernet_interfaces types can't be slim-re-exported after all) NOT
found in this Task 0. Phase 3 slim-re-export plan for
`ethernet_interfaces` and `redfish_endpoint` is still viable — the
generated types are drop-in for the hand-rolled hsm/inventory/types.rs
subset used by those two families.

## M. Release checklist (accumulated)

- [ ] **Verify baseurl against real OpenCHAMI**: curl
      `https://<host>/hsm/v2/State/Components` without token; expect 401.
      If real prefix differs, patch `gen_client()`'s baseurl format.
- [ ] **Watch for upstream ResourceName pattern fix**: if OpenCHAMI
      updates the schema to accept letters beyond `f`, drop the F.1
      jq patch.
- [ ] **Watch for upstream `/locks/*` operationId fix**: drop F.2 jq
      patch when upstream adds them.
- [ ] **Watch for progenitor 0.8+ handling `default:` + typed 4xx**:
      if a future progenitor release relaxes the
      `response_types.len() <= 1` assertion, drop F.3 and F.4 to
      surface typed 4xx handling.
- [ ] **File spec provenance in `README.md`** — upstream repo URL,
      commit/tag the vendored YAML derives from, and license text.
      Deferred to Phase 7's docs pass.

## N. Progenitor iteration count for this Task 0

- Iteration 1: raw swagger2openapi output → `missing operation ID` on
  `/locks/*`. Added F.2 patch.
- Iteration 2: `assertion failed: response_types.len() <= 1` on
  `progenitor-impl-0.8.0/src/method.rs:1264`. Suspected content-type
  duplication first — added F.3 patch (drop `application/problem+json`).
- Iteration 3: same assertion; the content-type dedup wasn't enough.
  Added F.4 patch (strip non-2xx and default responses) per csm-rs
  HSM Task 0 precedent.
- Iteration 4 (transient): first draft of F.3 introduced
  `"content": null` on responses without content (jq `|=` on missing
  path creates the field). Fixed by guarding with `has("content")`.
- Iteration 5: green. `cargo build` finishes in ~9 seconds warm.

Total: 4 jq patches added, 5 `cargo build` iterations. Well under the
brief's escalation threshold (>3 progenitor-error iterations OR >8
focused hours).
