# RTKD - Rust TikTok Video Downloader

A minimal and flexible Rust library for downloading TikTok videos.

## Features

- Simple API for downloading TikTok videos
- Builder pattern for flexible configuration
- Customizable HTTP headers
- Progress bar during download

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rtkd = "0.1.0"
```

Or add via cargo

```bash
cargo add rtkd
```

## Usage

```rust
use rtkd::TikTokDownloader;
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the downloader with required cookie
    let downloader = TikTokDownloader::builder()
        .with_cookie("your_tiktok_cookie_here")
        .build()?;
    
    // Download a TikTok video
    downloader.download_video(
        "https://www.tiktok.com/@username/video/1234567890123456789",
        "/path/to/save/video.mp4",
    ).await?;
    
    Ok(())
}
```

### Customizing Headers

You can customize the HTTP headers used for requests:

```rust
use rtkd::TikTokDownloader;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Custom headers
    let mut headers = HashMap::new();
    headers.insert("User-Agent".to_string(), "Your custom user agent".to_string());
    
    let downloader = TikTokDownloader::builder()
        .with_cookie("your_tiktok_cookie_here")
        .with_headers(headers)
        .build()?;
    
    // Download a TikTok video
    downloader.download_video(
        "https://www.tiktok.com/@username/video/1234567890123456789",
        "/path/to/save/video.mp4",
    ).await?;
    
    Ok(())
}
```

## License

This project is licensed under the MIT License. 
