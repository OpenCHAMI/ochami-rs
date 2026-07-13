# Progenitor HSM Client Migration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace ~11 hand-written `hsm/**/http_client.rs` modules with a progenitor-generated HTTP client derived from `src/hsm/ochami_api_docs.yaml`, exposed via `impl Ochami { hsm_*() }` methods.

**Architecture:** Codegen pipeline: swagger2openapi + jq patches → OpenAPI 3.0 JSON (committed) → build.rs runs progenitor → generated Rust in `$OUT_DIR` → wrapper layer exposes typed `impl Ochami { hsm_* }` methods. Some methods stay on raw reqwest due to multi-value query semantics or hand-rolled Redfish types.

**Tech Stack:** Rust 1.91.1+, progenitor 0.8, openapiv3 2, prettyplease 0.2, wiremock 0.6 for tests. Existing: reqwest 0.12, tokio 1.45, serde/serde_json.

## Global Constraints

- **Design spec:** `docs/superpowers/specs/2026-07-13-progenitor-hsm-client-design.md` (2034 lines). Every task references its Section/Phase for full context.
- **Rust MSRV:** 1.91.1 (matches csm-rs; set in Cargo.toml this migration).
- **Version bump:** `1.0.0-beta.15` → `1.0.0-beta.16`.
- **Scope:** HSM only. BSS/PCS/cloud-init deferred to future migrations (see spec's Section 5).
- **PR shape:** ONE PR, 23 commits (1 per task; Task 13 = Phase 5 atomic combines types + http_client-deletion).
- **Compileability invariant:** Every commit leaves `cargo build` green (except mid-Task-13 intermediate state — Phase 5 is atomic).
- **No `cargo test` regression:** Existing tests must pass at every task boundary.
- **All commits use conventional-commit prefixes** (`chore`, `feat`, `refactor`, `test`, `docs`) per each task's convention.
- **Body of every commit** references the design spec: `docs/superpowers/specs/2026-07-13-progenitor-hsm-client-design.md § <Section>`.
- **Prerequisites** (verify before Task 1): Rust 1.91.1+, `npm`/`npx`, `jq`, internet access for npm registry.

## Task Layout

| Task | Phase | Deliverable |
|---|---|---|
| 1 | Phase 1 (Task 0) | Spec conversion, build.rs, generated.rs, cargo build works |
| 2 | Phase 2 | Infrastructure: Error, http.rs helpers, wrapper/mod.rs, hsm/types.rs |
| 3-11 | Phase 3 (9 files) | 9 wrapper files with `impl Ochami { hsm_*() }` methods |
| 12 | Phase 4 | Migrate backend_connector.rs + hsm/group/utils.rs to wrapper methods |
| 13 | Phase 5 (atomic) | Slim types re-exports + delete 11 http_client.rs files + tests.rs update |
| 14-22 | Phase 6 (9 files) | 9 wiremock test files + retry tests + helper unit tests |
| 23 | Phase 7 | Docs (README, MIGRATION, CHANGELOG, CONTRIBUTING, SECURITY, module docstrings, retro stub) |

---

## Task 1: Phase 1 — Task 0 (Spec Conversion + First Progenitor Run)

**Design spec reference:** § Section 1 → "Phase 1 (Task 0) detail".

**Files:**
- Create: `src/hsm/ochami_api_docs.openapi3.json` (~1-1.5 MB)
- Create: `scripts/patch-hsm-spec.jq`
- Create: `Makefile`
- Create: `build.rs`
- Create: `src/hsm/generated.rs`
- Create: `docs/superpowers/plans/<YYYY-MM-DD>-progenitor-hsm-output-reference.md`
- Modify: `Cargo.toml` (add build-deps + runtime deps)
- Modify: `src/hsm/mod.rs` (add `pub(crate) mod generated;`)

**Interfaces:**
- Consumes: existing `src/hsm/ochami_api_docs.yaml` (Swagger 2.0, 12327 lines).
- Produces: `src/hsm/generated::Client` (progenitor-generated); `src/hsm/generated::types::*` (all typify-generated types).

- [ ] **Step 1: Verify baseurl assumption (~30 min)**

Check OpenCHAMI SMD deployment docs OR curl a live instance:
```bash
curl -sv https://<ochami-host>/hsm/v2/State/Components 2>&1 | grep "HTTP/"
# Expected: 401 Unauthorized (confirms path resolves)
```
If neither docs nor live instance available, proceed with in-crate evidence (all `backend_connector.rs`-exercised modules use `/hsm/v2`) and add "verify against real deployment" to the release checklist.

- [ ] **Step 2: Prerequisites sanity check**

```bash
rustc --version           # Expect 1.91.1+
node --version && npm --version  # Any recent version
jq --version              # Any recent version
npx --yes swagger2openapi@7.0.8 --help  # Fetches package if needed
```

- [ ] **Step 3: Create `Makefile`**

```makefile
.PHONY: convert-spec

convert-spec:
	npx --yes swagger2openapi@7.0.8 src/hsm/ochami_api_docs.yaml -o /tmp/hsm-raw.json
	jq -f scripts/patch-hsm-spec.jq /tmp/hsm-raw.json > src/hsm/ochami_api_docs.openapi3.json
	rm /tmp/hsm-raw.json
```

- [ ] **Step 4: Create `scripts/patch-hsm-spec.jq`**

```jq
# Patches applied to swagger2openapi output before progenitor consumes it.
# Each patch has an explanation. Periodically review whether upstream fixed
# the underlying issue and drop the patch.

# Widen ResourceName pattern to accept real HSM labels ("compute", "worker" —
# spec's ^[0-9a-f_\-.] rejects letters beyond 'f'). Server-side may still
# enforce narrower; this patch fixes client-side deserialization.
# TODO: file upstream PR to fix at source.
.components.schemas.ResourceName.pattern = "^[0-9a-zA-Z._-]{1,}$"
```

- [ ] **Step 5: Run `make convert-spec` and capture warnings**

```bash
make convert-spec 2>&1 | tee /tmp/convert-warnings.log
# Review /tmp/convert-warnings.log for swagger2openapi warnings — they may
# indicate constructs that trip progenitor.
```
Expected: `src/hsm/ochami_api_docs.openapi3.json` created (~1-1.5 MB).

- [ ] **Step 6: Verify convert-spec idempotency**

```bash
make convert-spec
git diff --exit-code src/hsm/ochami_api_docs.openapi3.json
# Expected: no output (exit 0). If diff appears, jq patch is non-idempotent.
```

- [ ] **Step 7: Update `Cargo.toml`**

Add under `[package]`:
```toml
version = "1.0.0-beta.16"       # was: 1.0.0-beta.15
rust-version = "1.91.1"
resolver = "2"
readme = "README.md"

[package.metadata.cargo-machete]
ignored = ["regress"]
```

Extend the existing `tokio` dep (add `"time"` feature):
```toml
tokio = { version = "1.45", features = ["macros", "rt-multi-thread", "time"] }
```

Add to `[dependencies]`:
```toml
progenitor-client = "0.8"
regress = "0.10"
http = "1.3.1"
chrono = { version = "0.4.45", default-features = false, features = ["clock", "serde"] }
```

Add new sections:
```toml
[build-dependencies]
progenitor = "0.8"
openapiv3 = "2"
serde_json = "1"
prettyplease = "0.2"
syn = "2"
```

- [ ] **Step 8: Create `build.rs` (adapts csm-rs pattern)**

```rust
//! Build-time codegen of the HSM HTTP client from the committed
//! OpenAPI 3.0 spec via progenitor.

use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let spec_path = manifest_dir.join("src/hsm/ochami_api_docs.openapi3.json");
    let out_path = out_dir.join("hsm_generated.rs");

    println!("cargo:rerun-if-changed={}", spec_path.display());

    let src = fs::read_to_string(&spec_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", spec_path.display()));

    let mut spec: openapiv3::OpenAPI = serde_json::from_str(&src)
        .unwrap_or_else(|e| panic!("parse {} as JSON: {e}", spec_path.display()));

    // Wrap info.description in ```text fence so rustdoc doesn't try to
    // compile embedded markdown code fences as doctests.
    if let Some(desc) = spec.info.description.as_mut() {
        let escaped = desc.replace("```", "``\u{200B}`");
        *desc = format!("```text\n{escaped}\n```");
    }

    let mut generator = progenitor::Generator::default();
    let tokens = generator.generate_tokens(&spec).unwrap_or_else(|e| {
        panic!(
            "progenitor codegen failed for {}: {e}. \
             Check the spec is valid OpenAPI 3.0 and contains no \
             progenitor-unsupported constructs.",
            spec_path.display()
        )
    });
    let ast: syn::File = syn::parse2(tokens).expect("generated tokens do not parse");
    let pretty = prettyplease::unparse(&ast);

    fs::write(&out_path, pretty)
        .unwrap_or_else(|e| panic!("write {}: {e}", out_path.display()));
}
```

- [ ] **Step 9: Create `src/hsm/generated.rs`**

```rust
//! progenitor-generated HSM client. Output of `build.rs`, included verbatim.
//!
//! `pub(crate)` because only the wrapper layer in `crate::hsm::wrapper` and
//! per-resource `types.rs` re-export aliases are allowed to touch the
//! generated symbols. Public consumers go through `Ochami`.
#![allow(dead_code, clippy::all, missing_docs, non_camel_case_types, non_snake_case, unused_imports)]
include!(concat!(env!("OUT_DIR"), "/hsm_generated.rs"));
```

- [ ] **Step 10: Add `pub(crate) mod generated;` to `src/hsm/mod.rs`**

Prepend (before existing `pub mod component;` etc.):
```rust
pub(crate) mod generated;
```

- [ ] **Step 11: Run `cargo build` (iterate until green)**

```bash
cargo build 2>&1 | tee /tmp/cargo-build.log
```
- On progenitor error: read `/tmp/convert-warnings.log` + build error message; add jq patch to `scripts/patch-hsm-spec.jq`; rerun `make convert-spec && cargo build`.
- On chrono/time crate mismatch: if error is "unresolved crate `time`", swap `chrono` → `time = { version = "0.3", features = ["serde"] }` in `Cargo.toml`.
- If >3 iterations OR >8h total time: **escalate to @masber** (per Task 0 escalation policy in spec).

Expected: `Finished` (build success).

- [ ] **Step 12: Verify generated code compiles with rustdoc**

```bash
cargo doc --no-deps
```
Expected: no warnings, no broken intra-doc-links.

- [ ] **Step 13: Verify existing tests still pass**

```bash
cargo test
```
Expected: existing `hsm/group/tests.rs` and any other tests pass unchanged.

- [ ] **Step 14: Locate generated code and inspect**

```bash
find target -name hsm_generated.rs -path '*/build/*' | head -1
# Copy the path; open the file in your editor for step 15.
```

- [ ] **Step 15: Write `docs/superpowers/plans/<YYYY-MM-DD>-progenitor-hsm-output-reference.md`**

Replace `<YYYY-MM-DD>` with today's date. Use csm-rs's `docs/superpowers/plans/2026-06-13-progenitor-bss-output-reference.md` at `github.com/eth-cscs/csm-rs` as the format template.

Record for each Verify item from Section 1 § Task 0 Verifies list:
- Baseurl verified against OCHAMI (yes/no + how).
- Upstream spec repo URL.
- OpenCHAMI spec license.
- Typify newtype inner-field visibility (`pub String` vs private).
- Typify `Default` emission per struct.
- Typify `Deref<Target = String>` on newtypes.
- Regex validation at Deserialize (affects ResourceName-patch criticality).
- Exact mangled names (grep for `pub struct Component100`, `pub enum HmsType100`, `pub fn do_state_components_get`, etc.).
- Duplicate-method spec-conformance findings.
- POST /members body-shape observations.
- `Members100.ids` / `Membership100.ids` shape.
- Wiremock `up_to_n_times` API existence (Task 0 or Phase 6 verification).
- Progenitor 0.8 pre_hook API existence.
- Typify date-time crate (chrono vs time).
- swagger2openapi warnings captured.

- [ ] **Step 16: Design reconciliation review (~1h)**

Compare Task 0 findings against Section 2/3 assumptions in the design spec. If XName isn't emitted as newtype, or naming differs from csm-rs precedent significantly, update the design spec **in-place** in this commit and note the change in the commit body. Structural rework (routing decisions per resource change): checkpoint with @masber before Phase 2.

- [ ] **Step 17: Commit**

```bash
git add src/hsm/ochami_api_docs.openapi3.json scripts/patch-hsm-spec.jq \
        Makefile build.rs src/hsm/generated.rs src/hsm/mod.rs Cargo.toml \
        docs/superpowers/plans/<YYYY-MM-DD>-progenitor-hsm-output-reference.md
