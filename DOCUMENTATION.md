# RTKD - API Documentation

## TikTokDownloader

The main struct for downloading TikTok videos.

### Methods

#### `builder() -> TikTokDownloaderBuilder`

Creates a new builder for configuring the downloader.

```rust
let builder = TikTokDownloader::builder();
```

#### `download_video(&self, video_url: &str, output_path: P) -> Result<()>`

Downloads a video from the given URL to the specified output path.

- `video_url` - The URL of the TikTok video to download
- `output_path` - The path where the video will be saved

```rust
downloader.download_video(
    "https://www.tiktok.com/@username/video/1234567890123456789",
    "/path/to/save/video.mp4"
).await?;
```

## TikTokDownloaderBuilder

Builder for configuring the TikTokDownloader.

### Methods

#### `new() -> Self`

Creates a new builder with default settings.

```rust
let builder = TikTokDownloaderBuilder::new();
```

#### `with_headers(mut self, headers: HashMap<String, String>) -> Self`

Sets custom HTTP headers.

```rust
let mut headers = HashMap::new();
headers.insert("User-Agent".to_string(), "Custom User Agent".to_string());
let builder = builder.with_headers(headers);
```

#### `with_header(mut self, key: &str, value: &str) -> Self`

Sets a specific HTTP header.

```rust
let builder = builder.with_header("User-Agent", "Custom User Agent");
```

#### `with_cookie(mut self, cookie: &str) -> Self`

Sets the cookie for authentication (required).

```rust
let builder = builder.with_cookie("your_tiktok_cookie_here");
```

#### `build(self) -> Result<TikTokDownloader>`

Builds the TikTokDownloader with the configured settings.

```rust
let downloader = builder.build()?;
```

## Constants

#### `CHUNK_SIZE: usize`

The chunk size used for downloading videos (2MB).

```rust
let chunk_size = rtkd::CHUNK_SIZE; // 2097152 bytes (2MB)
```

## Error Handling

All public methods return `anyhow::Result`, which allows for flexible error handling.

```rust
match downloader.download_video(url, path).await {
    Ok(_) => println!("Download successful"),
    Err(e) => eprintln!("Download failed: {}", e),
}
``` 