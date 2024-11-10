use config::Value;

use crate::error::Error;

use super::types::{ComponentEthernetInterface, IpAddressMapping};

// Get list of network interfaces
// ref --> https://csm12-apidocs.svc.cscs.ch/iaas/hardware-state-manager/operation/doCompEthInterfacesGetV2/
pub fn get(
    auth_token: &str,
    base_url: &str,
    root_cert: &[u8],
    mac_address: &str,
    ip_address: &str,
    network: &str,
    component_id: &str, // Node's xname
    r#type: &str,
    olther_than: &str,
    newer_than: &str,
) -> Result<Vec<ComponentEthernetInterface>, Error> {
    let client_builder = reqwest::blocking::Client::builder()
        .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?);

    // Build client
    let client = if let Ok(socks5_env) = std::env::var("SOCKS5") {
        // socks5 proxy
        log::debug!("SOCKS5 enabled");
        let socks5proxy = reqwest::Proxy::all(socks5_env)?;

        // rest client to authenticate
        client_builder.proxy(socks5proxy).build()?
    } else {
        client_builder.build()?
    };

    let api_url: String = base_url.to_owned() + "/smd/hsm/v2/Inventory/EthernetInterfaces";

    let response = client
        .get(api_url)
        .query(&[
            ("MACAddress", mac_address),
            ("IPAddress", ip_address),
            ("Network", network),
            ("ComponentID", component_id),
            ("Type", r#type),
            ("OlderThan", olther_than),
            ("NewerThan", newer_than),
        ])
        .bearer_auth(auth_token)
        .send()
        .map_err(|error| Error::NetError(error))?;

    if response.status().is_success() {
        response.json().map_err(|error| Error::NetError(error))
    } else {
        Err(Error::CsmError(response.json()?))
    }
}

pub fn get_one(
    auth_token: &str,
    base_url: &str,
    root_cert: &[u8],
    eth_interface_id: &str,
) -> Result<ComponentEthernetInterface, Error> {
    let client_builder = reqwest::blocking::Client::builder()
        .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?);

    // Build client
    let client = if let Ok(socks5_env) = std::env::var("SOCKS5") {
        // socks5 proxy
        log::debug!("SOCKS5 enabled");
        let socks5proxy = reqwest::Proxy::all(socks5_env)?;

        // rest client to authenticate
        client_builder.proxy(socks5proxy).build()?
    } else {
        client_builder.build()?
    };

    let api_url: String = format!(
        "{}/smd/hsm/v2/Inventory/EthernetInterfaces/{}",
        base_url, eth_interface_id
    );

    let response = client
        .get(api_url)
        .bearer_auth(auth_token)
        .send()
        .map_err(|error| Error::NetError(error))?;

    if response.status().is_success() {
        response.json().map_err(|error| Error::NetError(error))
    } else {
        Err(Error::CsmError(response.json()?))
    }
}

pub fn get_ip_addresses(
    auth_token: &str,
    base_url: &str,
    root_cert: &[u8],
    eth_interface_id: &str,
) -> Result<Vec<IpAddressMapping>, Error> {
    let client_builder = reqwest::blocking::Client::builder()
        .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?);

    // Build client
    let client = if let Ok(socks5_env) = std::env::var("SOCKS5") {
        // socks5 proxy
        log::debug!("SOCKS5 enabled");
        let socks5proxy = reqwest::Proxy::all(socks5_env)?;

        // rest client to authenticate
        client_builder.proxy(socks5proxy).build()?
    } else {
        client_builder.build()?
    };

    let api_url: String = format!(
        "{}/smd/hsm/v2/Inventory/EthernetInterfaces/{}/IPAddresses",
        base_url, eth_interface_id
    );

    let response = client
        .get(api_url)
        .bearer_auth(auth_token)
        .send()
        .map_err(|error| Error::NetError(error))?;

    if response.status().is_success() {
        response.json().map_err(|error| Error::NetError(error))
    } else {
        Err(Error::CsmError(response.json()?))
    }
}

