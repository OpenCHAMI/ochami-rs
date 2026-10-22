//! Thin wrapper bridging the progenitor-generated PCS client to the
//! public `Ochami` API. Mirrors `crate::hsm::wrapper` and
//! `crate::bss::wrapper`.
//!
//! # Surface
//!
//! - `pcs_power_status_{get, post}` (raw reqwest — see note below)
//! - `pcs_power_cap_get / _task_get / _snapshot_post / _patch`
//! - `pcs_transitions_get / _post`, `pcs_transition_get / _delete`,
//!   `pcs_transition_post_block`, `pcs_transition_wait_to_complete`
//!
//! Every method uses `crate::http::build_client` (45 min connect / 15
//! min request timeouts) and surfaces PCS 4xx/5xx as
//! [`Error::OchamiError { status, body }`] uniformly.
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
//! `power_cap` and `transitions` methods also use raw reqwest — the
//! trait-boundary types (`PowerCapTaskInfo`, `TransitionResponse`, ...)
//! are hand-rolled in `pcs/*/types.rs`; wrapping the generated client
//! would require an extra layer of type-bridge From impls with no
//! practical benefit. If typed request/response bodies from the
//! generated spec become important later, migrate individual methods
//! by swapping the raw block for `gen_client(...)` + progenitor call.
//!
//! # gen_client
//!
//! Baseurl is `{ochami base_url}/power-control/v1`.
//!
//! # Silent bug fixes vs pre-migration
//!
//! The hand-written `power_cap::http_client::post_snapshot` and
//! `patch` methods both used `.put()` on `/power-cap/snapshot`. Per
//! the OpenAPI spec, `POST /power-cap/snapshot` and `PATCH /power-cap`
//! are the correct verb/path pairs. Both are corrected in this
//! wrapper. Documented in MIGRATION.md.

use std::fmt::Debug;
use std::time::Duration;

