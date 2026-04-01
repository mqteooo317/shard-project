use std::sync::Arc;
use hyper::{body::Incoming, Request, Response, StatusCode};
use http_body_util::{BodyExt, Full, Empty};
use bytes::Bytes;
use crate::cache::multi_level::MultiLevelCache;
use crate::fragment::{BoundaryDetector, FragmentStore, Merger, TemplateCache, DependencyGraph};
use crate::backend::pool::BackendPool;
use crate::server::router::Router;
use crate::config::Config;

pub struct Handler {
    config: Arc<Config>,
    cache: Arc<MultiLevelCache>,
    fragment_store: Arc<FragmentStore>,
    template_cache: Arc<TemplateCache>,
    dependency_graph: Arc<DependencyGraph>,
    backend_pool: Arc<BackendPool>,
    router: Router,
}

impl Handler {
    pub fn new(config: Arc<Config>) -> Self {
        let cache = Arc::new(MultiLevelCache::new(
            config.cache.l1_memory_mb * 1024 * 1024,
            config.cache.default_ttl_seconds,
            &config.cache.l2_path,
            config.cache.l2_ttl_seconds,
        ));
        let fragment_store = Arc::new(FragmentStore::new());
        let template_cache = Arc::new(TemplateCache::new());
        let dependency_graph = Arc::new(DependencyGraph::new());
        let backend_pool = Arc::new(BackendPool::new(&config.backend.url));

        Self {
            config: Arc::new(config.clone()),
            cache,
            fragment_store,
            template_cache,
            dependency_graph,
            backend_pool,
            router: Router,
        }
    }

    pub async fn handle(&self, req: Request<Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
        let uri = req.uri().to_string();
        let method = req.method().as_str();

        // 1. Check if cacheable
        if !self.router.is_cacheable(&uri, method, req.headers()) {
            // Not cacheable -> proxy directly
            return self.proxy_backend(req).await;
        }

        // 2. Try to get from cache (full page or fragments)
        let cache_key = self.router.cache_key(&uri, method);
        if let Some(key) = cache_key {
            if let Some(cached_body) = self.cache.get(&key.to_string()).await {
                // Full page hit
                let response = Response::builder()
                    .status(StatusCode::OK)
                    .header("X-Shard", "hit")
                    .body(Full::new(Bytes::from(cached_body)))
                    .unwrap();
                return Ok(response);
            }
        }

        // 3. Miss: fetch from backend
        let backend_res = self.proxy_backend(req).await?;
        let (parts, body) = backend_res.into_parts();
        let body_bytes = body.collect().await?.to_bytes();

        // 4. If HTML and cacheable, fragment and store
        let content_type = parts.headers.get(hyper::header::CONTENT_TYPE);
        let is_html = content_type
            .and_then(|v| v.to_str().ok())
            .map(|ct| ct.contains("text/html"))
            .unwrap_or(false);

        if is_html && parts.status == StatusCode::OK {
            let html = String::from_utf8_lossy(&body_bytes);
            self.store_fragments(&uri, &html).await;
        }

        // 5. Store full page in cache
        if let Some(key) = cache_key {
            self.cache.set(&key.to_string(), body_bytes.to_vec()).await;
        }

        let response = Response::from_parts(parts, Full::new(body_bytes));
        Ok(response)
    }

    async fn proxy_backend(&self, req: Request<Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
        let client = hyper::client::Client::new();
        let backend_uri = format!("{}{}", self.config.backend.url, req.uri().path());
        let backend_req = Request::builder()
            .method(req.method())
            .uri(backend_uri)
            .body(req.into_body())
            .unwrap();
        let resp = client.request(backend_req).await?;
        let (parts, body) = resp.into_parts();
        let bytes = body.collect().await?.to_bytes();
        Ok(Response::from_parts(parts, Full::new(bytes)))
    }

    async fn store_fragments(&self, url: &str, html: &str) {
        let mut detector = BoundaryDetector::new();
        let candidates = detector.detect(html);
        let mut fragments = Vec::new();

        for cand in candidates {
            let fragment = crate::fragment::types::Fragment::new(cand.id.clone(), cand.content);
            self.fragment_store.set(cand.id.clone(), fragment.clone());
            fragments.push(fragment);
            self.dependency_graph.add_dependency(url.to_string(), cand.id);
        }

        // Create template with placeholders and store in template cache
        let placeholders: Vec<_> = fragments.iter().map(|f| crate::fragment::Placeholder::new(&f.id)).collect();
        let template = crate::fragment::Merger::create_template(html, &placeholders);
        self.template_cache.set(url.to_string(), template);
    }
}