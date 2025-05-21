use crate::config::Config;
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, COOKIE, RANGE};
use std::fs;
use std::path::Path;
use tokio::io::AsyncWriteExt;

pub async fn download_video(url: &str, config: &Config) -> Result<()> {
    // Create client with redirects enabled
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::custom(|attempt| {
            // Allow all redirects
            attempt.follow()
        }))
        .build()?;
    
    let mut headers = HeaderMap::new();
    for (k, v) in &config.http.headers {
        headers.insert(
            HeaderName::from_bytes(k.as_bytes())?,
            HeaderValue::from_str(v)?,
        );
    }
    if !config.http.cookie.is_empty() {
        headers.insert(COOKIE, HeaderValue::from_str(&config.http.cookie)?);
    }
    
    // Add Range header for full content
    headers.insert(RANGE, HeaderValue::from_static("bytes=0-"));

    // Get file path details
    let output_path = Path::new(&config.http.output_dir);
    
    // Create parent directory if it doesn't exist
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Create temporary file path for download
    let temp_path = if let Some(file_stem) = output_path.file_stem() {
        let mut temp_name = file_stem.to_string_lossy().to_string();
        if let Some(extension) = output_path.extension() {
            temp_name = format!("{}.part.{}", temp_name, extension.to_string_lossy());
        } else {
            temp_name = format!("{}.part", temp_name);
        }
        
        if let Some(parent) = output_path.parent() {
            parent.join(temp_name)
        } else {
            Path::new(&temp_name).to_path_buf()
        }
    } else {
        // Fallback if we can't extract a file stem
        let parent = output_path.parent().unwrap_or_else(|| Path::new("."));
        parent.join("video.part.mp4")
    };

    // Make request
    let resp = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .with_context(|| format!("Failed to GET {}", url))?;
    
    // Get content length if available
    let total_size = resp.content_length();

    // Create progress bar
    let pb = match total_size {
        Some(size) => ProgressBar::new(size),
        None => ProgressBar::new_spinner(),
    };

    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut file = tokio::fs::File::create(&temp_path).await?;
    let mut stream = resp.bytes_stream();
    let mut downloaded: u64 = 0;
    let _chunk_size = config.http.chunk_size; // Note: We're using the stream's natural chunking

    use futures_util::StreamExt;
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }
    
    pb.finish_with_message(format!("Downloaded {} bytes", downloaded));
    file.flush().await?;
    drop(file);
    
    // Move temp file to final location
    // We need to ensure the file is completely closed and flushed before renaming
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    tokio::fs::rename(&temp_path, &output_path).await?;
    Ok(())
} 