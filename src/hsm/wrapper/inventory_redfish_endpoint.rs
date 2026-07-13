//! Wrapper for `/Inventory/RedfishEndpoints`. Replaces
//! `src/hsm/inventory/redfish_endpoint/http_client.rs`.
//!
//! Per Task 0's output-reference doc (§ L, "Structural rework ...
//! NOT found in this Task 0"), the slim-re-export plan for this
//! resource family is viable: the generated
//! `RedfishEndpoint100RedfishEndpoint` / `RedfishEndpointArrayRedfishEndpointArray`
//! types are drop-in for the hand-rolled `RedfishEndpoint` /
//! `RedfishEndpointArray` types this module previously used. All
//! methods below route through the generated progenitor client
//! (`gen_client` bakes bearer auth into `default_headers` so no
//! per-request `.bearer_auth()` is needed).
//!
//! **Type-name deviation from the design spec's guess**: the spec
//! sketch guessed separate `RedfishEndpointCreate` / `RedfishEndpointPut`
//! / `RedfishEndpointPatch` body types. There is no such split in the
//! generated types — POST/PUT/PATCH/GET all share the single
//! `RedfishEndpoint100RedfishEndpoint` schema, aliased here as
//! `RedfishEndpoint`.
//!
//! **Return-type deviations from the design spec's guess**:
//! - `hsm_inventory_redfish_endpoint_post` returns `Vec<ResourceUri100>`
//!   (the created endpoint's resource URI), not `HsmActionResponse` —
//!   the generated `do_redfish_endpoints_post` has a 201 response with
//!   no `Response_1.0.0` body, matching the identical situation already
//!   documented in `partition.rs`/`group.rs`.
//! - `hsm_inventory_redfish_endpoint_put` and
//!   `hsm_inventory_redfish_endpoint_patch` return the full updated
//!   `RedfishEndpoint`, not `HsmActionResponse` — the generated
//!   `do_redfish_endpoint_put`/`do_redfish_endpoint_patch` 200 responses
//!   are typed as `RedfishEndpoint100RedfishEndpoint`, not
//!   `Response_1.0.0`.
//! - `hsm_inventory_redfish_endpoint_delete_one` does return
//!   `HsmActionResponse` (`Response_1.0.0`), matching the design spec's
//!   guess.
//!
//! **Method-name/verb correction vs. the design spec's guess**: the
//! spec sketch guessed a `hsm_inventory_redfish_endpoint_post_query`
//! method returning `HsmActionResponse`. There is no POST-based query
//! endpoint in the HSM OpenAPI spec — `/Inventory/RedfishEndpoints/Query/{xname}`
//! is a `GET` (`do_redfish_endpoint_query_get`) returning a
//! `RedfishEndpointArray`, matching the pre-existing (buggy) hand-written
//! `redfish_endpoint::http_client::get_query`. This wrapper implements
//! it as `hsm_inventory_redfish_endpoint_get_query`, fixing the verb and
//! return type to match reality.
//!
//! **Latent bug fixed**: the hand-written `http_client.rs::get_query`
//! built its URL from the singular `/Inventory/RedfishEndpoint/Query/{xname}`
//! (missing the trailing `s` on `RedfishEndpoint`). The generated
//! `do_redfish_endpoint_query_get` uses the correct plural
//! `/Inventory/RedfishEndpoints/Query/{xname}` path.

use std::str::FromStr;

use crate::{
  Ochami,
  error::Error,
  hsm::{
    generated::types::{
      DoRedfishEndpointsGetType, RedfishEndpoint100RedfishEndpoint as RedfishEndpoint,
      RedfishEndpointArrayRedfishEndpointArray as RedfishEndpointArray,
      ResourceUri100,
    },
    types::HsmActionResponse,
    wrapper::run,
  },
  http::retry_on_5xx,
};

