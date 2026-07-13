//! Thin wrapper bridging the generated HSM client to the public
//! `Ochami` API. Uses per-call reqwest client construction (matches
//! csm-rs). Shared pool via progenitor pre_hook is deferred (per
//! Section 5 follow-up).

use std::fmt::Debug;

use crate::{error::Error, hsm::generated, Ochami};

mod component;

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

pub(crate) async fn map_err<E: Debug>(
  err: progenitor_client::Error<E>,
) -> Error {
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
