//! Wrapper for `/groups`. Replaces `src/hsm/group/http_client.rs`.
//!
//! `hsm_group_get` stays on raw `reqwest` for multi-value `group=` query
//! support: the generated `do_groups_get` only accepts a single `group`
//! and a single `tag` value (`Option<&str>` each), so repeated `?group=`
//! params (as historically relied on by callers passing several group
//! labels) require hand-rolled query-building.
//!
//! All other methods route through the generated progenitor client
//! (`gen_client` bakes bearer auth into `default_headers` so no
//! per-request `.bearer_auth()` is needed on progenitor calls).
//!
//! Note: there is no `GroupArray` schema in the HSM OpenAPI spec — `GET
//! /groups` returns a bare `Vec<Group100>`. `GroupArray` below is a
//! module-local alias for that, kept for readability/signature parity
//! with the design spec's naming.

use crate::{
  error::Error,
  hsm::{
    generated::types::{
      Group100 as Group, MemberId, Members100 as Members, ResourceUri100,
      XNameRw100,
    },
    types::HsmActionResponse,
    wrapper::{gen_client, map_err, run},
  },
  http::{build_client, handle_json_response, retry_on_5xx},
  Ochami,
};

/// `GET /groups` response shape. No named array schema exists in the
/// HSM spec for this endpoint (see module docs).
type GroupArray = Vec<Group>;

impl Ochami {
  // --------------------------------------------------------------------
  // Convenience methods (delegate to raw `hsm_group_get`).
  // --------------------------------------------------------------------

  /// Fetch all HSM groups.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_group_get_all(
    &self,
    token: &str,
  ) -> Result<GroupArray, Error> {
    self.hsm_group_get(token, None, None).await
  }

  // --------------------------------------------------------------------
  // Raw reqwest methods (multi-value `group=` semantics).
  // --------------------------------------------------------------------

  /// `GET /hsm/v2/groups` — with optional filters. `group_name_vec` is
  /// expanded into repeated `?group=` params (preserves historical
  /// multi-value semantics); `tags` is a single `?tag=` value.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_group_get(
    &self,
    token: &str,
    group_name_vec: Option<&[&str]>,
    tags: Option<&str>,
  ) -> Result<GroupArray, Error> {
    retry_on_5xx(|| async {
      let client = build_client(self.root_cert())?;
      let url = format!("{}/hsm/v2/groups", self.base_url());

      let mut query_params: Vec<(&str, &str)> = Vec::new();
      if let Some(names) = group_name_vec {
        for name in names {
          query_params.push(("group", name));
        }
      }
      if let Some(tag) = tags {
        query_params.push(("tag", tag));
      }

      let response = client
        .get(&url)
        .query(&query_params)
        .bearer_auth(token)
        .send()
        .await?;
      handle_json_response::<GroupArray>(response).await
    })
    .await
  }

  // --------------------------------------------------------------------
  // Progenitor-routed methods.
  // --------------------------------------------------------------------

  /// `GET /hsm/v2/groups/{group_label}` — fetch one group by label.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_group_get_one(
    &self,
    token: &str,
    label: &str,
  ) -> Result<Group, Error> {
    retry_on_5xx(|| async {
      let gc = gen_client(self, token)?;
      match gc.do_group_get(label, None).await {
        Ok(rv) => Ok(rv.into_inner()),
        Err(e) => Err(map_err(e).await),
      }
    })
    .await
  }

  /// `GET /hsm/v2/groups/labels` — fetch all existing group labels.
  ///
  /// # Errors
  /// See [`Self::hsm_group_get_one`].
  pub async fn hsm_group_get_labels(
    &self,
    token: &str,
  ) -> Result<Vec<String>, Error> {
    retry_on_5xx(|| async {
      let gc = gen_client(self, token)?;
      match gc.do_group_labels_get().await {
        Ok(rv) => Ok(rv.into_inner()),
        Err(e) => Err(map_err(e).await),
      }
    })
    .await
  }

  /// `GET /hsm/v2/groups/{group_label}/members` — fetch a group's
  /// member xname IDs.
  ///
  /// # Errors
  /// See [`Self::hsm_group_get_one`].
  pub async fn hsm_group_get_members(
    &self,
    token: &str,
    label: &str,
  ) -> Result<Members, Error> {
    retry_on_5xx(|| async {
      let gc = gen_client(self, token)?;
      match gc.do_group_members_get(label, None).await {
        Ok(rv) => Ok(rv.into_inner()),
        Err(e) => Err(map_err(e).await),
      }
    })
    .await
  }

  /// `POST /hsm/v2/groups` — create a new group.
  ///
  /// Note: the generated `do_groups_post` returns
  /// `Vec<ResourceUri100>` (the created group's resource URI), not
  /// `HsmActionResponse` — the HSM spec has no `Response.1.0.0` body on
  /// this endpoint's 201 response.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_group_post(
    &self,
    token: &str,
    group: &Group,
  ) -> Result<Vec<ResourceUri100>, Error> {
    run(
      self,
      token,
      |gc| async move { gc.do_groups_post(group).await },
    )
    .await
  }

  /// `POST /hsm/v2/groups/{group_label}/members` — add a single member
  /// (xname ID) to a group.
  ///
  /// Note: the generated `do_group_members_post` returns
  /// `Vec<ResourceUri100>`, not `HsmActionResponse` (see
  /// [`Self::hsm_group_post`]).
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_group_post_member(
    &self,
    token: &str,
    label: &str,
    xname_id: &str,
  ) -> Result<Vec<ResourceUri100>, Error> {
    let member = MemberId {
      id: Some(XNameRw100(xname_id.to_string())),
    };
    run(self, token, |gc| async move {
      gc.do_group_members_post(label, &member).await
    })
    .await
  }

  /// `DELETE /hsm/v2/groups/{group_label}` — delete a single group.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_group_delete_one(
    &self,
    token: &str,
    label: &str,
  ) -> Result<HsmActionResponse, Error> {
    run(
      self,
      token,
      |gc| async move { gc.do_group_delete(label).await },
    )
    .await
  }

  /// `DELETE /hsm/v2/groups/{group_label}/members/{xname_id}` — remove a
  /// single member (xname ID) from a group.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_group_delete_member(
    &self,
    token: &str,
    label: &str,
    xname_id: &str,
  ) -> Result<HsmActionResponse, Error> {
    run(self, token, |gc| async move {
      gc.do_group_member_delete(label, xname_id).await
    })
    .await
  }
}
