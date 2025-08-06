use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    msl_engine::cli::run().await
}
