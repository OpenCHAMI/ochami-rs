//! Thin wrapper bridging the generated HSM client to the public
//! `Ochami` API.
//!
//! # Hybrid client pattern
//!
//! Most `impl Ochami { hsm_* }` methods route through the
//! progenitor-generated client (`crate::hsm::generated::Client`), but a
//! handful stay on raw `reqwest` where the generated client can't
//! express a needed request shape — most commonly multi-value query
//! parameters (e.g. repeated `?nid=1&nid=2`) that progenitor's typed
//! signatures only accept as a single `Option<&str>`. Each per-resource
//! file under this directory (`component.rs`, `group.rs`, `partition.rs`,
//! `memberships.rs`, `node_map.rs`, `inventory_*.rs`) documents in its
//! own module docstring exactly which of its methods take which path,
//! and why — read those for per-method routing decisions.
//!
//! # Shared helpers
//!
//! This file provides the three pieces every per-resource wrapper
//! method builds on:
//! - [`gen_client`] — constructs a fresh `generated::Client` per call,
//!   with the caller's bearer token baked into the underlying
//!   `reqwest::Client`'s `default_headers` (progenitor 0.8's direct
//!   methods build requests internally and expose no per-request
//!   header hook, so auth must be injected at client-construction
//!   time rather than per-call).
//! - [`map_err`] — converts a `progenitor_client::Error<E>` into this
//!   crate's [`crate::error::Error`], including decoding JSON error
//!   bodies from unexpected-status responses into
//!   `Error::OchamiError { status, body }`.
//! - [`run`] — glues the two together: build a client, invoke the
//!   caller-supplied progenitor operation closure, unwrap
//!   `ResponseValue<T>` into `T` on success or map the error on
//!   failure. Nearly every progenitor-routed method in this directory
//!   is a one-line call to `run(self, token, |c| async move {
//!   c.do_something(...).await })`.
//!
//! A shared connection pool across calls (instead of one `reqwest::Client`
//! per call) would require a pre_hook-capable progenitor build; deferred
//! per Section 5 follow-up work.

use std::fmt::Debug;

use crate::{error::Error, hsm::generated, Ochami};

mod component;
mod group;
mod inventory_ethernet_interfaces;
mod inventory_hardware;
mod inventory_hardware_by_fru;
mod inventory_redfish_endpoint;
mod memberships;
mod node_map;
mod partition;

// Bearer auth is baked into the underlying reqwest client's
// `default_headers` because progenitor 0.8's direct methods build the
// request internally (`self.client.get(url).build()`) and expose no
// hook for the caller to add per-request headers. Mirrors the csm-rs
// HSM wrapper's `build_client_with_auth`+`gen_client` pattern.
// A shared connection pool across calls would require a
// pre_hook-capable progenitor build (Section 5 follow-up); until
// then, each `gen_client` call constructs a fresh reqwest client
// carrying the caller's token.
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

  let baseurl = format!("{}/hsm/v2", client.base_url());
  Ok(generated::Client::new_with_client(&baseurl, inner))
}

/// Thin wrapper over [`crate::wrapper_common::map_err`] that pins the
/// service label to `"HSM"`. Kept `pub(crate)` so per-resource wrapper
/// files (`component.rs`, `group.rs`, ...) can still `use super::map_err;`
/// without needing to thread the service name through every call site.
pub(crate) async fn map_err<E: Debug + serde::Serialize>(
  err: progenitor_client::Error<E>,
) -> Error {
  crate::wrapper_common::map_err("HSM", err).await
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

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn map_err_unexpected_response_maps_to_ochami_error() {
    let response = http::Response::builder()
      .status(503)
      .body(r#"{"detail":"boom"}"#)
      .unwrap();
    let reqwest_response: reqwest::Response = response.into();
    let err: progenitor_client::Error<()> =
      progenitor_client::Error::UnexpectedResponse(reqwest_response);

    match map_err(err).await {
      Error::OchamiError { status, body } => {
        assert_eq!(status, 503);
        assert_eq!(body, serde_json::json!({"detail": "boom"}));
      }
      other => panic!("expected Error::OchamiError, got {other:?}"),
    }
  }

  #[tokio::test]
  async fn map_err_invalid_request_maps_to_message() {
    let err: progenitor_client::Error<()> =
      progenitor_client::Error::InvalidRequest("bad param".to_string());
    match map_err(err).await {
      Error::Message(msg) => assert!(msg.contains("bad param")),
      other => panic!("expected Error::Message, got {other:?}"),
    }
  }
}
