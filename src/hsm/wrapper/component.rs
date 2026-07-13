//! Wrapper for `/State/Components`. Replaces
//! `src/hsm/component/http_client.rs` and
//! `src/hsm/state/components/http_client.rs` (unified).
//!
//! Two methods stay on raw `reqwest` for multi-value query semantics:
//! - `hsm_component_get` — comma-split `nid` used by
//!   `backend_connector::nid_to_xname`.
//! - `hsm_component_get_query` — same query-param set on
//!   `/State/Components/Query/{xname}`.
//!
//! All other methods route through the generated progenitor client
//! (`gen_client` bakes bearer auth into `default_headers` so no
//! per-request `.bearer_auth()` is needed on progenitor calls).
//!
//! Note: the historical `hsm/component/http_client.rs::delete` typoed
//! the path as `Componnets`; the progenitor-routed
//! `hsm_component_delete` here uses the correct
//! `/State/Components` path.

use crate::{
  Ochami,
  error::Error,
  hsm::{
    generated::types::{
      Component100Component as Component, Component100Put as ComponentPut,
      ComponentArrayComponentArray as ComponentArray, ComponentArrayPostArray,
      ComponentArrayPostByNidQuery, ComponentArrayPostQuery,
    },
    types::HsmActionResponse,
    wrapper::{gen_client, map_err, run},
  },
  http::{build_client, handle_json_response, retry_on_5xx},
};

impl Ochami {
  // --------------------------------------------------------------------
  // Convenience methods (delegate to raw `hsm_component_get`).
  // --------------------------------------------------------------------

  /// Fetch all HSM components.
  ///
  /// `nid_only` toggles the lightweight nid-only response projection.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, deserialization, or HSM failure.
  pub async fn hsm_component_get_all(
    &self,
    token: &str,
    nid_only: Option<&str>,
  ) -> Result<ComponentArray, Error> {
    self
      .hsm_component_get(
        token, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, nid_only,
      )
      .await
  }