git commit -m "$(cat <<'EOF'
chore(hsm): Task 0 — vendor OpenAPI 3.0 JSON + progenitor output reference

Runs progenitor end-to-end against the converted HSM spec, captures
generated type/method names in the output-reference doc, and lands the
codegen infrastructure (build.rs, generated.rs, runtime deps).

References: docs/superpowers/specs/2026-07-13-progenitor-hsm-client-design.md § Section 1 (Task 0)
EOF
)"
```

---

## Task 2: Phase 2 — Infrastructure

**Design spec reference:** § Section 4 → "Phase 2 (Infrastructure) detail".

**Files:**
- Modify: `src/error.rs` (enrich `Error::OchamiError`)
- Modify: `src/pcs/power_cap/http_client.rs`, `src/pcs/power_status/http_client.rs` (5 sites)
- Modify: all 11 `src/hsm/**/http_client.rs` files (~25 sites)
- Modify: `src/http.rs` (add helpers, timeouts)
- Create: `src/hsm/types.rs` (HsmActionResponse alias)
- Create: `src/hsm/wrapper/mod.rs` (gen_client, map_err, run)
- Modify: `src/hsm/mod.rs` (add `pub mod types;` and `mod wrapper;`)
- Modify: `src/backend_connector.rs` (add Ochami accessors)
- Modify: `src/lib.rs` (add deny + pub use)

**Interfaces:**
- Consumes: `src/hsm/generated::Client`, `src/hsm/generated::types::Response100` (from Task 1).
- Produces: `Error::OchamiError { status: u16, body: Value }`, `crate::http::{handle_json_response, handle_action_response, retry_on_5xx}`, `crate::hsm::types::HsmActionResponse`, `crate::hsm::wrapper::{gen_client, map_err, run}`, `Ochami::base_url()`, `Ochami::root_cert()`.

- [ ] **Step 1: Enrich `Error::OchamiError` in `src/error.rs`**

Replace the existing variant:
```rust
// Before:
#[error("OCHAMI-RS > OCHAMI: {0}")]
OchamiError(Value),

// After:
#[error("OCHAMI-RS > OCHAMI: status={status} body={body}")]
#[non_exhaustive]
OchamiError { status: u16, body: Value },
```

- [ ] **Step 2: Update all ~30 Error::OchamiError emission sites**

Find them:
```bash
grep -rn "Error::OchamiError(" src/
# Expected: ~30 sites across src/pcs/power_cap/http_client.rs,
# src/pcs/power_status/http_client.rs, and src/hsm/**/http_client.rs
```

Per-site pattern (mechanical rename):
```rust
// Before:
Err(Error::OchamiError(payload))

// After:
Err(Error::OchamiError { status: response.status().as_u16(), body: payload })
```

`response` is the reqwest::Response variable in scope; check per-site for the actual variable name (some may be `resp`).

- [ ] **Step 3: Verify `cargo build` after Error enrichment**

```bash
cargo build
```
Expected: green (all ~30 sites updated).

- [ ] **Step 4: Extend `src/http.rs`**

Add at top of file:
```rust
use std::time::Duration;
use serde::de::DeserializeOwned;
use serde_json::Value;

pub(crate) const HTTP_CONNECT_TIMEOUT: Duration = Duration::from_mins(45);
pub(crate) const HTTP_REQUEST_TIMEOUT: Duration = Duration::from_mins(15);
pub(crate) const HTTP_5XX_RETRY_ATTEMPTS: u32 = 3;
pub(crate) const HTTP_5XX_RETRY_INITIAL_DELAY: Duration = Duration::from_millis(500);
```

Update existing `build_client` to include timeouts:
```rust
pub fn build_client(root_cert: &[u8]) -> Result<reqwest::Client, Error> {
    let builder = reqwest::Client::builder()
        .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?)
        .use_rustls_tls()
        .connect_timeout(HTTP_CONNECT_TIMEOUT)
        .timeout(HTTP_REQUEST_TIMEOUT);
    builder.build().map_err(Error::NetError)
}
```

Add new helpers:
```rust
pub(crate) async fn handle_json_response<T: DeserializeOwned>(
    response: reqwest::Response,
) -> Result<T, Error> {
    let status = response.status();
    if !status.is_success() {
        let payload = response.json::<Value>().await.unwrap_or(Value::Null);
        return Err(Error::OchamiError { status: status.as_u16(), body: payload });
    }
    response.json::<T>().await.map_err(Error::NetError)
}

pub(crate) async fn handle_action_response(
    response: reqwest::Response,
) -> Result<crate::hsm::types::HsmActionResponse, Error> {
    handle_json_response(response).await
}

pub(crate) async fn retry_on_5xx<F, Fut, T>(mut op: F) -> Result<T, Error>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, Error>>,
{
    let mut delay = HTTP_5XX_RETRY_INITIAL_DELAY;
    let mut last_err: Option<Error> = None;
    for attempt in 0..HTTP_5XX_RETRY_ATTEMPTS {
        match op().await {
            Ok(v) => return Ok(v),
            Err(e) => {
                let retry = matches!(&e, Error::OchamiError { status, .. } if (500..600).contains(status));
                if !retry || attempt + 1 >= HTTP_5XX_RETRY_ATTEMPTS {
                    return Err(e);
                }
                last_err = Some(e);
                tokio::time::sleep(delay).await;
                delay *= 2;
            }
        }
    }
    Err(last_err.unwrap_or_else(|| Error::Message("retry logic invariant broken".into())))
}
```

- [ ] **Step 5: Add Ochami accessors in `src/backend_connector.rs`**

Add near the existing `Ochami::new`:
```rust
impl Ochami {
    pub(crate) fn base_url(&self) -> &str {
        &self.base_url
    }
    pub(crate) fn root_cert(&self) -> &[u8] {
        &self.root_cert
    }
}
```

- [ ] **Step 6: Add to `src/lib.rs`**

Prepend:
```rust
#![deny(unsafe_code)]
```

Add at end:
```rust
pub use backend_connector::Ochami;
```

- [ ] **Step 7: Create `src/hsm/types.rs`**

```rust
//! Shared HSM response types.
pub use crate::hsm::generated::types::Response100 as HsmActionResponse;

use manta_backend_dispatcher::types::HsmActionResponse as FrontEndHsmActionResponse;

impl From<FrontEndHsmActionResponse> for HsmActionResponse {
    fn from(value: FrontEndHsmActionResponse) -> Self {
        HsmActionResponse {
            code: value.code,
            message: value.message,
        }
    }
}

impl From<HsmActionResponse> for FrontEndHsmActionResponse {
    fn from(value: HsmActionResponse) -> Self {
        FrontEndHsmActionResponse {
            code: value.code,
            message: value.message,
        }
    }
}
```

Verify dispatcher's `HsmActionResponse` field shape matches — if different, adapt.

- [ ] **Step 8: Add `pub mod types;` to `src/hsm/mod.rs`**

Between `pub(crate) mod generated;` (from Task 1) and existing `pub mod component;`:
```rust
pub mod types;
```

Verify: `cargo build` still green.

- [ ] **Step 9: Create `src/hsm/wrapper/mod.rs`**

```rust
//! Thin wrapper bridging the generated HSM client to the public
//! `Ochami` API. Uses per-call reqwest client construction (matches
//! csm-rs). Shared pool via progenitor pre_hook is deferred (per
//! Section 5 follow-up).

use std::fmt::Debug;
use crate::{Ochami, error::Error, hsm::generated};

pub(crate) fn gen_client(client: &Ochami, token: &str) -> Result<generated::Client, Error> {
    let inner = crate::http::build_client(client.root_cert())?;
    let baseurl = format!("{}/hsm/v2", client.base_url());
    // NOTE: bearer auth is applied per-request via reqwest's `.bearer_auth()`
    // in each wrapper method body since reqwest doesn't allow inserting
    // default headers post-build.
    Ok(generated::Client::new_with_client(&baseurl, inner))
}

pub(crate) async fn map_err<E: Debug>(err: progenitor_client::Error<E>) -> Error {
    use progenitor_client::Error::*;
    match err {
        InvalidRequest(s) => Error::Message(format!("HSM invalid request: {s}")),
        CommunicationError(e) | InvalidUpgrade(e) => Error::NetError(e),
        ErrorResponse(rv) => {
            let status = rv.status().as_u16();
            Error::Message(format!(
                "HSM error response: status={status} body={:?}",
                rv.into_inner()
            ))
        }
        ResponseBodyError(e) => Error::NetError(e),
        InvalidResponsePayload(_, e) => Error::SerdeError(e),
        UnexpectedResponse(resp) => {
            let status = resp.status().as_u16();
            let bytes = resp.bytes().await.unwrap_or_default();
            match serde_json::from_slice::<serde_json::Value>(&bytes) {
                Ok(body) => Error::OchamiError { status, body },
                Err(_) => Error::Message(format!(
                    "HSM unexpected: status={status} body={}",
                    String::from_utf8_lossy(&bytes)
                )),
            }
        }
        PreHookError(s) => Error::Message(format!("HSM pre-hook error: {s}")),
    }
}

