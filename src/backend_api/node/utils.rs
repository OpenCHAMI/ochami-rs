use regex::Regex;

use crate::backend_api::hsm;

/// Validate xname is correct (it uses regex taken from HPE Cray CSM docs)
pub fn validate_xname_format(xname: &str) -> bool {
    let xname_re = Regex::new(r"^x\d{4}c[0-7]s([0-9]|[1-5][0-9]|6[0-4])b[0-1]n[0-7]$").unwrap();

    xname_re.is_match(xname)
}

/// Validates a list of xnames.
/// Checks xnames strings are valid
/// If hsm_group_name_opt provided, then checks all xnames belongs to that hsm_group
pub async fn validate_xnames_format_and_membership_agaisnt_single_hsm(
    shasta_token: &str,
    shasta_base_url: &str,
    shasta_root_cert: &[u8],
    xnames: &[&str],
    hsm_group_name_opt: Option<&String>,
) -> bool {
    let hsm_group_members: Vec<String> = if let Some(hsm_group_name) = hsm_group_name_opt {
        hsm::group::utils::get_member_vec_from_hsm_name_vec_2(
            shasta_token,
            shasta_base_url,
            shasta_root_cert,
            vec![hsm_group_name.to_string()],
        )
        .await
        .unwrap()
    } else {
        Vec::new()
    };

    if xnames.iter().any(|&xname| {
        !validate_xname_format(xname)
            || (!hsm_group_members.is_empty() && !hsm_group_members.contains(&xname.to_string()))
    }) {
        return false;
    }

    true
}
