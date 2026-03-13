mod downloader;
mod link_parser;
mod telegram;

use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Telegram Media Downloader CLI", long_about = None)]
struct Args {
    #[arg(help = "Telegram message link (e.g., https://t.me/channelname/123)")]
    link: String,

    #[arg(short, long, default_value = "downloads", help = "Output directory")]
    output: PathBuf,

    #[arg(long, env = "TG_API_ID", help = "Telegram API ID")]
    api_id: i32,

    #[arg(long, env = "TG_API_HASH", help = "Telegram API Hash")]
    api_hash: String,

    #[arg(long, default_value = "tgdl.session", help = "Session file path")]
    session: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let parsed_link = link_parser::parse_link(&args.link)?;
    println!("🔍 Parsed link: {:?}", parsed_link);

    if !args.output.exists() {
        std::fs::create_dir_all(&args.output).context("Failed to create output directory")?;
    }

    println!("🔐 Authenticating with Telegram...");
    let mut client = telegram::authenticate(args.api_id, args.api_hash, &args.session).await?;

    println!("📥 Fetching message and downloading media...");
    downloader::download_media(&mut client, parsed_link, &args.output).await?;

    println!("✅ Operation completed successfully.");
    Ok(())
}