pub(crate) async fn run<F, Fut, T, E>(
    client: &Ochami,
    token: &str,
    op: F,
) -> Result<T, Error>
where
    F: FnOnce(generated::Client) -> Fut,
    Fut: std::future::Future<
        Output = Result<
            progenitor_client::ResponseValue<T>,
            progenitor_client::Error<E>,
        >,
    >,
    E: Debug,
{
    let gc = gen_client(client, token)?;
    match op(gc).await {
        Ok(rv) => Ok(rv.into_inner()),
        Err(e) => Err(map_err(e).await),
    }
}
```

- [ ] **Step 10: Add `mod wrapper;` to `src/hsm/mod.rs`**

```rust
mod wrapper;
```

- [ ] **Step 11: Verify `cargo build`, `cargo test`, `cargo doc`**

```bash
cargo build      # green
cargo test       # existing tests still pass
cargo doc --no-deps  # no warnings
```

- [ ] **Step 12: (Optional but recommended) Add helper unit tests**

In `src/http.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn retry_on_5xx_retries_then_succeeds() {
        use std::sync::atomic::{AtomicU32, Ordering};
        let counter = AtomicU32::new(0);
        let result: Result<u32, Error> = retry_on_5xx(|| async {
            let n = counter.fetch_add(1, Ordering::SeqCst);
            if n < 2 {
                Err(Error::OchamiError { status: 503, body: serde_json::Value::Null })
            } else {
                Ok(42)
            }
        }).await;
        assert_eq!(result.unwrap(), 42);
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }
}
```

In `src/hsm/wrapper/mod.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn map_err_unexpected_response_maps_to_ochami_error() {
        // ... test implementation
    }
}
```

- [ ] **Step 13: Commit**

```bash
git add src/error.rs src/pcs/**/http_client.rs src/hsm/**/http_client.rs \
        src/http.rs src/hsm/types.rs src/hsm/wrapper/mod.rs src/hsm/mod.rs \
        src/backend_connector.rs src/lib.rs Cargo.toml
git commit -m "$(cat <<'EOF'
feat(hsm): infrastructure — wrapper helpers, Error enrichment, http.rs additions

Adds shared helpers (gen_client, map_err, run, retry_on_5xx,
handle_json_response) required by Phase 3 wrappers. Enriches
Error::OchamiError to carry status. Updates ~30 emission sites
(5 PCS + ~25 HSM) to the new shape.

References: docs/superpowers/specs/2026-07-13-progenitor-hsm-client-design.md § Section 4 (Phase 2 detail)
EOF
)"
```

---

## Tasks 3-11: Phase 3 — Wrapper `impl Ochami` blocks (9 files)

**Design spec reference:** § Section 4 → "Phase 3 (Wrapper impl blocks) detail".

Tasks 3-11 each create ONE wrapper file. The pattern is uniform; Task 3 (component) shows it in detail. Subsequent tasks (4-11) follow the same structure with resource-specific method sets. Time budget per file: 1-6h (see spec § Phase 3 per-file time budget).

**Common file structure** for each wrapper file:
- Import generated types via `crate::hsm::generated::types::*` (Section 3 slim-re-export path — NOT the yet-to-be-slim-re-exported `hsm::<resource>::types::*`).
- One `impl Ochami { ... }` block containing all methods for the resource.
- Each method: `pub async fn hsm_<resource>_<action>(&self, token: &str, ...) -> Result<T, Error>`.
- Each method has `#[allow(clippy::too_many_arguments)]` if >7 params.
- Each method has doc comment with one-line summary + `# Errors` section.
- GET methods wrapped in `retry_on_5xx(|| async { ... }).await`.
- POST/PUT/PATCH/DELETE methods call `run(...)` directly, no retry.
- Raw-reqwest methods use `crate::http::build_client(&self.root_cert())?.bearer_auth(token)` chain, deserialize via `crate::http::handle_json_response<T>` or `handle_action_response`.

---

### Task 3: Phase 3 — wrapper/component.rs (~12 methods)

**Files:**
- Create: `src/hsm/wrapper/component.rs`
- Modify: `src/hsm/wrapper/mod.rs` (add `mod component;`)

**Interfaces:**
- Consumes: Task 2's wrapper helpers, generated types from Task 1.
- Produces: `Ochami::hsm_component_get_all`, `hsm_component_get_all_nodes`, `hsm_component_get`, `hsm_component_get_one`, `hsm_component_get_by_nid`, `hsm_component_get_query`, `hsm_component_post`, `hsm_component_post_query`, `hsm_component_post_bynid_query`, `hsm_component_put`, `hsm_component_delete_one`, `hsm_component_delete` (12 methods).

**Method routing** (per spec Section 2):
- Raw reqwest: `hsm_component_get` (multi-value nid), `hsm_component_get_query` (same).
- Progenitor-routed: rest.

- [ ] **Step 1: Create `src/hsm/wrapper/component.rs`**

Top of file:
```rust
//! Wrapper for /State/Components. Replaces src/hsm/component/http_client.rs
//! and src/hsm/state/components/http_client.rs (unified).
//!
//! Two methods stay on raw reqwest for multi-value query semantics:
//! - `hsm_component_get` — comma-split `nid` used by backend_connector::nid_to_xname.
//! - `hsm_component_get_query` — same query-param set.
//!
//! All other methods route through the generated progenitor client.

use serde_json::Value;
use crate::{
    Ochami,
    error::Error,
    hsm::{
        generated::types::{
            Component100Component as Component,
            ComponentArrayComponentArray as ComponentArray,
            ComponentArrayPostArray, ComponentArrayPostByNidQuery,
            ComponentArrayPostQuery, Component100Put as ComponentPut,
        },
        types::HsmActionResponse,
        wrapper::{run, gen_client, map_err},
    },
    http::{build_client, handle_json_response, handle_action_response, retry_on_5xx},
};

impl Ochami {
    // Methods below.
}
```

- [ ] **Step 2: Add convenience methods (delegate to raw `hsm_component_get`)**

Inside `impl Ochami { }`:
```rust
    /// Fetch all HSM components.
    ///
    /// # Errors
    /// Returns [`Error`] on transport, deserialization, or HSM failure.
    pub async fn hsm_component_get_all(
        &self,
        token: &str,
        nid_only: Option<&str>,
    ) -> Result<ComponentArray, Error> {
        self.hsm_component_get(
            token, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, nid_only,
        ).await
    }

    /// Fetch all HSM components filtered to type=Node.
    ///
    /// # Errors
    /// Same as [`hsm_component_get`].
    pub async fn hsm_component_get_all_nodes(
        &self,
        token: &str,
        nid_only: Option<&str>,
    ) -> Result<ComponentArray, Error> {
        self.hsm_component_get(
            token, None, Some("Node"), None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, nid_only,
        ).await
    }
```

- [ ] **Step 3: Add raw-reqwest `hsm_component_get` (multi-value nid)**

```rust
    /// GET /State/Components — with query filters. Comma-separated `nid`
    /// is split into repeated `?nid=` params (preserves the historical
    /// multi-value semantics that backend_connector::nid_to_xname relies on).
    ///
    /// # Errors
    /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization failure.
    #[allow(clippy::too_many_arguments)]
    pub async fn hsm_component_get(
        &self,
        token: &str,
        id: Option<&str>,
        r#type: Option<&str>,
        state: Option<&str>,
        flag: Option<&str>,
        role: Option<&str>,
        subrole: Option<&str>,
        enabled: Option<&str>,
        software_status: Option<&str>,
        subtype: Option<&str>,
        arch: Option<&str>,
        class: Option<&str>,
        nid: Option<&str>,
        nid_start: Option<&str>,
        nid_end: Option<&str>,
        partition: Option<&str>,
        group: Option<&str>,
        state_only: Option<&str>,
        flag_only: Option<&str>,
        role_only: Option<&str>,
        nid_only: Option<&str>,
    ) -> Result<ComponentArray, Error> {
        retry_on_5xx(|| async {
            let client = build_client(self.root_cert())?;
            let url = format!("{}/hsm/v2/State/Components", self.base_url());

            let nid_vec = nid.map(|nids| {
                nids.split(',').map(|n| ("nid", Some(n))).collect::<Vec<_>>()
            });

            let mut query_params: Vec<(&str, Option<&str>)> = vec![
                ("id", id), ("type", r#type), ("state", state), ("flag", flag),
                ("role", role), ("subrole", subrole), ("enabled", enabled),
                ("softwarestatus", software_status), ("subtype", subtype),
                ("arch", arch), ("class", class), ("nidstart", nid_start),
                ("nidend", nid_end), ("partition", partition), ("group", group),
                ("stateonly", state_only), ("flagonly", flag_only),
                ("roleonly", role_only), ("nidonly", nid_only),
            ];
            if let Some(mut nids) = nid_vec { query_params.append(&mut nids); }

            let response = client.get(&url).query(&query_params)
                .bearer_auth(token).send().await?;
            handle_json_response::<ComponentArray>(response).await
        }).await
    }
```

- [ ] **Step 4: Add progenitor-routed methods**

```rust
    /// GET /State/Components/{xname}. Fetch one component by xname.
    ///
    /// # Errors
    /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization failure.
    pub async fn hsm_component_get_one(
        &self,
        token: &str,
        xname: &str,
    ) -> Result<Component, Error> {
        retry_on_5xx(|| async {
            let gc = gen_client(self, token)?;
            let result = gc.do_state_components_get_one_builder()
                .xname(xname)  // may need XName100(xname.to_string()) per Task 0
                .send().await;
            match result {
                Ok(rv) => Ok(rv.into_inner()),
                Err(e) => Err(map_err(e).await),
            }
        }).await
    }

    /// GET /State/Components/ByNID/{nid}. Fetch one component by NID.
    ///
    /// # Errors
    /// See [`hsm_component_get_one`].
    pub async fn hsm_component_get_by_nid(
        &self,
        token: &str,
        nid: &str,
    ) -> Result<Component, Error> {
        let nid_int: i64 = nid.parse().map_err(|_| {
            Error::Message(format!("invalid NID: {nid}"))
        })?;
        retry_on_5xx(|| async {
            let gc = gen_client(self, token)?;
            let result = gc.do_state_components_by_nid_get_builder()
                .nid(nid_int)
                .send().await;
            match result {
                Ok(rv) => Ok(rv.into_inner()),
                Err(e) => Err(map_err(e).await),
            }
        }).await
    }

    // ... (repeat pattern for get_query [raw], post, post_query,
    //      post_bynid_query, put, delete_one, delete)
```

Continue for the remaining ~7 methods. Signatures should match current `hsm/component/http_client.rs` and `hsm/state/components/http_client.rs`, minus `base_url`/`root_cert` args, with `token` at position 2.

