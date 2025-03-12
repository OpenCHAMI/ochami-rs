use serde_json::Value;

use crate::{
    error::Error,
    hsm::inventory::types::{HWInventoryByLocation, HWInventoryByLocationList},
};

pub async fn get_query(
    auth_token: &str,
    base_url: &str,
    root_cert: &[u8],
    xname: &str,
    r#type: Option<&str>,
    children: Option<bool>,
    parents: Option<bool>,
    partition: Option<&str>,
    format: Option<&str>,
) -> Result<Value, Error> {
    let client_builder =
        reqwest::Client::builder().add_root_certificate(reqwest::Certificate::from_pem(root_cert)?);

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
        base_url, "hsm/v2/Inventory/Hardware/Query", xname
    );

    let response = client
        .get(api_url)
        .query(&[
            r#type,
            children.map(|value| value.to_string()).as_deref(),
            parents.map(|value| value.to_string()).as_deref(),
            partition,
            format,
        ])
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

pub async fn get(
    auth_token: &str,
    base_url: &str,
    root_cert: &[u8],
    id: Option<&str>,
    r#type: Option<&str>,
    manufacturer: Option<&str>,
    partnumber: Option<&str>,
    serialnumber: Option<&str>,
    fruid: Option<&str>,
) -> Result<Vec<HWInventoryByLocation>, Error> {
    let client_builder =
        reqwest::Client::builder().add_root_certificate(reqwest::Certificate::from_pem(root_cert)?);

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

    let api_url: String = format!("{}/{}", base_url, "hsm/v2/Inventory/Hardware");

    let response = client
        .get(api_url)
        .query(&[id, r#type, manufacturer, partnumber, serialnumber, fruid])
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
                let error_payload = response.text().await?;
                let error = Error::Message(error_payload);
                return Err(error);
            }
        }
    }

    response
        .json()
        .await
        .map_err(|error| Error::NetError(error))
}

pub async fn get_one(
    auth_token: &str,
    base_url: &str,
    root_cert: &[u8],
    xname: &str,
) -> Result<HWInventoryByLocation, Error> {
    let client_builder =
        reqwest::Client::builder().add_root_certificate(reqwest::Certificate::from_pem(root_cert)?);

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

    let api_url: String = format!("{}/{}/{}", base_url, "hsm/v2/Inventory/Hardware", xname);

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
                let error_payload = response.text().await?;
                let error = Error::Message(error_payload);
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
    auth_token: &str,
    base_url: &str,
    root_cert: &[u8],
    hardware: HWInventoryByLocationList,
) -> Result<Value, Error> {
    let client_builder =
        reqwest::Client::builder().add_root_certificate(reqwest::Certificate::from_pem(root_cert)?);

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

    let api_url: String = format!("{}/{}", base_url, "hsm/v2/Inventory/Hardware");

    let response = client
        .post(api_url)
        .bearer_auth(auth_token)
        .json(&hardware)
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
                let error_payload = response.text().await?;
                let error = Error::Message(error_payload);
                return Err(error);
            }
        }
    }

    let response_payload = response
        .json()
        .await
        .map_err(|error| Error::NetError(error));

    response_payload
}

pub async fn delete_all(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
) -> Result<Value, Error> {
    let client_builder =
        reqwest::Client::builder().add_root_certificate(reqwest::Certificate::from_pem(root_cert)?);

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

    let api_url: String = base_url.to_owned() + "hsm/v2/Inventory/Hardware";

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
                let error_payload = response.text().await?;
                let error = Error::Message(error_payload);
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
        reqwest::Client::builder().add_root_certificate(reqwest::Certificate::from_pem(root_cert)?);

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

    let api_url: String = format!("{}/{}/{}", base_url, "hsm/v2/Inventory/Hardware", xname);

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
                let error_payload = response.text().await?;
                let error = Error::Message(error_payload);
                return Err(error);
            }
        }
    }

    response
        .json()
        .await
        .map_err(|error| Error::NetError(error))
}
