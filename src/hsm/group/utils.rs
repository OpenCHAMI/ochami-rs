use std::collections::HashMap;

use crate::{
  Ochami,
  error::Error,
  hsm::group::types::{Group, GroupExt},
  node::utils::validate_xnames_format_and_membership_against_single_hsm,
};

/// Add a list of xnames to target HSM group
/// Returns the new list of nodes in target HSM group
pub async fn add_member(
  client: &Ochami,
  auth_token: &str,
  group_label: &str,
  new_member: &str,
) -> Result<Vec<String>, Error> {
  // Get HSM group from CSM (Group == Group100 via slim re-export)
  let group = client.hsm_group_get_one(auth_token, group_label).await?;

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

  let group_vec = client
    .hsm_group_get(auth_token, Some(&names_ref), None)
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
  client: &Ochami,
  auth_token: &str,
  hsm_name_vec: &[&str],
) -> Result<HashMap<String, Vec<String>>, Error> {
  let hsm_group_vec = client
    .hsm_group_get_all(auth_token)
    .await
    .map_err(|e| Error::Message(e.to_string()))?;

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
  let hsm_group_vec = client.hsm_group_get_all(shasta_token).await?;

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
    let label: &str = hsm_group.label.as_str();
    if hsm_name_vec.contains(&label) {
      hsm_group_map
        .entry(label.to_string())
        .or_insert_with(|| hsm_group.get_members());
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
    let members = hsm_group.get_members();
    if members
      .iter()
      .any(|member| hsm_group_member_vec.contains(&member.as_str()))
    {
      hsm_group_map.insert(hsm_group.label.as_str().to_string(), members);
    }
  }

  hsm_group_map
}

/// **Currently a no-op** (see roadmap Phase C).
///
/// This function is documented to remove `group_members_to_delete` and add
/// `group_members_to_add`, but the implementation both uses a backwards
/// `retain` predicate and never sends the resulting membership back to the
/// server. Every call returns `Ok(())` without touching HSM.
///
/// A correct implementation is deferred to a follow-up because it requires
/// a semantic decision (per-xname `post_member`/`delete_member` calls vs
/// a single PATCH). Do not rely on this function until it lands.
#[deprecated(
  since = "1.0.0-beta.17",
  note = "no-op — never sends changes to HSM. Use per-xname hsm_group_post_member/hsm_group_delete_member for now. See roadmap Phase C."
)]
pub async fn update_hsm_group_members(
  client: &Ochami,
  auth_token: &str,
  group_label: &str,
  group_members_to_delete: &[&str],
  group_members_to_add: &[&str],
) -> Result<(), Error> {
  let group = client.hsm_group_get_one(auth_token, group_label).await?;

  let mut group_members = group.get_members();

  group_members
    .retain(|xname| group_members_to_delete.contains(&xname.as_str()));

  for xname in group_members_to_add {
    group_members.push(xname.to_string());
  }

  Ok(())
}

/// Moves a list of xnames from `parent_hsm_group_name` to
/// `target_hsm_group_name`.
///
/// When `dry_run` is `true`, the planned target/parent membership lists
/// are logged and NO changes are sent to HSM. When `dry_run` is `false`,
/// each xname is posted to the target group and deleted from the parent
/// via `hsm_group_post_member` / `hsm_group_delete_member`.
///
/// Note: individual per-xname failures are currently swallowed (see
/// roadmap Phase B.1). The returned Ok tuple reflects the intended
/// target/parent membership lists, not what actually landed on the wire.
///
/// # Errors
///
/// Returns [`Error`] if the caller-supplied xnames are invalid, if
/// they don't belong to `parent_hsm_group_name`, or if the initial
/// HSM group membership lookups fail.
pub async fn migrate_hsm_members(
  client: &Ochami,
  shasta_token: &str,
  target_hsm_group_name: &str,
  parent_hsm_group_name: &str,
  new_target_hsm_members: &[&str],
  dry_run: bool,
) -> Result<(Vec<String>, Vec<String>), Error> {
  // Check nodes are valid xnames and they belong to parent HSM group
  if !validate_xnames_format_and_membership_against_single_hsm(
    client,
    shasta_token,
    new_target_hsm_members,
    Some(parent_hsm_group_name),
  )
  .await?
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

  // UPDATE HSM GROUP MEMBERS IN OCHAMI
  if dry_run {
    let target_hsm_group = serde_json::json!({
        "label": target_hsm_group_name,
        "description": "",
        "members": target_hsm_group_member_vec,
        "tags": []
    });

    log::info!(
      "dry-run: target HSM group: {}",
      serde_json::to_string(&target_hsm_group)?
    );

    let parent_hsm_group = serde_json::json!({
        "label": parent_hsm_group_name,
        "description": "",
        "members": parent_hsm_group_member_vec,
        "tags": []
    });

    log::info!(
      "dry-run: parent HSM group: {}",
      serde_json::to_string(&parent_hsm_group)?
    );

    log::info!("dry-run enabled, changes not persisted.");
  } else {
    let mut failures: Vec<String> = Vec::new();
    for xname in new_target_hsm_members {
      if let Err(e) = client
        .hsm_group_post_member(shasta_token, target_hsm_group_name, xname)
        .await
      {
        log::warn!(
          "post-member {xname} → {target_hsm_group_name} failed: {e}"
        );
        failures.push(format!("post {xname} → {target_hsm_group_name}: {e}"));
      }

      if let Err(e) = client
        .hsm_group_delete_member(shasta_token, parent_hsm_group_name, xname)
        .await
      {
        log::warn!(
          "delete-member {xname} ← {parent_hsm_group_name} failed: {e}"
        );
        failures
          .push(format!("delete {xname} ← {parent_hsm_group_name}: {e}"));
      }
    }
    if !failures.is_empty() {
      return Err(Error::Message(format!(
        "migrate_hsm_members: {} of {} xname operations failed: {}",
        failures.len(),
        new_target_hsm_members.len() * 2,
        failures.join("; ")
      )));
    }
  }

  Ok((target_hsm_group_member_vec, parent_hsm_group_member_vec))
}