- [ ] **Step 5: Add `mod component;` to `src/hsm/wrapper/mod.rs`**

Below the shared helpers:
```rust
mod component;
```

- [ ] **Step 6: Verify `cargo build`**

```bash
cargo build
```
Expected: green. If a method fails (typify's builder param name differs from expectation), consult Task 1's output-reference doc for the actual builder method names and adjust.

- [ ] **Step 7: Verify no clippy warnings**

```bash
cargo clippy --no-deps -- -D warnings
```
Allow: `#[allow(clippy::too_many_arguments)]` where applicable.

- [ ] **Step 8: Commit**

```bash
git add src/hsm/wrapper/component.rs src/hsm/wrapper/mod.rs
git commit -m "$(cat <<'EOF'
feat(hsm): wrapper/component.rs — impl Ochami methods for /State/Components

12 methods covering the component/state-components unified endpoint
family. Two methods (get, get_query) stay on raw reqwest for
multi-value nid semantics; rest route through progenitor.

References: docs/superpowers/specs/2026-07-13-progenitor-hsm-client-design.md § Section 4 (Phase 3 detail)
EOF
)"
```

---

### Phase 3 Wrapper Template (referenced by Tasks 4-11)

Every Phase 3 task follows this same 8-step shape. Task 3 fleshes it out for `component.rs`; Tasks 4-11 reuse the shape below and only vary in method list, routing table, and imports. **Do NOT skip these steps in Tasks 4-11 — each task's step list below cross-references the template step by number.**

- Template Step A: Create the wrapper file with module docstring header + imports.
- Template Step B: Add convenience methods (thin wrappers that delegate to the base GET with common query params filled).
- Template Step C: Add raw-reqwest methods (for multi-value query semantics or hand-rolled types).
- Template Step D: Add progenitor-routed methods (via `wrapper::run` or manual `gen_client(...)` + `map_err` for GETs needing `retry_on_5xx`).
- Template Step E: Add `mod <resource>;` to `src/hsm/wrapper/mod.rs`.
- Template Step F: Run `cargo build`, iterate on typify builder-method names as needed.
- Template Step G: Run `cargo clippy --no-deps -- -D warnings`; add `#[allow(clippy::too_many_arguments)]` where needed.
- Template Step H: Commit with the task's message.

**Routing decision rule** (applied per method):
- Raw reqwest if: query has multi-value semantics (comma-split into repeated `?k=`), OR types are hand-rolled per Section 3.
- Progenitor-routed otherwise.

---

### Task 4: Phase 3 — wrapper/group.rs (9 methods)

**Files:**
- Create: `src/hsm/wrapper/group.rs`
- Modify: `src/hsm/wrapper/mod.rs` (add `mod group;`)

**Interfaces:**
- Consumes: Task 2's helpers, generated types from Task 1.
- Produces: `Ochami::hsm_group_get_all(token) -> Result<GroupArray>`, `hsm_group_get(token, group_name_vec: Option<&[&str]>, tags: Option<&str>) -> Result<GroupArray>`, `hsm_group_get_one(token, label) -> Result<Group>`, `hsm_group_get_labels(token) -> Result<Vec<String>>`, `hsm_group_get_members(token, label) -> Result<Members>`, `hsm_group_post(token, group: &Group) -> Result<HsmActionResponse>`, `hsm_group_post_member(token, label, xname_id: &str) -> Result<HsmActionResponse>`, `hsm_group_delete_one(token, label) -> Result<HsmActionResponse>`, `hsm_group_delete_member(token, label, xname_id: &str) -> Result<HsmActionResponse>`.

**Routing table**:
| Method | Route |
|---|---|
| `hsm_group_get` | Raw (multi-value `group=` query built from `group_name_vec`) |
| `hsm_group_get_all` | Convenience — delegate to `hsm_group_get(token, None, None)` |
| all others | Progenitor-routed |

- [ ] **Step 1: Template Step A** — Create `src/hsm/wrapper/group.rs` with header:
```rust
//! Wrapper for /groups. Replaces src/hsm/group/http_client.rs.
//! `hsm_group_get` stays on raw reqwest for multi-value `group=` query support.
use crate::{Ochami, error::Error, hsm::{generated::types::{Group100Group as Group, GroupArrayGroupArray as GroupArray, Group100Members as Members}, types::HsmActionResponse, wrapper::{run, gen_client, map_err}}, http::{build_client, handle_json_response, handle_action_response, retry_on_5xx}};
impl Ochami { /* methods below */ }
```

- [ ] **Step 2: Template Step B** — Add `hsm_group_get_all` as thin delegate to `hsm_group_get(token, None, None)`.

- [ ] **Step 3: Template Step C** — Add raw-reqwest `hsm_group_get` (multi-value `group=` from `group_name_vec: Option<&[&str]>`, wrapped in `retry_on_5xx`).

- [ ] **Step 4: Template Step D** — Add 6 progenitor-routed methods (`get_one`, `get_labels`, `get_members`, `post`, `post_member`, `delete_one`, `delete_member`). GETs wrapped in `retry_on_5xx`; POSTs/DELETEs direct via `wrapper::run`.

- [ ] **Step 5: Template Step E** — In `src/hsm/wrapper/mod.rs`, add `mod group;` after `mod component;`.

- [ ] **Step 6: Template Steps F+G** — `cargo build`; `cargo clippy --no-deps -- -D warnings`.

- [ ] **Step 7: Template Step H** — Commit:
```bash
git add src/hsm/wrapper/group.rs src/hsm/wrapper/mod.rs
git commit -m "feat(hsm): wrapper/group.rs — impl Ochami methods for /groups

References: docs/superpowers/specs/2026-07-13-progenitor-hsm-client-design.md § Section 4 (Phase 3 detail)"
```

Time: 3-5h.

---

### Task 5: Phase 3 — wrapper/memberships.rs (2 methods)

**Files:**
- Create: `src/hsm/wrapper/memberships.rs`
- Modify: `src/hsm/wrapper/mod.rs` (add `mod memberships;`)

**Interfaces:**
- Produces: `Ochami::hsm_memberships_get(token, id, type_, state, flag, role, subrole, group, partition, arch, class) -> Result<Vec<Membership>>`, `Ochami::hsm_memberships_get_xname(token, xname) -> Result<Membership>`. Argument types: all `Option<&str>` (str inputs; coerced inline per next paragraph).

**Routing table**: both progenitor-routed.

**Special note** — `hsm_memberships_get` is the ONE method with in-body enum coercion for 7 filter fields. Per spec Section 2, method body includes:
```rust
use std::str::FromStr;
let gc = gen_client(self, token)?;
let result = gc.do_memberships_get_builder()
    .id(id.map(|s| XName100(s.to_string())))
    .type_(type_.and_then(|s| HmsType100::from_str(s).ok()))
    .state(state.and_then(|s| HmsState100::from_str(s).ok()))
    .flag(flag.and_then(|s| HmsFlag100::from_str(s).ok()))
    .role(role.map(|s| HmsRole100(s.to_string())))
    .subrole(subrole.map(|s| HmsSubRole100(s.to_string())))
    .group(group.map(|s| ResourceName(s.to_string())))
    .partition(partition.map(|s| ResourceName(s.to_string())))
    .arch(arch.and_then(|s| HmsArch100::from_str(s).ok()))
    .class(class.and_then(|s| HmsClass100::from_str(s).ok()))
    .send().await;
```
Exact newtype constructors depend on Task 0's typify emission (public inner-field vs private) — check output-reference doc; if private, use `HmsRole100::from_str(s).ok()` pattern instead of tuple construction.

- [ ] **Step 1: Template Step A** — Create file with header + imports for `Membership100Membership as Membership` + all HmsType/State/Flag/Role/SubRole/Arch/Class enum types.
- [ ] **Step 2: Template Step D** — Add both methods (both progenitor-routed, wrapped in `retry_on_5xx`).
- [ ] **Step 3: Template Step E** — Add `mod memberships;` to wrapper/mod.rs.
- [ ] **Step 4: Template Steps F+G** — cargo build + clippy.
- [ ] **Step 5: Template Step H** — Commit: `feat(hsm): wrapper/memberships.rs — impl Ochami methods for /memberships`.

Time: 1-2h.

---

### Task 6: Phase 3 — wrapper/node_map.rs (7 methods)

**Files:**
- Create: `src/hsm/wrapper/node_map.rs`
- Modify: `src/hsm/wrapper/mod.rs` (add `mod node_map;`)

**Interfaces:**
- Produces: `Ochami::hsm_node_map_get_all(token) -> Result<NodeMapArray>`, `hsm_node_map_get_one(token, xname) -> Result<NodeMap>`, `hsm_node_map_post(token, node_map: &NodeMapPost) -> Result<HsmActionResponse>`, `hsm_node_map_post_bulk(token, node_map_array: &NodeMapArray) -> Result<HsmActionResponse>`, `hsm_node_map_put(token, xname, node_map: &NodeMap) -> Result<HsmActionResponse>`, `hsm_node_map_delete_all(token) -> Result<HsmActionResponse>`, `hsm_node_map_delete_one(token, xname) -> Result<HsmActionResponse>`.

**Routing table**: all progenitor-routed. **Note**: unifies both `hsm::node_map` and `hsm::defaults::node_map` — replaces both http_client.rs files with this single wrapper (per spec § Section 1 discovery).

- [ ] **Step 1: Template Step A** — Create file with header + `NodeMap100 as NodeMap`, `NodeMapArray100 as NodeMapArray`, `NodeMap100Post as NodeMapPost` imports.
- [ ] **Step 2: Template Step D** — Add 7 methods (GETs with `retry_on_5xx`, mutating via `wrapper::run`).
- [ ] **Step 3: Template Step E** — Add `mod node_map;`.
- [ ] **Step 4: Template Steps F+G** — cargo build + clippy.
- [ ] **Step 5: Template Step H** — Commit: `feat(hsm): wrapper/node_map.rs — impl Ochami methods for /Defaults/NodeMaps`.

Time: 2-3h.

---

### Task 7: Phase 3 — wrapper/partition.rs (8 methods)

**Files:**
- Create: `src/hsm/wrapper/partition.rs`
- Modify: `src/hsm/wrapper/mod.rs` (add `mod partition;`)

**Interfaces:**
- Produces: `Ochami::hsm_partition_get_all(token) -> Result<PartitionArray>`, `hsm_partition_get(token, partition_name, tags: Option<&str>) -> Result<Partition>`, `hsm_partition_get_names(token) -> Result<Vec<String>>`, `hsm_partition_get_members(token, name) -> Result<Members>`, `hsm_partition_post(token, partition: &Partition) -> Result<HsmActionResponse>`, `hsm_partition_post_member(token, name, xname_id: &str) -> Result<HsmActionResponse>`, `hsm_partition_delete_one(token, name) -> Result<HsmActionResponse>`, `hsm_partition_delete_member(token, name, xname_id: &str) -> Result<HsmActionResponse>`.

**Routing table**: all progenitor-routed.

- [ ] **Step 1: Template Step A** — Create file with header + `Partition100 as Partition`, `PartitionArray100 as PartitionArray`, `Group100Members as Members` imports.
- [ ] **Step 2: Template Step D** — Add 8 methods.
- [ ] **Step 3: Template Step E** — Add `mod partition;`.
- [ ] **Step 4: Template Steps F+G** — cargo build + clippy.
- [ ] **Step 5: Template Step H** — Commit: `feat(hsm): wrapper/partition.rs — impl Ochami methods for /partitions`.

Time: 2-3h.

---

### Task 8: Phase 3 — wrapper/inventory_hardware.rs (4 methods; delete_all SKIPPED)

**Files:**
- Create: `src/hsm/wrapper/inventory_hardware.rs`
- Modify: `src/hsm/wrapper/mod.rs` (add `mod inventory_hardware;`)

**Interfaces:**
- Produces: `Ochami::hsm_inventory_hardware_get(token, id_vec: Option<&[&str]>, type_vec: Option<&[&str]>, state, flag, role, subrole, arch, class, format: Option<&str>) -> Result<Vec<HWInventoryByLocation>>`, `hsm_inventory_hardware_get_one(token, xname) -> Result<HWInventoryByLocation>`, `hsm_inventory_hardware_get_query(token, xname, format: Option<&str>, hierarchy: Option<bool>) -> Result<Vec<HWInventoryByLocation>>`, `hsm_inventory_hardware_post(token, hardware: &HWInventory) -> Result<HsmActionResponse>`.

**Types stay hand-rolled** per Section 3 (2767-LOC types.rs stays put). Import from `crate::hsm::inventory::hardware::types::*`, NOT generated. All methods on **raw reqwest** (Section 2 routing).

**`delete_all` SKIPPED** — the current hand-written `hsm_inventory_hardware_delete_all` is a footgun (rm -rf of hardware inventory) that no in-repo caller uses. Per spec § Section 1 (v6 fix), drop from wrapper surface. Note in commit body.

- [ ] **Step 1: Template Step A** — Create file with header, note "types hand-rolled, raw reqwest per Section 3" + imports from `crate::hsm::inventory::hardware::types::*`.
- [ ] **Step 2: Template Step C** — Add 4 raw-reqwest methods (GET with `retry_on_5xx`; POST direct).
- [ ] **Step 3: Template Step E** — Add `mod inventory_hardware;`.
- [ ] **Step 4: Template Steps F+G** — cargo build + clippy.
- [ ] **Step 5: Template Step H** — Commit: `feat(hsm): wrapper/inventory_hardware.rs — impl Ochami methods for /Inventory/Hardware (delete_all intentionally omitted)`.

Time: 2-3h.

---

### Task 9: Phase 3 — wrapper/inventory_hardware_by_fru.rs (3 methods; delete_all SKIPPED)

**Files:**
- Create: `src/hsm/wrapper/inventory_hardware_by_fru.rs`
- Modify: `src/hsm/wrapper/mod.rs` (add `mod inventory_hardware_by_fru;`)

**Interfaces:**
- Produces: `Ochami::hsm_inventory_hardware_by_fru_get(token, fru_id_vec: Option<&[&str]>, type_vec: Option<&[&str]>) -> Result<Vec<HWInventoryByFRU>>`, `hsm_inventory_hardware_by_fru_get_one(token, fru_id) -> Result<HWInventoryByFRU>`, `hsm_inventory_hardware_by_fru_delete_one(token, fru_id) -> Result<HsmActionResponse>`.

**Types hand-rolled**, **raw reqwest**. **`delete_all` SKIPPED** (same rationale as Task 8).

- [ ] **Step 1: Template Step A** — Create file, imports from `crate::hsm::inventory::hardware_by_fru::types::*`.
- [ ] **Step 2: Template Step C** — Add 3 raw-reqwest methods.
- [ ] **Step 3: Template Step E** — Add `mod inventory_hardware_by_fru;`.
- [ ] **Step 4: Template Steps F+G** — cargo build + clippy.
- [ ] **Step 5: Template Step H** — Commit: `feat(hsm): wrapper/inventory_hardware_by_fru.rs — impl Ochami methods for /Inventory/HardwareByFRU (delete_all intentionally omitted)`.

Time: 1-2h.

---

### Task 10: Phase 3 — wrapper/inventory_redfish_endpoint.rs (8 methods)

**Files:**
- Create: `src/hsm/wrapper/inventory_redfish_endpoint.rs`
- Modify: `src/hsm/wrapper/mod.rs` (add `mod inventory_redfish_endpoint;`)

**Interfaces:**
- Produces: `Ochami::hsm_inventory_redfish_endpoint_get(token, id, fqdn, type_, uuid, macaddr, ip_address, last_status: Option<&str>) -> Result<RedfishEndpointArray>`, `hsm_inventory_redfish_endpoint_get_one(token, xname) -> Result<RedfishEndpoint>`, `hsm_inventory_redfish_endpoint_post(token, ep: &RedfishEndpointCreate) -> Result<HsmActionResponse>`, `hsm_inventory_redfish_endpoint_post_query(token, xname) -> Result<HsmActionResponse>`, `hsm_inventory_redfish_endpoint_put(token, xname, ep: &RedfishEndpointPut) -> Result<HsmActionResponse>`, `hsm_inventory_redfish_endpoint_patch(token, xname, ep: &RedfishEndpointPatch) -> Result<HsmActionResponse>`, `hsm_inventory_redfish_endpoint_delete_one(token, xname) -> Result<HsmActionResponse>`.

**Routing**: **CHECK Task 0 OUTPUT-REFERENCE DOC FIRST**. If Task 0 confirmed slim-re-export is viable (typify emission matches shape needed): all progenitor-routed with slim types. Otherwise: keep types hand-rolled (import from `crate::hsm::inventory::redfish_endpoint::types::*`), raw reqwest for all methods.

- [ ] **Step 1: Consult Task 0 output-reference** — check ethernet_interfaces + redfish_endpoint slim-vs-hand-rolled decision.
- [ ] **Step 2: Template Step A** — Create file with header + imports (from `generated::types::*` if slim, else `hsm::inventory::redfish_endpoint::types::*`).
- [ ] **Step 3: Template Step D or C** — Add 7 methods matching routing decision.
- [ ] **Step 4: Template Step E** — Add `mod inventory_redfish_endpoint;`.
- [ ] **Step 5: Template Steps F+G** — cargo build + clippy.
- [ ] **Step 6: Template Step H** — Commit: `feat(hsm): wrapper/inventory_redfish_endpoint.rs — impl Ochami methods for /Inventory/RedfishEndpoints`.

Time: 3-4h.

---

### Task 11: Phase 3 — wrapper/inventory_ethernet_interfaces.rs (9 methods)

**Files:**
- Create: `src/hsm/wrapper/inventory_ethernet_interfaces.rs`
- Modify: `src/hsm/wrapper/mod.rs` (add `mod inventory_ethernet_interfaces;`)

**Interfaces:**
- Produces: `Ochami::hsm_inventory_ethernet_interface_get(token, mac_address, ip_address, network, component_id, type_: Option<&str>) -> Result<Vec<EthernetInterface>>`, `hsm_inventory_ethernet_interface_get_one(token, id) -> Result<EthernetInterface>`, `hsm_inventory_ethernet_interface_post(token, ei: &EthernetInterfaceCreate) -> Result<HsmActionResponse>`, `hsm_inventory_ethernet_interface_patch(token, id, ei: &CompEthInterface100Patch) -> Result<HsmActionResponse>`, `hsm_inventory_ethernet_interface_delete_one(token, id) -> Result<HsmActionResponse>`, `hsm_inventory_ethernet_interface_get_ip_addresses(token, id) -> Result<Vec<EthernetIPAddress>>`, `hsm_inventory_ethernet_interface_post_ip_address(token, id, ip: &EthernetIPAddress) -> Result<HsmActionResponse>`, `hsm_inventory_ethernet_interface_patch_ip_address(token, id, ip_id, patch: &EthernetIPAddressPatch) -> Result<HsmActionResponse>`, `hsm_inventory_ethernet_interface_delete_ip_address(token, id, ip_id) -> Result<HsmActionResponse>`.

**Routing**: same Task-0-conditional decision as Task 10. **PATCH endpoint** takes `CompEthInterface100Patch` (slim) or full body (hand-rolled).

**Signature note** — `hsm_inventory_ethernet_interface_delete_ip_address` drops the unused `_group_label` arg from the old signature (per spec § Latent Bugs).

- [ ] **Step 1: Consult Task 0 output-reference** — same slim-vs-hand-rolled decision as Task 10.
- [ ] **Step 2: Template Step A** — Create file with header + imports (route per Task 0 decision).
- [ ] **Step 3: Template Step D or C** — Add 9 methods matching routing decision; PATCH takes `CompEthInterface100Patch` if slim.
- [ ] **Step 4: Template Step E** — Add `mod inventory_ethernet_interfaces;`.
- [ ] **Step 5: Template Steps F+G** — cargo build + clippy.
- [ ] **Step 6: Template Step H** — Commit: `feat(hsm): wrapper/inventory_ethernet_interfaces.rs — impl Ochami methods for /Inventory/EthernetInterfaces`.

Time: 3-5h.

---

## Task 12: Phase 4 — Consumer migration

**Design spec reference:** § Section 4 → "Phase 4 (Consumer migration) detail".

**Files:**
- Modify: `src/backend_connector.rs` (~80-90 concrete edits)
- Modify: `src/hsm/group/utils.rs` (~2 signature changes + call-site updates)

**Interfaces:**
- Consumes: All `Ochami::hsm_*` methods from Tasks 3-11.
- Produces: Zero old-style `hsm::*::http_client::*` calls remain in `backend_connector.rs` or `utils.rs`.

- [ ] **Step 1: Verify starting state**

```bash
grep -rn "hsm::.*::http_client::" src/backend_connector.rs | wc -l
# Expected: ~39 (per pre-migration state)
```

- [ ] **Step 2: Migrate component-family callsites (~15-20 sites)**

For each site in `backend_connector.rs` matching `hsm::component::http_client::*` or `hsm::state::components::http_client::*`:
- Change from `hsm::component::http_client::get(&self.base_url, &self.root_cert, token, ...)` to `self.hsm_component_get(token, ...)`.
- Note arg-order variance across old signatures (some `base_url` first, some `auth_token` first — read per-callsite).
- Update field accessors on returns: `comp.id.as_ref().map(|x| x.0.as_str())` for newtype-wrapped id.
- Update return-type absorption: replace `value["field"]` with `.field` on typed structs.
- Update enum comparisons: `comp.r#type.as_deref() == Some("Node")` → `matches!(comp.type_, Some(HmsType100::Node))`.

Verify: `cargo build` green after each 3-5 sites (checkpoint the migration).

- [ ] **Step 3: Migrate group-family callsites (~15 sites in backend_connector + 2 in utils)**

Same pattern as Step 2 for `hsm::group::http_client::*`.

Also update `src/hsm/group/utils.rs`:
- Change utility function signatures to take `client: &Ochami` first arg.
- Update `hsm::group::http_client::*` calls inside to `client.hsm_group_*` methods.
- Update ~2 utility functions' downstream callers in `backend_connector.rs` (add `self` first arg).

- [ ] **Step 4: Migrate inventory-hardware callsites (~5 sites)**

Same pattern for `hsm::inventory::hardware::http_client::*`.

- [ ] **Step 5: Migrate inventory-redfish_endpoint callsites (~7 sites)**

Same pattern for `hsm::inventory::redfish_endpoint::http_client::*`.

- [ ] **Step 6: Migrate inventory-ethernet_interfaces callsites (~9 sites)**

Same pattern for `hsm::inventory::ethernet_interfaces::http_client::*`.

For the PATCH callsite (`hsm_inventory_ethernet_interface_patch`):
- If Task 11 slim-re-exported: option (a) inline `.into()` conversion won't work yet (`CompEthInterface100Patch::from` lands in Task 13). Do the manual conversion inline (~5 LOC), replaced with `.into()` after Task 13.
- If Task 11 hand-rolled: pass `ComponentEthernetInterface` directly, no conversion needed.

Also drop the unused `_group_label` arg from `hsm_inventory_ethernet_interface_delete_ip_address` call site.

- [ ] **Step 7: Verify all callsites migrated**

```bash
grep -rn "hsm::.*::http_client::" src/backend_connector.rs src/hsm/group/utils.rs
# Expected: no hits
```

- [ ] **Step 8: `cargo build`, `cargo test`, `cargo doc`, `cargo clippy`**

```bash
cargo build       # green
cargo test        # existing tests still pass (hsm/group/tests.rs uses old Group; not yet touched)
cargo doc --no-deps  # no warnings
cargo clippy --no-deps -- -D warnings
```

- [ ] **Step 9: Commit**

```bash
git add src/backend_connector.rs src/hsm/group/utils.rs
git commit -m "$(cat <<'EOF'
refactor(hsm): migrate consumers to Ochami wrapper methods

Rewrites ~80-90 callsites in backend_connector.rs and 2 in
hsm/group/utils.rs to use the new impl Ochami { hsm_* } wrapper
methods from Phase 3. No on-wire behavior change; only call-site
shape changes.

References: docs/superpowers/specs/2026-07-13-progenitor-hsm-client-design.md § Section 4 (Phase 4 detail)
EOF
)"
```

---

## Task 13: Phase 5 — Types migration + http_client deletion (atomic)

**Design spec reference:** § Section 4 → "Phase 5 (Types migration + http_client deletion) detail".

**⚠️ ATOMICITY WARNING:** This task has intermediate uncompileable states within a single commit. Only the final commit state compiles. Do NOT try to `git bisect` within this task.

**Files:**
- Modify: 7 slim-re-export types.rs files (component, state/components, group, memberships, node_map, defaults/node_map, partition).
- Modify: Task-0-conditional (per Task 1's output-reference doc): `hsm/inventory/ethernet_interfaces/types.rs`, `hsm/inventory/redfish_endpoint/types.rs`.
- Modify: `hsm/group/types.rs` (inherent Group methods rewrite).
- Delete: 11 `src/hsm/**/http_client.rs` files.
- Modify: 11 `src/hsm/**/mod.rs` files (remove `pub mod http_client;`).
- Modify: `src/backend_connector.rs` (PATCH callsite `.into()` swap — if Task 11 slim).
- Modify: `src/hsm/group/tests.rs` (update for new Group shape).

**Interfaces:**
- Produces: `hsm::component::types::Component = Component100Component`, `hsm::group::types::Group = Group100Group`, etc. (slim re-exports).

- [ ] **Step 1: (Task-0-conditional) Add `CompEthInterface100Patch` From impl**

Only if Task 0 said `inventory/ethernet_interfaces/types.rs` slim-re-exports. Otherwise skip.

In `src/hsm/inventory/ethernet_interfaces/types.rs`:
```rust
impl From<ComponentEthernetInterface> for crate::hsm::generated::types::CompEthInterface100Patch {
    fn from(value: ComponentEthernetInterface) -> Self {
        crate::hsm::generated::types::CompEthInterface100Patch {
            description: value.description,
            ip_addresses: value.ip_addresses,
            component_id: value.component_id,
        }
    }
}
```

- [ ] **Step 2: Slim-re-export the 7 non-Task-0-dependent types.rs files**

Replace each file's contents with slim re-exports (component, state/components, group, memberships, node_map, defaults/node_map, partition).

Example `src/hsm/component/types.rs`:
```rust
pub use crate::hsm::generated::types::{
    Component100Component as Component,
    Component100ComponentCreate as ComponentCreate,
    Component100Put as ComponentPut,
    ComponentArrayComponentArray as ComponentArray,
    ComponentArrayPostArray, ComponentArrayPostByNidQuery,
    ComponentArrayPostQuery, HmsArch100, HmsClass100, HmsFlag100, HmsRole100,
    HmsState100, HmsSubRole100, HmsType100, NetType100, NidRange100,
    XName100, XNameForQuery100, XNamePartition100, XNameRw100,
};
```

Repeat for the other 6 files with resource-appropriate re-exports (verify names per Task 1's output-reference doc).

- [ ] **Step 3: Rewrite ~5 From/Into impl pairs in component/types.rs + group/types.rs**

In `src/hsm/component/types.rs`, add `impl From<FrontEndComponent> for Component` etc. (per Section 3's From/Into pattern):
```rust
use manta_backend_dispatcher::types::{
    Component as FrontEndComponent,
    ComponentCreate as FrontEndComponentCreate,
    // ... etc
};
use std::str::FromStr;

impl From<FrontEndComponent> for Component {
    fn from(value: FrontEndComponent) -> Self {
        Component {
            id: value.id.map(XName100),
            type_: value.r#type.and_then(|s| HmsType100::from_str(&s).ok()),
            state: value.state.and_then(|s| HmsState100::from_str(&s).ok()),
            // ... etc
            nid: value.nid.map(|n| n as i64),
        }
    }
}

impl From<Component> for FrontEndComponent {
    fn from(value: Component) -> Self {
        FrontEndComponent {
            id: value.id.map(|x| x.0),
            r#type: value.type_.map(|t| t.to_string()),
            state: value.state.map(|s| s.to_string()),
            // ... etc
            nid: value.nid.map(|n| n as usize),
        }
    }
}
```

Also rewrite `impl Group { new, get_members, get_members_opt, add_xnames }` in `src/hsm/group/types.rs` for new field shapes:
```rust
impl Group {
    pub fn new(label: &str, member_vec_opt: Option<Vec<&str>>) -> Self {
        Self {
            label: ResourceName(label.to_string()),
            description: None,
            tags: None,
            members: member_vec_opt.map(|v| Group100Members {
                ids: Some(v.into_iter().map(|s| XNameRw100(s.to_string())).collect()),
            }),
            exclusive_group: None,
        }
    }
    // ... get_members, get_members_opt, add_xnames rewritten similarly
}
```

Exact `Group100Members` name per Task 1's output-reference doc.

- [ ] **Step 4: (Task-0-conditional) Slim-re-export ethernet_interfaces + redfish_endpoint**

Only if Task 0 said slim. Otherwise skip.

For each: replace types.rs contents with `pub use crate::hsm::generated::types::*` re-exports and add ~3 From/Into pairs per file.

- [ ] **Step 5: Delete 11 http_client.rs files + mod.rs cleanup**

```bash
rm src/hsm/component/http_client.rs
rm src/hsm/state/components/http_client.rs
rm src/hsm/group/http_client.rs
rm src/hsm/memberships/http_client.rs
rm src/hsm/node_map/http_client.rs
rm src/hsm/defaults/node_map/http_client.rs
rm src/hsm/partition/http_client.rs
rm src/hsm/inventory/hardware/http_client.rs
rm src/hsm/inventory/hardware_by_fru/http_client.rs
rm src/hsm/inventory/redfish_endpoint/http_client.rs
rm src/hsm/inventory/ethernet_interfaces/http_client.rs
```

In each of the 11 corresponding `mod.rs` files, remove the `pub mod http_client;` line.

- [ ] **Step 6: Verify `cargo build`**

```bash
cargo build
```
Expected: green. If failure, backend_connector.rs still references old-shape types — go back to Task 12 and fix missed sites.

- [ ] **Step 7: (Task-0-conditional) Swap Phase 4's PATCH inline conversion for `.into()`**

Only if Task 0 said ethernet_interfaces slim-re-exports:
- If Task 12 chose option (a) inline conversion: REPLACE the inline code (~5 LOC) with `.into()`.
- If Task 12 chose option (b) deferred: ADD the `.into()` call now.

If Task 0 said hand-rolled: SKIP — no swap needed.

- [ ] **Step 8: Update `hsm/group/tests.rs` for new Group shape**

Update `Group::new("label", Some(vec!["xname1", "xname2"]))` call site and `.add_xnames(...)` / `.get_members()` assertions to work with new field shapes (newtype accessors like `.label.0` etc.).

Verify: `cargo test hsm::group::tests` passes.

- [ ] **Step 9: Full verification**

```bash
cargo build                 # green
cargo test                  # ALL tests pass (including updated hsm/group/tests.rs)
cargo doc --no-deps         # no warnings
cargo clippy --no-deps -- -D warnings

# Grep verification:
find src/hsm -name http_client.rs        # empty output
grep -rn "pub mod http_client" src/hsm/  # empty output
```

- [ ] **Step 10: Commit (atomic)**

```bash
git add src/hsm/**/types.rs src/hsm/**/mod.rs src/backend_connector.rs \
        src/hsm/group/tests.rs
git commit -m "$(cat <<'EOF'
refactor(hsm): slim types re-exports + delete http_client (atomic)

Slim-re-exports 7 types.rs files (component, state/components, group,
memberships, node_map, defaults/node_map, partition) from generated
types; rewrites From/Into impls; adds Group inherent method bodies for
newtype fields; deletes 11 http_client.rs files.

Atomic commit — intermediate states during authoring do not compile
(types.rs slim-re-exports break old http_client.rs constructions).
Only the final state compiles.

References: docs/superpowers/specs/2026-07-13-progenitor-hsm-client-design.md § Section 4 (Phase 5 detail)
EOF
)"
```

---

## Tasks 14-22: Phase 6 — Wiremock tests (9 files)

**Design spec reference:** § Section 4 → "Phase 6 (Wiremock tests) detail".

Tasks 14-22 each create ONE test file. Pattern below in Task 14 (component); subsequent tasks follow. Time budget: 0.5-4h per file per spec.

---

### Task 14: Phase 6 — tests/common/mod.rs + tests/hsm_component.rs

**Files:**
- Create: `tests/common/mod.rs` (shared PEM fixture)
- Create: `tests/hsm_component.rs` (~10-12 wiremock tests + 2 retry-behavior tests)
- Modify: `Cargo.toml` (add `wiremock = "0.6"` to `[dev-dependencies]`)

- [ ] **Step 1: Add wiremock dev-dep**

In `Cargo.toml`:
```toml
[dev-dependencies]
wiremock = "0.6"
```

- [ ] **Step 2: Create `tests/common/mod.rs`**

```rust
//! Shared test fixtures. Copied from csm-rs's test pattern
//! (github.com/eth-cscs/csm-rs — src/hsm/wrapper/mod.rs::tests).

pub const TEST_PEM: &str = "-----BEGIN CERTIFICATE-----\n\
MIIBhTCCASugAwIBAgIQIRi6zePL6mKjOipn+dNuaTAKBggqhkjOPQQDAjASMRAw\n\
DgYDVQQKEwdBY21lIENvMB4XDTE3MTAyMDE5NDMwNloXDTE4MTAyMDE5NDMwNlow\n\
EjEQMA4GA1UEChMHQWNtZSBDbzBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABD0d\n\
7VNhbWvZLWPuj/RtHFjvtJBEwOkhbN/BnnE8rnZR8+sbwnc/KhCk3FhnpHZnQz7B\n\
5aETbbIgmuvewdjvSBSjYzBhMA4GA1UdDwEB/wQEAwICpDATBgNVHSUEDDAKBggr\n\
BgEFBQcDATAPBgNVHRMBAf8EBTADAQH/MCkGA1UdEQQiMCCCDmxvY2FsaG9zdDo1\n\
NDUzgg4xMjcuMC4wLjE6NTQ1MzAKBggqhkjOPQQDAgNIADBFAiEA2zpJEPQyz6/l\n\
Wf86aX6PepsntZv2GYlA5UpabfT2EZICICpJ5h/iI+i341gBmLiAFQOyTDT+/wQc\n\
6MF9+Yw1Yy0t\n\
-----END CERTIFICATE-----\n";
```

- [ ] **Step 3: Create `tests/hsm_component.rs` — one test per wrapper method + 2 retry + 1 non-retry**

Template (per spec Test structure):
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

#[tokio::test]
async fn hsm_component_get_one_retries_on_5xx() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/hsm/v2/State/Components/x1"))
        .respond_with(ResponseTemplate::new(503))
        .up_to_n_times(2)
        .mount(&server).await;
    Mock::given(method("GET"))
        .and(path("/hsm/v2/State/Components/x1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ID": "x1"})))
        .mount(&server).await;

    let ochami = Ochami::new(&server.uri(), TEST_PEM.as_bytes());
    let start = std::time::Instant::now();
    let comp = ochami.hsm_component_get_one("test-token", "x1").await.unwrap();
    let elapsed = start.elapsed();
    assert_eq!(comp.id.as_ref().unwrap().0, "x1");
    assert!(elapsed >= std::time::Duration::from_millis(1400));
}

// ... additional tests: one per hsm_component_* method, plus the
// 2nd retry test (`hsm_component_get_one_gives_up_after_3_5xx`).
```

If `wiremock::Mock::up_to_n_times` doesn't exist in wiremock 0.6, use priority-ordered mocks as fallback (per Task 0's Verifies list finding).

- [ ] **Step 4: Run tests**

```bash
cargo test hsm_component
# Expected: all 10-12 method tests + 2 retry tests pass (retry test wall-clock ≥1.4s)
```

- [ ] **Step 5: Commit**

```bash
git add tests/common/mod.rs tests/hsm_component.rs Cargo.toml
git commit -m "$(cat <<'EOF'
test(hsm): wiremock tests for /State/Components + retry tests

Adds ~10-12 wiremock method tests + 2 retry-behavior tests for the
component wrapper. Shared TEST_PEM fixture in tests/common/mod.rs.
Adds wiremock 0.6 as [dev-dependencies].

References: docs/superpowers/specs/2026-07-13-progenitor-hsm-client-design.md § Section 4 (Phase 6 detail)
EOF
)"
```

---

### Phase 6 Test Template (referenced by Tasks 15-22)

Every Phase 6 file follows Task 14's shape. Each task below lists specific tests and their responses; the test file skeleton is identical:

- Template Step A: `mod common; use common::TEST_PEM;` header + wiremock/serde_json imports + `ochami_rs::Ochami` + resource types.
- Template Step B: One `#[tokio::test] async fn hsm_<resource>_<method>_returns_expected(...)` per wrapper method. Each test constructs a `MockServer`, mounts a `Mock::given(method(...)).and(path(...)).respond_with(ResponseTemplate::new(200).set_body_json(json!({...})))`, constructs `Ochami::new(&server.uri(), TEST_PEM.as_bytes())`, calls the wrapper method with `"test-token"`, asserts on the returned typed struct's field values.
- Template Step C: (Task 14 only) `hsm_<resource>_get_one_retries_on_5xx` + `hsm_<resource>_get_one_gives_up_after_3_5xx` retry-behavior tests.
- Template Step D: (Task 15 only) `hsm_group_post_does_not_retry_on_5xx` — POST returns 503 once; wall-clock <500ms; asserts single upstream request only.
- Template Step E: Run `cargo test <file_name>` — all tests green.
- Template Step F: Commit with the task's message.

