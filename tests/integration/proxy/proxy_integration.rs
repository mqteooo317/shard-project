use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use hyper::{Request, Response, StatusCode};
use hyper::body::Incoming;
use http_body_util::{BodyExt, Full, Empty};
use bytes::Bytes;
use shard::server::{Handler, Listener};
use shard::config::Config;

// Backend server that returns a unique ID per request
async fn backend_handler(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let counter = req.headers().get("x-counter").and_then(|v| v.to_str().ok()).unwrap_or("0");
    let body = format!("Response from backend: {}", counter);
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(Full::new(Bytes::from(body)))
        .unwrap())
}

#[tokio::test]
async fn test_cache_hit_miss() {
    // Start a mock backend server
    let backend_addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let backend_listener = TcpListener::bind(backend_addr).await.unwrap();
    let backend_port = backend_listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        let service = hyper::service::service_fn(backend_handler);
        let server = hyper::server::Server::from_tcp(backend_listener.into_std().unwrap())
            .unwrap()
            .serve(service);
        server.await.unwrap();
    });

    // Configure Shard to point to the backend
    let mut config = Config::default();
    config.backend.url = format!("http://127.0.0.1:{}", backend_port);
    config.server.listen = "127.0.0.1:0".to_string();
    let config = Arc::new(config);

    let handler = Arc::new(Handler::new(config.clone()));
    let listener = Listener::new(config.server.listen.parse().unwrap(), handler);
    let shard_addr = listener.addr; // we need to expose addr in Listener; for now we'll parse from config

    // Actually we need to get the bound address after listener starts. But to simplify, we'll
    // run listener in a background task and connect.
    let shard_handle = tokio::spawn(async move {
        listener.run().await.unwrap();
    });

    // Wait a bit for servers to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Create a client
    let client = hyper::client::Client::new();

    // First request (should miss)
    let req = Request::builder()
        .uri(format!("http://{}", shard_addr))
        .body(Empty::<Bytes>::new())
        .unwrap();
    let resp = client.request(req).await.unwrap();
    let body = resp.collect().await.unwrap().to_bytes();
    let first_body = String::from_utf8_lossy(&body).to_string();

    // Second request (should hit)
    let req = Request::builder()
        .uri(format!("http://{}", shard_addr))
        .body(Empty::<Bytes>::new())
        .unwrap();
    let resp = client.request(req).await.unwrap();
    let body = resp.collect().await.unwrap().to_bytes();
    let second_body = String::from_utf8_lossy(&body).to_string();

    // The response should be the same (cached)
    assert_eq!(first_body, second_body);

    // Optional: verify cache header is set
    assert_eq!(resp.headers().get("X-Shard").unwrap(), "hit");

    shard_handle.abort();
}