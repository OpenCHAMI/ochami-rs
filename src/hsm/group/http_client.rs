use config::Value;

use crate::error::Error;

use super::types::{HsmGroup, Member};

pub fn get(
    auth_token: &str,
    base_url: &str,
    root_cert: &[u8],
    group: Option<&str>,
    tag: Option<&str>,
) -> Result<Vec<HsmGroup>, Error> {
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

    let api_url: String = format!("{}/{}", base_url, "smd/hsm/v2/groups");

    let response = client
        .get(api_url)
        .query(&[group, tag])
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
    group_label: &str,
) -> Result<HsmGroup, Error> {
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

    let api_url: String = format!("{}/{}/{}", base_url, "smd/hsm/v2/groups", group_label);

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

pub fn get_labels(
    auth_token: &str,
    base_url: &str,
    root_cert: &[u8],
) -> Result<Vec<String>, Error> {
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

    let api_url: String = format!("{}/{}", base_url, "smd/hsm/v2/groups/labels");

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

pub fn get_members(
    auth_token: &str,
    base_url: &str,
    root_cert: &[u8],
    group_label: &str,
) -> Result<Member, Error> {
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

    let api_url: String = format!("{}/smd/hsm/v2/groups/{}/members", base_url, group_label);

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

pub fn post(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    group: HsmGroup,
) -> Result<HsmGroup, Error> {
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

    let api_url: String = base_url.to_owned() + "/smd/hsm/v2/groups";

    let response = client
        .post(api_url)
        .bearer_auth(auth_token)
        .json(&group)
        .send()
        .map_err(|error| Error::NetError(error))?;

    if response.status().is_success() {
        response.json().map_err(|error| Error::NetError(error))
    } else {
        Err(Error::CsmError(response.json()?))
    }
}

pub fn post_members(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    partition_name: &str,
    members: Member,
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

    let api_url: String = format!("{}/smd/hsm/v2/groups/{}/members", base_url, partition_name);

    let response = client
        .post(api_url)
        .bearer_auth(auth_token)
        .json(&members)
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
    group_label: &str,
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

    let api_url: String = format!("{}/{}/{}", base_url, "smd/hsm/v2/groups", group_label);

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

pub fn delete_member(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    group_label: &str,
    xname: &str,
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
        "{}/smd/hsm/v2/groups/{}/members/{}",
        base_url, group_label, xname
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
