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

/// Parse a stringly-typed HSM enum field into a typed value, logging a
/// warning and returning `None` on parse failure instead of silently
/// dropping the value. `field_name` identifies the field in log output
/// so operators can debug schema-drift issues (e.g. HSM adding a new
/// component state the client doesn't yet know about).
fn parse_enum_field<T>(opt: Option<String>, field_name: &str) -> Option<T>
where
  T: FromStr,
  <T as FromStr>::Err: std::fmt::Debug,
{
  opt.and_then(|s| match T::from_str(&s) {
    Ok(v) => Some(v),
    Err(e) => {
      log::warn!(
        "dropped unparseable {field_name}: {s:?} ({e:?})"
      );
      None
    }
  })
}

// ---------------------------------------------------------------------------
// Component <-> FrontEndComponent
// ---------------------------------------------------------------------------

impl From<FrontEndComponent> for Component {
  fn from(value: FrontEndComponent) -> Self {
    Component {
      id: value.id.map(XName100),
      type_: parse_enum_field::<HmsType100>(value.r#type, "component.type"),
      state: parse_enum_field::<HmsState100>(value.state, "component.state"),
      flag: parse_enum_field::<HmsFlag100>(value.flag, "component.flag"),
      enabled: value.enabled,
      software_status: value.software_status,
      role: value.role.map(HmsRole100),
      sub_role: value.sub_role.map(HmsSubRole100),
      nid: value.nid.map(|n| n as i64),
      subtype: value.subtype,
      net_type: parse_enum_field::<NetType100>(value.net_type, "component.net_type"),
      arch: parse_enum_field::<HmsArch100>(value.arch, "component.arch"),
      class: parse_enum_field::<HmsClass100>(value.class, "component.class"),
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
      flag: parse_enum_field::<HmsFlag100>(value.flag, "component.flag"),
      enabled: value.enabled,
      software_status: value.software_status,
      role: value.role.map(HmsRole100),
      sub_role: value.sub_role.map(HmsSubRole100),
      nid: value.nid.map(|n| n as i64),
      subtype: value.subtype,
      net_type: parse_enum_field::<NetType100>(value.net_type, "component.net_type"),
      arch: parse_enum_field::<HmsArch100>(value.arch, "component.arch"),
      class: parse_enum_field::<HmsClass100>(value.class, "component.class"),
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
