//! Shared helpers for progenitor-generated wrapper layers (HSM, BSS, ...).
//!
//! Each service under `src/` that wraps a progenitor-generated client has
//! its own module — `hsm::wrapper`, `bss::wrapper`, etc. — because the
//! `generated::Client` types are per-service. What they all share is
//! [`map_err`], which converts a `progenitor_client::Error<E>` into this
//! crate's [`crate::error::Error`] uniformly, and this file is that
//! shared piece.
//!
//! Each service's `wrapper::mod.rs` retains its own `gen_client` (bakes
//! bearer auth + service baseurl) and a small `run` helper that stitches
//! `gen_client` + `map_err` together.

use std::fmt::Debug;

use crate::error::Error;

/// Convert a `progenitor_client::Error<E>` into this crate's
/// [`Error`]. `service` names the wrapper (`"HSM"`, `"BSS"`, ...) for
/// diagnostic messages so operators can tell which service surfaced a
/// low-level failure from the log line alone.
///
/// Typed error bodies (`ErrorResponse`) and untyped error bodies
/// (`UnexpectedResponse`) both round-trip through JSON into
/// [`Error::OchamiError { status, body }`] so downstream
/// `retry_on_5xx` and status-code-branching consumers can inspect the
/// HTTP status without parsing a message string.
pub(crate) async fn map_err<E: Debug + serde::Serialize>(
  service: &'static str,
  err: progenitor_client::Error<E>,
) -> Error {
  use progenitor_client::Error::*;
  match err {
    InvalidRequest(s) => {
      Error::Message(format!("{service} invalid request: {s}"))
    }
    CommunicationError(e) | InvalidUpgrade(e) => Error::NetError(e),
    // Emit structured Error::OchamiError so retry_on_5xx (which only
    // matches OchamiError with a 5xx status) can fire on typed error
    // responses too. Fall back to the Debug-formatted string if
    // serializing the typed body fails (unlikely — progenitor-emitted
    // error types derive Serialize).
    ErrorResponse(rv) => {
      let status = rv.status().as_u16();
      let inner = rv.into_inner();
      match serde_json::to_value(&inner) {
        Ok(body) => Error::OchamiError { status, body },
        Err(_) => Error::Message(format!(
          "{service} error response: status={status} body={inner:?}"
        )),
      }
    }
    ResponseBodyError(e) => Error::NetError(e),
    InvalidResponsePayload(_, e) => Error::SerdeError(e),
    UnexpectedResponse(resp) => {
      let status = resp.status().as_u16();
      let bytes = resp.bytes().await.unwrap_or_default();
      match serde_json::from_slice::<serde_json::Value>(&bytes) {
        Ok(body) => Error::OchamiError { status, body },
        Err(_) => Error::Message(format!(
          "{service} unexpected: status={status} body={}",
          String::from_utf8_lossy(&bytes)
        )),
      }
    }
    PreHookError(s) => Error::Message(format!("{service} pre-hook error: {s}")),
  }
}
