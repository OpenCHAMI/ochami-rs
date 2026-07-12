use std::time;

use crate::{
  error::Error,
  pcs::transitions::types::{
    Location, Operation, TransitionResponse, TransitionResponseList,
  },
};

use super::types::Transition;

pub async fn get(
  shasta_base_url: &str,
  shasta_token: &str,
  shasta_root_cert: &[u8],
) -> Result<Vec<TransitionResponse>, Error> {
  let client = crate::http::build_client(shasta_root_cert)?;
  let api_url = format!("{}/power-control/v1/transitions", shasta_base_url);

  log::debug!("PCS transition URL: {}", api_url);

  let response = client
    .get(api_url)
    .bearer_auth(shasta_token)
    .send()
    .await
    .map_err(Error::NetError)?;

  if response.status().is_success() {
    response
      .json::<TransitionResponseList>()
      .await
      .map_err(Error::NetError)
      .map(|transition_list| transition_list.transitions)
  } else {
    let payload = response.text().await.map_err(Error::NetError)?;
    Err(Error::Message(payload))
  }
}

pub async fn get_by_id(
  shasta_token: &str,
  shasta_base_url: &str,
  shasta_root_cert: &[u8],
  id: &str,
) -> Result<TransitionResponse, Error> {
  let client = crate::http::build_client(shasta_root_cert)?;
  let api_url =
    format!("{}/power-control/v1/transitions/{}", shasta_base_url, id);

  let response = client
    .get(api_url)
    .bearer_auth(shasta_token)
    .send()
    .await
    .map_err(Error::NetError)?;

  if response.status().is_success() {
    let payload = response.json().await.map_err(Error::NetError);
    log::debug!("PCS transition details\n{:#?}", payload);
    payload
  } else {
    let payload = response.text().await.map_err(Error::NetError)?;
    Err(Error::Message(payload))
  }
}

pub async fn post(
  shasta_base_url: &str,
  shasta_token: &str,
  shasta_root_cert: &[u8],
  operation: &str,
  xname_vec: &Vec<String>,
) -> Result<TransitionResponse, Error> {
  log::info!("Create PCS transition '{}' on {:?}", operation, xname_vec);

  let location_vec: Vec<Location> = xname_vec
    .iter()
    .map(|xname| Location {
      xname: xname.to_string(),
      deputy_key: None,
    })
    .collect();

  let request_payload = Transition {
    operation: Operation::from_str(operation)?,
    task_deadline_minutes: None,
    location: location_vec,
  };

  let client = crate::http::build_client(shasta_root_cert)?;
  let api_url = shasta_base_url.to_owned() + "/power-control/v1/transitions";

  let response = client
    .post(api_url)
    .json(&request_payload)
    .bearer_auth(shasta_token)
    .send()
    .await
    .map_err(Error::NetError)?;

  if response.status().is_success() {
    Ok(response.json::<TransitionResponse>().await.unwrap())
  } else {
    let payload = response.text().await.map_err(Error::NetError)?;
    Err(Error::Message(payload))
  }
}

pub async fn post_block(
  shasta_base_url: &str,
  shasta_token: &str,
  shasta_root_cert: &[u8],
  operation: &str,
  xname_vec: &Vec<String>,
) -> Result<TransitionResponse, Error> {
  let node_reset = post(
    shasta_base_url,
    shasta_token,
    shasta_root_cert,
    operation,
    xname_vec,
  )
  .await?;

  log::info!("PCS transition ID: {}", node_reset.transition_id);

  wait_to_complete(
    shasta_base_url,
    shasta_token,
    shasta_root_cert,
    &node_reset.transition_id,
  )
  .await
}

pub async fn wait_to_complete(
  shasta_base_url: &str,
  shasta_token: &str,
  shasta_root_cert: &[u8],
  transition_id: &str,
) -> Result<TransitionResponse, Error> {
  let mut transition: TransitionResponse = get_by_id(
    shasta_token,
    shasta_base_url,
    shasta_root_cert,
    transition_id,
  )
  .await?;

  let mut i = 1;
  let max_attempt = 300;

  while i <= max_attempt && transition.transition_status != "completed" {
    transition = get_by_id(
      shasta_token,
      shasta_base_url,
      shasta_root_cert,
      transition_id,
    )
    .await?;

    eprintln!(
       "Power '{}' summary - status: {}, failed: {}, in-progress: {}, succeeded: {}, total: {}. Attempt {} of {}",
       transition.operation, transition.transition_status, transition.task_counts.failed, transition.task_counts.in_progress, transition.task_counts.succeeded, transition.task_counts.total, i, max_attempt
    );

    tokio::time::sleep(time::Duration::from_secs(3)).await;
    i += 1;
  }

  Ok(transition)
}
