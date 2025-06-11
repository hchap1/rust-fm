use reqwest::ClientBuilder;

use crate::auth::WebOAuth;
use crate::auth::Signature;

#[derive(Clone, Debug)]
pub struct Scrobble {
    title: String,
    artist: String,
    album: Option<String>
}

impl Scrobble {
    pub fn new(title: String, artist: String, album: Option<String>) -> Self {
        Self { title, artist, album }
    }
}

#[derive(Clone, Debug)]
pub enum ScrobbleError {
    MissingFields,
    ClientFailed,
    ResponseError,
    TextError
}

pub struct NowPlaying;
impl NowPlaying {
    pub async fn set_now_playing(auth: WebOAuth, scrobble: Scrobble) -> Result<(), ScrobbleError> {
        let (key, session, secret) = match (auth.get_key(), auth.get_session(), auth.get_secret()) {
            (Some(key), Some(token), Some(secret)) => (key, token, secret),
            _ => return Err(ScrobbleError::MissingFields)
        };

        let mut buffer = String::new();

        let mut args_vec = vec![
            ("api_key", key),
            ("artist", scrobble.artist.as_str()),
            ("method", "track.updatenowplaying"),
            ("sk", session),
            ("track", scrobble.title.as_str())
        ];

        if let Some(album) = scrobble.album.as_ref() {
            args_vec.push(("album", album.as_str()));
        }

        let args = Signature::from_args(args_vec, &mut buffer, secret);

        let client = match ClientBuilder::new().build() {
            Ok(client) => client,
            Err(_) => return Err(ScrobbleError::ClientFailed)
        };

        let response = match client
            .post(crate::api::URL)
            .form(&args)
            .send()
            .await {
            Ok(response) => response,
            Err(_) => return Err(ScrobbleError::ResponseError)
        };

        let text = match response.text().await {
            Ok(text) => text,
            Err(_) => return Err(ScrobbleError::TextError)
        };

        println!("{text}");

        Ok(())
    }
}
