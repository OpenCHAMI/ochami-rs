use serde_json::{json, Value};

use crate::error::Error;

use super::types::PowerStatusAll;

pub async fn post(
  shasta_base_url: &str,
  shasta_token: &str,
  shasta_root_cert: &[u8],
  xname_vec_opt: Option<&[&str]>,
  power_state_filter_opt: Option<&str>,
  management_state_filter_opt: Option<&str>,
) -> Result<PowerStatusAll, Error> {
  let client = crate::http::build_client_no_tls(shasta_root_cert)?;

  let api_url = format!("{}/power-control/v1/power-status", shasta_base_url);

  let body = json!({
      "xname": xname_vec_opt.map(|xname_vec| xname_vec.iter().map(|&x| x.to_string()).collect::<Vec<String>>()).unwrap_or_default(),
      "powerStateFilter": power_state_filter_opt.unwrap_or(""),
      "managementStateFilter": management_state_filter_opt.unwrap_or(""),
  });

  let response = client
    .post(&api_url)
    .json(&body)
    .bearer_auth(shasta_token)
    .send()
    .await
    .map_err(|error| {
      println!("Failed POST query: {:?}", error);
      Error::NetError(error)
    })?;

  if response.status().is_success() {
    response.json().await.map_err(Error::NetError)
  } else {
    println!("response is failure");
    let payload = response.json::<Value>().await.map_err(Error::NetError)?;
    Err(Error::OchamiError(payload))
  }
}
