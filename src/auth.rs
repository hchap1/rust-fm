use std::{collections::BTreeMap, env::var};

#[derive(Clone)]
pub struct WebOAuth {
    api_key: Option<String>,
    api_secret: Option<String>,
    auth_token: Option<String>,
    session: Option<String>
}

impl WebOAuth {
    /// Does the session exist? No guarantee of validity.
    pub fn ready(&self) -> bool {
        self.session.is_some()
    }

    /// Attempt to load key, secret and session from environment variables.
    pub fn load_env() -> Self {
        Self {
            api_key: var("FM_KEY").ok(),
            api_secret: var("FM_SECRET").ok(),
            auth_token: None,
            session: var("FM_SESSION").ok()
        }
    }

    /// Create an authentication object from Option<String> for each field.
    /// It is expected that api_key and api_sec are provided.
    pub fn from_key_and_secret(
        api_key: Option<String>,
        api_sec: Option<String>,
        session: Option<String>
    ) -> Self {
        Self {
            api_key,
            api_secret: api_sec,
            auth_token: None,
            session
        }
    }

    /// Spawn a browser prompting authentication, requires valid api key.
    pub async fn browser_auth(&self) {
        let key = match self.get_key() {
            Some(key) => key,
            None => return
        };

        let url = format!(
            "{}?api_key={}&cb={}",
            crate::api::AUTHURL,
            key,
            crate::token::CALLBACK_URL
        );

        let handle = open::that_in_background(&url);
        let _ = match tokio::task::spawn_blocking(move || handle.join()).await {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        };
    }

    pub fn get_key(&self) -> Option<&str> { return self.api_key.as_ref().map(|x| x.as_str()) }
    pub fn get_secret(&self) -> Option<&str> { return self.api_secret.as_ref().map(|x| x.as_str()) }
    pub fn get_token(&self) -> Option<&str> { return self.auth_token.as_ref().map(|x| x.as_str()) }
    pub fn get_session(&self) -> Option<&str> { return self.session.as_ref().map(|x| x.as_str()) }

    pub fn set_token(&mut self, token: String) { self.auth_token = Some(token); }
    pub fn set_session(&mut self, session: String) { self.session = Some(session); }
}

pub struct Signature;

impl Signature {
    /// Construct a BTreeMap for command arguments, including api signature.
    pub fn from_args<'a>(
        args: Vec<(&'static str, &'a str)>, buffer: &'a mut String, secret: &'a str
    ) -> BTreeMap<&'a str, &'a str> {
        let mut map = BTreeMap::new();
        for (key, val) in args {
            map.insert(key, val);
        }

        let mut api_sig = String::new();

        for (key, value) in map.iter() {
            api_sig.push_str(key);
            api_sig.push_str(value);
        }

        api_sig.push_str(secret);

        buffer.clear();
        buffer.push_str(Self::md5_hash(api_sig.as_str()).as_str());

        map.insert("api_sig", buffer.as_str());
        map.insert("format", "json");
        map
    }

    pub fn md5_hash(value: &str) -> String {
        format!("{:x}", md5::compute(value))
    }
}
