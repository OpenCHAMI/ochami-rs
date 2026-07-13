use regex::Regex;

use crate::{Ochami, hsm};

/// Validate xname is correct (it uses regex taken from HPE Cray CSM docs)
pub fn validate_xname_format(xname: &str) -> bool {
  let xname_re =
    Regex::new(r"^x\d{4}c[0-7]s([0-9]|[1-5][0-9]|6[0-4])b[0-1]n[0-7]$")
      .unwrap();

  xname_re.is_match(xname)
}

/// Validates a list of xnames.
///
/// Checks xnames strings are valid. If `hsm_group_name_opt` is provided,
/// also checks all xnames belong to that HSM group.
///
/// # Errors
///
/// Returns [`Error`] if the underlying HSM group lookup fails
/// (network, deserialization, or HSM error response).
pub async fn validate_xnames_format_and_membership_against_single_hsm(
  client: &Ochami,
  shasta_token: &str,
  xnames: &[&str],
  hsm_group_name_opt: Option<&str>,
) -> Result<bool, crate::error::Error> {
  let hsm_group_members: Vec<String> =
    if let Some(hsm_group_name) = hsm_group_name_opt {
      hsm::group::utils::get_member_vec_from_hsm_name_vec_2(
        client,
        shasta_token,
        &[hsm_group_name.to_string()],
      )
      .await?
    } else {
      Vec::new()
    };

  if xnames.iter().any(|&xname| {
    !validate_xname_format(xname)
      || (!hsm_group_members.is_empty()
        && !hsm_group_members.contains(&xname.to_string()))
  }) {
    return Ok(false);
  }

  Ok(true)
}

/// Deprecated alias for
/// [`validate_xnames_format_and_membership_against_single_hsm`].
///
/// The old name contained the typo "agaisnt" and panicked on network
/// errors (`.unwrap()` on the HSM lookup). The correctly-spelled
/// replacement returns `Result<bool, Error>` and propagates instead.
///
/// This shim preserves the old signature (`-> bool`) by mapping any
/// error into `false`, which matches the previous panic-or-false
/// behaviour closely enough for existing callers.
#[deprecated(
  since = "1.0.0-beta.17",
  note = "renamed to validate_xnames_format_and_membership_against_single_hsm and returns Result"
)]
pub async fn validate_xnames_format_and_membership_agaisnt_single_hsm(
  client: &Ochami,
  shasta_token: &str,
  xnames: &[&str],
  hsm_group_name_opt: Option<&str>,
) -> bool {
  validate_xnames_format_and_membership_against_single_hsm(
    client,
    shasta_token,
    xnames,
    hsm_group_name_opt,
  )
  .await
  .unwrap_or(false)
}
