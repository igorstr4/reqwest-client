#[tokio::main]
async fn main() {

    env_logger::init();

    let http_client = reqwest::ClientBuilder::new()
        .connection_verbose(true)
        .build()
        .expect("Http client building error");

    let response = http_client.get("https://dummyjson.com/test")
        .send()
        .await
        .expect("HTTP GET request error");

    let json: serde_json::Value = response.json().await.expect("JSON error");

    println!("Response: {json:?}");
}
