use std::collections::HashMap;

use crate::{
  Ochami,
  error::Error,
  hsm::group::types::{Group, Member},
  node::utils::validate_xnames_format_and_membership_agaisnt_single_hsm,
};

/// Bridge helper for Task 12: converts between the progenitor-generated
/// `Group100` and the internal hand-rolled `Group` via a JSON round-trip.
/// Both types share the HSM OpenAPI schema serde renames, so this is
/// safe. Task 13's slim re-exports collapse these into a direct alias
/// and this helper becomes dead code (to be removed then).
fn generated_to_internal_groups(
  gen: Vec<crate::hsm::generated::types::Group100>,
) -> Result<Vec<Group>, Error> {
  let v = serde_json::to_value(&gen).map_err(|e| Error::Message(e.to_string()))?;
  serde_json::from_value(v).map_err(|e| Error::Message(e.to_string()))
}

fn generated_to_internal_group(
  gen: crate::hsm::generated::types::Group100,
) -> Result<Group, Error> {
  let v = serde_json::to_value(&gen).map_err(|e| Error::Message(e.to_string()))?;
  serde_json::from_value(v).map_err(|e| Error::Message(e.to_string()))
}

/// Add a list of xnames to target HSM group
/// Returns the new list of nodes in target HSM group
pub async fn add_member(
  client: &Ochami,
  auth_token: &str,
  group_label: &str,
  new_member: &str,
) -> Result<Vec<String>, Error> {
  // Get HSM group from CSM
  let group_gen = client.hsm_group_get_one(auth_token, group_label).await?;
  let group = generated_to_internal_group(group_gen)?;

  // Update HSM group with new memebers
  let new_member = new_member.to_string();
  let _ = client
    .hsm_group_post_member(auth_token, group_label, &new_member)
    .await?;

  // Generate list of updated group members
  let mut members = group.get_members();
  members.push(new_member);

  Ok(members)
}

pub async fn get_member_vec_from_hsm_name_vec_2(
  client: &Ochami,
  auth_token: &str,
  hsm_name_vec: &[String],
) -> Result<Vec<String>, Error> {
  log::info!("Get xnames for HSM groups: {:?}", hsm_name_vec);

  let names_ref: Vec<&str> = hsm_name_vec.iter().map(|s| s.as_str()).collect();

  let group_vec_gen = client
    .hsm_group_get(auth_token, Some(&names_ref), None)
    .await
    .map_err(|e| Error::Message(e.to_string()))?;

  let group_vec = generated_to_internal_groups(group_vec_gen)?;

  let hsm_group_member_vec: Vec<String> = group_vec
    .into_iter()
    .flat_map(|group| group.get_members())
    .collect();

  Ok(hsm_group_member_vec)
}

// Returns a HashMap with keys being the hsm names/labels the user has access a curated list of xnames
// for each hsm name as values
pub async fn get_hsm_map_and_filter_by_hsm_name_vec(
  client: &Ochami,
  auth_token: &str,
  hsm_name_vec: &[&str],
) -> Result<HashMap<String, Vec<String>>, Error> {
  let hsm_group_vec_gen = client
    .hsm_group_get_all(auth_token)
    .await
    .map_err(|e| Error::Message(e.to_string()))?;

  let hsm_group_vec = generated_to_internal_groups(hsm_group_vec_gen)?;

  Ok(filter_by_hsm_group_and_convert_to_map(
    hsm_name_vec,
    &hsm_group_vec,
  ))
}

// Returns a HashMap with keys being the hsm names/labels the user has access a curated list of xnames
// for each hsm name as values
pub async fn get_hsm_group_map_and_filter_by_hsm_group_member_vec(
  client: &Ochami,
  shasta_token: &str,
  hsm_name_vec: &[&str],
) -> Result<HashMap<String, Vec<String>>, Error> {
  let hsm_group_vec_gen = client.hsm_group_get_all(shasta_token).await?;

  let hsm_group_vec = generated_to_internal_groups(hsm_group_vec_gen)?;

  Ok(filter_by_hsm_group_members_and_convert_to_map(
    hsm_name_vec,
    hsm_group_vec,
  ))
}

/// Given a list of HsmGroup struct and a list of Hsm group names, it will filter out those
/// not in the Hsm group names and convert from HsmGroup struct to HashMap
pub fn filter_by_hsm_group_and_convert_to_map(
  hsm_name_vec: &[&str],
  hsm_group_vec: &[Group],
) -> HashMap<String, Vec<String>> {
  let mut hsm_group_map: HashMap<String, Vec<String>> = HashMap::new();

  for hsm_group in hsm_group_vec {
    if hsm_name_vec.contains(&hsm_group.label.as_str()) {
      hsm_group_map.entry(hsm_group.label.clone()).or_insert(
        hsm_group
          .members
          .clone()
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
  client: &Ochami,
  auth_token: &str,
  group_label: &str,
  group_members_to_delete: &[&str],
  group_members_to_add: &[&str],
) -> Result<(), Error> {
  let group_gen = client.hsm_group_get_one(auth_token, group_label).await?;
  let group = generated_to_internal_group(group_gen)?;

  let mut group_members = group.members.unwrap().ids.unwrap();

  group_members
    .retain(|xname| group_members_to_delete.contains(&xname.as_str()));

  for xname in group_members_to_add {
    group_members.push(xname.to_string());
  }

  Ok(())
}

/// Moves list of xnames from parent to target HSM group
pub async fn migrate_hsm_members(
  client: &Ochami,
  shasta_token: &str,
  target_hsm_group_name: &str,
  parent_hsm_group_name: &str,
  new_target_hsm_members: &[&str],
  nodryrun: bool,
) -> Result<(Vec<String>, Vec<String>), Error> {
  // Check nodes are valid xnames and they belong to parent HSM group
  if !validate_xnames_format_and_membership_agaisnt_single_hsm(
    client,
    shasta_token,
    new_target_hsm_members,
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
      client,
      shasta_token,
      &[target_hsm_group_name.to_string()],
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
      client,
      shasta_token,
      &[parent_hsm_group_name.to_string()],
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
      let _ = client
        .hsm_group_post_member(shasta_token, target_hsm_group_name, xname)
        .await;

      let _ = client
        .hsm_group_delete_member(shasta_token, parent_hsm_group_name, xname)
        .await;
    }
  }

  // Silence unused-import warning for `Member` while callers still expect
  // the type re-exported here; kept for API compatibility during Task 12.
  let _ = std::marker::PhantomData::<Member>;

  Ok((target_hsm_group_member_vec, parent_hsm_group_member_vec))
}
