//! Query PCS for the power state of one or more xnames (pass as CLI args).
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
//!     cargo run --example power_status -- x1c0s1b0n0 x1c0s1b0n1
//! ```
//! Passing no xnames queries every node PCS knows about.

use std::env;
use std::error::Error as StdError;

use ochami_rs::{error::Error, Ochami};

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
  let base_url = env::var("OCHAMI_URL")?;
  let token = env::var("OCHAMI_TOKEN")?;
  let ca_path = env::var("OCHAMI_CA")?;
  let root_cert = std::fs::read(&ca_path)?;

  // Skip argv[0]; treat everything else as an xname.
  let xname_args: Vec<String> = env::args().skip(1).collect();
  let xname_refs: Vec<&str> = xname_args.iter().map(String::as_str).collect();
  let xnames = if xname_refs.is_empty() {
    None
  } else {
    Some(xname_refs.as_slice())
  };

  let ochami = Ochami::new(&base_url, &root_cert);

  match ochami
    .pcs_power_status_post(&token, xnames, None, None)
    .await
  {
    Ok(all) => {
      for entry in &all.status {
        let power = entry
          .power_state
          .as_ref()
          .map(|p| format!("{p:?}"))
          .unwrap_or_else(|| "<unknown>".to_string());
        let mgmt = entry
          .management_state
          .as_ref()
          .map(|m| format!("{m:?}"))
          .unwrap_or_else(|| "<unknown>".to_string());
        println!("{:20}  power={power:9}  mgmt={mgmt}", entry.xname);
      }
      Ok(())
    }
    Err(Error::OchamiError { status, body, .. }) => {
      eprintln!("PCS returned {status}: {body}");
      std::process::exit(1);
    }
    Err(other) => {
      eprintln!("request failed: {other}");
      std::process::exit(1);
    }
  }
}
