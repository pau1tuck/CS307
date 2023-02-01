use reqwest::Client;
use serde_json::{Value};

async fn get_json(url: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?.json().await?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = get_json("https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline/Mae%20Chan?unitGroup=metric&include=current&key=ASRQL55QSXYKL63Y2DCEMFUC3&contentType=json").await?;
    println!("{:?}", json);
    Ok(())
}


