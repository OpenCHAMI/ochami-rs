//! Wrapper for `/Inventory/EthernetInterfaces`. Replaces
//! `src/hsm/inventory/ethernet_interfaces/http_client.rs`.
//!
//! Per Task 0's output-reference doc (§ L), the slim-re-export plan for
//! this resource family is viable: the generated `CompEthInterface100`
//! family of types are drop-in for the hand-rolled
//! `ComponentEthernetInterface`/`IpAddressMapping` types this module
//! previously used. All methods below route through the generated
//! progenitor client (`gen_client` bakes bearer auth into
//! `default_headers`, so no per-request `.bearer_auth()` is needed).
//!
//! **Type-name deviation from the design spec's/brief's guess**: there
//! is no separate `EthernetInterfaceCreate` type — POST and GET share
//! the single `CompEthInterface100` schema, aliased here as
//! `EthernetInterface`. Likewise there is no separate
//! `EthernetIPAddress`/`EthernetIPAddressPatch` — the generated
//! `CompEthInterface100IpAddressMapping` /
//! `CompEthInterface100IpAddressMappingPatch` are used (aliased to
//! those brief-guessed names for readability).
//!
//! **Return-type deviations from the design spec's/brief's guess**:
//! - `hsm_inventory_ethernet_interface_post` returns
//!   `ResourceUri100` (singular, not a `Vec`, and not
//!   `HsmActionResponse`) — the generated `do_comp_eth_interface_post_v2`
//!   has a 201 response typed as a single `ResourceUri100`.
//! - `hsm_inventory_ethernet_interface_post_ip_address` similarly
//!   returns a single `ResourceUri100`, not `HsmActionResponse`.
//! - `hsm_inventory_ethernet_interface_patch` and
//!   `hsm_inventory_ethernet_interface_patch_ip_address` return `()`,
//!   not `HsmActionResponse` and not the full updated resource — the
//!   generated `do_comp_eth_interface_patch_v2`/
//!   `do_comp_eth_interface_ip_address_patch_v2` have a 200 response
//!   with no body schema (matching the empty-body PATCH/PUT pattern
//!   already documented in `component.rs`'s
//!   `hsm_component_put`/`hsm_component_post`).
//! - `hsm_inventory_ethernet_interface_delete_one` and
//!   `hsm_inventory_ethernet_interface_delete_ip_address` do return
//!   `HsmActionResponse` (`Response100`), matching the design spec's
//!   guess.
//!
//! **Signature corrections vs. the brief**:
//! - `hsm_inventory_ethernet_interface_get` gains `older_than` /
//!   `newer_than` filter params not listed in the brief's sketch — the
//!   generated `do_comp_eth_interfaces_get_v2` (and the pre-existing
//!   hand-written `http_client.rs::get`) both support them. `type_` is
//!   a plain `Option<&str>` here (no generated per-endpoint enum type
//!   exists for this filter, unlike e.g. `RedfishEndpoints`' `type_`).
//! - `hsm_inventory_ethernet_interface_patch_ip_address` and
//!   `hsm_inventory_ethernet_interface_delete_ip_address` take
//!   `ip_address: &str` (the literal IP address string used as the URL
//!   path segment and lookup key), not an opaque `ip_id` as the brief's
//!   sketch implied.
//! - `hsm_inventory_ethernet_interface_delete_ip_address` drops the
//!   unused `_group_label` argument that the old hand-written
//!   `http_client.rs::delete_ip_address` carried (dead parameter,
//!   never referenced in its body) — per spec § Latent Bugs.
//!
//! `hsm_inventory_ethernet_interface_delete_all` (mapping to the
//! generated `do_comp_eth_interface_delete_all_v2`, an unconditional
//! "delete every component Ethernet interface" call) is intentionally
//! **not** ported, matching the same footgun-avoidance precedent set in
//! `inventory_hardware.rs`/`inventory_hardware_by_fru.rs`.

use crate::{
  error::Error,
  hsm::{
    generated::types::{
      CompEthInterface100 as EthernetInterface,
      CompEthInterface100IpAddressMapping as EthernetIpAddress,
      CompEthInterface100IpAddressMappingPatch as EthernetIpAddressPatch,
      CompEthInterface100Patch as EthernetInterfacePatch, ResourceUri100,
    },
    types::HsmActionResponse,
    wrapper::run,
  },
  http::retry_on_5xx,
  Ochami,
};

impl Ochami {
  /// `GET /hsm/v2/Inventory/EthernetInterfaces` — fetch all component
  /// Ethernet interfaces, filtered by the supplied criteria.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  #[allow(clippy::too_many_arguments)]
  pub async fn hsm_inventory_ethernet_interface_get(
    &self,
    token: &str,
    mac_address: Option<&str>,
    ip_address: Option<&str>,
    network: Option<&str>,
    component_id: Option<&str>,
    type_: Option<&str>,
    older_than: Option<&str>,
    newer_than: Option<&str>,
  ) -> Result<Vec<EthernetInterface>, Error> {
    retry_on_5xx(|| async {
      run(self, token, |c| async move {
        c.do_comp_eth_interfaces_get_v2(
          component_id,
          ip_address,
          mac_address,
          network,
          newer_than,
          older_than,
          type_,
        )
        .await
      })
      .await
    })
    .await
  }