  /// Fetch all HSM components filtered to `Type=Node`.
  ///
  /// # Errors
  /// Same as [`Self::hsm_component_get`].
  pub async fn hsm_component_get_all_nodes(
    &self,
    token: &str,
    nid_only: Option<&str>,
  ) -> Result<ComponentArray, Error> {
    self
      .hsm_component_get(
        token,
        None,
        Some("Node"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        nid_only,
      )
      .await
  }

  // --------------------------------------------------------------------
  // Raw reqwest methods (multi-value nid semantics).
  // --------------------------------------------------------------------

  /// `GET /hsm/v2/State/Components` — with query filters. Comma-separated
  /// `nid` is split into repeated `?nid=` params (preserves historical
  /// multi-value semantics that
  /// `backend_connector::nid_to_xname` relies on).
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  #[allow(clippy::too_many_arguments)]
  pub async fn hsm_component_get(
    &self,
    token: &str,
    id: Option<&str>,
    r#type: Option<&str>,
    state: Option<&str>,
    flag: Option<&str>,
    role: Option<&str>,
    subrole: Option<&str>,
    enabled: Option<&str>,
    software_status: Option<&str>,
    subtype: Option<&str>,
    arch: Option<&str>,
    class: Option<&str>,
    nid: Option<&str>,
    nid_start: Option<&str>,
    nid_end: Option<&str>,
    partition: Option<&str>,
    group: Option<&str>,
    state_only: Option<&str>,
    flag_only: Option<&str>,
    role_only: Option<&str>,
    nid_only: Option<&str>,
  ) -> Result<ComponentArray, Error> {
    retry_on_5xx(|| async {
      let client = build_client(self.root_cert())?;
      let url = format!("{}/hsm/v2/State/Components", self.base_url());

      let nid_vec = nid.map(|nids| {
        nids
          .split(',')
          .map(|n| ("nid", Some(n)))
          .collect::<Vec<(&str, Option<&str>)>>()
      });

      let mut query_params: Vec<(&str, Option<&str>)> = vec![
        ("id", id),
        ("type", r#type),
        ("state", state),
        ("flag", flag),
        ("role", role),
        ("subrole", subrole),
        ("enabled", enabled),
        ("softwarestatus", software_status),
        ("subtype", subtype),
        ("arch", arch),
        ("class", class),
        ("nidstart", nid_start),
        ("nidend", nid_end),
        ("partition", partition),
        ("group", group),
        ("stateonly", state_only),
        ("flagonly", flag_only),
        ("roleonly", role_only),
        ("nidonly", nid_only),
      ];
      if let Some(mut nids) = nid_vec {
        query_params.append(&mut nids);
      }

      let response = client
        .get(&url)
        .query(&query_params)
        .bearer_auth(token)
        .send()
        .await?;
      handle_json_response::<ComponentArray>(response).await
    })
    .await
  }

  /// `GET /hsm/v2/State/Components/Query/{xname}` — component query
  /// with the full filter set. Comma-separated `nid` is split into
  /// repeated `?nid=` params (same semantics as
  /// [`Self::hsm_component_get`]).
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  #[allow(clippy::too_many_arguments)]
  pub async fn hsm_component_get_query(
    &self,
    token: &str,
    xname: &str,
    r#type: Option<&str>,
    state: Option<&str>,
    flag: Option<&str>,
    role: Option<&str>,
    subrole: Option<&str>,
    enabled: Option<&str>,
    software_status: Option<&str>,
    subtype: Option<&str>,
    arch: Option<&str>,
    class: Option<&str>,
    nid: Option<&str>,
    nid_start: Option<&str>,
    nid_end: Option<&str>,
    partition: Option<&str>,
    group: Option<&str>,
    state_only: Option<&str>,
    flag_only: Option<&str>,
    role_only: Option<&str>,
    nid_only: Option<&str>,
  ) -> Result<ComponentArray, Error> {
    retry_on_5xx(|| async {
      let client = build_client(self.root_cert())?;
      let url = format!(
        "{}/hsm/v2/State/Components/Query/{}",
        self.base_url(),
        xname
      );

      let nid_vec = nid.map(|nids| {
        nids
          .split(',')
          .map(|n| ("nid", Some(n)))
          .collect::<Vec<(&str, Option<&str>)>>()
      });

      let mut query_params: Vec<(&str, Option<&str>)> = vec![
        ("type", r#type),
        ("state", state),
        ("flag", flag),
        ("role", role),
        ("subrole", subrole),
        ("enabled", enabled),
        ("softwarestatus", software_status),
        ("subtype", subtype),
        ("arch", arch),
        ("class", class),
        ("nidstart", nid_start),
        ("nidend", nid_end),
        ("partition", partition),
        ("group", group),
        ("stateonly", state_only),
        ("flagonly", flag_only),
        ("roleonly", role_only),
        ("nidonly", nid_only),
      ];
      if let Some(mut nids) = nid_vec {
        query_params.append(&mut nids);
      }

      let response = client
        .get(&url)
        .query(&query_params)
        .bearer_auth(token)
        .send()
        .await?;
      handle_json_response::<ComponentArray>(response).await
    })
    .await
  }

  // --------------------------------------------------------------------
  // Progenitor-routed methods.
  // --------------------------------------------------------------------

  /// `GET /hsm/v2/State/Components/{xname}` — fetch one component by
  /// xname.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_component_get_one(
    &self,
    token: &str,
    xname: &str,
  ) -> Result<Component, Error> {
    retry_on_5xx(|| async {
      let gc = gen_client(self, token)?;
      match gc.do_component_get(xname).await {
        Ok(rv) => Ok(rv.into_inner()),
        Err(e) => Err(map_err(e).await),
      }
    })
    .await
  }

  /// `GET /hsm/v2/State/Components/ByNID/{nid}` — fetch one component
  /// by NID.
  ///
  /// # Errors
  /// See [`Self::hsm_component_get_one`].
  pub async fn hsm_component_get_by_nid(
    &self,
    token: &str,
    nid: &str,
  ) -> Result<Component, Error> {
    retry_on_5xx(|| async {
      let gc = gen_client(self, token)?;
      match gc.do_component_by_nid_get(nid).await {
        Ok(rv) => Ok(rv.into_inner()),
        Err(e) => Err(map_err(e).await),
      }
    })
    .await
  }

  /// `POST /hsm/v2/State/Components` — bulk-create/update components.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_component_post(
    &self,
    token: &str,
    component: ComponentArrayPostArray,
  ) -> Result<(), Error> {
    run(self, token, |c| async move { c.do_components_post(&component).await })
      .await
  }

  /// `POST /hsm/v2/State/Components/Query` — retrieve components
  /// matching the supplied criteria.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_component_post_query(
    &self,
    token: &str,
    component: ComponentArrayPostQuery,
  ) -> Result<ComponentArray, Error> {
    run(self, token, |c| async move {
      c.do_components_query_post(&component).await
    })
    .await
  }

  /// `POST /hsm/v2/State/Components/ByNID/Query` — retrieve components
  /// matching the supplied NID query.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_component_post_bynid_query(
    &self,
    token: &str,
    component: ComponentArrayPostByNidQuery,
  ) -> Result<ComponentArray, Error> {
    run(self, token, |c| async move {
      c.do_component_by_nid_query_post(&component).await
    })
    .await
  }

  /// `PUT /hsm/v2/State/Components/{xname}` — replace a component.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_component_put(
    &self,
    token: &str,
    xname: &str,
    component: ComponentPut,
  ) -> Result<(), Error> {
    run(self, token, |c| async move {
      c.do_component_put(xname, &component).await
    })
    .await
  }

  /// `DELETE /hsm/v2/State/Components/{xname}` — delete a single
  /// component.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_component_delete_one(
    &self,
    token: &str,
    xname: &str,
  ) -> Result<HsmActionResponse, Error> {
    run(self, token, |c| async move { c.do_component_delete(xname).await })
      .await
  }

  /// `DELETE /hsm/v2/State/Components` — delete every component.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_component_delete(
    &self,
    token: &str,
  ) -> Result<HsmActionResponse, Error> {
    run(self, token, |c| async move { c.do_components_delete_all().await })
      .await
  }
}
