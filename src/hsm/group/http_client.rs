use serde_json::Value;

use crate::{error::Error, hsm::group::types::Member};

use super::types::{Group, Members};

pub async fn get_all(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
) -> Result<Vec<Group>, Error> {
  get(base_url, auth_token, root_cert, None, None).await
}

pub async fn get(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  label_vec_opt: Option<&[String]>,
  tag_vec_opt: Option<&[String]>,
) -> Result<Vec<Group>, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = format!("{}/hsm/v2/groups", base_url);

  let mut query = Vec::new();
  if let Some(label_vec) = label_vec_opt {
    for label in label_vec {
      query.push(("group", label));
    }
  }
  if let Some(tag_vec) = tag_vec_opt {
    for tag in tag_vec {
      query.push(("tag", tag));
    }
  }

  let response = client
    .get(api_url)
    .query(query.as_slice())
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
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  group_label: &str,
) -> Result<Group, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = format!("{}/hsm/v2/groups/{}", base_url, group_label);

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

pub async fn get_labels(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
) -> Result<Vec<String>, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = format!("{}/hsm/v2/groups/labels", base_url);

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

pub async fn get_members(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  group_label: &str,
) -> Result<Members, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url =
    format!("{}/hsm/v2/groups/{}/members", base_url, group_label);

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
  group: Group,
) -> Result<String, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = format!("{}/hsm/v2/groups", base_url);

  let response = client
    .post(api_url)
    .bearer_auth(auth_token)
    .json(&group)
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

  response
    .text()
    .await
    .map_err(|e| Error::Message(e.to_string()))
}

pub async fn post_member(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  group_label: &str,
  member: Member,
) -> Result<Value, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url =
    format!("{}/hsm/v2/groups/{}/members", base_url, group_label);

  let response = client
    .post(api_url)
    .bearer_auth(auth_token)
    .json(&member)
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

  response.json().await.map_err(|e| Error::Message(e.to_string()))
}

pub async fn delete_one(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  group_label: &str,
) -> Result<Value, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = format!("{}/hsm/v2/groups/{}", base_url, group_label);

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

pub async fn delete_member(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  group_label: &str,
  xname: &str,
) -> Result<(), Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = format!(
    "{}/hsm/v2/groups/{}/members/{}",
    base_url, group_label, xname
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

  Ok(())
}
