use serde_json::Value;

use crate::error::Error;

use super::types::Membership;

pub async fn get(
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
  let client_builder = reqwest::Client::builder()
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

  return Ok(
    response
      .json()
      .await
      .map_err(|error| Error::NetError(error))
      .unwrap(),
  );
}

pub async fn get_xname(
  shasta_token: &str,
  shasta_base_url: &str,
  shasta_root_cert: &[u8],
  xname: &str,
) -> Result<Membership, Error> {
  log::info!("Get membership of node '{}'", xname);
  let client_builder = reqwest::Client::builder()
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

  return Ok(
    response
      .json()
      .await
      .map_err(|error| Error::NetError(error))
      .unwrap(),
  );
}
