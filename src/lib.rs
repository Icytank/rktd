mod config;
mod downloader;
mod extractor;

use std::collections::HashMap;
use std::path::Path;
use anyhow::Result;

/// Constant chunk size for video downloads (2MB)
pub const CHUNK_SIZE: usize = 2097152;

/// TikTok video downloader with builder pattern for configuration
#[derive(Debug, Clone)]
pub struct TikTokDownloader {
    headers: HashMap<String, String>,
    cookie: String,
}

/// Builder for TikTokDownloader
#[derive(Debug, Clone)]
pub struct TikTokDownloaderBuilder {
    headers: HashMap<String, String>,
    cookie: Option<String>,
}

impl Default for TikTokDownloaderBuilder {
    fn default() -> Self {
        let mut headers = HashMap::new();
        // Default headers from config.toml
        headers.insert(
            "User-Agent".to_string(), 
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36".to_string()
        );
        headers.insert(
            "Accept".to_string(),
            "*/*".to_string()
        );
        headers.insert(
            "Referer".to_string(),
            "https://www.tiktok.com/explore".to_string()
        );

        Self {
            headers,
            cookie: None,
        }
    }
}

impl TikTokDownloaderBuilder {
    /// Create a new builder with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set custom HTTP headers
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    /// Set a specific header
    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    /// Set the cookie for authentication
    pub fn with_cookie(mut self, cookie: &str) -> Self {
        self.cookie = Some(cookie.to_string());
        self
    }

    /// Build the TikTokDownloader
    pub fn build(self) -> Result<TikTokDownloader> {
        let cookie = self.cookie.ok_or_else(|| {
            anyhow::anyhow!("Cookie is required for TikTok video downloads")
        })?;

        Ok(TikTokDownloader {
            headers: self.headers,
            cookie,
        })
    }
}

impl TikTokDownloader {
    /// Create a new builder
    pub fn builder() -> TikTokDownloaderBuilder {
        TikTokDownloaderBuilder::new()
    }

    /// Download a TikTok video
    pub async fn download_video<P: AsRef<Path>>(&self, video_url: &str, output_path: P) -> Result<()> {
        // First extract the actual video URL
        let video_url = extractor::extract_tiktok_video_url(
            video_url,
            &self.headers,
            &self.cookie
        ).await?;
        
        // Then download it
        let config = self.create_internal_config(output_path)?;
        downloader::download_video(&video_url, &config).await
    }

    // Create internal config for the downloader
    fn create_internal_config<P: AsRef<Path>>(&self, output_path: P) -> Result<config::Config> {
        let output_path = output_path.as_ref();
        
        // Get the output path as a string
        let output_path_str = output_path.to_string_lossy().to_string();

        // Construct the internal config
        let http_config = config::HttpConfig {
            headers: self.headers.clone(),
            cookie: self.cookie.clone(),
            chunk_size: CHUNK_SIZE,
            output_dir: output_path_str,
        };

        Ok(config::Config {
            http: http_config,
        })
    }
} 