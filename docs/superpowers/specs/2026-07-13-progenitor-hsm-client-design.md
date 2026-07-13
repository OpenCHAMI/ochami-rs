# Design: Progenitor-Generated HSM HTTP Client for ochami-rs

**Date**: 2026-07-13
**Author**: Manuel Sopena Ballesteros (@masber)
**Status**: Draft — pending user review

## Contents

- [Summary](#summary)
- [Goals](#goals)
- [Non-Goals](#non-goals)
- [Alternatives Considered](#alternatives-considered)
- [Section 1 — Codegen Architecture & Module Layout](#section-1--codegen-architecture--module-layout)
- [Section 2 — Per-Method Routing & Wrapper Design](#section-2--per-method-routing--wrapper-design)
- [Section 3 — Types & Dispatcher Conversions](#section-3--types--dispatcher-conversions)
- [Section 4 — Execution Plan & Rollout](#section-4--execution-plan--rollout)
- [Section 5 — Follow-Up Work & Long-Term Maintenance](#section-5--follow-up-work--long-term-maintenance)
- [Risks](#risks)
- [Latent Bugs](#latent-bugs)
- [Open Questions Deferred to Task 0](#open-questions-deferred-to-task-0)
- [Appendix: Cargo.toml diff](#appendix-cargotoml-diff)

## Summary

Replace the ~11 hand-written `hsm/**/http_client.rs` modules in ochami-rs with a
progenitor-generated HTTP client derived from the OpenAPI spec at
`src/hsm/ochami_api_docs.yaml`. Move from free-function-per-endpoint surface
(`hsm::component::http_client::get(base_url, root_cert, token, …)`) to
`impl Ochami { pub async fn hsm_*() }` methods, matching the reference pattern
established by csm-rs.

Ship as `1.0.0-beta.16`. Breaking API change; ~40 backend_connector.rs sites
rewrite. Migration fixes ~19 latent bugs in the current code as side effects
(URL typos, silent-deserialize failures, filter-dropped queries).

Estimated solo-maintainer effort: **6–10 weeks wall-clock** (86-136 hours
focused work per the phase table below, distributed over normal workload).

## Goals

- Automate HSM HTTP client generation from the vendored OpenAPI spec.
- Establish a schema-authoritative type system (typed newtypes for xnames,
  closed enums for state/type/flag, typed error responses).
- Fix known latent bugs in the current hand-written code.
- Set up the pattern for future BSS/PCS migrations.
- Preserve public method surface where possible; document breaking changes.

## Non-Goals

- End-to-end integration tests against a live OCHAMI server.
- BSS, PCS, and cloud-init migrations (deferred; see Section 5's "Extend
  to BSS / PCS / cloud-init" for per-service starting points). Validated
  2026-07-13:
  - All 3 services have vendored specs at
    `src/<service>/ochami_api_docs.yaml`.
  - PCS is already OpenAPI 3.0.0 (no swagger→OpenAPI conversion step
    needed when migrating).
  - BSS is Swagger 2.0 (needs conversion like HSM).
  - cloud-init spec is anomalous (starts with `definitions:`, no
    `swagger:` / `openapi:` header) — needs spec-fix pre-work.
  - Phase 2's Error enrichment only touches HSM and PCS emission sites
    (BSS uses `Error::RequestError` / `NetError` / `Message`, not
    `Error::OchamiError`; cloud-init has no Rust code today, only the
    spec YAML).
- Setting up CI (no CI exists today; follow-up ticket).
- Feature-gating `manta-backend-dispatcher` (follow-up).
- Retry/timeout added to non-HSM code paths (out of scope).
- Exposing all ~124 spec endpoints as wrappers (only ~55-63 currently-public
  methods; the remaining ~60-70 sit in `pub(crate) mod generated` for future
  contribution).

---

## Alternatives Considered

**Keep hand-written `http_client.rs` modules, fix bugs manually.**
Rejected: doesn't establish a schema-authoritative type system; ~20 latent
bugs would need one-by-one fixes with no guarantee against future drift.
User explicitly requested progenitor-based generation.

**Alternative code generator: openapi-generator (Java).**
Rejected: heavy JVM dependency; Java-driven codegen is awkward in a Rust-only
project; produces less-idiomatic Rust than progenitor/typify.

**Alternative code generator: paperclip / oas3-codegen / other Rust
generators.**
Rejected: less mature than progenitor; smaller user base; no existing
in-house pattern (csm-rs uses progenitor).

**Progenitor via proc-macro (`progenitor::generate_api!(...)`).**
Rejected: build-script approach (used here, matching csm-rs) keeps generated
Rust in `$OUT_DIR` and out of the source tree, gives clearer control over
progenitor's config, and avoids proc-macro compilation costs at the include
site.

**Commit the generated `hsm_generated.rs` under `src/`.**
Rejected: generated code drifts every progenitor version bump; would require
review noise on every dep update. Keeping in `$OUT_DIR` regenerates cleanly.

**Vendor OpenAPI 3.0 JSON only (drop the Swagger 2.0 YAML).**
Rejected: loses upstream traceability. Both files committed; jq patches
maintain the JSON deterministically from the YAML.

---

## Section 1 — Codegen Architecture & Module Layout

### Codegen pipeline

Mirror csm-rs's proven pattern:

1. **Manual conversion (developer step, committed output)**: `make convert-spec`
   runs `npx --yes swagger2openapi@7.0.8 src/hsm/ochami_api_docs.yaml` piped
   through `jq -f scripts/patch-hsm-spec.jq` to produce
   `src/hsm/ochami_api_docs.openapi3.json`. Both YAML (upstream) and JSON
   (post-conversion + patches) committed. Re-run only when the YAML changes.

2. **Automatic build-time codegen**: `build.rs` reads the OpenAPI 3.0 JSON,
   wraps `spec.info.description` in a ```text``` fence (mandatory — the
   description contains state-transition markdown that trips rustdoc), calls
   `progenitor::Generator::default().generate_tokens(&spec)`, pretty-prints via
   `prettyplease`, writes `$OUT_DIR/hsm_generated.rs`. Emits
   `cargo:rerun-if-changed=` for build.rs and the JSON. Generated Rust code is
   **not committed** — regenerated per build.

3. **Include site**: `src/hsm/generated.rs` = one file:
   ```rust
   #![allow(dead_code, clippy::all, missing_docs, non_camel_case_types, non_snake_case, unused_imports)]
   include!(concat!(env!("OUT_DIR"), "/hsm_generated.rs"));
   ```
   Declared as `pub(crate) mod generated;` in `src/hsm/mod.rs`. Only the
   wrapper layer and per-resource `types.rs` re-exports touch it.

### Baseurl override

`gen_client()` (wrapper helper) constructs the progenitor `Client` with
baseurl overridden to `{base_url}/hsm/v2` — matches every module currently
exercised by `backend_connector.rs`. **Assumption to verify at Task 0**: this
URL prefix is correct for real OCHAMI deployments (spec's basePath is
`/apis/smd/hsm/v2` but real OCHAMI SMD listens directly at `/hsm/v2/…` on its
own port; verify against OpenCHAMI deployment docs or a live instance).

Failure mode if assumption wrong: pivot the wrapper's baseurl decision.
Late-discovery risk = big rework. **Verify in the first 30 minutes of Phase 1.**

### Duplicate-module handling

The current crate has two conflicting URL-prefix conventions:
- Short prefix (`/hsm/v2/…`): `component`, `group`, `inventory/{hardware,
  redfish_endpoint, ethernet_interfaces}`, `node_map`, `partition`. All
  exercised by backend_connector.rs.
- Long prefix (`/smd/hsm/v2/…`): `state/components`, `defaults/node_map`,
  `memberships`, `inventory/hardware_by_fru`. No in-crate callers.

Duplicate pairs targeting the same endpoint with different prefixes:
- `hsm::component` ≡ `hsm::state::components` (both `/State/Components`).
- `hsm::node_map` ≡ `hsm::defaults::node_map` (both `/Defaults/NodeMaps`).

**Decision**: keep all module paths post-migration; both members of each pair
re-export from the same generated types. Wrapper file per endpoint family:
one `wrapper/component.rs` serves both `hsm::component` and
`hsm::state::components`. External importers of either path keep compiling;
URL prefix converges on `/hsm/v2`.

### Module layout under `src/hsm/`

```
ochami_api_docs.yaml            (Swagger 2, upstream)
ochami_api_docs.openapi3.json   (converted, committed)
generated.rs                    (include!'d; pub(crate))
types.rs                        (NEW: shared HsmActionResponse alias + dispatcher From/Into)
wrapper/
  mod.rs                        (gen_client, map_err, run helpers)
  component.rs                  (serves hsm::component AND hsm::state::components)
  group.rs
  memberships.rs
  node_map.rs                   (serves hsm::node_map AND hsm::defaults::node_map)
  partition.rs
  inventory_hardware.rs
  inventory_hardware_by_fru.rs
  inventory_redfish_endpoint.rs
  inventory_ethernet_interfaces.rs
component/{mod.rs, types.rs}    (types.rs = slim re-export)
state/components/{mod.rs, types.rs}
group/{mod.rs, types.rs, utils.rs, hacks.rs, tests.rs}
memberships/{mod.rs, types.rs}
node_map/{mod.rs, types.rs}
defaults/node_map/{mod.rs, types.rs}
partition/{mod.rs, types.rs}
inventory/
  types.rs                      (2767 LOC; STAYS hand-rolled)
  ethernet_interfaces/{mod.rs, types.rs}   (slim re-export IF Task 0 confirms shape match)
  hardware/{mod.rs}             (uses inventory/types.rs)
  hardware_by_fru/{mod.rs}      (uses inventory/types.rs)
  redfish_endpoint/{mod.rs, types.rs}      (slim re-export IF Task 0 confirms shape match)
```

11 `hsm/**/http_client.rs` files DELETED. Replaced by 10 wrapper files.

### `Ochami` struct

Add `pub(crate) fn base_url(&self) -> &str` and `pub(crate) fn root_cert(&self)
-> &[u8]` accessors. Wrapper helpers reach into these.

Also add `pub use backend_connector::Ochami;` in `src/lib.rs` so consumers can
write `ochami_rs::Ochami` (shorter than
`ochami_rs::backend_connector::Ochami`). Backwards-compatible.

`Ochami::new(base_url: &str, root_cert: &[u8]) -> Self` stays infallible
(no cert validation at construction; per-call `http::build_client(root_cert)`
validates).

### Error enum enrichment

Change `Error::OchamiError(Value)` to
`Error::OchamiError { status: u16, body: Value }` with `#[non_exhaustive]`.
Required so `retry_on_5xx` can match on status. Downstream: no destructure
sites found in the known consumer set (ochami-rs, manta,
manta-backend-dispatcher) via grep — other crates.io consumers unverified.
Breakage confined to richer Display format for verified consumers; unknown
downstream may need destructure updates.

Update the 5 in-crate PCS `Error::OchamiError(payload)` emission sites to the
new form (mechanical). PCS wrapper migration remains out of scope; only
Error emission updated.

Also add `#![deny(unsafe_code)]` at crate root — hygiene. Progenitor emits no
unsafe code.

### Deps (Cargo.toml)

```toml
[package]
version = "1.0.0-beta.16"
rust-version = "1.91.1"
resolver = "2"
readme = "README.md"
keywords = ["hpc", "ochami", "openchami", "hsm"]
categories = ["api-bindings", "asynchronous"]

[package.metadata.cargo-machete]
ignored = ["regress"]

[dependencies]
# ...existing deps mostly unchanged; extend tokio.features with "time":
tokio = { version = "1.45", features = ["macros", "rt-multi-thread", "time"] }  # +"time" for retry_on_5xx (Phase 2)
progenitor-client = "0.8"
regress = "0.10"                # typify format validators (referenced only via generated code)
http = "1.3.1"                  # progenitor-client's error types
chrono = { version = "0.4.45", default-features = false, features = ["clock", "serde"] }  # 9 date-time fields in spec

[build-dependencies]
progenitor = "0.8"
openapiv3 = "2"
serde_json = "1"
prettyplease = "0.2"
syn = "2"

[dev-dependencies]
wiremock = "0.6"
```

No `include = [...]` (relies on git-tracked defaults, matches csm-rs). No
`serde_yaml` in build-deps (spec is JSON). Follow-up: drop the unused
`serde_yaml` runtime dep (grep already confirmed zero uses in `src/`).

### `src/http.rs` growth

Add:
- `handle_json_response<T: DeserializeOwned>(response: reqwest::Response) -> Result<T, Error>` — collapses ~30 duplicated error-mapping blocks.
- `handle_action_response(response: reqwest::Response) -> Result<HsmActionResponse, Error>` — same for action-response methods.
- `retry_on_5xx<F, Fut, T>(op: F) -> Result<T, Error>` — 3 attempts, backoff 500ms → 1s.
- `HTTP_CONNECT_TIMEOUT = Duration::from_mins(45)` and `HTTP_REQUEST_TIMEOUT = Duration::from_mins(15)` applied to `build_client`.
- `build_client` also calls `.use_rustls_tls()` (existing behavior).

### Spec-patching mechanism

`scripts/patch-hsm-spec.jq` — Makefile pipes swagger2openapi output through jq
to produce the final committed JSON. Each patch is a jq expression with a
comment documenting rationale. First patch (identified in this design):

```jq
# Widen ResourceName pattern to accept real HSM labels ("compute", "worker" —
# spec's ^[0-9a-f_\-.] rejects letters beyond 'f'). Server-side may still
# enforce narrower; this patch fixes client-side deserialization.
.components.schemas.ResourceName.pattern = "^[0-9a-zA-Z._-]{1,}$"
```

Additional patches likely added during Task 0 as progenitor errors surface.

### Phase 1 (Task 0) detail

**Purpose**: run progenitor end-to-end against the converted **HSM** spec
(this migration's scope; see Non-Goals for BSS/PCS/cloud-init deferral);
capture generated type/method names; identify additional spec patches.

**Timebox**: target 6–8 hours (per-step breakdown below). Per-step overruns
at individual boundaries are expected (build.rs iteration is the usual
culprit); ESCALATE only when TOTAL Task 0 time reaches 8h OR when >3 spec
patches are needed.

**Per-step time budget** (best-case ~6.25h; loose upper 9.25h — hit
escalation trigger before the loose upper is reached):
- Baseurl verification: 0.5h.
- swagger2openapi run + first jq patch: 0.75h.
- `build.rs` skeleton + iteration until `cargo build` succeeds: 2–3h.
- Inspect generated code (`$OUT_DIR/hsm_generated.rs`): 1–2h.
- Write `docs/superpowers/plans/<phase-1-commit-date>-progenitor-hsm-output-reference.md`: 1–2h.
- Design reconciliation review (see below): 1h.

**Prerequisites** (verify before starting Phase 1):
- Rust 1.91.1+ (matches the `rust-version` in Cargo.toml this migration
  will set).
- `npm` / `npx` installed (for `npx --yes swagger2openapi@7.0.8`).
- `jq` installed (for `jq -f scripts/patch-hsm-spec.jq`).
- Internet access to npm registry (first `npx --yes` fetches the package).
  Firewalled environments: pre-install swagger2openapi via
  `npm install -g swagger2openapi@7.0.8`.

**First subtask (30 minutes)**: verify baseurl assumption
(`{base_url}/hsm/v2`) against OpenCHAMI SMD deployment docs OR by curling a
live instance's `/hsm/v2/State/Components` endpoint (expect 401 without
token, confirming the path).

**Fallback if baseurl cannot be verified** (no findable docs AND no
accessible live instance): proceed on the in-crate evidence that every
`backend_connector.rs`-exercised module uses `/hsm/v2`. Document in the
output-reference doc that the assumption is unverified and add a bold entry
to the release checklist to verify against a real deployment before
publishing beta.16.

**Deliverables** (all in a single commit for Phase 1):
- Committed `src/hsm/ochami_api_docs.openapi3.json`.
- `scripts/patch-hsm-spec.jq` with all identified patches. **Idempotency
  requirement**: all patches must use assignment (`=`), NOT append (`+=`)
  or in-place mutation, so re-running `make convert-spec` on unchanged
  input YAML produces byte-identical output. Verify by running twice
  and diffing.
- `Makefile` with `convert-spec` target.
- `Cargo.toml` `[build-dependencies]` added (progenitor, openapiv3,
  prettyplease, syn, serde_json).
- **`Cargo.toml` `[dependencies]` for runtime-side of generated code**:
  `progenitor-client = "0.8"`, `regress = "0.10"`, `http = "1.3.1"`,
  and a date-time crate. Default: `chrono = { version = "0.4.45", default-features = false, features = ["clock", "serde"] }` (matches csm-rs precedent). **Task-0-conditional**:
  if typify's output references `time::OffsetDateTime` instead of
  `chrono::DateTime<Utc>` (per Verify list below), swap `chrono` for
  `time = { version = "0.3", features = ["serde"] }` and drop the
  chrono line. `cargo build` failure would surface this immediately.
- `build.rs` written (adapts csm-rs's pattern: openapiv3 parse, description
  ```text``` wrap, progenitor generate, prettyplease format, write to
  `$OUT_DIR/hsm_generated.rs`).
- **`src/hsm/generated.rs`** with `#![allow(...)]` and
  `include!(concat!(env!("OUT_DIR"), "/hsm_generated.rs"));`.
- **`pub(crate) mod generated;`** added to `src/hsm/mod.rs`.
- `docs/superpowers/plans/<phase-1-commit-date>-progenitor-hsm-output-reference.md` — catalog of generated type
  names, method names, enum variants, typify config observations. **Use
  csm-rs's `docs/superpowers/plans/2026-06-13-progenitor-bss-output-reference.md`
  as the format template** (copy structure, fill for HSM). csm-rs repo:
  `github.com/eth-cscs/csm-rs`. `<phase-1-commit-date>` format: ISO
  `YYYY-MM-DD` matching this spec's filename convention.

**Success criteria**:
- `cargo build` succeeds — **verifies both build.rs AND typify's generated
  code compile end-to-end**. Any typify-output-validity bug surfaces here,
  in Task 0's escalation window, not later in Phase 2.
- `cargo doc --no-deps` succeeds (catches rustdoc-doctest failures on the
  generated module's docstrings).
- `cargo test` still passes — no regression on existing tests (e.g.,
  `hsm/group/tests.rs`) from new deps or module declarations.
- Output-reference doc committed with all Verifies-list items answered
  (or marked unverified with rationale).
- `make convert-spec` is idempotent: running it twice on the unchanged
  YAML produces byte-identical JSON (verify with `make convert-spec &&
  git diff --exit-code src/hsm/ochami_api_docs.openapi3.json`).

**Commit message convention**: `chore(hsm): Task 0 — vendor OpenAPI 3.0
JSON + progenitor output reference`. Body references
`docs/superpowers/specs/2026-07-13-progenitor-hsm-client-design.md § Section 1 (Task 0)`.

**Verifies during Phase 1**:
- Baseurl (`/hsm/v2`) is correct against real OCHAMI (or documented as
  unverified per the fallback above).
- Upstream spec repo URL (for future syncing).
- OpenCHAMI spec license compatibility with ochami-rs's Apache-2.0.
- typify's newtype inner-field visibility (`pub String` vs private).
- typify's `Default` emission per struct.
- typify's `Deref<Target = String>` on newtypes.
- Whether typify enforces regex validation at Deserialize (affects
  ResourceName-patch criticality).
- Exact mangled names for `XName100`, `Component100Component`, etc.
- Per-duplicate-method spec-conformance (identifies methods to drop).
- POST /members body-shape empirically (single vs bulk).
- `Membership100.ids` shape (`Vec<XNameRw100>` vs `Option<Vec<XNameRw100>>`).
- `Members100.ids` shape (same question for Group/Partition Members type).
- Wiremock's `up_to_n_times` API for retry tests.
- Progenitor 0.8's per-request pre_hook API existence (informs Section 5
  shared-pool follow-up).
- `cargo doc --no-deps` succeeds on generated code.
- typify's emitted date-time type for the 9 `format: date-time` fields —
  `chrono::DateTime<Utc>` (default assumption; csm-rs precedent) or
  `time::OffsetDateTime`. Determines which runtime dep is added per the
  Deliverables list above (chrono default; swap to `time` if Task 0
  finds otherwise).
- Capture and review swagger2openapi's stderr warnings — they flag
  constructs that survived conversion but might trip progenitor. Read them
  before iterating on build.rs errors; they may explain the errors.

**Contingency for progenitor upstream bugs**: if the generator emits invalid
Rust (typify bug, unsupported spec construct not fixable via jq patch),
options in order of ACTUAL preference:

1. **Drop the affected endpoint(s)** from the wrapper surface (default for
   NON-CRITICAL endpoints). Document in MIGRATION.md as "not yet migrated."
   Wrapper caller falls back to raw reqwest if the endpoint is still needed
   locally. No user escalation.
2. **File upstream progenitor issue** with a minimal reproducer (default for
   CRITICAL endpoints). Wait or contribute a fix. Blocks migration until
   resolved — escalate to user for a wait-vs-workaround decision.
3. **Post-generation patch** via a build.rs step editing tokens before
   `syn::parse2`. Fragile; last resort. REQUIRES user escalation before
   choosing this path.

**Definition of "critical"**: endpoint is called from `backend_connector.rs`
(i.e., the crate's own trait impls depend on it). Non-critical = only
present in the spec but no in-crate consumer. Task 0's per-duplicate-method
audit informs this classification.

**Design-update process if findings invalidate Sections 2 or 3**: if Task 0
reveals significant divergence (e.g. XName not emitted as newtype;
typify's naming convention differs from csm-rs precedent), update the design
spec doc **in-place** as part of the same Phase 1 commit — edit the affected
sections directly, add a brief note in the commit message describing what
changed vs the pre-Phase-1 draft. Larger structural rework (e.g. routing
decisions change per resource) triggers a checkpoint with the user before
proceeding to Phase 2 — checkpoint format: whatever channel works with
@masber (PR comment if the migration branch has one, otherwise Slack or
email), summarizing the divergence and proposed adjustment.

### Compile-time cost

Cold `cargo build` grows ~15–25 seconds for progenitor + typify + prettyplease
codegen. Warm builds unaffected. Downstream consumers (Manta) pay same on
their cold builds. Not a blocker.

### Vendored spec size

~2 MB total in the published `.crate` tarball (YAML 436 KB + JSON 1–1.5 MB).
Cargo max is 10 MB. Comfortable.

---

## Section 2 — Per-Method Routing & Wrapper Design

### Default: route through progenitor

Each wrapper method calls the generated `progenitor::Client` via the shared
`run()` helper. Wrapper adapts input types (path params, body types),
constructs the generated client per-call, awaits the future, unwraps
`ResponseValue<T>`, maps `progenitor_client::Error<E>` to crate `Error`.

Boilerplate is centralized in `wrapper/mod.rs`:

```rust
pub(crate) fn gen_client(client: &Ochami, token: &str) -> Result<generated::Client, Error> {
    // Task 0 confirms whether progenitor's Client::new_with_client accepts
    // a pre-built reqwest client with default bearer header, or whether
    // per-request .bearer_auth injection is needed. Below assumes the former.
    let inner = crate::http::build_client(client.root_cert())?;
    // NOTE: attach default Authorization header — API TBD at Task 0.
    let baseurl = format!("{}/hsm/v2", client.base_url());
    Ok(generated::Client::new_with_client(&baseurl, inner))
}

pub(crate) async fn map_err<E: Debug>(err: progenitor_client::Error<E>) -> Error {
    use progenitor_client::Error::*;
    match err {
        InvalidRequest(s) => Error::Message(format!("HSM invalid request: {s}")),
        CommunicationError(e) | InvalidUpgrade(e) => Error::NetError(e),
        ErrorResponse(rv) => {
            let status = rv.status().as_u16();
            Error::Message(format!("HSM error response: status={status} body={:?}", rv.into_inner()))
        }
        ResponseBodyError(e) => Error::NetError(e),
        InvalidResponsePayload(_, e) => Error::SerdeError(e),
        UnexpectedResponse(resp) => {
            let status = resp.status().as_u16();
            let bytes = resp.bytes().await.unwrap_or_default();
            match serde_json::from_slice::<serde_json::Value>(&bytes) {
                Ok(body) => Error::OchamiError { status, body },
                Err(_) => Error::Message(format!("HSM unexpected: status={status} body={}",
                                                   String::from_utf8_lossy(&bytes))),
            }
        }
        PreHookError(s) => Error::Message(format!("HSM pre-hook error: {s}")),
    }
}

pub(crate) async fn run<F, Fut, T, E>(client: &Ochami, token: &str, op: F) -> Result<T, Error>
where
    F: FnOnce(generated::Client) -> Fut,
    Fut: Future<Output = Result<progenitor_client::ResponseValue<T>, progenitor_client::Error<E>>>,
    E: Debug,
{
    let gc = gen_client(client, token)?;
    match op(gc).await {
        Ok(rv) => Ok(rv.into_inner()),
        Err(e) => Err(map_err(e).await),
    }
}
```

### Raw-reqwest carve-out

**~13-17 methods stay on raw reqwest** for two reasons (exact count settles at Task 0):

1. **Multi-value query semantics** the spec's single-value typing can't
   preserve:
   - `hsm_component_get` — comma-split `nid` used by
     `backend_connector::nid_to_xname`.
   - `hsm_state_components_get` — same signature as component_get.
   - `hsm_state_components_get_query` — same params.
   - `hsm_group_get` — multi-value `label_vec` iteration.

2. **Return types touching hand-rolled `hsm/inventory/types.rs`**
   (dispatcher_conv cascade too heavy):
   - All `hsm_inventory_hardware_*` methods.
   - All `hsm_inventory_hardware_by_fru_*` methods.
   - Possibly some `hsm_inventory_redfish_endpoint_*` or
     `hsm_inventory_ethernet_interfaces_*` methods (Task 0 confirms).

Raw-reqwest wrapper methods use `crate::http::build_client(root_cert)?
.bearer_auth(token).get/post/put/patch/delete(url).send().await` and pipe
the response through `crate::http::handle_json_response<T>` /
`handle_action_response`. URL construction manual, follows `{base_url}/hsm/v2/…`
by convention.

### Return-type migration

~25 wrapper methods change return type from `Result<Value, Error>` /
`Result<String, Error>` to typed shapes:
- DELETE methods: `Result<HsmActionResponse, Error>`.
- POST-creates: `Result<Vec<ResourceUri100>, Error>`.
- Actions returning 204: `Result<(), Error>`.
- Body returns: the resource type (Component, Group, etc.).

Callers in `backend_connector.rs` (~25 sites) migrate from
`value.get("field")` to `.field` field access on typed structs.

Exception: raw-reqwest wrapper methods against hand-rolled `inventory/types.rs`
KEEP `Result<Value, Error>` return where no typed shape exists on the
hand-rolled side.

### Adapter shapes

**Path-param wrapping** (~20-30 wrapper methods take `xname: &str` or similar):
```rust
// If typify's builder requires XName100:
c.do_something_builder().xname(XName100(xname.to_string())).send().await
// If typify accepts impl AsRef<str>:
c.do_something_builder().xname(xname).send().await
```
Task 0 confirms typify's default.

**Enum coercion** — needed for progenitor-routed methods with enum filter
params. Only `hsm_memberships_get` has 7 enum-typed filter fields; every other
enum-taking method stays raw. Coercion is log-and-drop for Optional fields:
```rust
.type_(r#type.and_then(|s| HmsType100::from_str(s).ok()))
```

Wrapper-body enum coercion is thus concentrated in ONE method.

**Retry-on-5xx** applied per-GET-method via `retry_on_5xx` helper:
```rust
pub async fn hsm_component_get_one(&self, token: &str, xname: &str) -> Result<Component, Error> {
    retry_on_5xx(|| async {
        run(self, token, |c| async move {
            // Task 0 (output-reference § B.2/B.3): progenitor 0.8 emits a
            // direct method (`do_component_get`), not a builder.
            c.do_component_get(xname).await
        }).await
    }).await
}
```
POST/PUT/PATCH/DELETE wrapper methods skip the retry outer wrapper.

### Wrapper method count and count breakdown

**~55-63 wrapper methods** total across 10 wrapper files. Distribution roughly:
- component (unified with state/components extras like get_by_nid, get_query): 12
- group: 9
- partition: 8
- memberships: 2
- node_map (unified with defaults/node_map): 7
- inventory/hardware: 5 (skip dangerous delete_all)
- inventory/hardware_by_fru: 4
- inventory/redfish_endpoint: 8
- inventory/ethernet_interfaces: 9

Sum: ~64 methods before dropping; subtract 1-3 for `hsm_state_components_post`
(dropped) and possibly `hsm_group_post_member` / `hsm_partition_post_members`
(Task-0-verified) → **~55-63 methods**. Exact count settles at Task 0.

**Dropped methods**:
- `hsm_state_components_post` — was buggy (sent single Component to bulk
  endpoint; deserialized empty body into Component).
- Possibly `hsm_group_post_member` / `hsm_partition_post_members` if Task 0
  reveals body-shape mismatch.

### Documentation contract

Each wrapper method has a doc comment with:
- One-line summary.
- `# Errors` section listing which `Error` variants can flow (uniform template
  per method type).

Example:
```rust
/// Fetch a component by xname.
///
/// # Errors
/// Returns [`Error`] on transport, deserialization, or HSM failure:
/// - [`Error::NetError`] — network I/O failure.
/// - [`Error::OchamiError`] — HSM returned non-2xx status.
/// - [`Error::SerdeError`] — response body did not match expected schema.
```

### Test strategy

**Framework**: wiremock 0.6, tokio async. NO end-to-end integration tests
against live OCHAMI (out of scope).

**Organization**: per-resource integration test files under `tests/`:
`tests/hsm_component.rs`, `tests/hsm_group.rs`, etc. (9 files). Shared test
PEM fixture in `tests/common/mod.rs`.

**Test count**: ~66-77 tests total:
- ~58-64 wiremock method tests (one per wrapper method).
- 2 retry-behavior tests.
- 1 non-retry-behavior test (verifies POST/PUT/PATCH/DELETE don't retry).
- 5-10 helper unit tests colocated with helpers (`gen_client`, `map_err`,
  `run`, `retry_on_5xx`, `handle_json_response`).

(See Phase 6 detail for the per-file breakdown.)

**Per-test shape**: assert URL path + query params + request body (for
mutating verbs) + response deserialization round-trip. ~15-25 LOC each.

**Fixtures**: inline JSON in each test (no fixture files). Test PEM copied
verbatim from csm-rs's test pattern.

**Retry tests**: use wiremock's `up_to_n_times(N)` (verify API at Task 0).
Adds ~3s wall-clock to `cargo test` (real time waits during backoff). Accept.

---

## Section 3 — Types & Dispatcher Conversions

### Types.rs classification

| File | Post-migration state | From/Into pairs |
|---|---|---|
| `hsm/component/types.rs` | slim re-export | 4 |
| `hsm/state/components/types.rs` | slim re-export (same generated types as `component/`) | 0 (unified) |
| `hsm/group/types.rs` | slim re-export + inherent `impl Group` methods | 1 |
| `hsm/memberships/types.rs` | slim re-export | 0 |
| `hsm/node_map/types.rs` | slim re-export | 0 |
| `hsm/defaults/node_map/types.rs` | slim re-export (same generated types) | 0 |
| `hsm/partition/types.rs` | slim re-export | 0 |
| `hsm/inventory/types.rs` (2767 LOC) | **STAYS hand-rolled** — zero migration work | 0 (unchanged) |
| `hsm/inventory/ethernet_interfaces/types.rs` | slim re-export (Task 0 confirms) | 3 + 1 CompEthInterfacePatch From |
| `hsm/inventory/redfish_endpoint/types.rs` | slim re-export (Task 0 confirms) | 3 |
| **NEW** `hsm/types.rs` | new file — HsmActionResponse alias | 1 |

**Total From/Into pairs: 12-13 (~24-26 impl blocks)** if ethernet_interfaces
and redfish_endpoint slim-re-export. **7 pairs (14 blocks)** if they stay
hand-rolled. Range 14-26 blocks.

### Routing choice is per-resource

If a resource's `types.rs` becomes slim re-export, ALL wrapper methods for
that resource use generated types (some via progenitor, some via raw-reqwest
deserializing into generated). If `types.rs` stays hand-rolled, ALL wrapper
methods stay raw with hand-rolled types.

Mixing within a resource (some methods progenitor→generated, some
raw→hand-rolled) would mean two `Component` types in the same crate. Not
viable.

### Type mapping (component family — verified via csm-rs precedent)

- `Component` ← `Component100Component`
- `ComponentCreate` ← `Component100ComponentCreate`
- `ComponentPut` ← `Component100Put`
- `ComponentArray` ← `ComponentArrayComponentArray`
- `ComponentArrayPostArray` ← `ComponentArrayPostArray`
- `ComponentArrayPostQuery` ← `ComponentArrayPostQuery`
- `ComponentArrayPostByNidQuery` ← `ComponentArrayPostByNidQuery`
- Enums (closed, FromStr): `HmsType100`, `HmsState100`, `HmsFlag100`,
  `HmsArch100`, `HmsClass100`, `NetType100`.
- Open newtypes-around-String: `HmsRole100`, `HmsSubRole100`.
- XName newtypes: `XName100`, `XNameRw100`, `XNameForQuery100`,
  `XNamePartition100`.
- Range/list: `NidRange100`.

Exact names confirmed at Task 0 (typify's mangling).

### Field-shape changes (breaking; documented in MIGRATION.md)

| Old | New | Consumer impact |
|---|---|---|
| `Component.id: Option<String>` | `Option<XName100>` | `.id.as_ref().map(\|x\| &x.0)` |
| `Component.r#type: Option<String>` | `Option<HmsType100>` (field `type_`) | `matches!(comp.type_, Some(HmsType100::Node))` |
| `Component.state/flag/arch/class/net_type` | closed enums | via `FromStr` and `Display` |
| `Component.role/sub_role` | open newtypes | `.role.as_ref().map(\|r\| &r.0)` |
| `Component.nid: Option<usize>` | `Option<i64>` | signed/unsigned shift |
| `ComponentArray.components: Option<Vec<...>>` | `Vec<...>` `#[serde(default)]` | drop `.unwrap_or_default()` |
| `ComponentArrayPostQuery.falg_only` | `.flagonly` | rename happens automatically via slim re-export to the generated type; consumers using `.falg_only` must update to `.flagonly` (the generated field is already correctly spelled — no typo-fix patch needed on the spec side) |
| `Group.label: String` | `ResourceName` | private inner; access via `Deref` (`&*group.label`), construct via `ResourceName::from_str()` |
| `Group.tags: Option<Vec<String>>` | `Option<Vec<ResourceName>>` | element newtype unwrap |
| `Partition.name: String` | `ResourceName` | same pattern |
| `NodeMap.nid: usize` | `i64` | signed/unsigned |
| `ComponentCreate` fields `pub(super)` | `pub` | now constructable outside module |

### From/Into impl pattern

Two coercion patterns for enum fields:

**Optional enum** (`Option<HmsX100>`) — log-and-drop:
```rust
type_: value.r#type.and_then(|s| {
    HmsType100::from_str(&s)
        .map_err(|_| log::warn!("ochami-rs: unknown HmsType '{s}' from dispatcher, dropping"))
        .ok()
}),
```

**Required enum** (`HmsX100` non-Optional) — log-and-fallback with sentinel:
```rust
state: HmsState100::from_str(&value.state).unwrap_or_else(|_| {
    log::warn!("ochami-rs: unknown HmsState '{}' from dispatcher, using Unknown", value.state);
    HmsState100::Unknown
}),
```

Per-enum sentinels for required fields (verify variants at Task 0):
- `HmsType100`: `INVALID` (spec-verified).
- `HmsState100`: `Unknown` (spec-verified).
- `HmsFlag100`: `Unknown` (spec-verified).
- Others: pick during Task 0.

**Open newtypes** (Role, SubRole): trivial wrap:
```rust
role: value.role.map(HmsRole100),
```

**XName wrapping**:
```rust
id: value.id.map(XName100),
// or for ComponentCreate:
id: XNameRw100(value.id),  // required field, direct wrap
```

### Preserved inherent methods (`Group::new`, `get_members`, `get_members_opt`, `add_xnames`)

Reimplement bodies against the generated field layout:

```rust
impl Group {
    pub fn new(label: &str, member_vec_opt: Option<Vec<&str>>) -> Self {
        // Group100Members is typify's likely nested-schema name; Task 0
        // confirms the actual mangled name.
        Self {
            // ResourceName's inner is PRIVATE (Task 0 § F.1/§ H); construct
            // via FromStr, which also enforces the widened pattern regex.
            label: ResourceName::from_str(label).expect("invalid group label"),
            description: None,
            tags: None,
            members: member_vec_opt.map(|v| Group100Members {
                ids: Some(v.into_iter().map(|s| XNameRw100(s.to_string())).collect()),
            }),
            exclusive_group: None,
        }
    }
    // Also rewritten (bodies use the same wrapping pattern):
    // - get_members(&self) -> Vec<String>
    // - get_members_opt(&self) -> Option<Vec<String>>
    // - add_xnames(&mut self, xnames: &[String]) -> Vec<String>
}
```

Orphan rule allows the impls (generated type is in our crate).

### Cross-crate `falg_only` typo handling

Task 0 (output-reference § I/§ L.7) confirms ochami-rs's own generated
`ComponentArrayPostQuery.flagonly` field has no typo — it's spelled
`flagonly` verbatim (single word, not mangled). The `falg_only` typo
exists only in the separate `manta-backend-dispatcher` crate (verified:
`manta-backend-dispatcher-1.0.0-beta.15/src/types/mod.rs:768`, `pub
falg_only: Option<bool>` with `#[serde(rename = "flagonly")]`). The
From/Into impl body maps the dispatcher's typo'd field onto ochami-rs's
clean one:
```rust
flagonly: value.falg_only,
```
Non-blocking; dispatcher stays as-is.

### `hsm/types.rs` (new file)

```rust
pub use crate::hsm::generated::types::Response100 as HsmActionResponse;

impl From<FrontEndHsmActionResponse> for HsmActionResponse { ... }
impl Into<FrontEndHsmActionResponse> for HsmActionResponse { ... }
```

Shared HSM response type + dispatcher conversions. ~30-50 LOC.

### CompEthInterfacePatch From impl

The PATCH endpoint uses a distinct `CompEthInterface100Patch` subset type.
Backend_connector currently builds a full `ComponentEthernetInterface` for the
PATCH call. Add:
```rust
impl From<ComponentEthernetInterface> for CompEthInterfacePatch {
    fn from(value: ComponentEthernetInterface) -> Self {
        CompEthInterfacePatch {
            description: value.description,
            ip_addresses: value.ip_addresses,
            component_id: value.component_id,
        }
    }
}
```
Backend_connector caller does `.into()`. One-way From (no reverse — Patch is a
subset).

### ResourceName spec patch (Task-0-conditional)

The spec's `ResourceName.pattern = '^[0-9a-f_\-.]{1,}$'` is hex-only and
rejects real HSM labels ("compute", "worker" — letters beyond 'f').

If typify enforces regex on Deserialize (config-dependent), any real Group
deserialization fails → **spec patch is release-blocking**.
If typify enforces only via FromStr → **spec patch is nice-to-have** (direct
newtype construction bypasses validation).

Task 0 verifies. Patch already sketched in `scripts/patch-hsm-spec.jq`.

### Backend_connector.rs impact

- ~40 http_client callsite rewrites (mechanical: drop `base_url`/`root_cert`
  first args; call method on `self`).
- ~10-15 field-accessor site updates (`.id.as_ref().map(|x| &x.0)`,
  `.type_.map(|t| t.to_string())`, etc.).
- ~25 return-type absorption updates (`value["field"]` → `.field` access).
- ~5 enum-comparison sites (`matches!(comp.type_, Some(HmsType100::Node))`).
- 1 site for `_group_label` param drop.
- 1 site for `.patch.into()` conversion.

Total: ~80-90 concrete edits in backend_connector.rs. Non-trivial but
mechanical.

---

## Section 4 — Execution Plan & Rollout

### Effort estimate

| Phase | Focused hours |
|---|---|
| 1: Task 0 (spec conversion, first progenitor run, output reference, baseurl verify, `build.rs`, `generated.rs`, runtime deps) | 6-8 |
| 2: Infrastructure — Error enrichment (~30 sites), PCS emission update, Ochami accessors, `src/lib.rs` additions, tokio `time` feature, `src/http.rs` helpers, `wrapper/mod.rs` helpers, `hsm/types.rs` | 8-12 |
| 3: Wrapper `impl Ochami` blocks (10 files, ~55-63 methods) | 24-40 |
| 4: Consumer migration (backend_connector.rs, hsm/group/utils.rs) | 12-20 |
| 5: Slim types re-exports + delete 11 http_client.rs files (atomic) | 8-12 |
| 6: Wiremock tests (~62-75 tests across ~9 files) | 20-32 |
| 7: Docs (README, MIGRATION, CHANGELOG, CONTRIBUTING, SECURITY, `hsm/mod.rs` docstring, per-wrapper module docstrings, spec provenance header, retrospective stub) | 8-12 |

**Total: 86-136 focused hours** (sum of the phase ranges above; matches
the Summary estimate). For a solo maintainer distributing over
normal workload (~3-5 hrs/day): **6-10 weeks wall-clock.**

### Phase 2 (Infrastructure) detail

**Purpose**: land all shared helpers and Error-shape changes that Phases 3+
depend on. No wrapper methods yet — just the plumbing.

**Content** (post-Phase-1-move; build.rs, generated.rs, runtime deps are
already in Phase 1):
- `src/error.rs`: enrich `Error::OchamiError` to
  `{ status: u16, body: Value }` with `#[non_exhaustive]`. Update Display
  format string.
- **`src/pcs/power_cap/http_client.rs` and
  `src/pcs/power_status/http_client.rs`**: update 5 `Error::OchamiError(payload)`
  construction sites to the new destructured form. PCS wrapper migration
  itself remains out of scope; only the Error emission changes.
- **`src/hsm/**/http_client.rs` (all 11 files)**: update ALL ~25
  `Error::OchamiError(payload)` construction sites to the new destructured
  form. These files are DELETED in Phase 5 but must compile in the interim
  (Phases 2-4) because backend_connector.rs still calls into them.
  Mechanical rename; ~1h.
- `src/http.rs`: add `handle_json_response<T: DeserializeOwned>`,
  `handle_action_response`, `retry_on_5xx`, `HTTP_CONNECT_TIMEOUT = 45min`,
  `HTTP_REQUEST_TIMEOUT = 15min`; ensure `build_client` calls
  `.use_rustls_tls()` (existing behavior preserved).
- `src/hsm/types.rs` (new file): `pub use crate::hsm::generated::types::Response100 as HsmActionResponse;`
  + `From<FrontEndHsmActionResponse>` / `Into<FrontEndHsmActionResponse>`.
- `src/hsm/wrapper/mod.rs` (new file): `gen_client`, `map_err`, `run`
  helpers (per Section 2 sketches).
- `src/hsm/mod.rs`: add `pub mod types;` and `mod wrapper;` (interleaved
  with file creation — see ordering below).
- **`src/backend_connector.rs`**: add `pub(crate) fn base_url(&self) -> &str`
  and `pub(crate) fn root_cert(&self) -> &[u8]` accessors to `Ochami`.
  Required by wrapper's `gen_client`.
- **`src/lib.rs`**: add `#![deny(unsafe_code)]` at crate root (hygiene) and
  `pub use backend_connector::Ochami;` (so consumers can write
  `ochami_rs::Ochami` — backwards-compatible with the longer path).
- **`Cargo.toml`**: extend `tokio.features` with `"time"` — required by
  `tokio::time::sleep` in `retry_on_5xx`. Result:
  `features = ["macros", "rt-multi-thread", "time"]`.

Total Error emission sites updated: **5 PCS + ~25 HSM = ~30 sites**.

*Why not BSS and cloud-init* (both are in the deferred-services list per
Non-Goals): validated 2026-07-13 that BSS's `http_client.rs` uses
`Error::RequestError` / `Error::NetError` / `Error::Message` — NOT
`Error::OchamiError` — so the enum-shape change doesn't cascade there.
cloud-init has no Rust code today (only the spec YAML), so nothing to
update. Neither service's future migration is blocked by Phase 2's
scope choice here.

**Ordering within Phase 2** (each substep leaves crate compileable):
1. Error enrichment in `src/error.rs` + **all ~30 emission sites** updated
   simultaneously (Error shape change is atomic; PCS + HSM http_client.rs
   files all touched together). Without this, crate doesn't compile.
2. `Cargo.toml` tokio `time` feature; Ochami accessors in
   `backend_connector.rs`; `src/lib.rs` additions. All dependency-free —
   land as part of step 1 (same commit).
3. `src/http.rs` helpers (foundation for Phase 3 wrappers, both progenitor
   and raw-reqwest paths). Requires tokio `time` feature (step 2).
4. Create `src/hsm/types.rs` AND add `pub mod types;` to `src/hsm/mod.rs`
   (atomic: file must exist alongside the declaration).
5. Create `src/hsm/wrapper/mod.rs` (uses http.rs helpers + Error + types +
   Ochami accessors) AND add `mod wrapper;` to `src/hsm/mod.rs` (atomic:
   same reason).

**Per-step time budget** (sums to ~8-12h):
- Error enrichment + all ~30 emission-site updates (PCS + HSM): 1.5-2h.
- Ochami accessors + `src/lib.rs` additions: 15 min.
- `src/http.rs` helpers (handlers, retry, timeouts): 3-4h.
- `src/hsm/types.rs` + mod.rs declaration: 1-2h.
- `src/hsm/wrapper/mod.rs` + mod.rs declaration: 2-3h.
- Compile check + smoke test: 0.5h.

**Deliverables** (one commit for Phase 2):
- All above files present and compileable.
- Existing tests still pass.
- Optional helper unit tests: ~3-5 inline `#[cfg(test)] mod tests { ... }`
  tests for `handle_json_response`, `map_err`, `retry_on_5xx`. If skipped in
  Phase 2, they land in Phase 6 as part of the "5-10 helper unit tests
  colocated with helpers" count. Recommend including them in Phase 2 for
  early validation.

**Success criteria**:
- `cargo build` succeeds (no wrapper methods exist yet; existing
  http_client.rs still primary path for backend_connector).
- `cargo test` still passes (no test regression).
- `cargo doc --no-deps` succeeds (new modules `hsm/types.rs` and
  `hsm/wrapper/mod.rs` add docstrings; verify no rustdoc failures).
- Optionally: `cargo test wrapper::` runs the ~3-5 new helper unit tests
  green (if included in this phase).

**Commit message convention**:
`feat(hsm): infrastructure — wrapper helpers, Error enrichment, http.rs
additions`. Body references
`docs/superpowers/specs/2026-07-13-progenitor-hsm-client-design.md § Section 4`.

### Phase 3 (Wrapper impl blocks) detail

**Purpose**: create 9 wrapper files with `impl Ochami { pub async fn hsm_*() }`
method blocks. Wrappers exist and compile, but nothing calls them yet
(backend_connector still uses old `hsm::*::http_client::*` free functions
until Phase 4). Phase 3 shifts NO on-wire behavior.

**Type-reference strategy** (important decision):

For wrappers whose resource's `types.rs` is slated for slim re-export
(Section 3 classification — component, group, memberships, node_map,
partition, and Task-0-confirmed inventory/redfish_endpoint +
inventory/ethernet_interfaces):

Wrapper methods reference generated types via
`crate::hsm::generated::types::Component100Component` directly, NOT via the
hand-rolled `crate::hsm::component::types::Component` (which is still
hand-rolled at this point; slim re-export lands in Phase 5). This
avoids one-shot generated→hand-rolled conversion code that would be
thrown away at Phase 5. Wrappers keep working unchanged when Phase 5
slim-re-exports the types (both paths resolve to the same underlying
type post-Phase-5).

Concrete top-of-file imports:
```rust
use crate::hsm::generated::types::{
    Component100Component, Component100ComponentCreate, ComponentArrayComponentArray,
    // ... etc
};
```

**Exception** — wrappers for resources whose `types.rs` stays hand-rolled
(`inventory_hardware.rs`, `inventory_hardware_by_fru.rs`; and possibly
redfish_endpoint / ethernet_interfaces if Task 0 says so): import from
`crate::hsm::inventory::types::*` (the 2767-LOC hand-rolled file that
does NOT get slim re-exported). These wrappers return hand-rolled types
like `Vec<HWInventoryByLocation>` unchanged from the current
`http_client.rs` return types.

**Content**: 9 wrapper files, one per endpoint family (per Section 1
module layout):
- `src/hsm/wrapper/component.rs` — serves both `hsm::component` and
  `hsm::state::components` paths. ~12 methods (get, get_one, get_all,
  get_all_nodes, get_by_nid, get_query, post, post_query,
  post_bynid_query, put, delete_one, delete). Includes the raw-reqwest
  carve-outs (hsm_component_get, hsm_state_components_get_query).
- `src/hsm/wrapper/group.rs` — ~9 methods. Includes raw-reqwest carve-out
  for `hsm_group_get` (multi-value label_vec).
- `src/hsm/wrapper/memberships.rs` — 2 methods. Includes `hsm_memberships_get`
  which currently has 7 in-body enum coercion fields (Task 0 may find
  additional progenitor-routed methods elsewhere with the same pattern).
- `src/hsm/wrapper/node_map.rs` — serves both `hsm::node_map` and
  `hsm::defaults::node_map` paths. ~7 methods (all progenitor).
- `src/hsm/wrapper/partition.rs` — ~8 methods (all progenitor).
- `src/hsm/wrapper/inventory_hardware.rs` — ~5 methods (ALL raw reqwest;
  types stay hand-rolled per Section 3).
- `src/hsm/wrapper/inventory_hardware_by_fru.rs` — ~4 methods (ALL raw
  reqwest).
- `src/hsm/wrapper/inventory_redfish_endpoint.rs` — ~8 methods (Task 0
  decides slim-re-export or hand-rolled).
- `src/hsm/wrapper/inventory_ethernet_interfaces.rs` — ~9 methods.
  Includes the PATCH endpoint that uses `CompEthInterface100Patch` body.
- `src/hsm/wrapper/mod.rs` — EDIT the file Phase 2 created (which
  currently holds only the shared helpers `gen_client`, `map_err`,
  `run`) to add `mod component; mod group; ...` for all 9 new wrapper
  files (all `mod`, not `pub mod` — private submodules).

Every method includes:
- `#[allow(clippy::too_many_arguments)]` where >7 params (~5 methods).
- Doc comment with one-line summary + `# Errors` section (template from
  Section 2).
- Retry wrapping (`retry_on_5xx(...)`) for GET methods only.
- No retry for POST/PUT/PATCH/DELETE.

**Per-file order** (component first to establish patterns; simpler ones
next; complex/Task-0-dependent last):
1. `component.rs` (biggest, unifies duplicates; catches most patterns).
2. `group.rs` (mix of progenitor + raw; inherent Group methods live in
   `hsm/group/types.rs` — added in Phase 5, so wrapper doesn't need
   them yet).
3. `memberships.rs` (small; exercises in-body enum coercion pattern).
4. `partition.rs` (all progenitor; simple).
5. `node_map.rs` (unified path).
6. `inventory_hardware.rs` (all raw).
7. `inventory_hardware_by_fru.rs` (all raw).
8. `inventory_redfish_endpoint.rs`.
9. `inventory_ethernet_interfaces.rs` (includes PATCH _Patch body pattern).

**Per-file time budget** (sums to ~21-33h; effort table's 24-40h has
margin for iteration):
- `component.rs`: 4-6h.
- `group.rs`: 3-5h.
- `memberships.rs`: 1-2h.
- `partition.rs`: 2-3h.
- `node_map.rs`: 2-3h.
- `inventory_hardware.rs`: 2-3h.
- `inventory_hardware_by_fru.rs`: 1-2h.
- `inventory_redfish_endpoint.rs`: 3-4h.
- `inventory_ethernet_interfaces.rs`: 3-5h.

**Deliverables** (one commit for Phase 3):
- 9 wrapper files under `src/hsm/wrapper/`.
- `src/hsm/wrapper/mod.rs` extended with `mod <resource>;` declarations.
- Every wrapper method compiles and has a `# Errors` docstring.
- No changes to `backend_connector.rs` (Phase 4 does that).
- No changes to `types.rs` files (Phase 5 does that).
- No changes to `http_client.rs` files (Phase 5 deletes them).

**Success criteria**:
- `cargo build` succeeds.
- `cargo doc --no-deps` succeeds (`# Errors` sections rendered).
- `cargo clippy --no-deps` clean or with only expected
  `too_many_arguments` allows.
- Wrapper methods NOT YET called by any in-crate code — they exist as
  dead-code `impl Ochami { pub async fn hsm_*() }` blocks (verified by
  `cargo build` succeeding with `#[allow(dead_code)]` NOT needed on the
  methods; `pub` methods aren't flagged as dead-code by default).

**Verifies during Phase 3**:
- Every generated method name from Task 0's output-reference doc
  resolves and is callable via the builder pattern. If a method needed
  here was omitted from Task 0's doc, add its entry to the doc as part
  of Phase 3's commit.
- Every generated body type is constructable (typify's newtype-inner
  visibility works as Task 0 verified).
- The identified in-body enum coercion sites (currently just
  `hsm_memberships_get` per Section 2; Task 0 may reveal additional
  progenitor-routed methods taking enum filter params) type-check with
  the log-and-drop pattern.
- Path-param typing works (either accepts `&str` via `impl AsRef<str>`
  or requires newtype wrapping like `XName100(xname.to_string())`,
  per Task 0's finding).
- The `CompEthInterface100Patch` body construction works.

**Commit message convention**:
`feat(hsm): wrapper impl Ochami methods across all resource families`.
Body references
`docs/superpowers/specs/2026-07-13-progenitor-hsm-client-design.md § Section 4`.

### Phase 4 (Consumer migration) detail

**Purpose**: rewrite in-crate consumers to call the new `impl Ochami`
wrapper methods (Phase 3 output) instead of the old
`hsm::*::http_client::*` free functions. After Phase 4, the old
`http_client.rs` files exist but are UNUSED in-crate. They still ship
(Phase 5 deletes them). Phase 4 also shifts NO on-wire behavior — only
call-site shape changes.

**Content**:

*In `src/backend_connector.rs` (~80-90 concrete edits, per Section 3):*
- ~40 http_client callsite rewrites: uniform destination shape
  `self.hsm_x_foo(token, ...)`. **Old free-function signatures vary in
  arg order across the crate** — some put `base_url` first
  (`hsm::component::http_client::get(base_url, root_cert, auth_token, ...)`),
  others put `auth_token` first
  (`hsm::state::components::http_client::get(auth_token, base_url, root_cert, ...)`,
  `hsm::group::http_client::post_member(auth_token, ...)`). Read the old
  signature per-callsite; don't sed-transform blindly. Migration drops
  `base_url`/`root_cert` args and moves `token` to the second position
  (after `&self`).
- ~10-15 field-accessor site updates: `comp.id.as_ref().map(|x| x.0.as_str())`, `.type_.as_ref().map(|t| t.to_string())`, `.state.as_ref().map(|s| s.to_string())`, etc. Newtype and enum unwrap patterns.
- ~25 return-type absorption updates: `value["field"].as_str()` → `.field` on typed structs (HsmActionResponse (alias for Response100), Vec<ResourceUri100>, etc.). `.unwrap_or_default()` on `Option<Vec<T>>` returns drops (typed returns are `Vec<T>` with `#[serde(default)]`).
- ~5 enum-comparison sites: `comp.r#type.as_deref() == Some("Node")` → `matches!(comp.type_, Some(HmsType100::Node))`. Requires enum imports.
- 1 site for `_group_label` param drop on `hsm_inventory_ethernet_interface_delete_ip_address` call.
- 1 site for `.patch.into()` conversion — `ComponentEthernetInterface` → `CompEthInterface100Patch` at the PATCH callsite. Uses the `From` impl Phase 5 adds (Section 3). **Sequencing note**: this specific site fails to compile until Phase 5's From impl lands. Options: (a) do the manual conversion inline in Phase 4 (~5 LOC), replaced by `.into()` after Phase 5; (b) defer this one callsite to Phase 5. Recommend (a) — keeps Phase 4 fully compileable.

*In `src/hsm/group/utils.rs` (~2 sites):*
- Change utility function signatures to take `client: &Ochami` first arg.
- Rewrite `hsm::group::http_client::*` calls inside to `client.hsm_group_*` methods.
- Update the ~2 utility functions' downstream callers in `backend_connector.rs` (add `self` first arg).

*NOT in Phase 4*:
- `hsm/group/tests.rs` update — deferred to Phase 5 (Group's shape only changes when types.rs slim-re-exports).
- `types.rs` slim re-exports — Phase 5.
- `http_client.rs` deletion — Phase 5.

**Per-resource ordering** (mechanically: match the resource layout used
in Phase 3 wrappers, so each wrapper file's methods get exercised
in order):
1. Component-family callsites (~15-20 sites): backend_connector's
   `ComponentTrait` and `nid_to_xname` uses.
2. Group-family callsites (~15 sites in backend_connector + 2 in
   utils.rs): `GroupTrait`, `group::utils` migration.
3. Inventory-hardware callsites (~5 sites): `HardwareInventory` trait.
4. Inventory-redfish_endpoint callsites (~7 sites):
   `RedfishEndpointTrait`.
5. Inventory-ethernet_interfaces callsites (~9 sites, includes the PATCH
   reshape): `ComponentEthernetInterfaceTrait`.
6. Miscellaneous: any other trait impl or utility call.

Each resource-group is a self-contained pass: touch backend_connector,
compile, move on.

**Per-resource time budget** (sums to ~10-16h; effort table's 12-20h
includes margin for tricky sites):
- Component callsites: 3-4h.
- Group callsites + utils.rs migration: 2-3h.
- Inventory-hardware callsites: 1-2h.
- Inventory-redfish_endpoint callsites: 2-3h.
- Inventory-ethernet_interfaces callsites (including PATCH reshape): 2-3h.
- Misc + final compile-fix sweep: 0.5-1h.

**Deliverables** (one commit for Phase 4):
- `src/backend_connector.rs` — all ~80-90 sites rewritten. Dispatcher
  trait return types (`Result<T, dispatcher::Error>`) mapped from wrapper
  return types (`Result<T, ochami::Error>`) via existing
  `From`/`.map_err` patterns — no new mapping mechanism needed in
  Phase 4.
- `src/hsm/group/utils.rs` — signatures updated to take `&Ochami`; call
  sites in `backend_connector.rs` updated.
- No changes to `types.rs` files (Phase 5 slim-re-exports them).
- No new changes to `http_client.rs` files:
  - Phase 2 already updated Error emission (~30 sites) in these files.
  - Phase 4 makes no additional edits (they compile from Phase 2's state).
  - Phase 5 deletes them.
- No changes to wrapper files (Phase 3 output stays as-is).

**Success criteria**:
- `cargo build` succeeds.
- `cargo test` still passes (no test regression). Includes existing
  `hsm/group/tests.rs` (still uses old hand-rolled Group; Phase 5
  updates it).
- `cargo doc --no-deps` succeeds.
- `cargo clippy --no-deps` clean.
- Grep verifies: `grep -rn "hsm::.*::http_client::" src/backend_connector.rs`
  and `grep -rn "hsm::group::http_client::" src/hsm/group/utils.rs` both
  return zero hits — all in-crate callsites migrated.
- Every wrapper method whose old free-function counterpart HAD an
  in-crate caller (in backend_connector.rs or utils.rs) now has a
  corresponding new caller. Wrappers whose old counterpart was only used
  externally (e.g., `hsm_group_get_labels`, `hsm_partition_get_names`)
  correctly have no new in-crate caller — this is expected, not a gap.

**Verifies during Phase 4**:
- All ~80-90 backend_connector sites compile with the new signatures.
- Newtype unwrap patterns (`.id.as_ref().map(|x| &x.0)`) work as Task 0
  confirmed newtype visibility permits.
- Enum matches (`matches!(comp.type_, Some(HmsType100::Node))`) compile
  with the imported enum names from Task 0.
- Utility function `client: &Ochami` first-arg pattern doesn't break
  any downstream ergonomic constraint (callers now
  `hsm::group::utils::foo(&ochami, ...)`).

**Commit message convention**:
`refactor(hsm): migrate consumers to Ochami wrapper methods`. Body
references
`docs/superpowers/specs/2026-07-13-progenitor-hsm-client-design.md § Section 4`.

### Phase 5 (Types migration + http_client deletion) detail

**Purpose**: land the types migration (slim re-exports + From/Into
rewrites + Group inherent method rewrites + CompEthInterfacePatch From
impl) ATOMICALLY with the deletion of the 11 `http_client.rs` files
(they'd break otherwise). Also swap Phase 4's inline PATCH conversion
for `.into()`, and update `hsm/group/tests.rs` for the new Group shape.

**Atomicity nuance**: Phase 5's single commit has an intermediate
uncompileable state within it — as soon as `types.rs` slim-re-exports
change (say `id: Option<XName100>` newtype), old `http_client.rs`
construction code (`Component { id: Some(xname.to_string()) }`) breaks.
Only after `http_client.rs` deletion does the crate compile again.
**Don't attempt to `git bisect` within Phase 5's commit.**

**Content**:

*Types migration:*
- 7 `types.rs` files slim-re-exported (7-9 total depending on Task 0's
  ethernet_interfaces / redfish_endpoint decision — those two are the
  Task-0-conditional additions):
  - `hsm/component/types.rs` — slim re-export from generated; ~4
    From/Into pairs updated.
  - `hsm/state/components/types.rs` — slim re-export (same generated
    types as component; unified — no From/Into work).
  - `hsm/group/types.rs` — slim re-export + 1 From/Into pair + rewritten
    inherent methods (`Group::new`, `get_members`, `get_members_opt`,
    `add_xnames`) for newtype field shapes.
  - `hsm/memberships/types.rs` — slim re-export (0 From/Into work).
  - `hsm/node_map/types.rs` — slim re-export (0 From/Into).
  - `hsm/defaults/node_map/types.rs` — slim re-export (same types as
    node_map; unified).
  - `hsm/partition/types.rs` — slim re-export (0 From/Into).
  - `hsm/inventory/ethernet_interfaces/types.rs` — Task-0-dependent: IF
    slim, +3 From/Into pairs + 1 CompEthInterfacePatch From impl. If
    hand-rolled, unchanged.
  - `hsm/inventory/redfish_endpoint/types.rs` — Task-0-dependent: IF
    slim, +3 From/Into pairs. If hand-rolled, unchanged.
- `hsm/inventory/types.rs` (2767 LOC) — **UNCHANGED**. Stays hand-rolled
  per Section 3. Zero migration work in this file.

*Group inherent method rewrites (details Task-0-informed)*:
- `Group::new(label, member_vec_opt) -> Self` — body constructs `label`
  via `ResourceName::from_str(label)` (inner is **private**; `.0`/tuple
  construction doesn't compile — Task 0 § F.1/§ H) and members via
  `Group100Members { ids: Some(vec of XNameRw100(...)) }`. Exact
  `Group100Members` name confirmed at Task 0.
- `get_members()`, `get_members_opt()`, `add_xnames()` bodies rewritten
  to unwrap/wrap newtypes.

*http_client.rs deletion*:
- Delete 11 files: `hsm/{component, state/components, group, memberships,
  node_map, defaults/node_map, partition, inventory/hardware,
  inventory/hardware_by_fru, inventory/redfish_endpoint,
  inventory/ethernet_interfaces}/http_client.rs`.
- Also remove `pub mod http_client;` lines from each corresponding
  `mod.rs`.

*Backend_connector.rs cleanup* (Task-0-conditional — only if
`inventory/ethernet_interfaces/types.rs` slim-re-exports):
- Replace Phase 4's ~5 LOC inline `ComponentEthernetInterface` →
  `CompEthInterface100Patch` conversion at the PATCH callsite with
  `.into()` (now that Phase 5 added the From impl).
- If Task 0 said hand-rolled: NO cleanup needed. Wrapper stays raw with
  hand-rolled `ComponentEthernetInterface` type; backend_connector's
  PATCH callsite is unchanged from Phase 4's state.

*Tests update*:
- `hsm/group/tests.rs` — update `Group::new("label", Some(vec!["xname1",
  "xname2"]))` and `.add_xnames(...)` and `.get_members()` assertions to
  work with the new field shapes (newtype `Deref` access like `&*group.label`
  — NOT `.label.0`, since `ResourceName`'s inner is private; Task 0 § H).

**Ordering within Phase 5's single atomic commit** (informational — all
land together, but for the author's implementation order):
1. **IF Task 0 said `inventory/ethernet_interfaces/types.rs` slim-re-exports**:
   add the `From<ComponentEthernetInterface> for CompEthInterface100Patch`
   impl in that file. This unblocks step 6 below. **If Task 0 said
   hand-rolled** (wrappers stay raw with hand-rolled types): skip this
   step entirely; no CompEthInterfacePatch type exists in the crate.
2. Slim-re-export the 7 non-Task-0-dependent `types.rs` files
   (component, state/components, group, memberships, node_map,
   defaults/node_map, partition).
3. Rewrite the ~5 From/Into impl pairs in `component/types.rs` and
   `group/types.rs`. At this point Group's inherent methods (Group::new
   etc.) also rewrite.
4. Task-0-dependent: slim-re-export inventory/ethernet_interfaces and/or
   inventory/redfish_endpoint types.rs if Task 0 said so; rewrite
   From/Into. **SKIP entirely if Task 0 said both stay hand-rolled** —
   those types.rs files stay unchanged.
5. Delete the 11 `http_client.rs` files + `pub mod http_client;` lines.
   Crate compiles because Phase 4 already migrated backend_connector to
   wrapper methods; the http_client.rs files were the only remaining
   callers of the OLD-shape types, and they're now deleted.
6. Backend_connector PATCH callsite (Task-0-conditional):
   - **IF Task 0 said inventory/ethernet_interfaces slim-re-exports**
     (step 1 added the From impl):
     - If Phase 4 chose option (a) — inline conversion: REPLACE the
       inline conversion code with `.into()`.
     - If Phase 4 chose option (b) — deferred: ADD the `.into()` call
       at the (previously untouched) callsite.
   - **IF Task 0 said hand-rolled** (wrapper stays raw with hand-rolled
     `ComponentEthernetInterface`): SKIP this step. Backend_connector's
     PATCH callsite already passes the hand-rolled type unchanged.
7. Update `hsm/group/tests.rs` for new Group shape.
8. `cargo build` + `cargo test` — final compile check.

**Per-step time budget** (sums to ~8-12h; matches effort table):
- CompEthInterfacePatch From impl: 0.5h (0h if Task 0 said
  ethernet_interfaces hand-rolled).
- 7 non-Task-0-dependent `types.rs` slim re-exports: 1.5-2h.
- ~5 From/Into impl pairs + Group inherent methods: 2-3h.
- Task-0-dependent inventory slim re-exports + From/Into: 1.5-2.5h
  (0.5h if Task 0 said stay hand-rolled).
- 11 file deletions + mod.rs cleanup: 0.5h.
- PATCH callsite `.into()` swap: 15 min (0m if Task 0 said
  ethernet_interfaces hand-rolled — no swap needed).
- hsm/group/tests.rs update: 0.5-1h.
- Final compile + test check: 0.5-1h.

**Deliverables** (one atomic commit for Phase 5):
- All 7-9 `types.rs` files slim-re-exported (7 always + up to 2
  Task-0-conditional).
- ~5-12 From/Into pair rewrites in Phase 5 (5 baseline: 4 in
  component/types.rs + 1 in group/types.rs; 0-7 Task-0-conditional:
  3-3.5 for ethernet_interfaces if slim + 3 for redfish_endpoint if
  slim). Note: `hsm/types.rs`'s HsmActionResponse From/Into pair is
  Phase 2 work, not Phase 5.
- Group inherent methods rewritten.
- CompEthInterfacePatch From impl added (Task-0-conditional — only if
  ethernet_interfaces slim-re-exports; skipped if hand-rolled).
- `hsm/inventory/types.rs` UNCHANGED (60+ existing impl blocks carry
  over verbatim).
- 11 `http_client.rs` files deleted.
- `pub mod http_client;` lines removed from 11 `mod.rs` files.
- Backend_connector's PATCH callsite uses `.into()` (Task-0-conditional
  — only if ethernet_interfaces slim-re-exports; otherwise unchanged).
- `hsm/group/tests.rs` updated for new Group shape.

**Success criteria**:
- `cargo build` succeeds AT THE FINAL COMMIT STATE (intermediate states
  during authoring will not compile — expected).
- `cargo test` passes end-to-end, INCLUDING the updated
  `hsm/group/tests.rs`.
- `cargo doc --no-deps` succeeds.
- `cargo clippy --no-deps` clean.
- Grep verifies: `find src/hsm -name http_client.rs` returns nothing —
  all 11 files gone.
- Grep verifies: `grep -rn "pub mod http_client" src/hsm/` returns
  nothing — all mod declarations removed.

**Verifies during Phase 5**:
- Every type mapping in Section 3's table resolves to a compiling
  `pub use` line.
- Log-and-drop enum coercion (Optional enum fields) and log-and-fallback
  (Required enum fields with picked sentinels) work in From/Into bodies.
- Newtype wrapping (`XName100(...)`, `XNameRw100(...)`, `ResourceName(...)`)
  works with typify's inner-field visibility from Task 0.
- Group inherent methods (`::new`, `.add_xnames`, `.get_members`,
  `.get_members_opt`) preserve their public signatures unchanged.
- Backend_connector's `.into()` at the PATCH site compiles via
  the CompEthInterfacePatch From impl (Task-0-conditional — verified
  only if ethernet_interfaces slim-re-exports; N/A otherwise).

**Commit message convention**:
`refactor(hsm): slim types re-exports + delete http_client (atomic)`.
Body references
`docs/superpowers/specs/2026-07-13-progenitor-hsm-client-design.md § Section 4`
and notes the atomic-commit reason (types slim + http_client delete
can't split).

### Phase 6 (Wiremock tests) detail

**Purpose**: add integration test coverage for all wrapper methods
using wiremock mock HTTP servers. Verifies wire-shape (URL, query
params, request body) and response deserialization end-to-end. Includes
retry-behavior tests and (if not landed in Phase 2) helper unit tests.

**Content**:

*Test file layout under `tests/`:*
- `tests/common/mod.rs` — shared test fixtures. Primary contents:
  ```rust
  pub const TEST_PEM: &str = "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----\n";
  ```
  Test PEM copied verbatim from csm-rs's test pattern (see csm-rs
  `src/hsm/wrapper/mod.rs::tests` module for the exact cert bytes).
  ~15-20 LOC.
- 9 per-resource integration test files, matching Phase 3's wrapper file layout:
  - `tests/hsm_component.rs` — ~10-12 tests (one per hsm_component_* method).
  - `tests/hsm_group.rs` — ~8-9 tests.
  - `tests/hsm_memberships.rs` — ~2 tests.
  - `tests/hsm_node_map.rs` — ~5-7 tests.
  - `tests/hsm_partition.rs` — ~8 tests.
  - `tests/hsm_inventory_hardware.rs` — ~5 tests.
  - `tests/hsm_inventory_hardware_by_fru.rs` — ~4 tests.
  - `tests/hsm_inventory_redfish_endpoint.rs` — ~7-8 tests.
  - `tests/hsm_inventory_ethernet_interfaces.rs` — ~9 tests.

Each test file starts with `mod common;` to pull in TEST_PEM.

*Test structure template* (per csm-rs pattern):
```rust
mod common;
use common::TEST_PEM;
use wiremock::matchers::{method, path, header};
use wiremock::{Mock, MockServer, ResponseTemplate};
use serde_json::json;
use ochami_rs::Ochami;
use ochami_rs::hsm::component::types::{Component, HmsType100};

#[tokio::test]
async fn hsm_component_get_one_returns_component() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/hsm/v2/State/Components/x1c0s1b0n0"))
        .and(header("authorization", "Bearer test-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "ID": "x1c0s1b0n0", "Type": "Node", "State": "Ready",
        })))
        .mount(&server).await;
    let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
    let comp = ochami.hsm_component_get_one("test-token", "x1c0s1b0n0")
        .await.expect("should succeed");
    assert_eq!(comp.id.as_ref().unwrap().0, "x1c0s1b0n0");
    assert!(matches!(comp.type_, Some(HmsType100::Node)));
}
```
~15-25 LOC per test (excluding imports, which land once at file top).
Inline JSON — no fixture files.

*Test naming convention*: mirror csm-rs's descriptive pattern —
`<method>_<condition>_<expected>` (e.g.,
`hsm_component_post_nodes_posts_to_state_components`,
`hsm_group_get_sends_multiple_group_query_params`). Reads as a
one-line spec of the assertion.

*Concurrent execution*: `cargo test` runs in parallel by default. Each
test spawns its own `MockServer` (random port); no shared state → safe.
No `--test-threads=1` needed.

*Retry-behavior tests (3)*:
- `hsm_component_get_one_retries_on_5xx` — GET, 2×503 then 200; assert
  wall-clock elapsed >= 1.4s (retry backoff 500ms + 1s).
- `hsm_component_get_one_gives_up_after_3_5xx` — GET, 3×503; assert
  final Err with `OchamiError { status: 503, .. }`.
- `hsm_group_post_does_not_retry_on_5xx` — POST, 1×503; assert Err
  after ONE attempt, wall-clock < 500ms (no retry backoff). Prevents
  a future refactor from silently adding retry to non-idempotent verbs.

The first two use wiremock's `up_to_n_times(N)` API to sequence
responses across attempts. **Task-0-conditional**: if `up_to_n_times`
doesn't exist in wiremock 0.6, fall back to priority-ordered mocks
(higher-priority mock overrides after N attempts — awkward but
functional).

*Helper unit tests (5-10)*:
Colocated with helpers as `#[cfg(test)] mod tests { ... }` blocks:
- In `src/hsm/wrapper/mod.rs`: `gen_client_with_invalid_token_returns_error`,
  `map_err_unexpected_response_maps_to_ochami_error`,
  `run_unwraps_response_value`.
- In `src/http.rs`: `retry_on_5xx_retries_then_succeeds`,
  `retry_on_5xx_gives_up_after_max_attempts`,
  `handle_json_response_maps_unauthorized_to_request_error`.

**IF Phase 2 already added these**, Phase 6 skips them (Phase 2 recommended
including them for early validation). **IF Phase 2 deferred**, Phase 6
adds them here.

**Per-file time budget** (sums to ~18-28h; effort table's 20-32h has
margin for tricky tests):
- `tests/common/mod.rs` (shared PEM): 15 min.
- `tests/hsm_component.rs`: 3-4h.
- `tests/hsm_group.rs`: 2-3h.
- `tests/hsm_memberships.rs`: 0.5-1h.
- `tests/hsm_node_map.rs`: 1.5-2h.
- `tests/hsm_partition.rs`: 2-3h.
- `tests/hsm_inventory_hardware.rs`: 1.5-2h.
- `tests/hsm_inventory_hardware_by_fru.rs`: 1-1.5h.
- `tests/hsm_inventory_redfish_endpoint.rs`: 2-3h.
- `tests/hsm_inventory_ethernet_interfaces.rs`: 2-3h.

(The 3 retry-related tests are folded into the per-file estimates above:
2 retry-succeeds tests inside `tests/hsm_component.rs` and 1
non-retry-behavior test inside `tests/hsm_group.rs`. They add ~30 min
each to those files' totals.)

- 5-10 helper unit tests (if not in Phase 2): 1.5-3h. Skip if Phase 2
  added them.

**Deliverables** (one commit for Phase 6):
- `tests/common/mod.rs` — shared PEM fixture.
- 9 per-resource integration test files (~58-64 method tests total).
- 2 retry-behavior tests + 1 non-retry-behavior test.
- 5-10 helper unit tests (colocated with helpers) — if not in Phase 2.
- `Cargo.toml`: add `wiremock = "0.6"` to `[dev-dependencies]` (not
  needed before Phase 6 since no tests exist yet; landed here).
- No changes to `src/` beyond helper unit tests (if added in this phase).

**Ordering within Phase 6** (each per-file test can be written
independently; wall-clock parallelizable if multiple contributors):
1. Create `tests/common/mod.rs` first (all test files reference it).
2. Add `wiremock = "0.6"` to `Cargo.toml` `[dev-dependencies]` (first
   time this dep lands — Phase 1's Cargo.toml only added build-deps and
   runtime deps).
3. Add 9 per-resource test files in any order (component first for
   pattern-setting is recommended).
4. Add 2 retry-succeeds tests (in `tests/hsm_component.rs`) + 1
   non-retry-behavior test (in `tests/hsm_group.rs` for `hsm_group_post`
   or wherever a POST wrapper lives).
5. If Phase 2 deferred helper unit tests, add them now.
6. `cargo test` — verify all pass.

**Success criteria**:
- `cargo test` passes end-to-end (~66-77 tests green — 58-64 wiremock
  method tests + 2 retry + 1 non-retry-behavior + 5-10 helper unit tests).
- Every wrapper method exercised by ≥1 wiremock test.
- Retry test wall-clock elapsed >= 1.4s confirms real backoff (not
  time-mocked).
- Helper unit tests (5-10 total) green: `cargo test hsm::wrapper::tests`
  covers the ~3 in `src/hsm/wrapper/mod.rs`, and `cargo test http::tests`
  covers the ~3 in `src/http.rs`. (Or simply `cargo test tests::` to
  match both modules' `tests` inner-modules in one command.)
- `cargo clippy --tests --no-deps` clean (test code has its own clippy
  patterns — unused variables in fixtures, redundant clones — worth
  catching here rather than in a follow-up).
- `cargo doc --no-deps` still succeeds.

**Verifies during Phase 6**:
- Every wrapper method's on-wire URL + query params + request body +
  response deserialization round-trip works against a mock server.
- Retry-on-5xx GET methods actually retry (verified via wall-clock).
- Non-GET methods (POST/PUT/PATCH/DELETE) do NOT retry (verified by the
  `hsm_group_post_does_not_retry_on_5xx` test in the Retry-behavior
  Tests section above).
- Helper functions (`map_err`, `handle_json_response`, `retry_on_5xx`)
  match documented behavior.

**Commit message convention**:
`test(hsm): add wiremock test suite covering all wrapper methods`.
Body references
`docs/superpowers/specs/2026-07-13-progenitor-hsm-client-design.md § Section 4`.

### Phase 7 (Docs + release prep) detail

**Purpose**: land all consumer-facing documentation and release-prep
artifacts for beta.16. NO code changes (Phase 3 already added method
`# Errors` docstrings; Phase 7 adds file-level docs + top-level docs).

**Content** (Phase 7's scope explicitly does NOT include method-level
`# Errors` docstrings — those are Phase 3 output):

*Consumer-facing top-level docs:*
- **`README.md`** — rewrite. Currently ~225 bytes. New version explains
  `Ochami` as the client entry point, shows `use ochami_rs::Ochami;`
  example, one code sample calling a wrapper method, points at
  MIGRATION.md and CHANGELOG.md. Target ~50-100 LOC.
- **`MIGRATION.md`** — new file. Structure per the "MIGRATION.md
  outline" subsection earlier in Section 4 (Breaking API changes,
  Runtime behavior changes, Latent bugs fixed [link to this spec's
  Latent Bugs section for the 19-item list], Concrete before/after
  examples, Downstream cold-build cost). Target ~150-250 LOC.
- **`CHANGELOG.md`** — new file, Keep-a-Changelog format
  (`keepachangelog.com`). Single `[1.0.0-beta.16]` entry summarizing the
  migration under standard subsection headers: Breaking Changes, Runtime
  Behavior Changes, Fixed, Added. Fill date at release time. Target
  ~80-120 LOC.
- **`CONTRIBUTING.md`** — new file. Requirements (rust-version, node.js
  for `make convert-spec`, jq for spec patches), local build/test
  instructions, regeneration procedure, firewalled-network note,
  release process. Target ~60-100 LOC.
- **`SECURITY.md`** — new file. Contact email (verified with @masber
  before publishing), supported versions, disclosure process. Target
  ~30-50 LOC.

*Module-level docs (added inside code files, not new files):*
- **`src/hsm/mod.rs`** — top-level docstring (~40 lines). Explains: HSM
  is the source of truth for hardware state; how this module is built
  (spec pipeline: YAML → jq patches → JSON → progenitor → wrapper);
  which submodules exist and their roles. Copy the shape of csm-rs's
  `src/hsm/mod.rs` docstring.
- **Per-wrapper-file module docstrings** — 10 files under `src/hsm/wrapper/`
  (all 9 per-resource wrappers PLUS `wrapper/mod.rs` itself). Each
  ~10-30 lines.
  - Per-resource wrappers (9): state which endpoints the file covers,
    which methods stay on raw reqwest (per Section 2's carve-out) and
    why. csm-rs pattern: each wrapper file's module docstring lists
    per-method routing decisions.
  - `wrapper/mod.rs`: explains the "hybrid client" pattern
    (progenitor-routed + raw-reqwest) upfront and points at the
    per-resource files for method-level routing.

*Spec provenance:*
- **Update the top of `src/hsm/ochami_api_docs.yaml`** with a
  provenance-header comment: source URL (verified at Task 0), retrieval
  date, list of local patches applied via `scripts/patch-hsm-spec.jq`.
  ~5-10 lines of YAML comments.

*Retrospective template stub:*
- **`docs/superpowers/retros/<phase-7-commit-date>-progenitor-hsm-migration.md`** —
  empty stub with section headers (per Section 5's retrospective
  template). Fill 1-2 weeks after beta.16 release. ~30 LOC.

*Cargo.toml final polish:*
- Add `keywords = ["hpc", "ochami", "openchami", "hsm"]` and
  `categories = ["api-bindings", "asynchronous"]` to `[package]`
  (crates.io discoverability). Verify the full Cargo.toml matches this
  design spec's Appendix diff (fields, deps, features all consistent).

**Ordering within Phase 7** (each file is standalone; wall-clock
parallelizable):
1. `src/hsm/mod.rs` top-level docstring — foundation for other doc
   cross-references.
2. Per-wrapper-file module docstrings (10 files, in any order).
3. Update `src/hsm/ochami_api_docs.yaml` provenance header.
4. `README.md` rewrite — visible entry point.
5. `MIGRATION.md` — dense breaking-change reference (per the MIGRATION.md
   outline subsection earlier in Section 4).
6. `CHANGELOG.md` — Keep-a-Changelog entry for beta.16 (date filled at
   release).
7. `CONTRIBUTING.md` — dev workflow.
8. `SECURITY.md` — **BLOCKER: verify contact email with @masber before
   committing Phase 7.** Committing a placeholder would ship in the
   release. Ask @masber which address to publish (personal? work?
   security@… alias?).
9. `docs/superpowers/retros/<phase-7-commit-date>-progenitor-hsm-migration.md`
   stub (fill 1-2 weeks after beta.16 release, per Section 5's retro
   convention).
10. Add Cargo.toml `keywords` and `categories` to `[package]`.
11. `cargo doc --no-deps --open` — final visual check that docs render
    cleanly.

**Per-step time budget** (sums to ~7-11h; effort table's 8-12h has
margin):
- `src/hsm/mod.rs` docstring: 0.5-1h.
- Per-wrapper-file module docstrings (10 files): 2-3h (~15-20 min each).
- `src/hsm/ochami_api_docs.yaml` provenance header: 15 min.
- README rewrite: 1-1.5h.
- MIGRATION.md: 1.5-2.5h.
- CHANGELOG.md: 0.5-1h.
- CONTRIBUTING.md: 0.5-1h.
- SECURITY.md: 15 min (mostly template).
- Retrospective stub: 15 min.
- Cargo.toml polish + verify: 15 min.
- `cargo doc --open` visual check: 15 min.

**Deliverables** (one commit for Phase 7):
- README.md rewritten.
- New: MIGRATION.md, CHANGELOG.md, CONTRIBUTING.md, SECURITY.md.
- src/hsm/mod.rs top-level docstring.
- 10 per-wrapper-file module docstrings.
- src/hsm/ochami_api_docs.yaml provenance header comment.
- docs/superpowers/retros/<phase-7-commit-date>-progenitor-hsm-migration.md stub.
- Cargo.toml `keywords` and `categories` added under `[package]`.

**Success criteria**:
- `cargo doc --no-deps` builds with zero warnings (no broken
  intra-doc-links, no malformed rustdoc-fence, etc.). Re-run with
  `--open` for a manual visual check that module docstrings render
  as expected in the browser.
- MIGRATION.md complete: all breaking changes documented; concrete
  before/after examples table populated; latent-bug-fix list matches
  this design spec's Latent Bugs section (19 fixed items).
- CHANGELOG.md `[1.0.0-beta.16]` entry complete except date (filled at
  release).
- SECURITY.md contains a real contact email (not placeholder).
- All new doc files pass a `markdownlint`-style spot-check (no broken
  markdown; consistent heading levels).

**Verifies during Phase 7**:
- Provenance header points to the upstream spec repo (from Task 0's
  output-reference doc's "Upstream spec repo URL" verify item).
- README code sample compiles as-is against beta.16 (best-effort by
  eye; a full compile-check requires `doc-test` which we're not
  running for the README sample).

**Commit message convention**:
`docs(hsm): README rewrite + MIGRATION + CHANGELOG + module docstrings`.
Body references
`docs/superpowers/specs/2026-07-13-progenitor-hsm-client-design.md § Section 4`.

### PR shape

**One PR, 7 commits, one per phase**. Each commit leaves crate compileable.
Phase 5 (slim types + delete http_client) is atomic (can't split — slim types
break old http_client).

PR description structured by phase so reviewer navigates commit-by-commit.

### Phase ordering rationale

Phase 3 (wrappers) BEFORE Phase 5 (slim types) — wrappers reference
generated types directly via `crate::hsm::generated::types::*` (see
Phase 3 detail's type-reference strategy). Phase 5 adds shorter alias
paths (`hsm::component::types::Component`) that resolve to the same
underlying generated type; wrappers require zero updates at Phase 5.
Old hand-rolled type definitions are dropped and callers migrate to the
generated types in Phases 4 (consumer migration) and 5 (types +
http_client deletion, atomic).

### Long-lived branch management

2-4 weeks focused ≈ 6-10 weeks wall-clock ≈ periodic rebases from main.
Soft-freeze on `backend_connector.rs` / `http_client.rs` files during
migration to reduce conflict risk. Solo project → convention, not enforced.

### Downstream Manta lockstep

Manta's `Cargo.toml` depends on `ochami-rs = "1.0.0-beta.15"`. Post-release,
Manta needs a companion PR:
- Update Cargo dep to beta.16.
- Rewrite call sites: `hsm::*::http_client::*` → `Ochami::hsm_*`.
- Update field accessors for newtype wrappers.
- Update Error match arms if any destructure `OchamiError`.

Estimated Manta effort: **1-2 focused days** for a maintainer familiar with
the code (~3-5 wall-clock days at part-time). Coordinate release timing:
publish ochami-rs beta.16 → immediately open Manta PR. Same person (@masber),
self-coordinated.

### Baseurl verification (Phase 1, FIRST subtask, 30 min)

Confirm `{base_url}/hsm/v2` is correct for real OCHAMI deployments. Check
OpenCHAMI SMD deployment docs OR curl a live instance's
`/hsm/v2/State/Components` endpoint. Bail early if assumption wrong.

### Task 0 escalation

If Task 0 exceeds 8 hours OR requires >3 spec patches, pause and present to
user. Options: continue with more patches, scope down (drop problematic
endpoints from wrapping), abort and revisit approach.

### Pre-release smoke test

Before `cargo publish`:
- Build Manta locally with git-dep pointing at the migration branch.
- Run Manta's test suite.
- If test cluster accessible, issue real HSM operations.

### Release checklist

- [ ] All follow-up items for beta.16 milestone done.
- [ ] `cargo test` passes end-to-end.
- [ ] `cargo doc --no-deps` builds.
- [ ] `cargo publish --dry-run` succeeds; verify `.crate` tarball contains
      `src/hsm/ochami_api_docs.openapi3.json`.
- [ ] `cargo audit` clean.
- [ ] MIGRATION.md, CHANGELOG.md, README.md updated.
- [ ] Baseurl assumption verified against real OCHAMI.
- [ ] `docs/superpowers/plans/<phase-1-commit-date>-progenitor-hsm-output-reference.md` committed.
- [ ] `docs/superpowers/retros/<phase-7-commit-date>-progenitor-hsm-migration.md` stub
      committed.
- [ ] Manta companion PR drafted.
- [ ] Optional: publish `1.0.0-beta.16-rc.1` first, get Manta to test, then
      `1.0.0-beta.16`.

### Rollback plan

- `cargo yank --version 1.0.0-beta.16` marks version as "do not select for
  new lockfiles". Existing lockfiles keep working; new consumers skip.
- Fix-forward with `1.0.0-beta.17`. No git-history rewriting.

### Post-release monitoring

- 24-hour watch for regressions.
- One-week retrospective (fill in the committed template stub).

### MIGRATION.md outline

Sections:
1. **Breaking API changes** — compile-breaking items (method surface, field
   types, return types, Error enum, PUT signatures, ComponentPut, dropped
   `hsm_state_components_post`).
2. **Runtime behavior changes** — silent shifts (timeouts, retry semantics,
   URL bug fixes, filter fixes, silent-deserialize fixes).
3. **Latent bugs fixed** (from the 19-item Fixed list below; plus 1 pre-existing bug known but not fixed by this migration).
4. **Concrete before/after examples** table:

| Before (beta.15) | After (beta.16) |
|---|---|
| `hsm::component::http_client::get(&url, &cert, &token, None, Some("Node"), ...)` | `ochami.hsm_component_get(&token, None, Some("Node"), ...)` |
| `let n: usize = comp.nid.unwrap();` | `let n: i64 = comp.nid.unwrap();` |
| `if comp.r#type.as_deref() == Some("Node")` | `if matches!(comp.type_, Some(HmsType100::Node))` |
| `match err { Error::OchamiError(v) => log(v) }` | `match err { Error::OchamiError { status, body, .. } => log(status, body) }` |
| `let s: String = group::post(...)?;` | `let uris: Vec<ResourceUri100> = ochami.hsm_group_post(...)?;` |

5. **Downstream cold-build cost** — ~15-25s added for consumers.

---

## Section 5 — Follow-Up Work & Long-Term Maintenance

### Follow-up work flagged during design (out of scope for beta.16)

**Infrastructure**:
- Set up GitHub Actions CI mirroring csm-rs's `ci.yml` (build, clippy, test,
  doc, audit, MSRV check). No CI today.
- Drop unused `serde_yaml` runtime dep.
- Drop reqwest `blocking` feature (if grep confirms zero uses).
- Add `SECURITY.md` (contact email verified with @masber).

**API surface expansion**:
- Wrap the ~60-70 spec endpoints in `pub(crate) mod generated` not surfaced by
  wrapper methods (bulk PATCH endpoints, subscriptions, discovery, service
  values). Each is small: wrapper + test.
- Split `backend_connector.rs` into per-resource files (csm-rs pattern) once
  beta.16 lands.

**Feature gating**:
- Add `manta-dispatcher` cargo feature to make dispatcher dep optional.
  Requires moving From/Into behind `#[cfg(feature = "manta-dispatcher")]`.

**Observability**:
- `#[tracing::instrument]` on wrapper methods for span-per-call.

**Performance**:
- Shared reqwest client via progenitor pre_hook (once verified in progenitor
  0.8). csm-rs also deferred this.

**Extend to BSS / PCS / cloud-init** (all 4 services have vendored specs
under `src/<service>/ochami_api_docs.yaml`; validated 2026-07-13):
- **PCS** (`src/pcs/ochami_api_docs.yaml`, 944 LOC, already OpenAPI 3.0.0):
  no swagger→OpenAPI conversion step needed. Skip the `swagger2openapi`
  step of Phase 1; jq patches (if any) still apply. Estimated ~20-30% of
  ochami-rs HSM effort.
- **BSS** (`src/bss/ochami_api_docs.yaml`, 1,032 LOC, Swagger 2.0):
  same 7-phase pattern as HSM. Estimated ~30-40% of HSM effort (smaller
  spec, learned patterns).
- **cloud-init** (`src/cloud-init/ochami_api_docs.yaml`, 701 LOC): SPEC
  ANOMALY — file starts with `definitions:` at column 0, no top-level
  `swagger:` or `openapi:` header. Not a valid Swagger 2 or OpenAPI 3
  spec; both swagger2openapi and progenitor will reject it as-is.
  Additional pre-work required:
  1. Investigate whether upstream cloud-init spec has a header (may be
     a partial vendored copy).
  2. Either fix the header upstream OR prepend a synthetic header via
     a jq/sed step in `make convert-spec-cloud-init`.
  3. Also: `src/cloud-init/` directory contains a HYPHEN, which Rust
     module names can't. Either rename to `src/cloud_init/` OR use
     `#[path = "cloud-init/mod.rs"] pub mod cloud_init;` shim.
  Estimated ~40-60% of HSM effort (smaller spec but anomaly resolution
  adds work).

**Error enrichment**:
- Add `url: String` and `headers: HeaderMap` fields to `Error::OchamiError`
  for richer diagnostics.

### Long-term spec-sync workflow

**Upstream tracking**: OpenCHAMI HSM spec source — URL verified at Task 0.
Refresh: no fixed cadence; monitor upstream releases or when consumers
request new endpoints.

**Regeneration procedure** (documented in CONTRIBUTING.md):
1. Fetch new upstream YAML → `src/hsm/ochami_api_docs.yaml`.
2. Update provenance header (upstream commit hash + date).
3. `make convert-spec`.
4. Review JSON diff.
5. `cargo build` — address new compile errors.
6. Update `docs/superpowers/plans/<phase-1-commit-date>-progenitor-hsm-output-reference.md` if names changed.
7. Test, bump version, update MIGRATION.md.

**Spec-patch maintenance**: periodically check whether upstream fixed
patched issues → drop patches. If patches accumulate (>10), reconsider
upstream contribution.

### Version policy

- **Pre-1.0** (current): breaking changes allowed on beta.N → beta.N+1.
  Pure fixes: beta.N.M.
- **1.0 milestone**: when API stabilized (no forced spec-driven breaks for
  2+ releases).
- **Post-1.0**: semver strict.

### Deprecation timeline for legacy paths

Skip `#[deprecated]` on legacy long-prefix module paths (would emit downstream
warning noise). Plan removal in **2.0 major bump**. Between beta.16 and 2.0,
leave as pure re-exports.

### Cross-crate coordination

- **manta-backend-dispatcher**: independent versioning (no lockstep).
  Coordinate bumps only when types change.
- **manta**: companion PR per ochami-rs release. Self-coordinated (@masber).
- **csm-rs**: parallel project, same pattern. Back-port lessons where
  applicable.

### Retrospective

Commit stub `docs/superpowers/retros/<phase-7-commit-date>-progenitor-hsm-migration.md`
during Phase 7. Fill 1-2 weeks after beta.16 release.

---

## Risks

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| Baseurl assumption (`/hsm/v2`) wrong for real OCHAMI | Low | High (rework of all wrapper URL construction) | Verify in first 30 min of Phase 1 against OpenCHAMI deployment docs / live instance |
| Progenitor 0.8 rejects the converted spec | Medium | Medium (requires spec patches; may take longer than Task 0 budget) | Spec-patching mechanism (`scripts/patch-hsm-spec.jq`); escalate to user if >3 patches needed |
| Typify's inner-field visibility on newtypes differs from csm-rs precedent | Low | Medium (constructor patterns rewrite) | Task 0 confirms; may require additional wrapper API design |
| Typify enforces regex on Deserialize (ResourceName spec patch becomes release-blocking) | Medium | Medium | Patch already sketched; verify at Task 0 |
| POST /members body-shape is bulk-only (single-shape callers broken today) | Medium | Low (no in-crate caller uses these methods currently) | Task 0 verifies empirically; drop-methods policy already defined |
| Progenitor 0.9 breaking release during migration | Low | Low (0.8 is pinned; no auto-upgrade) | Version pinning in Cargo.toml; upgrade evaluated as separate PR |
| Upstream OCHAMI spec ships incompatible v2 → v3 changes | Low | High (major re-migration) | Not currently a concern; monitor upstream repo |
| Downstream `Error::OchamiError` destructure breaks unknown consumers | Unknown | Low | `#[non_exhaustive]` added; documented in MIGRATION.md |
| Manta migration takes longer than 1-2 focused days | Medium | Low | Self-coordinated; only affects @masber's own timeline |
| Solo maintainer paused mid-migration for external commitments | Medium | Medium (branch drifts, rebase overhead) | Phase-by-phase commits keep intermediate compileable state; work-log convention aids resumption |

### Rollback triggers (post-release)

- Compile failure in >1 known consumer within 24h of beta.16 release.
- Runtime regression that timeouts/retry can't mask.
- Server-rejection of previously-working wire payloads (spec drift the design missed).

Trigger → `cargo yank` + fix-forward beta.17 (per Section 4 rollback plan).

---

## Latent Bugs

### Fixed by this migration (release notes for beta.16)

19 latent bugs in the current code fixed as side effects of the migration:

1. `hsm::component::delete` URL typo `Componnets` → `Components`.
2. `hsm::inventory::hardware::delete_all` missing leading `/` before `hsm`.
3. `hsm::inventory::redfish_endpoint::delete_all` missing leading `/`.
4. `hsm::inventory::redfish_endpoint::get_query` singular path (`RedfishEndpoint` → `RedfishEndpoints`).
5. `hsm::inventory::ethernet_interfaces::patch` bogus query params (should be body-only PATCH).
6. `hsm::inventory::ethernet_interfaces::post` `dbg!` leftover on error path.
7. `hsm::inventory::ethernet_interfaces::post` deserializes into `()` on non-empty response body — every 2xx today fails deserialization.
8. `hsm::inventory::ethernet_interfaces::delete_ip_address` unused `_group_label` param + URL casing (`/IpAddress/` → `/IPAddresses/`).
9. `state/components::get`, `inventory/hardware::get`, `partition::get`, `memberships::get`, `defaults/node_map::get` — positional `.query(&[opt, opt, ...])` bug drops filters silently.
10. `Component.nid: Option<usize>` misread of spec — becomes `Option<i64>`.
11. `state/components::Component::nid`, `subtype`, `reservation_disabled` — serialize-only rename bug (wire round-trip broken); silent-deserialize to `None`.
12. `state/components::ComponentPostByNidQuery.nid_ranger` Rust field typo.
13. `Group.exclusive_group` silent-deserialize bug (rename-serialize-only).
14. `ComponentArrayPostQuery.falg_only` Rust field typo.
15. `hsm::group::post` returns raw text (`.text()`) instead of typed JSON array.
16. `hsm::state::components::post` — wrong body shape (single Component to bulk endpoint) + deserialize-into-Component-of-empty-body. Method DROPPED post-migration.
17. `hsm::state::components::put` empty-body deserialize.
18. `hsm::component::put` empty-body deserialize.
19. `hsm::group::post_member` and `hsm::partition::post_members` body-shape mismatch (single vs bulk) — Task 0 verifies empirically.

### Known but not fixed by this migration

20. `hsm::group::utils::49` pre-existing `.get_members().push(...)` bug: `get_members()` returns `Vec<String>` by value; `push()` mutates the return value which is immediately discarded. Out of migration scope; noted for a follow-up fix.

---

## Open Questions Deferred to Task 0

> **Task 0 resolution (2026-07-13)**: all items below are answered in
> `docs/superpowers/plans/2026-07-13-progenitor-hsm-output-reference.md`.
> Highlights that override this spec's Sections 2 and 3 sketches (see
> § L of the output-reference doc for the full list):
>
> - Progenitor 0.8's default `Generator` emits **direct methods**, not
>   builders. Wrapper `run()` callsites use
>   `c.do_component_get(xname).await`, not
>   `c.do_state_components_get_one_builder().xname(x).send().await`
>   (verified against the actual generated signature —
>   `hsm_generated.rs`'s `do_component_get` takes only `xname: &'a str`,
>   no trailing `Option` arg).
> - No schema named `HsmActionResponse` exists. Alias `Response100`
>   instead: `pub use crate::hsm::generated::types::Response100 as HsmActionResponse;`.
> - `ResourceName` inner is **private** and Deserialize enforces regex.
>   Access via `Deref<Target = String>` (e.g. `&*group.label`);
>   construct via `ResourceName::from_str()` / `try_from()`. The
>   ResourceName widening patch is release-blocking.
> - `Group100.tags` is `Vec<ResourceName>` (not `Option<Vec<…>>`).
> - `Membership100` has no `ids` field — has `id: Option<XName100>` and
>   `group_labels: Vec<String>`.
> - `Members100.ids` is `Vec<XNameRw100>`, not Option.
> - POST /members body is single-member (`MemberId { id:
>   Option<XNameRw100> }`), not bulk.
> - `ComponentArrayPostQuery` has no `falg_only` typo; the field is
>   `flagonly` (single word).
> - date-time crate: `chrono` (confirmed).
> - Baseurl `{base_url}/hsm/v2` remains the working assumption based on
>   in-crate evidence, but is **not** yet verified against a real
>   deployment — see the output-reference doc's § M release checklist.
>
> No structural (routing-per-resource) rework surfaced. Phase 2+
> proceeds on the design as originally drafted, subject to the
> mechanical adjustments above.

- Baseurl `{base_url}/hsm/v2` correct for real OCHAMI deployments?
- Upstream OpenCHAMI HSM spec repository URL and license?
- Exact typify-mangled type names (`XName100`, `Component100Component`, etc.)?
- typify's newtype inner-field visibility (`pub String` vs private)?
- typify's `Default` emission per struct?
- typify's `Deref<Target = String>` on newtypes?
- Whether typify enforces regex validation at Deserialize (determines
  ResourceName-patch criticality)?
- Enum variant naming conventions?
- `NodeMap.id` required (spec) vs Optional (current code)?
- `Members100.ids` shape (`Vec<XNameRw100>` vs `Option<Vec<XNameRw100>>`)?
- `Membership100.ids` shape (same question for the /memberships endpoint's
  return type)?
- POST /members endpoint body shape (single vs bulk)?
- Typify's date-time crate for `format: date-time` fields —
  `chrono::DateTime<Utc>` (default assumption) or `time::OffsetDateTime`?
  Determines the runtime dep swap in Phase 1's Deliverables.
- Progenitor 0.8's per-request pre_hook API (if exists, enables future
  shared-pool optimization)?
- Wiremock 0.6's `up_to_n_times` API for retry tests?
- Whether inventory/ethernet_interfaces and inventory/redfish_endpoint types
  slim-re-export cleanly OR stay hand-rolled?
- Additional spec patches beyond ResourceName (identified during first
  `cargo build` attempt)?
- Path-param typing: does typify's builder accept `impl AsRef<str>` OR
  require newtype wrapping?

---

## Appendix: Cargo.toml diff

Added under `[package]`:
```toml
rust-version = "1.91.1"
resolver = "2"
readme = "README.md"
keywords = ["hpc", "ochami", "openchami", "hsm"]
categories = ["api-bindings", "asynchronous"]

[package.metadata.cargo-machete]
ignored = ["regress"]
```

Added under `[dependencies]`:
```toml
progenitor-client = "0.8"
regress = "0.10"
http = "1.3.1"
chrono = { version = "0.4.45", default-features = false, features = ["clock", "serde"] }
```

Extended (existing dep, additional feature):
```toml
tokio = { version = "1.45", features = ["macros", "rt-multi-thread", "time"] }
# ^ added "time" to features (Phase 2, for retry_on_5xx's tokio::time::sleep)
```

New sections:
```toml
[build-dependencies]
progenitor = "0.8"
openapiv3 = "2"
serde_json = "1"
prettyplease = "0.2"
syn = "2"

[dev-dependencies]
wiremock = "0.6"
```

Version bump: `1.0.0-beta.15` → `1.0.0-beta.16`.

`src/lib.rs` gains `#![deny(unsafe_code)]` and `pub use backend_connector::Ochami;`.
