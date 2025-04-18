use std::io;

use serde_json::Value;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("ERROR - MESA: {0}")]
    Message(String),
    #[error("ERROR - IO: {0}")]
    IoError(#[from] io::Error),
    #[error("ERROR - Serde: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("ERROR - Net: {0}")]
    NetError(#[from] reqwest::Error),
    #[error("ERROR - http request:\nresponse: {response}\npayload: {payload}")]
    RequestError {
        response: reqwest::Error,
        payload: String, // NOTE: CSM/OCHAMI Apis either returns plain text or a json therefore, we
                         // will just return a String
    },
    #[error("ERROR - OCHAMI: {0}")]
    CsmError(Value),
    #[error("ERROR - field '{0}' missing")]
    MissingField(String),
}
