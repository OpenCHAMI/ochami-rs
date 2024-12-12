use std::collections::HashMap;

use serde_json::Value;

use super::BootParameters;

pub fn convert_kernel_params_to_map(kernel_params: &str) -> HashMap<String, String> {
    kernel_params
        .split_whitespace()
        .map(|kernel_param| {
            let (key_str, value_str) = kernel_param.split_once('=').unwrap_or((kernel_param, ""));

            let key = key_str.to_string();
            let value = value_str.to_string();

            (key, value)
        })
        .collect()
}

pub fn find_boot_params_related_to_node(
    node_boot_params_list: &[BootParameters],
    node: &String,
) -> Option<BootParameters> {
    node_boot_params_list
        .iter()
        .find(|node_boot_param| node_boot_param.hosts.iter().any(|host| host.eq(node)))
        .cloned()
}

/// Get Image ID from kernel field
#[deprecated(
    since = "1.26.6",
    note = "Please convert from serde_json::Value to struct BootParameters use function `BootParameters::get_boot_image` instead"
)]
pub fn get_image_id(node_boot_params: &Value) -> String {
    serde_json::from_value::<BootParameters>(node_boot_params.clone())
        .unwrap()
        .get_boot_image()
}
