use anyhow::Ok;
use axum::{Json, Router, routing::get};
use serde_json::json;
use tokio::net::TcpListener;

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "service": "ledger-wallet-engine"
    }))
}

pub async fn run() -> anyhow::Result<()> {
    let app = Router::new().route("/health", get(health_check));

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    print!("API escuchando en http://127.0.0.1.3000");

    axum::serve(listener, app).await?;

    Ok(())
}
