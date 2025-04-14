use std::{collections::HashMap, pin::Pin};

use backend_dispatcher::{
    contracts::BackendTrait,
    error::Error,
    interfaces::{
        apply_hw_cluster_pin::ApplyHwClusterPin,
        apply_session::ApplySessionTrait,
        bos::ClusterSessionTrait,
        bss::BootParametersTrait,
        cfs::CfsTrait,
        get_bos_session_templates::GetTemplatesTrait,
        get_images_and_details::GetImagesAndDetailsTrait,
        hsm::{
            component::ComponentTrait, group::GroupTrait, hardware_inventory::HardwareInventory,
            redfish_endpoint::RedfishEndpointTrait,
        },
        ims::ImsTrait,
        migrate_backup::MigrateBackupTrait,
        migrate_restore::MigrateRestoreTrait,
        pcs::PCSTrait,
        sat::SatTrait,
    },
    types::{
        hsm::inventory::{RedfishEndpoint, RedfishEndpointArray},
        BootParameters, Component, ComponentArrayPostArray as FrontEndComponentArrayPostArray,
        Group as FrontEndGroup, HWInventoryByLocationList as FrontEndHWInventoryByLocationList,
        NodeMetadataArray,
    },
};
use futures::AsyncBufRead;
use hostlist_parser::parse;
use regex::Regex;
use serde_json::Value;

use crate::{authentication, bss};
use crate::{
    hsm::{self, component::types::ComponentArrayPostArray, group::types::Group},
    pcs,
};

#[derive(Clone)]
pub struct Ochami {
    base_url: String,
    root_cert: Vec<u8>,
}

impl Ochami {
    pub fn new(base_url: &str, root_cert: &[u8]) -> Self {
        Self {
            base_url: base_url.to_string(),
            root_cert: root_cert.to_vec(),
        }
    }
}

impl GroupTrait for Ochami {
    // Returns a list of all groups available to the user
    // NOTE: We don't have user/Group mapping in OCHAMI (neither OpenFGA not Keycloak user roles)
    // therefore all groups are available to all users
    async fn get_group_available(&self, auth_token: &str) -> Result<Vec<FrontEndGroup>, Error> {
        self.get_all_groups(auth_token).await
    }

    // Returns a list of all groups name available to the user
    // NOTE: We don't have user/Group mapping in OCHAMI (neither OpenFGA not Keycloak user roles)
    // therefore all groups are available to all users
    async fn get_group_name_available(&self, token: &str) -> Result<Vec<String>, Error> {
        let hsm_group_vec_rslt = self.get_all_groups(token).await;

        hsm_group_vec_rslt.and_then(|hsm_group_vec| {
            Ok(hsm_group_vec
                .iter()
                .map(|hsm_group| hsm_group.label.clone())
                .collect())
        })
    }

    async fn add_group(
        &self,
        auth_token: &str,
        hsm_group: FrontEndGroup,
    ) -> Result<FrontEndGroup, Error> {
        let hsm_group_backend = hsm::group::http_client::post(
            &self.base_url,
            auth_token,
            &self.root_cert,
            hsm_group.clone().into(),
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))?;

        log::info!("Group created: {}", hsm_group_backend);