**Assertion patterns** — for slim-re-exported (newtype) types: `assert_eq!(x.id.as_ref().unwrap().0, "expected")`. For hand-rolled types: `assert_eq!(x.id, Some("expected".to_string()))`.

---

### Task 15: Phase 6 — tests/hsm_group.rs (9 method tests + 1 non-retry-behavior test)

**Files:**
- Create: `tests/hsm_group.rs`

**Interfaces:**
- Consumes: Tasks 4 (wrapper methods) and 14 (common::TEST_PEM fixture).
- Produces: `~10` tests exercising `Ochami::hsm_group_*` methods via wiremock.

**Tests to include** (one per wrapper method + POST-no-retry check):
- `hsm_group_get_all_returns_group_array` — GET `/hsm/v2/groups` → 200 with `[{"label": "compute"}]` → assert length + label.
- `hsm_group_get_returns_filtered` — GET `/hsm/v2/groups?group=compute&group=worker` → 200 → assert 2 items.
- `hsm_group_get_one_returns_group` — GET `/hsm/v2/groups/compute` → 200 → assert label match.
- `hsm_group_get_labels_returns_names` — GET `/hsm/v2/groups/labels` → 200 with `["compute","worker"]`.
- `hsm_group_get_members_returns_ids` — GET `/hsm/v2/groups/compute/members` → 200 with `{"ids": ["x1", "x2"]}`.
- `hsm_group_post_creates_group` — POST `/hsm/v2/groups` → 201 with `{"code": 201, "message": "created"}` → assert `HsmActionResponse.code == 201`.
- `hsm_group_post_member_adds_xname` — POST `/hsm/v2/groups/compute/members` → 200.
- `hsm_group_delete_one_removes_group` — DELETE `/hsm/v2/groups/compute` → 204.
- `hsm_group_delete_member_removes_xname` — DELETE `/hsm/v2/groups/compute/members/x1` → 204.
- `hsm_group_post_does_not_retry_on_5xx` — POST returns 503 once; wall-clock <500ms; verify single request via `.expect(1)`.

