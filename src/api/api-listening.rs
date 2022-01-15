use podcast_api::Error;
use serde_json::json;

// type TResult<T> = result::Result<T, TError>;
// type TError = Box<dyn error::Error>;

#[tokio::main]
pub async fn search(term: &str) -> Result<String, String> {
    let api_key = Some("6bffe94f48f442fab7ea09f38b8d130d");
    // let api_key = None;

    // Create client
    let client = podcast_api::Client::new(api_key);
    let mut search_result: String  ="nothing".to_string();


    // Call API
    match client
        .search(&json!({
            "q": term,
            "type": "podcast",
        }))
        .await
    {
        Ok(response) => {
            println!("Successfully called Listen Notes API.");
            if let Ok(body) = response.json().await {
                search_result = serde_json::to_string(&body).unwrap();
            } else {
                println!("Response body JSON data parsing error.")
            }
        }
        Err(err) => match err {
            Error::NotFoundError => {
                println!("Not Found: {}", err);
            }
            Error::AuthenticationError => {
                println!("Authentication Issue: {}", err);
            }
            Error::RateLimitError => {
                println!("Rate Limit: {}", err);
            }
            Error::InvalidRequestError => {
                println!("Invalid Request: {}", err);
            }
            Error::ListenApiError => {
                println!("API Error: {}", err);
            }
            Error::ApiConnectionError => {
                println!("Connection Issue: {}", err);
            }
            Error::Reqwest(err) => {
                println!("Reqwest HTTP Client Error: {}", err);
            }
            Error::Json(err) => {
                println!("JSON Parsing Error: {}", err);
            }
        },
    };
    Ok(search_result)
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_search() {
//         let res = search("news");
//         assert!(res.is_ok());
//     }
// }
