use anyhow::{Context, Result};
use grammers_client::{Client, Config, InitParams, SignInError};
use grammers_session::Session;
use std::io::{self, Write};

pub async fn authenticate(api_id: i32, api_hash: String, session_path: &str) -> Result<Client> {
    let session = Session::load_file_or_create(session_path)?;

    let client = Client::connect(Config {
        session,
        api_id,
        api_hash,
        params: InitParams {
            catch_up: true,
            ..Default::default()
        },
    })
    .await
    .context("Failed to connect to Telegram MTProto API")?;

    if !client.is_authorized().await? {
        println!("📝 No active session found. Please log in.");

        let phone = prompt("Enter your phone number (e.g., +1234567890): ")?;
        let token = client.request_login_code(&phone).await?;

        let code = prompt("Enter the verification code sent to your Telegram: ")?;

        match client.sign_in(&token, &code).await {
            Ok(_) => println!("✅ Successfully logged in!"),
            Err(SignInError::PasswordRequired(password_token)) => {
                let password = prompt("Two-Step Verification enabled. Enter password: ")?;
                client.check_password(password_token, password.trim()).await?;
                println!("✅ Successfully logged in with password!");
            }
            Err(e) => return Err(e.into()),
        }

        client.session().save_to_file(session_path)?;
    }

    Ok(client)
}

fn prompt(message: &str) -> Result<String> {
    print!("{}", message);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