- [ ] Steps 1-6: mirror Task 14 Steps 1-4 (per Test Template). Commit: `test(hsm): wiremock tests for /groups`.

Time: 2-3h.

---

### Task 16: Phase 6 — tests/hsm_memberships.rs (2 method tests)

**Files:** Create `tests/hsm_memberships.rs`.

**Tests to include**:
- `hsm_memberships_get_returns_array` — GET `/hsm/v2/memberships?type=Node&role=Compute` → 200 with array; assert filter behavior via wiremock's `query_param` matcher.
- `hsm_memberships_get_xname_returns_one` — GET `/hsm/v2/memberships/x1c0s1b0n0` → 200 → assert xname match.

- [ ] Steps 1-4: per Test Template. Commit: `test(hsm): wiremock tests for /memberships`.

Time: 0.5-1h.

---

### Task 17: Phase 6 — tests/hsm_node_map.rs (7 method tests)

**Files:** Create `tests/hsm_node_map.rs`.

**Tests to include** (one per Task 6 method):
- `hsm_node_map_get_all_returns_array` — GET `/hsm/v2/Defaults/NodeMaps` → 200.
- `hsm_node_map_get_one_returns_map` — GET `/hsm/v2/Defaults/NodeMaps/x1c0s1b0n0` → 200.
- `hsm_node_map_post_creates_map` — POST → 201.
- `hsm_node_map_post_bulk_creates_array` — POST bulk → 201.
- `hsm_node_map_put_updates_map` — PUT → 200.
- `hsm_node_map_delete_all_removes_all` — DELETE `/hsm/v2/Defaults/NodeMaps` → 204.
- `hsm_node_map_delete_one_removes_map` — DELETE `/hsm/v2/Defaults/NodeMaps/x1c0s1b0n0` → 204.

