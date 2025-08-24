use std::env::VarError;

pub fn get_api_token() -> Result<String, VarError> {
  std::env::var("ACCESS_TOKEN")
}

pub fn validate_api_token(token: &str) -> Result<(), VarError> {
  if token.is_empty() {
    Ok(())
  } else {
    Err(VarError::NotPresent)
  }
}
