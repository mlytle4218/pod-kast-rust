use podcast_api::{Response,Error};
use serde_json::json;

// #[tokio::main]
// pub async fn find(terms: &str) -> Result<(), Error> {
//     // Api Key  (None => Test API, Some(key) => Production API)
//     let api_key = None;
//     // let api_key = Some("put your api key here");    

//     // Create client
//     let client = podcast_api::Client::new(api_key);

//     let res = client.typeahead(&json!({"q": terms,"show_podcasts":1}));
//     Ok(res)

// }



#[tokio::main]
pub async fn search(terms: &str) -> Result<Response, Error>{
    // Api Key  (None => Test API, Some(key) => Production API)
    let api_key = None;
    // let api_key = Some("put your api key here");    

    // Create client
    let client = podcast_api::Client::new(api_key);

    // Call API
    client
        .typeahead(&json!({
            "q": terms,
            "show_podcasts": 1
        }))
        .await
}

// #[tokio::main]
// pub async fn search(terms: &str) {
//     // Api Key  (None => Test API, Some(key) => Production API)
//     let api_key = None;
//     // let api_key = Some("put your api key here");    

//     // Create client
//     let client = podcast_api::Client::new(api_key);

//     // Call API
//     match client
//         .typeahead(&json!({
//             "q": terms,
//             "show_podcasts": 1
//         }))
//         .await
//     {
//         Ok(response) => {
//             println!("Successfully called Listen Notes API.");
//             if let Ok(body) = response.json().await {
//                 println!("Response Body:");
//                 println!("{}", body);
//             } else {
//                 println!("Response body JSON data parsing error.")
//             }
//         }
//         Err(err) => {
//             match err {
//                 Error::NotFoundError => { println!("Not Found: {}", err); }
//                 Error::AuthenticationError => { println!("Authentication Issue: {}", err); }
//                 Error::RateLimitError => { println!("Rate Limit: {}", err); }
//                 Error::InvalidRequestError => { println!("Invalid Request: {}", err); }
//                 Error::ListenApiError => { println!("API Error: {}", err); }
//                 Error::ApiConnectionError => { println!("Connection Issue: {}", err); }
//                 Error::Reqwest(err) => { println!("Reqwest HTTP Client Error: {}", err); }
//                 Error::Json(err) => { println!("JSON Parsing Error: {}", err); }
//             }
//         }
//     };
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[tokio::main]
//     async fn test_main() {
//         let res = search("news");
//         assert_eq!(res, true);
//     }
// }
