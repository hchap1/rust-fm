use auth::AuthFields;
use dotenvy::dotenv;

mod token;
mod auth;
mod api;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let auth = AuthFields::load_env();
    let (auth, _) = crate::token::GetTokenResponse::get(auth).await;
    println!("TOKEN: {:?}", auth.get_token());
}
