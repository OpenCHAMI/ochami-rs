use config::Value;

use crate::{error::Error, hsm::inventory::types::HWInventoryByFRU};

pub fn get(
    auth_token: &str,
    base_url: &str,
    root_cert: &[u8],
    fruid: Option<&str>,
    r#type: Option<&str>,
    manufacturer: Option<&str>,
    partnumber: Option<&str>,
    serialnumber: Option<&str>,
) -> Result<Vec<HWInventoryByFRU>, Error> {
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

    let api_url: String = format!("{}/{}", base_url, "/smd/hsm/v2/Inventory/HardwareByFRU");

    let response = client
        .get(api_url)
        .query(&[fruid, r#type, manufacturer, partnumber, serialnumber, fruid])
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
    fruid: &str,
) -> Result<HWInventoryByFRU, Error> {
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
        "{}/{}/{}",
        base_url, "/smd/hsm/v2/Inventory/Hardware", fruid
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

    let api_url: String = base_url.to_owned() + "/smd/hsm/v2/Inventory/HardwareByFRU";

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
    fruid: &str,
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
        "{}/{}/{}",
        base_url, "smd/hsm/v2/Inventory/HardwareByFRU", fruid
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
