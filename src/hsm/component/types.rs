//! Slim re-exports from the progenitor-generated HSM types, plus
//! `From`/`Into` conversions between the generated types and the
//! `manta_backend_dispatcher` front-end types.
//!
//! Task 13 (Phase 5) replaced the hand-written duplicate types here
//! with direct aliases to the generated equivalents. Design spec
//! Section 4 (Phase 5) — Types migration.

use std::str::FromStr;

pub use crate::hsm::generated::types::{
  Component100Component as Component,
  Component100ComponentCreate as ComponentCreate, Component100Put as ComponentPut,
  ComponentArrayComponentArray as ComponentArray, ComponentArrayPostArray,
  ComponentArrayPostByNidQuery, ComponentArrayPostQuery, HmsArch100, HmsClass100,
  HmsFlag100, HmsRole100, HmsState100, HmsSubRole100, HmsType100, NetType100,
  NidRange100, XName100, XNameForQuery100, XNamePartition100, XNameRw100,
};

use manta_backend_dispatcher::types::{
  Component as FrontEndComponent,
  ComponentArrayPostArray as FrontEndComponentArrayPostArray,
  ComponentCreate as FrontEndComponentCreate, NodeMetadataArray,
};

// ---------------------------------------------------------------------------
// Component <-> FrontEndComponent
// ---------------------------------------------------------------------------

impl From<FrontEndComponent> for Component {
  fn from(value: FrontEndComponent) -> Self {
    Component {
      id: value.id.map(XName100),
      type_: value.r#type.and_then(|s| HmsType100::from_str(&s).ok()),
      state: value.state.and_then(|s| HmsState100::from_str(&s).ok()),
      flag: value.flag.and_then(|s| HmsFlag100::from_str(&s).ok()),
      enabled: value.enabled,
      software_status: value.software_status,
      role: value.role.map(HmsRole100),
      sub_role: value.sub_role.map(HmsSubRole100),
      nid: value.nid.map(|n| n as i64),
      subtype: value.subtype,
      net_type: value.net_type.and_then(|s| NetType100::from_str(&s).ok()),
      arch: value.arch.and_then(|s| HmsArch100::from_str(&s).ok()),
      class: value.class.and_then(|s| HmsClass100::from_str(&s).ok()),
      reservation_disabled: value.reservation_disabled,
      locked: value.locked,
    }
  }
}

impl From<Component> for FrontEndComponent {
  fn from(value: Component) -> Self {
    FrontEndComponent {
      id: value.id.map(|x| x.0),
      r#type: value.type_.map(|t| t.to_string()),
      state: value.state.map(|s| s.to_string()),
      flag: value.flag.map(|f| f.to_string()),
      enabled: value.enabled,
      software_status: value.software_status,
      role: value.role.map(|r| r.0),
      sub_role: value.sub_role.map(|s| s.0),
      nid: value.nid.map(|n| n as usize),
      subtype: value.subtype,
      net_type: value.net_type.map(|n| n.to_string()),
      arch: value.arch.map(|a| a.to_string()),
      class: value.class.map(|c| c.to_string()),
      reservation_disabled: value.reservation_disabled,
      locked: value.locked,
    }
  }
}

// ---------------------------------------------------------------------------
// ComponentArray <-> NodeMetadataArray
// ---------------------------------------------------------------------------

impl From<NodeMetadataArray> for ComponentArray {
  fn from(value: NodeMetadataArray) -> Self {
    ComponentArray {
      components: value
        .components
        .map(|v| v.into_iter().map(Component::from).collect())
        .unwrap_or_default(),
    }
  }
}

impl From<ComponentArray> for NodeMetadataArray {
  fn from(value: ComponentArray) -> Self {
    let component_vec: Vec<FrontEndComponent> =
      value.components.into_iter().map(Into::into).collect();
    NodeMetadataArray {
      components: Some(component_vec),
    }
  }
}

// ---------------------------------------------------------------------------
// ComponentCreate <-> FrontEndComponentCreate
// ---------------------------------------------------------------------------

impl From<FrontEndComponentCreate> for ComponentCreate {
  fn from(value: FrontEndComponentCreate) -> Self {
    ComponentCreate {
      id: XNameRw100(value.id),
      state: HmsState100::from_str(&value.state).unwrap_or(HmsState100::Unknown),
      flag: value.flag.and_then(|s| HmsFlag100::from_str(&s).ok()),
      enabled: value.enabled,
      software_status: value.software_status,
      role: value.role.map(HmsRole100),
      sub_role: value.sub_role.map(HmsSubRole100),
      nid: value.nid.map(|n| n as i64),
      subtype: value.subtype,
      net_type: value.net_type.and_then(|s| NetType100::from_str(&s).ok()),
      arch: value.arch.and_then(|s| HmsArch100::from_str(&s).ok()),
      class: value.class.and_then(|s| HmsClass100::from_str(&s).ok()),
    }
  }
}

impl From<ComponentCreate> for FrontEndComponentCreate {
  fn from(value: ComponentCreate) -> Self {
    FrontEndComponentCreate {
      id: value.id.0,
      state: value.state.to_string(),
      flag: value.flag.map(|f| f.to_string()),
      enabled: value.enabled,
      software_status: value.software_status,
      role: value.role.map(|r| r.0),
      sub_role: value.sub_role.map(|s| s.0),
      nid: value.nid.map(|n| n as usize),
      subtype: value.subtype,
      net_type: value.net_type.map(|n| n.to_string()),
      arch: value.arch.map(|a| a.to_string()),
      class: value.class.map(|c| c.to_string()),
    }
  }
}

// ---------------------------------------------------------------------------
// ComponentArrayPostArray <-> FrontEndComponentArrayPostArray
// ---------------------------------------------------------------------------

impl From<FrontEndComponentArrayPostArray> for ComponentArrayPostArray {
  fn from(value: FrontEndComponentArrayPostArray) -> Self {
    ComponentArrayPostArray {
      components: value.components.into_iter().map(ComponentCreate::from).collect(),
      force: value.force,
    }
  }
}

impl From<ComponentArrayPostArray> for FrontEndComponentArrayPostArray {
  fn from(value: ComponentArrayPostArray) -> Self {
    FrontEndComponentArrayPostArray {
      components: value.components.into_iter().map(Into::into).collect(),
      force: value.force,
    }
  }
}
