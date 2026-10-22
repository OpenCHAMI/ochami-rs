use manta_backend_dispatcher::{
  interfaces::{
    authentication::AuthenticationTrait,
    delete_configurations_and_data_related::DeleteConfigurationsAndDataRelatedTrait,
  },
  types::pcs::power_status::types::PowerStatusAll as FrontEndPowerStatusAll,
};
use std::{collections::HashMap, pin::Pin};

use futures_io::AsyncBufRead;
use hostlist_parser::parse;
use manta_backend_dispatcher::{
  error::Error,
  interfaces::{
    apply_hw_cluster_pin::ApplyHwClusterPin,
    apply_sat_file::SatTrait,
    apply_session::ApplySessionTrait,
    bos::{ClusterSessionTrait, ClusterTemplateTrait},
    bss::BootParametersTrait,
    cfs::CfsTrait,
    console::ConsoleTrait,
    hsm::{
      component::ComponentTrait,
      component_ethernet_interface::ComponentEthernetInterfaceTrait,
      group::GroupTrait, hardware_inventory::HardwareInventory,
      redfish_endpoint::RedfishEndpointTrait,
    },
    ims::{GetImagesAndDetailsTrait, ImsTrait},
    migrate_backup::MigrateBackupTrait,
    migrate_restore::MigrateRestoreTrait,
    pcs::PCSTrait,
  },
  types::{
    bss::BootParameters,
    hsm::inventory::{
      ComponentEthernetInterface, RedfishEndpoint, RedfishEndpointArray,
    },
    Component, ComponentArrayPostArray as FrontEndComponentArrayPostArray,
    Group as FrontEndGroup, HWInventory as FrontEndHWInventory,
    HWInventoryByLocationList as FrontEndHWInventoryByLocationList,
    HsmActionResponse, NodeMetadataArray, NodeSummary as FrontEndNodeSummary,
  },
};
use regex::Regex;
use serde_json::Value;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::authentication;
use crate::hsm;
use crate::hsm::generated::types::{
  CompEthInterface100IpAddressMapping as GenIpAddressMapping,
  CompEthInterface100Patch as GenCompEthInterfacePatch, XNameRw100,
};

/// Top-level client for the OCHAMI backend (HSM, BSS, PCS).
///
/// Holds the API base URL and the root certificate used to build the
/// underlying `reqwest::Client` for each request.
#[derive(Debug, Clone)]
pub struct Ochami {
  base_url: String,
  root_cert: Vec<u8>,
}

impl Ochami {
  /// Constructs an [`Ochami`] client from an API base URL and a PEM-encoded root certificate.
  pub fn new(base_url: &str, root_cert: &[u8]) -> Self {
    Self {
      base_url: base_url.to_string(),
      root_cert: root_cert.to_vec(),
    }
  }

  pub(crate) fn base_url(&self) -> &str {
    &self.base_url
  }

  pub(crate) fn root_cert(&self) -> &[u8] {
    &self.root_cert
  }
}

impl GroupTrait for Ochami {
  // Returns a list of all groups available to the user
  // NOTE: We don't have user/Group mapping in OCHAMI (neither OpenFGA not Keycloak user roles)
  // therefore all groups are available to all users
  async fn get_group_available(
    &self,
    token: &str,
  ) -> Result<Vec<FrontEndGroup>, Error> {
    // Get all groups
    let hsm_group_backend_vec = self
      .hsm_group_get(token, None, None)
      .await
      .map_err(Error::from)?;

    // Slim re-export makes Group100 == crate::hsm::group::types::Group with
    // From<Group> for FrontEndGroup impls.
    let hsm_group_vec: Vec<FrontEndGroup> =
      hsm_group_backend_vec.into_iter().map(Into::into).collect();

    Ok(hsm_group_vec)
  }

  // Returns a list of all groups name available to the user
  // NOTE: We don't have user/Group mapping in OCHAMI (neither OpenFGA not Keycloak user roles)
  // therefore all groups are available to all users
  async fn get_group_name_available(
    &self,
    token: &str,
  ) -> Result<Vec<String>, Error> {
    let hsm_group_vec = self
      .hsm_group_get(token, None, None)
      .await
      .map_err(Error::from)?;

    Ok(
      hsm_group_vec
        .iter()
        .map(|hsm_group| hsm_group.label.as_str().to_string())
        .collect(),
    )
  }

