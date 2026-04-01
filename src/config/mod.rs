use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub cache: CacheConfig,
    pub backend: BackendConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub listen: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CacheConfig {
    pub l1_memory_mb: usize,
    pub l2_path: String,
    pub default_ttl_seconds: u64,
    pub l2_ttl_seconds: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BackendConfig {
    pub url: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn default() -> Self {
        Self {
            server: ServerConfig {
                listen: "127.0.0.1:8080".to_string(),
            },
            cache: CacheConfig {
                l1_memory_mb: 2048,
                l2_path: "/var/lib/shard".to_string(),
                default_ttl_seconds: 3600,
                l2_ttl_seconds: 86400,
            },
            backend: BackendConfig {
                url: "http://127.0.0.1:8081".to_string(),
            },
        }
    }
}