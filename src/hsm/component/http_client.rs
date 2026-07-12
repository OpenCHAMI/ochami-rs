use serde_json::Value;

use crate::{error::Error, hsm::state::components::types::Component};

use super::types::{
  ComponentArray, ComponentArrayPostArray, ComponentArrayPostByNidQuery,
  ComponentArrayPostQuery, ComponentPut,
};

pub async fn get_all(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
) -> Result<ComponentArray, Error> {
  get(
    base_url, root_cert, auth_token, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None,
  )
  .await
}

pub async fn get_all_nodes(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  nid_only: Option<&str>,
) -> Result<ComponentArray, Error> {
  get(
    base_url,
    root_cert,
    auth_token,
    None,
    Some("Node"),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    nid_only,
  )
  .await
}

/// Get all components.
/// NOTE: nid is a comma separated list of NIDs like "1,2,3".
pub async fn get(
  base_url: &str,
  root_cert: &[u8],
  auth_token: &str,
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
  state_only: Option<&str>,
  flag_only: Option<&str>,
  role_only: Option<&str>,
  nid_only: Option<&str>,
) -> Result<ComponentArray, Error> {
  let client = crate::http::build_client(root_cert)?;

  let mut nid_vec_query = nid.map(|nids| {
    nids
      .split(",")
      .map(|nid| ("nid", Some(nid)))
      .collect::<Vec<(&str, Option<&str>)>>()
  });

  let mut query_params = vec![
    ("id", id),
    ("type", r#type),
    ("state", state),
    ("flag", flag),
    ("role", role),
    ("subrole", subrole),
    ("enabled", enabled),
    ("softwarestatus", software_status),
    ("subtype", subtype),
    ("arch", arch),
    ("class", class),
    ("nidstart", nid_start),
    ("nidend", nid_end),
    ("partition", partition),
    ("group", group),
    ("stateonly", state_only),
    ("flagonly", flag_only),
    ("roleonly", role_only),
    ("nidonly", nid_only),
  ];

  if let Some(mut nid_vec_query) = nid_vec_query.take() {
    query_params.append(&mut nid_vec_query);
  }

  let api_url = format!("{}/hsm/v2/State/Components", base_url);

  let response = client
    .get(api_url)
    .query(&query_params)
    .bearer_auth(auth_token)
    .send()
    .await?;

  if !response.status().is_success() {
    match response.status() {
      reqwest::StatusCode::UNAUTHORIZED => {
        let error_payload = response.text().await?;
        return Err(Error::Message(error_payload));
      }
      _ => {
        let error_payload = response.text().await?;
        return Err(Error::Message(error_payload));
      }
    }
  }

  response
    .json::<ComponentArray>()
    .await
    .map_err(Error::NetError)
}

pub async fn get_one(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  xname: &str,
) -> Result<Component, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = format!("{}/hsm/v2/State/Components/{}", base_url, xname);

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
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  component: ComponentArrayPostArray,
) -> Result<(), Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = base_url.to_owned() + "/hsm/v2/State/Components";

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

  Ok(())
}

pub async fn post_query(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  component: ComponentArrayPostQuery,
) -> Result<ComponentArray, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = base_url.to_owned() + "/hsm/v2/State/Components";

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

pub async fn post_bynid_query(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  component: ComponentArrayPostByNidQuery,
) -> Result<ComponentArray, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = base_url.to_owned() + "/hsm/v2/State/Components/ByNID/Query";

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

pub async fn put(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  xname: &str,
  component: ComponentPut,
) -> Result<(), Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = format!("{}/hsm/v2/State/Components/{}", base_url, xname);

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

  response.json().await.map_err(Error::NetError)
}

pub async fn delete_one(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  xname: &str,
) -> Result<Value, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = format!("{}/hsm/v2/State/Components/{}", base_url, xname);

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

pub async fn delete(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
) -> Result<Value, Error> {
  let client = crate::http::build_client(root_cert)?;
  // NOTE: pre-existing typo in the URL ("Componnets")
  let api_url = format!("{}/hsm/v2/State/Componnets", base_url);

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
