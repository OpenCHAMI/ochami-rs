use serde_json::Value;

use crate::error::Error;

use super::types::Membership;

pub async fn get(
  shasta_token: &str,
  shasta_base_url: &str,
  shasta_root_cert: &[u8],
  socks5_proxy: Option<&str>,
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
  let client = crate::http::build_client(shasta_root_cert, socks5_proxy)?;
  let api_url = format!("{}/smd/hsm/v2/memberships", shasta_base_url);

  let response = client
    .get(api_url)
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
        return Err(Error::RequestError {
          response: e,
          payload: error_payload,
        });
      }
      _ => {
        let error_payload = response.json::<Value>().await?;
        return Err(Error::OchamiError(error_payload));
      }
    }
  }

  response.json().await.map_err(Error::NetError)
}

pub async fn get_xname(
  shasta_token: &str,
  shasta_base_url: &str,
  shasta_root_cert: &[u8],
  socks5_proxy: Option<&str>,
  xname: &str,
) -> Result<Membership, Error> {
  log::info!("Get membership of node '{}'", xname);

  let client = crate::http::build_client(shasta_root_cert, socks5_proxy)?;
  let api_url =
    format!("{}/smd/hsm/v2/memberships/{}", shasta_base_url, xname);

  let response = client
    .get(api_url)
    .header("Authorization", format!("Bearer {}", shasta_token))
    .send()
    .await?;

  if let Err(e) = response.error_for_status_ref() {
    match response.status() {
      reqwest::StatusCode::UNAUTHORIZED => {
        let error_payload = response.text().await?;
        return Err(Error::RequestError {
          response: e,
          payload: error_payload,
        });
      }
      _ => {
        let error_payload = response.json::<Value>().await?;
        return Err(Error::OchamiError(error_payload));
      }
    }
  }

  response.json().await.map_err(Error::NetError)
}
