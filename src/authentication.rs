use std::env::VarError;

pub async fn get_api_token() -> Result<String, VarError> {
    std::env::var("ACCESS_TOKEN")
}
