use std::io;

use serde_json::Value;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("OCHAMI-RS: {0}")]
    Message(String),
    #[error("OCHAMI-RS > IO: {0}")]
    IoError(#[from] io::Error),
    #[error("OCHAMI-RS > Serde: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("OCHAMI-RS > Net: {0}")]
    NetError(#[from] reqwest::Error),
    #[error("OCHAMI-RS: http request:\nresponse: {response}\npayload: {payload}")]
    RequestError {
        response: reqwest::Error,
        payload: String, // NOTE: CSM/OCHAMI Apis either returns plain text or a json therefore, we
                         // will just return a String
    },
    #[error("OCHAMI-RS > OCHAMI: {0}")]
    OchamiError(Value),
}
