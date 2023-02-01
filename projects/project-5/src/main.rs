extern crate dotenv;
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct Weather {
    temperature: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // This line loads the environment variables from the ".env" file.
    let api_key = std::env::var("API_KEY").expect("API_KEY must be defined.");
    let address = "https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline/Mae%20Chan?unitGroup=metric&include=current&key=" + api_key + "&contentType=json";

    let client = Client::new();
    let response = client.get(address).send()?.json::<Weather>()?;

    println!("{:?}", response);
    Ok(())
}
