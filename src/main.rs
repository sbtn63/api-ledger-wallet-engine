#[tokio::main]
async fn main() -> anyhow::Result<()> {
    api_ledger_wallet_engine::run().await
}
