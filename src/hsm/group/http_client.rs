use serde_json::Value;

use crate::{error::Error, hsm::group::types::Member};

use super::types::{Group, Members};

pub async fn get_all(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
) -> Result<Vec<Group>, Error> {
    get(base_url, auth_token, root_cert, None, None).await
}

pub async fn get(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    label_vec_opt: Option<&[&str]>,
    tag_vec_opt: Option<&[&str]>,
) -> Result<Vec<Group>, Error> {
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

    let api_url: String = format!("{}/{}", base_url, "hsm/v2/groups");

    let mut query = Vec::new();

    if let Some(label_vec) = label_vec_opt {
        for label in label_vec {
            query.push(("group", label));
        }
    }
    if let Some(tag_vec) = tag_vec_opt {
        for tag in tag_vec {
            query.push(("tag", tag));
        }
    }

    let response = client
        .get(api_url)
        .query(query.as_slice())
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
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    group_label: &str,
) -> Result<Group, Error> {
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

    let api_url: String = format!("{}/{}/{}", base_url, "hsm/v2/groups", group_label);

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
                let error = Error::OchamiError(error_payload);
                return Err(error);
            }
        }
    }

    response
        .json()
        .await
        .map_err(|error| Error::NetError(error))
}

pub async fn get_labels(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
) -> Result<Vec<String>, Error> {
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

    let api_url: String = format!("{}/{}", base_url, "hsm/v2/groups/labels");

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
                let error = Error::OchamiError(error_payload);
                return Err(error);
            }
        }
    }

    response
        .json()
        .await
        .map_err(|error| Error::NetError(error))
}

pub async fn get_members(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    group_label: &str,
) -> Result<Members, Error> {
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

    let api_url: String = format!("{}/hsm/v2/groups/{}/members", base_url, group_label);

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
                let error = Error::OchamiError(error_payload);
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
    group: Group,
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

    let api_url: String = base_url.to_owned() + "/hsm/v2/groups";

    let response = client
        .post(api_url)
        .bearer_auth(auth_token)
        .json(&group)
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
                let error = Error::OchamiError(error_payload);
                return Err(error);
            }
        }
    }

    response
        .text()
        .await
        .map_err(|error| Error::Message(error.to_string()))
}

pub async fn post_member(
    auth_token: &str,
    base_url: &str,
    root_cert: &[u8],
    group_label: &str,
    member: Member,
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

    let api_url: String = format!("{}/hsm/v2/groups/{}/members", base_url, group_label);

    let response = client
        .post(api_url)
        .bearer_auth(auth_token)
        .json(&member)
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
        .map_err(|e| Error::Message(e.to_string()))
}

pub async fn delete_one(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    group_label: &str,
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

    let api_url: String = format!("{}/{}/{}", base_url, "hsm/v2/groups", group_label);

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
                let error = Error::OchamiError(error_payload);
                return Err(error);
            }
        }
    }

    response
        .json()
        .await
        .map_err(|error| Error::NetError(error))
}

pub async fn delete_member(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    group_label: &str,
    xname: &str,
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

    let api_url: String = format!(
        "{}/hsm/v2/groups/{}/members/{}",
        base_url, group_label, xname
    );

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
                let error = Error::OchamiError(error_payload);
                return Err(error);
            }
        }
    }

    Ok(())
}
