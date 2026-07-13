//! Conversion from [`crate::error::Error`] into
//! [`manta_backend_dispatcher::error::Error`] so trait-impl callsites can
//! propagate crate-level errors with `?` instead of stringifying via
//! `.map_err(|e| Error::Message(e.to_string()))`.
//!
//! The critical mapping is `Error::OchamiError { status, body }` →
//! `MantaError::CsmError { status, detail, body }`, which preserves the
//! HTTP status code and JSON body so downstream consumers can branch on
//! status instead of parsing a flat message string.

use manta_backend_dispatcher::error::Error as MantaError;
use serde_json::Value;

use crate::error::Error;

impl From<Error> for MantaError {
  fn from(err: Error) -> Self {
    match err {
      Error::Message(s) => MantaError::Message(s),
      Error::SerdeError(e) => MantaError::SerdeError(e),
      Error::NetError(e) => MantaError::NetError(e),
      Error::RequestError { response, payload } => {
        MantaError::RequestError { response, payload }
      }
      Error::OchamiError { status, body } => {
        let detail = extract_rfc7807_detail(&body).unwrap_or_else(|| {
          format!("HSM/OCHAMI error status={status}")
        });
        MantaError::CsmError {
          status,
          detail,
          body: Some(body),
        }
      }
    }
  }
}

/// Extract a human-readable detail string from an RFC 7807 Problem
/// Details body, or from other common error-body shapes. Falls back to
/// `None` if no known field is present so the caller can synthesize a
/// generic message from the HTTP status.
fn extract_rfc7807_detail(body: &Value) -> Option<String> {
  body
    .get("detail")
    .and_then(Value::as_str)
    .or_else(|| body.get("title").and_then(Value::as_str))
    .or_else(|| body.get("message").and_then(Value::as_str))
    .or_else(|| body.get("error").and_then(Value::as_str))
    .map(String::from)
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::json;

  #[test]
  fn ochami_error_maps_to_csm_error_with_status_and_body() {
    let err = Error::OchamiError {
      status: 409,
      body: json!({"detail": "conflict: group exists"}),
    };
    let manta: MantaError = err.into();
    match manta {
      MantaError::CsmError { status, detail, body } => {
        assert_eq!(status, 409);
        assert_eq!(detail, "conflict: group exists");
        assert!(body.is_some());
      }
      other => panic!("expected CsmError, got {other:?}"),
    }
  }

  #[test]
  fn ochami_error_extracts_title_when_no_detail() {
    let err = Error::OchamiError {
      status: 500,
      body: json!({"title": "internal server error"}),
    };
    let manta: MantaError = err.into();
    match manta {
      MantaError::CsmError { detail, .. } => {
        assert_eq!(detail, "internal server error");
      }
      other => panic!("expected CsmError, got {other:?}"),
    }
  }

  #[test]
  fn ochami_error_synthesizes_detail_when_body_has_no_known_fields() {
    let err = Error::OchamiError {
      status: 502,
      body: json!({"whatever": "some value"}),
    };
    let manta: MantaError = err.into();
    match manta {
      MantaError::CsmError { detail, .. } => {
        assert_eq!(detail, "HSM/OCHAMI error status=502");
      }
      other => panic!("expected CsmError, got {other:?}"),
    }
  }

  #[test]
  fn message_variant_passes_through() {
    let err = Error::Message("boom".into());
    let manta: MantaError = err.into();
    match manta {
      MantaError::Message(s) => assert_eq!(s, "boom"),
      other => panic!("expected Message, got {other:?}"),
    }
  }
}
