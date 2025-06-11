use auth::WebOAuth;
use token::WebCallback;

use dotenvy::dotenv;

mod token;
mod auth;
mod api;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let auth = WebOAuth::load_env();
    let token = WebCallback::oauth(auth).await;
    println!("{token:?}");
}