- [ ] Steps 1-4: per Test Template. Commit: `test(hsm): wiremock tests for /Defaults/NodeMaps`.

Time: 1.5-2h.

---

### Task 18: Phase 6 — tests/hsm_partition.rs (8 method tests)

**Files:** Create `tests/hsm_partition.rs`.

**Tests to include** (one per Task 7 method):
- `hsm_partition_get_all_returns_array`, `hsm_partition_get_returns_partition`, `hsm_partition_get_names_returns_list`, `hsm_partition_get_members_returns_ids`, `hsm_partition_post_creates_partition`, `hsm_partition_post_member_adds_xname`, `hsm_partition_delete_one_removes_partition`, `hsm_partition_delete_member_removes_xname`. Bodies mirror Task 15's pattern with `/hsm/v2/partitions/*` paths.

- [ ] Steps 1-4: per Test Template. Commit: `test(hsm): wiremock tests for /partitions`.

Time: 2-3h.

---

### Task 19: Phase 6 — tests/hsm_inventory_hardware.rs (4 method tests)

**Files:** Create `tests/hsm_inventory_hardware.rs`.

**Note**: Return types are **hand-rolled** `HWInventoryByLocation` (import from `ochami_rs::hsm::inventory::hardware::types::*`), NOT slim-re-exported. Assertions use `.field == Some("val".to_string())` (hand-rolled shape), not newtype `.field.as_ref().unwrap().0`.

