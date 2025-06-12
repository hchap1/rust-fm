use reqwest::ClientBuilder;
use chrono::Utc;
use chrono::DateTime;

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

    pub async fn push_scrobble(
        auth: WebOAuth, scrobble: Scrobble, start_time: Option<DateTime<Utc>>
    ) -> Result<(), ScrobbleError> {

        let utc = match start_time {
            Some(start_time) => start_time,
            None => Utc::now()
        };

        let (key, session, secret) = match (auth.get_key(), auth.get_session(), auth.get_secret()) {
            (Some(key), Some(token), Some(secret)) => (key, token, secret),
            _ => return Err(ScrobbleError::MissingFields)
        };

        let mut buffer = String::new();
        let timestamp = utc.timestamp().to_string();

        let mut args_vec = vec![
            ("api_key", key),
            ("artist[0]", scrobble.artist.as_str()),
            ("method", "track.scrobble"),
            ("sk", session),
            ("track[0]", scrobble.title.as_str()),
            ("timestamp[0]", timestamp.as_str())
        ];

        if let Some(album) = scrobble.album.as_ref() {
            args_vec.push(("album[0]", album.as_str()));
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
