use std::{io::Read, path::PathBuf};

use directories::ProjectDirs;
use error::Error;
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};

pub mod backend_api;
pub mod cli;
pub mod error;

fn main() {
    // start procesing configuration
    let settings = config::Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .add_source(
            config::Environment::with_prefix("MANTA")
                .try_parsing(true)
                .prefix_separator("_"),
        )
        .build()
        .expect("ERROR - could not process configuration file");

    let port = "8443";
    let system_name = settings
        .get_string("SYSTEM_NAME")
        .expect("ERROR - 'SYSTEM_NAME' config parameter not found");
    let system_domain = settings
        .get_string("SYSTEM_DOMAIN")
        .expect("ERROR - 'SYSTEM_DOMAIN' config parameter not found");

    let base_url = format!("https://{system_name}.{system_domain}:{port}");
    let auth_token = std::env::var("ACCESS_TOKEN").expect("ERROR - 'ACCESS_TOKEN' env missing");
    // Validate JWT token
    /* let base64_claims = auth_token
        .split(' ')
        .nth(1)
        .unwrap_or(auth_token)
        .split('.')
        .nth(1)
        .unwrap_or("JWT Token not valid");

    let claims_u8 = decode(base64_claims).unwrap(); */

    /* let token_rslt = decode::<Claims>(
        &auth_token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    ); */

    /* let token = decode::<Claims>(
        &auth_token,
        &DecodingKey::from_rsa_components(jwk["n"], jwk["e"]),
        &Validation::new(Algorithm::RS256),
    )?; */

    /* if let Err(e) = token_rslt {
        eprintln!("ERROR - Auth token invalid. Reason:\n{:#?}", e);
        std::process::exit(1);
    } */

    let root_ca_cert_file: String = "cacert.pem".to_string();

    let root_cert_rslt = get_csm_root_cert_content(&root_ca_cert_file);

    let root_cert = if let Ok(root_cert) = root_cert_rslt {
        root_cert
    } else {
        eprintln!(
            "ERROR - CA public root file '{}' not found. Exit",
            root_ca_cert_file
        );
        std::process::exit(1);
    };
    // end processing configuration

    // start process CLI
    // Create CLI
    let matches = crate::cli::build_cli::build_cli().get_matches();
    // Process user command
    let cli_rslt = crate::cli::process_commands::process_cli(
        matches,
        &base_url,
        &auth_token,
        &root_cert,
        &settings,
    );

    match cli_rslt {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

// TODO: move to 'config' module
pub fn get_default_config_path() -> PathBuf {
    // XDG Base Directory Specification
    let project_dirs = ProjectDirs::from(
        "local", /*qualifier*/
        "cscs",  /*organization*/
        "manta", /*application*/
    );

    PathBuf::from(project_dirs.unwrap().config_dir())
}

// TODO: move to 'config' module
pub fn get_csm_root_cert_content(file_path: &str) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();
    let root_cert_file_rslt = std::fs::File::open(file_path);

    let file_rslt = if root_cert_file_rslt.is_err() {
        let mut config_path = get_default_config_path();
        config_path.push(file_path);
        std::fs::File::open(config_path)
    } else {
        root_cert_file_rslt
    };

    match file_rslt {
        Ok(mut file) => {
            let _ = file.read_to_end(&mut buf);

            Ok(buf)
        }
        Err(_) => Err(Error::Message(
            "CA public root file cound not be found.".to_string(),
        )),
    }
}