pub fn patch(
    shasta_token: &str,
    shasta_base_url: &str,
    shasta_root_cert: &[u8],
    eth_interface_id: &str,
    description: Option<&str>,
    eth_interface_id: &str,
    ip_address_mapping: (&str, &str), // [(<ip address>, <network>), ...], examle
                                      // [("192.168.1.10", "HMN"), ...]
) -> Result<(), Error> {
    let ip_address = ip_address_mapping.0;
    let network = ip_address_mapping.1;
    let cei = ComponentEthernetInterface {
        description: description.map(|value| value.to_string()),
        ip_addresses: vec![IpAddressMapping {
            ip_address: ip_address.to_string(),
            network: Some(network.to_string()),
        }],
        component_id: Some(eth_interface_id.to_string()),
    };

    let client_builder = reqwest::blocking::Client::builder()
        .add_root_certificate(reqwest::Certificate::from_pem(shasta_root_cert)?);

    // Build client
    let client = if let Ok(socks5_env) = std::env::var("SOCKS5") {
        // socks5 proxy
        log::debug!("SOCKS5 enabled");
        let socks5proxy = reqwest::Proxy::all(socks5_env)?;

        // rest client to authenticate
        client_builder.proxy(socks5proxy).build()?
    } else {
        client_builder.build()?
    };

    let api_url: String = format!(
        "{}/smd/hsm/v2/Inventory/EthernetInterfaces/{}",
        shasta_base_url, eth_interface_id
    );

    let response = client
        .patch(api_url)
        .query(&[("ethInterfaceID", ip_address), ("ipAddress", ip_address)])
        .bearer_auth(shasta_token)
        .json(&cei)
        .send()
        .map_err(|error| Error::NetError(error))?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(Error::CsmError(response.json()?))
    }
}

pub fn delete_all(base_url: &str, auth_token: &str, root_cert: &[u8]) -> Result<Value, Error> {
    let client_builder = reqwest::blocking::Client::builder()
        .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?);

    // Build client
    let client = if let Ok(socks5_env) = std::env::var("SOCKS5") {
        // socks5 proxy
        log::debug!("SOCKS5 enabled");
        let socks5proxy = reqwest::Proxy::all(socks5_env)?;

        // rest client to authenticate
        client_builder.proxy(socks5proxy).build()?
    } else {
        client_builder.build()?
    };

    let api_url: String = format!("{}/smd/hsm/v2/Inventory/EthernetInterfaces", base_url);

    let response = client
        .delete(api_url)
        .bearer_auth(auth_token)
        .send()
        .map_err(|error| Error::NetError(error))?;

    if response.status().is_success() {
        response.json().map_err(|error| Error::NetError(error))
    } else {
        Err(Error::CsmError(response.json()?))
    }
}

pub fn delete_one(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    eth_interface_id: &str,
) -> Result<Value, Error> {
    let client_builder = reqwest::blocking::Client::builder()
        .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?);

    // Build client
    let client = if let Ok(socks5_env) = std::env::var("SOCKS5") {
        // socks5 proxy
        log::debug!("SOCKS5 enabled");
        let socks5proxy = reqwest::Proxy::all(socks5_env)?;

        // rest client to authenticate
        client_builder.proxy(socks5proxy).build()?
    } else {
        client_builder.build()?
    };

    let api_url: String = format!(
        "{}/smd/hsm/v2/Inventory/EthernetInterfaces/{}",
        base_url, eth_interface_id
    );

    let response = client
        .delete(api_url)
        .bearer_auth(auth_token)
        .send()
        .map_err(|error| Error::NetError(error))?;

    if response.status().is_success() {
        response.json().map_err(|error| Error::NetError(error))
    } else {
        Err(Error::CsmError(response.json()?))
    }
}

pub fn delete_ip_address(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    group_label: &str,
    eth_interface_id: &str,
    ip_address: &str,
) -> Result<Value, Error> {
    let client_builder = reqwest::blocking::Client::builder()
        .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?);

    // Build client
    let client = if let Ok(socks5_env) = std::env::var("SOCKS5") {
        // socks5 proxy
        log::debug!("SOCKS5 enabled");
        let socks5proxy = reqwest::Proxy::all(socks5_env)?;

        // rest client to authenticate
        client_builder.proxy(socks5proxy).build()?
    } else {
        client_builder.build()?
    };

    let api_url: String = format!(
        "{}/smd/hsm/v2/Inventory/EthernetInterfaces/{}/IpAddress/{}",
        base_url, eth_interface_id, ip_address
    );

    let response = client
        .delete(api_url)
        .bearer_auth(auth_token)
        .send()
        .map_err(|error| Error::NetError(error))?;

    if response.status().is_success() {
        response.json().map_err(|error| Error::NetError(error))
    } else {
        Err(Error::CsmError(response.json()?))
    }
}
