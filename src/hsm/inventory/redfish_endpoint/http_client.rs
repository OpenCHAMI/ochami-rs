use serde_json::Value;

use crate::error::Error;

use super::types::{RedfishEndpoint, RedfishEndpointArray};

pub async fn get_query(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  xname: &str,
) -> Result<RedfishEndpointArray, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = format!(
    "{}/hsm/v2/Inventory/RedfishEndpoint/Query/{}",
    base_url, xname
  );

  let response = client
    .get(api_url)
    .query(&[xname])
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

pub async fn get_all(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
) -> Result<RedfishEndpointArray, Error> {
  get(
    auth_token, base_url, root_cert, None, None, None, None,
    None, None, None,
  )
  .await
}

pub async fn get(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  id: Option<&str>,
  fqdn: Option<&str>,
  r#type: Option<&str>,
  uuid: Option<&str>,
  macaddr: Option<&str>,
  ip_address: Option<&str>,
  last_status: Option<&str>,
) -> Result<RedfishEndpointArray, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url =
    format!("{}/hsm/v2/Inventory/RedfishEndpoints", base_url);

  let response = client
    .get(api_url)
    .query(&[
      ("id", id),
      ("fqdn", fqdn),
      ("type", r#type),
      ("uuid", uuid),
      ("madaddr", macaddr),
      ("ip_address", ip_address),
      ("last_status", last_status),
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
) -> Result<RedfishEndpoint, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = format!(
    "{}/hsm/v2/Inventory/RedfishEndpoints/{}",
    base_url, xname
  );

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
  redfish_endpoint: RedfishEndpointArray,
) -> Result<Value, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url =
    format!("{}/hsm/v2/Inventory/RedfishEndpoints", base_url);

  let response = client
    .post(api_url)
    .bearer_auth(auth_token)
    .json(&redfish_endpoint)
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

pub async fn put(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  xname: &str,
  redfish_endpoint: RedfishEndpoint,
) -> Result<RedfishEndpoint, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = format!(
    "{}/hsm/v2/Inventory/RedfishEndpoints/{}",
    base_url, xname
  );

  let response = client
    .put(api_url)
    .bearer_auth(auth_token)
    .json(&redfish_endpoint)
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
  let api_url = base_url.to_owned() + "hsm/v2/Inventory/RedfishEndpoints";

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
  let api_url = format!(
    "{}/hsm/v2/Inventory/RedfishEndpoints/{}",
    base_url, xname
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
        let error_payload = response.text().await?;
        return Err(Error::Message(error_payload));
      }
    }
  }

  response.json().await.map_err(Error::NetError)
}
