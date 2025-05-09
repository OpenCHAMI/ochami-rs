use serde_json::Value;

use crate::error::Error;

use super::types::{NodeMap, NodeMapArray};

/// Returns all NddeMaps in the system. Created for convenience. This function is equivalent to "get"
pub async fn get_all(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
) -> Result<NodeMapArray, Error> {
    get(base_url, auth_token, root_cert).await
}

/// Returns all NodeMaps in the system
pub async fn get(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
) -> Result<NodeMapArray, Error> {
    let client_builder =
        reqwest::Client::builder().add_root_certificate(reqwest::tls::Certificate::from_pem(root_cert)?);

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

    let api_url: String = format!("{}/{}", base_url, "hsm/v2/Defaults/NodeMaps");

    let response = client.get(api_url).bearer_auth(auth_token).send().await?;

    if let Err(e) = response.error_for_status_ref() {
        match response.status() {
            reqwest::StatusCode::UNAUTHORIZED => {
                let error_payload = response.text().await?;
                let error = Error::RequestError {
                    response: e,
                    payload: error_payload,
                };
                return Err(error);
            }
            _ => {
                let error_payload = response.json::<Value>().await?;
                let error = Error::CsmError(error_payload);
                return Err(error);
            }
        }
    }

    response
        .json()
        .await
        .map_err(|error| Error::NetError(error))
}

/// Get single node map
pub async fn get_one(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    xname: &str,
) -> Result<NodeMap, Error> {
    let client_builder =
        reqwest::Client::builder().add_root_certificate(reqwest::tls::Certificate::from_pem(root_cert)?);

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

    let api_url: String = format!("{}/{}/{}", base_url, "hsm/v2/Defaults/NodeMaps", xname);

    let response = client.get(api_url).bearer_auth(auth_token).send().await?;

    if let Err(e) = response.error_for_status_ref() {
        match response.status() {
            reqwest::StatusCode::UNAUTHORIZED => {
                let error_payload = response.text().await?;
                let error = Error::RequestError {
                    response: e,
                    payload: error_payload,
                };
                return Err(error);
            }
            _ => {
                let error_payload = response.json::<Value>().await?;
                let error = Error::CsmError(error_payload);
                return Err(error);
            }
        }
    }

    response
        .json()
        .await
        .map_err(|error| Error::NetError(error))
}

pub async fn post(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    node_maps: NodeMapArray,
) -> Result<Value, Error> {
    let client_builder =
        reqwest::Client::builder().add_root_certificate(reqwest::tls::Certificate::from_pem(root_cert)?);

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

    let api_url: String = base_url.to_owned() + "/hsm/v2/Defaults/NodeMaps";

    let response = client
        .post(api_url)
        .bearer_auth(auth_token)
        .json(&node_maps)
        .send()
        .await?;

    if let Err(e) = response.error_for_status_ref() {
        match response.status() {
            reqwest::StatusCode::UNAUTHORIZED => {
                let error_payload = response.text().await?;
                let error = Error::RequestError {
                    response: e,
                    payload: error_payload,
                };
                return Err(error);
            }
            _ => {
                let error_payload = response.json::<Value>().await?;
                let error = Error::CsmError(error_payload);
                return Err(error);
            }
        }
    }

    response
        .json()
        .await
        .map_err(|error| Error::NetError(error))
}

pub async fn put(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    xname: &str,
    node_map: NodeMap,
) -> Result<Value, Error> {
    let client_builder =
        reqwest::Client::builder().add_root_certificate(reqwest::tls::Certificate::from_pem(root_cert)?);

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

    let api_url: String = format!("{}/{}/{}", base_url, "hsm/v2/Defaults/NodeMaps/", xname);

    let response = client
        .put(api_url)
        .bearer_auth(auth_token)
        .json(&node_map)
        .send()
        .await?;

    if let Err(e) = response.error_for_status_ref() {
        match response.status() {
            reqwest::StatusCode::UNAUTHORIZED => {
                let error_payload = response.text().await?;
                let error = Error::RequestError {
                    response: e,
                    payload: error_payload,
                };
                return Err(error);
            }
            _ => {
                let error_payload = response.json::<Value>().await?;
                let error = Error::CsmError(error_payload);
                return Err(error);
            }
        }
    }

    response
        .json()
        .await
        .map_err(|error| Error::NetError(error))
}

pub async fn delete_one(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    xname: &str,
) -> Result<Value, Error> {
    let client_builder =
        reqwest::Client::builder().add_root_certificate(reqwest::tls::Certificate::from_pem(root_cert)?);

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

    let api_url: String = format!("{}/{}/{}", base_url, "hsm/v2/Defaults/NodeMaps", xname);

    let response = client
        .delete(api_url)
        .bearer_auth(auth_token)
        .send()
        .await?;

    if let Err(e) = response.error_for_status_ref() {
        match response.status() {
            reqwest::StatusCode::UNAUTHORIZED => {
                let error_payload = response.text().await?;
                let error = Error::RequestError {
                    response: e,
                    payload: error_payload,
                };
                return Err(error);
            }
            _ => {
                let error_payload = response.json::<Value>().await?;
                let error = Error::CsmError(error_payload);
                return Err(error);
            }
        }
    }

    response
        .json()
        .await
        .map_err(|error| Error::NetError(error))
}

pub async fn delete(base_url: &str, auth_token: &str, root_cert: &[u8]) -> Result<Value, Error> {
    let client_builder =
        reqwest::Client::builder().add_root_certificate(reqwest::tls::Certificate::from_pem(root_cert)?);

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

    let api_url: String = format!("{}/{}", base_url, "hsm/v2/Defaults/NodeMaps");

    let response = client
        .delete(api_url)
        .bearer_auth(auth_token)
        .send()
        .await?;

    if let Err(e) = response.error_for_status_ref() {
        match response.status() {
            reqwest::StatusCode::UNAUTHORIZED => {
                let error_payload = response.text().await?;
                let error = Error::RequestError {
                    response: e,
                    payload: error_payload,
                };
                return Err(error);
            }
            _ => {
                let error_payload = response.json::<Value>().await?;
                let error = Error::CsmError(error_payload);
                return Err(error);
            }
        }
    }

    response
        .json()
        .await
        .map_err(|error| Error::NetError(error))
}