  async fn add_group(
    &self,
    auth_token: &str,
    hsm_group: FrontEndGroup,
  ) -> Result<FrontEndGroup, Error> {
    let generated: hsm::group::types::Group = hsm_group.clone().into();
    let created_uris = self
      .hsm_group_post(auth_token, &generated)
      .await
      .map_err(Error::from)?;

    log::info!("Group created: {:?}", created_uris);

    Ok(hsm_group)
  }

  // FIXME: rename function to 'get_hsm_group_members'
  async fn get_member_vec_from_group_name_vec(
    &self,
    auth_token: &str,
    hsm_group_name_vec: &[String],
  ) -> Result<Vec<String>, Error> {
    hsm::group::utils::get_member_vec_from_hsm_name_vec_2(
      self,
      auth_token,
      hsm_group_name_vec,
    )
    .await
    .map_err(Error::from)
  }

  async fn get_group_map_and_filter_by_group_vec(
    &self,
    auth_token: &str,
    hsm_name_vec: &[&str],
  ) -> Result<HashMap<String, Vec<String>>, Error> {
    hsm::group::utils::get_hsm_map_and_filter_by_hsm_name_vec(
      self,
      auth_token,
      hsm_name_vec,
    )
    .await
    .map_err(Error::from)
  }

  async fn get_group_map_and_filter_by_member_vec(
    &self,
    auth_token: &str,
    hsm_name_vec: &[&str],
  ) -> Result<HashMap<String, Vec<String>>, Error> {
    hsm::group::utils::get_hsm_group_map_and_filter_by_hsm_group_member_vec(
      self,
      auth_token,
      hsm_name_vec,
    )
    .await
    .map_err(Error::from)
  }

  async fn get_group(
    &self,
    auth_token: &str,
    hsm_name: &str,
  ) -> Result<FrontEndGroup, Error> {
    // Get one HSM group
    let hsm_group_backend = self
      .hsm_group_get_one(auth_token, hsm_name)
      .await
      .map_err(Error::from)?;

    Ok(hsm_group_backend.into())
  }

  async fn get_groups(
    &self,
    auth_token: &str,
    hsm_name_vec: Option<&[String]>,
  ) -> Result<Vec<FrontEndGroup>, Error> {
    // Get all HSM groups
    let names_owned: Option<Vec<&str>> =
      hsm_name_vec.map(|v| v.iter().map(|s| s.as_str()).collect());
    let hsm_group_backend_vec = self
      .hsm_group_get(auth_token, names_owned.as_deref(), None)
      .await
      .map_err(Error::from)?;

    let hsm_group_vec: Vec<FrontEndGroup> =
      hsm_group_backend_vec.into_iter().map(Into::into).collect();

    Ok(hsm_group_vec)
  }

  async fn delete_group(
    &self,
    auth_token: &str,
    hsm_group_name: &str,
  ) -> Result<HsmActionResponse, Error> {
    let resp = self
      .hsm_group_delete_one(auth_token, hsm_group_name)
      .await
      .map_err(Error::from)?;
    Ok(resp.into())
  }

  async fn get_group_map_and_filter_by_group_name_vec(
    &self,
    shasta_token: &str,
    hsm_name_vec: &[&str],
  ) -> Result<HashMap<String, Vec<String>>, Error> {
    hsm::group::utils::get_hsm_map_and_filter_by_hsm_name_vec(
      self,
      shasta_token,
      hsm_name_vec,
    )
    .await
    .map_err(Error::from)
  }

  async fn post_member(
    &self,
    auth_token: &str,
    group_label: &str,
    xname: &str,
  ) -> Result<HsmActionResponse, Error> {
    // hsm_group_post_member returns Vec<ResourceUri100>; synthesize a
    // successful HsmActionResponse for trait parity. The trait signature
    // requires a `HsmActionResponse`, so we stringify the created URIs
    // into the `message` field. Changing the trait to return the raw URI
    // list is a follow-up on the front-end side.
    let uris = self
      .hsm_group_post_member(auth_token, group_label, xname)
      .await
      .map_err(Error::from)?;
    Ok(HsmActionResponse {
      code: "0".to_string(),
      message: format!(
        "created: {}",
        uris
          .iter()
          .map(|u| u.resource_uri.as_deref().unwrap_or(""))
          .collect::<Vec<_>>()
          .join(","),
      ),
    })
  }

