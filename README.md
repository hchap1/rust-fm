# Rust-FM is a lightweight client for integrating with the last.fm API, including the following featureset:
- Web OAUTH flow
  - Automatically redirects users to authentication page
  - Creates a local webserver to host a redirect
  - Grabs token from local webserver
  - Automatically creates a session token from this
- Now playing
  - Asynchronously set the now playing status of the user
- Scrobbling
  - Push a scrobble, relies on you to implement the LAST.FM rules
  - Generate timestamp if one is not provided
 
# Full example:
```rust
use rust_fm::auth::WebOAuth;
use rust_fm::token::WebCallback;
use rust_fm::session::WebSession;
use rust_fm::playing::{Scrobble, NowPlaying};

#[tokio::main]
async fn main() {
    let auth = WebOAuth::from_key_and_secret(
        Some("my_key".to_string()),
        Some("my_secret".to_string()),
        None // Do not yet have permanent session token
    );

    let (auth, res) = WebCallback::oauth(auth).await;
    res.expect("Failed to retrieve Authentication Token");
    println!("Token: {:?}", auth.get_token());

    let (auth, res) = WebSession::get(auth).await;
    res.expect("Failed to retrieve Web Session Token");
    println!("Session: {:?}", auth.get_session());

    let scrobble = Scrobble::new(
        "Song title".to_string(),
        "Song artist".to_string(),
        Some("Optional Album".to_string())
    );

    let _ = NowPlaying::set_now_playing(auth.clone(), scrobble.clone());

    // Optionally specify the timestamp the song started playing.
    let _ = NowPlaying::push_scrobble(auth, scrobble, None);
}

```
