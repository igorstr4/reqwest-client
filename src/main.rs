use thiserror::Error;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::{SubscriberInitExt, TryInitError},
};

#[derive(Debug, Error)]
enum Error {
    #[error("Tracing initialization error {0}")]
    TracingInitError(#[from] TryInitError),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .try_init()?;

    let span = tracing::span!(tracing::Level::DEBUG, "span");

    let _entered = span.enter();

    tracing::info!("Before API call");

    let http_client = reqwest::ClientBuilder::new()
        .connection_verbose(true)
        .build()
        .expect("Http client building error");

    let response = http_client
        .get("https://dummyjson.com/test")
        .send()
        .await
        .expect("HTTP GET request error");

    let json: serde_json::Value = response.json().await.expect("JSON error");

    println!("Response: {json:?}");

    tracing::info!("After API call");

    Ok(())
}
