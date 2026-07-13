# Migration Guide: `1.0.0-beta.15` → `1.0.0-beta.16`

`1.0.0-beta.16` replaces the HSM (Hardware State Manager) client's
hand-written HTTP layer with one generated at build time from HSM's
vendored OpenAPI spec, via [`progenitor`](https://docs.rs/progenitor).
This is the single largest change in the crate's history: essentially
all of `src/hsm/` was rewritten. Every OCHAMI consumer directly calling
`ochami_rs::hsm::*::http_client::*` functions, or matching on
`Error::OchamiError`, will need updates.

This document covers, in order:

1. [Breaking API changes](#1-breaking-api-changes) (compile-breaking)
2. [Silent URL bug fixes](#2-silent-url-bug-fixes) (same name, new behavior)
3. [Runtime behavior changes](#3-runtime-behavior-changes)
4. [Latent bugs fixed](#4-latent-bugs-fixed)
5. [Fix-your-code recipes](#5-fix-your-code-recipes)
6. [Downstream cold-build cost](#6-downstream-cold-build-cost)

If you only read one section, read [§5](#5-fix-your-code-recipes).

---

## 1. Breaking API changes

### 1.1 `hsm::*::http_client::*` free functions removed

Every hand-written `hsm::<resource>::http_client::<verb>(url, cert,
token, ...)` free function is gone. All HSM operations are now async
methods on the `Ochami` client, named `hsm_<resource>_<verb>`:

```rust
// beta.15
let comp = hsm::component::http_client::get_one(base_url, root_cert, token, xname).await?;

// beta.16
let comp = ochami.hsm_component_get_one(token, xname).await?;
```

`Ochami` already carries the base URL and root cert (set once via
`Ochami::new`), so those two arguments drop from every call.

### 1.2 `Error::OchamiError(Value)` → `Error::OchamiError { status, body }`

```rust
// beta.15
pub enum Error {
    OchamiError(serde_json::Value),
    // ...
}

// beta.16
#[non_exhaustive]
pub enum Error {
    OchamiError { status: u16, body: serde_json::Value },
    // ...
}
```

The variant now carries the HTTP status code alongside the response
body, and the whole enum is `#[non_exhaustive]` (future variants may be
added without a semver-major bump). Any `match`/`if let` on
`Error::OchamiError` must be updated, and non-exhaustive matches need a
wildcard arm.

### 1.3 `Group::new` / `add_xnames` / `get_members` require `GroupExt`

`Group` is now a slim re-export of the generated `Group100` type, which
has no inherent methods. The pre-migration convenience constructors and
accessors move to an extension trait:

```rust
use ochami_rs::hsm::group::types::GroupExt; // <-- required, orphan rules

let group = Group::new("my-group", Some(vec!["x3000c0s1b0n0"]));
let members: Vec<String> = group.get_members();
```

Without the `use` line, these calls won't resolve (`Group` no longer
implements them inherently — the generated type lives in this crate,
but the convenience methods only exist via the trait).

`Members.ids` also changes shape, from `Option<Vec<String>>` to
`Vec<XNameRw100>`: it's no longer optional (an empty member list is
`vec![]`, not `None`), and each entry is the `XNameRw100` newtype
rather than a raw `String`. Code that does
`group.members.map(|m| m.ids.unwrap_or_default())` will fail to
compile on the inner `.unwrap_or_default()` — drop it — and code that
reads individual members should use `member.ids[i].0` (owned) or
`&*member.ids[i]` (borrowed, via `Deref`) instead of indexing a
`String` directly. `GroupExt::get_members()` / `get_members_opt()`
already do this conversion for you, so callers going through the
trait are unaffected.

### 1.4 `Group.label`: `String` → `ResourceName`

```rust
// beta.15
let label: String = group.label;

// beta.16
use std::str::FromStr;
let label: &String = &*group.label;                 // Deref access
let group = Group { label: ResourceName::from_str("my-group")?, .. };
```

`ResourceName`'s inner `String` is private (typify-generated newtype
with a regex-validated `FromStr`/`Deserialize`). Read access goes
through `Deref<Target = String>`; construction goes through
`ResourceName::from_str()`, which returns `Err` for labels that don't
match `^[0-9a-zA-Z._-]{1,}$`. Same pattern applies to `Partition.name`.
`Group.tags` changes similarly, from `Option<Vec<String>>` to
`Vec<ResourceName>` (no longer optional — an empty tag list serializes
as an empty array, per `#[serde(default, skip_serializing_if =
"Vec::is_empty")]`).

`Group.exclusive_group` changes from `Option<String>` to
`Option<ResourceName>`. Code that reads it as `Option<String>`
directly (e.g. `if let Some(s) = &group.exclusive_group { s.as_str()
}`) will get a type error, since `ResourceName`'s inner `String` is
private. Access it the same way as `label`/`tags` — through
`Deref<Target = String>` (`&**s` or `s.to_string()` inside the
`Option`) — and construct new values with
`ResourceName::from_str("some-group")?`.

### 1.5 `Component` field newtype/enum wrapping

| Field | Old | New |
|---|---|---|
| `id` | `Option<String>` | `Option<XName100>` — `.id.as_ref().map(\|x\| &x.0)` (tuple field is `pub`) |
| `type_` (was `r#type`) | `Option<String>` | `Option<HmsType100>` — `matches!(comp.type_, Some(HmsType100::Node))` |
| `state` | `Option<String>` | `Option<HmsState100>` (closed enum) |
| `flag` | `Option<String>` | `Option<HmsFlag100>` (closed enum) |
| `arch` | `Option<String>` | `Option<HmsArch100>` (closed enum) |
| `class` | `Option<String>` | `Option<HmsClass100>` (closed enum) |
| `net_type` | `Option<String>` | `Option<NetType100>` (closed enum) |
| `role` / `sub_role` | `Option<String>` | `Option<HmsRole100>` / `Option<HmsSubRole100>` (open newtypes around `String`) |
| `nid` | `Option<usize>` | `Option<i64>` (spec is a signed integer; old code misread it) |

All the closed enums (`HmsType100`, `HmsState100`, `HmsFlag100`,
`HmsArch100`, `HmsClass100`, `NetType100`) implement `FromStr` and
`Display`, so `"Node".parse::<HmsType100>()` and
`hms_type.to_string()` work for round-tripping against code that still
wants strings. `HmsType100`/`HmsFlag100`/most others carry an
`Invalid`/`Unknown` sentinel variant used when parsing unrecognized
values from a front-end/dispatcher type.

`ComponentArray.components` also changes from `Option<Vec<Component>>`
to `Vec<Component>` with `#[serde(default)]` — drop any
`.unwrap_or_default()` calls, the field is never `None` at the type
level anymore.

### 1.6 POST return types: `HsmActionResponse` → `Vec<ResourceUri100>`

Affects `hsm_group_post`, `hsm_group_post_member`, `hsm_partition_post`,
`hsm_partition_post_member`. These endpoints' 201 responses have no
`Response.1.0.0`-shaped body in the real HSM spec — they return the
created resource's URI list instead:

```rust
// beta.15 (returned an ad-hoc "action response" struct)
let resp: HsmActionResponse = ochami.hsm_group_post(token, &group).await?;

// beta.16
let uris: Vec<ResourceUri100> = ochami.hsm_group_post(token, &group).await?;
```

### 1.7 Ethernet-interface PATCH/PUT/POST return-type shifts

- `hsm_inventory_ethernet_interface_patch` / `patch_ip_address` now
  return `()` — the endpoints' 200 responses have no body schema.
- `hsm_inventory_ethernet_interface_post` /
  `post_ip_address` now return a singular `ResourceUri100` (not a
  `Vec`, not `HsmActionResponse`).

### 1.8 `delete_all` methods removed (footgun)

`hsm_inventory_hardware_delete_all`,
`hsm_inventory_hardware_by_fru_delete_all`, and
`hsm_inventory_ethernet_interface_delete_all` are **not** ported. Each
underlying endpoint unconditionally deletes every resource of that
kind (one is even spec-tagged `cli_danger`); none had in-crate callers.
If you genuinely need this, call the generated
`hsm::generated::Client::do_*_delete_all_v2` methods directly via
`crate::hsm::wrapper` — there is no safe public wrapper, and the
extension-trait default for callers going through the dispatcher
returns a loud `Err` rather than silently deleting everything.

### 1.9 `hsm_inventory_ethernet_interface_delete_ip_address` dropped `_group_label`

The parameter was unused dead code in the old hand-written function.
New signature: `(token, id, ip_address)` — no group label argument.

### 1.10 Filter-parameter narrowing (endpoints that never supported them)

Several old hand-written functions exposed filter parameters the real
HSM endpoint doesn't accept. These are now correctly narrowed:

- `hsm_group_get`: `tags: Vec<String>` → `tags: Option<&str>` (the
  endpoint's `tag` query param is single-value only — the hand-written
  version's multi-value support never worked server-side).
- `hsm_partition_get` (single-partition lookup): `tags` parameter
  dropped entirely — `GET /partitions/{name}` has no tag filter (tag
  filtering only exists on the list endpoint,
  `hsm_partition_get_all`).
- `hsm_memberships_get`: dropped `enabled`, `softwarestatus`,
  `subtype`, `nid`, `nid_start`, `nid_end` (6 params) — the generated
  `do_memberships_get` endpoint doesn't accept them. Remaining filters:
  `id`, `type_`, `state`, `flag`, `role`, `subrole`, `group`,
  `partition`, `arch`, `class` (10 total).

### 1.11 `hsm_component_get_query` gained comma-split `nid` semantics

Matches `hsm_component_get`'s existing behavior: a comma-separated
`nid` string (e.g. `"1,2,3"`) is expanded into repeated `?nid=`
query params rather than sent as one literal value.

### 1.12 `hsm_component_delete_one` / `hsm_component_delete`: `Value` → `HsmActionResponse`

Both now return the typed `HsmActionResponse` (alias for the
generated `Response100`) instead of an untyped `serde_json::Value`.

### 1.13 `hsm_inventory_hardware_by_fru_delete_one`: `Value` → `HsmActionResponse`

Same shift as 1.12, applied to the FRU-inventory delete endpoint.

### 1.14 `hsm_node_map_put` return type: `()` → `NodeMapPost`

The old code discarded the PUT response body. The generated
`do_node_map_put` genuinely returns the updated `NodeMapPost`
representation, so the typed body is now surfaced instead of thrown
away.

---

## 2. Silent URL bug fixes

These wrapper methods keep their old names but now hit corrected URLs.
If your code (or tests) depended on the *old, broken* URL — unlikely,
since these were bugs — behavior changes silently at the same call
site.

| Method | Old (buggy) path | New (correct) path |
|---|---|---|
| `hsm_component_delete` | `/State/Componnets` (typo) | `/State/Components` |
| `hsm_component_post_query` | `/State/Components` (missing suffix) | `/State/Components/Query` |
| `hsm_inventory_hardware_by_fru_get_one` | `/Inventory/Hardware/{fruid}` | `/Inventory/HardwareByFRU/{fruid}` |
| `hsm_inventory_redfish_endpoint_get_query` (was named `get_query`/`post_query` inconsistently before) | `/Inventory/RedfishEndpoint/Query/{xname}` (singular) | `/Inventory/RedfishEndpoints/Query/{xname}` (plural) |

The redfish-endpoint query method is also a verb correction, not just a
URL fix: the real endpoint is `GET`, not `POST` — see
[§4 item 4](#4-latent-bugs-fixed).

---

## 3. Runtime behavior changes

- **HTTP timeouts added.** All HSM (and shared `http::build_client`)
  requests now set a 45-minute connect timeout and a 15-minute total
  request timeout. Previously there was no timeout — a hung connection
  could block forever.
- **Retry-on-5xx for read (`GET`) methods.** Idempotent read methods
  now retry up to 3 times on a 5xx response, with exponential backoff
  starting at 500ms (500ms, 1000ms, ...). Mutating methods
  (`POST`/`PUT`/`PATCH`/`DELETE`) are not retried, to avoid
  duplicating non-idempotent side effects.
- **`hsm_component_delete_one` request body fixed.** The old
  hand-written code sent an incorrect request body on this DELETE
  call; the generated wrapper sends the correct (empty) request.

---

## 4. Latent bugs fixed

19 latent bugs present in the beta.15 hand-written HTTP client were
fixed as side effects of this migration (see the design spec's
"Latent Bugs" section for full detail):

1. `hsm::component::delete` URL typo `Componnets` → `Components`.
2. `hsm::inventory::hardware::delete_all` missing leading `/` before `hsm` (moot: method removed, see [§1.8](#18-delete_all-methods-removed-footgun)).
3. `hsm::inventory::redfish_endpoint::delete_all` missing leading `/` (also removed upstream of this crate's wrapper).
4. `hsm::inventory::redfish_endpoint::get_query` singular path fixed, and its verb corrected from a guessed `POST` to the real `GET`.
5. `hsm::inventory::ethernet_interfaces::patch` no longer sends bogus query params on what should be a body-only PATCH.
6. `hsm::inventory::ethernet_interfaces::post` dropped a `dbg!` leftover on the error path.
7. `hsm::inventory::ethernet_interfaces::post` no longer tries (and fails) to deserialize a non-empty response body into `()`.
8. `hsm::inventory::ethernet_interfaces::delete_ip_address` drops the unused `_group_label` param and fixes URL casing (`/IpAddress/` → `/IPAddresses/`).
9. `state/components::get`, `inventory/hardware::get`, `partition::get`, `memberships::get`, `defaults/node_map::get` — a positional `.query(&[opt, opt, ...])` bug that silently dropped filters is fixed (named query-param pairs now used throughout).
10. `Component.nid: Option<usize>` corrected to `Option<i64>` (spec is signed).
11. `Component.nid` / `subtype` / `reservation_disabled` — a serialize-only rename bug that broke the wire round-trip (silently deserialized to `None`) is fixed.
12. `ComponentPostByNidQuery`'s `nid_ranger` field-name typo is fixed (generated field is correctly named).
13. `Group.exclusive_group` — same rename-serialize-only silent-deserialize bug, fixed.
14. `ComponentArrayPostQuery.falg_only` Rust-side field typo fixed (generated field is `flagonly`; the typo only ever existed in the separate `manta-backend-dispatcher` crate, which this crate's `From` impl now maps explicitly: `flagonly: value.falg_only`).
15. `hsm::group::post` no longer returns raw `.text()` — it returns a typed `Vec<ResourceUri100>`.
16. `hsm::state::components::post` (wrong body shape sent to a bulk endpoint, plus an empty-body deserialize bug) is **dropped** rather than fixed — no in-crate caller used it correctly, and the bulk-only real endpoint doesn't match the old single-`Component` signature.
17. `hsm::state::components::put` no longer fails deserializing an empty response body (returns `()`).
18. `hsm::component::put` — same empty-body deserialize fix, returns `()`.
19. `hsm::group::post_member` / `hsm::partition::post_members` body-shape mismatch (single item posted to bulk-shaped endpoints) fixed to match the real generated request bodies.

**Known but not fixed by this migration:**

20. `hsm::group::utils` has a pre-existing bug where `.get_members().push(...)` mutates a temporary — `get_members()` returns `Vec<String>` **by value**, so the `push` is immediately discarded. This bug predates and is orthogonal to the HSM client rewrite; it is preserved as-is (shape-only migration) and tracked separately for a future fix.

---

## 5. Fix-your-code recipes

### Recipe 1: replace `http_client` calls with `Ochami` methods

```rust
// Before
use ochami_rs::hsm::component;
let comp = component::http_client::get(base_url, root_cert, token, None, Some("Node"), ...).await?;

// After
let comp = ochami.hsm_component_get(token, None, Some("Node"), ...).await?;
```

### Recipe 2: update `Error::OchamiError` matches

```rust
// Before
match err {
    Error::OchamiError(v) => log::error!("HSM error: {v}"),
    other => return Err(other),
}

// After
match err {
    Error::OchamiError { status, body } => log::error!("HSM error {status}: {body}"),
    other => return Err(other),
}
```
(Because the enum is `#[non_exhaustive]`, external crates must also add
a wildcard `_ =>` arm even if they already handle every variant known
today.)

### Recipe 3: construct/read a `Group`

```rust
// Before
let group = Group { label: "my-group".to_string(), members: Some(vec!["x3000c0s1b0n0".into()]), .. };
println!("{}", group.label);

// After
use ochami_rs::hsm::group::types::GroupExt;
use std::str::FromStr;

let group = Group::new("my-group", Some(vec!["x3000c0s1b0n0"]));
println!("{}", &*group.label); // Deref<Target = String>
```

### Recipe 4: adapt to `Component`'s newtype/enum fields

```rust
// Before
if comp.r#type.as_deref() == Some("Node") { ... }
let n: usize = comp.nid.unwrap();

// After
use ochami_rs::hsm::component::types::HmsType100;
if matches!(comp.type_, Some(HmsType100::Node)) { ... }
let n: i64 = comp.nid.unwrap();
```

### Recipe 5: handle the `Vec<ResourceUri100>` POST return shape

```rust
// Before
let resp: HsmActionResponse = ochami.hsm_group_post(token, &group).await?;

// After
let uris: Vec<ResourceUri100> = ochami.hsm_group_post(token, &group).await?;
let created_uri = uris.first().and_then(|u| u.resource_uri.as_deref());
```

---

## 6. Downstream cold-build cost

Because the HSM client is now generated at build time (`build.rs`
invokes `progenitor` against the vendored, patched OpenAPI spec),
expect roughly **15-25 additional seconds** on a cold/clean `cargo
build` of any crate depending on `ochami-rs` (incremental rebuilds are
unaffected unless the spec or `build.rs` changes). This is a one-time
per-clean-build cost, not a runtime cost.

---

See [`CHANGELOG.md`](CHANGELOG.md) for the condensed per-release
summary, and [`CONTRIBUTING.md`](CONTRIBUTING.md) if you need to
regenerate the client against an updated upstream spec.
