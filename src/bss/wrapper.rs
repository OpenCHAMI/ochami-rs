//! Thin wrapper bridging the progenitor-generated BSS client to the
//! public `Ochami` API. Mirrors `crate::hsm::wrapper`.
//!
//! # Design notes
//!
//! BSS's hand-rolled [`super::types::BootParameters`] is kept as the
//! wrapper's input/output type rather than switching to the generated
//! `bss::generated::types::BootParams`. Two reasons:
//!
//! 1. [`super::types::BootParameters`] carries non-trivial methods
//!    (`get_boot_image`, `update_boot_image`, `update_kernel_param`,
//!    `new`) that we want to keep working without porting.
//! 2. Its field shape matches `manta_backend_dispatcher::types::bss::
//!    BootParameters` exactly (`Option<Vec<u32>>` nids,
//!    non-`Option<String>` params/kernel/initrd), so the trait-impl
//!    layer in `backend_connector.rs` can pass it through without
//!    conversion.
//!
//! An internal [`super::types::BootParameters`] ↔ generated
//! `BootParams` conversion (see [`super::types`] for the impls) sits
//! at the boundary between the wrapper method body and the generated
//! client call. Widening `nids` from `u32` to `i64` happens in the
//! From impl; the values in practice fit both.
//!
//! # gen_client
//!
//! Follows the same pattern as `hsm::wrapper::gen_client`: bakes the
//! bearer token into `default_headers` on a fresh `reqwest::Client`
//! (progenitor 0.8 exposes no per-request header hook), applies the
//! crate-wide timeout, and hangs the generated `Client` off the base
//! URL suffixed with `/boot/v1` (matching every path in the BSS
//! OpenAPI spec).

use std::fmt::Debug;

use crate::{
  bss::{generated, types::BootParameters},
  error::Error,
  http::retry_on_5xx,
  Ochami,
};

pub(crate) fn gen_client(
  client: &Ochami,
  token: &str,
) -> Result<generated::Client, Error> {
  let mut headers = reqwest::header::HeaderMap::new();
  let auth = format!("Bearer {token}");
  let mut value = reqwest::header::HeaderValue::from_str(&auth)
    .map_err(|e| Error::Message(format!("invalid bearer token: {e}")))?;
  value.set_sensitive(true);
  headers.insert(reqwest::header::AUTHORIZATION, value);

  let inner = reqwest::Client::builder()
    .add_root_certificate(reqwest::Certificate::from_pem(client.root_cert())?)
    .use_rustls_tls()
    .connect_timeout(crate::http::HTTP_CONNECT_TIMEOUT)
    .timeout(crate::http::HTTP_REQUEST_TIMEOUT)
    .default_headers(headers)
    .build()
    .map_err(Error::NetError)?;

  // The BSS spec's paths all live under `/boot/v1/*`, so the generated
  // Client only needs the base URL. Every operation prefixes the path
  // in its own `format!("{}/boot/v1/...", self.baseurl)`.
  Ok(generated::Client::new_with_client(client.base_url(), inner))
}

/// Service-scoped [`crate::wrapper_common::map_err`] wrapper.
pub(crate) async fn map_err<E: Debug + serde::Serialize>(
  err: progenitor_client::Error<E>,
) -> Error {
  crate::wrapper_common::map_err("BSS", err).await
}

