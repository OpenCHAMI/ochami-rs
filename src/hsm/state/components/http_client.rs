use serde_json::Value;

use crate::error::Error;

use super::types::{
  Component, ComponentArray, ComponentPostByNidQuery, ComponentPostQuery,
};

pub async fn get(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  id: Option<&str>,
  r#type: Option<&str>,
  state: Option<&str>,
  flag: Option<&str>,
  role: Option<&str>,
  subrole: Option<&str>,
  enabled: Option<&str>,
  software_status: Option<&str>,
  subtype: Option<&str>,
  arch: Option<&str>,
  class: Option<&str>,
  nid: Option<&str>,
  nid_start: Option<&str>,
  nid_end: Option<&str>,
  partition: Option<&str>,
  group: Option<&str>,
  state_only: Option<bool>,
  flag_only: Option<bool>,
  role_only: Option<bool>,
  nid_only: Option<&str>,
) -> Result<ComponentArray, Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url = base_url.to_owned() + "/smd/hsm/v2/State/Components";

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
      software_status,
      subtype,
      arch,
      class,
      nid,
      nid_start,
      nid_end,
      partition,
      group,
      state_only.map(|value| value.to_string()).as_deref(),
      flag_only.map(|value| value.to_string()).as_deref(),
      role_only.map(|value| value.to_string()).as_deref(),
      nid_only,
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

pub async fn get_one(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  id: &str,
) -> Result<Component, Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url =
    format!("{}/smd/hsm/v2/State/Components/{}", base_url, id);

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

pub async fn get_by_nid(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  nid: &str,
) -> Result<Component, Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url = format!(
    "{}/smd/hsm/v2/State/Components/ByNID/{}",
    base_url, nid
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
        let error_payload = response.json::<Value>().await?;
        return Err(Error::OchamiError(error_payload));
      }
    }
  }

  response.json().await.map_err(Error::NetError)
}

pub async fn get_query(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  xname: &str,
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
  stateonly: Option<bool>,
  flagonly: Option<bool>,
  roleonly: Option<bool>,
  nidonly: Option<&str>,
) -> Result<ComponentArray, Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url = format!(
    "{}/smd/hsm/v2/State/Components/Query/{}",
    base_url, xname
  );

  let response = client
    .get(api_url)
    .query(&[
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
      stateonly.map(|value| value.to_string()).as_deref(),
      flagonly.map(|value| value.to_string()).as_deref(),
      roleonly.map(|value| value.to_string()).as_deref(),
      nidonly,
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

pub async fn post(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  component: Component,
) -> Result<Component, Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url = base_url.to_owned() + "/smd/hsm/v2/State/Components";

  let response = client
    .post(api_url)
    .bearer_auth(auth_token)
    .json(&component)
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

pub async fn post_query(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  component_query: ComponentPostQuery,
) -> Result<ComponentArray, Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url =
    base_url.to_owned() + "/smd/hsm/v2/State/Components/Query";

  let response = client
    .post(api_url)
    .bearer_auth(auth_token)
    .json(&component_query)
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

pub async fn post_by_nid_query(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  component_by_nid_query: ComponentPostByNidQuery,
) -> Result<Component, Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url =
    base_url.to_owned() + "/smd/hsm/v2/State/Components/ByNID/Query";

  let response = client
    .post(api_url)
    .bearer_auth(auth_token)
    .json(&component_by_nid_query)
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
  component: Component,
) -> Result<(), Error> {
  if component.id.is_none() {
    return Err(Error::Message(
      "ERROR - component.id not defined".to_string(),
    ));
  }

  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url = format!(
    "{}/smd/hsm/v2/State/Components/{}",
    base_url,
    component.id.as_ref().unwrap()
  );

  let response = client
    .put(api_url)
    .bearer_auth(auth_token)
    .json(&component)
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
  let api_url = base_url.to_owned() + "/smd/hsm/v2/State/Components";

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
  id: &str,
) -> Result<Value, Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url =
    format!("{}/smd/hsm/v2/State/Components/{}", base_url, id);

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
