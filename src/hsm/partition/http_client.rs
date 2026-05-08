use serde_json::Value;

use crate::error::Error;

use super::types::{Member, Partition};

pub async fn get(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  name: Option<&str>,
  tag: Option<&str>,
) -> Result<Vec<Partition>, Error> {
  let client_builder = reqwest::Client::builder()
    .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?)
    .use_rustls_tls();

  let client = match socks5_proxy {
    Some(proxy) => client_builder.proxy(reqwest::Proxy::all(proxy)?).build()?,
    None => client_builder.build()?,
  };

  let api_url: String = format!("{}/{}", base_url, "hsm/v2/partitions");

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
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  partition_name: &str,
) -> Result<Partition, Error> {
  let client_builder = reqwest::Client::builder()
    .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?)
    .use_rustls_tls();

  let client = match socks5_proxy {
    Some(proxy) => client_builder.proxy(reqwest::Proxy::all(proxy)?).build()?,
    None => client_builder.build()?,
  };

  let api_url: String =
    format!("{}/{}/{}", base_url, "hsm/v2/partitions", partition_name);

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

pub async fn get_names(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
) -> Result<Vec<String>, Error> {
  let client_builder = reqwest::Client::builder()
    .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?)
    .use_rustls_tls();

  let client = match socks5_proxy {
    Some(proxy) => client_builder.proxy(reqwest::Proxy::all(proxy)?).build()?,
    None => client_builder.build()?,
  };

  let api_url: String = format!("{}/{}", base_url, "hsm/v2/partitions/names");

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

pub async fn get_members(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  partition_name: &str,
) -> Result<Member, Error> {
  let client_builder = reqwest::Client::builder()
    .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?)
    .use_rustls_tls();

  let client = match socks5_proxy {
    Some(proxy) => client_builder.proxy(reqwest::Proxy::all(proxy)?).build()?,
    None => client_builder.build()?,
  };

  let api_url: String =
    format!("{}/hsm/v2/partitions/{}/members", base_url, partition_name);

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

pub async fn post(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  partition: Partition,
) -> Result<Value, Error> {
  let client_builder = reqwest::Client::builder()
    .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?)
    .use_rustls_tls();

  let client = match socks5_proxy {
    Some(proxy) => client_builder.proxy(reqwest::Proxy::all(proxy)?).build()?,
    None => client_builder.build()?,
  };

  let api_url: String = base_url.to_owned() + "/hsm/v2/partitions";

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

pub async fn post_members(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  partition_name: &str,
  members: Member,
) -> Result<Value, Error> {
  let client_builder = reqwest::Client::builder()
    .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?)
    .use_rustls_tls();

  let client = match socks5_proxy {
    Some(proxy) => client_builder.proxy(reqwest::Proxy::all(proxy)?).build()?,
    None => client_builder.build()?,
  };

  let api_url: String =
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
  partition_name: &str,
) -> Result<Value, Error> {
  let client_builder = reqwest::Client::builder()
    .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?)
    .use_rustls_tls();

  let client = match socks5_proxy {
    Some(proxy) => client_builder.proxy(reqwest::Proxy::all(proxy)?).build()?,
    None => client_builder.build()?,
  };

  let api_url: String =
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

pub async fn delete_member(
  base_url: &str,
  auth_token: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  partition_name: &str,
  xname: &str,
) -> Result<Value, Error> {
  let client_builder = reqwest::Client::builder()
    .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?)
    .use_rustls_tls();

  let client = match socks5_proxy {
    Some(proxy) => client_builder.proxy(reqwest::Proxy::all(proxy)?).build()?,
    None => client_builder.build()?,
  };

  let api_url: String = format!(
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
