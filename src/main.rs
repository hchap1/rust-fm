use auth::WebOAuth;
use session::WebSession;
use token::WebCallback;

use dotenvy::dotenv;

mod token;
mod auth;
mod api;
mod session;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let auth = WebOAuth::load_env();
    let (auth, _) = WebCallback::oauth(auth).await;
    let (auth, _) = WebSession::get(auth).await;
}
