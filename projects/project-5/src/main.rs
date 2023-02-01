use reqwest::Client;
use serde_json::{Value};
use mini_redis::{client, Result};

async fn get_json(url: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let reqwest_client = Client::new();
    let response = reqwest_client.get(url).send().await?.json().await?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    let json = get_json("https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline/Mae%20Chan?unitGroup=metric&include=current&key=ASRQL55QSXYKL63Y2DCEMFUC3&contentType=json").await?;
    println!("{:?}", json);
    Ok(())
}