use std::{collections::BTreeMap, env::var};

pub struct WebOAuth {
    api_key: Option<String>,
    api_secret: Option<String>,
    auth_token: Option<String>
}

impl WebOAuth {
    pub fn load_env() -> Self {
        Self {
            api_key: var("FM_KEY").ok(),
            api_secret: var("FM_SECRET").ok(),
            auth_token: var("FM_TOKEN").ok()
        }
    }

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
}

pub struct Signature;

impl Signature {
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
        buffer.push_str(format!("{:x}", md5::compute(api_sig)).as_str());

        map.insert("api_sig", buffer.as_str());
        map.insert("format", "json");
        map
    }
}
