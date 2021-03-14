use std::env;
use std::collections::HashMap;

use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Translation {
    #[serde(alias = "translatedText")]
    translated_text: String,
}

#[derive(Deserialize, Debug)]
struct Translations {
    translations: Vec<Translation>,
}

#[derive(Deserialize, Debug)]
struct Data {
    data: Translations,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let base_endpoint = "https://translation.googleapis.com/language/translate/v2";
    let args: Vec<String> = env::args().collect();
    let api_key = match env::var("GOOGLE_API_KEY") {
        Ok(val) => val,
        Err(_e) => panic!("Set up the GOOGLE_API_KEY environment variable first"),
    };

    println!(
        "endpoint: {:?}, args: {:?}, key: {:?}",
        base_endpoint, args, api_key
    );

    let query = "rust";
    let source = "en";
    let target = "fr";

    let mut map = HashMap::new();
    map.insert("q", query);
    map.insert("source", source);
    map.insert("target", target);
    map.insert("key", &api_key);

    let request_url = format!("{base}?key={key}&q={query}&source={source}&target={target}", base = base_endpoint, key = api_key, query = query, source = source, target = target);
    // let request_url = format!("{base}?key={key}", base = base_endpoint, key = &api_key);

    let client = reqwest::Client::new();

    let response = client.post(&request_url).form(&map).send().await?;

    let text_response = response.text().await?;

    let translations= serde_json::from_str::<Data>(&text_response).unwrap();

    println!("{:?}", translations);

    Ok(())
}
