use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeMapArray {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "NodeMaps"))]
    pub node_maps: Option<NodeMap>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "ID"))]
    pub id: Option<String>,
    /// put http request payload does not use "ID" field since it is
    /// part of the URL
    #[serde(rename(serialize = "NID"))]
    pub nid: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "Role"))]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "SubRole"))]
    pub sub_role: Option<String>,
}
