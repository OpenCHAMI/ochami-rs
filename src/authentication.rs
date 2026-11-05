use std::env::VarError;
use std::time::Duration;

use serde_json::Value;

use crate::error::Error;

/// Reads the OCHAMI bearer token from the `ACCESS_TOKEN` environment variable.
pub fn get_api_token() -> Result<String, VarError> {
  std::env::var("ACCESS_TOKEN")
}

/// Deadline for one token exchange. Every request waits on it, so it is
/// much shorter than the OCHAMI APIs' `HTTP_REQUEST_TIMEOUT`.
const TOKEN_EXCHANGE_TIMEOUT: Duration = Duration::from_secs(30);

/// Client credentials for exchanging a token from an external identity
/// provider (e.g. the CSCS Keycloak) for an OCHAMI token at the site's
/// Hydra, using the JWT bearer grant (RFC 7523).
#[derive(Clone)]
pub struct TokenExchange {
  /// Hydra client allowed to use the JWT bearer grant.
  pub client_id: String,
  /// Secret of `client_id`.
  pub client_secret: String,
  /// Space-separated scopes to request, e.g. `smd.read bss.read`.
  pub scope: String,
}

impl std::fmt::Debug for TokenExchange {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("TokenExchange")
      .field("client_id", &self.client_id)
      .field("client_secret", &"<redacted>")
      .field("scope", &self.scope)
      .finish()
  }
}

/// Exchange `subject_token` for an OCHAMI access token at
/// `{base_url}/hydra/oauth2/token`.
///
/// Hydra accepts an assertion's `jti` only once until the assertion
/// expires, so a second exchange of the same token fails with
/// [`Error::Unauthorized`]. Callers that need the result more than once
/// must keep it.
///
/// # Errors
///
/// - [`Error::Unauthorized`] when Hydra rejects the subject token
///   (`invalid_grant`: expired, bad signature, no trust relationship;
///   `jti_known`: already exchanged).
/// - [`Error::Forbidden`] when Hydra answers 403.
/// - [`Error::Message`] for any other 4xx, including a wrong client
///   secret (`invalid_client`) or a scope the client may not request
///   (`invalid_scope`), or a success body without `access_token`.
/// - [`Error::OchamiError`] for a 5xx and [`Error::NetError`] when Hydra
///   is unreachable or times out.
pub async fn exchange_token(
  client: &reqwest::Client,
  base_url: &str,
  exchange: &TokenExchange,
  subject_token: &str,
) -> Result<String, Error> {
  let api_url =
    format!("{}/hydra/oauth2/token", base_url.trim_end_matches('/'));

  log::debug!("Exchange token at {api_url}");

  let params = [
    ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
    ("assertion", subject_token),
    ("scope", exchange.scope.as_str()),
  ];

  let resp = client
    .post(&api_url)
    .basic_auth(&exchange.client_id, Some(&exchange.client_secret))
    .form(&params)
    .timeout(TOKEN_EXCHANGE_TIMEOUT)
    .send()
    .await?;

  let status = resp.status();
  if status.is_success() {
    return resp
      .json::<Value>()
      .await?
      .get("access_token")
      .and_then(Value::as_str)
      .map(str::to_string)
      .ok_or_else(|| {
        Error::Message(
          "Hydra token response is missing 'access_token'".to_string(),
        )
      });
  }

  let payload = resp.text().await.unwrap_or_default();
  let body: Value =
    serde_json::from_str(&payload).unwrap_or(Value::String(payload.clone()));
  if status.is_server_error() {
    return Err(Error::OchamiError {
      status: status.as_u16(),
      body,
    });
  }

  // OAuth error body: {"error": "...", "error_description": "..."}.
  let error = body.get("error").and_then(Value::as_str).unwrap_or("");
  let description = body
    .get("error_description")
    .and_then(Value::as_str)
    .unwrap_or(&payload);
  let detail = format!(
    "token exchange at {api_url} failed: status={status} error={error} {description}"
  );

  Err(match (status.as_u16(), error) {
    (_, "invalid_grant" | "jti_known") => Error::Unauthorized(detail),
    (403, _) => Error::Forbidden(detail),
    _ => Error::Message(detail),
  })
}