  async fn add_members_to_group(
    &self,
    auth_token: &str,
    group_label: &str,
    new_members: &[&str],
  ) -> Result<Vec<String>, Error> {
    let mut sol: Vec<String> = Vec::new();

    for new_member in new_members {
      sol = hsm::group::utils::add_member(
        self,
        auth_token,
        group_label,
        new_member,
      )
      .await
      .map_err(Error::from)?;
    }

    Ok(sol)
  }

  async fn delete_member_from_group(
    &self,
    auth_token: &str,
    group_label: &str,
    xname: &str,
  ) -> Result<(), Error> {
    self
      .hsm_group_delete_member(auth_token, group_label, xname)
      .await
      .map(|_| ())
      .map_err(Error::from)
  }

  async fn update_group_members(
    &self,
    auth_token: &str,
    group_name: &str,
    members_to_remove: &[&str],
    members_to_add: &[&str],
  ) -> Result<(), Error> {
    hsm::group::utils::update_hsm_group_members(
      self,
      auth_token,
      group_name,
      members_to_remove,
      members_to_add,
    )
    .await
    .map_err(Error::from)
  }

  async fn migrate_group_members(
    &self,
    shasta_token: &str,
    target_hsm_group_name: &str,
    parent_hsm_group_name: &str,
    new_target_hsm_members: &[&str],
    dryrun: bool,
  ) -> Result<(Vec<String>, Vec<String>), Error> {
    hsm::group::utils::migrate_hsm_members(
      self,
      shasta_token,
      target_hsm_group_name,
      parent_hsm_group_name,
      new_target_hsm_members,
      dryrun,
    )
    .await
    .map_err(Error::from)
  }
}

impl HardwareInventory for Ochami {
  async fn get_inventory_hardware(
    &self,
    auth_token: &str,
    xname: &str,
  ) -> Result<FrontEndNodeSummary, Error> {
    // OCHAMI returns the inventory as a flat `HWInventoryByLocation` list
    // (csm-rs's swagger does too, but its client extracts the single
    // `/Nodes/0` entry inside the HTTP layer). Mirror that here: take the
    // first inventory entry, map it through ochami's `NodeSummary` From,
    // then into the dispatcher's `NodeSummary`. Returns `Default` if the
    // response is empty — matches the original "always return something"
    // shape of the pre-typed implementation.
    let ids = [xname];
    self
      .hsm_inventory_hardware_get(
        auth_token,
        Some(&ids),
        None,
        None,
        None,
        None,
        None,
      )
      .await
      .map(|inventory_vec| {
        inventory_vec
          .into_iter()
          .find_map(|entry| match entry {
            crate::hsm::inventory::types::HWInventoryByLocation::HWInvByLocNode(node) => {
              Some(node)
            }
            _ => None,
          })
          .map(crate::hsm::inventory::types::NodeSummary::from)
          .map(Into::into)
          .unwrap_or_default()
      })
      .map_err(Error::from)
  }

  async fn get_inventory_hardware_query(
    &self,
    auth_token: &str,
    xname: &str,
    r#type: Option<&str>,
    children: Option<bool>,
    parents: Option<bool>,
    partition: Option<&str>,
    format: Option<&str>,
  ) -> Result<FrontEndHWInventory, Error> {
    let hw = self
      .hsm_inventory_hardware_get_query(
        auth_token, xname, r#type, children, parents, partition, format,
      )
      .await
      .map_err(Error::from)?;
    Ok(hw.into())
  }

