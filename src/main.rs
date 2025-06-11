use auth::WebOAuth;
use session::WebSession;
use token::WebCallback;
use playing::Scrobble;
use playing::NowPlaying;

use dotenvy::dotenv;

mod token;
mod auth;
mod api;
mod session;
mod playing;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let auth = WebOAuth::load_env();
    let (auth, _) = WebCallback::oauth(auth).await;
    let (auth, _) = WebSession::get(auth).await;

    let test = Scrobble::new(
        "Julian Touching Connor".to_string(),
        "Your Mother".to_string(),
        Some("I'm going to end it all".to_string())
    );
    let res = NowPlaying::set_now_playing(auth, test).await;
    println!("{res:?}");
}
