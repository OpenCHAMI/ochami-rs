//! Thin wrapper bridging the progenitor-generated PCS client to the
//! public `Ochami` API. Mirrors `crate::hsm::wrapper` and
//! `crate::bss::wrapper`.
//!
//! # Scope
//!
//! Only `pcs_power_status_post` is provided at present — the sole PCS
//! path exercised by the crate's own trait implementation
//! (`impl PCSTrait for Ochami`). Wrappers for the `power_cap` and
//! `transitions` endpoints (which had no in-crate consumers under the
//! hand-written surface either) are deferred to a follow-up commit.
//!
//! # Raw-reqwest carve-out for `power_status_post`
//!
//! `power_status_post` uses raw reqwest rather than the generated
//! `Client::power_status_post` method. Reason: the PCS OpenAPI spec's
//! `xname` field on the request body references the schema
//! `non_empty_string_list` (a `type: array` of strings). Progenitor
//! 0.8 emits `Vec<String>`-newtypes with no `Display` impl, which
//! trips progenitor's own query-string builder (`.to_string()` on the
//! newtype). Our `scripts/patch-pcs-spec.jq` patch 2 downgrades that
//! schema to `type: string` at spec-conversion time to make progenitor
//! codegen succeed; the price is that the generated `PowerStatusGet`
//! body type carries `Option<NonEmptyStringList>` (a `String`
//! newtype) which cannot express the multi-value array the server
//! actually accepts. Raw reqwest sends the correct on-wire shape
//! directly.
//!
//! Every other planned wrapper method can use the generated client
//! normally — this carve-out is a targeted, single-endpoint workaround.
//!
//! # gen_client
//!
//! Baseurl is `{ochami base_url}/power-control/v1` — matches the
//! path prefix the hand-written `pcs::power_status::http_client::post`
//! used.

use std::fmt::Debug;

use crate::{
  error::Error,
  http::build_client,
  pcs::{generated, power_status::types::PowerStatusAll},
  Ochami,
};

#[allow(dead_code)]
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

  let baseurl = format!("{}/power-control/v1", client.base_url());
  Ok(generated::Client::new_with_client(&baseurl, inner))
}

/// Service-scoped [`crate::wrapper_common::map_err`] wrapper. Currently
/// unused (only the raw-reqwest `power_status_post` ships in this
/// commit) but retained so follow-up wrapper methods for `power_cap`
/// and `transitions` can drop straight in.
#[allow(dead_code)]
pub(crate) async fn map_err<E: Debug + serde::Serialize>(
  err: progenitor_client::Error<E>,
) -> Error {
  crate::wrapper_common::map_err("PCS", err).await
}

impl Ochami {
  /// Retrieve power state for a set of xnames.
  ///
  /// Sends `POST /power-control/v1/power-status` with a JSON body of
  /// `{"xname": [...], "powerStateFilter": "...", "managementStateFilter": "..."}`.
  ///
  /// The pre-migration hand-written client used
  /// `crate::http::build_client_no_tls` (no timeouts). This wrapper
  /// uses `crate::http::build_client` (45 min connect / 15 min request
  /// timeouts) and surfaces PCS 4xx/5xx as
  /// [`Error::OchamiError { status, body }`] instead of the previous
  /// mix of `Error::NetError` / `Error::OchamiError`.
  ///
  /// # Errors
  ///
  /// Returns [`Error`] on transport, PCS 4xx/5xx (as
  /// `Error::OchamiError { status, body }` when body parses as JSON),
  /// or deserialization failure. POSTs do NOT retry on 5xx.
  pub async fn pcs_power_status_post(
    &self,
    token: &str,
    xname_vec_opt: Option<&[&str]>,
    power_state_filter_opt: Option<&str>,
    management_state_filter_opt: Option<&str>,
  ) -> Result<PowerStatusAll, Error> {
    let http = build_client(self.root_cert())?;
    let url = format!("{}/power-control/v1/power-status", self.base_url());
    let body = serde_json::json!({
      "xname": xname_vec_opt
        .map(|xs| xs.iter().map(|x| x.to_string()).collect::<Vec<String>>())
        .unwrap_or_default(),
      "powerStateFilter": power_state_filter_opt.unwrap_or(""),
      "managementStateFilter": management_state_filter_opt.unwrap_or(""),
    });
    let response = http
      .post(&url)
      .bearer_auth(token)
      .json(&body)
      .send()
      .await
      .map_err(Error::NetError)?;
    let status = response.status();
    if status.is_success() {
      // Prefer the generated types::PowerStatusAll for the wire read
      // (matches the spec more strictly than the hand-rolled type),
      // then convert to the hand-rolled trait-boundary type.
      let generated_all: generated::types::PowerStatusAll =
        response.json().await.map_err(Error::NetError)?;
      Ok(generated_all.into())
    } else {
      let body = response
        .json::<serde_json::Value>()
        .await
        .unwrap_or(serde_json::Value::Null);
      Err(Error::OchamiError {
        status: status.as_u16(),
        body,
      })
    }
  }
}

// Bridge: generated `PowerStatusAll` -> hand-rolled `PowerStatusAll`.
// Kept here rather than in `power_status/types.rs` so that hand-rolled
// module stays free of `crate::pcs::generated` imports.

impl From<generated::types::PowerStatusAll> for PowerStatusAll {
  fn from(value: generated::types::PowerStatusAll) -> Self {
    PowerStatusAll {
      status: value.status.into_iter().map(Into::into).collect(),
    }
  }
}

impl From<generated::types::PowerStatus>
  for crate::pcs::power_status::types::PowerStatus
{
  fn from(value: generated::types::PowerStatus) -> Self {
    crate::pcs::power_status::types::PowerStatus {
      xname: value
        .xname
        .map(|x| (*x).clone())
        .unwrap_or_default(),
      power_state: value.power_state.map(|ps| match ps {
        generated::types::PowerState::On => {
          crate::pcs::power_status::types::PowerState::On
        }
        generated::types::PowerState::Off => {
          crate::pcs::power_status::types::PowerState::Off
        }
        generated::types::PowerState::Undefined => {
          crate::pcs::power_status::types::PowerState::Undefined
        }
      }),
      management_state: value.management_state.map(|ms| match ms {
        generated::types::ManagementState::Unavailable => {
          crate::pcs::power_status::types::ManagementState::Unavailable
        }
        generated::types::ManagementState::Available => {
          crate::pcs::power_status::types::ManagementState::Available
        }
      }),
      error: value.error,
      supported_power_transitions: value
        .supported_power_transitions
        .into_iter()
        .filter_map(|op| {
          crate::pcs::transitions::types::Operation::from_str(
            &op.to_string().to_lowercase(),
          )
          .ok()
          .or_else(|| {
            log::warn!(
              "dropped unparseable pcs supported_power_transitions entry: {op:?}"
            );
            None
          })
        })
        .collect(),
      last_updated: value
        .last_updated
        .map(|ts| ts.to_rfc3339())
        .unwrap_or_default(),
    }
  }
}