  async fn post_inventory_hardware(
    &self,
    auth_token: &str,
    hardware: FrontEndHWInventoryByLocationList,
  ) -> Result<HsmActionResponse, Error> {
    let hw_internal: crate::hsm::inventory::types::HWInventoryByLocationList =
      hardware.into();
    let resp = self
      .hsm_inventory_hardware_post(auth_token, &hw_internal)
      .await
      .map_err(Error::from)?;
    Ok(resp.into())
  }
}

impl ComponentTrait for Ochami {
  async fn get_all_nodes(
    &self,
    auth_token: &str,
    nid_only: Option<&str>,
  ) -> Result<NodeMetadataArray, Error> {
    let arr = self
      .hsm_component_get_all_nodes(auth_token, nid_only)
      .await
      .map_err(Error::from)?;
    Ok(arr.into())
  }

  async fn get_node_metadata_available(
    &self,
    auth_token: &str,
  ) -> Result<Vec<Component>, Error> {
    self
      .get_all_nodes(auth_token, Some("true"))
      .await
      .map(|c| c.components.unwrap_or_default())
  }

  async fn get(
    &self,
    auth_token: &str,
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
  ) -> Result<NodeMetadataArray, Error> {
    let arr = self
      .hsm_component_get(
        auth_token,
        id,
        r#type,
        state,
        flag,
        role,
        subrole,
        enabled,
        software_status,
        subtype,
        arch,
        class,
        nid,
        nid_start,
        nid_end,
        partition,
        group,
        state_only,
        flag_only,
        role_only,
        nid_only,
      )
      .await
      .map_err(Error::from)?;
    Ok(arr.into())
  }

  async fn post_nodes(
    &self,
    auth_token: &str,
    component: FrontEndComponentArrayPostArray,
  ) -> Result<(), Error> {
    // Slim re-export: hsm::component::types::ComponentArrayPostArray ==
    // generated ComponentArrayPostArray, with From<FrontEnd..> impl.
    let generated: hsm::component::types::ComponentArrayPostArray =
      component.into();
    self
      .hsm_component_post(auth_token, generated)
      .await
      .map_err(Error::from)
  }

  async fn delete_node(
    &self,
    auth_token: &str,
    id: &str,
  ) -> Result<HsmActionResponse, Error> {
    let resp = self
      .hsm_component_delete_one(auth_token, id)
      .await
      .map_err(Error::from)?;
    Ok(resp.into())
  }

  /// Get list of xnames from NIDs
  /// The list of NIDs can be:
  ///     - comma separated list of NIDs (eg: nid000001,nid000002,nid000003)
  ///     - regex (eg: nid00000.*)
  ///     - hostlist (eg: nid0000[01-15])
  async fn nid_to_xname(
    &self,
    shasta_token: &str,
    user_input_nid: &str,
    is_regex: bool,
  ) -> Result<Vec<String>, Error> {
    if is_regex {
      log::debug!("Regex found, getting xnames from NIDs");
      // Get list of regex
      let regex_vec: Vec<Regex> = user_input_nid
        .split(",")
        .map(|regex_str| Regex::new(regex_str.trim()))
        .collect::<Result<Vec<Regex>, regex::Error>>()
        .map_err(|e| Error::Message(e.to_string()))?;

      // Get all HSM components (list of xnames + nids)
      let hsm_component_vec = self
        .hsm_component_get_all_nodes(shasta_token, Some("true"))
        .await
        .map_err(Error::from)?
        .components;

      let mut xname_vec: Vec<String> = vec![];

      // Get list of xnames the user is asking for
      for hsm_component in hsm_component_vec {
        let nid_long =
          format!("nid{:06}", &hsm_component.nid.expect("No NID found"));
        for regex in &regex_vec {
          if regex.is_match(&nid_long) {
            log::debug!(
              "Nid '{}' IS included in regex '{}'",
              nid_long,
              regex.as_str()
            );
            xname_vec.push(hsm_component.id.clone().expect("No XName found").0);
          }
        }
      }

      Ok(xname_vec)
    } else {
      log::debug!(
        "No regex found, getting xnames from list of NIDs or NIDs hostlist"
      );
      let nid_hostlist_expanded_vec_rslt = parse(user_input_nid);

      let nid_hostlist_expanded_vec = match nid_hostlist_expanded_vec_rslt {
        Ok(xname_requested_vec) => xname_requested_vec,
        Err(e) => {
          return Err(Error::Message(format!(
            "Could not parse list of nodes as a hostlist. Reason:\n{}Exit",
            e
          )));
        }
      };

      log::debug!("hostlist: {}", user_input_nid);
      log::debug!("hostlist expanded: {:?}", nid_hostlist_expanded_vec);

      let nid_short = nid_hostlist_expanded_vec
        .iter()
        .map(|nid_long| {
          nid_long
            .strip_prefix("nid")
            .unwrap_or_else(|| {
              panic!("Nid '{}' not valid, 'nid' prefix missing", nid_long)
            })
            .trim_start_matches("0")
        })
        .collect::<Vec<&str>>()
        .join(",");

      log::debug!("short NID list: {}", nid_short);

      let hsm_components = self
        .hsm_component_get(
          shasta_token,
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
          Some(&nid_short),
          None,
          None,
          None,
          None,
          None,
          None,
          None,
          Some("true"),
        )
        .await
        .map_err(Error::from)?;

      // Get list of xnames from HSM components. Components with a
      // missing `id` are skip-logged rather than panicking (a
      // malformed HSM record should not crash the caller).
      let xname_vec: Vec<String> = hsm_components
        .components
        .iter()
        .filter_map(|component| match component.id.as_ref() {
          Some(id) => Some(id.0.clone()),
          None => {
            log::warn!("HSM component with no id skipped: {component:?}");
            None
          }
        })
        .collect();

      log::debug!("xname list:\n{:#?}", xname_vec);

      Ok(xname_vec)
    }
  }
}

