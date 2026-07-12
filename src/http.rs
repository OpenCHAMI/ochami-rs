use crate::error::Error;

pub fn build_client(root_cert: &[u8]) -> Result<reqwest::Client, Error> {
  let builder = reqwest::Client::builder()
    .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?)
    .use_rustls_tls();
  builder.build().map_err(Error::NetError)
}

pub fn build_client_no_tls(root_cert: &[u8]) -> Result<reqwest::Client, Error> {
  let builder = reqwest::Client::builder()
    .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?);
  builder.build().map_err(Error::NetError)
}
