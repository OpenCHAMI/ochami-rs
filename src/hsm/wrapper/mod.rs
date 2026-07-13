//! Thin wrapper bridging the generated HSM client to the public
//! `Ochami` API. Uses per-call reqwest client construction (matches
//! csm-rs). Shared pool via progenitor pre_hook is deferred (per
//! Section 5 follow-up).

use std::fmt::Debug;

use crate::{error::Error, hsm::generated, Ochami};

// NOTE: `_token` is unused today — bearer auth is applied per-request (see
// below) rather than baked into the generated client. Kept in the
// signature so Phase 3 call sites are uniform (`gen_client(client, token)`)
// and so a future shared-pool/pre_hook refactor (Section 5 follow-up) can
// wire it in without changing callers.
pub(crate) fn gen_client(
  client: &Ochami,
  _token: &str,
) -> Result<generated::Client, Error> {
  let inner = crate::http::build_client(client.root_cert())?;
  let baseurl = format!("{}/hsm/v2", client.base_url());
  // NOTE: bearer auth is applied per-request via reqwest's `.bearer_auth()`
  // in each wrapper method body since reqwest doesn't allow inserting
  // default headers post-build.
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
