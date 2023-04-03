mod config;
mod http;

pub(crate) type Result<T = ()> = anyhow::Result<T>;

#[tokio::main]
async fn main() -> Result {
    http::run().await?;
    Ok(())
}