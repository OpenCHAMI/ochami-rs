//! Wrapper for `/partitions`. Replaces `src/hsm/partition/http_client.rs`.
//!
//! All methods route through the generated progenitor client
//! (`gen_client` bakes bearer auth into `default_headers` so no
//! per-request `.bearer_auth()` is needed).
//!
//! Note: there is no `PartitionArray` schema in the HSM OpenAPI spec —
//! `GET /partitions` returns a bare `Vec<Partition100>`. `PartitionArray`
//! below is a module-local alias for that, kept for readability/signature
//! parity with the design spec's naming (mirrors `group.rs`'s
//! `GroupArray`).
//!
//! **Type-name deviation from the design spec's guess**: the spec sketch
//! guessed `Group100Members as Members` for the members type. There is no
//! such type — the real generated type is `Members100` (the same shared
//! type `group.rs` already aliases as `Members`), used as-is here.
//!
//! **`hsm_partition_get` signature deviation**: the design spec sketch
//! included a `tags: Option<&str>` parameter, presumably by analogy with
//! the list endpoint's `partition`+`tag` filters. The real generated
//! single-partition endpoint, `do_partition_get(partition_name)` (`GET
//! /partitions/{partition_name}`), takes no tag (or any other) filter —
//! tag-based filtering only exists on the list endpoint
//! (`do_partitions_get`, already exposed with `None, None` by
//! `hsm_partition_get_all`). Per the same narrowing precedent used in
//! `memberships.rs` (dropping filters the real generated method doesn't
//! support), this wrapper drops the impossible `tags` parameter and
//! implements `hsm_partition_get(token, partition_name) -> Result<Partition>`
//! as a direct passthrough to `do_partition_get`.
//!
//! **Return-type deviation**: `hsm_partition_post` and
//! `hsm_partition_post_member` return `Vec<ResourceUri100>`, not
//! `HsmActionResponse` as the design spec sketch guessed — the generated
//! `do_partitions_post`/`do_partition_members_post` return the created
//! resource's URI list (201 response), matching the identical situation
//! already documented in `group.rs` for `do_groups_post`/
//! `do_group_members_post` (no `Response.1.0.0` body on these endpoints).

use crate::{
  Ochami,
  error::Error,
  hsm::{
    generated::types::{
      MemberId, Members100 as Members, Partition100 as Partition,
      ResourceUri100, XNameRw100,
    },
    types::HsmActionResponse,
    wrapper::run,
  },
  http::retry_on_5xx,
};

/// `GET /partitions` response shape. No named array schema exists in the
/// HSM spec for this endpoint (see module docs).
type PartitionArray = Vec<Partition>;

impl Ochami {
  /// `GET /hsm/v2/partitions` — fetch all partitions.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_partition_get_all(
    &self,
    token: &str,
  ) -> Result<PartitionArray, Error> {
    retry_on_5xx(|| async {
      run(self, token, |c| async move {
        c.do_partitions_get(None, None).await
      })
      .await
    })
    .await
  }

  /// `GET /hsm/v2/partitions/{partition_name}` — fetch a single
  /// partition by name. See the module-level doc for why this has no
  /// `tags` parameter (the design spec's sketch guessed one, but the
  /// real single-partition endpoint has no such filter).
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_partition_get(
    &self,
    token: &str,
    partition_name: &str,
  ) -> Result<Partition, Error> {
    retry_on_5xx(|| async {
      run(self, token, |c| async move {
        c.do_partition_get(partition_name).await
      })
      .await
    })
    .await
  }

  /// `GET /hsm/v2/partitions/names` — fetch all existing partition
  /// names.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_partition_get_names(
    &self,
    token: &str,
  ) -> Result<Vec<String>, Error> {
    retry_on_5xx(|| async {
      run(self, token, |c| async move { c.do_partition_names_get().await })
        .await
    })
    .await
  }

  /// `GET /hsm/v2/partitions/{partition_name}/members` — fetch a
  /// partition's member xname IDs.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_partition_get_members(
    &self,
    token: &str,
    name: &str,
  ) -> Result<Members, Error> {
    retry_on_5xx(|| async {
      run(self, token, |c| async move {
        c.do_partition_members_get(name).await
      })
      .await
    })
    .await
  }

  /// `POST /hsm/v2/partitions` — create a new partition.
  ///
  /// Note: the generated `do_partitions_post` returns
  /// `Vec<ResourceUri100>` (the created partition's resource URI), not
  /// `HsmActionResponse` — see the module-level doc.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_partition_post(
    &self,
    token: &str,
    partition: &Partition,
  ) -> Result<Vec<ResourceUri100>, Error> {
    run(self, token, |c| async move {
      c.do_partitions_post(partition).await
    })
    .await
  }

  /// `POST /hsm/v2/partitions/{partition_name}/members` — add a single
  /// member (xname ID) to a partition.
  ///
  /// Note: the generated `do_partition_members_post` returns
  /// `Vec<ResourceUri100>`, not `HsmActionResponse` (see
  /// [`Self::hsm_partition_post`]).
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_partition_post_member(
    &self,
    token: &str,
    name: &str,
    xname_id: &str,
  ) -> Result<Vec<ResourceUri100>, Error> {
    let member = MemberId {
      id: Some(XNameRw100(xname_id.to_string())),
    };
    run(self, token, |c| async move {
      c.do_partition_members_post(name, &member).await
    })
    .await
  }

  /// `DELETE /hsm/v2/partitions/{partition_name}` — delete a single
  /// partition.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_partition_delete_one(
    &self,
    token: &str,
    name: &str,
  ) -> Result<HsmActionResponse, Error> {
    run(self, token, |c| async move { c.do_partition_delete(name).await })
      .await
  }

  /// `DELETE /hsm/v2/partitions/{partition_name}/members/{xname_id}` —
  /// remove a single member (xname ID) from a partition.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_partition_delete_member(
    &self,
    token: &str,
    name: &str,
    xname_id: &str,
  ) -> Result<HsmActionResponse, Error> {
    run(self, token, |c| async move {
      c.do_partition_member_delete(name, xname_id).await
    })
    .await
  }
}
