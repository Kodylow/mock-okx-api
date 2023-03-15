use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

use axum::http::HeaderValue;
use base64::{
    alphabet::{self, Alphabet},
    engine, Engine as _,
};
use hmac::{Hmac, Mac};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, net::SocketAddr};

const OKX_API_URL: &str = "https://www.okx.com";
pub const OKEX_MINIMUM_WITHDRAWAL_FEE: Decimal = dec!(0.0002);
pub const OKEX_MAXIMUM_WITHDRAWAL_FEE: Decimal = dec!(0.0004);
pub const OKEX_MINIMUM_WITHDRAWAL_AMOUNT: Decimal = dec!(0.001);
pub const OKEX_MAXIMUM_WITHDRAWAL_AMOUNT: Decimal = dec!(500);

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OkexClientConfig {
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub passphrase: String,
    #[serde(default)]
    pub secret_key: String,
    #[serde(default)]
    pub simulated: bool,
}

#[tokio::main]
async fn main() {
    // Load environment variables from .envrc file
    dotenv::from_filename(".env").ok();

    // Get the values of the environment variables
    let okex_api_key = env::var("OKEX_API_KEY").unwrap();
    let okex_secret_key = env::var("OKEX_SECRET_KEY").unwrap();
    let okex_passphrase = env::var("OKEX_PASSPHRASE").unwrap();

    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Result<(), AppError> {
    try_thing()?;
    Ok(())
}

fn try_thing() -> Result<(), anyhow::Error> {
    anyhow::bail!("it failed!")
}

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
