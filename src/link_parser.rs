use anyhow::{anyhow, Result};
use regex::Regex;

#[derive(Debug)]
pub enum ChatRef {
    Username(String),
    Id(i64),
}

#[derive(Debug)]
pub struct ParsedLink {
    pub chat: ChatRef,
    pub message_id: i32,
}

pub fn parse_link(link: &str) -> Result<ParsedLink> {
    let public_re = Regex::new(r"https?://t\.me/([^/]+)(?:/\d+)?/(\d+)")?;
    let private_re = Regex::new(r"https?://t\.me/c/(\d+)(?:/\d+)?/(\d+)")?;

    if let Some(caps) = private_re.captures(link) {
        let chat_id = caps[1].parse::<i64>()?;
        let formatted_id = if chat_id > 0 {
            format!("-100{}", chat_id).parse::<i64>()?
        } else {
            chat_id
        };
        let message_id = caps[2].parse::<i32>()?;
        return Ok(ParsedLink {
            chat: ChatRef::Id(formatted_id),
            message_id,
        });
    }

    if let Some(caps) = public_re.captures(link) {
        let username = caps[1].to_string();
        if username.to_lowercase() == "c" {
            return Err(anyhow!("Invalid private link format"));
        }
        let message_id = caps[2].parse::<i32>()?;
        return Ok(ParsedLink {
            chat: ChatRef::Username(username),
            message_id,
        });
    }

    Err(anyhow!("Unsupported Telegram link format."))
}
