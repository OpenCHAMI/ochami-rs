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
  let client = crate::http::build_client(root_cert)?;
  let api_url = format!(
    "{}/hsm/v2/Inventory/Hardware/Query/{}",
    base_url, xname
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
  let client = crate::http::build_client(root_cert)?;
  let api_url = format!("{}/hsm/v2/Inventory/Hardware", base_url);

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
        return Err(Error::RequestError {
          response: e,
          payload: error_payload,
        });
      }
      _ => {
        let error_payload = response.text().await?;
        return Err(Error::Message(error_payload));
      }
    }
  }

  response.json().await.map_err(Error::NetError)
}

pub async fn get_one(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  xname: &str,
) -> Result<HWInventoryByLocation, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = format!("{}/hsm/v2/Inventory/Hardware/{}", base_url, xname);

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
        let error_payload = response.text().await?;
        return Err(Error::Message(error_payload));
      }
    }
  }

  response.json().await.map_err(Error::NetError)
}

pub async fn post(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  hardware: HWInventoryByLocationList,
) -> Result<Value, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = format!("{}/hsm/v2/Inventory/Hardware", base_url);

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
        return Err(Error::RequestError {
          response: e,
          payload: error_payload,
        });
      }
      _ => {
        let error_payload = response.text().await?;
        return Err(Error::Message(error_payload));
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
  // NOTE: pre-existing bug — missing leading '/' before "hsm"
  let api_url = base_url.to_owned() + "hsm/v2/Inventory/Hardware";

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
        let error_payload = response.text().await?;
        return Err(Error::Message(error_payload));
      }
    }
  }

  response.json().await.map_err(Error::NetError)
}

pub async fn delete_one(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  xname: &str,
) -> Result<Value, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url =
    format!("{}/hsm/v2/Inventory/Hardware/{}", base_url, xname);

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
        let error_payload = response.text().await?;
        return Err(Error::Message(error_payload));
      }
    }
  }

  response.json().await.map_err(Error::NetError)
}
