use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use crate::server::handler::Handler;

pub struct Listener {
    addr: SocketAddr,
    handler: Arc<Handler>,
}

impl Listener {
    pub fn new(addr: SocketAddr, handler: Arc<Handler>) -> Self {
        Self { addr, handler }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let tcp_listener = TcpListener::bind(&self.addr).await?;
        println!("Shard listening on {}", self.addr);

        loop {
            let (stream, _) = tcp_listener.accept().await?;
            let handler = self.handler.clone();
            tokio::task::spawn(async move {
                if let Err(e) = Self::handle_connection(stream, handler).await {
                    eprintln!("Connection error: {}", e);
                }
            });
        }
    }

    async fn handle_connection(
        mut stream: TcpStream,
        handler: Arc<Handler>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: PROXY protocol v2 parsing (optional)
        let io = TokioIo::new(stream);
        http1::Builder::new()
            .serve_connection(io, hyper::service::service_fn(move |req| {
                let handler = handler.clone();
                async move { handler.handle(req).await }
            }))
            .await?;
        Ok(())
    }
}