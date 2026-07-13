//! Slim re-exports from the progenitor-generated HSM types, plus
//! `From`/`Into` conversions between the generated `Group100` /
//! `Members100` types and the `manta_backend_dispatcher` front-end
//! `Group` / `Member` types.
//!
//! Task 13 (Phase 5) replaced the hand-written duplicate types here
//! with direct aliases to the generated equivalents. The `impl Group`
//! block was rewritten to work with the new field shapes (private
//! newtype `ResourceName` for `label`; `Vec<XNameRw100>` for the
//! members list; `Vec<ResourceName>` for tags). Design spec Section 4
//! (Phase 5) — Types migration.

use std::str::FromStr;

pub use crate::hsm::generated::types::{
  Group100 as Group, Group100Patch as GroupPatch, MemberId,
  Members100 as Members, ResourceName, XNameRw100,
};

use manta_backend_dispatcher::types::{
  Group as FrontEndGroup, Member as FrontEndMember,
};

// ---------------------------------------------------------------------------
// Inherent methods on Group, rewritten for the new field shapes.
// ---------------------------------------------------------------------------

/// Convenience helper — builds a [`ResourceName`] from a `&str`,
/// panicking if the label doesn't match the HSM resource-name pattern
/// (`^[0-9a-zA-Z._-]{1,}$`). This mirrors the hand-written
/// constructor's infallible signature; callers with untrusted input
/// should use [`ResourceName::from_str`] directly.
fn resource_name(s: &str) -> ResourceName {
  ResourceName::from_str(s).unwrap_or_else(|e| {
    panic!("invalid HSM ResourceName {s:?}: {e}");
  })
}

/// Extension trait providing the pre-migration `Group::new` /
/// `.get_members()` / `.get_members_opt()` / `.add_xnames()`
/// convenience methods. Kept as a trait so downstream code that
/// uses these methods continues to compile against the slim re-export.
pub trait GroupExt {
  fn new(label: &str, member_vec_opt: Option<Vec<&str>>) -> Self;
  fn get_members(&self) -> Vec<String>;
  fn get_members_opt(&self) -> Option<Vec<String>>;
  fn add_xnames(&mut self, xnames: &[String]) -> Vec<String>;
}

impl GroupExt for Group {
  fn new(label: &str, member_vec_opt: Option<Vec<&str>>) -> Self {
    let members = member_vec_opt.map(|v| Members {
      ids: v
        .into_iter()
        .map(|s| XNameRw100(s.to_string()))
        .collect(),
    });
    Group {
      label: resource_name(label),
      description: None,
      tags: Vec::new(),
      members,
      exclusive_group: None,
    }
  }

  fn get_members(&self) -> Vec<String> {
    self
      .members
      .as_ref()
      .map(|m| m.ids.iter().map(|x| x.0.clone()).collect())
      .unwrap_or_default()
  }

  fn get_members_opt(&self) -> Option<Vec<String>> {
    self
      .members
      .as_ref()
      .map(|m| m.ids.iter().map(|x| x.0.clone()).collect())
  }

  fn add_xnames(&mut self, xnames: &[String]) -> Vec<String> {
    if let Some(members) = self.members.as_mut() {
      for x in xnames {
        members.ids.push(XNameRw100(x.clone()));
      }
    } else {
      self.members = Some(Members {
        ids: xnames.iter().map(|x| XNameRw100(x.clone())).collect(),
      });
    }
    self.get_members()
  }
}

// ---------------------------------------------------------------------------
// Group <-> FrontEndGroup
// ---------------------------------------------------------------------------

impl From<FrontEndGroup> for Group {
  fn from(value: FrontEndGroup) -> Self {
    let members = value.members.map(|m| Members {
      ids: m
        .ids
        .unwrap_or_default()
        .into_iter()
        .map(XNameRw100)
        .collect(),
    });
    Group {
      label: resource_name(&value.label),
      description: value.description,
      tags: value
        .tags
        .unwrap_or_default()
        .into_iter()
        .filter_map(|t| match ResourceName::from_str(&t) {
          Ok(r) => Some(r),
          Err(e) => {
            log::warn!(
              "dropped unparseable Group.tag: {t:?} ({e:?})"
            );
            None
          }
        })
        .collect(),
      members,
      exclusive_group: value.exclusive_group.and_then(|s| {
        match ResourceName::from_str(&s) {
          Ok(r) => Some(r),
          Err(e) => {
            log::warn!(
              "dropped unparseable Group.exclusive_group: {s:?} ({e:?})"
            );
            None
          }
        }
      }),
    }
  }
}

impl From<Group> for FrontEndGroup {
  fn from(value: Group) -> Self {
    let members = value.members.map(|m| FrontEndMember {
      ids: Some(m.ids.into_iter().map(|x| x.0).collect()),
    });
    FrontEndGroup {
      label: (*value.label).clone(),
      description: value.description,
      tags: if value.tags.is_empty() {
        None
      } else {
        Some(value.tags.into_iter().map(|t| (*t).clone()).collect())
      },
      members,
      exclusive_group: value.exclusive_group.map(|e| (*e).clone()),
    }
  }
}
