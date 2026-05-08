use crate::error::Error;

pub fn build_client(
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
) -> Result<reqwest::Client, Error> {
  let builder = reqwest::Client::builder()
    .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?)
    .use_rustls_tls();
  match socks5_proxy {
    Some(proxy) => builder.proxy(reqwest::Proxy::all(proxy)?),
    None => builder,
  }
  .build()
  .map_err(Error::NetError)
}

pub fn build_client_no_tls(
  root_cert: &[u8],
  socks5_proxy: Option<&str>,
) -> Result<reqwest::Client, Error> {
  let builder = reqwest::Client::builder()
    .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?);
  match socks5_proxy {
    Some(proxy) => builder.proxy(reqwest::Proxy::all(proxy)?),
    None => builder,
  }
  .build()
  .map_err(Error::NetError)
}
