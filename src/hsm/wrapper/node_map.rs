//! Wrapper for `/Defaults/NodeMaps`. Replaces
//! `src/hsm/node_map/http_client.rs` and
//! `src/hsm/defaults/node_map/http_client.rs` (unified).
//!
//! All methods route through the generated progenitor client
//! (`gen_client` bakes bearer auth into `default_headers` so no
//! per-request `.bearer_auth()` is needed).
//!
//! **Method-count divergence from the design spec's Phase 3 sketch**:
//! the sketch anticipated 7 methods — separate single-item
//! `hsm_node_map_post` and bulk `hsm_node_map_post_bulk` entries. The
//! generated client exposes only ONE `POST /Defaults/NodeMaps`
//! endpoint (`do_node_map_post`), and it is array-shaped
//! (`&NodeMapArrayNodeMapArray` in, i.e. always "bulk"). This matches
//! both historical hand-written `http_client.rs::post` functions,
//! which likewise only ever accepted a `NodeMapArray` — there was
//! never a genuine single-item POST to unify. This wrapper therefore
//! has 6 methods, all named to match the real generated surface.
//!
//! **Return-type divergence**: `hsm_node_map_put` returns
//! `NodeMap100PostNodeMap` (aliased `NodeMapPost` here), not
//! `HsmActionResponse`/`Response100` as a naive read of the design
//! spec sketch might suggest. The generated `do_node_map_put` genuinely
//! returns the created/updated `NodeMapPost` representation, so that
//! real type is used as-is.

use crate::{
  error::Error,
  hsm::{
    generated::types::{
      NodeMap100NodeMap as NodeMap, NodeMap100PostNodeMap as NodeMapPost,
      NodeMapArrayNodeMapArray as NodeMapArray,
    },
    types::HsmActionResponse,
    wrapper::run,
  },
  http::retry_on_5xx,
  Ochami,
};

impl Ochami {
  /// `GET /hsm/v2/Defaults/NodeMaps` — fetch all NodeMap entries.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_node_map_get_all(
    &self,
    token: &str,
  ) -> Result<NodeMapArray, Error> {
    retry_on_5xx(|| async {
      run(self, token, |c| async move { c.do_node_maps_get().await }).await
    })
    .await
  }

  /// `GET /hsm/v2/Defaults/NodeMaps/{xname}` — fetch a single NodeMap
  /// entry by xname.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_node_map_get_one(
    &self,
    token: &str,
    xname: &str,
  ) -> Result<NodeMap, Error> {
    retry_on_5xx(|| async {
      run(
        self,
        token,
        |c| async move { c.do_node_map_get(xname).await },
      )
      .await
    })
    .await
  }

  /// `POST /hsm/v2/Defaults/NodeMaps` — create or modify the given set
  /// of NodeMap entries. The endpoint is array-shaped; see the
  /// module-level doc for why there is no separate single-item POST.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_node_map_post(
    &self,
    token: &str,
    node_map_array: &NodeMapArray,
  ) -> Result<HsmActionResponse, Error> {
    run(self, token, |c| async move {
      c.do_node_map_post(node_map_array).await
    })
    .await
  }

  /// `PUT /hsm/v2/Defaults/NodeMaps/{xname}` — create or update a
  /// single NodeMap entry.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_node_map_put(
    &self,
    token: &str,
    xname: &str,
    node_map: &NodeMap,
  ) -> Result<NodeMapPost, Error> {
    run(self, token, |c| async move {
      c.do_node_map_put(xname, node_map).await
    })
    .await
  }

  /// `DELETE /hsm/v2/Defaults/NodeMaps` — delete every NodeMap entry.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_node_map_delete_all(
    &self,
    token: &str,
  ) -> Result<HsmActionResponse, Error> {
    run(
      self,
      token,
      |c| async move { c.do_node_maps_delete_all().await },
    )
    .await
  }

  /// `DELETE /hsm/v2/Defaults/NodeMaps/{xname}` — delete a single
  /// NodeMap entry.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_node_map_delete_one(
    &self,
    token: &str,
    xname: &str,
  ) -> Result<HsmActionResponse, Error> {
    run(
      self,
      token,
      |c| async move { c.do_node_map_delete(xname).await },
    )
    .await
  }
}
