//! Wrapper for `/Inventory/HardwareByFRU`. Replaces
//! `src/hsm/inventory/hardware_by_fru/http_client.rs`.
//!
//! Per design spec § Section 3 / Section 4 (Phase 3 detail),
//! `hsm/inventory/types.rs` (2767 LOC) **stays hand-rolled** — it is not
//! slim-re-exported from progenitor's typify emission (Redfish-source
//! types like `HWInventoryByFRU` don't map cleanly). Consequently all
//! methods in this file route on raw `reqwest`, deserializing directly
//! into the hand-rolled `HWInventoryByFRU` type imported from
//! `crate::hsm::inventory::types` (confirmed present at
//! `src/hsm/inventory/types.rs:1182`, matching the vendored OpenAPI
//! spec's `HWInventory.1.0.0_HWInventoryByFRU` definition).
//!
//! Signature corrections vs. the brief:
//! - The brief sketched `fru_id_vec: Option<&[&str]>` (multi-value
//!   `fruid`) for [`Self::hsm_inventory_hardware_by_fru_get`]. Per the
//!   HSM OpenAPI spec (`src/hsm/ochami_api_docs.yaml`,
//!   `/Inventory/HardwareByFRU` GET), the `fruid` query param's
//!   description does **not** carry the "can be specified multiple
//!   times" note that `type` (`compTypeParam`) does — unlike
//!   `/Inventory/Hardware`'s `id`/`type` params (both explicitly
//!   multi-value, per Task 8's `hsm_inventory_hardware_get`), this
//!   endpoint's `fruid` is single-value only, matching the existing
//!   `hardware_by_fru/http_client.rs::get`'s `fruid: Option<&str>`
//!   parameter exactly. Only `type` is promoted to `type_vec`.
//! - The brief omitted `manufacturer`/`partnumber`/`serialnumber`,
//!   which the real endpoint (and existing `http_client.rs::get`)
//!   supports; they are included here.
//! - `hsm_inventory_hardware_by_fru_get_one` fetches
//!   `/Inventory/HardwareByFRU/{fruid}` (not `/Inventory/Hardware/{fruid}`
//!   as the existing, buggy `http_client.rs::get_one` does — that path
//!   would hit the HWInventoryByLocation endpoint, a location xname
//!   lookup, not a FRU lookup; corrected here per the OpenAPI spec).
//! - `hsm_inventory_hardware_by_fru_delete_one` returns
//!   [`HsmActionResponse`] (`Response_1.0.0` per spec), matching Task
//!   8's convention for mutation responses, rather than the existing
//!   `http_client.rs`'s untyped `serde_json::Value`.
//!
//! `hsm_inventory_hardware_by_fru_delete_all` is intentionally **not**
//! ported. The hand-written `http_client.rs::delete_all` is an
//! unconditional "delete every HWInventoryByFRU entry" call (a footgun;
//! the OpenAPI spec even tags it `cli_danger$This will delete all FRUs
//! for HSM, continue?`) with zero in-repo callers. Per spec § Section 1
//! (v6 fix) and the same rationale applied in Task 8, it is dropped
//! from the wrapper's public surface.

use crate::{
  error::Error,
  hsm::{inventory::types::HWInventoryByFRU, types::HsmActionResponse},
  http::{
    build_client, handle_action_response, handle_json_response, retry_on_5xx,
  },
  Ochami,
};

impl Ochami {
  /// `GET /hsm/v2/Inventory/HardwareByFRU` — list HWInventoryByFRU
  /// entries (flat array), filtered by the supplied criteria.
  ///
  /// `type` is multi-value per the HSM OpenAPI spec ("Can be specified
  /// multiple times for selecting entries of multiple types"); the
  /// `type_vec` slice is emitted as repeated `?type=` params. `fruid`,
  /// `manufacturer`, `partnumber`, and `serialnumber` are single-value
  /// filters (matches `hsm/inventory/hardware_by_fru/http_client.rs::get`'s
  /// existing filter set).
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  #[allow(clippy::too_many_arguments)]
  pub async fn hsm_inventory_hardware_by_fru_get(
    &self,
    token: &str,
    fruid: Option<&str>,
    type_vec: Option<&[&str]>,
    manufacturer: Option<&str>,
    partnumber: Option<&str>,
    serialnumber: Option<&str>,
  ) -> Result<Vec<HWInventoryByFRU>, Error> {
    retry_on_5xx(|| async {
      let client = build_client(self.root_cert())?;
      let url = format!("{}/hsm/v2/Inventory/HardwareByFRU", self.base_url());

      let mut query_params: Vec<(&str, Option<&str>)> = vec![
        ("fruid", fruid),
        ("manufacturer", manufacturer),
        ("partnumber", partnumber),
        ("serialnumber", serialnumber),
      ];
      if let Some(types) = type_vec {
        query_params.extend(types.iter().map(|t| ("type", Some(*t))));
      }

      let response = client
        .get(&url)
        .query(&query_params)
        .bearer_auth(token)
        .send()
        .await?;
      handle_json_response::<Vec<HWInventoryByFRU>>(response).await
    })
    .await
  }

  /// `GET /hsm/v2/Inventory/HardwareByFRU/{fruid}` — fetch one
  /// HWInventoryByFRU entry.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_inventory_hardware_by_fru_get_one(
    &self,
    token: &str,
    fruid: &str,
  ) -> Result<HWInventoryByFRU, Error> {
    retry_on_5xx(|| async {
      let client = build_client(self.root_cert())?;
      let url = format!(
        "{}/hsm/v2/Inventory/HardwareByFRU/{}",
        self.base_url(),
        fruid
      );

      let response = client.get(&url).bearer_auth(token).send().await?;
      handle_json_response::<HWInventoryByFRU>(response).await
    })
    .await
  }

  /// `DELETE /hsm/v2/Inventory/HardwareByFRU/{fruid}` — delete one
  /// HWInventoryByFRU entry.
  ///
  /// Per the HSM OpenAPI spec, this fails if the FRU ID is currently
  /// associated with a HWInventoryByLocation entry.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_inventory_hardware_by_fru_delete_one(
    &self,
    token: &str,
    fruid: &str,
  ) -> Result<HsmActionResponse, Error> {
    let client = build_client(self.root_cert())?;
    let url = format!(
      "{}/hsm/v2/Inventory/HardwareByFRU/{}",
      self.base_url(),
      fruid
    );

    let response = client.delete(&url).bearer_auth(token).send().await?;
    handle_action_response(response).await
  }
}
