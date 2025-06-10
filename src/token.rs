use std::collections::BTreeMap;

use reqwest::ClientBuilder;
use serde::Deserialize;

use crate::auth::AuthFields;

pub enum TokenError {
    MissingKey,
    MissingSecret,
    NetworkError,
    ResponseError
}

#[derive(Debug, Deserialize, Clone)]
pub struct GetTokenResponse {
    token: String,
}

impl GetTokenResponse {

    /// Consume an AuthFields and return a new one, possibly containing a token.
    /// Result indicates success () or error TokenError.
    pub async fn get(mut auth_fields: AuthFields) -> (AuthFields, Result<(), TokenError>){

        let api_key: &str = match auth_fields.get_key() {
            Some(key) => key,
            None => return (auth_fields, Err(TokenError::MissingKey))
        };

        let mut params: BTreeMap<&str, &str> = BTreeMap::new();
        params.insert("api_key", api_key);
        params.insert("method", "auth.gettoken");
        params.insert("format", "json");

        let client = match ClientBuilder::new().build() {
            Ok(client) => client,
            Err(_) => return (auth_fields, Err(TokenError::NetworkError))
        };

        let result = client.post(crate::api::URL)
            .form(&params)
            .send().await;

        let text = match result {
            Ok(json_string) => match json_string.text().await {
                Ok(text) => text,
                Err(_) => return (auth_fields, Err(TokenError::ResponseError))
            },
            Err(_) => return (auth_fields, Err(TokenError::ResponseError))
        };

        let token = match Self::from_response(text) {
            Some(token) => token,
            None => return (auth_fields, Err(TokenError::ResponseError))
        };

        auth_fields.set_token(token);
        (auth_fields, Ok(()))
    }

    fn from_response(response: String) -> Option<Self> {
        serde_json::from_str::<Self>(&response).ok()
    }
}
