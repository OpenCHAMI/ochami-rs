use std::collections::HashMap;

use crate::{
  error::Error,
  hsm::{self, group::types::Member},
  node::utils::validate_xnames_format_and_membership_agaisnt_single_hsm,
};

use crate::hsm::group::{http_client, types::Group};

use super::http_client::{delete_member, post_member};

/// Add a list of xnames to target HSM group
/// Returns the new list of nodes in target HSM group
pub async fn add_member(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  group_label: &str,
  new_member: &str,
) -> Result<Vec<String>, Error> {
  // Get HSM group from CSM
  let group = crate::hsm::group::http_client::get_one(
    base_url,
    auth_token,
    root_cert,
    group_label,
  )
  .await?;

  // Update HSM group with new memebers
  // Create Member struct
  let new_member = new_member.to_string();
  let member = crate::hsm::group::types::Member {
    id: Some(new_member.clone()),
  };

  // Update HSM group in CSM
  let _ = crate::hsm::group::http_client::post_member(
    auth_token,
    base_url,
    root_cert,
    group_label,
    member,
  )
  .await?;

  // Generate list of updated group members
  group.get_members().push(new_member);

  Ok(group.get_members())

  /* // get list of target HSM group members
  let mut target_hsm_group_member_vec: Vec<String> =
      hsm::group::http_client::get_members(base_url, auth_token, root_cert, group_label)
          .await
          .map(|member| member.ids.unwrap())?;

  // merge HSM group list with the list of xnames provided by the user
  target_hsm_group_member_vec.extend(members.iter().map(|xname| xname.to_string()));

  target_hsm_group_member_vec.sort();
  target_hsm_group_member_vec.dedup();

  // *********************************************************************************************************
  // UPDATE HSM GROUP MEMBERS IN CSM
  if dryrun {
      println!(
          "Add following nodes to HSM group {}:\n{:?}",
          group_label, members
      );

      println!("dry-run enabled, changes not persisted.");
  } else {
      for xname in members {
          let member = Member {
              ids: Some(vec![xname.to_string()]),
          };
          let _ = post_members(auth_token, base_url, root_cert, group_label, member).await;
      }
  }

  Ok(target_hsm_group_member_vec) */
}

pub async fn get_member_vec_from_hsm_name_vec_2(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  hsm_name_vec: &[&str],
) -> Result<Vec<String>, Error> {
  log::info!("Get xnames for HSM groups: {:?}", hsm_name_vec);

  /* let start = Instant::now();

  let mut hsm_group_member_vec: Vec<String> = Vec::new();

  let pipe_size = 10;

  let mut tasks = tokio::task::JoinSet::new();

  let sem = Arc::new(Semaphore::new(pipe_size)); // CSM 1.3.1 higher number of concurrent tasks won't
                                                 //
  for hsm_name in hsm_name_vec {
      let auth_token_string = auth_token.to_string();
      let base_url_string = base_url.to_string();
      let root_cert_vec = root_cert.to_vec();

      let permit = Arc::clone(&sem).acquire_owned().await;

      tasks.spawn(async move {
          let _permit = permit; // Wait semaphore to allow new tasks https://github.com/tokio-rs/tokio/discussions/2648#discussioncomment-34885

          let group: Result<Group, Error> = http_client::get_one(
              &base_url_string,
              &auth_token_string,
              &root_cert_vec,
              &hsm_name,
          )
          .await;

          group
      });
  }

  while let Some(message) = tasks.join_next().await {
      match message {
          Ok(Ok(hsm_group_vec)) => {
              let mut hsm_grop_members = hsm_group_vec
                  .first()
                  .unwrap()
                  .members
                  .as_ref()
                  .unwrap()
                  .ids
                  .clone()
                  .unwrap();

              hsm_group_member_vec.append(&mut hsm_grop_members);
          }
          Ok(Err(error)) => log::warn!("{error}"),
          Err(error) => {
              return Err(Error::Message(error.to_string()));
          }
      }
  }

  let duration = start.elapsed();
  log::info!("Time elapsed to get HSM members is: {:?}", duration); */

  let hsm_group_name_vec: Vec<&str> =
    hsm_name_vec.iter().map(|x| &**x).collect();

  let group_vec = http_client::get(
    base_url,
    auth_token,
    root_cert,
    Some(&hsm_group_name_vec),
    None,
  )
  .await
  .map_err(|e| Error::Message(e.to_string()))?;

  let hsm_group_member_vec: Vec<String> = group_vec
    .into_iter()
    .flat_map(|group| group.get_members())
    .collect();

  Ok(hsm_group_member_vec)
}

// Returns a HashMap with keys being the hsm names/labels the user has access a curated list of xnames
// for each hsm name as values
pub async fn get_hsm_map_and_filter_by_hsm_name_vec(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  hsm_name_vec: Vec<&str>,
) -> Result<HashMap<String, Vec<String>>, Error> {
  let hsm_group_vec = http_client::get_all(base_url, auth_token, root_cert)
    .await
    .map_err(|e| Error::Message(e.to_string()))?;

  Ok(filter_by_hsm_group_and_convert_to_map(
    hsm_name_vec,
    hsm_group_vec,
  ))
}

// Returns a HashMap with keys being the hsm names/labels the user has access a curated list of xnames
// for each hsm name as values
pub async fn get_hsm_group_map_and_filter_by_hsm_group_member_vec(
  shasta_token: &str,
  shasta_base_url: &str,
  shasta_root_cert: &[u8],
  hsm_name_vec: &[&str],
) -> Result<HashMap<String, Vec<String>>, Error> {
  let hsm_group_vec =
    http_client::get_all(shasta_base_url, shasta_token, shasta_root_cert)
      .await?;

  Ok(filter_by_hsm_group_members_and_convert_to_map(
    hsm_name_vec,
    hsm_group_vec,
  ))
}

