use crate::token::GetTokenResponse;

#[derive(Clone, Debug)]
pub struct AuthFields {
    key: Option<String>,
    secret: Option<String>,
    token: Option<GetTokenResponse>
}
