use anyhow::{Context, Result};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, COOKIE};
use std::collections::HashMap;
use regex::Regex;

/// Extract the actual video URL from a TikTok page URL
pub async fn extract_tiktok_video_url(
    url: &str, 
    headers: &HashMap<String, String>,
    cookie: &str
) -> Result<String> {
    // Create client with redirects enabled
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::custom(|attempt| {
            attempt.follow()
        }))
        .build()?;
    
    // Set up headers for the request
    let mut header_map = HeaderMap::new();
    for (k, v) in headers {
        header_map.insert(
            HeaderName::from_bytes(k.as_bytes())?,
            HeaderValue::from_str(v)?,
        );
    }
    
    if !cookie.is_empty() {
        header_map.insert(COOKIE, HeaderValue::from_str(cookie)?);
    }
    
    // Fetch the HTML content
    let resp = client
        .get(url)
        .headers(header_map)
        .send()
        .await
        .context("Failed to fetch TikTok page")?;
        
    let html = resp.text().await?;
    
    // Extract the actual video URL from the HTML
    // Look for patterns like: "playAddr":"https://..."
    let re = Regex::new(r#""playAddr":"([^"]+)""#)?;
    
    if let Some(captures) = re.captures(&html) {
        if let Some(url_match) = captures.get(1) {
            let video_url = url_match.as_str().replace("\\u002F", "/");
            return Ok(video_url);
        }
    }
    
    // If we can't find the video URL, save the HTML for debugging
    let debug_file = format!("tiktok_debug_{}.html", 
        url.split('/').last().unwrap_or("debug"));
    std::fs::write(&debug_file, &html)?;
    
    anyhow::bail!("Could not extract video URL from TikTok page")
} 