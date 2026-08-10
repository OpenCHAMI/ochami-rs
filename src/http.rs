use std::time::Duration;

use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::error::Error;

pub(crate) const HTTP_CONNECT_TIMEOUT: Duration = Duration::from_mins(45);
pub(crate) const HTTP_REQUEST_TIMEOUT: Duration = Duration::from_mins(15);
pub(crate) const HTTP_5XX_RETRY_ATTEMPTS: u32 = 3;
pub(crate) const HTTP_5XX_RETRY_INITIAL_DELAY: Duration =
  Duration::from_millis(500);

/// Builds a `reqwest::Client` trusting `root_cert` (PEM), using rustls and the crate's timeouts.
pub fn build_client(root_cert: &[u8]) -> Result<reqwest::Client, Error> {
  let builder = reqwest::Client::builder()
    .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?)
    .use_rustls_tls()
    .connect_timeout(HTTP_CONNECT_TIMEOUT)
    .timeout(HTTP_REQUEST_TIMEOUT);
  builder.build().map_err(Error::NetError)
}

/// Builds a `reqwest::Client` trusting `root_cert` (PEM), without forcing rustls or timeouts.
pub fn build_client_no_tls(root_cert: &[u8]) -> Result<reqwest::Client, Error> {
  let builder = reqwest::Client::builder()
    .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?);
  builder.build().map_err(Error::NetError)
}

/// Deserializes a successful JSON response body into `T`, mapping non-2xx
/// responses to `Error::OchamiError` and body-decode failures to
/// `Error::NetError`.
pub(crate) async fn handle_json_response<T: DeserializeOwned>(
  response: reqwest::Response,
) -> Result<T, Error> {
  let status = response.status();
  if !status.is_success() {
    let payload = response.json::<Value>().await.unwrap_or(Value::Null);
    return Err(Error::OchamiError {
      status: status.as_u16(),
      body: payload,
    });
  }
  response.json::<T>().await.map_err(Error::NetError)
}

/// Deserializes an HSM mutating-endpoint acknowledgement body
/// (`crate::hsm::types::HsmActionResponse`).
pub(crate) async fn handle_action_response(
  response: reqwest::Response,
) -> Result<crate::hsm::types::HsmActionResponse, Error> {
  handle_json_response(response).await
}

/// Retries `op` up to `HTTP_5XX_RETRY_ATTEMPTS` times, with exponential
/// backoff starting at `HTTP_5XX_RETRY_INITIAL_DELAY`, whenever `op`
/// fails with an `Error::OchamiError` carrying a 5xx status.
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
  Err(
    last_err
      .unwrap_or_else(|| Error::Message("retry logic invariant broken".into())),
  )
}

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
        Err(Error::OchamiError {
          status: 503,
          body: serde_json::Value::Null,
        })
      } else {
        Ok(42)
      }
    })
    .await;
    assert_eq!(result.unwrap(), 42);
    assert_eq!(counter.load(Ordering::SeqCst), 3);
  }

  #[tokio::test]
  async fn retry_on_5xx_does_not_retry_non_5xx() {
    use std::sync::atomic::{AtomicU32, Ordering};
    let counter = AtomicU32::new(0);
    let result: Result<u32, Error> = retry_on_5xx(|| async {
      counter.fetch_add(1, Ordering::SeqCst);
      Err(Error::OchamiError {
        status: 404,
        body: serde_json::Value::Null,
      })
    })
    .await;
    assert!(result.is_err());
    assert_eq!(counter.load(Ordering::SeqCst), 1);
  }
}
