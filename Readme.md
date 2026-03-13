# 🚀 Tealdr: Telegram Media Downloader CLI

**Tealdr** is a high-performance, terminal-based utility written in Rust that allows you to download media directly from Telegram message links using the MTProto API. Unlike traditional bot-based downloaders, Tealdr acts as a native client, offering faster speeds and access to content available to your specific account.

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/tealdr)](https://crates.io/crates/tealdr)

## ✨ Features

- **High Performance**: Built with Rust for memory safety and zero-cost abstractions
- **Large File Support**: Streams downloads directly to disk to handle files over 2GB without memory overhead
- **Public & Private Support**: Seamlessly handles both public (`t.me/username/123`) and private (`t.me/c/12345/678`) links
- **Persistent Sessions**: Log in once via phone/2FA; your session is saved locally and securely for future use
- **Progress Tracking**: Interactive terminal progress bars showing download speed, file size, and ETA
- **Cross-Platform**: Works on Windows, macOS, and Linux

## 🛠️ Prerequisites

To use Tealdr, you must obtain your own Telegram API credentials. This ensures your downloads are associated with your own developer application.

1. Go to [my.telegram.org](https://my.telegram.org) and log in
2. Click on **API development tools**
3. Fill out the form (App Title and Short Name can be anything)
4. Save your **App api_id** and **App api_hash**

## 🚀 Installation

### Option 1: Build from Source

Ensure you have [Rust](https://rustup.rs/) installed on your system (version 1.70+ required).

```bash
# Clone the repository
git clone https://github.com/TosmimForidMehtab/tealdr.git
cd tealdr

# Build the release binary
cargo build --release

# The binary will be available at ./target/release/tealdr
```

### Option 2: Install via Cargo

```bash
cargo install tealdr
```

## 📖 Usage

### First Time Setup (Authentication)

The first time you run the tool, you need to provide your API credentials. You will be prompted in the terminal to enter your phone number and the verification code sent to your Telegram app.

```bash
# Run with API credentials
tealdr "https://t.me/channelname/123" --api-id YOUR_API_ID --api-hash "YOUR_API_HASH"
```

### Regular Use

Once authenticated, a `tgdl.session` file is created. You can simply pass the link for future downloads:

```bash
# Download a media file
tealdr "https://t.me/channelname/123"

# Download multiple files
tealdr "https://t.me/channelname/123" "https://t.me/channelname/124"
```

### Custom Output Directory

Use the `--output` flag to save files to a specific folder:

```bash
tealdr "https://t.me/channelname/123" --output ./my_videos
```

### Command Line Options

```bash
Usage: tealdr [OPTIONS] <LINKS>...

Arguments:
  <LINKS>...  Telegram message links to download media from

Options:
      --api-id <API_ID>        Your Telegram API ID (required for first run)
      --api-hash <API_HASH>    Your Telegram API Hash (required for first run)
  -o, --output <OUTPUT>        Output directory for downloaded files [default: .]
  -h, --help                   Print help
  -V, --version                Print version
```

## 🔧 Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/TosmimForidMehtab/tealdr.git
cd tealdr

# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- "https://t.me/channelname/123"
```

### Project Structure

```
src/
├── main.rs          # CLI entry point and argument parsing
├── telegram.rs      # Telegram client implementation
├── downloader.rs    # Media download logic
└── link_parser.rs   # Telegram link parsing utilities
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⚠️ Disclaimer

This tool is intended for personal use and educational purposes. Please respect Telegram's Terms of Service and only download content that you have the right to access. The developers are not responsible for any misuse of this software.

## 🐛 Troubleshooting

### Common Issues

- **Authentication failed**: Ensure your API credentials are correct and your phone number is verified
- **Session file issues**: Delete the `tgdl.session` file and re-authenticate
- **Connection issues**: Check your internet connection and firewall settings

### Getting Help

If you encounter any issues:

1. Check the [existing issues](https://github.com/TosmimForidMehtab/tealdr/issues)
2. Create a new issue with detailed information about the problem
3. Include your operating system, Rust version, and any error messages

## 🙏 Acknowledgments

- [Grammers](https://github.com/Lonami/grammers) - Telegram MTProto client library for Rust
- [Tokio](https://tokio.rs/) - Asynchronous runtime for Rust
- [Clap](https://clap.rs/) - Command line argument parser for Rust
- [Indicatif](https://github.com/console-rs/indicatif) - Progress bars and terminal utilities