**Tests to include** (one per Task 8 method; delete_all is not included since intentionally omitted from wrapper):
- `hsm_inventory_hardware_get_returns_array` — GET `/hsm/v2/Inventory/Hardware?id=x1&type=Node` → 200 with array.
- `hsm_inventory_hardware_get_one_returns_hardware` — GET `/hsm/v2/Inventory/Hardware/x1c0s1b0n0` → 200.
- `hsm_inventory_hardware_get_query_returns_hierarchy` — GET `/hsm/v2/Inventory/Hardware/Query/x1c0?hierarchy=true` → 200.
- `hsm_inventory_hardware_post_creates_hardware` — POST → 201.

- [ ] Steps 1-4: per Test Template. Commit: `test(hsm): wiremock tests for /Inventory/Hardware`.

Time: 1.5-2h.

---

### Task 20: Phase 6 — tests/hsm_inventory_hardware_by_fru.rs (3 method tests)

**Files:** Create `tests/hsm_inventory_hardware_by_fru.rs`.

**Note**: Return types are **hand-rolled** `HWInventoryByFRU`. Same assertion pattern as Task 19.

**Tests to include** (one per Task 9 method):
- `hsm_inventory_hardware_by_fru_get_returns_array` — GET `/hsm/v2/Inventory/HardwareByFRU` → 200 with array.
- `hsm_inventory_hardware_by_fru_get_one_returns_fru` — GET `/hsm/v2/Inventory/HardwareByFRU/abc123` → 200.
- `hsm_inventory_hardware_by_fru_delete_one_removes_fru` — DELETE `/hsm/v2/Inventory/HardwareByFRU/abc123` → 204.

- [ ] Steps 1-4: per Test Template. Commit: `test(hsm): wiremock tests for /Inventory/HardwareByFRU`.

Time: 1-1.5h.

---

### Task 21: Phase 6 — tests/hsm_inventory_redfish_endpoint.rs (7 method tests)

**Files:** Create `tests/hsm_inventory_redfish_endpoint.rs`.

**Tests to include** (one per Task 10 method):
- `hsm_inventory_redfish_endpoint_get_returns_array` — GET `/hsm/v2/Inventory/RedfishEndpoints?fqdn=host1` → 200.
- `hsm_inventory_redfish_endpoint_get_one_returns_endpoint` — GET `/hsm/v2/Inventory/RedfishEndpoints/x1c0s1b0` → 200.
- `hsm_inventory_redfish_endpoint_post_creates_endpoint` — POST → 201.
- `hsm_inventory_redfish_endpoint_post_query_returns_result` — POST `/hsm/v2/Inventory/RedfishEndpoints/Query/x1c0s1b0` → 200.
- `hsm_inventory_redfish_endpoint_put_updates_endpoint` — PUT → 200.
- `hsm_inventory_redfish_endpoint_patch_updates_field` — PATCH → 200.
- `hsm_inventory_redfish_endpoint_delete_one_removes_endpoint` — DELETE → 204.

**Assertion note**: Choose newtype vs hand-rolled assertion pattern based on Task 0's slim-vs-hand-rolled decision from Task 10.

- [ ] Steps 1-4: per Test Template. Commit: `test(hsm): wiremock tests for /Inventory/RedfishEndpoints`.

Time: 2-3h.

---

### Task 22: Phase 6 — tests/hsm_inventory_ethernet_interfaces.rs (9 method tests)

**Files:** Create `tests/hsm_inventory_ethernet_interfaces.rs`.

**Tests to include** (one per Task 11 method):
- `hsm_inventory_ethernet_interface_get_returns_array` — GET `/hsm/v2/Inventory/EthernetInterfaces?mac_address=aa:bb:cc:dd:ee:ff` → 200.
- `hsm_inventory_ethernet_interface_get_one_returns_ei` — GET `/hsm/v2/Inventory/EthernetInterfaces/abc123` → 200.
- `hsm_inventory_ethernet_interface_post_creates_ei` — POST → 201.
- `hsm_inventory_ethernet_interface_patch_updates_ei` — PATCH `/hsm/v2/Inventory/EthernetInterfaces/abc123` → 200 with body serialized from `CompEthInterface100Patch` (slim) or hand-rolled (per Task 0).
- `hsm_inventory_ethernet_interface_delete_one_removes_ei` — DELETE → 204.
- `hsm_inventory_ethernet_interface_get_ip_addresses_returns_array` — GET `/hsm/v2/Inventory/EthernetInterfaces/abc123/IPAddresses` → 200.
- `hsm_inventory_ethernet_interface_post_ip_address_adds_ip` — POST → 201.
- `hsm_inventory_ethernet_interface_patch_ip_address_updates_ip` — PATCH `/hsm/v2/Inventory/EthernetInterfaces/abc123/IPAddresses/10.0.0.1` → 200.
- `hsm_inventory_ethernet_interface_delete_ip_address_removes_ip` — DELETE `/hsm/v2/Inventory/EthernetInterfaces/abc123/IPAddresses/10.0.0.1` → 204.

- [ ] Steps 1-4: per Test Template. Commit: `test(hsm): wiremock tests for /Inventory/EthernetInterfaces`.

Time: 2-3h.

---

## Task 23: Phase 7 — Docs + release prep

**Design spec reference:** § Section 4 → "Phase 7 (Docs + release prep) detail".

**Files:**
- Modify: `README.md` (rewrite)
- Create: `MIGRATION.md`
- Create: `CHANGELOG.md`
- Create: `CONTRIBUTING.md`
- Create: `SECURITY.md`
- Modify: `src/hsm/mod.rs` (top-level docstring)
- Modify: 10 files under `src/hsm/wrapper/` (module docstrings)
- Modify: `src/hsm/ochami_api_docs.yaml` (provenance header)
- Create: `docs/superpowers/retros/<YYYY-MM-DD>-progenitor-hsm-migration.md` (stub)
- Modify: `Cargo.toml` (add keywords + categories)

- [ ] **Step 1: Add `src/hsm/mod.rs` top-level docstring**

~40 lines explaining HSM's role + the codegen pipeline (YAML → jq patches → JSON → progenitor → wrapper). Model after csm-rs's `src/hsm/mod.rs` docstring.

- [ ] **Step 2: Add per-wrapper-file module docstrings (10 files under src/hsm/wrapper/)**

Each ~10-30 lines. For per-resource files: state which endpoints, which methods stay on raw reqwest and why. For `wrapper/mod.rs`: explain hybrid client pattern.

- [ ] **Step 3: Update `src/hsm/ochami_api_docs.yaml` provenance header**

Prepend YAML comments (~5-10 lines):
```yaml
# Upstream source: <URL from Task 1's output-reference doc>
# Retrieved: <date>
# Local patches applied via scripts/patch-hsm-spec.jq. See that file
# for per-patch rationale.
---
swagger: '2.0'
...
```

- [ ] **Step 4: Rewrite `README.md`**

New content explains `Ochami` as client entry point, shows `use ochami_rs::Ochami;` example, one code sample calling a wrapper method, points at MIGRATION.md and CHANGELOG.md.

- [ ] **Step 5: Create `MIGRATION.md`**

Structure per Section 4's MIGRATION.md outline:
1. Breaking API changes (compile-breaking items).
2. Runtime behavior changes (timeouts, retry, URL bug fixes, filter fixes, silent-deserialize fixes).
3. Latent bugs fixed (19-item list from spec's Latent Bugs section).
4. Concrete before/after examples table (from spec's table).
5. Downstream cold-build cost (~15-25s added).

- [ ] **Step 6: Create `CHANGELOG.md`**

Keep-a-Changelog format (`keepachangelog.com`). Single `[1.0.0-beta.16] - YYYY-MM-DD` entry (fill date at release) with subsections: Breaking Changes, Runtime Behavior Changes, Fixed, Added.

- [ ] **Step 7: Create `CONTRIBUTING.md`**

Requirements (rust-version, node.js, jq), local build/test, regeneration procedure, firewalled-network note, release process.

- [ ] **Step 8: Verify SECURITY.md contact email with @masber (BLOCKER)**

Ask @masber which email to publish. Do NOT commit with placeholder.

- [ ] **Step 9: Create `SECURITY.md`**

```markdown
# Security Policy

## Supported Versions

Latest published version on crates.io is supported: 1.0.0-beta.16.

## Reporting a Vulnerability

Please report security vulnerabilities privately by emailing:
<verified email from Step 8>

Include: description, reproduction steps, affected versions, suggested fix.
We aim to acknowledge within 5 business days.

## Scope

In-scope: bugs in ochami-rs, vulnerabilities in dep tree that ochami-rs exposes.
Out-of-scope: OpenCHAMI SMD server bugs (report upstream), consumer misuse.
```

- [ ] **Step 10: Create retro stub**

`docs/superpowers/retros/<YYYY-MM-DD>-progenitor-hsm-migration.md` with section headers per Section 5's retro template. Fill 1-2 weeks after release.

- [ ] **Step 11: Add Cargo.toml keywords/categories**

```toml
[package]
keywords = ["hpc", "ochami", "openchami", "hsm"]
categories = ["api-bindings", "asynchronous"]
```

- [ ] **Step 12: Final doc verification**

```bash
cargo doc --no-deps  # zero warnings
cargo doc --no-deps --open  # visual check that module docstrings render
```

- [ ] **Step 13: Commit**

```bash
git add README.md MIGRATION.md CHANGELOG.md CONTRIBUTING.md SECURITY.md \
        src/hsm/mod.rs src/hsm/wrapper/**.rs src/hsm/ochami_api_docs.yaml \
        docs/superpowers/retros/<YYYY-MM-DD>-progenitor-hsm-migration.md \
        Cargo.toml
git commit -m "$(cat <<'EOF'
docs(hsm): README rewrite + MIGRATION + CHANGELOG + module docstrings

Adds all consumer-facing documentation and release-prep artifacts for
beta.16. NO code changes (Phase 3 already added method docstrings;
Phase 7 adds file-level + top-level docs).

References: docs/superpowers/specs/2026-07-13-progenitor-hsm-client-design.md § Section 4 (Phase 7 detail)
EOF
)"
```

---

## Release Checklist (post-Task 23, before `cargo publish`)

From spec § Section 4 release checklist:
- [ ] All 23 tasks completed.
- [ ] `cargo test` passes end-to-end (~66-77 tests green).
- [ ] `cargo doc --no-deps` builds.
- [ ] `cargo publish --dry-run` succeeds; `.crate` tarball contains `src/hsm/ochami_api_docs.openapi3.json`.
- [ ] `cargo audit` clean.
- [ ] MIGRATION.md, CHANGELOG.md, README.md complete.
- [ ] Baseurl assumption verified against real OCHAMI.
- [ ] Manta companion PR drafted.
- [ ] Optional: publish `1.0.0-beta.16-rc.1` first; smoke-test with Manta; then `1.0.0-beta.16`.

## Rollback (if regression surfaces post-release)

- `cargo yank --version 1.0.0-beta.16`.
- Fix-forward with `1.0.0-beta.17`.
- No git-history rewriting.
