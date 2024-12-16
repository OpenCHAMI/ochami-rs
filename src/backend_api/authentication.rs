use backend_dispatcher::error::Error;

pub async fn get_api_token() -> Result<String, Error> {
    std::env::var("ACCESS_TOKEN")
        .map_err(|_e| Error::Message("environment variable 'AUTH_TOKEN' not found".to_string()))
}