impl PCSTrait for Ochami {
  async fn power_status(
    &self,
    auth_token: &str,
    nodes: &[String],
    //xname_vec_opt: Option<&[&str]>,
    power_state_filter: Option<&str>,
    management_state_filter: Option<&str>,
  ) -> Result<FrontEndPowerStatusAll, Error> {
    // Convert &[String] to Vec<&str> and wrap in Some
    let nodes_str: Vec<&str> = nodes.iter().map(|s| s.as_str()).collect();
    let nodes_opt = Some(nodes_str.as_slice());

    self
      .pcs_power_status_post(
        auth_token,
        nodes_opt,
        power_state_filter,
        management_state_filter,
      )
      .await
      .map(|status| {
        log::debug!("power_status returned: {status:?}");
        status.into()
      })
      .map_err(Error::from)
  }
}

impl BootParametersTrait for Ochami {
  async fn get_all_bootparameters(
    &self,
    auth_token: &str,
  ) -> Result<Vec<BootParameters>, Error> {
    let boot_parameter_vec = self
      .bss_bootparameters_get_all(auth_token)
      .await
      .map_err(Error::from)?;

    Ok(boot_parameter_vec.into_iter().map(Into::into).collect())
  }

  async fn get_bootparameters(
    &self,
    auth_token: &str,
    hosts: &[String],
  ) -> Result<Vec<BootParameters>, Error> {
    let host_refs: Vec<&str> = hosts.iter().map(String::as_str).collect();
    let filter = if host_refs.is_empty() {
      None
    } else {
      Some(host_refs.as_slice())
    };

    let boot_parameter_vec = self
      .bss_bootparameters_get(auth_token, filter)
      .await
      .map_err(Error::from)?;

    Ok(boot_parameter_vec.into_iter().map(Into::into).collect())
  }

  async fn add_bootparameters(
    &self,
    auth_token: &str,
    boot_parameters: &BootParameters,
  ) -> Result<(), Error> {
    let bp: crate::bss::types::BootParameters = boot_parameters.clone().into();
    self
      .bss_bootparameters_post(auth_token, &bp)
      .await
      .map_err(Error::from)
  }

  async fn update_bootparameters(
    &self,
    auth_token: &str,
    boot_parameter: &BootParameters,
  ) -> Result<(), Error> {
    let bp: crate::bss::types::BootParameters = boot_parameter.clone().into();
    self
      .bss_bootparameters_patch(auth_token, &bp)
      .await
      .map_err(Error::from)
  }