use crate::{
  error::Error,
  http::{build_client, retry_on_5xx},
  pcs::{
    generated,
    power_cap::types::{PowerCapComponent, PowerCapTaskInfo},
    power_status::types::PowerStatusAll,
    transitions::types::{
      Location, Operation, Transition, TransitionResponse,
      TransitionResponseList,
    },
  },
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

  // -----------------------------------------------------------------
  // power_cap
  // -----------------------------------------------------------------

  /// GET /power-control/v1/power-cap — list every power-cap task.
  /// Retries up to 3 times on 5xx.
  ///
  /// # Errors
  ///
  /// Returns [`Error`] on transport failure or PCS 4xx/5xx.
  pub async fn pcs_power_cap_get(
    &self,
    token: &str,
  ) -> Result<PowerCapTaskInfo, Error> {
    retry_on_5xx(|| async {
      let http = build_client(self.root_cert())?;
      let url = format!("{}/power-control/v1/power-cap", self.base_url());
      let response = http
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(Error::NetError)?;
      pcs_json_or_ochami_error(response).await
    })
    .await
  }

  /// GET /power-control/v1/power-cap/{task_id}.
  /// Retries up to 3 times on 5xx.
  ///
  /// # Errors
  ///
  /// Returns [`Error`] on transport failure or PCS 4xx/5xx.
  pub async fn pcs_power_cap_task_get(
    &self,
    token: &str,
    task_id: &str,
  ) -> Result<PowerCapTaskInfo, Error> {
    retry_on_5xx(|| async {
      let http = build_client(self.root_cert())?;
      let url =
        format!("{}/power-control/v1/power-cap/{}", self.base_url(), task_id);
      let response = http
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(Error::NetError)?;
      pcs_json_or_ochami_error(response).await
    })
    .await
  }

  /// POST /power-control/v1/power-cap/snapshot — start a snapshot
  /// task for `xname_vec`. **Silent bug fix vs pre-migration**: the
  /// hand-written `post_snapshot` used `.put()`; the spec declares
  /// POST for this endpoint. Callers who depended on the server
  /// accepting both verbs (unlikely) may see a status change.
  ///
  /// # Errors
  ///
  /// See [`Ochami::pcs_power_cap_get`]. Does NOT retry on 5xx (POST).
  pub async fn pcs_power_cap_snapshot_post(
    &self,
    token: &str,
    xname_vec: &[&str],
  ) -> Result<PowerCapTaskInfo, Error> {
    log::info!("Create PCS power snapshot for nodes: {xname_vec:?}");
    let http = build_client(self.root_cert())?;
    let url =
      format!("{}/power-control/v1/power-cap/snapshot", self.base_url());
    let response = http
      .post(&url)
      .bearer_auth(token)
      .json(&serde_json::json!({ "xnames": xname_vec }))
      .send()
      .await
      .map_err(Error::NetError)?;
    pcs_json_or_ochami_error(response).await
  }

  /// PATCH /power-control/v1/power-cap — apply power-cap changes.
  /// **Silent bug fix vs pre-migration**: hand-written `patch` used
  /// `.put()` and the wrong URL `/power-cap/snapshot`; both are
  /// corrected here. Callers who worked around the broken URL are
  /// unlikely to exist.
  ///
  /// # Errors
  ///
  /// See [`Ochami::pcs_power_cap_get`]. Does NOT retry on 5xx (PATCH).
  pub async fn pcs_power_cap_patch(
    &self,
    token: &str,
    power_cap: &[PowerCapComponent],
  ) -> Result<PowerCapTaskInfo, Error> {
    log::info!("Apply PCS power cap: {} components", power_cap.len());
    let http = build_client(self.root_cert())?;
    let url = format!("{}/power-control/v1/power-cap", self.base_url());
    let response = http
      .patch(&url)
      .bearer_auth(token)
      .json(&power_cap)
      .send()
      .await
      .map_err(Error::NetError)?;
    pcs_json_or_ochami_error(response).await
  }

  // -----------------------------------------------------------------
  // transitions
  // -----------------------------------------------------------------

  /// GET /power-control/v1/transitions — list every transition
  /// currently tracked by the server. Retries up to 3 times on 5xx.
  ///
  /// # Errors
  ///
  /// Returns [`Error`] on transport failure or PCS 4xx/5xx.
  pub async fn pcs_transitions_get(
    &self,
    token: &str,
  ) -> Result<Vec<TransitionResponse>, Error> {
    retry_on_5xx(|| async {
      let http = build_client(self.root_cert())?;
      let url = format!("{}/power-control/v1/transitions", self.base_url());
      let response = http
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(Error::NetError)?;
      let list: TransitionResponseList =
        pcs_json_or_ochami_error(response).await?;
      Ok(list.transitions)
    })
    .await
  }

  /// GET /power-control/v1/transitions/{id}. Retries up to 3 times on
  /// 5xx.
  ///
  /// # Errors
  ///
  /// Returns [`Error`] on transport failure or PCS 4xx/5xx.
  pub async fn pcs_transition_get(
    &self,
    token: &str,
    transition_id: &str,
  ) -> Result<TransitionResponse, Error> {
    retry_on_5xx(|| async {
      let http = build_client(self.root_cert())?;
      let url = format!(
        "{}/power-control/v1/transitions/{}",
        self.base_url(),
        transition_id
      );
      let response = http
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(Error::NetError)?;
      pcs_json_or_ochami_error(response).await
    })
    .await
  }

  /// DELETE /power-control/v1/transitions/{id} — abort a running
  /// transition. Returns the response body as raw JSON (server-side
  /// shape is `TransitionsAbort` per the spec but the hand-written
  /// client discarded it; kept flexible here as a `serde_json::Value`
  /// pending a real caller need).
  ///
  /// # Errors
  ///
  /// See [`Ochami::pcs_transition_get`]. Does NOT retry on 5xx (DELETE).
  pub async fn pcs_transition_delete(
    &self,
    token: &str,
    transition_id: &str,
  ) -> Result<serde_json::Value, Error> {
    let http = build_client(self.root_cert())?;
    let url = format!(
      "{}/power-control/v1/transitions/{}",
      self.base_url(),
      transition_id
    );
    let response = http
      .delete(&url)
      .bearer_auth(token)
      .send()
      .await
      .map_err(Error::NetError)?;
    pcs_json_or_ochami_error(response).await
  }

  /// POST /power-control/v1/transitions — start a new transition
  /// (power on/off/soft-off/...). `operation` accepts the same
  /// lowercase strings as [`Operation::from_str`] (`"on"`, `"off"`,
  /// `"soft-off"`, `"soft-restart"`, `"hard-restart"`, `"init"`,
  /// `"force-off"`).
  ///
  /// # Errors
  ///
  /// Returns [`Error::Message`] if `operation` is not a valid
  /// `Operation` value, or [`Error`] on transport / PCS 4xx/5xx.
  /// Does NOT retry on 5xx (POST).
  pub async fn pcs_transitions_post(
    &self,
    token: &str,
    operation: &str,
    xname_vec: &[String],
  ) -> Result<TransitionResponse, Error> {
    log::info!("Create PCS transition '{operation}' on {xname_vec:?}");
    let location_vec: Vec<Location> = xname_vec
      .iter()
      .map(|xname| Location {
        xname: xname.clone(),
        deputy_key: None,
      })
      .collect();
    let request_payload = Transition {
      operation: Operation::from_str(operation)?,
      task_deadline_minutes: None,
      location: location_vec,
    };
    let http = build_client(self.root_cert())?;
    let url = format!("{}/power-control/v1/transitions", self.base_url());
    let response = http
      .post(&url)
      .bearer_auth(token)
      .json(&request_payload)
      .send()
      .await
      .map_err(Error::NetError)?;
    pcs_json_or_ochami_error(response).await
  }

  /// Post a transition and block until it reports `transition_status
  /// == "completed"`. Polls with `pcs_transition_get` every 3 seconds
  /// up to 300 attempts (~15 minutes).
  ///
  /// # Errors
  ///
  /// Any error from the underlying `pcs_transitions_post` or
  /// `pcs_transition_wait_to_complete` calls propagates.
  pub async fn pcs_transition_post_block(
    &self,
    token: &str,
    operation: &str,
    xname_vec: &[String],
  ) -> Result<TransitionResponse, Error> {
    let started = self
      .pcs_transitions_post(token, operation, xname_vec)
      .await?;
    log::info!("PCS transition ID: {}", started.transition_id);
    self
      .pcs_transition_wait_to_complete(token, &started.transition_id)
      .await
  }

  /// Poll `pcs_transition_get` until the transition reports
  /// `transition_status == "completed"` or 300 attempts elapse.
  /// Sleeps 3 seconds between polls.
  ///
  /// # Errors
  ///
  /// Any error from `pcs_transition_get` propagates. Returns the
  /// last observed `TransitionResponse` even if the loop exits
  /// without reaching `"completed"` (matches hand-written
  /// `wait_to_complete` behaviour).
  pub async fn pcs_transition_wait_to_complete(
    &self,
    token: &str,
    transition_id: &str,
  ) -> Result<TransitionResponse, Error> {
    let mut transition = self.pcs_transition_get(token, transition_id).await?;
    let max_attempt = 300;
    for attempt in 1..=max_attempt {
      if transition.transition_status == "completed" {
        break;
      }
      transition = self.pcs_transition_get(token, transition_id).await?;
      log::info!(
        "Power '{}' summary - status: {}, failed: {}, in-progress: {}, succeeded: {}, total: {}. Attempt {} of {}",
        transition.operation,
        transition.transition_status,
        transition.task_counts.failed,
        transition.task_counts.in_progress,
        transition.task_counts.succeeded,
        transition.task_counts.total,
        attempt,
        max_attempt,
      );
      tokio::time::sleep(Duration::from_secs(3)).await;
    }
    Ok(transition)
  }
}

/// Deserialize a successful response body as `T`, or convert a 4xx/5xx
/// response into `Error::OchamiError { status, body }`. Shared by
/// every raw-reqwest PCS wrapper method above.
async fn pcs_json_or_ochami_error<T>(
  response: reqwest::Response,
) -> Result<T, Error>
where
  T: serde::de::DeserializeOwned,
{
  let status = response.status();
  if status.is_success() {
    response.json::<T>().await.map_err(Error::NetError)
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
