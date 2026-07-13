//! Shared HSM response types.
pub use crate::hsm::generated::types::Response100 as HsmActionResponse;

use manta_backend_dispatcher::types::HsmActionResponse as FrontEndHsmActionResponse;

impl From<FrontEndHsmActionResponse> for HsmActionResponse {
  fn from(value: FrontEndHsmActionResponse) -> Self {
    HsmActionResponse {
      code: value.code,
      message: value.message,
    }
  }
}

impl From<HsmActionResponse> for FrontEndHsmActionResponse {
  fn from(value: HsmActionResponse) -> Self {
    FrontEndHsmActionResponse {
      code: value.code,
      message: value.message,
    }
  }
}