  /// `GET /hsm/v2/Inventory/EthernetInterfaces/{ethInterfaceID}` —
  /// fetch a single component Ethernet interface by ID.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_inventory_ethernet_interface_get_one(
    &self,
    token: &str,
    id: &str,
  ) -> Result<EthernetInterface, Error> {
    retry_on_5xx(|| async {
      run(self, token, |c| async move {
        c.do_comp_eth_interface_get_v2(id).await
      })
      .await
    })
    .await
  }

  /// `POST /hsm/v2/Inventory/EthernetInterfaces` — create a new
  /// component Ethernet interface.
  ///
  /// Note: the generated `do_comp_eth_interface_post_v2` returns a
  /// single `ResourceUri100` (the created interface's resource URI),
  /// not `HsmActionResponse` — see the module-level doc.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_inventory_ethernet_interface_post(
    &self,
    token: &str,
    ei: &EthernetInterface,
  ) -> Result<ResourceUri100, Error> {
    run(self, token, |c| async move {
      c.do_comp_eth_interface_post_v2(ei).await
    })
    .await
  }

  /// `PATCH /hsm/v2/Inventory/EthernetInterfaces/{ethInterfaceID}` —
  /// partially update a component Ethernet interface's metadata
  /// (description, IP addresses, and/or component ID).
  ///
  /// Note: the generated `do_comp_eth_interface_patch_v2` has a 200
  /// response with no body — this returns `()`, not
  /// `HsmActionResponse` — see the module-level doc.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_inventory_ethernet_interface_patch(
    &self,
    token: &str,
    id: &str,
    ei: &EthernetInterfacePatch,
  ) -> Result<(), Error> {
    run(self, token, |c| async move {
      c.do_comp_eth_interface_patch_v2(id, ei).await
    })
    .await
  }

  /// `DELETE /hsm/v2/Inventory/EthernetInterfaces/{ethInterfaceID}` —
  /// delete a single component Ethernet interface.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_inventory_ethernet_interface_delete_one(
    &self,
    token: &str,
    id: &str,
  ) -> Result<HsmActionResponse, Error> {
    run(self, token, |c| async move {
      c.do_comp_eth_interface_delete_v2(id).await
    })
    .await
  }

  /// `GET /hsm/v2/Inventory/EthernetInterfaces/{ethInterfaceID}/IPAddresses`
  /// — retrieve all IP addresses of a component Ethernet interface.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_inventory_ethernet_interface_get_ip_addresses(
    &self,
    token: &str,
    id: &str,
  ) -> Result<Vec<EthernetIpAddress>, Error> {
    retry_on_5xx(|| async {
      run(self, token, |c| async move {
        c.do_comp_eth_interface_ip_addresses_get_v2(id).await
      })
      .await
    })
    .await
  }

  /// `POST /hsm/v2/Inventory/EthernetInterfaces/{ethInterfaceID}/IPAddresses`
  /// — create a new IP address mapping in a component Ethernet
  /// interface.
  ///
  /// Note: the generated `do_comp_eth_interface_ip_addresses_post_v2`
  /// returns a single `ResourceUri100`, not `HsmActionResponse` — see
  /// the module-level doc.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_inventory_ethernet_interface_post_ip_address(
    &self,
    token: &str,
    id: &str,
    ip: &EthernetIpAddress,
  ) -> Result<ResourceUri100, Error> {
    run(self, token, |c| async move {
      c.do_comp_eth_interface_ip_addresses_post_v2(id, ip).await
    })
    .await
  }

  /// `PATCH /hsm/v2/Inventory/EthernetInterfaces/{ethInterfaceID}/IPAddresses/{ipAddress}`
  /// — partially update (the network of) an IP address mapping in a
  /// component Ethernet interface.
  ///
  /// Note: the generated `do_comp_eth_interface_ip_address_patch_v2`
  /// has a 200 response with no body — this returns `()`, not
  /// `HsmActionResponse` — see the module-level doc. `ip_address` is
  /// the literal IP address string (used as both the URL path segment
  /// and the lookup key), not an opaque ID.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_inventory_ethernet_interface_patch_ip_address(
    &self,
    token: &str,
    id: &str,
    ip_address: &str,
    patch: &EthernetIpAddressPatch,
  ) -> Result<(), Error> {
    run(self, token, |c| async move {
      c.do_comp_eth_interface_ip_address_patch_v2(id, ip_address, patch)
        .await
    })
    .await
  }

  /// `DELETE /hsm/v2/Inventory/EthernetInterfaces/{ethInterfaceID}/IPAddresses/{ipAddress}`
  /// — delete an IP address mapping from a component Ethernet
  /// interface.
  ///
  /// `ip_address` is the literal IP address string, not an opaque ID.
  /// This drops the unused `_group_label` argument the old
  /// hand-written `http_client.rs::delete_ip_address` carried (dead
  /// parameter, never referenced in its body) — see the module-level
  /// doc.
  ///
  /// # Errors
  /// Returns [`Error`] on transport, HSM 4xx/5xx, or deserialization
  /// failure.
  pub async fn hsm_inventory_ethernet_interface_delete_ip_address(
    &self,
    token: &str,
    id: &str,
    ip_address: &str,
  ) -> Result<HsmActionResponse, Error> {
    run(self, token, |c| async move {
      c.do_comp_eth_interface_ip_address_delete_v2(id, ip_address)
        .await
    })
    .await
  }
}
