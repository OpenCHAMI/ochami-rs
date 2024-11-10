use serde_json::Value;

use crate::error::Error;

use super::types::Membership;

pub fn get(
    shasta_token: &str,
    shasta_base_url: &str,
    shasta_root_cert: &[u8],
    id: Option<&str>,
    r#type: Option<&str>,
    state: Option<&str>,
    flag: Option<&str>,
    role: Option<&str>,
    subrole: Option<&str>,
    enabled: Option<&str>,
    softwarestatus: Option<&str>,
    subtype: Option<&str>,
    arch: Option<&str>,
    class: Option<&str>,
    nid: Option<&str>,
    nid_start: Option<&str>,
    nid_end: Option<&str>,
    partition: Option<&str>,
    group: Option<&str>,
) -> Result<Vec<Membership>, Error> {
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

    let api_url = format!("{}/smd/hsm/v2/memberships", shasta_base_url);

    let response = client
        .get(api_url.clone())
        .query(&[
            id,
            r#type,
            state,
            flag,
            role,
            subrole,
            enabled,
            softwarestatus,
            subtype,
            arch,
            class,
            nid,
            nid_start,
            nid_end,
            partition,
            group,
        ])
        .header("Authorization", format!("Bearer {}", shasta_token))
        .send()
        .map_err(|error| Error::NetError(error))?;

    if response.status().is_success() {
        Ok(response
            .json::<Vec<Membership>>()
            .map_err(|error| Error::NetError(error))
            .unwrap())
    } else {
        let payload = response
            .json::<Value>()
            .map_err(|error| Error::NetError(error))?;
        Err(Error::CsmError(payload))
    }
}

pub fn get_xname(
    shasta_token: &str,
    shasta_base_url: &str,
    shasta_root_cert: &[u8],
    xname: &str,
) -> Result<Membership, Error> {
    log::info!("Get membership of node '{}'", xname);
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

    let api_url = format!("{}/smd/hsm/v2/memberships/{}", shasta_base_url, xname);

    let response = client
        .get(api_url.clone())
        .header("Authorization", format!("Bearer {}", shasta_token))
        .send()
        .map_err(|error| Error::NetError(error))?;

    if response.status().is_success() {
        Ok(response
            .json::<Membership>()
            .map_err(|error| Error::NetError(error))
            .unwrap())
    } else {
        let payload = response
            .json::<Value>()
            .map_err(|error| Error::NetError(error))?;
        Err(Error::CsmError(payload))
    }
}
