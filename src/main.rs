use std::collections::BTreeMap;

mod token;
mod auth;

use md5::compute;

const CLIENT_ID: &str = "acd629208ea24b1fba383162d72e5d4f";
const CLIENT_SECRET: &str = "a0bf91b01b964ba4a922a5c600eba2e1";
const FM_USERNAME: &str = "TimeParadox_";
const FM_PASSWORD: &str = "f4a*e!hI$es3eh5r2pWh01OlXq8oF8Sq";
const FM_KEY: &str = "dadc1ebac6b8b736a639f4b0b3700a08";
const FM_SECRET: &str = "ac2fe7d93608e053bccd31415b61a127";

fn main() {
    let client = Client::new();

    let mut params: BTreeMap<String, String> = BTreeMap::new();
    params.insert("api_key".to_string(), FM_KEY.to_string());
    params.insert("method".to_string(), "auth.gettoken".to_string());
    params.insert("format".to_string(), "json".to_string());

    

    let res = client.post("https://ws.audioscrobbler.com/2.0/")
        .form(&params)
        .send();

    println!("{:?}", res.unwrap().text());
}
