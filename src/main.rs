use std::env;
use std::collections::HashMap;

use reqwest::Error;
use serde::Deserialize;

mod utils;
use crate::utils::read_lines;

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

fn main() {
    let api_key = match env::var("GOOGLE_API_KEY") {
        Ok(val) => val,
        Err(_e) => panic!("Set up the GOOGLE_API_KEY environment variable first"),
    };

    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        panic!("You need to pass a text file name, a source and a target language");
    }

    println!(
        "args: {:?}, key: {:?}",
        args, api_key
    );

    let source = &args[2];
    let target = &args[3];

    // let translations = translate(query, source, target, &api_key);

    // println!("{:?}", translations);

    if let Ok(lines) = read_lines(&args[1]) {
        for line in lines {
            if let Ok(row) = line {
                let parsed_row = row.replace("\u{a0}", "");
                // println!("{}", parsed_row);
                let translation = match translate(&parsed_row, source, target, &api_key) {
                    Ok(t) => {
                        let t2 = String::from(&t.data.translations[0].translated_text);
                        t2
                    }
                    Err(_) => String::from(""),
                };
                println!("{}", translation);
            }
        }
    }
}

#[tokio::main]
async fn translate(query: &str, source: &str, target: &str, api_key: &str) -> Result<Data, Error> {
    let translations = match query.len() > 0 {
        true => {
            let base_endpoint = "https://translation.googleapis.com/language/translate/v2";
            let mut map = HashMap::new();
            map.insert("q", query);
            map.insert("source", source);
            map.insert("target", target);
            map.insert("key", &api_key);

            let request_url = format!("{base}?key={key}&q={query}&source={source}&target={target}", base = base_endpoint, key = api_key, query = query, source = source, target = target);

            let client = reqwest::Client::new();

            let response = client.post(&request_url).form(&map).send().await?;

            let text_response = response.text().await?;

            serde_json::from_str::<Data>(&text_response).unwrap()
        },
        false => {
            let tr =  Translation { translated_text: String::from("") };
            let trs: Vec<Translation> = vec!(tr);
            let data = Translations {
                translations: trs,
            };
            let empty_output = Data {
                data: data,
            };
            empty_output
        },
    };

    Ok(translations)
}