/// Construct an empty `BootParams` payload. Typify doesn't emit
/// `#[derive(Default)]` on generated structs (all fields have `#[serde(
/// default)]` for on-wire semantics, but no Default impl in Rust), so
/// callers who want an empty body have to build one explicitly.
fn empty_boot_params() -> generated::types::BootParams {
  generated::types::BootParams {
    cloud_init: None,
    hosts: Vec::new(),
    initrd: None,
    kernel: None,
    macs: Vec::new(),
    nids: Vec::new(),
    params: None,
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
  E: Debug + serde::Serialize,
{
  let gc = gen_client(client, token)?;
  match op(gc).await {
    Ok(rv) => Ok(rv.into_inner()),
    Err(e) => Err(map_err(e).await),
  }
}

impl Ochami {
  /// Fetch every set of boot parameters known to BSS.
  ///
  /// # Errors
  ///
  /// See [`Ochami::bss_bootparameters_get`].
  pub async fn bss_bootparameters_get_all(
    &self,
    token: &str,
  ) -> Result<Vec<BootParameters>, Error> {
    self.bss_bootparameters_get(token, None).await
  }

  /// Fetch boot parameters, optionally filtered to a set of hosts (xnames).
  ///
  /// The old hand-written client filtered via a request body containing
  /// `{"hosts": [...]}` on `GET /boot/v1/bootparameters`. Progenitor
  /// preserves that shape — the generated `bootparameters_get` accepts
  /// both `mac`/`name`/`nid` query params AND a body. This method
  /// keeps the body-filter behaviour so the on-wire shape matches
  /// pre-migration behavior for existing consumers.
  ///
  /// # Errors
  ///
  /// Returns [`Error`] on transport, BSS 4xx/5xx, or deserialization
  /// failure. GETs are retried up to 3 times on 5xx with exponential
  /// backoff.
  pub async fn bss_bootparameters_get(
    &self,
    token: &str,
    xnames_opt: Option<&[&str]>,
  ) -> Result<Vec<BootParameters>, Error> {
    let filter = xnames_opt.map(|xnames| BootParameters {
      hosts: xnames.iter().map(|s| (*s).to_string()).collect(),
      ..Default::default()
    });
    retry_on_5xx(|| async {
      let gc = gen_client(self, token)?;
      let body: generated::types::BootParams = filter
        .clone()
        .map(Into::into)
        .unwrap_or_else(empty_boot_params);
      match gc.bootparameters_get(None, None, None, &body).await {
        Ok(rv) => Ok(
          rv.into_inner()
            .into_iter()
            .map(BootParameters::from)
            .collect(),
        ),
        Err(e) => Err(map_err(e).await),
      }
    })
    .await
  }

  /// Create boot parameters (POST). Returns `()`; the generated
  /// endpoint has no response body on 201.
  ///
  /// # Errors
  ///
  /// Returns [`Error`] on transport, BSS 4xx/5xx, or (for typed BSS
  /// error responses) `Error::OchamiError { status, body }`.
  pub async fn bss_bootparameters_post(
    &self,
    token: &str,
    boot_parameters: &BootParameters,
  ) -> Result<(), Error> {
    let body: generated::types::BootParams = boot_parameters.clone().into();
    run(self, token, |gc| async move {
      gc.bootparameters_post(&body).await
    })
    .await
  }

  /// Set (PUT) boot parameters. Returns `()`; the generated endpoint
  /// has no response body on 200. NOTE: this is a behaviour change
  /// from the hand-written client, which returned the round-tripped
  /// `BootParameters` after re-fetching. Existing consumers that
  /// relied on the response value must re-fetch via
  /// `bss_bootparameters_get` if they need the server-canonical form.
  ///
  /// # Errors
  ///
  /// See [`Ochami::bss_bootparameters_post`].
  pub async fn bss_bootparameters_put(
    &self,
    token: &str,
    boot_parameters: &BootParameters,
  ) -> Result<(), Error> {
    let body: generated::types::BootParams = boot_parameters.clone().into();
    run(self, token, |gc| async move {
      gc.bootparameters_put(&body).await
    })
    .await
  }

  /// Update (PATCH) boot parameters. Returns `()`; the generated
  /// endpoint has no response body on 200.
  ///
  /// # Errors
  ///
  /// See [`Ochami::bss_bootparameters_post`].
  pub async fn bss_bootparameters_patch(
    &self,
    token: &str,
    boot_parameters: &BootParameters,
  ) -> Result<(), Error> {
    let body: generated::types::BootParams = boot_parameters.clone().into();
    run(self, token, |gc| async move {
      gc.bootparameters_patch(&body).await
    })
    .await
  }

  /// Delete boot parameters. Returns `()`; the generated endpoint has
  /// no response body on 200. NOTE: the hand-written client returned
  /// `String` (raw response text); consumers that used it for logging
  /// only can drop the return value.
  ///
  /// # Errors
  ///
  /// See [`Ochami::bss_bootparameters_post`].
  pub async fn bss_bootparameters_delete(
    &self,
    token: &str,
    boot_parameters: &BootParameters,
  ) -> Result<(), Error> {
    let body: generated::types::BootParams = boot_parameters.clone().into();
    run(self, token, |gc| async move {
      gc.bootparameters_delete(&body).await
    })
    .await
  }
}
