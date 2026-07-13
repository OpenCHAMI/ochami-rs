//! Wrapper for `/Inventory/Hardware`. Replaces
//! `src/hsm/inventory/hardware/http_client.rs`.
//!
//! Per design spec § Section 3, `hsm/inventory/types.rs` (2767 LOC)
//! **stays hand-rolled** — it is not slim-re-exported from progenitor's
//! typify emission (Redfish-source types like `HWInvByLocNode` /
//! `HWInvByLocProcessor` don't map cleanly). Consequently all methods in
//! this file route on raw `reqwest`, deserializing directly into the
//! hand-rolled types imported from `crate::hsm::inventory::types`
//! (note: the brief's stated path
//! `crate::hsm::inventory::hardware::types` does not exist in this repo
//! layout — the actual hand-rolled tree lives at
//! `src/hsm/inventory/types.rs`; confirmed by the 2767-LOC match cited
//! in the design spec).
//!
//! `hsm_inventory_hardware_delete_all` is intentionally **not**
//! ported. The hand-written `http_client.rs::delete_all` is an
//! unconditional "delete every hardware-inventory-by-location entry"
//! call (a footgun) with zero in-repo callers. Per spec § Section 1 (v6
//! fix), it is dropped from the wrapper's public surface.
//!
//! Signature note: the brief's sketched interface for
//! `hsm_inventory_hardware_get`/`_get_query`/`_post` listed filter
//! params (`state`/`flag`/`role`/`subrole`/`arch`/`class`/`hierarchy`)
//! and a body type (`&HWInventory`) that do not match either the real
//! HSM OpenAPI spec (`src/hsm/ochami_api_docs.yaml`) or the existing
//! `http_client.rs` implementation. Those params/types belong to
//! `/State/Components`, not `/Inventory/Hardware`. This file follows
//! `http_client.rs` + the vendored OpenAPI spec as source of truth
//! instead (per task instructions); see `hsm_inventory_hardware_get`,
//! `hsm_inventory_hardware_get_query`, and `hsm_inventory_hardware_post`
//! doc comments below for the corrected shapes.

use crate::{
  Ochami,
  error::Error,
  hsm::{
    inventory::types::{
      HWInventory, HWInventoryByLocation, HWInventoryByLocationList,
    },
    types::HsmActionResponse,
  },
  http::{
    build_client, handle_action_response, handle_json_response, retry_on_5xx,
  },
};

impl Ochami {
  /// `GET /hsm/v2/Inventory/Hardware` — list hardware-inventory-by-location
  /// entries, filtered by the supplied criteria.
  ///
  /// `id` and `type` are multi-value per the HSM OpenAPI spec ("Can be
  /// specified multiple times for selecting entries with multiple
  /// specific xnames/types"); `id_vec`/`type_vec` slices are emitted as
  /// repeated `?id=`/`?type=` params. `manufacturer`, `partnumber`,
  /// `serialnumber`, and `fruid` are the remaining filters the real API
  /// supports for this endpoint (matches
  /// `hsm/inventory/hardware/http_client.rs::get`'s existing filter set;
  /// `state`/`flag`/`role`/`subrole`/`arch`/`class`/`format` are
  /// `/State/Components`-only filters and do not apply here).
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  #[allow(clippy::too_many_arguments)]
  pub async fn hsm_inventory_hardware_get(
    &self,
    token: &str,
    id_vec: Option<&[&str]>,
    type_vec: Option<&[&str]>,
    manufacturer: Option<&str>,
    partnumber: Option<&str>,
    serialnumber: Option<&str>,
    fruid: Option<&str>,
  ) -> Result<Vec<HWInventoryByLocation>, Error> {
    retry_on_5xx(|| async {
      let client = build_client(self.root_cert())?;
      let url = format!("{}/hsm/v2/Inventory/Hardware", self.base_url());

      let mut query_params: Vec<(&str, Option<&str>)> = vec![
        ("manufacturer", manufacturer),
        ("partnumber", partnumber),
        ("serialnumber", serialnumber),
        ("fruid", fruid),
      ];
      if let Some(ids) = id_vec {
        query_params.extend(ids.iter().map(|id| ("id", Some(*id))));
      }
      if let Some(types) = type_vec {
        query_params.extend(types.iter().map(|t| ("type", Some(*t))));
      }

      let response = client
        .get(&url)
        .query(&query_params)
        .bearer_auth(token)
        .send()
        .await?;
      handle_json_response::<Vec<HWInventoryByLocation>>(response).await
    })
    .await
  }

