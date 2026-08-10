use std::env::VarError;

/// Reads the OCHAMI bearer token from the `ACCESS_TOKEN` environment variable.
pub fn get_api_token() -> Result<String, VarError> {
  std::env::var("ACCESS_TOKEN")
}

/// Returns `Ok(())` if `token` is non-empty; otherwise `Err(VarError::NotPresent)`.
pub fn validate_api_token(token: &str) -> Result<(), VarError> {
  if token.is_empty() {
    Err(VarError::NotPresent)
  } else {
    Ok(())
  }
}
