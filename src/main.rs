use std::sync::Arc;
use shard::server::Listener;
use shard::server::Handler;
use shard::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = match Config::from_file("shard.toml") {
        Ok(c) => c,
        Err(_) => Config::default(),
    };
    let config = Arc::new(config);

    let handler = Arc::new(Handler::new(config.clone()));

    let addr = config.server.listen.parse()?;
    let listener = Listener::new(addr, handler);
    listener.run().await?;

    Ok(())
}