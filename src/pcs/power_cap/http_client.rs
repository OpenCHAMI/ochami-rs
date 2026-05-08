use serde_json::Value;

use crate::error::Error;

use super::types::{PowerCapComponent, PowerCapTaskInfo};

pub async fn get(
  shasta_base_url: &str,
  shasta_token: &str,
  shasta_root_cert: &[u8],
  socks5_proxy: Option<&str>,
) -> Result<PowerCapTaskInfo, Error> {
  let client = crate::http::build_client(shasta_root_cert, socks5_proxy)?;
  let api_url = format!("{}/power-control/v1/power-cap", shasta_base_url);

  let response = client
    .get(api_url)
    .bearer_auth(shasta_token)
    .send()
    .await
    .map_err(Error::NetError)?;

  if response.status().is_success() {
    response.json().await.map_err(Error::NetError)
  } else {
    let payload = response.json::<Value>().await.map_err(Error::NetError)?;
    Err(Error::OchamiError(payload))
  }
}

pub async fn get_task_id(
  shasta_base_url: &str,
  shasta_token: &str,
  shasta_root_cert: &[u8],
  socks5_proxy: Option<&str>,
  task_id: &str,
) -> Result<PowerCapTaskInfo, Error> {
  let client = crate::http::build_client(shasta_root_cert, socks5_proxy)?;
  let api_url =
    format!("{}/power-control/v1/power-cap/{}", shasta_base_url, task_id);

  let response = client
    .get(api_url)
    .bearer_auth(shasta_token)
    .send()
    .await
    .map_err(Error::NetError)?;

  if response.status().is_success() {
    response.json().await.map_err(Error::NetError)
  } else {
    let payload = response.json::<Value>().await.map_err(Error::NetError)?;
    Err(Error::OchamiError(payload))
  }
}

pub async fn post_snapshot(
  shasta_base_url: &str,
  shasta_token: &str,
  shasta_root_cert: &[u8],
  socks5_proxy: Option<&str>,
  xname_vec: Vec<&str>,
) -> Result<PowerCapTaskInfo, Error> {
  log::info!("Create PCS power snapshot for nodes:\n{:?}", xname_vec);

  let client = crate::http::build_client(shasta_root_cert, socks5_proxy)?;
  let api_url =
    shasta_base_url.to_owned() + "/power-control/v1/power-cap/snapshot";

  let response = client
    .put(api_url)
    .json(&serde_json::json!({ "xnames": xname_vec }))
    .bearer_auth(shasta_token)
    .send()
    .await
    .map_err(Error::NetError)?;

  if response.status().is_success() {
    response.json().await.map_err(Error::NetError)
  } else {
    let payload = response.json::<Value>().await.map_err(Error::NetError)?;
    Err(Error::OchamiError(payload))
  }
}

pub async fn patch(
  shasta_base_url: &str,
  shasta_token: &str,
  shasta_root_cert: &[u8],
  socks5_proxy: Option<&str>,
  power_cap: Vec<PowerCapComponent>,
) -> Result<PowerCapTaskInfo, Error> {
  log::info!("Create PCS power cap:\n{:#?}", power_cap);

  let client = crate::http::build_client(shasta_root_cert, socks5_proxy)?;
  let api_url =
    shasta_base_url.to_owned() + "/power-control/v1/power-cap/snapshot";

  let response = client
    .put(api_url)
    .json(&power_cap)
    .bearer_auth(shasta_token)
    .send()
    .await
    .map_err(Error::NetError)?;

  if response.status().is_success() {
    response.json().await.map_err(Error::NetError)
  } else {
    let payload = response.json::<Value>().await.map_err(Error::NetError)?;
    Err(Error::OchamiError(payload))
  }
}
