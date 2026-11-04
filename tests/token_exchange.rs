//! wiremock tests for `AuthenticationTrait::exchange_api_token`: the
//! JWT bearer exchange at the site's Hydra.

mod common;

use common::TEST_PEM;
use manta_backend_dispatcher::error::Error;
use manta_backend_dispatcher::interfaces::authentication::AuthenticationTrait;
use ochami_rs::authentication::TokenExchange;
use ochami_rs::Ochami;
use serde_json::json;
use wiremock::matchers::{body_string_contains, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const TOKEN_PATH: &str = "/hydra/oauth2/token";

fn ochami(server: &MockServer) -> Ochami {
  Ochami::new(&server.uri(), TEST_PEM.as_bytes()).with_token_exchange(
    TokenExchange {
      client_id: "client".to_string(),
      client_secret: "secret".to_string(),
      scope: "smd.read bss.read".to_string(),
    },
  )
}

/// A server whose token endpoint answers `status` with `body` to any POST.
async fn server_answering(status: u16, body: serde_json::Value) -> MockServer {
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path(TOKEN_PATH))
    .respond_with(ResponseTemplate::new(status).set_body_json(body))
    .mount(&server)
    .await;
  server
}

#[tokio::test]
async fn sends_the_jwt_bearer_grant_and_returns_the_access_token() {
  let server = MockServer::start().await;
  Mock::given(method("POST"))
    .and(path(TOKEN_PATH))
    // base64("client:secret")
    .and(header("authorization", "Basic Y2xpZW50OnNlY3JldA=="))
    .and(body_string_contains(
      "grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Ajwt-bearer",
    ))
    .and(body_string_contains("assertion=kc-token"))
    .and(body_string_contains("scope=smd.read+bss.read"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
      "access_token": "hydra-token",
      "expires_in": 3599,
      "scope": "smd.read bss.read",
      "token_type": "bearer",
    })))
    .expect(1)
    .mount(&server)
    .await;

  let token = ochami(&server)
    .exchange_api_token("kc-token")
    .await
    .unwrap();

  assert_eq!(token, "hydra-token");
}

#[tokio::test]
async fn a_trailing_slash_on_the_base_url_is_tolerated() {
  let server =
    server_answering(200, json!({"access_token": "hydra-token"})).await;
  let ochami = Ochami::new(&format!("{}/", server.uri()), TEST_PEM.as_bytes())
    .with_token_exchange(TokenExchange {
      client_id: "client".to_string(),
      client_secret: "secret".to_string(),
      scope: "smd.read".to_string(),
    });

  let token = ochami.exchange_api_token("kc-token").await.unwrap();

  assert_eq!(token, "hydra-token");
}

#[tokio::test]
async fn without_credentials_nothing_is_sent() {
  let err = Ochami::new("http://127.0.0.1:9", TEST_PEM.as_bytes())
    .exchange_api_token("kc-token")
    .await
    .unwrap_err();

  assert!(
    matches!(&err, Error::Message(m) if m.contains("not configured")),
    "{err:?}"
  );
}

#[tokio::test]
async fn invalid_grant_is_unauthorized() {
  let server = server_answering(
    400,
    json!({"error": "invalid_grant", "error_description": "expired"}),
  )
  .await;

  let err = ochami(&server)
    .exchange_api_token("kc-token")
    .await
    .unwrap_err();

  assert!(
    matches!(&err, Error::Unauthorized(m) if m.contains("expired")),
    "{err:?}"
  );
}

#[tokio::test]
async fn an_already_exchanged_token_is_unauthorized() {
  let server = server_answering(
    400,
    json!({"error": "jti_known", "error_description": "The jti was already used."}),
  )
  .await;

  let err = ochami(&server)
    .exchange_api_token("kc-token")
    .await
    .unwrap_err();

  assert!(
    matches!(&err, Error::Unauthorized(m) if m.contains("jti_known")),
    "{err:?}"
  );
}

#[tokio::test]
async fn status_403_is_forbidden() {
  let server = server_answering(403, json!({"error": "access_denied"})).await;

  let err = ochami(&server)
    .exchange_api_token("kc-token")
    .await
    .unwrap_err();

  assert!(matches!(err, Error::Forbidden(_)), "{err:?}");
}

#[tokio::test]
async fn a_wrong_client_secret_is_a_plain_message_not_unauthorized() {
  let server = server_answering(401, json!({"error": "invalid_client"})).await;

  let err = ochami(&server)
    .exchange_api_token("kc-token")
    .await
    .unwrap_err();

  assert!(
    matches!(&err, Error::Message(m) if m.contains("invalid_client")),
    "{err:?}"
  );
}

#[tokio::test]
async fn a_server_error_keeps_its_status() {
  let server = server_answering(503, json!({"error": "unavailable"})).await;

  let err = ochami(&server)
    .exchange_api_token("kc-token")
    .await
    .unwrap_err();

  assert!(
    matches!(err, Error::CsmError { status: 503, .. }),
    "{err:?}"
  );
}

#[tokio::test]
async fn a_success_without_access_token_is_an_error() {
  let server = server_answering(200, json!({"token_type": "bearer"})).await;

  let err = ochami(&server)
    .exchange_api_token("kc-token")
    .await
    .unwrap_err();

  assert!(
    matches!(&err, Error::Message(m) if m.contains("access_token")),
    "{err:?}"
  );
}

#[test]
fn debug_redacts_the_client_secret() {
  let exchange = TokenExchange {
    client_id: "client".to_string(),
    client_secret: "hunter2".to_string(),
    scope: "smd.read".to_string(),
  };

  let rendered = format!("{exchange:?}");

  assert!(!rendered.contains("hunter2"), "{rendered}");
  assert!(rendered.contains("<redacted>"), "{rendered}");
}
