use reqwest::Error;
use serde_json::Value;

use core::result::Result;

use super::types::BootParameters;

pub fn post(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    boot_parameters: BootParameters,
) -> Result<(), Error> {
    let client_builder = reqwest::blocking::Client::builder()
        .add_root_certificate(reqwest::Certificate::from_pem(root_cert)?);

    // Build client
    let client = if let Ok(socks5_env) = std::env::var("SOCKS5") {
        // socks5 proxy
        log::debug!("SOCKS5 enabled");
        let socks5proxy = reqwest::Proxy::all(socks5_env)?;

        // rest client to authenticate
        client_builder.proxy(socks5proxy).build()?
    } else {
        client_builder.build()?
    };

    let api_url = format!("{}/boot/v1/bootparameters", base_url);

    let _response = client
        .post(api_url)
        .bearer_auth(auth_token)
        .json(&boot_parameters)
        .send()?
        .error_for_status()?;

    Ok(())
}

/// Change nodes boot params, ref --> https://apidocs.svc.cscs.ch/iaas/bss/tag/bootparameters/paths/~1bootparameters/put/
// FIXME: change return type from Result<BootParameters, backend_dispatcher::error::Error> to
// Result<BootParameters, reqwest::error::Error>
pub async fn put(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    boot_parameters: &BootParameters,
) -> Result<BootParameters, backend_dispatcher::error::Error> {
    let client_builder =
        reqwest::Client::builder().add_root_certificate(reqwest::Certificate::from_pem(root_cert)?);

    // Build client
    let client = if let Ok(socks5_env) = std::env::var("SOCKS5") {
        // socks5 proxy
        log::debug!("SOCKS5 enabled");
        let socks5proxy = reqwest::Proxy::all(socks5_env)?;

        // rest client to authenticate
        client_builder.proxy(socks5proxy).build()?
    } else {
        client_builder.build()?
    };

    let api_url = format!("{}/boot/v1/bootparameters", base_url);

    let response = client
        .put(api_url)
        .json(&boot_parameters)
        .bearer_auth(auth_token)
        .send()
        .await?;

    if let Err(e) = response.error_for_status_ref() {
        let error_payload = response.json::<Value>().await?;
        let error = backend_dispatcher::error::Error::RequestError {
            response: e,
            payload: serde_json::to_string_pretty(&error_payload)?,
        };
        return Err(error);
    }

    response
        .json()
        .await
        .map_err(|e| backend_dispatcher::error::Error::NetError(e))
}

pub async fn patch(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    boot_parameters: &BootParameters,
) -> Result<(), backend_dispatcher::error::Error> {
    let client_builder =
        reqwest::Client::builder().add_root_certificate(reqwest::Certificate::from_pem(root_cert)?);

    // Build client
    let client = if let Ok(socks5_env) = std::env::var("SOCKS5") {
        // socks5 proxy
        log::debug!("SOCKS5 enabled");
        let socks5proxy = reqwest::Proxy::all(socks5_env)?;

        // rest client to authenticate
        client_builder.proxy(socks5proxy).build()?
    } else {
        client_builder.build()?
    };

    let api_url = format!("{}/boot/v1/bootparameters", base_url);

    let response = client
        .patch(api_url)
        .json(&boot_parameters)
        .bearer_auth(auth_token)
        .send()
        .await?;

    if let Err(e) = response.error_for_status_ref() {
        let error_payload = response.json::<Value>().await?;
        let error = backend_dispatcher::error::Error::RequestError {
            response: e,
            payload: serde_json::to_string_pretty(&error_payload)?,
        };
        return Err(error);
    }

    Ok(())
}

/* pub async fn get(
    shasta_token: &str,
    shasta_base_url: &str,
    shasta_root_cert: &[u8],
    xnames: &[String],
) -> Result<Vec<BootParameters>, reqwest::Error> {
    let start = Instant::now();

    let chunk_size = 30;

    let mut boot_params_vec = Vec::new();

    let mut tasks = tokio::task::JoinSet::new();

    let sem = Arc::new(Semaphore::new(10)); // CSM 1.3.1 higher number of concurrent tasks won't

    for sub_node_list in xnames.chunks(chunk_size) {
        let shasta_token_string = shasta_token.to_string();
        let shasta_base_url_string = shasta_base_url.to_string();
        let shasta_root_cert_vec = shasta_root_cert.to_vec();

        let permit = Arc::clone(&sem).acquire_owned().await;

        let node_vec = sub_node_list.to_vec();

        tasks.spawn(async move {
            let _permit = permit; // Wait semaphore to allow new tasks https://github.com/tokio-rs/tokio/discussions/2648#discussioncomment-34885

            get_raw(
                &shasta_token_string,
                &shasta_base_url_string,
                &shasta_root_cert_vec,
                &node_vec,
            )
            .unwrap()
        });
    }

    while let Some(message) = tasks.join_next().await {
        if let Ok(mut node_status_vec) = message {
            boot_params_vec.append(&mut node_status_vec);
        }
    }

    let duration = start.elapsed();
    log::info!("Time elapsed to get BSS bootparameters is: {:?}", duration);

    Ok(boot_params_vec)
} */

/// Get node boot params, ref --> https://apidocs.svc.cscs.ch/iaas/bss/tag/bootparameters/paths/~1bootparameters/get/
/// NOTE: MANTA MIGRATION! the 'url_api' value changes compared to CSM
/// NOTE: if db is empty, then OCHAMI API will return 'Null' therefore, if we want to handle this
/// situation, then, we need to return serde_json::Value
pub async fn get(
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    xnames_opt: &Option<Vec<String>>,
) -> Result<Vec<BootParameters>, Error> {
    let client;

    let client_builder =
        reqwest::Client::builder().add_root_certificate(reqwest::Certificate::from_pem(root_cert)?);

    // Build client
    if std::env::var("SOCKS5").is_ok() {
        // socks5 proxy
        log::debug!("SOCKS5 enabled");
        let socks5proxy = reqwest::Proxy::all(std::env::var("SOCKS5").unwrap())?;

        // rest client to authenticate
        client = client_builder.proxy(socks5proxy).build()?;
    } else {
        client = client_builder.build()?;
    }

    let url_api = format!("{}/apis/bss/boot/v1/bootparameters", base_url.to_string());

    let params: Option<Vec<_>> = if let Some(xname_vec) = xnames_opt {
        Some(xname_vec.iter().map(|xname| ("name", xname)).collect())
    } else {
        None
    };

    client
        .get(url_api)
        .query(&params)
        .bearer_auth(auth_token)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
}
