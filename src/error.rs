use serde_json::Value;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
  #[error("OCHAMI-RS: {0}")]
  Message(String),
  #[error("OCHAMI-RS > Serde: {0}")]
  SerdeError(#[from] serde_json::Error),
  #[error("OCHAMI-RS > Net: {0}")]
  NetError(#[from] reqwest::Error),
  #[error(
    "OCHAMI-RS: http request:\nresponse: {response}\npayload: {payload}"
  )]
  RequestError {
    response: reqwest::Error,
    payload: String, // NOTE: CSM/OCHAMI Apis either returns plain text or a json therefore, we
                     // will just return a String
  },
  #[error("OCHAMI-RS > OCHAMI: status={status} body={body}")]
  #[non_exhaustive]
  OchamiError { status: u16, body: Value },
}
