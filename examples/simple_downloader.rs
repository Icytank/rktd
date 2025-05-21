use anyhow::Result;
use rtkd::TikTokDownloader;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Get video URL and output path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <tiktok_url> <output_path>", args[0]);
        std::process::exit(1);
    }
    
    let tiktok_url = &args[1];
    let output_path = &args[2];
    
    // Replace this with your actual TikTok cookie
    let cookie = "cookie string from webui";
    
    println!("Initializing TikTok downloader...");
    
    // Initialize downloader with the cookie
    let downloader = TikTokDownloader::builder()
        .with_cookie(cookie)
        .build()?;
    
    println!("Downloading video from {} to {}", tiktok_url, output_path);
    
    // Download the video
    downloader.download_video(tiktok_url, output_path).await?;
    
    println!("Download complete!");
    Ok(())
} 