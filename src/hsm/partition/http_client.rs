use serde_json::Value;

use crate::error::Error;

use super::types::{Member, Partition};

pub async fn get(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  name: Option<&str>,
  tag: Option<&str>,
) -> Result<Vec<Partition>, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = format!("{}/hsm/v2/partitions", base_url);

  let response = client
    .get(api_url)
    .query(&[name, tag])
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
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  partition_name: &str,
) -> Result<Partition, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url =
    format!("{}/hsm/v2/partitions/{}", base_url, partition_name);

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

pub async fn get_names(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
) -> Result<Vec<String>, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = format!("{}/hsm/v2/partitions/names", base_url);

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
  partition_name: &str,
) -> Result<Member, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url =
    format!("{}/hsm/v2/partitions/{}/members", base_url, partition_name);

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
  partition: Partition,
) -> Result<Value, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = base_url.to_owned() + "/hsm/v2/partitions";

  let response = client
    .post(api_url)
    .bearer_auth(auth_token)
    .json(&partition)
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

pub async fn post_members(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  partition_name: &str,
  members: Member,
) -> Result<Value, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url =
    format!("{}/hsm/v2/partitions/{}/members", base_url, partition_name);

  let response = client
    .post(api_url)
    .bearer_auth(auth_token)
    .json(&members)
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
  partition_name: &str,
) -> Result<Value, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url =
    format!("{}/hsm/v2/partitions/{}", base_url, partition_name);

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
  partition_name: &str,
  xname: &str,
) -> Result<Value, Error> {
  let client = crate::http::build_client(root_cert)?;
  let api_url = format!(
    "{}/hsm/v2/partitions/{}/members/{}",
    base_url, partition_name, xname
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
