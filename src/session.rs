use reqwest::ClientBuilder;
use serde::Deserialize;

use crate::auth::WebOAuth;
use crate::auth::Signature;

#[derive(Debug, Clone)]
pub enum WebSessionError {
    MissingFields,
    ClientFailed,
    ResponseError,
    TextError,
    ParseError
}

/// Struct to grab the permanent session token from a one-use authentication token.
pub struct WebSession;

impl WebSession {
    /// This function attempts to load the session token into the auth. Requires a valid key, token and secret.
    pub async fn get(mut auth: WebOAuth) -> (WebOAuth, Result<(), WebSessionError>) {
        let (key, token, secret) = match (auth.get_key(), auth.get_token(), auth.get_secret()) {
            (Some(key), Some(token), Some(secret)) => (key, token, secret),
            _ => return (auth, Err(WebSessionError::MissingFields))
        };

        let mut buffer = String::new();

        let args = Signature::from_args(
            vec![
                ("api_key", key),
                ("method", "auth.getsession"),
                ("token", token)
            ], &mut buffer, secret
        );

        let client = match ClientBuilder::new().build() {
            Ok(client) => client,
            Err(_) => return (auth, Err(WebSessionError::ClientFailed))
        };

        let response = match client
            .post(crate::api::URL)
            .form(&args)
            .send()
            .await {
            Ok(response) => response,
            Err(_) => return (auth, Err(WebSessionError::ResponseError))
        };

        let text = match response.text().await {
            Ok(text) => text,
            Err(_) => return (auth, Err(WebSessionError::TextError))
        };

        let parsed: SessionResponse = match serde_json::from_str(text.as_str()) {
            Ok(session_response) => session_response,
            Err(_) => return (auth, Err(WebSessionError::ParseError))
        };

        auth.set_session(parsed.session.key);
        (auth, Ok(()))
    }
}

#[derive(Debug, Deserialize)]
struct SessionResponse {
    session: Session,
}

#[derive(Debug, Deserialize)]
struct Session {
    key: String,
}
