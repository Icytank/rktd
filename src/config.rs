use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct HttpConfig {
    pub headers: HashMap<String, String>,
    pub cookie: String,
    pub chunk_size: usize,
    pub output_dir: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub http: HttpConfig,
} 