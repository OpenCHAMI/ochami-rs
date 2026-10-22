//! Wrapper for `/memberships`. Replaces
//! `src/hsm/memberships/http_client.rs`.
//!
//! Both methods route through the generated progenitor client
//! (`gen_client` bakes bearer auth into `default_headers` so no
//! per-request `.bearer_auth()` is needed).
//!
//! `hsm_memberships_get` is the one method in this migration requiring
//! in-body enum coercion. The generated `do_memberships_get` takes 5
//! per-endpoint enum newtypes (`DoMembershipsGetArch`,
//! `DoMembershipsGetClass`, `DoMembershipsGetFlag`,
//! `DoMembershipsGetState`, `DoMembershipsGetType`) — these are
//! endpoint-scoped enums typify emitted from the inline OpenAPI query
//! parameter schemas, not the shared domain-wide `HmsArch100` /
//! `HmsClass100` / etc. types used elsewhere in the HSM spec. `role`
//! and `subrole` are plain `Option<&str>` in the generated signature
//! (no `HmsRole100`/`HmsSubRole100` newtype involved), and `group` /
//! `partition` are likewise plain `Option<&str>` (no `ResourceName`
//! wrapping) — so only the 5 enum fields need `FromStr` coercion; `id`,
//! `role`, `subrole`, `group`, and `partition` pass through untouched.
//!
//! Divergence from the historical hand-written
//! `memberships::http_client::get`: the historical function additionally
//! exposed `enabled`, `softwarestatus`, `subtype`, `nid`, `nid_start`,
//! and `nid_end` filters (17 total incl. `group`). Per the design spec's
//! Phase 3 interface contract, this wrapper method exposes only the 10
//! filters listed there (`id`, `type_`, `state`, `flag`, `role`,
//! `subrole`, `group`, `partition`, `arch`, `class`); the omitted
//! generated parameters are passed as `None`. There are no in-crate
//! callers of the old function, so this narrowing has no known
//! downstream impact.

use std::str::FromStr;

use crate::{
  error::Error,
  hsm::{
    generated::types::{
      DoMembershipsGetArch, DoMembershipsGetClass, DoMembershipsGetFlag,
      DoMembershipsGetState, DoMembershipsGetType, Membership100 as Membership,
    },
    wrapper::{gen_client, map_err},
  },
  http::retry_on_5xx,
  Ochami,
};

impl Ochami {
  /// `GET /hsm/v2/memberships` — with optional filters.
  ///
  /// `type_`, `state`, `flag`, `arch`, and `class` are coerced from
  /// `&str` into their generated per-endpoint enum types; unparsable
  /// values are dropped (treated as "no filter"), matching the
  /// log-and-drop coercion pattern used across this migration.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  #[allow(clippy::too_many_arguments)]
  pub async fn hsm_memberships_get(
    &self,
    token: &str,
    id: Option<&str>,
    type_: Option<&str>,
    state: Option<&str>,
    flag: Option<&str>,
    role: Option<&str>,
    subrole: Option<&str>,
    group: Option<&str>,
    partition: Option<&str>,
    arch: Option<&str>,
    class: Option<&str>,
  ) -> Result<Vec<Membership>, Error> {
    retry_on_5xx(|| async {
      let gc = gen_client(self, token)?;
      let arch = arch.and_then(|s| DoMembershipsGetArch::from_str(s).ok());
      let class = class.and_then(|s| DoMembershipsGetClass::from_str(s).ok());
      let flag = flag.and_then(|s| DoMembershipsGetFlag::from_str(s).ok());
      let state = state.and_then(|s| DoMembershipsGetState::from_str(s).ok());
      let type_ = type_.and_then(|s| DoMembershipsGetType::from_str(s).ok());

      match gc
        .do_memberships_get(
          arch, class, None, flag, group, id, None, None, None, partition,
          role, None, state, subrole, None, type_,
        )
        .await
      {
        Ok(rv) => Ok(rv.into_inner()),
        Err(e) => Err(map_err(e).await),
      }
    })
    .await
  }

  /// `GET /hsm/v2/memberships/{xname}` — fetch the membership of a
  /// single component by xname.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_memberships_get_xname(
    &self,
    token: &str,
    xname: &str,
  ) -> Result<Membership, Error> {
    retry_on_5xx(|| async {
      let gc = gen_client(self, token)?;
      match gc.do_membership_get(xname).await {
        Ok(rv) => Ok(rv.into_inner()),
        Err(e) => Err(map_err(e).await),
      }
    })
    .await
  }
}
