use serde_json::Value;

use crate::{error::Error, hsm::inventory::types::HWInventoryByFRU};

pub async fn get(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  fruid: Option<&str>,
  r#type: Option<&str>,
  manufacturer: Option<&str>,
  partnumber: Option<&str>,
  serialnumber: Option<&str>,
) -> Result<Vec<HWInventoryByFRU>, Error> {
  let client_builder = reqwest::Client::builder()
    .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?)
    .use_rustls_tls();

  let client = match socks5_proxy {
    Some(proxy) => client_builder.proxy(reqwest::Proxy::all(proxy)?).build()?,
    None => client_builder.build()?,
  };

  let api_url: String =
    format!("{}/{}", base_url, "/smd/hsm/v2/Inventory/HardwareByFRU");

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

pub async fn get_one(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  fruid: &str,
) -> Result<HWInventoryByFRU, Error> {
  let client_builder = reqwest::Client::builder()
    .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?)
    .use_rustls_tls();

  let client = match socks5_proxy {
    Some(proxy) => client_builder.proxy(reqwest::Proxy::all(proxy)?).build()?,
    None => client_builder.build()?,
  };

  let api_url: String = format!(
    "{}/{}/{}",
    base_url, "/smd/hsm/v2/Inventory/Hardware", fruid
  );

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

pub async fn delete_all(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
) -> Result<Value, Error> {
  let client_builder = reqwest::Client::builder()
    .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?)
    .use_rustls_tls();

  let client = match socks5_proxy {
    Some(proxy) => client_builder.proxy(reqwest::Proxy::all(proxy)?).build()?,
    None => client_builder.build()?,
  };

  let api_url: String =
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

pub async fn delete_one(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  fruid: &str,
) -> Result<Value, Error> {
  let client_builder = reqwest::Client::builder()
    .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?)
    .use_rustls_tls();

  let client = match socks5_proxy {
    Some(proxy) => client_builder.proxy(reqwest::Proxy::all(proxy)?).build()?,
    None => client_builder.build()?,
  };

  let api_url: String = format!(
    "{}/{}/{}",
    base_url, "smd/hsm/v2/Inventory/HardwareByFRU", fruid
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

  response
    .json()
    .await
    .map_err(|error| Error::NetError(error))
}
