use crate::link_parser::{ChatRef, ParsedLink};
use anyhow::{anyhow, Context, Result};
use grammers_client::{types::Media, types::Message, Client};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn download_media(client: &mut Client, link: ParsedLink, out_dir: &Path) -> Result<()> {
    // 1. Resolve Chat Peer
    let chat = match link.chat {
        ChatRef::Username(ref username) => client
            .resolve_username(username)
            .await?
            .context("Could not find the channel/user")?,
        ChatRef::Id(id) => {
            let mut dialogs = client.iter_dialogs();
            let mut found_chat = None;
            while let Some(dialog) = dialogs.next().await? {
                if dialog.chat().id() == id {
                    found_chat = Some(dialog.chat().clone());
                    break;
                }
            }
            found_chat.context("Could not find the private chat in your dialogs")?
        }
    };

    // 2. Fetch the specific message
    let mut messages: Vec<Option<Message>> =
        client.get_messages_by_id(chat, &[link.message_id]).await?;
    let message = messages
        .pop()
        .flatten()
        .context("Message not found or deleted")?;

    // 3. Extract media
    let media = message.media().context("No media found in this message")?;

    let size = match &media {
        Media::Document(doc) => doc.size(),
        Media::Photo(_) => 0,
        _ => return Err(anyhow!("Unsupported media type")),
    };

    let filename = match &media {
        Media::Document(doc) => {
            let name = doc.name();
            if name.is_empty() {
                format!("document_{}.bin", link.message_id)
            } else {
                name.to_string()
            }
        }
        Media::Photo(_) => format!("photo_{}.jpg", link.message_id),
        _ => format!("media_{}", link.message_id),
    };

    let file_path = out_dir.join(&filename);
    println!(
        "📄 Found media: {} (Size: {} bytes)",
        filename,
        if size > 0 {
            size.to_string()
        } else {
            "Unknown".into()
        }
    );

    // 4. Setup Progress Bar
    let pb = if size > 0 {
        ProgressBar::new(size as u64)
    } else {
        ProgressBar::new_spinner()
    };

    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
            .progress_chars("#>-"),
    );

    // 5. Convert Media to Downloadable
    let downloadable = grammers_client::types::Downloadable::Media(media.clone());

    // 6. Stream download to file
    let mut file = File::create(&file_path).await?;
    let mut download_stream = client.iter_download(&downloadable);

    while let Some(chunk) = download_stream.next().await? {
        file.write_all(&chunk).await?;
        pb.inc(chunk.len() as u64);
    }

    pb.finish_with_message("Download complete");
    println!("💾 Saved to: {}", file_path.display());

    Ok(())
}
