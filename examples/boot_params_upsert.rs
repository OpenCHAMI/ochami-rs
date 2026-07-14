//! Read the current boot parameters for one xname, print them, and
//! PATCH-update the kernel image to a placeholder value.
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
//!     cargo run --example boot_params_upsert -- x1c0s1b0n0
//! ```
//!
//! WARNING: this example writes to BSS. Point at a non-production
//! deployment or expect the target node's boot parameters to change.

use std::env;
use std::error::Error as StdError;

use ochami_rs::{error::Error, Ochami};

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
  let xname = env::args()
    .nth(1)
    .ok_or("pass exactly one xname as a positional argument")?;

  let base_url = env::var("OCHAMI_URL")?;
  let token = env::var("OCHAMI_TOKEN")?;
  let ca_path = env::var("OCHAMI_CA")?;
  let root_cert = std::fs::read(&ca_path)?;

  let ochami = Ochami::new(&base_url, &root_cert);

  // Read.
  let current = ochami
    .bss_bootparameters_get(&token, Some(&[xname.as_str()]))
    .await?;
  let mut bp = current
    .into_iter()
    .next()
    .ok_or_else(|| format!("BSS has no boot parameters for xname {xname}"))?;
  println!("Before: kernel = {}", bp.kernel);

  // Modify — placeholder for demonstration; a real caller would
  // substitute the new image id/URL here.
  bp.kernel =
    "s3://boot-images/EXAMPLE-PLACEHOLDER-IMAGE-ID/kernel".to_string();

  // Write.
  match ochami.bss_bootparameters_patch(&token, &bp).await {
    Ok(()) => println!("PATCH ok — re-fetch to confirm."),
    Err(Error::OchamiError { status, body, .. }) => {
      eprintln!("BSS returned {status}: {body}");
      std::process::exit(1);
    }
    Err(other) => {
      eprintln!("request failed: {other}");
      std::process::exit(1);
    }
  }
  Ok(())
}