  /// `GET /hsm/v2/Inventory/Hardware/{xname}` — fetch one
  /// hardware-inventory-by-location entry.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_inventory_hardware_get_one(
    &self,
    token: &str,
    xname: &str,
  ) -> Result<HWInventoryByLocation, Error> {
    retry_on_5xx(|| async {
      let client = build_client(self.root_cert())?;
      let url =
        format!("{}/hsm/v2/Inventory/Hardware/{}", self.base_url(), xname);

      let response = client.get(&url).bearer_auth(token).send().await?;
      handle_json_response::<HWInventoryByLocation>(response).await
    })
    .await
  }

  /// `GET /hsm/v2/Inventory/Hardware/Query/{xname}` — query hardware
  /// inventory starting at `xname`, returning a nested `HWInventory`
  /// tree (FRU data included for populated locations).
  ///
  /// Real query params per the HSM OpenAPI spec: `type`, `children`,
  /// `parents`, `partition`, `format` (matches the existing
  /// `http_client.rs::get_query`'s param set exactly). The brief's
  /// sketched `hierarchy: Option<bool>` param does not exist on this
  /// endpoint — the actual boolean toggles are `children`/`parents` —
  /// and the response shape is a single `HWInventory` object, not
  /// `Vec<HWInventoryByLocation>` (the array-typed
  /// `/Inventory/Hardware` list endpoint is
  /// [`Self::hsm_inventory_hardware_get`]).
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  #[allow(clippy::too_many_arguments)]
  pub async fn hsm_inventory_hardware_get_query(
    &self,
    token: &str,
    xname: &str,
    r#type: Option<&str>,
    children: Option<bool>,
    parents: Option<bool>,
    partition: Option<&str>,
    format: Option<&str>,
  ) -> Result<HWInventory, Error> {
    retry_on_5xx(|| async {
      let client = build_client(self.root_cert())?;
      let url = format!(
        "{}/hsm/v2/Inventory/Hardware/Query/{}",
        self.base_url(),
        xname
      );

      let children_str = children.map(|value| value.to_string());
      let parents_str = parents.map(|value| value.to_string());

      let query_params: Vec<(&str, Option<&str>)> = vec![
        ("type", r#type),
        ("children", children_str.as_deref()),
        ("parents", parents_str.as_deref()),
        ("partition", partition),
        ("format", format),
      ];

      let response = client
        .get(&url)
        .query(&query_params)
        .bearer_auth(token)
        .send()
        .await?;
      handle_json_response::<HWInventory>(response).await
    })
    .await
  }

  /// `POST /hsm/v2/Inventory/Hardware` — create/update hardware
  /// inventory entries.
  ///
  /// Body is `{"Hardware": [...]}`
  /// (`HWInventoryByLocationList`), matching both the real OpenAPI
  /// request schema and the existing `http_client.rs::post`'s param
  /// type. The brief's sketched `hardware: &HWInventory` body type is
  /// the nested Cabinets/Chassis/... tree returned by
  /// [`Self::hsm_inventory_hardware_get_query`], not the flat
  /// `{"Hardware": [...]}` array this endpoint expects.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_inventory_hardware_post(
    &self,
    token: &str,
    hardware: &HWInventoryByLocationList,
  ) -> Result<HsmActionResponse, Error> {
    let client = build_client(self.root_cert())?;
    let url = format!("{}/hsm/v2/Inventory/Hardware", self.base_url());

    let response = client
      .post(&url)
      .json(hardware)
      .bearer_auth(token)
      .send()
      .await?;
    handle_action_response(response).await
  }
}
