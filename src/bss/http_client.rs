use crate::error::Error;
use serde_json::Value;

use core::result::Result;

use super::types::BootParameters;

pub async fn get_all(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
) -> Result<Vec<BootParameters>, Error> {
    get(base_url, auth_token, root_cert, &None).await
}

/// Get node boot params, ref --> https://apidocs.svc.cscs.ch/iaas/bss/tag/bootparameters/paths/~1bootparameters/get/
/// NOTE: MANTA MIGRATION! the 'url_api' value changes compared to CSM
/// NOTE: if db is empty, then OCHAMI API will return 'Null' therefore, if we want to handle this
/// situation, then, we need to return serde_json::Value
pub async fn get(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    xnames_opt: &Option<Vec<String>>,
) -> Result<Vec<BootParameters>, Error> {
    let client;

    let client_builder =
        reqwest::Client::builder().add_root_certificate(reqwest::Certificate::from_pem(root_cert)?);

    // Build client
    if std::env::var("SOCKS5").is_ok() {
        // socks5 proxy
        log::debug!("SOCKS5 enabled");
        let socks5proxy = reqwest::Proxy::all(std::env::var("SOCKS5").unwrap())?;

        // rest client to authenticate
        client = client_builder.proxy(socks5proxy).build()?;
    } else {
        client = client_builder.build()?;
    }

    let url_api = format!("{}/boot/v1/bootparameters", base_url.to_string());

    let params: Option<Vec<_>> = if let Some(xname_vec) = xnames_opt {
        Some(xname_vec.iter().map(|xname| ("name", xname)).collect())
    } else {
        None
    };

    let response = client
        .get(url_api)
        .query(&params)
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
                let error = Error::RequestError {
                    response: e,
                    payload: serde_json::to_string_pretty(&error_payload)?,
                };
                return Err(error);
            }
        }
    }

    match response.json().await {
        Ok(Value::Null) => Ok(Vec::new()), // NOTE: In case OCHAMI decides to return 'Null' instead
        // of empty array
        Ok(v) => serde_json::from_value(v).map_err(|e| Error::Message(e.to_string())),
        Err(e) => Err(Error::NetError(e)),
    }
}

pub async fn post(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    boot_parameters: BootParameters,
) -> Result<(), Error> {
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

    let api_url = format!("{}/boot/v1/bootparameters", base_url);

    let response = client
        .post(api_url)
        .bearer_auth(auth_token)
        .json(&boot_parameters)
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
                let error = Error::RequestError {
                    response: e,
                    payload: serde_json::to_string_pretty(&error_payload)?,
                };
                return Err(error);
            }
        }
    }

    Ok(())
}

/// Change nodes boot params, ref --> https://apidocs.svc.cscs.ch/iaas/bss/tag/bootparameters/paths/~1bootparameters/put/
pub async fn put(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    boot_parameters: &BootParameters,
) -> Result<BootParameters, Error> {
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

    let api_url = format!("{}/boot/v1/bootparameters", base_url);

    let response = client
        .put(api_url)
        .json(&boot_parameters)
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
                let error = Error::RequestError {
                    response: e,
                    payload: serde_json::to_string_pretty(&error_payload)?,
                };
                return Err(error);
            }
        }
    }

    response.json().await.map_err(|e| Error::NetError(e))
}

pub async fn patch(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    boot_parameters: &BootParameters,
) -> Result<(), Error> {
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

    let api_url = format!("{}/boot/v1/bootparameters", base_url);

    let response = client
        .patch(api_url)
        .json(&boot_parameters)
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
                let error = Error::RequestError {
                    response: e,
                    payload: serde_json::to_string_pretty(&error_payload)?,
                };
                return Err(error);
            }
        }
    }

    Ok(())
}

pub async fn delete(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    boot_parameters: &BootParameters,
) -> Result<String, Error> {
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

    let api_url = format!("{}/boot/v1/bootparameters", base_url);

    let response = client
        .delete(api_url)
        .json(&boot_parameters)
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
                let error = Error::RequestError {
                    response: e,
                    payload: serde_json::to_string_pretty(&error_payload)?,
                };
                return Err(error);
            }
        }
    }

    response
        .text()
        .await
        .map_err(|e| Error::Message(e.to_string()))
}