  async fn delete_bootparameters(
    &self,
    auth_token: &str,
    boot_parameter: &BootParameters,
  ) -> Result<String, Error> {
    let bp: crate::bss::types::BootParameters = boot_parameter.clone().into();
    // The wrapper method returns `()` (matches BSS's actual 200-empty
    // response); the trait signature returns `String` (a legacy from
    // the old hand-written client which returned raw response text).
    // Synthesize an empty string on success — no known callers consume
    // this value for anything but log output.
    self
      .bss_bootparameters_delete(auth_token, &bp)
      .await
      .map(|()| String::new())
      .map_err(Error::from)
  }
}

impl RedfishEndpointTrait for Ochami {
  async fn get_all_redfish_endpoints(
    &self,
    auth_token: &str,
  ) -> Result<RedfishEndpointArray, Error> {
    let gen_arr = self
      .hsm_inventory_redfish_endpoint_get(
        auth_token, None, None, None, None, None, None, None,
      )
      .await
      .map_err(Error::from)?;
    Ok(gen_arr.into())
  }

  async fn get_redfish_endpoints(
    &self,
    auth_token: &str,
    id: Option<&str>,
    fqdn: Option<&str>,
    r#type: Option<&str>,
    uuid: Option<&str>,
    macaddr: Option<&str>,
    ip_address: Option<&str>,
    last_status: Option<&str>,
  ) -> Result<RedfishEndpointArray, Error> {
    let gen_arr = self
      .hsm_inventory_redfish_endpoint_get(
        auth_token,
        id,
        fqdn,
        r#type,
        uuid,
        macaddr,
        ip_address,
        last_status,
      )
      .await
      .map_err(Error::from)?;
    Ok(gen_arr.into())
  }

  async fn add_redfish_endpoint(
    &self,
    auth_token: &str,
    redfish_endpoint: &RedfishEndpointArray,
  ) -> Result<(), Error> {
    // The generated `do_redfish_endpoints_post` takes a single endpoint
    // per call (mirrors the real HSM OpenAPI schema — the hand-written
    // http_client.rs::post's array-payload was overly permissive). Iterate
    // over the front-end array and POST each endpoint individually.
    let eps = redfish_endpoint.redfish_endpoints.as_deref().unwrap_or(&[]);
    for ep in eps {
      let gen_ep: hsm::inventory::redfish_endpoint::types::RedfishEndpoint =
        ep.clone().into();
      self
        .hsm_inventory_redfish_endpoint_post(auth_token, &gen_ep)
        .await
        .map_err(Error::from)?;
    }
    Ok(())
  }

  async fn update_redfish_endpoint(
    &self,
    auth_token: &str,
    redfish_endpoint: &RedfishEndpoint,
  ) -> Result<(), Error> {
    let xname = redfish_endpoint.id.clone();
    let gen_ep: hsm::inventory::redfish_endpoint::types::RedfishEndpoint =
      redfish_endpoint.clone().into();
    self
      .hsm_inventory_redfish_endpoint_put(auth_token, xname.as_str(), &gen_ep)
      .await
      .map_err(Error::from)?;
    Ok(())
  }

  async fn delete_redfish_endpoint(
    &self,
    auth_token: &str,
    id: &str,
  ) -> Result<Value, Error> {
    let resp = self
      .hsm_inventory_redfish_endpoint_delete_one(auth_token, id)
      .await
      .map_err(Error::from)?;
    serde_json::to_value(resp).map_err(Error::from)
  }
}

impl ComponentEthernetInterfaceTrait for Ochami {
  async fn get_all_component_ethernet_interfaces(
    &self,
    auth_token: &str,
  ) -> Result<Vec<ComponentEthernetInterface>, Error> {
    let gen_vec = self
      .hsm_inventory_ethernet_interface_get(
        auth_token, None, None, None, None, None, None, None,
      )
      .await
      .map_err(Error::from)?;
    Ok(gen_vec.into_iter().map(Into::into).collect())
  }

  async fn get_component_ethernet_interface(
    &self,
    auth_token: &str,
    eth_interface_id: &str,
  ) -> Result<ComponentEthernetInterface, Error> {
    let gen_ei = self
      .hsm_inventory_ethernet_interface_get_one(auth_token, eth_interface_id)
      .await
      .map_err(Error::from)?;
    Ok(gen_ei.into())
  }