impl Ochami {
  /// `GET /hsm/v2/Inventory/RedfishEndpoints` — fetch all
  /// RedfishEndpoints, filtered by the supplied criteria.
  ///
  /// `type_` is coerced from `&str` into the generated per-endpoint
  /// enum type (`DoRedfishEndpointsGetType`); an unparsable value is
  /// dropped (treated as "no filter"), matching the log-and-drop
  /// coercion pattern used elsewhere in this migration (e.g.
  /// `memberships.rs`).
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  #[allow(clippy::too_many_arguments)]
  pub async fn hsm_inventory_redfish_endpoint_get(
    &self,
    token: &str,
    id: Option<&str>,
    fqdn: Option<&str>,
    type_: Option<&str>,
    uuid: Option<&str>,
    macaddr: Option<&str>,
    ip_address: Option<&str>,
    last_status: Option<&str>,
  ) -> Result<RedfishEndpointArray, Error> {
    retry_on_5xx(|| async {
      let type_ =
        type_.and_then(|s| DoRedfishEndpointsGetType::from_str(s).ok());
      run(self, token, |c| async move {
        c.do_redfish_endpoints_get(
          fqdn, id, ip_address, last_status, macaddr, type_, uuid,
        )
        .await
      })
      .await
    })
    .await
  }

  /// `GET /hsm/v2/Inventory/RedfishEndpoints/{xname}` — fetch a single
  /// RedfishEndpoint by xname.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_inventory_redfish_endpoint_get_one(
    &self,
    token: &str,
    xname: &str,
  ) -> Result<RedfishEndpoint, Error> {
    retry_on_5xx(|| async {
      run(self, token, |c| async move {
        c.do_redfish_endpoint_get(xname).await
      })
      .await
    })
    .await
  }

  /// `GET /hsm/v2/Inventory/RedfishEndpoints/Query/{xname}` — query a
  /// single xname, returning zero or more matching RedfishEndpoint
  /// entries. See the module-level doc for why this is a `GET`
  /// returning `RedfishEndpointArray` rather than the design spec's
  /// guessed `POST` returning `HsmActionResponse`.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_inventory_redfish_endpoint_get_query(
    &self,
    token: &str,
    xname: &str,
  ) -> Result<RedfishEndpointArray, Error> {
    retry_on_5xx(|| async {
      run(self, token, |c| async move {
        c.do_redfish_endpoint_query_get(xname).await
      })
      .await
    })
    .await
  }

  /// `POST /hsm/v2/Inventory/RedfishEndpoints` — create a new
  /// RedfishEndpoint.
  ///
  /// Note: the generated `do_redfish_endpoints_post` returns
  /// `Vec<ResourceUri100>` (the created endpoint's resource URI), not
  /// `HsmActionResponse` — see the module-level doc.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_inventory_redfish_endpoint_post(
    &self,
    token: &str,
    ep: &RedfishEndpoint,
  ) -> Result<Vec<ResourceUri100>, Error> {
    run(self, token, |c| async move {
      c.do_redfish_endpoints_post(ep).await
    })
    .await
  }

  /// `PUT /hsm/v2/Inventory/RedfishEndpoints/{xname}` — create or
  /// update a RedfishEndpoint record.
  ///
  /// Note: the generated `do_redfish_endpoint_put` returns the full
  /// updated `RedfishEndpoint`, not `HsmActionResponse` — see the
  /// module-level doc.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_inventory_redfish_endpoint_put(
    &self,
    token: &str,
    xname: &str,
    ep: &RedfishEndpoint,
  ) -> Result<RedfishEndpoint, Error> {
    run(self, token, |c| async move {
      c.do_redfish_endpoint_put(xname, ep).await
    })
    .await
  }

  /// `PATCH /hsm/v2/Inventory/RedfishEndpoints/{xname}` — partially
  /// update a RedfishEndpoint record.
  ///
  /// Note: the generated `do_redfish_endpoint_patch` returns the full
  /// updated `RedfishEndpoint`, not `HsmActionResponse` — see the
  /// module-level doc.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_inventory_redfish_endpoint_patch(
    &self,
    token: &str,
    xname: &str,
    ep: &RedfishEndpoint,
  ) -> Result<RedfishEndpoint, Error> {
    run(self, token, |c| async move {
      c.do_redfish_endpoint_patch(xname, ep).await
    })
    .await
  }

  /// `DELETE /hsm/v2/Inventory/RedfishEndpoints/{xname}` — delete a
  /// single RedfishEndpoint.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_inventory_redfish_endpoint_delete_one(
    &self,
    token: &str,
    xname: &str,
  ) -> Result<HsmActionResponse, Error> {
    run(self, token, |c| async move {
      c.do_redfish_endpoint_delete(xname).await
    })
    .await
  }
}
