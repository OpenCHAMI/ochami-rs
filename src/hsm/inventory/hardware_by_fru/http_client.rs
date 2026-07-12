use serde_json::Value;

use crate::{error::Error, hsm::inventory::types::HWInventoryByFRU};

pub async fn get(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  fruid: Option<&str>,
  r#type: Option<&str>,
  manufacturer: Option<&str>,
  partnumber: Option<&str>,
  serialnumber: Option<&str>,
) -> Result<Vec<HWInventoryByFRU>, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url =
    format!("{}/smd/hsm/v2/Inventory/HardwareByFRU", base_url);

  let response = client
    .get(api_url)
    .query(&[fruid, r#type, manufacturer, partnumber, serialnumber, fruid])
    .bearer_auth(auth_token)
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

pub async fn get_one(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  fruid: &str,
) -> Result<HWInventoryByFRU, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url =
    format!("{}/smd/hsm/v2/Inventory/Hardware/{}", base_url, fruid);

  let response = client.get(api_url).bearer_auth(auth_token).send().await?;

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

pub async fn delete_all(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
) -> Result<Value, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url =
    base_url.to_owned() + "/smd/hsm/v2/Inventory/HardwareByFRU";

  let response = client
    .delete(api_url)
    .bearer_auth(auth_token)
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

pub async fn delete_one(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  fruid: &str,
) -> Result<Value, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = format!(
    "{}/smd/hsm/v2/Inventory/HardwareByFRU/{}",
    base_url, fruid
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
