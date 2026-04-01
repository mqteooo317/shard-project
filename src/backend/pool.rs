use std::sync::Arc;
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;

pub struct BackendPool {
    client: hyper::client::Client<HttpsConnector<HttpConnector>>,
    base_url: String,
}

impl BackendPool {
    pub fn new(base_url: &str) -> Self {
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .build();
        let client = hyper::client::Client::builder().build(https);
        Self {
            client,
            base_url: base_url.to_string(),
        }
    }

    pub fn client(&self) -> &hyper::client::Client<HttpsConnector<HttpConnector>> {
        &self.client
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}