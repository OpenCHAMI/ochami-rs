use serde_json::Value;

use crate::error::Error;

use super::types::{ComponentEthernetInterface, IpAddressMapping};

pub async fn post(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  eht_interface: ComponentEthernetInterface,
) -> Result<(), Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url = format!("{}/hsm/v2/Inventory/EthernetInterfaces", base_url);

  let response = client
    .post(api_url)
    .bearer_auth(auth_token)
    .json(&eht_interface)
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
        dbg!(&error_payload);
        return Err(Error::Message(error_payload));
      }
    }
  }

  response.json().await.map_err(Error::NetError)
}

pub async fn post_ip_addresses(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  eht_interface: ComponentEthernetInterface,
) -> Result<Value, Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url = format!(
    "{}/hsm/v2/Inventory/EthernetInterfaces/{}/IPAddresses",
    base_url,
    eht_interface.component_id.as_ref().unwrap()
  );

  let response = client
    .post(api_url)
    .bearer_auth(auth_token)
    .json(&eht_interface)
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

pub async fn get(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  mac_address: Option<&str>,
  ip_address: Option<&str>,
  network: Option<&str>,
  component_id: Option<&str>,
  r#type: Option<&str>,
  older_than: Option<&str>,
  newer_than: Option<&str>,
) -> Result<Vec<ComponentEthernetInterface>, Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url =
    base_url.to_owned() + "/hsm/v2/Inventory/EthernetInterfaces";

  let response = client
    .get(api_url)
    .query(&[
      ("MACAddress", mac_address),
      ("IPAddress", ip_address),
      ("Network", network),
      ("ComponentID", component_id),
      ("Type", r#type),
      ("OlderThan", older_than),
      ("NewerThan", newer_than),
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
  socks5_proxy: Option<&str>,
  eth_interface_id: &str,
) -> Result<ComponentEthernetInterface, Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url = format!(
    "{}/hsm/v2/Inventory/EthernetInterfaces/{}",
    base_url, eth_interface_id
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

pub async fn patch(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  eth_interface_id: &str,
  description: Option<&str>,
  ip_address_mapping: (&str, &str),
) -> Result<Value, Error> {
  let ip_address = ip_address_mapping.0;
  let network = ip_address_mapping.1;
  let cei = ComponentEthernetInterface {
    id: None,
    description: description.map(|value| value.to_string()),
    mac_address: None,
    ip_addresses: Some(vec![IpAddressMapping {
      ip_address: ip_address.to_string(),
      network: Some(network.to_string()),
    }]),
    last_update: None,
    component_id: Some(eth_interface_id.to_string()),
    parent_hms_type: None,
  };

  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url = format!(
    "{}/hsm/v2/Inventory/EthernetInterfaces/{}",
    base_url, eth_interface_id
  );

  let response = client
    .patch(api_url)
    .query(&[("ethInterfaceID", ip_address), ("ipAddress", ip_address)])
    .bearer_auth(auth_token)
    .json(&cei)
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

pub async fn delete_all(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
) -> Result<Value, Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url =
    format!("{}/hsm/v2/Inventory/EthernetInterfaces", base_url);

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
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  eth_interface_id: &str,
) -> Result<Value, Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url = format!(
    "{}/hsm/v2/Inventory/EthernetInterfaces/{}",
    base_url, eth_interface_id
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

pub async fn get_ip_addresses(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  eth_interface_id: &str,
) -> Result<Vec<IpAddressMapping>, Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url = format!(
    "{}/hsm/v2/Inventory/EthernetInterfaces/{}/IPAddresses",
    base_url, eth_interface_id
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

pub async fn delete_ip_address(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
  _group_label: &str,
  eth_interface_id: &str,
  ip_address: &str,
) -> Result<Value, Error> {
  let client = crate::http::build_client(root_cert, socks5_proxy)?;
  let api_url = format!(
    "{}/hsm/v2/Inventory/EthernetInterfaces/{}/IpAddress/{}",
    base_url, eth_interface_id, ip_address
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