/// Given a list of HsmGroup struct and a list of Hsm group names, it will filter out those
/// not in the Hsm group names and convert from HsmGroup struct to HashMap
pub fn filter_by_hsm_group_and_convert_to_map(
  hsm_name_vec: Vec<&str>,
  hsm_group_vec: Vec<Group>,
) -> HashMap<String, Vec<String>> {
  let mut hsm_group_map: HashMap<String, Vec<String>> = HashMap::new();

  for hsm_group in hsm_group_vec {
    if hsm_name_vec.contains(&hsm_group.label.as_str()) {
      hsm_group_map.entry(hsm_group.label).or_insert(
        hsm_group
          .members
          .and_then(|members| Some(members.ids.unwrap_or_default()))
          .unwrap(),
      );
    }
  }

  hsm_group_map
}

/// Given a list of HsmGroup struct and a list of Hsm group members, it will filter out those
/// not in the Hsm group names and convert from HsmGroup struct to HashMap
pub fn filter_by_hsm_group_members_and_convert_to_map(
  hsm_group_member_vec: &[&str],
  hsm_group_vec: Vec<Group>,
) -> HashMap<String, Vec<String>> {
  let mut hsm_group_map: HashMap<String, Vec<String>> = HashMap::new();

  for hsm_group in hsm_group_vec {
    if hsm_group
      .get_members()
      .iter()
      .any(|member| hsm_group_member_vec.contains(&member.as_str()))
    {
      hsm_group_map
        .entry(hsm_group.label)
        .or_insert(hsm_group.members.and_then(|members| members.ids).unwrap());
    }
  }

  hsm_group_map
}

/// Receives 2 lists of xnames old xnames to remove from parent HSM group and new xhanges to add to target HSM group, and does just that
pub async fn update_hsm_group_members(
  auth_token: &str,
  base_url: &str,
  root_cert: &[u8],
  group_label: &str,
  group_members_to_delete: &Vec<String>,
  group_members_to_add: &Vec<String>,
) -> Result<(), Error> {
  let group = hsm::group::http_client::get_one(
    base_url,
    auth_token,
    root_cert,
    group_label,
  )
  .await?;

  let mut group_members = group.members.unwrap().ids.unwrap();

  group_members.retain(|xname| group_members_to_delete.contains(xname));

  for xname in group_members_to_add {
    group_members.push(xname.to_string());
  }

  Ok(())
}

/// Moves list of xnames from parent to target HSM group
pub async fn migrate_hsm_members(
  shasta_token: &str,
  shasta_base_url: &str,
  shasta_root_cert: &[u8],
  target_hsm_group_name: &str,
  parent_hsm_group_name: &str,
  new_target_hsm_members: Vec<&str>,
  nodryrun: bool,
) -> Result<(Vec<String>, Vec<String>), Error> {
  // Check nodes are valid xnames and they belong to parent HSM group
  if !validate_xnames_format_and_membership_agaisnt_single_hsm(
    shasta_token,
    shasta_base_url,
    shasta_root_cert,
    new_target_hsm_members.as_slice(),
    Some(&parent_hsm_group_name.to_string()),
  )
  .await
  {
    let error_msg =
      format!("Nodes '{}' not valid", new_target_hsm_members.join(", "));
    return Err(Error::Message(error_msg));
  }

  // get list of target HSM group members
  let mut target_hsm_group_member_vec: Vec<String> =
    get_member_vec_from_hsm_name_vec_2(
      shasta_token,
      shasta_base_url,
      shasta_root_cert,
      &[target_hsm_group_name],
    )
    .await?;

  // merge HSM group list with the list of xnames provided by the user
  target_hsm_group_member_vec
    .extend(new_target_hsm_members.iter().map(|xname| xname.to_string()));

  target_hsm_group_member_vec.sort();
  target_hsm_group_member_vec.dedup();

  // get list of parent HSM group members
  let mut parent_hsm_group_member_vec: Vec<String> =
    get_member_vec_from_hsm_name_vec_2(
      shasta_token,
      shasta_base_url,
      shasta_root_cert,
      &[parent_hsm_group_name],
    )
    .await?;

  parent_hsm_group_member_vec.retain(|parent_member| {
    !target_hsm_group_member_vec.contains(parent_member)
  });

  parent_hsm_group_member_vec.sort();
  parent_hsm_group_member_vec.dedup();

  // *********************************************************************************************************
  // UPDATE HSM GROUP MEMBERS IN CSM
  if !nodryrun {
    let target_hsm_group = serde_json::json!({
        "label": target_hsm_group_name,
        "decription": "",
        "members": target_hsm_group_member_vec,
        "tags": []
    });

    println!(
      "Target HSM group:\n{}",
      serde_json::to_string_pretty(&target_hsm_group).unwrap()
    );

    let parent_hsm_group = serde_json::json!({
        "label": parent_hsm_group_name,
        "decription": "",
        "members": parent_hsm_group_member_vec,
        "tags": []
    });

    println!(
      "Parent HSM group:\n{}",
      serde_json::to_string_pretty(&parent_hsm_group).unwrap()
    );

    println!("dry-run enabled, changes not persisted.");
  } else {
    for xname in new_target_hsm_members {
      let member = Member {
        id: Some(xname.to_string()),
      };

      let _ = post_member(
        shasta_token,
        shasta_base_url,
        shasta_root_cert,
        target_hsm_group_name,
        member,
      )
      .await;

      let _ = delete_member(
        shasta_token,
        shasta_base_url,
        shasta_root_cert,
        parent_hsm_group_name,
        xname,
      )
      .await;
    }
  }

  Ok((target_hsm_group_member_vec, parent_hsm_group_member_vec))
}
