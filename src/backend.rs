use infra::{
    self,
    contracts::BackendTrait,
    error::Error,
    types::{BootParameters, HsmGroup, Member},
};
use serde_json::Value;

use crate::backend_api::{self, bss};

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

    fn get_hsm_name_available(&self, _token: &str) -> Result<Vec<String>, Error> {
        Ok(Vec::new())
    }

    async fn get_api_token(&self, _site_name: &str) -> Result<String, Error> {
        backend_api::authentication::get_api_token().await
    }

    // FIXME: rename function to 'get_hsm_group_members'
    async fn get_member_vec_from_hsm_name_vec(
        &self,
        auth_token: &str,
        hsm_group_name_vec: Vec<String>,
    ) -> Result<Vec<String>, Error> {
        crate::backend_api::hsm::group::utils::get_member_vec_from_hsm_name_vec_2(
            auth_token,
            &self.base_url,
            &self.root_cert,
            hsm_group_name_vec,
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))
    }

    async fn get_all_hsm(&self, auth_token: &str) -> Result<Vec<HsmGroup>, Error> {
        // Get all HSM groups
        let hsm_group_backend_vec = crate::backend_api::hsm::group::http_client::get(
            &self.base_url,
            auth_token,
            &self.root_cert,
            None,
            None,
        )
        .await
        .map_err(|e| Error::Message(e.to_string()))?;

        // Convert all HSM groups from mesa to infra
        let mut hsm_group_vec = Vec::new();

        for hsm_group_backend in hsm_group_backend_vec {
            let mut member_vec = Vec::new();
            let member_vec_backend = hsm_group_backend.members.unwrap().ids.unwrap();

            for member in member_vec_backend {
                member_vec.push(member);
            }

            let members = Member {
                ids: Some(member_vec),
            };

            let hsm_group = HsmGroup {
                label: hsm_group_backend.label,
                description: hsm_group_backend.description,
                tags: hsm_group_backend.tags,
                members: Some(members),
                exclusive_group: hsm_group_backend.exclusive_group,
            };

            hsm_group_vec.push(hsm_group);
        }

        Ok(hsm_group_vec)
    }

    /* async fn power_reset_sync(
        &self,
        auth_token: &str,
        nodes: &[String],
        force: bool,
    ) -> Result<Value, Error> {
        Err(Error::Message(
            "Power reset command not implemented for this backend".to_string(),
        ))
    } */

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
    ) -> Result<BootParameters, Error> {
        let boot_parameters = bss::types::BootParameters {
            hosts: boot_parameter.hosts.clone(),
            macs: boot_parameter.macs.clone(),
            nids: boot_parameter.nids.clone(),
            params: boot_parameter.params.clone(),
            kernel: boot_parameter.kernel.clone(),
            initrd: boot_parameter.initrd.clone(),
            cloud_init: boot_parameter.cloud_init.clone(),
        };

        bss::http_client::put(&self.base_url, auth_token, &self.root_cert, boot_parameters)
            .await
            .map_err(|e| Error::Message(e.to_string()))
            .map(|bp| BootParameters {
                hosts: bp.hosts,
                macs: bp.macs,
                nids: bp.nids,
                params: bp.params,
                kernel: bp.kernel,
                initrd: bp.initrd,
                cloud_init: bp.cloud_init,
            })
    }
}
