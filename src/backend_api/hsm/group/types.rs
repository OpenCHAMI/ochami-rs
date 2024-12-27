use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Member>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "exclusiveGroup"))]
    pub exclusive_group: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Member {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
}

impl From<backend_dispatcher::types::HsmGroup> for Group {
    fn from(value: backend_dispatcher::types::HsmGroup) -> Self {
        let mut member_vec = Vec::new();
        let member_vec_backend = value.members.unwrap().ids.unwrap();

        for member in member_vec_backend {
            member_vec.push(member);
        }

        let members = Member {
            ids: Some(member_vec),
        };

        Group {
            label: value.label,
            description: value.description,
            tags: value.tags,
            members: Some(members),
            exclusive_group: value.exclusive_group,
        }
    }
}

impl Into<backend_dispatcher::types::HsmGroup> for Group {
    fn into(self) -> backend_dispatcher::types::HsmGroup {
        let mut member_vec = Vec::new();
        let member_vec_backend = self.members.unwrap().ids.unwrap();

        for member in member_vec_backend {
            member_vec.push(member);
        }

        let members = backend_dispatcher::types::Member {
            ids: Some(member_vec),
        };

        backend_dispatcher::types::HsmGroup {
            label: self.label,
            description: self.description,
            tags: self.tags,
            members: Some(members),
            exclusive_group: self.exclusive_group,
        }
    }
}
