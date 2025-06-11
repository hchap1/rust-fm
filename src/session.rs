use reqwest::ClientBuilder;
use reqwest::RequestBuilder;

use crate::auth::WebOAuth;
use crate::auth::Signature;

#[derive(Debug, Clone)]
pub enum WebSessionError {
    MissingFields,
    ClientFailed,
    ResponseError,
    TextError
}

pub struct WebSession;

impl WebSession {
    pub async fn get(auth: WebOAuth) -> (WebOAuth, Result<(), WebSessionError>) {
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

        println!("{text}");
        (auth, Ok(()))
    }
}
