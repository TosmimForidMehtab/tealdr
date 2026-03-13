Project Title: Telegram MTProto Media Downloader CLI

Description:
Build a high-performance, terminal-based CLI utility that downloads media from specific Telegram message links using the Telegram MTProto API.

Core Requirements:

Authentication: * Authenticate via Telegram API ID, API Hash, and Phone Number (including support for 2FA/Passwords).

Implement persistent session storage locally so the user only logs in once.

Input & Parsing: * Accept a Telegram message link as the primary CLI argument.

Support public channel links (https://t.me/channelname/123), private chat links (https://t.me/c/123456789/123), and forum topic links (e.g., https://t.me/c/123456789/456/789).

Allow an optional flag to specify the output directory (defaulting to a downloads/ folder).

Advanced Media Handling:
* Support for downloading all items in a media album (grouped media) when a single link from the album is provided.

Media Extraction & Downloading:

Resolve the chat peer and fetch the specific message by its ID.

Detect media attachments (Documents, Videos, Photos). Return a clear error if no media is found.

Stream the download chunk-by-chunk to disk to handle large multi-gigabyte files without overloading memory.

Preserve the original file name and extension if available; otherwise, generate a fallback name using the message ID.

User Experience:

Display a real-time terminal progress bar during the download showing file size, download speed, and ETA.

Provide clear CLI feedback for authentication steps, parsed metadata, and successful file saves.
