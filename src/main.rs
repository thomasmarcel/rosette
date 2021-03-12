use std::env;

fn main() {
    println!("Hello, world!");

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
}
