use serde_json::Value;

/// Crate-wide error type returned by all fallible OCHAMI operations.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
  /// Free-form error message from this crate.
  #[error("OCHAMI-RS: {0}")]
  Message(String),
  /// JSON (de)serialisation failure.
  #[error("OCHAMI-RS > Serde: {0}")]
  SerdeError(#[from] serde_json::Error),
  /// Underlying `reqwest` HTTP/network error.
  #[error("OCHAMI-RS > Net: {0}")]
  NetError(#[from] reqwest::Error),
  /// HTTP request failure paired with the raw response body captured at the call site.
  #[error(
    "OCHAMI-RS: http request:\nresponse: {response}\npayload: {payload}"
  )]
  RequestError {
    /// The underlying `reqwest` error returned when reading the response.
    response: reqwest::Error,
    /// Raw response body (plain text or JSON string) captured for context.
    payload: String, // NOTE: CSM/OCHAMI Apis either returns plain text or a json therefore, we
                     // will just return a String
  },
  /// OCHAMI service returned a non-2xx HTTP response.
  #[error("OCHAMI-RS > OCHAMI: status={status} body={body}")]
  #[non_exhaustive]
  OchamiError {
    /// HTTP status code returned by the OCHAMI service.
    status: u16,
    /// Response body decoded as JSON (or `Value::Null` if not JSON).
    body: Value,
  },
}
