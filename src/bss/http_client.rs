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

pub async fn get(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  xnames_opt: &Option<Vec<String>>,
) -> Result<Vec<BootParameters>, Error> {
  let client = crate::http::build_client(root_cert)?;

  let url_api = format!("{}/boot/v1/bootparameters", base_url);

  let payload = xnames_opt.as_ref().map(|xname_vec| BootParameters {
    hosts: xname_vec.clone(),
    macs: None,
    nids: None,
    params: String::new(),
    kernel: String::new(),
    initrd: String::new(),
    cloud_init: None,
  });

  let response = client
    .get(url_api)
    .bearer_auth(auth_token)
    .json(&payload)
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
        return Err(Error::RequestError {
          response: e,
          payload: serde_json::to_string_pretty(&error_payload)?,
        });
      }
    }
  }

  match response.json().await {
    Ok(Value::Null) => Ok(Vec::new()),
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
  let client = crate::http::build_client(root_cert)?;
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
        return Err(Error::RequestError {
          response: e,
          payload: error_payload,
        });
      }
      _ => {
        let error_payload = response.json::<Value>().await?;
        return Err(Error::RequestError {
          response: e,
          payload: serde_json::to_string_pretty(&error_payload)?,
        });
      }
    }
  }

  Ok(())
}

pub async fn put(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  boot_parameters: &BootParameters,
) -> Result<BootParameters, Error> {
  let client = crate::http::build_client(root_cert)?;
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
        return Err(Error::RequestError {
          response: e,
          payload: error_payload,
        });
      }
      _ => {
        let error_payload = response.json::<Value>().await?;
        return Err(Error::RequestError {
          response: e,
          payload: serde_json::to_string_pretty(&error_payload)?,
        });
      }
    }
  }

  response.json().await.map_err(Error::NetError)
}

pub async fn patch(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  boot_parameters: &BootParameters,
) -> Result<(), Error> {
  let client = crate::http::build_client(root_cert)?;
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
        return Err(Error::RequestError {
          response: e,
          payload: error_payload,
        });
      }
      _ => {
        let error_payload = response.json::<Value>().await?;
        return Err(Error::RequestError {
          response: e,
          payload: serde_json::to_string_pretty(&error_payload)?,
        });
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
  let client = crate::http::build_client(root_cert)?;
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
        return Err(Error::RequestError {
          response: e,
          payload: error_payload,
        });
      }
      _ => {
        let error_payload = response.text().await?;
        return Err(Error::RequestError {
          response: e,
          payload: serde_json::to_string_pretty(&error_payload)?,
        });
      }
    }
  }

  response
    .text()
    .await
    .map_err(|e| Error::Message(e.to_string()))
}