        Ok(hsm_group)
    }

    // FIXME: rename function to 'get_hsm_group_members'
    async fn get_member_vec_from_group_name_vec(
        &self,
        auth_token: &str,
        hsm_group_name_vec: Vec<String>,
    ) -> Result<Vec<String>, Error> {
        hsm::group::utils::get_member_vec_from_hsm_name_vec_2(
            auth_token,
            &self.base_url,
            &self.root_cert,
            hsm_group_name_vec,
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
    }

    async fn get_group_map_and_filter_by_group_vec(
        &self,
        auth_token: &str,
        hsm_name_vec: Vec<&str>,
    ) -> Result<HashMap<String, Vec<String>>, Error> {
        hsm::group::utils::get_hsm_map_and_filter_by_hsm_name_vec(
            auth_token,
            &self.base_url,
            &self.root_cert,
            hsm_name_vec,
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
    }

    async fn get_group_map_and_filter_by_member_vec(
        &self,
        auth_token: &str,
        hsm_name_vec: &[&str],
    ) -> Result<HashMap<String, Vec<String>>, Error> {
        hsm::group::utils::get_hsm_group_map_and_filter_by_hsm_group_member_vec(
            auth_token,
            &self.base_url,
            &self.root_cert,
            hsm_name_vec,
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
    }

    async fn get_all_groups(&self, auth_token: &str) -> Result<Vec<FrontEndGroup>, Error> {
        // Get all HSM groups
        let hsm_group_backend_vec =
            hsm::group::http_client::get(&self.base_url, auth_token, &self.root_cert, None, None)
                .await
                .map_err(|e| Error::Message(e.to_string()))?;

        // Convert from HsmGroup (silla) to HsmGroup (infra)
        let hsm_group_vec = hsm_group_backend_vec.into_iter().map(Group::into).collect();

        Ok(hsm_group_vec)
    }

    async fn get_group(&self, auth_token: &str, hsm_name: &str) -> Result<FrontEndGroup, Error> {
        // Get all HSM groups
        let hsm_group_backend =
            hsm::group::http_client::get_one(&self.base_url, auth_token, &self.root_cert, hsm_name)
                .await
                .map_err(|e| Error::Message(e.to_string()))?;

        let hsm_group: FrontEndGroup = hsm_group_backend.into();

        Ok(hsm_group)
    }

    async fn get_groups(
        &self,
        auth_token: &str,
        hsm_name_vec: Option<&[&str]>,
    ) -> Result<Vec<FrontEndGroup>, Error> {
        // Get all HSM groups
        let hsm_group_backend_vec = hsm::group::http_client::get(
            &self.base_url,
            auth_token,
            &self.root_cert,
            hsm_name_vec,
            None,
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))?;

        // Convert from HsmGroup (silla) to HsmGroup (infra)
        let mut hsm_group_vec = Vec::new();
        for hsm_group_backend in hsm_group_backend_vec {
            let hsm_group: FrontEndGroup = hsm_group_backend.into();
            hsm_group_vec.push(hsm_group);
        }

        Ok(hsm_group_vec)
    }

    async fn delete_group(&self, auth_token: &str, hsm_group_name: &str) -> Result<Value, Error> {
        hsm::group::http_client::delete_one(
            &self.base_url,
            auth_token,
            &self.root_cert,
            hsm_group_name,
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
    }

    async fn get_hsm_map_and_filter_by_hsm_name_vec(
        &self,
        shasta_token: &str,
        hsm_name_vec: Vec<&str>,
    ) -> Result<HashMap<String, Vec<String>>, Error> {
        hsm::group::utils::get_hsm_map_and_filter_by_hsm_name_vec(
            shasta_token,
            &self.base_url,
            &self.root_cert,
            hsm_name_vec,
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
    }

    async fn post_member(
        &self,
        auth_token: &str,
        group_label: &str,
        xname: &str,
    ) -> Result<Value, Error> {
        let member = hsm::group::types::Member {
            id: Some(xname.to_string()),
        };

        hsm::group::http_client::post_member(
            auth_token,
            &self.base_url,
            &self.root_cert,
            group_label,
            member,
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
    }

    async fn add_members_to_group(
        &self,
        auth_token: &str,
        group_label: &str,
        new_members: Vec<&str>,
    ) -> Result<Vec<String>, Error> {
        let mut sol: Vec<String> = Vec::new();

        for new_member in new_members {
            sol = hsm::group::utils::add_member(
                auth_token,
                &self.base_url,
                &self.root_cert,
                group_label,
                new_member,
            )
            .await
            .map_err(|e| Error::Message(e.to_string()))?;
        }

        Ok(sol)
    }

    async fn delete_member_from_group(
        &self,
        auth_token: &str,
        group_label: &str,
        xname: &str,
    ) -> Result<(), Error> {
        hsm::group::http_client::delete_member(
            &self.base_url,
            auth_token,
            &self.root_cert,
            group_label,
            xname,
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
    }

    async fn update_group_members(
        &self,
        auth_token: &str,
        group_name: &str,
        members_to_remove: &Vec<String>,
        members_to_add: &Vec<String>,
    ) -> Result<(), Error> {
        hsm::group::utils::update_hsm_group_members(
            auth_token,
            &self.base_url,
            &self.root_cert,
            group_name,
            members_to_remove,
            members_to_add,
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
    }

    async fn migrate_group_members(
        &self,
        shasta_token: &str,
        target_hsm_group_name: &str,
        parent_hsm_group_name: &str,
        new_target_hsm_members: Vec<&str>,
    ) -> Result<(Vec<String>, Vec<String>), Error> {
        hsm::group::utils::migrate_hsm_members(
            shasta_token,
            &self.base_url,
            &self.root_cert,
            target_hsm_group_name,
            parent_hsm_group_name,
            new_target_hsm_members,
            true,
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
    }
}

impl HardwareInventory for Ochami {
    async fn get_inventory_hardware(&self, auth_token: &str, xname: &str) -> Result<Value, Error> {
        hsm::inventory::hardware::http_client::get(
            &auth_token,
            &self.base_url,
            &self.root_cert,
            Some(xname),
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
        .and_then(|hw_inventory| {
            serde_json::to_value(hw_inventory).map_err(|e| Error::Message(e.to_string()))
        })
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
    ) -> Result<Value, Error> {
        hsm::inventory::hardware::http_client::get_query(
            &auth_token,
            &self.base_url,
            &self.root_cert,
            xname,
            r#type,
            children,
            parents,
            partition,
            format,
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
    }

    async fn post_inventory_hardware(
        &self,
        auth_token: &str,
        hardware: FrontEndHWInventoryByLocationList,
    ) -> Result<Value, Error> {
        hsm::inventory::hardware::http_client::post(
            auth_token,
            &self.base_url,
            &self.root_cert,
            hardware.into(),
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ComponentTrait for Ochami {
    async fn get_all_nodes(
        &self,
        auth_token: &str,
        nid_only: Option<&str>,
    ) -> Result<NodeMetadataArray, Error> {
        hsm::component::http_client::get(
            &self.base_url,
            &self.root_cert,
            auth_token,
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
        .map(|c| c.into())
        .map_err(|e| Error::Message(e.to_string()))
    }

    async fn get_node_metadata_available(&self, auth_token: &str) -> Result<Vec<Component>, Error> {
        self.get_all_nodes(auth_token, Some("true"))
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
        hsm::component::http_client::get(
            &self.base_url,
            &self.root_cert,
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
        .map(|c| c.into())
        .map_err(|e| Error::Message(e.to_string()))
    }

    async fn post_nodes(
        &self,
        auth_token: &str,
        component: FrontEndComponentArrayPostArray,
    ) -> Result<(), Error> {
        let component_backend: ComponentArrayPostArray = component.into();

        hsm::component::http_client::post(
            auth_token,
            &self.base_url,
            &self.root_cert,
            component_backend,
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
    }

    async fn delete_node(&self, auth_token: &str, id: &str) -> Result<Value, Error> {
        hsm::component::http_client::delete_one(auth_token, &self.base_url, &self.root_cert, id)
            .await
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl PCSTrait for Ochami {
    async fn power_on_sync(&self, auth_token: &str, nodes: &[String]) -> Result<Value, Error> {
        let operation = "on";

        pcs::transitions::http_client::post_block(
            &self.base_url,
            auth_token,
            &self.root_cert,
            operation,
            &nodes.to_vec(),
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
    }

    async fn power_off_sync(
        &self,
        auth_token: &str,
        nodes: &[String],
        force: bool,
    ) -> Result<serde_json::Value, Error> {
        let operation = if force { "force-off" } else { "soft-off" };

        pcs::transitions::http_client::post_block(
            &self.base_url,
            auth_token,
            &self.root_cert,
            operation,
            &nodes.to_vec(),
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
    }

    async fn power_reset_sync(
        &self,
        auth_token: &str,
        nodes: &[String],
        force: bool,
    ) -> Result<serde_json::Value, Error> {
        let operation = if force {
            "hard-restart"
        } else {
            "soft-restart"
        };

        pcs::transitions::http_client::post_block(
            &self.base_url,
            auth_token,
            &self.root_cert,
            operation,
            &nodes.to_vec(),
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
    }
}

impl BootParametersTrait for Ochami {
    async fn get_bootparameters(
        &self,
        auth_token: &str,
        hosts: &[String],
    ) -> Result<Vec<BootParameters>, Error> {
        let hosts = if hosts.is_empty() {
            None
        } else {
            Some(hosts.to_vec())
        };

        let boot_parameter_vec =
            bss::http_client::get(&self.base_url, auth_token, &self.root_cert, &hosts)
                .await
                .map_err(|e| Error::Message(e.to_string()))?;

        let boot_parameter_infra_vec = boot_parameter_vec
            .into_iter()
            .map(|boot_parameter| boot_parameter.into())
            .collect();

        Ok(boot_parameter_infra_vec)
    }

    async fn add_bootparameters(
        &self,
        auth_token: &str,
        boot_parameters: &BootParameters,
    ) -> Result<(), Error> {
        bss::http_client::post(
            &self.base_url,
            auth_token,
            &self.root_cert,
            boot_parameters.clone().into(),
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
        .map(|boot_parameter| boot_parameter.into())
    }

    async fn update_bootparameters(
        &self,
        auth_token: &str,
        boot_parameter: &BootParameters,
    ) -> Result<(), Error> {
        bss::http_client::patch(
            &self.base_url,
            auth_token,
            &self.root_cert,
            &boot_parameter.clone().into(),
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
    }

    async fn delete_bootparameters(
        &self,
        auth_token: &str,
        boot_parameter: &BootParameters,
    ) -> Result<String, Error> {
        bss::http_client::delete(
            &self.base_url,
            auth_token,
            &self.root_cert,
            &boot_parameter.clone().into(),
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
    }
}

impl RedfishEndpointTrait for Ochami {
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
        hsm::inventory::redfish_endpoint::http_client::get(
            auth_token,
            &self.base_url,
            &self.root_cert,
            id,
            fqdn,
            r#type,
            uuid,
            macaddr,
            ip_address,
            last_status,
        )
        .await
        .map(|re| re.into())
        .map_err(|e| Error::Message(e.to_string()))
    }

    async fn add_redfish_endpoint(
        &self,
        auth_token: &str,
        redfish_endpoint: &RedfishEndpoint,
    ) -> Result<(), Error> {
        hsm::inventory::redfish_endpoint::http_client::post(
            auth_token,
            &self.base_url,
            &self.root_cert,
            redfish_endpoint.clone().into(),
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))?;

        Ok(())
    }

    async fn update_redfish_endpoint(
        &self,
        auth_token: &str,
        redfish_endpoint: &RedfishEndpoint,
    ) -> Result<(), Error> {
        hsm::inventory::redfish_endpoint::http_client::put(
            auth_token,
            &self.base_url,
            &self.root_cert,
            redfish_endpoint.id.as_str(),
            redfish_endpoint.clone().into(),
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))?;

        Ok(())
    }

    async fn delete_redfish_endpoint(&self, auth_token: &str, id: &str) -> Result<Value, Error> {
        hsm::inventory::redfish_endpoint::http_client::delete_one(
            auth_token,
            &self.base_url,
            &self.root_cert,
            id,
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
    }
}

impl BackendTrait for Ochami {
    fn test_backend_trait(&self) -> String {
        println!("in silla backend");
        "in silla backend".to_string()
    }

    async fn get_api_token(&self, _site_name: &str) -> Result<String, Error> {
        authentication::get_api_token().await.map_err(|_e| {
            Error::Message("environment variable 'ACCESS_TOKEN' not found".to_string())
        })
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
            let hsm_component_vec = hsm::component::http_client::get_all_nodes(
                &self.base_url,
                shasta_token,
                &self.root_cert,
                Some("true"),
            )
            .await
            .map_err(|e| Error::Message(e.to_string()))?
            .components
            .unwrap_or_default();

            let mut xname_vec: Vec<String> = vec![];

            // Get list of xnames the user is asking for
            for hsm_component in hsm_component_vec {
                let nid_long = format!("nid{:06}", &hsm_component.nid.expect("No NID found"));
                for regex in &regex_vec {
                    if regex.is_match(&nid_long) {
                        log::debug!(
                            "Nid '{}' IS included in regex '{}'",
                            nid_long,
                            regex.as_str()
                        );
                        xname_vec.push(hsm_component.id.clone().expect("No XName found"));
                    }
                }
            }

            return Ok(xname_vec);
        } else {
            log::debug!("No regex found, getting xnames from list of NIDs or NIDs hostlist");
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
                        .expect(
                            format!("Nid '{}' not valid, 'nid' prefix missing", nid_long).as_str(),
                        )
                        .trim_start_matches("0")
                })
                .collect::<Vec<&str>>()
                .join(",");

            log::debug!("short NID list: {}", nid_short);

            let hsm_components = hsm::component::http_client::get(
                &self.base_url,
                &self.root_cert,
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
            .map_err(|e| Error::Message(e.to_string()))?;

            // Get list of xnames from HSM components
            let xname_vec: Vec<String> = hsm_components
                .components
                .unwrap_or_default()
                .iter()
                .map(|component| component.id.clone().unwrap())
                .collect();

            log::debug!("xname list:\n{:#?}", xname_vec);

            return Ok(xname_vec);
        };
    }
}

impl CfsTrait for Ochami {
    type T = Pin<Box<dyn AsyncBufRead + Send>>;

    /* async fn get_session_logs_stream(
        &self,
        _shasta_token: &str,
        _site_name: &str,
        _cfs_session_name: &str,
        _k8s_api_url: &str,
        _k8s: &K8sDetails,
    ) -> Result<Pin<Box<dyn AsyncBufRead>>, Error> {
        Err::<Pin<Box<dyn AsyncBufRead>>, backend_dispatcher::error::Error>(Error::Message(
            "Get logs stream command not implemented for this backend".to_string(),
        ))
    }

    async fn get_session_logs_stream_by_xname(
        &self,
        _auth_token: &str,
        _site_name: &str,
        _xname: &str,
        _k8s_api_url: &str,
        _k8s: &K8sDetails,
    ) -> Result<Pin<Box<dyn AsyncBufRead>>, Error> {
        Err::<Pin<Box<dyn AsyncBufRead>>, backend_dispatcher::error::Error>(Error::Message(
            "Get logs stream command not implemented for this backend".to_string(),
        ))
    }

    async fn post_session(
        &self,
        _shasta_token: &str,
        _shasta_base_url: &str,
        _shasta_root_cert: &[u8],
        _session: &CfsSessionPostRequest,
    ) -> Result<CfsSessionGetResponse, Error> {
        Err(Error::Message(
            "Post session command not implemented for this backend".to_string(),
        ))
    }

    async fn get_sessions(
        &self,
        _shasta_token: &str,
        _shasta_base_url: &str,
        _shasta_root_cert: &[u8],
        _session_name_opt: Option<&String>,
        _limit_opt: Option<u8>,
        _after_id_opt: Option<String>,
        _min_age_opt: Option<String>,
        _max_age_opt: Option<String>,
        _status_opt: Option<String>,
        _name_contains_opt: Option<String>,
        _is_succeded_opt: Option<bool>,
        _tags_opt: Option<String>,
    ) -> Result<Vec<backend_dispatcher::types::cfs::CfsSessionGetResponse>, Error> {
        Err(Error::Message(
            "Get sessions command not implemented for this backend".to_string(),
        ))
    }

    async fn get_and_filter_sessions(
        &self,
        _shasta_token: &str,
        _shasta_base_url: &str,
        _shasta_root_cert: &[u8],
        _hsm_group_name_vec_opt: Option<Vec<String>>,
        _xname_vec_opt: Option<Vec<&str>>,
        _min_age_opt: Option<&String>,
        _max_age_opt: Option<&String>,
        _status_opt: Option<&String>,
        _cfs_session_name_opt: Option<&String>,
        _limit_number_opt: Option<&u8>,
        _is_succeded_opt: Option<bool>,
    ) -> Result<Vec<backend_dispatcher::types::cfs::CfsSessionGetResponse>, Error> {
        Err(Error::Message(
            "Get and filter sessions command not implemented for this backend".to_string(),
        ))
    }

    async fn get_sessions_by_xname(
        &self,
        _shasta_token: &str,
        _shasta_base_url: &str,
        _shasta_root_cert: &[u8],
        _xname_vec: &[&str],
        _limit_opt: Option<u8>,
        _after_id_opt: Option<String>,
        _min_age_opt: Option<String>,
        _max_age_opt: Option<String>,
        _status_opt: Option<String>,
        _name_contains_opt: Option<String>,
        _is_succeded_opt: Option<bool>,
        _tags_opt: Option<String>,
    ) -> Result<Vec<backend_dispatcher::types::cfs::CfsSessionGetResponse>, Error> {
        Err(Error::Message(
            "Get sessions command not implemented for this backend".to_string(),
        ))
    }

    async fn get_configuration(
        &self,
        _auth_token: &str,
        _shasta_base_url: &str,
        _shasta_root_cert: &[u8],
        _cfs_configuration_name: Option<&String>,
    ) -> Result<Vec<CfsConfigurationResponse>, Error> {
        Err(Error::Message(
            "Get configuration command not implemented for this backend".to_string(),
        ))
    }

    async fn get_and_filter_configuration(
        &self,
        _shasta_token: &str,
        _shasta_base_url: &str,
        _shasta_root_cert: &[u8],
        _configuration_name: Option<&str>,
        _configuration_name_pattern: Option<&str>,
        _hsm_group_name_vec: &[String],
        _limit_number_opt: Option<&u8>,
    ) -> Result<Vec<CfsConfigurationResponse>, Error> {
        Err(Error::Message(
            "Get and filter configuration command not implemented for this backend".to_string(),
        ))
    }

    async fn get_configuration_layer_details(
        &self,
        _shasta_root_cert: &[u8],
        _gitea_base_url: &str,
        _gitea_token: &str,
        _layer: Layer,
        _site_name: &str,
    ) -> Result<LayerDetails, Error> {
        Err(Error::Message(
            "Get configuration layer details command not implemented for this backend".to_string(),
        ))
    }

    async fn create_configuration_from_repos(
        &self,
        _gitea_token: &str,
        _gitea_base_url: &str,
        _shasta_root_cert: &[u8],
        _repo_name_vec: Vec<String>,
        _local_git_commit_vec: Vec<String>,
        _playbook_file_name_opt: Option<&String>,
    ) -> Result<CfsConfigurationRequest, Error> {
        Err(Error::Message(
            "Create from repos command not implemented for this backend".to_string(),
        ))
    }

    async fn put_configuration(
        &self,
        _shasta_token: &str,
        _shasta_base_url: &str,
        _shasta_root_cert: &[u8],
        _configuration: &CfsConfigurationRequest,
        _configuration_name: &str,
    ) -> Result<CfsConfigurationResponse, Error> {
        Err(Error::Message(
            "Put configuration layer details command not implemented for this backend".to_string(),
        ))
    }

    async fn update_runtime_configuration(
        &self,
        _shasta_token: &str,
        _shasta_base_url: &str,
        _shasta_root_cert: &[u8],
        _xnames: Vec<String>,
        _desired_configuration: &str,
        _enabled: bool,
    ) -> Result<(), Error> {
        Err(Error::Message(
            "Update runtime configuration command not implemented for this backend".to_string(),
        ))
    }

    // Get all CFS sessions, IMS images and BOS sessiontemplates related to a CFS configuration
    async fn get_derivatives(
        &self,
        _shasta_token: &str,
        _shasta_base_url: &str,
        _shasta_root_cert: &[u8],
        _configuration_name: &str,
    ) -> Result<
        (
            Option<Vec<CfsSessionGetResponse>>,
            Option<Vec<BosSessionTemplate>>,
            Option<Vec<Image>>,
        ),
        Error,
    > {
        Err(Error::Message(
            "Get configuration derivatives command not implemented for this backend".to_string(),
        ))
    } */
}

impl SatTrait for Ochami {
    /* async fn apply_sat_file(
        &self,
        _shasta_token: &str,
        _shasta_base_url: &str,
        _shasta_root_cert: &[u8],
        _vault_base_url: &str,
        _vault_secret_path: &str,
        _k8s_api_url: &str,
        _shasta_k8s_secrets: serde_json::Value,
        // _sat_file_content: String,
        _sat_template_file_yaml: serde_yaml::Value,
        _hsm_group_param_opt: Option<&String>,
        _hsm_group_available_vec: &Vec<String>,
        _ansible_verbosity_opt: Option<u8>,
        _ansible_passthrough_opt: Option<&String>,
        _gitea_base_url: &str,
        _gitea_token: &str,
        _do_not_reboot: bool,
        _watch_logs: bool,
        _image_only: bool,
        _session_template_only: bool,
        _debug_on_failure: bool,
        _dry_run: bool,
    ) -> Result<(), Error> {
        Err(Error::Message(
            "Process SAT file command not implemented for this backend".to_string(),
        ))
    } */
}

impl ApplyHwClusterPin for Ochami {
    /* async fn apply_hw_cluster_pin(
        &self,
        _shasta_token: &str,
        _shasta_base_url: &str,
        _shasta_root_cert: &[u8],
        _target_hsm_group_name: &str,
        _parent_hsm_group_name: &str,
        _pattern: &str,
        _nodryrun: bool,
        _create_target_hsm_group: bool,
        _delete_empty_parent_hsm_group: bool,
    ) -> Result<(), Error> {
        Err(Error::Message(
            "Apply HW Cluster Pin command not implemented for this backend".to_string(),
        ))
    } */
}

impl ImsTrait for Ochami {
    /* async fn get_images(
        &self,
        _shasta_token: &str,
        _shasta_base_url: &str,
        _shasta_root_cert: &[u8],
        _image_id_opt: Option<&str>,
    ) -> Result<Vec<Image>, Error> {
        Err(Error::Message(
            "Get images command not implemented for this backend".to_string(),
        ))
    } */
}

impl ApplySessionTrait for Ochami {
    /* async fn apply_session(
        &self,
        _gitea_token: &str,
        _gitea_base_url: &str,
        _shasta_token: &str,
        _shasta_base_url: &str,
        _shasta_root_cert: &[u8],
        _k8s_api_url: &str,
        _cfs_conf_sess_name: Option<&String>,
        _playbook_yaml_file_name_opt: Option<&String>,
        _hsm_group: Option<&String>,
        _repos_paths: Vec<PathBuf>,
        _ansible_limit: Option<String>,
        _ansible_verbosity: Option<String>,
        _ansible_passthrough: Option<String>,
        _watch_logs: bool,
        /* _kafka_audit: &Kafka,
        _k8s: &K8sDetails, */
    ) -> Result<(String, String), Error> {
        Err(Error::Message(
            "Apply session command not implemented for this backend".to_string(),
        ))
    } */
}

impl MigrateRestoreTrait for Ochami {
    /* async fn migrate_restore(
        &self,
        _shasta_token: &str,
        _shasta_base_url: &str,
        _shasta_root_cert: &[u8],
        _bos_file: Option<&String>,
        _cfs_file: Option<&String>,
        _hsm_file: Option<&String>,
        _ims_file: Option<&String>,
        _image_dir: Option<&String>,
    ) -> Result<(), Error> {
        Err(Error::Message(
            "Migrate/restore command not implemented for this backend".to_string(),
        ))
    } */
}

impl MigrateBackupTrait for Ochami {
    /* async fn migrate_backup(
        &self,
        _shasta_token: &str,
        _shasta_base_url: &str,
        _shasta_root_cert: &[u8],
        _bos: Option<&String>,
        _destination: Option<&String>,
    ) -> Result<(), Error> {
        Err(Error::Message(
            "Migrate/backup command not implemented for this backend".to_string(),
        ))
    } */
}

impl GetImagesAndDetailsTrait for Ochami {}

impl GetTemplatesTrait for Ochami {}

impl ClusterSessionTrait for Ochami {}
