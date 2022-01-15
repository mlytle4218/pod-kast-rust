use std::collections::HashMap;
use reqwest;

#[tokio::main]
pub async fn search() -> Result<(), reqwest::Error> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}