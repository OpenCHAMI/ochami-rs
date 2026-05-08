use serde_json::Value;

use crate::error::Error;

use super::types::{NodeMap, NodeMapArray};

pub async fn get(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
) -> Result<NodeMapArray, Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url = base_url.to_owned() + "/smd/hsm/v2/Defaults/NodeMaps";

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

pub async fn get_one(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  xname: &str,
) -> Result<NodeMap, Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url =
    format!("{}/smd/hsm/v2/Defaults/NodeMaps/{}", base_url, xname);

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

pub async fn post(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  node_maps: NodeMapArray,
) -> Result<Value, Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url = base_url.to_owned() + "/smd/hsm/v2/Defaults/NodeMaps";

  let response = client
    .post(api_url)
    .bearer_auth(auth_token)
    .json(&node_maps)
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

pub async fn put(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  node_map: NodeMap,
) -> Result<(), Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url = format!(
    "{}/smd/hsm/v2/Defaults/NodeMaps/{}",
    base_url, node_map.id
  );

  let response = client
    .put(api_url)
    .bearer_auth(auth_token)
    .json(&node_map)
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

  Ok(())
}

pub async fn delete_all(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
) -> Result<Value, Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url = base_url.to_owned() + "/smd/hsm/v2/Defaults/NodeMaps";

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
  socks5_proxy: Option<&str>,
  xname: &str,
) -> Result<Value, Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url =
    format!("{}/smd/hsm/v2/Defaults/NodeMaps/{}", base_url, xname);

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
