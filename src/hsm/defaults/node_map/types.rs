use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeMapArray {
    #[serde(rename(serialize = "NodeMaps"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    node_maps: Option<Vec<NodeMap>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeMap {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "NID"))]
    pub nid: u32,
    #[serde(rename(serialize = "Role"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename(serialize = "SubRole"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_role: Option<String>,
}