  async fn add_component_ethernet_interface(
    &self,
    auth_token: &str,
    ethernet_interface: &ComponentEthernetInterface,
  ) -> Result<(), Error> {
    let gen_ei: hsm::inventory::ethernet_interfaces::types::ComponentEthernetInterface =
      ethernet_interface.clone().into();
    self
      .hsm_inventory_ethernet_interface_post(auth_token, &gen_ei)
      .await
      .map(|_uri| ())
      .map_err(Error::from)
  }

  async fn update_component_ethernet_interface(
    &self,
    auth_token: &str,
    //base_url: &str,
    //root_cert: &[u8],
    //shasta_token: &str,
    //shasta_base_url: &str,
    //shasta_root_cert: &[u8],
    eth_interface_id: &str,
    description: Option<&str>,
    ip_address_mapping: (&str, &str),
  ) -> Result<Value, Error> {
    // Build the CompEthInterface100Patch inline. The old
    // http_client::patch bundled description + a single IP mapping into a
    // ComponentEthernetInterface body — the generated PATCH schema
    // (`CompEthInterface100Patch`, aliased via
    // `hsm::inventory::ethernet_interfaces::types::ComponentEthernetInterfacePatch`)
    // carries description, ip_addresses, and component_id (no
    // MACAddress / LastUpdate on PATCH).
    let (ip_address, network) = ip_address_mapping;
    let patch = GenCompEthInterfacePatch {
      component_id: Some(XNameRw100(eth_interface_id.to_string())),
      description: description.map(|s| s.to_string()),
      ip_addresses: vec![GenIpAddressMapping {
        ip_address: ip_address.to_string(),
        network: Some(network.to_string()),
      }],
    };
    self
      .hsm_inventory_ethernet_interface_patch(
        auth_token,
        eth_interface_id,
        &patch,
      )
      .await
      .map_err(Error::from)?;
    Ok(Value::Null)
  }

  async fn delete_component_ethernet_interface(
    &self,
    auth_token: &str,
    //base_url: &str,
    //root_cert: &[u8],
    eth_interface_id: &str,
  ) -> Result<Value, Error> {
    let resp = self
      .hsm_inventory_ethernet_interface_delete_one(auth_token, eth_interface_id)
      .await
      .map_err(Error::from)?;
    serde_json::to_value(resp).map_err(Error::from)
  }
}

impl AuthenticationTrait for Ochami {
  async fn get_api_token(
    &self,
    _uername: &str,
    _password: &str,
  ) -> Result<String, Error> {
    authentication::get_api_token().map_err(|_e| {
      Error::Message(
        "environment variable 'ACCESS_TOKEN' not found".to_string(),
      )
    })
  }

  /// Pass-through for now: returns `subject_token` unchanged after a
  /// non-empty check. OpenCHAMI does not accept the CSCS token, so the
  /// real exchange (via TokenSmith) still has to be implemented here.
  async fn exchange_api_token(
    &self,
    subject_token: &str,
  ) -> Result<String, Error> {
    authentication::validate_api_token(subject_token).map_err(|_e| {
      Error::Unauthorized("Authentication token is empty".to_string())
    })?;
    Ok(subject_token.to_string())
  }
}

impl CfsTrait for Ochami {
  type T = Pin<Box<dyn AsyncBufRead + Send>>;
}

impl SatTrait for Ochami {}

impl ApplyHwClusterPin for Ochami {}

impl ImsTrait for Ochami {}

impl ApplySessionTrait for Ochami {}

impl MigrateRestoreTrait for Ochami {}

impl MigrateBackupTrait for Ochami {}

impl GetImagesAndDetailsTrait for Ochami {}

impl ClusterSessionTrait for Ochami {}

impl ClusterTemplateTrait for Ochami {}

impl DeleteConfigurationsAndDataRelatedTrait for Ochami {}

impl ConsoleTrait for Ochami {
  type T = Box<dyn AsyncWrite + Unpin + Send>;
  type U = Box<dyn AsyncRead + Unpin + Send>;
}
