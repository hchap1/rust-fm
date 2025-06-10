use serde::Deserialize;

use crate::auth::AuthFields;

pub enum TokenError {
    MissingKey,
    MissingSecret,
    NetworkError
}

#[derive(Debug, Deserialize, Clone)]
pub struct GetTokenResponse {
    token: String,
}

impl GetTokenResponse {

    pub async fn get(data: AuthFields) -> Result<AuthFields, TokenError> {
        
    }

    fn from_response(response: Option<String>) -> Option<Self> {
        let response = if let Some(response) = response { response } else { return None };
        serde_json::from_str::<Self>(&response).ok()
    }
}
