use anyhow::Ok;
use axum::{Json, Router, routing::get};
use serde_json::json;
use std::env;
use tokio::net::TcpListener;

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "service": "ledger-wallet-engine"
    }))
}

pub async fn run() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("APP_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()?;
    let address = format!("{host}:{port}");

    let app = Router::new().route("/health", get(health_check));

    let listener = TcpListener::bind(address).await?;

    print!("API escuchando en http://{host}:{port}");

    axum::serve(listener, app).await?;

    Ok(())
}
