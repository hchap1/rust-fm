use std::env::var;
use crate::token::GetTokenResponse;

#[derive(Clone, Debug)]
pub struct AuthFields {
    key: Option<String>,
    secret: Option<String>,
    token: Option<GetTokenResponse>
}

impl AuthFields {
    pub fn load_env() -> Self {
        Self {
            key: var("FM_KEY").ok(),
            secret: var("FM_SECRET").ok(),
            token: None
        }
    }

    pub fn get_key(&self) -> Option<&str> { self.key.as_ref().map(|x| x.as_str()) }
    pub fn get_secret(&self) -> Option<&str> { self.key.as_ref().map(|x| x.as_str()) }
    pub fn get_token(&self) -> Option<&str> { self.key.as_ref().map(|x| x.as_str()) }

    pub fn set_token(&mut self, token: GetTokenResponse) { self.token = Some(token); }
}
