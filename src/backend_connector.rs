use std::collections::HashMap;

use backend_dispatcher::{
    contracts::BackendTrait,
    error::Error,
    types::{BootParameters, Group as FrontEndGroup},
};
use serde_json::Value;

use crate::hsm::{self, group::types::Group};
use crate::{authentication, bss};

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

impl BackendTrait for Ochami {
    fn test_backend_trait(&self) -> String {
        println!("in silla backend");
        "in silla backend".to_string()
    }

    async fn get_api_token(&self, _site_name: &str) -> Result<String, Error> {
        authentication::get_api_token()
            .await
            .map_err(|_e| Error::Message("environment variable 'AUTH_TOKEN' not found".to_string()))
    }

    async fn get_group_name_available(&self, token: &str) -> Result<Vec<String>, Error> {
        let hsm_group_vec_rslt = self.get_all_groups(token).await;

        hsm_group_vec_rslt.and_then(|hsm_group_vec| {
            Ok(hsm_group_vec
                .iter()
                .map(|hsm_group| hsm_group.label.clone())
                .collect())
        })
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

    async fn post_members(
        &self,
        auth_token: &str,
        group_label: &str,
        xnames: &[&str],
    ) -> Result<(), Error> {
        let member = hsm::group::types::Member {
            ids: Some(xnames.into_iter().map(|value| value.to_string()).collect()),
        };

        hsm::group::http_client::post_members(
            &self.base_url,
            auth_token,
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
        members: Vec<&str>,
    ) -> Result<Vec<String>, Error> {
        hsm::group::utils::add_members(
            auth_token,
            &self.base_url,
            &self.root_cert,
            group_label,
            members.to_vec(),
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
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
        let hsm_group_backend_vec = hsm::group::http_client::get(
            &self.base_url,
            auth_token,
            &self.root_cert,
            Some(hsm_name),
            None,
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))?;

        // Error if more than one HSM group found
        if hsm_group_backend_vec.len() > 1 {
            return Err(Error::Message(format!(
                "ERROR - multiple HSM groups with name '{}' found. Exit",
                hsm_name
            )));
        }

        // Convert from HsmGroup (silla) to HsmGroup (infra)
        let hsm_group_backend = hsm_group_backend_vec.first().unwrap().to_owned();

        let hsm_group: FrontEndGroup = hsm_group_backend.into();

        Ok(hsm_group)
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
            hsm_group.into(),
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))?;

        let hsm_group: FrontEndGroup = hsm_group_backend.into();

        Ok(hsm_group)
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

    async fn get_bootparameters(
        &self,
        auth_token: &str,
        nodes: &[String],
    ) -> Result<Vec<BootParameters>, Error> {
        let boot_parameter_vec = bss::http_client::get(
            &self.base_url,
            auth_token,
            &self.root_cert,
            &Some(nodes.to_vec()),
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))?;

        let mut boot_parameter_infra_vec = vec![];

        for boot_parameter in boot_parameter_vec {
            boot_parameter_infra_vec.push(BootParameters {
                hosts: boot_parameter.hosts,
                macs: boot_parameter.macs,
                nids: boot_parameter.nids,
                params: boot_parameter.params,
                kernel: boot_parameter.kernel,
                initrd: boot_parameter.initrd,
                cloud_init: boot_parameter.cloud_init,
            });
        }

        Ok(boot_parameter_infra_vec)
    }

    async fn update_bootparameters(
        &self,
        auth_token: &str,
        boot_parameter: &BootParameters,
    ) -> Result<(), Error> {
        let boot_parameters = bss::types::BootParameters {
            hosts: boot_parameter.hosts.clone(),
            macs: boot_parameter.macs.clone(),
            nids: boot_parameter.nids.clone(),
            params: boot_parameter.params.clone(),
            kernel: boot_parameter.kernel.clone(),
            initrd: boot_parameter.initrd.clone(),
            cloud_init: boot_parameter.cloud_init.clone(),
        };

        bss::http_client::patch(
            &self.base_url,
            auth_token,
            &self.root_cert,
            &boot_parameters,
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
}
