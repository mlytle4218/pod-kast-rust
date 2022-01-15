use reqwest::Client;
use reqwest::StatusCode;


#[tokio::main]
pub async fn search() {
    let client = Client::new();
    let resp = client.post("http://httpbin.org/post")
    .body("possibly too large")
    .send();

match resp.status() {
    StatusCode::OK => println!("success!"),
    StatusCode::PAYLOAD_TOO_LARGE => {
        println!("Request payload is too large!");
    }
    s => println!("Received response status: {:?}", s),
};
}