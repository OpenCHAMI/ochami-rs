use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscoveryInfo {
    #[serde(rename(serialize = "LastAttempt"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_attempt: Option<String>,
    #[serde(rename(serialize = "LastStatus"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_status: Option<String>,
    #[serde(rename(serialize = "RedfishVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    redfish_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishEndpoint {
    #[serde(rename(serialize = "ID"))]
    id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    #[serde(rename(serialize = "Name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename(serialize = "Hostname"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<String>,
    #[serde(rename(serialize = "Domain"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<String>,
    #[serde(rename(serialize = "FQDN"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    fqdn: Option<String>,
    #[serde(rename(serialize = "Enabled"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(rename(serialize = "UUID"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    uuid: Option<String>,
    #[serde(rename(serialize = "User"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
    #[serde(rename(serialize = "Password"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<String>,
    #[serde(rename(serialize = "UseSSDP"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    use_ssdp: Option<bool>,
    #[serde(rename(serialize = "MacRequired"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    mac_required: Option<bool>,
    #[serde(rename(serialize = "MACAddr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    mac_addr: Option<String>,
    #[serde(rename(serialize = "IPAddress"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<String>,
    #[serde(rename(serialize = "RediscoveryOnUpdate"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    rediscover_on_update: Option<bool>,
    #[serde(rename(serialize = "TemplateID"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    template_id: Option<String>,
    #[serde(rename(serialize = "DiscoveryInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    discovery_info: Option<DiscoveryInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishEndpointArray {
    #[serde(rename(serialize = "RedfishEndpoints"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    redfish_endpoints: Option<Vec<RedfishEndpoint>>,
}
