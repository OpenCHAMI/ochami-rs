//! List every HSM component of type `Node`, printing xname and
//! current power state (`state`).
//!
//! # Env vars
//!
//! - `OCHAMI_URL` — base URL of the OpenCHAMI ingress
//! - `OCHAMI_TOKEN` — bearer token
//! - `OCHAMI_CA` — path to a PEM CA-cert file
//!
//! Run:
//! ```bash
//! OCHAMI_URL=... OCHAMI_TOKEN=... OCHAMI_CA=/path/ca.pem \
//!     cargo run --example fetch_components
//! ```

use std::env;
use std::error::Error as StdError;

use ochami_rs::{error::Error, Ochami};

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
  let base_url = env::var("OCHAMI_URL")
    .map_err(|_| "OCHAMI_URL must be set (base URL of the OpenCHAMI ingress)")?;
  let token = env::var("OCHAMI_TOKEN")
    .map_err(|_| "OCHAMI_TOKEN must be set (bearer token)")?;
  let ca_path = env::var("OCHAMI_CA")
    .map_err(|_| "OCHAMI_CA must be set (path to PEM CA file)")?;

  let root_cert = std::fs::read(&ca_path)
    .map_err(|e| format!("failed to read OCHAMI_CA ({ca_path}): {e}"))?;

  let ochami = Ochami::new(&base_url, &root_cert);

  match ochami.hsm_component_get_all_nodes(&token, Some("true")).await {
    Ok(array) => {
      println!("{} components:", array.components.len());
      for c in &array.components {
        let xname = c.id.as_ref().map(|x| x.0.as_str()).unwrap_or("<no-id>");
        let state = c
          .state
          .as_ref()
          .map(|s| format!("{s:?}"))
          .unwrap_or_else(|| "<no-state>".to_string());
        println!("  {xname:20}  {state}");
      }
      Ok(())
    }
    Err(Error::OchamiError { status, body, .. }) => {
      eprintln!("HSM returned {status}: {body}");
      std::process::exit(1);
    }
    Err(other) => {
      eprintln!("request failed: {other}");
      std::process::exit(1);
    }
  }
}
